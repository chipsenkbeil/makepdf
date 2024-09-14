use chrono::prelude::*;
use mlua::prelude::*;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Date for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfDate(NaiveDate);

impl Deref for PdfDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PdfDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for PdfDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl From<NaiveDate> for PdfDate {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl From<PdfDate> for NaiveDate {
    fn from(date: PdfDate) -> Self {
        date.0
    }
}

impl FromStr for PdfDate {
    type Err = chrono::format::ParseError;

    /// Parses a hex string into a color.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl<'lua> IntoLua<'lua> for PdfDate {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(self.to_string()).map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfDate {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_str()?.parse().map_err(LuaError::external)?),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.date",
                message: None,
            }),
        }
    }
}
