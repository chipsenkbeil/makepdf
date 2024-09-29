mod key;
mod kind;
mod page;

pub use key::RuntimePageKey;
pub use kind::RuntimePageKind;
pub use page::RuntimePage;

use crate::pdf::{PdfConfigPlanner, PdfDate};
use anyhow::Context;
use chrono::Datelike;
use log::*;
use std::collections::HashMap;

/// Manages a collection of pages.
#[derive(Debug, Default)]
pub struct RuntimePages {
    /// Collection of page key -> page.
    pages: HashMap<RuntimePageKey, RuntimePage>,
}

impl<'a> IntoIterator for &'a RuntimePages {
    type Item = &'a RuntimePage;
    type IntoIter = std::collections::hash_map::Values<'a, RuntimePageKey, RuntimePage>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.values()
    }
}

impl<'a> IntoIterator for &'a mut RuntimePages {
    type Item = &'a mut RuntimePage;
    type IntoIter = std::collections::hash_map::ValuesMut<'a, RuntimePageKey, RuntimePage>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.values_mut()
    }
}

impl IntoIterator for RuntimePages {
    type Item = RuntimePage;
    type IntoIter = std::collections::hash_map::IntoValues<RuntimePageKey, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.into_values()
    }
}

impl RuntimePages {
    /// Builds a set of empty pages based on the PDF configuration.
    pub fn for_planner(config: &PdfConfigPlanner) -> anyhow::Result<Self> {
        let mut pages = Self {
            pages: Default::default(),
        };
        let year = config.year;

        let first_day = PdfDate::beginning_of_year(year)
            .with_context(|| format!("Failed to construct beginning of year {year}"))?;
        let last_day = PdfDate::end_of_year(year)
            .with_context(|| format!("Failed to construct end of year {year}"))?;

        // Build the month pages (all empty)
        if config.monthly.enabled {
            info!("Building monthly pages");
            for i in 1..=12 {
                let date = PdfDate::beginning_of_month(year, i)
                    .with_context(|| format!("Failed to construct month {i} of year {year}"))?;

                debug!("Adding monthly page: {}", date.format("%B %Y"));
                pages.add_monthly_page(date);
            }
        }

        // Build the weekly pages (all empty)
        if config.weekly.enabled {
            info!("Building weekly pages");
            for date in first_day
                .iter_weeks()
                .take_while(|date| date.year() == year)
            {
                debug!(
                    "Adding weekly page: {}-{}",
                    date.format("%B %d"),
                    PdfDate::from(date)
                        .into_end_of_week_monday()
                        .format("%d %Y")
                );
                pages.add_weekly_page(date);
            }
        }

        // Build the daily pages (all empty)
        if config.daily.enabled {
            info!("Building daily pages");
            for date in first_day.iter_days().take_while(|date| date <= &last_day) {
                debug!("Adding daily page for {}", date.format("%B %d, %Y"));
                pages.add_daily_page(date);
            }
        }

        Ok(pages)
    }

    /// Returns the total number of pages.
    pub fn len(&self) -> usize {
        self.pages.len()
    }

    /// Returns `true` if pages is empty.
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// Returns an iterator over the keys of the pages.
    pub fn keys(&self) -> impl Iterator<Item = RuntimePageKey> + '_ {
        self.pages.keys().copied()
    }

    /// Returns an iterator over the pages representing months.
    ///
    /// The iterator traverses the months in order.
    pub fn iter_monthly_pages(&self) -> impl Iterator<Item = RuntimePage> + '_ {
        (0..12).flat_map(|i| {
            let key = RuntimePageKey::from((RuntimePageKind::Monthly, i));
            self.get_page(key)
        })
    }

    /// Returns an iterator over the pages representing weeks.
    ///
    /// The iterator traverses the weeks in order.
    pub fn iter_weekly_pages(&self) -> impl Iterator<Item = RuntimePage> + '_ {
        (0..53).flat_map(|i| {
            let key = RuntimePageKey::from((RuntimePageKind::Weekly, i));
            self.get_page(key)
        })
    }

    /// Returns an iterator over the pages representing days.
    ///
    /// The iterator traverses the days in order.
    pub fn iter_daily_pages(&self) -> impl Iterator<Item = RuntimePage> + '_ {
        (0..366).flat_map(|i| {
            let key = RuntimePageKey::from((RuntimePageKind::Daily, i));
            self.get_page(key)
        })
    }

    /// Adds a monthly page for the given `date`.
    pub fn add_monthly_page(&mut self, date: impl Into<PdfDate>) -> RuntimePageKey {
        self.add_page(RuntimePageKind::Monthly, date)
    }

    /// Adds a weekly page for the given `date`.
    pub fn add_weekly_page(&mut self, date: impl Into<PdfDate>) -> RuntimePageKey {
        self.add_page(RuntimePageKind::Weekly, date)
    }

    /// Adds a daily page for the given `date`.
    pub fn add_daily_page(&mut self, date: impl Into<PdfDate>) -> RuntimePageKey {
        self.add_page(RuntimePageKind::Daily, date)
    }

    /// Creates and inserts an empty page of `kind` at `date`.
    pub fn add_page(&mut self, kind: RuntimePageKind, date: impl Into<PdfDate>) -> RuntimePageKey {
        self.insert_page(RuntimePage::new(kind, date.into()))
    }

    /// Inserts a page by its `key`, returning the key itself.
    pub fn insert_page(&mut self, page: RuntimePage) -> RuntimePageKey {
        let key = page.key();
        self.pages.insert(key, page);
        key
    }

    /// Retrieves a copy of a page by its `key`.
    pub fn get_page(&self, key: RuntimePageKey) -> Option<RuntimePage> {
        self.pages.get(&key).cloned()
    }

    /// Retrieves a copy of a page by its `kind` and `date`.
    pub fn get_page_by_date(&self, kind: RuntimePageKind, date: PdfDate) -> Option<RuntimePage> {
        self.get_page((kind, date).into())
    }
}
