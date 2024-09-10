use mlua::prelude::*;

/// Daily planner-specific configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct DailyPlannerPdfConfig {
    pub enabled: bool,
}

impl Default for DailyPlannerPdfConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl<'lua> IntoLua<'lua> for DailyPlannerPdfConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("enabled", self.enabled)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for DailyPlannerPdfConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                enabled: raw_get!(table, "enabled")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config.planner.daily",
                message: None,
            }),
        }
    }
}
