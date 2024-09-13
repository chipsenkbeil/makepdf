use crate::pdf::PdfLuaTableExt;
use mlua::prelude::*;
use printpdf::{Mm, Point};

/// Coordinate x,y for something within a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfPoint {
    pub x: Mm,
    pub y: Mm,
}

impl PdfPoint {
    /// Creates a new point at x, y.
    pub fn new(x: Mm, y: Mm) -> Self {
        Self { x, y }
    }

    /// Converts bounds into (x, y).
    pub fn to_coords(&self) -> (Mm, Mm) {
        (self.x, self.y)
    }

    /// Adds point fields to an existing Lua table.
    pub fn add_to_table(&self, table: &LuaTable) -> LuaResult<()> {
        table.raw_set("x", self.x.0)?;
        table.raw_set("y", self.y.0)?;
        Ok(())
    }
}

impl From<PdfPoint> for Point {
    fn from(point: PdfPoint) -> Self {
        Self::new(point.x, point.y)
    }
}

impl<'lua> IntoLua<'lua> for PdfPoint {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        self.add_to_table(&table)?;
        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfPoint {
    /// Converts from either
    ///
    /// - `{x:number, y:number}`
    /// - `{number, number}`
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                x: Mm(table.raw_get_ext(0).or_else(|_| table.raw_get_ext("x"))?),
                y: Mm(table.raw_get_ext(1).or_else(|_| table.raw_get_ext("y"))?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.point",
                message: None,
            }),
        }
    }
}
