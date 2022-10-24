use crate::datetime::output::{
    DateOutput, DateOutputElement, DateTimeOutput, DateTimeOutputElement, TimeOutput,
    TimeOutputElement, TimezoneOutput, TimezoneOutputElement,
};
use crate::datetime::DateTimeData;
use crate::datetime::resolver::{DateResolver, DateTimeResolver, TimeResolver, TimezoneResolver};
use crate::resolver::Resolver;
use crate::{
    output::Output,
    pattern::{Never, Pattern, PatternElement},
};

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

impl<'output> Pattern<'output, DatePatternElement> for Vec<PatternElement<DatePatternElement>> {
    type OutputElement = DateOutputElement<'output>;
    type Resolver = DateResolver<'output>;
    type Iter = DatePatternIterator<'output>;

    fn resolve(&'output self, resolver: &Self::Resolver) -> Self::Iter {
        DatePatternIterator {
            elements: self.iter(),
        }
    }
}

pub struct DatePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<DatePatternElement>>,
}

impl<'output> Iterator for DatePatternIterator<'output> {
    type Item = DateOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.next().map(|e| {
            match e {
                PatternElement::Element(e) => {
                    DateOutputElement::Date(e)
                },
                PatternElement::Literal(l) => {
                    DateOutputElement::Literal(l)
                },
                PatternElement::Placeholder(_) => todo!(),
            }
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

impl<'output> Pattern<'output, TimePatternElement> for Vec<PatternElement<TimePatternElement>> {
    type OutputElement = TimeOutputElement<'output>;
    type Resolver = TimeResolver<'output>;
    type Iter = TimePatternIterator<'output>;

    fn resolve(&'output self, resolver: &Self::Resolver) -> Self::Iter {
        TimePatternIterator {
            elements: self.iter(),
        }
    }
}

pub struct TimePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<TimePatternElement>>,
}

impl<'output> Iterator for TimePatternIterator<'output> {
    type Item = TimeOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.next().map(|e| {
            match e {
                PatternElement::Element(e) => {
                    TimeOutputElement::Time(e)
                },
                PatternElement::Literal(l) => {
                    TimeOutputElement::Literal(l)
                },
                PatternElement::Placeholder(_) => todo!(),
            }
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

impl<'output> Pattern<'output, TimezonePatternElement>
    for Vec<PatternElement<TimezonePatternElement>>
{
    type OutputElement = TimezoneOutputElement<'output>;
    type Resolver = TimezoneResolver<'output>;
    type Iter = TimezonePatternIterator<'output>;

    fn resolve(&'output self, resolver: &Self::Resolver) -> Self::Iter {
        TimezonePatternIterator {
            elements: self.iter(),
            time: None,
            data: resolver.data,
        }
    }
}

pub struct TimezonePatternIterator<'output> {
    pub elements: std::slice::Iter<'output, PatternElement<TimezonePatternElement>>,
    pub time: Option<TimePatternIterator<'output>>,
    pub data: &'output DateTimeData,
}

impl<'output> Iterator for TimezonePatternIterator<'output> {
    type Item = TimezoneOutputElement<'output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut time) = self.time {
            if let Some(item) = time.next() {
                match item {
                    TimeOutputElement::Literal(l) => {
                        return Some(TimezoneOutputElement::Literal(l));
                    },
                    TimeOutputElement::Time(t) => {
                        return Some(TimezoneOutputElement::Time(t));
                    },
                    TimeOutputElement::Timezone(_) => {
                        unreachable!()
                    },
                }
            } else {
                self.time = None;
            }
        }
        self.elements.next().map(|e| {
            match e {
                PatternElement::Element(e) => {
                    TimezoneOutputElement::Timezone(e)
                },
                PatternElement::Literal(l) => {
                    TimezoneOutputElement::Literal(l)
                },
                PatternElement::Placeholder(p) => {
                    assert_eq!(*p, 0usize);
                    let resolver = TimeResolver {
                        data: self.data,
                    };
                    let mut iter = self.data.get_time_pattern().resolve(&resolver);
                    let item = iter.next().unwrap();
                    self.time = Some(iter);
                    match item {
                        TimeOutputElement::Literal(l) => {
                            TimezoneOutputElement::Literal(l)
                        },
                        TimeOutputElement::Time(t) => {
                            TimezoneOutputElement::Time(t)
                        },
                        TimeOutputElement::Timezone(_) => {
                            unreachable!()
                        },
                    }
                },
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
//
// impl<'output> Pattern<'output, DateTimePatternElement>
//     for Vec<PatternElement<DateTimePatternElement>>
// {
//     type OutputElement = DateTimeOutputElement<'output>;
//     type Output = DateTimeOutput<'output>;
//     type Resolver = DateTimeResolver<'output>;
//
//     fn resolve(&'output self, output: &mut Self::Output, resolver: &Self::Resolver) {
//         for element in self {
//             match element {
//                 PatternElement::Element(_) => {
//                     unreachable!()
//                 }
//                 PatternElement::Literal(l) => {
//                     output.push_element(DateTimeOutputElement::Literal(l));
//                 }
//                 PatternElement::Placeholder(p) => {
//                     resolver.get(*p, output);
//                 }
//             }
//         }
//     }
// }
