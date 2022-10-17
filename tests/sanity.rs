mod data;

use data::*;
use icu_pattern::{Pattern, PatternElement};

pub struct DateTimeResolver {}

impl DateTimeResolver {
    pub fn populate(data: &Data, key: usize, output: &mut Output) {
        match key {
            0 => {
                let start = output.elements.len();
                for element in &data.time.format.elements {
                    match element {
                        PatternElement::Element(e) => {
                            TimeResolver::populate(data, e, output);
                        }
                        PatternElement::Literal(l) => {
                            output.elements.push(OutputElement::Literal(l.clone()));
                        }
                        PatternElement::Placeholder(p) => todo!(),
                    }
                }
                let end = output.elements.len();
                output.ranges.push(OutputRange {
                    role: OutputRole::Time,
                    range: start..end,
                });
            }
            1 => {
                let start = output.elements.len();
                for element in &data.date.format.elements {
                    match element {
                        PatternElement::Element(e) => {
                            output.elements.push(e.into());
                        }
                        PatternElement::Literal(l) => {
                            output.elements.push(OutputElement::Literal(l.clone()));
                        }
                        PatternElement::Placeholder(p) => todo!(),
                    }
                }
                let end = output.elements.len();
                output.ranges.push(OutputRange {
                    role: OutputRole::Date,
                    range: start..end,
                });
            }
            _ => todo!(),
        }
    }
}

pub struct TimeResolver {}

impl TimeResolver {
    pub fn populate(data: &Data, key: &TimePatternElement, output: &mut Output) {
        match key {
            TimePatternElement::Timezone => {
                let start = output.elements.len();
                for element in &data.timezone.format.elements {
                    match element {
                        PatternElement::Element(_) => todo!(),
                        PatternElement::Literal(l) => {
                            output.elements.push(OutputElement::Literal(l.clone()));
                        }
                        PatternElement::Placeholder(p) => match p {
                            0 => {
                                output
                                    .elements
                                    .push(OutputElement::Timezone(TimezonePatternElement::Name));
                            }
                            _ => todo!(),
                        },
                    }
                }
                let end = output.elements.len();
                output.ranges.push(OutputRange {
                    role: OutputRole::Timezone,
                    range: start..end,
                });
            }
            e => {
                output.elements.push(e.into());
            }
        }
    }
}

fn resolve(data: &Data, input: &Pattern) -> Output {
    let mut result = Output {
        elements: vec![],
        ranges: vec![],
    };
    let start = 0;
    for element in &input.elements {
        match element {
            PatternElement::Element(e) => todo!(),
            PatternElement::Literal(l) => {
                result.elements.push(OutputElement::Literal(l.clone()));
            }
            PatternElement::Placeholder(p) => {
                DateTimeResolver::populate(data, *p, &mut result);
            }
        }
    }
    let end = result.elements.len();
    result.ranges.push(OutputRange {
        role: OutputRole::DateTime,
        range: start..end,
    });
    result
}

#[test]
fn it_works() {
    let ds = get_data();

    let combination = &ds.date.date_combination;

    let output = resolve(&ds, combination);

    let expected = Output {
        elements: vec![
            OutputElement::Date(DatePatternElement::Month),
            OutputElement::Literal(" ".to_string()),
            OutputElement::Date(DatePatternElement::Day),
            OutputElement::Literal(", ".to_string()),
            OutputElement::Date(DatePatternElement::Year),
            OutputElement::Literal(" at ".to_string()),
            OutputElement::Time(TimePatternElement::Hour),
            OutputElement::Literal(":".to_string()),
            OutputElement::Time(TimePatternElement::Minute),
            OutputElement::Literal(" ".to_string()),
            OutputElement::Timezone(TimezonePatternElement::Name),
            OutputElement::Literal(" Time".to_string()),
        ],
        ranges: vec![
            OutputRange {
                role: OutputRole::Date,
                range: 0..5,
            },
            OutputRange {
                role: OutputRole::Timezone,
                range: 10..12,
            },
            OutputRange {
                role: OutputRole::Time,
                range: 6..12,
            },
            OutputRange {
                role: OutputRole::DateTime,
                range: 0..12,
            },
        ],
    };
    assert_eq!(output, expected);
}
