#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

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
            ColorParseError(css_color_parser::ColorParseError);
            IoError(std::io::Error);
            ParseIntError(std::num::ParseIntError);
        }
    }
}

pub use chart::{Chart, Stitch};
