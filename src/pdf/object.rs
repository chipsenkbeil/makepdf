mod bounds;
mod line;
mod rect;
mod shape;
mod space;
mod text;

pub use bounds::BoundsPdfObject;
pub use line::LinePdfObject;
pub use rect::RectPdfObject;
pub use shape::ShapePdfObject;
pub use space::{Margin, Padding, SpacePdfObject};
pub use text::TextPdfObject;

use mlua::prelude::*;
use owned_ttf_parser::Face;
use printpdf::{IndirectFontRef, PdfLayerReference};

/// Context provided to a [`PdfObject`] in order to draw it.
#[derive(Copy, Clone, Debug)]
pub struct PdfObjectContext<'a> {
    pub face: &'a Face<'a>,
    pub font: &'a IndirectFontRef,
    pub layer: &'a PdfLayerReference,
}

#[derive(Clone, Debug)]
pub enum PdfObject {
    Line(LinePdfObject),
    Rect(RectPdfObject),
    Shape(ShapePdfObject),
    Text(TextPdfObject),
}

impl PdfObject {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        match self {
            Self::Line(x) => x.draw(ctx),
            Self::Rect(_) => {}
            Self::Shape(_) => {}
            Self::Text(_) => {}
        }
    }
}

impl<'lua> IntoLua<'lua> for PdfObject {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        match self {
            Self::Line(_) => {}
            Self::Rect(_) => {}
            Self::Shape(_) => {}
            Self::Text(_) => {}
        }

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObject {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::Table(table) => match table.raw_get::<_, String>("type")?.as_str() {
                "line" => Ok(Self::Line(LinePdfObject::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                "rect" => Ok(Self::Rect(RectPdfObject::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                "shape" => Ok(Self::Shape(ShapePdfObject::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
                "text" => Ok(Self::Text(TextPdfObject::from_lua(
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
