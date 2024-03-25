use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, _, C)]
enum Enum {
    A, B, C
}
