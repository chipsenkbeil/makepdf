use crate::pdf::{PdfColor, PdfContext, PdfLuaTableExt, PdfPoint};
use mlua::prelude::*;
use owned_ttf_parser::{Face, GlyphId};
use printpdf::{GlyphMetrics, Mm, Pt};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectText {
    pub point: PdfPoint,
    pub text: String,
    pub size: Option<f32>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
}

impl PdfObjectText {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfContext<'_>) {
        // Get optional values, setting defaults when not specified
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        let fill_color = self.fill_color.unwrap_or(ctx.config.page.fill_color);
        let outline_color = self.outline_color.unwrap_or(ctx.config.page.outline_color);
        let (x, y) = self.point.to_coords();

        // Set color and render text
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
        ctx.layer.use_text(&self.text, size, x, y, ctx.font);
    }

    /// Returns the width of the text in millimeters for the given font face.
    pub fn text_width(&self, ctx: &PdfContext<'_>) -> Mm {
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        let units_per_em = ctx.face.units_per_em() as f64;
        let scale = size as f64 / units_per_em;
        Pt(self
            .text
            .chars()
            .map(|ch| {
                glyph_metrics(ctx.face, ch as u16)
                    .map(|glyph| glyph.width as f64 * scale)
                    .unwrap_or(0.0)
            })
            .sum::<f64>() as f32)
        .into()
    }

    /// Returns the height of the text in millimeters for the given font face.
    pub fn text_height(&self, ctx: &PdfContext<'_>) -> Mm {
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        let units_per_em = ctx.face.units_per_em() as f64;
        let ascender = ctx.face.ascender() as f64;
        let descender = ctx.face.descender() as f64;
        let line_gap = ctx.face.line_gap() as f64;

        // Calculate the total height of the text
        let text_height = (ascender - descender + line_gap) * (size as f64 / units_per_em);

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

        self.point.add_to_table(&table)?;
        table.raw_set("text", self.text)?;
        table.raw_set("size", self.size)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectText {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                let point = PdfPoint::from_lua(LuaValue::Table(table.clone()), lua)?;
                Ok(Self {
                    point,
                    text: table.raw_get_ext("text")?,
                    size: table.raw_get_ext("size")?,
                    fill_color: table.raw_get_ext("fill_color")?,
                    outline_color: table.raw_get_ext("outline_color")?,
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
