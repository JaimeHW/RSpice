//! Theme selection and egui style mapping.
//!
//! [`Theme`] is the persisted visual contract (Instrument × mode × density). Its
//! [`Theme::apply`] resolves the choice to a [`Tokens`] set, maps the tokens
//! onto egui's [`egui::Style`] so every stock widget inherits the design
//! system, and installs the tokens for custom widgets to read.

use egui::{
    Color32, Context, CornerRadius, FontFamily, FontId, Stroke, TextStyle,
    style::{ScrollStyle, Selection, TextCursorStyle, WidgetVisuals, Widgets},
};
use serde::{Deserialize, Serialize};

use super::fonts;
use super::palette::{INSTRUMENT_DARK, INSTRUMENT_LIGHT, Palette};
use super::tokens::{self, Density, Direction, Mode, Tokens};

/// Paint the design-system keyboard focus indicator for a custom widget.
/// Stock egui widgets draw their own focus state; painter-backed controls
/// must call this after their normal content so focus is never hidden by fill.
pub fn paint_focus_ring(ui: &egui::Ui, response: &egui::Response, rect: egui::Rect) {
    if !response.has_focus() || !ui.is_rect_visible(rect) {
        return;
    }
    let tokens = Tokens::get(ui.ctx());
    ui.painter().rect_stroke(
        rect.shrink(1.0),
        tokens.radius,
        Stroke::new(2.0, tokens.color.accent),
        egui::StrokeKind::Inside,
    );
}

/// The user-selected theme. Cheap to copy and compare; persisted with the
/// application state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringCanvasTheme {
    /// Reviewed low-luminance palette for schematic, waveform, layout, and
    /// field-view document wells.
    #[default]
    Dark,
    /// Reviewed high-luminance palette for schematic, waveform, layout, and
    /// field-view document wells.
    Light,
}

/// Paint the mockup's standard 2 px keyboard outline with a 2 px exterior gap.
/// Use the inset helper above only for dense tabular/tab-strip controls whose
/// canonical contract explicitly keeps the indicator inside their bounds.
pub fn paint_focus_ring_outset(ui: &egui::Ui, response: &egui::Response, rect: egui::Rect) {
    if !response.has_focus() || !ui.is_rect_visible(rect) {
        return;
    }
    let tokens = Tokens::get(ui.ctx());
    ui.painter().rect_stroke(
        rect.expand(4.0),
        tokens.radius + 4.0,
        Stroke::new(2.0, tokens.color.accent),
        egui::StrokeKind::Inside,
    );
}

impl EngineeringCanvasTheme {
    /// Every engineering-canvas palette that this build can genuinely render.
    pub const ALL: [Self; 2] = [Self::Dark, Self::Light];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Dark => "RSpice dark engineering",
            Self::Light => "RSpice light engineering",
        }
    }

    const fn palette(self) -> Palette {
        match self {
            Self::Dark => INSTRUMENT_DARK,
            Self::Light => INSTRUMENT_LIGHT,
        }
    }

    const fn migrate_from_interface_mode(mode: Mode) -> Self {
        match mode {
            Mode::Light => Self::Light,
            Mode::Dark | Mode::System => Self::Dark,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Theme {
    /// Versioned visual identity. Retired placeholder-shell values migrate to Instrument.
    pub direction: Direction,
    /// Dark or light surfaces.
    pub mode: Mode,
    /// Control density.
    pub density: Density,
    /// Substitute the Okabe–Ito colorblind-safe set for the trace colors
    /// (≈8 % of male engineers cannot separate the default red/green pairs).
    #[serde(default = "default_color_safe_traces")]
    pub colorblind_traces: bool,
    /// Symbol/grid separation on engineering canvases. `62` is the reviewed
    /// palette baseline used by the mockup.
    #[serde(
        default = "default_canvas_contrast",
        deserialize_with = "deserialize_canvas_contrast"
    )]
    pub canvas_contrast: u8,
    /// Independent reviewed palette for engineering document wells. This is
    /// deliberately separate from application chrome color mode.
    pub canvas_theme: EngineeringCanvasTheme,
}

const fn default_canvas_contrast() -> u8 {
    62
}

const fn default_color_safe_traces() -> bool {
    true
}

