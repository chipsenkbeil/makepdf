use crate::pdf::{PdfLuaExt, PdfLuaTableExt};
use mlua::prelude::*;
use palette::Srgb;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Color for some object in a PDF.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PdfColor(Srgb);

impl PdfColor {
    /// Produces color from RGB where each value is between 0 and 1.
    pub const fn from_rgb_f32(red: f32, green: f32, blue: f32) -> Self {
        Self(Srgb::new(red, green, blue))
    }

    /// Produces color from RGB where each value is between 0 and 255.
    pub fn from_rgb_u8(red: u8, green: u8, blue: u8) -> Self {
        Self(Srgb::new(red, green, blue).into())
    }

    /// Returns the color as (red, green, blue) float tuple.
    pub fn into_colors_f32(self) -> (f32, f32, f32) {
        (self.0.red, self.0.green, self.0.blue)
    }

    /// Returns the color as (red, green, blue) byte tuple.
    pub fn into_colors_u8(self) -> (u8, u8, u8) {
        let inner: Srgb<u8> = self.0.into();
        (inner.red, inner.green, inner.blue)
    }

    /// Returns the luminance (brightness of the color) as a value between 0 and 1.
    ///
    /// This uses weighted red/green/blue values with
    /// `L = 0.2126 * R + 0.7152 * G + 0.0722 * B`.
    #[inline]
    pub fn into_luminance(self) -> f32 {
        0.2126 * self.red + 0.7152 * self.green + 0.0722 * self.blue
    }

    /// Returns true if the color is considered light, meaning luminance is greater than `0.5`.
    #[inline]
    pub fn is_light(self) -> bool {
        self.into_luminance() > 0.5
    }

    /// Consumes the color, returning a new variant lightened by `percentage`.
    pub fn lighten(mut self, percentage: f32) -> Self {
        self.red = (self.red + (1.0 - self.red) * percentage).min(1.0);
        self.green = (self.green + (1.0 - self.green) * percentage).min(1.0);
        self.blue = (self.blue + (1.0 - self.blue) * percentage).min(1.0);
        self
    }

    /// Consumes the color, returning a new variant darkened by `percentage`.
    pub fn darken(mut self, percentage: f32) -> Self {
        self.red = (self.red * (1.0 - percentage)).max(0.0);
        self.green = (self.green * (1.0 - percentage)).max(0.0);
        self.blue = (self.blue * (1.0 - percentage)).max(0.0);
        self
    }

    /// Produces a traditional black color.
    #[inline]
    pub const fn black() -> Self {
        Self::from_rgb_f32(0.0, 0.0, 0.0)
    }

    /// Produces a traditional grey color.
    #[inline]
    pub const fn grey() -> Self {
        let c = 0.313_725_5; // == 80 / 255
        Self::from_rgb_f32(c, c, c)
    }

    /// Produces a traditional blue color.
    #[inline]
    pub const fn blue() -> Self {
        Self::from_rgb_f32(0.0, 0.0, 1.0)
    }

    /// Produces a traditional green color.
    #[inline]
    pub const fn green() -> Self {
        Self::from_rgb_f32(0.0, 1.0, 0.0)
    }

    /// Produces a traditional red color.
    #[inline]
    pub const fn red() -> Self {
        Self::from_rgb_f32(1.0, 0.0, 0.0)
    }

    /// Produces a traditional white color.
    #[inline]
    pub const fn white() -> Self {
        Self::from_rgb_f32(1.0, 1.0, 1.0)
    }
}

impl Deref for PdfColor {
    type Target = Srgb;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PdfColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for PdfColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", Srgb::<u8>::from(self.0))
    }
}

impl FromStr for PdfColor {
    type Err = palette::rgb::FromHexError;

    /// Parses a hex string into a color.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<Srgb<u8>>()?.into()))
    }
}

impl From<PdfColor> for printpdf::Color {
    /// Converts a PDF color into an RGB format.
    fn from(color: PdfColor) -> Self {
        Self::Rgb(printpdf::Rgb {
            r: color.red,
            g: color.green,
            b: color.blue,
            icc_profile: None,
        })
    }
}

impl<'lua> IntoLua<'lua> for PdfColor {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let (table, metatable) = lua.create_table_ext()?;

        // Store fields as u8, not float
        let (red, green, blue) = self.into_colors_u8();
        table.raw_set("red", red)?;
        table.raw_set("green", green)?;
        table.raw_set("blue", blue)?;

        metatable.raw_set(
            "luminance",
            lua.create_function(|_, this: PdfColor| Ok(this.into_luminance()))?,
        )?;

        metatable.raw_set(
            "is_light",
            lua.create_function(|_, this: PdfColor| Ok(this.is_light()))?,
        )?;

        metatable.raw_set(
            "lighten",
            lua.create_function(|_, (this, percentage): (PdfColor, f32)| {
                Ok(this.lighten(percentage))
            })?,
        )?;

