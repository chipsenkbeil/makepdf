use crate::pdf::{PdfContext, PdfLinkAnnotation, PdfLuaExt, PdfObject};
use mlua::prelude::*;
use printpdf::Mm;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

/// Type of unique id associated with a page.
pub type RuntimePageId = u32;

/// Lua-only struct providing an interface for specialized operations within Lua to manipulate a
/// PDF page.
#[derive(Clone, Debug)]
pub struct RuntimePage {
    /// Unique id associated with the page.
    pub id: RuntimePageId,

    /// Title associated with the page.
    pub title: String,

    /// Optional, explicit width of the page.
    pub width: Option<Mm>,

    /// Optional, explicit height of the page.
    pub height: Option<Mm>,

    /// Collection of objects to add to the PDF.
    ///
    /// Page Id -> Depth -> Objects
    objects: Arc<RwLock<BTreeMap<i64, Vec<PdfObject>>>>,
}

impl RuntimePage {
    /// Creates a new empty page.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: rand::random(),
            title: title.into(),
            width: None,
            height: None,
            objects: Default::default(),
        }
    }

    /// Returns a collection of link annotations associated with the page.
    pub fn link_annotations(&self, ctx: PdfContext) -> Vec<PdfLinkAnnotation> {
        let mut annotations = Vec::new();

        for (_, objs) in self.objects.read().unwrap().iter() {
            for obj in objs {
                annotations.extend(obj.link_annotations(ctx));
            }
        }

        annotations
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

impl<'lua> IntoLua<'lua> for RuntimePage {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let objects = Arc::downgrade(&self.objects);

        let (table, metatable) = lua.create_table_ext()?;
        table.raw_set("id", self.id)?;
        table.raw_set("title", self.title)?;
        table.raw_set("width", self.width.map(|x| x.0))?;
        table.raw_set("height", self.height.map(|x| x.0))?;

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
