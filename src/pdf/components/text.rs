use crate::constants::*;
use crate::{Bounds, Component, Context, Margin, Rect};
use owned_ttf_parser::Face;
use printpdf::{Color, GlyphMetrics, Mm, Pt};
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct TextComponent {
    foreground: Color,
    margin: Option<Margin>,
    bounds: Rect,
    text: String,
    text_size: f32,
}

impl Default for TextComponent {
    fn default() -> Self {
        Self {
            foreground: BANNER_TEXT_COLOR,
            margin: None,
            bounds: Rect::default(),
            text: String::default(),
            text_size: 36.0,
        }
    }
}

impl TextComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        self.foreground = color;
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

    pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn with_text_size(&mut self, size: impl Into<f32>) -> &mut Self {
        self.text_size = size.into();
        self
    }

    /// Returns the width of the text in millimeters for the given font face.
    pub fn text_width_for_face(&self, face: &Face<'_>) -> Mm {
        let units_per_em = face.units_per_em() as f64;
        let scale = self.text_size as f64 / units_per_em;
        Pt(self
            .text
            .chars()
            .map(|ch| {
                glyph_metrics(face, ch as u16)
                    .map(|glyph| glyph.width as f64 * scale)
                    .unwrap_or(0.0)
            })
            .sum::<f64>() as f32)
        .into()
    }

    /// Returns the height of the text in millimeters for the given font face.
    pub fn text_height_for_face(&self, face: &Face<'_>) -> Mm {
        let units_per_em = face.units_per_em() as f64;
        let ascender = face.ascender() as f64;
        let descender = face.descender() as f64;
        let line_gap = face.line_gap() as f64;

        // Calculate the total height of the text
        let text_height =
            (ascender - descender + line_gap) * (self.text_size as f64 / units_per_em);

        Pt(text_height as f32).into()
    }
}

impl From<&str> for TextComponent {
    fn from(s: &str) -> Self {
        Self::new().with_text(s).clone()
    }
}

impl From<String> for TextComponent {
    fn from(s: String) -> Self {
        Self::new().with_text(s).clone()
    }
}

impl FromStr for TextComponent {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Bounds for TextComponent {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.bounds = rect;
    }
}

impl Component for TextComponent {
    fn draw(&self, ctx: &Context<'_>) {
        let (llx, lly, urx, ury) = self.outer_bounds().to_coords();

        // If given text, we'll populate within the middle of the bounds
        if !self.text.is_empty() {
            let font_size: f32 = self.text_size;
            let text_width = self.text_width_for_face(ctx.face);
            let text_height = self.text_height_for_face(ctx.face);

            // Calculate the middle of the bounds and then shift over by half the text length to
            // place it roughly within the middle of the bounds itself
            let x = llx + ((urx - llx) / 2.0) - (text_width / 2.0);

            // Calculate the space remaining from height of text and move up to vertically center.
            // We do this by calculating the position vertically centered within the box and then
            // moving down to account for a portion of the text's height.
            //
            // NOTE: I don't really understand this anymore. I was trying text_height / 2.0 and
            //       that didn't work, but using a quarter of text height gets pretty close.
            let y = lly + ((ury - lly) / 2.0) - (text_height / 4.0);

            ctx.layer.set_fill_color(self.foreground.clone());
            ctx.layer.set_outline_color(self.foreground.clone());
            ctx.layer.use_text(&self.text, font_size, x, y, ctx.font);
        }
    }

    fn margin(&self) -> Option<Margin> {
        self.margin
    }
}

fn glyph_metrics(face: &Face<'_>, glyph_id: u16) -> Option<GlyphMetrics> {
    let glyph_id = owned_ttf_parser::GlyphId(glyph_id);
    if let Some(width) = face.glyph_hor_advance(glyph_id) {
        let width = width as u32;
        let height = face
            .glyph_bounding_box(glyph_id)
            .map(|bbox| bbox.y_max - bbox.y_min - face.descender())
            .unwrap_or(1000) as u32;
        Some(GlyphMetrics { width, height })
    } else {
        None
    }
}
