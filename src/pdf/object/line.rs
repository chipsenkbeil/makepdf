mod style;

pub use style::PdfObjectLineStyle;

use crate::pdf::{
    PdfAlign, PdfBounds, PdfColor, PdfContext, PdfHorizontalAlign, PdfLink, PdfLinkAnnotation,
    PdfLuaExt, PdfLuaTableExt, PdfObjectType, PdfPoint, PdfVerticalAlign,
};
use mlua::prelude::*;
use printpdf::{Line, LineCapStyle, LineDashPattern};

/// Represents a line to be drawn in the PDF.
#[derive(Clone, Debug, Default)]
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
        // Set default for lower-left and upper-right to our first point, or
        // default if we have no points. We do this to avoid the issue where
        // the default point is 0,0 and all existing points are positive, the
        // bounds would have a lower-left of 0,0.
        let mut ll = self
            .points
            .first()
            .copied()
            .unwrap_or_else(PdfPoint::default);
        let mut ur = self
            .points
            .first()
            .copied()
            .unwrap_or_else(PdfPoint::default);

        for point in self.points.iter() {
            if point.x < ll.x {
                ll.x = point.x;
            }

            if point.x > ur.x {
                ur.x = point.x;
            }

            if point.y < ll.y {
                ll.y = point.y;
            }

            if point.y > ur.y {
                ur.y = point.y;
            }
        }

        PdfBounds::new(ll, ur)
    }

    /// Aligns the line to a set of bounds.
    pub fn align_to(&mut self, bounds: PdfBounds, align: (PdfVerticalAlign, PdfHorizontalAlign)) {
        // Get new bounds for series of points
        let src_bounds = self.bounds();
        let dst_bounds = src_bounds.align_to(bounds, align);

        // Figure out the shift from original to new bounds
        let x_offset = dst_bounds.ll.x - src_bounds.ll.x;
        let y_offset = dst_bounds.ll.y - src_bounds.ll.y;

        // Apply the changes to all of the points
        for point in self.points.iter_mut() {
            point.x += x_offset;
            point.y += y_offset;
        }
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
        table.raw_set("type", PdfObjectType::Line)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("thickness", self.thickness)?;
        table.raw_set("style", self.style)?;
        table.raw_set("link", self.link)?;

        metatable.raw_set(
            "align_to",
            lua.create_function(
                move |_, (mut this, bounds, align): (Self, PdfBounds, PdfAlign)| {
                    this.align_to(bounds, align.to_v_h());
                    Ok(this)
                },
            )?,
        )?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::Pdf;
    use mlua::chunk;

    #[test]
    fn should_be_able_to_align_line_to_some_bounds_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        // Test the bounds, which should correctly cover full shape
        lua.load(chunk! {
            // Create an initial shape at some position
            local line = pdf.object.line({
                { x = 1, y = 5 },
                { x = 3, y = 4 },
            })

            // Assert the line is where we expect prior to alignment
            pdf.utils.assert_deep_equal(line:bounds(), {
                ll = { x = 1, y = 4 },
                ur = { x = 3, y = 5 },
            })

            // Do the alignment with some bounds that are elsewhere
            line = line:align_to({
                ll = { x = 5,  y = 5 },
                ur = { x = 10, y = 10 },
            }, { v = "bottom", h = "left" })

            // Assert the line has moved into place
            pdf.utils.assert_deep_equal(line:bounds(), {
                ll = { x = 5, y = 5 },
                ur = { x = 7, y = 6 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_line() {
        // No points
        let line = PdfObjectLine::default();
        assert_eq!(
            line.bounds(),
            PdfBounds::from_coords_f32(0.0, 0.0, 0.0, 0.0)
        );

        // Single point
        let line = PdfObjectLine {
            points: vec![PdfPoint::from_coords_f32(3.0, 4.0)],
            ..Default::default()
        };
        assert_eq!(
            line.bounds(),
            PdfBounds::from_coords_f32(3.0, 4.0, 3.0, 4.0)
        );

        // Multiple points
        let line = PdfObjectLine {
            points: vec![
                PdfPoint::from_coords_f32(1.0, 5.0),
                PdfPoint::from_coords_f32(3.0, 4.0),
            ],
            ..Default::default()
        };
        assert_eq!(
            line.bounds(),
            PdfBounds::from_coords_f32(1.0, 4.0, 3.0, 5.0)
        );
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_line_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        lua.load(chunk! {
            // No points
            local line = pdf.object.line({})
            pdf.utils.assert_deep_equal(line:bounds(), {
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })

            // Single point
            local line = pdf.object.line({
                { x = 3, y = 4 },
            })
            pdf.utils.assert_deep_equal(line:bounds(), {
                ll = { x = 3, y = 4 },
                ur = { x = 3, y = 4 },
            })

            // Multiple points
            local line = pdf.object.line({
                { x = 1, y = 5 },
                { x = 3, y = 4 },
            })
            pdf.utils.assert_deep_equal(line:bounds(), {
                ll = { x = 1, y = 4 },
                ur = { x = 3, y = 5 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }
}
