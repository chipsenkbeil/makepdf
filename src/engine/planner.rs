use super::{EnginePageKind, EnginePages, Font};
use crate::pdf::{PdfConfig, PdfContext, PdfHooks};
use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use owned_ttf_parser::{AsFaceRef, OwnedFace};
use printpdf::*;

pub struct EnginePlanner {
    config: PdfConfig,
    pages: EnginePages,
    face: OwnedFace,
    font: IndirectFontRef,
}

impl EnginePlanner {
    pub fn new(config: PdfConfig) -> anyhow::Result<Self> {
        // Initialize our PDF document
        let doc = PdfDocument::empty(&config.title);
        let (page_width, page_height) = (config.page.width, config.page.height);
        let year = config.planner.year;

        // Load up our configured font
        let face = Font::load(config.page.font.as_deref())
            .context("Failed to load font")?
            .face;
        let font = doc
            .add_external_font(face.as_slice())
            .context("Failed to add external font")?;

        // Create our empty collection of pages to populate
        let pages = EnginePages::new(doc, (page_width, page_height));

        let first_day = NaiveDate::from_ymd_opt(year, 1, 1)
            .with_context(|| format!("Failed to construct beginning of year {year}"))?;
        let last_day = NaiveDate::from_ymd_opt(year, 12, 31)
            .with_context(|| format!("Failed to construct end of year {year}"))?;

        // Build the month pages (all empty)
        if config.planner.monthly.enabled {
            for i in 1..=12 {
                let date = NaiveDate::from_ymd_opt(year, i, 1)
                    .with_context(|| format!("Failed to construct month {i} of year {year}"))?;

                pages.add_monthly_page(date);
            }
        }

        // Build the weekly pages (all empty)
        if config.planner.weekly.enabled {
            for date in first_day
                .iter_weeks()
                .take_while(|date| date.year() == year)
            {
                pages.add_weekly_page(date);
            }
        }

        // Build the daily pages (all empty)
        if config.planner.daily.enabled {
            for date in first_day.iter_days().take_while(|date| date <= &last_day) {
                pages.add_daily_page(date);
            }
        }

        Ok(Self {
            config,
            pages,
            face,
            font,
        })
    }

    pub fn run_hooks(&self, hooks: PdfHooks) -> anyhow::Result<()> {
        // Run the hooks for the monthly, weekly, daily pages
        self.pages.for_each_page(|page| {
            if let Some(layer) = self.pages.get_page_layer_by_key(page.key()) {
                match page.kind {
                    EnginePageKind::Daily => {
                        for f in hooks.on_daily_page.iter() {
                            f.call(page.clone())
                                .context("Failed invoking hook: on_daily_page")?;
                        }
                    }
                    EnginePageKind::Monthly => {
                        for f in hooks.on_monthly_page.iter() {
                            f.call(page.clone())
                                .context("Failed invoking hook: on_monthly_page")?;
                        }
                    }
                    EnginePageKind::Weekly => {
                        for f in hooks.on_weekly_page.iter() {
                            f.call(page.clone())
                                .context("Failed invoking hook: on_weekly_page")?;
                        }
                    }
                }

                // With hooks finished running, draw the page (adding queued objects to it)
                page.draw(PdfContext {
                    config: &self.config,
                    face: self.face.as_face_ref(),
                    font: &self.font,
                    layer: &layer,
                });
            }

            Ok(())
        })
    }

    /// Consume planner, returning the raw PDF document.
    pub fn into_doc(self) -> PdfDocumentReference {
        self.pages.into_doc()
    }
}
