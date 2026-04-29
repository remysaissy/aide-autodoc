#[test]
fn trybuild_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/pass/*.rs");
}
