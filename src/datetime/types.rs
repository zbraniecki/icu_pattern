use crate::datetime::output::{
    DateOutputElement, DateTimeOutputElement, TimeOutputElement, TimezoneOutputElement,
};
use crate::datetime::DateTimeData;
use crate::{
    pattern::{Pattern, PatternElement},
    ranges::{Range, RangeList},
};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DatePatternElement {
    Year,
    Month,
    Day,
}

impl TryFrom<char> for DatePatternElement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'y' => Ok(Self::Year),
            'M' => Ok(Self::Month),
            'd' => Ok(Self::Day),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DateRole {
    Day,
    Month,
    Year,
}

impl<'output> Pattern<'output, DatePatternElement> for Vec<PatternElement<DatePatternElement>> {
    type OutputElement = DateOutputElement<'output>;
    type Provider = DateTimeData;
    type Iter = DatePatternIterator<'output>;
    type Scheme = ();
    type OutputRole = DateRole;

    fn resolve(
        &'output self,
        _provider: &Self::Provider,
        _scheme: Option<Self::Scheme>,
        ranges: Option<&'output mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        DatePatternIterator {
            elements: self.iter().enumerate(),
            ranges,
        }
    }
}

pub struct DatePatternIterator<'output> {
    pub elements:
        std::iter::Enumerate<std::slice::Iter<'output, PatternElement<DatePatternElement>>>,
    pub ranges: Option<&'output mut RangeList<DateRole>>,
}

impl<'output> Iterator for DatePatternIterator<'output> {
    type Item = DateOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.next().map(|(idx, e)| match e {
            PatternElement::Element(e) => {
                if let Some(ranges) = &mut self.ranges {
                    let role = match e {
                        DatePatternElement::Year => DateRole::Year,
                        DatePatternElement::Month => DateRole::Month,
                        DatePatternElement::Day => DateRole::Day,
                    };
                    ranges.push(Range {
                        role,
                        range: idx..idx + 1,
                    });
                }
                DateOutputElement::Date(Cow::Borrowed(e))
            }
            PatternElement::Literal(l) => DateOutputElement::Literal(l.into()),
            PatternElement::Placeholder(_) => todo!(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimePatternElement {
    Hour,
    Minute,
    Second,
    Timezone,
}

impl TryFrom<char> for TimePatternElement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'H' => Ok(Self::Hour),
            'h' => Ok(Self::Hour),
            'm' => Ok(Self::Minute),
            's' => Ok(Self::Second),
            'z' => Ok(Self::Timezone),
            _ => Err(()),
        }
    }
}

pub enum TimeRole {
    Hour,
    Minute,
    Timezone,
}

impl<'output> Pattern<'output, TimePatternElement> for Vec<PatternElement<TimePatternElement>> {
    type OutputElement = TimeOutputElement<'output>;
    type Provider = DateTimeData;
    type Iter = TimePatternIterator<'output>;
    type Scheme = ();
    type OutputRole = TimeRole;

    fn resolve(
        &'output self,
        provider: &'output Self::Provider,
        _scheme: Option<Self::Scheme>,
        ranges: Option<&mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        TimePatternIterator {
            elements: self.iter(),
            data: provider,
            timezone: None,
        }
    }
}

pub struct TimePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<TimePatternElement>>,
    pub data: &'output DateTimeData,
    pub timezone: Option<TimezonePatternIterator<'output>>,
}

