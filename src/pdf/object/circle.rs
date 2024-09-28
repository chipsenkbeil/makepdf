use crate::pdf::*;
use mlua::prelude::*;
use printpdf::utils::calculate_points_for_circle as printpdf_calculate_points_for_circle;
use printpdf::{Mm, Polygon};

/// Represents a circle to be drawn in the PDF.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfObjectCircle {
    pub center: PdfPoint,
    pub radius: Mm,
    pub depth: Option<i64>,
    pub fill_color: Option<PdfColor>,
    pub outline_color: Option<PdfColor>,
    pub outline_thickness: Option<f32>,
    pub mode: Option<PdfPaintMode>,
    pub order: Option<PdfWindingOrder>,
    pub dash_pattern: Option<PdfLineDashPattern>,
    pub cap_style: Option<PdfLineCapStyle>,
    pub join_style: Option<PdfLineJoinStyle>,
    pub link: Option<PdfLink>,
}

impl PdfObjectCircle {
    /// Calculates the points for a circle based on its center and radius, returning an iterator
    /// over the points.
    pub fn iter_points(&self) -> impl Iterator<Item = PdfPoint> {
        printpdf_calculate_points_for_circle(self.radius, self.center.x, self.center.y)
            .into_iter()
            .map(|(p, _)| PdfPoint::from(p))
    }

