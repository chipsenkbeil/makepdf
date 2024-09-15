use owned_ttf_parser::OwnedFace;
use printpdf::IndirectFontRef;

/// Represents the font used by the planner.
#[derive(Debug)]
pub struct EngineDocFont {
    pub face: OwnedFace,
    pub font: IndirectFontRef,
}
