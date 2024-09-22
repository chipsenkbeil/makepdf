mod weekday;

pub use weekday::PdfDateWeekday;

use crate::pdf::{PdfLuaExt, PdfLuaTableExt};
use chrono::prelude::*;
use chrono::Datelike;
use chrono::{Days, Months};
use mlua::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Date for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfDate(NaiveDate);

impl PdfDate {
    /// Returns the year associated with the date.
    ///
    /// Negatives represent BCE. e.g. -309 == 308 BCE.
    pub fn year(self) -> i32 {
        self.0.year()
    }

    /// Returns the weekday associated with the date.
    pub fn weekday(self) -> PdfDateWeekday {
        self.0.weekday().into()
    }

    /// Creates a new date for beginning of `year`. Returns None if invalid.
    pub fn beginning_of_year(year: i32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, 1, 1).map(PdfDate)
    }

    /// Returns a new date representing beginning of the year for current date.
    pub fn into_beginning_of_year(self) -> Self {
        Self::beginning_of_year(self.year()).unwrap()
    }

    /// Creates a new date for end of `year`. Returns None if invalid.
    pub fn end_of_year(year: i32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, 12, 31).map(PdfDate)
    }

    /// Returns a new date representing end of the year for current date.
    pub fn into_end_of_year(self) -> Self {
        Self::end_of_year(self.year()).unwrap()
    }

    /// Creates a new date for `year` at beginning of `month`. Returns None if invalid.
    pub fn beginning_of_month(year: i32, month: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, 1).map(PdfDate)
    }

    /// Returns a new date representing beginning of month for current date.
    pub fn into_beginning_of_month(self) -> Self {
        Self::beginning_of_month(self.year(), self.month()).unwrap()
    }

    /// Creates a new date for `year` at end of `month`. Returns None if invalid.
    pub fn end_of_month(year: i32, month: u32) -> Option<Self> {
        // Go to beginning of this month, advance a month, and then get yesterday
        Self::beginning_of_month(year, month)?
            .next_month()?
            .yesterday()
    }

    /// Returns a new date representing end of month for current date.
    pub fn into_end_of_month(self) -> Self {
        Self::end_of_month(self.year(), self.month()).unwrap()
    }

    /// Returns a new date representing beginning of week (Sunday-based) for current date.
    pub fn into_beginning_of_week_sunday(self) -> Self {
        let weekday = self.weekday();
        let num_days = weekday.num_days_from_sunday();
        self.add_days(-(num_days as i64)).unwrap()
    }

    /// Returns a new date representing end of week (Sunday-based) for current date.
    pub fn into_end_of_week_sunday(self) -> Self {
        self.into_beginning_of_week_sunday().add_days(6).unwrap()
    }

    /// Returns a new date representing beginning of week (Monday-based) for current date.
    pub fn into_beginning_of_week_monday(self) -> Self {
        let weekday = self.weekday();
        let num_days = weekday.num_days_from_monday();
        self.add_days(-(num_days as i64)).unwrap()
    }

    /// Returns a new date representing end of week (Monday-based) for current date.
    pub fn into_end_of_week_monday(self) -> Self {
        self.into_beginning_of_week_monday().add_days(6).unwrap()
    }

    /// Adds days to the date, returning the new date or none if the date would be out of range.
    ///
    /// The days to add can be negative, which will result in going backwards.
    pub fn add_days(self, days: i64) -> Option<Self> {
        match days.cmp(&0) {
            Ordering::Greater => self.0.checked_add_days(Days::new(days as u64)).map(Self),
            Ordering::Less => self.0.checked_sub_days(Days::new(-days as u64)).map(Self),
            Ordering::Equal => Some(self),
        }
    }

    /// Adds weeks to the date, returning the new date or none if the date would be out of range.
    ///
    /// The weeks to add can be negative, which will result in going backwards.
    pub fn add_weeks(self, weeks: i64) -> Option<Self> {
        self.add_days(weeks * 7)
    }

    /// Adds months to the date, returning the new date or none if the date would be out of range.
    ///
    /// The months to add can be negative, which will result in going backwards.
    pub fn add_months(self, months: i32) -> Option<Self> {
        match months.cmp(&0) {
            Ordering::Greater => self
                .0
                .checked_add_months(Months::new(months as u32))
                .map(Self),
            Ordering::Less => self
                .0
                .checked_sub_months(Months::new(-months as u32))
                .map(Self),
            Ordering::Equal => Some(self),
        }
    }

    /// Returns tomorrow's date, or none if the date would be out of range.
    #[inline]
    pub fn tomorrow(self) -> Option<Self> {
        self.add_days(1)
    }

    /// Returns yesterday's date, or none if the date would be out of range.
    #[inline]
    pub fn yesterday(self) -> Option<Self> {
        self.add_days(-1)
    }

    /// Returns next week's date, or none if the date would be out of range.
    #[inline]
    pub fn next_week(self) -> Option<Self> {
        self.add_weeks(1)
    }

    /// Returns last week's date, or none if the date would be out of range.
    #[inline]
    pub fn last_week(self) -> Option<Self> {
        self.add_weeks(-1)
    }

    /// Returns next month's date, or none if the date would be out of range.
    #[inline]
    pub fn next_month(self) -> Option<Self> {
        self.add_months(1)
    }

    /// Returns last month's date, or none if the date would be out of range.
    #[inline]
    pub fn last_month(self) -> Option<Self> {
        self.add_months(-1)
    }
}

