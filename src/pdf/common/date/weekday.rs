use crate::pdf::{PdfLuaExt, PdfLuaTableExt};
use chrono::Weekday;
use mlua::prelude::*;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Weekday associated with a PDF date.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PdfDateWeekday(Weekday);

impl PdfDateWeekday {
    #[inline]
    pub const fn monday() -> Self {
        Self(Weekday::Mon)
    }

    #[inline]
    pub const fn tuesday() -> Self {
        Self(Weekday::Tue)
    }

    #[inline]
    pub const fn wednesday() -> Self {
        Self(Weekday::Wed)
    }

    #[inline]
    pub const fn thursday() -> Self {
        Self(Weekday::Thu)
    }

    #[inline]
    pub const fn friday() -> Self {
        Self(Weekday::Fri)
    }

    #[inline]
    pub const fn saturday() -> Self {
        Self(Weekday::Sat)
    }

    #[inline]
    pub const fn sunday() -> Self {
        Self(Weekday::Sun)
    }

    /// Returns a static string in the short form for a weekday.
    ///
    /// For example, Sunday would return `sun`.
    #[inline]
    pub const fn into_short_static_str(self) -> &'static str {
        match self.0 {
            Weekday::Mon => "mon",
            Weekday::Tue => "tue",
            Weekday::Wed => "wed",
            Weekday::Thu => "thu",
            Weekday::Fri => "fri",
            Weekday::Sat => "sat",
            Weekday::Sun => "sun",
        }
    }

    /// Returns a static string in the long form for a weekday.
    ///
    /// For example, Sunday would return `sunday`.
    #[inline]
    pub const fn into_long_static_str(self) -> &'static str {
        match self.0 {
            Weekday::Mon => "monday",
            Weekday::Tue => "tuesday",
            Weekday::Wed => "wednesday",
            Weekday::Thu => "thursday",
            Weekday::Fri => "friday",
            Weekday::Sat => "saturday",
            Weekday::Sun => "sunday",
        }
    }
}

impl Deref for PdfDateWeekday {
    type Target = Weekday;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PdfDateWeekday {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Weekday> for PdfDateWeekday {
    fn from(weekday: Weekday) -> Self {
        Self(weekday)
    }
}

impl From<PdfDateWeekday> for Weekday {
    fn from(weekday: PdfDateWeekday) -> Self {
        weekday.0
    }
}

impl FromStr for PdfDateWeekday {
    type Err = chrono::ParseWeekdayError;

    /// Parses short or long form weekday, case-insensitive.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl fmt::Display for PdfDateWeekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_long_static_str())
    }
}

impl<'lua> IntoLua<'lua> for PdfDateWeekday {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        metatable.raw_set(
            "short_name",
            lua.create_function(move |_, this: PdfDateWeekday| {
                Ok(this.into_short_static_str().to_string())
            })?,
        )?;

        metatable.raw_set(
            "long_name",
            lua.create_function(move |_, this: PdfDateWeekday| {
                Ok(this.into_long_static_str().to_string())
            })?,
        )?;

        metatable.raw_set(
            "next_weekday",
            lua.create_function(move |_, this: PdfDateWeekday| Ok(Self(this.0.succ())))?,
        )?;

        metatable.raw_set(
            "prev_weekday",
            lua.create_function(move |_, this: PdfDateWeekday| Ok(Self(this.0.pred())))?,
        )?;

        metatable.raw_set(
            "number_from_monday",
            lua.create_function(move |_, this: PdfDateWeekday| Ok(this.0.number_from_monday()))?,
        )?;

        metatable.raw_set(
            "number_from_sunday",
            lua.create_function(move |_, this: PdfDateWeekday| Ok(this.0.number_from_sunday()))?,
        )?;

        metatable.raw_set(
            "num_days_from_monday",
            lua.create_function(move |_, this: PdfDateWeekday| Ok(this.0.num_days_from_monday()))?,
        )?;

        metatable.raw_set(
            "num_days_from_sunday",
            lua.create_function(move |_, this: PdfDateWeekday| Ok(this.0.num_days_from_sunday()))?,
        )?;

        metatable.raw_set(
            "days_since",
            lua.create_function(move |_, (this, other): (PdfDateWeekday, PdfDateWeekday)| {
                Ok(this.0.days_since(other.0))
            })?,
        )?;

        metatable.raw_set(
            "__eq",
            lua.create_function(|_, (a, b): (PdfDateWeekday, PdfDateWeekday)| Ok(a == b))?,
        )?;

        metatable.raw_set(
            "__tostring",
            // NOTE: We explicitly don't leverage `this` because it causes a recursion issue. Since
            //       there are no fields within the table (so you cannot change the weekday without
            //       methods), we can instead use self from above.
            lua.create_function(move |_, ()| Ok(self.to_string()))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfDateWeekday {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        let to = "pdf.common.date.weekday";

        match value {
            // For a string, attempt to parse it as a weekday
            LuaValue::String(s) => Ok(s.to_str()?.parse().map_err(LuaError::external)?),

            // For a table, attempt to convert it to a string and then parse it as a weekday
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
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.date.weekday",
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
    fn should_be_able_to_retrieve_short_name_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:short_name()))
                .eval::<String>()
                .unwrap(),
            "mon"
        );
    }

