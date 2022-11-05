use smallvec::SmallVec;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Range<R> {
    pub range: std::ops::Range<usize>,
    pub role: R,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RangeList<R> {
    pub elements: SmallVec<[Range<R>; 4]>,
}

impl<R> RangeList<R> {
    pub fn new() -> Self {
        Self {
            elements: SmallVec::new(),
        }
    }

    pub fn push(&mut self, element: Range<R>) {
        self.elements.push(element);
    }
}
