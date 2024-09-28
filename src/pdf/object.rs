mod circle;
mod group;
mod line;
mod rect;
mod shape;
mod text;
mod r#type;

pub use circle::PdfObjectCircle;
pub use group::PdfObjectGroup;
pub use line::PdfObjectLine;
pub use r#type::PdfObjectType;
pub use rect::PdfObjectRect;
pub use shape::PdfObjectShape;
pub use text::PdfObjectText;

use crate::pdf::{PdfBounds, PdfContext, PdfLinkAnnotation, PdfLuaTableExt};
use mlua::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PdfObject {
    Circle(PdfObjectCircle),
    Group(PdfObjectGroup),
    Line(PdfObjectLine),
    Rect(PdfObjectRect),
    Shape(PdfObjectShape),
    Text(PdfObjectText),
}

impl PdfObject {
    /// Returns the type corresponding to the object.
    pub fn to_type(&self) -> PdfObjectType {
        match self {
            Self::Circle(_) => PdfObjectType::Circle,
            Self::Group(_) => PdfObjectType::Group,
            Self::Line(_) => PdfObjectType::Line,
            Self::Rect(_) => PdfObjectType::Rect,
            Self::Shape(_) => PdfObjectType::Shape,
            Self::Text(_) => PdfObjectType::Text,
        }
    }

    /// Return a static str representing the type of object.
    pub fn to_type_name(&self) -> &'static str {
        self.to_type().to_type_str()
    }

    /// Returns bounds for the object, sometimes calculated using `ctx`.
    pub fn bounds(&self, ctx: PdfContext<'_>) -> PdfBounds {
        match self {
            Self::Circle(x) => x.bounds(),
            Self::Group(x) => x.bounds(ctx),
            Self::Line(x) => x.bounds(),
            Self::Rect(x) => x.bounds,
            Self::Shape(x) => x.bounds(),
            Self::Text(x) => x.bounds(ctx),
        }
    }

    /// Calculates bounds from a [`Lua`] runtime, which occurs earlier than when a [`PdfContext`]
    /// is available.
    pub(crate) fn lua_bounds(&self, lua: &Lua) -> LuaResult<PdfBounds> {
        Ok(match self {
            Self::Circle(x) => x.bounds(),
            Self::Group(x) => x.lua_bounds(lua)?,
            Self::Line(x) => x.bounds(),
            Self::Rect(x) => x.bounds,
            Self::Shape(x) => x.bounds(),
            Self::Text(x) => x.lua_bounds(lua)?,
        })
    }

    /// Returns depth of the object with 0 being the default.
    pub fn depth(&self) -> i64 {
        match self {
            Self::Circle(x) => x.depth,
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
            Self::Circle(x) => x.link_annotations(ctx),
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
            Self::Circle(x) => x.draw(ctx),
            Self::Group(x) => x.draw(ctx),
            Self::Line(x) => x.draw(ctx),
            Self::Rect(x) => x.draw(ctx),
            Self::Shape(x) => x.draw(ctx),
            Self::Text(x) => x.draw(ctx),
        }
    }
}

impl From<PdfObjectCircle> for PdfObject {
    fn from(obj: PdfObjectCircle) -> Self {
        Self::Circle(obj)
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
            Self::Circle(x) => x.into_lua(lua)?,
            Self::Group(x) => x.into_lua(lua)?,
            Self::Line(x) => x.into_lua(lua)?,
            Self::Rect(x) => x.into_lua(lua)?,
            Self::Shape(x) => x.into_lua(lua)?,
            Self::Text(x) => x.into_lua(lua)?,
        };

        match value {
            LuaValue::Table(table) => {
                // Ensure that the type is reflected on the object
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
            LuaValue::Table(table) => {
                match table.raw_get_ext::<_, Option<PdfObjectType>>("type")? {
                    Some(PdfObjectType::Circle) => Ok(Self::Circle(PdfObjectCircle::from_lua(
                        LuaValue::Table(table),
                        lua,
                    )?)),
                    Some(PdfObjectType::Group) => Ok(Self::Group(PdfObjectGroup::from_lua(
                        LuaValue::Table(table),
                        lua,
                    )?)),
                    Some(PdfObjectType::Line) => Ok(Self::Line(PdfObjectLine::from_lua(
                        LuaValue::Table(table),
                        lua,
                    )?)),
                    Some(PdfObjectType::Rect) => Ok(Self::Rect(PdfObjectRect::from_lua(
                        LuaValue::Table(table),
                        lua,
                    )?)),
                    Some(PdfObjectType::Shape) => Ok(Self::Shape(PdfObjectShape::from_lua(
                        LuaValue::Table(table),
                        lua,
                    )?)),
                    Some(PdfObjectType::Text) => Ok(Self::Text(PdfObjectText::from_lua(
                        LuaValue::Table(table),
                        lua,
                    )?)),
                    None => Err(LuaError::FromLuaConversionError {
                        from,
                        to: "pdf.object",
                        message: Some(String::from("unknown type")),
                    }),
                }
            }
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.object",
                message: None,
            }),
        }
    }
}
