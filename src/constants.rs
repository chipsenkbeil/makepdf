use printpdf::{Color, Rgb};

/// Maximum rows to support on a page
pub const MAX_ROWS: f32 = 24.0;

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

/// Default font to use
pub const DEFAULT_FONT: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");

/// Internal scripts available to be run
pub static SCRIPTS: phf::Map<&'static str, &[u8]> = phf::phf_map! {
    "panda" => include_bytes!("../assets/scripts/panda.luau"),
};
