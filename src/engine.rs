mod font;
mod script;

use anyhow::Context;
use chrono::{Datelike, Days, NaiveDate};
use font::Font;
use owned_ttf_parser::OwnedFace;
use printpdf::*;
use script::Script;
use std::{fs::File, io::BufWriter};

use super::{Pdf, PdfConfig};

/// PDF generation engine.
pub struct Engine {
    pdf: Pdf,
    doc: PdfDocumentReference,
    face: OwnedFace,
    font: IndirectFontRef,
    months: Vec<(PdfPageIndex, PdfLayerIndex)>,
    weeks: Vec<(PdfPageIndex, PdfLayerIndex)>,
    days: Vec<(PdfPageIndex, PdfLayerIndex)>,
}

impl Engine {
    /// Initializes the engine using the specified `config`.
    ///
    /// This will involve loading the appropriate script, executing it to prepare, and
    /// generating an empty document.
    pub fn build(config: PdfConfig) -> anyhow::Result<Self> {
        // Execute a script to populate the information we need to generate a PDF
        let mut script = Script::load(&config.script).context("Failed to load script")?;
        script
            .set_global("pdf", Pdf::new(config))
            .context("Failed to initialize PDF script global")?;
        script.exec()?;

        // Retrieve the post-script PDF information
        let pdf: Pdf = script
            .get_global("pdf")
            .context("Failed to retrieve PDF information post-script")?;
        println!("{pdf:?}");

        // Initialize our PDF document
        let doc = PdfDocument::empty(format!("Planner {}", pdf.config.planner.year));
        let (page_width, page_height) = (pdf.config.page.width, pdf.config.page.height);
        let year = pdf.config.planner.year;

        // Load up our configured font
        let face = Font::load(pdf.config.page.font.as_deref())
            .context("Failed to load font")?
            .face;
        let font = doc
            .add_external_font(face.as_slice())
            .context("Failed to add external font")?;

        let first_day = NaiveDate::from_ymd_opt(year, 1, 1)
            .with_context(|| format!("Failed to construct beginning of year {year}"))?;
        let last_day = NaiveDate::from_ymd_opt(year, 12, 31)
            .with_context(|| format!("Failed to construct end of year {year}"))?;

        // Build the month pages (all empty)
        let mut months = Vec::new();
        if pdf.config.planner.monthly.enabled {
            for i in 1..=12 {
                let month = NaiveDate::from_ymd_opt(year, i, 1)
                    .with_context(|| format!("Failed to construct month {i} of year {year}"))?;
                let month_name = format!("{}", month.format("%B"));
                let page = doc.add_page(page_width, page_height, month_name);
                months.push(page);
            }
        }

        // Build the weekly pages (all empty)
        let mut weeks = Vec::new();
        if pdf.config.planner.weekly.enabled {
            for (i, _) in first_day
                .iter_weeks()
                .enumerate()
                .take_while(|(_, date)| date.year() == year)
            {
                let page = doc.add_page(page_width, page_height, format!("Week {i}"));
                weeks.push(page);
            }
        }

        // Build the daily pages (all empty)
        let mut days = Vec::new();
        if pdf.config.planner.daily.enabled {
            for date in first_day.iter_days().take_while(|date| date <= &last_day) {
                let i = date.day0();
                let page = doc.add_page(page_width, page_height, format!("Day {i}"));
                days.push(page);
            }
        }

        Ok(Self {
            pdf,
            doc,
            face,
            font,
            months,
            weeks,
            days,
        })
    }

    /// Returns the year associated with this planner.
    pub fn year(&self) -> i32 {
        self.pdf.config.planner.year
    }

    /// Saves the planner to the specified `filename`.
    pub fn save(self, filename: impl Into<String>) -> anyhow::Result<()> {
        let filename = filename.into();
        let f = File::create(&filename).with_context(|| format!("Failed to create {filename}"))?;
        self.doc
            .save(&mut BufWriter::new(f))
            .with_context(|| format!("Failed to save {filename}"))
    }

    /// Retrieves the page & layer index for the date.
    pub fn get_monthly_index(&self, date: NaiveDate) -> Option<(PdfPageIndex, PdfLayerIndex)> {
        if date.year() == self.pdf.config.planner.year {
            let idx = date.month0() as usize;
            self.months.get(idx).copied()
        } else {
            None
        }
    }

    /// Retrieves the page & layer for the date.
    pub fn get_monthly_reference(
        &self,
        date: NaiveDate,
    ) -> Option<(PdfPageReference, PdfLayerReference)> {
        self.get_monthly_index(date).map(|(page, layer)| {
            let page = self.doc.get_page(page);
            let layer = page.get_layer(layer);
            (page, layer)
        })
    }

    /// Retrieves the page & layer index for the date.
    pub fn get_weekly_index(&self, date: NaiveDate) -> Option<(PdfPageIndex, PdfLayerIndex)> {
        if date.year() == self.pdf.config.planner.year {
            self.weeks.get(date.iso_week().week0() as usize).copied()
        } else {
            None
        }
    }

    /// Retrieves the page & layer for the date.
    pub fn get_weekly_reference(
        &self,
        date: NaiveDate,
    ) -> Option<(PdfPageReference, PdfLayerReference)> {
        self.get_weekly_index(date).map(|(page, layer)| {
            let page = self.doc.get_page(page);
            let layer = page.get_layer(layer);
            (page, layer)
        })
    }

    /// Retrieves the page & layer index for the date.
    pub fn get_daily_index(&self, date: NaiveDate) -> Option<(PdfPageIndex, PdfLayerIndex)> {
        if date.year() == self.pdf.config.planner.year {
            self.days.get(date.ordinal0() as usize).copied()
        } else {
            None
        }
    }

    /// Retrieves the page & layer for the date, panicking if out of range of valid date within the
    /// year.
    pub fn get_daily_reference(
        &self,
        date: NaiveDate,
    ) -> Option<(PdfPageReference, PdfLayerReference)> {
        self.get_daily_index(date).map(|(page, layer)| {
            let page = self.doc.get_page(page);
            let layer = page.get_layer(layer);
            (page, layer)
        })
    }

    pub fn get_prev_daily_index(&self, date: NaiveDate) -> Option<(PdfPageIndex, PdfLayerIndex)> {
        self.get_daily_index(date - Days::new(1))
    }

    pub fn get_prev_daily_reference(
        &self,
        date: NaiveDate,
    ) -> Option<(PdfPageReference, PdfLayerReference)> {
        self.get_daily_reference(date - Days::new(1))
    }

    pub fn get_next_daily_index(&self, date: NaiveDate) -> Option<(PdfPageIndex, PdfLayerIndex)> {
        self.get_daily_index(date + Days::new(1))
    }

    pub fn get_next_daily_reference(
        &self,
        date: NaiveDate,
    ) -> Option<(PdfPageReference, PdfLayerReference)> {
        self.get_daily_reference(date + Days::new(1))
    }
}
