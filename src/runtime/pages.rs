mod page;

pub use page::{RuntimePage, RuntimePageId};

use std::collections::HashMap;

/// Manages a collection of pages.
#[derive(Debug, Default)]
pub struct RuntimePages {
    /// Collection of page id -> page.
    pages: HashMap<RuntimePageId, RuntimePage>,

    /// Contains manual ordering of pages.
    ids: Vec<RuntimePageId>,
}

impl<'a> IntoIterator for &'a RuntimePages {
    type Item = &'a RuntimePage;
    type IntoIter = std::collections::hash_map::Values<'a, RuntimePageId, RuntimePage>;

    /// Returns iterator over refs of pages, not in order.
    fn into_iter(self) -> Self::IntoIter {
        self.pages.values()
    }
}

impl<'a> IntoIterator for &'a mut RuntimePages {
    type Item = &'a mut RuntimePage;
    type IntoIter = std::collections::hash_map::ValuesMut<'a, RuntimePageId, RuntimePage>;

    /// Returns iterator over mut refs of pages, not in order.
    fn into_iter(self) -> Self::IntoIter {
        self.pages.values_mut()
    }
}

impl IntoIterator for RuntimePages {
    type Item = RuntimePage;
    type IntoIter = std::collections::hash_map::IntoValues<RuntimePageId, Self::Item>;

    /// Returns iterator over pages, not in order.
    fn into_iter(self) -> Self::IntoIter {
        self.pages.into_values()
    }
}

impl RuntimePages {
    /// Creates a new, empty set of pages.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total number of pages.
    pub fn len(&self) -> usize {
        self.pages.len()
    }

    /// Returns `true` if pages is empty.
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// Returns an iterator over the ids of the pages, in order.
    pub fn ids(&self) -> impl Iterator<Item = RuntimePageId> + '_ {
        self.ids.iter().copied()
    }

    /// Inserts a page by its `id`, adding it to the end of the list, returning the id of the page.
    pub fn insert_page(&mut self, page: RuntimePage) -> RuntimePageId {
        let id = page.id;
        self.ids.push(id);
        self.pages.insert(id, page);
        id
    }

    /// Retrieves a copy of a page by its `id`.
    pub fn get_page(&self, id: RuntimePageId) -> Option<RuntimePage> {
        self.pages.get(&id).cloned()
    }
}
