mod data;

use data::*;
use icu_pattern::{Pattern, PatternElement};

pub struct DateTimeResolver {}

impl DateTimeResolver {
    pub fn populate(data: &Data, key: usize, mut sink: &mut Vec<OutputElement>) {
        match key {
            0 => {
                for element in &data.time.format.elements {
                    match element {
                        PatternElement::Element(e) => {
                            sink.push(e.into());
                        }
                        PatternElement::Literal(l) => todo!(),
                        PatternElement::Placeholder(p) => todo!(),
                    }
                }
            }
            1 => {
                for element in &data.date.format.elements {
                    match element {
                        PatternElement::Element(e) => {
                            sink.push(e.into());
                        }
                        PatternElement::Literal(l) => todo!(),
                        PatternElement::Placeholder(p) => todo!(),
                    }
                }
            }
            _ => {}
        }
    }
}

fn resolve(data: &Data, input: &Pattern, resolver: DateTimeResolver) -> Output {
    let mut elements = vec![];
    for element in &input.elements {
        match element {
            PatternElement::Element(e) => todo!(),
            PatternElement::Literal(l) => {
                elements.push(OutputElement::Literal(l.clone()));
            }
            PatternElement::Placeholder(p) => {
                DateTimeResolver::populate(data, *p, &mut elements);
            }
        }
    }
    Output {
        elements,
        ranges: vec![],
    }
}

#[test]
fn it_works() {
    let ds = get_data();

    let combination = &ds.date.date_combination;
    // let date = &ds.date.format;
    // let time = &ds.time.format;

    let output = resolve(&ds, combination, DateTimeResolver {});

    let expected = Output {
        elements: vec![
            OutputElement::Date(DatePatternElement::Year),
            OutputElement::Literal(" at ".to_string()),
            OutputElement::Time(TimePatternElement::Hour),
        ],
        ranges: vec![],
    };
    assert_eq!(output, expected);
}
