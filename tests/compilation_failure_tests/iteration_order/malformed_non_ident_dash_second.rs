use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(A, -, C)]
enum Enum {
    A, B, C
}
