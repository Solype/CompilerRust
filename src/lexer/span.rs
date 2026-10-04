/// A piece of the source, as byte offsets (`end` excluded): `&src[start..end]` is its text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Line and column of the start, both from 1; the column counts characters, not bytes
    pub fn line_col(&self, src: &str) -> (usize, usize) {
        let before = &src[..self.start];
        let line = before.matches('\n').count() + 1;
        let line_start = before.rfind('\n').map_or(0, |i| i + 1);
        let col = before[line_start..].chars().count() + 1;
        (line, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_counts_characters() {
        let src = "가나\n정수를 주는";
        assert_eq!(Span::new(0, 3).line_col(src), (1, 1));
        assert_eq!(Span::new(3, 6).line_col(src), (1, 2));
        // 주는 starts after "정수를 " (4 characters, 10 bytes) on line 2
        let start = src.find("주는").unwrap();
        assert_eq!(Span::new(start, start + 6).line_col(src), (2, 5));
    }
}
