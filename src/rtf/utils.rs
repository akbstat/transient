use super::symbol::{
    ALTHABET_RANGE_END, ALTHABET_RANGE_START, LEFT_BRACE, NUMBER_RANGE_END, NUMBER_RANGE_START,
    RIGHT_BRACE,
};

/// find out a group posistion(index of start and end + 1), start with "{", end with "}"
///
/// if multiple braces including in one group, and they will be regarded as content of the group
///
/// @source: target that you want to find out group
///
/// @pointer: the start position in the source
///
/// @tail: the ending posistion in source
///
/// ```rust
/// #[test]
/// fn test_group_position() {
///     let content = b"abcd{abcd{}}".to_vec();
///     let result = group_position(&content, 0, content.len()).unwrap();
///     let result = String::from_utf8(content.get(result.0..result.1).unwrap().into()).unwrap();
///     assert_eq!("{abcd{}}", result);
/// }
/// ```
pub fn group_position(source: &[u8], pointer: usize, tail: usize) -> Option<(usize, usize)> {
    let mut pointer = pointer;
    let mut offset: Option<i32> = None;
    let mut start = None;
    while pointer < tail {
        match source.get(pointer) {
            Some(c) => match *c {
                LEFT_BRACE => {
                    if let None = start {
                        start = Some(pointer)
                    }
                    offset = match offset {
                        Some(offset) => Some(offset + 1),
                        None => Some(1),
                    };
                }
                RIGHT_BRACE => {
                    if let Some(_) = start {
                        offset = match offset {
                            Some(offset) => Some(offset - 1),
                            None => Some(-1),
                        }
                    }
                }
                _ => {}
            },
            None => {}
        }
        if let Some(offset) = offset {
            if 0.eq(&offset) {
                break;
            }
        }
        pointer += 1;
    }
    match start {
        Some(start) => Some((start, pointer + 1)),
        None => None,
    }
}

/// find out the position of specify pattern in occurs for the first time
///
/// @pattern: the pattern you want to find out
///
/// @source: target that you want to find out group
///
/// @pointer: the start position in the source
///
/// ```rust
/// #[test]
/// fn test_pattern_position() {
///     let content = br"{\fonttbl{\f1\froman\fprq2\fcharset0 SimSun;}".to_vec();
///     let pattern = br"\fonttbl".to_vec();
///     let result = pattern_position(&pattern, &content, 0).unwrap();
///     let result = String::from_utf8(content.get(result.0..result.1).unwrap().into()).unwrap();
///     assert_eq!(pattern, result.as_bytes());
///     let pattern = br"\test".to_vec();
///     let result = pattern_position(&pattern, &content, 0);
///     assert_eq!(result, None);
/// }
/// ```
pub fn pattern_position(pattern: &[u8], source: &[u8], pointer: usize) -> Option<(usize, usize)> {
    let mut pointer = pointer;
    let pattern_size = pattern.len();
    if pointer > source.len() {
        return None;
    }
    while pointer < source.len() {
        if pointer < pattern_size + 1 {
            pointer += 1;
            continue;
        }
        let start = pointer - pattern_size - 1;
        let end = pointer - 1;
        match source.get(start..end) {
            Some(target) => {
                if pattern.eq(target) {
                    return Some((start, end));
                }
            }
            None => {
                continue;
            }
        };
        pointer += 1;
    }
    None
}

pub fn not_alphabet_or_number(c: u8) -> bool {
    !not_number(c) && !not_alphabet(c)
}

fn not_number(c: u8) -> bool {
    c.ge(&NUMBER_RANGE_START) && c.le(&NUMBER_RANGE_END)
}

fn not_alphabet(c: u8) -> bool {
    c.ge(&ALTHABET_RANGE_START) && c.le(&ALTHABET_RANGE_END)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_group_position() {
        let content = b"abcd{abcd{}}".to_vec();
        let result = group_position(&content, 0, content.len()).unwrap();
        let result = String::from_utf8(content.get(result.0..result.1).unwrap().into()).unwrap();
        assert_eq!("{abcd{}}", result);
    }
    #[test]
    fn test_pattern_position() {
        let content = br"{\fonttbl{\f1\froman\fprq2\fcharset0 SimSun;}".to_vec();
        let pattern = br"\fonttbl".to_vec();
        let result = pattern_position(&pattern, &content, 0).unwrap();
        let result = String::from_utf8(content.get(result.0..result.1).unwrap().into()).unwrap();
        assert_eq!(pattern, result.as_bytes());
        let pattern = br"\test".to_vec();
        let result = pattern_position(&pattern, &content, 0);
        assert_eq!(result, None);
    }
    #[test]
    fn char_range_test() {
        let c = b'a';
        assert!(!not_alphabet_or_number(c));
        let c = b'1';
        assert!(!not_alphabet_or_number(c));
        let c = b'{';
        assert!(not_alphabet_or_number(c));
    }
}
