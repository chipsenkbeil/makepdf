use crate::constants::*;
use crate::utils::{Padding, Rect};
use printpdf::Mm;

/// Represents ability to define padding for some object.
///
/// Requires that the object also implements bounds.
pub trait WithPadding: WithBounds {
    /// Returns padding of object.
    fn padding(&self) -> Option<Padding>;

    /// Sets new padding of the object.
    fn set_padding(&mut self, padding: Option<Padding>);
}

pub trait WithPaddingExt: WithPadding {
    fn with_padding(&mut self, padding: impl Into<Padding>) -> &mut Self {
        self.set_padding(Some(padding.into()));
        self
    }

    fn with_no_padding(&mut self) -> &mut Self {
        self.set_padding(None);
        self
    }

    /// Returns bounds adjusted to account for padding.
    fn bounds_with_padding(&self) -> Rect {
        let (mut llx, mut lly, mut urx, mut ury) = self.bounds().to_coords();

        if let Some(padding) = self.padding() {
            lly += padding.bottom;
            ury -= padding.top;
            llx += padding.left;
            urx -= padding.right;
        }

        Rect::from_coords(llx, lly, urx, ury)
    }
}

impl<T: WithPadding> WithPaddingExt for T {}

/// Represents ability to define bounds for some object.
pub trait WithBounds {
    /// Returns bounds of the object in form of a rectangle.
    fn bounds(&self) -> Rect;

    /// Sets new bounds of the object in form of a rectangle.
    fn set_bounds(&mut self, bounds: Rect);
}

/// Extension of the [`WithBounds`] trait to provide convience methods to adjust the bounds
/// of the object within the page.
pub trait WithBoundsExt: WithBounds {
    fn with_bounds(&mut self, bounds: impl Into<Rect>) -> &mut Self {
        let bounds = bounds.into();
        self.set_bounds(bounds);
        self
    }

    fn with_width(&mut self, width: Mm) -> &mut Self {
        let mut bounds = self.bounds();
        bounds.width = width;
        self.set_bounds(bounds);
        self
    }

    fn with_height(&mut self, height: Mm) -> &mut Self {
        let mut bounds = self.bounds();
        bounds.height = height;
        self.set_bounds(bounds);
        self
    }

    fn with_full_width(&mut self) -> &mut Self {
        self.with_width(PAGE_WIDTH)
    }

    fn with_three_quarters_width(&mut self) -> &mut Self {
        self.with_width(PAGE_WIDTH * 3.0 / 4.0)
    }

    fn with_half_width(&mut self) -> &mut Self {
        self.with_width(PAGE_WIDTH / 2.0)
    }

    fn with_quarter_width(&mut self) -> &mut Self {
        self.with_width(PAGE_WIDTH / 4.0)
    }

    fn with_eighth_width(&mut self) -> &mut Self {
        self.with_width(PAGE_WIDTH / 8.0)
    }

    fn with_sixteenth_width(&mut self) -> &mut Self {
        self.with_width(PAGE_WIDTH / 16.0)
    }

    fn shift_three_quarters_right(&mut self) -> &mut Self {
        self.shift_half_right();
        self.shift_quarter_right();
        self
    }

    fn shift_half_right(&mut self) -> &mut Self {
        self.shift_quarter_right();
        self.shift_quarter_right();
        self
    }

    fn shift_quarter_right(&mut self) -> &mut Self {
        self.shift_eighth_right();
        self.shift_eighth_right();
        self
    }

    fn shift_eighth_right(&mut self) -> &mut Self {
        let mut bounds = self.bounds();
        bounds.x += PAGE_WIDTH / 8.0;
        self.set_bounds(bounds);
        self
    }

    /// Columns are up to 8 with `col` being zero-indexed.
    fn at_col(&mut self, col: usize) -> &mut Self {
        let mut bounds = self.bounds();
        let col = col as f32;
        bounds.x = Mm(col * (PAGE_WIDTH.0 / 8.0));
        self.set_bounds(bounds);
        self
    }

    fn at_row(&mut self, row: usize) -> &mut Self {
        let mut bounds = self.bounds();
        bounds.y = PAGE_HEIGHT - (ROW_HEIGHT * (row + 1) as f32);
        bounds.height = ROW_HEIGHT;
        self.set_bounds(bounds);
        self
    }
}

impl<T: WithBounds> WithBoundsExt for T {}
