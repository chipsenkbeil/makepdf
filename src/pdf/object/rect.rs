use crate::pdf::{PdfBounds, PdfColor, PdfContext, PdfLuaTableExt};
use mlua::prelude::*;
use printpdf::{
    path::{PaintMode, WindingOrder},
    Rect,
};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectRect {
    pub bounds: PdfBounds,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
}

impl PdfObjectRect {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfContext<'_>) {
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
        let table = lua.create_table()?;

        self.bounds.add_to_table(&table)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;

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
                    fill_color: table.raw_get_ext("fill_color")?,
                    outline_color: table.raw_get_ext("outline_color")?,
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
