use crate::constants::*;
use crate::{
    Component, Context, Padding, Rect, TextComponent, WithBounds, WithBoundsExt, WithPadding,
    WithPaddingExt,
};
use printpdf::Color;

#[derive(Clone, Debug)]
pub struct BoxComponent {
    background: Color,
    foreground: Color,
    padding: Option<Padding>,
    rect: Rect,
    text: String,
    text_size: f32,
}

impl Default for BoxComponent {
    fn default() -> Self {
        Self {
            background: BANNER_BACKGROUND_COLOR,
            foreground: BANNER_TEXT_COLOR,
            padding: None,
            rect: Rect::default(),
            text: String::default(),
            text_size: 36.0,
        }
    }
}

impl BoxComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_background(&mut self, color: Color) -> &mut Self {
        self.background = color;
        self
    }

    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        self.foreground = color;
        self
    }

    pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn with_text_size(&mut self, size: impl Into<f32>) -> &mut Self {
        self.text_size = size.into();
        self
    }
}

impl Component for BoxComponent {
    fn draw(&self, ctx: &Context<'_>) {
        // Render a rectangle representing the box
        ctx.layer.set_fill_color(self.background.clone());
        ctx.layer.set_outline_color(self.background.clone());
        ctx.layer.add_rect(self.bounds_with_padding().into());

        // Render text within the padded rect if we have text to render
        if !self.text.is_empty() {
            TextComponent::new()
                .with_text(&self.text)
                .with_text_size(self.text_size)
                .with_bounds(self.bounds_with_padding())
                .draw(ctx);
        }
    }
}

impl WithBounds for BoxComponent {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.rect = bounds;
    }
}

impl WithPadding for BoxComponent {
    fn padding(&self) -> Option<Padding> {
        self.padding
    }

    fn set_padding(&mut self, padding: Option<Padding>) {
        self.padding = padding
    }
}
