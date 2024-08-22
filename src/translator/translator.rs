use std::collections::HashMap;

use regex::Regex;

use crate::{
    rtf::rtf_cell::{GeneralCell, RtfCell},
    utils::contains_chinese,
};

use super::llm::{self, TranslateParam};

#[derive(Debug)]
pub struct Translator {
    cache: HashMap<String, String>,
    api_key: String,
}

impl Translator {
    pub fn new(api_key: &str) -> Translator {
        let mut cache = HashMap::new();
        cache.insert("中山康方生物医药有限公司".into(), "Akesobio".into());
        cache.insert("康方赛诺医药有限公司".into(), "Akesobio".into());
        cache.insert(
            r"第 {\field{\*\fldinst { PAGE }}} 页 共 {\field{\*\fldinst { NUMPAGES }}} 页".into(),
            "Page Of ".into(),
        );
        Translator {
            cache,
            api_key: api_key.into(),
        }
    }
    pub fn translate(&mut self, rtf_cell: &RtfCell) -> RtfCell {
        match rtf_cell {
            RtfCell::General(cell) => {
                let mut translated_lines = vec![];
                cell.lines.iter().for_each(|line| {
                    let line = line.trim();
                    if line.is_empty() {
                        translated_lines.push("".into());
                    } else {
                        if let Some(data) = self.cache.get(line) {
                            translated_lines.push(data.into());
                            return;
                        }

                        let re = Regex::new(r"^方案编号：").unwrap();

                        // let result = llm::translate(line).unwrap();
                        // println!("{} - {}", line, result);
                        let result = if !contains_chinese(line) {
                            line.to_owned()
                        } else if re.is_match(line) {
                            line.replace("方案编号：", "Protocol: ")
                        } else {
                            llm::translate(TranslateParam {
                                source: &line,
                                api_key: &self.api_key,
                            })
                            .unwrap()
                        };
                        self.cache.insert(line.to_string(), result.clone());
                        translated_lines.push(result);
                    }
                });
                RtfCell::General(GeneralCell {
                    lines: cell.lines.clone(),
                    translated_lines,
                    styles: cell.styles.clone(),
                })
            }
            RtfCell::FootNote(source) => RtfCell::FootNote(
                source
                    .replace(r"\uc0\u26085 \uc0\u26399 \uc0\u65306", "Date:")
                    .replace(r"\uc0\u26469 \uc0\u28304 \uc0\u65306", "Source:")
                    .replace(r"\uc0\u65292", ", ")
                    .replace(r"\uc0\u36755 \uc0\u20986", "Output"),
            ),
        }
    }
}
