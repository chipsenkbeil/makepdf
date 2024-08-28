use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

// Dimensions explained in https://gist.github.com/Fantailed/7bb84ca394db9ff6b13b1c45e1be161a
//
// pixel resolution to 1404x1872 and the DPI to 300.
const PAGE_WIDTH: Mm = Mm(371.475);
const PAGE_HEIGHT: Mm = Mm(495.3);
const PAGE_DPI: f32 = 300.0;

fn main() {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", PAGE_WIDTH, PAGE_HEIGHT, "Layer 1");
    let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1");

    doc.save(&mut BufWriter::new(File::create("planner.pdf").unwrap()))
        .unwrap();
}
