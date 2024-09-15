use mlua::chunk;
use mlua::prelude::*;
use std::fmt;

pub trait PdfLuaExt {
    /// Creates a new [`LuaTable`] with a metatable associated that has `__index`
    /// preset to the table itself.
    ///
    /// Returns the (table, metatable) pair.
    fn create_table_ext(&self) -> LuaResult<(LuaTable, LuaTable)>;

    /// Marks a table as read-only. This both sets the flag for `Luau` and also overwrites the
    /// `__newindex` metatable field to fail when attempting to change a field.
    fn mark_readonly(&self, tbl: LuaTable) -> LuaResult<()>;
}

impl PdfLuaExt for Lua {
    fn create_table_ext(&self) -> LuaResult<(LuaTable, LuaTable)> {
        let table = self.create_table()?;

        let metatable = self.create_table()?;
        metatable.raw_set("__index", metatable.clone())?;

        table.set_metatable(Some(metatable.clone()));
        Ok((table, metatable))
    }

    fn mark_readonly(&self, tbl: LuaTable) -> LuaResult<()> {
        let metatable = match tbl.get_metatable() {
            Some(x) => x,
            None => self.create_table()?,
        };

        metatable.raw_set(
            "__newindex",
            self.create_function(|lua, ()| {
                lua.load(chunk!(error("attempt to update a read-only table", 2)))
                    .exec()
            })?,
        )?;

        tbl.set_readonly(true);

        Ok(())
    }
}

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
