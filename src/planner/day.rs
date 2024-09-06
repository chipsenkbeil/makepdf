use chrono::{Datelike, Month, NaiveDate};
use owned_ttf_parser::AsFaceRef;
use printpdf::*;

use crate::{BoxComponent, Component, Context, PdfPlanner};

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
pub fn make_page(planner: &PdfPlanner, date: NaiveDate) {
    println!(
        "Building daily page: {}",
        date.format("%A, %-d %B, %C%y (WK%W)")
    );

    let month: Month = TryFrom::try_from(date.month() as u8).unwrap();
    let (_, layer) = planner.get_for_day(month, date.day());
    let font = &planner.font;
    let face = planner.face.as_face_ref();

    let ctx = Context {
        face,
        font,
        layer: &layer,
    };

    //
    // Date Marker
    //
    BoxComponent::new()
        .at_row(0)
        .with_quarter_width()
        .with_text("DATE REVIEW")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(0)
        .shift_quarter_right()
        .with_three_quarters_width()
        .with_text(format!("{}", date.format("%A, %-d %B, %C%y (WK%W)")))
        .with_padding(2.5)
        .draw(&ctx);

    //
    // Morning Review
    //
    BoxComponent::new()
        .at_row(1)
        .with_full_width()
        .with_text("MORNING REVIEW")
        .with_padding(2.5)
        .draw(&ctx);

    //
    // Grateful/Excited For
    //
    BoxComponent::new()
        .at_row(2)
        .with_half_width()
        .with_text("I'M GRATEFUL FOR")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(2)
        .shift_half_right()
        .with_half_width()
        .with_text("I'M EXCITED ABOUT")
        .with_padding(2.5)
        .draw(&ctx);

    // Row 3-8, lines with numbers

    //
    // Affirmation
    //
    BoxComponent::new()
        .at_row(9)
        .with_quarter_width()
        .with_text("AFFIRMATION")
        .with_padding(2.5)
        .draw(&ctx);

    //
    // Focus & Exercise
    //
    BoxComponent::new()
        .at_row(11)
        .with_quarter_width()
        .with_text("FOCUS")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(11)
        .shift_half_right()
        .with_quarter_width()
        .with_text("EXERCISE")
        .with_padding(2.5)
        .draw(&ctx);

    //
    // Priorities
    //
    BoxComponent::new()
        .at_row(13)
        .with_eighth_width()
        .with_text("P1")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(13)
        .shift_half_right()
        .with_eighth_width()
        .with_text("P2")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(15)
        .with_eighth_width()
        .with_text("P3")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(15)
        .shift_half_right()
        .with_eighth_width()
        .with_text("P4")
        .with_padding(2.5)
        .draw(&ctx);

    //
    // End of Day Review
    //
    BoxComponent::new()
        .at_row(17)
        .with_full_width()
        .with_text("END OF DAY REVIEW")
        .with_padding(2.5)
        .draw(&ctx);

    //
    // Today's Wins
    //
    BoxComponent::new()
        .at_row(18)
        .shift_quarter_right()
        .with_half_width()
        .with_text("TODAY'S WINS")
        .with_padding(2.5)
        .draw(&ctx);

    // Row 19-21, lines with numbers
    BoxComponent::new()
        .at_row(19)
        .with_sixteenth_width()
        .with_text("1")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(20)
        .with_sixteenth_width()
        .with_text("2")
        .with_padding(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(21)
        .with_sixteenth_width()
        .with_text("3")
        .with_padding(2.5)
        .draw(&ctx);

    //
    // How I'll Improve
    //
    BoxComponent::new()
        .at_row(22)
        .shift_quarter_right()
        .with_half_width()
        .with_text("HOW I'LL IMPROVE")
        .with_padding(2.5)
        .draw(&ctx);

    // Row 23, lines with numbers
    BoxComponent::new()
        .at_row(23)
        .with_sixteenth_width()
        .with_text("1")
        .with_padding(2.5)
        .draw(&ctx);

    // Hard-coded link for now
    let (page_index, _) = planner.get_index_for_day(Month::January, 1);
    layer.add_link_annotation(LinkAnnotation::new(
        BoxComponent::new()
            .at_row(0)
            .shift_quarter_right()
            .with_three_quarters_width()
            .with_text(format!("{}", date.format("%A, %-d %B, %C%y (WK%W)")))
            .with_padding(2.5)
            .bounds_rect(),
        None,
        None,
        Actions::go_to(Destination::XYZ {
            page: page_index,
            left: None,
            top: None,
            zoom: None,
        }),
        None,
    ));
}
