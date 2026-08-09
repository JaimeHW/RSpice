//! Symbol geometry types.

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
// Symbol Definition
// ============================================================================

/// A complete schematic symbol definition
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Human-readable name
    pub name: String,
    /// All paths that make up this symbol
    pub paths: Vec<SymbolPath>,
    /// Bounding box (min_x, min_y, max_x, max_y) in normalized coords
    pub bounds: (f32, f32, f32, f32),
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

    /// Distinct path vertices that land on the authored view-box boundary,
    /// transformed into centered schematic coordinates at `target_width` ×
    /// `target_height`. Clean device SVGs terminate every electrical lead on
    /// that boundary, so this is the fail-closed bridge between artwork and
    /// the separately authoritative electrical terminal contract.
    pub fn boundary_anchors(&self, target_width: f32, target_height: f32) -> Vec<(f32, f32)> {
        let tolerance = self.width().max(self.height()).max(1.0) * 0.001;
        let (min_x, min_y, max_x, max_y) = self.bounds;
        let (center_x, center_y) = self.center();
        let scale_x = target_width / self.width().max(0.001);
        let scale_y = target_height / self.height().max(0.001);
        let mut anchors = Vec::new();
        let mut consider = |x: f32, y: f32| {
            let boundary = (x - min_x).abs() <= tolerance
                || (x - max_x).abs() <= tolerance
                || (y - min_y).abs() <= tolerance
                || (y - max_y).abs() <= tolerance;
            if !boundary {
                return;
            }
            let transformed = ((x - center_x) * scale_x, (y - center_y) * scale_y);
            if !anchors.iter().any(|(existing_x, existing_y)| {
                let dx: f32 = *existing_x - transformed.0;
                let dy: f32 = *existing_y - transformed.1;
                dx.abs() <= 0.01 && dy.abs() <= 0.01
            }) {
                anchors.push(transformed);
            }
        };
        for path in &self.paths {
            for command in &path.commands {
                match command {
                    PathCommand::MoveTo(x, y) | PathCommand::LineTo(x, y) => consider(*x, *y),
                    PathCommand::CurveTo { end, .. } => consider(end.0, end.1),
                    PathCommand::Close => {}
                }
            }
        }
        anchors.sort_by(|left, right| {
            left.0
                .total_cmp(&right.0)
                .then_with(|| left.1.total_cmp(&right.1))
        });
        anchors
    }
}
