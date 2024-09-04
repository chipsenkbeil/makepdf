use crate::constants::{PAGE_HEIGHT, PAGE_WIDTH, REGULAR_FONT};
use chrono::{Datelike, Month, NaiveDate};
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

    /// Retrieves the page & layer index for the specified month.
    pub fn get_index_for_month(&self, month: impl Into<Month>) -> (PdfPageIndex, PdfLayerIndex) {
        self.months
            .get(month.into() as usize)
            .expect("Month not found")
            .to_owned()
    }

    /// Retrieves the page & layer for the specified month.
    pub fn get_for_month(&self, month: impl Into<Month>) -> (PdfPageReference, PdfLayerReference) {
        let (page, layer) = self.get_index_for_month(month);
        let page = self.doc.get_page(page);
        let layer = page.get_layer(layer);
        (page, layer)
    }

    /// Retrieves the page & layer index for the specified week (index-1), panicking if out of
    /// range.
    pub fn get_index_for_week(&self, week: usize) -> (PdfPageIndex, PdfLayerIndex) {
        assert!(week > 0 && week < 53, "Week must be between 1 and 52");
        self.weeks.get(week - 1).expect("Week not found").to_owned()
    }

    /// Retrieves the page & layer for the specified week (index-1), panicking if out of range.
    pub fn get_for_week(&self, week: usize) -> (PdfPageReference, PdfLayerReference) {
        let (page, layer) = self.get_index_for_week(week);
        let page = self.doc.get_page(page);
        let layer = page.get_layer(layer);
        (page, layer)
    }

    /// Retrieves the page & layer index for the specified month & day (index-1), panicking if out
    /// of range of valid date within the year.
    pub fn get_index_for_day(
        &self,
        month: impl Into<Month>,
        day: u32,
    ) -> (PdfPageIndex, PdfLayerIndex) {
        let month = month.into();
        let date = NaiveDate::from_ymd_opt(self.year, (month as u32) + 1, day)
            .expect("Date must be valid for the year");
        self.days
            .get(date.ordinal0() as usize)
            .expect("Date not found")
            .to_owned()
    }

    /// Retrieves the page & layer for the specified month & day (index-1), panicking if out
    /// of range of valid date within the year.
    pub fn get_for_day(
        &self,
        month: impl Into<Month>,
        day: u32,
    ) -> (PdfPageReference, PdfLayerReference) {
        let (page, layer) = self.get_index_for_day(month, day);
        let page = self.doc.get_page(page);
        let layer = page.get_layer(layer);
        (page, layer)
    }
}
