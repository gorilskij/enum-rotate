extern crate self as enum_rotate;

pub use derive_enum_rotate::EnumRotate;

pub struct Iter<E>(Vec<E>);

impl<E> Iter<E> {
    pub fn new(vec: Vec<E>) -> Self {
        Self(vec)
    }
}

impl<E> Iterator for Iter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub trait EnumRotate where Self: Sized + Copy {
    #[must_use]
    fn next(self) -> Self;

    #[must_use]
    fn prev(self) -> Self;

    fn rotate_next(&mut self) -> Self {
        *self = self.next();
        *self
    }

    fn rotate_prev(&mut self) -> Self {
        *self = self.prev();
        *self
    }

    fn iter() -> Iter<Self>;

    fn iter_from(self) -> Iter<Self>;
}
