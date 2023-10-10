#[rustversion::stable(1.65)]
#[test]
fn compile_macros() {
    let t = trybuild::TestCases::new();

    t.pass("tests/trybuild/001.rs");
}
