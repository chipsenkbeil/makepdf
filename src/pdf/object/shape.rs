use crate::pdf::{
    PdfBounds, PdfColor, PdfContext, PdfLink, PdfLinkAnnotation, PdfLuaTableExt, PdfPaintMode,
    PdfPoint, PdfWindingOrder,
};
use mlua::prelude::*;
use printpdf::Polygon;

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug)]
pub struct PdfObjectShape {
    pub points: Vec<PdfPoint>,
    pub depth: Option<i64>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub mode: Option<PdfPaintMode>,
    pub order: Option<PdfWindingOrder>,
    pub link: Option<PdfLink>,
}

impl PdfObjectShape {
    /// Returns bounds for the shape by getting the lower and upper point ranges.
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
    pub fn draw(&self, ctx: PdfContext) {
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
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("mode", self.mode)?;
        table.raw_set("order", self.order)?;
        table.raw_set("link", self.link)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfObjectShape {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                points: table.clone().sequence_values().collect::<LuaResult<_>>()?,
                depth: table.raw_get_ext("depth")?,
                fill_color: table.raw_get_ext("fill_color")?,
                outline_color: table.raw_get_ext("outline_color")?,
                mode: table.raw_get_ext("mode")?,
                order: table.raw_get_ext("order")?,
                link: table.raw_get_ext("link")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.shape",
                message: None,
            }),
        }
    }
}
