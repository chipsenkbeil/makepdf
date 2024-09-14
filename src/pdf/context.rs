use crate::pdf::PdfConfig;
use owned_ttf_parser::Face;
use printpdf::{IndirectFontRef, PdfLayerReference};

/// Context provided to a [`PdfObject`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct PdfContext<'a> {
    pub config: &'a PdfConfig,
    pub face: &'a Face<'a>,
    pub font: &'a IndirectFontRef,
    pub layer: &'a PdfLayerReference,
}
