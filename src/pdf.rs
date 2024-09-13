mod common;
mod config;
mod object;

pub use common::*;
pub use config::*;
pub use object::*;

use mlua::prelude::*;

/// Primary entrypoint for performing PDF operations.
#[derive(Clone, Debug)]
pub struct Pdf {
    pub config: PdfConfig,
}

impl Pdf {
    pub fn new(config: PdfConfig) -> Self {
        Self { config }
    }
}

impl<'lua> IntoLua<'lua> for Pdf {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.raw_set("config", self.config)?;

        let object = lua.create_table()?;
        macro_rules! define_object_constructor {
            ($type:expr, $kind:ident) => {{
                object.raw_set(
                    $type,
                    lua.create_function(|lua, tbl: LuaTable| {
                        tbl.raw_set("type", $type)?;
                        $kind::from_lua(LuaValue::Table(tbl), lua)
                    })?,
                )?;
            }};
        }
        define_object_constructor!("line", PdfObjectLine);
        define_object_constructor!("rect", PdfObjectRect);
        define_object_constructor!("shape", PdfObjectShape);
        define_object_constructor!("text", PdfObjectText);
        table.raw_set("object", object)?;

        // TODO: Some notes on what to do next
        //
        // 1. Flatten config into pdf.page.width (etc) and pdf.planner.monthly.enabled (etc)
        // 2. Add a `hooks` table with `on_monthly_page` and companions that is by default an
        //    empty list, but can be either Some(OwnedFunction), None, or Vec<OwnedFunction>
        // 3. Move PdfObjectContext to PdfContext within context.rs
        // 4. Add more defaults to page config including outline_color, fill_color, line style,
        //    etc. to cover all of the fields for our objects such that everything is optional
        //    except the position and - in the case of the text object - a string of text
        // 5. Update PdfContext to have access to page config so we have access to page dimensions
        //    and all of the defaults needed to draw the object
        // 6. Think through how to make some object that can contain others (container?) and
        //    have PdfContext contain bounds representing the container, and a function that can
        //    translate the bounds or points of an object relative to the container
        // 7. Think through how linking works. do we just have link annotations maintained as a
        //    list within the pdf? how do we make it work linking an object (with its dimensions)
        //    to another is easy?
        // 8. How do we handle z ordering to make it easier to ensure drawing order is correct?
        //    At the moment, things would be drawn by when they're added to a page.
        // 9. Outline out what we need for the page object provided to the hooks
        // 10. Write a `PdfDate` wrapper around a `NaiveDate` to convert to a string and back
        let page = lua.create_table()?;
        //todo!("create pdf.hooks.on_monthly_page = ...");
        //todo!("create pdf.hooks.on_weekly_page = ...");
        //todo!("create pdf.hooks.on_daily_page = ...");
        table.raw_set("page", page)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Pdf {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                config: table.raw_get_ext("config")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf",
                message: None,
            }),
        }
    }
}
