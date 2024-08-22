use std::ops::Sub;

const RTF_UNICODE_MIN_LENGTH: usize = 5;
const RTF_UNICODE_MAX_LENGTH: usize = 8;
const PREFIX: &str = r"\u";
const SEMICOLON: &str = ";";

/// convert unicode sequences into string, for example: \u30740;\u31350; => "研究"
///
/// ```rust
/// #[test]
/// fn test_unicode_to_cn() {
///     let source = r"\u30740;\u31350;\u12290;";
///     let result = unicode_to_cn(source);
///     assert_eq!("研究。".to_string(), result);
///     let source = r"2023-02-25 (-25)";
///     let result = unicode_to_cn(source);
///     assert_eq!(source.to_string(), result);
/// }
/// ```
pub fn decode_unicode(source: &str) -> String {
    // a unicode char in rtf must owns more than 8 chars, such as "\u30740"
    let size = source.len();
    if size.lt(&RTF_UNICODE_MIN_LENGTH) {
        return source.into();
    }
    let mut result = String::with_capacity(size);
    let mut head = 0;
    let mut tail = head + PREFIX.len();
    while tail.lt(&size) {
        if tail.sub(&head).lt(&PREFIX.len()) {
            tail += 1;
            continue;
        }
        if source[head..tail].ne(PREFIX) {
            result.push_str(&source[head..head + 1]);
            head += 1;
            continue;
        }
        while tail.lt(&size)
            && source[tail - 1..tail].ne(SEMICOLON)
            && tail.sub(&head).le(&RTF_UNICODE_MAX_LENGTH)
        {
            tail += 1;
        }
        if source[head..head + PREFIX.len()].eq(PREFIX) && source[tail - 1..tail].eq(SEMICOLON) {
            if let Some(c) = unicode_to_char(&source[head..tail]) {
                result.push(c);
            }
        } else {
            result.push_str(&source[head..tail]);
        }
        head = tail;
        tail += 1;
    }
    // sweep the rest contents into result
    if head.lt(&size.sub(&PREFIX.len()))
        && source[head..head + &PREFIX.len()].eq(PREFIX)
        && source[tail - 1..tail].eq(SEMICOLON)
    {
        if let Some(c) = unicode_to_char(&source[head..tail]) {
            result.push(c);
        }
    } else {
        result.push_str(&source[head..]);
    }
    result
}

/// convert unicode in rtf to a char, for example: "\u30740;" => '研'
///
/// if invalid unicode then return None
///
/// ```rust
/// #[test]
/// fn test_decimal_str_to_hex() {
///     let source = r"\u30740;";
///     assert_eq!(unicode_to_char(source), Some('研'));
///     let source = r"\u31350;";
///     assert_eq!(unicode_to_char(source), Some('究'));
///     let source = r"\uc0";
///     assert_eq!(unicode_to_char(source), None);
/// }
/// ```
fn unicode_to_char(source: &str) -> Option<char> {
    if source.len().lt(&RTF_UNICODE_MIN_LENGTH) {
        return None;
    }
    if let Ok(code) = source[2..source.len() - 1].parse::<u32>() {
        char::from_u32(code)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_unicode_to_cn() {
        let source = r"\u30740;\u31350;\u12290;";
        let result = decode_unicode(source);
        assert_eq!("研究。".to_string(), result);
        let source = r"2023-02-25 (-25)";
        let result = decode_unicode(source);
        assert_eq!(source.to_string(), result);
        let source = r"\u20307;\u28201; (\u176;C)";
        let result = decode_unicode(source);
        assert_eq!("体温 (°C)".to_string(), result);
        let source = r"\u20307;\u28201; (\u176;C)\u30740;\u31350;";
        let result = decode_unicode(source);
        assert_eq!("体温 (°C)研究".to_string(), result);
    }
    #[test]
    fn test_decimal_str_to_hex() {
        let source = r"\u30740;";
        assert_eq!(unicode_to_char(source), Some('研'));
        let source = r"\u176;";
        assert_eq!(unicode_to_char(source), Some('°'));
        let source = r"\u31350;";
        assert_eq!(unicode_to_char(source), Some('究'));
        let source = r"\uc0";
        assert_eq!(unicode_to_char(source), None);
    }
}

// head == tail == pointer = 0
// tail - head < 2, tail + 1

// tail - head == 2
//  - source[head..tail] == "\u"
//      - source[tail] != ";", tail + 1
//      - source[tail] == ";"
//          - unicode = source[head+1..tail]
//          - tail = tail + 1, head = tail, pointer = tail
//  - source[head..tail] != "\u"
//  -
