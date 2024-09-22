use crate::pdf::PdfLuaTableExt;
use mlua::prelude::*;
use palette::Srgb;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Color for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfColor(Srgb);

impl PdfColor {
    /// Produces color from RGB where each value is between 0 and 1.
    pub const fn from_rgb_f32(red: f32, green: f32, blue: f32) -> Self {
        Self(Srgb::new(red, green, blue))
    }

    /// Produces color from RGB where each value is between 0 and 255.
    pub fn from_rgb_u8(red: u8, green: u8, blue: u8) -> Self {
        Self(Srgb::new(red, green, blue).into())
    }

    /// Produces a traditional black color.
    #[inline]
    pub const fn black() -> Self {
        Self::from_rgb_f32(0.0, 0.0, 0.0)
    }

    /// Produces a traditional grey color.
    #[inline]
    pub const fn grey() -> Self {
        let c = 0.313_725_5; // == 80 / 255
        Self::from_rgb_f32(c, c, c)
    }

    /// Produces a traditional blue color.
    #[inline]
    pub const fn blue() -> Self {
        Self::from_rgb_f32(0.0, 0.0, 1.0)
    }

    /// Produces a traditional green color.
    #[inline]
    pub const fn green() -> Self {
        Self::from_rgb_f32(0.0, 1.0, 0.0)
    }

    /// Produces a traditional red color.
    #[inline]
    pub const fn red() -> Self {
        Self::from_rgb_f32(1.0, 0.0, 0.0)
    }

    /// Produces a traditional white color.
    #[inline]
    pub const fn white() -> Self {
        Self::from_rgb_f32(1.0, 1.0, 1.0)
    }
}

impl Deref for PdfColor {
    type Target = Srgb;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PdfColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for PdfColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", Srgb::<u8>::from(self.0))
    }
}

impl FromStr for PdfColor {
    type Err = palette::rgb::FromHexError;

    /// Parses a hex string into a color.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<Srgb<u8>>()?.into()))
    }
}

impl From<PdfColor> for printpdf::Color {
    /// Converts a PDF color into an RGB format.
    fn from(color: PdfColor) -> Self {
        Self::Rgb(printpdf::Rgb {
            r: color.red,
            g: color.green,
            b: color.blue,
            icc_profile: None,
        })
    }
}

impl<'lua> IntoLua<'lua> for PdfColor {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(format!("{:X}", Srgb::<u8>::from(self.0)))
            .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfColor {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_str()?.parse().map_err(LuaError::external)?),
            LuaValue::Table(table) => {
                let maybe_vec_u8: Option<Vec<u8>> = table
                    .clone()
                    .sequence_values()
                    .collect::<LuaResult<_>>()
                    .ok();

                // If we have color vec, check to make sure we have three, and use them as rgb
                if let Some(v) = maybe_vec_u8 {
                    if v.len() >= 3 {
                        return Ok(Self::from_rgb_u8(v[0], v[1], v[2]));
                    }
                }

                // Otherwise, look for r, g, b fields
                Ok(Self::from_rgb_u8(
                    table.raw_get_ext("r")?,
                    table.raw_get_ext("g")?,
                    table.raw_get_ext("b")?,
                ))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.color",
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
        let color = PdfColor::from_rgb_u8(0, 128, 255);

        // Can convert uppercase hex string into color
        assert_eq!(
            Lua::new()
                .load(chunk!("0080FF"))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert lowercase hex string into color
        assert_eq!(
            Lua::new()
                .load(chunk!("0080ff"))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert hex string starting with # into color
        assert_eq!(
            Lua::new()
                .load(chunk!("#0080ff"))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert { number, number, number } (u8) into color
        assert_eq!(
            Lua::new()
                .load(chunk!({0, 128, 255}))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert { r:number, g:number, b:number } (u8) into color
        assert_eq!(
            Lua::new()
                .load(chunk!({ r = 0, g = 128, b = 255 }))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let color = PdfColor::from_rgb_u8(0, 128, 255);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($color, "0080FF")
            })
            .exec()
            .expect("Assertion failed");
    }
}
