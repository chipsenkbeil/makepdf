use crate::planner::PlannerConfig;
use mlua::prelude::*;

/// Global table provided to script by planner.
#[derive(Clone, Debug)]
pub struct PlannerGlobal {
    pub config: PlannerConfig,
}

impl<'lua> IntoLua<'lua> for PlannerGlobal {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        let utils = lua.create_table()?;

        table.raw_set("config", self.config)?;
        table.raw_set("utils", utils)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PlannerGlobal {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                config: table.raw_get("config")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "planner global",
                message: None,
            }),
        }
    }
}
