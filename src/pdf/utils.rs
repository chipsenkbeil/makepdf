use crate::pdf::{PdfBounds, PdfColor, PdfDate, PdfLink, PdfLuaExt, PdfPadding, PdfPoint};
use mlua::prelude::*;
use printpdf::{Mm, Pt};
use tailcall::tailcall;

/// Collection of utility functions.
#[derive(Copy, Clone, Debug, Default)]
pub struct PdfUtils;

impl PdfUtils {
    /// Inspects a Lua value, returning a string.
    pub fn inspect(value: LuaValue) -> String {
        format!("{value:#?}")
    }

    /// Inspects a Lua value, returning true if it is a string that starts with the prefix.
    pub fn starts_with(value: LuaValue, prefix: LuaValue) -> bool {
        match (value, prefix) {
            (LuaValue::String(value), LuaValue::String(prefix)) => {
                value.as_bytes().starts_with(prefix.as_bytes())
            }
            _ => false,
        }
    }

    /// Inspects a Lua value, returning true if it is a string that starts with the prefix.
    pub fn ends_with(value: LuaValue, prefix: LuaValue) -> bool {
        match (value, prefix) {
            (LuaValue::String(value), LuaValue::String(prefix)) => {
                value.as_bytes().ends_with(prefix.as_bytes())
            }
            _ => false,
        }
    }

    /// Converts a numeric point to millimeters.
    pub fn pt_to_mm(pt: LuaValue) -> LuaResult<f32> {
        match pt {
            LuaValue::Integer(value) => Ok(Mm::from(Pt(value as f32)).0),
            LuaValue::Number(value) => Ok(Mm::from(Pt(value as f32)).0),
            _ => Err(LuaError::runtime("value not numeric")),
        }
    }

    /// Converts numeric millimeters to a point.
    pub fn mm_to_pt(mm: LuaValue) -> LuaResult<f32> {
        match mm {
            LuaValue::Integer(value) => Ok(Mm(value as f32).into_pt().0),
            LuaValue::Number(value) => Ok(Mm(value as f32).into_pt().0),
            _ => Err(LuaError::runtime("value not numeric")),
        }
    }

    /// Deep compare values for equality, throwing an error if not matching equality expectation.
    ///
    /// Like [`PdfUtils::try_deep_equal`], but fails instead of returning equality.
    ///
    /// Provides a descriptive error message when not matching expectation by inspecting each
    /// value.
    pub fn try_assert_deep_equal(
        a: LuaValue,
        b: LuaValue,
        expected: bool,
        ignore_metatable: bool,
    ) -> LuaResult<()> {
        if Self::try_deep_equal(a.clone(), b.clone(), ignore_metatable)? != expected {
            let lines = [
                format!(
                    "Attempt to assert deeply a {} b failed!",
                    if expected { "==" } else { "~=" }
                ),
                String::new(),
                String::new(),
                format!("inspect(a): {}", Self::inspect(a)),
                String::new(),
                format!("inspect(b): {}", Self::inspect(b)),
            ];

            return Err(LuaError::runtime(lines.join(if cfg!(windows) {
                "\r\n"
            } else {
                "\n"
            })));
        }

        Ok(())
    }

    /// Deep compare values for equality.
    ///
    /// Tables are compared recursively unless they both provide the `eq` metamethod. All other
    /// types are compared using the equality `==` operator.
    #[tailcall]
    pub fn try_deep_equal(a: LuaValue, b: LuaValue, ignore_metatable: bool) -> LuaResult<bool> {
        // If first arg is a table, check if we are going to use its metatable __eq method if
        // available, otherwise do normal equality comparison check
        if let Some(tbl) = a.as_table() {
            if !ignore_metatable {
                if let Some(metatable) = tbl.get_metatable() {
                    if let Some(f) = metatable.get::<_, Option<LuaFunction>>("__eq")? {
                        if let Ok(true) = f.call::<_, bool>((tbl, b.clone())) {
                            return Ok(true);
                        }
                    }
                }
            }
        } else if a == b {
            return Ok(true);
        }

        // Check if different types, which should fail
        if a.type_name() != b.type_name() {
            return Ok(false);
        }

        // If both tables, we have already tested for metatable equality,
        // so now we want to compare recursively.
        if let (LuaValue::Table(a), LuaValue::Table(b)) = (a, b) {
            // Loop through all key/value pairs in first table, comparing
            // their values to the matching values in the second table.
            //
            // If the second table does not have a matching key, then
            // they are not equivalent.
            for pair in a.clone().pairs::<LuaValue, LuaValue>() {
                let (key, value) = pair?;
                if !Self::try_deep_equal(value, b.get(key)?, ignore_metatable)? {
                    return Ok(false);
                }
            }

            // Second, loop through all of the keys in the second table,
            // checking if they exist in the first table. If there is a
            // key that is not found, then they are not equivalent.
            for pair in b.pairs::<LuaValue, LuaValue>() {
                let (key, _) = pair?;
                if a.get::<_, LuaValue>(key)? == LuaNil {
                    return Ok(false);
                }
            }

            // Table comparison checks passed in both directions, so considered deeply equal
            return Ok(true);
        }

        Ok(false)
    }
}

