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

enum ParsedValue {
    UsizeValue(usize),
    CharValue(char),
    ColorValue(CssColor),
}

trait UnpackParsedValue {
    fn unpack(pv: ParsedValue) -> Self;
}

impl UnpackParsedValue for usize {
    fn unpack(pv: ParsedValue) -> usize {
        match pv {
            ParsedValue::UsizeValue(val) => val,
            _ => unimplemented!("put in a good error here TODO"),
        }
    }
}

impl UnpackParsedValue for char {
    fn unpack(pv: ParsedValue) -> char {
        match pv {
            ParsedValue::CharValue(val) => val,
            _ => unimplemented!("put in a good error here TODO"),
        }
    }
}

impl UnpackParsedValue for CssColor {
    fn unpack(pv: ParsedValue) -> CssColor {
        match pv {
            ParsedValue::ColorValue(val) => val,
            _ => unimplemented!("put in a good error here TODO"),
        }
    }
}

type ParserType = dyn Sync + (Fn(&str) -> Result<ParsedValue>);
struct AttributeSpec {
    name: &'static str,
    default_value: &'static str,
    parser: &'static ParserType,
}

static ROWS_ATTR_SPEC: AttributeSpec = AttributeSpec {
    name: "rows",
    default_value: "0",
    parser: &|s| Ok(ParsedValue::UsizeValue(s.parse()?)),
};

static COLS_ATTR_SPEC: AttributeSpec = AttributeSpec {
    name: "columns",
    default_value: "0",
    parser: &|s| Ok(ParsedValue::UsizeValue(s.parse()?)),
};

static KNIT_ATTR_SPEC: AttributeSpec = AttributeSpec {
    name: "knit",
    default_value: ".",
    parser: &|s| Ok(ParsedValue::CharValue(parse_char_name(s)?))
};

static PURL_ATTR_SPEC: AttributeSpec = AttributeSpec {
    name: "purl",
    default_value: "X",
    parser: &|s| Ok(ParsedValue::CharValue(parse_char_name(s)?))
};

static EMPTY_ATTR_SPEC: AttributeSpec = AttributeSpec {
    name: "empty",
    default_value: "SPACE",
    parser: &|s| Ok(ParsedValue::CharValue(parse_char_name(s)?))
};

static BACKGROUND_COLOR_ATTR_SPEC: AttributeSpec = AttributeSpec {
    name: "backgroundcolor",
    default_value: "whitesmoke",
    parser: &|s| Ok(ParsedValue::ColorValue(s.parse()?)),
};

impl AttributeSpec {
    fn parsed_default<T>(&'static self) -> T where T: UnpackParsedValue {
        // unwrap: safe because default values should parse or it's a programmer error.
        UnpackParsedValue::unpack((self.parser)(self.default_value).unwrap())
    }


//    fn insert(&'static self, map: &mut HashMap<&'static str, &'static AttributeSpec>) {
//        map.insert(self.name, self);
//    }
}
//
//lazy_static! {
//    static ref ATTRIBUTE_MAP: HashMap<&'static str, &'static AttributeSpec> = {
//        let mut map = HashMap::new();
//        ROWS_ATTR_SPEC.insert(&mut map);
//        COLS_ATTR_SPEC.insert(&mut map);
//        KNIT_ATTR_SPEC.insert(&mut map);
//        PURL_ATTR_SPEC.insert(&mut map);
//        EMPTY_ATTR_SPEC.insert(&mut map);
//        map
//    };
//}

fn parse_char_name(s: &str) -> Result<char> {
    if s.is_empty() {
        // TODO: Get the line number in here.
        return Err(ErrorKind::InvalidCharName.into());
    }

    // Special values
    match s.to_ascii_uppercase().as_str() {
        "SPACE" => return Ok(' '),
        _ => { /* fall through */ }
    }

    if s.len() > 1 {
        // TODO: Get the line number in here and the failing value.
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
        // TODO: Ugh. Replace this with something more elegant, like the map approach.
        match name {
            _ if name == ROWS_ATTR_SPEC.name =>
                    Ok(self.rows = UnpackParsedValue::unpack((ROWS_ATTR_SPEC.parser)(value)?)),
            _ if name == COLS_ATTR_SPEC.name =>
                Ok(self.cols = UnpackParsedValue::unpack((COLS_ATTR_SPEC.parser)(value)?)),
            _ if name == KNIT_ATTR_SPEC.name =>
                Ok(self.knit_char = UnpackParsedValue::unpack((KNIT_ATTR_SPEC.parser)(value)?)),
            _ if name == PURL_ATTR_SPEC.name =>
                Ok(self.purl_char = UnpackParsedValue::unpack((PURL_ATTR_SPEC.parser)(value)?)),
            _ if name == EMPTY_ATTR_SPEC.name =>
                Ok(self.empty_char = UnpackParsedValue::unpack((EMPTY_ATTR_SPEC.parser)(value)?)),
            _ if name == BACKGROUND_COLOR_ATTR_SPEC.name =>
                Ok(self.background_color = UnpackParsedValue::unpack((BACKGROUND_COLOR_ATTR_SPEC.parser)(value)?)),
            _ => unimplemented!("TODO put a decent error here."),
        }
//        let spec = ATTRIBUTE_MAP.get(name);
//        match spec {
//            None => Err(ErrorKind::UnknownAttrName(name.into()).into()),
//            Some(s) => (s.setter)(&mut self, value)
//        }
    }
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            rows: ROWS_ATTR_SPEC.parsed_default(),
            cols: COLS_ATTR_SPEC.parsed_default(),
            knit_char: KNIT_ATTR_SPEC.parsed_default(),
            purl_char: PURL_ATTR_SPEC.parsed_default(),
            empty_char: EMPTY_ATTR_SPEC.parsed_default(),
            background_color: BACKGROUND_COLOR_ATTR_SPEC.parsed_default(),
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

        assert_eq!(0, attrs.rows);
        assert_eq!(0, attrs.cols);
        assert_eq!('.', attrs.knit_char);
        assert_eq!('X', attrs.purl_char);
        assert_eq!(' ', attrs.empty_char);
        assert_eq!(
            CssColor::from_str("whitesmoke").unwrap(),
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
        let hdr = Header::new(&mut BufReader::new(header_str.as_bytes())).unwrap();
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
