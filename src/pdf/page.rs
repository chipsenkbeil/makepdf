use crate::pdf::{PdfContext, PdfObject};
use mlua::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// Lua-only struct providing an interface for specialized operations within Lua to manipulate a
/// PDF page.
#[derive(Clone, Debug, Default)]
pub struct PdfPage {
    /// Collection of objects to add to the PDF.
    ///
    /// Keys represent the z order with lower-value keys being added first.
    objects: Arc<RwLock<BTreeMap<i64, Vec<PdfObject>>>>,
}

impl PdfPage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clones the objects maintained by this page.
    #[inline]
    pub fn clone_objects(&self) -> Arc<RwLock<BTreeMap<i64, Vec<PdfObject>>>> {
        Arc::clone(&self.objects)
    }

    /// Consumes this page, returning the objects to be drawn.
    pub fn into_objects(self) -> Arc<RwLock<BTreeMap<i64, Vec<PdfObject>>>> {
        self.objects
    }

    /// Draws the page by adding objects in order based on their z-index.
    pub fn draw(&self, ctx: PdfContext<'_>) {
        // Iterate over the objects sorted by z-order
        for (_, objs) in self.objects.read().unwrap().iter() {
            for obj in objs {
                obj.draw(ctx);
            }
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfPage {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        let objects = self.clone_objects();
        table.raw_set(
            "push",
            lua.create_function(move |lua, tbl: LuaTable| {
                // Construct a PDF object based on the other table fields
                let obj = PdfObject::from_lua(LuaValue::Table(tbl), lua)?;

                // Acquire a lock on our object map and queue the object at z
                objects
                    .write()
                    .unwrap()
                    .entry(obj.depth())
                    .or_default()
                    .push(obj);

                Ok(())
            })?,
        )?;

        Ok(LuaValue::Table(table))
    }
}
