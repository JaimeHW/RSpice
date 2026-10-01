//! Exact dark/light palettes for the reviewed Instrument visual direction.
//! Values mirror the semantic design tokens in the workbench mockup.

use egui::Color32;

const fn color_is_rgb(color: Color32, r: u8, g: u8, b: u8) -> bool {
    color.r() == r && color.g() == g && color.b() == b && color.a() == 255
}

/// One complete semantic color palette for a surface mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    /// Application background (outermost chrome).
    pub bg_app: Color32,
    /// Panel surfaces (side panels, bars, cards).
    pub bg_panel: Color32,
    /// Secondary panel treatment (table heads and section labels).
    pub bg_panel_2: Color32,
    /// Inset wells (inputs, embedded lists).
    pub bg_inset: Color32,
    /// Elevated surfaces (menus, popovers, toasts).
    pub bg_elevated: Color32,
    /// Hover fill for interactive rows/buttons.
    pub bg_hover: Color32,
    /// Pressed/active fill.
    pub bg_active: Color32,
    /// Hairline borders between surfaces.
    pub border: Color32,
    /// Emphasized borders (focused inputs, popovers).
    pub border_strong: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary text.
    pub text_dim: Color32,
    /// Tertiary text (metadata, disabled).
    pub text_faint: Color32,
    /// Accent (selection, primary actions).
    pub accent: Color32,
    /// Text/icon color on accent fills.
    pub accent_ink: Color32,
    /// Translucent accent wash (selected rows, toggles).
    pub accent_dim: Color32,
    /// Success state.
    pub ok: Color32,
    /// Warning state.
    pub warn: Color32,
    /// Error state.
    pub err: Color32,
    /// Informational state.
    pub info: Color32,
    /// Document-well background (schematic, plots, code).
    pub canvas_bg: Color32,
    /// Canvas grid dots / plot gridlines.
    pub canvas_grid: Color32,
    /// Schematic wire stroke.
    pub wire: Color32,
    /// Schematic device symbol stroke.
    pub symbol: Color32,
    /// Schematic net-name labels.
    pub net_label: Color32,
    /// Waveform trace cycle (assignment order for new traces).
    pub traces: [Color32; 6],
    /// Popover/menu shadow color (translucent).
    pub shadow_color: Color32,
    /// Popover shadow geometry: (y-offset, blur) in points.
    pub shadow_geom: (i8, u8),
}

/// Instrument · dark
pub const INSTRUMENT_DARK: Palette = Palette {
    bg_app: Color32::from_rgb(17, 23, 27),
    bg_panel: Color32::from_rgb(26, 32, 36),
    bg_panel_2: Color32::from_rgb(0x20, 0x28, 0x2d),
    bg_inset: Color32::from_rgb(11, 17, 21),
    bg_elevated: Color32::from_rgb(33, 41, 46),
    bg_hover: Color32::from_rgb(38, 46, 51),
    bg_active: Color32::from_rgb(46, 58, 65),
    border: Color32::from_rgb(43, 49, 54),
    border_strong: Color32::from_rgb(68, 76, 80),
    text: Color32::from_rgb(215, 219, 222),
    text_dim: Color32::from_rgb(147, 153, 158),
    text_faint: Color32::from_rgb(0x96, 0x9d, 0xa2),
    accent: Color32::from_rgb(242, 184, 36),
    accent_ink: Color32::from_rgb(23, 24, 26),
    accent_dim: Color32::from_rgba_premultiplied(27, 20, 4, 28),
    ok: Color32::from_rgb(101, 214, 138),
    warn: Color32::from_rgb(234, 147, 68),
    err: Color32::from_rgb(0xef, 0x80, 0x6f),
    info: Color32::from_rgb(0x7c, 0xb8, 0xe8),
    canvas_bg: Color32::from_rgb(7, 12, 16),
    canvas_grid: Color32::from_rgb(28, 35, 39),
    wire: Color32::from_rgb(127, 191, 149),
    symbol: Color32::from_rgb(203, 210, 215),
    net_label: Color32::from_rgb(220, 177, 85),
    traces: [
        Color32::from_rgb(242, 184, 36),
        Color32::from_rgb(124, 184, 232),
        Color32::from_rgb(217, 132, 176),
        Color32::from_rgb(89, 212, 131),
        Color32::from_rgb(120, 165, 239),
        Color32::from_rgb(242, 113, 106),
    ],
    shadow_color: Color32::from_rgba_premultiplied(0, 0, 0, 107),
    shadow_geom: (14, 42),
};

