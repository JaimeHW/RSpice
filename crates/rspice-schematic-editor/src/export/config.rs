//! SVG export configuration.
//!
//! Colors, stroke widths, and page setup for schematic SVG output.

use std::{error::Error, fmt};

/// A validated sRGB color that can be serialized into an SVG attribute
/// without accepting arbitrary XML/CSS text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SvgColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl SvgColor {
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: u8::MAX,
        }
    }

    #[must_use]
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl fmt::Display for SvgColor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alpha == u8::MAX {
            write!(
                formatter,
                "#{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue
            )
        } else {
            write!(
                formatter,
                "#{:02X}{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue, self.alpha
            )
        }
    }
}

impl TryFrom<&str> for SvgColor {
    type Error = SvgColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let digits = value
            .strip_prefix('#')
            .ok_or(SvgColorParseError::MissingHash)?;
        if !matches!(digits.len(), 6 | 8) {
            return Err(SvgColorParseError::InvalidLength(digits.len()));
        }
        if !digits.as_bytes().iter().all(u8::is_ascii_hexdigit) {
            return Err(SvgColorParseError::InvalidHexDigit);
        }

        let channel = |offset| {
            u8::from_str_radix(&digits[offset..offset + 2], 16)
                .map_err(|_| SvgColorParseError::InvalidHexDigit)
        };
        Ok(Self::rgba(
            channel(0)?,
            channel(2)?,
            channel(4)?,
            if digits.len() == 8 {
                channel(6)?
            } else {
                u8::MAX
            },
        ))
    }
}

impl TryFrom<String> for SvgColor {
    type Error = SvgColorParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvgColorParseError {
    MissingHash,
    InvalidLength(usize),
    InvalidHexDigit,
}

impl fmt::Display for SvgColorParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHash => formatter.write_str("SVG colors must begin with '#'"),
            Self::InvalidLength(length) => write!(
                formatter,
                "SVG colors require exactly 6 or 8 hexadecimal digits, found {length}"
            ),
            Self::InvalidHexDigit => {
                formatter.write_str("SVG colors contain a non-hexadecimal digit")
            }
        }
    }
}

impl Error for SvgColorParseError {}

impl From<egui::Color32> for SvgColor {
    fn from(color: egui::Color32) -> Self {
        Self::rgba(color.r(), color.g(), color.b(), color.a())
    }
}

/// Configuration for SVG export.
///
/// An exported sheet is paper: one light print palette whatever the screen
/// theme, in the canvas's nominal stroke widths and world-unit type, all
/// scaled by `units_per_world`.
#[derive(Debug, Clone)]
pub struct SvgExportConfig {
    /// SVG units per schematic world unit.
    pub units_per_world: f64,
    /// Wires, buses, taps and junction dots.
    pub wire_color: SvgColor,
    /// Symbol strokes, pin leads and symbol body text.
    pub symbol_color: SvgColor,
    /// Instance names.
    pub name_color: SvgColor,
    /// Values, pin names, notes, documentation shapes and probes.
    pub text_color: SvgColor,
    /// Net labels, connectors, bus declarations, tap slices and attachment
    /// diamonds.
    pub net_label_color: SvgColor,
    /// Page background color. `None` preserves a transparent SVG page.
    pub background_color: Option<SvgColor>,
    /// Margin around content
    pub margin: f64,
}

impl Default for SvgExportConfig {
    /// The light screen tokens on white paper.
    fn default() -> Self {
        let ink = rspice_ui_kit::palette::INSTRUMENT_LIGHT;
        Self {
            units_per_world: 10.0,
            wire_color: ink.wire.into(),
            symbol_color: ink.symbol.into(),
            name_color: ink.text.into(),
            text_color: ink.text_dim.into(),
            net_label_color: ink.net_label.into(),
            background_color: Some(SvgColor::rgb(0xFF, 0xFF, 0xFF)),
            margin: 50.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_color_accepts_only_canonical_hex_srgb() {
        assert_eq!(
            SvgColor::try_from("#1a2B3c").unwrap(),
            SvgColor::rgb(0x1A, 0x2B, 0x3C)
        );
        assert_eq!(
            SvgColor::try_from("#10203040").unwrap(),
            SvgColor::rgba(0x10, 0x20, 0x30, 0x40)
        );
        assert_eq!(SvgColor::rgb(0x01, 0xA2, 0x0F).to_string(), "#01A20F");

        assert_eq!(
            SvgColor::try_from("red").unwrap_err(),
            SvgColorParseError::MissingHash
        );
        assert!(matches!(
            SvgColor::try_from("#fff\" onload=\"alert(1)").unwrap_err(),
            SvgColorParseError::InvalidLength(_)
        ));
        assert_eq!(
            SvgColor::try_from("#12GG00").unwrap_err(),
            SvgColorParseError::InvalidHexDigit
        );
    }

    /// C-10: the default export is the print palette of D1 section 11, the
    /// light screen tokens on white, not neon on near-black.
    #[test]
    fn the_default_export_inks_are_the_light_tokens_on_white() {
        let config = SvgExportConfig::default();
        for (ink, expected) in [
            (config.wire_color, "#4E3400"),
            (config.symbol_color, "#2C3135"),
            (config.name_color, "#1F2528"),
            (config.text_color, "#53595D"),
            (config.net_label_color, "#7D5200"),
        ] {
            assert_eq!(ink.to_string(), expected);
        }
        assert_eq!(
            config.background_color,
            Some(SvgColor::rgb(0xFF, 0xFF, 0xFF))
        );
        assert_eq!(config.units_per_world, 10.0);
    }
}
