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

        table.raw_set("x", self.x.0)?;
        table.raw_set("y", self.y.0)?;

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
                x: Mm(raw_get!(table, 0).or_else(|_| raw_get!(table, "x"))?),
                y: Mm(raw_get!(table, 1).or_else(|_| raw_get!(table, "y"))?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.point",
                message: None,
            }),
        }
    }
}
