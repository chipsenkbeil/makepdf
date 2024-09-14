/// Default font to use.
pub const DEFAULT_FONT: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");

/// Internal scripts available to be run.
pub static SCRIPTS: phf::Map<&'static str, &[u8]> = phf::phf_map! {
    "example" => include_bytes!("../assets/scripts/example.lua"),
    "panda" => include_bytes!("../assets/scripts/panda.lua"),
};
