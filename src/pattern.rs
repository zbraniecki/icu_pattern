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
    fn interpolate<R, P>(
        &'e self,
        replacements: &'e R,
    ) -> PatternIterator<'e, Self, P, R, Self::Element, Self::Key>
    where
        R: ReplacementProvider<'e, P, Element = Self::Element, Key = Self::Key>,
        P: Pattern<'e, Element = Self::Element, Key = Self::Key>,
        Self: Sized;
}

impl<'e, E: 'e, K: 'e> Pattern<'e> for Vec<PatternElement<E, K>> {
    type Element = E;
    type Key = K;
    type Iterator = std::slice::Iter<'e, PatternElement<E, K>>;

    fn iterate(&'e self) -> Self::Iterator {
        self.iter()
    }

    fn interpolate<R, P>(
        &'e self,
        replacements: &'e R,
    ) -> PatternIterator<'e, Self, P, R, Self::Element, Self::Key>
    where
        R: ReplacementProvider<'e, P, Element = Self::Element, Key = Self::Key>,
        P: Pattern<'e, Element = Self::Element, Key = Self::Key>,
        Self: Sized,
    {
        PatternIterator {
            pattern: self.iterate(),
            replacements,
            current_pattern: None,
            marker: Default::default(),
        }
    }
}

impl<'e, E: 'e, K: 'e> Pattern<'e> for &'e [PatternElement<E, K>] {
    type Element = E;
    type Key = K;
    type Iterator = std::slice::Iter<'e, PatternElement<E, K>>;

    fn iterate(&'e self) -> Self::Iterator {
        self.iter()
    }

    fn interpolate<R, P>(
        &'e self,
        replacements: &'e R,
    ) -> PatternIterator<'e, Self, P, R, Self::Element, Self::Key>
    where
        R: ReplacementProvider<'e, P, Element = Self::Element, Key = Self::Key>,
        P: Pattern<'e, Element = Self::Element, Key = Self::Key>,
        Self: Sized,
    {
        PatternIterator {
            pattern: self.iterate(),
            replacements,
            current_pattern: None,
            marker: Default::default(),
        }
    }
}

pub struct PatternIterator<'e, P, P2, R, E: 'e, K: 'e>
where
    P: Pattern<'e, Element = E, Key = K>,
    P2: Pattern<'e, Element = E, Key = K>,
    R: ReplacementProvider<'e, P2, Element = E, Key = K>,
{
    pattern: <P as Pattern<'e>>::Iterator,
    replacements: &'e R,
    current_pattern: Option<P2::Iterator>,
    marker: std::marker::PhantomData<P2>,
}

impl<'e, P, P2, R, E, K> Iterator for PatternIterator<'e, P, P2, R, E, K>
where
    P: Pattern<'e, Element = E, Key = K> + 'e,
    P2: Pattern<'e, Element = E, Key = K> + 'e,
    R: ReplacementProvider<'e, P2, Element = E, Key = K>,
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