impl<'lua> IntoLua<'lua> for PdfUtils {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        metatable.raw_set(
            "assert_deep_equal",
            lua.create_function(|_, (a, b, opts): (LuaValue, LuaValue, Option<LuaTable>)| {
                let ignore_metatable = opts
                    .map(|opts| opts.raw_get("ignore_metatable").unwrap_or(false))
                    .unwrap_or(false);
                PdfUtils::try_assert_deep_equal(a, b, true, ignore_metatable)
            })?,
        )?;

        metatable.raw_set(
            "assert_not_deep_equal",
            lua.create_function(|_, (a, b, opts): (LuaValue, LuaValue, Option<LuaTable>)| {
                let ignore_metatable = opts
                    .map(|opts| opts.raw_get("ignore_metatable").unwrap_or(false))
                    .unwrap_or(false);
                PdfUtils::try_assert_deep_equal(a, b, false, ignore_metatable)
            })?,
        )?;

        metatable.raw_set(
            "bounds",
            lua.create_function(|_, bounds: PdfBounds| Ok(bounds))?,
        )?;

        metatable.raw_set(
            "color",
            lua.create_function(|_, color: PdfColor| Ok(color))?,
        )?;

        metatable.raw_set("date", lua.create_function(|_, date: PdfDate| Ok(date))?)?;

        metatable.raw_set("link", lua.create_function(|_, link: PdfLink| Ok(link))?)?;

        metatable.raw_set("now", lua.create_function(|_, ()| Ok(PdfDate::now()))?)?;

        metatable.raw_set(
            "padding",
            lua.create_function(|_, padding: PdfPadding| Ok(padding))?,
        )?;

        metatable.raw_set(
            "point",
            lua.create_function(|_, point: PdfPoint| Ok(point))?,
        )?;

        metatable.raw_set(
            "deep_equal",
            lua.create_function(|_, (a, b, opts): (LuaValue, LuaValue, Option<LuaTable>)| {
                let ignore_metatable = opts
                    .map(|opts| opts.raw_get("ignore_metatable").unwrap_or(false))
                    .unwrap_or(false);
                PdfUtils::try_deep_equal(a, b, ignore_metatable)
            })?,
        )?;

        metatable.raw_set(
            "inspect",
            lua.create_function(|_, value: LuaValue| Ok(PdfUtils::inspect(value)))?,
        )?;

        metatable.raw_set(
            "starts_with",
            lua.create_function(|_, (value, prefix): (LuaValue, LuaValue)| {
                Ok(PdfUtils::starts_with(value, prefix))
            })?,
        )?;

        metatable.raw_set(
            "ends_with",
            lua.create_function(|_, (value, prefix): (LuaValue, LuaValue)| {
                Ok(PdfUtils::ends_with(value, prefix))
            })?,
        )?;

        metatable.raw_set(
            "mm_to_pt",
            lua.create_function(|_, value: LuaValue| PdfUtils::mm_to_pt(value))?,
        )?;

