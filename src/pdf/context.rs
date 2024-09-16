use crate::pdf::PdfConfig;
use crate::runtime::RuntimeDocFont;
use owned_ttf_parser::Face;
use printpdf::{IndirectFontRef, PdfLayerReference};

/// Context provided to a [`PdfObject`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct PdfContext<'a> {
    pub config: &'a PdfConfig,
    pub font: &'a RuntimeDocFont,
    pub layer: &'a PdfLayerReference,
}

impl PdfContext<'_> {
    /// Returns a reference to the font face.
    pub fn as_face(&self) -> &Face {
        self.font.as_face()
    }

    /// Returns a reference to the underlying font.
    pub fn as_font_ref(&self) -> Option<&IndirectFontRef> {
        self.font.as_font_ref()
    }
}
