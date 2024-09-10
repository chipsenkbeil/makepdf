use mlua::prelude::*;

/// Monthly planner-specific configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct MonthlyPlannerPdfConfig {
    pub enabled: bool,
}

impl Default for MonthlyPlannerPdfConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl<'lua> IntoLua<'lua> for MonthlyPlannerPdfConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("enabled", self.enabled)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for MonthlyPlannerPdfConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                enabled: table.raw_get("enabled")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config.planner.monthly",
                message: None,
            }),
        }
    }
}
