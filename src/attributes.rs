use css_color_parser::Color as CssColor;

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
const DEFAULT_ROWS: usize = 0; // 0 means whatever the chart says
const DEFAULT_COLS: usize = 0; // 0 means whatever the chart says

const DEFAULT_KNIT_CHAR: char = '.';
const DEFAULT_PURL_CHAR: char = 'X';
const DEFAULT_EMPTY_CHAR: char = ' ';

const ROWS_ATTR_NAME: &str = "rows";
const COLS_ATTR_NAME: &str = "columns";
const KNIT_ATTR_NAME: &str = "knit";
const PURL_ATTR_NAME: &str = "purl";
const EMPTY_ATTR_NAME: &str = "empty";

const BACKGROUND_ATTR_NAME: &str = "backgroundcolor";
const DEFAULT_BACKGROUND_VALUE: &str = "whitesmoke";

fn parse_char_name(s: &str) -> Result<char> {
    if s.is_empty() {
        // TODO: Get the line number in here.
        return Err(ErrorKind::InvalidCharName.into());
    }

    // Special values
    match s.to_ascii_uppercase().as_str() {
        "SPACE" => return Ok(' '),
        _ => {}
    }

    if s.len() > 1 {
        // TODO: Get the line number in here.
        return Err(ErrorKind::InvalidCharName.into());
    }

    // unwrap: string is not empty, so unwrap will work.
    Ok(s.chars().next().unwrap())
}

#[derive(Debug)]
pub struct Attributes {
    pub rows: usize,
    pub cols: usize,

    pub knit_char: char,
    pub purl_char: char,
    pub empty_char: char,

    pub background_color: CssColor,
}

impl Attributes {
    pub fn new(hdr: Header) -> Result<Attributes> {
        let mut attrs = Attributes::default();

        for (name, line) in hdr.iter() {
            attrs.set_value_with_name(name, &line.value)?;
        }

        return Ok(attrs);
    }

    fn set_value_with_name(&mut self, name: &str, value: &str) -> Result<()> {
        match name {
            ROWS_ATTR_NAME => self.rows = value.parse()?,
            COLS_ATTR_NAME => self.cols = value.parse()?,
            KNIT_ATTR_NAME => self.knit_char = parse_char_name(value)?,
            PURL_ATTR_NAME => self.purl_char = parse_char_name(value)?,
            EMPTY_ATTR_NAME => self.empty_char = parse_char_name(value)?,
            BACKGROUND_ATTR_NAME => self.background_color = value.parse()?,
            // TODO: line number in this error.
            _ => return Err(ErrorKind::UnknownAttrName(name.into()).into()),
        }
        Ok(())
    }
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            rows: DEFAULT_ROWS,
            cols: DEFAULT_COLS,
            knit_char: DEFAULT_KNIT_CHAR,
            purl_char: DEFAULT_PURL_CHAR,
            empty_char: DEFAULT_EMPTY_CHAR,
            background_color: DEFAULT_BACKGROUND_VALUE.parse().unwrap(),
        }
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

        assert_eq!(DEFAULT_ROWS, attrs.rows);
        assert_eq!(DEFAULT_COLS, attrs.cols);
        assert_eq!(DEFAULT_KNIT_CHAR, attrs.knit_char);
        assert_eq!(DEFAULT_PURL_CHAR, attrs.purl_char);
        assert_eq!(DEFAULT_EMPTY_CHAR, attrs.empty_char);
        assert_eq!(
            CssColor::from_str(DEFAULT_BACKGROUND_VALUE).unwrap(),
            attrs.background_color
        );
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
knit=SPACE
purl=X
empty=#
backgroundcolor=sienna
"#;
        let hdr = dbg!(Header::new(&mut BufReader::new(header_str.as_bytes()))).unwrap();
        let attrs = Attributes::new(hdr).unwrap();

        assert_eq!(32, attrs.rows);
        assert_eq!(64, attrs.cols);
        assert_eq!(' ', attrs.knit_char);
        assert_eq!('X', attrs.purl_char);
        assert_eq!('#', attrs.empty_char);
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
