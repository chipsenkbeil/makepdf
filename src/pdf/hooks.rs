mod daily;
mod monthly;
mod weekly;

pub use daily::OnDailyPageFn;
pub use monthly::OnMonthlyPageFn;
pub use weekly::OnWeeklyPageFn;

use crate::pdf::PdfLuaTableExt;
use mlua::prelude::*;

/// Collection of hooks that contain callbacks to invoke in different situations.
#[derive(Clone, Debug, Default)]
pub struct PdfHooks {
    /// Invoked when a monthly page is created.
    pub on_monthly_page: Vec<OnMonthlyPageFn>,
    /// Invoked when a weekly page is created.
    pub on_weekly_page: Vec<OnWeeklyPageFn>,
    /// Invoked when a daily page is created.
    pub on_daily_page: Vec<OnDailyPageFn>,
}

impl<'lua> IntoLua<'lua> for PdfHooks {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        // NOTE: We don't place any hooks into the Lua engine.
        Ok(LuaValue::Table(lua.create_table()?))
    }
}

impl<'lua> FromLua<'lua> for PdfHooks {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let get_hook_fns = |table: &LuaTable<'_>, key: &str| {
            table
                .raw_get_ext::<_, Vec<LuaOwnedFunction>>(key)
                .unwrap_or_else(|_| {
                    table
                        .raw_get_ext::<_, LuaOwnedFunction>(key)
                        .ok()
                        .into_iter()
                        .collect()
                })
        };

        match value {
            // For hooks, we accept either a single function or a list of functions, and therefore
            // attempt to cast to either a OwnedTable or a Vec<OwnedTable>.
            LuaValue::Table(table) => Ok(Self {
                on_monthly_page: get_hook_fns(&table, "on_monthly_page"),
                on_weekly_page: get_hook_fns(&table, "on_weekly_page"),
                on_daily_page: get_hook_fns(&table, "on_daily_page"),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.hooks",
                message: None,
            }),
        }
    }
}
