use mlua::prelude::*;
use palette::Srgb;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Spacing for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfColor(Srgb);

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

impl ToString for PdfColor {
    /// Converts color to an uppercase hex string.
    fn to_string(&self) -> String {
        format!("{:X}", Srgb::<u8>::from(self.0))
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
