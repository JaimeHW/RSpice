//! Component Rotation
//!
//! Handles component rotation state and transformations.

use serde::{Deserialize, Serialize};

// =============================================================================
// Rotation
// =============================================================================

/// Component rotation (clockwise, in 90-degree increments)
///
/// All schematic elements use orthogonal rotation (0°, 90°, 180°, 270°).
/// This matches standard EDA tool behavior and simplifies grid alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Rotation {
    /// No rotation (0 degrees)
    #[default]
    R0,
    /// 90 degrees clockwise
    R90,
    /// 180 degrees (upside down)
    R180,
    /// 270 degrees clockwise (90 counter-clockwise)
    R270,
}

impl Rotation {
    /// Rotate 90 degrees clockwise
    ///
    /// Returns the next rotation in the sequence: R0 → R90 → R180 → R270 → R0
    #[inline]
    pub fn rotate_cw(self) -> Self {
        match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }

    /// Rotate 90 degrees counter-clockwise
    ///
    /// Returns the previous rotation in the sequence: R0 → R270 → R180 → R90 → R0
    #[inline]
    pub fn rotate_ccw(self) -> Self {
        match self {
            Rotation::R0 => Rotation::R270,
            Rotation::R90 => Rotation::R0,
            Rotation::R180 => Rotation::R90,
            Rotation::R270 => Rotation::R180,
        }
    }

    /// Get rotation angle in degrees (0, 90, 180, or 270)
    #[inline]
    pub fn degrees(self) -> i32 {
        match self {
            Rotation::R0 => 0,
            Rotation::R90 => 90,
            Rotation::R180 => 180,
            Rotation::R270 => 270,
        }
    }

    /// Get rotation angle in radians
    #[inline]
    pub fn radians(self) -> f64 {
        (self.degrees() as f64).to_radians()
    }

    /// Create from degrees (must be 0, 90, 180, or 270)
    ///
    /// Other values are normalized to the nearest valid rotation.
    pub fn from_degrees(degrees: i32) -> Self {
        // Normalize to 0-360 range
        let normalized = degrees.rem_euclid(360);
        match normalized {
            0..=44 => Rotation::R0,
            45..=134 => Rotation::R90,
            135..=224 => Rotation::R180,
            225..=314 => Rotation::R270,
            _ => Rotation::R0,
        }
    }

    /// Check if rotated 180 degrees (vertical flip for horizontal components)
    #[inline]
    pub fn is_flipped(self) -> bool {
        matches!(self, Rotation::R180 | Rotation::R270)
    }

    /// Check if rotated 90 or 270 degrees (horizontal becomes vertical)
    #[inline]
    pub fn is_vertical(self) -> bool {
        matches!(self, Rotation::R90 | Rotation::R270)
    }

    /// Check if rotated 0 or 180 degrees (horizontal orientation)
    #[inline]
    pub fn is_horizontal(self) -> bool {
        matches!(self, Rotation::R0 | Rotation::R180)
    }

    /// Combine two rotations
    pub fn combine(self, other: Rotation) -> Rotation {
        let total = (self.degrees() + other.degrees()) % 360;
        Rotation::from_degrees(total)
    }
}

// =============================================================================
// Tests
// =============================================================================

