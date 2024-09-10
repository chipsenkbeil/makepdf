mod page;
mod planner;

use mlua::prelude::*;

pub use page::PagePdfConfig;
pub use planner::{
    DailyPlannerPdfConfig, MonthlyPlannerPdfConfig, PlannerPdfConfig, WeeklyPlannerPdfConfig,
};

/// Configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct PdfConfig {
    /// Configuration tied to a PDF page
    pub page: PagePdfConfig,
    /// Configuration tied to a PDF planner
    pub planner: PlannerPdfConfig,
    /// Path or name of script (e.g. `mpdf:panda`)
    pub script: String,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            page: Default::default(),
            planner: Default::default(),
            script: String::from("mpdf:panda"),
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("planner", self.planner)?;
        table.raw_set("page", self.page)?;
        table.raw_set("script", self.script)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                planner: table.raw_get("planner")?,
                page: table.raw_get("page")?,
                script: table.raw_get("script")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config",
                message: None,
            }),
        }
    }
}