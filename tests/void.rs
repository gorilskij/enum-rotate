use enum_rotate::EnumRotate;

mod utils;

#[derive(EnumRotate, Debug)]
enum Void {}

// #[test]
// fn test_iter() {
//     assert!(Void::iter().collect::<Vec<_>>().is_empty());
// }

test_iter_iter_from! {
    Void;
    [];
}
