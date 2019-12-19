use std::convert::TryFrom;
use std::convert::TryInto;
use std::env;
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

fn the_thing(filename: &str, chart: &Chart) {
    let background_color = to_color_array(chart.background_color());
    let dot_radius = 10.0;
    let cell_size = 15u32;

    let rows = u32::try_from(chart.rows()).unwrap();
    let cols = u32::try_from(chart.columns()).unwrap();

    // TODO: make chart return u32 for rows()/columns().
    let mut buffer = RenderBuffer::new(cols * cell_size, rows * cell_size);

    buffer.clear(background_color);

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
    for file in env::args().skip(1) {
        process_file(&file)?;
    }
    Ok(())
}
