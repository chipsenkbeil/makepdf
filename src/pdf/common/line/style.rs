use mlua::prelude::*;
use printpdf::{LineCapStyle, LineJoinStyle};

/// Cap style to use with the line.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PdfLineCapStyle(LineCapStyle);

impl PdfLineCapStyle {
    #[inline]
    pub const fn butt() -> Self {
        Self(LineCapStyle::Butt)
    }

    #[inline]
    pub const fn round() -> Self {
        Self(LineCapStyle::Round)
    }

    #[inline]
    pub const fn projecting_square() -> Self {
        Self(LineCapStyle::ProjectingSquare)
    }
}

impl From<LineCapStyle> for PdfLineCapStyle {
    fn from(style: LineCapStyle) -> Self {
        Self(style)
    }
}

impl From<PdfLineCapStyle> for LineCapStyle {
    fn from(style: PdfLineCapStyle) -> Self {
        style.0
    }
}

impl<'lua> IntoLua<'lua> for PdfLineCapStyle {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self.0 {
            LineCapStyle::Butt => "butt",
            LineCapStyle::Round => "round",
            LineCapStyle::ProjectingSquare => "projecting_square",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfLineCapStyle {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "butt" => Ok(Self(LineCapStyle::Butt)),
                "round" => Ok(Self(LineCapStyle::Round)),
                "projecting_square" => Ok(Self(LineCapStyle::ProjectingSquare)),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.line.cap_style",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.line.cap_style",
                message: None,
            }),
        }
    }
}

/// Join style to use with the line.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PdfLineJoinStyle(LineJoinStyle);

impl PdfLineJoinStyle {
    #[inline]
    pub const fn limit() -> Self {
        Self(LineJoinStyle::Limit)
    }

    #[inline]
    pub const fn miter() -> Self {
        Self(LineJoinStyle::Miter)
    }

    #[inline]
    pub const fn round() -> Self {
        Self(LineJoinStyle::Round)
    }
}

impl From<LineJoinStyle> for PdfLineJoinStyle {
    fn from(style: LineJoinStyle) -> Self {
        Self(style)
    }
}

impl From<PdfLineJoinStyle> for LineJoinStyle {
    fn from(style: PdfLineJoinStyle) -> Self {
        style.0
    }
}

impl<'lua> IntoLua<'lua> for PdfLineJoinStyle {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self.0 {
            LineJoinStyle::Limit => "limit",
            LineJoinStyle::Miter => "miter",
            LineJoinStyle::Round => "round",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfLineJoinStyle {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "limit" => Ok(Self(LineJoinStyle::Limit)),
                "miter" => Ok(Self(LineJoinStyle::Miter)),
                "round" => Ok(Self(LineJoinStyle::Round)),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.line.join_style",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.line.join_style",
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
    fn cap_style_should_be_able_to_convert_from_lua() {
        assert_eq!(
            Lua::new()
                .load(chunk!("butt"))
                .eval::<PdfLineCapStyle>()
                .unwrap(),
            PdfLineCapStyle::butt(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("round"))
                .eval::<PdfLineCapStyle>()
                .unwrap(),
            PdfLineCapStyle::round(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("projecting_square"))
                .eval::<PdfLineCapStyle>()
                .unwrap(),
            PdfLineCapStyle::projecting_square(),
        );
    }

    #[test]
    fn cap_style_should_be_able_to_convert_into_lua() {
        let butt_cap_style = PdfLineCapStyle::butt();
        let round_cap_style = PdfLineCapStyle::round();
        let projecting_square_cap_style = PdfLineCapStyle::projecting_square();

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($butt_cap_style, "butt")
                u.assert_deep_equal($round_cap_style, "round")
                u.assert_deep_equal($projecting_square_cap_style, "projecting_square")
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn join_style_should_be_able_to_convert_from_lua() {
        assert_eq!(
            Lua::new()
                .load(chunk!("limit"))
                .eval::<PdfLineJoinStyle>()
                .unwrap(),
            PdfLineJoinStyle::limit(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("miter"))
                .eval::<PdfLineJoinStyle>()
                .unwrap(),
            PdfLineJoinStyle::miter(),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("round"))
                .eval::<PdfLineJoinStyle>()
                .unwrap(),
            PdfLineJoinStyle::round(),
        );
    }

    #[test]
    fn join_style_should_be_able_to_convert_into_lua() {
        let limit_join_style = PdfLineJoinStyle::limit();
        let miter_join_style = PdfLineJoinStyle::miter();
        let round_join_style = PdfLineJoinStyle::round();

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($limit_join_style, "limit")
                u.assert_deep_equal($miter_join_style, "miter")
                u.assert_deep_equal($round_join_style, "round")
            })
            .exec()
            .expect("Assertion failed");
    }
}
