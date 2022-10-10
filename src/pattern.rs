use crate::replacement::ReplacementProvider;
use crate::PatternElement;

pub trait PatternElementCollection<E, K> {
    type Iterator<'r>: Iterator<Item = &'r PatternElement<E, K>>
    where
        Self: 'r,
        E: 'r,
        K: 'r;

    fn iterate<'r>(&'r self) -> Self::Iterator<'r>;
}

impl<E, K> PatternElementCollection<E, K> for Vec<PatternElement<E, K>> {
    type Iterator<'r> = std::slice::Iter<'r, PatternElement<E, K>>
        where E: 'r, K: 'r;

    fn iterate<'r>(&'r self) -> Self::Iterator<'r> {
        self.iter()
    }
}

pub struct Pattern<'e, C, R> {
    pub elements: C,
    pub replacements: Option<&'e R>,
}

impl<'e, C, R> Pattern<'e, C, R> {
    pub fn new(elements: C, replacements: Option<&'e R>) -> Self {
        Self {
            elements,
            replacements,
        }
    }

    pub fn iterate<E, K>(&'e self) -> C::Iterator<'e>
    where
        C: PatternElementCollection<E, K>,
    {
        self.elements.iterate()
    }

    pub fn interpolate<E, K>(&'e self) -> PatternIterator<'e, C, R, E, K>
    where
        C: PatternElementCollection<E, K>,
        R: ReplacementProvider<'e, Key = K, Collection = C>,
    {
        PatternIterator {
            elements: self.iterate(),
            replacements: self.replacements.clone(),
            current_pattern: None,
        }
    }
}

pub struct PatternIterator<'e, C, R, E: 'e, K: 'e>
where
    C: PatternElementCollection<E, K> + 'e,
    R: ReplacementProvider<'e, Key = K, Collection = C>,
{
    elements: C::Iterator<'e>,
    replacements: Option<&'e R>,
    current_pattern: Option<<R::Collection as PatternElementCollection<E, K>>::Iterator<'e>>,
}

impl<'e, C, R, E, K> Iterator for PatternIterator<'e, C, R, E, K>
where
    C: PatternElementCollection<E, K> + 'e,
    R: ReplacementProvider<'e, Key = K, Collection = C>,
{
    type Item = <C::Iterator<'e> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(element) = self.elements.next() {
            if let PatternElement::Placeholder(p) = element {
                // let r = self.replacements.get_replacement(p).unwrap();
                // let mut i = r.iterate();
                // if let Some(e) = i.next() {
                //     self.current_pattern = Some(i);
                //     return Some(e);
                // }
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
        let replacements = vec![];
        let pattern: Pattern<_, Vec<PatternElement<usize, usize>>> = Pattern {
            elements: vec![
                PatternElement::Element::<_, usize>(0_usize),
                PatternElement::Element::<_, usize>(1_usize),
            ],
            replacements: Some(&replacements),
        };
        let result = pattern.iterate();
        assert_eq!(
            result.collect::<Vec<_>>(),
            vec![&PatternElement::Element(0), &PatternElement::Element(1),]
        );
    }
}
