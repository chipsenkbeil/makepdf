use crate::pdf::*;
use mlua::prelude::*;
use printpdf::{Mm, Px};

/// Page-specific configuration for PDFs.
///
/// Supports converting to & from a Lua table.
#[derive(Clone, Debug)]
pub struct PdfConfigPage {
    /// DPI of a page.
    pub dpi: f32,
    /// Optional font for the PDF.
    pub font: Option<String>,
    /// Width of a page in millimeters.
    pub width: Mm,
    /// Height of a page in millimeters.
    pub height: Mm,

    /// Default font size used when none specified.
    pub font_size: f32,
    /// Default fill color used when none specified.
    pub fill_color: PdfColor,
    /// Default outline color used when none specified.
    pub outline_color: PdfColor,
    /// Default thickness for an outline when none specified.
    pub outline_thickness: f32,
    /// Default dash pattern of lines when none specified.
    pub line_dash_pattern: PdfLineDashPattern,
    /// Default cap style of lines when none specified.
    pub line_cap_style: PdfLineCapStyle,
    /// Default join style of lines when none specified.
    pub line_join_style: PdfLineJoinStyle,
}

impl Default for PdfConfigPage {
    /// Page defaults are modeled after the Supernote A6 X2 Nomad.
    fn default() -> Self {
        let dpi = 300.0;
        Self {
            dpi,
            font: None,
            width: Px(1404).into_pt(dpi).into(),
            height: Px(1872).into_pt(dpi).into(),

            font_size: 32.0,
            fill_color: PdfColor::grey(),
            outline_color: PdfColor::black(),
            outline_thickness: 1.0,
            line_dash_pattern: PdfLineDashPattern::solid(),
            line_cap_style: PdfLineCapStyle::round(),
            line_join_style: PdfLineJoinStyle::round(),
        }
    }
}

impl PdfConfigPage {
    /// Returns bounds covering the entire page based on its width and height.
    pub fn bounds(&self) -> PdfBounds {
        let (llx, lly) = (Mm(0.0), Mm(0.0));
        let (urx, ury) = (llx + self.width, lly + self.height);
        PdfBounds::from_coords(llx, lly, urx, ury)
    }
}

impl<'lua> IntoLua<'lua> for PdfConfigPage {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        // Configurations for page
        table.raw_set("dpi", self.dpi)?;
        table.raw_set("font", self.font)?;
        table.raw_set("width", self.width.0)?;
        table.raw_set("height", self.height.0)?;

        // Defaults for page
        table.raw_set("font_size", self.font_size)?;
        table.raw_set("fill_color", self.fill_color)?;
        table.raw_set("outline_color", self.outline_color)?;
        table.raw_set("outline_thickness", self.outline_thickness)?;
        table.raw_set("line_dash_pattern", self.line_dash_pattern)?;
        table.raw_set("line_cap_style", self.line_cap_style)?;
        table.raw_set("line_join_style", self.line_join_style)?;

        // Specialized helper functions
        metatable.raw_set(
            "bounds",
            lua.create_function(|_, this: PdfConfigPage| Ok(this.bounds()))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfConfigPage {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                // Configurations for page
                dpi: table.raw_get_ext("dpi")?,
                font: table.raw_get_ext("font")?,
                width: Mm(table.raw_get_ext("width")?),
                height: Mm(table.raw_get_ext("height")?),

                // Defaults for page
                font_size: table.raw_get_ext("font_size")?,
                fill_color: table.raw_get_ext("fill_color")?,
                outline_color: table.raw_get_ext("outline_color")?,
                outline_thickness: table.raw_get_ext("outline_thickness")?,
                line_dash_pattern: table.raw_get_ext("line_dash_pattern")?,
                line_cap_style: table.raw_get_ext("line_cap_style")?,
                line_join_style: table.raw_get_ext("line_join_style")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.config.page",
                message: None,
            }),
        }
    }
}

impl PdfConfigPage {
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
