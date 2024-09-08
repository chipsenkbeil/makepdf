use printpdf::Mm;

pub type Margin = Space;
pub type Padding = Space;

/// Contains dimensions related to spacing associated with a component.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Space {
    pub top: Mm,
    pub left: Mm,
    pub right: Mm,
    pub bottom: Mm,
}

impl From<f32> for Space {
    fn from(space: f32) -> Self {
        Self::all(Mm(space))
    }
}

impl From<(f32, f32, f32, f32)> for Space {
    /// Converts a tuple of (top, left, right, bottom) into space.
    fn from((top, left, right, bottom): (f32, f32, f32, f32)) -> Self {
        Self {
            top: Mm(top),
            left: Mm(left),
            right: Mm(right),
            bottom: Mm(bottom),
        }
    }
}

impl From<Mm> for Space {
    fn from(space: Mm) -> Self {
        Self::all(space)
    }
}

impl From<(Mm, Mm, Mm, Mm)> for Space {
    /// Converts a tuple of (top, left, right, bottom) into space.
    fn from((top, left, right, bottom): (Mm, Mm, Mm, Mm)) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }
}

impl Space {
    pub fn all(space: impl Into<Mm>) -> Self {
        let space = space.into();
        Self {
            top: space,
            left: space,
            right: space,
            bottom: space,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn top(space: impl Into<Mm>) -> Self {
        Self {
            top: space.into(),
            ..Default::default()
        }
    }

    pub fn left(space: impl Into<Mm>) -> Self {
        Self {
            left: space.into(),
            ..Default::default()
        }
    }

    pub fn right(space: impl Into<Mm>) -> Self {
        Self {
            right: space.into(),
            ..Default::default()
        }
    }

    pub fn bottom(space: impl Into<Mm>) -> Self {
        Self {
            bottom: space.into(),
            ..Default::default()
        }
    }
}
