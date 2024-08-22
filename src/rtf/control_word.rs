use std::cell::Cell;

use super::{
    symbol::{CR, LF, SLASH},
    utils::not_alphabet_or_number,
};

pub struct ControlWordIterator<'a> {
    cursor: Cell<usize>,
    bytes: &'a [u8],
}

impl<'a> ControlWordIterator<'a> {
    pub fn new(bytes: &'a [u8], cursor: usize) -> Self {
        ControlWordIterator {
            bytes,
            cursor: Cell::new(cursor),
        }
    }
    pub fn next(&self) -> Option<(usize, usize)> {
        if self.is_drained() {
            return None;
        }
        let mut cursor = self.cursor.get();
        // find out the first slash
        while cursor.lt(&self.bytes.len()) && self.bytes[cursor].ne(&SLASH) {
            let c = self.bytes[cursor];
            if not_alphabet_or_number(c) && CR.ne(&c) && LF.ne(&c) {
                return None;
            }
            cursor += 1;
        }
        if !cursor.lt(&self.bytes.len()) {
            return None;
        }
        let control_char_start = cursor;
        cursor += 1;
        // find out the second slash
        while cursor.lt(&self.bytes.len()) && self.bytes[cursor].ne(&SLASH) {
            let c = self.bytes[cursor];
            if not_alphabet_or_number(c) {
                break;
            }
            cursor += 1;
        }
        let control_char_end = cursor;
        self.cursor.set(cursor);
        if control_char_start < control_char_end {
            Some((control_char_start, control_char_end))
        } else {
            None
        }
    }
    pub fn set_cursor(&self, cursor: usize) {
        self.cursor.set(cursor);
    }
    pub fn cursor_move_one_step(&self) {
        self.set_cursor(self.cursor.get() + 1)
    }
    pub fn is_drained(&self) -> bool {
        self.cursor.get().ge(&self.bytes.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn control_word_itertor_test() {
        let content = br"\trowd\trkeep\trqc
\cltxlrtb\clvertalt\clcbpat20\cellx6977
\cltxlrtb\clvertalt\clcbpat20\cellx13954
\pard\plain\intbl\sb0\sa0\ql\f1\f0\f2\fs21\cf21{\u24247;\u26041;\u36187;\u35834;\u21307;\u33647;\u26377;\u38480;\u20844;\u21496;\cell}".to_vec();
        let c = ControlWordIterator::new(&content, 0);
        let word = c.next().unwrap();
        assert_eq!(br"\trowd".to_vec(), content[word.0..word.1]);
        let word = c.next().unwrap();
        assert_eq!(br"\trkeep".to_vec(), content[word.0..word.1]);
        let word = c.next().unwrap();
        assert_eq!(br"\trqc".to_vec(), content[word.0..word.1]);
        let word = c.next().unwrap();
        assert_eq!(br"\cltxlrtb".to_vec(), content[word.0..word.1]);
        loop {
            let cw = c.next();
            match cw {
                Some(cw) => println!(
                    "{}",
                    String::from_utf8(content[cw.0..cw.1].to_vec()).unwrap()
                ),
                None => break,
            }
        }
    }
}
