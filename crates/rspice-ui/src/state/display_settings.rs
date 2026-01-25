//! Schematic Display Settings
//!
//! Centralized configuration for schematic rendering and label display.
//! Follows the pattern of separating display preferences
//! from schematic data, enabling per-session or per-project customization.

use serde::{Deserialize, Serialize};

/// Display settings for schematic components and labels.
///
/// These settings control the visual presentation without affecting
/// the underlying circuit data. Simulators
/// separate these concerns to allow flexible visualization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchematicDisplaySettings {
    // =========================================================================
    // Label Visibility
    // =========================================================================
    /// Show component reference designators (R1, C2, M1, etc.)
    pub show_component_names: bool,

    /// Show component values (1k, 10uF, etc.)
    pub show_component_values: bool,

    /// Show terminal pin names on components (B, C, E for BJT; G, D, S for MOSFET)
    pub show_pin_names: PinNameVisibility,

    /// Show net names on wires
    pub show_net_names: bool,

    // =========================================================================
    // Label Typography
    // =========================================================================
    /// Font size for component names (reference designators)
    pub name_font_size: f32,

    /// Font size for component values
    pub value_font_size: f32,

    /// Font size for terminal pin names
    pub pin_font_size: f32,

    /// Font weight for component names (400 = normal, 600 = semibold, 700 = bold)
    pub name_font_weight: u16,

    // =========================================================================
    // Label Rendering
    // =========================================================================
    /// Use drop shadow on labels for improved contrast
    pub label_shadow_enabled: bool,

    /// Maximum number of parameter lines to display below value
    pub max_param_lines: u8,

    // =========================================================================
    // Component Display
    // =========================================================================
    /// Show selection highlight ring around selected components
    pub show_selection_ring: bool,

    /// Show component bounding boxes (debug/alignment aid)
    pub show_bounding_boxes: bool,

    // =========================================================================
    // Grid Display
    // =========================================================================
    /// Background grid style in schematic editor
    /// Commercial simulators (Cadence Virtuoso) offer Lines, Dots, or Hidden
    pub grid_style: GridStyle,
}

/// Pin name visibility modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PinNameVisibility {
    /// Never show pin names (Default - clean schematic)
    #[default]
    Hidden,
    /// Show pin names only on hover
    OnHover,
    /// Always show pin names (toggle via View menu)
    Always,
}

/// Grid display style modes.
///
/// Commercial EDA tools like Cadence Virtuoso offer multiple grid display options.
/// The default is Lines for maximum visibility during schematic capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GridStyle {
    /// Major and minor grid lines (Default - best for alignment)
    #[default]
    Lines,
    /// Dot grid at intersection points (reduced visual clutter)
    Dots,
    /// No grid displayed (clean view for presentations/printing)
    Hidden,
}

impl GridStyle {
    /// Cycle to the next grid style.
    ///
    /// Order: Lines → Dots → Hidden → Lines
    /// This matches the typical EDA workflow where users toggle through options.
    #[must_use]
    pub fn cycle(self) -> Self {
        match self {
            GridStyle::Lines => GridStyle::Dots,
            GridStyle::Dots => GridStyle::Hidden,
            GridStyle::Hidden => GridStyle::Lines,
        }
    }

    /// Get the display icon for this grid style.
    ///
    /// Icons are chosen to be visually representative:
    /// - Lines: grid symbol
    /// - Dots: dot symbol
    /// - Hidden: empty box
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            GridStyle::Lines => "⊞",
            GridStyle::Dots => "⦁",
            GridStyle::Hidden => "◻",
        }
    }

    /// Get the display label for this grid style.
    ///
    /// Labels are user-friendly descriptions for UI buttons.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            GridStyle::Lines => "Lines",
            GridStyle::Dots => "Dots",
            GridStyle::Hidden => "Off",
        }
    }

    /// Get the button text combining icon and label.
    ///
    /// Format: "icon label" for toolbar display.
    #[must_use]
    pub fn button_text(self) -> String {
        format!("{} {}", self.icon(), self.label())
    }

    /// Check if this style renders any grid pattern.
    #[must_use]
    pub fn is_visible(self) -> bool {
        !matches!(self, GridStyle::Hidden)
    }

    /// Get all available grid styles in cycle order.
    #[must_use]
    pub fn all() -> [GridStyle; 3] {
        [GridStyle::Lines, GridStyle::Dots, GridStyle::Hidden]
    }
}

