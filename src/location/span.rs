#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn union_of(s1: Self, s2: Self) -> Self {
        let start = std::cmp::min(s1.start, s2.start);
        let end = std::cmp::max(s1.end, s2.end);
        Self::new(start, end)
    }

    pub fn as_range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}
