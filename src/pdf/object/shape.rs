use crate::pdf::{PdfColor, PdfContext, PdfLuaTableExt, PdfPaintMode, PdfPoint, PdfWindingOrder};
use mlua::prelude::*;
use printpdf::Polygon;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectShape {
    pub points: Vec<PdfPoint>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub mode: Option<PdfPaintMode>,
    pub order: Option<PdfWindingOrder>,
}

impl PdfObjectShape {
    /// Draws the object within the PDF.
    pub fn draw(&self, ctx: &PdfContext<'_>) {
        // Get optional values, setting defaults when not specified
        let fill_color = self.fill_color.unwrap_or(ctx.config.page.fill_color);
        let outline_color = self.fill_color.unwrap_or(ctx.config.page.outline_color);

        // Set the color and thickness of our shape
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
        ctx.layer.add_polygon(Polygon {
            rings: vec![self.points.iter().map(|p| ((*p).into(), false)).collect()],
            mode: self.mode.unwrap_or_default().into(),
            winding_order: self.order.unwrap_or_default().into(),
        });
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectShape {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        // Add the points as a list
        for point in self.points {
            table.raw_push(point)?;
        }

        // Add properties as extra named fields
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("mode", self.mode)?;
        table.raw_set("order", self.order)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectShape {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                points: table.clone().sequence_values().collect::<LuaResult<_>>()?,
                fill_color: table.raw_get_ext("fill_color")?,
                outline_color: table.raw_get_ext("outline_color")?,
                mode: table.raw_get_ext("mode")?,
                order: table.raw_get_ext("order")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.rect",
                message: None,
            }),
        }
    }
}
