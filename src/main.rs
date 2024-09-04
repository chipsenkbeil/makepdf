use std::fs::File;
use std::io::BufWriter;

mod constants;
mod context;
mod pdf_box;
mod planner;

use context::Context;
pub use pdf_box::PdfBox;
pub use planner::PdfPlanner;

fn main() {
    let planner = PdfPlanner::new(2024);

    planner
        .doc
        .save(&mut BufWriter::new(File::create("planner.pdf").unwrap()))
        .unwrap();
}
