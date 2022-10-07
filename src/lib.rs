use std::{fmt::Debug, str::FromStr};

mod pattern;
mod replacement;
use replacement::ReplacementProvider;

#[derive(Debug, PartialEq)]
pub enum InterpolatedElement<'e, E> {
    Element(&'e E),
    Literal(&'e bool),
}

// type PatternElement2<E> = InterpolatedElement<Either<E, usize>, bool>;

#[derive(Debug, PartialEq)]
pub enum PatternElement<E, K> {
    Element(E),
    Literal(bool),
    Placeholder(K),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Pattern;

    #[test]
    fn owned() {
        let pattern: Vec<PatternElement<usize, usize>> = vec![
            PatternElement::Element(0),
            PatternElement::Element(1),
            PatternElement::Placeholder(0),
        ];
        let replacements: Vec<Vec<PatternElement<usize, usize>>> =
            vec![vec![PatternElement::Element(2)]];

        let result = pattern.interpolate(&replacements);
        assert_eq!(
            result.collect::<Vec<_>>(),
            vec![
                &PatternElement::Element(0),
                &PatternElement::Element(1),
                &PatternElement::Element(2),
            ]
        );
    }

    #[test]
    fn slice() {
        let pattern: &[PatternElement<usize, usize>] = &[
            PatternElement::Placeholder(0),
            PatternElement::Element(1),
            PatternElement::Element(2),
        ];
        let replacements = vec![vec![PatternElement::Element(0)]];
        let result = pattern.interpolate(&replacements);
        assert_eq!(
            result.collect::<Vec<_>>(),
            vec![
                &PatternElement::Element(0),
                &PatternElement::Element(1),
                &PatternElement::Element(2),
            ]
        );
    }
}
