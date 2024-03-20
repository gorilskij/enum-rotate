extern crate self as enum_rotate;

pub use derive_enum_rotate::EnumRotate;
use std::iter::successors;
use std::mem::discriminant;

/// Trait implementing iterator-like behavior for enums.
///
/// Implementing this trait provides the
/// [`next`], [`prev`], [`rotate_next`], [`rotate_prev`], [`iter`], and [`iter_from`] methods on any enum.
///
/// An implementation of this trait describes an *iteration order* of the enum's variants.
/// An iteration order is simply an ordering of **all** the variants of the enum with each variant
/// appearing exactly once. Any given implementation of this trait must ensure that all functions
/// respect the same iteration order.
///
/// This trait should only be implemented for enums, if this trait is implemented for a struct or
/// a union, its requirements are unspecified and the behavior of some of the default implementations
/// is also unspecified.
///
/// Note that this trait currently excludes enums with non-empty (and not effectively unit) tuple or struct
/// variants due to how the [`next`], [`prev`], [`rotate_next`], and [`rotate_prev`] functions are defined.
/// This is because any information carried by a variant would be overridden by calling `.next().prev()`,
/// which would break an invariant. This part of the specification might change in the future.
///
/// It is recommended to implement this trait using the provided derive macro like this:
/// ```rust
/// use enum_rotate::EnumRotate;
///
/// #[derive(EnumRotate, Copy, Clone)]
/// enum Enum { A, B, C }
/// ```
///
/// Here is a brief overview of the functions in this trait:
///
///  - The [`next`] and [`prev`] functions cycle through the variants of the enum.
///
///    The implementation must
///    ensure that
///    - `x.next().prev() == x` is always true
///    - `x.prev().next() == x` is always true
///    - [`next`] and [`prev`] describe the same iteration order as all the other functions
///
///  - The [`rotate_next`] and [`rotate_prev`] functions are in-place versions of [`next`] and [`prev`] respectively.
///    They modify the receiver they operate on and return a copy of the new value.
///
///    The implementation must ensure that
///    - `x.rotate_next(); x.rotate_prev()` will leave `x` unchanged
///    - `x.rotate_prev(); x.rotate_next()` will leave `x` unchanged
///    - [`rotate_next`] and [`rotate_prev`] describe the same iteration order as all the other functions
///
///  - The [`iter`] function returns an iterator over the variants of the enum.
///
///    The implementation must ensure that the iterator returned by [`iter`]
///    - Returns each variant of the enum **exactly once**
///    - Describes the same iteration order as all the other functions
///
///    No guarantees are made about the precise type of the iterator.
///
///  - The [`iter_from`] function returns an iterator over the variants of the enum starting from a given variant.
///    The implementation must ensure that the iterator returned by [`iter_from`]
///    - Returns the variant passed to [`iter_from`] as its first element
///    - Returns each variant of the enum **exactly once**
///    - Describes the same iteration order as all the other functions
///
///     No guarantees are made about the precise type of the iterator
///
/// [`next`]: EnumRotate::next
/// [`prev`]: EnumRotate::prev
/// [`rotate_next`]: EnumRotate::rotate_next
/// [`rotate_prev`]: EnumRotate::rotate_prev
/// [`iter`]: EnumRotate::iter
/// [`iter_from`]: EnumRotate::iter_from
pub trait EnumRotate
where
    Self: Sized + Copy,
{
    /// This method returns the **next** variant in the *iteration order* described by this implementation.
    ///
    /// Example:
    /// ```rust
    /// use enum_rotate::EnumRotate;
    /// use Enum::*;
    ///
    /// #[derive(EnumRotate, Copy, Clone, PartialEq, Debug)]
    /// enum Enum { A, B, C }
    ///
    /// fn main() {
    ///     assert_eq!(A.next(), B)
    /// }
    /// ```
    #[must_use]
    fn next(self) -> Self;

    /// This method returns the **previous** variant in the *iteration order* described by this implementation
    #[must_use]
    fn prev(self) -> Self;

    /// This method assigns the **next** variant in the *iteration order* described by this implementation
    /// to the receiver and returns the new value
    fn rotate_next(&mut self) -> Self {
        *self = self.next();
        *self
    }

    /// This method assigns the **previous** variant in the *iteration order* described by this implementation
    /// to the receiver and returns the new value
    fn rotate_prev(&mut self) -> Self {
        *self = self.prev();
        *self
    }

    // TODO: docs
    fn iter() -> impl Iterator<Item=Self>;

    // TODO: docs
    fn iter_from(self) -> impl Iterator<Item=Self> {
        let self_discriminant = discriminant(&self);
        successors(Some(self), move |var| {
            match var.next() {
                // Compare discriminants because PartialEq is not guaranteed to be implemented.
                // If Self is not an enum, the behavior of this method is unspecified
                next if discriminant(&next) == self_discriminant => None,
                next => Some(next),
            }
        })
    }
}
