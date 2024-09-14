mod common;
mod config;
mod context;
mod hooks;
mod object;
mod page;
mod utils;

pub use common::*;
pub use config::*;
pub use context::*;
pub use hooks::*;
pub use object::*;
pub use page::*;
pub use utils::*;

use mlua::prelude::*;

/// Primary entrypoint for performing PDF operations.
#[derive(Clone, Debug, Default)]
pub struct Pdf {
    /// Configuration associated with the PDF.
    pub config: PdfConfig,

    /// Hooks associated with the PDF.
    pub hooks: PdfHooks,
}

impl Pdf {
    /// Creates a new PDF instance using `config` and a default, empty set of hooks.
    pub fn new(config: PdfConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Creates a new Lua table that contains methods to create objects and other manipulation.
    fn create_object_table(lua: &Lua) -> LuaResult<LuaTable> {
        let table = lua.create_table()?;

        table.raw_set(
            "line",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectLine::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Line)?
                    .into_lua(lua)
            })?,
        )?;

        table.raw_set(
            "rect",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectRect::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Rect)?
                    .into_lua(lua)
            })?,
        )?;

        table.raw_set(
            "shape",
            lua.create_function(|lua, tbl: LuaTable| {
                PdfObjectShape::from_lua(LuaValue::Table(tbl), lua)
                    .map(PdfObject::Shape)?
                    .into_lua(lua)
            })?,
        )?;

        table.raw_set(
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
        table.raw_set("hooks", self.hooks)?;
        table.raw_set("object", Pdf::create_object_table(lua)?)?;
        table.raw_set("utils", PdfUtils)?;

        // TODO: Some notes on what to do next
        //
        // 4. Add more defaults to page config including outline_color, fill_color, line style,
        //    etc. to cover all of the fields for our objects such that everything is optional
        //    except the position and - in the case of the text object - a string of text
        // 6. Think through how to make some object that can contain others (container?) and
        //    have PdfContext contain bounds representing the container, and a function that can
        //    translate the bounds or points of an object relative to the container
        // 7. Think through how linking works. do we just have link annotations maintained as a
        //    list within the pdf? how do we make it work linking an object (with its dimensions)
        //    to another is easy?
        // 8. How do we handle z ordering to make it easier to ensure drawing order is correct?
        //    At the moment, things would be drawn by when they're added to a page.
        // 9. Outline out what we need for the page object provided to the hooks

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Pdf {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                config: PdfConfig::from_lua(LuaValue::Table(table.clone()), lua)?,
                hooks: table.raw_get_ext("hooks")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf",
                message: None,
            }),
        }
    }
}
