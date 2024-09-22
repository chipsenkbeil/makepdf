use crate::pdf::{PdfHorizontalAlign, PdfLuaExt, PdfLuaTableExt, PdfPoint, PdfVerticalAlign};
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

    /// Returns a new bounds aligned to some other bounds based on `align`.
    pub fn align_to(&self, other: Self, align: (PdfVerticalAlign, PdfHorizontalAlign)) -> Self {
        let (valign, halign) = align;
        let x_offset = match halign {
            PdfHorizontalAlign::Left => other.ll.x - self.ll.x,
            PdfHorizontalAlign::Middle => {
                other.ll.x - self.ll.x + ((other.width() - self.width()) / 2.0)
            }
            PdfHorizontalAlign::Right => other.ur.x - self.ur.x,
        };

        let y_offset = match valign {
            PdfVerticalAlign::Top => other.ur.y - self.ur.y,
            PdfVerticalAlign::Middle => {
                other.ll.y - self.ll.y + ((other.height() - self.height()) / 2.0)
            }
            PdfVerticalAlign::Bottom => other.ll.y - self.ll.y,
        };

        let mut this = *self;
        this.ll.x += x_offset;
        this.ur.x += x_offset;
        this.ll.y += y_offset;
        this.ur.y += y_offset;
        this
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
            lua.create_function(move |_, this: Self| Ok(this.width().0))?,
        )?;

        metatable.raw_set(
            "height",
            lua.create_function(move |_, this: Self| Ok(this.height().0))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfBounds {
    /// Converts from any of
    ///
    /// - `{ll:{x:number, y:number}, ur:{x:number, y:number}}`
    /// - `{ll:{number, number}, ur:{number, number}}`
    /// - `{{number, number}, {number, number}}`
    /// - `{number, number, number, number}`
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
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

                // Otherwise, this table is not valid bounds
                Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.bounds",
                    message: Some(String::from("table is not bounds like")),
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from,
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
    fn should_support_aligning_with_another_set_of_bounds() {
        type H = PdfHorizontalAlign;
        type V = PdfVerticalAlign;

        // 5x5 square
        let this = PdfBounds::from_coords_f32(1.0, 1.0, 6.0, 6.0);

        // 20x20 square
        let other = PdfBounds::from_coords_f32(5.0, 5.0, 25.0, 25.0);

        // Upper-left
        let actual = this.align_to(other, (V::Top, H::Left));
        assert_eq!(actual.to_coords_f32(), (5.0, 20.0, 10.0, 25.0));

        // Upper-middle
        let actual = this.align_to(other, (V::Top, H::Middle));
        assert_eq!(actual.to_coords_f32(), (12.5, 20.0, 17.5, 25.0));

        // Upper-right
        let actual = this.align_to(other, (V::Top, H::Right));
        assert_eq!(actual.to_coords_f32(), (20.0, 20.0, 25.0, 25.0));

        // Middle-left
        let actual = this.align_to(other, (V::Middle, H::Left));
        assert_eq!(actual.to_coords_f32(), (5.0, 12.5, 10.0, 17.5));

        // Middle-middle
        let actual = this.align_to(other, (V::Middle, H::Middle));
        assert_eq!(actual.to_coords_f32(), (12.5, 12.5, 17.5, 17.5));

        // Middle-right
        let actual = this.align_to(other, (V::Middle, H::Right));
        assert_eq!(actual.to_coords_f32(), (20.0, 12.5, 25.0, 17.5));

        // Bottom-left
        let actual = this.align_to(other, (V::Bottom, H::Left));
        assert_eq!(actual.to_coords_f32(), (5.0, 5.0, 10.0, 10.0));

        // Bottom-middle
        let actual = this.align_to(other, (V::Bottom, H::Middle));
        assert_eq!(actual.to_coords_f32(), (12.5, 5.0, 17.5, 10.0));

        // Bottom-right
        let actual = this.align_to(other, (V::Bottom, H::Right));
        assert_eq!(actual.to_coords_f32(), (20.0, 5.0, 25.0, 10.0));
    }

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

        // Can convert { ll = { number, number }, ur = { number, number } } into bounds
        assert_eq!(
            Lua::new()
                .load(chunk!({ ll = { 1,  2 }, ur = { 3,  4 } }))
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
