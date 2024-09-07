mod r#box;
mod link;
mod text;

pub use link::LinkComponent;
pub use r#box::BoxComponent;
pub use text::TextComponent;

use crate::{WithBounds, WithPadding};
use owned_ttf_parser::Face;
use printpdf::{Destination, IndirectFontRef, PdfLayerReference, PdfPageIndex};

/// Abstraction of a component that can be drawn on a PDF.
pub trait Component: Clone + WithBounds + WithPadding {
    /// Draws the component within the PDF.
    fn draw(&self, ctx: &Context<'_>);
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
