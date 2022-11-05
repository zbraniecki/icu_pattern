use icu_pattern::datetime::{
    output::{
        DateOutput, DateOutputElement, DateTimeOutput, DateTimeOutputElement, TimeOutput,
        TimeOutputElement, TimezoneOutput, TimezoneOutputElement,
    },
    types::{
        DatePatternElement, DateRole, TimePatternElement, TimezonePatternElement,
        TimezonePatternPlaceholderScheme, TimezonePatternVariant,
    },
    DateTimeData,
};
use icu_pattern::{
    output::Output,
    pattern::Pattern,
    ranges::{Range, RangeList},
};
use smallvec::SmallVec;
use std::borrow::Cow;

#[test]
fn core_date_test() {
    let data = DateTimeData::default();
    let pattern = data.get_date_pattern();

    let mut ranges = RangeList::new();

    let elements = pattern.resolve(&data, None, Some(&mut ranges));

    let output = DateOutput {
        elements: elements.collect(),
    };

    let expected = DateOutput {
        elements: vec![
            DateOutputElement::Date(Cow::Borrowed(&DatePatternElement::Month)),
            DateOutputElement::Literal(Cow::Borrowed(" ")),
            DateOutputElement::Date(Cow::Borrowed(&DatePatternElement::Day)),
            DateOutputElement::Literal(Cow::Borrowed(", ")),
            DateOutputElement::Date(Cow::Borrowed(&DatePatternElement::Year)),
        ],
    };
    let expected_ranges = RangeList {
        elements: SmallVec::from_vec(vec![
            Range {
                role: DateRole::Month,
                range: 0..1,
            },
            Range {
                role: DateRole::Day,
                range: 2..3,
            },
            Range {
                role: DateRole::Year,
                range: 4..5,
            },
        ]),
    };
    assert_eq!(output, expected);
    assert_eq!(ranges, expected_ranges);
}

// #[test]
// fn core_time_test() {
//     let data = DateTimeData::default();
//     let pattern = data.get_time_pattern();
//
//     let elements = pattern.resolve(&data, None);
//
//     let output = TimeOutput {
//         elements: elements.collect(),
//     };
//
//     let expected = TimeOutput {
//         elements: vec![
//             TimeOutputElement::Time(Cow::Borrowed(&TimePatternElement::Hour)),
//             TimeOutputElement::Literal(Cow::Borrowed(":")),
//             TimeOutputElement::Time(Cow::Borrowed(&TimePatternElement::Minute)),
//             TimeOutputElement::Literal(Cow::Borrowed(" ")),
//             TimeOutputElement::Timezone(Cow::Borrowed(&TimezonePatternElement::Name)),
//             TimeOutputElement::Literal(Cow::Borrowed(" Time")),
//         ],
//         // ranges: vec![
//         //     OutputRange {
//         //         role: OutputRole::Date,
//         //         range: 0..5,
//         //     },
//         // ],
//     };
//     assert_eq!(output, expected);
// }
//
// #[test]
// fn core_timezone_test() {
//     let data = DateTimeData::default();
//     let variant = TimezonePatternVariant::Format;
//     let (pattern, scheme) = data.get_timezone_pattern(variant);
//
//     let elements = pattern.resolve(&data, scheme);
//
//     let output = TimezoneOutput {
//         elements: elements.collect(),
//     };
//
//     let expected = TimezoneOutput {
//         elements: vec![
//             TimezoneOutputElement::Timezone(Cow::Borrowed(&TimezonePatternElement::Name)),
//             TimezoneOutputElement::Literal(Cow::Borrowed(" Time")),
//         ],
//         // ranges: vec![
//         //     OutputRange {
//         //         role: OutputRole::Date,
//         //         range: 0..5,
//         //     },
//         // ],
//     };
//     assert_eq!(output, expected);
// }
//
// #[test]
// fn core_timezone_fallback_test() {
//     let data = DateTimeData::default();
//     let variant = TimezonePatternVariant::FallbackFormat;
//     let (pattern, scheme) = data.get_timezone_pattern(variant);
//
//     let mut output = TimezoneOutput::default();
//
//     let elements = pattern.resolve(&data, scheme);
//
//     let output = TimezoneOutput {
//         elements: elements.collect(),
//     };
//
//     let expected = TimezoneOutput {
//         elements: vec![
//             TimezoneOutputElement::Timezone(Cow::Borrowed(&TimezonePatternElement::Name)),
//             TimezoneOutputElement::Literal(Cow::Borrowed(" (")),
//             TimezoneOutputElement::Literal(Cow::Borrowed("+")),
//             TimezoneOutputElement::Timezone(Cow::Borrowed(&TimezonePatternElement::Time(
//                 TimePatternElement::Hour,
//             ))),
//             TimezoneOutputElement::Literal(Cow::Borrowed(":")),
//             TimezoneOutputElement::Timezone(Cow::Borrowed(&TimezonePatternElement::Time(
//                 TimePatternElement::Minute,
//             ))),
//             TimezoneOutputElement::Literal(Cow::Borrowed(")")),
//         ],
//         // ranges: vec![
//         //     OutputRange {
//         //         role: OutputRole::Date,
//         //         range: 0..5,
//         //     },
//         // ],
//     };
//     assert_eq!(output, expected);
// }
//
// #[test]
// fn core_datetime_test() {
//     let data = DateTimeData::default();
//     let pattern = data.get_datetime_pattern();
//
//     let elements = pattern.resolve(&data, None);
//
//     let output = DateTimeOutput {
//         elements: elements.collect(),
//     };
//
//     let expected = DateTimeOutput {
//         elements: vec![
//             DateTimeOutputElement::Date(Cow::Borrowed(&DatePatternElement::Month)),
//             DateTimeOutputElement::Literal(" ".into()),
//             DateTimeOutputElement::Date(Cow::Borrowed(&DatePatternElement::Day)),
//             DateTimeOutputElement::Literal(", ".into()),
//             DateTimeOutputElement::Date(Cow::Borrowed(&DatePatternElement::Year)),
//             DateTimeOutputElement::Literal(" at ".into()),
//             DateTimeOutputElement::Time(Cow::Borrowed(&TimePatternElement::Hour)),
//             DateTimeOutputElement::Literal(":".into()),
//             DateTimeOutputElement::Time(Cow::Borrowed(&TimePatternElement::Minute)),
//             DateTimeOutputElement::Literal(" ".into()),
//             DateTimeOutputElement::Timezone(Cow::Owned(TimezonePatternElement::Name)),
//             DateTimeOutputElement::Literal(" Time".into()),
//         ],
//         // ranges: vec![
//         //     OutputRange {
//         //         role: OutputRole::Date,
//         //         range: 0..5,
//         //     },
//         //     OutputRange {
//         //         role: OutputRole::Timezone,
//         //         range: 10..12,
//         //     },
//         //     OutputRange {
//         //         role: OutputRole::Time,
//         //         range: 6..12,
//         //     },
//         //     OutputRange {
//         //         role: OutputRole::DateTime,
//         //         range: 0..12,
//         //     },
//         // ],
//     };
//     assert_eq!(output, expected);
// }
