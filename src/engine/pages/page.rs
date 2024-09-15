use super::{EnginePageKey, EnginePageKind, EnginePages};
use crate::pdf::{PdfContext, PdfDate, PdfLuaExt, PdfObject};
use mlua::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

/// Lua-only struct providing an interface for specialized operations within Lua to manipulate a
/// PDF page.
#[derive(Clone, Debug)]
pub struct EnginePage {
    /// Unique id associated with the page.
    pub id: u64,

    /// Indicates the kind of page.
    pub kind: EnginePageKind,

    /// Date associated with the page.
    pub date: PdfDate,

    /// Collection of objects to add to the PDF.
    ///
    /// Page Id -> Depth -> Objects
    objects: Arc<RwLock<BTreeMap<i64, Vec<PdfObject>>>>,
}

impl EnginePage {
    /// Creates a new page of `kind` for the specified `date`.
    pub fn new(kind: EnginePageKind, date: PdfDate) -> Self {
        Self {
            id: rand::random(),
            kind,
            date,
            objects: Default::default(),
        }
    }

    /// Constructs a key associated with the page.
    #[inline]
    pub fn key(&self) -> EnginePageKey {
        (self.kind, self.date).into()
    }

    /// Draws the page by adding objects in order based on their depth.
    pub fn draw(&self, ctx: PdfContext<'_>) {
        for (_, objs) in self.objects.read().unwrap().iter() {
            for obj in objs {
                obj.draw(ctx);
            }
        }
    }
}

impl<'lua> IntoLua<'lua> for EnginePage {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let kind = self.kind;
        let objects = Arc::downgrade(&self.objects);

        let (table, metatable) = lua.create_table_ext()?;
        table.raw_set("id", self.id)?;
        table.raw_set("date", self.date)?;

        // Define a field function to get the monthly page for the current page (optionally taking
        // a date). This will retrieve the page from the weak reference, upgrading it in the
        // process, and can potentially return nil if no reference is found.
        let date = self.date;
        metatable.raw_set(
            "monthly",
            lua.create_function(move |lua, maybe_date: Option<PdfDate>| {
                Ok(match lua.app_data_ref::<EnginePages>() {
                    Some(pages) => {
                        pages.get_page_by_date(EnginePageKind::Monthly, maybe_date.unwrap_or(date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the weekly page for the current page (optionally taking a
        // date). This will retrieve the page from the weak reference, upgrading it in the process,
        // and can potentially return nil if no reference is found.
        let date = self.date;
        metatable.raw_set(
            "weekly",
            lua.create_function(move |lua, maybe_date: Option<PdfDate>| {
                Ok(match lua.app_data_ref::<EnginePages>() {
                    Some(pages) => {
                        pages.get_page_by_date(EnginePageKind::Weekly, maybe_date.unwrap_or(date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the daily page for the current page (optionally taking a
        // date). This will retrieve the page from the weak reference, upgrading it in the process,
        // and can potentially return nil if no reference is found.
        let date = self.date;
        metatable.raw_set(
            "daily",
            lua.create_function(move |lua, maybe_date: Option<PdfDate>| {
                Ok(match lua.app_data_ref::<EnginePages>() {
                    Some(pages) => {
                        pages.get_page_by_date(EnginePageKind::Daily, maybe_date.unwrap_or(date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the next page for the current page based on its type and
        // date. This will retrieve the page from the weak reference, upgrading it in the process,
        // and can potentially return nil if no reference is found.
        let date = self.date;
        metatable.raw_set(
            "next_page",
            lua.create_function(move |lua, ()| {
                Ok(match lua.app_data_ref::<EnginePages>() {
                    Some(pages) => {
                        let date = match kind {
                            EnginePageKind::Daily => date.tomorrow(),
                            EnginePageKind::Monthly => date.next_month(),
                            EnginePageKind::Weekly => date.next_week(),
                        };

                        date.and_then(|date| pages.get_page_by_date(kind, date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the previous page for the current page based on its type
        // and date. This will retrieve the page from the weak reference, upgrading it in the
        // process, and can potentially return nil if no reference is found.
        let date = self.date;
        metatable.raw_set(
            "prev_page",
            lua.create_function(move |lua, ()| {
                Ok(match lua.app_data_ref::<EnginePages>() {
                    Some(pages) => {
                        let date = match kind {
                            EnginePageKind::Daily => date.yesterday(),
                            EnginePageKind::Monthly => date.last_month(),
                            EnginePageKind::Weekly => date.last_week(),
                        };

                        date.and_then(|date| pages.get_page_by_date(kind, date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function that supports pushing any PDF object into a queue that will be
        // drawn for the current PDF page. The object's depth will be used to determine where in
        // the queue to add the object itself.
        metatable.raw_set(
            "push",
            lua.create_function(move |_, obj: PdfObject| {
                // Add object to list for page `id` at object's depth
                if let Some(objects) = Weak::upgrade(&objects) {
                    objects
                        .write()
                        .unwrap()
                        .entry(obj.depth())
                        .or_default()
                        .push(obj);
                }

                Ok(())
            })?,
        )?;

        // Prevent altering the page object
        lua.mark_readonly(table.clone())?;

        Ok(LuaValue::Table(table))
    }
}
