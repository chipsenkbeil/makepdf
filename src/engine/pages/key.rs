use super::EnginePageKind;
use crate::pdf::PdfDate;
use chrono::Datelike;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnginePageKey(EnginePageKind, u32);

impl EnginePageKey {
    /// Returns a copy of the page kind associated with the key.
    pub fn kind(self) -> EnginePageKind {
        self.0
    }
}

impl From<(EnginePageKind, u32)> for EnginePageKey {
    fn from((kind, x): (EnginePageKind, u32)) -> Self {
        Self(kind, x)
    }
}

impl From<(EnginePageKind, PdfDate)> for EnginePageKey {
    fn from((kind, date): (EnginePageKind, PdfDate)) -> Self {
        Self(
            kind,
            match kind {
                EnginePageKind::Daily => date.ordinal0(),
                EnginePageKind::Monthly => date.month0(),
                EnginePageKind::Weekly => date.iso_week().week0(),
            },
        )
    }
}
