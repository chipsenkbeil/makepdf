use crate::pdf::{
    PdfAlign, PdfBounds, PdfColor, PdfContext, PdfHorizontalAlign, PdfLink, PdfLinkAnnotation,
    PdfLuaExt, PdfLuaTableExt, PdfVerticalAlign,
};
use mlua::prelude::*;
use printpdf::{
    path::{PaintMode, WindingOrder},
    Rect,
};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug, Default)]
pub struct PdfObjectRect {
    pub bounds: PdfBounds,
    pub depth: Option<i64>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
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
            mode: PaintMode::default(),
            winding: WindingOrder::default(),
        });
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectRect {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        self.bounds.add_to_table(&table)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
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

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectRect {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                let bounds = PdfBounds::from_lua(LuaValue::Table(table.clone()), lua)?;
                Ok(Self {
                    bounds,
                    depth: table.raw_get_ext("depth")?,
                    fill_color: table.raw_get_ext("fill_color")?,
                    outline_color: table.raw_get_ext("outline_color")?,
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
    fn should_be_able_to_calculate_bounds_of_rect_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        lua.load(chunk! {
            // No bounds specified
            /*local rect = pdf.object.rect({})
            pdf.utils.assert_deep_equal(rect:bounds(), {
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })*/

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
}
