use enum_rotate::EnumRotate;

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
enum Test {
    A, B, C
}

#[derive(EnumRotate, Copy, Clone, Eq, PartialEq, Debug)]
enum Void {}

#[test]
fn test_next_prev() {
    assert_eq!(Test::A.next(), Test::B);
    assert_eq!(Test::B.next(), Test::C);
    assert_eq!(Test::C.next(), Test::A);

    assert_eq!(Test::A.prev(), Test::C);
    assert_eq!(Test::B.prev(), Test::A);
    assert_eq!(Test::C.prev(), Test::B);
}

#[test]
fn test_rotate() {
    let mut x = Test::A;
    assert_eq!(x.rotate_next(), Test::B);
    assert_eq!(x, Test::B);

    let mut x = Test::B;
    assert_eq!(x.rotate_next(), Test::C);
    assert_eq!(x, Test::C);

    let mut x = Test::C;
    assert_eq!(x.rotate_next(), Test::A);
    assert_eq!(x, Test::A);

    let mut x = Test::A;
    assert_eq!(x.rotate_prev(), Test::C);
    assert_eq!(x, Test::C);

    let mut x = Test::B;
    assert_eq!(x.rotate_prev(), Test::A);
    assert_eq!(x, Test::A);

    let mut x = Test::C;
    assert_eq!(x.rotate_prev(), Test::B);
    assert_eq!(x, Test::B);
}

#[test]
fn test_iter() {
    assert_eq!(Test::iter().collect::<Vec<_>>(), vec![Test::A, Test::B, Test::C]);
    assert_eq!(Void::iter().collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_iter_from() {
    assert_eq!(Test::A.iter_from().collect::<Vec<_>>(), vec![Test::A, Test::B, Test::C]);
    assert_eq!(Test::B.iter_from().collect::<Vec<_>>(), vec![Test::B, Test::C, Test::A]);
    assert_eq!(Test::C.iter_from().collect::<Vec<_>>(), vec![Test::C, Test::A, Test::B]);
}
