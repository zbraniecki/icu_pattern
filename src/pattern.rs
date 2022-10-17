#[derive(Debug, PartialEq, Eq)]
pub enum PatternElement<E> {
    Element(E),
    Literal(String),
    Placeholder(usize),
}

// pub enum ResolvedPatternElement<E> {
//     Element(E),
//     Literal(String),
// }

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern<E = ()> {
    pub elements: Vec<PatternElement<E>>,
}
