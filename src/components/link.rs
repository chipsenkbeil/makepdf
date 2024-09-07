use crate::{Component, Context, Padding, Rect, WithBounds, WithPadding, WithPaddingExt};
use printpdf::{Actions, Destination, LinkAnnotation};

/// Represents a component that provides a link around some other [`Component`].
#[derive(Clone, Debug)]
pub struct LinkComponent<T: Component> {
    inner: T,
    action: Option<Actions>,
}

impl<T: Component> LinkComponent<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            action: None,
        }
    }

    pub fn with_uri_action(&mut self, uri: impl Into<String>) -> &mut Self {
        self.with_action(Actions::uri(uri.into()))
    }

    pub fn with_go_to_action(&mut self, destination: impl Into<Destination>) -> &mut Self {
        self.with_action(Actions::go_to(destination.into()))
    }

    pub fn with_action(&mut self, action: impl Into<Actions>) -> &mut Self {
        self.action = Some(action.into());
        self
    }

    pub fn with_no_action(&mut self) -> &mut Self {
        self.action = None;
        self
    }
}

impl<T: Component> Component for LinkComponent<T> {
    fn draw(&self, ctx: &Context<'_>) {
        // Draw the component we are wrapping with an action
        self.inner.draw(ctx);

        // If we have an action associated (for a link), create a new annotation
        //
        // NOTE: Components should only be drawn once, so we don't need to worry
        //       about the annotation being added multiple times.
        if let Some(action) = self.action.clone() {
            ctx.layer.add_link_annotation(LinkAnnotation::new(
                self.bounds_with_padding().into(),
                None,
                None,
                action,
                None,
            ));
        }
    }
}

impl<T: Component> WithBounds for LinkComponent<T> {
    fn bounds(&self) -> Rect {
        self.inner.bounds()
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.inner.set_bounds(bounds);
    }
}

impl<T: Component> WithPadding for LinkComponent<T> {
    fn padding(&self) -> Option<Padding> {
        self.inner.padding()
    }

    fn set_padding(&mut self, padding: Option<Padding>) {
        self.inner.set_padding(padding)
    }
}
