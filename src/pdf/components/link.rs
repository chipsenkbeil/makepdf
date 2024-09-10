use crate::{Bounds, Component, Context, Margin, Padding, Rect};
use printpdf::{Actions, Destination, LinkAnnotation};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

/// Represents a component that provides a link around some other [`Component`].
#[derive(Clone, Debug)]
pub struct LinkComponent<T: Component> {
    inner: T,
    action: RefCell<Option<Actions>>,
}

impl<T: Component> Deref for LinkComponent<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Component> DerefMut for LinkComponent<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Component> LinkComponent<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            action: RefCell::new(None),
        }
    }

    pub fn with_uri_action(&mut self, uri: impl Into<String>) -> &mut Self {
        self.with_action(Actions::uri(uri.into()))
    }

    pub fn with_go_to_action(&mut self, destination: impl Into<Destination>) -> &mut Self {
        self.with_action(Actions::go_to(destination.into()))
    }

    pub fn with_action(&mut self, action: impl Into<Actions>) -> &mut Self {
        *self.action.borrow_mut() = Some(action.into());
        self
    }

    pub fn with_no_action(&mut self) -> &mut Self {
        *self.action.borrow_mut() = None;
        self
    }
}

impl<T: Component> Bounds for LinkComponent<T> {
    fn bounds(&self) -> Rect {
        self.inner.bounds()
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.inner.set_bounds(rect)
    }
}

impl<T: Component> Component for LinkComponent<T> {
    fn draw(&self, ctx: &Context<'_>) {
        // Draw the component we are wrapping with an action
        self.inner.draw(ctx);

        // If we have an action associated (for a link), create a new annotation.
        //
        // NOTE: We should only do this once, so we leverage a RefCell that
        //       contains the action to add the link, removing it after
        //       the first time the link annotation is added.
        if let Some(action) = self.action.borrow_mut().take() {
            ctx.layer.add_link_annotation(LinkAnnotation::new(
                self.outer_bounds().into(),
                None,
                None,
                action,
                None,
            ));
        }
    }

    fn margin(&self) -> Option<Margin> {
        self.inner.margin()
    }

    fn padding(&self) -> Option<Padding> {
        self.inner.padding()
    }
}