fn deserialize_canvas_contrast<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(u8::deserialize(deserializer)?.min(100))
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PersistedTheme {
            #[serde(default)]
            direction: Direction,
            #[serde(default)]
            mode: Mode,
            #[serde(default)]
            density: Density,
            #[serde(default = "default_color_safe_traces")]
            colorblind_traces: bool,
            #[serde(
                default = "default_canvas_contrast",
                deserialize_with = "deserialize_canvas_contrast"
            )]
            canvas_contrast: u8,
            #[serde(default)]
            canvas_theme: Option<EngineeringCanvasTheme>,
        }

        let persisted = PersistedTheme::deserialize(deserializer)?;
        Ok(Self {
            direction: persisted.direction,
            mode: persisted.mode,
            density: persisted.density,
            colorblind_traces: persisted.colorblind_traces,
            canvas_contrast: persisted.canvas_contrast,
            canvas_theme: persisted.canvas_theme.unwrap_or_else(|| {
                EngineeringCanvasTheme::migrate_from_interface_mode(persisted.mode)
            }),
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            direction: Direction::default(),
            mode: Mode::default(),
            density: Density::default(),
            colorblind_traces: default_color_safe_traces(),
            canvas_contrast: default_canvas_contrast(),
            canvas_theme: EngineeringCanvasTheme::default(),
        }
    }
}

/// The Okabe–Ito colorblind-safe categorical set (less yellow/black/grey,
/// which fail on one of the two surface modes).
const OKABE_ITO_TRACES: [Color32; 6] = [
    Color32::from_rgb(0x00, 0x72, 0xB2), // blue
    Color32::from_rgb(0xE6, 0x9F, 0x00), // orange
    Color32::from_rgb(0x00, 0x9E, 0x73), // bluish green
    Color32::from_rgb(0xD5, 0x5E, 0x00), // vermillion
    Color32::from_rgb(0x56, 0xB4, 0xE9), // sky blue
    Color32::from_rgb(0xCC, 0x79, 0xA7), // reddish purple
];

impl Theme {
    /// Resolve this selection into a concrete token set.
    pub fn tokens(self) -> Tokens {
        let mut tokens = Tokens::new(self.direction, self.mode, self.density);
        let canvas_palette = self.canvas_theme.palette();
        apply_engineering_canvas_palette(&mut tokens, canvas_palette);
        tokens.color_safe_traces = self.colorblind_traces;
        if self.colorblind_traces {
            tokens.color.traces = OKABE_ITO_TRACES;
        }
        apply_canvas_contrast(&mut tokens, self.canvas_contrast, canvas_palette.text);
        tokens
    }

    /// Apply the theme to an egui context: install fonts (once), map tokens
    /// onto the egui style, and publish the tokens for custom widgets.
    pub fn apply(self, ctx: &Context) {
        let fonts_installed_key = egui::Id::new("rspice.fonts.installed");
        let fonts_installed: bool =
            ctx.data_mut(|d| *d.get_temp_mut_or_default(fonts_installed_key));
        if !fonts_installed {
            fonts::install(ctx);
            ctx.data_mut(|d| d.insert_temp(fonts_installed_key, true));
        }

        let mut effective = self;
        effective.mode = self.mode.effective(ctx);
        let t = effective.tokens();
        ctx.set_style(build_style(&t));
        t.install(ctx);
    }
}

fn apply_engineering_canvas_palette(tokens: &mut Tokens, canvas: Palette) {
    tokens.color.canvas_bg = canvas.canvas_bg;
    tokens.color.canvas_grid = canvas.canvas_grid;
    tokens.color.wire = canvas.wire;
    tokens.color.symbol = canvas.symbol;
    tokens.color.net_label = canvas.net_label;
    tokens.color.traces = canvas.traces;
}

