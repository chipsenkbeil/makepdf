use mlua::prelude::*;

/// Style to use with the line.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PdfObjectLineStyle {
    Solid,
    Dashed,
}

impl<'lua> IntoLua<'lua> for PdfObjectLineStyle {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfObjectLineStyle {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "solid" => Ok(Self::Solid),
                "dashed" => Ok(Self::Dashed),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.object.line.style",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.object.line.style",
                message: None,
            }),
        }
    }
}
