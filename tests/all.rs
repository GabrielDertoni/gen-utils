#[test]
fn all() {
    let t = trybuild::TestCases::new();

    t.pass("tests/parse.rs");
}