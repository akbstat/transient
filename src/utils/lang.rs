pub fn contains_chinese(source: &str) -> bool {
    if source.chars().any(|c| c > '\u{7F}') {
        true
    } else {
        false
    }
}
