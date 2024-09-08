use crate::constants::*;
use crate::utils::Rect;
use printpdf::Mm;

/// Provides methods to access and manipulate bounds on some object.
pub trait Bounds {
    /// Retrieve the bounds for an object.
    fn bounds(&self) -> Rect;

    /// Sets the bounds for some object.
    fn set_bounds(&mut self, rect: Rect);

    /// Updates bounds of the object, returning a mutable reference to it.
    fn with_bounds(&mut self, rect: Rect) -> &mut Self
    where
        Self: Sized,
    {
        self.set_bounds(rect);
        self
    }
}

impl Bounds for Rect {
    fn bounds(&self) -> Rect {
        *self
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.x = rect.x;
        self.y = rect.y;
        self.width = rect.width;
        self.height = rect.height;
    }
}

/// Provides additional methods to manipulate [`Bounds`].
pub trait BoundsExt: Bounds {
    fn with_width(&mut self, width: Mm) -> &mut Self;
    fn with_height(&mut self, height: Mm) -> &mut Self;
    fn with_full_width(&mut self) -> &mut Self;
    fn with_three_quarters_width(&mut self) -> &mut Self;
    fn with_half_width(&mut self) -> &mut Self;
    fn with_quarter_width(&mut self) -> &mut Self;
    fn with_eighth_width(&mut self) -> &mut Self;
    fn with_sixteenth_width(&mut self) -> &mut Self;
    fn shift_three_quarters_right(&mut self) -> &mut Self;
    fn shift_half_right(&mut self) -> &mut Self;
    fn shift_quarter_right(&mut self) -> &mut Self;
    fn shift_eighth_right(&mut self) -> &mut Self;
    fn at_col(&mut self, col: usize) -> &mut Self;
    fn at_row(&mut self, row: usize) -> &mut Self;
}

impl<T: Bounds + Sized> BoundsExt for T {
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
        bounds.set_bounds(bounds);
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
