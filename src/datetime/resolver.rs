use crate::datetime::output::*;
use crate::datetime::types::*;
use crate::datetime::DateTimeData;
use crate::pattern::PatternElement;
use crate::resolver::Resolver;
use crate::output::Output;
use crate::pattern::Pattern;

pub struct DateResolver<'data> {
    data: &'data DateTimeData,
}

impl<'data> DateResolver<'data> {
    pub fn new(data: &'data DateTimeData) -> Self {
        Self { data }
    }
}

impl<'output> Resolver<'output> for DateResolver<'output> {
    type OutputElement = DateOutputElement<'output>;
    type Output = DateOutput<'output>;

    fn get(&self, key: usize, output: &mut Self::Output) {
        todo!()
    }
}

pub struct DatePatternIterator<'output> {
    pub date_pattern: &'output Vec<PatternElement<DatePatternElement>>,
    pub idx: usize,
}

impl<'output> Iterator for DatePatternIterator<'output> {
    type Item = DateOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct TimeResolver<'data> {
    pub data: &'data DateTimeData,
}

impl<'data> TimeResolver<'data> {
    pub fn new(data: &'data DateTimeData) -> Self {
        Self { data }
    }
}

impl<'output> Resolver<'output> for TimeResolver<'output> {
    type OutputElement = TimeOutputElement<'output>;
    type Output = TimeOutput<'output>;

    fn get(&self, key: usize, output: &mut Self::Output) {
        // let timezone_pattern = self.data.get_timezone_pattern();
        // let resolver = TimezoneResolver::new(self.data);
        // timezone_pattern.resolve(&resolver);
        // let timezone_pattern = TimezonePatternIterator {
        //     name: self.data.get_timezone_name(),
        //     idx: 0,
        // };
        // TimePatternIterator {
        //     timezone_pattern,
        //     name: self.data.get_timezone_name(),
        //     idx: 0,
        // }
    }
}

pub struct TimePatternIterator<'output> {
    pub timezone_pattern: TimezonePatternIterator<'output>,
    pub name: &'output str,
    pub idx: usize,
}

impl<'output> Iterator for TimePatternIterator<'output> {
    type Item = TimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.timezone_pattern.next() {
            match item {
                TimezoneOutputElement::Literal(l) => {
                    Some(TimeOutputElement::Literal(l))
                },
                TimezoneOutputElement::Timezone(t) => {
                    Some(TimeOutputElement::Timezone(t))
                },
                TimezoneOutputElement::Time(t) => {
                    Some(TimeOutputElement::Time(t))
                },
            }

        } else {
            None
        }
    }
}

pub struct TimezoneResolver<'data> {
    pub data: &'data DateTimeData,
}

impl<'data> TimezoneResolver<'data> {
    pub fn new(data: &'data DateTimeData) -> Self {
        Self { data }
    }
}

impl<'output> Resolver<'output> for TimezoneResolver<'output> {
    type OutputElement = TimezoneOutputElement<'output>;
    type Output = TimezoneOutput<'output>;

    fn get(&self, key: usize, output: &mut Self::Output) {
        output.push_element(TimezoneOutputElement::Literal(self.data.get_timezone_name()))
    }
}

pub struct TimezonePatternIterator<'output> {
    pub name: &'output str,
    pub idx: usize,
}

impl<'output> Iterator for TimezonePatternIterator<'output> {
    type Item = TimezoneOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 0 {
            self.idx += 1;
            Some(TimezoneOutputElement::Literal(self.name))
        } else {
            None
        }
    }
}

pub struct DateTimeResolver<'data> {
    data: &'data DateTimeData,
}

impl<'data> DateTimeResolver<'data> {
    pub fn new(data: &'data DateTimeData) -> Self {
        Self { data }
    }
}

impl<'output> Resolver<'output> for DateTimeResolver<'output> {
    type OutputElement = DateTimeOutputElement<'output>;
    type Output = DateTimeOutput<'output>;

    fn get(&self, key: usize, output: &mut Self::Output) {
        // match key {
        //     0 => DateTimePatternIterator {
        //         date_pattern: None,
        //         time_pattern: Some(self.data.get_time_pattern()),
        //         idx: 0,
        //     },
        //     1 => DateTimePatternIterator {
        //         date_pattern: Some(self.data.get_date_pattern()),
        //         time_pattern: None,
        //         idx: 0,
        //     },
        //     _ => unreachable!(),
        // }
    }
}

pub struct DateTimePatternIterator<'output> {
    pub date_pattern: Option<&'output Vec<PatternElement<DatePatternElement>>>,
    pub time_pattern: Option<&'output Vec<PatternElement<TimePatternElement>>>,
    pub idx: usize,
}

impl<'output> Iterator for DateTimePatternIterator<'output> {
    type Item = DateTimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pattern) = &self.date_pattern {
            if let Some(item) = pattern.get(self.idx) {
                self.idx += 1;
                match item {
                    PatternElement::Element(e) => Some(DateTimeOutputElement::Date(e)),
                    PatternElement::Literal(l) => Some(DateTimeOutputElement::Literal(l)),
                    PatternElement::Placeholder(_) => todo!(),
                }
            } else {
                None
            }
        } else if let Some(item) = self.time_pattern.unwrap().get(self.idx) {
            self.idx += 1;
            match item {
                PatternElement::Element(e) => match e {
                    TimePatternElement::Timezone => {
                        todo!()
                    }
                    e => Some(DateTimeOutputElement::Time(e)),
                },
                PatternElement::Literal(l) => Some(DateTimeOutputElement::Literal(l)),
                PatternElement::Placeholder(_) => todo!(),
            }
        } else {
            None
        }
    }
}
