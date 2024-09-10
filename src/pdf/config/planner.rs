mod daily;
mod monthly;
mod weekly;

use chrono::offset::Local;
use chrono::Datelike;
use mlua::prelude::*;

pub use daily::DailyPlannerPdfConfig;
pub use monthly::MonthlyPlannerPdfConfig;
pub use weekly::WeeklyPlannerPdfConfig;

/// Planner-specific configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct PlannerPdfConfig {
    /// Year associated with planner
    pub year: i32,

    /// Configuration tied to monthly planner pages
    pub monthly: MonthlyPlannerPdfConfig,

    /// Configuration tied to weekly planner pages
    pub weekly: WeeklyPlannerPdfConfig,

    /// Configuration tied to daily planner pages
    pub daily: DailyPlannerPdfConfig,
}

impl Default for PlannerPdfConfig {
    fn default() -> Self {
        Self {
            year: Local::now().year(),
            monthly: Default::default(),
            weekly: Default::default(),
            daily: Default::default(),
        }
    }
}

impl<'lua> IntoLua<'lua> for PlannerPdfConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("year", self.year)?;
        table.raw_set("monthly", self.monthly)?;
        table.raw_set("weekly", self.weekly)?;
        table.raw_set("daily", self.daily)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PlannerPdfConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                year: raw_get!(table, "year")?,
                monthly: raw_get!(table, "monthly")?,
                weekly: raw_get!(table, "weekly")?,
                daily: raw_get!(table, "daily")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config.planner",
                message: None,
            }),
        }
    }
}
