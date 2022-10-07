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

// pub struct Pattern<E, C, R> {
//     pub list: C,
//     pub replacements: R,
//     pub ph: std::marker::PhantomData<E>,
// }
//
// impl<'e, E: 'e, C, R> Pattern<E, C, R> {
//     pub fn new(list: C, replacements: R) -> Self {
//         Self {
//             list,
//             replacements,
//             ph: Default::default(),
//         }
//     }
//
//     pub fn interpolate<K: 'e>(&'e self) -> Vec<InterpolatedElement<E>>
//     where
//         C: PatternElementIntoIterator<'e, E, K>,
//         R: ReplacementProvider<'e, E, Key = K>,
//         K: Debug + FromStr + PartialEq + Clone,
//         <K as FromStr>::Err: Debug + PartialEq,
//     {
//         let mut result = vec![];
//         let iter = self.list.to_iter();
//
//         for e in iter {
//             match e {
//                 PatternElement::Element(e) => result.push(InterpolatedElement::Element(e)),
//                 PatternElement::Literal(l) => result.push(InterpolatedElement::Literal(l)),
//                 PatternElement::Placeholder(p) => {
//                     let repl = self.replacements.take_replacement(p).unwrap();
//                     // result.extend(repl);
//                 }
//             }
//         }
//         result
//     }
// }
//
// pub trait PatternElementIntoIterator<'e, E: 'e, K: 'e> {
//     type IntoIter: Iterator<Item = &'e PatternElement<E, K>>;
//     fn to_iter(&'e self) -> Self::IntoIter;
// }
//
// impl<'e, E: 'e, K: 'e> PatternElementIntoIterator<'e, E, K> for Vec<PatternElement<E, K>> {
//     type IntoIter = std::slice::Iter<'e, PatternElement<E, K>>;
//
//     fn to_iter(&'e self) -> Self::IntoIter {
//         self.iter()
//     }
// }
//
// impl<'e, E: 'e, K: 'e> PatternElementIntoIterator<'e, E, K> for &'e Vec<PatternElement<E, K>> {
//     type IntoIter = std::slice::Iter<'e, PatternElement<E, K>>;
//
//     fn to_iter(&'e self) -> Self::IntoIter {
//         self.iter()
//     }
// }
//
// impl<'e, E: 'e, K: 'e> PatternElementIntoIterator<'e, E, K> for &'e [PatternElement<E, K>] {
//     type IntoIter = std::slice::Iter<'e, PatternElement<E, K>>;
//
//     fn to_iter(&'e self) -> Self::IntoIter {
//         self.iter()
//     }
// }

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

    // #[test]
    // fn borowed_vec() {
    //     let input = vec![
    //         PatternElement::Placeholder(0),
    //         PatternElement::Element(1),
    //         PatternElement::Element(2),
    //     ];
    //     let p1 = Pattern::new(&input, vec![]);
    //     // let p1 = Pattern {
    //     //     list: vec![
    //     //     ],
    //     //     replacements: vec![
    //     //         Pattern {
    //     //             list: vec![
    //     //                 PatternElement::Element(0),
    //     //             ],
    //     //             replacements: vec![],
    //     //         }
    //     //     ],
    //     // };
    //     let result = p1.interpolate();
    //     assert_eq!(
    //         result,
    //         vec![
    //             InterpolatedElement::Element(&1),
    //             InterpolatedElement::Element(&2),
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn slice() {
    //     let input = &[
    //         PatternElement::Placeholder(0),
    //         PatternElement::Element(1),
    //         PatternElement::Element(2),
    //     ];
    //     let p1 = Pattern::new(input.as_slice(), vec![]);
    //     // let p1 = Pattern {
    //     //     list: vec![
    //     //     ],
    //     //     replacements: vec![
    //     //         Pattern {
    //     //             list: vec![
    //     //                 PatternElement::Element(0),
    //     //             ],
    //     //             replacements: vec![],
    //     //         }
    //     //     ],
    //     // };
    //     let result = p1.interpolate();
    //     assert_eq!(
    //         result,
    //         vec![
    //             InterpolatedElement::Element(&1),
    //             InterpolatedElement::Element(&2),
    //         ]
    //     );
    // }
}
