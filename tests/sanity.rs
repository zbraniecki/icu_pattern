// use icu_pattern::data::{get_data, output::*, resolvers::*, types::*};
use icu_pattern::datetime::{
    output::{
        DateOutput, DateOutputElement, DateTimeOutput, DateTimeOutputElement, TimeOutput,
        TimeOutputElement, TimezoneOutput, TimezoneOutputElement,
    },
    resolver::{DateResolver, DateTimeResolver, TimeResolver, TimezoneResolver},
    types::{DatePatternElement, TimePatternElement, TimezonePatternElement},
    DateTimeData,
};
use icu_pattern::{output::Output, pattern::Pattern};

#[test]
fn core_date_test() {
    let data = DateTimeData::default();
    let date_pattern = data.get_date_pattern();
    let resolver = DateResolver::new(&data);

    // let mut output = DateOutput::default();

    let elements = date_pattern.resolve(&resolver);
    let output = DateOutput {
        elements: elements.collect()
    };

    let expected = DateOutput {
        elements: vec![
            DateOutputElement::Date(&DatePatternElement::Month),
            DateOutputElement::Literal(" "),
            DateOutputElement::Date(&DatePatternElement::Day),
            DateOutputElement::Literal(", "),
            DateOutputElement::Date(&DatePatternElement::Year),
        ],
        // ranges: vec![
        //     OutputRange {
        //         role: OutputRole::Date,
        //         range: 0..5,
        //     },
        // ],
    };
    assert_eq!(output, expected);
}

// #[test]
// fn core_time_test() {
//     let data = DateTimeData::default();
//     let time_pattern = data.get_time_pattern();
//     let resolver = TimeResolver::new(&data);
//
//     let mut output = TimeOutput::default();
//
//     time_pattern.resolve(&mut output, &resolver);
//
//     let expected = TimeOutput {
//         elements: vec![
//             TimeOutputElement::Time(&TimePatternElement::Hour),
//             TimeOutputElement::Literal(":"),
//             TimeOutputElement::Time(&TimePatternElement::Minute),
//             TimeOutputElement::Literal(" "),
//             TimeOutputElement::Literal("America/Los_Angeles"),
//             TimeOutputElement::Literal(" Time"),
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
#[test]
fn core_timezone_test() {
    let data = DateTimeData::default();
    let timezone_pattern = data.get_timezone_pattern();
    let resolver = TimezoneResolver::new(&data);

    let mut output = TimezoneOutput::default();

    let elements = timezone_pattern.resolve(&resolver);

    let output = TimezoneOutput {
        elements: elements.collect()
    };

    let expected = TimezoneOutput {
        elements: vec![
            TimezoneOutputElement::Literal("America/Los_Angeles"),
            TimezoneOutputElement::Literal(" Time"),
        ],
        // elements: vec![
        //     TimezoneOutputElement::Literal("+"),
        //     TimezoneOutputElement::Timezone(&TimezonePatternElement::Time(
        //         TimePatternElement::Hour,
        //     )),
        //     TimezoneOutputElement::Literal(":"),
        //     TimezoneOutputElement::Timezone(&TimezonePatternElement::Time(
        //         TimePatternElement::Minute,
        //     )),
        // ],
        // ranges: vec![
        //     OutputRange {
        //         role: OutputRole::Date,
        //         range: 0..5,
        //     },
        // ],
    };
    assert_eq!(output, expected);
}
//
// #[test]
// fn core_datetime_test() {
//     let data = DateTimeData::default();
//     let dt_pattern = data.get_datetime_pattern();
//     let resolver = DateTimeResolver::new(&data);
//
//     let mut output = DateTimeOutput::default();
//     dt_pattern.resolve(&mut output, &resolver);
//
//     let expected = DateTimeOutput {
//         elements: vec![
//             DateTimeOutputElement::Date(&DatePatternElement::Month),
//             DateTimeOutputElement::Literal(" "),
//             DateTimeOutputElement::Date(&DatePatternElement::Day),
//             DateTimeOutputElement::Literal(", "),
//             DateTimeOutputElement::Date(&DatePatternElement::Year),
//             DateTimeOutputElement::Literal(" at "),
//             DateTimeOutputElement::Time(&TimePatternElement::Hour),
//             DateTimeOutputElement::Literal(":"),
//             DateTimeOutputElement::Time(&TimePatternElement::Minute),
//             DateTimeOutputElement::Literal(" "),
//             DateTimeOutputElement::Time(&TimePatternElement::Timezone),
//             // OutputElement::Timezone(TimezonePatternElement::Name),
//             // OutputElement::Literal(" Time".to_string()),
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
