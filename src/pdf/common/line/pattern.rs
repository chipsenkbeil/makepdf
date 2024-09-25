use crate::pdf::PdfLuaTableExt;
use mlua::prelude::*;
use printpdf::LineDashPattern;

/// Pattern to use with the line.
///
/// Default pattern is a solid line.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfLineDashPattern(LineDashPattern);

impl PdfLineDashPattern {
    pub const DEFAULT_DASH_LENGTH: i64 = 5;

    /// Creates a new solid pattern.
    #[inline]
    pub const fn solid() -> Self {
        Self(LineDashPattern {
            offset: 0,
            dash_1: None,
            gap_1: None,
            dash_2: None,
            gap_2: None,
            dash_3: None,
            gap_3: None,
        })
    }

    /// Creates a new dashed pattern where length of each dash is the same `length`.
    pub fn dashed(length: i64) -> Self {
        Self(LineDashPattern {
            dash_1: Some(length),
            ..Default::default()
        })
    }
}

impl From<LineDashPattern> for PdfLineDashPattern {
    fn from(style: LineDashPattern) -> Self {
        Self(style)
    }
}

impl From<PdfLineDashPattern> for LineDashPattern {
    fn from(style: PdfLineDashPattern) -> Self {
        style.0
    }
}

impl<'lua> IntoLua<'lua> for PdfLineDashPattern {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("offset", self.0.offset)?;
        table.raw_set("dash_1", self.0.dash_1)?;
        table.raw_set("dash_2", self.0.dash_2)?;
        table.raw_set("dash_3", self.0.dash_3)?;
        table.raw_set("gap_1", self.0.gap_1)?;
        table.raw_set("gap_2", self.0.gap_2)?;
        table.raw_set("gap_3", self.0.gap_3)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfLineDashPattern {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            // Support some shortcut keywords to provide meaning
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "solid" => Ok(Self::default()),
                "dashed" => Ok(Self::dashed(Self::DEFAULT_DASH_LENGTH)),
                s => match s.strip_prefix("dashed:") {
                    Some(length) => Ok(Self::dashed(
                        length.trim().parse().map_err(LuaError::external)?,
                    )),
                    None => Err(LuaError::FromLuaConversionError {
                        from,
                        to: "pdf.common.line.pattern",
                        message: Some(format!("unknown pattern format: {s}")),
                    }),
                },
            },

            // Support reading table containing values directly
            LuaValue::Table(tbl) => Ok(Self(LineDashPattern {
                // For offset, we will use 0 as default if not provided
                offset: tbl
                    .raw_get_ext::<_, Option<_>>("offset")?
                    .unwrap_or_default(),
                dash_1: tbl.raw_get_ext("dash_1")?,
                gap_1: tbl.raw_get_ext("gap_1")?,
                dash_2: tbl.raw_get_ext("dash_2")?,
                gap_2: tbl.raw_get_ext("gap_2")?,
                dash_3: tbl.raw_get_ext("dash_3")?,
                gap_3: tbl.raw_get_ext("gap_3")?,
            })),
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.line.pattern",
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
        // Supports converting from certain strings
        assert_eq!(
            Lua::new()
                .load(chunk!("solid"))
                .eval::<PdfLineDashPattern>()
                .unwrap(),
            PdfLineDashPattern::solid(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("dashed"))
                .eval::<PdfLineDashPattern>()
                .unwrap(),
            PdfLineDashPattern::dashed(PdfLineDashPattern::DEFAULT_DASH_LENGTH),
        );

        // Supports converting from specialized string format
        assert_eq!(
            Lua::new()
                .load(chunk!("dashed:123"))
                .eval::<PdfLineDashPattern>()
                .unwrap(),
            PdfLineDashPattern::dashed(123),
        );

        // Supports converting from an empty table
        assert_eq!(
            Lua::new()
                .load(chunk!({}))
                .eval::<PdfLineDashPattern>()
                .unwrap(),
            PdfLineDashPattern::default(),
        );

        // Supports converting from a full table
        assert_eq!(
            Lua::new()
                .load(chunk!({
                    offset  = 1,
                    dash_1  = 2,
                    dash_2  = 3,
                    dash_3  = 4,
                    gap_1   = 5,
                    gap_2   = 6,
                    gap_3   = 7,
                }))
                .eval::<PdfLineDashPattern>()
                .unwrap(),
            PdfLineDashPattern::from(LineDashPattern {
                offset: 1,
                dash_1: Some(2),
                gap_1: Some(5),
                dash_2: Some(3),
                gap_2: Some(6),
                dash_3: Some(4),
                gap_3: Some(7),
            }),
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let dash_pattern = PdfLineDashPattern::default();

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($dash_pattern, { offset  = 0 })
            })
            .exec()
            .expect("Assertion failed");

        let dash_pattern = PdfLineDashPattern::from(LineDashPattern {
            offset: 1,
            dash_1: Some(2),
            gap_1: Some(5),
            dash_2: Some(3),
            gap_2: Some(6),
            dash_3: Some(4),
            gap_3: Some(7),
        });

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($dash_pattern, {
                    offset  = 1,
                    dash_1  = 2,
                    dash_2  = 3,
                    dash_3  = 4,
                    gap_1   = 5,
                    gap_2   = 6,
                    gap_3   = 7,
                })
            })
            .exec()
            .expect("Assertion failed");
    }
}