/// Instrument · light
pub const INSTRUMENT_LIGHT: Palette = Palette {
    bg_app: Color32::from_rgb(232, 236, 238),
    bg_panel: Color32::from_rgb(245, 247, 248),
    bg_panel_2: Color32::from_rgb(0xed, 0xf1, 0xf3),
    bg_inset: Color32::from_rgb(225, 229, 231),
    bg_elevated: Color32::from_rgb(251, 253, 254),
    bg_hover: Color32::from_rgb(230, 234, 237),
    bg_active: Color32::from_rgb(212, 220, 226),
    border: Color32::from_rgb(209, 213, 216),
    border_strong: Color32::from_rgb(166, 172, 175),
    text: Color32::from_rgb(31, 37, 40),
    text_dim: Color32::from_rgb(83, 89, 93),
    text_faint: Color32::from_rgb(0x5c, 0x63, 0x67),
    accent: Color32::from_rgb(143, 100, 0),
    accent_ink: Color32::from_rgb(252, 250, 246),
    accent_dim: Color32::from_rgba_premultiplied(13, 9, 0, 23),
    ok: Color32::from_rgb(0, 124, 58),
    warn: Color32::from_rgb(194, 110, 18),
    err: Color32::from_rgb(192, 58, 40),
    info: Color32::from_rgb(0x3d, 0x72, 0xa8),
    canvas_bg: Color32::from_rgb(251, 253, 254),
    canvas_grid: Color32::from_rgb(224, 227, 229),
    wire: Color32::from_rgb(114, 78, 0),
    symbol: Color32::from_rgb(44, 49, 53),
    net_label: Color32::from_rgb(125, 82, 0),
    traces: [
        Color32::from_rgb(161, 97, 0),
        Color32::from_rgb(61, 114, 168),
        Color32::from_rgb(168, 83, 126),
        Color32::from_rgb(0, 137, 66),
        Color32::from_rgb(48, 101, 190),
        Color32::from_rgb(189, 56, 56),
    ],
    shadow_color: Color32::from_rgba_premultiplied(7, 8, 9, 46),
    shadow_geom: (14, 42),
};

const _: [(); 1] = [(); color_is_rgb(INSTRUMENT_DARK.wire, 0x7f, 0xbf, 0x95) as usize];
const _: [(); 1] = [(); color_is_rgb(INSTRUMENT_LIGHT.wire, 0x72, 0x4e, 0x00) as usize];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instrument_wire_tokens_match_rspice_schematic_design_refs() {
        assert_eq!(INSTRUMENT_DARK.wire, Color32::from_rgb(0x7f, 0xbf, 0x95));
        assert_eq!(INSTRUMENT_LIGHT.wire, Color32::from_rgb(0x72, 0x4e, 0x00));
    }

    #[test]
    fn instrument_semantic_colors_match_the_workbench_mockup() {
        assert_eq!(
            INSTRUMENT_DARK.bg_panel_2,
            Color32::from_rgb(0x20, 0x28, 0x2d)
        );
        assert_eq!(
            INSTRUMENT_DARK.text_faint,
            Color32::from_rgb(0x96, 0x9d, 0xa2)
        );
        assert_eq!(INSTRUMENT_DARK.err, Color32::from_rgb(0xef, 0x80, 0x6f));
        assert_eq!(INSTRUMENT_DARK.info, Color32::from_rgb(0x7c, 0xb8, 0xe8));
        assert_eq!(INSTRUMENT_DARK.accent_dim.a(), 28);
        assert_eq!(
            INSTRUMENT_LIGHT.text_faint,
            Color32::from_rgb(0x5c, 0x63, 0x67)
        );
        assert_eq!(
            INSTRUMENT_LIGHT.bg_panel_2,
            Color32::from_rgb(0xed, 0xf1, 0xf3)
        );
        assert_eq!(INSTRUMENT_LIGHT.info, Color32::from_rgb(0x3d, 0x72, 0xa8));
        assert_eq!(INSTRUMENT_LIGHT.accent_dim.a(), 23);
    }
}
