use crate::constants::*;
use crate::{Component, Context, Padding, Rect, WithBounds, WithPadding, WithPaddingExt};
use owned_ttf_parser::Face;
use printpdf::{Color, GlyphMetrics, Mm};

#[derive(Clone, Debug)]
pub struct TextComponent {
    foreground: Color,
    padding: Option<Padding>,
    rect: Rect,
    text: String,
    text_size: f32,
}

impl Default for TextComponent {
    fn default() -> Self {
        Self {
            foreground: BANNER_TEXT_COLOR,
            padding: None,
            rect: Rect::default(),
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
        let mm = self
            .text
            .chars()
            .map(|ch| {
                glyph_metrics(face, ch as u16)
                    .map(|glyph| glyph.width as f64 * scale)
                    .unwrap_or(0.0)
            })
            .sum::<f64>()
            * PT_TO_MM;

        Mm(mm as f32)
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

        // Convert to millimeters (1 point = 0.352778 mm)
        Mm((text_height * PT_TO_MM) as f32)
    }
}

impl Component for TextComponent {
    fn draw(&self, ctx: &Context<'_>) {
        let (llx, lly, urx, ury) = self.bounds_with_padding().to_coords();

        // If given text, we'll populate within the middle of the banner
        if !self.text.is_empty() {
            let font_size: f32 = self.text_size;
            let text_width = self.text_width_for_face(ctx.face);
            let text_height = self.text_height_for_face(ctx.face);

            // Calculate the middle of the banner and then shift over by half the text length to
            // place it roughly within the middle of the banner itself
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
}

impl WithBounds for TextComponent {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.rect = bounds;
    }
}

impl WithPadding for TextComponent {
    fn padding(&self) -> Option<Padding> {
        self.padding
    }

    fn set_padding(&mut self, padding: Option<Padding>) {
        self.padding = padding
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
