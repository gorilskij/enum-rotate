extern crate self as enum_rotate;

use delegate::delegate;
pub use derive_enum_rotate::EnumRotate;
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::vec;

pub struct Iter<E>(vec::IntoIter<E>);

impl<E> Iter<E> {
    pub fn new(vec: Vec<E>) -> Self {
        Self(vec.into_iter())
    }
}

impl<E> Iterator for Iter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    delegate! {
        to self.0 {
            fn size_hint(&self) -> (usize, Option<usize>);
            fn all<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool;
            fn any<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool;
            fn cmp<I>(self, other: I) -> Ordering where I: IntoIterator<Item=Self::Item>, Self::Item: Ord, Self: Sized;
            fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized;
            fn count(self) -> usize where Self: Sized;
            fn eq<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<I::Item>, Self: Sized;
            fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool;
            fn find_map<B, F>(&mut self, f: F) -> Option<B> where Self: Sized, F: FnMut(Self::Item) -> Option<B>;
            fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized, F: FnMut(B, Self::Item) -> B;
            fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item);
            fn ge<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<I::Item>, Self: Sized;
            fn gt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<I::Item>, Self: Sized;
            fn last(self) -> Option<Self::Item> where Self: Sized;
            fn le<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<I::Item>, Self: Sized;
            fn lt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<I::Item>, Self: Sized;
            fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord;
            fn max_by<F>(self, compare: F) -> Option<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering;
            fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item> where Self: Sized, F: FnMut(&Self::Item) -> B;
            fn min(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord;
            fn min_by<F>(self, compare: F) -> Option<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering;
            fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item> where Self: Sized, F: FnMut(&Self::Item) -> B;
            fn ne<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<I::Item>, Self: Sized;
            fn nth(&mut self, n: usize) -> Option<Self::Item>;
            fn partial_cmp<I>(self, other: I) -> Option<Ordering> where I: IntoIterator, Self::Item: PartialOrd<I::Item>, Self: Sized;
            fn partition<B, F>(self, f: F) -> (B, B) where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool;
            fn position<P>(&mut self, predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool;
            fn product<P>(self) -> P where Self: Sized, P: Product<Self::Item>;
            fn reduce<F>(self, f: F) -> Option<Self::Item> where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item;
            fn sum<S>(self) -> S where Self: Sized, S: Sum<Self::Item>;
        }
    }

    // for some reason, E == Self::Item cannot be resolved
    // fn rposition<P>(&mut self, predicate: P) -> Option<usize> where P: FnMut(Self::Item) -> bool, Self: Sized + ExactSizeIterator + DoubleEndedIterator {
    //     self.0.rposition(predicate)
    // }
}

impl<E> ExactSizeIterator for Iter<E> {}

impl<E> DoubleEndedIterator for Iter<E> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

pub trait EnumRotate
where
    Self: Sized + Copy,
{
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
