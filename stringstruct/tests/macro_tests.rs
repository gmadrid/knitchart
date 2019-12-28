#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic.rs");

    // Things that can go wrong:
    //
    // field type with no default.
    // field type that won't parse.
    
}

fn main() {
}
