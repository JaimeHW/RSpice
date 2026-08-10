//! Value types the design-management model shares with the rest of RSpice.
//!
//! Each type is a leaf: identity, coordinates, and page geometry with no
//! behaviour beyond its own invariants. They live here rather than in the
//! application crate because the persisted design-management schema and the
//! signed drawing-sheet package contract both name them, and neither may
//! depend on the GUI. `rspice-ui` re-exports every type below from the module
//! that owned it, so application paths are unchanged.

use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

// =============================================================================
// Content identity
// =============================================================================

/// Exact SHA-256 content identity. Parsing is strict and serialization is
/// always 64 lowercase hexadecimal characters.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentDigest([u8; 32]);

impl ContentDigest {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for ContentDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ContentDigest")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for ContentDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for ContentDigest {
    type Err = DigestError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 64 {
            return Err(DigestError::Length(value.len()));
        }
        let mut bytes = [0_u8; 32];
        for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
            bytes[index] = (decode_nibble(pair[0])? << 4) | decode_nibble(pair[1])?;
        }
        Ok(Self(bytes))
    }
}

fn decode_nibble(value: u8) -> Result<u8, DigestError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(DigestError::Character(value as char)),
    }
}

impl Serialize for ContentDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ContentDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DigestError {
    #[error("SHA-256 digest must contain 64 hexadecimal characters, received {0}")]
    Length(usize),
    #[error("SHA-256 digest contains non-hexadecimal character {0:?}")]
    Character(char),
}

// =============================================================================
// Configuration-set identity
// =============================================================================

/// Stable project identity for one configuration set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConfigurationSetId(Uuid);

impl ConfigurationSetId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }

    #[must_use]
    pub fn is_nil(self) -> bool {
        self.0.is_nil()
    }
}

impl Default for ConfigurationSetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConfigurationSetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Point
// =============================================================================

/// Grid-aligned point (in grid units, not pixels)
///
/// The schematic uses a grid-based coordinate system where all elements
/// snap to grid intersections. This ensures clean, aligned circuit diagrams.
///
/// # Coordinate System
/// - Origin (0, 0) is at the center of the canvas
/// - X increases to the right
/// - Y increases downward (screen coordinates)
/// - Grid size is typically 10 pixels per unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point at (x, y)
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Create a point at the origin (0, 0)
    #[inline]
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Convert to pixel coordinates
    ///
    /// # Arguments
    /// * `grid_size` - Size of each grid cell in pixels
    #[inline]
    pub fn to_pixels(self, grid_size: i32) -> (f64, f64) {
        ((self.x * grid_size) as f64, (self.y * grid_size) as f64)
    }

    /// Create from pixel coordinates (snaps to nearest grid point)
    ///
    /// # Arguments
    /// * `px` - X coordinate in pixels
    /// * `py` - Y coordinate in pixels
    /// * `grid_size` - Size of each grid cell in pixels
    #[inline]
    pub fn from_pixels(px: f64, py: f64, grid_size: i32) -> Self {
        Self {
            x: (px / grid_size as f64).round() as i32,
            y: (py / grid_size as f64).round() as i32,
        }
    }

    /// Get the 4 adjacent points (cardinal directions)
    ///
    /// Returns neighbors in order: left, right, up, down
    #[inline]
    pub fn neighbors(self) -> [Point; 4] {
        [
            Point::new(self.x - 1, self.y), // Left
            Point::new(self.x + 1, self.y), // Right
            Point::new(self.x, self.y - 1), // Up
            Point::new(self.x, self.y + 1), // Down
        ]
    }

    /// Calculate Manhattan distance to another point
    #[inline]
    pub fn manhattan_distance(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Calculate squared Euclidean distance to another point
    /// (avoids sqrt for comparison purposes)
    #[inline]
    pub fn distance_squared(self, other: Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Add another point (vector addition)
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract another point (vector subtraction)
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        self.sub(rhs)
    }
}

// =============================================================================
// Page geometry
// =============================================================================

/// Physical drawing-sheet format retained with the schematic document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicPageSize {
    #[default]
    A4,
    A3,
    UsLetter,
    UsLedger,
}

impl SchematicPageSize {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::A4 => "A4",
            Self::A3 => "A3",
            Self::UsLetter => "US Letter",
            Self::UsLedger => "US Ledger",
        }
    }

    /// Nominal portrait dimensions in tenths of a millimetre.
    #[must_use]
    pub const fn portrait_dimensions_tenth_mm(self) -> (u32, u32) {
        match self {
            Self::A4 => (2_100, 2_970),
            Self::A3 => (2_970, 4_200),
            Self::UsLetter => (2_159, 2_794),
            Self::UsLedger => (2_794, 4_318),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicPageOrientation {
    Portrait,
    #[default]
    Landscape,
}

impl SchematicPageOrientation {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Portrait => "portrait",
            Self::Landscape => "landscape",
        }
    }
}
