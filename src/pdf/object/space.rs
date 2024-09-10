use mlua::prelude::*;
use printpdf::Mm;

pub type Margin = SpacePdfObject;
pub type Padding = SpacePdfObject;

/// Spacing for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SpacePdfObject {
    pub top: Mm,
    pub left: Mm,
    pub right: Mm,
    pub bottom: Mm,
}

impl<'lua> IntoLua<'lua> for SpacePdfObject {
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

impl<'lua> FromLua<'lua> for SpacePdfObject {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                top: Mm(table.raw_get("top")?),
                left: Mm(table.raw_get("left")?),
                right: Mm(table.raw_get("right")?),
                bottom: Mm(table.raw_get("bottom")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.space",
                message: None,
            }),
        }
    }
}
