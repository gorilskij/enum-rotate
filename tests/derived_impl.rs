use enum_rotate::EnumRotate;
use Enum::*;

mod utils;

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
enum Enum {
    A,
    B,
    C,
}

test_prev_next!(A, B);
test_prev_next!(B, C);
test_prev_next!(C, A);

#[test]
fn test_iter() {
    assert_eq!(Enum::iter().collect::<Vec<_>>(), vec![A, B, C]);
}

#[test]
fn test_iter_from() {
    assert_eq!(A.iter_from().collect::<Vec<_>>(), vec![A, B, C]);
    assert_eq!(B.iter_from().collect::<Vec<_>>(), vec![B, C, A]);
    assert_eq!(C.iter_from().collect::<Vec<_>>(), vec![C, A, B]);
}