        metatable.raw_set(
            "darken",
            lua.create_function(|_, (this, percentage): (PdfColor, f32)| {
                Ok(this.darken(percentage))
            })?,
        )?;

        // Return copy of the color as a hex string.
        metatable.raw_set(
            "__tostring",
            lua.create_function(|_, this: PdfColor| Ok(this.to_string()))?,
        )?;

        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for PdfColor {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_str()?.parse().map_err(LuaError::external)?),
            LuaValue::Table(table) => {
                let maybe_vec_u8: Option<Vec<u8>> = table
                    .clone()
                    .sequence_values()
                    .collect::<LuaResult<_>>()
                    .ok();

                // If we have color vec, check to make sure we have three, and use them as rgb
                if let Some(v) = maybe_vec_u8 {
                    if v.len() >= 3 {
                        return Ok(Self::from_rgb_u8(v[0], v[1], v[2]));
                    }
                }

                let get_field = |long_name: &str, short_name: &str| match table
                    .raw_get_ext::<_, Option<u8>>(short_name)?
                {
                    Some(value) => Ok(value),
                    None => table.raw_get_ext::<_, u8>(long_name),
                };

                // Otherwise, look for red, green, blue fields
                Ok(Self::from_rgb_u8(
                    get_field("red", "r")?,
                    get_field("green", "g")?,
                    get_field("blue", "b")?,
                ))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "pdf.common.color",
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
    fn should_be_able_to_calculate_luminance_approximation() {
        let color = PdfColor::from_rgb_u8(50, 150, 200);
        assert_eq!(color.into_luminance(), 0.51901966);
    }

    #[test]
    fn should_be_able_to_calculate_luminance_approximation_in_lua() {
        let color = PdfColor::from_rgb_u8(50, 150, 200);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                local luminance = $color:luminance()
                u.assert_deep_equal(math.floor(luminance * 1000), 519)
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_determine_if_is_light() {
        // Green is weighted highest, so high green factors to make luminance high
        let color = PdfColor::from_rgb_u8(50, 150, 200);
        assert!(color.is_light());

        // Green is weighted highest, so no green makes luminance too low
        let color = PdfColor::from_rgb_u8(50, 0, 200);
        assert!(!color.is_light());
    }

    #[test]
    fn should_be_able_to_determine_if_is_light_in_lua() {
        let light_color = PdfColor::from_rgb_u8(50, 150, 200);
        let dark_color = PdfColor::from_rgb_u8(50, 0, 200);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($light_color:is_light(), true)
                u.assert_deep_equal($dark_color:is_light(), false)
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_lighten() {
        let color = PdfColor::from_rgb_u8(50, 150, 200).lighten(0.2);
        assert_eq!(color.into_colors_u8(), (91, 171, 211));
    }

    #[test]
    fn should_be_able_to_lighten_in_lua() {
        let color = PdfColor::from_rgb_u8(50, 150, 200);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($color:lighten(0.2), {
                    red = 91,       // +20% of 205 remaining
                    green = 171,    // +20% of 105 remaining
                    blue = 211,     // +20% of 55 remaining
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_darken() {
        let color = PdfColor::from_rgb_u8(50, 150, 200).darken(0.2);
        assert_eq!(color.into_colors_u8(), (40, 120, 160));
    }

    #[test]
    fn should_be_able_to_darken_in_lua() {
        let color = PdfColor::from_rgb_u8(50, 150, 200);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($color:darken(0.2), {
                    red = 40,       // -20% of 50
                    green = 120,    // -20% of 150
                    blue = 160,     // -20% of 200
                })
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_convert_to_string_in_lua() {
        let color = PdfColor::from_rgb_u8(0, 128, 255);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal(tostring($color), "0080FF")
            })
            .exec()
            .expect("Assertion failed");
    }

    #[test]
    fn should_be_able_to_convert_from_lua() {
        let color = PdfColor::from_rgb_u8(0, 128, 255);

        // Can convert uppercase hex string into color
        assert_eq!(
            Lua::new()
                .load(chunk!("0080FF"))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert lowercase hex string into color
        assert_eq!(
            Lua::new()
                .load(chunk!("0080ff"))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert hex string starting with # into color
        assert_eq!(
            Lua::new()
                .load(chunk!("#0080ff"))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert { number, number, number } (u8) into color
        assert_eq!(
            Lua::new()
                .load(chunk!({0, 128, 255}))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert { r:number, g:number, b:number } (u8) into color
        assert_eq!(
            Lua::new()
                .load(chunk!({ r = 0, g = 128, b = 255 }))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );

        // Can convert { red:number, green:number, blue:number } (u8) into color
        assert_eq!(
            Lua::new()
                .load(chunk!({ red = 0, green = 128, blue = 255 }))
                .eval::<PdfColor>()
                .unwrap(),
            color,
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let color = PdfColor::from_rgb_u8(0, 128, 255);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($color, {
                    red = 0,
                    green = 128,
                    blue = 255,
                })
            })
            .exec()
            .expect("Assertion failed");
    }
}
