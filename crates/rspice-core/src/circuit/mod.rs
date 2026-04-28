//! Circuit representation with Struct-of-Arrays design and pre-indexed matrix stamping
//!
//! This module provides high-performance circuit storage using data-oriented design:
//! - Struct-of-Arrays (SoA) for cache-friendly iteration
//! - Pre-computed matrix indices for zero-cost stamping in the hot loop
//! - Separation of topology (static) from values (mutable)

#![allow(clippy::too_many_arguments)]
use crate::Value;
use crate::analysis::CompanionCoefficients;
use crate::device::behavioral::BehavioralSources;
use crate::device::{Cccs, Ccvs, MatrixStamper, NonlinearConvergenceCriteria, Vccs, Vcvs};
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};
use crate::xspice::{CodeModelRegistry, XspiceInstance};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

mod storage;
pub use storage::{
    Bjts, Capacitors, CurrentSources, Diodes, Inductors, Mosfets, Resistors, VoltageSources,
};
mod construction;
mod external_models;
mod force_accept;
mod introspection;
mod linear_stamping;
mod magnetic;
mod nonlinear;

/// Node identifier (0 = ground, always)
pub type NodeId = usize;

#[inline]
fn solution_node_voltage(solution: &[Value], node: NodeId) -> Option<Value> {
    if node == 0 {
        Some(0.0)
    } else {
        solution
            .get(node - 1)
            .copied()
            .filter(|value| value.is_finite())
    }
}

#[inline]
fn project_two_terminal_voltage(
    solution: &mut [Value],
    node_pos: NodeId,
    node_neg: NodeId,
    target_voltage: Value,
) {
    if !target_voltage.is_finite() {
        return;
    }

    if node_neg == 0 && node_pos > 0 {
        if let Some(v) = solution.get_mut(node_pos - 1) {
            *v = target_voltage;
        }
        return;
    }

    if node_pos == 0 && node_neg > 0 {
        if let Some(v) = solution.get_mut(node_neg - 1) {
            *v = -target_voltage;
        }
        return;
    }

    if node_pos == 0 || node_neg == 0 {
        return;
    }

    let vp_idx = node_pos - 1;
    let vn_idx = node_neg - 1;
    if vp_idx >= solution.len() || vn_idx >= solution.len() {
        return;
    }

    let vp = solution[vp_idx];
    let vn = solution[vn_idx];
    if !(vp.is_finite() && vn.is_finite()) {
        return;
    }

    let midpoint = 0.5 * (vp + vn);
    let half_diff = 0.5 * target_voltage;
    solution[vp_idx] = midpoint + half_diff;
    solution[vn_idx] = midpoint - half_diff;
}

/// Errors in circuit construction
#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Duplicate node name: {0}")]
    DuplicateNode(String),

    #[error("Invalid component: {0}")]
    InvalidComponent(String),

    #[error("No ground node in circuit")]
    NoGround,

    #[error("Floating node detected: {0}")]
    FloatingNode(String),

    #[error("Singular matrix - circuit may have issues")]
    SingularMatrix,
}

/// Pre-computed index into the matrix values array for O(1) stamping
#[derive(Debug, Clone, Copy)]
pub struct MatrixIndex(pub usize);

/// Pre-computed stamp location (row, col) - used during setup
#[derive(Debug, Clone, Copy)]
pub struct StampLocation {
    pub row: NodeId,
    pub col: NodeId,
}

/// Pre-indexed stamp for a two-terminal device
/// During simulation, we write directly to these matrix positions
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

    /// Stamp a conductance value into the matrix
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

//=============================================================================
// Main Circuit Data Structure
//=============================================================================

/// High-performance circuit representation using Struct-of-Arrays
#[derive(Debug, Clone)]
pub struct JilesAthertonBinding {
    /// Index into `inductors` SoA arrays that owns this runtime branch/state.
    pub inductor_index: usize,
    /// Branch ordinal (1-indexed) allocated for this inductor.
    pub branch_ordinal: NodeId,
    /// Stateful Jiles-Atherton model used to update effective inductance.
    pub device: crate::device::passive::JilesAthertonInductor,
}

/// Runtime binding for a coupled inductor pair.
#[derive(Debug, Clone)]
pub struct CoupledInductorPairBinding {
    /// Branch ordinal (1-indexed) for winding 1.
    pub branch1_ordinal: NodeId,
    /// Branch ordinal (1-indexed) for winding 2.
    pub branch2_ordinal: NodeId,
    /// Coupled inductor runtime device.
    pub device: crate::device::CoupledInductorPair,
}

/// Runtime binding for a multi-winding transformer.
#[derive(Debug, Clone)]
pub struct MultiWindingTransformerBinding {
    /// Branch ordinals (1-indexed) for each winding current.
    pub branch_ordinals: Vec<NodeId>,
    /// Transformer runtime device.
    pub device: crate::device::MultiWindingTransformer,
}

