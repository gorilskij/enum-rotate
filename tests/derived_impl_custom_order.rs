use enum_rotate::EnumRotate;
use Enum::*;

mod utils;

#[derive(EnumRotate, Debug)]
#[iteration_order(B, A, C)]
enum Enum {
    A,
    B,
    C,
}

test_prev_next!(A, C);
test_prev_next!(C, B);
test_prev_next!(B, A);

test_iter_iter_from! {
    Enum;
    [B, A, C];
    A: [A, C, B];
    B: [B, A, C];
    C: [C, B, A];
}