fn apply_canvas_contrast(tokens: &mut Tokens, preference: u8, canvas_text: Color32) {
    const BASELINE: f32 = 62.0;
    let contrast = f32::from(preference.min(100));
    if contrast < BASELINE {
        let amount = contrast / BASELINE;
        tokens.color.canvas_grid = mix(tokens.color.canvas_bg, tokens.color.canvas_grid, amount);
        // Symbols remain legible at the low end while still responding to the
        // same user control as the grid.
        tokens.color.symbol = mix(
            tokens.color.canvas_bg,
            tokens.color.symbol,
            0.35 + amount * 0.65,
        );
    } else if contrast > BASELINE {
        let amount = (contrast - BASELINE) / (100.0 - BASELINE);
        tokens.color.canvas_grid = mix(tokens.color.canvas_grid, canvas_text, amount * 0.45);
        tokens.color.symbol = mix(tokens.color.symbol, canvas_text, amount * 0.35);
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

    // ------------------------------------------------------------- typography
    let mut style = egui::Style {
        text_styles: [
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
        .into(),
        drag_value_text_style: TextStyle::Monospace,
        ..Default::default()
    };

    // --------------------------------------------------------------- spacing
    let m = &t.metrics;
    style.spacing.item_spacing = egui::vec2(tokens::SP_4, tokens::SP_2);
    style.spacing.button_padding = egui::vec2(12.0, (m.ctl_h - 17.0).max(2.0) * 0.5);
    style.spacing.menu_margin = egui::Margin::same(5);
    style.spacing.window_margin = egui::Margin::same(tokens::SP_5 as i8);
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
    let radius = corner_radius(t.radius);
    let visuals = &mut style.visuals;
    visuals.dark_mode = t.mode == Mode::Dark;
    visuals.override_text_color = None;

    visuals.panel_fill = c.bg_panel;
    visuals.window_fill = c.bg_elevated;
    visuals.window_stroke = Stroke::new(1.0, c.border_strong);
    visuals.window_corner_radius = corner_radius(t.radius_lg);
    visuals.window_shadow = t.shadow();
    visuals.popup_shadow = t.shadow();
    visuals.menu_corner_radius = corner_radius(t.radius_lg);

    visuals.extreme_bg_color = c.bg_inset;
    visuals.code_bg_color = c.bg_inset;
    visuals.faint_bg_color = c.bg_hover.gamma_multiply(0.5);

    visuals.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: c.bg_panel,
            weak_bg_fill: c.bg_panel,
            bg_stroke: Stroke::new(1.0, c.border),
            fg_stroke: Stroke::new(1.0, c.text),
            corner_radius: radius,
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            bg_fill: c.bg_inset,
            weak_bg_fill: c.bg_panel,
            bg_stroke: Stroke::new(1.0, c.border),
            fg_stroke: Stroke::new(1.0, c.text_dim),
            corner_radius: radius,
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            bg_fill: c.bg_hover,
            weak_bg_fill: c.bg_hover,
            bg_stroke: Stroke::new(1.0, c.border_strong),
            fg_stroke: Stroke::new(1.0, c.text),
            corner_radius: radius,
            expansion: 0.0,
        },
        active: WidgetVisuals {
            bg_fill: c.bg_active,
            weak_bg_fill: c.bg_active,
            bg_stroke: Stroke::new(1.0, c.border_strong),
            fg_stroke: Stroke::new(1.0, c.text),
            corner_radius: radius,
            expansion: 0.0,
        },
        open: WidgetVisuals {
            bg_fill: c.bg_active,
            weak_bg_fill: c.bg_active,
            bg_stroke: Stroke::new(1.0, c.border),
            fg_stroke: Stroke::new(1.0, c.text),
            corner_radius: radius,
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

fn corner_radius(radius: f32) -> CornerRadius {
    CornerRadius::same(radius.round().clamp(0.0, u8::MAX as f32) as u8)
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
    fn canvas_contrast_baseline_preserves_reviewed_palette() {
        let baseline = Tokens::new(Direction::Instrument, Mode::Dark, Density::Compact);
        let resolved = Theme::default().tokens();

        assert_eq!(resolved.color.canvas_grid, baseline.color.canvas_grid);
        assert_eq!(resolved.color.symbol, baseline.color.symbol);
        assert!(resolved.color_safe_traces);
    }

    #[test]
    fn engineering_canvas_theme_labels_match_the_mockup_supported_choices() {
        assert_eq!(
            EngineeringCanvasTheme::ALL.map(EngineeringCanvasTheme::label),
            ["RSpice dark engineering", "RSpice light engineering"]
        );
    }

    #[test]
    fn engineering_canvas_theme_changes_document_wells_without_recoloring_chrome() {
        let resolved = Theme {
            mode: Mode::Dark,
            canvas_theme: EngineeringCanvasTheme::Light,
            colorblind_traces: false,
            canvas_contrast: 62,
            ..Theme::default()
        }
        .tokens();

        assert_eq!(resolved.color.bg_app, INSTRUMENT_DARK.bg_app);
        assert_eq!(resolved.color.bg_panel, INSTRUMENT_DARK.bg_panel);
        assert_eq!(resolved.color.canvas_bg, INSTRUMENT_LIGHT.canvas_bg);
        assert_eq!(resolved.color.canvas_grid, INSTRUMENT_LIGHT.canvas_grid);
        assert_eq!(resolved.color.wire, INSTRUMENT_LIGHT.wire);
        assert_eq!(resolved.color.symbol, INSTRUMENT_LIGHT.symbol);
        assert_eq!(resolved.color.net_label, INSTRUMENT_LIGHT.net_label);
        assert_eq!(resolved.color.traces, INSTRUMENT_LIGHT.traces);
    }

    #[test]
    fn canvas_contrast_is_consumed_by_grid_and_symbol_render_tokens() {
        let baseline = Tokens::new(Direction::Instrument, Mode::Dark, Density::Compact);
        let low = Theme {
            canvas_contrast: 0,
            ..Theme::default()
        }
        .tokens();
        let high = Theme {
            canvas_contrast: 100,
            ..Theme::default()
        }
        .tokens();

        assert_eq!(low.color.canvas_grid, low.color.canvas_bg);
        assert_ne!(low.color.symbol, baseline.color.symbol);
        assert_ne!(high.color.canvas_grid, baseline.color.canvas_grid);
        assert_ne!(high.color.symbol, baseline.color.symbol);
    }

    #[test]
    fn system_mode_and_canvas_contrast_round_trip_as_user_preferences() {
        let theme = Theme {
            mode: Mode::System,
            canvas_contrast: 87,
            ..Theme::default()
        };

        let json = serde_json::to_string(&theme).expect("theme serializes");
        let restored: Theme = serde_json::from_str(&json).expect("theme deserializes");

        assert_eq!(restored, theme);
    }

    #[test]
    fn legacy_and_out_of_range_canvas_contrast_are_normalized() {
        let legacy = r#"{"direction":"Instrument","mode":"Dark","density":"Compact","colorblind_traces":false}"#;
        let restored: Theme = serde_json::from_str(legacy).expect("legacy theme deserializes");
        assert_eq!(restored.canvas_contrast, 62);

        let out_of_range = r#"{"direction":"Instrument","mode":"Dark","density":"Compact","colorblind_traces":false,"canvas_contrast":255}"#;
        let restored: Theme =
            serde_json::from_str(out_of_range).expect("theme contrast is recoverable");
        assert_eq!(restored.canvas_contrast, 100);
    }

    #[test]
    fn legacy_theme_migrates_canvas_palette_from_its_interface_mode() {
        let legacy_light = r#"{"direction":"Instrument","mode":"Light","density":"Compact"}"#;
        let restored: Theme =
            serde_json::from_str(legacy_light).expect("legacy light theme deserializes");
        assert_eq!(restored.canvas_theme, EngineeringCanvasTheme::Light);

        let legacy_dark = r#"{"direction":"Instrument","mode":"Dark","density":"Compact"}"#;
        let restored: Theme =
            serde_json::from_str(legacy_dark).expect("legacy dark theme deserializes");
        assert_eq!(restored.canvas_theme, EngineeringCanvasTheme::Dark);

        let independent = Theme {
            mode: Mode::Light,
            canvas_theme: EngineeringCanvasTheme::Dark,
            ..Theme::default()
        };
        let encoded = serde_json::to_string(&independent).expect("theme serializes");
        let restored: Theme = serde_json::from_str(&encoded).expect("theme round-trips");
        assert_eq!(restored.canvas_theme, EngineeringCanvasTheme::Dark);
    }

    #[test]
    fn legacy_theme_defaults_to_the_mockup_color_safe_trace_policy() {
        let legacy = r#"{"direction":"Instrument","mode":"Dark","density":"Compact"}"#;
        let restored: Theme = serde_json::from_str(legacy).expect("legacy theme deserializes");

        assert!(restored.colorblind_traces);
        assert!(restored.tokens().color_safe_traces);
    }

    #[test]
    fn mix_endpoints() {
        let a = Color32::from_rgb(0, 0, 0);
        let b = Color32::from_rgb(255, 255, 255);
        assert_eq!(mix(a, b, 0.0), a);
        assert_eq!(mix(a, b, 1.0), b);
    }
}
