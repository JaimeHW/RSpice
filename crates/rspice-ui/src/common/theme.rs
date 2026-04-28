//! RSpice Theme System
//!
//! Professional theming for the egui-based UI that matches
//! commercial EDA tool aesthetics.
//!
//! # Design Principles
//!
//! 1. **Dark Mode First** - EDA engineers work long hours; dark themes reduce eye strain
//! 2. **High Contrast** - Critical circuit elements must be clearly visible
//! 3. **Consistent Colors** - Same colors for same concepts across all views
//! 4. **Accessibility** - Colorblind-friendly palettes where possible

use egui::{Color32, Rounding, Stroke, Visuals};

// =============================================================================
// Color Definitions
// =============================================================================

/// Wire and net colors
pub mod wire_colors {
    use super::Color32;

    /// Default wire color (light blue)
    pub const DEFAULT: Color32 = Color32::from_rgb(100, 180, 255);
    /// Selected wire color (bright cyan)
    pub const SELECTED: Color32 = Color32::from_rgb(0, 255, 255);
    /// Highlighted wire color (yellow)
    pub const HIGHLIGHTED: Color32 = Color32::from_rgb(255, 255, 100);
    /// Power net (red)
    pub const POWER: Color32 = Color32::from_rgb(255, 100, 100);
    /// Ground net (green)
    pub const GROUND: Color32 = Color32::from_rgb(100, 255, 100);
}

/// Component colors
pub mod component_colors {
    use super::Color32;

    /// Component body outline
    pub const OUTLINE: Color32 = Color32::from_rgb(200, 200, 200);
    /// Component body fill
    pub const FILL: Color32 = Color32::from_rgb(40, 40, 50);
    /// Selected component
    pub const SELECTED: Color32 = Color32::from_rgb(100, 200, 255);
    /// Component pin
    pub const PIN: Color32 = Color32::from_rgb(255, 200, 100);
    /// Component text/labels
    pub const TEXT: Color32 = Color32::from_rgb(220, 220, 220);
}

/// Grid colors
pub mod grid_colors {
    use super::Color32;

    /// Minor grid lines
    pub const MINOR: Color32 = Color32::from_rgba_premultiplied(60, 60, 70, 128);
    /// Major grid lines (every 10 units)
    pub const MAJOR: Color32 = Color32::from_rgba_premultiplied(80, 80, 100, 180);
    /// Origin axes
    pub const ORIGIN: Color32 = Color32::from_rgba_premultiplied(100, 100, 120, 200);
}

/// Background colors
pub mod background_colors {
    use super::Color32;

    /// Main canvas background
    pub const CANVAS: Color32 = Color32::from_rgb(25, 25, 30);
    /// Panel background
    pub const PANEL: Color32 = Color32::from_rgb(35, 35, 40);
    /// Toolbar background
    pub const TOOLBAR: Color32 = Color32::from_rgb(45, 45, 50);
    /// Status bar background
    pub const STATUS_BAR: Color32 = Color32::from_rgb(30, 30, 35);
}

/// UI element colors
pub mod ui_colors {
    use super::Color32;

    /// Primary accent color
    pub const ACCENT: Color32 = Color32::from_rgb(100, 150, 255);
    /// Success/positive
    pub const SUCCESS: Color32 = Color32::from_rgb(100, 200, 100);
    /// Warning
    pub const WARNING: Color32 = Color32::from_rgb(255, 180, 50);
    /// Error/danger
    pub const ERROR: Color32 = Color32::from_rgb(255, 80, 80);
    /// Disabled/inactive
    pub const DISABLED: Color32 = Color32::from_rgb(100, 100, 100);
    /// Text primary
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(230, 230, 230);
    /// Text secondary
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 170);
    /// Text muted
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 100, 110);
}

// =============================================================================
// Theme Configuration
// =============================================================================