    /// Returns bounds for the circle by calculating the points for the circle dynamically.
    pub fn bounds(&self) -> PdfBounds {
        // Get an iterator over all of the points
        let mut it = self.iter_points();

        // Set default for lower-left and upper-right to our first point, or
        // default if we have no points. We do this to avoid the issue where
        // the default point is 0,0 and all existing points are positive, the
        // bounds would have a lower-left of 0,0.
        let mut ll = it.next().unwrap_or_default();
        let mut ur = ll;

        // Loop through remaining points to calculate the actual bounds
        for point in it {
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

    /// Aligns the circle to a set of bounds.
    pub fn align_to(&mut self, bounds: PdfBounds, align: (PdfVerticalAlign, PdfHorizontalAlign)) {
        // Get new bounds for the circle
        let src_bounds = self.bounds();
        let dst_bounds = src_bounds.align_to(bounds, align);

        // Figure out the shift from original to new bounds
        let x_offset = dst_bounds.ll.x - src_bounds.ll.x;
        let y_offset = dst_bounds.ll.y - src_bounds.ll.y;

        // Apply the shift to the center of the circle
        self.center.x += x_offset;
        self.center.y += y_offset;
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
        let outline_thickness = self
            .outline_thickness
            .unwrap_or(ctx.config.page.outline_thickness);
        let line_cap_style = self.cap_style.unwrap_or(ctx.config.page.line_cap_style);
        let line_join_style = self.join_style.unwrap_or(ctx.config.page.line_join_style);
        let line_dash_pattern = self
            .dash_pattern
            .unwrap_or(ctx.config.page.line_dash_pattern);

        // Set layer configurations before adding the circle
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
        ctx.layer.set_outline_thickness(outline_thickness);
        ctx.layer.set_line_cap_style(line_cap_style.into());
        ctx.layer.set_line_join_style(line_join_style.into());
        ctx.layer.set_line_dash_pattern(line_dash_pattern.into());

        ctx.layer.add_polygon(Polygon {
            rings: vec![printpdf_calculate_points_for_circle(
                self.radius,
                self.center.x,
                self.center.y,
            )],
            mode: self.mode.unwrap_or_default().into(),
            winding_order: self.order.unwrap_or_default().into(),
        });
    }
}

impl<'lua> IntoLua<'lua> for PdfObjectCircle {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        // Add properties as extra named fields
        table.raw_set("type", PdfObjectType::Circle)?;
        table.raw_set("center", self.center)?;
        table.raw_set("radius", self.radius.0)?;
        table.raw_set("depth", self.depth)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("outline_thickness", self.outline_thickness)?;
        table.raw_set("mode", self.mode)?;
        table.raw_set("order", self.order)?;
        table.raw_set("dash_pattern", self.dash_pattern)?;
        table.raw_set("cap_style", self.cap_style)?;
        table.raw_set("join_style", self.join_style)?;
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

impl<'lua> FromLua<'lua> for PdfObjectCircle {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                center: table
                    .raw_get_ext::<_, Option<_>>("center")?
                    .unwrap_or_default(),
                radius: Mm(table
                    .raw_get_ext::<_, Option<_>>("radius")?
                    .unwrap_or_default()),
                depth: table.raw_get_ext("depth")?,
                fill_color: table.raw_get_ext("fill_color")?,
                outline_color: table.raw_get_ext("outline_color")?,
                outline_thickness: table.raw_get_ext("outline_thickness")?,
                mode: table.raw_get_ext("mode")?,
                order: table.raw_get_ext("order")?,
                dash_pattern: table.raw_get_ext("dash_pattern")?,
                cap_style: table.raw_get_ext("cap_style")?,
                join_style: table.raw_get_ext("join_style")?,
                link: table.raw_get_ext("link")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.object.circle",
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
    fn should_be_able_to_align_circle_to_some_bounds_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        // Test the bounds, which should correctly cover full circle
        lua.load(chunk! {
            // Create an initial circle at some position
            local circle = pdf.object.circle({
                center = { x = 3, y = 5 },
                radius = 3,
            })

            // Assert the circle is where we expect prior to alignment
            pdf.utils.assert_deep_equal(circle:bounds():with_precision(0), {
                ll = { x = 0, y = 2 },
                ur = { x = 6, y = 8 },
            })

            // Do the alignment with some bounds that are elsewhere
            circle = circle:align_to({
                ll = { x = 5,  y = 5 },
                ur = { x = 10, y = 10 },
            }, { v = "bottom", h = "left" })

            // Assert the circle has moved into place
            pdf.utils.assert_deep_equal(circle:bounds():with_precision(0), {
                ll = { x = 5, y = 5 },
                ur = { x = 11, y = 11 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_circle() {
        // Center at 0,0 with radius of 0
        let circle = PdfObjectCircle::default();
        assert_eq!(
            circle.bounds(),
            PdfBounds::from_coords_f32(0.0, 0.0, 0.0, 0.0)
        );

        // Center at 3,4 with radius of 1
        let circle = PdfObjectCircle {
            center: PdfPoint::from_coords_f32(3.0, 4.0),
            radius: Mm(1.0),
            ..Default::default()
        };
        assert_eq!(
            circle.bounds(),
            PdfBounds::from_coords_f32(2.0, 3.0, 4.0, 5.0)
        );

        // Center at 3,4 with radius > 1
        let circle = PdfObjectCircle {
            center: PdfPoint::from_coords_f32(3.0, 4.0),
            radius: Mm(5.0),
            ..Default::default()
        };
        assert_eq!(
            circle.bounds(),
            PdfBounds::from_coords_f32(-2.0, -1.0, 8.0, 9.0)
        );
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_circle_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        lua.load(chunk! {
            // Nothing specified will set center to 0,0 and radius to 0
            local circle = pdf.object.circle({})
            pdf.utils.assert_deep_equal(circle:bounds(), {
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })

            // Center at 3,4 with radius of 1
            local circle = pdf.object.circle({
                center = { x = 3, y = 4 },
                radius = 1,
            })
            pdf.utils.assert_deep_equal(circle:bounds():with_precision(0), {
                ll = { x = 2, y = 3 },
                ur = { x = 4, y = 5 },
            })

            // Center at 3,4 with radius > 1
            local circle = pdf.object.circle({
                center = { x = 3, y = 4 },
                radius = 5,
            })
            pdf.utils.assert_deep_equal(circle:bounds():with_precision(0), {
                ll = { x = -2, y = -1 },
                ur = { x = 8, y = 9 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        // Can convert from empty table into a circle
        assert_eq!(
            Lua::new()
                .load(chunk!({}))
                .eval::<PdfObjectCircle>()
                .unwrap(),
            PdfObjectCircle::default(),
        );

        // Can convert from an table with no points (but everything else) into a circle
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    center = { x = 1, y = 2 },
                    radius = 3,
                    depth = 123,
                    fill_color = "123456",
                    outline_color = "789abc",
                    outline_thickness = 456,
                    mode = "stroke",
                    order = "non_zero",
                    dash_pattern = "dashed:999",
                    cap_style = "butt",
                    join_style = "miter",
                    link = {
                        type = "uri",
                        uri = "https://example.com",
                    },
                }))
                .eval::<PdfObjectCircle>()
                .unwrap(),
            PdfObjectCircle {
                center: PdfPoint::from_coords_f32(1.0, 2.0),
                radius: Mm(3.0),
                depth: Some(123),
                fill_color: Some("#123456".parse().unwrap()),
                outline_color: Some("#789ABC".parse().unwrap()),
                outline_thickness: Some(456.0),
                mode: Some(PdfPaintMode::stroke()),
                order: Some(PdfWindingOrder::non_zero()),
                dash_pattern: Some(PdfLineDashPattern::dashed(999)),
                cap_style: Some(PdfLineCapStyle::butt()),
                join_style: Some(PdfLineJoinStyle::miter()),
                link: Some(PdfLink::Uri {
                    uri: String::from("https://example.com"),
                }),
            },
        );

        // Can convert from a table with only center & radius into a circle
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    center = { x = 1, y = 2 },
                    radius = 3,
                }))
                .eval::<PdfObjectCircle>()
                .unwrap(),
            PdfObjectCircle {
                center: PdfPoint::from_coords_f32(1.0, 2.0),
                radius: Mm(3.0),
                ..Default::default()
            },
        );

        // Can convert from a table with center, radius, and properties into a circle
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    center = { x = 1, y = 2 },
                    radius = 3,
                    depth = 123,
                    fill_color = "123456",
                    outline_color = "789ABC",
                    outline_thickness = 456,
                    mode = "stroke",
                    order = "non_zero",
                    dash_pattern = "dashed:999",
                    cap_style = "butt",
                    join_style = "miter",
                    link = {
                        type = "uri",
                        uri = "https://example.com",
                    },
                }))
                .eval::<PdfObjectCircle>()
                .unwrap(),
            PdfObjectCircle {
                center: PdfPoint::from_coords_f32(1.0, 2.0),
                radius: Mm(3.0),
                depth: Some(123),
                fill_color: Some("#123456".parse().unwrap()),
                outline_color: Some("#789ABC".parse().unwrap()),
                outline_thickness: Some(456.0),
                mode: Some(PdfPaintMode::stroke()),
                order: Some(PdfWindingOrder::non_zero()),
                dash_pattern: Some(PdfLineDashPattern::dashed(999)),
                cap_style: Some(PdfLineCapStyle::butt()),
                join_style: Some(PdfLineJoinStyle::miter()),
                link: Some(PdfLink::Uri {
                    uri: String::from("https://example.com"),
                }),
            },
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        // Test circle with nothing
        let circle = PdfObjectCircle::default();

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($circle, {
                type = "circle",
                center = { x = 0, y = 0 },
                radius = 0,
            })
        })
        .exec()
        .expect("Assertion failed");

        // Test circle with everything
        let circle = PdfObjectCircle {
            center: PdfPoint::from_coords_f32(1.0, 2.0),
            radius: Mm(3.0),
            depth: Some(123),
            fill_color: Some("#123456".parse().unwrap()),
            outline_color: Some("#789ABC".parse().unwrap()),
            outline_thickness: Some(456.0),
            mode: Some(PdfPaintMode::stroke()),
            order: Some(PdfWindingOrder::non_zero()),
            dash_pattern: Some(PdfLineDashPattern::dashed(999)),
            cap_style: Some(PdfLineCapStyle::butt()),
            join_style: Some(PdfLineJoinStyle::miter()),
            link: Some(PdfLink::Uri {
                uri: String::from("https://example.com"),
            }),
        };

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($circle, {
                type = "circle",
                center = { x = 1, y = 2 },
                radius = 3,
                depth = 123,
                fill_color = { red = 18, green = 52, blue = 86 },
                outline_color = { red = 120, green = 154, blue = 188 },
                outline_thickness = 456,
                mode = "stroke",
                order = "non_zero",
                dash_pattern = { offset = 0, dash_1 = 999 },
                cap_style = "butt",
                join_style = "miter",
                link = {
                    type = "uri",
                    uri = "https://example.com",
                },
            })
        })
        .exec()
        .expect("Assertion failed");
    }
}
