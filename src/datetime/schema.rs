use crate::datetime::types::*;
use crate::PatternElement;

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub format: Vec<PatternElement<DatePatternElement>>,
    pub date_combination: Vec<PatternElement<DateTimePatternElement>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub format: Vec<PatternElement<TimePatternElement>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Timezone {
    pub format: Vec<PatternElement<TimezonePatternElement>>,
    pub hour_format: Vec<PatternElement<TimezonePatternElement>>,
    pub fallback_format: Vec<PatternElement<TimezonePatternElement>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataSchema {
    pub time: Time,
    pub date: Date,
    pub timezone: Timezone,
}
