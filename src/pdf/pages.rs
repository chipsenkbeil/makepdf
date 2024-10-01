use crate::pdf::PdfLuaExt;
use crate::runtime::{RuntimePage, RuntimePageId, RuntimePages};
use mlua::prelude::*;

/// Collection of pages functions.
#[derive(Copy, Clone, Debug, Default)]
pub struct PdfPages;

impl<'lua> IntoLua<'lua> for PdfPages {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        // Function to create a new page with the specified title.
        metatable.raw_set(
            "create",
            lua.create_function(|lua, title: String| {
                if let Some(mut pages) = lua.app_data_mut::<RuntimePages>() {
                    Ok(pages.insert_page(RuntimePage::new(title)))
                } else {
                    Err(LuaError::runtime("Runtime pages are missing"))
                }
            })?,
        )?;

        // Function to retrieve a page by its id.
        metatable.raw_set(
            "get",
            lua.create_function(|lua, id: RuntimePageId| {
                if let Some(pages) = lua.app_data_ref::<RuntimePages>() {
                    Ok(pages.get_page(id))
                } else {
                    Err(LuaError::runtime("Runtime pages are missing"))
                }
            })?,
        )?;

        // Function to return all page ids as a list.
        metatable.raw_set(
            "ids",
            lua.create_function(|lua, ()| {
                if let Some(pages) = lua.app_data_ref::<RuntimePages>() {
                    Ok(pages.ids().collect::<Vec<_>>())
                } else {
                    Err(LuaError::runtime("Runtime pages are missing"))
                }
            })?,
        )?;

        Ok(LuaValue::Table(table))
    }
}
