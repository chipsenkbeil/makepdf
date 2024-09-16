use super::RuntimePageKind;
use crate::pdf::PdfDate;
use chrono::Datelike;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimePageKey(RuntimePageKind, u32);

impl RuntimePageKey {
    /// Returns a copy of the page kind associated with the key.
    pub fn kind(self) -> RuntimePageKind {
        self.0
    }
}

impl From<(RuntimePageKind, u32)> for RuntimePageKey {
    fn from((kind, x): (RuntimePageKind, u32)) -> Self {
        Self(kind, x)
    }
}

impl From<(RuntimePageKind, PdfDate)> for RuntimePageKey {
    fn from((kind, date): (RuntimePageKind, PdfDate)) -> Self {
        Self(
            kind,
            match kind {
                RuntimePageKind::Daily => date.ordinal0(),
                RuntimePageKind::Monthly => date.month0(),
                RuntimePageKind::Weekly => date.iso_week().week0(),
            },
        )
    }
}
