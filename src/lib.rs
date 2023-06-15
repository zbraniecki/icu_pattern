#[derive(Clone, PartialEq, Debug)]
pub enum PatternItem<I> {
    Item(I),
    Literal(String),
}

pub struct Pattern<I>(pub Vec<PatternItem<I>>);

impl<I> Pattern<I>
where I: Clone {
    pub fn into_iter(&self) -> PatternIterator<I> {
        PatternIterator {
            iter: self.0.clone().into_iter()
        }
    }
}

pub struct PatternIterator<I, Iter: Iterator<Item = PatternItem<I>> = std::vec::IntoIter<PatternItem<I>>> {
    iter: Iter,
}

impl<I> Iterator for PatternIterator<I> {
    type Item = PatternItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait ReplacementProvider<'a, KEY, OPI, Iter: 'a> {
    fn get_pattern(&'a self, key: &KEY) -> Option<Iter>;
}

pub trait RangeCollector {}

// pub struct PatternInterpolator<'data, IPI, OPI, RP>
// where
//     RP: ReplacementProvider<'data, IPI, OPI>,
//     OPI: From<IPI>,
// {
//     pattern: PatternIterator<IPI>,
//     provider: &'data RP,
// }
//
// impl<'data, IPI, OPI, RP> PatternInterpolator<'data, IPI, OPI, RP>
// where
//     RP: ReplacementProvider<'data, IPI, OPI>,
//     OPI: From<IPI>,
// {
//     pub fn new(pattern: PatternIterator<IPI>, provider: &'data RP) -> Self {
//         Self {
//             pattern,
//             provider,
//         }
//     }
// }
//
// impl<'data, IPI, OPI, RP> Iterator for PatternInterpolator<'data, IPI, OPI, RP>
// where
//     RP: ReplacementProvider<'data, IPI, OPI>,
//     OPI: From<IPI>,
// {
//     type Item = PatternItem<OPI>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!();
//         // if let Some(ref mut sub) = &mut self.sub_pattern {
//         //     let item = sub.next();
//         //     if let Some(item) = item {
//         //         return Some(item);
//         //     } else {
//         //     }
//         // }
//         // let result: Option<PatternItem<IPI>> = self.pattern.next();
//         // if let Some(pi) = result {
//         //     self.idx += 1;
//         //     match pi {
//         //         PatternItem::Literal(l) => Some(PatternItem::Literal(l)),
//         //         PatternItem::Item(i) => {
//         //             let pattern: Option<Iter> = self.provider.get_pattern(&i);
//         //             if let Some(mut pattern) = pattern {
//         //                 let item = pattern.next();
//         //                 if item.is_some() {
//         //                     self.sub_pattern = Some(pattern);
//         //                 }
//         //                 item
//         //             } else {
//         //                 Some(PatternItem::Item(i.into()))
//         //             }
//         //         }
//         //     }
//         // } else {
//         //     None
//         // }
//     }
// }
