use std::borrow::Cow;

use crate::datetime::types::{DatePatternElement, TimePatternElement, TimezonePatternElement};
use crate::output::{Output, OutputElement};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DateOutputElement<'output> {
    Literal(Cow<'output, str>),
    Date(Cow<'output, DatePatternElement>),
}

impl OutputElement for DateOutputElement<'_> {}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct DateOutput<'output> {
    pub elements: Vec<DateOutputElement<'output>>,
}

impl<'output> Output<DateOutputElement<'output>> for DateOutput<'output> {
    fn push_element(&mut self, element: DateOutputElement<'output>) {
        self.elements.push(element);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TimeOutputElement<'output> {
    Literal(Cow<'output, str>),
    Time(Cow<'output, TimePatternElement>),
    Timezone(Cow<'output, TimezonePatternElement>),
}

impl OutputElement for TimeOutputElement<'_> {}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct TimeOutput<'output> {
    pub elements: Vec<TimeOutputElement<'output>>,
}

impl<'output> Output<TimeOutputElement<'output>> for TimeOutput<'output> {
    fn push_element(&mut self, element: TimeOutputElement<'output>) {
        self.elements.push(element);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TimezoneOutputElement<'output> {
    Literal(Cow<'output, str>),
    Timezone(Cow<'output, TimezonePatternElement>),
    Time(Cow<'output, TimePatternElement>),
}

impl OutputElement for TimezoneOutputElement<'_> {}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct TimezoneOutput<'output> {
    pub elements: Vec<TimezoneOutputElement<'output>>,
}

impl<'output> Output<TimezoneOutputElement<'output>> for TimezoneOutput<'output> {
    fn push_element(&mut self, element: TimezoneOutputElement<'output>) {
        self.elements.push(element);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DateTimeOutputElement<'output> {
    Literal(Cow<'output, str>),
    Date(Cow<'output, DatePatternElement>),
    Time(Cow<'output, TimePatternElement>),
    Timezone(Cow<'output, TimezonePatternElement>),
}

impl OutputElement for DateTimeOutputElement<'_> {}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct DateTimeOutput<'output> {
    pub elements: Vec<DateTimeOutputElement<'output>>,
}

impl<'output> Output<DateTimeOutputElement<'output>> for DateTimeOutput<'output> {
    fn push_element(&mut self, element: DateTimeOutputElement<'output>) {
        self.elements.push(element);
    }
}
