use icu_pattern::{Pattern, PatternElement};
use std::ops::Range;

pub struct SourceTimezone {
    pub format: String,
    pub hour_format: String,
    pub fallback_format: String,
}

pub struct SourceDate {
    pub format: String,
    pub date_combination: String,
}

pub struct SourceTime {
    pub format: String,
}

pub struct SourceData {
    pub time: SourceTime,
    pub date: SourceDate,
    pub timezone: SourceTimezone,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DatePatternElement {
    Year,
    Month,
    Day,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimePatternElement {
    Hour,
    Minute,
    Second,
    Timezone,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimezonePatternElement {
    Name,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub format: Pattern<DatePatternElement>,
    pub date_combination: Pattern,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub format: Pattern<TimePatternElement>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Timezone {
    pub format: Pattern,
    pub hour_format: Pattern<TimePatternElement>,
    pub fallback_format: Pattern,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    pub time: Time,
    pub date: Date,
    pub timezone: Timezone,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputElement {
    Literal(String),
    Date(DatePatternElement),
    Time(TimePatternElement),
    Timezone(TimezonePatternElement),
}

impl From<&TimePatternElement> for OutputElement {
    fn from(input: &TimePatternElement) -> Self {
        Self::Time(input.clone())
    }
}

impl From<&DatePatternElement> for OutputElement {
    fn from(input: &DatePatternElement) -> Self {
        Self::Date(input.clone())
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

#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    pub elements: Vec<OutputElement>,
    pub ranges: Vec<OutputRange>,
}

pub fn get_data() -> Data {
    let _source_data = SourceData {
        time: SourceTime {
            format: "h:mm zzzz".to_string(),
        },
        date: SourceDate {
            format: "MMMM d, y".to_string(),
            date_combination: "(1} 'at' {0}".to_string(),
        },
        timezone: SourceTimezone {
            format: "{0} Time".to_string(),
            hour_format: "+HH:mm".to_string(),
            fallback_format: "{1} ({0})".to_string(),
        },
    };

    Data {
        time: Time {
            format: Pattern {
                elements: vec![
                    PatternElement::Element(TimePatternElement::Hour),
                    PatternElement::Literal(":".to_string()),
                    PatternElement::Element(TimePatternElement::Minute),
                    PatternElement::Literal(" ".to_string()),
                    PatternElement::Element(TimePatternElement::Timezone),
                ],
            },
        },
        date: Date {
            format: Pattern {
                elements: vec![
                    PatternElement::Element(DatePatternElement::Month),
                    PatternElement::Literal(" ".to_string()),
                    PatternElement::Element(DatePatternElement::Day),
                    PatternElement::Literal(", ".to_string()),
                    PatternElement::Element(DatePatternElement::Year),
                ],
            },
            date_combination: Pattern {
                elements: vec![
                    PatternElement::Placeholder(1),
                    PatternElement::Literal(" at ".to_string()),
                    PatternElement::Placeholder(0),
                ],
            },
        },
        timezone: Timezone {
            format: Pattern {
                elements: vec![
                    PatternElement::Placeholder(0),
                    PatternElement::Literal(" Time".to_string()),
                ],
            },
            hour_format: Pattern {
                elements: vec![
                    PatternElement::Literal("+".to_string()),
                    PatternElement::Element(TimePatternElement::Hour),
                    PatternElement::Literal(":".to_string()),
                    PatternElement::Element(TimePatternElement::Minute),
                ],
            },
            fallback_format: Pattern {
                elements: vec![
                    PatternElement::Placeholder(1),
                    PatternElement::Literal(" (".to_string()),
                    PatternElement::Placeholder(0),
                    PatternElement::Literal(")".to_string()),
                ],
            },
        },
    }
}
