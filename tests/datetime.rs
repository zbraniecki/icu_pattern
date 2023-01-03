use std::borrow::Cow;
use std::ops::Range;

use icu_pattern::{
    interpolator::Interpolator,
    pattern::{Pattern, PatternElement},
    range::RangeCollector,
};

pub enum BidiRangeKind {
    LTR,
    RTL,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DateElement {
    Year,
    Month,
    Day,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DateOutputElement<'output> {
    Literal(Cow<'output, str>),
    Element(&'output DateElement),
}

impl<'input> From<&PatternElement<'input, DateElement>> for DateOutputElement<'input> {
    fn from(value: &PatternElement<'input, DateElement>) -> Self {
        match value {
            PatternElement::Literal(l) => DateOutputElement::Literal(l.clone()),
            PatternElement::Placeholder(p) => todo!(),
        }
    }
}

impl<'input> From<&'input DateElement> for DateOutputElement<'input> {
    fn from(input: &'input DateElement) -> Self {
        Self::Element(input)
    }
}

pub struct Data {
    pub date_pattern: DatePattern<'static>,
}

pub struct DatePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<'output, DateElement>>,
}

impl<'output> Iterator for DatePatternIterator<'output> {
    type Item = DateOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct DatePattern<'input> {
    pub elements: Vec<PatternElement<'input, DateElement>>,
}

impl<'input> Iterator for DatePattern<'input> {
    type Item = PatternElement<'input, DateElement>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'input> Pattern<'input> for DatePattern<'input> {
    type PatternElement = PatternElement<'input, DateElement>;
    type OutputPatternElement = DateOutputElement<'input>;

    fn interpolate<'p>(
        &'p self,
    ) -> Interpolator<std::slice::Iter<'p, Self::PatternElement>, Self::OutputPatternElement> {
        Interpolator {
            pattern: self.elements.iter(),
            marker: std::marker::PhantomData,
        }
    }
}

struct MyDateRangeCollector {
    pub bidi_ranges: Vec<(Range<usize>, BidiRangeKind)>,
}

impl MyDateRangeCollector {
    pub fn new() -> Self {
        Self {
            bidi_ranges: vec![],
        }
    }
}

impl RangeCollector for MyDateRangeCollector {
    fn populate_collector(&mut self) {
        self.bidi_ranges.push((0..1, BidiRangeKind::LTR));
    }
}

#[test]
fn core_date_test() {
    let data = Data {
        date_pattern: DatePattern {
            elements: vec![PatternElement::Literal(Cow::Borrowed("Foo"))],
        },
    };

    let pattern = &data.date_pattern;

    let mut interpolator = pattern.interpolate();

    let mut rc = MyDateRangeCollector::new();

    let item = interpolator.next_with_collector(&mut rc);

    assert_eq!(rc.bidi_ranges.len(), 1);
    assert_eq!(item, Some(DateOutputElement::Literal("Foo".into())));
}
