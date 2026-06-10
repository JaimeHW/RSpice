//! Theme selection and egui style mapping.
//!
//! [`Theme`] is the persisted user choice (direction × mode × density). Its
//! [`Theme::apply`] resolves the choice to a [`Tokens`] set, maps the tokens
//! onto egui's [`egui::Style`] so every stock widget inherits the design
//! system, and installs the tokens for custom widgets to read.

use egui::{
    Color32, Context, FontFamily, FontId, Rounding, Stroke, TextStyle,
    style::{ScrollStyle, Selection, TextCursorStyle, WidgetVisuals, Widgets},
};
use serde::{Deserialize, Serialize};

use super::fonts;
use super::tokens::{self, Density, Direction, Mode, Tokens};

/// The user-selected theme. Cheap to copy and compare; persisted with the
/// application state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Theme {
    /// Visual identity (Instrument / Meridian / Graphite).
    pub direction: Direction,
    /// Dark or light surfaces.
    pub mode: Mode,
    /// Control density.
    pub density: Density,
}

impl Theme {
    /// Resolve this selection into a concrete token set.
    pub fn tokens(self) -> Tokens {
        Tokens::new(self.direction, self.mode, self.density)
    }

    /// Apply the theme to an egui context: install fonts (once), map tokens
    /// onto the egui style, and publish the tokens for custom widgets.
    pub fn apply(self, ctx: &Context) {
        let fonts_installed_key = egui::Id::new("volta.fonts.installed");
        let fonts_installed: bool =
            ctx.data_mut(|d| *d.get_temp_mut_or_default(fonts_installed_key));
        if !fonts_installed {
            fonts::install(ctx);
            ctx.data_mut(|d| d.insert_temp(fonts_installed_key, true));
        }

        let t = self.tokens();
        ctx.set_style(build_style(&t));
        t.install(ctx);
    }
}

/// Map a token set onto an egui [`egui::Style`].
///
/// Token → egui mapping (from the design spec):
/// - `bg-app`            → window background of the viewport (per-panel fills)
/// - `bg-panel`          → `Visuals::panel_fill`
/// - `bg-inset`          → `Visuals::extreme_bg_color` (inputs, wells)
/// - `bg-elevated`       → `Visuals::window_fill` (dialogs, menus, popovers)
/// - `bg-hover/active`   → `widgets.hovered/.active.bg_fill`
/// - `border`            → `widgets.noninteractive.bg_stroke`
/// - `accent/accent-dim` → `Visuals::selection` + `hyperlink_color`
/// - `radius`            → widget rounding; `radius-lg` → window/menu rounding
fn build_style(t: &Tokens) -> egui::Style {
    let c = &t.color;
    let mut style = egui::Style::default();

    // ------------------------------------------------------------- typography
    style.text_styles = [
        (TextStyle::Small, FontId::new(tokens::FS_0, fonts::sans())),
        (TextStyle::Body, FontId::new(tokens::FS_2, fonts::sans())),
        (TextStyle::Button, FontId::new(tokens::FS_1, fonts::sans())),
        (
            TextStyle::Monospace,
            FontId::new(tokens::FS_1, fonts::mono()),
        ),
        (
            TextStyle::Heading,
            FontId::new(tokens::FS_3, fonts::sans_semibold()),
        ),
    ]
    .into();
    style.drag_value_text_style = TextStyle::Monospace;

    // --------------------------------------------------------------- spacing
    let m = &t.metrics;
    style.spacing.item_spacing = egui::vec2(tokens::SP_2, 4.0);
    style.spacing.button_padding = egui::vec2(12.0, (m.ctl_h - 17.0).max(2.0) * 0.5);
    style.spacing.menu_margin = egui::Margin::same(5.0);
    style.spacing.window_margin = egui::Margin::same(tokens::SP_3);
    style.spacing.indent = 16.0;
    style.spacing.interact_size = egui::vec2(40.0, m.ctl_h);
    style.spacing.combo_height = 260.0;
    style.spacing.icon_width = 14.0;
    style.spacing.icon_width_inner = 8.0;
    style.spacing.icon_spacing = 6.0;
    style.spacing.tooltip_width = 360.0;
    style.spacing.scroll = ScrollStyle {
        bar_width: 10.0,
        handle_min_length: 30.0,
        bar_inner_margin: 3.0,
        bar_outer_margin: 0.0,
        ..ScrollStyle::solid()
    };

    // --------------------------------------------------------------- visuals
    let radius = Rounding::same(t.radius);
    let visuals = &mut style.visuals;
    visuals.dark_mode = t.mode == Mode::Dark;
    visuals.override_text_color = None;

    visuals.panel_fill = c.bg_panel;
    visuals.window_fill = c.bg_elevated;
    visuals.window_stroke = Stroke::new(1.0, c.border_strong);
    visuals.window_rounding = Rounding::same(t.radius_lg);
    visuals.window_shadow = t.shadow();
    visuals.popup_shadow = t.shadow();
    visuals.menu_rounding = Rounding::same(t.radius_lg);

    visuals.extreme_bg_color = c.bg_inset;
    visuals.code_bg_color = c.bg_inset;
    visuals.faint_bg_color = c.bg_hover.gamma_multiply(0.5);

    visuals.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: c.bg_panel,
            weak_bg_fill: c.bg_panel,
            bg_stroke: Stroke::new(1.0, c.border),
            fg_stroke: Stroke::new(1.0, c.text),
            rounding: radius,
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            bg_fill: c.bg_inset,
            weak_bg_fill: c.bg_panel,
            bg_stroke: Stroke::new(1.0, c.border),
            fg_stroke: Stroke::new(1.0, c.text_dim),
            rounding: radius,
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            bg_fill: c.bg_hover,
            weak_bg_fill: c.bg_hover,
            bg_stroke: Stroke::new(1.0, c.border_strong),
            fg_stroke: Stroke::new(1.0, c.text),
            rounding: radius,
            expansion: 0.0,
        },
        active: WidgetVisuals {
            bg_fill: c.bg_active,
            weak_bg_fill: c.bg_active,
            bg_stroke: Stroke::new(1.0, c.border_strong),
            fg_stroke: Stroke::new(1.0, c.text),
            rounding: radius,
            expansion: 0.0,
        },
        open: WidgetVisuals {
            bg_fill: c.bg_active,
            weak_bg_fill: c.bg_active,
            bg_stroke: Stroke::new(1.0, c.border),
            fg_stroke: Stroke::new(1.0, c.text),
            rounding: radius,
            expansion: 0.0,
        },
    };

    visuals.selection = Selection {
        bg_fill: c.accent_dim,
        stroke: Stroke::new(1.0, c.accent),
    };
    visuals.hyperlink_color = c.accent;
    visuals.warn_fg_color = c.warn;
    visuals.error_fg_color = c.err;
    visuals.text_cursor = TextCursorStyle {
        stroke: Stroke::new(1.5, c.accent),
        ..Default::default()
    };

    visuals.striped = false;
    visuals.slider_trailing_fill = true;
    visuals.indent_has_left_vline = false;
    visuals.collapsing_header_frame = false;
    visuals.window_highlight_topmost = false;

    // ------------------------------------------------------------- animation
    style.animation_time = 0.16;

    style
}

