use crate::ranges::RangeList;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Never;

impl TryFrom<char> for Never {
    type Error = ();

    fn try_from(_: char) -> Result<Self, Self::Error> {
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternElement<E = Never> {
    Element(E),
    Literal(String),
    Placeholder(usize),
}

pub trait Pattern<'output, E = Never> {
    type OutputElement: 'output;
    type Provider;
    type Iter: Iterator<Item = Self::OutputElement>;
    type Scheme;
    type OutputRole;

    fn resolve(
        &'output self,
        provider: &'output Self::Provider,
        scheme: Option<Self::Scheme>,
        ranges: Option<&'output mut RangeList<Self::OutputRole>>,
    ) -> Self::Iter;
}
