mod font;
mod script;

use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use font::Font;
use owned_ttf_parser::AsFaceRef;
use printpdf::*;
use script::Script;
use std::{fs::File, io::BufWriter};

use super::{Pdf, PdfConfig, PdfContext, PdfDate, PdfPage};

/// PDF generation engine.
pub struct Engine {
    config: PdfConfig,
    doc: PdfDocumentReference,
}

impl Engine {
    /// Initialize and build the PDF using a Lua script engine, returning a reference to
    /// the engine that can be used to save the PDF externally.
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

        // Initialize our PDF document
        let doc = PdfDocument::empty(&pdf.config.title);
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
                let date = NaiveDate::from_ymd_opt(year, i, 1)
                    .with_context(|| format!("Failed to construct month {i} of year {year}"))?;
                let month_name = format!("{}", date.format("%B"));

                months.push((
                    doc.add_page(page_width, page_height, month_name),
                    PdfDate::from(date),
                    PdfPage::new(),
                ));
            }
        }

        // Build the weekly pages (all empty)
        let mut weeks = Vec::new();
        if pdf.config.planner.weekly.enabled {
            for (i, date) in first_day
                .iter_weeks()
                .enumerate()
                .take_while(|(_, date)| date.year() == year)
            {
                weeks.push((
                    doc.add_page(page_width, page_height, format!("Week {i}")),
                    PdfDate::from(date),
                    PdfPage::new(),
                ));
            }
        }

        // Build the daily pages (all empty)
        let mut days = Vec::new();
        if pdf.config.planner.daily.enabled {
            for date in first_day.iter_days().take_while(|date| date <= &last_day) {
                let i = date.day0();
                days.push((
                    doc.add_page(page_width, page_height, format!("Day {i}")),
                    PdfDate::from(date),
                    PdfPage::new(),
                ));
            }
        }

        // Run the hooks for the monthly page
        for ((pidx, lidx), date, page) in months {
            let layer = doc.get_page(pidx).get_layer(lidx);
            let ctx = PdfContext {
                config: &pdf.config,
                face: face.as_face_ref(),
                font: &font,
                layer: &layer,
            };

            for f in pdf.hooks.on_monthly_page.iter() {
                f.call((page.clone(), date))
                    .context("Failed invoking hook: on_monthly_page")?;
            }

            page.draw(&ctx);
        }

        // Run the hooks for the weekly page
        for ((pidx, lidx), date, page) in weeks {
            let layer = doc.get_page(pidx).get_layer(lidx);
            let ctx = PdfContext {
                config: &pdf.config,
                face: face.as_face_ref(),
                font: &font,
                layer: &layer,
            };

            for f in pdf.hooks.on_weekly_page.iter() {
                f.call((page.clone(), date))
                    .context("Failed invoking hook: on_weekly_page")?;
            }

            page.draw(&ctx);
        }

        // Run the hooks for the daily page
        for ((pidx, lidx), date, page) in days {
            let layer = doc.get_page(pidx).get_layer(lidx);
            let ctx = PdfContext {
                config: &pdf.config,
                face: face.as_face_ref(),
                font: &font,
                layer: &layer,
            };

            for f in pdf.hooks.on_daily_page.iter() {
                f.call((page.clone(), date))
                    .context("Failed invoking hook: on_daily_page")?;
            }

            page.draw(&ctx);
        }

        Ok(Self {
            config: pdf.config,
            doc,
        })
    }

    /// Returns the year associated with this planner.
    pub fn year(&self) -> i32 {
        self.config.planner.year
    }

    /// Saves the planner to the specified `filename`.
    pub fn save(self, filename: impl Into<String>) -> anyhow::Result<()> {
        let filename = filename.into();
        let f = File::create(&filename).with_context(|| format!("Failed to create {filename}"))?;
        self.doc
            .save(&mut BufWriter::new(f))
            .with_context(|| format!("Failed to save {filename}"))
    }
}
