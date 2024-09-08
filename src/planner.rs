use crate::constants::{DEFAULT_FONT, SCRIPTS};
use anyhow::Context;
use chrono::{Datelike, Days, NaiveDate};
use mlua::Lua;
use owned_ttf_parser::OwnedFace;
use printpdf::*;
use std::{fs::File, io::BufWriter};

//mod day;

/// Configuration tied to building a planner.
#[derive(Clone, Debug)]
pub struct PlannerConfig {
    /// Year associated with planner
    pub year: i32,
    /// Width x Height of each page within the planner
    pub dimensions: (Mm, Mm),
    /// DPI of PDF document
    pub dpi: f32,
    /// Optional font for the planner
    pub font: Option<String>,
    /// Path or name of script (e.g. `lpdf:panda`)
    pub script: String,
}

/// Primary entrypoint to building a planner.
pub struct Planner {
    config: PlannerConfig,
    doc: PdfDocumentReference,
    face: OwnedFace,
    font: IndirectFontRef,
    months: Vec<(PdfPageIndex, PdfLayerIndex)>,
    weeks: Vec<(PdfPageIndex, PdfLayerIndex)>,
    days: Vec<(PdfPageIndex, PdfLayerIndex)>,
}

impl Planner {
    /// Builds a planner - does not save it - using the provided `config`.
    pub fn build(config: PlannerConfig) -> anyhow::Result<Self> {
        let doc = PdfDocument::empty(format!("LPDF Planner {}", config.year));
        let (page_width, page_height) = config.dimensions;
        let font_bytes = match config.font.clone() {
            Some(path) => std::fs::read(path).context("Failed to read font")?,
            None => DEFAULT_FONT.to_vec(),
        };

        let year = config.year;
        let face = OwnedFace::from_vec(font_bytes, 0).context("Failed to build font into face")?;
        let font = doc
            .add_external_font(face.as_slice())
            .context("Failed to add external font")?;

        // Load our script either from our internal map or an external file
        let script = match config
            .script
            .strip_prefix("lpdf:")
            .and_then(|s| SCRIPTS.get(s))
        {
            Some(bytes) => bytes.to_vec(),
            None => std::fs::read(&config.script)
                .with_context(|| format!("Failed to load script {}", config.script))?,
        };

        let lua = Lua::new();
        let chunk = lua.load(script);
        chunk.exec().context("Failed to execute script")?;

        let mut this = Self {
            config,
            doc,
            face,
            font,
            months: Vec::new(),
            weeks: Vec::new(),
            days: Vec::new(),
        };

        let first_day = NaiveDate::from_ymd_opt(year, 1, 1)
            .with_context(|| format!("Failed to construct beginning of year {year}"))?;
        let last_day = NaiveDate::from_ymd_opt(year, 12, 31)
            .with_context(|| format!("Failed to construct end of year {year}"))?;

        // Build the month pages (all empty)
        for i in 1..=12 {
            let month = NaiveDate::from_ymd_opt(year, i, 1)
                .with_context(|| format!("Failed to construct month {i} of year {year}"))?;
            let month_name = format!("{}", month.format("%B"));
            this.months
                .push(this.doc.add_page(page_width, page_height, month_name));
        }

        // Build the weekly pages (all empty)
        for (i, _) in first_day
            .iter_weeks()
            .enumerate()
            .take_while(|(_, date)| date.year() == year)
        {
            this.weeks.push(
                this.doc
                    .add_page(page_width, page_height, format!("Week {i}")),
            );
        }

        // Build the daily pages (all empty)
        for date in first_day.iter_days().take_while(|date| date <= &last_day) {
            let i = date.day0();
            this.days.push(
                this.doc
                    .add_page(page_width, page_height, format!("Day {i}")),
            );

            // Build the page
            //day::make_page(&this, date);
        }

        Ok(this)
    }

    /// Returns the year associated with this planner.
    pub fn year(&self) -> i32 {
        self.config.year
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
        if date.year() == self.config.year {
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
        if date.year() == self.config.year {
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
        if date.year() == self.config.year {
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
