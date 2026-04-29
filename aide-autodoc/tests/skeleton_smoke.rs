// Trivial smoke test: the macro is exposed and applying it to an empty fn
// produces compilable code. Stronger tests land in PR #8.
#[aide_autodoc::aide_autodoc]
fn _smoke() {}

#[test]
fn skeleton_compiles() {}
