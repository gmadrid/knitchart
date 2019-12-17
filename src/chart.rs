use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::attributes::Attributes;
use crate::errors::*;
use crate::header::Header;

#[derive(Debug)]
pub struct Chart {
    attributes: Attributes,
    stitches: Vec<Vec<Stitch>>,
}

enum Stitch {
    Knit,
    Purl,
    Empty,
}

impl Debug for Stitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Stitch::Knit => ".",
            Stitch::Purl => "*",
            Stitch::Empty => "#",
        };
        
        write!(f, "{}", ch)
    }
}

impl Chart {
    pub fn open(filename: &str) -> Result<Chart> {
        let file = File::open(filename)?;
        let rdr = BufReader::new(file);
        Chart::read(rdr)
    }

    pub fn read(mut rdr: impl BufRead) -> Result<Chart> {
        let hdr = Header::new(&mut rdr)?;
        let attributes = Attributes::new(hdr)?;

        let mut lines: Vec<String> = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            let size = rdr.read_line(&mut line)?;
            if size == 0 {
                // TODO: Do you want to require the OSAAT?
                break;
            }

            if line.starts_with("OSAAT") {
                break;
            }

            lines.push(line.trim_end_matches('\n').to_string());
        }
        
        let stitches = Chart::read_stitches(&attributes, &mut lines)?;
        let chart = Chart { attributes, stitches };

        Ok(chart)
    }

    fn read_stitches(attributes: &Attributes, lines: &mut Vec<String>) -> Result<Vec<Vec<Stitch>>> {
        let mut stitches_vec: Vec<Vec<Stitch>> = Vec::new();
        
        for line in lines {
            stitches_vec.push(Chart::read_line(attributes, line)?)
        }

        Ok(stitches_vec)
    }

    fn read_line(attributes: &Attributes, line: &str) -> Result<Vec<Stitch>> {
        let mut stitch_vec: Vec<Stitch> = Vec::new();
        
        for ch in line.chars() {
            let stitch = match ch {
                c if c == attributes.knit_char => Stitch::Knit,
                c if c == attributes.purl_char => Stitch::Purl,
                c if c == attributes.empty_char => Stitch::Empty,
                _ => return Err(ErrorKind::BadStitchChar.into())
            };
            stitch_vec.push(stitch);
        }

        Ok(stitch_vec)
    }
}
