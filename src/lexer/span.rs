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

    /// The text this span covers in `src`
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }

    /// The smallest span covering both (`가 + 나`: from the start of 가 to the end of 나)
    pub fn merge(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
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
    fn text_of_hangul_word() {
        let src = "정수를 주는";
        // 3 syllables of 3 bytes each
        assert_eq!(Span::new(0, 9).text(src), "정수를");
        assert_eq!(Span::new(10, 16).text(src), "주는");
    }

    #[test]
    fn merge_covers_both() {
        assert_eq!(Span::new(0, 3).merge(Span::new(6, 9)), Span::new(0, 9));
        assert_eq!(Span::new(6, 9).merge(Span::new(0, 3)), Span::new(0, 9));
    }

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
