use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, B, D)]
enum Enum {
    A, B, C
}
