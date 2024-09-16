use crate::pdf::PdfConfig;
use crate::runtime::{RuntimeFontId, RuntimeFonts};
use printpdf::PdfLayerReference;

/// Context provided to a [`PdfObject`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct PdfContext<'a> {
    pub config: &'a PdfConfig,
    pub layer: &'a PdfLayerReference,
    pub fonts: &'a RuntimeFonts,
    pub fallback_font_id: RuntimeFontId,
}
