mod common;
mod config;
mod context;
mod hooks;
mod object;
mod utils;

pub use common::*;
pub use config::*;
pub use context::*;
pub use hooks::*;
pub use object::*;
pub use utils::*;

use mlua::prelude::*;

/// Primary entrypoint for performing PDF operations.
#[derive(Clone, Debug, Default)]
pub struct Pdf {
    /// Configuration associated with the PDF.
    pub config: PdfConfig,
}

impl Pdf {
    /// Creates a new PDF instance using `config` and a default, empty set of hooks.
    pub fn new(config: PdfConfig) -> Self {
        Self { config }
    }

    /// Creates a new Lua table that contains methods to create objects and other manipulation.
    fn create_object_table(lua: &Lua) -> LuaResult<LuaTable> {
        let (table, metatable) = lua.create_table_ext()?;

        metatable.raw_set(
            "group",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectGroup::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Group)?
                    .into_lua(lua)
            })?,
        )?;

        metatable.raw_set(
            "line",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectLine::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Line)?
                    .into_lua(lua)
            })?,
        )?;

        metatable.raw_set(
            "rect",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectRect::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Rect)?
                    .into_lua(lua)
            })?,
        )?;

        metatable.raw_set(
            "shape",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectShape::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Shape)?
                    .into_lua(lua)
            })?,
        )?;

        metatable.raw_set(
            "text",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectText::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Text)?
                    .into_lua(lua)
            })?,
        )?;

        Ok(table)
    }
}

impl<'lua> IntoLua<'lua> for Pdf {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        // Create our base table by turning our config into a table
        let table = match PdfConfig::into_lua(self.config, lua)? {
            LuaValue::Table(x) => x,
            _ => unreachable!("pdf.config should always be converted into a Lua table"),
        };

        // Add in the API instances to the base table
        table.raw_set("hooks", PdfHooks)?;
        table.raw_set("object", Pdf::create_object_table(lua)?)?;
        table.raw_set("utils", PdfUtils)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Pdf {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                config: PdfConfig::from_lua(LuaValue::Table(table.clone()), lua)?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf",
                message: None,
            }),
        }
    }
}
