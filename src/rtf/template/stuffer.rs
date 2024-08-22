use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use regex::Regex;

use crate::rtf::{
    rtf_cell::{RtfCell, RtfCellSet},
    symbol::LINE,
};

pub struct Stuffer {
    reader: BufReader<File>,
    writer: BufWriter<File>,
}

pub struct StufferParam<'a> {
    pub template: &'a Path,
    pub destination: &'a Path,
}

impl Stuffer {
    pub fn new(param: &StufferParam) -> anyhow::Result<Self> {
        let reader = OpenOptions::new().read(true).open(param.template)?;
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .open(param.destination)?;
        let reader = BufReader::new(reader);
        let writer = BufWriter::new(writer);
        Ok(Stuffer { reader, writer })
    }
    pub fn stuff(&mut self, data: &RtfCellSet) -> anyhow::Result<()> {
        let mut line = String::new();
        let re = Regex::new(r"\{#(\d+)#\}")?;

        while let Ok(size) = self.reader.read_line(&mut line) {
            if size.eq(&0) {
                break;
            }

            let captures = re.captures_iter(&line);
            if captures.count().gt(&0) {
                let mut alter_line = line.clone();
                for caps in re.captures_iter(&line) {
                    if let Some(id) = caps.get(1).map(|item| item.as_str()) {
                        if let Ok(id) = id.parse::<usize>() {
                            if let Some(cell) = data.find(id) {
                                match cell {
                                    RtfCell::General(cell) => {
                                        let content = format!(
                                            "{{{}{}}}",
                                            cell.translated_lines.join(LINE),
                                            cell.styles
                                        );
                                        // let line = re.replace(&line, content).to_string();
                                        alter_line = re.replace(&alter_line, content).to_string();
                                        // io::copy(&mut line.as_bytes(), &mut self.writer)?;
                                    }
                                    RtfCell::FootNote(cell) => {
                                        let content = format!("{}", cell);
                                        alter_line = re.replace(&alter_line, content).to_string();
                                        // io::copy(&mut line.as_bytes(), &mut self.writer)?;
                                    }
                                }
                            }
                        }
                    }
                }
                io::copy(&mut alter_line.as_bytes(), &mut self.writer)?;
            } else {
                io::copy(&mut line.as_bytes(), &mut self.writer)?;
            }

            line.clear();
        }
        Ok(())
    }
    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::rtf::{
        rtf_cell::{GeneralCell, RtfCell},
        symbol::CELL,
    };

    use super::*;
    #[test]
    fn stuffer_test() {
        let param = StufferParam {
            template: Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\template.rtf"),
            destination: Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\stuffer.rtf"),
        };
        let mut stuffer = Stuffer::new(&param).unwrap();
        let cell_set = RtfCellSet::new();
        for i in 0..32 {
            let style = if i.eq(&31) { "".into() } else { CELL.into() };
            cell_set.add(RtfCell::General(GeneralCell {
                lines: vec![r"\u9794;".into()],
                translated_lines: vec![r"\u9794;".into()],
                styles: style,
            }));
        }
        stuffer.stuff(&cell_set).unwrap();
        stuffer.flush().unwrap();
    }
}
