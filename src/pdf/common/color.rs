use mlua::prelude::*;
use palette::Srgb;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Color for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfColor(Srgb);

impl PdfColor {
    /// Produces a traditional black color.
    pub const fn black() -> Self {
        Self(Srgb::new(0.0, 0.0, 0.0))
    }

    /// Produces a traditional blue color.
    pub const fn blue() -> Self {
        Self(Srgb::new(0.0, 0.0, 1.0))
    }

    /// Produces a traditional green color.
    pub const fn green() -> Self {
        Self(Srgb::new(0.0, 1.0, 0.0))
    }

    /// Produces a traditional red color.
    pub const fn red() -> Self {
        Self(Srgb::new(1.0, 0.0, 0.0))
    }

    /// Produces a traditional white color.
    pub const fn white() -> Self {
        Self(Srgb::new(1.0, 1.0, 1.0))
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
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.color",
                message: None,
            }),
        }
    }
}
