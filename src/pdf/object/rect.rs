use crate::pdf::{PdfBounds, PdfColor, PdfObjectContext};
use mlua::prelude::*;
use printpdf::{
    path::{PaintMode, WindingOrder},
    Rect,
};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectRect {
    pub bounds: PdfBounds,
    pub color: PdfColor,
}

impl PdfObjectRect {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        ctx.layer.set_fill_color(self.color.into());
        ctx.layer.set_outline_color(self.color.into());
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

        table.raw_set("bounds", self.bounds)?;
        table.raw_set("color", self.color)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectRect {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                bounds: raw_get!(table, "bounds")?,
                color: raw_get!(table, "color")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.rect",
                message: None,
            }),
        }
    }
}
