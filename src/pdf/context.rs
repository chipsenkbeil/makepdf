use crate::engine::EngineDocFont;
use crate::pdf::PdfConfig;
use owned_ttf_parser::{AsFaceRef, Face};
use printpdf::{IndirectFontRef, PdfLayerReference};

/// Context provided to a [`PdfObject`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct PdfContext<'a> {
    pub config: &'a PdfConfig,
    pub font: &'a EngineDocFont,
    pub layer: &'a PdfLayerReference,
}

impl PdfContext<'_> {
    /// Returns a reference to the font face.
    pub fn as_face(&self) -> &Face<'_> {
        self.font.face.as_face_ref()
    }

    /// Returns a reference to the underlying font.
    pub fn as_font_ref(&self) -> &IndirectFontRef {
        &self.font.font
    }
}
