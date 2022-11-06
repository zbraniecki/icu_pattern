use crate::datetime::types::*;
use crate::PatternElement;

#[derive(Debug, PartialEq, Eq)]
pub struct Date<'input> {
    pub format: Vec<PatternElement<'input, DatePatternElement>>,
    pub date_combination: Vec<PatternElement<'input, DateTimePatternElement>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time<'input> {
    pub format: Vec<PatternElement<'input, TimePatternElement>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Timezone<'input> {
    pub format: Vec<PatternElement<'input, TimezonePatternElement>>,
    pub hour_format: Vec<PatternElement<'input, TimezonePatternElement>>,
    pub fallback_format: Vec<PatternElement<'input, TimezonePatternElement>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataSchema<'input> {
    pub time: Time<'input>,
    pub date: Date<'input>,
    pub timezone: Timezone<'input>,
}