    #[test]
    fn should_be_able_to_retrieve_long_name_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:long_name()))
                .eval::<String>()
                .unwrap(),
            "monday"
        );
    }

    #[test]
    fn should_be_able_to_get_next_weekday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:next_weekday()))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::tuesday()
        );

        let weekday = PdfDateWeekday::saturday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:next_weekday()))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::sunday()
        );

        let weekday = PdfDateWeekday::sunday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:next_weekday()))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::monday()
        );
    }

    #[test]
    fn should_be_able_to_get_previous_weekday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:prev_weekday()))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::sunday()
        );

        let weekday = PdfDateWeekday::saturday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:prev_weekday()))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::friday()
        );

        let weekday = PdfDateWeekday::sunday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:prev_weekday()))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::saturday()
        );
    }

    #[test]
    fn should_be_able_to_get_number_from_monday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:number_from_monday()))
                .eval::<u8>()
                .unwrap(),
            1
        );

        let weekday = PdfDateWeekday::saturday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:number_from_monday()))
                .eval::<u8>()
                .unwrap(),
            6
        );

        let weekday = PdfDateWeekday::sunday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:number_from_monday()))
                .eval::<u8>()
                .unwrap(),
            7
        );
    }

    #[test]
    fn should_be_able_to_get_number_from_sunday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:number_from_sunday()))
                .eval::<u8>()
                .unwrap(),
            2
        );

        let weekday = PdfDateWeekday::saturday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:number_from_sunday()))
                .eval::<u8>()
                .unwrap(),
            7
        );

        let weekday = PdfDateWeekday::sunday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:number_from_sunday()))
                .eval::<u8>()
                .unwrap(),
            1
        );
    }

    #[test]
    fn should_be_able_to_get_num_days_from_monday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:num_days_from_monday()))
                .eval::<u8>()
                .unwrap(),
            0
        );

        let weekday = PdfDateWeekday::saturday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:num_days_from_monday()))
                .eval::<u8>()
                .unwrap(),
            5
        );

        let weekday = PdfDateWeekday::sunday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:num_days_from_monday()))
                .eval::<u8>()
                .unwrap(),
            6
        );
    }

    #[test]
    fn should_be_able_to_get_num_days_from_sunday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:num_days_from_sunday()))
                .eval::<u8>()
                .unwrap(),
            1
        );

        let weekday = PdfDateWeekday::saturday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:num_days_from_sunday()))
                .eval::<u8>()
                .unwrap(),
            6
        );

        let weekday = PdfDateWeekday::sunday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:num_days_from_sunday()))
                .eval::<u8>()
                .unwrap(),
            0
        );
    }

    #[test]
    fn should_be_able_to_get_days_since_weekday_in_lua() {
        let weekday = PdfDateWeekday::monday();
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:days_since("monday")))
                .eval::<u8>()
                .unwrap(),
            0
        );
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:days_since("tuesday")))
                .eval::<u8>()
                .unwrap(),
            6
        );
        assert_eq!(
            Lua::new()
                .load(chunk!($weekday:days_since("sunday")))
                .eval::<u8>()
                .unwrap(),
            1
        );
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        // Can convert long string to weekday
        assert_eq!(
            Lua::new()
                .load(chunk!("monday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::monday(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("tuesday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::tuesday(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("wednesday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::wednesday(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("thursday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::thursday(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("friday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::friday(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("saturday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::saturday(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("sunday"))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::sunday(),
        );

        // Can convert table with tostring metatable function to weekday
        assert_eq!(
            Lua::new()
                .load(chunk!(setmetatable({}, {
                    __tostring = function()
                        return "monday"
                    end
                })))
                .eval::<PdfDateWeekday>()
                .unwrap(),
            PdfDateWeekday::monday(),
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let weekday = PdfDateWeekday::monday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "monday")
            })
            .exec()
            .expect("Assertion failed");

        let weekday = PdfDateWeekday::tuesday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "tuesday")
            })
            .exec()
            .expect("Assertion failed");

        let weekday = PdfDateWeekday::wednesday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "wednesday")
            })
            .exec()
            .expect("Assertion failed");

        let weekday = PdfDateWeekday::thursday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "thursday")
            })
            .exec()
            .expect("Assertion failed");

        let weekday = PdfDateWeekday::friday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "friday")
            })
            .exec()
            .expect("Assertion failed");

        let weekday = PdfDateWeekday::saturday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "saturday")
            })
            .exec()
            .expect("Assertion failed");

        let weekday = PdfDateWeekday::sunday();
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($weekday), "sunday")
            })
            .exec()
            .expect("Assertion failed");
    }
}
