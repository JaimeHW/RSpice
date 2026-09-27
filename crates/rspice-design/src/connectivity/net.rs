//! Electrical net identity and conductor membership.

use rspice_design_model::Point;
use std::collections::HashSet;

/// Represents an electrical net (set of connected points)

#[derive(Debug, Clone)]
pub struct Net {
    /// Unique identifier for this net
    pub id: usize,

    /// All grid points that belong to this net
    pub points: HashSet<Point>,

    /// User-assigned label (if any, from net label component)
    pub label: Option<String>,

    /// Conductors belonging to this net, for a finding or a highlight that has
    /// to name the drawn object rather than the node.
    pub wires: Vec<u64>,
}

impl Net {
    /// Create a new empty net
    pub fn new(id: usize) -> Self {
        Self {
            id,
            points: HashSet::new(),
            label: None,
            wires: Vec::new(),
        }
    }

    /// Add a point to this net
    pub fn add_point(&mut self, point: Point) {
        self.points.insert(point);
    }

    /// Merge another net into this one
    pub fn merge(&mut self, other: &Net) {
        self.points.extend(&other.points);
        self.wires.extend(&other.wires);
        if self.label.is_none() {
            self.label = other.label.clone();
        }
    }

    /// Get SPICE node name
    ///
    /// Returns:
    /// - "0" for ground nets
    /// - User label if provided
    /// - "net{id}" otherwise
    pub fn spice_name(&self) -> String {
        if let Some(ref label) = self.label {
            if label.eq_ignore_ascii_case("0")
                || label.eq_ignore_ascii_case("gnd")
                || label.eq_ignore_ascii_case("ground")
            {
                return "0".to_string();
            }
            return label.clone();
        }
        format!("net{}", self.id)
    }
}
