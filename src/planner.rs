use crate::constants::{PAGE_HEIGHT, PAGE_WIDTH, REGULAR_FONT};
use chrono::{Datelike, Days, NaiveDate};
use owned_ttf_parser::OwnedFace;
use printpdf::*;

mod day;

pub struct PdfPlanner {
    pub doc: PdfDocumentReference,
    pub face: OwnedFace,
    pub font: IndirectFontRef,
    year: i32,
    months: Vec<(PdfPageIndex, PdfLayerIndex)>,
    weeks: Vec<(PdfPageIndex, PdfLayerIndex)>,
    days: Vec<(PdfPageIndex, PdfLayerIndex)>,
}

impl PdfPlanner {
    pub fn new(year: i32) -> Self {
        let doc = PdfDocument::empty(format!("Beatrix Planner {year}"));
        let face = OwnedFace::from_vec(REGULAR_FONT.to_vec(), 0).unwrap();
        let font = doc.add_external_font(REGULAR_FONT).unwrap();

        let mut this = Self {
            doc,
            face,
            font,
            year,
            months: Vec::new(),
            weeks: Vec::new(),
            days: Vec::new(),
        };

        let first_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let last_day = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        // Build the month pages (all empty)
        for i in 1..=12 {
            let month = NaiveDate::from_ymd_opt(year, i, 1).unwrap();
            let month_name = format!("{}", month.format("%B"));
            this.months
                .push(this.doc.add_page(PAGE_WIDTH, PAGE_HEIGHT, month_name));
        }

        // Build the weekly pages (all empty)
        for (i, _) in first_day
            .iter_weeks()
            .enumerate()
            .take_while(|(_, date)| date.year() == year)
        {
            this.weeks.push(
                this.doc
                    .add_page(PAGE_WIDTH, PAGE_HEIGHT, format!("Week {i}")),
            );
        }

        // Build the daily pages (all empty)
        for date in first_day.iter_days().take_while(|date| date <= &last_day) {
            let i = date.day0();
            this.days.push(
                this.doc
                    .add_page(PAGE_WIDTH, PAGE_HEIGHT, format!("Day {i}")),
            );

            // Build the page
            day::make_page(&this, date);
        }

        this
    }

    /// Returns the year associated with this planner.
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Retrieves the page & layer index for the date.
    pub fn get_monthly_index(&self, date: NaiveDate) -> Option<(PdfPageIndex, PdfLayerIndex)> {
        if date.year() == self.year {
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
        if date.year() == self.year {
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
        if date.year() == self.year {
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
