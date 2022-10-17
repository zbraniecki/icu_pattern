#[derive(Debug, PartialEq, Eq)]
pub enum PatternElement<E> {
    Element(E),
    Literal(String),
    Placeholder(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern<E = ()> {
    pub elements: Vec<PatternElement<E>>,
}
