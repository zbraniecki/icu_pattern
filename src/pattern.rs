use crate::ranges::RangeList;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Never;

impl TryFrom<char> for Never {
    type Error = ();

    fn try_from(_: char) -> Result<Self, Self::Error> {
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternElement<'input, E = Never> {
    Element(E),
    Literal(Cow<'input, str>),
    Placeholder(usize),
}

pub trait Pattern<'output, E = Never> {
    type OutputElement: 'output;
    type Provider;
    type Iter: Iterator<Item = Self::OutputElement>;
    type Scheme;
    type OutputRole;

    fn interpolate(
        &'output self,
        provider: &'output Self::Provider,
        scheme: Option<Self::Scheme>,
        ranges: Option<&'output mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter;
}
