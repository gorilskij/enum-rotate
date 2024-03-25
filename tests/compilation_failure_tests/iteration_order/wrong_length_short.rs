use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, B)]
enum Enum {
    A, B, C
}
