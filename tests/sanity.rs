use icu_pattern::{
    Labellable, Pattern, PatternItem, PatternIterator, RangeCollector, RangeCollectorMarker,
    RangeCollectorMarkerType, ReplacementProvider,
};

/* Timezone */
#[derive(PartialEq, Debug, Copy, Clone)]
enum TimezonePatternItem {
    Name,
}

impl Labellable for TimezonePatternItem {
    fn get_label(&self) -> &'static str {
        match self {
            TimezonePatternItem::Name => "timezone name",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum TimezonePatternResolvedItem {
    Timezone(TimezonePatternItem),
}

impl From<TimezonePatternItem> for TimezonePatternResolvedItem {
    fn from(input: TimezonePatternItem) -> Self {
        Self::Timezone(input)
    }
}

/* End Timezone */

#[derive(PartialEq, Debug, Copy, Clone)]
enum DatePatternItem {
    Year,
    Month,
    Day,
}

impl Labellable for DatePatternItem {
    fn get_label(&self) -> &'static str {
        match self {
            DatePatternItem::Year => "date year",
            DatePatternItem::Month => "date month",
            DatePatternItem::Day => "date day",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum TimePatternItem {
    Hour,
    Minute,
    Timezone,
}

impl Labellable for TimePatternItem {
    fn get_label(&self) -> &'static str {
        match self {
            TimePatternItem::Hour => "time hour",
            TimePatternItem::Minute => "time minute",
            TimePatternItem::Timezone => "time timezone",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum DatePatternResolvedItem {
    Date(DatePatternItem),
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum TimePatternResolvedItem {
    Time(TimePatternItem),
    Timezone(TimezonePatternItem),
}

impl From<TimePatternItem> for TimePatternResolvedItem {
    fn from(input: TimePatternItem) -> Self {
        Self::Time(input)
    }
}

impl From<TimezonePatternItem> for TimePatternResolvedItem {
    fn from(input: TimezonePatternItem) -> Self {
        Self::Timezone(input)
    }
}

impl From<DatePatternItem> for DatePatternResolvedItem {
    fn from(input: DatePatternItem) -> Self {
        Self::Date(input)
    }
}

impl From<DateTimePatternItem> for DateTimePatternResolvedItem {
    fn from(input: DateTimePatternItem) -> Self {
        Self::DateTime(input)
    }
}

impl From<DatePatternItem> for DateTimePatternResolvedItem {
    fn from(input: DatePatternItem) -> Self {
        Self::Date(input)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum DateTimePatternItem {
    Date,
    Time,
}

impl Labellable for DateTimePatternItem {
    fn get_label(&self) -> &'static str {
        match self {
            DateTimePatternItem::Date => "datetime date",
            DateTimePatternItem::Time => "datetime time",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum DateTimePatternResolvedItem {
    DateTime(DateTimePatternItem),
    Time(TimePatternItem),
    Date(DatePatternItem),
    Timezone(TimezonePatternItem),
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum GenericPatternResolvedItem {
    Item(&'static str),
}

impl From<TimezonePatternItem> for GenericPatternResolvedItem {
    fn from(input: TimezonePatternItem) -> Self {
        match input {
            TimezonePatternItem::Name => Self::Item("timezone name"),
        }
    }
}

impl From<TimePatternItem> for DateTimePatternResolvedItem {
    fn from(input: TimePatternItem) -> Self {
        DateTimePatternResolvedItem::Time(input)
    }
}

impl From<TimezonePatternItem> for DateTimePatternResolvedItem {
    fn from(input: TimezonePatternItem) -> Self {
        DateTimePatternResolvedItem::Timezone(input)
    }
}

struct MyData {
    date_time: Pattern<DateTimePatternItem>,
    date: Pattern<DatePatternItem>,
    time: Pattern<TimePatternItem>,
    timezone: Pattern<TimezonePatternItem>,
}

trait PatternGetter<InputItem, OutputItem> {
    fn get_pattern<RC>(&self) -> Option<PatternIterator<Self, RC, InputItem, OutputItem>>
    where
        Self: Sized;
}

impl PatternGetter<TimezonePatternItem, TimezonePatternResolvedItem> for MyData {
    fn get_pattern<RC>(
        &self,
    ) -> Option<PatternIterator<Self, RC, TimezonePatternItem, TimezonePatternResolvedItem>>
    where
        Self: Sized,
    {
        Some(PatternIterator {
            data: self,
            iter: Box::new(self.timezone.0.clone().into_iter()),
            sub_iter: None,
            collector: None,
            idx: 0,
        })
    }
}

impl PatternGetter<TimePatternItem, TimePatternResolvedItem> for MyData {
    fn get_pattern<RC>(
        &self,
    ) -> Option<PatternIterator<Self, RC, TimePatternItem, TimePatternResolvedItem>>
    where
        Self: Sized,
    {
        Some(PatternIterator {
            data: self,
            iter: Box::new(self.time.0.clone().into_iter()),
            sub_iter: None,
            collector: None,
            idx: 0,
        })
    }
}

impl PatternGetter<DatePatternItem, DatePatternResolvedItem> for MyData {
    fn get_pattern<RC>(
        &self,
    ) -> Option<PatternIterator<Self, RC, DatePatternItem, DatePatternResolvedItem>>
    where
        Self: Sized,
    {
        Some(PatternIterator {
            data: self,
            iter: Box::new(self.date.0.clone().into_iter()),
            sub_iter: None,
            collector: None,
            idx: 0,
        })
    }
}

impl PatternGetter<DateTimePatternItem, DateTimePatternResolvedItem> for MyData {
    fn get_pattern<RC>(
        &self,
    ) -> Option<PatternIterator<Self, RC, DateTimePatternItem, DateTimePatternResolvedItem>>
    where
        Self: Sized,
    {
        Some(PatternIterator {
            data: self,
            iter: Box::new(self.date_time.0.clone().into_iter()),
            sub_iter: None,
            collector: None,
            idx: 0,
        })
    }
}

impl PatternGetter<TimezonePatternItem, GenericPatternResolvedItem> for MyData {
    fn get_pattern<RC>(
        &self,
    ) -> Option<PatternIterator<Self, RC, TimezonePatternItem, GenericPatternResolvedItem>>
    where
        Self: Sized,
    {
        Some(PatternIterator {
            data: self,
            iter: Box::new(self.timezone.0.clone().into_iter()),
            sub_iter: None,
            collector: None,
            idx: 0,
        })
    }
}

impl<'s, RC> ReplacementProvider<'s, RC, TimezonePatternItem, TimezonePatternResolvedItem>
    for MyData
{
    fn get_replacement(
        &'s self,
        _key: TimezonePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<TimezonePatternResolvedItem>> + 's>> {
        None
    }
}

impl<'s, RC> ReplacementProvider<'s, RC, TimezonePatternItem, TimePatternResolvedItem> for MyData {
    fn get_replacement(
        &'s self,
        _key: TimezonePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<TimePatternResolvedItem>> + 's>> {
        None
    }
}

impl<'s, RC> ReplacementProvider<'s, RC, DatePatternItem, DatePatternResolvedItem> for MyData {
    fn get_replacement(
        &'s self,
        _key: DatePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<DatePatternResolvedItem>> + 's>> {
        None
    }
}

impl<'s, RC: 's> ReplacementProvider<'s, RC, TimePatternItem, TimePatternResolvedItem> for MyData
where
    RC: RangeCollector,
{
    fn get_replacement(
        &'s self,
        key: TimePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<TimePatternResolvedItem>> + 's>> {
        match key {
            TimePatternItem::Timezone => Some(Box::new(PatternIterator {
                data: self,
                iter: Box::new(self.timezone.0.clone().into_iter()),
                sub_iter: None,
                collector: None::<&'s mut RC>,
                idx: 0,
            })),
            _ => None,
        }
    }
}

impl<'s, RC: 's> ReplacementProvider<'s, RC, DateTimePatternItem, DateTimePatternResolvedItem>
    for MyData
where
    RC: RangeCollector,
{
    fn get_replacement(
        &'s self,
        key: DateTimePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<DateTimePatternResolvedItem>> + 's>> {
        match key {
            DateTimePatternItem::Time => Some(Box::new(PatternIterator {
                data: self,
                iter: Box::new(self.time.0.clone().into_iter()),
                sub_iter: None,
                collector: None::<&'s mut RC>,
                idx: 0,
            })),
            DateTimePatternItem::Date => Some(Box::new(PatternIterator {
                data: self,
                iter: Box::new(self.date.0.clone().into_iter()),
                sub_iter: None,
                collector: None::<&'s mut RC>,
                idx: 0,
            })),
        }
    }
}

impl<'s, RC: 's> ReplacementProvider<'s, RC, TimePatternItem, DateTimePatternResolvedItem>
    for MyData
where
    RC: RangeCollector,
{
    fn get_replacement(
        &'s self,
        key: TimePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<DateTimePatternResolvedItem>> + 's>> {
        match key {
            TimePatternItem::Timezone => Some(Box::new(PatternIterator {
                data: self,
                iter: Box::new(self.timezone.0.clone().into_iter()),
                sub_iter: None,
                collector: None::<&'s mut RC>,
                idx: 0,
            })),
            _ => None,
        }
    }
}

impl<'s, RC> ReplacementProvider<'s, RC, TimezonePatternItem, DateTimePatternResolvedItem>
    for MyData
{
    fn get_replacement(
        &'s self,
        _key: TimezonePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<DateTimePatternResolvedItem>> + 's>> {
        None
    }
}

impl<'s, RC> ReplacementProvider<'s, RC, DatePatternItem, DateTimePatternResolvedItem> for MyData {
    fn get_replacement(
        &'s self,
        _key: DatePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<DateTimePatternResolvedItem>> + 's>> {
        None
    }
}

impl<'s, RC> ReplacementProvider<'s, RC, TimezonePatternItem, GenericPatternResolvedItem>
    for MyData
{
    fn get_replacement(
        &'s self,
        _key: TimezonePatternItem,
    ) -> Option<Box<dyn Iterator<Item = PatternItem<GenericPatternResolvedItem>> + 's>> {
        None
    }
}

struct MyRangeCollector(Vec<RangeCollectorMarker>);

impl RangeCollector for MyRangeCollector {
    type Iter = std::vec::IntoIter<RangeCollectorMarker>;

    fn add_marker(
        &mut self,
        label: &'static str,
        marker_type: RangeCollectorMarkerType,
        idx: usize,
    ) {
        self.0.push((label, marker_type, idx));
    }

    fn get_markers(&self) -> Self::Iter {
        self.0.clone().into_iter()
    }
}

#[test]
fn core_date_test() {
    let data = MyData {
        date_time: Pattern(vec![
            PatternItem::Item(DateTimePatternItem::Time),
            PatternItem::Literal(" at ".to_string()),
            PatternItem::Item(DateTimePatternItem::Date),
        ]),
        date: Pattern(vec![
            PatternItem::Item(DatePatternItem::Year),
            PatternItem::Literal("/".to_string()),
            PatternItem::Item(DatePatternItem::Month),
            PatternItem::Literal("/".to_string()),
            PatternItem::Item(DatePatternItem::Day),
        ]),
        time: Pattern(vec![
            PatternItem::Item(TimePatternItem::Hour),
            PatternItem::Literal(":".to_string()),
            PatternItem::Item(TimePatternItem::Minute),
            PatternItem::Literal(" ".to_string()),
            PatternItem::Item(TimePatternItem::Timezone),
        ]),
        timezone: Pattern(vec![
            PatternItem::Item(TimezonePatternItem::Name),
            PatternItem::Literal(" Time".to_string()),
        ]),
    };

    let mut rc = MyRangeCollector(vec![]);

    {
        // timezone
        {
            let mut pattern = data.get_pattern().unwrap();

            pattern.set_range_collector(&mut rc);

            let item = pattern.get_next();

            assert_eq!(
                item,
                Some(PatternItem::Item(TimezonePatternResolvedItem::Timezone(
                    TimezonePatternItem::Name
                )))
            );

            let item = pattern.get_next();

            assert_eq!(item, Some(PatternItem::Literal(" Time".to_string())),);

            let item = pattern.get_next();

            assert_eq!(item, None);
        }

        assert_eq!(rc.get_markers().collect::<Vec<_>>(), vec![]);
    }

    {
        // date
        let mut pattern: PatternIterator<'_, _, _, _, DatePatternResolvedItem> =
            data.get_pattern().unwrap();

        pattern.set_range_collector(&mut rc);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DatePatternResolvedItem::Date(
                DatePatternItem::Year
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal("/".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DatePatternResolvedItem::Date(
                DatePatternItem::Month
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal("/".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DatePatternResolvedItem::Date(
                DatePatternItem::Day
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, None);
    }

    {
        // time
        {
            let mut pattern: PatternIterator<'_, _, _, _, TimePatternResolvedItem> =
                data.get_pattern().unwrap();

            pattern.set_range_collector(&mut rc);

            let item = pattern.get_next();

            assert_eq!(
                item,
                Some(PatternItem::Item(TimePatternResolvedItem::Time(
                    TimePatternItem::Hour
                )))
            );

            let item = pattern.get_next();

            assert_eq!(item, Some(PatternItem::Literal(":".to_string())),);

            let item = pattern.get_next();

            assert_eq!(
                item,
                Some(PatternItem::Item(TimePatternResolvedItem::Time(
                    TimePatternItem::Minute
                )))
            );

            let item = pattern.get_next();

            assert_eq!(item, Some(PatternItem::Literal(" ".to_string())),);

            let item = pattern.get_next();

            assert_eq!(
                item,
                Some(PatternItem::Item(TimePatternResolvedItem::Timezone(
                    TimezonePatternItem::Name
                )))
            );

            let item = pattern.get_next();

            assert_eq!(item, Some(PatternItem::Literal(" Time".to_string())),);

            let item = pattern.get_next();

            assert_eq!(item, None);
        }

        assert_eq!(
            rc.get_markers().collect::<Vec<_>>(),
            vec![
                ("time timezone", RangeCollectorMarkerType::Start, 4),
                ("time timezone", RangeCollectorMarkerType::End, 5)
            ]
        );
    }

    {
        // date_time
        let mut pattern: PatternIterator<'_, _, _, _, DateTimePatternResolvedItem> =
            data.get_pattern().unwrap();

        pattern.set_range_collector(&mut rc);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DateTimePatternResolvedItem::Time(
                TimePatternItem::Hour
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal(":".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DateTimePatternResolvedItem::Time(
                TimePatternItem::Minute
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal(" ".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DateTimePatternResolvedItem::Timezone(
                TimezonePatternItem::Name
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal(" Time".to_string())),);

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal(" at ".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DateTimePatternResolvedItem::Date(
                DatePatternItem::Year
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal("/".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DateTimePatternResolvedItem::Date(
                DatePatternItem::Month
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal("/".to_string())),);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(DateTimePatternResolvedItem::Date(
                DatePatternItem::Day
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, None);
    }

    {
        // timezone
        let mut pattern: PatternIterator<
            '_,
            _,
            _,
            TimezonePatternItem,
            GenericPatternResolvedItem,
        > = data.get_pattern().unwrap();

        pattern.set_range_collector(&mut rc);

        let item = pattern.get_next();

        assert_eq!(
            item,
            Some(PatternItem::Item(GenericPatternResolvedItem::Item(
                "timezone name"
            )))
        );

        let item = pattern.get_next();

        assert_eq!(item, Some(PatternItem::Literal(" Time".to_string())),);

        let item = pattern.get_next();

        assert_eq!(item, None);
    }
}
