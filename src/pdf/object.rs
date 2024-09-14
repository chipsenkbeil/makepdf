mod line;
mod rect;
mod shape;
mod text;

pub use line::{PdfObjectLine, PdfObjectLineStyle};
pub use rect::PdfObjectRect;
pub use shape::PdfObjectShape;
pub use text::PdfObjectText;

use crate::pdf::{PdfContext, PdfLuaTableExt};
use mlua::prelude::*;

#[derive(Clone, Debug)]
pub enum PdfObject {
    Line(PdfObjectLine),
    Rect(PdfObjectRect),
    Shape(PdfObjectShape),
    Text(PdfObjectText),
}

impl PdfObject {
    /// Return a static str representing the type of object.
    pub fn to_type_name(&self) -> &'static str {
        match self {
            Self::Line(_) => "line",
            Self::Rect(_) => "rect",
            Self::Shape(_) => "shape",
            Self::Text(_) => "text",
        }
    }

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfContext<'_>) {
        match self {
            Self::Line(x) => x.draw(ctx),
            Self::Rect(x) => x.draw(ctx),
            Self::Shape(x) => x.draw(ctx),
            Self::Text(x) => x.draw(ctx),
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfObject {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let ty = self.to_type_name();
        let value = match self {
            Self::Line(x) => x.into_lua(lua)?,
            Self::Rect(x) => x.into_lua(lua)?,
            Self::Shape(x) => x.into_lua(lua)?,
            Self::Text(x) => x.into_lua(lua)?,
        };

        match value {
            LuaValue::Table(table) => {
                // Inject a type name to mark the type of object
                table.raw_set("type", ty)?;

                Ok(LuaValue::Table(table))
            }
            _ => Err(LuaError::ToLuaConversionError {
                from: "pdf.object",
                to: value.type_name(),
                message: None,
            }),
        }
    }
}

impl<'lua> FromLua<'lua> for PdfObject {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::Table(table) => match table.raw_get_ext::<_, String>("type")?.as_str() {
                "line" => Ok(Self::Line(PdfObjectLine::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                "rect" => Ok(Self::Rect(PdfObjectRect::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                "shape" => Ok(Self::Shape(PdfObjectShape::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                "text" => Ok(Self::Text(PdfObjectText::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.object",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.object",
                message: None,
            }),
        }
    }
}
