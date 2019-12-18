use std::convert::TryFrom;
use std::convert::TryInto;
use std::env;
use std::path::PathBuf;

use graphics::grid::Grid;
use graphics::line::Line;
use graphics_buffer::{RenderBuffer, IDENTITY};

use crate::chart::Chart;
use crate::chart::Stitch;
use crate::errors::*;

#[macro_use]
extern crate error_chain;

mod attributes;
mod chart;
mod header;

pub mod errors {
    error_chain! {
        errors {
            // TODO: these errors should contain the character and some context.
            BadStitchChar {
                description("Bad stitch char")
                display("Bad stitch char")
            }
            BadHeaderLine(line_number: usize) {
                description("A badly formed header line was found")
                display("Header line {} should have the form 'name=value'",
                        line_number)
            }
            IdentInitialNotAlpha(line_number: usize) {
                description("An identifier has an invalid first character.")
                display("Identifier on line {} must start with alpha character.",
                        line_number)
            }
            IdentInvalidChar(line_number: usize) {
                description("An identifier has a non-alnum character.")
                display("Identifier on line {} contains non-alnum character",
                        line_number)
            }
            InvalidCharName {
                description("A char was badly named.")
                display("A char was badly named.")
            }
            MissingIdent(line_number: usize) {
                description("An identifier is missing in the header.")
                display("Identifier missing on line {}.", line_number)
            }
            UnknownAttrName(name: String) {
                description("Unknown attr name")
                display("The attr {} is unknown.", name)
            }
        }
        foreign_links {
            IoError(std::io::Error);
            ParseIntError(std::num::ParseIntError);
        }
    }
}

fn the_thing(filename: &str, chart: &Chart) {
    let dot_radius = 10.0;
    let cell_size = 15u32;

    let rows = u32::try_from(chart.rows()).unwrap();
    let cols = u32::try_from(chart.columns()).unwrap();

    // TODO: make chart return u32 for rows()/columns().
    let mut buffer = RenderBuffer::new(cols * cell_size, rows * cell_size);

    buffer.clear([0.9, 0.9, 0.9, 1.0]);

    let grid = Grid {
        cols: cols,
        rows: rows,
        units: f64::from(cell_size),
    };
    let line = Line::new([0.1, 0.1, 0.1, 1.0], 1.0);
    grid.draw(&line, &Default::default(), IDENTITY, &mut buffer);

    for cell in grid.cells() {
        let (col, row) = cell;
        let cell_pos = grid.cell_position(cell);

        let center_y = cell_pos[0] + f64::from(cell_size) / 2.0;
        let center_x = cell_pos[1] + f64::from(cell_size) / 2.0;

        let stitch = chart.stitch(row.try_into().unwrap(), col.try_into().unwrap());

        if let Stitch::Purl = stitch {
            let rectangle = [
                center_y - dot_radius / 2.0,
                center_x - dot_radius / 2.0,
                dot_radius,
                dot_radius,
            ];
            println!("{:?}", cell_pos);
            graphics::ellipse([0.1, 0.1, 0.1, 1.0], rectangle, IDENTITY, &mut buffer);
        }
    }

    let outfile = PathBuf::from(filename).with_extension("png");
    buffer.save(outfile).unwrap();
}

fn process_file(filename: &str) -> Result<()> {
    let chart = Chart::open(filename)?;
    println!("Chart \"{}\":", filename);
    println!("     rows: {}", chart.rows());
    println!("  columns: {}", chart.columns());

    the_thing(filename, &chart);

    Ok(())
}

fn main() -> Result<()> {
    let files = env::args().skip(1).collect::<Vec<String>>();

    for file in files {
        process_file(&file)?;
    }
    Ok(())
}
