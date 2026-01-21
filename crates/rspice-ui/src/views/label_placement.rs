//! Label Placement System
//!
//! Implements Cadence-style smart auto-placement for component labels.
//! Features:
//! - Collision detection with wires and other components
//! - Multiple candidate positions scored by proximity and overlap
//! - User-overridable custom positions
//! - Rotation-aware placement

use crate::state::{Component, LabelPosition, Point, Rotation, Wire};

/// Label placement candidate position
#[derive(Debug, Clone, Copy)]
pub struct LabelCandidate {
    /// Position offset from component center (in pixels, pre-rotation)
    pub x: f64,
    pub y: f64,
    /// Base score (higher = preferred default position)
    pub preference: f64,
}

/// Computed label position result
#[derive(Debug, Clone, Copy)]
pub struct ComputedLabelPos {
    pub x: f64,
    pub y: f64,
}

/// Label placement engine for smart auto-placement
pub struct LabelPlacer<'a> {
    /// All wires in the schematic (for collision detection)
    wires: &'a [Wire],
    /// All components in the schematic (for collision detection)
    components: &'a [Component],
    /// Grid size in pixels
    grid_size: i32,
}

impl<'a> LabelPlacer<'a> {
    /// Create a new label placer
    pub fn new(wires: &'a [Wire], components: &'a [Component], grid_size: i32) -> Self {
        Self {
            wires,
            components,
            grid_size,
        }
    }

    /// Candidate positions for name label (above component by default)
    fn name_candidates() -> Vec<LabelCandidate> {
        vec![
            // Primary: above center (close to component)
            LabelCandidate {
                x: 0.0,
                y: -18.0,
                preference: 1.0,
            },
            // Alternative: above-left
            LabelCandidate {
                x: -15.0,
                y: -18.0,
                preference: 0.9,
            },
            // Alternative: above-right
            LabelCandidate {
                x: 15.0,
                y: -18.0,
                preference: 0.9,
            },
            // Alternative: left side
            LabelCandidate {
                x: -28.0,
                y: 0.0,
                preference: 0.7,
            },
            // Alternative: right side
            LabelCandidate {
                x: 28.0,
                y: 0.0,
                preference: 0.7,
            },
            // Fallback: below (if everything else collides)
            LabelCandidate {
                x: 0.0,
                y: 30.0,
                preference: 0.3,
            },
        ]
    }

    /// Candidate positions for value label (below component by default)
    fn value_candidates() -> Vec<LabelCandidate> {
        vec![
            // Primary: below center (close to component, symmetric with name at -18)
            LabelCandidate {
                x: 0.0,
                y: 18.0,
                preference: 1.0,
            },
            // Alternative: below-left
            LabelCandidate {
                x: -12.0,
                y: 18.0,
                preference: 0.9,
            },
            // Alternative: below-right
            LabelCandidate {
                x: 12.0,
                y: 18.0,
                preference: 0.9,
            },
            // Alternative: left side (tighter)
            LabelCandidate {
                x: -22.0,
                y: 6.0,
                preference: 0.6,
            },
            // Alternative: right side (tighter)
            LabelCandidate {
                x: 22.0,
                y: 6.0,
                preference: 0.6,
            },
            // Fallback: above (if everything else collides)
            LabelCandidate {
                x: 0.0,
                y: -28.0,
                preference: 0.2,
            },
        ]
    }

    /// Compute optimal position for name label
    pub fn compute_name_position(
        &self,
        component: &Component,
        label_pos: LabelPosition,
    ) -> ComputedLabelPos {
        match label_pos {
            LabelPosition::Custom { x_offset, y_offset } => {
                // User-defined position - apply rotation
                let (rx, ry) = self.rotate_offset(x_offset, y_offset, component.rotation);
                ComputedLabelPos { x: rx, y: ry }
            }
            LabelPosition::Auto => {
                // Smart placement - find best candidate
                self.find_best_position(component, Self::name_candidates(), true)
            }
        }
    }

    /// Compute optimal position for value label
    pub fn compute_value_position(
        &self,
        component: &Component,
        label_pos: LabelPosition,
        name_pos: Option<ComputedLabelPos>,
    ) -> ComputedLabelPos {
        match label_pos {
            LabelPosition::Custom { x_offset, y_offset } => {
                let (rx, ry) = self.rotate_offset(x_offset, y_offset, component.rotation);
                ComputedLabelPos { x: rx, y: ry }
            }
            LabelPosition::Auto => {
                // Smart placement - avoid name label position too
                self.find_best_position_avoiding(
                    component,
                    Self::value_candidates(),
                    false,
                    name_pos,
                )
            }
        }
    }

    /// Find best position from candidates
    fn find_best_position(
        &self,
        component: &Component,
        candidates: Vec<LabelCandidate>,
        _is_name: bool,
    ) -> ComputedLabelPos {
        let (comp_px, comp_py) = component.pos.to_pixels(self.grid_size);
        let mut best_candidate = candidates[0];
        let mut best_score = f64::NEG_INFINITY;

        for candidate in &candidates {
            // Rotate candidate position by component rotation
            let (rx, ry) = self.rotate_offset(candidate.x, candidate.y, component.rotation);
            let label_x = comp_px + rx;
            let label_y = comp_py + ry;

            // Calculate collision penalty
            let collision_penalty = self.calculate_collision_penalty(
                component.id,
                label_x,
                label_y,
                50.0, // Approximate label width
                14.0, // Approximate label height
            );

            // Score = preference - collision penalty
            let score = candidate.preference - collision_penalty;

            if score > best_score {
                best_score = score;
                best_candidate = *candidate;
            }
        }

        let (rx, ry) = self.rotate_offset(best_candidate.x, best_candidate.y, component.rotation);
        ComputedLabelPos { x: rx, y: ry }
    }

