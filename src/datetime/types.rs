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

impl<'input> Pattern<'input, DatePatternElement>
    for Vec<PatternElement<'input, DatePatternElement>>
{
    type OutputElement = DateOutputElement<'input>;
    type Provider = DateTimeData<'input>;
    type Iter = DatePatternIterator<'input>;
    type Scheme = ();
    type OutputRole = DateRole;

    fn resolve(
        &'input self,
        _provider: &Self::Provider,
        _scheme: Option<Self::Scheme>,
        ranges: Option<&'input mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        DatePatternIterator {
            elements: self.iter().enumerate(),
            ranges,
        }
    }
}

pub struct DatePatternIterator<'input> {
    pub elements:
        std::iter::Enumerate<std::slice::Iter<'input, PatternElement<'input, DatePatternElement>>>,
    pub ranges: Option<&'input mut RangeList<DateRole>>,
}

impl<'input> Iterator for DatePatternIterator<'input> {
    type Item = DateOutputElement<'input>;

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
            PatternElement::Literal(l) => DateOutputElement::Literal(l.clone()),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimeRole {
    Hour,
    Minute,
    Second,
    Timezone,
}

impl<'input> Pattern<'input, TimePatternElement>
    for Vec<PatternElement<'input, TimePatternElement>>
{
    type OutputElement = TimeOutputElement<'input>;
    type Provider = DateTimeData<'input>;
    type Iter = TimePatternIterator<'input>;
    type Scheme = ();
    type OutputRole = TimeRole;

    fn resolve(
        &'input self,
        provider: &'input Self::Provider,
        _scheme: Option<Self::Scheme>,
        ranges: Option<&'input mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        TimePatternIterator {
            elements: self.iter(),
            data: provider,
            timezone: None,
            ranges,
            idx: 0,
        }
    }
}

pub struct TimePatternIterator<'input> {
    pub elements: std::slice::Iter<'input, PatternElement<'input, TimePatternElement>>,
    pub data: &'input DateTimeData<'input>,
    pub timezone: Option<(TimezonePatternIterator<'input>, usize)>,
    pub ranges: Option<&'input mut RangeList<TimeRole>>,
    pub idx: usize,
}

impl<'input> Iterator for TimePatternIterator<'input> {
    type Item = TimeOutputElement<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if let Some((ref mut tz, idx)) = self.timezone {
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
                if let Some(ref mut ranges) = self.ranges {
                    ranges.push(Range {
                        role: TimeRole::Timezone,
                        range: idx..self.idx,
                    });
                }
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
                    self.timezone = Some((iter, self.idx - 1));
                    match item {
                        TimezoneOutputElement::Literal(l) => TimeOutputElement::Literal(l),
                        TimezoneOutputElement::Timezone(t) => TimeOutputElement::Timezone(t),
                        TimezoneOutputElement::Time(t) => TimeOutputElement::Time(t),
                    }
                } else {
                    if let Some(ranges) = &mut self.ranges {
                        let role = match e {
                            TimePatternElement::Hour => TimeRole::Hour,
                            TimePatternElement::Minute => TimeRole::Minute,
                            _ => unreachable!(),
                        };
                        ranges.push(Range {
                            role,
                            range: self.idx - 1..self.idx,
                        });
                    }
                    TimeOutputElement::Time(Cow::Borrowed(e))
                }
            }
            PatternElement::Literal(l) => TimeOutputElement::Literal(l.clone()),
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
    Offset,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimezoneRole {
    Name,
    Offset,
}

impl<'input> Pattern<'input, TimezonePatternElement>
    for Vec<PatternElement<'input, TimezonePatternElement>>
{
    type OutputElement = TimezoneOutputElement<'input>;
    type Provider = DateTimeData<'input>;
    type Iter = TimezonePatternIterator<'input>;
    type Scheme = TimezonePatternPlaceholderScheme;
    type OutputRole = TimezoneRole;

    fn resolve(
        &'input self,
        provider: &'input Self::Provider,
        scheme: Option<Self::Scheme>,
        ranges: Option<&'input mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        TimezonePatternIterator {
            elements: self.iter(),
            time: None,
            data: provider,
            scheme,
            ranges,
            idx: 0,
        }
    }
}

pub struct TimezonePatternIterator<'input> {
    pub elements: std::slice::Iter<'input, PatternElement<'input, TimezonePatternElement>>,
    pub time: Option<(Box<TimezonePatternIterator<'input>>, usize)>,
    pub data: &'input DateTimeData<'input>,
    pub scheme: Option<TimezonePatternPlaceholderScheme>,
    pub ranges: Option<&'input mut RangeList<TimezoneRole>>,
    pub idx: usize,
}

