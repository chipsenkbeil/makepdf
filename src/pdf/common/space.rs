use crate::PdfLuaTableExt;
use mlua::prelude::*;
use printpdf::Mm;

pub type Margin = PdfSpace;
pub type Padding = PdfSpace;

/// Spacing for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfSpace {
    pub top: Mm,
    pub left: Mm,
    pub right: Mm,
    pub bottom: Mm,
}

impl<'lua> IntoLua<'lua> for PdfSpace {
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

impl<'lua> FromLua<'lua> for PdfSpace {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                top: Mm(table.raw_get_ext("top")?),
                left: Mm(table.raw_get_ext("left")?),
                right: Mm(table.raw_get_ext("right")?),
                bottom: Mm(table.raw_get_ext("bottom")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.space",
                message: None,
            }),
        }
    }
}
