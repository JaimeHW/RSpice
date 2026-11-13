//! Theme selection and egui style mapping.
//!
//! [`Theme`] is the persisted visual contract (Instrument × mode × density). Its
//! [`Theme::apply`] resolves the choice to a [`Tokens`] set, maps the tokens
//! onto egui's [`egui::Style`] so every stock widget inherits the design
//! system, and installs the tokens for custom widgets to read.

use egui::{
    Color32, Context, CornerRadius, FontFamily, FontId, Stroke, TextStyle,
    style::{ScrollFadeStyle, ScrollStyle, Selection, TextCursorStyle, WidgetVisuals, Widgets},
};
use serde::{Deserialize, Serialize};

use super::fonts;
use super::palette::{ContrastInks, INSTRUMENT_DARK, INSTRUMENT_LIGHT, Palette, high_contrast};
use super::tokens::{self, Density, Direction, Metrics, Mode, Tokens};

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

/// Whether a widget's keyboard focus should show, decided the way a browser
/// decides `:focus-visible`: focus that Tab moved there shows, and focus a
/// pointer press gave does not. For surfaces that take focus on every press so
/// their shortcuts work, such as the drawing canvases, where a ring would frame
/// the work for as long as anyone used it.
///
/// Call it every frame the widget is drawn, not only while it is focused, so it
/// sees the input that moved focus.
pub fn focus_is_visible(ctx: &Context, response: &egui::Response) -> bool {
    let tab_moved_focus = tab_moved_focus_last(ctx);
    let id = response.id.with("focus-visible");
    if !response.has_focus() {
        ctx.data_mut(|data| data.remove::<bool>(id));
        return false;
    }
    // A press on the focused widget takes the ring away again.
    let pressed_here =
        response.contains_pointer() && ctx.input(|input| input.pointer.any_pressed());
    let visible = match ctx.data(|data| data.get_temp::<bool>(id)) {
        Some(visible) => visible && !pressed_here,
        None => tab_moved_focus && !pressed_here,
    };
    ctx.data_mut(|data| data.insert_temp(id, visible));
    visible
}

/// Whether the last input that moves focus was Tab rather than a pointer press.
fn tab_moved_focus_last(ctx: &Context) -> bool {
    #[derive(Clone, Copy, Default)]
    struct Modality {
        frame: Option<u64>,
        tab: bool,
    }
    let id = egui::Id::new("rspice.focus-modality");
    let frame = ctx.cumulative_frame_nr();
    let mut modality = ctx
        .data(|data| data.get_temp::<Modality>(id))
        .unwrap_or_default();
    if modality.frame != Some(frame) {
        ctx.input(|input| {
            for event in &input.events {
                match event {
                    egui::Event::Key {
                        key: egui::Key::Tab,
                        pressed: true,
                        ..
                    } => modality.tab = true,
                    egui::Event::PointerButton { pressed: true, .. }
                    | egui::Event::Touch {
                        phase: egui::TouchPhase::Start,
                        ..
                    } => modality.tab = false,
                    _ => {}
                }
            }
        });
        modality.frame = Some(frame);
        ctx.data_mut(|data| data.insert_temp(id, modality));
    }
    modality.tab
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    /// Visual identity.
    pub direction: Direction,
    /// Dark or light surfaces.
    pub mode: Mode,
    /// Control density.
    pub density: Density,
    /// Tell traces apart by dash and marker as well as by hue (≈8 % of male
    /// engineers cannot separate red from green). Off by default: a curve is
    /// a solid line in its palette hue, as in every waveform viewer, and
    /// symbols and dashes are a style the reader opts into. Stored as
    /// `color_safe_traces`.
    #[serde(rename = "color_safe_traces")]
    pub colorblind_traces: bool,
    /// Symbol/grid separation on engineering canvases. `62` is the reviewed
    /// palette baseline used by the mockup.
    #[serde(deserialize_with = "deserialize_canvas_contrast")]
    pub canvas_contrast: u8,
    /// Independent reviewed palette for engineering document wells. This is
    /// deliberately separate from application chrome color mode.
    pub canvas_theme: EngineeringCanvasTheme,
}

