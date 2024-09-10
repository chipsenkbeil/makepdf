use super::{BoundsPdfObject, Margin, PdfObjectContext};
use mlua::prelude::*;
use palette::Srgb;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct ShapePdfObject {
    pub color: Srgb,
    pub margin: Option<Margin>,
    pub bounds: BoundsPdfObject,
    pub thickness: f32,
    pub style: bool,
}

impl ShapePdfObject {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        todo!("implement");
    }
}

impl<'lua> IntoLua<'lua> for ShapePdfObject {
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

impl<'lua> FromLua<'lua> for ShapePdfObject {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                color: table
                    .raw_get::<_, String>("color")?
                    .parse::<Srgb<u8>>()
                    .map_err(LuaError::external)?
                    .into(),
                margin: table.raw_get("margin")?,
                bounds: table.raw_get("bounds")?,
                thickness: table.raw_get("thickness")?,
                style: table.raw_get("style")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.line",
                message: None,
            }),
        }
    }
}
