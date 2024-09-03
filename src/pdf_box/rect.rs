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
