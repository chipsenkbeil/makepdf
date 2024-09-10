use crate::constants::*;
use crate::{Bounds, Component, Context, Margin, Padding, Rect, TextComponent};
use printpdf::Color;

#[derive(Clone, Debug)]
pub struct BoxComponent {
    background: Color,
    foreground: Color,
    padding: Option<Padding>,
    margin: Option<Margin>,
    bounds: Rect,
    text: String,
    text_size: f32,
}

impl Default for BoxComponent {
    fn default() -> Self {
        Self {
            background: BANNER_BACKGROUND_COLOR,
            foreground: BANNER_TEXT_COLOR,
            padding: None,
            margin: None,
            bounds: Rect::default(),
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

    pub fn with_padding(&mut self, padding: impl Into<Padding>) -> &mut Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn with_no_padding(&mut self) -> &mut Self {
        self.padding = None;
        self
    }

    pub fn with_margin(&mut self, margin: impl Into<Margin>) -> &mut Self {
        self.margin = Some(margin.into());
        self
    }

    pub fn with_no_margin(&mut self) -> &mut Self {
        self.margin = None;
        self
    }
}

impl Bounds for BoxComponent {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.bounds = rect;
    }
}

impl Component for BoxComponent {
    fn draw(&self, ctx: &Context<'_>) {
        // Render a rectangle representing the box
        ctx.layer.set_fill_color(self.background.clone());
        ctx.layer.set_outline_color(self.background.clone());
        ctx.layer.add_rect(self.outer_bounds().into());

        // Render text within the padded rect if we have text to render
        if !self.text.is_empty() {
            TextComponent::new()
                .with_text(&self.text)
                .with_text_size(self.text_size)
                .with_bounds(self.inner_bounds())
                .draw(ctx);
        }
    }

    fn margin(&self) -> Option<Margin> {
        self.margin
    }

    fn padding(&self) -> Option<Padding> {
        self.padding
    }
}
