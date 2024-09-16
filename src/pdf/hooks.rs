use crate::pdf::PdfLuaExt;
use crate::runtime::RuntimeHooks;
use mlua::prelude::*;

/// Lua-only struct providing an interface for specialized hook registration.
#[derive(Copy, Clone, Debug, Default)]
pub struct PdfHooks;

impl<'lua> IntoLua<'lua> for PdfHooks {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        metatable.raw_set(
            "on_daily_page",
            lua.create_function(|lua, f: LuaOwnedFunction| {
                match lua.app_data_mut::<RuntimeHooks>() {
                    Some(mut hooks) => {
                        hooks.register_on_daily_page(f);
                        Ok(())
                    }
                    None => Err(LuaError::runtime("failed to register daily page hook")),
                }
            })?,
        )?;

        metatable.raw_set(
            "on_monthly_page",
            lua.create_function(|lua, f: LuaOwnedFunction| {
                match lua.app_data_mut::<RuntimeHooks>() {
                    Some(mut hooks) => {
                        hooks.register_on_monthly_page(f);
                        Ok(())
                    }
                    None => Err(LuaError::runtime("failed to register monthly page hook")),
                }
            })?,
        )?;

        metatable.raw_set(
            "on_weekly_page",
            lua.create_function(|lua, f: LuaOwnedFunction| {
                match lua.app_data_mut::<RuntimeHooks>() {
                    Some(mut hooks) => {
                        hooks.register_on_weekly_page(f);
                        Ok(())
                    }
                    None => Err(LuaError::runtime("failed to register weekly page hook")),
                }
            })?,
        )?;

        Ok(LuaValue::Table(table))
    }
}
