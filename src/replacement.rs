use crate::pattern::Pattern;
use crate::PatternElement;
use std::collections::HashMap;

pub trait ReplacementProvider<'r> {
    type Key;
    type Collection;

    fn get_replacement(
        &'r self,
        key: &Self::Key,
    ) -> Option<&'r Pattern<Self::Collection, Self>> where Self: Sized;
}

impl<'r, C, R> ReplacementProvider<'r> for Vec<Pattern<'r, C, R>> {
    type Key = usize;
    type Collection = C;

    fn get_replacement(
        &'r self,
        key: &Self::Key,
    ) -> Option<&'r Pattern<Self::Collection, Self>> {
        todo!()
        // let replacement = self.get(*key)?;
        // Some(replacement)
    }
}

// impl<'r, E: 'r> ReplacementProvider<'r> for Vec<PatternElement<E, usize>> {
//     type Element = E;
//     type Key = usize;
//     type Pattern = Vec<&'r PatternElement<Self::Element, usize>>;
//
//     fn get_replacement(&'r self, key: &Self::Key) -> Option<&'r Self::Pattern> {
//         let replacement = self.get(*key)?;
//         Some(vec![replacement])
//         // Some(std::iter::once(replacement))
//     }
// }

// impl<'r, E: 'r> ReplacementProvider<'r, E> for HashMap<String, Vec<E>> {
//     type Key = String;
//     type Iter = std::slice::Iter<'r, E>;
//
//     fn take_replacement(&'r self, input: &String) -> Option<Self::Iter> {
//         let replacement = self.get(input)?;
//         Some(replacement.iter())
//     }
// }
//
// impl<'r, E: 'r> ReplacementProvider<'r, E> for HashMap<String, E> {
//     type Key = String;
//     type Iter = std::iter::Once<&'r E>;
//
//     fn take_replacement(&'r self, input: &String) -> Option<Self::Iter> {
//         let replacement = self.get(input)?;
//         Some(std::iter::once(replacement))
//     }
// }
