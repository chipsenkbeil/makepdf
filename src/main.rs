use chrono::NaiveDate;
use owned_ttf_parser::{AsFaceRef, Face, OwnedFace};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

mod constants;
mod context;
mod pdf_box;

use constants::*;
use context::Context;
pub use pdf_box::PdfBox;

fn main() {
    let doc = PdfDocument::empty("Beatrix Planner 2024");
    let owned_face = OwnedFace::from_vec(REGULAR_FONT.to_vec(), 0).unwrap();
    let font = doc.add_external_font(REGULAR_FONT).unwrap();
    make_daily_page_1(&doc, &font, owned_face.as_face_ref());

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
///           [ HOW I'LL IMPROVE ]
/// 1_________________________________________
/// ```
fn make_daily_page_1(
    doc: &PdfDocumentReference,
    font: &IndirectFontRef,
    face: &Face<'_>,
) -> (PdfPageIndex, PdfLayerIndex) {
    let (page, layer) = doc.add_page(PAGE_WIDTH, PAGE_HEIGHT, "");

    {
        let layer = doc.get_page(page).get_layer(layer);
        let ctx = Context {
            face,
            font,
            layer: &layer,
        };

        let today = NaiveDate::from_ymd_opt(2024, 9, 30).unwrap();

        PdfBox::new()
            .at_row(0)
            .with_quarter_width()
            .with_text("DATE REVIEW")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(0)
            .shift_quarter_right()
            .with_three_quarters_width()
            .with_text(format!("{}", today.format("%A, %-d %B, %C%y (WK%W)")))
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(1)
            .with_full_width()
            .with_text("MORNING REVIEW")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(2)
            .with_half_width()
            .with_text("I'M GRATEFUL FOR")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(2)
            .shift_half_right()
            .with_half_width()
            .with_text("I'M EXCITED ABOUT")
            .with_padding(2.5)
            .draw(&ctx);
        // Row 3-8, lines with numbers
        // Row 9-10, affirmation
        PdfBox::new()
            .at_row(9)
            .with_quarter_width()
            .with_text("AFFIRMATION")
            .with_padding(2.5)
            .draw(&ctx);
        // Row 11-12, focus/exercise
        PdfBox::new()
            .at_row(11)
            .with_quarter_width()
            .with_text("FOCUS")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(11)
            .shift_half_right()
            .with_quarter_width()
            .with_text("EXERCISE")
            .with_padding(2.5)
            .draw(&ctx);
        // Row 13-16, priorities
        PdfBox::new()
            .at_row(13)
            .with_eighth_width()
            .with_text("P1")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(13)
            .shift_half_right()
            .with_eighth_width()
            .with_text("P2")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(15)
            .with_eighth_width()
            .with_text("P3")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(15)
            .shift_half_right()
            .with_eighth_width()
            .with_text("P4")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(17)
            .with_full_width()
            .with_text("END OF DAY REVIEW")
            .with_padding(2.5)
            .draw(&ctx);
        PdfBox::new()
            .at_row(18)
            .shift_quarter_right()
            .with_half_width()
            .with_text("TODAY'S WINS")
            .with_padding(2.5)
            .draw(&ctx);
        // Row 19-21, lines with numbers
        PdfBox::new()
            .at_row(22)
            .shift_quarter_right()
            .with_half_width()
            .with_text("HOW I'LL IMPROVE")
            .with_padding(2.5)
            .draw(&ctx);
        // Row 23, lines with numbers
    }

    (page, layer)
}

/// Schedule + end of day review
fn make_daily_page_2(
    doc: &PdfDocumentReference,
    font: &IndirectFontRef,
) -> (PdfPageIndex, PdfLayerIndex) {
    let (page, layer) = doc.add_page(PAGE_WIDTH, PAGE_HEIGHT, "");
    (page, layer)
}
