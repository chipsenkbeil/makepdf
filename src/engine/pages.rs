mod kind;
mod map;
mod page;

pub use kind::EnginePageKind;
pub use map::{EnginePagesMap, WeakEnginePagesMap};
pub use page::{EnginePage, EnginePageKey};

use crate::pdf::PdfDate;
use chrono::Datelike;
use printpdf::{Mm, PdfDocumentReference, PdfLayerReference};
use std::ops::Deref;

/// Manages a collection of pages.
pub struct EnginePages {
    /// Reference to the PDF document.
    doc: PdfDocumentReference,

    /// Width x height of the pages within the PDF document.
    size: (Mm, Mm),

    /// Collection of page key -> page.
    pages: EnginePagesMap,
}

impl EnginePages {
    pub fn new(doc: PdfDocumentReference, size: (Mm, Mm)) -> Self {
        Self {
            doc,
            size,
            pages: Default::default(),
        }
    }

    /// Consumes manager, returning the underlying PDF document.
    pub fn into_doc(self) -> PdfDocumentReference {
        self.doc
    }

    pub fn add_monthly_page(&self, date: impl Into<PdfDate>) -> EnginePageKey {
        self.add_page(EnginePageKind::Monthly, date)
    }

    pub fn add_weekly_page(&self, date: impl Into<PdfDate>) -> EnginePageKey {
        self.add_page(EnginePageKind::Weekly, date)
    }

    pub fn add_daily_page(&self, date: impl Into<PdfDate>) -> EnginePageKey {
        self.add_page(EnginePageKind::Daily, date)
    }

    /// Adds a page of `kind` at `date` to the PDF document.
    pub fn add_page(&self, kind: EnginePageKind, date: impl Into<PdfDate>) -> EnginePageKey {
        let date = date.into();
        // Add the page to the PDF document
        let indexes = self.doc.add_page(
            self.size.0,
            self.size.1,
            match kind {
                EnginePageKind::Daily => format!("Day {}", date.day()),
                EnginePageKind::Monthly => date.format("%B").to_string(),
                EnginePageKind::Weekly => format!("Week {}", date.iso_week().week()),
            },
        );

        self.pages.insert_page(EnginePage {
            id: rand::random(),
            kind,
            date,
            indexes,
            pages: self.pages.downgrade(),
            objects: Default::default(),
        })
    }

    /// Retrieves the primary layer of a page.
    pub fn get_page_layer_by_key(&self, key: EnginePageKey) -> Option<PdfLayerReference> {
        self.pages
            .get_page_by_key(key)
            .map(|page| self.doc.get_page(page.indexes.0).get_layer(page.indexes.1))
    }
}

impl Deref for EnginePages {
    type Target = EnginePagesMap;

    fn deref(&self) -> &Self::Target {
        &self.pages
    }
}
