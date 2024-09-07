use printpdf::{Color, Mm, Rgb};

/// 1404 pixels in millimeters
pub const PAGE_WIDTH: Mm = Mm(371.475);

/// 1872 pixels in millimeters
pub const PAGE_HEIGHT: Mm = Mm(495.3);

/// DPI for page images
pub const PAGE_DPI: f32 = 300.0;

/// Maximum rows to support on a page
pub const MAX_ROWS: f32 = 24.0;

/// Multiplier to convert points to millimeters.
pub const PT_TO_MM: f64 = 0.352778;

/// Height per row in millimeters
pub const ROW_HEIGHT: Mm = Mm(PAGE_HEIGHT.0 / MAX_ROWS);

/// Color for a banner's background
pub const BANNER_BACKGROUND_COLOR: Color = Color::Rgb(Rgb {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    icc_profile: None,
});

/// Color for a banner's text
pub const BANNER_TEXT_COLOR: Color = Color::Rgb(Rgb {
    r: 0.75,
    g: 0.75,
    b: 0.75,
    icc_profile: None,
});

/// Regular font to use
pub const REGULAR_FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");

/// Bold font to use
pub const BOLD_FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono-Bold.ttf");
