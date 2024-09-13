use crate::pdf::{PdfBounds, PdfColor, PdfLuaTableExt, PdfObjectContext};
use mlua::prelude::*;
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::Polygon;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectShape {
    pub bounds: PdfBounds,
    pub color: PdfColor,
}

impl PdfObjectShape {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        ctx.layer.set_fill_color(self.color.into());
        ctx.layer.set_outline_color(self.color.into());
        ctx.layer.add_polygon(Polygon {
            rings: todo!(),
            mode: PaintMode::default(),
            winding_order: WindingOrder::default(),
        });
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectShape {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("bounds", self.bounds)?;
        table.raw_set("color", self.color)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectShape {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                bounds: table.raw_get_ext("bounds")?,
                color: table.raw_get_ext("color")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.rect",
                message: None,
            }),
        }
    }
}
