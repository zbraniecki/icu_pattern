use crate::datetime::types::{DatePatternElement, TimePatternElement, TimezonePatternElement};
use crate::output::{Output, OutputElement};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DateOutputElement<'output> {
    Literal(&'output str),
    Date(&'output DatePatternElement),
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
    Literal(&'output str),
    Time(&'output TimePatternElement),
    Timezone(&'output TimezonePatternElement),
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
    Literal(&'output str),
    Timezone(&'output TimezonePatternElement),
    Time(&'output TimePatternElement),
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
    Literal(&'output str),
    Date(&'output DatePatternElement),
    Time(&'output TimePatternElement),
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
