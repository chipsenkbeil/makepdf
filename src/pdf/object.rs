mod group;
mod line;
mod rect;
mod shape;
mod text;

pub use group::PdfObjectGroup;
pub use line::{PdfObjectLine, PdfObjectLineStyle};
pub use rect::PdfObjectRect;
pub use shape::PdfObjectShape;
pub use text::PdfObjectText;

use crate::pdf::{PdfBounds, PdfContext, PdfLinkAnnotation, PdfLuaTableExt};
use mlua::prelude::*;

#[derive(Clone, Debug)]
pub enum PdfObject {
    Group(PdfObjectGroup),
    Line(PdfObjectLine),
    Rect(PdfObjectRect),
    Shape(PdfObjectShape),
    Text(PdfObjectText),
}

impl PdfObject {
    /// Return a static str representing the type of object.
    pub fn to_type_name(&self) -> &'static str {
        match self {
            Self::Group(_) => "group",
            Self::Line(_) => "line",
            Self::Rect(_) => "rect",
            Self::Shape(_) => "shape",
            Self::Text(_) => "text",
        }
    }

    /// Returns bounds for the object, sometimes calculated using `ctx`.
    pub fn bounds(&self, ctx: PdfContext<'_>) -> PdfBounds {
        match self {
            Self::Group(x) => x.bounds(ctx),
            Self::Line(x) => x.bounds(),
            Self::Rect(x) => x.bounds,
            Self::Shape(x) => x.bounds(),
            Self::Text(x) => x.bounds(ctx),
        }
    }

    /// Returns depth of the object with 0 being the default.
    pub fn depth(&self) -> i64 {
        match self {
            Self::Group(x) => Some(x.depth()),
            Self::Line(x) => x.depth,
            Self::Rect(x) => x.depth,
            Self::Shape(x) => x.depth,
            Self::Text(x) => x.depth,
        }
        .unwrap_or_default()
    }

    /// Returns a collection of link annotations.
    pub fn link_annotations(&self, ctx: PdfContext) -> Vec<PdfLinkAnnotation> {
        match self {
            Self::Group(x) => x.link_annotations(ctx),
            Self::Line(x) => x.link_annotations(ctx),
            Self::Rect(x) => x.link_annotations(ctx),
            Self::Shape(x) => x.link_annotations(ctx),
            Self::Text(x) => x.link_annotations(ctx),
        }
    }

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext<'_>) {
        match self {
            Self::Group(x) => x.draw(ctx),
            Self::Line(x) => x.draw(ctx),
            Self::Rect(x) => x.draw(ctx),
            Self::Shape(x) => x.draw(ctx),
            Self::Text(x) => x.draw(ctx),
        }
    }
}

impl From<PdfObjectGroup> for PdfObject {
    fn from(obj: PdfObjectGroup) -> Self {
        Self::Group(obj)
    }
}

impl From<PdfObjectLine> for PdfObject {
    fn from(obj: PdfObjectLine) -> Self {
        Self::Line(obj)
    }
}

impl From<PdfObjectRect> for PdfObject {
    fn from(obj: PdfObjectRect) -> Self {
        Self::Rect(obj)
    }
}

impl From<PdfObjectShape> for PdfObject {
    fn from(obj: PdfObjectShape) -> Self {
        Self::Shape(obj)
    }
}

impl From<PdfObjectText> for PdfObject {
    fn from(obj: PdfObjectText) -> Self {
        Self::Text(obj)
    }
}

impl<'lua> IntoLua<'lua> for PdfObject {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let ty = self.to_type_name();
        let value = match self {
            Self::Group(x) => x.into_lua(lua)?,
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
                "group" => Ok(Self::Group(PdfObjectGroup::from_lua(
                    LuaValue::Table(table),
                    lua,
                )?)),
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
