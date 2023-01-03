use std::ops::Range;

#[derive(Debug)]
enum BidiRangeKind {
    LTR,
    RTL,
}

struct LangRangeKind(pub String);

trait RangeCollector {
    fn populate_collectors(&mut self);
}

trait BidiRangeCollector {}

#[derive(Debug)]
struct MyRangeCollector {
    pub bidi_ranges: Vec<(Range<usize>, BidiRangeKind)>,
}

impl MyRangeCollector {
    pub fn new() -> Self {
        Self {
            bidi_ranges: vec![],
        }
    }
}

impl RangeCollector for MyRangeCollector {
    fn populate_collectors(&mut self) {
        self.bidi_ranges.push((0..1, BidiRangeKind::LTR));
    }
}

impl BidiRangeCollector for MyRangeCollector {}

#[derive(Debug)]
enum PatternElement {
    Literal(String),
    Element,
}

struct Interpolator<'r> {
    pattern: &'r [char],
    idx: usize,
}

impl<'r> Interpolator<'r> {
    pub fn new(pattern: &'r [char]) -> Self {
        Self { pattern, idx: 0 }
    }

    fn next_with_ranges<R>(&mut self, collector: &mut R) -> Option<PatternElement>
    where
        R: RangeCollector,
    {
        if let Some(ch) = self.pattern.get(self.idx) {
            self.idx += 1;
            collector.populate_collectors();
            return Some(PatternElement::Literal(ch.to_string()));
        } else {
            return None;
        }
    }
}

impl<'r> Iterator for Interpolator<'r> {
    type Item = PatternElement;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.pattern.get(self.idx) {
            self.idx += 1;
            return Some(PatternElement::Literal(ch.to_string()));
        } else {
            return None;
        }
    }
}

fn main() {
    let pattern = &['a', 'b', 'c'];
    let mut rc = MyRangeCollector::new();
    let mut interpolator = Interpolator::new(pattern);

    let item = interpolator.next_with_ranges(&mut rc);
    println!("{:?}", rc);
    println!("{:?}", item);

    let item = interpolator.next_with_ranges(&mut rc);
    println!("{:?}", rc);
    println!("{:?}", item);

    let item = interpolator.next_with_ranges(&mut rc);
    println!("{:?}", rc);
    println!("{:?}", item);

    let item = interpolator.next_with_ranges(&mut rc);
    println!("{:?}", rc);
    println!("{:?}", item);
}
