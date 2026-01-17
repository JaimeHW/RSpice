//! Matrix Stamping Primitives
//!
//! This module provides pre-indexed stamp structures for efficient
//! matrix/RHS stamping. The stamps cache CSC indices for O(1) access
//! during the simulation hot loop.

use crate::Value;
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};

/// Node identifier (0 = ground, always)
pub type NodeId = usize;

/// Pre-computed stamp location (row, col) - used during setup
#[derive(Debug, Clone, Copy)]
pub struct StampLocation {
    pub row: NodeId,
    pub col: NodeId,
}

/// Pre-indexed stamp for a two-terminal device
///
/// During simulation, we write directly to these matrix positions.
/// The stamp caches CSC indices after the matrix structure is finalized.
#[derive(Debug, Clone, Copy)]
pub struct TwoTerminalStamp {
    /// (node+, node+)
    pub pp: StampLocation,
    /// (node+, node-)
    pub pn: StampLocation,
    /// (node-, node+)
    pub np: StampLocation,
    /// (node-, node-)
    pub nn: StampLocation,
    /// Pre-baked CSC indices for O(1) stamping (populated after matrix build)
    pub csc_pp: Option<CscIndex>,
    pub csc_pn: Option<CscIndex>,
    pub csc_np: Option<CscIndex>,
    pub csc_nn: Option<CscIndex>,
}

impl TwoTerminalStamp {
    /// Create a new stamp for nodes at positions node_pos and node_neg
    pub fn new(node_pos: NodeId, node_neg: NodeId) -> Self {
        Self {
            pp: StampLocation {
                row: node_pos,
                col: node_pos,
            },
            pn: StampLocation {
                row: node_pos,
                col: node_neg,
            },
            np: StampLocation {
                row: node_neg,
                col: node_pos,
            },
            nn: StampLocation {
                row: node_neg,
                col: node_neg,
            },
            csc_pp: None,
            csc_pn: None,
            csc_np: None,
            csc_nn: None,
        }
    }

    /// Link this stamp to a StaticMatrix, caching CSC indices for O(1) access
    pub fn link(&mut self, matrix: &StaticMatrix) {
        if self.pp.row > 0 && self.pp.col > 0 {
            self.csc_pp = matrix.get_index(self.pp.row - 1, self.pp.col - 1);
        }
        if self.pn.row > 0 && self.pn.col > 0 {
            self.csc_pn = matrix.get_index(self.pn.row - 1, self.pn.col - 1);
        }
        if self.np.row > 0 && self.np.col > 0 {
            self.csc_np = matrix.get_index(self.np.row - 1, self.np.col - 1);
        }
        if self.nn.row > 0 && self.nn.col > 0 {
            self.csc_nn = matrix.get_index(self.nn.row - 1, self.nn.col - 1);
        }
    }

    /// Stamp a conductance value using pre-baked CSC indices (O(1) per stamp)
    #[inline]
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, g: Value) {
        if let Some(idx) = self.csc_pp {
            matrix.stamp_direct(idx, g);
        }
        if let Some(idx) = self.csc_pn {
            matrix.stamp_direct(idx, -g);
        }
        if let Some(idx) = self.csc_np {
            matrix.stamp_direct(idx, -g);
        }
        if let Some(idx) = self.csc_nn {
            matrix.stamp_direct(idx, g);
        }
    }

    /// Stamp a conductance value into the triplet matrix during setup
    #[inline]
    pub fn stamp_conductance(&self, matrix: &mut TripletMatrix, g: Value) {
        if self.pp.row != 0 && self.pp.col != 0 {
            matrix.push(self.pp.row - 1, self.pp.col - 1, g);
        }
        if self.pn.row != 0 && self.pn.col != 0 {
            matrix.push(self.pn.row - 1, self.pn.col - 1, -g);
        }
        if self.np.row != 0 && self.np.col != 0 {
            matrix.push(self.np.row - 1, self.np.col - 1, -g);
        }
        if self.nn.row != 0 && self.nn.col != 0 {
            matrix.push(self.nn.row - 1, self.nn.col - 1, g);
        }
    }
}
