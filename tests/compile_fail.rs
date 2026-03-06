#![cfg(feature = "unstable-attr-regex")]

#[test]
fn unbound_named_field() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/unbound_named_field.rs");
}

#[test]
fn missing_named_capture_field() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/missing_named_capture_field.rs");
}

#[test]
fn group_index_out_of_bounds() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/group_index_out_of_bounds.rs");
}

#[test]
fn missing_unnamed_capture_group_binding() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/missing_unnamed_capture_group_binding.rs");
}
