macro_rules! raw_get_wrap {
    ($e:expr, $key:expr) => {{
        $e.map_err(|x| match x {
            ::mlua::Error::FromLuaConversionError { from, to, message } => {
                ::mlua::Error::FromLuaConversionError {
                    from,
                    to,
                    message: Some(format!(
                        "key '{}'{}",
                        $key,
                        message
                            .map(|m| format!(": {m}"))
                            .unwrap_or_else(String::new)
                    )),
                }
            }
            x => x,
        })
    }};
}

macro_rules! raw_get {
    ($table:expr, $key:expr) => {{
        raw_get_wrap!($table.raw_get($key), $key)
    }};
}

mod common;
mod config;
mod object;

pub use common::*;
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

        let object = lua.create_table()?;
        macro_rules! define_object_constructor {
            ($type:expr, $kind:ident) => {{
                object.raw_set(
                    $type,
                    lua.create_function(|lua, tbl: LuaTable| {
                        tbl.raw_set("type", $type)?;
                        $kind::from_lua(LuaValue::Table(tbl), lua)
                    })?,
                )?;
            }};
        }
        define_object_constructor!("line", PdfObjectLine);
        define_object_constructor!("rect", PdfObjectRect);
        define_object_constructor!("shape", PdfObjectShape);
        define_object_constructor!("text", PdfObjectText);
        table.raw_set("object", object)?;

        let page = lua.create_table()?;
        //todo!("create pdf.page.on_monthly(...)");
        //todo!("create pdf.page.on_weekly(...)");
        //todo!("create pdf.page.on_daily(...)");
        table.raw_set("page", page)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Pdf {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                config: raw_get!(table, "config")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf",
                message: None,
            }),
        }
    }
}
