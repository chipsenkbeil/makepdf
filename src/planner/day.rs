use chrono::NaiveDate;
use owned_ttf_parser::AsFaceRef;

use crate::constants::ROW_HEIGHT;
use crate::{
    BoundsExt, BoxComponent, Component, Context, LineComponent, OutlineComponent, Planner,
    TextComponent,
};

macro_rules! grateful_excited_line {
    ($ctx:expr, $row:expr) => {{
        grateful_excited_line!($ctx, $row, "");
    }};
    ($ctx:expr, $row:expr, $label:expr) => {{
        let margin = crate::Margin::from((0.0, 10.0, 10.0, 7.0));

        if !$label.is_empty() {
            let mut text_margin = margin;
            text_margin.left += printpdf::Mm(10.0);

            TextComponent::new()
                .at_row($row)
                .with_sixteenth_width()
                .with_text($label)
                .with_margin(text_margin)
                .draw(&$ctx);
            TextComponent::new()
                .at_row($row)
                .shift_half_right()
                .with_sixteenth_width()
                .with_text($label)
                .with_margin(text_margin)
                .draw(&$ctx);
        }

        LineComponent::new()
            .at_row($row)
            .with_half_width()
            .with_margin(margin)
            .draw(&$ctx);
        LineComponent::new()
            .at_row($row)
            .shift_half_right()
            .with_half_width()
            .with_margin(margin)
            .draw(&$ctx);
    }};
}

