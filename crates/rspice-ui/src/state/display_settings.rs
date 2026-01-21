//! Schematic Display Settings
//!
//! Centralized configuration for schematic rendering and label display.
//! Follows the Cadence/Spectre pattern of separating display preferences
//! from schematic data, enabling per-session or per-project customization.

use serde::{Deserialize, Serialize};

/// Display settings for schematic components and labels.
///
/// These settings control the visual presentation without affecting
/// the underlying circuit data. Professional simulators like Cadence Spectre
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
}

/// Pin name visibility modes (Virtuoso-style display option)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PinNameVisibility {
    /// Never show pin names (Virtuoso default - clean schematic)
    #[default]
    Hidden,
    /// Show pin names only on hover (convenience feature, not Virtuoso-authentic)
    OnHover,
    /// Always show pin names (toggle via View menu)
    Always,
}

impl Default for SchematicDisplaySettings {
    fn default() -> Self {
        Self {
            // Label visibility - sensible defaults matching professional tools
            show_component_names: true,
            show_component_values: true,
            show_pin_names: PinNameVisibility::Hidden, // Virtuoso default: hidden, toggle via View menu
            show_net_names: false,

            // Typography - match existing CompSvg defaults, professional sizing
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
}
