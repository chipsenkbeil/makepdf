use crate::pdf::{
    PdfAlign, PdfBounds, PdfColor, PdfContext, PdfHorizontalAlign, PdfLink, PdfLinkAnnotation,
    PdfLuaExt, PdfLuaTableExt, PdfObjectType, PdfPaintMode, PdfVerticalAlign, PdfWindingOrder,
};
use mlua::prelude::*;
use printpdf::Rect;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfObjectRect {
    pub bounds: PdfBounds,
    pub depth: Option<i64>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub mode: Option<PdfPaintMode>,
    pub order: Option<PdfWindingOrder>,
    pub link: Option<PdfLink>,
}

impl PdfObjectRect {
    /// Aligns the rect to a set of bounds.
    pub fn align_to(&mut self, bounds: PdfBounds, align: (PdfVerticalAlign, PdfHorizontalAlign)) {
        self.bounds = self.bounds.align_to(bounds, align);
    }

    /// Returns a collection of link annotations.
    pub fn link_annotations(&self, _ctx: PdfContext) -> Vec<PdfLinkAnnotation> {
        match self.link.clone() {
            Some(link) => vec![PdfLinkAnnotation {
                bounds: self.bounds,
                depth: self.depth.unwrap_or_default(),
                link,
            }],
            None => Vec::new(),
        }
    }

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext) {
        // Get optional values, setting defaults when not specified
        let fill_color = self.fill_color.unwrap_or(ctx.config.page.fill_color);
        let outline_color = self.outline_color.unwrap_or(ctx.config.page.outline_color);

        // Set the color and positioning of our rect
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
        ctx.layer.add_rect(Rect {
            ll: self.bounds.ll.into(),
            ur: self.bounds.ur.into(),
            mode: self.mode.unwrap_or_default().into(),
            winding: self.order.unwrap_or_default().into(),
        });
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectRect {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        self.bounds.add_to_table(&table)?;
        table.raw_set("type", PdfObjectType::Rect)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("mode", self.mode)?;
        table.raw_set("order", self.order)?;
        table.raw_set("link", self.link)?;

        metatable.raw_set(
            "align_to",
            lua.create_function(
                move |_, (mut this, bounds, align): (Self, PdfBounds, PdfAlign)| {
                    this.align_to(bounds, align.to_v_h());
                    Ok(this)
                },
            )?,
        )?;

        metatable.raw_set(
            "bounds",
            lua.create_function(move |_, this: Self| Ok(this.bounds))?,
        )?;

        metatable.raw_set(
            "with_bounds",
            lua.create_function(
                move |_, (mut this, bounds): (Self, Option<PdfBounds>)| match bounds {
                    Some(bounds) => {
                        this.bounds = bounds;
                        Ok(this)
                    }
                    None => Ok(this),
                },
            )?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectRect {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                // Support missing bounds converting into default bounds
                //
                // TODO: This will result in invalid bounds becoming default bounds.
                //       We want to correct this to support missing bounds only.
                let bounds =
                    PdfBounds::from_lua(LuaValue::Table(table.clone()), lua).unwrap_or_default();

                Ok(Self {
                    bounds,
                    depth: table.raw_get_ext("depth")?,
                    fill_color: table.raw_get_ext("fill_color")?,
                    outline_color: table.raw_get_ext("outline_color")?,
                    mode: table.raw_get_ext("mode")?,
                    order: table.raw_get_ext("order")?,
                    link: table.raw_get_ext("link")?,
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.rect",
                message: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::Pdf;
    use mlua::chunk;

    #[test]
    fn should_be_able_to_align_rect_to_some_bounds_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        // Test the bounds, which should correctly cover full shape
        lua.load(chunk! {
            // Create an initial rect at some position
            local rect = pdf.object.rect({
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
            })

            // Assert the rect is where we expect prior to alignment
            pdf.utils.assert_deep_equal(rect:bounds(), {
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
            })

            // Do the alignment with some bounds that are elsewhere
            rect = rect:align_to({
                ll = { x = 5,  y = 5 },
                ur = { x = 10, y = 10 },
            }, { v = "bottom", h = "left" })

            // Assert the rect has moved into place
            pdf.utils.assert_deep_equal(rect:bounds(), {
                ll = { x = 5, y = 5 },
                ur = { x = 7, y = 7 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_rect_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        lua.load(chunk! {
            // No bounds specified
            local rect = pdf.object.rect({})
            pdf.utils.assert_deep_equal(rect:bounds(), {
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })

            // Explicit bounds
            local rect = pdf.object.rect({
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
            })
            pdf.utils.assert_deep_equal(rect:bounds(), {
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_create_a_duplicate_rect_with_bounds_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        lua.load(chunk! {
            // No bounds specified
            local rect = pdf.object.rect({})
            pdf.utils.assert_deep_equal(rect:with_bounds(), {
                type = "rect",
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })

            // Explicit bounds
            local rect = pdf.object.rect({})
            pdf.utils.assert_deep_equal(rect:with_bounds({
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
            }), {
                type = "rect",
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        // Can convert from empty table into a rect
        assert_eq!(
            Lua::new().load(chunk!({})).eval::<PdfObjectRect>().unwrap(),
            PdfObjectRect::default(),
        );

        // Can convert from a table with flattened bounds into a rect
        assert_eq!(
            Lua::new()
                .load(chunk!({1, 2, 3, 4}))
                .eval::<PdfObjectRect>()
                .unwrap(),
            PdfObjectRect {
                bounds: PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0),
                ..Default::default()
            },
        );

        // Can convert from a table with flattened points into a rect
        assert_eq!(
            Lua::new()
                .load(chunk!({{1, 2}, {3, 4}}))
                .eval::<PdfObjectRect>()
                .unwrap(),
            PdfObjectRect {
                bounds: PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0),
                ..Default::default()
            },
        );

        // Can convert from a table with simplified points into a rect
        assert_eq!(
            Lua::new()
                .load(chunk!({ ll = { 1, 2 }, ur = { 3, 4 } }))
                .eval::<PdfObjectRect>()
                .unwrap(),
            PdfObjectRect {
                bounds: PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0),
                ..Default::default()
            },
        );

        // Can convert from a table with everything into a rect
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    ll = { x = 1, y = 2 },
                    ur = { x = 3, y = 4 },
                    depth = 123,
                    fill_color = "123456",
                    outline_color = "789ABC",
                    mode = "stroke",
                    order = "non_zero",
                    link = {
                        type = "uri",
                        uri = "https://example.com",
                    },
                }))
                .eval::<PdfObjectRect>()
                .unwrap(),
            PdfObjectRect {
                bounds: PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0),
                depth: Some(123),
                fill_color: Some("#123456".parse().unwrap()),
                outline_color: Some("#789ABC".parse().unwrap()),
                mode: Some(PdfPaintMode::stroke()),
                order: Some(PdfWindingOrder::non_zero()),
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

        // Test rect with nothing
        let rect = PdfObjectRect::default();

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($rect, {
                type = "rect",
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })
        })
        .exec()
        .expect("Assertion failed");

        // Test rect with everything
        let rect = PdfObjectRect {
            bounds: PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0),
            depth: Some(123),
            fill_color: Some("#123456".parse().unwrap()),
            outline_color: Some("#789ABC".parse().unwrap()),
            mode: Some(PdfPaintMode::stroke()),
            order: Some(PdfWindingOrder::non_zero()),
            link: Some(PdfLink::Uri {
                uri: String::from("https://example.com"),
            }),
        };

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($rect, {
                type = "rect",
                ll = { x = 1, y = 2 },
                ur = { x = 3, y = 4 },
                depth = 123,
                fill_color = "123456",
                outline_color = "789ABC",
                mode = "stroke",
                order = "non_zero",
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