        metatable.raw_set(
            "pt_to_mm",
            lua.create_function(|_, value: LuaValue| PdfUtils::pt_to_mm(value))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::chunk;

    #[test]
    fn should_support_deeply_comparing_two_values_for_equality() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // All non-table values are compared directly
                u.assert_deep_equal(nil, nil)
                u.assert_not_deep_equal(nil, 1)

                u.assert_deep_equal(1, 1)
                u.assert_not_deep_equal(1, 2)
                u.assert_not_deep_equal(1, "1")

                u.assert_deep_equal(1.5, 1.5)
                u.assert_not_deep_equal(1.5, 2.5)
                u.assert_not_deep_equal(1.5, "1.5")

                u.assert_deep_equal("a", "a")
                u.assert_not_deep_equal("a", "b")
                u.assert_not_deep_equal("a", 1)

                u.assert_deep_equal(true, true)
                u.assert_deep_equal(false, false)
                u.assert_not_deep_equal(true, false)
                u.assert_not_deep_equal(true, 1)

                local f = function() end
                u.assert_deep_equal(f, f)
                u.assert_not_deep_equal(f, function() end)
                u.assert_not_deep_equal(f, 1)

                u.assert_deep_equal(vector(1, 2, 3), vector(1, 2, 3))
                u.assert_not_deep_equal(vector(1, 2, 3), vector(4, 5, 6))
                u.assert_not_deep_equal(vector(1, 2, 3), 1)

                // Table comparisons without eq metatables are recursive
                u.assert_deep_equal({}, {})
                u.assert_not_deep_equal({}, {a=1})
                u.assert_not_deep_equal({a=1}, {})
                u.assert_not_deep_equal({a=1}, {a=2})
                u.assert_deep_equal({a=1}, {a=1})

                // Table comparisons with eq metatable should be used unless disabled
                local tbl = setmetatable({}, {__eq = function(_, value) return value == 3 end})
                u.assert_deep_equal(tbl, 3)
                u.assert_not_deep_equal(tbl, 2)
                u.assert_not_deep_equal(tbl, 3, {ignore_metatable = true})
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_values_to_strings() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                u.assert_deep_equal(u.inspect(nil), "nil")
                u.assert_deep_equal(u.inspect(true), "true")
                u.assert_deep_equal(u.inspect(1), "1")
                u.assert_deep_equal(u.inspect(1.5), "1.5")
                u.assert_deep_equal(u.inspect("hello"), "\"hello\"")
                u.assert_deep_equal(u.inspect(vector(1, 2, 3)), "vector(1, 2, 3)")
                u.assert_deep_equal(u.inspect({}), "{}")
                u.assert_deep_equal(u.inspect({a=1}), "{\n  [\"a\"] = 1,\n}")

                // Things like functions are more dynamic, containing a pointer id
                local f_str = u.inspect(function() end)
                assert(u.starts_with(f_str, "function: "))

            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_checking_if_value_starts_with_prefix() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                assert(u.starts_with("", ""))
                assert(u.starts_with("abc", "abc"))
                assert(u.starts_with("abc", "ab"))
                assert(not u.starts_with("abc", "abcd"))
                assert(not u.starts_with("abc", "b"))
                assert(not u.starts_with("abc", 1))
                assert(not u.starts_with(1, "abc"))

            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_checking_if_value_ends_with_prefix() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                assert(u.ends_with("", ""))
                assert(u.ends_with("abc", "abc"))
                assert(u.ends_with("abc", "bc"))
                assert(not u.ends_with("abc", "abcd"))
                assert(not u.ends_with("abc", "b"))
                assert(not u.ends_with("abc", 1))
                assert(not u.ends_with(1, "abc"))

            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_millimeters_to_point() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // NOTE: We do some shortening because the calculations are approximate
                local expected = 2.8346
                local actual = math.floor(u.mm_to_pt(10000)) / 10000
                u.assert_deep_equal(expected, actual)
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_point_to_millimeters() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils

                // NOTE: We do some shortening because the calculations are approximate
                local expected = 0.3527
                local actual = math.floor(u.pt_to_mm(10000)) / 10000
                u.assert_deep_equal(expected, actual)
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_value_to_bounds() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(u.bounds({ 1, 2, 3, 4 }), {
                    ll = { x = 1, y = 2 },
                    ur = { x = 3, y = 4 },
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_value_to_color() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(u.color("#010203"), {
                    red = 1,
                    green = 2,
                    blue = 3,
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_value_to_link() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(u.link(123), {
                    type = "goto",
                    page = 123,
                })
                u.assert_deep_equal(u.link("https://example.com"), {
                    type = "uri",
                    uri = "https://example.com",
                })
                u.assert_deep_equal(u.link({
                    type = "goto",
                    page = 123,
                }), {
                    type = "goto",
                    page = 123,
                })
                u.assert_deep_equal(u.link({
                    type = "uri",
                    uri = "https://example.com",
                }), {
                    type = "uri",
                    uri = "https://example.com",
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_value_to_padding() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(u.padding({ 1, 2 }), {
                    top = 1,
                    right = 2,
                    bottom = 1,
                    left = 2,
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_support_converting_value_to_point() {
        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(u.point({ 1, 2 }), {
                    x = 1,
                    y = 2,
                })
            })
            .exec()
            .expect("Assertion failed");
    }
}
