use std::{fmt::Debug, str::FromStr};

mod pattern;
mod replacement;
// use replacement::ReplacementProvider;

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
        let replacements2 = vec![Pattern {
            elements: vec![PatternElement::Element::<_, usize>(3_usize)],
            replacements: None,
        }];
        let replacements =
            vec![Pattern {
                elements: vec![PatternElement::Element::<_, usize>(2_usize)],
                replacements: Some(&replacements2),
            }];
        let pattern = Pattern {
            elements: vec![
                PatternElement::Element(0),
                PatternElement::Element(1),
                PatternElement::Placeholder(0),
            ],
            replacements: Some(&replacements),
        };
        let result = pattern.interpolate();
        // assert_eq!(
        //     result.collect::<Vec<_>>(),
        //     vec![
        //         &PatternElement::Element(0),
        //         &PatternElement::Element(1),
        //         &PatternElement::Element(2),
        //     ]
        // );
    }
    //
    // #[test]
    // fn slice() {
    //     let pattern: &[PatternElement<usize, usize>] = &[
    //         PatternElement::Placeholder(0),
    //         PatternElement::Element(1),
    //         PatternElement::Element(2),
    //     ];
    //     let replacements = vec![vec![PatternElement::Element(0)]];
    //     let result = pattern.interpolate(&replacements);
    //     assert_eq!(
    //         result.collect::<Vec<_>>(),
    //         vec![
    //             &PatternElement::Element(0),
    //             &PatternElement::Element(1),
    //             &PatternElement::Element(2),
    //         ]
    //     );
    // }
}
