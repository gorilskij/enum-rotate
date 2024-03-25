#[test]
fn test_compilation_failure_non_enums() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compilation_failure_tests/non_enum/*.rs");
}

#[test]
fn test_compilation_failure_non_unit_variants() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compilation_failure_tests/non_unit_variant/*.rs");
}

#[test]
fn test_compilation_failure_iteration_order() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compilation_failure_tests/iteration_order/*.rs");
}
