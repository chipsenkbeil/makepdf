mod daily;
mod monthly;
mod weekly;

use crate::pdf::PdfLuaTableExt;
use chrono::offset::Local;
use chrono::Datelike;
use mlua::prelude::*;

pub use daily::PdfConfigDailyPlanner;
pub use monthly::PdfConfigMonthlyPlanner;
pub use weekly::PdfConfigWeeklyPlanner;

/// Planner-specific configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct PdfConfigPlanner {
    /// Year associated with planner
    pub year: i32,

    /// Configuration tied to monthly planner pages
    pub monthly: PdfConfigMonthlyPlanner,

    /// Configuration tied to weekly planner pages
    pub weekly: PdfConfigWeeklyPlanner,

    /// Configuration tied to daily planner pages
    pub daily: PdfConfigDailyPlanner,
}

impl Default for PdfConfigPlanner {
    fn default() -> Self {
        Self {
            year: Local::now().year(),
            monthly: Default::default(),
            weekly: Default::default(),
            daily: Default::default(),
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfConfigPlanner {
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

impl<'lua> FromLua<'lua> for PdfConfigPlanner {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                year: table.raw_get_ext("year")?,
                monthly: table.raw_get_ext("monthly")?,
                weekly: table.raw_get_ext("weekly")?,
                daily: table.raw_get_ext("daily")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config.planner",
                message: None,
            }),
        }
    }
}
