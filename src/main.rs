use std::fs::File;
use std::io::BufWriter;

mod components;
mod constants;
mod planner;
mod utils;

pub use components::*;
pub use planner::PdfPlanner;
pub use utils::*;

fn main() {
    let planner = PdfPlanner::new(2024);

    planner
        .doc
        .save(&mut BufWriter::new(File::create("planner.pdf").unwrap()))
        .unwrap();
}
