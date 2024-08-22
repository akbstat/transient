use super::{symbol::CHCBPAT0, utils::pattern_position};

/// to find out the position of first footnote in given bytes
pub fn footnote_position(source: &[u8], range: (usize, usize)) -> Option<(usize, usize)> {
    if source[range.0..range.0 + 6].eq(r"\f2\fs".as_bytes()) {
        let chcbpat0_position = pattern_position(CHCBPAT0.as_bytes(), source, range.0);
        if let Some(chcbpat0_position) = chcbpat0_position {
            Some((range.0, chcbpat0_position.1))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn footnote_position_test() {
        let source = br"\f2\fs16 \par \uc0\u26469 \uc0\u28304 \uc0\u65306  ..\\ak112\\303\\stats\\csr\\product\\program\\tfl\\t-14-02-02-01-os-fas.sas Zhengkun.Liang SAS9.4\uc0\u65288 DCO \uc0\u26085 \uc0\u26399 \uc0\u65306 29JAN2024\uc0\u65292 \uc0\u36755 \uc0\u20986 \uc0\u26085 \uc0\u26399 \uc0\u65306 29MAY2024 15:26\uc0\u65289 \uc1\cf0\chcbpat0
{\par}{\pard\plain\qc{
}\par}{\page\par}
\trowd\trkeep\trqc
\cltxlrtb\clvertalt\clcbpat20\cellx6977
\cltxlrtb\clvertalt\clcbpat20\cellx13954
\pard\plain\intbl\sb0\sa0\ql\f1\f2\f2\fs21\cf21";
        let result = footnote_position(source, (0, source.len()));
        assert!(result.is_some());
    }
}
