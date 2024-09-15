use super::{EnginePage, EnginePageKey, EnginePageKind};
use crate::pdf::PdfDate;
use chrono::Datelike;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

/// Collection of [`EnginePageKey`] -> [`EnginePage`].
#[derive(Clone, Debug, Default)]
pub struct EnginePagesMap(Arc<RwLock<HashMap<EnginePageKey, EnginePage>>>);

/// Weak pointer to a collection of [`EnginePageKey`] -> [`EnginePage`].
#[derive(Clone, Debug, Default)]
pub struct WeakEnginePagesMap(Weak<RwLock<HashMap<EnginePageKey, EnginePage>>>);

impl WeakEnginePagesMap {
    /// Upgrades a weak map pointer to a full pointer.
    pub fn upgrade(&self) -> Option<EnginePagesMap> {
        Weak::upgrade(&self.0).map(EnginePagesMap)
    }
}

impl EnginePagesMap {
    /// Downgrads a map pointer.
    pub fn downgrade(&self) -> WeakEnginePagesMap {
        WeakEnginePagesMap(Arc::downgrade(&self.0))
    }

    /// Insert a new `page` into the map, indexed by the page's key.
    pub fn insert_page(&self, page: EnginePage) -> EnginePageKey {
        let key = Self::new_page_key(page.kind, page.date);
        self.0.write().unwrap().insert(key, page);
        key
    }

    /// Retrieves a page of `kind` associated with `date` from the manager.
    pub fn get_page(&self, kind: EnginePageKind, date: PdfDate) -> Option<EnginePage> {
        let key = Self::new_page_key(kind, date);
        self.get_page_by_key(key)
    }

    /// Retrieves a page of `kind` associated with `date` from the manager.
    pub fn get_page_by_key(&self, key: EnginePageKey) -> Option<EnginePage> {
        self.0.read().unwrap().get(&key).cloned()
    }

    /// Iterates over all pages, invoking `f` per page.
    pub fn for_each_page(
        &self,
        f: impl Fn(EnginePage) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        for page in self.0.read().unwrap().values() {
            f(page.clone())?;
        }

        Ok(())
    }

    /// Constructs a key for a page of `kind` associated with `date`.
    #[inline]
    pub fn new_page_key(kind: EnginePageKind, date: PdfDate) -> (EnginePageKind, u32) {
        let x = match kind {
            EnginePageKind::Daily => date.ordinal0(),
            EnginePageKind::Monthly => date.month0(),
            EnginePageKind::Weekly => date.iso_week().week0(),
        };

        (kind, x)
    }
}
