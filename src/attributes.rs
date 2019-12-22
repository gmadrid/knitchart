use std::default::Default;

use css_color_parser::Color as CssColor;
use stringstruct::StringStruct;

use crate::errors::*;
use crate::header::Header;

// Notes on new attributes
// TODO: In order to do all of these colors, StringStruct has
//       to support Option<CssColor>
//   knitcolor = color of the knit marker
//   purlcolor = color of the purl marker
//   emptycolor = color of the empty marker
//   knitmarker = marker used for knits (DOT, X, BLANK)
//   purlmarker = marker used for purls
//   emptymarker = marker used for empty cells
//       Ideally, we won't draw the empty cells.

fn parse_char_name(s: &str) -> std::result::Result<char, String> {
    if s.is_empty() {
        // TODO: Get the line number in here.
        return Err("Value for char cannot be empty string.".into());
    }

    // Special values
    match s.to_ascii_uppercase().as_str() {
        "SPACE" => return Ok(' '),
        _ => { /* fall through */ }
    }

    if s.len() > 1 {
        return Err(format!("'{}' is not a valid char name.", s));
    }

    // unwrap: string is not empty, so unwrap will work.
    Ok(s.chars().next().unwrap())
}

#[derive(Debug, StringStruct)]
pub struct Attributes {
    pub rows: usize,
    pub columns: usize,

    #[ssfield(default = "15")]
    pub cell_size: f64,
    #[ssfield(default = "10")]
    pub dot_size: f64,

    #[ssfield(default = ".", parse = "parse_char_name")]
    pub knit: char,
    #[ssfield(default = "X", parse = "parse_char_name")]
    pub purl: char,
    #[ssfield(default = "SPACE", parse = "parse_char_name")]
    pub empty: char,

    // TODO: implement this.
    #[ssfield(default = "15")]
    pub cell_size: u32,

    // TODO: implement this.
    #[ssfield(default = "whitesmoke")]
    pub background_color: CssColor,

    #[ssfield(default = "darkslategray")]
    pub grid_color: CssColor,

    pub in_the_round: bool,
}

impl Attributes {
    pub fn new(hdr: Header) -> Result<Attributes> {
        let mut attrs = Attributes::default();

        for (name, line) in hdr.iter() {
            // TODO: return a Result from set_value.
            attrs.set_value(name, &line.value);
        }

        return Ok(attrs);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::BufReader;
    use std::str::FromStr;

    #[test]
    fn test_default() {
        let attrs = Attributes::default();

        assert_eq!(0, attrs.rows);
        assert_eq!(0, attrs.columns);
        assert_eq!(15.0, attrs.cell_size);
        assert_eq!(10.0, attrs.dot_size);
        assert_eq!('.', attrs.knit);
        assert_eq!('X', attrs.purl);
        assert_eq!(' ', attrs.empty);
        assert_eq!(
            CssColor::from_str("whitesmoke").unwrap(),
            attrs.background_color
        );
        assert_eq!(
            CssColor::from_str("darkslategray").unwrap(),
            attrs.grid_color
        );
        assert_eq!(false, attrs.in_the_round);
    }

    #[test]
    fn test_parse_char_name() {
        assert_eq!(' ', parse_char_name(" ").unwrap());
        assert_eq!('.', parse_char_name(".").unwrap());

        assert_eq!(' ', parse_char_name("SPACE").unwrap());

        assert!(parse_char_name("").is_err());
        assert!(parse_char_name("XX").is_err());
    }

    #[test]
    fn test_attributes() {
        let header_str = r#"
rows=32
columns=64
cell_size=25
dot_size=15.5
knit=SPACE
purl=X
empty=#
background_color=sienna
grid_color=crimson
in_the_round=true
"#;
        let hdr = Header::new(&mut BufReader::new(header_str.as_bytes())).unwrap();
        let attrs = Attributes::new(hdr).unwrap();

        assert_eq!(32, attrs.rows);
        assert_eq!(64, attrs.columns);
        assert_eq!(25.0, attrs.cell_size);
        assert_eq!(15.5, attrs.dot_size);
        assert_eq!(' ', attrs.knit);
        assert_eq!('X', attrs.purl);
        assert_eq!('#', attrs.empty);
        assert_eq!(
            CssColor {
                r: 0xa0,
                g: 0x52,
                b: 0x2d,
                a: 1.0
            },
            attrs.background_color
        );
        assert_eq!(CssColor::from_str("crimson").unwrap(), attrs.grid_color);
        assert_eq!(true, attrs.in_the_round);
    }
}
