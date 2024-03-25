use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, B, C, A)]
enum Enum {
    A, B, C
}
