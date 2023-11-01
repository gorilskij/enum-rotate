extern crate self as enum_rotate;

pub use derive_enum_rotate::RotateEnum;

pub trait RotateEnum where Self: Sized + Copy {
    fn next(self) -> Self;

    fn prev(self) -> Self;

    fn rotate_next(&mut self) -> Self {
        *self = self.next();
        *self
    }

    fn rotate_prev(&mut self) -> Self {
        *self = self.prev();
        *self
    }
}