    /// Find best position avoiding another label
    fn find_best_position_avoiding(
        &self,
        component: &Component,
        candidates: Vec<LabelCandidate>,
        is_name: bool,
        avoid_pos: Option<ComputedLabelPos>,
    ) -> ComputedLabelPos {
        let (comp_px, comp_py) = component.pos.to_pixels(self.grid_size);
        let mut best_candidate = candidates[0];
        let mut best_score = f64::NEG_INFINITY;

        for candidate in &candidates {
            let (rx, ry) = self.rotate_offset(candidate.x, candidate.y, component.rotation);
            let label_x = comp_px + rx;
            let label_y = comp_py + ry;

            // Calculate collision penalty with wires/components
            let mut collision_penalty =
                self.calculate_collision_penalty(component.id, label_x, label_y, 50.0, 14.0);

            // Add penalty for overlapping with name label position
            if let Some(name_pos) = avoid_pos {
                let dist_sq = (rx - name_pos.x).powi(2) + (ry - name_pos.y).powi(2);
                if dist_sq < 400.0 {
                    // Within 20px
                    collision_penalty += 0.8;
                }
            }

            let score = candidate.preference - collision_penalty;

            if score > best_score {
                best_score = score;
                best_candidate = *candidate;
            }
        }

        let (rx, ry) = self.rotate_offset(best_candidate.x, best_candidate.y, component.rotation);
        // Pass is_name to suppress warning (for future use)
        let _ = is_name;
        ComputedLabelPos { x: rx, y: ry }
    }

    /// Calculate collision penalty for a label at given position
    fn calculate_collision_penalty(
        &self,
        component_id: u64,
        label_x: f64,
        label_y: f64,
        label_width: f64,
        label_height: f64,
    ) -> f64 {
        let mut penalty = 0.0;

        // Check wire collisions
        for wire in self.wires {
            if self.label_intersects_wire(label_x, label_y, label_width, label_height, wire) {
                penalty += 1.0;
            }
        }

        // Check component collisions (excluding self)
        for comp in self.components {
            if comp.id != component_id {
                if self.label_near_component(label_x, label_y, comp) {
                    penalty += 0.5;
                }
            }
        }

        penalty
    }

    /// Check if label bounding box intersects a wire
    fn label_intersects_wire(
        &self,
        label_x: f64,
        label_y: f64,
        label_width: f64,
        label_height: f64,
        wire: &Wire,
    ) -> bool {
        let half_w = label_width / 2.0;
        let half_h = label_height / 2.0;
        let label_left = label_x - half_w;
        let label_right = label_x + half_w;
        let label_top = label_y - half_h;
        let label_bottom = label_y + half_h;

        for segment in wire.points.windows(2) {
            let (x1, y1) = segment[0].to_pixels(self.grid_size);
            let (x2, y2) = segment[1].to_pixels(self.grid_size);

            // Simple AABB vs line segment intersection
            if self.line_intersects_rect(
                x1,
                y1,
                x2,
                y2,
                label_left,
                label_top,
                label_right,
                label_bottom,
            ) {
                return true;
            }
        }

        false
    }

    /// Check if line segment intersects rectangle
    fn line_intersects_rect(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    ) -> bool {
        // Check if either endpoint is inside rect
        if (x1 >= left && x1 <= right && y1 >= top && y1 <= bottom)
            || (x2 >= left && x2 <= right && y2 >= top && y2 <= bottom)
        {
            return true;
        }

        // Check if line crosses any edge of rect
        // Horizontal wire
        if y1 == y2 {
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            return y1 >= top && y1 <= bottom && max_x >= left && min_x <= right;
        }

        // Vertical wire
        if x1 == x2 {
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            return x1 >= left && x1 <= right && max_y >= top && min_y <= bottom;
        }

        // Diagonal - simplified check using line bounding box
        let line_left = x1.min(x2);
        let line_right = x1.max(x2);
        let line_top = y1.min(y2);
        let line_bottom = y1.max(y2);

        !(line_right < left || line_left > right || line_bottom < top || line_top > bottom)
    }

    /// Check if label is near another component
    fn label_near_component(&self, label_x: f64, label_y: f64, other: &Component) -> bool {
        let (cx, cy) = other.pos.to_pixels(self.grid_size);
        let dx: f64 = label_x - cx;
        let dy: f64 = label_y - cy;
        let dist_sq = dx * dx + dy * dy;
        dist_sq < 900.0 // Within 30px of component center
    }

    /// Rotate offset by component rotation (counter-rotate for label stability)
    fn rotate_offset(&self, x: f64, y: f64, rotation: Rotation) -> (f64, f64) {
        match rotation {
            Rotation::R0 => (x, y),
            Rotation::R90 => (y, -x),
            Rotation::R180 => (-x, -y),
            Rotation::R270 => (-y, x),
        }
    }
}

/// Convenience function for simple label position computation
pub fn compute_label_positions(
    component: &Component,
    wires: &[Wire],
    components: &[Component],
    grid_size: i32,
) -> (ComputedLabelPos, ComputedLabelPos) {
    let placer = LabelPlacer::new(wires, components, grid_size);
    let name_pos = placer.compute_name_position(component, component.name_label_pos);
    let value_pos =
        placer.compute_value_position(component, component.value_label_pos, Some(name_pos));
    (name_pos, value_pos)
}
