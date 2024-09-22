use crate::constants::GLOBAL_PDF_VAR_NAME;
use crate::pdf::{
    PdfAlign, PdfBounds, PdfColor, PdfConfig, PdfContext, PdfHorizontalAlign, PdfLink,
    PdfLinkAnnotation, PdfLuaExt, PdfLuaTableExt, PdfObjectType, PdfPoint, PdfVerticalAlign,
};
use crate::runtime::{RuntimeFontId, RuntimeFonts};
use mlua::prelude::*;
use owned_ttf_parser::{Face, GlyphId};
use printpdf::{GlyphMetrics, Mm, Pt};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfObjectText {
    pub point: PdfPoint,
    pub text: String,
    pub depth: Option<i64>,
    pub font: Option<RuntimeFontId>,
    pub size: Option<f32>,
    pub color: Option<PdfColor>,
    pub link: Option<PdfLink>,
}

impl PdfObjectText {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext) {
        // Get optional values, setting defaults when not specified
        let size = self.size.unwrap_or(ctx.config.page.font_size);
        let fill_color = self.color.unwrap_or(ctx.config.page.fill_color);
        let (x, y) = self.point.to_coords();

        // Retrieve the font to use for the text, leveraging the configured font first, otherwise
        // falling back to a default font
        if let Some(font_ref) = self
            .font
            .and_then(|id| ctx.fonts.get_font_doc_ref(id))
            .or_else(|| ctx.fonts.get_font_doc_ref(ctx.fallback_font_id))
        {
            ctx.layer.set_fill_color(fill_color.into());
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

    /// Aligns the text to a set of bounds.
    pub fn align_to(
        &mut self,
        ctx: PdfContext,
        bounds: PdfBounds,
        align: (PdfVerticalAlign, PdfHorizontalAlign),
    ) {
        // Get new bounds for the text
        let src_bounds = self.bounds(ctx);
        let dst_bounds = src_bounds.align_to(bounds, align);

        // Figure out changes from original bounds of points
        let x_offset = dst_bounds.width() - src_bounds.width();
        let y_offset = dst_bounds.height() - src_bounds.height();

        // Apply the changes to the text coordinates
        self.point.x += x_offset;
        self.point.y += y_offset;
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

    /// Returns bounds for the text by calculating the width and height and applying to get the
    /// upper-right-point.
    ///
    /// Calculates bounds from a [`Lua`] runtime, which occurs earlier than when a [`PdfContext`]
    /// is available.
    pub(crate) fn lua_bounds(&self, lua: &Lua) -> LuaResult<PdfBounds> {
        // Figure out the font's size by loading the explicit size or searching our global
        // pdf instance for the default page font size
        let font_size = match self.size {
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
            let font_id = match self.font {
                Some(id) => Some(id),
                None => fonts.fallback_font_id(),
            };

            if let Some(face) = font_id.and_then(|id| fonts.get_font_face(id)) {
                Ok(bounds(
                    &self.text,
                    face,
                    font_size,
                    self.point.x,
                    self.point.y,
                ))
            } else {
                Err(LuaError::runtime("Runtime fallback font is missing"))
            }
        } else {
            Err(LuaError::runtime("Runtime fonts are missing"))
        }
    }

    /// Aligns the text to a set of bounds.
    ///
    /// Calculates bounds from a [`Lua`] runtime, which occurs earlier than when a [`PdfContext`]
    /// is available.
    pub(crate) fn lua_align_to(
        &mut self,
        lua: &Lua,
        bounds: PdfBounds,
        align: (PdfVerticalAlign, PdfHorizontalAlign),
    ) -> LuaResult<()> {
        // Get new bounds for the text
        let src_bounds = self.lua_bounds(lua)?;
        let dst_bounds = src_bounds.align_to(bounds, align);

        // Figure out the shift from original to new bounds
        let x_offset = dst_bounds.ll.x - src_bounds.ll.x;
        let y_offset = dst_bounds.ll.y - src_bounds.ll.y;

        // Apply the changes to the text coordinates
        self.point.x += x_offset;
        self.point.y += y_offset;

        Ok(())
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
        table.raw_set("type", PdfObjectType::Text)?;
        table.raw_set("text", self.text)?;
        table.raw_set("size", self.size)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("font", self.font)?;
        table.raw_set("color", self.color)?;
        table.raw_set("link", self.link)?;

        metatable.raw_set(
            "align_to",
            lua.create_function(
                move |lua, (mut this, bounds, align): (Self, PdfBounds, PdfAlign)| {
                    this.lua_align_to(lua, bounds, align.to_v_h())?;
                    Ok(this)
                },
            )?,
        )?;

        metatable.raw_set(
            "bounds",
            lua.create_function(move |lua, this: Self| this.lua_bounds(lua))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectText {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                // Support missing point converting into default point
                //
                // TODO: This will result in invalid point becoming default point.
                //       We want to correct this to support missing point only.
                let point = match PdfPoint::from_lua(LuaValue::Table(table.clone()), lua) {
                    Ok(pt) => pt,
                    Err(_) => table
                        .clone()
                        .sequence_values::<PdfPoint>()
                        .next()
                        .transpose()
                        .ok()
                        .flatten()
                        .unwrap_or_default(),
                };

                Ok(Self {
                    point,
                    text: table
                        .raw_get_ext::<_, Option<_>>("text")?
                        .unwrap_or_default(),
                    size: table.raw_get_ext("size")?,
                    depth: table.raw_get_ext("depth")?,
                    font: table.raw_get_ext("font")?,
                    color: table.raw_get_ext("color")?,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::Pdf;
    use crate::runtime::RuntimeFonts;
    use mlua::chunk;
    use printpdf::{Mm, PdfDocument};

    #[test]
    fn should_be_able_to_align_text_to_some_bounds_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();
        lua.set_app_data({
            let mut fonts = RuntimeFonts::new();
            let id = fonts.add_builtin_font().unwrap();
            fonts.add_font_as_fallback(id);
            fonts
        });

        // Test the bounds, which should correctly cover full text
        lua.load(chunk! {
            local text = pdf.object.text({
                x = 0,
                y = 0,
                text = "hello world",
                size = 36.0,
            })

            // Assert the text is where we expect prior to alignment
            pdf.utils.assert_deep_equal(text:bounds(), {
                ll = { x = 0,                   y = -3.810002326965332 },
                ur = { x = 83.82005310058594,   y = 12.954007148742676 },
            })

            // Do the alignment with some bounds that are elsewhere
            text = text:align_to({
                ll = { x = 5,  y = 5 },
                ur = { x = 10, y = 10 },
            }, { v = "bottom", h = "left" })

            // Assert the text has moved into place
            pdf.utils.assert_deep_equal(text:bounds(), {
                ll = { x = 5,                   y = 5 },
                ur = { x = 88.82005310058594,   y = 21.764009475708008 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_text() {
        // Create a pdf context that we need for bounds calculations
        let doc = PdfDocument::empty("");
        let (page_idx, layer_idx) = doc.add_page(Mm(0.0), Mm(0.0), "");
        let layer = doc.get_page(page_idx).get_layer(layer_idx);
        let mut font = RuntimeFonts::new();
        let font_id = font.add_builtin_font().unwrap();
        font.add_font_as_fallback(font_id);
        let ctx = PdfContext {
            config: &PdfConfig::default(),
            layer: &layer,
            fonts: &font,
            fallback_font_id: font_id,
        };

        let text = PdfObjectText {
            point: PdfPoint::from_coords_f32(0.0, 0.0),
            text: String::from("hello world"),
            size: Some(36.0),
            ..Default::default()
        };

        assert_eq!(
            text.bounds(ctx),
            PdfBounds::from_coords_f32(0.0, -3.810_002_3, 83.820_05, 12.954_007)
        );
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_text_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();
        lua.set_app_data({
            let mut fonts = RuntimeFonts::new();
            let id = fonts.add_builtin_font().unwrap();
            fonts.add_font_as_fallback(id);
            fonts
        });

        // Test the bounds, which should correctly cover full text
        lua.load(chunk! {
            local text = pdf.object.text({
                x = 0,
                y = 0,
                text = "hello world",
                size = 36.0,
            })
            pdf.utils.assert_deep_equal(text:bounds(), {
                ll = { x = 0,                   y = -3.810002326965332 },
                ur = { x = 83.82005310058594,   y = 12.954007148742676 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        // Can convert from empty table into a text
        assert_eq!(
            Lua::new().load(chunk!({})).eval::<PdfObjectText>().unwrap(),
            PdfObjectText::default(),
        );

        // Can convert from a table with flattened point into text
        assert_eq!(
            Lua::new()
                .load(chunk!({ 1, 2 }))
                .eval::<PdfObjectText>()
                .unwrap(),
            PdfObjectText {
                point: PdfPoint::from_coords_f32(1.0, 2.0),
                ..Default::default()
            },
        );

        // Can convert from a table with simplified point into text
        assert_eq!(
            Lua::new()
                .load(chunk!({ { 1, 2 } }))
                .eval::<PdfObjectText>()
                .unwrap(),
            PdfObjectText {
                point: PdfPoint::from_coords_f32(1.0, 2.0),
                ..Default::default()
            },
        );

        // Can convert from a table with everything into a text
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    text = "hello world",
                    x = 1,
                    y = 2,
                    depth = 123,
                    font = 456,
                    size = 789,
                    color = "123456",
                    link = {
                        type = "uri",
                        uri = "https://example.com",
                    },
                }))
                .eval::<PdfObjectText>()
                .unwrap(),
            PdfObjectText {
                point: PdfPoint::from_coords_f32(1.0, 2.0),
                text: String::from("hello world"),
                depth: Some(123),
                font: Some(456),
                size: Some(789.0),
                color: Some("#123456".parse().unwrap()),
                link: Some(PdfLink::Uri {
                    uri: String::from("https://example.com"),
                }),
            },
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        // Test text with nothing
        let text = PdfObjectText::default();

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($text, {
                type = "text",
                text = "",
                x = 0,
                y = 0,
            })
        })
        .exec()
        .expect("Assertion failed");

        // Test text with everything
        let text = PdfObjectText {
            point: PdfPoint::from_coords_f32(1.0, 2.0),
            text: String::from("hello world"),
            depth: Some(123),
            font: Some(456),
            size: Some(789.0),
            color: Some("#123456".parse().unwrap()),
            link: Some(PdfLink::Uri {
                uri: String::from("https://example.com"),
            }),
        };

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($text, {
                type = "text",
                text = "hello world",
                x = 1,
                y = 2,
                depth = 123,
                font = 456,
                size = 789,
                color = "123456",
                link = {
                    type = "uri",
                    uri = "https://example.com",
                },
            })
        })
        .exec()
        .expect("Assertion failed");
    }
}
