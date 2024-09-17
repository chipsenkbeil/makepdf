/// Default font to use.
pub const DEFAULT_FONT: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");

/// Name of global variable representing PDF interface.
pub const GLOBAL_PDF_VAR_NAME: &str = "pdf";

/// Internal scripts available to be run.
pub static SCRIPTS: phf::Map<&'static str, &[u8]> = phf::phf_map! {
    "example" => include_bytes!("../assets/scripts/example.lua"),
    "panda" => include_bytes!("../assets/scripts/panda.lua"),
};
