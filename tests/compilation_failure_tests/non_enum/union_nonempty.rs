use derive_enum_rotate::EnumRotate;

#[derive(EnumRotate)]
union Union {
    field1: usize,
    field2: u8,
}
