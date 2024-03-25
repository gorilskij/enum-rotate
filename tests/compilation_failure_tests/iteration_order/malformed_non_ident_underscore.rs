use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(_)]
enum Enum {
    A, B, C
}
