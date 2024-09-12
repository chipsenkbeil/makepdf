use crate::pdf::PdfPoint;
use mlua::prelude::*;
use printpdf::Mm;

/// Coordinate bounds for something within a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfBounds {
    /// Lower-left coordinates
    pub ll: PdfPoint,
    /// Upper-right coordinates
    pub ur: PdfPoint,
}

impl PdfBounds {
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

    /// Converts bounds into (llx, lly, urx, ury).
    pub fn to_coords(&self) -> (Mm, Mm, Mm, Mm) {
        (self.ll.x, self.ll.y, self.ur.x, self.ur.y)
    }

    /// Adds bounds fields to an existing Lua table.
    pub fn add_to_table(&self, table: &LuaTable) -> LuaResult<()> {
        table.raw_set("llx", self.ll.x.0)?;
        table.raw_set("lly", self.ll.y.0)?;
        table.raw_set("urx", self.ur.x.0)?;
        table.raw_set("ury", self.ur.y.0)?;
        Ok(())
    }
}

impl<'lua> IntoLua<'lua> for PdfBounds {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        self.add_to_table(&table)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfBounds {
    /// Converts from either
    ///
    /// - `{llx:number, lly:number, urx:number, ury:number}`
    /// - `{number, number, number, number}`
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                ll: PdfPoint::new(
                    Mm(raw_get!(table, 0).or_else(|_| raw_get!(table, "llx"))?),
                    Mm(raw_get!(table, 1).or_else(|_| raw_get!(table, "lly"))?),
                ),
                ur: PdfPoint::new(
                    Mm(raw_get!(table, 2).or_else(|_| raw_get!(table, "urx"))?),
                    Mm(raw_get!(table, 3).or_else(|_| raw_get!(table, "ury"))?),
                ),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.bounds",
                message: None,
            }),
        }
    }
}