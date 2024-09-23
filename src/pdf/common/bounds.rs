use crate::pdf::{
    PdfAlign, PdfHorizontalAlign, PdfLuaExt, PdfLuaTableExt, PdfPadding, PdfPoint, PdfVerticalAlign,
};
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

    /// Applies `padding` to the bounds, returning a copy of the newly-adjusted bounds.
    pub fn with_padding(&self, padding: PdfPadding) -> Self {
        let mut this = *self;

        this.ll.x += padding.left;
        this.ll.y += padding.bottom;
        this.ur.x -= padding.right;
        this.ur.y -= padding.top;

        this
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

        self.shift_by(x_offset, y_offset)
    }

    /// Moves the bounds to `x` and `y`, placing the lower-left coordinate at that position,
    /// returning a copy of the new bounds.
    #[inline]
    pub fn move_to(&self, x: Mm, y: Mm) -> Self {
        // Figure out the offset we need to apply from the current lower-left to get there
        let x_offset = x - self.ll.x;
        let y_offset = y - self.ll.y;

        self.shift_by(x_offset, y_offset)
    }

    /// Shifts the bounds by `x_offset` and `y_offset`, returning a copy of the new bounds.
    #[inline]
    pub fn shift_by(&self, x_offset: Mm, y_offset: Mm) -> Self {
        let mut this = *self;
        this.ll.x += x_offset;
        this.ur.x += x_offset;
        this.ll.y += y_offset;
        this.ur.y += y_offset;
        this
    }

    /// Scales the bounds to fit `width` and `height`, returning a copy of the new bounds.
    #[inline]
    pub fn scale_to(&self, width: Mm, height: Mm) -> Self {
        // Figure out the difference in current width/height and desired width/height
        let width_diff = width - self.width();
        let height_diff = height - self.height();

        // Adjust the bounds by applying the difference to the upper-right coordinate
        let mut this = *self;
        this.ur.x += width_diff;
        this.ur.y += height_diff;
        this
    }

    /// Scales the bounds by a factor of `width` and `height`, returning a copy of the new bounds.
    #[inline]
    pub fn scale_by_factor(&self, width: f32, height: f32) -> Self {
        // Figure out desired absolute width and height from the scale factors
        let width = self.width() * width;
        let height = self.height() * height;

        self.scale_to(width, height)
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
            "align_to",
            lua.create_function(move |_, (this, other, align): (Self, Self, PdfAlign)| {
                Ok(this.align_to(other, align.to_v_h()))
            })?,
        )?;

        metatable.raw_set(
            "with_padding",
            lua.create_function(
                move |_, (this, opts): (Self, Option<PdfPadding>)| match opts {
                    Some(padding) => Ok(this.with_padding(padding)),
                    None => Ok(this),
                },
            )?,
        )?;

        metatable.raw_set(
            "move_to",
            lua.create_function(
                move |_, (this, opts): (Self, Option<LuaTable>)| match opts {
                    Some(opts) => {
                        let x = opts
                            .raw_get_ext::<_, Option<f32>>("x")?
                            .map(Mm)
                            .unwrap_or(this.ll.x);
                        let y = opts
                            .raw_get_ext::<_, Option<f32>>("y")?
                            .map(Mm)
                            .unwrap_or(this.ll.y);

                        Ok(this.move_to(x, y))
                    }
                    None => Ok(this),
                },
            )?,
        )?;

        metatable.raw_set(
            "shift_by",
            lua.create_function(
                move |_, (this, opts): (Self, Option<LuaTable>)| match opts {
                    Some(opts) => {
                        let x = opts
                            .raw_get_ext::<_, Option<f32>>("x")?
                            .map(Mm)
                            .unwrap_or_default();
                        let y = opts
                            .raw_get_ext::<_, Option<f32>>("y")?
                            .map(Mm)
                            .unwrap_or_default();

                        Ok(this.shift_by(x, y))
                    }
                    None => Ok(this),
                },
            )?,
        )?;

        metatable.raw_set(
            "scale_to",
            lua.create_function(
                move |_, (this, opts): (Self, Option<LuaTable>)| match opts {
                    Some(opts) => {
                        let width = opts
                            .raw_get_ext::<_, Option<f32>>("width")?
                            .map(Mm)
                            .unwrap_or_else(|| this.width());
                        let height = opts
                            .raw_get_ext::<_, Option<f32>>("height")?
                            .map(Mm)
                            .unwrap_or_else(|| this.height());

                        Ok(this.scale_to(width, height))
                    }
                    None => Ok(this),
                },
            )?,
        )?;

        metatable.raw_set(
            "scale_by_factor",
            lua.create_function(
                move |_, (this, opts): (Self, Option<LuaTable>)| match opts {
                    Some(opts) => {
                        let width = opts.raw_get_ext::<_, Option<f32>>("width")?.unwrap_or(1.0);
                        let height = opts.raw_get_ext::<_, Option<f32>>("height")?.unwrap_or(1.0);
                        Ok(this.scale_by_factor(width, height))
                    }
                    None => Ok(this),
                },
            )?,
        )?;

        metatable.raw_set(
            "width",
            lua.create_function(move |_, this: Self| Ok(this.width().0))?,
        )?;

        metatable.raw_set(
            "height",
            lua.create_function(move |_, this: Self| Ok(this.height().0))?,
        )?;

        metatable.raw_set(
            "to_coords",
            lua.create_function(move |_, this: Self| {
                // NOTE: We need to return a Vec<f32> to make it a table {nunber, nunber, ...}
                //       as returning a tuple makes it act like a vararg return instead.
                let (llx, lly, urx, ury) = this.to_coords_f32();
                Ok(vec![llx, lly, urx, ury])
            })?,
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
    fn should_support_with_padding() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        // Can apply positive padding to push bounds inward
        assert_eq!(
            bounds.with_padding(PdfPadding {
                top: Mm(0.1),
                right: Mm(0.2),
                bottom: Mm(0.3),
                left: Mm(0.4),
            }),
            PdfBounds::from_coords_f32(1.4, 2.3, 2.8, 3.9)
        );

        // Can apply negative padding to push bounds outward
        assert_eq!(
            bounds.with_padding(PdfPadding {
                top: Mm(-0.1),
                right: Mm(-0.2),
                bottom: Mm(-0.3),
                left: Mm(-0.4),
            }),
            PdfBounds::from_coords_f32(0.6, 1.7, 3.2, 4.1)
        );

        // Zero padding will make no adjustments
        assert_eq!(
            bounds.with_padding(PdfPadding {
                top: Mm(0.0),
                right: Mm(0.0),
                bottom: Mm(0.0),
                left: Mm(0.0),
            }),
            PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn should_support_with_padding_in_lua() {
        // NOTE: With smaller bounds and padding like 0.1, we get a crazy decimal
        //       such as 3.0 -> 2.9000000000036 or something like that. Not sure
        //       why, but to prevent this from affecting our test, we scale everything
        //       up by a factor of 10 to make it easier.
        let bounds = PdfBounds::from_coords_f32(10.0, 20.0, 30.0, 40.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // Supports adjusting neither parameter
                u.assert_deep_equal($bounds:with_padding(), {
                    ll = { x = 10,  y = 20 },
                    ur = { x = 30,  y = 40 },
                })
                u.assert_deep_equal($bounds:with_padding({}), {
                    ll = { x = 10,  y = 20 },
                    ur = { x = 30,  y = 40 },
                })

                // Supports adjusting a single parameters
                u.assert_deep_equal($bounds:with_padding({ top = 1 }), {
                    ll = { x = 10,  y = 20 },
                    ur = { x = 30,  y = 39 },
                })
                u.assert_deep_equal($bounds:with_padding({ right = 1 }), {
                    ll = { x = 10,    y = 20 },
                    ur = { x = 29,  y = 40 },
                })
                u.assert_deep_equal($bounds:with_padding({ bottom = 1 }), {
                    ll = { x = 10,  y = 21 },
                    ur = { x = 30,  y = 40 },
                })
                u.assert_deep_equal($bounds:with_padding({ left = 1 }), {
                    ll = { x = 11,  y = 20 },
                    ur = { x = 30,    y = 40 },
                })

                // Supports adjusting all parameters
                u.assert_deep_equal($bounds:with_padding({
                    top = 1,
                    right = 2,
                    bottom = 3,
                    left = 4,
                }), {
                    ll = { x = 14,  y = 23 },
                    ur = { x = 28,  y = 39 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_move_to() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        // Supports increases and decreases in position
        assert_eq!(
            bounds.move_to(Mm(5.0), Mm(1.0)),
            PdfBounds::from_coords_f32(5.0, 1.0, 7.0, 3.0)
        );

        // Moving to same lower-left should do nothing
        assert_eq!(
            bounds.move_to(Mm(1.0), Mm(2.0)),
            PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn should_support_move_to_in_lua() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // Supports adjusting neither parameter
                u.assert_deep_equal($bounds:move_to(), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })
                u.assert_deep_equal($bounds:move_to({}), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })

                // Supports adjusting a single parameters
                u.assert_deep_equal($bounds:move_to({ x = 5 }), {
                    ll = { x = 5,  y = 2 },
                    ur = { x = 7,  y = 4 },
                })
                u.assert_deep_equal($bounds:move_to({ y = 1 }), {
                    ll = { x = 1,  y = 1 },
                    ur = { x = 3,  y = 3 },
                })

                // Supports adjusting both parameters
                u.assert_deep_equal($bounds:move_to({ x = 5, y = 1 }), {
                    ll = { x = 5,  y = 1 },
                    ur = { x = 7,  y = 3 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_shift_by() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        // Supports increases and decreases in position
        assert_eq!(
            bounds.shift_by(Mm(4.0), Mm(-1.0)),
            PdfBounds::from_coords_f32(5.0, 1.0, 7.0, 3.0)
        );

        // Shifting by 0 should do nothing
        assert_eq!(
            bounds.shift_by(Mm(0.0), Mm(0.0)),
            PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn should_support_shift_by_in_lua() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // Supports adjusting neither parameter
                u.assert_deep_equal($bounds:shift_by(), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })
                u.assert_deep_equal($bounds:shift_by({}), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })

                // Supports adjusting a single parameters
                u.assert_deep_equal($bounds:shift_by({ x = 4 }), {
                    ll = { x = 5,  y = 2 },
                    ur = { x = 7,  y = 4 },
                })
                u.assert_deep_equal($bounds:shift_by({ y = -1 }), {
                    ll = { x = 1,  y = 1 },
                    ur = { x = 3,  y = 3 },
                })

                // Supports adjusting both parameters
                u.assert_deep_equal($bounds:shift_by({ x = 4, y = -1 }), {
                    ll = { x = 5,  y = 1 },
                    ur = { x = 7,  y = 3 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_scale_to() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        // Supports scaling to larger and smaller dimensions
        assert_eq!(
            bounds.scale_to(Mm(4.0), Mm(1.0)),
            PdfBounds::from_coords_f32(1.0, 2.0, 5.0, 3.0)
        );

        // Scaling to the same dimensions should do nothing
        assert_eq!(
            bounds.scale_to(bounds.width(), bounds.height()),
            PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn should_support_scale_to_in_lua() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // Supports adjusting neither parameter
                u.assert_deep_equal($bounds:scale_to(), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })
                u.assert_deep_equal($bounds:scale_to({}), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })

                // Supports adjusting a single parameters
                u.assert_deep_equal($bounds:scale_to({ width = 4 }), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 5,  y = 4 },
                })
                u.assert_deep_equal($bounds:scale_to({ height = 1 }), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 3 },
                })

                // Supports adjusting both parameters
                u.assert_deep_equal($bounds:scale_to({ width = 4, height = 1 }), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 5,  y = 3 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_scale_by_factor() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        // Supports scaling to larger and smaller dimensions
        assert_eq!(
            bounds.scale_by_factor(2.0, 0.5),
            PdfBounds::from_coords_f32(1.0, 2.0, 5.0, 3.0)
        );

        // Scaling by a factor of 1 should do nothing
        assert_eq!(
            bounds.scale_by_factor(1.0, 1.0),
            PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn should_support_scale_by_factor_in_lua() {
        let bounds = PdfBounds::from_coords_f32(1.0, 2.0, 3.0, 4.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // Supports adjusting neither parameter
                u.assert_deep_equal($bounds:scale_by_factor(), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })
                u.assert_deep_equal($bounds:scale_by_factor({}), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 4 },
                })

                // Supports adjusting a single parameters
                u.assert_deep_equal($bounds:scale_by_factor({ width = 2 }), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 5,  y = 4 },
                })
                u.assert_deep_equal($bounds:scale_by_factor({ height = 0.5 }), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 3,  y = 3 },
                })

                // Supports adjusting both parameters
                u.assert_deep_equal($bounds:scale_by_factor({ width = 2, height = 0.5 }), {
                    ll = { x = 1,  y = 2 },
                    ur = { x = 5,  y = 3 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }

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
