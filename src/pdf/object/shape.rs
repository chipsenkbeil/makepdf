use crate::pdf::*;
use mlua::prelude::*;
use printpdf::Polygon;

/// Represents a polygonal shape to be drawn in the PDF.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfObjectShape {
    pub points: Vec<PdfPoint>,
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

impl PdfObjectShape {
    /// Returns bounds for the shape by getting the lower and upper point ranges.
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

    /// Aligns the shape to a set of bounds.
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

        // Set layer configurations before adding the shape
        ctx.layer.set_fill_color(fill_color.into());
        ctx.layer.set_outline_color(outline_color.into());
        ctx.layer.set_outline_thickness(outline_thickness);
        ctx.layer.set_line_cap_style(line_cap_style.into());
        ctx.layer.set_line_join_style(line_join_style.into());
        ctx.layer.set_line_dash_pattern(line_dash_pattern.into());

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
        let (table, metatable) = lua.create_table_ext()?;

        // Add the points as a list
        for point in self.points {
            table.raw_push(point)?;
        }

        // Add properties as extra named fields
        table.raw_set("type", PdfObjectType::Shape)?;
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

impl<'lua> FromLua<'lua> for PdfObjectShape {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                points: table.clone().sequence_values().collect::<LuaResult<_>>()?,
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
                to: "pdf.object.shape",
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
    fn should_be_able_to_align_shape_to_some_bounds_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        // Test the bounds, which should correctly cover full shape
        lua.load(chunk! {
            // Create an initial shape at some position
            local shape = pdf.object.shape({
                { x = 1, y = 5 },
                { x = 3, y = 4 },
            })

            // Assert the shape is where we expect prior to alignment
            pdf.utils.assert_deep_equal(shape:bounds(), {
                ll = { x = 1, y = 4 },
                ur = { x = 3, y = 5 },
            })

            // Do the alignment with some bounds that are elsewhere
            shape = shape:align_to({
                ll = { x = 5,  y = 5 },
                ur = { x = 10, y = 10 },
            }, { v = "bottom", h = "left" })

            // Assert the shape has moved into place
            pdf.utils.assert_deep_equal(shape:bounds(), {
                ll = { x = 5, y = 5 },
                ur = { x = 7, y = 6 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_shape() {
        // No points
        let shape = PdfObjectShape::default();
        assert_eq!(
            shape.bounds(),
            PdfBounds::from_coords_f32(0.0, 0.0, 0.0, 0.0)
        );

        // Single point
        let shape = PdfObjectShape {
            points: vec![PdfPoint::from_coords_f32(3.0, 4.0)],
            ..Default::default()
        };
        assert_eq!(
            shape.bounds(),
            PdfBounds::from_coords_f32(3.0, 4.0, 3.0, 4.0)
        );

        // Multiple points
        let shape = PdfObjectShape {
            points: vec![
                PdfPoint::from_coords_f32(1.0, 5.0),
                PdfPoint::from_coords_f32(3.0, 4.0),
            ],
            ..Default::default()
        };
        assert_eq!(
            shape.bounds(),
            PdfBounds::from_coords_f32(1.0, 4.0, 3.0, 5.0)
        );
    }

    #[test]
    fn should_be_able_to_calculate_bounds_of_shape_in_lua() {
        // Stand up Lua runtime with everything configured properly for tests
        let lua = Lua::new();
        lua.globals().raw_set("pdf", Pdf::default()).unwrap();

        lua.load(chunk! {
            // No points
            local shape = pdf.object.shape({})
            pdf.utils.assert_deep_equal(shape:bounds(), {
                ll = { x = 0, y = 0 },
                ur = { x = 0, y = 0 },
            })

            // Single point
            local shape = pdf.object.shape({
                { x = 3, y = 4 },
            })
            pdf.utils.assert_deep_equal(shape:bounds(), {
                ll = { x = 3, y = 4 },
                ur = { x = 3, y = 4 },
            })

            // Multiple points
            local shape = pdf.object.shape({
                { x = 1, y = 5 },
                { x = 3, y = 4 },
            })
            pdf.utils.assert_deep_equal(shape:bounds(), {
                ll = { x = 1, y = 4 },
                ur = { x = 3, y = 5 },
            })
        })
        .exec()
        .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        // Can convert from empty table into a shape
        assert_eq!(
            Lua::new()
                .load(chunk!({}))
                .eval::<PdfObjectShape>()
                .unwrap(),
            PdfObjectShape::default(),
        );

        // Can convert from an table with no points (but everything else) into a shape
        assert_eq!(
            Lua::new()
                .load(chunk!({
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
                .eval::<PdfObjectShape>()
                .unwrap(),
            PdfObjectShape {
                points: Vec::new(),
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

        // Can convert from a table with only points into a shape
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    { x = 1, y = 2 },
                    { x = 3, y = 4 },
                }))
                .eval::<PdfObjectShape>()
                .unwrap(),
            PdfObjectShape {
                points: vec![
                    PdfPoint::from_coords_f32(1.0, 2.0),
                    PdfPoint::from_coords_f32(3.0, 4.0),
                ],
                ..Default::default()
            },
        );

        // Can convert from a table with points and properties into a shape
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    { x = 1, y = 2 },
                    { x = 3, y = 4 },
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
                .eval::<PdfObjectShape>()
                .unwrap(),
            PdfObjectShape {
                points: vec![
                    PdfPoint::from_coords_f32(1.0, 2.0),
                    PdfPoint::from_coords_f32(3.0, 4.0),
                ],
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

        // Test shape with nothing
        let shape = PdfObjectShape::default();

        lua.load(chunk! {
            pdf.utils.assert_deep_equal($shape, {
                type = "shape",
            })
        })
        .exec()
        .expect("Assertion failed");

        // Test shape with everything
        let shape = PdfObjectShape {
            points: vec![
                PdfPoint::from_coords_f32(1.0, 2.0),
                PdfPoint::from_coords_f32(3.0, 4.0),
            ],
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
            pdf.utils.assert_deep_equal($shape, {
                { x = 1, y = 2 },
                { x = 3, y = 4 },
                type = "shape",
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
