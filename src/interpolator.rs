use crate::pattern::PatternElement;
use crate::range::RangeCollector;

pub struct Interpolator<I, OPE> {
    pub pattern: I,
    pub marker: std::marker::PhantomData<OPE>,
}

impl<'p, 'input: 'p, I, PE: 'input, OPE: 'input> Interpolator<I, OPE>
where
    I: Iterator<Item = &'p PatternElement<'input, PE>>,
    OPE: From<&'p PatternElement<'input, PE>>,
{
    pub fn next_with_collector<R>(&mut self, collector: &mut R) -> Option<OPE>
    where
        R: RangeCollector,
    {
        if let Some(element) = self.pattern.next() {
            collector.populate_collector();
            Some(element.into())
        } else {
            None
        }
    }
}