impl Deref for PdfDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PdfDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for PdfDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl From<NaiveDate> for PdfDate {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl From<PdfDate> for NaiveDate {
    fn from(date: PdfDate) -> Self {
        date.0
    }
}

impl FromStr for PdfDate {
    type Err = chrono::format::ParseError;

    /// Parses a hex string into a color.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl<'lua> IntoLua<'lua> for PdfDate {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        table.raw_set("year", self.0.year())?;
        table.raw_set("month", self.0.month())?;
        table.raw_set("day", self.0.day())?;

        table.raw_set("week", self.0.iso_week().week())?;
        table.raw_set("weekday", self.weekday())?;
        table.raw_set("ordinal", self.0.ordinal())?;

        metatable.raw_set(
            "format",
            lua.create_function(move |_, format: String| {
                Ok(self.0.format(format.as_str()).to_string())
            })?,
        )?;

        metatable.raw_set(
            "add_days",
            lua.create_function(move |_, days: i64| {
                self.add_days(days)
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "tomorrow",
            lua.create_function(move |_, ()| {
                self.tomorrow()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "yesterday",
            lua.create_function(move |_, ()| {
                self.yesterday()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "add_weeks",
            lua.create_function(move |_, weeks: i64| {
                self.add_weeks(weeks)
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "next_week",
            lua.create_function(move |_, ()| {
                self.next_week()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "last_week",
            lua.create_function(move |_, ()| {
                self.last_week()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "add_months",
            lua.create_function(move |_, months: i32| {
                self.add_months(months)
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "next_month",
            lua.create_function(move |_, ()| {
                self.next_month()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "last_month",
            lua.create_function(move |_, ()| {
                self.last_month()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        metatable.raw_set(
            "beginning_of_year",
            lua.create_function(move |_, ()| Ok(self.into_beginning_of_year()))?,
        )?;

        metatable.raw_set(
            "end_of_year",
            lua.create_function(move |_, ()| Ok(self.into_end_of_year()))?,
        )?;

        metatable.raw_set(
            "beginning_of_month",
            lua.create_function(move |_, ()| Ok(self.into_beginning_of_month()))?,
        )?;

        metatable.raw_set(
            "end_of_month",
            lua.create_function(move |_, ()| Ok(self.into_end_of_month()))?,
        )?;

        metatable.raw_set(
            "beginning_of_week_sunday",
            lua.create_function(move |_, ()| Ok(self.into_beginning_of_week_sunday()))?,
        )?;

        metatable.raw_set(
            "end_of_week_sunday",
            lua.create_function(move |_, ()| Ok(self.into_end_of_week_sunday()))?,
        )?;

        metatable.raw_set(
            "beginning_of_week_monday",
            lua.create_function(move |_, ()| Ok(self.into_beginning_of_week_monday()))?,
        )?;

        metatable.raw_set(
            "end_of_week_monday",
            lua.create_function(move |_, ()| Ok(self.into_end_of_week_monday()))?,
        )?;

        metatable.raw_set(
            "__eq",
            lua.create_function(|_, (a, b): (PdfDate, PdfDate)| Ok(a.0 == b.0))?,
        )?;
        metatable.raw_set(
            "__lt",
            lua.create_function(|_, (a, b): (PdfDate, PdfDate)| Ok(a.0 < b.0))?,
        )?;
        metatable.raw_set(
            "__le",
            lua.create_function(|_, (a, b): (PdfDate, PdfDate)| Ok(a.0 <= b.0))?,
        )?;

        // Return copy of the date as a string. As the table is immutable, this shouldn't have any
        // issue staying current with the date instance.
        metatable.raw_set(
            "__tostring",
            lua.create_function(move |_, ()| Ok(self.to_string()))?,
        )?;

        // Mark table as read-only to prevent tampering without using specialized methods
        lua.mark_readonly(table.clone())?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfDate {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        let to = "pdf.common.date";

        match value {
            // For a string, attempt to parse it as a date
            LuaValue::String(s) => Ok(s.to_str()?.parse().map_err(LuaError::external)?),

            // For a table, attempt to convert it to a string and then parse it as a date
            LuaValue::Table(table) => match table.get_metatable() {
                Some(metatable) => {
                    let f = metatable.raw_get_ext::<_, LuaFunction>("__tostring")?;
                    f.call(table)
                }
                None => Err(LuaError::FromLuaConversionError {
                    from,
                    to,
                    message: Some(String::from(
                        "table does not have __tostring metatable method",
                    )),
                }),
            },

            // Anything else is invalid as a date
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to,
                message: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::PdfUtils;
    use mlua::chunk;

    #[test]
    fn should_be_able_to_format_in_lua() {
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.format("%B")))
                .eval::<String>()
                .unwrap(),
            "September",
        );
    }

    #[test]
    fn should_be_able_to_add_days_in_lua() {
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());

        // Test advancing a single day within same month
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_days(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()),
        );

        // Test backtracking a single day within same month
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_days(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 13).unwrap()),
        );

        // Test advancing to end of same month
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_days(16)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // Test backtracking to beginning of same month
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_days(-13)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 1).unwrap()),
        );

        // Test advancing to next month
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_days(17)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap()),
        );

        // Test backtracking to previous month
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_days(-14)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 8, 31).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_add_weeks_in_lua() {
        // Test advancing within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_weeks(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 21).unwrap()),
        );

        // Test backtracking within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_weeks(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
        );

        // Test advancing to next month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 25).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_weeks(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 11, 1).unwrap()),
        );

        // Test backtracking to last month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 7).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_weeks(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // Test advancing to next year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_weeks(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );

        // Test backtracking to previous year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_weeks(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_next_week_in_lua() {
        // Test advancing within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.next_week()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 21).unwrap()),
        );

        // Test advancing to next month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 25).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.next_week()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 11, 1).unwrap()),
        );

        // Test advancing to next year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.next_week()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_last_week_in_lua() {
        // Test backtracking within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.last_week()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
        );

        // Test backtracking to last month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 7).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.last_week()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // Test backtracking to previous year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.last_week()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_add_months_in_lua() {
        // Test advancing a single month within same year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_months(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 10, 14).unwrap()),
        );

        // Test backtracking a single month within same year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_months(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 8, 14).unwrap()),
        );

        // Test advancing a month that is shorter than current month (Oct 31 -> Nov 30)
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_months(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 11, 30).unwrap()),
        );

