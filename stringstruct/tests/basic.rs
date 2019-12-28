use stringstruct::StringStruct;

fn add_xxx(s: &str) -> std::result::Result<String, String> {
    Ok(format!("{}XXX", s))
}

#[derive(StringStruct)]
struct SSTest {
    b: bool,
    ew8: u8,
    ew32: u32,

    s: String,

    #[ssfield(default="def")]
    defstr: String,

    #[ssfield(default="defs", parse="add_xxx")]
    defsetter: String,

    #[ssfield(parse="add_xxx")]
    justsetter: String,

    // Also testing reversed attributes.
    #[ssfield(parse="add_xxx", default="")]
    emptydefault: String,
}

fn main() {
    let mut ss = SSTest::default();

    // First, check all of the defaults.
    assert_eq!(false, ss.b);
    assert_eq!(0, ss.ew8);
    assert_eq!(0, ss.ew32);
    assert_eq!("", ss.s);
    assert_eq!("def", ss.defstr);
    assert_eq!("defsXXX", ss.defsetter);
    assert_eq!("", ss.justsetter);
    assert_eq!("XXX", ss.emptydefault);

    // Then make sure we can set values
    ss.set_value("b", "true");
    assert_eq!(true,ss.b);

    ss.set_value("ew8", "63");
    assert_eq!(63, ss.ew8);

    ss.set_value("ew32", "123456");
    assert_eq!(123456, ss.ew32);

    ss.set_value("s", "modified");
    assert_eq!("modified", ss.s);

    ss.set_value("defsetter", "AAA");
    assert_eq!("AAAXXX", ss.defsetter);

    ss.set_value("justsetter", "BBB");
    assert_eq!("BBBXXX", ss.justsetter);
}
