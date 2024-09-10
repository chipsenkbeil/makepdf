use mlua::prelude::*;
use printpdf::Mm;

/// Spacing for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfObjectBounds {
    /// Lower-left x coordinate
    pub llx: Mm,
    /// Lower-left y coordinate
    pub lly: Mm,
    /// Upper-right x coordinate
    pub urx: Mm,
    /// Upper-right y coordinate
    pub ury: Mm,
}

impl<'lua> IntoLua<'lua> for PdfObjectBounds {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("llx", self.llx.0)?;
        table.raw_set("lly", self.lly.0)?;
        table.raw_set("urx", self.urx.0)?;
        table.raw_set("ury", self.ury.0)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectBounds {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                llx: Mm(raw_get!(table, "llx")?),
                lly: Mm(raw_get!(table, "lly")?),
                urx: Mm(raw_get!(table, "urx")?),
                ury: Mm(raw_get!(table, "ury")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.bounds",
                message: None,
            }),
        }
    }
}
