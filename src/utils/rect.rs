use printpdf::Mm;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    /// Lower-left x position
    pub x: Mm,
    /// Lower-left y position
    pub y: Mm,
    /// Width of the box
    pub width: Mm,
    /// Height of the box
    pub height: Mm,
}

impl Rect {
    /// Creates a new [`Rect`] using the provided coordinates representing the lower-left and
    /// upper-right of the rectangle.
    #[inline]
    pub fn from_coords(llx: Mm, lly: Mm, urx: Mm, ury: Mm) -> Self {
        let width = urx - llx;
        let height = ury - lly;

        Self {
            x: llx,
            y: lly,
            width,
            height,
        }
    }

    /// Converts the [`Rect`] into the lower-left and upper-right coordinates of the rectangle.
    ///
    /// Specifically, `(lower-left x, lower-left y, upper-right x, upper-right y)`.
    #[inline]
    pub fn to_coords(self) -> (Mm, Mm, Mm, Mm) {
        (self.x, self.y, self.urx(), self.ury())
    }

    /// Returns the upper-right x coordinate based on the rectangle.
    #[inline]
    pub fn urx(&self) -> Mm {
        self.x + self.width
    }

    /// Returns the upper-right y coordinate based on the rectangle.
    #[inline]
    pub fn ury(&self) -> Mm {
        self.y + self.height
    }

    /// Checks if the rectangle has a defined width and height (greater than zero), otherwise it is
    /// considered unsized and should have sizing calculated in some other manner.
    #[inline]
    pub fn has_defined_size(&self) -> bool {
        self.has_defined_width() && self.has_defined_height()
    }

    /// Checks if the rectangle has a defined width (greater than zero), otherwise it is considered
    /// unsized for width and should have the width calculated in some other manner.
    #[inline]
    pub fn has_defined_width(&self) -> bool {
        self.width.0 > 0.0
    }

    /// Checks if the rectangle has a defined height (greater than zero), otherwise it is
    /// considered unsized for height and should have the height calculated in some other manner.
    #[inline]
    pub fn has_defined_height(&self) -> bool {
        self.height.0 > 0.0
    }
}

impl From<printpdf::Rect> for Rect {
    fn from(rect: printpdf::Rect) -> Self {
        Self::from_coords(
            rect.ll.x.into(),
            rect.ll.y.into(),
            rect.ur.x.into(),
            rect.ur.y.into(),
        )
    }
}

impl From<Rect> for printpdf::Rect {
    fn from(rect: Rect) -> Self {
        let (llx, lly, urx, ury) = rect.to_coords();
        Self::new(llx, lly, urx, ury)
    }
}