impl<'input> Iterator for TimezonePatternIterator<'input> {
    type Item = TimezoneOutputElement<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if let Some((ref mut time, idx)) = self.time {
            if let Some(item) = time.next() {
                return Some(item);
            } else {
                if let Some(ref mut ranges) = self.ranges {
                    ranges.push(Range {
                        role: TimezoneRole::Offset,
                        range: idx..self.idx - 1,
                    });
                }
                self.time = None;
            }
        }
        self.elements.next().map(|e| -> TimezoneOutputElement {
            match e {
                PatternElement::Element(e) => TimezoneOutputElement::Timezone(Cow::Borrowed(e)),
                PatternElement::Literal(l) => TimezoneOutputElement::Literal(l.clone()),
                PatternElement::Placeholder(p)
                    if self.scheme == Some(TimezonePatternPlaceholderScheme::Name) =>
                {
                    assert_eq!(*p, 0usize);
                    if let Some(ref mut ranges) = self.ranges {
                        ranges.push(Range {
                            role: TimezoneRole::Name,
                            range: self.idx - 1..self.idx,
                        });
                    }
                    TimezoneOutputElement::Timezone(Cow::Owned(TimezonePatternElement::Name))
                }
                PatternElement::Placeholder(p)
                    if self.scheme == Some(TimezonePatternPlaceholderScheme::Offset) =>
                {
                    match *p {
                        0 => {
                            let variant = TimezonePatternVariant::HourFormat;
                            let (pattern, scheme) = self.data.get_timezone_pattern(variant);
                            let mut iter = pattern.resolve(self.data, scheme, None);
                            let item = iter.next().unwrap();
                            self.time = Some((Box::new(iter), self.idx - 1));
                            item
                        }
                        1 => {
                            if let Some(ref mut ranges) = self.ranges {
                                ranges.push(Range {
                                    role: TimezoneRole::Name,
                                    range: self.idx - 1..self.idx,
                                });
                            }
                            TimezoneOutputElement::Timezone(Cow::Owned(
                                TimezonePatternElement::Name,
                            ))
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                PatternElement::Placeholder(_) => {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DateTimeRole {
    Date,
    Time,
}

impl<'input> Pattern<'input, DateTimePatternElement>
    for Vec<PatternElement<'input, DateTimePatternElement>>
{
    type OutputElement = DateTimeOutputElement<'input>;
    type Provider = DateTimeData<'input>;
    type Iter = DateTimePatternIterator<'input>;
    type Scheme = ();
    type OutputRole = DateTimeRole;

    fn resolve(
        &'input self,
        provider: &'input Self::Provider,
        _scheme: Option<Self::Scheme>,
        ranges: Option<&'input mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter {
        DateTimePatternIterator {
            elements: self.iter(),
            date: None,
            time: None,
            data: provider,
            ranges,
            idx: 0,
        }
    }
}

pub struct DateTimePatternIterator<'input> {
    pub elements: std::slice::Iter<'input, PatternElement<'input, DateTimePatternElement>>,
    pub date: Option<(Box<DatePatternIterator<'input>>, usize)>,
    pub time: Option<(Box<TimePatternIterator<'input>>, usize)>,
    pub data: &'input DateTimeData<'input>,
    pub ranges: Option<&'input mut RangeList<DateTimeRole>>,
    pub idx: usize,
}

impl<'input> Iterator for DateTimePatternIterator<'input> {
    type Item = DateTimeOutputElement<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if let Some((ref mut date, idx)) = self.date {
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
                if let Some(ref mut ranges) = self.ranges {
                    ranges.push(Range {
                        role: DateTimeRole::Date,
                        range: idx..self.idx - 1,
                    });
                }
                self.date = None;
            }
        }

        if let Some((ref mut time, idx)) = self.time {
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
                if let Some(ref mut ranges) = self.ranges {
                    ranges.push(Range {
                        role: DateTimeRole::Time,
                        range: idx..self.idx - 1,
                    });
                }
                self.time = None;
            }
        }

        if let Some(element) = self.elements.next() {
            match element {
                PatternElement::Element(_) => unreachable!(),
                PatternElement::Literal(l) => Some(DateTimeOutputElement::Literal(l.clone())),
                PatternElement::Placeholder(p) => match p {
                    0 => {
                        let pattern = self.data.get_time_pattern();
                        let mut iter = pattern.resolve(self.data, None, None);
                        let item = iter.next().unwrap();
                        self.time = Some((Box::new(iter), self.idx - 1));
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
                        self.date = Some((Box::new(iter), self.idx - 1));
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
