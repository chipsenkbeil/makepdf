use owned_ttf_parser::{AsFaceRef, Face, OwnedFace};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

// Dimensions explained in https://gist.github.com/Fantailed/7bb84ca394db9ff6b13b1c45e1be161a
//
// pixel resolution to 1404x1872 and the DPI to 300.
const PAGE_WIDTH: Mm = Mm(371.475);
const PAGE_HEIGHT: Mm = Mm(495.3);
const PAGE_DPI: f32 = 300.0;
const MAX_ROWS: f32 = 24.0;
const ROW_HEIGHT: Mm = Mm(PAGE_HEIGHT.0 / MAX_ROWS);
const BANNER_HEIGHT: Mm = Mm(PAGE_HEIGHT.0 / 24.0);
const BANNER_BACKGROUND_COLOR: Color = Color::Rgb(Rgb {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    icc_profile: None,
});
const BANNER_TEXT_COLOR: Color = Color::Rgb(Rgb {
    r: 0.75,
    g: 0.75,
    b: 0.75,
    icc_profile: None,
});

const REGULAR_FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
const BOLD_FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono-Bold.ttf");

enum BoxSize {
    /// Entire row in width
    Full,
    /// Left half of row in width
    Left,
    /// Right half of row in width
    Right,
    /// Custom position and size, overwriting previous calculations
    Custom {
        /// Lower-left x position
        x: Mm,
        /// Lower-left y position
        y: Mm,
        /// Width of the box
        width: Mm,
        /// Height of the box
        height: Mm,
    },
}

struct BoxPadding {
    top: Mm,
    left: Mm,
    right: Mm,
    bottom: Mm,
}

impl BoxPadding {
    pub fn all(padding: impl Into<Mm>) -> Self {
        let padding = padding.into();
        Self {
            top: padding,
            left: padding,
            right: padding,
            bottom: padding,
        }
    }
}

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
        draw_box(
            &layer,
            0,
            font,
            face,
            "DATE REVIEW",
            BoxSize::Left,
            BoxPadding::all(Mm(1.0)),
        );
        // Row 0, date calculated on the right
        draw_box(
            &layer,
            1,
            font,
            face,
            "MORNING REVIEW",
            BoxSize::Full,
            BoxPadding::all(Mm(1.0)),
        );
        draw_box(
            &layer,
            2,
            font,
            face,
            "I'M GRATEFUL FOR",
            BoxSize::Left,
            BoxPadding::all(Mm(1.0)),
        );
        draw_box(
            &layer,
            2,
            font,
            face,
            "I'M EXCITED ABOUT",
            BoxSize::Right,
            BoxPadding::all(Mm(1.0)),
        );
        // Row 3-8, lines with numbers
        // Row 9-10, affirmation
        // Row 11-12, focus/exercise
        // Row 13-16, priorities
        draw_box(
            &layer,
            17,
            font,
            face,
            "END OF DAY REVIEW",
            BoxSize::Full,
            BoxPadding::all(Mm(1.0)),
        );
        // Row 18, today's wins
        // Row 19-21, lines with numbers
        // Row 22, how I'll improve
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

/// Draws a banner that is the full width of the page at row N that has text vertically and
/// horizontally centered within it.
fn draw_box(
    layer: &PdfLayerReference,
    row: usize,
    font: &IndirectFontRef,
    face: &Face<'_>,
    text: &str,
    size: BoxSize,
    padding: impl Into<Option<BoxPadding>>,
) {
    let padding = padding.into();

    // Define the rectangular region which uses a lower-left x/y and an upper-right x/y
    //
    // We apply a row (index 0) to figure out where to place evenly within a page
    let mut lly = PAGE_HEIGHT - (BANNER_HEIGHT * (row + 1) as f32);
    let mut ury = PAGE_HEIGHT - (BANNER_HEIGHT * row as f32);

    let (mut llx, mut urx) = match size {
        BoxSize::Full => (Mm(0.0), PAGE_WIDTH),
        BoxSize::Left => (Mm(0.0), PAGE_WIDTH / 2.0),
        BoxSize::Right => (PAGE_WIDTH / 2.0, PAGE_WIDTH),
        BoxSize::Custom {
            x,
            y,
            width,
            height,
        } => {
            lly = y;
            ury = y + height;
            (x, x + width)
        }
    };

    // Calculate the padding
    if let Some(padding) = padding {
        lly += padding.bottom;
        ury -= padding.top;
        llx += padding.left;
        urx -= padding.right;
    }

    layer.set_fill_color(BANNER_BACKGROUND_COLOR);
    layer.set_outline_color(BANNER_BACKGROUND_COLOR);
    layer.add_rect(Rect::new(llx, lly, urx, ury));

    // If given text, we'll populate within the middle of the banner
    if !text.is_empty() {
        let font_size: f32 = 36.0;
        let text_width = text_width_in_mm(text, face, font_size as f64) as f32;
        let text_height = text_height_in_mm(face, font_size as f64) as f32;

        // Calculate the middle of the banner and then shift over by half the text length to
        // place it roughly within the middle of the banner itself
        let x = llx + ((urx - llx) / 2.0) - Mm(text_width / 2.0);

        // Calculate the space remaining from height of text and move up to vertically center
        // TODO: We calculate hard-coded padding at the bottom using text height because I
        //       cannot figure out how to determine the actual space to add since the font
        //       has some padding below or something. This is weird and I don't care anymore!
        //let y = lly + Mm(text_height * 0.125);
        let y = lly;

        layer.set_fill_color(BANNER_TEXT_COLOR);
        layer.set_outline_color(BANNER_TEXT_COLOR);
        layer.use_text(text, font_size, x, y, font);
    }
}

fn text_width_in_mm(text: &str, face: &Face<'_>, font_size: f64) -> f64 {
    let units_per_em = face.units_per_em() as f64;
    let scale = font_size / units_per_em;

    text.chars()
        .map(|ch| {
            glyph_metrics(face, ch as u16)
                .map(|glyph| glyph.width as f64 * scale)
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        * 0.352778 // Convert points to millimeters
}

fn text_height_in_mm(face: &Face<'_>, font_size: f64) -> f64 {
    let units_per_em = face.units_per_em() as f64;
    let ascender = face.ascender() as f64;
    let descender = face.descender() as f64;
    let line_gap = face.line_gap() as f64;

    // Calculate the total height of the text
    let text_height = (ascender - descender + line_gap) * (font_size / units_per_em);

    // Convert to millimeters (1 point = 0.352778 mm)
    text_height * 0.352778
}

fn glyph_metrics(face: &owned_ttf_parser::Face<'_>, glyph_id: u16) -> Option<GlyphMetrics> {
    let glyph_id = owned_ttf_parser::GlyphId(glyph_id);
    if let Some(width) = face.glyph_hor_advance(glyph_id) {
        let width = width as u32;
        let height = face
            .glyph_bounding_box(glyph_id)
            .map(|bbox| bbox.y_max - bbox.y_min - face.descender())
            .unwrap_or(1000) as u32;
        Some(GlyphMetrics { width, height })
    } else {
        None
    }
}
