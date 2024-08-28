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
    let doc = PdfDocument::empty("Beatrix Planner 2024");

    doc.save(&mut BufWriter::new(File::create("planner.pdf").unwrap()))
        .unwrap();
}

/// Creates a page representing the morning review and priorities.
///
/// This is modeled after the Panda Planner, but collapses the two pages by removing the schedule
/// and task list as I maintain those somewhere else and want this to fit within a single page on
/// the Supernote Nomad.
///
/// ```text
/// [DAY & DATE] Month Day, Year (Day of Week)
/// [             MORNING REVIEW             ]
/// [ I'M GRATEFUL FOR ] [ I'M EXCITED ABOUT ]
/// 1___________________ 1____________________
/// ____________________ _____________________
/// 2___________________ 2____________________
/// ____________________ _____________________
/// 3___________________ 3____________________
/// ____________________ _____________________
/// [ AFFIRMATION ] ..........................
/// ..........................................
/// [ FOCUS ] .......... [ EXERCISE ] ........
/// .................... .....................
/// [ P1 ] ------------- [ P2 ] --------------
/// -------------------- ---------------------
/// [ P3 ] ------------- [ P4 ] --------------
/// -------------------- ---------------------
/// [           END OF DAY REVIEW            ]
///             [ TODAY'S WINS ]
/// 1_________________________________________
/// 2_________________________________________
/// 3_________________________________________
///
///           [ HOW I'LL IMPROVE ]
/// 1_________________________________________
/// ```
fn make_daily_page_1(doc: &PdfDocumentReference) -> (PdfPageIndex, PdfLayerIndex) {
    let (page, layer) = doc.add_page(PAGE_WIDTH, PAGE_HEIGHT, "Morning Review + Priorities");

    (page, layer)
}

/// Schedule + end of day review
fn make_daily_page_2(doc: &PdfDocumentReference) -> (PdfPageIndex, PdfLayerIndex) {
    let (page, layer) = doc.add_page(PAGE_WIDTH, PAGE_HEIGHT, "Schedule + End of Day Review");
    (page, layer)
}
