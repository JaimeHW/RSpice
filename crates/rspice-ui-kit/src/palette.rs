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
    /// Selection on the schematic canvas: a selected conductor, symbol,
    /// junction or mark, and the selection rect behind selected text. Its
    /// own hue, apart from the wire and the net highlight, rather than the
    /// interface `accent`.
    pub canvas_select: Color32,
    /// A highlighted net on the schematic canvas.
    pub canvas_net_highlight: Color32,
    /// Net-class colours: ground, supply and clock nets.
    pub net_class_ground: Color32,
    pub net_class_supply: Color32,
    pub net_class_clock: Color32,
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
    // One step below `text_dim` against a dark ground, which the mockup's
    // own token (0x96, 0x9d, 0xa2) is not: at (150, 157, 162) the tertiary
    // tone carried *more* contrast than the secondary one, so a unit caption
    // outshone the value it annotated and a placeholder read as a declared
    // value. Same hue, 21% less contrast than `text_dim` on every ground the
    // palette paints on. `text_tones_descend_against_every_ground` holds the
    // ladder.
    text_faint: Color32::from_rgb(126, 133, 138),
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
    canvas_select: Color32::from_rgb(255, 226, 92),
    // Magenta: orange beside the yellow selection collapses for deuteranopes.
    canvas_net_highlight: Color32::from_rgb(232, 110, 210),
    net_class_ground: Color32::from_rgb(0x7c, 0xb8, 0xe8),
    net_class_supply: Color32::from_rgb(234, 147, 68),
    net_class_clock: Color32::from_rgb(217, 132, 176),
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
    canvas_select: Color32::from_rgb(30, 136, 255),
    // Orange, the complement of the azure selection.
    canvas_net_highlight: Color32::from_rgb(194, 110, 18),
    net_class_ground: Color32::from_rgb(0, 124, 58),
    net_class_supply: Color32::from_rgb(194, 110, 18),
    net_class_clock: Color32::from_rgb(168, 83, 126),
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

    /// Selection, the wire and the canvas stand apart in both palettes: a
    /// selected wire 3:1 off an unselected one, both 3:1 off the canvas, and
    /// a highlighted net 3:1 off the canvas. Selection, highlight and the
    /// wire are different hues, so the state also reads for a deuteranope.
    #[test]
    fn canvas_state_colours_stand_apart() {
        for (mode, palette) in [("dark", INSTRUMENT_DARK), ("light", INSTRUMENT_LIGHT)] {
            for (pair, ink, ground) in [
                ("wire on the canvas", palette.wire, palette.canvas_bg),
                (
                    "selection on the canvas",
                    palette.canvas_select,
                    palette.canvas_bg,
                ),
                (
                    "net highlight on the canvas",
                    palette.canvas_net_highlight,
                    palette.canvas_bg,
                ),
            ] {
                let ratio = contrast(ink, ground);
                assert!(ratio >= 3.0, "{mode} {pair} is only {ratio:.2}:1");
            }
            assert_ne!(palette.canvas_select, palette.canvas_net_highlight);
            assert_ne!(palette.canvas_select, palette.accent, "{mode}");
        }
    }

    #[test]
    fn instrument_semantic_colors_match_the_workbench_mockup() {
        assert_eq!(
            INSTRUMENT_DARK.bg_panel_2,
            Color32::from_rgb(0x20, 0x28, 0x2d)
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

    /// sRGB relative luminance (WCAG 2.x).
    fn relative_luminance(color: Color32) -> f64 {
        fn linear(channel: u8) -> f64 {
            let channel = f64::from(channel) / 255.0;
            if channel <= 0.040_45 {
                channel / 12.92
            } else {
                ((channel + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * linear(color.r()) + 0.7152 * linear(color.g()) + 0.0722 * linear(color.b())
    }

    /// How far an ink stands off the ground it is painted on, as the WCAG
    /// contrast ratio. Channel values cannot answer this: on a dark ground a
    /// brighter grey stands further off, on a light ground it stands closer,
    /// which is exactly how the dark tertiary tone came to be brighter than
    /// the secondary one and nobody noticed.
    fn contrast(ink: Color32, ground: Color32) -> f64 {
        let (ink, ground) = (relative_luminance(ink), relative_luminance(ground));
        let (brighter, darker) = if ink > ground {
            (ink, ground)
        } else {
            (ground, ink)
        };
        (brighter + 0.05) / (darker + 0.05)
    }

    /// Primary, secondary, tertiary: each tone must stand off its ground less
    /// than the one above it, in both palettes and on every surface the
    /// palette paints text on.
    ///
    /// A tertiary tone is a third step or it is noise, so it must also give
    /// up a real share of the secondary tone's contrast — and still be
    /// readable, since a ladder nobody can read at the bottom is a worse
    /// defect than the inversion it replaced.
    #[test]
    fn text_tones_descend_against_every_ground() {
        for (mode, palette) in [("dark", INSTRUMENT_DARK), ("light", INSTRUMENT_LIGHT)] {
            for (surface, ground) in [
                ("bg_app", palette.bg_app),
                ("bg_panel", palette.bg_panel),
                ("bg_panel_2", palette.bg_panel_2),
                ("bg_inset", palette.bg_inset),
                ("bg_elevated", palette.bg_elevated),
            ] {
                let text = contrast(palette.text, ground);
                let dim = contrast(palette.text_dim, ground);
                let faint = contrast(palette.text_faint, ground);
                assert!(
                    dim < text,
                    "{mode} text_dim ({dim:.2}:1) must be fainter than text ({text:.2}:1) on {surface}"
                );
                assert!(
                    faint < dim,
                    "{mode} text_faint ({faint:.2}:1) must be fainter than text_dim ({dim:.2}:1) on {surface}"
                );
                assert!(
                    faint <= 0.90 * dim,
                    "{mode} text_faint ({faint:.2}:1) is not a step below text_dim ({dim:.2}:1) on {surface}, only {:.0}% of it",
                    100.0 * faint / dim
                );
                assert!(
                    faint >= 3.5,
                    "{mode} text_faint is unreadable on {surface} at {faint:.2}:1"
                );
            }
        }
    }
}
