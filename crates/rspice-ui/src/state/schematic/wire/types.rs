//! Junction classification: how many wires meet at a point, and what that
//! means electrically and visually.
//!
//! This file also held `WireDragMode`, `DragConstraint`, and a `SnapTarget`
//! with its own position/priority/kind queries. None were constructed, and the
//! snap target in particular duplicated `state::schematic::snap::SnapTarget`,
//! which is the one the editor uses.

use serde::{Deserialize, Serialize};

/// Type of wire junction
///
/// Junctions are classified by how many wires meet at a point,
/// which affects both electrical connectivity and visual rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JunctionType {
    /// No junction - two collinear wire segments (pass-through)
    PassThrough,
    /// L-junction - two wires at 90 degrees (corner)
    Corner,
    /// T-junction - three wires meeting (electrical connection)
    TJunction,
    /// Cross junction - four wires meeting (electrical connection)
    CrossJunction,
    /// Complex junction - more than 4 wires meeting
    Complex { wire_count: usize },
}

impl JunctionType {
    /// Determine junction type from number of connected wire segments
    pub fn from_wire_count(count: usize) -> Self {
        match count {
            0 | 1 => JunctionType::PassThrough,
            2 => JunctionType::Corner, // Could be pass-through or corner
            3 => JunctionType::TJunction,
            4 => JunctionType::CrossJunction,
            n => JunctionType::Complex { wire_count: n },
        }
    }

    /// Check if this junction represents an electrical connection
    /// (not just a corner or pass-through)
    pub fn is_electrical_connection(&self) -> bool {
        matches!(
            self,
            JunctionType::TJunction | JunctionType::CrossJunction | JunctionType::Complex { .. }
        )
    }

    /// Get the number of wires at this junction
    pub fn wire_count(&self) -> usize {
        match self {
            JunctionType::PassThrough => 1,
            JunctionType::Corner => 2,
            JunctionType::TJunction => 3,
            JunctionType::CrossJunction => 4,
            JunctionType::Complex { wire_count } => *wire_count,
        }
    }

    /// Check if this is a simple corner (no electrical junction dot needed)
    pub fn is_corner(&self) -> bool {
        matches!(self, JunctionType::Corner)
    }

    /// Check if this junction needs a visual dot indicator
    pub fn needs_dot(&self) -> bool {
        self.is_electrical_connection()
    }
}

// =============================================================================
// Tests
// =============================================================================
