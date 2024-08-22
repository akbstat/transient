use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::translator::Translator;

use super::{
    control_word::ControlWordIterator,
    font_definition,
    footnote::footnote_position,
    processor::{decode_unicode, depress_lf_cr, double_quote, percent, single_quote},
    rtf_cell::{GeneralCell, RtfCell, RtfCellSet},
    symbol::WINDOW_CTRL,
    template::TemplateGenerator,
    utils::{group_position, pattern_position},
};

pub struct SundererParam<'a> {
    pub source: &'a Path,
    pub workspace: &'a Path,
}

/// to seperate rtf to template and contents
pub struct Sunderer {
    generator: TemplateGenerator,
    bytes: Vec<u8>,
    cell_set: RtfCellSet,
    template_path: PathBuf,
}

impl Sunderer {
    pub fn new(param: &SundererParam) -> anyhow::Result<Self> {
        let SundererParam { source, workspace } = param;
        let file_stem = source.file_stem().unwrap().to_string_lossy();
        let template = workspace.join(format!("{}.rtf.tmp", file_stem));
        let template = template.as_path();
        if template.exists() {
            fs::remove_file(template)?;
        }
        let generator = TemplateGenerator::new(&template)?;
        let cell_set = RtfCellSet::new();
        let bytes = fs::read(source)?;
        Ok(Sunderer {
            generator,
            bytes,
            cell_set,
            template_path: template.into(),
        })
    }
    /// split content rtf to cell set and template, return cell set
    pub fn split(&mut self) -> anyhow::Result<&mut Self> {
        let font_set = font_definition::font_set(&self.bytes);
        let mut content_start = 0;
        // find out the position of content part
        let window_ctrl = pattern_position(&WINDOW_CTRL, &self.bytes, 0).unwrap();
        let control_word_iterator = ControlWordIterator::new(&self.bytes, window_ctrl.0);

        let mut last_group: Option<(usize, usize)> = None;
        while !control_word_iterator.is_drained() {
            let cw = control_word_iterator.next();
            match cw {
                Some(cw) => {
                    let b = self.bytes[cw.0..cw.1].to_vec();
                    let control_word = String::from_utf8(b).unwrap();
                    if let Some(_) = font_set.get(&control_word[1..]) {
                        let start = cw.1;
                        let group: (usize, usize) =
                            group_position(&self.bytes, start, self.bytes.len()).unwrap();

                        // check contents between 2 groups whether it is a footnote
                        if let Some(last_group) = last_group {
                            if let Some(footnote_range) =
                                footnote_position(&self.bytes, (last_group.1, group.0))
                            {
                                if let Some(footnote) =
                                    self.bytes.get(footnote_range.0..footnote_range.1)
                                {
                                    let footnote_cell =
                                        RtfCell::FootNote(String::from_utf8(footnote.to_vec())?);
                                    let id = self.cell_set.add(footnote_cell);
                                    self.generator
                                        .push(&self.bytes[content_start..footnote_range.0])?;
                                    self.generator.add_placeholder(id)?;
                                    content_start = footnote_range.1;
                                }
                            }
                        }

                        let cell = GeneralCell::new(&self.bytes[group.0 + 1..group.1 - 1])?;
                        let cell = cell
                            .process(decode_unicode)
                            .process(depress_lf_cr)
                            .process(percent)
                            .process(single_quote)
                            .process(double_quote);

                        let id = self.cell_set.add(RtfCell::General(cell));

                        // push contents into buffer
                        self.generator.push(&self.bytes[content_start..group.0])?;
                        self.generator.add_placeholder(id)?;
                        content_start = group.1;
                        control_word_iterator.set_cursor(group.1 + 1);
                        last_group = Some(group);
                    }
                }
                None => {
                    control_word_iterator.cursor_move_one_step();
                }
            }
        }

        // check contents between 2 groups whether it is a footnote
        if let Some(last_group) = last_group {
            if let Some(footnote_range) =
                footnote_position(&self.bytes, (last_group.1, self.bytes.len()))
            {
                if let Some(footnote) = self.bytes.get(footnote_range.0..footnote_range.1) {
                    let footnote_cell = RtfCell::FootNote(String::from_utf8(footnote.to_vec())?);
                    // let footnote_cell = translator.translate(&footnote_cell);
                    let id = self.cell_set.add(footnote_cell);
                    self.generator
                        .push(&self.bytes[content_start..footnote_range.0])?;
                    self.generator.add_placeholder(id)?;
                    content_start = footnote_range.1;
                }
            }
        }
        self.generator.push(&self.bytes[content_start..])?;
        self.generator.flush()?;
        Ok(self)
    }

    pub fn translate(&mut self, translator: &mut Translator) -> anyhow::Result<RtfCellSet> {
        for id in 0..self.cell_set.size() {
            if let Some(cell) = self.cell_set.find(id) {
                let cell = translator.translate(&cell);
                self.cell_set.update(id, &cell)
            }
        }
        Ok(self.cell_set.clone())
    }

    pub fn template_path(&self) -> PathBuf {
        self.template_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::translator::Translator;

    use super::*;
    #[test]
    fn sunderder_test() -> anyhow::Result<()> {
        let mut translator = Translator::new("<your api key>");
        let source = Path::new(
            r"D:\projects\rusty\playground\rtf\.sample_data\test\t-14-01-03-08-pr-fas.rtf",
        );
        let workspace = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\workspace");
        let mut sunderder = Sunderer::new(&SundererParam { source, workspace })?;
        let cell_set = sunderder.split()?.translate(&mut translator)?;
        println!("{:?}", cell_set);
        Ok(())
    }
}
