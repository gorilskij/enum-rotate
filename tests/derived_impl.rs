use enum_rotate::EnumRotate;
use Enum::*;

mod utils;

#[derive(EnumRotate, Debug)]
enum Enum {
    A,
    B,
    C,
}

test_prev_next!(A, B);
test_prev_next!(B, C);
test_prev_next!(C, A);

test_iter_iter_from! {
    Enum;
    [A, B, C];
    A: [A, B, C];
    B: [B, C, A];
    C: [C, A, B];
}