impl Default for SchematicDisplaySettings {
    fn default() -> Self {
        Self {
            // Label visibility - sensible defaults
            show_component_names: true,
            show_component_values: true,
            show_pin_names: PinNameVisibility::Hidden,
            show_net_names: false,

            // Typography - match existing CompSvg defaults
            name_font_size: 10.0,
            value_font_size: 9.0,
            pin_font_size: 7.0,
            name_font_weight: 600,

            // Rendering features
            label_shadow_enabled: true,
            max_param_lines: 2,

            // Component display
            show_selection_ring: true,
            show_bounding_boxes: false,

            // Grid display - Lines is the commercial standard default
            grid_style: GridStyle::Lines,
        }
    }
}

impl SchematicDisplaySettings {
    /// Create settings optimized for dense schematics (smaller labels)
    pub fn dense() -> Self {
        Self {
            name_font_size: 8.0,
            value_font_size: 7.0,
            pin_font_size: 6.0,
            max_param_lines: 1,
            ..Default::default()
        }
    }

    /// Create settings for presentation/printing (larger labels)
    pub fn presentation() -> Self {
        Self {
            name_font_size: 12.0,
            value_font_size: 11.0,
            pin_font_size: 9.0,
            label_shadow_enabled: false, // Shadows don't print well
            ..Default::default()
        }
    }

    /// Get CSS font-weight string for name labels
    pub fn name_font_weight_css(&self) -> &'static str {
        match self.name_font_weight {
            w if w >= 700 => "bold",
            w if w >= 600 => "600",
            w if w >= 500 => "500",
            _ => "normal",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = SchematicDisplaySettings::default();
        assert!(settings.show_component_names);
        assert!(settings.show_component_values);
        assert_eq!(settings.show_pin_names, PinNameVisibility::Hidden);
        assert_eq!(settings.name_font_size, 10.0);
    }

    #[test]
    fn test_dense_preset() {
        let settings = SchematicDisplaySettings::dense();
        assert!(settings.name_font_size < SchematicDisplaySettings::default().name_font_size);
    }

