#[derive(Clone, PartialEq, Debug)]
pub enum PatternItem<I> {
    Item(I),
    Literal(String),
}

pub trait Labellable {
    fn get_label(&self) -> &'static str;
}

pub trait PatternIterator {
    type Item;
    fn next(&mut self) -> Option<PatternItem<Self::Item>>;
}

pub struct PatternInterpolator<'s, RP, RC, Iter, InputItem, OutputItem> {
    pub data: &'s RP,
    pub iter: Iter,
    pub sub_iter: Option<(InputItem, Box<dyn PatternIterator<Item = OutputItem> + 's>)>,
    pub collector: Option<&'s mut RC>,
    pub idx: usize,
}

impl<'s, RP, RC, Iter, InputItem, OutputItem>
    PatternInterpolator<'s, RP, RC, Iter, InputItem, OutputItem>
where
    RP: ReplacementProvider<'s, RC, InputItem, OutputItem>,
    RC: RangeCollector,
    Iter: PatternIterator<Item = InputItem>,
    OutputItem: From<InputItem>,
    InputItem: Copy + Labellable,
{
    pub fn new(data: &'s RP, iter: Iter) -> Self {
        Self {
            data,
            iter,
            sub_iter: None,
            collector: None,
            idx: 0,
        }
    }

    pub fn set_range_collector(&mut self, rc: &'s mut RC) {
        self.collector = Some(rc);
    }

    pub fn get_next(&mut self) -> Option<PatternItem<OutputItem>> {
        if let Some((sub_item, ref mut sub_iter)) = &mut self.sub_iter {
            let item = sub_iter.next();
            if let Some(item) = item {
                self.idx += 1;
                return Some(item);
            } else {
                if let Some(rc) = &mut self.collector {
                    rc.add_marker(
                        sub_item.get_label(),
                        RangeCollectorMarkerType::End,
                        self.idx - 1,
                    );
                }
                self.sub_iter = None;
            }
        }

        let item = self.iter.next()?;

        self.idx += 1;
        match item {
            PatternItem::Item(i) => {
                let repl = self.data.get_replacement(i);
                if let Some(mut repl) = repl {
                    if let Some(rc) = &mut self.collector {
                        rc.add_marker(i.get_label(), RangeCollectorMarkerType::Start, self.idx - 1);
                    }
                    let item = repl.next();
                    self.sub_iter = Some((i, repl));
                    item
                } else {
                    Some(PatternItem::Item(i.into()))
                }
            }
            PatternItem::Literal(l) => Some(PatternItem::Literal(l)),
        }
    }
}

impl<'s, RP, RC, Iter, InputItem, OutputItem> PatternIterator
    for PatternInterpolator<'s, RP, RC, Iter, InputItem, OutputItem>
where
    RP: ReplacementProvider<'s, RC, InputItem, OutputItem>,
    RC: RangeCollector,
    Iter: PatternIterator<Item = InputItem>,
    OutputItem: From<InputItem>,
    InputItem: Copy + Labellable,
{
    type Item = OutputItem;

    fn next(&mut self) -> Option<PatternItem<Self::Item>> {
        self.get_next()
    }
}

pub trait ReplacementProvider<'s, RC, Key, OutputItem> {
    fn get_replacement(
        &'s self,
        key: Key,
    ) -> Option<Box<dyn PatternIterator<Item = OutputItem> + 's>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangeCollectorMarkerType {
    Start,
    End,
}

pub type RangeCollectorMarker = (&'static str, RangeCollectorMarkerType, usize);

pub trait RangeCollector {
    type Iter: Iterator<Item = RangeCollectorMarker>;

    fn add_marker(
        &mut self,
        label: &'static str,
        marker_type: RangeCollectorMarkerType,
        idx: usize,
    );
    fn get_markers(&self) -> Self::Iter;
}
