use std::default::Default;

use css_color_parser::Color as CssColor;
use stringstruct::StringStruct;

use crate::errors::*;
use crate::header::Header;

// Notes on new attributes
//   cellsize = dimensions of cell (square)
//   background = color of the background
//   gridcolor = color of the grid
//   knitcolor = color of the knit marker
//   purlcolor = color of the purl marker
//   emptycolor = color of the empty marker
//   knitmarker = marker used for knits (DOT, X, BLANK)
//   purlmarker = marker used for purls
//   emptymarker = marker used for empty cells
//       Ideally, we won't draw the empty cells.

// TODO: make this return Result
fn parse_char_name(s: &str) -> char {
    if s.is_empty() {
        // TODO: Get the line number in here.
        //        return Err(ErrorKind::InvalidCharName.into());
        panic!("foo");
    }

    // Special values
    match s.to_ascii_uppercase().as_str() {
        "SPACE" => return ' ', //return Ok(' '),
        _ => { /* fall through */ }
    }

    if s.len() > 1 {
        // TODO: Get the line number in here and the failing value.
        panic!("bar");
//        return Err(ErrorKind::InvalidCharName.into());
    }

    // unwrap: string is not empty, so unwrap will work.
    //    Ok(s.chars().next().unwrap())
    s.chars().next().unwrap()
}

#[derive(Debug, StringStruct)]
pub struct Attributes {
    pub rows: usize,
    pub columns: usize,

    #[ssfield(default=".", parse="parse_char_name")]
    pub knit: char,
    #[ssfield(default="X", parse="parse_char_name")]
    pub purl: char,
    #[ssfield(default="SPACE", parse="parse_char_name")]
    pub empty: char,

    #[ssfield(default="whitesmoke")]
    pub background_color: CssColor,
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
        assert_eq!('.', attrs.knit);
        assert_eq!('X', attrs.purl);
        assert_eq!(' ', attrs.empty);
        assert_eq!(
            CssColor::from_str("whitesmoke").unwrap(),
            attrs.background_color
        );
    }

    #[test]
    fn test_parse_char_name() {
        assert_eq!(' ', parse_char_name(" "));
        assert_eq!('.', parse_char_name("."));

        assert_eq!(' ', parse_char_name("SPACE"));

//        assert!(parse_char_name("").is_err());
//        assert!(parse_char_name("XX").is_err());
    }

    #[test]
    fn test_attributes() {
        let header_str = r#"
rows=32
columns=64
knit=SPACE
purl=X
empty=#
background_color=sienna
"#;
        let hdr = Header::new(&mut BufReader::new(header_str.as_bytes())).unwrap();
        let attrs = Attributes::new(hdr).unwrap();

        assert_eq!(32, attrs.rows);
        assert_eq!(64, attrs.columns);
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
    }
}
