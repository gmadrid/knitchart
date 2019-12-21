use css_color_parser::Color as CssColor;
use macros::Attributes;

#[derive(Attributes, Debug)]
pub struct Attributes {
    rows: usize,
    #[attr(default="64")]
    cols: usize,
    #[attr(default="."/*, parse=parse_char_name*/)]
    knit_char: char,  // but uses special parser function
    #[attr(default="whitesmoke")]
    color: CssColor,  // still using s.parse()?
}

fn main() {
    eprintln!("The attributes: {:?}", Attributes::default());
}
