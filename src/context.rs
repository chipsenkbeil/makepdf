use owned_ttf_parser::Face;
use printpdf::{IndirectFontRef, PdfLayerReference};

#[derive(Copy, Clone, Debug)]
pub struct Context<'a> {
    pub face: &'a Face<'a>,
    pub font: &'a IndirectFontRef,
    pub layer: &'a PdfLayerReference,
}