/// Creates a page representing the morning review and priorities.
///
/// This is modeled after the Panda Planner, but collapses the two pages by removing the schedule
/// and task list as I maintain those somewhere else and want this to fit within a single page on
/// the Supernote Nomad.
///
/// ```text
/// [DAY & DATE] Month Day, Year (Day of Week)
///
/// [             MORNING REVIEW             ]
/// [ I'M GRATEFUL FOR ] [ I'M EXCITED ABOUT ]
/// . 1________________. . 1_________________.
/// . _________________. . __________________.
/// . 2________________. . 2_________________.
/// . _________________. . __________________.
/// . 3________________. . 3_________________.
/// .__________________. .___________________.
/// ..........................................
///
/// [ AFFIRMATION ] ..........................
/// ..........................................
///
/// [ FOCUS ] .......... [ EXERCISE ] ........
/// .................... .....................
///
/// [ P1 ] ............. [ P2 ] ..............
/// .................... .....................
/// [ P3 ] ............. [ P4 ] ..............
/// .................... .....................
///
/// [           END OF DAY REVIEW            ]
///             [ TODAY'S WINS ]
/// 1_________________________________________
/// 2_________________________________________
/// 3_________________________________________
///           [ HOW I'LL IMPROVE ]
/// 1_________________________________________
/// ```
pub fn make_page(planner: &Planner, date: NaiveDate) {
    println!(
        "Building daily page: {}",
        date.format("%A, %-d %B, %C%y (WK%W)")
    );

    // Get indexes for monthly, weekly, prev, next pages
    let monthly_page_index = planner
        .get_monthly_index(date)
        .expect("Missing monthly entry")
        .0;
    let weekly_page_index = planner
        .get_weekly_index(date)
        .expect("Missing weekly entry")
        .0;
    let maybe_prev_page_index = planner.get_prev_daily_index(date).map(|x| x.0);
    let maybe_next_page_index = planner.get_next_daily_index(date).map(|x| x.0);

    // Get information about current date
    let layer = planner
        .get_daily_reference(date)
        .expect("Missing daily entry")
        .1;
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
        .with_text("DAY & DATE")
        .with_margin(2.5)
        .draw(&ctx);
    LineComponent::new()
        .at_row(0)
        .shift_quarter_right()
        .with_three_quarters_width()
        .with_margin(2.5)
        .with_dashed_line()
        .draw(&ctx);
    TextComponent::new()
        .at_row(0)
        .shift_quarter_right()
        .with_three_quarters_width()
        .with_text(format!("{}", date.format("%A, %-d %B, %Y (WK%W)")))
        .with_margin(2.5)
        .draw(&ctx);

    //
    // Morning Review
    //
    BoxComponent::new()
        .at_row(1)
        .with_full_width()
        .with_text("MORNING REVIEW")
        .with_margin(2.5)
        .draw(&ctx);

    // Big outlines to cover the next sections
    OutlineComponent::new()
        .at_row(8)
        .with_half_width()
        .with_height(ROW_HEIGHT * 7.0)
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(8)
        .with_half_width()
        .shift_half_right()
        .with_height(ROW_HEIGHT * 7.0)
        .with_margin(2.5)
        .draw(&ctx);

    //
    // Grateful/Excited For
    //
    BoxComponent::new()
        .at_row(2)
        .with_half_width()
        .with_text("I'M GRATEFUL FOR")
        .with_margin(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(2)
        .shift_half_right()
        .with_half_width()
        .with_text("I'M EXCITED ABOUT")
        .with_margin(2.5)
        .draw(&ctx);

    // Row 3-8, lines with numbers
    grateful_excited_line!(ctx, 3, "1");
    grateful_excited_line!(ctx, 4);
    grateful_excited_line!(ctx, 5, "2");
    grateful_excited_line!(ctx, 6);
    grateful_excited_line!(ctx, 7, "3");
    grateful_excited_line!(ctx, 8);

    //
    // Affirmation
    //
    BoxComponent::new()
        .at_row(9)
        .with_quarter_width()
        .with_text("AFFIRMATION")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(10)
        .with_full_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);

    //
    // Focus & Exercise
    //
    BoxComponent::new()
        .at_row(11)
        .with_quarter_width()
        .with_text("FOCUS")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(12)
        .with_half_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(11)
        .shift_half_right()
        .with_quarter_width()
        .with_text("EXERCISE")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(12)
        .shift_half_right()
        .with_half_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);

    //
    // Priorities
    //
    BoxComponent::new()
        .at_row(13)
        .with_eighth_width()
        .with_text("P1")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(14)
        .with_half_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(13)
        .shift_half_right()
        .with_eighth_width()
        .with_text("P2")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(14)
        .shift_half_right()
        .with_half_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(15)
        .with_eighth_width()
        .with_text("P3")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(16)
        .with_half_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);
    BoxComponent::new()
        .at_row(15)
        .shift_half_right()
        .with_eighth_width()
        .with_text("P4")
        .with_margin(2.5)
        .draw(&ctx);
    OutlineComponent::new()
        .at_row(16)
        .shift_half_right()
        .with_half_width()
        .with_height(ROW_HEIGHT * 2.0)
        .with_margin(2.5)
        .draw(&ctx);

    //
    // End of Day Review
    //
    BoxComponent::new()
        .at_row(17)
        .with_full_width()
        .with_text("END OF DAY REVIEW")
        .with_margin(2.5)
        .draw(&ctx);

    //
    // Today's Wins
    //
    BoxComponent::new()
        .at_row(18)
        .shift_quarter_right()
        .with_half_width()
        .with_text("TODAY'S WINS")
        .with_margin(2.5)
        .draw(&ctx);

    // Row 19-21, lines with numbers
    TextComponent::new()
        .at_row(19)
        .with_sixteenth_width()
        .with_text("1")
        .with_margin(2.5)
        .draw(&ctx);
    LineComponent::new()
        .at_row(19)
        .with_full_width()
        .with_margin(2.5)
        .draw(&ctx);
    TextComponent::new()
        .at_row(20)
        .with_sixteenth_width()
        .with_text("2")
        .with_margin(2.5)
        .draw(&ctx);
    LineComponent::new()
        .at_row(20)
        .with_full_width()
        .with_margin(2.5)
        .draw(&ctx);
    TextComponent::new()
        .at_row(21)
        .with_sixteenth_width()
        .with_text("3")
        .with_margin(2.5)
        .draw(&ctx);
    LineComponent::new()
        .at_row(21)
        .with_full_width()
        .with_margin(2.5)
        .draw(&ctx);

    //
    // How I'll Improve
    //
    BoxComponent::new()
        .at_row(22)
        .shift_quarter_right()
        .with_half_width()
        .with_text("HOW I'LL IMPROVE")
        .with_margin(2.5)
        .draw(&ctx);

    // Row 23, lines with numbers
    TextComponent::new()
        .at_row(23)
        .with_sixteenth_width()
        .with_text("1")
        .with_margin(2.5)
        .draw(&ctx);
    LineComponent::new()
        .at_row(23)
        .with_full_width()
        .with_margin(2.5)
        .draw(&ctx);
}
