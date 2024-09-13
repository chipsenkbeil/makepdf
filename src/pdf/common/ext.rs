use mlua::prelude::*;
use std::fmt;

pub trait PdfLuaTableExt {
    /// Like [`LuaTable::raw_get`], but provides a more detailed error message.
    fn raw_get_ext<'lua, K, V>(&'lua self, key: K) -> LuaResult<V>
    where
        K: IntoLua<'lua> + Copy + fmt::Display,
        V: FromLua<'lua>;
}

impl PdfLuaTableExt for LuaTable<'_> {
    fn raw_get_ext<'lua, K, V>(&'lua self, key: K) -> LuaResult<V>
    where
        K: IntoLua<'lua> + Copy + fmt::Display,
        V: FromLua<'lua>,
    {
        match self.raw_get(key) {
            Err(LuaError::FromLuaConversionError { from, to, message }) => {
                Err(LuaError::FromLuaConversionError {
                    from,
                    to,
                    message: Some(format!(
                        "key '{}'{}",
                        key,
                        message
                            .map(|m| format!(": {m}"))
                            .unwrap_or_else(String::new)
                    )),
                })
            }
            x => x,
        }
    }
}
