pub trait OutputElement {}

pub trait Output<E> {
    fn push_element(&mut self, element: E);
}