/// High-performance circuit representation using Struct-of-Arrays
#[derive(Debug)]
pub struct CircuitData {
    /// Node name to ID mapping
    node_map: HashMap<String, NodeId>,
    /// Tracks whether any element explicitly referenced the SPICE ground node.
    has_explicit_ground_reference: bool,
    /// Branch element name to branch ordinal mapping (for CCCS/CCVS control lookup)
    /// Keys are element names (e.g., "V1", "L1"), values are branch ordinals (1-indexed)
    branch_names: HashMap<String, NodeId>,
    /// Canonical branch names indexed by branch ordinal - 1.
    branch_name_by_ordinal: Vec<Option<String>>,
    /// Number of nodes (excluding ground)
    num_nodes: usize,
    /// Number of branch current variables (voltage sources, inductors)
    num_branches: usize,

    // Linear device storage (SoA for cache efficiency)
    pub(crate) resistors: Resistors,
    pub(crate) capacitors: Capacitors,
    pub(crate) inductors: Inductors,
    pub(crate) voltage_sources: VoltageSources,
    pub(crate) current_sources: CurrentSources,

    // Nonlinear device storage (require Newton-Raphson iteration)
    pub(crate) diodes: Diodes,
    pub(crate) bjts: Bjts,
    pub(crate) mosfets: Mosfets,
    pub(crate) jfets: Vec<crate::device::Jfet>,

    // Controlled sources
    pub(crate) vcvs: Vcvs,
    pub(crate) vccs: Vccs,
    pub(crate) cccs: Cccs,
    pub(crate) ccvs: Ccvs,

    // Pending control element resolutions (element name -> CCCS/CCVS indices)
    /// CCCS elements pending control branch resolution: (cccs_index, control_element_name)
    pending_cccs: Vec<(usize, String)>,
    /// CCVS elements pending control branch resolution: (ccvs_index, control_element_name)
    pending_ccvs: Vec<(usize, String)>,
    /// ISWITCH elements pending control branch resolution: (iswitch_index, control_element_name)
    pending_iswitch: Vec<(usize, String)>,

    // Advanced device storage
    pub(crate) vswitches: Vec<crate::device::VoltageSwitch>,
    pub(crate) iswitches: Vec<crate::device::CurrentSwitch>,
    pub(crate) tlines: Vec<crate::device::TransmissionLine>,
    pub(crate) coupled_tlines: Vec<crate::device::CoupledTransmissionLine>,
    pub(crate) couplings: Vec<crate::device::InductorCoupling>,
    pub(crate) coupled_inductor_pairs: Vec<CoupledInductorPairBinding>,
    pub(crate) multi_winding_transformers: Vec<MultiWindingTransformerBinding>,
    pub(crate) jiles_atherton_inductors: Vec<JilesAthertonBinding>,
    /// Circuit-level transient step-size hint for synthesized distributed
    /// structures that need finer temporal resolution than the user-level
    /// `.tran` print/max-step request to preserve propagation fidelity.
    pub(crate) transient_max_step_hint: Option<Value>,

    // Behavioral sources (expression-based B-elements)
    pub(crate) behavioral_sources: BehavioralSources,

    // XSPICE code model instances
    /// XSPICE instance storage for code model evaluation
    pub(crate) xspice_instances: Vec<XspiceInstance>,
    /// XSPICE code model registry (shared across instances)
    pub(crate) xspice_registry: Arc<CodeModelRegistry>,

    // Verilog-A devices (feature-gated)
    #[cfg(feature = "veriloga")]
    pub(crate) veriloga_devices: crate::device::veriloga::VerilogADevices,
}

/// Stable probe metadata for inductor-backed branch measurements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InductorProbeInfo {
    pub canonical_name: String,
    pub branch_ordinal: NodeId,
    pub state_index: usize,
}

/// Adapter to use StaticMatrix with MatrixStamper trait
#[allow(dead_code)] // Reserved for future direct stamping integration
struct StaticMatrixStamper<'a> {
    matrix: &'a mut StaticMatrix,
    rhs: &'a mut [Value],
}

impl<'a> MatrixStamper for StaticMatrixStamper<'a> {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            self.matrix.add(row - 1, col - 1, value);
        }
    }

    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        if index > 0 && index <= self.rhs.len() {
            self.rhs[index - 1] += value;
        }
    }
}

impl Default for CircuitData {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Legacy re-exports for compatibility
//=============================================================================

/// Circuit node (legacy compatibility)
#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub id: NodeId,
}

/// Circuit (legacy compatibility wrapper)
pub type Circuit = CircuitData;
