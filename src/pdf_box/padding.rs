use printpdf::Mm;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Padding {
    pub top: Mm,
    pub left: Mm,
    pub right: Mm,
    pub bottom: Mm,
}

impl From<f32> for Padding {
    fn from(padding: f32) -> Self {
        Self::all(Mm(padding))
    }
}

impl From<(f32, f32, f32, f32)> for Padding {
    /// Converts a tuple of (top, left, right, bottom) into padding.
    fn from((top, left, right, bottom): (f32, f32, f32, f32)) -> Self {
        Self {
            top: Mm(top),
            left: Mm(left),
            right: Mm(right),
            bottom: Mm(bottom),
        }
    }
}

impl From<Mm> for Padding {
    fn from(padding: Mm) -> Self {
        Self::all(padding)
    }
}

impl From<(Mm, Mm, Mm, Mm)> for Padding {
    /// Converts a tuple of (top, left, right, bottom) into padding.
    fn from((top, left, right, bottom): (Mm, Mm, Mm, Mm)) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }
}

impl Padding {
    pub fn all(padding: impl Into<Mm>) -> Self {
        let padding = padding.into();
        Self {
            top: padding,
            left: padding,
            right: padding,
            bottom: padding,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn top(padding: impl Into<Mm>) -> Self {
        Self {
            top: padding.into(),
            ..Default::default()
        }
    }

    pub fn left(padding: impl Into<Mm>) -> Self {
        Self {
            left: padding.into(),
            ..Default::default()
        }
    }

    pub fn right(padding: impl Into<Mm>) -> Self {
        Self {
            right: padding.into(),
            ..Default::default()
        }
    }

    pub fn bottom(padding: impl Into<Mm>) -> Self {
        Self {
            bottom: padding.into(),
            ..Default::default()
        }
    }
}
