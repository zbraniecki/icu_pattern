use icu_pattern::{
    Pattern,
    PatternIterator,
    // PatternInterpolator,
    PatternItem,
    // RangeCollector,
    ReplacementProvider
};

// struct MyDateRangeCollector {}

// impl RangeCollector for MyDateRangeCollector {}

/* Timezone */
#[derive(PartialEq, Debug, Copy, Clone)]
enum MyTimezonePatternItem {
    Name,
    Time,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum MyTimezonePatternResolvedItem {
    Timezone(MyTimezonePatternItem),
}


impl From<MyTimezonePatternItem> for MyTimezonePatternResolvedItem {
    fn from(input: MyTimezonePatternItem) -> Self {
        Self::Timezone(input)
    }
}

/* End Timezone */

#[derive(PartialEq, Debug, Copy, Clone)]
enum MyDatePatternItem {
    Year,
    Month,
    Day,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum MyTimePatternItem {
    Hour,
    Minute,
    Timezone,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum MyTimePatternResolvedItem {
    Time(MyTimePatternItem),
    Timezone(MyTimezonePatternItem),
}

impl From<MyTimePatternItem> for MyTimePatternResolvedItem {
    fn from(input: MyTimePatternItem) -> Self {
        Self::Time(input)
    }
}

impl From<MyDateTimePatternItem> for MyDateTimePatternResolvedItem {
    fn from(input: MyDateTimePatternItem) -> Self {
        Self::DateTime(input)
    }
}
//
// impl From<MyTimePatternResolvedItem> for MyDateTimePatternResolvedItem {
//     fn from(input: MyTimePatternResolvedItem) -> Self {
//         match input {
//             MyTimePatternResolvedItem::Time(t) => Self::Time(t),
//             MyTimePatternResolvedItem::Timezone(t) => Self::Timezone(t),
//         }
//     }
// }
//
// impl From<MyTimezonePatternItem> for MyTimePatternResolvedItem {
//     fn from(input: MyTimezonePatternItem) -> Self {
//         Self::Timezone(input)
//     }
// }
//
#[derive(PartialEq, Debug, Copy, Clone)]
enum MyDateTimePatternItem {
    Date,
    Time,
    Timezone,
}
//
#[derive(PartialEq, Debug, Copy, Clone)]
enum MyDateTimePatternResolvedItem {
    DateTime(MyDateTimePatternItem),
    Time(MyTimePatternItem),
    Date(MyDatePatternItem),
    Timezone(MyTimezonePatternItem),
}
//
// impl From<MyTimePatternItem> for MyDateTimePatternResolvedItem {
//     fn from(input: MyTimePatternItem) -> Self {
//         MyDateTimePatternResolvedItem::Time(input)
//     }
// }
//
// impl From<MyDatePatternItem> for MyDateTimePatternResolvedItem {
//     fn from(input: MyDatePatternItem) -> Self {
//         MyDateTimePatternResolvedItem::Date(input)
//     }
// }
//
// impl From<MyDateTimePatternItem> for MyDateTimePatternResolvedItem {
//     fn from(input: MyDateTimePatternItem) -> Self {
//         MyDateTimePatternResolvedItem::DateTime(input)
//     }
// }
//
// impl<'a>
//     ReplacementProvider<
//         'a,
//         MyDateTimePatternItem,
//         MyDateTimePatternResolvedItem,
//         DTReplacementProviderIterator<'a>,
//     > for MyData
// {
//     fn get_pattern(&'a self, key: &MyDateTimePatternItem) -> Option<DTReplacementProviderIterator> {
//         Some(DTReplacementProviderIterator {
//             data: self,
//             key: *key,
//             idx: 0,
//         })
//     }
// }
//
// struct DTReplacementProviderIterator<'a> {
//     data: &'a MyData,
//     key: MyDateTimePatternItem,
//     idx: usize,
// }
//
// impl<'a> Iterator for DTReplacementProviderIterator<'a> {
//     type Item = PatternItem<MyDateTimePatternResolvedItem>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.key {
//             MyDateTimePatternItem::Time => {
//                 if let Some(item) = self.data.time.0.get(self.idx) {
//                     self.idx += 1;
//                     match item {
//                         PatternItem::Item(i) => {
//                             let mut sub_pattern = self.data.get_pattern(i).unwrap();
//                             let item = sub_pattern.next().unwrap();
//                             match item {
//                                 PatternItem::Item(i) => {
//                                     Some(PatternItem::Item(i.into()))
//                                 },
//                                 PatternItem::Literal(l) => {
//                                     Some(PatternItem::Literal(l))
//                                 },
//                             }
//                         },
//                         PatternItem::Literal(l) => Some(PatternItem::Literal(l.clone())),
//                     }
//                 } else {
//                     None
//                 }
//             }
//             MyDateTimePatternItem::Date => {
//                 if let Some(item) = self.data.date.0.get(self.idx) {
//                     self.idx += 1;
//                     match item {
//                         PatternItem::Item(i) => Some(PatternItem::Item((*i).into())),
//                         PatternItem::Literal(l) => Some(PatternItem::Literal(l.clone())),
//                     }
//                 } else {
//                     None
//                 }
//             }
//             MyDateTimePatternItem::Timezone => {
//                 if let Some(item) = self.data.date.0.get(self.idx) {
//                     self.idx += 1;
//                     match item {
//                         PatternItem::Item(i) => Some(PatternItem::Item((*i).into())),
//                         PatternItem::Literal(l) => Some(PatternItem::Literal(l.clone())),
//                     }
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
// }
//
// impl<'a>
//     ReplacementProvider<
//         'a,
//         MyTimePatternItem,
//         MyTimePatternResolvedItem,
//         // PatternInterpolator<'a, MyTimePatternItem, MyTimePatternResolvedItem, Self, _, _>,
//         TReplacementProviderIterator<'a>,
//     > for MyData
// {
//     fn get_pattern(&'a self, key: &MyTimePatternItem) -> Option<TReplacementProviderIterator<'a>> {
//         match key {
//             MyTimePatternItem::Timezone => Some(TReplacementProviderIterator {
//                 data: self,
//                 key: *key,
//                 idx: 0,
//             }),
//             _ => None,
//         }
//     }
// }
//
// struct TReplacementProviderIterator<'a> {
//     data: &'a MyData,
//     key: MyTimePatternItem,
//     idx: usize,
// }
//
// impl<'a> Iterator for TReplacementProviderIterator<'a> {
//     type Item = PatternItem<MyTimePatternResolvedItem>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.key {
//             MyTimePatternItem::Timezone => {
//                 if let Some(item) = self.data.timezone.0.get(self.idx) {
//                     self.idx += 1;
//                     match item {
//                         PatternItem::Item(i) => {
//                             let mut sub_pattern = self.data.get_pattern(i).unwrap();
//                             let item = sub_pattern.next().unwrap();
//                             match item {
//                                 PatternItem::Item(i) => {
//                                     Some(PatternItem::Item(i.into()))
//                                 },
//                                 PatternItem::Literal(l) => {
//                                     Some(PatternItem::Literal(l))
//                                 },
//                             }
//                             // Some(PatternItem::Item((*i).into()))
//                         },
//                         PatternItem::Literal(l) => Some(PatternItem::Literal(l.clone())),
//                     }
//                 } else {
//                     None
//                 }
//             }
//             _ => None,
//         }
//     }
// }
//
// impl<'a>
//     ReplacementProvider<
//         'a,
//         MyTimezonePatternItem,
//         MyTimezonePatternResolvedItem,
//         TZReplacementProviderIterator<'a>,
//     > for MyData
// {
//     fn get_pattern(&'a self, key: &MyTimezonePatternItem) -> Option<TZReplacementProviderIterator<'a>> {
//         Some(TZReplacementProviderIterator {
//             data: self,
//             key: *key,
//             idx: 0,
//         })
//     }
// }
//
// struct TZReplacementProviderIterator<'a> {
//     data: &'a MyData,
//     key: MyTimezonePatternItem,
//     idx: usize,
// }
//
// impl<'a> Iterator for TZReplacementProviderIterator<'a> {
//     type Item = PatternItem<MyTimezonePatternResolvedItem>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(PatternItem::Item(self.key.into()))
//         // match self.key {
//         //     MyTimezonePatternItem:: => {
//         //         if let Some(item) = self.data.date.0.get(self.idx) {
//         //             self.idx += 1;
//         //             match item {
//         //                 PatternItem::Item(i) => Some(PatternItem::Item((*i).into())),
//         //                 PatternItem::Literal(l) => Some(PatternItem::Literal(l.clone())),
//         //             }
//         //         } else {
//         //             None
//         //         }
//         //     }
//         // }
//     }
// }


enum DataType {
    DateTime,
    Date,
    Time,
    Timezone,
}

struct MyData {
    date_time: Pattern<MyDateTimePatternItem>,
    date: Pattern<MyDatePatternItem>,
    time: Pattern<MyTimePatternItem>,
    timezone: Pattern<MyTimezonePatternItem>,
}

impl<'a>
    ReplacementProvider<
        'a,
        DataType,
        MyTimezonePatternResolvedItem,
        TimezonePatternIterator<'a>,
    > for MyData
{
    fn get_pattern(&'a self, key: &DataType) -> Option<TimezonePatternIterator<'a>> {
        match key {
            DataType::Timezone => {
                Some(TimezonePatternIterator {
                    data: self,
                    pattern: self.timezone.into_iter(),
                })
            },
            _ => {
                todo!()
            }
        }
    }
}

struct TimezonePatternIterator<'a> {
    pub data: &'a MyData,
    pub pattern: PatternIterator<MyTimezonePatternItem>,
}

impl<'a> Iterator for TimezonePatternIterator<'a> {
    type Item = PatternItem<MyTimezonePatternResolvedItem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pattern.next().map(|pi| {
            match pi {
                PatternItem::Item(i) => {
                    PatternItem::Item(i.into())
                },
                PatternItem::Literal(l) => {
                    PatternItem::Literal(l)
                },
            }
        })
    }
}


type TimePatternIterator1<'a> = TimePatternIterator<'a, std::vec::IntoIter<PatternItem<MyTimePatternResolvedItem>>>;
impl<'a>
    ReplacementProvider<
        'a,
        DataType,
        MyTimePatternResolvedItem,
        TimePatternIterator1<'a>,
    > for MyData
{
    fn get_pattern(&'a self, key: &DataType) -> Option<TimePatternIterator1<'a>> {
        match key {
            DataType::Time => {
                Some(TimePatternIterator {
                    data: self,
                    pattern: self.time.into_iter(),
                })
            },
            _ => {
                todo!()
            }
        }
    }
}

impl<'a>
    ReplacementProvider<
        'a,
        MyTimePatternItem,
        MyTimePatternResolvedItem,
        TimePatternIterator1<'a>,
    > for MyData
{
    fn get_pattern(&'a self, key: &MyTimePatternItem) -> Option<TimePatternIterator1<'a>> {
        match key {
            // MyTimePatternItem::Timezone => {
            //     Some(TimePatternIterator {
            //         data: self,
            //         pattern: self.timezone.into_iter().map(|i| i.into())
            //     })
            // },
            _ => {
                todo!()
            }
        }
    }
}

impl<'a>
    ReplacementProvider<
        'a,
        DataType,
        MyDateTimePatternResolvedItem,
        DateTimePatternIterator<'a>,
    > for MyData
{
    fn get_pattern(&'a self, key: &DataType) -> Option<DateTimePatternIterator<'a>> {
        match key {
            DataType::DateTime => {
                Some(DateTimePatternIterator {
                    data: self,
                    pattern: self.date_time.into_iter(),
                })
            },
            _ => {
                todo!()
            }
        }
    }
}

struct TimePatternIterator<'a, Iter>
where Iter: Iterator<Item = PatternItem<MyTimePatternResolvedItem>> {
    pub data: &'a MyData,
    pub pattern: Iter,
}

impl<'a, Iter> Iterator for TimePatternIterator<'a, Iter> 
where Iter: Iterator<Item = Self::Item> {
    type Item = PatternItem<MyTimePatternResolvedItem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pattern.next().map(|pi| {
            match pi {
                PatternItem::Item(i) => {
                    PatternItem::Item(i.into())
                },
                PatternItem::Literal(l) => {
                    PatternItem::Literal(l)
                },
            }
        })
    }
}

struct DateTimePatternIterator<'a> {
    pub data: &'a MyData,
    pub pattern: PatternIterator<MyDateTimePatternItem>,
}

impl<'a> Iterator for DateTimePatternIterator<'a> {
    type Item = PatternItem<MyDateTimePatternResolvedItem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pattern.next().map(|pi| {
            match pi {
                PatternItem::Item(i) => {
                    PatternItem::Item(i.into())
                },
                PatternItem::Literal(l) => {
                    PatternItem::Literal(l)
                },
            }
        })
    }
}

#[test]
fn core_date_test() {
    let data = MyData {
        date_time: Pattern(vec![
            PatternItem::Item(MyDateTimePatternItem::Time),
            PatternItem::Literal(" at ".to_string()),
            PatternItem::Item(MyDateTimePatternItem::Date),
        ]),
        date: Pattern(vec![
            PatternItem::Item(MyDatePatternItem::Year),
            PatternItem::Literal("/".to_string()),
            PatternItem::Item(MyDatePatternItem::Month),
            PatternItem::Literal("/".to_string()),
            PatternItem::Item(MyDatePatternItem::Day),
        ]),
        time: Pattern(vec![
            PatternItem::Item(MyTimePatternItem::Hour),
            PatternItem::Literal(":".to_string()),
            PatternItem::Item(MyTimePatternItem::Minute),
            PatternItem::Literal(" ".to_string()),
            PatternItem::Item(MyTimePatternItem::Timezone),
        ]),
        timezone: Pattern(vec![
            PatternItem::Item(MyTimezonePatternItem::Name),
            PatternItem::Literal(" Time".to_string()),
        ]),
    };

    {
        // timezone
        let mut pattern: TimezonePatternIterator = data.get_pattern(&DataType::Timezone)
            .unwrap();

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Item(MyTimezonePatternResolvedItem::Timezone(MyTimezonePatternItem::Name)))
        );

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Literal(" Time".to_string())),
        );

        let item = pattern.next();

        assert_eq!(
            item,
            None
        );
    }

    {
        // time
        let mut pattern: TimePatternIterator = data.get_pattern(&DataType::Time)
            .unwrap();

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Item(MyTimePatternResolvedItem::Time(MyTimePatternItem::Hour)))
        );

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Literal(":".to_string())),
        );

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Item(MyTimePatternResolvedItem::Time(MyTimePatternItem::Minute)))
        );

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Literal(" ".to_string())),
        );

        let item = pattern.next();

        assert_eq!(
            item,
            Some(PatternItem::Item(MyTimePatternResolvedItem::Timezone(MyTimezonePatternItem::Time)))
        );
    }

    {
        // date_time
        // let mut pattern: DateTimePatternIterator = data.get_pattern(&DataType::DateTime)
        //     .unwrap();
        //
        // let item = pattern.next();
        //
        // assert_eq!(
        //     item,
        //     Some(PatternItem::Item(MyDateTimePatternResolvedItem::Time(MyTimePatternItem::Hour)))
        // );
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(
        //     value,
        //     Some(PatternItem::Item(MyDateTimePatternResolvedItem::Time(
        //         MyTimePatternItem::Hour
        //     )))
        // );
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(value, Some(PatternItem::Literal(":".to_string())));
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(
        //     value,
        //     Some(PatternItem::Item(MyDateTimePatternResolvedItem::Time(
        //         MyTimePatternItem::Minute
        //     )))
        // );
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(value, Some(PatternItem::Literal(" ".to_string())));
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(
        //     value,
        //     Some(PatternItem::Item(MyDateTimePatternResolvedItem::Time(
        //         MyTimePatternItem::Timezone
        //     )))
        // );
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(value, Some(PatternItem::Literal(" at ".to_string())));
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(
        //     value,
        //     Some(PatternItem::Item(MyDateTimePatternResolvedItem::Date(
        //         MyDatePatternItem::Year
        //     )))
        // );
        //
        // let value = Iterator::next(&mut interpolator);
        // assert_eq!(value, Some(PatternItem::Literal("/".to_string())));
    }
}
