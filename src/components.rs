mod r#box;
mod padding;
mod rect;

pub use padding::Padding;
pub use r#box::BoxComponent;
pub use rect::Rect;

use owned_ttf_parser::Face;
use printpdf::{IndirectFontRef, PdfLayerReference};

/// Abstraction of a component that can be drawn on a PDF.
pub trait Component {
    /// Draws the component within the PDF.
    fn draw(&self, ctx: &Context<'_>);
}

/// Context provided to a [`Component`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct Context<'a> {
    pub face: &'a Face<'a>,
    pub font: &'a IndirectFontRef,
    pub layer: &'a PdfLayerReference,
}
