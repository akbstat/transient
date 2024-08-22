use std::{cell::RefCell, collections::HashMap};

use regex::Regex;

use super::symbol::{CELL, LINE};

#[derive(Debug, Clone)]
pub enum RtfCell {
    General(GeneralCell),
    FootNote(String),
}

// cells which contents are enclosed in curly brackets
#[derive(Debug, Clone)]
pub struct GeneralCell {
    pub lines: Vec<String>,
    pub translated_lines: Vec<String>,
    pub styles: String,
}

impl GeneralCell {
    pub fn new(bytes: &[u8]) -> anyhow::Result<Self> {
        let lines = String::from_utf8(bytes.to_vec())?;
        let (lines, style) = split_cell_content_style(&lines);
        Ok(GeneralCell {
            lines: lines
                .replace(CELL, "")
                .split(LINE)
                .map(|s| s.replace('\n', "").to_string())
                .collect::<Vec<String>>(),
            translated_lines: Vec::with_capacity(lines.len()),
            styles: style,
        })
    }
    pub fn process<F>(mut self, processor: F) -> Self
    where
        F: Fn(&str) -> String,
    {
        let lines = self
            .lines
            .iter()
            .map(|line| processor(line))
            .collect::<Vec<String>>();
        self.lines = lines;
        self
    }
}

#[derive(Debug, Clone)]
pub struct RtfCellSet {
    set: RefCell<HashMap<usize, RtfCell>>,
}

impl RtfCellSet {
    pub fn new() -> RtfCellSet {
        RtfCellSet {
            set: RefCell::new(HashMap::new()),
        }
    }
    pub fn add(&self, cell: RtfCell) -> usize {
        let mut set = self.set.borrow_mut();
        let id = set.len();
        set.insert(id, cell);
        id
    }
    pub fn find(&self, id: usize) -> Option<RtfCell> {
        let set = self.set.borrow();
        if let Some(cell) = set.get(&id) {
            Some(cell.clone())
        } else {
            None
        }
    }
    pub fn update(&mut self, id: usize, cell: &RtfCell) {
        if let Some(_) = self.find(id) {
            (*self.set.borrow_mut()).insert(id, cell.clone());
        }
    }
    pub fn size(&self) -> usize {
        self.set.borrow().len()
    }

    pub fn term_set(&self) -> Vec<(String, String)> {
        let mut set: HashMap<String, String> = HashMap::new();
        for (_, cell) in self.set.borrow().iter() {
            if let RtfCell::General(cell) = cell {
                let sources = cell.lines.clone();
                let translations = cell.translated_lines.clone();
                for (i, source) in sources.iter().enumerate() {
                    if let Some(translation) = translations.get(i) {
                        set.insert(source.to_owned(), translation.to_owned());
                    }
                }
            }
        }
        set.into_iter().map(|pair| pair).collect()
    }

    pub fn rebuild(&self, mapper: &HashMap<String, String>) -> Self {
        let mut new_set = self.clone();
        for (id, cell) in self.set.borrow().iter() {
            if let RtfCell::General(cell) = cell {
                let mut new_cell = cell.clone();
                for (index, source) in cell.lines.iter().enumerate() {
                    let origin_translate = cell.lines.get(index).unwrap();
                    if let Some(alter_translate) = mapper.get(source) {
                        if alter_translate.ne(origin_translate) {
                            new_cell.translated_lines[index] = alter_translate.to_owned();
                        }
                    }
                }
                new_set.update(*id, &RtfCell::General(new_cell));
            }
        }
        new_set
    }
}

/// split the content and style code within a cell in rtf, for example:
///
/// (N=198)\brdrb\brdrs\cell will be seperated into 2 parts:
///
///     1. (N=198)
///
///     2. \brdrb\brdrs\cell
fn split_cell_content_style(source: &str) -> (String, String) {
    let re = Regex::new(r"(\\[a-z|0-9]+?)*?\\cell$").unwrap();
    if let Some(range) = re.find(source) {
        let range = range.range();
        let content = &source[..range.start];
        let style = &source[range];
        (content.into(), style.into())
    } else {
        (source.into(), "".into())
    }
}

#[cfg(test)]
mod tests {

    use crate::rtf::processor::{double_quote, single_quote};

    use super::*;
    #[test]
    fn test_split_cell_content_style() {
        let source = r"(N=198)\brdrb\brdrs\cell";
        let result = split_cell_content_style(source);
        assert_eq!("(N=198)", &result.0);
        assert_eq!(r"\brdrb\brdrs\cell", &result.1);
        let source = r"(N=198)\cell";
        let result = split_cell_content_style(source);
        assert_eq!("(N=198)", &result.0);
        assert_eq!(r"\cell", &result.1);
    }

    #[test]
    fn cell_test() {
        let content = r"{\line}
CTCAE: Common Terminology Criteria for Adverse Events. DLT: Dose-Limiting Toxicity. IP: Investigational Product. MedDRA: Medical Dictionary for Regulatory Activities. Q2W: Once Every 2 Weeks.{\line}
[a] Study day is calculated relative to the first IP administration and is only presented for subjects with an administration date.{\line}
[b] Duration = the difference between the time of resolution and onset time of adverse event. Using ISO 8601 duration formats: PnYnMnDTnHnMnS ([P] is the duration designator, [T] is used to separate the date components from time components, the {\u34 }n{\u34 } preceding each letter represents the number of years, months, days, hours, minutes, seconds, or the number of weeks). If {\uc0\u39 }M{\uc0\u39 } is before {\uc0\u39 }T{\uc0\u39 } like {\uc0\u39 }P5M{\uc0\u39 } then it presents months, and if {\uc0\u39 }M{\uc0\u39 } is after {\uc0\u39 }T{\uc0\u39 } like {\uc0\u39 }PT5M{\uc0\u39 } then it presents minutes.{\line}
Adverse events were coded using MedDRA Version 25.0 and severity were graded according to CTCAE version 5.0.\cell".as_bytes();
        let c: GeneralCell = GeneralCell::new(content).unwrap();
        assert_eq!(c.lines.len(), 5);
        let c = c.process(single_quote).process(double_quote);
        c.lines.iter().for_each(|line| println!("{}", line));

        let content = r"{\line}
        \u30740;\u31350;\u26085;\u22522;\u20110;\u21463;\u35797;\u32773;\u30740;\u31350;\u33647;\u29289;\u65288;\u20381;\u27779;\u35199;\u21333;\u25239;/\u24085;\u21338;\u21033;\u29664;\u21333;\u25239;\u65289;\u39318;\u27425;\u32473;\u33647;\u26085;\u26399;\u36827;\u34892;\u35745;\u31639;\u12290;\cell".as_bytes();
        let c: GeneralCell = GeneralCell::new(content).unwrap();
        assert_eq!(c.lines.len(), 2);
        c.lines.iter().for_each(|line| println!("{}", line));
    }
}
