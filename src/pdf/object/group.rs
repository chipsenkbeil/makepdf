use crate::pdf::{PdfBounds, PdfContext, PdfObject};
use mlua::prelude::*;

/// Represents a group of objects to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectGroup {
    pub objects: Vec<PdfObject>,
}

impl PdfObjectGroup {
    /// Returns bounds for the group by calculating the bounds of each object within the group and
    /// returning the minimum bounds that will contain all of them.
    pub fn bounds(&self, ctx: PdfContext<'_>) -> PdfBounds {
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
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectGroup {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        for obj in self.objects {
            table.raw_push(obj)?;
        }

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectGroup {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                objects: table.sequence_values().collect::<LuaResult<_>>()?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.group",
                message: None,
            }),
        }
    }
}
