use crate::data::types::*;
use crate::pattern::Never;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub enum OutputElement {
    Literal(String),
    Date(DatePatternElement),
    Time(TimePatternElement),
    Timezone(TimezonePatternElement),
}

impl From<TimePatternElement> for OutputElement {
    fn from(input: TimePatternElement) -> Self {
        Self::Time(input)
    }
}

impl From<DatePatternElement> for OutputElement {
    fn from(input: DatePatternElement) -> Self {
        Self::Date(input)
    }
}

impl From<Never> for OutputElement {
    fn from(_: Never) -> Self {
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OutputRange {
    pub range: Range<usize>,
    pub role: OutputRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputRole {
    Date,
    Time,
    DateTime,
    Timezone,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Output {
    pub elements: Vec<OutputElement>,
    pub ranges: Vec<OutputRange>,
}
