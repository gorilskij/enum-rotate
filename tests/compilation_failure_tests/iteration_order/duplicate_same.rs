use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, B, C)]
#[iteration_order(A, B, C)]
enum Enum {
    A, B, C
}
