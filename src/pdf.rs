mod config;
mod object;

pub use config::*;
pub use object::*;

use mlua::prelude::*;

/// Primary entrypoint for performing PDF operations.
#[derive(Clone, Debug)]
pub struct Pdf {
    pub config: PdfConfig,
}

impl Pdf {
    pub fn new(config: PdfConfig) -> Self {
        Self { config }
    }
}

impl<'lua> IntoLua<'lua> for Pdf {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("config", self.config)?;
        todo!("create pdf.object.rect()");
        todo!("create pdf.object.text()");
        todo!("create pdf.object.shape()");
        todo!("create pdf.object.line()");
        todo!("create pdf.page.on_monthly(...)");
        todo!("create pdf.page.on_weekly(...)");
        todo!("create pdf.page.on_daily(...)");

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Pdf {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                config: table.raw_get("config")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf",
                message: None,
            }),
        }
    }
}
