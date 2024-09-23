use crate::PdfLuaTableExt;
use mlua::prelude::*;
use printpdf::Mm;

/// Padding for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfPadding {
    pub top: Mm,
    pub right: Mm,
    pub bottom: Mm,
    pub left: Mm,
}

impl PdfPadding {
    /// Create a new padding instance from the individual params.
    #[inline]
    pub const fn new(top: Mm, right: Mm, bottom: Mm, left: Mm) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Create a new padding instance from the individual params.
    #[inline]
    pub const fn new_f32(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self::new(Mm(top), Mm(right), Mm(bottom), Mm(left))
    }

    /// Create a new padding instance from top/bottom and right/left.
    #[inline]
    pub const fn from_pair(top_bottom: Mm, right_left: Mm) -> Self {
        Self::new(top_bottom, right_left, top_bottom, right_left)
    }

    /// Create a new padding instance from top, right/left, bottom.
    #[inline]
    pub const fn from_pair_f32(top_bottom: f32, right_left: f32) -> Self {
        Self::new_f32(top_bottom, right_left, top_bottom, right_left)
    }

    /// Create a new padding instance from top, right/left, bottom.
    #[inline]
    pub const fn from_triple(top: Mm, right_left: Mm, bottom: Mm) -> Self {
        Self::new(top, right_left, bottom, right_left)
    }

    /// Create a new padding instance from top, right/left, bottom.
    #[inline]
    pub const fn from_triple_f32(top: f32, right_left: f32, bottom: f32) -> Self {
        Self::new_f32(top, right_left, bottom, right_left)
    }

    /// Create a new padding instance where all sides match `padding`.
    #[inline]
    pub const fn from_single(padding: Mm) -> Self {
        Self::new(padding, padding, padding, padding)
    }

    /// Create a new padding instance where all sides match `padding`.
    #[inline]
    pub const fn from_single_f32(padding: f32) -> Self {
        Self::new_f32(padding, padding, padding, padding)
    }
}

impl<'lua> IntoLua<'lua> for PdfPadding {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("top", self.top.0)?;
        table.raw_set("right", self.right.0)?;
        table.raw_set("bottom", self.bottom.0)?;
        table.raw_set("left", self.left.0)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfPadding {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Number(num) => Ok(Self::from_single_f32(num as f32)),
            LuaValue::Integer(num) => Ok(Self::from_single_f32(num as f32)),
            LuaValue::Table(table) => {
                let maybe_vec_f32: Option<Vec<f32>> = table
                    .clone()
                    .sequence_values()
                    .collect::<LuaResult<_>>()
                    .ok();

                // If we have vec, check to make sure we have four items, and use them as padding
                if let Some(v) = maybe_vec_f32 {
                    if v.len() >= 4 {
                        return Ok(Self::new_f32(v[0], v[1], v[2], v[3]));
                    }
                    if v.len() == 3 {
                        return Ok(Self::from_triple_f32(v[0], v[1], v[2]));
                    }
                    if v.len() == 2 {
                        return Ok(Self::from_pair_f32(v[0], v[1]));
                    }
                    if v.len() == 1 {
                        return Ok(Self::from_single_f32(v[0]));
                    }
                }

                Ok(Self {
                    top: table
                        .raw_get_ext::<_, Option<f32>>("top")?
                        .map(Mm)
                        .unwrap_or_default(),
                    right: table
                        .raw_get_ext::<_, Option<f32>>("right")?
                        .map(Mm)
                        .unwrap_or_default(),
                    bottom: table
                        .raw_get_ext::<_, Option<f32>>("bottom")?
                        .map(Mm)
                        .unwrap_or_default(),
                    left: table
                        .raw_get_ext::<_, Option<f32>>("left")?
                        .map(Mm)
                        .unwrap_or_default(),
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.padding",
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
        // Can convert empty table into zero padding
        assert_eq!(
            Lua::new().load(chunk!({})).eval::<PdfPadding>().unwrap(),
            PdfPadding::new_f32(0.0, 0.0, 0.0, 0.0),
        );

        // Can convert integer into padding
        assert_eq!(
            Lua::new().load(chunk!(1)).eval::<PdfPadding>().unwrap(),
            PdfPadding::new_f32(1.0, 1.0, 1.0, 1.0),
        );

        // Can convert number into padding
        assert_eq!(
            Lua::new().load(chunk!(1.5)).eval::<PdfPadding>().unwrap(),
            PdfPadding::new_f32(1.5, 1.5, 1.5, 1.5),
        );

        // Can convert { number } into padding
        assert_eq!(
            Lua::new()
                .load(chunk!({ 1.5 }))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(1.5, 1.5, 1.5, 1.5),
        );

        // Can convert { number, number } into padding
        assert_eq!(
            Lua::new()
                .load(chunk!({1.5, 2.5}))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(1.5, 2.5, 1.5, 2.5),
        );

        // Can convert { number, number, number } into padding
        assert_eq!(
            Lua::new()
                .load(chunk!({1.5, 2.5, 3.5}))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(1.5, 2.5, 3.5, 2.5),
        );

        // Can convert { number, number, number, number } into padding
        assert_eq!(
            Lua::new()
                .load(chunk!({1.5, 2.5, 3.5, 4.5}))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(1.5, 2.5, 3.5, 4.5),
        );

        // Can convert { top, right, bottom, left } into padding
        assert_eq!(
            Lua::new()
                .load(chunk!({ top = 1.5, right = 2.5, bottom = 3.5, left = 4.5 }))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(1.5, 2.5, 3.5, 4.5),
        );

        // Can convert missing padding fields to zero padding
        assert_eq!(
            Lua::new()
                .load(chunk!({ top = 1.5 }))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(1.5, 0.0, 0.0, 0.0),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!({ right = 2.5 }))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(0.0, 2.5, 0.0, 0.0),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!({ bottom = 3.5 }))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(0.0, 0.0, 3.5, 0.0),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!({ left = 4.5 }))
                .eval::<PdfPadding>()
                .unwrap(),
            PdfPadding::new_f32(0.0, 0.0, 0.0, 4.5),
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let padding = PdfPadding::new_f32(1.0, 2.0, 3.0, 4.0);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($padding, { top = 1, right = 2, bottom = 3, left = 4 })
            })
            .exec()
            .expect("Assertion failed");
    }
}
