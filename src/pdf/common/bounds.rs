use mlua::prelude::*;
use printpdf::Mm;

/// Coordinate bounds for something within a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfBounds {
    /// Lower-left x coordinate
    pub llx: Mm,
    /// Lower-left y coordinate
    pub lly: Mm,
    /// Upper-right x coordinate
    pub urx: Mm,
    /// Upper-right y coordinate
    pub ury: Mm,
}

impl PdfBounds {
    /// Returns the width of the bounds.
    ///
    /// This is the difference between the upper-right x and lower-left x, and will clip to 0 if
    /// the upper-right coordinate is less than the lower-left.
    pub fn width(&self) -> Mm {
        if self.urx > self.llx {
            self.urx - self.llx
        } else {
            Mm(0.0)
        }
    }

    /// Returns the height of the bounds.
    ///
    /// This is the difference between the upper-right y and lower-left y, and will clip to 0 if
    /// the upper-right coordinate is less than the lower-left.
    pub fn height(&self) -> Mm {
        if self.ury > self.lly {
            self.ury - self.lly
        } else {
            Mm(0.0)
        }
    }

    /// Converts bounds into (llx, lly, urx, ury).
    pub fn to_coords(&self) -> (Mm, Mm, Mm, Mm) {
        (self.llx, self.lly, self.urx, self.ury)
    }
}

impl<'lua> IntoLua<'lua> for PdfBounds {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("llx", self.llx.0)?;
        table.raw_set("lly", self.lly.0)?;
        table.raw_set("urx", self.urx.0)?;
        table.raw_set("ury", self.ury.0)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfBounds {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                llx: Mm(raw_get!(table, "llx")?),
                lly: Mm(raw_get!(table, "lly")?),
                urx: Mm(raw_get!(table, "urx")?),
                ury: Mm(raw_get!(table, "ury")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.bounds",
                message: None,
            }),
        }
    }
}