/// RSpice theme configuration
#[derive(Debug, Clone)]
pub struct RSpiceTheme {
    /// Name of the theme
    pub name: String,
    /// Whether this is a dark theme
    pub is_dark: bool,
    /// Canvas background color
    pub canvas_bg: Color32,
    /// Panel background color
    pub panel_bg: Color32,
    /// Primary text color
    pub text_primary: Color32,
    /// Secondary text color
    pub text_secondary: Color32,
    /// Accent color
    pub accent: Color32,
    /// Grid minor color
    pub grid_minor: Color32,
    /// Grid major color
    pub grid_major: Color32,
    /// Wire default color
    pub wire_default: Color32,
    /// Component outline color
    pub component_outline: Color32,
}

impl Default for RSpiceTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl RSpiceTheme {
    /// Create the default dark theme (professional EDA style)
    pub fn dark() -> Self {
        Self {
            name: "RSpice Dark".to_string(),
            is_dark: true,
            canvas_bg: background_colors::CANVAS,
            panel_bg: background_colors::PANEL,
            text_primary: ui_colors::TEXT_PRIMARY,
            text_secondary: ui_colors::TEXT_SECONDARY,
            accent: ui_colors::ACCENT,
            grid_minor: grid_colors::MINOR,
            grid_major: grid_colors::MAJOR,
            wire_default: wire_colors::DEFAULT,
            component_outline: component_colors::OUTLINE,
        }
    }

    /// Create a light theme (for printing/projectors)
    pub fn light() -> Self {
        Self {
            name: "RSpice Light".to_string(),
            is_dark: false,
            canvas_bg: Color32::from_rgb(245, 245, 250),
            panel_bg: Color32::from_rgb(230, 230, 235),
            text_primary: Color32::from_rgb(30, 30, 35),
            text_secondary: Color32::from_rgb(80, 80, 90),
            accent: Color32::from_rgb(50, 100, 200),
            grid_minor: Color32::from_rgba_premultiplied(180, 180, 190, 100),
            grid_major: Color32::from_rgba_premultiplied(150, 150, 160, 150),
            wire_default: Color32::from_rgb(30, 100, 180),
            component_outline: Color32::from_rgb(50, 50, 60),
        }
    }

