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

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext<'_>) {
        for obj in self.objects.iter() {
            obj.draw(ctx);
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
