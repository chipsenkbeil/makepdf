use crate::pdf::{Margin, PdfBounds, PdfObjectContext};
use mlua::prelude::*;
use palette::Srgb;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectRect {
    pub color: Srgb,
    pub margin: Option<Margin>,
    pub bounds: PdfBounds,
    pub thickness: f32,
    pub style: bool,
}

impl PdfObjectRect {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        todo!("implement");
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectRect {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("color", format!("{:X}", Srgb::<u8>::from(self.color)))?;
        table.raw_set("margin", self.margin)?;
        table.raw_set("bounds", self.bounds)?;
        table.raw_set("thickness", self.thickness)?;
        table.raw_set("style", self.style)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectRect {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                color: raw_get_wrap!(table.raw_get::<_, String>("color"), "color")?
                    .parse::<Srgb<u8>>()
                    .map_err(LuaError::external)?
                    .into(),
                margin: raw_get!(table, "margin")?,
                bounds: raw_get!(table, "bounds")?,
                thickness: raw_get!(table, "thickness")?,
                style: raw_get!(table, "style")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.line",
                message: None,
            }),
        }
    }
}
