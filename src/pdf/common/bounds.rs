use crate::pdf::{PdfLuaExt, PdfLuaTableExt, PdfPoint};
use mlua::prelude::*;
use printpdf::{Mm, Rect};

/// Coordinate bounds for something within a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfBounds {
    /// Lower-left coordinates
    pub ll: PdfPoint,
    /// Upper-right coordinates
    pub ur: PdfPoint,
}

impl From<PdfBounds> for Rect {
    fn from(bounds: PdfBounds) -> Self {
        Self::new(bounds.ll.x, bounds.ll.y, bounds.ur.x, bounds.ur.y)
    }
}

impl PdfBounds {
    /// Creates a new set of bounds identified by lower-left and upper-right points.
    pub const fn new(ll: PdfPoint, ur: PdfPoint) -> Self {
        Self { ll, ur }
    }

    /// Returns the width of the bounds.
    ///
    /// This is the difference between the upper-right x and lower-left x, and will clip to 0 if
    /// the upper-right coordinate is less than the lower-left.
    pub fn width(&self) -> Mm {
        if self.ur.x > self.ll.x {
            self.ur.x - self.ll.x
        } else {
            Mm(0.0)
        }
    }

    /// Returns the height of the bounds.
    ///
    /// This is the difference between the upper-right y and lower-left y, and will clip to 0 if
    /// the upper-right coordinate is less than the lower-left.
    pub fn height(&self) -> Mm {
        if self.ur.y > self.ll.y {
            self.ur.y - self.ll.y
        } else {
            Mm(0.0)
        }
    }

    /// Converts coordinates into bounds.
    #[inline]
    pub const fn from_coords(llx: Mm, lly: Mm, urx: Mm, ury: Mm) -> Self {
        Self {
            ll: PdfPoint::new(llx, lly),
            ur: PdfPoint::new(urx, ury),
        }
    }

    /// Converts coordinates into bounds.
    #[inline]
    pub const fn from_coords_f32(llx: f32, lly: f32, urx: f32, ury: f32) -> Self {
        Self::from_coords(Mm(llx), Mm(lly), Mm(urx), Mm(ury))
    }

    /// Converts bounds into (llx, lly, urx, ury).
    #[inline]
    pub const fn to_coords(&self) -> (Mm, Mm, Mm, Mm) {
        (self.ll.x, self.ll.y, self.ur.x, self.ur.y)
    }

    /// Converts bounds into (llx, lly, urx, ury).
    #[inline]
    pub const fn to_coords_f32(&self) -> (f32, f32, f32, f32) {
        let (llx, lly, urx, ury) = self.to_coords();
        (llx.0, lly.0, urx.0, ury.0)
    }

    /// Adds bounds fields to an existing Lua table.
    pub fn add_to_table(&self, table: &LuaTable) -> LuaResult<()> {
        table.raw_set("ll", self.ll)?;
        table.raw_set("ur", self.ur)?;
        Ok(())
    }
}

impl<'lua> IntoLua<'lua> for PdfBounds {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        self.add_to_table(&table)?;

        metatable.raw_set(
            "width",
            lua.create_function(move |_, this: Option<Self>| {
                Ok(this
                    .map(|this| this.width())
                    .unwrap_or_else(|| self.width())
                    .0)
            })?,
        )?;

        metatable.raw_set(
            "height",
            lua.create_function(move |_, this: Option<Self>| {
                Ok(this
                    .map(|this| this.height())
                    .unwrap_or_else(|| self.height())
                    .0)
            })?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfBounds {
    /// Converts from any of
    ///
    /// - `{ll:{x:number, y:number}, ur:{x:number, y:number}}`
    /// - `{llx:number, lly:number, urx:number, ury:number}`
    /// - `{{number, number}, {number, number}}`
    /// - `{number, number, number, number}`
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                let maybe_coords: Option<Vec<f32>> = table
                    .clone()
                    .sequence_values()
                    .collect::<LuaResult<_>>()
                    .ok();

                let maybe_points: Option<Vec<PdfPoint>> = table
                    .clone()
                    .sequence_values()
                    .collect::<LuaResult<_>>()
                    .ok();

                // If we have coordinates, check to make sure we have four, and use them as bounds
                if let Some(coords) = maybe_coords {
                    if coords.len() >= 4 {
                        return Ok(Self::from_coords_f32(
                            coords[0], coords[1], coords[2], coords[3],
                        ));
                    }
                }

                // If we have points, check to make sure we have two, and use them as bounds
                if let Some(points) = maybe_points {
                    if points.len() >= 2 {
                        return Ok(Self::new(points[0], points[1]));
                    }
                }

                // If we have point fields, use them as bounds
                if let (Ok(ll), Ok(ur)) = (table.raw_get_ext("ll"), table.raw_get_ext("ur")) {
                    return Ok(Self::new(ll, ur));
                }

                // Otherwise, try to get coordinate fields and use them as bounds
                Ok(Self::new(
                    PdfPoint::new(Mm(table.raw_get_ext("llx")?), Mm(table.raw_get_ext("lly")?)),
                    PdfPoint::new(Mm(table.raw_get_ext("urx")?), Mm(table.raw_get_ext("ury")?)),
                ))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.bounds",
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
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        // Can convert { { number, number }, { number, number } } into bounds
        assert_eq!(
            Lua::new()
                .load(chunk!({ {1, 2}, {3, 4} }))
                .eval::<PdfBounds>()
                .unwrap(),
            bounds,
        );

        // Can convert { number, number, number, number } into bounds
        assert_eq!(
            Lua::new()
                .load(chunk!({1, 2, 3, 4}))
                .eval::<PdfBounds>()
                .unwrap(),
            bounds,
        );

        // Can convert { llx, lly,  urx, ury } into bounds
        assert_eq!(
            Lua::new()
                .load(chunk!({ llx = 1, lly = 2, urx = 3, ury = 4 }))
                .eval::<PdfBounds>()
                .unwrap(),
            bounds,
        );

        // Can convert { ll = { x, y }, ur = { x, y } } into bounds
        assert_eq!(
            Lua::new()
                .load(chunk!({ ll = { x = 1,  y = 2 }, ur = { x = 3,  y = 4 } }))
                .eval::<PdfBounds>()
                .unwrap(),
            bounds,
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($bounds, {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }
}
