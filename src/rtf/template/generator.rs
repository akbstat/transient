use std::{
    fs::{remove_file, File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

pub struct TemplateGenerator {
    buffer: BufWriter<File>,
}

impl TemplateGenerator {
    pub fn new(filepath: &Path) -> anyhow::Result<Self> {
        if filepath.exists() {
            remove_file(filepath)?;
        }
        let file = OpenOptions::new().write(true).create(true).open(filepath)?;
        let buffer = BufWriter::new(file);
        Ok(TemplateGenerator { buffer })
    }
    pub fn push(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.buffer.write_all(bytes)?;
        Ok(())
    }
    pub fn add_placeholder(&mut self, id: usize) -> anyhow::Result<()> {
        self.buffer.write_all(format!(r"{{#{}#}}", id).as_bytes())?;
        Ok(())
    }
    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.buffer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rtf_template() {
        let filepath = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\template.rtf");
        let mut t = TemplateGenerator::new(filepath).unwrap();
        t.push(r"\pard\plain\intbl\sb0\sa0\ql\f1\f0\f2\fs21\cf21".as_bytes())
            .unwrap();
        t.add_placeholder(0).unwrap();
        t.flush().unwrap();
    }
}
