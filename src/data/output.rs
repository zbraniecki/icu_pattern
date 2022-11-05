use crate::data::types::*;
use crate::pattern::Never;
use std::ops::Range;

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
    pub ranges: Vec<OutputRange>,
}
