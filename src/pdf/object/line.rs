mod style;

pub use style::PdfObjectLineStyle;

use crate::pdf::{PdfColor, PdfContext, PdfLuaTableExt, PdfPoint};
use mlua::prelude::*;
use printpdf::{Line, LineCapStyle, LineDashPattern};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectLine {
    pub color: Option<PdfColor>,
    pub points: Vec<PdfPoint>,
    pub thickness: Option<f32>,
    pub style: Option<PdfObjectLineStyle>,
}

impl PdfObjectLine {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfContext<'_>) {
        // Get optional values, setting defaults when not specified
        let color = self.color.unwrap_or(ctx.config.page.fill_color);
        let thickness = self.thickness.unwrap_or(ctx.config.page.outline_thickness);
        let style = self.style.unwrap_or(ctx.config.page.line_style);

        // Set the color and thickness of our line
        ctx.layer.set_fill_color(color.into());
        ctx.layer.set_outline_color(color.into());
        ctx.layer.set_outline_thickness(thickness);

        // If the line should be dashed, set the appropriate styling information,
        // otherwise we reset to a default line dash pattern
        if let PdfObjectLineStyle::Dashed = style {
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
                color: table.raw_get_ext("color")?,
                points: table.raw_get_ext("points")?,
                thickness: table.raw_get_ext("thickness")?,
                style: table.raw_get_ext("style")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.line",
                message: None,
            }),
        }
    }
}
