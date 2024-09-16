use super::PdfBounds;
use crate::pdf::PdfLuaTableExt;
use mlua::prelude::*;

/// Represents an annotation on a PDF that provides a link.
#[derive(Clone, Debug)]
pub struct PdfLinkAnnotation {
    pub bounds: PdfBounds,
    pub depth: i64,
    pub link: PdfLink,
}

/// Represents an action to take as a link.
#[derive(Clone, Debug)]
pub enum PdfLink {
    /// Link should go to an internal page denoted by the page's id.
    GoTo { page: u32 },

    /// Link should go to an external URI.
    Uri { uri: String },
}

impl PdfLink {
    /// Returns a static str representing the type name of the action.
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::GoTo { .. } => "goto",
            Self::Uri { .. } => "uri",
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfLink {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        // Mark the type of action
        table.raw_set("type", self.type_name())?;

        // Set action-specific fields
        match self {
            Self::GoTo { page } => table.raw_set("page", page)?,
            Self::Uri { uri } => table.raw_set("uri", uri)?,
        }

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfLink {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::Number(num) => Ok(Self::GoTo { page: num as u32 }),
            LuaValue::Integer(num) => Ok(Self::GoTo { page: num as u32 }),
            LuaValue::String(s) => Ok(Self::Uri {
                uri: s.to_str()?.to_string(),
            }),
            LuaValue::Table(tbl) => match tbl.raw_get_ext::<_, String>("type")?.as_str() {
                "goto" => Ok(Self::GoTo {
                    page: tbl.raw_get_ext("page")?,
                }),
                "uri" => Ok(Self::Uri {
                    uri: tbl.raw_get_ext("uri")?,
                }),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.link_action",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.link_action",
                message: None,
            }),
        }
    }
}