/// Resolve a [`FontId`] for UI text at a given size and weight.
///
/// Weights follow the design spec: 400 regular, 500 medium, 600 semibold.
pub fn sans(size: f32, weight: FontWeight) -> FontId {
    FontId::new(size, weight.sans_family())
}

/// Resolve a [`FontId`] for monospace data text.
pub fn mono(size: f32, weight: FontWeight) -> FontId {
    FontId::new(size, weight.mono_family())
}

/// Font weight selection (egui has no variable axis, so weights map to
/// dedicated families).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    /// 400.
    #[default]
    Regular,
    /// 500.
    Medium,
    /// 600.
    SemiBold,
}

impl FontWeight {
    fn sans_family(self) -> FontFamily {
        match self {
            FontWeight::Regular => fonts::sans(),
            FontWeight::Medium => fonts::sans_medium(),
            FontWeight::SemiBold => fonts::sans_semibold(),
        }
    }

    fn mono_family(self) -> FontFamily {
        match self {
            FontWeight::Regular => fonts::mono(),
            // Mono ships regular + medium; semibold falls back to medium.
            FontWeight::Medium | FontWeight::SemiBold => fonts::mono_medium(),
        }
    }
}

/// Linear interpolation between two colors in gamma space (for subtle
/// hover transitions on custom widgets).
pub fn mix(a: Color32, b: Color32, k: f32) -> Color32 {
    let k = k.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 { (f32::from(x) + (f32::from(y) - f32::from(x)) * k) as u8 };
    Color32::from_rgba_premultiplied(
        lerp(a.r(), b.r()),
        lerp(a.g(), b.g()),
        lerp(a.b(), b.b()),
        lerp(a.a(), b.a()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_maps_core_tokens() {
        let theme = Theme::default();
        let t = theme.tokens();
        let style = build_style(&t);
        assert_eq!(style.visuals.panel_fill, t.color.bg_panel);
        assert_eq!(style.visuals.window_fill, t.color.bg_elevated);
        assert_eq!(style.visuals.extreme_bg_color, t.color.bg_inset);
        assert_eq!(style.visuals.selection.bg_fill, t.color.accent_dim);
        assert_eq!(style.visuals.hyperlink_color, t.color.accent);
        assert!(style.visuals.dark_mode);
    }

    #[test]
    fn light_mode_flips_dark_flag() {
        let theme = Theme {
            mode: Mode::Light,
            ..Theme::default()
        };
        let style = build_style(&theme.tokens());
        assert!(!style.visuals.dark_mode);
    }

    #[test]
    fn mix_endpoints() {
        let a = Color32::from_rgb(0, 0, 0);
        let b = Color32::from_rgb(255, 255, 255);
        assert_eq!(mix(a, b, 0.0), a);
        assert_eq!(mix(a, b, 1.0), b);
    }
}
