use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
enum Enum {
    A,
    B(),
    C,
}