impl<'output> Iterator for TimePatternIterator<'output> {
    type Item = TimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut tz) = self.timezone {
            if let Some(item) = tz.next() {
                match item {
                    TimezoneOutputElement::Literal(l) => {
                        return Some(TimeOutputElement::Literal(l));
                    }
                    TimezoneOutputElement::Timezone(t) => {
                        return Some(TimeOutputElement::Timezone(t));
                    }
                    TimezoneOutputElement::Time(t) => {
                        return Some(TimeOutputElement::Time(t));
                    }
                }
            } else {
                self.timezone = None;
            }
        }
        self.elements.next().map(|e| match e {
            PatternElement::Element(e) => {
                if *e == TimePatternElement::Timezone {
                    let variant = TimezonePatternVariant::Format;
                    let (pattern, scheme) = self.data.get_timezone_pattern(variant);
                    let mut iter = pattern.resolve(self.data, scheme, None);
                    let item = iter.next().unwrap();
                    self.timezone = Some(iter);
                    match item {
                        TimezoneOutputElement::Literal(l) => TimeOutputElement::Literal(l),
                        TimezoneOutputElement::Timezone(t) => TimeOutputElement::Timezone(t),
                        TimezoneOutputElement::Time(t) => TimeOutputElement::Time(t),
                    }
                } else {
                    TimeOutputElement::Time(Cow::Borrowed(e))
                }
            }
            PatternElement::Literal(l) => TimeOutputElement::Literal(l.into()),
            PatternElement::Placeholder(_) => todo!(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimezonePatternElement {
    Name,
    Time(TimePatternElement),
}

impl TryFrom<char> for TimezonePatternElement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'z' => Ok(Self::Name),
            ch => TimePatternElement::try_from(ch).map(TimezonePatternElement::Time),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimezonePatternVariant {
    Format,
    HourFormat,
    FallbackFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimezonePatternPlaceholderScheme {
    Name,
    NameOffset,
}

pub enum TimezoneRole {
    Name,
}

impl<'output> Pattern<'output, TimezonePatternElement>
    for Vec<PatternElement<TimezonePatternElement>>
{
    type OutputElement = TimezoneOutputElement<'output>;
    type Provider = DateTimeData;
    type Iter = TimezonePatternIterator<'output>;
    type Scheme = TimezonePatternPlaceholderScheme;
    type OutputRole = TimezoneRole;

    fn resolve(
        &'output self,
        provider: &'output Self::Provider,
        scheme: Option<Self::Scheme>,
        ranges: Option<&mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        TimezonePatternIterator {
            elements: self.iter(),
            time: None,
            data: provider,
            scheme,
        }
    }
}

pub struct TimezonePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<TimezonePatternElement>>,
    pub time: Option<Box<TimezonePatternIterator<'output>>>,
    pub data: &'output DateTimeData,
    pub scheme: Option<TimezonePatternPlaceholderScheme>,
}

impl<'output> Iterator for TimezonePatternIterator<'output> {
    type Item = TimezoneOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut time) = self.time {
            if let Some(item) = time.next() {
                return Some(item);
            } else {
                self.time = None;
            }
        }
        self.elements.next().map(|e| -> TimezoneOutputElement {
            match e {
                PatternElement::Element(e) => TimezoneOutputElement::Timezone(Cow::Borrowed(e)),
                PatternElement::Literal(l) => TimezoneOutputElement::Literal(l.into()),
                PatternElement::Placeholder(p)
                    if self.scheme == Some(TimezonePatternPlaceholderScheme::Name) =>
                {
                    assert_eq!(*p, 0usize);
                    TimezoneOutputElement::Timezone(Cow::Owned(TimezonePatternElement::Name))
                }
                PatternElement::Placeholder(p)
                    if self.scheme == Some(TimezonePatternPlaceholderScheme::NameOffset) =>
                {
                    match *p {
                        0 => {
                            let variant = TimezonePatternVariant::HourFormat;
                            let (pattern, scheme) = self.data.get_timezone_pattern(variant);
                            let mut iter = pattern.resolve(self.data, scheme, None);
                            let item = iter.next().unwrap();
                            self.time = Some(Box::new(iter));
                            item
                        }
                        1 => TimezoneOutputElement::Timezone(Cow::Owned(
                            TimezonePatternElement::Name,
                        )),
                        _ => {
                            unreachable!()
                        }
                    }
                }
                PatternElement::Placeholder(p) => {
                    unreachable!()
                }
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DateTimePatternElement {
    Date(DatePatternElement),
    Time(TimePatternElement),
}

impl TryFrom<char> for DateTimePatternElement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        DatePatternElement::try_from(value)
            .map(DateTimePatternElement::Date)
            .or_else(|_| TimePatternElement::try_from(value).map(DateTimePatternElement::Time))
    }
}

pub enum DateTimeRole {
    Date,
    Time,
}

impl<'output> Pattern<'output, DateTimePatternElement>
    for Vec<PatternElement<DateTimePatternElement>>
{
    type OutputElement = DateTimeOutputElement<'output>;
    type Provider = DateTimeData;
    type Iter = DateTimePatternIterator<'output>;
    type Scheme = ();
    type OutputRole = DateTimeRole;

    fn resolve(
        &'output self,
        provider: &'output Self::Provider,
        _scheme: Option<Self::Scheme>,
        ranges: Option<&mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        DateTimePatternIterator {
            elements: self.iter(),
            date: None,
            time: None,
            data: provider,
        }
    }
}

pub struct DateTimePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<DateTimePatternElement>>,
    pub date: Option<Box<DatePatternIterator<'output>>>,
    pub time: Option<Box<TimePatternIterator<'output>>>,
    pub data: &'output DateTimeData,
}

impl<'output> Iterator for DateTimePatternIterator<'output> {
    type Item = DateTimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut date) = self.date {
            if let Some(item) = date.next() {
                match item {
                    DateOutputElement::Literal(l) => {
                        return Some(DateTimeOutputElement::Literal(l));
                    }
                    DateOutputElement::Date(d) => {
                        return Some(DateTimeOutputElement::Date(d));
                    }
                }
            } else {
                self.date = None;
            }
        }

        if let Some(ref mut time) = self.time {
            if let Some(item) = time.next() {
                match item {
                    TimeOutputElement::Literal(l) => {
                        return Some(DateTimeOutputElement::Literal(l));
                    }
                    TimeOutputElement::Time(d) => {
                        return Some(DateTimeOutputElement::Time(d));
                    }
                    TimeOutputElement::Timezone(t) => {
                        return Some(DateTimeOutputElement::Timezone(t));
                    }
                }
            } else {
                self.time = None;
            }
        }

        if let Some(element) = self.elements.next() {
            match element {
                PatternElement::Element(_) => unreachable!(),
                PatternElement::Literal(l) => Some(DateTimeOutputElement::Literal(l.into())),
                PatternElement::Placeholder(p) => match p {
                    0 => {
                        let pattern = self.data.get_time_pattern();
                        let mut iter = pattern.resolve(self.data, None, None);
                        let item = iter.next().unwrap();
                        self.time = Some(Box::new(iter));
                        match item {
                            TimeOutputElement::Literal(l) => {
                                Some(DateTimeOutputElement::Literal(l))
                            }
                            TimeOutputElement::Time(d) => Some(DateTimeOutputElement::Time(d)),
                            TimeOutputElement::Timezone(t) => {
                                Some(DateTimeOutputElement::Timezone(t))
                            }
                        }
                    }
                    1 => {
                        let pattern = self.data.get_date_pattern();
                        let mut iter = pattern.resolve(self.data, None, None);
                        let item = iter.next().unwrap();
                        self.date = Some(Box::new(iter));
                        match item {
                            DateOutputElement::Literal(l) => {
                                Some(DateTimeOutputElement::Literal(l))
                            }
                            DateOutputElement::Date(d) => Some(DateTimeOutputElement::Date(d)),
                        }
                    }
                    _ => unreachable!(),
                },
            }
        } else {
            None
        }
    }
}
