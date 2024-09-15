mod key;
mod kind;
mod page;

pub use key::EnginePageKey;
pub use kind::EnginePageKind;
pub use page::EnginePage;

use crate::pdf::{PdfConfigPlanner, PdfDate};
use anyhow::Context;
use chrono::Datelike;
use std::collections::HashMap;

/// Manages a collection of pages.
#[derive(Debug, Default)]
pub struct EnginePages {
    /// Collection of page key -> page.
    pages: HashMap<EnginePageKey, EnginePage>,
}

impl<'a> IntoIterator for &'a EnginePages {
    type Item = &'a EnginePage;
    type IntoIter = std::collections::hash_map::Values<'a, EnginePageKey, EnginePage>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.values()
    }
}

impl<'a> IntoIterator for &'a mut EnginePages {
    type Item = &'a mut EnginePage;
    type IntoIter = std::collections::hash_map::ValuesMut<'a, EnginePageKey, EnginePage>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.values_mut()
    }
}

impl IntoIterator for EnginePages {
    type Item = EnginePage;
    type IntoIter = std::collections::hash_map::IntoValues<EnginePageKey, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.into_values()
    }
}

impl EnginePages {
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
            for i in 1..=12 {
                let date = PdfDate::beginning_of_month(year, i)
                    .with_context(|| format!("Failed to construct month {i} of year {year}"))?;

                pages.add_monthly_page(date);
            }
        }

        // Build the weekly pages (all empty)
        if config.weekly.enabled {
            for date in first_day
                .iter_weeks()
                .take_while(|date| date.year() == year)
            {
                pages.add_weekly_page(date);
            }
        }

        // Build the daily pages (all empty)
        if config.daily.enabled {
            for date in first_day.iter_days().take_while(|date| date <= &last_day) {
                pages.add_daily_page(date);
            }
        }

        Ok(pages)
    }

    /// Returns an iterator over the keys of the pages.
    pub fn keys(&self) -> impl Iterator<Item = EnginePageKey> + '_ {
        self.pages.keys().copied()
    }

    /// Returns an iterator over the pages representing months.
    ///
    /// The iterator traverses the months in order.
    pub fn iter_monthly_pages(&self) -> impl Iterator<Item = EnginePage> + '_ {
        (0..12).flat_map(|i| {
            let key = EnginePageKey::from((EnginePageKind::Monthly, i));
            self.get_page(key)
        })
    }

    /// Returns an iterator over the pages representing weeks.
    ///
    /// The iterator traverses the weeks in order.
    pub fn iter_weekly_pages(&self) -> impl Iterator<Item = EnginePage> + '_ {
        (0..53).flat_map(|i| {
            let key = EnginePageKey::from((EnginePageKind::Weekly, i));
            self.get_page(key)
        })
    }

    /// Returns an iterator over the pages representing days.
    ///
    /// The iterator traverses the days in order.
    pub fn iter_daily_pages(&self) -> impl Iterator<Item = EnginePage> + '_ {
        (0..366).flat_map(|i| {
            let key = EnginePageKey::from((EnginePageKind::Daily, i));
            self.get_page(key)
        })
    }

    /// Adds a monthly page for the given `date`.
    pub fn add_monthly_page(&mut self, date: impl Into<PdfDate>) -> EnginePageKey {
        self.add_page(EnginePageKind::Monthly, date)
    }

    /// Adds a weekly page for the given `date`.
    pub fn add_weekly_page(&mut self, date: impl Into<PdfDate>) -> EnginePageKey {
        self.add_page(EnginePageKind::Weekly, date)
    }

    /// Adds a daily page for the given `date`.
    pub fn add_daily_page(&mut self, date: impl Into<PdfDate>) -> EnginePageKey {
        self.add_page(EnginePageKind::Daily, date)
    }

    /// Creates and inserts an empty page of `kind` at `date`.
    pub fn add_page(&mut self, kind: EnginePageKind, date: impl Into<PdfDate>) -> EnginePageKey {
        self.insert_page(EnginePage::new(kind, date.into()))
    }

    /// Inserts a page by its `key`, returning the key itself.
    pub fn insert_page(&mut self, page: EnginePage) -> EnginePageKey {
        let key = page.key();
        self.pages.insert(key, page);
        key
    }

    /// Retrieves a copy of a page by its `key`.
    pub fn get_page(&self, key: EnginePageKey) -> Option<EnginePage> {
        self.pages.get(&key).cloned()
    }

    /// Retrieves a copy of a page by its `kind` and `date`.
    pub fn get_page_by_date(&self, kind: EnginePageKind, date: PdfDate) -> Option<EnginePage> {
        self.get_page((kind, date).into())
    }
}
