use crate::pdf::PdfLuaTableExt;
use mlua::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfAlign {
    pub h: PdfHorizontalAlign,
    pub v: PdfVerticalAlign,
}

impl PdfAlign {
    /// Convert to (horizontal, vertical) tuple.
    pub fn to_h_v(self) -> (PdfHorizontalAlign, PdfVerticalAlign) {
        (self.h, self.v)
    }

    /// Convert to (vertical, horizontal) tuple.
    pub fn to_v_h(self) -> (PdfVerticalAlign, PdfHorizontalAlign) {
        (self.v, self.h)
    }
}

impl<'lua> IntoLua<'lua> for PdfAlign {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.raw_set("h", self.h)?;
        table.raw_set("v", self.v)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfAlign {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::Table(tbl) => Ok(Self {
                h: tbl.raw_get_ext::<_, Option<_>>("h")?.unwrap_or_default(),
                v: tbl.raw_get_ext::<_, Option<_>>("v")?.unwrap_or_default(),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.horizontal_align",
                message: None,
            }),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum PdfHorizontalAlign {
    Left,
    #[default]
    Middle,
    Right,
}

impl<'lua> IntoLua<'lua> for PdfHorizontalAlign {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self {
            Self::Left => "left",
            Self::Middle => "middle",
            Self::Right => "right",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfHorizontalAlign {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "left" => Ok(Self::Left),
                "middle" => Ok(Self::Middle),
                "right" => Ok(Self::Right),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.horizontal_align",
                    message: Some(format!("unknown alignment: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.horizontal_align",
                message: None,
            }),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum PdfVerticalAlign {
    Top,
    #[default]
    Middle,
    Bottom,
}

impl<'lua> IntoLua<'lua> for PdfVerticalAlign {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self {
            Self::Top => "top",
            Self::Middle => "middle",
            Self::Bottom => "bottom",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfVerticalAlign {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "top" => Ok(Self::Top),
                "middle" => Ok(Self::Middle),
                "bottom" => Ok(Self::Bottom),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.vertical_align",
                    message: Some(format!("unknown alignment: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.vertical_align",
                message: None,
            }),
        }
    }
}
