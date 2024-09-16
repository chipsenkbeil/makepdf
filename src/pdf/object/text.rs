use crate::pdf::{
    PdfBounds, PdfColor, PdfContext, PdfLink, PdfLinkAnnotation, PdfLuaExt, PdfLuaTableExt,
    PdfPoint,
};
use crate::runtime::RuntimeDocFont;
use mlua::prelude::*;
use owned_ttf_parser::{Face, GlyphId};
use printpdf::{GlyphMetrics, Mm, Pt};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectText {
    pub point: PdfPoint,
    pub text: String,
    pub depth: Option<i64>,
    pub size: Option<f32>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub link: Option<PdfLink>,
}

impl PdfObjectText {
    /// Returns bounds for the text by calculating the width and height and applying to
    /// get the upper-right point.
    pub fn bounds(&self, ctx: PdfContext) -> PdfBounds {
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        self.bounds_impl(ctx.font, size)
    }

    /// Returns a collection of link annotations.
    pub fn link_annotations(&self, ctx: PdfContext) -> Vec<PdfLinkAnnotation> {
        match self.link.clone() {
            Some(link) => vec![PdfLinkAnnotation {
                bounds: self.bounds(ctx),
                depth: self.depth.unwrap_or_default(),
                link,
            }],
            None => Vec::new(),
        }
    }

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext) {
        // Get optional values, setting defaults when not specified
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        let fill_color = self.fill_color.unwrap_or(ctx.config.page.fill_color);
        let outline_color = self.outline_color.unwrap_or(ctx.config.page.outline_color);
        let (x, y) = self.point.to_coords();

        // Set color and render text
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
        ctx.layer
            .use_text(&self.text, size, x, y, ctx.as_font_ref());
    }

    /// Returns bounds for the text by calculating the width and height and applying to
    /// get the upper-right point.
    fn bounds_impl(&self, font: &RuntimeDocFont, default_size: f32) -> PdfBounds {
        let x = self.point.x;
        let y = self.text_ll_y(font, default_size);
        let width = self.text_width(font, default_size);
        let height = self.text_height(font, default_size);
        PdfBounds::from_coords(x, y, x + width, y + height)
    }

    /// Returns the width of the text in millimeters for the given font face.
    fn text_width(&self, font: &RuntimeDocFont, default_size: f32) -> Mm {
        let size = self.size.unwrap_or(default_size);
        let units_per_em = font.as_face().units_per_em() as f64;
        let scale = size as f64 / units_per_em;

        // Calculate the total width of the text
        let text_width = self
            .text
            .chars()
            .map(|ch| {
                glyph_metrics(font.as_face(), ch as u16)
                    .map(|glyph| glyph.width as f64 * scale)
                    .unwrap_or(0.0)
            })
            .sum::<f64>();

        Pt(text_width as f32).into()
    }

    /// Returns the height of the text in millimeters for the given font face.
    fn text_height(&self, font: &RuntimeDocFont, default_size: f32) -> Mm {
        let size = self.size.unwrap_or(default_size);
        let units_per_em = font.as_face().units_per_em() as f64;
        let ascender = font.as_face().ascender() as f64;
        let descender = font.as_face().descender() as f64;
        let line_gap = font.as_face().line_gap() as f64;

        // Calculate the total height of the text
        let text_height = (ascender - descender + line_gap) * (size as f64 / units_per_em);

        Pt(text_height as f32).into()
    }

    /// Returns true lower-left y position of text, accounting for descenders (like `p` and `g`).
    fn text_ll_y(&self, font: &RuntimeDocFont, default_size: f32) -> Mm {
        let size = self.size.unwrap_or(default_size) as f64;
        let units_per_em = font.as_face().units_per_em() as f64;
        let descender = font.as_face().descender() as f64;

        // Calculate the descender max size
        let descender_mm: Mm = Pt((descender * size / units_per_em) as f32).into();

        // NOTE: For some reason, I need to add instead of subtract the descender
        //       because the above seems to be yielding a negative descender.
        //
        //       I believe this is because the baseline is considered origin (y=0),
        //       so going below it would yield a negative value.
        self.point.y + descender_mm
    }
}

fn glyph_metrics(face: &Face, glyph_id: u16) -> Option<GlyphMetrics> {
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
        let (table, metatable) = lua.create_table_ext()?;

        self.point.add_to_table(&table)?;
        table.raw_set("text", self.text)?;
        table.raw_set("size", self.size)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("link", self.link)?;

        // Add specialized methods to calculate the bounds, width, and height of the text
        // by looking up the global config, grabbing the default font size, and accessing
        // the repository of fonts to get the information needed for the current text.
        //
        // Each of these methods also needs to create a reference to the underlying table
        // so it can get the current size and text to use.
        let tbl = table.clone();
        metatable.raw_set(
            "bounds",
            lua.create_function(move |lua, ()| {
                let this = Self::from_lua(LuaValue::Table(tbl.clone()), lua)?;
                Ok(this.bounds())
            })?,
        )?;

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
                    depth: table.raw_get_ext("depth")?,
                    fill_color: table.raw_get_ext("fill_color")?,
                    outline_color: table.raw_get_ext("outline_color")?,
                    link: table.raw_get_ext("link")?,
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
