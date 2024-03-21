use enum_rotate::EnumRotate;

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
enum Void {}

#[test]
fn test_iter() {
    assert_eq!(Void::iter().collect::<Vec<_>>(), vec![]);
}