        // Test backtracking to a month that is shorter than the current month (Oct 31 -> Sept 30)
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_months(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // Test advancing to next year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 18).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_months(1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2025, 1, 18).unwrap()),
        );

        // Test backtracking to previous year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 18).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.add_months(-1)))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2023, 12, 18).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_next_month_in_lua() {
        // Test advancing to next month within same year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.next_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 10, 14).unwrap()),
        );

        // Test advancing to next month that is shorter than current month (Oct 31 -> Nov 30)
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.next_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 11, 30).unwrap()),
        );

        // Test advancing to next year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 18).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.next_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2025, 1, 18).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_last_month_in_lua() {
        // Test backtracking to last month within same year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.last_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 8, 14).unwrap()),
        );

        // Test backtracking to last month that is shorter than the current month (Oct 31 -> Sept 30)
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.last_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // Test backtracking to previous year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 18).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.last_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2023, 12, 18).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_tomorrow_in_lua() {
        // From middle of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.tomorrow()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()),
        );

        // From end of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.tomorrow()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap()),
        );

        // From end of a year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.tomorrow()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_yesterday_in_lua() {
        // From middle of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.yesterday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 13).unwrap()),
        );

        // From beginning of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.yesterday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // From beginning of a year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.yesterday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_beginning_of_year_in_lua() {
        // From middle of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_year()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        );

        // From beginning of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_year()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        );

        // From end of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_year()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_end_of_year_in_lua() {
        // From middle of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_year()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        );

        // From beginning of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_year()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        );

        // From end of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_year()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_beginning_of_month_in_lua() {
        // From middle of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 1).unwrap()),
        );

        // From beginning of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 1).unwrap()),
        );

        // From end of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 1).unwrap()),
        );

        // From beginning of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        );

        // From end of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 12, 1).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_end_of_month_in_lua() {
        // From middle of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // From beginning of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // From end of a month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );

        // From beginning of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()),
        );

        // From end of year
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_month()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_beginning_of_week_sunday_in_lua() {
        // From a Sunday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
        );

        // From a Monday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
        );

        // From a Tuesday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 10).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
        );

        // From a Saturday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
        );

        // From a date that will result in going to the previous month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 29).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_end_of_week_sunday_in_lua() {
        // From a Sunday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap()),
        );

        // From a Monday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap()),
        );

        // From a Tuesday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 10).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap()),
        );

        // From a Saturday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap()),
        );

        // From a date that will result in going to the next month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_sunday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 10, 5).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_beginning_of_week_monday_in_lua() {
        // From a Sunday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 2).unwrap()),
        );

        // From a Monday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap()),
        );

        // From a Tuesday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 10).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap()),
        );

        // From a Saturday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap()),
        );

        // From a date that will result in going to the previous month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.beginning_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_get_end_of_week_monday_in_lua() {
        // From a Sunday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
        );

        // From a Monday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()),
        );

        // From a Tuesday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 10).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()),
        );

        // From a Saturday within same month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()),
        );

        // From a date that will result in going to the next month
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap());
        assert_eq!(
            Lua::new()
                .load(chunk!($date.end_of_week_monday()))
                .eval::<PdfDate>()
                .unwrap(),
            PdfDate(NaiveDate::from_ymd_opt(2024, 10, 6).unwrap()),
        );
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        // Create date 2024/09/14 (September 14th, 2024)
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());

        // Can convert string into date
        assert_eq!(
            Lua::new()
                .load(chunk!("2024-09-14"))
                .eval::<PdfDate>()
                .unwrap(),
            date,
        );

        // Can convert table into date using __tostring metatable method
        assert_eq!(
            Lua::new()
                .load(chunk!(setmetatable({}, {
                    __tostring = function()
                        return "2024-09-14"
                    end
                })))
                .eval::<PdfDate>()
                .unwrap(),
            date,
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        // Create date 2024/09/14 (September 14th, 2024)
        let date = PdfDate(NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                local date = $date
                u.assert_deep_equal(date, {
                    year = 2024,
                    month = 9,
                    day = 14,
                    week = 37,
                    weekday = {}, // NOTE: Everything is in a metatable here.
                    ordinal = 258,
                }, {ignore_metatable = true})
            })
            .exec()
            .expect("Assertion failed");
    }
}
