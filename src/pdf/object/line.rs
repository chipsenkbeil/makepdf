mod style;

pub use style::PdfObjectLineStyle;

use crate::pdf::{
    PdfBounds, PdfColor, PdfContext, PdfLink, PdfLinkAnnotation, PdfLuaExt, PdfLuaTableExt,
    PdfPoint,
};
use mlua::prelude::*;
use printpdf::{Line, LineCapStyle, LineDashPattern};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectLine {
    pub points: Vec<PdfPoint>,
    pub depth: Option<i64>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub thickness: Option<f32>,
    pub style: Option<PdfObjectLineStyle>,
    pub link: Option<PdfLink>,
}

impl PdfObjectLine {
    /// Returns bounds for the line(s) by getting the lower and upper point ranges.
    pub fn bounds(&self) -> PdfBounds {
        let mut ll = PdfPoint::default();
        let mut ur = PdfPoint::default();

        for point in self.points.iter() {
            if point.x < ll.x {
                ll.x = point.x;
            }

            if point.x > ur.x {
                ur.x = point.x;
            }

            if point.y < ll.y {
                ll.y = point.x;
            }

            if point.y > ur.y {
                ur.y = point.y;
            }
        }

        PdfBounds::new(ll, ur)
    }

    /// Returns a collection of link annotations.
    pub fn link_annotations(&self, _ctx: PdfContext) -> Vec<PdfLinkAnnotation> {
        match self.link.clone() {
            Some(link) => vec![PdfLinkAnnotation {
                bounds: self.bounds(),
                depth: self.depth.unwrap_or_default(),
                link,
            }],
            None => Vec::new(),
        }
    }

    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: PdfContext<'_>) {
        // Get optional values, setting defaults when not specified
        let fill_color = self.fill_color.unwrap_or(ctx.config.page.fill_color);
        let outline_color = self.fill_color.unwrap_or(ctx.config.page.outline_color);
        let thickness = self.thickness.unwrap_or(ctx.config.page.outline_thickness);
        let style = self.style.unwrap_or(ctx.config.page.line_style);

        // Set the color and thickness of our line
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
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
        let (table, metatable) = lua.create_table_ext()?;

        // Add the points as a list
        for point in self.points {
            table.raw_push(point)?;
        }

        // Add properties as extra named fields
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("thickness", self.thickness)?;
        table.raw_set("style", self.style)?;
        table.raw_set("link", self.link)?;

        metatable.raw_set(
            "bounds",
            lua.create_function(move |_, this: Self| Ok(this.bounds()))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectLine {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                points: table.clone().sequence_values().collect::<LuaResult<_>>()?,
                depth: table.raw_get_ext("depth")?,
                fill_color: table.raw_get_ext("fill_color")?,
                outline_color: table.raw_get_ext("outline_color")?,
                thickness: table.raw_get_ext("thickness")?,
                style: table.raw_get_ext("style")?,
                link: table.raw_get_ext("link")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.line",
                message: None,
            }),
        }
    }
}