const fn default_canvas_contrast() -> u8 {
    62
}

const fn default_color_safe_traces() -> bool {
    false
}

fn deserialize_canvas_contrast<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(u8::deserialize(deserializer)?.min(100))
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

fn system_contrast_id() -> egui::Id {
    egui::Id::new("rspice.theme.system-contrast")
}

impl Theme {
    /// Resolve this selection into a concrete token set.
    pub fn tokens(self) -> Tokens {
        let mut tokens = Tokens::new(self.direction, self.mode, self.density);
        tokens.color_safe_traces = self.colorblind_traces;
        // A contrast palette is whole: its canvas and traces are part of it.
        if self.mode == Mode::HighContrast {
            return tokens;
        }
        // Color-safe traces keep the canvas theme's own eight hues, each of
        // which stands off that canvas, and add a dash and a marker per slot.
        // A substitute set would have to stand 4.5:1 off both canvases, which
        // the colourblind sets' sky blue and orange do not on paper.
        let canvas_palette = self.canvas_theme.palette();
        apply_engineering_canvas_palette(&mut tokens, canvas_palette);
        apply_canvas_contrast(&mut tokens, self.canvas_contrast, canvas_palette.text);
        tokens
    }

    /// This selection's tokens on `ctx`'s host. A contrast theme the operating
    /// system is running takes precedence over any selection, as it does in
    /// every other application on the machine.
    fn tokens_on_host(self, ctx: &Context) -> Tokens {
        if let Some(inks) = Self::system_contrast(ctx) {
            let mut tokens = Tokens::new(self.direction, Mode::HighContrast, self.density);
            tokens.color = high_contrast(inks);
            tokens.color_safe_traces = self.colorblind_traces;
            tokens.mode = if inks.has_dark_ground() {
                Mode::Dark
            } else {
                Mode::Light
            };
            return tokens;
        }
        let mut effective = self;
        effective.mode = self.mode.effective(ctx);
        effective.tokens()
    }

    /// The contrast theme the operating system is running, as last reported.
    pub fn system_contrast(ctx: &Context) -> Option<ContrastInks> {
        ctx.data(|data| data.get_temp::<Option<ContrastInks>>(system_contrast_id()))
            .flatten()
    }

    /// Report the contrast theme the operating system is running.
    pub fn report_system_contrast(ctx: &Context, inks: Option<ContrastInks>) {
        ctx.data_mut(|data| data.insert_temp(system_contrast_id(), inks));
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

        let t = self.tokens_on_host(ctx);
        ctx.set_global_style(build_style(&t));
        t.install(ctx);
    }

    /// Re-resolve the active theme for the current input capability and
    /// viewport. Responsive touch targets are transient layout policy, not a
    /// persisted density preference: leaving a narrow/coarse viewport must
    /// restore the user's Compact or Relaxed metrics immediately.
    /// Apply responsive metrics with the user's validated minimum target.
    /// Device capability may increase this value but never reduce it.
    pub fn apply_responsive_metrics_with_target(self, ctx: &Context, touch_target: Option<f32>) {
        let mut t = self.tokens_on_host(ctx);
        if let Some(touch_target) = touch_target {
            let touch_target = touch_target.max(44.0);
            t.metrics = Metrics {
                ctl_h: touch_target,
                row_h: touch_target,
                bar_pad: 9.0 + (touch_target - 44.0) * 0.5,
            };
        }
        ctx.set_global_style(build_style(&t));
        t.install(ctx);
    }
}

