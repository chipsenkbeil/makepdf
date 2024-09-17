use crate::constants::GLOBAL_PDF_VAR_NAME;
use crate::pdf::{
    PdfBounds, PdfColor, PdfConfig, PdfContext, PdfLink, PdfLinkAnnotation, PdfLuaExt,
    PdfLuaTableExt, PdfPoint,
};
use crate::runtime::{RuntimeFontId, RuntimeFonts};
use mlua::prelude::*;
use owned_ttf_parser::{Face, GlyphId};
use printpdf::{GlyphMetrics, Mm, Pt};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectText {
    pub point: PdfPoint,
    pub text: String,
    pub depth: Option<i64>,
    pub font: Option<RuntimeFontId>,
    pub size: Option<f32>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub link: Option<PdfLink>,
}

impl PdfObjectText {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext) {
        // Get optional values, setting defaults when not specified
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        let fill_color = self.fill_color.unwrap_or(ctx.config.page.fill_color);
        let outline_color = self.outline_color.unwrap_or(ctx.config.page.outline_color);
        let (x, y) = self.point.to_coords();

        // Retrieve the font to use for the text, leveraging the configured font first, otherwise
        // falling back to a default font
        if let Some(font_ref) = self
            .font
            .and_then(|id| ctx.fonts.get_font_doc_ref(id))
            .or_else(|| ctx.fonts.get_font_doc_ref(ctx.fallback_font_id))
        {
            ctx.layer.set_fill_color(fill_color.into());
            ctx.layer.set_outline_color(outline_color.into());
            ctx.layer.use_text(&self.text, size, x, y, font_ref);
        }
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

    /// Returns bounds for the text by calculating the width and height and applying to
    /// get the upper-right point.
    pub fn bounds(&self, ctx: PdfContext) -> PdfBounds {
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        if let Some(face) = self
            .font
            .and_then(|id| ctx.fonts.get_font_face(id))
            .or_else(|| ctx.fonts.get_font_face(ctx.fallback_font_id))
        {
            bounds(&self.text, face, size, self.point.x, self.point.y)
        } else {
            unreachable!("Fallback font should always be available");
        }
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
        let text = self.text.to_string();

        self.point.add_to_table(&table)?;
        table.raw_set("text", self.text)?;
        table.raw_set("size", self.size)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("font", self.font)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("link", self.link)?;

        // Add specialized methods to calculate the bounds, width, and height of the text
        // by looking up the global config, grabbing the default font size, and accessing
        // the repository of fonts to get the information needed for the current text.
        //
        // This only shows the bounds for the point in time. If the defaults change or the
        // text object itself changes, this will not update with it. To refresh the function,
        // the text object needs to be recreated.
        metatable.raw_set(
            "bounds",
            lua.create_function(move |lua, this: Option<Self>| {
                // Figure out the font's size by loading the explicit size or searching our global
                // pdf instance for the default page font size
                let font_size = match this.as_ref().and_then(|this| this.size).or(self.size) {
                    Some(size) => size,
                    None => {
                        lua.globals()
                            .raw_get::<_, PdfConfig>(GLOBAL_PDF_VAR_NAME)?
                            .page
                            .font_size
                    }
                };

                // Retrieve the loaded fonts so we can figure out the actual text bounds
                // for the associated font
                if let Some(fonts) = lua.app_data_ref::<RuntimeFonts>() {
                    let font_id = match this.as_ref().and_then(|this| this.font).or(self.font) {
                        Some(id) => Some(id),
                        None => fonts.fallback_font_id(),
                    };

                    let point = this.as_ref().map(|this| this.point).unwrap_or(self.point);
                    if let Some(face) = font_id.and_then(|id| fonts.get_font_face(id)) {
                        Ok(bounds(&text, face, font_size, point.x, point.y))
                    } else {
                        Err(LuaError::runtime("Runtime fallback font is missing"))
                    }
                } else {
                    Err(LuaError::runtime("Runtime fonts are missing"))
                }
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
                let maybe_points: Option<Vec<PdfPoint>> = table
                    .clone()
                    .sequence_values()
                    .collect::<LuaResult<_>>()
                    .ok();

                // Check if the first argument in sequence is a point, otherwise
                // we assume that the point was flattened into the object
                let point = match maybe_points.and_then(|p| p.into_iter().next()) {
                    Some(p) => p,
                    None => PdfPoint::from_lua(LuaValue::Table(table.clone()), lua)?,
                };

                Ok(Self {
                    point,
                    text: table.raw_get_ext("text")?,
                    size: table.raw_get_ext("size")?,
                    depth: table.raw_get_ext("depth")?,
                    font: table.raw_get_ext("font")?,
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

/// Returns bounds for the text by calculating the width and height and applying to
/// get the upper-right point.
fn bounds(text: &str, face: &Face, font_size: f32, baseline_x: Mm, baseline_y: Mm) -> PdfBounds {
    let x = baseline_x;
    let y = text_ll_y(face, font_size, baseline_y);
    let width = text_width(text, face, font_size);
    let height = text_height(face, font_size);
    PdfBounds::from_coords(x, y, x + width, y + height)
}

/// Returns the width of the text in millimeters for the given font face.
fn text_width(text: &str, face: &Face, font_size: f32) -> Mm {
    let units_per_em = face.units_per_em() as f64;
    let scale = font_size as f64 / units_per_em;

    // Calculate the total width of the text
    let text_width = text
        .chars()
        .map(|ch| {
            glyph_metrics(face, ch as u16)
                .map(|glyph| glyph.width as f64 * scale)
                .unwrap_or(0.0)
        })
        .sum::<f64>();

    Pt(text_width as f32).into()
}

/// Returns the height of the text in millimeters for the given font face.
fn text_height(face: &Face, font_size: f32) -> Mm {
    let units_per_em = face.units_per_em() as f64;
    let ascender = face.ascender() as f64;
    let descender = face.descender() as f64;
    let line_gap = face.line_gap() as f64;

    // Calculate the total height of the text
    let text_height = (ascender - descender + line_gap) * (font_size as f64 / units_per_em);

    Pt(text_height as f32).into()
}

/// Returns true lower-left y position of text, accounting for descenders (like `p` and `g`).
fn text_ll_y(face: &Face, font_size: f32, baseline_y: Mm) -> Mm {
    let units_per_em = face.units_per_em() as f64;
    let descender = face.descender() as f64;

    // Calculate the descender max size
    let descender_mm: Mm = Pt((descender * (font_size as f64) / units_per_em) as f32).into();

    // NOTE: For some reason, I need to add instead of subtract the descender
    //       because the above seems to be yielding a negative descender.
    //
    //       I believe this is because the baseline is considered origin (y=0),
    //       so going below it would yield a negative value.
    baseline_y + descender_mm
}
