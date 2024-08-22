use regex::Regex;

use crate::rtf::symbol::{CR, LF};

pub fn single_quote(source: &str) -> String {
    let re = Regex::new(r"\{\s*?(\\uc0)?\\u39\s*?\}").unwrap();
    re.replace_all(source, "'").to_string()
}

pub fn double_quote(source: &str) -> String {
    let re = Regex::new(r"\{\s*?(\\uc0)?\\u34\s*?\}").unwrap();
    re.replace_all(source, "\"").to_string()
}

pub fn percent(source: &str) -> String {
    let re = Regex::new(r"\{\s*?(\\uc0)?\\u37\s*?\}").unwrap();
    re.replace_all(source, "%").to_string()
}

pub fn depress_lf_cr(source: &str) -> String {
    source.replace(LF as char, "").replace(CR as char, "")
}