    #[test]
    fn test_presentation_preset() {
        let settings = SchematicDisplaySettings::presentation();
        assert!(settings.name_font_size > SchematicDisplaySettings::default().name_font_size);
        assert!(!settings.label_shadow_enabled);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let settings = SchematicDisplaySettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: SchematicDisplaySettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, parsed);
    }

    // =========================================================================
    // GridStyle Enum Tests
    // =========================================================================

    #[test]
    fn test_grid_style_default_is_lines() {
        // Commercial simulators like Cadence show grid lines by default
        // for maximum visibility during schematic capture
        assert_eq!(GridStyle::default(), GridStyle::Lines);
    }

    #[test]
    fn test_grid_style_in_settings_default() {
        let settings = SchematicDisplaySettings::default();
        assert_eq!(
            settings.grid_style,
            GridStyle::Lines,
            "Default grid style should be Lines for alignment"
        );
    }

    #[test]
    fn test_grid_style_cycle_lines_to_dots() {
        assert_eq!(GridStyle::Lines.cycle(), GridStyle::Dots);
    }

    #[test]
    fn test_grid_style_cycle_dots_to_hidden() {
        assert_eq!(GridStyle::Dots.cycle(), GridStyle::Hidden);
    }

    #[test]
    fn test_grid_style_cycle_hidden_to_lines() {
        assert_eq!(GridStyle::Hidden.cycle(), GridStyle::Lines);
    }

    #[test]
    fn test_grid_style_cycle_full_rotation() {
        // Complete cycle should return to original
        let start = GridStyle::Lines;
        let after_one = start.cycle();
        let after_two = after_one.cycle();
        let after_three = after_two.cycle();
        assert_eq!(after_three, start, "Full cycle should return to Lines");
    }

    #[test]
    fn test_grid_style_cycle_all_variants() {
        // Verify each variant cycles correctly
        for style in GridStyle::all() {
            let next = style.cycle();
            assert_ne!(style, next, "cycle() should return different variant");
        }
    }

    #[test]
    fn test_grid_style_icon_lines() {
        assert_eq!(GridStyle::Lines.icon(), "⊞");
    }

    #[test]
    fn test_grid_style_icon_dots() {
        assert_eq!(GridStyle::Dots.icon(), "⦁");
    }

    #[test]
    fn test_grid_style_icon_hidden() {
        assert_eq!(GridStyle::Hidden.icon(), "◻");
    }

    #[test]
    fn test_grid_style_icons_are_unique() {
        let icons: Vec<&str> = GridStyle::all().iter().map(|s| s.icon()).collect();
        assert_eq!(icons.len(), 3);
        assert!(icons[0] != icons[1] && icons[1] != icons[2] && icons[0] != icons[2]);
    }

    #[test]
    fn test_grid_style_label_lines() {
        assert_eq!(GridStyle::Lines.label(), "Lines");
    }

    #[test]
    fn test_grid_style_label_dots() {
        assert_eq!(GridStyle::Dots.label(), "Dots");
    }

    #[test]
    fn test_grid_style_label_hidden() {
        assert_eq!(GridStyle::Hidden.label(), "Off");
    }

    #[test]
    fn test_grid_style_button_text_format() {
        // Button text should be "icon label"
        assert!(GridStyle::Lines.button_text().contains("⊞"));
        assert!(GridStyle::Lines.button_text().contains("Lines"));
    }

    #[test]
    fn test_grid_style_button_text_all_variants() {
        for style in GridStyle::all() {
            let text = style.button_text();
            assert!(text.contains(style.icon()));
            assert!(text.contains(style.label()));
        }
    }

    #[test]
    fn test_grid_style_is_visible_lines() {
        assert!(GridStyle::Lines.is_visible());
    }

    #[test]
    fn test_grid_style_is_visible_dots() {
        assert!(GridStyle::Dots.is_visible());
    }

    #[test]
    fn test_grid_style_is_visible_hidden() {
        assert!(!GridStyle::Hidden.is_visible());
    }

    #[test]
    fn test_grid_style_all_returns_three_variants() {
        let all = GridStyle::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_grid_style_all_in_cycle_order() {
        let all = GridStyle::all();
        assert_eq!(all[0], GridStyle::Lines);
        assert_eq!(all[1], GridStyle::Dots);
        assert_eq!(all[2], GridStyle::Hidden);
    }

    #[test]
    fn test_grid_style_serialization_lines() {
        let style = GridStyle::Lines;
        let json = serde_json::to_string(&style).unwrap();
        let parsed: GridStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(style, parsed);
    }

    #[test]
    fn test_grid_style_serialization_dots() {
        let style = GridStyle::Dots;
        let json = serde_json::to_string(&style).unwrap();
        let parsed: GridStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(style, parsed);
    }

    #[test]
    fn test_grid_style_serialization_hidden() {
        let style = GridStyle::Hidden;
        let json = serde_json::to_string(&style).unwrap();
        let parsed: GridStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(style, parsed);
    }

    #[test]
    fn test_grid_style_persists_in_settings_serialization() {
        let mut settings = SchematicDisplaySettings::default();
        settings.grid_style = GridStyle::Dots;

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: SchematicDisplaySettings = serde_json::from_str(&json).unwrap();

        assert_eq!(
            parsed.grid_style,
            GridStyle::Dots,
            "Grid style should persist through serialization"
        );
    }

    #[test]
    fn test_dense_preset_preserves_default_grid_style() {
        // Dense preset should use default grid style (Lines)
        let settings = SchematicDisplaySettings::dense();
        assert_eq!(settings.grid_style, GridStyle::Lines);
    }

    #[test]
    fn test_presentation_preset_preserves_default_grid_style() {
        // Presentation preset should use default grid style (Lines)
        let settings = SchematicDisplaySettings::presentation();
        assert_eq!(settings.grid_style, GridStyle::Lines);
    }

    #[test]
    fn test_grid_style_copy_semantics() {
        // GridStyle should be Copy (cheap to pass around)
        let a = GridStyle::Lines;
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn test_grid_style_equality() {
        assert_eq!(GridStyle::Lines, GridStyle::Lines);
        assert_eq!(GridStyle::Dots, GridStyle::Dots);
        assert_eq!(GridStyle::Hidden, GridStyle::Hidden);
        assert_ne!(GridStyle::Lines, GridStyle::Dots);
        assert_ne!(GridStyle::Dots, GridStyle::Hidden);
        assert_ne!(GridStyle::Lines, GridStyle::Hidden);
    }

    #[test]
    fn test_grid_style_debug_format() {
        // Debug should produce readable output
        let debug = format!("{:?}", GridStyle::Lines);
        assert!(debug.contains("Lines"));
    }

    // =========================================================================
    // Font Weight Tests
    // =========================================================================

    #[test]
    fn test_font_weight_css_bold() {
        let mut settings = SchematicDisplaySettings::default();
        settings.name_font_weight = 700;
        assert_eq!(settings.name_font_weight_css(), "bold");
    }

    #[test]
    fn test_font_weight_css_semibold() {
        let mut settings = SchematicDisplaySettings::default();
        settings.name_font_weight = 600;
        assert_eq!(settings.name_font_weight_css(), "600");
    }

    #[test]
    fn test_font_weight_css_medium() {
        let mut settings = SchematicDisplaySettings::default();
        settings.name_font_weight = 500;
        assert_eq!(settings.name_font_weight_css(), "500");
    }

    #[test]
    fn test_font_weight_css_normal() {
        let mut settings = SchematicDisplaySettings::default();
        settings.name_font_weight = 400;
        assert_eq!(settings.name_font_weight_css(), "normal");
    }

    // =========================================================================
    // Pin Visibility Tests
    // =========================================================================

    #[test]
    fn test_pin_visibility_enum_variants() {
        // Ensure all variants are properly defined
        let hidden = PinNameVisibility::Hidden;
        let on_hover = PinNameVisibility::OnHover;
        let always = PinNameVisibility::Always;

        assert_ne!(hidden, on_hover);
        assert_ne!(on_hover, always);
        assert_ne!(hidden, always);
    }

    #[test]
    fn test_pin_visibility_default_is_hidden() {
        // Clean schematics don't show pins by default
        assert_eq!(PinNameVisibility::default(), PinNameVisibility::Hidden);
    }

    #[test]
    fn test_pin_visibility_serialization() {
        let visibility = PinNameVisibility::Always;
        let json = serde_json::to_string(&visibility).unwrap();
        let parsed: PinNameVisibility = serde_json::from_str(&json).unwrap();
        assert_eq!(visibility, parsed);
    }

    // =========================================================================
    // Complete Settings Validation
    // =========================================================================

    #[test]
    fn test_all_fields_have_reasonable_defaults() {
        let s = SchematicDisplaySettings::default();

        // Visibility
        assert!(s.show_component_names);
        assert!(s.show_component_values);
        assert!(!s.show_net_names); // Nets clutter the view
        assert!(s.grid_style.is_visible()); // Grid visible by default
        assert!(s.show_selection_ring);
        assert!(!s.show_bounding_boxes); // Debug only

        // Typography
        assert!(s.name_font_size > 0.0);
        assert!(s.value_font_size > 0.0);
        assert!(s.pin_font_size > 0.0);
        assert!(s.name_font_weight >= 400);

        // Features
        assert!(s.label_shadow_enabled);
        assert!(s.max_param_lines > 0);
    }
}
