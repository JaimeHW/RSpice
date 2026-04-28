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

    // =========================================================================
    // Render Backend
    // =========================================================================
    /// Rendering backend for the schematic canvas.
    /// SVG is the default for compatibility; GPU offers better performance
    /// for large designs with 1000+ components.
    pub render_mode: RenderMode,
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

/// Rendering backend for schematic canvas.
///
/// Commercial EDA tools increasingly offer GPU-accelerated rendering
/// for large designs. SVG remains the default for maximum compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RenderMode {
    /// SVG rendering via DOM (Default - maximum compatibility)
    /// Best for: small to medium designs (<500 components)
    #[default]
    Svg,
    /// GPU-accelerated rendering via wgpu
    /// Best for: large designs (1000+ components), smooth pan/zoom
    Gpu,
}

impl RenderMode {
    /// Check if this is GPU rendering mode
    #[must_use]
    pub fn is_gpu(self) -> bool {
        matches!(self, RenderMode::Gpu)
    }

    /// Get display label for this mode
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            RenderMode::Svg => "SVG",
            RenderMode::Gpu => "GPU",
        }
    }

    /// Get icon for this mode
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            RenderMode::Svg => "🖼️",
            RenderMode::Gpu => "🚀",
        }
    }

    /// Toggle between SVG and GPU
    #[must_use]
    pub fn toggle(self) -> Self {
        match self {
            RenderMode::Svg => RenderMode::Gpu,
            RenderMode::Gpu => RenderMode::Svg,
        }
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

            // Render backend - SVG for compatibility (GPU for performance)
            render_mode: RenderMode::Svg,
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
