use crate::pdf::{PdfBounds, PdfColor, PdfLuaTableExt, PdfObjectContext};
use mlua::prelude::*;
use owned_ttf_parser::{Face, GlyphId};
use printpdf::{GlyphMetrics, Mm, Pt};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectText {
    /// Color of the text
    pub color: PdfColor,

    /// Bounds within to render text
    pub bounds: PdfBounds,

    /// Text to render
    pub text: String,

    /// Font size of text
    pub size: f32,
}

impl PdfObjectText {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        let (llx, lly, urx, ury) = self.bounds.to_coords();

        // If given text, we'll populate within the middle of the bounds
        if !self.text.is_empty() {
            let font_size: f32 = self.size;
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

            ctx.layer.set_fill_color(self.color.into());
            ctx.layer.set_outline_color(self.color.into());
            ctx.layer.use_text(&self.text, font_size, x, y, ctx.font);
        }
    }

    /// Returns the width of the text in millimeters for the given font face.
    pub fn text_width_for_face(&self, face: &Face<'_>) -> Mm {
        let units_per_em = face.units_per_em() as f64;
        let scale = self.size as f64 / units_per_em;
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
        let text_height = (ascender - descender + line_gap) * (self.size as f64 / units_per_em);

        Pt(text_height as f32).into()
    }
}

fn glyph_metrics(face: &Face<'_>, glyph_id: u16) -> Option<GlyphMetrics> {
    let glyph_id = GlyphId(glyph_id);
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

impl<'lua> IntoLua<'lua> for PdfObjectText {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        self.bounds.add_to_table(&table)?;
        table.raw_set("color", self.color)?;
        table.raw_set("text", self.text)?;
        table.raw_set("size", self.size)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectText {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                let bounds = PdfBounds::from_lua(LuaValue::Table(table.clone()), lua)?;
                Ok(Self {
                    bounds,
                    color: table.raw_get_ext("color")?,
                    text: table.raw_get_ext("text")?,
                    size: table.raw_get_ext("size")?,
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.text",
                message: None,
            }),
        }
    }
}
