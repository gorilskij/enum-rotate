use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
#[iteration_order(-)]
enum Enum {
    A, B, C
}
