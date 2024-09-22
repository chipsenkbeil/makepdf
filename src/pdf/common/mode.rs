use mlua::prelude::*;
use printpdf::path::PaintMode;

/// Paint mode to use with shapes.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfPaintMode(PaintMode);

impl PdfPaintMode {
    #[inline]
    pub const fn clip() -> Self {
        Self(PaintMode::Clip)
    }

    #[inline]
    pub const fn fill() -> Self {
        Self(PaintMode::Fill)
    }

    #[inline]
    pub const fn fill_stroke() -> Self {
        Self(PaintMode::FillStroke)
    }

    #[inline]
    pub const fn stroke() -> Self {
        Self(PaintMode::Stroke)
    }
}

impl From<PdfPaintMode> for PaintMode {
    fn from(mode: PdfPaintMode) -> Self {
        mode.0
    }
}

impl<'lua> IntoLua<'lua> for PdfPaintMode {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self.0 {
            PaintMode::Clip => "clip",
            PaintMode::Fill => "fill",
            PaintMode::FillStroke => "fill_stroke",
            PaintMode::Stroke => "stroke",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfPaintMode {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "clip" => Ok(Self::clip()),
                "fill" => Ok(Self::fill()),
                "fill_stroke" => Ok(Self::fill_stroke()),
                "stroke" => Ok(Self::stroke()),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.paint_mode",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.paint_mode",
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
        assert_eq!(
            Lua::new()
                .load(chunk!("clip"))
                .eval::<PdfPaintMode>()
                .unwrap(),
            PdfPaintMode(PaintMode::Clip),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("fill"))
                .eval::<PdfPaintMode>()
                .unwrap(),
            PdfPaintMode(PaintMode::Fill),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("fill_stroke"))
                .eval::<PdfPaintMode>()
                .unwrap(),
            PdfPaintMode(PaintMode::FillStroke),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("stroke"))
                .eval::<PdfPaintMode>()
                .unwrap(),
            PdfPaintMode(PaintMode::Stroke),
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let clip_mode = PdfPaintMode(PaintMode::Clip);
        let fill_mode = PdfPaintMode(PaintMode::Fill);
        let fill_stroke_mode = PdfPaintMode(PaintMode::FillStroke);
        let stroke_mode = PdfPaintMode(PaintMode::Stroke);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($clip_mode, "clip")
                u.assert_deep_equal($fill_mode, "fill")
                u.assert_deep_equal($fill_stroke_mode, "fill_stroke")
                u.assert_deep_equal($stroke_mode, "stroke")
            })
            .exec()
            .expect("Assertion failed");
    }
}
