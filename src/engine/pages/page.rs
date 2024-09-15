use super::{EnginePageKind, WeakEnginePagesMap};
use crate::pdf::{PdfContext, PdfDate, PdfLuaExt, PdfObject};
use chrono::Datelike;
use mlua::prelude::*;
use printpdf::{PdfLayerIndex, PdfPageIndex};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

pub type EnginePageKey = (EnginePageKind, u32);

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

    /// Internal indexes used to access page and layer references.
    pub indexes: (PdfPageIndex, PdfLayerIndex),

    /// Collection of page key -> page.
    pub(super) pages: WeakEnginePagesMap,

    /// Collection of objects to add to the PDF.
    ///
    /// Page Id -> Depth -> Objects
    pub(super) objects: Arc<RwLock<BTreeMap<i64, Vec<PdfObject>>>>,
}

impl EnginePage {
    /// Returns the key associated with the page.
    pub fn key(&self) -> EnginePageKey {
        Self::key_impl(self.kind, self.date)
    }

    fn key_impl(kind: EnginePageKind, date: PdfDate) -> EnginePageKey {
        (
            kind,
            match kind {
                EnginePageKind::Daily => date.ordinal0(),
                EnginePageKind::Monthly => date.month0(),
                EnginePageKind::Weekly => date.iso_week().week0(),
            },
        )
    }

    /// Retrieves the next page in sequence based on the current page's kind (daily, weekly,
    /// or monthly).
    pub fn next_page(&self) -> Option<EnginePage> {
        let date = match self.kind {
            EnginePageKind::Daily => self.date.tomorrow(),
            EnginePageKind::Monthly => self.date.next_month(),
            EnginePageKind::Weekly => self.date.next_week(),
        };

        date.and_then(|date| self.page_at(self.kind, date))
    }

    /// Retrieves the previous page in sequence based on the current page's kind (daily, weekly,
    /// or monthly).
    pub fn prev_page(&self) -> Option<EnginePage> {
        let date = match self.kind {
            EnginePageKind::Daily => self.date.yesterday(),
            EnginePageKind::Monthly => self.date.last_month(),
            EnginePageKind::Weekly => self.date.last_week(),
        };

        date.and_then(|date| self.page_at(self.kind, date))
    }

    /// Retrieves the page of `kind` at `date`.
    pub fn page_at(&self, kind: EnginePageKind, date: PdfDate) -> Option<EnginePage> {
        self.pages
            .upgrade()
            .and_then(|pages| pages.get_page(kind, date))
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

        let table = lua.create_table()?;
        table.raw_set("id", self.id)?;
        table.raw_set("date", self.date)?;

        // Define a field function to get the monthly page for the current page (optionally taking
        // a date). This will retrieve the page from the weak reference, upgrading it in the
        // process, and can potentially return nil if no reference is found.
        let date = self.date;
        let pages = self.pages.clone();
        table.raw_set(
            "monthly",
            lua.create_function(move |_, maybe_date: Option<PdfDate>| {
                Ok(match pages.upgrade() {
                    Some(pages) => {
                        pages.get_page(EnginePageKind::Monthly, maybe_date.unwrap_or(date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the weekly page for the current page (optionally taking a
        // date). This will retrieve the page from the weak reference, upgrading it in the process,
        // and can potentially return nil if no reference is found.
        let date = self.date;
        let pages = self.pages.clone();
        table.raw_set(
            "weekly",
            lua.create_function(move |_, maybe_date: Option<PdfDate>| {
                Ok(match pages.upgrade() {
                    Some(pages) => {
                        pages.get_page(EnginePageKind::Weekly, maybe_date.unwrap_or(date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the daily page for the current page (optionally taking a
        // date). This will retrieve the page from the weak reference, upgrading it in the process,
        // and can potentially return nil if no reference is found.
        let date = self.date;
        let pages = self.pages.clone();
        table.raw_set(
            "daily",
            lua.create_function(move |_, maybe_date: Option<PdfDate>| {
                Ok(match pages.upgrade() {
                    Some(pages) => {
                        pages.get_page(EnginePageKind::Daily, maybe_date.unwrap_or(date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the next page for the current page based on its type and
        // date. This will retrieve the page from the weak reference, upgrading it in the process,
        // and can potentially return nil if no reference is found.
        let date = self.date;
        let pages = self.pages.clone();
        table.raw_set(
            "next_page",
            lua.create_function(move |_, ()| {
                Ok(match pages.upgrade() {
                    Some(pages) => {
                        let date = match kind {
                            EnginePageKind::Daily => date.tomorrow(),
                            EnginePageKind::Monthly => date.next_month(),
                            EnginePageKind::Weekly => date.next_week(),
                        };

                        date.and_then(|date| pages.get_page(kind, date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function to get the previous page for the current page based on its type
        // and date. This will retrieve the page from the weak reference, upgrading it in the
        // process, and can potentially return nil if no reference is found.
        let date = self.date;
        let pages = self.pages.clone();
        table.raw_set(
            "prev_page",
            lua.create_function(move |_, ()| {
                Ok(match pages.upgrade() {
                    Some(pages) => {
                        let date = match kind {
                            EnginePageKind::Daily => date.yesterday(),
                            EnginePageKind::Monthly => date.last_month(),
                            EnginePageKind::Weekly => date.last_week(),
                        };

                        date.and_then(|date| pages.get_page(kind, date))
                    }
                    None => None,
                })
            })?,
        )?;

        // Define a field function that supports pushing any PDF object into a queue that will be
        // drawn for the current PDF page. The object's depth will be used to determine where in
        // the queue to add the object itself.
        table.raw_set(
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
