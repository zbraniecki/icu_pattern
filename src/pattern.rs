use std::borrow::Cow;

use crate::interpolator::Interpolator;

#[derive(Debug, PartialEq, Eq)]
pub enum PatternElement<'input, PE> {
    Literal(Cow<'input, str>),
    Placeholder(PE),
}

pub trait Pattern<'output>: Iterator<Item = Self::PatternElement> {
    type PatternElement;
    type OutputPatternElement;

    fn interpolate(
        &self,
    ) -> Interpolator<std::slice::Iter<'_, Self::PatternElement>, Self::OutputPatternElement>;
}