fn apply_engineering_canvas_palette(tokens: &mut Tokens, canvas: Palette) {
    tokens.color.canvas_bg = canvas.canvas_bg;
    tokens.color.canvas_grid = canvas.canvas_grid;
    tokens.color.canvas_frame = canvas.canvas_frame;
    tokens.color.wire = canvas.wire;
    tokens.color.symbol = canvas.symbol;
    tokens.color.net_label = canvas.net_label;
    tokens.color.canvas_select = canvas.canvas_select;
    tokens.color.canvas_net_highlight = canvas.canvas_net_highlight;
    tokens.color.net_class_ground = canvas.net_class_ground;
    tokens.color.net_class_supply = canvas.net_class_supply;
    tokens.color.net_class_clock = canvas.net_class_clock;
    tokens.color.canvas_text = canvas.canvas_text;
    tokens.color.canvas_text_dim = canvas.canvas_text_dim;
    tokens.color.canvas_text_faint = canvas.canvas_text_faint;
    tokens.color.canvas_cursor_a = canvas.canvas_cursor_a;
    tokens.color.canvas_cursor_b = canvas.canvas_cursor_b;
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
            (field_text_style(), field_mono()),
        ]
        .into(),
        // A drag value is a number field: its number reads at the size every
        // other field draws its value at.
        drag_value_text_style: field_text_style(),
        ..Default::default()
    };

    // --------------------------------------------------------------- spacing
    let m = &t.metrics;
    style.spacing.item_spacing = egui::vec2(tokens::SP_4, tokens::SP_2);
    style.spacing.button_padding = egui::vec2(12.0, (m.ctl_h - 17.0).max(2.0) * 0.5);
    style.spacing.menu_margin = egui::Margin::same(5);
    style.spacing.window_margin = egui::Margin::same(tokens::SP_5 as i8);
    style.spacing.indent = 16.0;
    style.spacing.interact_size = egui::vec2(40.0_f32.max(m.ctl_h), m.ctl_h);
    style.spacing.combo_height = 260.0;
    style.spacing.icon_width = 14.0;
    style.spacing.icon_width_inner = 8.0;
    style.spacing.icon_spacing = 6.0;
    style.spacing.tooltip_width = 360.0;
    // A thin bar over its own reserved gutter: the thumb shows whenever a list
    // overflows, widens under the pointer, and its track appears only while
    // it is dragged or hovered. egui's solid bar paints its thumb in the
    // inactive widget fill, which is `bg-inset`, the track's own colour, so
    // no list ever showed that it scrolled; and its opaque track cut every
    // row fill short of the edge. The gutter keeps the solid bar's 13 points,
    // so nothing laid out beside a bar moves.
    style.spacing.scroll = ScrollStyle {
        floating: true,
        bar_width: 10.0,
        floating_width: 6.0,
        floating_allocated_width: 13.0,
        handle_min_length: 30.0,
        bar_inner_margin: 3.0,
        bar_outer_margin: 0.0,
        foreground_color: true,
        dormant_background_opacity: 0.0,
        active_background_opacity: 0.0,
        interact_background_opacity: 0.4,
        dormant_handle_opacity: 0.35,
        active_handle_opacity: 0.5,
        interact_handle_opacity: 0.75,
        // egui fades the content at a scrolled edge by default, which greys
        // the last visible row of a table or line of a log as if it were
        // disabled. The thumb is the sign that more is there.
        fade: ScrollFadeStyle {
            strength: 0.0,
            ..ScrollFadeStyle::default()
        },
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

/// What a text input or a select draws the value it holds in: the body size,
/// so every field and every choice in a form reads at one size. Names,
/// numbers, expressions and code keep the mono face.
pub fn field_mono() -> FontId {
    mono(tokens::FS_2, FontWeight::Regular)
}

/// [`field_mono`] for prose, and for a choice named in words.
pub fn field_sans() -> FontId {
    sans(tokens::FS_2, FontWeight::Regular)
}

/// The text style that resolves to [`field_mono`]: what a drag value draws
/// its number in.
pub fn field_text_style() -> TextStyle {
    TextStyle::Name("field".into())
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
    fn responsive_touch_metrics_are_transient_and_restore_user_density() {
        let ctx = Context::default();
        let theme = Theme {
            density: Density::Relaxed,
            ..Theme::default()
        };

        theme.apply_responsive_metrics_with_target(&ctx, Some(44.0));
        assert_eq!(Tokens::get(&ctx).metrics.ctl_h, 44.0);
        assert_eq!(Tokens::get(&ctx).metrics.row_h, 44.0);
        assert_eq!(ctx.global_style().spacing.interact_size.y, 44.0);
        assert_eq!(ctx.global_style().spacing.interact_size.x, 44.0);

        theme.apply_responsive_metrics_with_target(&ctx, None);
        assert_eq!(Tokens::get(&ctx).metrics.ctl_h, 32.0);
        assert_eq!(Tokens::get(&ctx).metrics.row_h, 33.0);
        assert_eq!(ctx.global_style().spacing.interact_size.y, 32.0);
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
        assert!(!resolved.color_safe_traces);
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
    fn out_of_range_canvas_contrast_is_clamped() {
        let mut stored = serde_json::to_value(Theme::default()).expect("theme serializes");
        stored["canvas_contrast"] = 255.into();
        let restored: Theme =
            serde_json::from_value(stored).expect("theme contrast is recoverable");
        assert_eq!(restored.canvas_contrast, 100);
    }

    #[test]
    fn the_canvas_theme_round_trips_independently_of_the_interface_mode() {
        let independent = Theme {
            mode: Mode::Light,
            canvas_theme: EngineeringCanvasTheme::Dark,
            ..Theme::default()
        };
        let encoded = serde_json::to_string(&independent).expect("theme serializes");
        let restored: Theme = serde_json::from_str(&encoded).expect("theme round-trips");
        assert_eq!(restored.canvas_theme, EngineeringCanvasTheme::Dark);
    }

    /// Traces are told apart by hue alone unless the reader asks for more, and
    /// the choice is stored under `color_safe_traces`.
    #[test]
    fn color_safe_traces_are_off_by_default_and_stored_under_their_key() {
        assert!(!Theme::default().colorblind_traces);
        let chosen = Theme {
            colorblind_traces: true,
            ..Theme::default()
        };
        let encoded = serde_json::to_string(&chosen).expect("theme serializes");
        assert!(encoded.contains(r#""color_safe_traces":true"#), "{encoded}");
        let restored: Theme = serde_json::from_str(&encoded).expect("theme round-trips");
        assert!(restored.tokens().color_safe_traces);
    }

    /// One frame of a canvas-like surface that, like the drawing canvases,
    /// takes focus on every press. Returns (has focus, ring visible).
    fn focus_frame(ctx: &Context, events: Vec<egui::Event>) -> (bool, bool) {
        let mut result = (false, false);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 400.0),
            )),
            events,
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| {
            let response = ui.interact(
                egui::Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(200.0, 200.0)),
                egui::Id::new("focus-visible-canvas"),
                egui::Sense::click_and_drag(),
            );
            if response.is_pointer_button_down_on() {
                response.request_focus();
            }
            result = (response.has_focus(), focus_is_visible(ui.ctx(), &response));
        });
        result
    }

    fn press(pressed: bool) -> Vec<egui::Event> {
        let at = egui::pos2(60.0, 60.0);
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            },
        ]
    }

    fn tab() -> Vec<egui::Event> {
        vec![egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]
    }

    fn settle(ctx: &Context, events: Vec<egui::Event>) -> (bool, bool) {
        focus_frame(ctx, events);
        focus_frame(ctx, Vec::new());
        focus_frame(ctx, Vec::new())
    }

    #[test]
    fn a_press_focuses_a_canvas_without_showing_its_ring() {
        let ctx = Context::default();
        focus_frame(&ctx, Vec::new());
        focus_frame(&ctx, press(true));
        assert_eq!(settle(&ctx, press(false)), (true, false));
    }

    #[test]
    fn tab_into_a_canvas_shows_its_ring_until_a_press_lands_on_it() {
        let ctx = Context::default();
        focus_frame(&ctx, Vec::new());
        assert_eq!(settle(&ctx, tab()), (true, true));

        focus_frame(&ctx, press(true));
        assert_eq!(settle(&ctx, press(false)), (true, false));
    }

    #[test]
    fn mix_endpoints() {
        let a = Color32::from_rgb(0, 0, 0);
        let b = Color32::from_rgb(255, 255, 255);
        assert_eq!(mix(a, b, 0.0), a);
        assert_eq!(mix(a, b, 1.0), b);
    }

    /// Every canvas the preferences can produce, and the contrast themes an
    /// operating system can impose.
    fn every_canvas_palette() -> Vec<(String, Palette)> {
        let mut palettes: Vec<(String, Palette)> = Vec::new();
        for mode in Mode::ALL {
            for canvas_theme in EngineeringCanvasTheme::ALL {
                for colorblind_traces in [false, true] {
                    for canvas_contrast in [0, 62, 100] {
                        let theme = Theme {
                            mode,
                            canvas_theme,
                            colorblind_traces,
                            canvas_contrast,
                            ..Theme::default()
                        };
                        palettes.push((format!("{theme:?}"), theme.tokens().color));
                    }
                }
            }
        }
        for (name, inks) in crate::palette::tests::CONTRAST_THEMES {
            palettes.push((format!("{name} contrast"), high_contrast(inks)));
        }
        palettes
    }

    /// The marks that bound a plot on a canvas — its frame, a unit circle, a
    /// Smith chart's rim — take the canvas theme's own strong border whatever
    /// the interface mode beside it: a light interface over the dark canvas
    /// drew the frame in its own light grey, which glares there.
    #[test]
    fn a_canvas_frame_is_the_canvas_themes_own() {
        for mode in [Mode::Dark, Mode::Light] {
            for canvas_theme in EngineeringCanvasTheme::ALL {
                let theme = Theme {
                    mode,
                    canvas_theme,
                    ..Theme::default()
                };
                assert_eq!(
                    theme.tokens().color.canvas_frame,
                    canvas_theme.palette().border_strong,
                    "{theme:?}"
                );
            }
        }
    }

    /// Text painted on a canvas reads off that canvas whatever the interface
    /// mode beside it: a light interface over the dark canvas painted its
    /// tick labels dark on dark.
    #[test]
    fn canvas_text_stands_off_every_canvas() {
        use crate::palette::contrast_ratio;
        for (name, c) in every_canvas_palette() {
            for (role, ink) in [
                ("text", c.canvas_text),
                ("secondary text", c.canvas_text_dim),
                ("tertiary text", c.canvas_text_faint),
            ] {
                let ratio = contrast_ratio(ink, c.canvas_bg);
                assert!(ratio >= 4.5, "{name}: canvas {role} {ratio:.2}:1");
            }
        }
    }

    /// Eight trace hues, each 4.5:1 off its canvas and 0.09 apart from every
    /// other in OKLab, and the cursors colours no trace is near. A contrast
    /// theme's cursor A is the operating system's highlight, which the
    /// palette cannot choose, so only cursor B is held there.
    #[test]
    fn traces_are_eight_distinct_hues_and_the_cursors_are_none_of_them() {
        use crate::palette::{TRACE_HUES, contrast_ratio, perceptual_distance};
        for (name, c) in every_canvas_palette() {
            if !name.ends_with("contrast") && !name.contains("HighContrast") {
                for (index, trace) in c.traces.iter().enumerate() {
                    let distance = perceptual_distance(c.canvas_cursor_a, *trace);
                    assert!(
                        distance >= 0.09,
                        "{name}: cursor A is {distance:.3} from trace {index}"
                    );
                }
            }
            assert_eq!(c.traces.len(), TRACE_HUES);
            for (index, trace) in c.traces.iter().enumerate() {
                let ratio = contrast_ratio(*trace, c.canvas_bg);
                assert!(ratio >= 4.5, "{name}: trace {index} {ratio:.2}:1");
                for (other, next) in c.traces.iter().enumerate().skip(index + 1) {
                    let distance = perceptual_distance(*trace, *next);
                    assert!(
                        distance >= 0.09,
                        "{name}: traces {index} and {other} are {distance:.3} apart"
                    );
                }
                let distance = perceptual_distance(c.canvas_cursor_b, *trace);
                assert!(
                    distance >= 0.09,
                    "{name}: cursor B is {distance:.3} from trace {index}"
                );
            }
            assert!(
                perceptual_distance(c.canvas_cursor_b, c.canvas_cursor_a) >= 0.09,
                "{name}"
            );
            for cursor in [c.canvas_cursor_a, c.canvas_cursor_b] {
                assert!(contrast_ratio(cursor, c.canvas_bg) >= 4.5, "{name}");
            }
        }
    }
}
