use css_color_parser::Color as CssColor;
use stringstruct::StringStruct;

#[derive(StringStruct, Debug)]
pub struct Attributes {
    rows: usize,
    #[ssfield(default = "64")]
    cols: usize,
    #[ssfield(default = ".", parse = "parse_char_name")]
    knit_char: char, // but uses special parser function
    #[ssfield(default = "whitesmoke")]
    color: CssColor,
}

fn parse_char_name(s: &str) -> std::result::Result<char, String> {
    if s == "SPACE" {
        Ok(' ')
    } else {
        Ok(s.chars().next().unwrap())
    }
}

fn main() {
    let mut attrs = Attributes::default();

    attrs.set_value("rows", "18");
    attrs.set_value("knit_char", "SPACE");

    eprintln!("The attributes: {:?}", attrs);
}
