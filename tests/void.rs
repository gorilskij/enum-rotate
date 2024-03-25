use enum_rotate::EnumRotate;

mod utils;

#[derive(EnumRotate, Debug)]
enum Void {}

test_iter_iter_from! {
    Void;
    [];
}

#[derive(EnumRotate)]
#[iteration_order()]
enum Void2 {}
