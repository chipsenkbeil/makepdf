use mlua::prelude::*;
use printpdf::path::PaintMode;

/// Paint mode to use with shapes.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfPaintMode(PaintMode);

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
                "clip" => Ok(Self(PaintMode::Clip)),
                "fill" => Ok(Self(PaintMode::Fill)),
                "fill_stroke" => Ok(Self(PaintMode::FillStroke)),
                "stroke" => Ok(Self(PaintMode::Stroke)),
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
