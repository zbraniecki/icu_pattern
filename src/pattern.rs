use crate::replacement::ReplacementProvider;
use crate::PatternElement;

pub trait Pattern<'e> {
    type Element;
    type Key;
    type Iterator: Iterator<Item = &'e PatternElement<Self::Element, Self::Key>>
    where
        Self::Key: 'e,
        Self::Element: 'e;

    fn iterate(&'e self) -> Self::Iterator;
    fn interpolate<R>(
        &'e self,
        replacements: &'e R,
    ) -> PatternIterator<'e, Self, R, Self::Element, Self::Key>
    where
        R: ReplacementProvider<'e, Element = Self::Element, Key = Self::Key>,
        Self: Sized,
        R::Pattern: Pattern<'e>;
}

// fn m<E, K, R>(i: &PatternElement<E, K>, replacements: R) -> &PatternElement<E, K> {
//     if let PatternElement::Placeholder(p) = i {
//         i
//     } else {
//         i
//     }
// }

impl<'e, E: 'e, K: 'e> Pattern<'e> for Vec<PatternElement<E, K>> {
    type Element = E;
    type Key = K;
    type Iterator = std::slice::Iter<'e, PatternElement<E, K>>;

    fn iterate(&'e self) -> Self::Iterator {
        self.iter()
    }

    fn interpolate<R>(
        &'e self,
        replacements: &'e R,
    ) -> PatternIterator<'e, Self, R, Self::Element, Self::Key>
    where
        R: ReplacementProvider<'e, Element = Self::Element, Key = Self::Key>,
        Self: Sized,
        R::Pattern: Pattern<'e>,
    {
        PatternIterator {
            pattern: self.iterate(),
            replacements,
            current_pattern: None,
        }
    }
}

pub struct PatternIterator<'e, P, R, E: 'e, K: 'e>
where
    P: Pattern<'e, Element = E, Key = K>,
    R: ReplacementProvider<'e, Element = E, Key = K>,
    R::Pattern: Pattern<'e>,
{
    pattern: <P as Pattern<'e>>::Iterator,
    replacements: &'e R,
    current_pattern: Option<<R::Pattern as Pattern<'e>>::Iterator>,
}

impl<'e, P, R, E, K> Iterator for PatternIterator<'e, P, R, E, K>
where
    P: Pattern<'e, Element = E, Key = K>,
    R: ReplacementProvider<'e, Element = E, Key = K>,
    R::Pattern: Pattern<'e, Element = E, Key = K>,
{
    type Item = &'e PatternElement<E, K>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(element) = self.pattern.next() {
            if let PatternElement::Placeholder(p) = element {
                let r = self.replacements.get_replacement(p).unwrap();
                let mut i = r.iterate();
                if let Some(e) = i.next() {
                    self.current_pattern = Some(i);
                    return Some(e);
                }
                self.next()
            } else {
                Some(element)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p: Vec<PatternElement<usize, usize>> =
            vec![PatternElement::Element(0), PatternElement::Element(1)];

        let result = p.iterate();
        assert_eq!(
            result.collect::<Vec<_>>(),
            vec![&PatternElement::Element(0), &PatternElement::Element(1),]
        );
    }
}
