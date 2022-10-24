use crate::datetime::output::*;
use crate::datetime::types::*;
use crate::datetime::DateTimeData;
use crate::pattern::PatternElement;
use crate::resolver::Resolver;

pub struct DateResolver<'data> {
    pub data: &'data DateTimeData,
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

    fn get(&self, key: usize, output: &mut Self::Output) {}
}

pub struct TimePatternIterator<'output> {
    pub timezone_pattern: TimezonePatternIterator<'output>,
    pub name: &'output str,
    pub idx: usize,
}

impl<'output> Iterator for TimePatternIterator<'output> {
    type Item = TimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        None
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

    fn get(&self, key: usize, output: &mut Self::Output) {}
}

pub struct TimezonePatternIterator<'output> {
    pub name: &'output str,
    pub idx: usize,
}

impl<'output> Iterator for TimezonePatternIterator<'output> {
    type Item = TimezoneOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct DateTimeResolver<'data> {
    pub data: &'data DateTimeData,
}

impl<'data> DateTimeResolver<'data> {
    pub fn new(data: &'data DateTimeData) -> Self {
        Self { data }
    }
}

impl<'output> Resolver<'output> for DateTimeResolver<'output> {
    type OutputElement = DateTimeOutputElement<'output>;
    type Output = DateTimeOutput<'output>;

    fn get(&self, key: usize, output: &mut Self::Output) {}
}

pub struct DateTimePatternIterator<'output> {
    pub date_pattern: Option<&'output Vec<PatternElement<DatePatternElement>>>,
    pub time_pattern: Option<&'output Vec<PatternElement<TimePatternElement>>>,
    pub idx: usize,
}

impl<'output> Iterator for DateTimePatternIterator<'output> {
    type Item = DateTimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
