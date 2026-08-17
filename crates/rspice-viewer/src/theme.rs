//! Role palettes for hydrated figures.
//!
//! A hydrated pane replaces the page's static SVG in place, so its colors
//! must be byte-identical to the CSS custom properties `rspice-publish`
//! pins in the page stylesheet. The palettes below are those values; the
//! parity test parses [`rspice_publish`]'s `PAGE_STYLES` and fails the
//! build of any drift.

use egui::Color32;
use rspice_publication_contract::{Paint, PaintRole};

/// Number of distinct trace-series colors, shared with the page stylesheet.
pub const TRACE_PALETTE_SIZE: usize = 8;

/// One theme's resolved role colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub foreground: Color32,
    pub secondary: Color32,
    pub grid: Color32,
    pub accent: Color32,
    pub warning: Color32,
    pub success: Color32,
    pub traces: [Color32; TRACE_PALETTE_SIZE],
}

const fn rgb(value: u32) -> Color32 {
    Color32::from_rgb(
        ((value >> 16) & 0xff) as u8,
        ((value >> 8) & 0xff) as u8,
        (value & 0xff) as u8,
    )
}

/// Light-theme palette, equal to the page's `:root` custom properties.
pub const LIGHT: Palette = Palette {
    foreground: rgb(0x2c3135),
    secondary: rgb(0x53595d),
    grid: rgb(0xe0e3e5),
    accent: rgb(0x8f6400),
    warning: rgb(0xc26e12),
    success: rgb(0x007c3a),
    traces: [
        rgb(0xa16100),
        rgb(0x3d72a8),
        rgb(0xa8537e),
        rgb(0x008942),
        rgb(0x3065be),
        rgb(0xbd3838),
        rgb(0x6f5aa8),
        rgb(0x4f791f),
    ],
};

/// Dark-theme palette, equal to the page's dark-scheme custom properties.
pub const DARK: Palette = Palette {
    foreground: rgb(0xcbd2d7),
    secondary: rgb(0x93999e),
    grid: rgb(0x1c2327),
    accent: rgb(0xf2b824),
    warning: rgb(0xea9344),
    success: rgb(0x65d68a),
    traces: [
        rgb(0xf2b824),
        rgb(0x7cb8e8),
        rgb(0xd984b0),
        rgb(0x59d483),
        rgb(0x78a5ef),
        rgb(0xf2716a),
        rgb(0xb493e8),
        rgb(0x9fc66d),
    ],
};

impl Palette {
    /// Palette for the given egui theme.
    #[must_use]
    pub fn for_dark_mode(dark: bool) -> Self {
        if dark { DARK } else { LIGHT }
    }

    /// Resolve a contract paint selection to a concrete color.
    #[must_use]
    pub fn resolve(&self, paint: Paint) -> Color32 {
        match paint {
            Paint::Rgba([r, g, b, a]) => Color32::from_rgba_unmultiplied(r, g, b, a),
            Paint::Role(role) => match role {
                PaintRole::Foreground => self.foreground,
                PaintRole::Secondary => self.secondary,
                PaintRole::Grid => self.grid,
                PaintRole::Accent => self.accent,
                PaintRole::Warning => self.warning,
                PaintRole::Success => self.success,
                PaintRole::TraceSeries(index) => {
                    self.traces[usize::from(index) % TRACE_PALETTE_SIZE]
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Extract `--name:#rrggbb` pairs from one CSS block.
    fn css_variables(block: &str) -> Vec<(String, Color32)> {
        let mut variables = Vec::new();
        for declaration in block.split(';') {
            let Some((name, value)) = declaration.split_once(':') else {
                continue;
            };
            let name = name.trim_start_matches(|c: char| !c.is_alphanumeric() && c != '-');
            let (Some(name), value) = (name.strip_prefix("--").map(str::to_owned), value.trim())
            else {
                continue;
            };
            let Some(hex) = value.strip_prefix('#') else {
                continue;
            };
            let hex: String = hex.chars().take_while(char::is_ascii_hexdigit).collect();
            if hex.len() == 6 {
                let parsed = u32::from_str_radix(&hex, 16).expect("page palette hex");
                variables.push((name, rgb(parsed)));
            }
        }
        variables
    }

    fn palette_from(variables: &[(String, Color32)]) -> Palette {
        let lookup = |name: &str| -> Color32 {
            variables
                .iter()
                .rev()
                .find(|(candidate, _)| candidate == name)
                .unwrap_or_else(|| panic!("page styles must define --{name}"))
                .1
        };
        Palette {
            foreground: lookup("foreground"),
            secondary: lookup("secondary"),
            grid: lookup("grid"),
            accent: lookup("accent"),
            warning: lookup("warning"),
            success: lookup("success"),
            traces: core::array::from_fn(|index| lookup(&format!("trace-{index}"))),
        }
    }

    #[test]
    fn palettes_match_the_published_page_styles_exactly() {
        let styles = rspice_publish::PAGE_STYLES;
        let dark_start = styles
            .find(":root[data-theme=\"dark\"]")
            .expect("page styles must carry an explicit dark scheme");
        let (light_block, dark_block) = styles.split_at(dark_start);
        let declarations_start = dark_block
            .find('{')
            .expect("dark scheme must open its root rule");
        let dark_end = dark_block[declarations_start..]
            .find('}')
            .map(|end| declarations_start + end)
            .expect("dark scheme must close its root rule");

        assert_eq!(LIGHT, palette_from(&css_variables(light_block)));
        assert_eq!(
            DARK,
            palette_from(&css_variables(
                &dark_block[declarations_start + 1..dark_end]
            ))
        );
    }

    #[test]
    fn author_rgba_paint_survives_verbatim_in_both_themes() {
        let paint = Paint::Rgba([0x12, 0x34, 0x56, 0xff]);
        assert_eq!(
            LIGHT.resolve(paint),
            Color32::from_rgba_unmultiplied(0x12, 0x34, 0x56, 0xff)
        );
        assert_eq!(LIGHT.resolve(paint), DARK.resolve(paint));
    }

    #[test]
    fn trace_series_indices_wrap_the_shared_palette() {
        let wrapped = LIGHT.resolve(Paint::Role(PaintRole::TraceSeries(8)));
        assert_eq!(wrapped, LIGHT.traces[0]);
    }
}
