/// Eye diagram display mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EyeDisplayMode {
    /// Overlay mode - all traces visible
    #[default]
    Overlay,
    /// Persistence/density mode - heat map
    Persistence,
    /// Single trace view
    SingleTrace,
}

impl EyeDisplayMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Overlay => "Overlay",
            Self::Persistence => "Persistence",
            Self::SingleTrace => "Single Trace",
        }
    }

    /// Get all modes
    pub fn all() -> &'static [EyeDisplayMode] {
        &[Self::Overlay, Self::Persistence, Self::SingleTrace]
    }
}

/// Color map for persistence/density display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMap {
    /// Hot (black -> red -> yellow -> white)
    Hot,
    /// Cool (black -> blue -> cyan -> white)
    Cool,
    /// Viridis (perceptually uniform)
    Viridis,
    /// Rainbow
    Rainbow,
    /// Green phosphor (classic scope look)
    #[default]
    Phosphor,
}

impl ColorMap {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Viridis => "Viridis",
            Self::Rainbow => "Rainbow",
            Self::Phosphor => "Phosphor",
        }
    }

    /// Get all color maps
    pub fn all() -> &'static [ColorMap] {
        &[
            Self::Hot,
            Self::Cool,
            Self::Viridis,
            Self::Rainbow,
            Self::Phosphor,
        ]
    }

    /// Map intensity (0-1) to RGB color
    pub fn map(&self, intensity: f32) -> (u8, u8, u8) {
        let t = intensity.clamp(0.0, 1.0);

        match self {
            Self::Hot => {
                let r = (t * 3.0).min(1.0);
                let g = ((t - 0.33) * 3.0).clamp(0.0, 1.0);
                let b = ((t - 0.66) * 3.0).clamp(0.0, 1.0);
                (
                    (r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8,
                )
            }
            Self::Cool => {
                let r = ((t - 0.66) * 3.0).clamp(0.0, 1.0);
                let g = ((t - 0.33) * 3.0).clamp(0.0, 1.0);
                let b = (t * 3.0).min(1.0);
                (
                    (r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8,
                )
            }
            Self::Viridis => {
                let r = (0.267 + 0.329 * t + 0.278 * t * t).min(1.0);
                let g = (0.004 + 0.873 * t - 0.268 * t * t).clamp(0.0, 1.0);
                let b = (0.329 + 0.475 * t - 0.785 * t * t).clamp(0.0, 1.0);
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
            Self::Rainbow => {
                let hue = t * 5.0;
                let (r, g, b) = match hue as u32 {
                    0 => (1.0, hue, 0.0),
                    1 => (2.0 - hue, 1.0, 0.0),
                    2 => (0.0, 1.0, hue - 2.0),
                    3 => (0.0, 4.0 - hue, 1.0),
                    4 => (hue - 4.0, 0.0, 1.0),
                    _ => (1.0, 0.0, 1.0),
                };
                (
                    (r.min(1.0) * 255.0) as u8,
                    (g.min(1.0) * 255.0) as u8,
                    (b.min(1.0) * 255.0) as u8,
                )
            }
            Self::Phosphor => {
                let g = t * 0.9 + 0.1 * t * t;
                let r = t * 0.1;
                let b = t * 0.2;
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
        }
    }
}
