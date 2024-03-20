use enum_rotate::EnumRotate;
use Test::*;

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
enum Test {
    A,
    B,
    C,
}

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
enum Void {}

#[test]
fn test_next_prev() {
    assert_eq!(A.next(), B);
    assert_eq!(B.next(), C);
    assert_eq!(C.next(), A);

    assert_eq!(A.prev(), C);
    assert_eq!(B.prev(), A);
    assert_eq!(C.prev(), B);
}

#[test]
fn test_rotate() {
    let mut x = A;
    assert_eq!(x.rotate_next(), B);
    assert_eq!(x, B);

    let mut x = B;
    assert_eq!(x.rotate_next(), C);
    assert_eq!(x, C);

    let mut x = C;
    assert_eq!(x.rotate_next(), A);
    assert_eq!(x, A);

    let mut x = A;
    assert_eq!(x.rotate_prev(), C);
    assert_eq!(x, C);

    let mut x = B;
    assert_eq!(x.rotate_prev(), A);
    assert_eq!(x, A);

    let mut x = C;
    assert_eq!(x.rotate_prev(), B);
    assert_eq!(x, B);
}

#[test]
fn test_iter() {
    assert_eq!(Test::iter().collect::<Vec<_>>(), vec![A, B, C]);
    assert_eq!(Void::iter().collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_iter_from() {
    assert_eq!(A.iter_from().collect::<Vec<_>>(), vec![A, B, C]);
    assert_eq!(B.iter_from().collect::<Vec<_>>(), vec![B, C, A]);
    assert_eq!(C.iter_from().collect::<Vec<_>>(), vec![C, A, B]);
}
