use crate::PdfLuaTableExt;
use mlua::prelude::*;
use printpdf::{Mm, Px};

/// Page-specific configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct PagePdfConfig {
    /// DPI of a page
    pub dpi: f32,
    /// Optional font for the PDF
    pub font: Option<String>,
    /// Width of a page in millimeters
    pub width: Mm,
    /// Height of a page in millimeters
    pub height: Mm,
}

impl Default for PagePdfConfig {
    /// Page defaults are modeled after the Supernote A6 X2 Nomad.
    fn default() -> Self {
        let dpi = 300.0;
        Self {
            dpi,
            font: None,
            width: Px(1404).into_pt(dpi).into(),
            height: Px(1872).into_pt(dpi).into(),
        }
    }
}

impl<'lua> IntoLua<'lua> for PagePdfConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("dpi", self.dpi)?;
        table.raw_set("font", self.font)?;
        table.raw_set("width", self.width.0)?;
        table.raw_set("height", self.height.0)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PagePdfConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                dpi: table.raw_get_ext("dpi")?,
                font: table.raw_get_ext("font")?,
                width: Mm(table.raw_get_ext("width")?),
                height: Mm(table.raw_get_ext("height")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config.page",
                message: None,
            }),
        }
    }
}

impl PagePdfConfig {
    /// Creates a string in the form of `{WIDTH}x{HEIGHT}px`
    /// for the size represented within the config.
    pub fn to_px_size_string(&self) -> String {
        let width = (self.width.0 * self.dpi / 25.4).floor();
        let height = (self.height.0 * self.dpi / 25.4).floor();
        format!("{width}x{height}px")
    }

    /// Parse a string into dimensions `(width, height)`, supporting the following formats:
    ///
    /// 1. `{WIDTH}x{HEIGHT}in` for inches
    /// 2. `{WIDTH}x{HEIGHT}mm` for millimeters
    /// 3. `{WIDTH}x{HEIGHT}px` for pixels
    pub fn parse_size(s: &str, dpi: f32) -> anyhow::Result<(Mm, Mm)> {
        if s.len() < 2 {
            anyhow::bail!("Missing dimension units");
        }

        let s = s.to_lowercase();
        let (s, units) = s.split_at(s.len() - 2);
        let (width, height) = s.split_once('x').ok_or(anyhow::anyhow!(
            "Missing 'x' separator between dimension width & height"
        ))?;
        let width: f32 = width
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid dimension width! Must be numeric."))?;
        let height: f32 = height
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid dimension height! Must be numeric."))?;

        match units.trim() {
            // 1 in -> 25.4 mm
            "in" => Ok((Mm(width * 25.4), Mm(height * 25.4))),
            // mm is straight conversion
            "mm" => Ok((Mm(width), Mm(height))),
            // px -> pt (using DPI) -> mm
            "px" => Ok((
                Mm::from(Px(width as usize).into_pt(dpi)),
                Mm::from(Px(height as usize).into_pt(dpi)),
            )),
            // if we get a blank, still an error
            "" => Err(anyhow::anyhow!("Missing dimension units")),
            // otherwise, got something unexpected and should fail
            _ => Err(anyhow::anyhow!("Unknown dimension units")),
        }
    }
}
