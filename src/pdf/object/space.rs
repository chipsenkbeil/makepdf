use mlua::prelude::*;
use printpdf::Mm;

pub type Margin = PdfObjectSpace;
pub type Padding = PdfObjectSpace;

/// Spacing for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfObjectSpace {
    pub top: Mm,
    pub left: Mm,
    pub right: Mm,
    pub bottom: Mm,
}

impl<'lua> IntoLua<'lua> for PdfObjectSpace {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("top", self.top.0)?;
        table.raw_set("left", self.left.0)?;
        table.raw_set("right", self.right.0)?;
        table.raw_set("bottom", self.bottom.0)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectSpace {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                top: Mm(raw_get!(table, "top")?),
                left: Mm(raw_get!(table, "left")?),
                right: Mm(raw_get!(table, "right")?),
                bottom: Mm(raw_get!(table, "bottom")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.space",
                message: None,
            }),
        }
    }
}
