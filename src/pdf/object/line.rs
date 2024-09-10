use super::{Margin, PdfObjectBounds, PdfObjectContext};
use mlua::prelude::*;
use palette::Srgb;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectLine {
    pub bounds: PdfObjectBounds,
    pub color: Option<Srgb>,
    pub margin: Option<Margin>,
    pub thickness: Option<f32>,
    pub style: Option<bool>,
}

impl PdfObjectLine {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        todo!("implement");
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectLine {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set(
            "color",
            self.color.map(|c| format!("{:X}", Srgb::<u8>::from(c))),
        )?;
        table.raw_set("margin", self.margin)?;
        table.raw_set("bounds", self.bounds)?;
        table.raw_set("thickness", self.thickness)?;
        table.raw_set("style", self.style)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectLine {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                color: raw_get_wrap!(table.raw_get::<_, Option<String>>("color"), "color")?
                    .map(|c| c.parse::<Srgb<u8>>())
                    .transpose()
                    .map_err(LuaError::external)?
                    .map(Into::into),
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
