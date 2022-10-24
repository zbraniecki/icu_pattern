use crate::data::{output::*, structs::*, types::*};
use crate::{Pattern, PatternElement};

pub struct DateTimeResolver {}

impl DateTimeResolver {
    pub fn resolve(data: &Data, pattern: &Pattern, output: &mut Output) {
        let start = output.elements.len();
        for element in &pattern.elements {
            match element {
                PatternElement::Element(e) => todo!(),
                PatternElement::Literal(l) => {
                    output.elements.push(OutputElement::Literal(l.clone()));
                }
                PatternElement::Placeholder(p) => {
                    Self::expand(data, *p, output);
                }
            }
        }
        let end = output.elements.len();
        output.ranges.push(OutputRange {
            role: OutputRole::DateTime,
            range: start..end,
        });
    }

    pub fn expand(data: &Data, key: usize, output: &mut Output) {
        match key {
            0 => {
                let pattern = &data.time.format;
                TimeResolver::resolve(data, pattern, output);
            }
            1 => {
                let pattern = &data.date.format;
                DateResolver::resolve(data, pattern, output);
            }
            _ => todo!(),
        }
    }
}

pub struct DateResolver {}

impl DateResolver {
    pub fn resolve(data: &Data, pattern: &Pattern<DatePatternElement>, output: &mut Output) {
        let start = output.elements.len();
        for element in &pattern.elements {
            match element {
                PatternElement::Element(e) => {
                    Self::populate(data, e, output);
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

    pub fn populate(_: &Data, key: &DatePatternElement, output: &mut Output) {
        output.elements.push(key.clone().into());
    }
}

pub struct TimeResolver {}

impl TimeResolver {
    pub fn resolve(data: &Data, pattern: &Pattern<TimePatternElement>, output: &mut Output) {
        let start = output.elements.len();
        for element in &pattern.elements {
            match element {
                PatternElement::Element(e) => {
                    Self::populate(data, e, output);
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

    pub fn populate(data: &Data, key: &TimePatternElement, output: &mut Output) {
        match key {
            TimePatternElement::Timezone => {
                let pattern = &data.timezone.format;
                TimezoneResolver::resolve(data, pattern, output);
            }
            e => {
                output.elements.push(e.clone().into());
            }
        }
    }
}

pub struct TimezoneResolver {}

impl TimezoneResolver {
    pub fn resolve<E>(data: &Data, pattern: &Pattern<E>, output: &mut Output)
    where
        E: Into<OutputElement> + Clone,
    {
        let start = output.elements.len();
        for element in &pattern.elements {
            match element {
                PatternElement::Element(e) => {
                    Self::populate(data, (*e).clone(), output);
                }
                PatternElement::Literal(l) => {
                    output.elements.push(OutputElement::Literal(l.clone()));
                }
                PatternElement::Placeholder(p) => {
                    Self::expand(data, *p, output);
                }
            }
        }
        let end = output.elements.len();
        output.ranges.push(OutputRange {
            role: OutputRole::Timezone,
            range: start..end,
        });
    }

    pub fn expand(data: &Data, key: usize, output: &mut Output) {
        match key {
            0 => {
                output
                    .elements
                    .push(OutputElement::Timezone(TimezonePatternElement::Name));
            }
            1 => {
                let pattern = &data.date.format;
                DateResolver::resolve(data, pattern, output);
            }
            _ => todo!(),
        }
    }

    pub fn populate<E>(_: &Data, key: E, output: &mut Output)
    where
        E: Into<OutputElement>,
    {
        output.elements.push(key.into());
    }
}
