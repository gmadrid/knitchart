use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{repeat, repeat_with};

use css_color_parser::Color as CssColor;

use crate::attributes::Attributes;
use crate::errors::*;
use crate::header::Header;

#[derive(Debug)]
pub struct Chart {
    attributes: Attributes,
    stitches: Vec<Vec<Stitch>>,

    rows: usize,
    cols: usize,
}

#[derive(Clone, Copy)]
pub enum Stitch {
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

        let (stitches, rows, cols) = Chart::read_stitches(&attributes, &mut lines)?;
        let chart = Chart {
            attributes,
            stitches,
            rows,
            cols,
        };

        Ok(chart)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn columns(&self) -> usize {
        self.cols
    }
    pub fn background_color(&self) -> CssColor {
        self.attributes.background_color
    }

    pub fn stitch(&self, row: usize, col: usize) -> Stitch {
        self.stitches[row][col]
    }

    fn read_stitches(
        attributes: &Attributes,
        lines: &mut Vec<String>,
    ) -> Result<(Vec<Vec<Stitch>>, usize, usize)> {
        let mut stitches_vec: Vec<Vec<Stitch>> = Vec::new();

        for line in lines {
            stitches_vec.push(Chart::read_line(attributes, line)?)
        }

        let (rows, cols) = fix_problems(attributes, &mut stitches_vec);

        Ok((stitches_vec, rows, cols))
    }

    fn read_line(attributes: &Attributes, line: &str) -> Result<Vec<Stitch>> {
        let mut stitch_vec: Vec<Stitch> = Vec::new();

        for ch in line.chars() {
            let stitch = match ch {
                c if c == attributes.knit_char => Stitch::Knit,
                c if c == attributes.purl_char => Stitch::Purl,
                c if c == attributes.empty_char => Stitch::Empty,
                _ => return Err(ErrorKind::BadStitchChar.into()),
            };
            stitch_vec.push(stitch);
        }

        Ok(stitch_vec)
    }
}

fn warn(s: &str) {
    eprintln!("{}", s);
}

fn fix_problems(attributes: &Attributes, mut stitches: &mut Vec<Vec<Stitch>>) -> (usize, usize) {
    let cols = figure_out_cols(attributes, stitches);
    let rows = figure_out_rows(attributes, stitches);

    fixup_rows(rows, &mut stitches);
    fixup_cols(cols, &mut stitches);

    (rows, cols)
}

fn fixup_rows(rows: usize, stitches: &mut Vec<Vec<Stitch>>) {
    if rows < stitches.len() {
        warn("You have more rows then you specified");
    } else if rows > stitches.len() {
        warn("You have too few rows. I'm adding some for you.");
        repeat_with(|| Vec::new())
            .take(rows - stitches.len())
            .for_each(|v| stitches.push(v));
    }
}

fn fixup_cols(cols: usize, stitches: &mut Vec<Vec<Stitch>>) {
    for row in stitches {
        if cols < row.len() {
            warn("You have too many stitches in one row. Truncating");
            row.truncate(cols);
        } else if cols > row.len() {
            warn("You are missing stitches in one row. Adding knits.");
            repeat(Stitch::Knit)
                .take(cols - row.len())
                .for_each(|s| row.push(s));
        }
    }
}

fn figure_out_rows(attributes: &Attributes, stitches: &Vec<Vec<Stitch>>) -> usize {
    if attributes.rows == 0 {
        stitches.len()
    } else {
        attributes.rows
    }
}

fn figure_out_cols(attributes: &Attributes, stitches: &Vec<Vec<Stitch>>) -> usize {
    if attributes.cols == 0 {
        stitches.iter().map(|v| v.len()).max().unwrap_or(0)
    } else {
        attributes.cols
    }
}
