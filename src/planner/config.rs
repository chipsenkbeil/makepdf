use crate::units;
use mlua::prelude::*;

/// Configuration tied to building a planner.
#[derive(Clone, Debug)]
pub struct PlannerConfig {
    /// Year associated with planner
    pub year: i32,
    /// Width x Height of each page within the planner
    pub dimensions: PlannerDimensions,
    /// DPI of PDF document
    pub dpi: f32,
    /// Optional font for the planner
    pub font: Option<String>,
    /// Path or name of script (e.g. `mpdf:panda`)
    pub script: String,
}

impl<'lua> IntoLua<'lua> for PlannerConfig {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("year", self.year)?;
        table.raw_set("dimensions", self.dimensions)?;
        table.raw_set("dpi", self.dpi)?;
        table.raw_set("font", self.font)?;
        table.raw_set("script", self.script)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PlannerConfig {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                year: table.raw_get("year")?,
                dimensions: table.raw_get("dimensions")?,
                dpi: table.raw_get("dpi")?,
                font: table.raw_get("font")?,
                script: table.raw_get("script")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "planner config",
                message: None,
            }),
        }
    }
}

/// Dimensions for a planner's page.
#[derive(Clone, Debug)]
pub struct PlannerDimensions {
    pub width: units::Mm,
    pub height: units::Mm,
}

impl<'lua> IntoLua<'lua> for PlannerDimensions {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;

        table.raw_set("width", self.width.0)?;
        table.raw_set("height", self.height.0)?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PlannerDimensions {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Ok(Self {
                width: units::Mm(table.raw_get("width")?),
                height: units::Mm(table.raw_get("height")?),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "planner dimensions",
                message: None,
            }),
        }
    }
}

impl PlannerDimensions {
    /// Parse a string into dimensions `(width, height)`, supporting the following formats:
    ///
    /// 1. `{WIDTH}x{HEIGHT}in` for inches
    /// 2. `{WIDTH}x{HEIGHT}mm` for millimeters
    /// 3. `{WIDTH}x{HEIGHT}px` for pixels
    pub fn from_str(s: &str, dpi: f32) -> anyhow::Result<Self> {
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
            "in" => Ok(Self {
                width: units::Mm(width * 25.4),
                height: units::Mm(height * 25.4),
            }),
            // mm is straight conversion
            "mm" => Ok(Self {
                width: units::Mm(width),
                height: units::Mm(height),
            }),
            // px -> pt (using DPI) -> mm
            "px" => Ok(Self {
                width: units::Mm::from(units::Px(width as usize).into_pt(dpi)),
                height: units::Mm::from(units::Px(height as usize).into_pt(dpi)),
            }),
            // if we get a blank, still an error
            "" => Err(anyhow::anyhow!("Missing dimension units")),
            // otherwise, got something unexpected and should fail
            _ => Err(anyhow::anyhow!("Unknown dimension units")),
        }
    }
}
