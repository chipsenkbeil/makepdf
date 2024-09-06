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

impl From<printpdf::Rect> for Rect {
    fn from(rect: printpdf::Rect) -> Self {
        let width = rect.ur.x - rect.ll.x;
        let height = rect.ur.y - rect.ll.y;
        Self {
            x: rect.ll.x.into(),
            y: rect.ll.y.into(),
            width: width.into(),
            height: height.into(),
        }
    }
}

impl From<Rect> for printpdf::Rect {
    fn from(rect: Rect) -> Self {
        Self::new(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
    }
}
