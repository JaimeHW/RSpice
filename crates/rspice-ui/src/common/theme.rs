//! Canvas Color Bridge
//!
//! `RSpiceTheme` is the legacy color surface the schematic canvas renderers
//! still read (`state.theme.*`). The VOLTA design system
//! (`crate::ui::tokens`) is the single source of truth: the bridge is
//! re-projected from the active tokens whenever the theme changes, so every
//! canvas renderer follows the selected direction/mode.
//!
//! Slated for retirement: canvas painters will read tokens directly.

use egui::Color32;

/// Canvas color bridge derived from the active design tokens.
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
        Self::from_tokens(&crate::ui::tokens::Tokens::default())
    }
}

impl RSpiceTheme {
    /// Derive the canvas color bridge from the active design tokens.
    pub fn from_tokens(tokens: &crate::ui::tokens::Tokens) -> Self {
        let c = &tokens.color;
        Self {
            name: format!("{} {}", tokens.direction.label(), tokens.mode.label()),
            is_dark: tokens.mode == crate::ui::tokens::Mode::Dark,
            canvas_bg: c.canvas_bg,
            panel_bg: c.bg_panel,
            text_primary: c.text,
            text_secondary: c.text_dim,
            accent: c.accent,
            grid_minor: c.canvas_grid,
            grid_major: c.canvas_grid,
            wire_default: c.wire,
            component_outline: c.symbol,
        }
    }
}
