//! Theme System - Design tokens and styling constants
//!
//! Provides a centralized theme system with dark/light mode support
//! and consistent design tokens throughout the application.

/// Application theme with design tokens
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    /// Whether dark mode is enabled
    pub is_dark: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self { is_dark: true }
    }
}

impl Theme {
    /// Dark theme (default)
    pub const DARK: Self = Self { is_dark: true };

    /// Light theme
    pub const LIGHT: Self = Self { is_dark: false };

    //=========================================================================
    // Background Colors
    //=========================================================================

    /// Primary background color (deepest layer)
    pub fn bg_primary(&self) -> &'static str {
        if self.is_dark {
            "#0a0a0f"
        } else {
            "#f8f9fa"
        }
    }

    /// Secondary background (panels, cards)
    pub fn bg_secondary(&self) -> &'static str {
        if self.is_dark {
            "#12121a"
        } else {
            "#ffffff"
        }
    }

    /// Tertiary background (elevated elements)
    pub fn bg_tertiary(&self) -> &'static str {
        if self.is_dark {
            "#1a1a24"
        } else {
            "#f0f1f3"
        }
    }

    /// Surface color for interactive elements
    pub fn surface(&self) -> &'static str {
        if self.is_dark {
            "#252532"
        } else {
            "#e8eaed"
        }
    }

    /// Hover state background
    pub fn surface_hover(&self) -> &'static str {
        if self.is_dark {
            "#2d2d3d"
        } else {
            "#dcdfe3"
        }
    }

    //=========================================================================
    // Border Colors
    //=========================================================================

    /// Primary border color
    pub fn border(&self) -> &'static str {
        if self.is_dark {
            "#2a2a3a"
        } else {
            "#d0d3d8"
        }
    }

    /// Subtle border for dividers
    pub fn border_subtle(&self) -> &'static str {
        if self.is_dark {
            "#1f1f2a"
        } else {
            "#e0e3e8"
        }
    }

    //=========================================================================
    // Text Colors
    //=========================================================================

    /// Primary text color
    pub fn text_primary(&self) -> &'static str {
        if self.is_dark {
            "#e8e8ed"
        } else {
            "#1a1a24"
        }
    }

    /// Secondary text color
    pub fn text_secondary(&self) -> &'static str {
        if self.is_dark {
            "#9898a8"
        } else {
            "#5a5a6a"
        }
    }

    /// Muted text color
    pub fn text_muted(&self) -> &'static str {
        if self.is_dark {
            "#686878"
        } else {
            "#8a8a9a"
        }
    }

    //=========================================================================
    // Accent Colors (consistent across themes)
    //=========================================================================

    /// Primary accent (blue)
    pub fn accent_primary(&self) -> &'static str {
        "#3b82f6"
    }

    /// Success color (green)
    pub fn accent_success(&self) -> &'static str {
        "#22c55e"
    }

    /// Warning color (yellow/amber)
    pub fn accent_warning(&self) -> &'static str {
        "#eab308"
    }

    /// Error/danger color (red)
    pub fn accent_error(&self) -> &'static str {
        "#ef4444"
    }

    //=========================================================================
    // Waveform Trace Colors
    //=========================================================================

    /// Trace color palette for waveforms
    pub fn trace_colors(&self) -> &'static [&'static str] {
        &[
            "#FF6600", // Orange (primary - best visibility)
            "#00FF00", // Green (high visibility)
            "#00FFFF", // Cyan
            "#FF00FF", // Magenta
            "#FFFF00", // Yellow
            "#FF0000", // Red
            "#00FF80", // Spring green
            "#FF80FF", // Pink
        ]
    }

    /// Get trace color by index (wraps around)
    pub fn trace_color(&self, index: usize) -> &'static str {
        Self::trace_color_static(index)
    }

    /// Get trace color by index (static version)
    /// Uses high-saturation colors for distinct visibility
    pub fn trace_color_static(index: usize) -> &'static str {
        const COLORS: &[&str] = &[
            "#FF6600", // Orange (primary - best visibility)
            "#00FF00", // Green (high visibility)
            "#00FFFF", // Cyan
            "#FF00FF", // Magenta
            "#FFFF00", // Yellow
            "#FF0000", // Red
            "#00FF80", // Spring green
            "#FF80FF", // Pink
        ];
        COLORS[index % COLORS.len()]
    }

    //=========================================================================
    // Spacing Scale
    //=========================================================================

    /// Extra small spacing (4px)
    pub const SPACING_XS: &'static str = "4px";

    /// Small spacing (8px)
    pub const SPACING_SM: &'static str = "8px";

    /// Medium spacing (16px)
    pub const SPACING_MD: &'static str = "16px";

    /// Large spacing (24px)
    pub const SPACING_LG: &'static str = "24px";

    /// Extra large spacing (32px)
    pub const SPACING_XL: &'static str = "32px";

    //=========================================================================
    // Typography
    //=========================================================================

    /// Base font family
    pub const FONT_FAMILY: &'static str =
        "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif";

    /// Monospace font for code/values
    pub const FONT_MONO: &'static str =
        "'JetBrains Mono', 'Fira Code', 'SF Mono', Consolas, monospace";

    /// Small font size (12px)
    pub const FONT_SIZE_SM: &'static str = "12px";

    /// Base font size (14px)
    pub const FONT_SIZE_BASE: &'static str = "14px";

    /// Large font size (16px)
    pub const FONT_SIZE_LG: &'static str = "16px";

    /// Heading font size (20px)
    pub const FONT_SIZE_HEADING: &'static str = "20px";

    //=========================================================================
    // Border Radius
    //=========================================================================

    /// Small radius (4px)
    pub const RADIUS_SM: &'static str = "4px";

    /// Medium radius (8px)
    pub const RADIUS_MD: &'static str = "8px";

    /// Large radius (12px)
    pub const RADIUS_LG: &'static str = "12px";

    //=========================================================================
    // Transitions
    //=========================================================================

    /// Fast transition duration
    pub const TRANSITION_FAST: &'static str = "150ms ease";

    /// Normal transition duration
    pub const TRANSITION_NORMAL: &'static str = "200ms ease";
}
