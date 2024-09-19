use crate::pdf::{
    PdfBounds, PdfContext, PdfLink, PdfLinkAnnotation, PdfLuaExt, PdfLuaTableExt, PdfObject,
};
use mlua::prelude::*;

/// Represents a group of objects to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectGroup {
    pub objects: Vec<PdfObject>,
    pub link: Option<PdfLink>,
}

impl PdfObjectGroup {
    /// Returns bounds for the group by calculating the bounds of each object within the group and
    /// returning the minimum bounds that will contain all of them.
    pub fn bounds(&self, ctx: PdfContext) -> PdfBounds {
        let mut bounds = PdfBounds::default();

        for obj in self.objects.iter() {
            let b = obj.bounds(ctx);
            if b.ll.x < bounds.ll.x {
                bounds.ll.x = b.ll.x;
            }

            if b.ur.x > bounds.ur.x {
                bounds.ur.x = b.ur.x;
            }

            if b.ll.y < bounds.ll.y {
                bounds.ll.y = b.ll.x;
            }

            if b.ur.y > bounds.ur.y {
                bounds.ur.y = b.ur.y;
            }
        }

        bounds
    }

    /// Returns bounds for the group by calculating the bounds of each object within the group and
    /// returning the minimum bounds that will contain all of them.
    ///
    /// Calculates bounds from a [`Lua`] runtime, which occurs earlier than when a [`PdfContext`]
    /// is available.
    pub(crate) fn lua_bounds(&self, lua: &Lua) -> LuaResult<PdfBounds> {
        let mut bounds = PdfBounds::default();

        for obj in self.objects.iter() {
            let b = obj.lua_bounds(lua)?;
            if b.ll.x < bounds.ll.x {
                bounds.ll.x = b.ll.x;
            }

            if b.ur.x > bounds.ur.x {
                bounds.ur.x = b.ur.x;
            }

            if b.ll.y < bounds.ll.y {
                bounds.ll.y = b.ll.x;
            }

            if b.ur.y > bounds.ur.y {
                bounds.ur.y = b.ur.y;
            }
        }

        Ok(bounds)
    }

    /// Returns a collection of link annotations.
    pub fn link_annotations(&self, ctx: PdfContext) -> Vec<PdfLinkAnnotation> {
        // Get initial links for group overall
        let mut links = match self.link.clone() {
            Some(link) => vec![PdfLinkAnnotation {
                bounds: self.bounds(ctx),
                depth: self.depth(),
                link,
            }],
            None => Vec::new(),
        };

        // Combine it with each object's links
        for obj in self.objects.iter() {
            links.extend(obj.link_annotations(ctx));
        }

        links
    }

    /// Returns depth for the group by examining the depth of each object and selecting the
    /// largest depth that will include them all. This means that objects with an earlier depth
    /// will be drawn at a later position.
    pub fn depth(&self) -> i64 {
        self.objects
            .iter()
            .map(|obj| obj.depth())
            .max()
            .unwrap_or_default()
    }

    /// Pushes a [`PdfObject`] into the end of the group.
    pub fn push(&mut self, obj: impl Into<PdfObject>) {
        self.objects.push(obj.into());
    }

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext<'_>) {
        for obj in self.objects.iter() {
            obj.draw(ctx);
        }
    }

    /// Returns an iterator over the objects grouped together.
    pub fn iter(&self) -> impl Iterator<Item = &PdfObject> {
        self.objects.iter()
    }

    /// Returns a mutable iterator over the objects grouped together.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PdfObject> {
        self.objects.iter_mut()
    }
}

impl<'a> IntoIterator for &'a PdfObjectGroup {
    type Item = &'a PdfObject;
    type IntoIter = std::slice::Iter<'a, PdfObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter()
    }
}

impl<'a> IntoIterator for &'a mut PdfObjectGroup {
    type Item = &'a mut PdfObject;
    type IntoIter = std::slice::IterMut<'a, PdfObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter_mut()
    }
}

impl IntoIterator for PdfObjectGroup {
    type Item = PdfObject;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
    }
}

impl FromIterator<PdfObject> for PdfObjectGroup {
    fn from_iter<I: IntoIterator<Item = PdfObject>>(iter: I) -> Self {
        Self {
            objects: iter.into_iter().collect(),
            link: None,
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectGroup {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        for obj in self.objects {
            table.raw_push(obj)?;
        }

        table.raw_set("link", self.link)?;

        metatable.raw_set(
            "bounds",
            lua.create_function(move |lua, this: Self| this.lua_bounds(lua))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectGroup {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                objects: table.clone().sequence_values().collect::<LuaResult<_>>()?,
                link: table.raw_get_ext("link")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.group",
                message: None,
            }),
        }
    }
}
