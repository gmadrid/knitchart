use std::collections::HashMap;
use std::io::BufRead;

use crate::errors::*;

const CHART_STRING: &str = "CHART";

#[derive(Default, Debug)]
pub struct Header {
    attributes: HashMap<String, Line>,
}

#[derive(Clone, Debug)]
pub struct Line {
    line_number: usize,
    name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
enum RawLine {
    BlankLine(usize),
    Comment(usize),
    Single(usize, String),
    Double(usize, String, String),
    HeaderDone,
}

impl Header {
    pub fn new<'a>(reader: &'a mut impl BufRead) -> Result<Header> {
        let mut rdr = HeaderReader::new(reader);

        let mut attributes = HashMap::<String, Line>::new();
        while let Some(raw) = rdr.next_raw_line()? {
            match raw {
                // Store attributes
                RawLine::Double(line_number, name, value) => {
                    attributes.insert(
                        name.clone(),
                        Line {
                            line_number,
                            name,
                            value,
                        },
                    );
                }
                // Ignore blank lines and comments.
                RawLine::BlankLine(_) | RawLine::Comment(_) => continue,
                RawLine::Single(line_number, _) => {
                    return Err(ErrorKind::BadHeaderLine(line_number).into());
                }
                RawLine::HeaderDone => break,
            }
        }

        Ok(Header { attributes })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Line)> {
        self.attributes.iter()
    }

    pub fn value(&self, name: &str) -> Option<&str> {
        if let Some(line) = self.attributes.get(name) {
            Some(&line.value)
        } else {
            None
        }
    }

    fn num_attributes(&self) -> usize {
        self.attributes.len()
    }
}

struct HeaderReader<'a, R> {
    // The number of the line we are about to read. 1-indexed.
    current_line_number: usize,
    reader: &'a mut R,
}

impl<'a, R> HeaderReader<'a, R>
where
    R: BufRead,
{
    fn new(reader: &mut R) -> HeaderReader<R> {
        HeaderReader {
            current_line_number: 1,
            reader,
        }
    }

    // Returns None when the header is done.
    fn next_raw_line(&mut self) -> Result<Option<RawLine>> {
        let mut buf = String::new();

        match self.reader.read_line(&mut buf)? {
            0 => Ok(None),
            _ => Ok(Some(self.parse_attribute(&buf)?)),
        }
    }

    fn parse_attribute(&mut self, buf: &str) -> Result<RawLine> {
        // Comments MUST be at the beginning of the line.
        if buf.starts_with("//") {
            self.current_line_number += 1;
            return Ok(RawLine::Comment(self.current_line_number - 1));
        }

        if buf.trim() == CHART_STRING {
            self.current_line_number += 1;
            return Ok(RawLine::HeaderDone);
        }

        let mut splits = buf.trim().splitn(2, '=');
        let name_maybe = splits.next();
        if let None = name_maybe {
            self.current_line_number += 1;
            return Ok(RawLine::BlankLine(self.current_line_number - 1));
        }

        let value_maybe = splits.next();
        if let None = value_maybe {
            // unwrap: name_maybe checked above.
            let name = name_maybe.unwrap();
            self.current_line_number += 1;
            if name.is_empty() {
                return Ok(RawLine::BlankLine(self.current_line_number - 1));
            } else {
                return Ok(RawLine::Single(
                    self.current_line_number - 1,
                    name_maybe.unwrap().into(),
                ));
            }
        }

        // Check for valid idents.

        self.current_line_number += 1;
        Ok(RawLine::Double(
            self.current_line_number - 1,
            self.check_ident(name_maybe.unwrap())?,
            value_maybe.unwrap().into(),
        ))
    }

    fn check_ident(&self, ident: &str) -> Result<String> {
        if ident.is_empty() {
            return Err(ErrorKind::MissingIdent(self.current_line_number).into());
        }

        let mut iter = ident.chars();
        if !iter.next().unwrap().is_ascii_alphabetic() {
            return Err(ErrorKind::IdentInitialNotAlpha(self.current_line_number).into());
        }

        for ch in iter {
            if !ch.is_ascii_alphanumeric() {
                return Err(ErrorKind::IdentInvalidChar(self.current_line_number).into());
            }
        }

        Ok(ident.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::BufReader;
    
    #[test]
    fn header_test() {
        let s = "foo=bar\nquux=bam\nblimey=swear";
        let hdr = Header::new(&mut BufReader::new(s.as_bytes())).unwrap();

        assert_eq!(3, hdr.num_attributes());
        assert_eq!("bar", hdr.value("foo").unwrap());
        assert_eq!("bam", hdr.value("quux").unwrap());
        assert_eq!("swear", hdr.value("blimey").unwrap());

        assert_eq!(None, hdr.value("notthere"));
    }

    #[test]
    fn header_done() {
        let s = "foo=bar\nCHART\nquux=quiggly";
        let mut rdr = BufReader::new(s.as_bytes());
        let hdr = Header::new(&mut rdr).unwrap();

        assert_eq!(1, hdr.num_attributes());
        assert_eq!("bar", hdr.value("foo").unwrap());
        assert!(hdr.value("quux").is_none());

        let mut next_line = String::new();
        rdr.read_line(&mut next_line).unwrap();
        assert_eq!("quux=quiggly", next_line);
    }

    #[test]
    fn hr_new_test() {
        let mut brdr = BufReader::new("test".as_bytes());
        let rdr = HeaderReader::new(&mut brdr);
        assert_eq!(1, rdr.current_line_number);
    }

    #[test]
    fn check_ident_test() {
        let mut brdr = BufReader::new("".as_bytes());
        let rdr = HeaderReader::new(&mut brdr);
        assert_eq!("foo", rdr.check_ident("foo").unwrap());
        assert_eq!("Foo", rdr.check_ident("Foo").unwrap());
        assert_eq!("FOO", rdr.check_ident("FOO").unwrap());
        assert_eq!("foo3", rdr.check_ident("foo3").unwrap());
        assert!(rdr.check_ident("4foo").is_err());
        assert!(rdr.check_ident("foo#").is_err());
    }

    #[test]
    fn hr_no_input() {
        let mut brdr = BufReader::new("".as_bytes());
        let mut rdr = HeaderReader::new(&mut brdr);
        assert_eq!(None, rdr.next_raw_line().unwrap());
    }

    #[test]
    fn hr_empty_line_test() {
        let mut brdr = BufReader::new("\n\n\n".as_bytes());
        let mut rdr = HeaderReader::new(&mut brdr);

        assert_eq!(Some(RawLine::BlankLine(1)), rdr.next_raw_line().unwrap());
        assert_eq!(Some(RawLine::BlankLine(2)), rdr.next_raw_line().unwrap());
        assert_eq!(Some(RawLine::BlankLine(3)), rdr.next_raw_line().unwrap());
        assert_eq!(None, rdr.next_raw_line().unwrap());
    }
}
