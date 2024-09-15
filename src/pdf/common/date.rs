use crate::pdf::PdfLuaTableExt;
use chrono::prelude::*;
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
        let table = lua.create_table()?;

        table.raw_set("year", self.0.year())?;
        table.raw_set("month", self.0.month())?;
        table.raw_set("day", self.0.day())?;

        table.raw_set("week", self.0.iso_week().week())?;
        table.raw_set("ordinal", self.0.ordinal())?;

        table.raw_set(
            "add_days",
            lua.create_function(move |_, days: i64| {
                self.add_days(days)
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "tomorrow",
            lua.create_function(move |_, ()| {
                self.tomorrow()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "yesterday",
            lua.create_function(move |_, ()| {
                self.yesterday()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "add_weeks",
            lua.create_function(move |_, weeks: i64| {
                self.add_weeks(weeks)
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "next_week",
            lua.create_function(move |_, ()| {
                self.next_week()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "last_week",
            lua.create_function(move |_, ()| {
                self.last_week()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "add_months",
            lua.create_function(move |_, months: i32| {
                self.add_months(months)
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "next_month",
            lua.create_function(move |_, ()| {
                self.next_month()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        table.raw_set(
            "last_month",
            lua.create_function(move |_, ()| {
                self.last_month()
                    .ok_or_else(|| LuaError::runtime("resulting date out of range"))
            })?,
        )?;

        let metatable = lua.create_table()?;
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
        metatable.raw_set(
            "__tostring",
            lua.create_function(|_, date: PdfDate| Ok(date.to_string()))?,
        )?;
        table.set_metatable(Some(metatable));

        // Mark table as read-only to prevent tampering without using specialized methods
        table.set_readonly(true);

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
                    f.call(())
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
                    ordinal = 258,

                    // NOTE: Deep equality check will include added field methods, so
                    //       we just copy these from the converted value for this to pass!
                    add_days = date.add_days,
                    add_months = date.add_months,
                }, {ignore_metatable = true})
            })
            .exec()
            .expect("Assertion failed");
    }
}
