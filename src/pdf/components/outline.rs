use crate::constants::*;
use crate::{Bounds, Component, Context, Margin, Rect};
use printpdf::{Color, Line, LineCapStyle, LineDashPattern, Point};

#[derive(Clone, Debug)]
pub struct OutlineComponent {
    color: Color,
    margin: Option<Margin>,
    bounds: Rect,
    thickness: f32,
    solid: bool,
}

impl Default for OutlineComponent {
    fn default() -> Self {
        Self {
            color: BANNER_BACKGROUND_COLOR,
            margin: None,
            bounds: Rect::default(),
            thickness: 3.0,
            solid: true,
        }
    }
}

impl OutlineComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn with_margin(&mut self, margin: impl Into<Margin>) -> &mut Self {
        self.margin = Some(margin.into());
        self
    }

    pub fn with_no_margin(&mut self) -> &mut Self {
        self.margin = None;
        self
    }

    pub fn with_thickness(&mut self, thickness: f32) -> &mut Self {
        self.thickness = thickness;
        self
    }

    pub fn with_solid_line(&mut self) -> &mut Self {
        self.solid = true;
        self
    }

    pub fn with_dashed_line(&mut self) -> &mut Self {
        self.solid = false;
        self
    }
}

impl Bounds for OutlineComponent {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.bounds = rect;
    }
}

impl Component for OutlineComponent {
    fn draw(&self, ctx: &Context<'_>) {
        ctx.layer.set_fill_color(self.color.clone());
        ctx.layer.set_outline_color(self.color.clone());
        ctx.layer.set_outline_thickness(self.thickness);

        if !self.solid {
            ctx.layer.set_line_cap_style(LineCapStyle::Round);
            ctx.layer.set_line_dash_pattern(LineDashPattern {
                dash_1: Some(5),
                ..Default::default()
            });
        } else {
            ctx.layer.set_line_cap_style(LineCapStyle::Round);
            ctx.layer.set_line_dash_pattern(LineDashPattern::default());
        }

        let bounds = self.outer_bounds();
        ctx.layer.add_line(Line {
            points: vec![
                (Point::new(bounds.x, bounds.y), false),
                (Point::new(bounds.urx(), bounds.y), false),
            ],
            is_closed: false,
        });
        ctx.layer.add_line(Line {
            points: vec![
                (Point::new(bounds.x, bounds.ury()), false),
                (Point::new(bounds.urx(), bounds.ury()), false),
            ],
            is_closed: false,
        });
        ctx.layer.add_line(Line {
            points: vec![
                (Point::new(bounds.x, bounds.y), false),
                (Point::new(bounds.x, bounds.ury()), false),
            ],
            is_closed: false,
        });
        ctx.layer.add_line(Line {
            points: vec![
                (Point::new(bounds.urx(), bounds.y), false),
                (Point::new(bounds.urx(), bounds.ury()), false),
            ],
            is_closed: false,
        });
    }

    fn margin(&self) -> Option<Margin> {
        self.margin
    }
}
