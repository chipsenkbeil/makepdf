use printpdf::Mm;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Padding {
    pub top: Mm,
    pub left: Mm,
    pub right: Mm,
    pub bottom: Mm,
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
