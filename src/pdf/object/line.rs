use crate::pdf::{PdfColor, PdfObjectContext, PdfPoint};
use mlua::prelude::*;
use printpdf::{Line, LineCapStyle, LineDashPattern};

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

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectLine {
    pub color: PdfColor,
    pub points: Vec<PdfPoint>,
    pub thickness: f32,
    pub style: PdfObjectLineStyle,
}

impl PdfObjectLine {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfObjectContext<'_>) {
        ctx.layer.set_fill_color(self.color.into());
        ctx.layer.set_outline_color(self.color.into());
        ctx.layer.set_outline_thickness(self.thickness);

        if let PdfObjectLineStyle::Dashed = self.style {
            ctx.layer.set_line_cap_style(LineCapStyle::Round);
            ctx.layer.set_line_dash_pattern(LineDashPattern {
                dash_1: Some(5),
                ..Default::default()
            });
        } else {
            ctx.layer.set_line_dash_pattern(LineDashPattern::default());
        }

        ctx.layer.add_line(Line {
            points: self.points.iter().map(|p| ((*p).into(), false)).collect(),
            is_closed: false,
        });
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectLine {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("color", self.color)?;
        table.raw_set("points", self.points)?;
        table.raw_set("thickness", self.thickness)?;
        table.raw_set("style", self.style)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectLine {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                color: raw_get!(table, "color")?,
                points: raw_get!(table, "points")?,
                thickness: raw_get!(table, "thickness")?,
                style: raw_get!(table, "style")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.line",
                message: None,
            }),
        }
    }
}
