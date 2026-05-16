use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Span { start: value.start, end: value.end }
    }
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.start .. value.end
    }
}

pub enum Result<T> {
    Ok(T),
    Error(
        Span, // span of the erroneous code
        String // the error message
    )
}
