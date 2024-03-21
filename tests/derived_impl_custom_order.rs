use enum_rotate::EnumRotate;
use Enum::*;

mod utils;

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
#[iteration_order(B, A, C)]
enum Enum {
    A,
    B,
    C,
}

test_prev_next!(A, C);
test_prev_next!(C, B);
test_prev_next!(B, A);

#[test]
fn test_iter() {
    assert_eq!(Enum::iter().collect::<Vec<_>>(), vec![B, A, C]);
}

#[test]
fn test_iter_from() {
    assert_eq!(A.iter_from().collect::<Vec<_>>(), vec![A, C, B]);
    assert_eq!(B.iter_from().collect::<Vec<_>>(), vec![B, A, C]);
    assert_eq!(C.iter_from().collect::<Vec<_>>(), vec![C, B, A]);
}
