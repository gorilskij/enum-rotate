use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, B, C)]
#[iteration_order(C, B, A)]
enum Enum {
    A, B, C
}
