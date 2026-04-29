/// A single path command for rendering symbol graphics.
/// These are pre-parsed from SVG paths for efficient runtime rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// Move to absolute position
    MoveTo(f32, f32),
    /// Draw line to absolute position
    LineTo(f32, f32),
    /// Cubic bezier curve with control points
    CurveTo {
        ctrl1: (f32, f32),
        ctrl2: (f32, f32),
        end: (f32, f32),
    },
    /// Close the current path
    Close,
}

/// A complete path within a symbol (may have multiple paths per symbol)
#[derive(Debug, Clone)]
pub struct SymbolPath {
    /// The sequence of drawing commands
    pub commands: Vec<PathCommand>,
    /// Whether to fill this path (vs stroke only)
    pub filled: bool,
}

// ============================================================================
// Symbol Pin Definitions
// ============================================================================

/// Direction a pin faces for wire attachment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinDirection {
    Left,
    Right,
    Up,
    Down,
}

/// A connection point on a symbol
#[derive(Debug, Clone)]
pub struct SymbolPin {
    /// Pin name (e.g., "1", "2", "G", "D", "S")
    pub name: String,
    /// Position relative to symbol center (in normalized coords)
    pub position: (f32, f32),
    /// Direction the pin faces (for wire routing)
    pub direction: PinDirection,
}

// ============================================================================
// Symbol Definition
// ============================================================================

/// A complete schematic symbol definition
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Human-readable name
    pub name: String,
    /// All paths that make up this symbol
    pub paths: Vec<SymbolPath>,
    /// Connection points
    pub pins: Vec<SymbolPin>,
    /// Bounding box (min_x, min_y, max_x, max_y) in normalized coords
    pub bounds: (f32, f32, f32, f32),
    /// Original SVG viewBox for scaling
    pub view_box: (f32, f32, f32, f32),
    /// Target width for rendering (in grid units) - commercial-grade per-component sizing
    pub target_width: f32,
    /// Target height for rendering (in grid units) - commercial-grade per-component sizing
    pub target_height: f32,
}

impl Symbol {
    /// Get symbol width in normalized coordinates
    pub fn width(&self) -> f32 {
        self.bounds.2 - self.bounds.0
    }

    /// Get symbol height in normalized coordinates
    pub fn height(&self) -> f32 {
        self.bounds.3 - self.bounds.1
    }

    /// Center point of the symbol (in normalized coordinates starting at 0,0)
    pub fn center(&self) -> (f32, f32) {
        // Use bounds center since paths are normalized to start at (0,0)
        (
            (self.bounds.0 + self.bounds.2) / 2.0,
            (self.bounds.1 + self.bounds.3) / 2.0,
        )
    }
}
