use crate::constants::*;
use crate::context::Context;
use owned_ttf_parser::Face;
use printpdf::*;

mod padding;
mod rect;

#[derive(Clone, Debug)]
pub struct PdfBox {
    background: Color,
    foreground: Color,
    padding: Option<padding::Padding>,
    rect: rect::Rect,
    text: String,
    text_size: f32,
}

impl Default for PdfBox {
    fn default() -> Self {
        Self {
            background: BANNER_BACKGROUND_COLOR,
            foreground: BANNER_TEXT_COLOR,
            padding: None,
            rect: rect::Rect::default(),
            text: String::default(),
            text_size: 36.0,
        }
    }
}

impl PdfBox {
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

    pub fn with_padding(&mut self, padding: impl Into<padding::Padding>) -> &mut Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn with_no_padding(&mut self) -> &mut Self {
        self.padding = None;
        self
    }

    pub fn with_rect(&mut self, rect: impl Into<rect::Rect>) -> &mut Self {
        self.rect = rect.into();
        self
    }

    pub fn with_full_width(&mut self) -> &mut Self {
        self.rect.width = PAGE_WIDTH;
        self
    }

    pub fn with_three_quarters_width(&mut self) -> &mut Self {
        self.rect.width = PAGE_WIDTH * 3.0 / 4.0;
        self
    }

    pub fn with_half_width(&mut self) -> &mut Self {
        self.rect.width = PAGE_WIDTH / 2.0;
        self
    }

    pub fn with_quarter_width(&mut self) -> &mut Self {
        self.rect.width = PAGE_WIDTH / 4.0;
        self
    }

    pub fn with_eighth_width(&mut self) -> &mut Self {
        self.rect.width = PAGE_WIDTH / 8.0;
        self
    }

    pub fn shift_three_quarters_right(&mut self) -> &mut Self {
        self.shift_half_right();
        self.shift_quarter_right();
        self
    }

    pub fn shift_half_right(&mut self) -> &mut Self {
        self.shift_quarter_right();
        self.shift_quarter_right();
        self
    }

    pub fn shift_quarter_right(&mut self) -> &mut Self {
        self.shift_eighth_right();
        self.shift_eighth_right();
        self
    }

    pub fn shift_eighth_right(&mut self) -> &mut Self {
        self.rect.x += PAGE_WIDTH / 8.0;
        self
    }

    /// Columns are up to 8 with `col` being zero-indexed.
    pub fn at_col(&mut self, col: usize) -> &mut Self {
        let col = col as f32;
        self.rect.x = Mm(col * (PAGE_WIDTH.0 / 8.0));
        self
    }

    pub fn at_row(&mut self, row: usize) -> &mut Self {
        self.rect.y = PAGE_HEIGHT - (ROW_HEIGHT * (row + 1) as f32);
        self.rect.height = ROW_HEIGHT;
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

    pub fn draw(&self, ctx: &Context<'_>) {
        let mut llx = self.rect.x;
        let mut lly = self.rect.y;
        let mut urx = self.rect.x + self.rect.width;
        let mut ury = self.rect.y + self.rect.height;

        // Calculate the padding
        if let Some(padding) = self.padding {
            lly += padding.bottom;
            ury -= padding.top;
            llx += padding.left;
            urx -= padding.right;
        }

        ctx.layer.set_fill_color(self.background.clone());
        ctx.layer.set_outline_color(self.background.clone());
        ctx.layer.add_rect(Rect::new(llx, lly, urx, ury));

        // If given text, we'll populate within the middle of the banner
        if !self.text.is_empty() {
            let font_size: f32 = self.text_size;
            let text_width = text_width_in_mm(&self.text, ctx.face, font_size as f64) as f32;
            let text_height = text_height_in_mm(ctx.face, font_size as f64) as f32;

            // Calculate the middle of the banner and then shift over by half the text length to
            // place it roughly within the middle of the banner itself
            let x = llx + ((urx - llx) / 2.0) - Mm(text_width / 2.0);

            // Calculate the space remaining from height of text and move up to vertically center.
            // We do this by calculating the position vertically centered within the box and then
            // moving down to account for a portion of the text's height.
            //
            // NOTE: I don't really understand this anymore. I was trying text_height / 2.0 and
            //       that didn't work, but using a quarter of text height gets pretty close.
            let y = lly + ((ury - lly) / 2.0) - Mm(text_height / 4.0);

            ctx.layer.set_fill_color(self.foreground.clone());
            ctx.layer.set_outline_color(self.foreground.clone());
            ctx.layer.use_text(&self.text, font_size, x, y, ctx.font);
        }
    }
}

fn text_width_in_mm(text: &str, face: &Face<'_>, font_size: f64) -> f64 {
    let units_per_em = face.units_per_em() as f64;
    let scale = font_size / units_per_em;

    text.chars()
        .map(|ch| {
            glyph_metrics(face, ch as u16)
                .map(|glyph| glyph.width as f64 * scale)
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        * 0.352778 // Convert points to millimeters
}

fn text_height_in_mm(face: &Face<'_>, font_size: f64) -> f64 {
    let units_per_em = face.units_per_em() as f64;
    let ascender = face.ascender() as f64;
    let descender = face.descender() as f64;
    let line_gap = face.line_gap() as f64;

    // Calculate the total height of the text
    let text_height = (ascender - descender + line_gap) * (font_size / units_per_em);

    // Convert to millimeters (1 point = 0.352778 mm)
    text_height * 0.352778
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
