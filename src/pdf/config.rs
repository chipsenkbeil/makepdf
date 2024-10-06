mod page;

use crate::pdf::PdfLuaTableExt;
use chrono::offset::Local;
use mlua::prelude::*;

pub use page::PdfConfigPage;

/// Configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct PdfConfig {
    /// Configuration tied to a PDF page
    pub page: PdfConfigPage,
    /// Path of script
    pub script: String,
    /// Title of the pdf document
    pub title: String,
}

impl Default for PdfConfig {
    fn default() -> Self {
        let page = PdfConfigPage::default();

        Self {
            page,
            script: String::from("makepdf.lua"),
            title: format!("MakePDF {}", Local::now().naive_local().date()),
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("page", self.page)?;
        table.raw_set("script", self.script)?;
        table.raw_set("title", self.title)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                page: table.raw_get_ext("page")?,
                script: table.raw_get_ext("script").unwrap_or_default(),
                title: table.raw_get_ext("title").unwrap_or_default(),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config",
                message: None,
            }),
        }
    }
}