    /// Create a high contrast theme (accessibility)
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            is_dark: true,
            canvas_bg: Color32::BLACK,
            panel_bg: Color32::from_rgb(20, 20, 20),
            text_primary: Color32::WHITE,
            text_secondary: Color32::from_rgb(200, 200, 200),
            accent: Color32::from_rgb(255, 255, 0),
            grid_minor: Color32::from_rgba_premultiplied(80, 80, 80, 200),
            grid_major: Color32::from_rgba_premultiplied(120, 120, 120, 255),
            wire_default: Color32::from_rgb(0, 255, 255),
            component_outline: Color32::WHITE,
        }
    }

    /// Apply this theme to egui's style
    pub fn apply_to_egui(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // Configure visuals based on dark/light mode
        if self.is_dark {
            style.visuals = Visuals::dark();
        } else {
            style.visuals = Visuals::light();
        }

        // =========================================================================
        // Modern, Professional Color Scheme
        // =========================================================================

        // Define accent colors for a modern look
        let accent_primary = Color32::from_rgb(88, 166, 255); // Bright blue
        let accent_hover = Color32::from_rgb(110, 180, 255); // Lighter blue on hover
        let accent_active = Color32::from_rgb(66, 133, 244); // Google blue on click

        // Panel backgrounds with subtle depth
        let panel_bg_dark = Color32::from_rgb(28, 30, 36); // Darker panels
        let panel_bg_mid = Color32::from_rgb(35, 38, 45); // Mid panels
        let widget_bg = Color32::from_rgb(45, 48, 58); // Widget background
        let widget_hover = Color32::from_rgb(58, 62, 75); // Widget hover

        // Border and separator colors
        let border_subtle = Color32::from_rgb(50, 54, 65);
        let border_visible = Color32::from_rgb(65, 70, 85);

        // =========================================================================
        // Panel & Window Styling
        // =========================================================================
        style.visuals.panel_fill = panel_bg_dark;
        style.visuals.window_fill = panel_bg_mid;
        style.visuals.extreme_bg_color = Color32::from_rgb(20, 22, 26);
        style.visuals.faint_bg_color = Color32::from_rgb(40, 43, 52);

        // =========================================================================
        // Widget Styling - Non-interactive
        // =========================================================================
        style.visuals.widgets.noninteractive.bg_fill = panel_bg_mid;
        style.visuals.widgets.noninteractive.weak_bg_fill = panel_bg_dark;
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border_subtle);
        style.visuals.widgets.noninteractive.fg_stroke =
            Stroke::new(1.0, Color32::from_rgb(180, 185, 195));
        style.visuals.widgets.noninteractive.rounding = Rounding::same(4.0);

        // =========================================================================
        // Widget Styling - Inactive (clickable but not hovered)
        // =========================================================================
        style.visuals.widgets.inactive.bg_fill = widget_bg;
        style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(50, 53, 62);
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_subtle);
        style.visuals.widgets.inactive.fg_stroke =
            Stroke::new(1.0, Color32::from_rgb(200, 205, 215));
        style.visuals.widgets.inactive.rounding = Rounding::same(5.0);
        style.visuals.widgets.inactive.expansion = 0.0;

        // =========================================================================
        // Widget Styling - Hovered
        // =========================================================================
        style.visuals.widgets.hovered.bg_fill = widget_hover;
        style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(62, 66, 80);
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent_primary);
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        style.visuals.widgets.hovered.rounding = Rounding::same(5.0);
        style.visuals.widgets.hovered.expansion = 1.0;

        // =========================================================================
        // Widget Styling - Active (being clicked)
        // =========================================================================
        style.visuals.widgets.active.bg_fill = accent_active;
        style.visuals.widgets.active.weak_bg_fill = accent_primary;
        style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, accent_hover);
        style.visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
        style.visuals.widgets.active.rounding = Rounding::same(5.0);
        style.visuals.widgets.active.expansion = 0.0;

        // =========================================================================
        // Widget Styling - Open (dropdown open, etc.)
        // =========================================================================
        style.visuals.widgets.open.bg_fill = Color32::from_rgb(55, 58, 70);
        style.visuals.widgets.open.weak_bg_fill = widget_hover;
        style.visuals.widgets.open.bg_stroke = Stroke::new(1.0, accent_primary);
        style.visuals.widgets.open.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        style.visuals.widgets.open.rounding = Rounding::same(5.0);

        // =========================================================================
        // Selection & Highlights
        // =========================================================================
        style.visuals.selection.bg_fill = accent_primary.linear_multiply(0.4);
        style.visuals.selection.stroke = Stroke::new(1.0, accent_primary);

        // Hyperlink color
        style.visuals.hyperlink_color = accent_primary;

        // Separator color (more visible)
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border_visible);

        // =========================================================================
        // Window Styling
        // =========================================================================
        style.visuals.window_rounding = Rounding::same(8.0);
        style.visuals.window_stroke = Stroke::new(1.0, border_visible);
        style.visuals.menu_rounding = Rounding::same(6.0);
        style.visuals.popup_shadow = egui::epaint::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 12.0,
            spread: 0.0,
            color: Color32::from_black_alpha(100),
        };

        // =========================================================================
        // Spacing & Sizing
        // =========================================================================
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(12.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.menu_margin = egui::Margin::same(6.0);
        style.spacing.combo_width = 180.0;
        style.spacing.icon_width = 16.0;
        style.spacing.icon_spacing = 6.0;

        // Scrollbar styling
        style.spacing.scroll = egui::style::ScrollStyle {
            bar_width: 10.0,
            handle_min_length: 30.0,
            bar_inner_margin: 2.0,
            bar_outer_margin: 2.0,
            ..Default::default()
        };

        // =========================================================================
        // Interaction
        // =========================================================================
        style.interaction.tooltip_delay = 0.3;

        ctx.set_style(style);
    }
}

// =============================================================================
// Tests
// =============================================================================

