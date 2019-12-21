use css_color_parser::Color as CssColor;
use macros::Attributes;

#[derive(Attributes, Debug)]
pub struct Attributes {
    rows: usize,
    #[attr(default = "64")]
    cols: usize,
    #[attr(default = ".", parse = "parse_char_name")]
    knit_char: char, // but uses special parser function
    #[attr(default = "whitesmoke")]
    color: CssColor,
}

fn parse_char_name(s: &str) -> char {
    if s == "SPACE" {
        ' '
    } else {
        s.parse().unwrap()
    }
}

fn main() {
    let mut attrs = Attributes::default();

    attrs.set_value("rows", "18");
    attrs.set_value("knit_char", "SPACE");

    eprintln!("The attributes: {:?}", attrs);
}
