use mlua::prelude::*;

/// Type associated with a PDF object.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PdfObjectType {
    Circle,
    Group,
    Line,
    Rect,
    Shape,
    Text,
}

impl PdfObjectType {
    /// Return a static str representing the type of object.
    pub fn to_type_str(&self) -> &'static str {
        match self {
            Self::Circle => "circle",
            Self::Group => "group",
            Self::Line => "line",
            Self::Rect => "rect",
            Self::Shape => "shape",
            Self::Text => "text",
        }
    }

    /// Create type from string, returning `None` if not a valid type.
    pub fn from_type_str(s: &str) -> Option<Self> {
        match s {
            "circle" => Some(Self::Circle),
            "group" => Some(Self::Group),
            "line" => Some(Self::Line),
            "rect" => Some(Self::Rect),
            "shape" => Some(Self::Shape),
            "text" => Some(Self::Text),
            _ => None,
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectType {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(self.to_type_str()).map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfObjectType {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match Self::from_type_str(s.to_string_lossy().as_ref()) {
                Some(ty) => Ok(ty),
                None => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.object.type",
                    message: Some(format!("unknown alignment: {}", s.to_string_lossy())),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.object.type",
                message: None,
            }),
        }
    }
}
