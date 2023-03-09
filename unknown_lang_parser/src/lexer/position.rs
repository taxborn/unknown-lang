#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytePos(pub u32);

impl BytePos {
    pub fn advance(self, ch: char) -> Self {
        Self(self.0 + ch.len_utf8() as u32)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start: BytePos(start),
            end: BytePos(end),
        }
    }

    pub fn empty() -> Self {
        Self {
            start: BytePos(0),
            end: BytePos(0),
        }
    }

    /// Constructing a [`Span`] by taking the union of the two spans, where we
    /// create a Span from getting the 'earliest' of the two spans as the
    /// start, and the 'latest' of the two as the end.
    ///
    /// # Examples
    /// ```
    /// use unknown_lang_parser::lexer::position::{Span, BytePos};
    ///
    /// let s1: Span = Span::new(4, 10);
    /// let s2: Span = Span::new(2, 8);
    ///
    /// // This would yield a span from the BytePos 2 to the BytePos 10.
    /// let s3: Span = Span::union_span(s1, s2);
    /// ```
    pub fn union_span(span_a: Self, span_b: Self) -> Self {
        Self {
            start: std::cmp::min(span_a.start, span_b.start),
            end: std::cmp::max(span_a.end, span_b.end),
        }
    }

    /// Constructing a [`Span`] by taking the union of the two spans, where we
    /// create a Span from getting the 'earliest' of the two spans as the
    /// start, and the 'latest' of the two as the end.
    ///
    /// # Examples
    /// ```
    /// use unknown_lang_parser::lexer::position::{Span, BytePos};
    ///
    /// let s1: Span = Span::new(4, 10);
    /// let s2: Span = Span::new(2, 8);
    ///
    /// // This would yield a span from the BytePos 2 to the BytePos 10.
    /// let s3: Span = s1.union(s2);
    /// ```
    pub fn union(self, other: Self) -> Self {
        Self {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        }
    }
}

impl<T> From<Spanned<T>> for Span {
    fn from(spanned: Spanned<T>) -> Span {
        spanned.span
    }
}

impl<T> From<&Spanned<T>> for Span {
    fn from(spanned: &Spanned<T>) -> Span {
        spanned.span
    }
}

pub struct Spanned<T> {
    pub data: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Self { data, span }
    }

    pub fn empty(data: T) -> Self {
        Self {
            data,
            span: Span::empty(),
        }
    }

    pub fn new_span(data: T, start: u32, end: u32) -> Self {
        Self {
            data,
            span: Span::new(start, end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let span_1 = Span::new(4, 10);
        let span_2 = Span::new(2, 8);

        let union = Span::union_span(span_1, span_2);

        assert_eq!(union, Span::new(2, 10));
    }

    #[test]
    fn test_union_on_span() {
        let span_1 = Span::new(4, 10);
        let span_2 = Span::new(2, 8);

        let union = span_1.union(span_2);

        assert_eq!(union, Span::new(2, 10))
    }

    #[test]
    fn test_union_equivalence() {
        let span_1 = Span::new(4, 10);
        let span_2 = Span::new(2, 8);

        let union_1 = span_1.union(span_2);
        let union_2 = span_2.union(span_1);

        assert_eq!(union_1, union_2);
    }
}
