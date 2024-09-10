mod r#box;
mod line;
mod link;
mod outline;
mod text;

pub use line::LineComponent;
pub use link::LinkComponent;
pub use outline::OutlineComponent;
pub use r#box::BoxComponent;
pub use text::TextComponent;

use crate::{Bounds, Margin, Padding, Rect};
use owned_ttf_parser::Face;
use printpdf::{Destination, IndirectFontRef, PdfLayerReference, PdfPageIndex};

/// Abstraction of a component that can be drawn on a PDF.
pub trait Component: Bounds + Clone {
    /// Draws the component within the PDF.
    fn draw(&self, ctx: &Context<'_>);

    /// Returns the padding associated with the component.
    fn padding(&self) -> Option<Padding> {
        None
    }

    /// Returns the margin associated with the component.
    fn margin(&self) -> Option<Margin> {
        None
    }

    /// Returns bounds adjusted to account for margin.
    fn outer_bounds(&self) -> Rect {
        let (mut llx, mut lly, mut urx, mut ury) = self.bounds().to_coords();

        if let Some(margin) = self.margin() {
            lly += margin.bottom;
            ury -= margin.top;
            llx += margin.left;
            urx -= margin.right;
        }

        Rect::from_coords(llx, lly, urx, ury)
    }

    /// Returns bounds adjusted to account for margin and padding.
    fn inner_bounds(&self) -> Rect {
        let (mut llx, mut lly, mut urx, mut ury) = self.outer_bounds().to_coords();

        if let Some(padding) = self.padding() {
            lly += padding.bottom;
            ury -= padding.top;
            llx += padding.left;
            urx -= padding.right;
        }

        Rect::from_coords(llx, lly, urx, ury)
    }
}

pub trait ComponentExt: Component {
    fn with_link(&self) -> LinkComponent<Self> {
        LinkComponent::new(self.clone())
    }

    fn with_go_to_link(&self, destination: impl Into<Destination>) -> LinkComponent<Self> {
        self.with_link().with_go_to_action(destination).clone()
    }

    fn with_go_to_xyz_link(
        &self,
        page: PdfPageIndex,
        left: Option<f32>,
        top: Option<f32>,
        zoom: Option<f32>,
    ) -> LinkComponent<Self> {
        self.with_go_to_link(Destination::XYZ {
            page,
            left,
            top,
            zoom,
        })
    }

    fn with_uri_link(&self, uri: impl Into<String>) -> LinkComponent<Self> {
        self.with_link().with_uri_action(uri).clone()
    }
}

impl<T: Component> ComponentExt for T {}

/// Context provided to a [`Component`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct Context<'a> {
    pub face: &'a Face<'a>,
    pub font: &'a IndirectFontRef,
    pub layer: &'a PdfLayerReference,
}
