use crate::constants::DEFAULT_FONT;
use anyhow::Context;
use owned_ttf_parser::OwnedFace;

/// Represents the font used by the planner.
#[derive(Debug)]
pub struct PlannerFont {
    pub face: OwnedFace,
}

impl PlannerFont {
    /// Loads the font from the provided `path`, falling back to the default font if none
    /// specified.
    pub fn load(path: Option<&str>) -> anyhow::Result<Self> {
        let font_bytes = match path {
            Some(path) => std::fs::read(path).context("Failed to read font")?,
            None => DEFAULT_FONT.to_vec(),
        };
        let face = OwnedFace::from_vec(font_bytes, 0).context("Failed to build font into face")?;

        Ok(Self { face })
    }
}
