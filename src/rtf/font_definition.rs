use std::collections::HashSet;

use super::{
    symbol::{FONT_TBL, SLASH},
    utils,
};

/// find out codes of fonts in rtf files, return a hashset of font codes defined in rtf
///
/// ```rust
/// #[test]
/// fn test_fonts() {
///     let filepath = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\cn.rtf");
///     let bytes = fs::read(filepath).unwrap();
///     let font_list = font_set(&bytes);
///     assert_eq!(3, font_list.len());
// }
/// ```
pub fn font_set(bytes: &[u8]) -> HashSet<String> {
    let mut fonts = HashSet::new();
    let mut pointer = 0;
    while pointer < bytes.len() {
        if let Some(pos) = utils::pattern_position(&FONT_TBL, &bytes, pointer) {
            pointer = pos.0;
        } else {
            pointer += 1;
            continue;
        }
        if let Some(font_groups) = utils::group_position(&bytes, pointer - 1, bytes.len()) {
            while pointer < font_groups.1 {
                if let Some(font_detail) = utils::group_position(&bytes, pointer, font_groups.1) {
                    if let Some(font) = font_code(&bytes[font_detail.0..font_detail.1]) {
                        fonts.insert(font);
                    }
                    pointer = font_detail.1;
                } else {
                    pointer += 1;
                }
            }
            break;
        }
    }

    fonts
}

/// find out the font code in the target, such as f1, f2 etc
/// ```rust
/// #[test]
/// fn test_font_code() {
///     let source = br"{\f1\froman\fprq2\fcharset0 SimSun;}".to_vec();
///     let font = font_code(&source);
///     assert_eq!(Some("f1".to_string()), font);
/// }
/// ```
fn font_code(source: &[u8]) -> Option<String> {
    let mut start = 0;
    let mut end = start;
    while start < source.len() {
        if let Some(c) = source.get(start) {
            if SLASH.ne(c) {
                start += 1;
                continue;
            }
        } else {
            return None;
        }
        start += 1;
        end += start + 1;
        while end < source.len() {
            if let Some(c) = source.get(end) {
                if SLASH.ne(c) {
                    end += 1;
                    continue;
                } else {
                    break;
                }
            }
            break;
        }
        break;
    }
    Some(String::from_utf8(source.get(start..end).unwrap().to_vec()).unwrap())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;
    #[test]
    fn test_fonts() {
        let filepath = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\cn.rtf");
        let bytes = fs::read(filepath).unwrap();
        let font_list = font_set(&bytes);
        assert_eq!(3, font_list.len());
        let filepath = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\en.rtf");
        let bytes = fs::read(filepath).unwrap();
        let font_list = font_set(&bytes);
        assert_eq!(1, font_list.len());
    }
    #[test]
    fn test_font_code() {
        let source = br"{\f1\froman\fprq2\fcharset0 SimSun;}".to_vec();
        let font = font_code(&source);
        assert_eq!(Some("f1".to_string()), font);
    }
}
