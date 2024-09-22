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
    #[inline]
    pub const fn new(x: Mm, y: Mm) -> Self {
        Self::from_coords(x, y)
    }

    /// Converts coordinates into point.
    #[inline]
    pub const fn from_coords(x: Mm, y: Mm) -> Self {
        Self { x, y }
    }

    /// Converts coordinates into point.
    #[inline]
    pub const fn from_coords_f32(x: f32, y: f32) -> Self {
        Self::from_coords(Mm(x), Mm(y))
    }

    /// Converts point into (x, y).
    #[inline]
    pub const fn to_coords(&self) -> (Mm, Mm) {
        (self.x, self.y)
    }

    /// Converts point into (x, y).
    #[inline]
    pub const fn to_coords_f32(&self) -> (f32, f32) {
        let (x, y) = self.to_coords();
        (x.0, y.0)
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
        let from = value.type_name();
        match value {
            LuaValue::Table(table) => {
                let coords: Vec<f32> = table.clone().sequence_values().collect::<LuaResult<_>>()?;

                // If we have coordinates, make sure there are two, and use them as point
                if coords.len() >= 2 {
                    return Ok(Self::from_coords_f32(coords[0], coords[1]));
                }

                // If we have point fields, use them as a point
                if let (Ok(x), Ok(y)) = (table.raw_get_ext("x"), table.raw_get_ext("y")) {
                    return Ok(Self::from_coords_f32(x, y));
                }

                // Otherwise, this table is not valid point
                Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.point",
                    message: Some(String::from("table is not point like")),
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.point",
                message: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::PdfUtils;
    use mlua::chunk;

    #[test]
    fn should_be_able_to_convert_from_lua() {
        let point = PdfPoint::from_coords_f32(1.0, 2.0);

        // Can convert { number, number } into point
        assert_eq!(
            Lua::new().load(chunk!({1, 2})).eval::<PdfPoint>().unwrap(),
            point,
        );

        // Can convert { x, y } into point
        assert_eq!(
            Lua::new()
                .load(chunk!({ x = 1, y = 2 }))
                .eval::<PdfPoint>()
                .unwrap(),
            point,
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let point = PdfPoint::from_coords_f32(1.0, 2.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($point, { x = 1,  y = 2 })
            })
            .exec()
            .expect("Assertion failed");
    }
}
