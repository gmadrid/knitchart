use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::attributes::Attributes;
use crate::errors::*;
use crate::header::Header;

#[derive(Debug)]
pub struct Chart {
    attributes: Attributes,
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
            let size = rdr.read_line(&mut line)?;
            if size == 0 {
                // TODO: Do you want to require the OSAAT?
                break;
            }

            lines.push(line.clone())            
        }
        
        let chart = Chart { attributes };

        Ok(chart)
    }
}
