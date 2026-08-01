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

/// Configuration for SVG export.
#[derive(Debug, Clone)]
pub struct SvgExportConfig {
    /// Grid size in SVG units
    pub grid_size: f64,
    /// Stroke width for wires
    pub wire_stroke_width: f64,
    /// Stroke width for components
    pub component_stroke_width: f64,
    /// Wire color
    pub wire_color: SvgColor,
    /// Component color
    pub component_color: SvgColor,
    /// Text color
    pub text_color: SvgColor,
    /// Page background color. `None` preserves a transparent SVG page.
    pub background_color: Option<SvgColor>,
    /// Font size for labels
    pub font_size: f64,
    /// Margin around content
    pub margin: f64,
}

impl Default for SvgExportConfig {
    fn default() -> Self {
        Self {
            grid_size: 10.0,
            wire_stroke_width: 2.0,
            component_stroke_width: 2.0,
            wire_color: SvgColor::rgb(0x00, 0xFF, 0x00),
            component_color: SvgColor::rgb(0xFF, 0xFF, 0xFF),
            text_color: SvgColor::rgb(0xAA, 0xAA, 0xAA),
            background_color: Some(SvgColor::rgb(0x1A, 0x1A, 0x1A)),
            font_size: 12.0,
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
}
