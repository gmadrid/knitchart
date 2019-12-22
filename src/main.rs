use std::convert::TryFrom;
use std::convert::TryInto;
use std::env;
use std::io::Write;
use std::path::PathBuf;

use css_color_parser::Color as CssColor;
use graphics::grid::Grid;
use graphics::line::Line;
use graphics_buffer::{RenderBuffer, IDENTITY};
use knitchart::errors::*;
use knitchart::{Chart, Stitch};

fn to_color_array(color: CssColor) -> graphics::types::Color {
    use graphics::types::ColorComponent;

    [
        ColorComponent::from(color.r) / 255.0,
        ColorComponent::from(color.g) / 255.0,
        ColorComponent::from(color.b) / 255.0,
        color.a,
    ]
}

fn the_thing(filename: &str, chart: &Chart) -> Result<()> {
    let background_color = to_color_array(chart.background_color());
    let dot_size = chart.dot_size();
    let cell_size = chart.cell_size();
    let grid_color = to_color_array(chart.grid_color());

    let rows = u32::try_from(chart.rows())?;
    let cols = u32::try_from(chart.columns())?;

    let cell_size_int = cell_size as u32;
    let mut buffer = RenderBuffer::new(cols * cell_size_int, rows * cell_size_int);

    buffer.clear(background_color);

    let grid = Grid {
        cols: cols,
        rows: rows,
        units: f64::from(cell_size),
    };
    let line = Line::new(grid_color, 1.0);
    grid.draw(&line, &Default::default(), IDENTITY, &mut buffer);

    for cell in grid.cells() {
        let (col, row) = cell;
        let cell_pos = grid.cell_position(cell);

        let center_y = cell_pos[0] + f64::from(cell_size) / 2.0;
        let center_x = cell_pos[1] + f64::from(cell_size) / 2.0;

        let stitch = chart.stitch(row.try_into()?, col.try_into()?);

        if let Stitch::Purl = stitch {
            let rectangle = [
                center_y - dot_size / 2.0,
                center_x - dot_size / 2.0,
                dot_size,
                dot_size,
            ];
            std::io::stdout().flush()?;
            graphics::ellipse([0.1, 0.1, 0.1, 1.0], rectangle, IDENTITY, &mut buffer);
        }
        print!("\r{:?}          ", cell);
    }
    print!("\r");

    let outfile = PathBuf::from(filename).with_extension("png");
    println!("Output file: {}", outfile.to_string_lossy());
    Ok(buffer.save(outfile)?)
}

fn process_file(filename: &str) -> Result<()> {
    let chart = Chart::open(filename)?;
    println!("Chart: {}", filename);
    println!("     rows: {}", chart.rows());
    println!("  columns: {}", chart.columns());

    the_thing(filename, &chart)?;

    Ok(())
}

fn main() -> Result<()> {
    for file in env::args().skip(1) {
        process_file(&file)?;
    }
    Ok(())
}
