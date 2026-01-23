//! Circuit representation with Struct-of-Arrays design and pre-indexed matrix stamping
//!
//! This module provides high-performance circuit storage using data-oriented design:
//! - Struct-of-Arrays (SoA) for cache-friendly iteration
//! - Pre-computed matrix indices for zero-cost stamping in the hot loop
//! - Separation of topology (static) from values (mutable)

use crate::Value;
use crate::analysis::{CompanionCoefficients, IntegrationMethod};
use crate::device::behavioral::BehavioralSources;
use crate::device::{Bjt, Cccs, Ccvs, Diode, MatrixStamper, Mosfet, Vccs, Vcvs};
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};
use crate::xspice::{CodeModelRegistry, XspiceInstance};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Node identifier (0 = ground, always)
pub type NodeId = usize;

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
// Struct-of-Arrays Device Storage
//=============================================================================

/// Resistor storage (SoA layout for cache efficiency)
#[derive(Debug, Default)]
pub struct Resistors {
    /// Device names
    pub names: Vec<String>,
    /// Pre-computed stamp locations
    pub stamps: Vec<TwoTerminalStamp>,
    /// Conductance values (1/R)
    pub conductances: Vec<Value>,
}

impl Resistors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, resistance: Value) {
        self.names.push(name);
        self.stamps.push(TwoTerminalStamp::new(node_pos, node_neg));
        self.conductances.push(1.0 / resistance);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Link all stamps to a StaticMatrix for O(1) access
    pub fn link_indices(&mut self, matrix: &StaticMatrix) {
        for stamp in &mut self.stamps {
            stamp.link(matrix);
        }
    }

    /// Stamp all resistors using pre-baked CSC indices (O(1) per stamp)
    #[inline]
    pub fn stamp_all_direct(&self, matrix: &mut StaticMatrix) {
        for (stamp, &g) in self.stamps.iter().zip(self.conductances.iter()) {
            stamp.stamp_direct(matrix, g);
        }
    }

    /// Stamp all resistors into the matrix (hot path - optimized)
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix) {
        for (stamp, &g) in self.stamps.iter().zip(self.conductances.iter()) {
            stamp.stamp_conductance(matrix, g);
        }
    }
}

/// Capacitor storage (SoA)
#[derive(Debug, Default)]
pub struct Capacitors {
    pub names: Vec<String>,
    /// Pre-computed stamps for the capacitor matrix entries
    pub stamps: Vec<TwoTerminalStamp>,
    /// Capacitance values in Farads
    pub capacitances: Vec<Value>,
    /// Previous timestep voltage (t - dt)
    pub v_prev: Vec<Value>,
    /// Voltage from 2 steps ago (t - 2*dt) for Gear2/BDF2
    pub v_prev_prev: Vec<Value>,
    /// Previous timestep capacitor current (for trapezoidal companion model)
    /// Required for accurate trapezoidal integration: ieq = geq * v_n + i_n
    pub i_prev: Vec<Value>,
    /// Equivalent current source (legacy, kept for compatibility)
    pub i_eq: Vec<Value>,
    /// Initial condition voltage (IC=)
    pub ic: Vec<Option<Value>>,
}

impl Capacitors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, capacitance: Value) {
        self.names.push(name);
        self.stamps.push(TwoTerminalStamp::new(node_pos, node_neg));
        self.capacitances.push(capacitance);
        self.v_prev.push(0.0);
        self.v_prev_prev.push(0.0);
        self.i_prev.push(0.0); // Initial capacitor current is zero
        self.i_eq.push(0.0);
        self.ic.push(None);
    }

    pub fn add_with_ic(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        capacitance: Value,
        ic: Value,
    ) {
        self.names.push(name);
        self.stamps.push(TwoTerminalStamp::new(node_pos, node_neg));
        self.capacitances.push(capacitance);
        self.v_prev.push(ic); // Initialize v_prev to IC
        self.v_prev_prev.push(ic); // Initialize v_prev_prev to IC as well
        self.i_prev.push(0.0); // Initial capacitor current is zero (DC steady state)
        self.i_eq.push(0.0);
        self.ic.push(Some(ic));
    }

    /// Apply initial conditions to v_prev
    pub fn apply_initial_conditions(&mut self) {
        for (i, ic) in self.ic.iter().enumerate() {
            if let Some(v) = ic {
                self.v_prev[i] = *v;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Link all stamps to a StaticMatrix for O(1) access
    pub fn link_indices(&mut self, matrix: &StaticMatrix) {
        for stamp in &mut self.stamps {
            stamp.link(matrix);
        }
    }

    /// Stamp all capacitors for transient analysis using optimized direct stamping
    ///
    /// This is the unified stamping method for both StaticMatrix (direct) and TripletMatrix
    #[inline]
    pub fn stamp_transient_companion(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        for (i, stamp) in self.stamps.iter().enumerate() {
            // geq = coeff_g * C / dt
            let geq = coeff.capacitor_geq(self.capacitances[i], dt);
            stamp.stamp_direct(matrix, geq);

            // Compute equivalent current source based on history
            let i_eq = coeff.capacitor_ieq(
                self.capacitances[i],
                dt,
                self.v_prev[i],
                self.v_prev_prev[i],
                self.i_prev[i],
            );

            if stamp.pp.row != 0 {
                rhs[stamp.pp.row - 1] += i_eq;
            }
            if stamp.nn.row != 0 {
                rhs[stamp.nn.row - 1] -= i_eq;
            }
        }
    }

    /// Update capacitor state after a successful timestep
    ///
    /// Stores current voltage and calculates internal current based on integration method.
    pub fn update_state(&mut self, solution: &[Value], dt: Value, method: IntegrationMethod) {
        let coeff = CompanionCoefficients::for_method(method);
        for (i, stamp) in self.stamps.iter().enumerate() {
            let v_curr = if stamp.pp.row != 0 {
                solution[stamp.pp.row - 1]
            } else {
                0.0
            } - if stamp.nn.row != 0 {
                solution[stamp.nn.row - 1]
            } else {
                0.0
            };

            // geq and ieq based on history (v_prev, v_prev_prev, i_prev)
            let geq = coeff.capacitor_geq(self.capacitances[i], dt);
            let i_eq = coeff.capacitor_ieq(
                self.capacitances[i],
                dt,
                self.v_prev[i],
                self.v_prev_prev[i],
                self.i_prev[i],
            );

            // Compute newest current: i_{n+1} = geq * v_{n+1} - i_eq
            let i_curr = geq * v_curr - i_eq;

            // Advance history
            self.v_prev_prev[i] = self.v_prev[i];
            self.v_prev[i] = v_curr;
            self.i_prev[i] = i_curr;
        }
    }

    /// Stamp all capacitors (legacy TripletMatrix support)
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value], dt: Value) {
        for (i, stamp) in self.stamps.iter().enumerate() {
            let geq = 2.0 * self.capacitances[i] / dt;
            stamp.stamp_conductance(matrix, geq);

            // Fallback to basic Trapezoidal for i_eq if update_state hasn't been unified yet
            let i_eq = geq * self.v_prev[i] + self.i_prev[i];
            if stamp.pp.row != 0 {
                rhs[stamp.pp.row - 1] += i_eq;
            }
            if stamp.nn.row != 0 {
                rhs[stamp.nn.row - 1] -= i_eq;
            }
        }
    }
}

/// Voltage source storage (SoA) - requires branch current variable
#[derive(Debug, Default)]
pub struct VoltageSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub dc_values: Vec<Value>,
    /// AC magnitude for AC/HB analysis
    pub ac_magnitudes: Vec<Value>,
    /// AC phase in radians for AC/HB analysis
    pub ac_phases: Vec<Value>,
    /// Full source specification for transient waveform evaluation
    pub source_specs: Vec<Option<crate::netlist::SourceSpec>>,
    /// Pre-baked CSC indices: [br->np, np->br, br->nn, nn->br] per source
    csc_indices: Vec<[Option<CscIndex>; 4]>,
}

impl VoltageSources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(0.0);
        self.ac_phases.push(0.0);
        self.source_specs.push(None);
        self.csc_indices.push([None; 4]);
    }

    /// Add voltage source with full AC and transient specification
    pub fn add_with_ac_and_spec(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(source_spec);
        self.csc_indices.push([None; 4]);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Link indices to StaticMatrix for O(1) stamping
    pub fn link_indices(&mut self, matrix: &StaticMatrix, get_branch_idx: impl Fn(usize) -> usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = get_branch_idx(self.branch_indices[i]);

            // br->np and np->br
            if np > 0 {
                self.csc_indices[i][0] = matrix.get_index(br - 1, np - 1);
                self.csc_indices[i][1] = matrix.get_index(np - 1, br - 1);
            }
            // br->nn and nn->br
            if nn > 0 {
                self.csc_indices[i][2] = matrix.get_index(br - 1, nn - 1);
                self.csc_indices[i][3] = matrix.get_index(nn - 1, br - 1);
            }
        }
    }

    /// Stamp all voltage sources using pre-baked CSC indices
    #[inline]
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
            let v = self.dc_values[i];

            // Stamp matrix entries using pre-baked indices
            if let Some(idx) = self.csc_indices[i][0] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][1] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][2] {
                matrix.stamp_direct(idx, -1.0);
            }
            if let Some(idx) = self.csc_indices[i][3] {
                matrix.stamp_direct(idx, -1.0);
            }

            rhs[br - 1] = v;
        }
    }

    /// Stamp voltage sources with scaled values (for source stepping)
    #[inline]
    pub fn stamp_all_direct_scaled(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        scale: Value,
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
            let v = self.dc_values[i] * scale;

            if let Some(idx) = self.csc_indices[i][0] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][1] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][2] {
                matrix.stamp_direct(idx, -1.0);
            }
            if let Some(idx) = self.csc_indices[i][3] {
                matrix.stamp_direct(idx, -1.0);
            }

            rhs[br - 1] = v;
        }
    }

    /// Stamp all voltage sources
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];
            let v = self.dc_values[i];

            // MNA stamp: add branch equation V(n+) - V(n-) = Vs
            if br > 0 && np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if br > 0 && nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            if br > 0 {
                rhs[br - 1] = v;
            }
        }
    }

    /// Update voltage source RHS values for transient analysis at time t
    ///
    /// Evaluates time-varying sources (PULSE, SIN, PWL, EXP) at the given time
    /// and updates the RHS vector. Matrix structure is unchanged.
    #[inline]
    pub fn update_transient_rhs(
        &self,
        rhs: &mut [Value],
        time: Value,
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);

            let v = match &self.source_specs[i] {
                Some(spec) => Self::evaluate_source_at_time(spec, time),
                None => self.dc_values[i], // DC only
            };

            rhs[br - 1] = v;
        }
    }

    /// Evaluate source specification at given time
    fn evaluate_source_at_time(spec: &crate::netlist::SourceSpec, time: Value) -> Value {
        use crate::netlist::SourceSpec;
        use std::f64::consts::PI;

        match spec {
            SourceSpec::Dc(v) => *v,
            SourceSpec::Ac { .. } => 0.0, // AC sources are DC=0 in transient
            SourceSpec::DcAc { dc_value, .. } => *dc_value,
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
            } => {
                if time < *delay {
                    return *v1;
                }
                let t = (time - delay) % period;
                if t < *rise {
                    v1 + (v2 - v1) * t / rise
                } else if t < rise + width {
                    *v2
                } else if t < rise + width + fall {
                    v2 + (v1 - v2) * (t - rise - width) / fall
                } else {
                    *v1
                }
            }
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                if time < *delay {
                    *offset
                } else {
                    let t = time - delay;
                    offset
                        + amplitude
                            * (-damping * t).exp()
                            * (2.0 * PI * frequency * t + phase).sin()
                }
            }
            SourceSpec::Pwl { points } => {
                if points.is_empty() {
                    return 0.0;
                }
                if time <= points[0].0 {
                    return points[0].1;
                }
                if time >= points[points.len() - 1].0 {
                    return points[points.len() - 1].1;
                }
                // Linear interpolation
                for j in 0..points.len() - 1 {
                    if time >= points[j].0 && time < points[j + 1].0 {
                        let (t1, v1) = points[j];
                        let (t2, v2) = points[j + 1];
                        return v1 + (v2 - v1) * (time - t1) / (t2 - t1);
                    }
                }
                0.0
            }
            SourceSpec::PwlFile { value_offset, .. } => *value_offset, // TODO: file loading
            SourceSpec::Exp {
                v1,
                v2,
                td1,
                tau1,
                td2,
                tau2,
            } => {
                if time < *td1 {
                    *v1
                } else if time < *td2 {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                } else {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                        - (v2 - v1) * (1.0 - (-(time - td2) / tau2).exp())
                }
            }
        }
    }
}

/// Current source storage (SoA)
#[derive(Debug, Default)]
pub struct CurrentSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub dc_values: Vec<Value>,
    /// AC magnitude for HB/AC analysis
    pub ac_magnitudes: Vec<Value>,
    /// AC phase in radians for HB/AC analysis
    pub ac_phases: Vec<Value>,
}

impl CurrentSources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, dc_value: Value) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(0.0);
        self.ac_phases.push(0.0);
    }

    /// Add current source with AC parameters
    pub fn add_with_ac(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
    }

    /// Set AC parameters for existing source
    pub fn set_ac(&mut self, index: usize, magnitude: Value, phase: Value) {
        if index < self.ac_magnitudes.len() {
            self.ac_magnitudes[index] = magnitude;
            self.ac_phases[index] = phase;
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Stamp all current sources
    #[inline]
    pub fn stamp_all(&self, rhs: &mut [Value]) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let current = self.dc_values[i];

            if np > 0 {
                rhs[np - 1] -= current;
            }
            if nn > 0 {
                rhs[nn - 1] += current;
            }
        }
    }

    /// Stamp current sources with scaled values (for source stepping)
    #[inline]
    pub fn stamp_all_scaled(&self, rhs: &mut [Value], scale: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let current = self.dc_values[i] * scale;

            if np > 0 {
                rhs[np - 1] -= current;
            }
            if nn > 0 {
                rhs[nn - 1] += current;
            }
        }
    }
}

/// Inductor storage (SoA) - requires branch current variable
#[derive(Debug, Default)]
pub struct Inductors {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub inductances: Vec<Value>,
    /// Previous current (t - dt) for companion model
    pub i_prev: Vec<Value>,
    /// Current from 2 steps ago (t - 2*dt) for Gear2/BDF2
    pub i_prev_prev: Vec<Value>,
    /// Previous voltage for companion model
    pub v_prev: Vec<Value>,
    /// Initial condition current (IC=)
    pub ic: Vec<Option<Value>>,
}

impl Inductors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        inductance: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.inductances.push(inductance);
        self.i_prev.push(0.0);
        self.i_prev_prev.push(0.0);
        self.v_prev.push(0.0);
        self.ic.push(None);
    }

    pub fn add_with_ic(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        inductance: Value,
        ic: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.inductances.push(inductance);
        self.i_prev.push(ic); // Initialize i_prev to IC
        self.i_prev_prev.push(ic); // Initialize i_prev_prev to IC as well
        self.v_prev.push(0.0);
        self.ic.push(Some(ic));
    }

    /// Apply initial conditions to i_prev
    pub fn apply_initial_conditions(&mut self) {
        for (i, ic) in self.ic.iter().enumerate() {
            if let Some(current) = ic {
                self.i_prev[i] = *current;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Get equivalent resistance for trapezoidal integration
    #[inline]
    pub fn req(&self, idx: usize, dt: Value) -> Value {
        2.0 * self.inductances[idx] / dt
    }

    /// Stamp all inductors for transient analysis using optimized direct stamping
    ///
    /// This is the unified stamping method for both StaticMatrix (direct) and TripletMatrix
    #[inline]
    pub fn stamp_transient_companion(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        num_nodes: usize,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            let r_eq = coeff.inductor_req(self.inductances[i], dt);
            let v_eq = coeff.inductor_veq(
                self.inductances[i],
                dt,
                self.i_prev[i],
                self.i_prev_prev[i],
                self.v_prev[i],
            );

            // MNA stamp for inductor companion model (V-source branch)
            // L: v = L*di/dt → v_n+1 - v_eq = r_eq * i_n+1
            if np > 0 {
                matrix.add(br - 1, np - 1, 1.0);
                matrix.add(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(br - 1, nn - 1, -1.0);
                matrix.add(nn - 1, br - 1, -1.0);
            }
            matrix.add(br - 1, br - 1, -r_eq);
            rhs[br - 1] = v_eq;
        }
    }

    /// Update inductor state after a successful timestep
    pub fn update_state(
        &mut self,
        solution: &[Value],
        num_nodes: usize,
        _dt: Value,
        _method: IntegrationMethod,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_idx = num_nodes + self.branch_indices[i] - 1;

            let v_curr = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            let i_curr = solution[br_idx];

            // Advance history
            self.i_prev_prev[i] = self.i_prev[i];
            self.i_prev[i] = i_curr;
            self.v_prev[i] = v_curr;
        }
    }

    /// Stamp all inductors (legacy TripletMatrix support)
    #[inline]
    pub fn stamp_all(
        &self,
        matrix: &mut TripletMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
        dt: Value,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            let req = 2.0 * self.inductances[i] / dt;
            let veq = req * self.i_prev[i] + self.v_prev[i];

            if np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            matrix.push(br - 1, br - 1, -req);
            rhs[br - 1] = veq;
        }
    }

    /// Update state after timestep using trapezoidal rule
    pub fn step(&mut self, voltages: &[Value], currents: &[Value], dt: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];

            let v_pos = if np == 0 {
                0.0
            } else {
                voltages.get(np - 1).copied().unwrap_or(0.0)
            };
            let v_neg = if nn == 0 {
                0.0
            } else {
                voltages.get(nn - 1).copied().unwrap_or(0.0)
            };
            let v = v_pos - v_neg;

            // Update current: i = i_prev + (dt / 2L) * (v + v_prev)
            let l = self.inductances[i];
            self.i_prev[i] = self.i_prev[i] + (dt / (2.0 * l)) * (v + self.v_prev[i]);
            self.v_prev[i] = v;

            // Also get branch current from solution for accuracy
            if br > 0 {
                if let Some(&i_br) = currents.get(br - 1) {
                    self.i_prev[i] = i_br;
                }
            }
        }
    }
}

//=============================================================================
// Nonlinear Device Storage (SoA)
//=============================================================================

/// Diode storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Default)]
pub struct Diodes {
    pub devices: Vec<Diode>,
}

impl Diodes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, diode: Diode) {
        self.devices.push(diode);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all diodes with current solution
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all diodes into matrix for Newton iteration
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all diodes have converged
    pub fn all_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all diodes to matrix for O(1) stamping
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all diodes using O(1) direct indexing
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

/// BJT storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Default)]
pub struct Bjts {
    pub devices: Vec<Bjt>,
}

impl Bjts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, bjt: Bjt) {
        self.devices.push(bjt);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all BJTs with current solution
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all BJTs into matrix for Newton iteration
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all BJTs have converged
    pub fn all_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all BJTs to matrix for O(1) stamping
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all BJTs using O(1) direct indexing
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

/// MOSFET storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Default)]
pub struct Mosfets {
    pub devices: Vec<Mosfet>,
}

impl Mosfets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, mosfet: Mosfet) {
        self.devices.push(mosfet);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all MOSFETs with current solution
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all MOSFETs into matrix for Newton iteration
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all MOSFETs have converged
    pub fn all_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all MOSFETs to matrix for O(1) stamping
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all MOSFETs using O(1) direct indexing
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

//=============================================================================
// Main Circuit Data Structure
//=============================================================================

/// High-performance circuit representation using Struct-of-Arrays
#[derive(Debug)]
pub struct CircuitData {
    /// Node name to ID mapping
    node_map: HashMap<String, NodeId>,
    /// Branch element name to branch ordinal mapping (for CCCS/CCVS control lookup)
    /// Keys are element names (e.g., "V1", "L1"), values are branch ordinals (1-indexed)
    branch_names: HashMap<String, NodeId>,
    /// Number of nodes (excluding ground)
    num_nodes: usize,
    /// Number of branch current variables (voltage sources, inductors)
    num_branches: usize,

    // Linear device storage (SoA for cache efficiency)
    pub resistors: Resistors,
    pub capacitors: Capacitors,
    pub inductors: Inductors,
    pub voltage_sources: VoltageSources,
    pub current_sources: CurrentSources,

    // Nonlinear device storage (require Newton-Raphson iteration)
    pub diodes: Diodes,
    pub bjts: Bjts,
    pub mosfets: Mosfets,
    pub jfets: Vec<crate::device::Jfet>,

    // Controlled sources
    pub vcvs: Vcvs,
    pub vccs: Vccs,
    pub cccs: Cccs,
    pub ccvs: Ccvs,

    // Pending control element resolutions (element name -> CCCS/CCVS indices)
    /// CCCS elements pending control branch resolution: (cccs_index, control_element_name)
    pending_cccs: Vec<(usize, String)>,
    /// CCVS elements pending control branch resolution: (ccvs_index, control_element_name)
    pending_ccvs: Vec<(usize, String)>,

    // Advanced device storage
    pub vswitches: Vec<crate::device::VoltageSwitch>,
    pub iswitches: Vec<crate::device::CurrentSwitch>,
    pub tlines: Vec<crate::device::TransmissionLine>,
    pub couplings: Vec<crate::device::InductorCoupling>,

    // Behavioral sources (expression-based B-elements)
    pub behavioral_sources: BehavioralSources,

    // XSPICE code model instances
    /// XSPICE instance storage for code model evaluation
    pub xspice_instances: Vec<XspiceInstance>,
    /// XSPICE code model registry (shared across instances)
    pub xspice_registry: Arc<CodeModelRegistry>,

    // Verilog-A devices (feature-gated)
    #[cfg(feature = "veriloga")]
    pub veriloga_devices: crate::device::veriloga::VerilogADevices,
}

impl CircuitData {
    pub fn new() -> Self {
        let mut node_map = HashMap::new();
        // Ground is always node 0
        node_map.insert("0".to_string(), 0);
        node_map.insert("gnd".to_string(), 0);
        node_map.insert("GND".to_string(), 0);

        Self {
            node_map,
            branch_names: HashMap::new(),
            num_nodes: 0,
            num_branches: 0,
            resistors: Resistors::new(),
            capacitors: Capacitors::new(),
            inductors: Inductors::new(),
            voltage_sources: VoltageSources::new(),
            current_sources: CurrentSources::new(),
            diodes: Diodes::new(),
            bjts: Bjts::new(),
            mosfets: Mosfets::new(),
            jfets: Vec::new(),
            vcvs: Vcvs::new(),
            vccs: Vccs::new(),
            cccs: Cccs::new(),
            ccvs: Ccvs::new(),
            pending_cccs: Vec::new(),
            pending_ccvs: Vec::new(),
            // New device types
            vswitches: Vec::new(),
            iswitches: Vec::new(),
            tlines: Vec::new(),
            couplings: Vec::new(),
            behavioral_sources: BehavioralSources::new(),
            // XSPICE instances
            xspice_instances: Vec::new(),
            xspice_registry: Arc::new(CodeModelRegistry::with_builtins()),
            // Verilog-A devices
            #[cfg(feature = "veriloga")]
            veriloga_devices: crate::device::veriloga::VerilogADevices::new(),
        }
    }

    /// Get or create a node ID for the given name
    /// Node "0" is always ground (NodeId 0) - this is the SPICE standard
    pub fn get_or_create_node(&mut self, name: &str) -> NodeId {
        // Node "0" is always ground - return 0 immediately
        if name == "0" {
            self.node_map.insert("0".to_string(), 0);
            return 0;
        }

        if let Some(&id) = self.node_map.get(name) {
            return id;
        }

        self.num_nodes += 1;
        self.node_map.insert(name.to_string(), self.num_nodes);
        self.num_nodes
    }

    /// Check if any device in the circuit actually uses ground (node 0)
    /// This is different from just having "0" in the node map - we need to check
    /// if any device terminal is connected to node 0
    pub fn has_ground_node(&self) -> bool {
        // Check resistors
        for stamp in &self.resistors.stamps {
            if stamp.pp.row == 0
                || stamp.pp.col == 0
                || stamp.pn.row == 0
                || stamp.pn.col == 0
                || stamp.np.row == 0
                || stamp.np.col == 0
                || stamp.nn.row == 0
                || stamp.nn.col == 0
            {
                return true;
            }
        }

        // Check voltage sources
        for i in 0..self.voltage_sources.len() {
            if self.voltage_sources.node_pos[i] == 0 || self.voltage_sources.node_neg[i] == 0 {
                return true;
            }
        }

        // Check current sources
        for i in 0..self.current_sources.len() {
            if self.current_sources.node_pos[i] == 0 || self.current_sources.node_neg[i] == 0 {
                return true;
            }
        }

        // Check capacitors
        for stamp in &self.capacitors.stamps {
            if stamp.pp.row == 0 || stamp.nn.row == 0 {
                return true;
            }
        }

        // Check inductors
        for i in 0..self.inductors.len() {
            if self.inductors.node_pos[i] == 0 || self.inductors.node_neg[i] == 0 {
                return true;
            }
        }

        false
    }

    /// Ensure a ground reference exists. If no explicit node "0" was specified,
    /// pick the first node connected to a voltage source's negative terminal
    /// as the reference.
    /// This should be called after all elements are added but before simulation.
    pub fn ensure_ground_reference(&mut self) {
        if self.has_ground_node() {
            return; // Already have explicit ground
        }

        // No explicit ground - pick first voltage source's negative terminal
        // This matches standard behavior
        if !self.voltage_sources.is_empty() {
            let ref_node_id = self.voltage_sources.node_neg[0];
            if ref_node_id > 0 {
                // Find the name of this node and remap it to 0
                let mut ref_node_name = None;
                for (name, &id) in &self.node_map {
                    if id == ref_node_id {
                        ref_node_name = Some(name.clone());
                        break;
                    }
                }

                if let Some(name) = ref_node_name {
                    // Remap this node to ground (0)
                    self.remap_node_to_ground(ref_node_id);
                    log::info!("Auto-selected node '{}' as ground reference", name);
                }
            }
        }
    }

    /// Remap all occurrences of old_node_id to ground (0) and shift all higher
    /// node IDs down by 1 to maintain contiguous matrix indices
    fn remap_node_to_ground(&mut self, old_node_id: NodeId) {
        // Helper closure to remap a single node ID
        let remap = |id: NodeId| -> NodeId {
            if id == old_node_id {
                0
            } else if id > old_node_id {
                id - 1 // Shift down to fill the gap
            } else {
                id
            }
        };

        // Update node map
        for (_, id) in self.node_map.iter_mut() {
            *id = remap(*id);
        }

        // Update all device node references
        // Resistors
        for stamp in &mut self.resistors.stamps {
            Self::remap_stamp_full(stamp, old_node_id);
        }

        // Capacitors
        for stamp in &mut self.capacitors.stamps {
            Self::remap_stamp_full(stamp, old_node_id);
        }

        // Voltage sources
        for i in 0..self.voltage_sources.len() {
            self.voltage_sources.node_pos[i] = remap(self.voltage_sources.node_pos[i]);
            self.voltage_sources.node_neg[i] = remap(self.voltage_sources.node_neg[i]);
        }

        // Current sources
        for i in 0..self.current_sources.len() {
            self.current_sources.node_pos[i] = remap(self.current_sources.node_pos[i]);
            self.current_sources.node_neg[i] = remap(self.current_sources.node_neg[i]);
        }

        // Inductors
        for i in 0..self.inductors.len() {
            self.inductors.node_pos[i] = remap(self.inductors.node_pos[i]);
            self.inductors.node_neg[i] = remap(self.inductors.node_neg[i]);
        }

        // Decrement num_nodes since one node is now ground
        if self.num_nodes > 0 {
            self.num_nodes -= 1;
        }
    }

    /// Helper to remap a two-terminal stamp with full shifting
    fn remap_stamp_full(stamp: &mut TwoTerminalStamp, old_id: NodeId) {
        // Helper to remap a single node ID
        let remap = |id: NodeId| -> NodeId {
            if id == old_id {
                0
            } else if id > old_id {
                id - 1 // Shift down to fill the gap
            } else {
                id
            }
        };

        stamp.pp.row = remap(stamp.pp.row);
        stamp.pp.col = remap(stamp.pp.col);
        stamp.pn.row = remap(stamp.pn.row);
        stamp.pn.col = remap(stamp.pn.col);
        stamp.np.row = remap(stamp.np.row);
        stamp.np.col = remap(stamp.np.col);
        stamp.nn.row = remap(stamp.nn.row);
        stamp.nn.col = remap(stamp.nn.col);
    }

    /// Allocate a branch current variable - returns branch ordinal (1-indexed)
    /// Note: The stored value is the branch ordinal, NOT the matrix index.
    /// Use get_branch_matrix_index() to get the actual matrix row/column.
    pub fn allocate_branch(&mut self) -> NodeId {
        self.num_branches += 1;
        self.num_branches // Return branch ordinal (1, 2, 3...)
    }

    /// Allocate a branch and register it with the given element name
    /// This allows CCCS/CCVS to look up control branches by name
    pub fn allocate_branch_named(&mut self, name: &str) -> NodeId {
        let branch = self.allocate_branch();
        // Store both original and uppercase for case-insensitive lookup
        self.branch_names.insert(name.to_string(), branch);
        self.branch_names.insert(name.to_uppercase(), branch);
        branch
    }

    /// Look up a branch ordinal by element name (for CCCS/CCVS control element)
    /// Returns None if the element is not found
    pub fn get_branch_by_name(&self, name: &str) -> Option<NodeId> {
        self.branch_names
            .get(name)
            .or_else(|| self.branch_names.get(&name.to_uppercase()))
            .copied()
    }

    /// Register a CCCS element for pending control branch resolution
    /// The control_element_name will be resolved after all elements are added
    pub fn add_cccs_pending(&mut self, cccs_index: usize, control_element_name: String) {
        self.pending_cccs.push((cccs_index, control_element_name));
    }

    /// Register a CCVS element for pending control branch resolution
    pub fn add_ccvs_pending(&mut self, ccvs_index: usize, control_element_name: String) {
        self.pending_ccvs.push((ccvs_index, control_element_name));
    }

    /// Resolve all pending CCCS/CCVS control element references
    /// Call this after all elements have been added to the circuit
    /// Returns an error if any control element is not found
    pub fn resolve_control_elements(&mut self) -> Result<(), CircuitError> {
        // Resolve CCCS control branches
        for (cccs_idx, control_name) in self.pending_cccs.drain(..).collect::<Vec<_>>() {
            let branch = self.get_branch_by_name(&control_name).ok_or_else(|| {
                CircuitError::InvalidComponent(format!(
                    "CCCS control element not found: {}",
                    control_name
                ))
            })?;
            if cccs_idx < self.cccs.ctrl_branch.len() {
                self.cccs.ctrl_branch[cccs_idx] = branch;
            }
        }

        // Resolve CCVS control branches
        for (ccvs_idx, control_name) in self.pending_ccvs.drain(..).collect::<Vec<_>>() {
            let branch = self.get_branch_by_name(&control_name).ok_or_else(|| {
                CircuitError::InvalidComponent(format!(
                    "CCVS control element not found: {}",
                    control_name
                ))
            })?;
            if ccvs_idx < self.ccvs.ctrl_branch.len() {
                self.ccvs.ctrl_branch[ccvs_idx] = branch;
            }
        }

        Ok(())
    }

    /// Convert branch ordinal to matrix index
    /// Branch ordinals start at 1, matrix indices for branches start at num_nodes
    pub fn get_branch_matrix_index(&self, branch_ordinal: NodeId) -> usize {
        self.num_nodes + branch_ordinal
    }

    /// Total matrix size
    pub fn matrix_size(&self) -> usize {
        self.num_nodes + self.num_branches
    }

    /// Number of nodes (excluding ground)
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Number of branches
    pub fn num_branches(&self) -> usize {
        self.num_branches
    }

    /// Get node names sorted by their node index (1, 2, 3, ...)
    /// Returns a Vec where index i contains the name of node (i+1)
    /// This is useful for waveform output labels like V(N001), V(N002)
    pub fn node_names_sorted(&self) -> Vec<String> {
        // Create a vec with one entry per non-ground node
        let mut names: Vec<(NodeId, String)> = self
            .node_map
            .iter()
            .filter(|(_, id)| **id > 0) // Exclude ground (id 0)
            .map(|(name, id)| (*id, name.clone()))
            .collect();

        // Sort by node ID
        names.sort_by_key(|(id, _)| *id);

        // Remove duplicates (keep first occurrence for each ID - in case of aliases like GND/gnd/0)
        names.dedup_by_key(|(id, _)| *id);

        // Extract just the names in order
        names.into_iter().map(|(_, name)| name).collect()
    }

    /// Total device count (for parallel stamping threshold)
    pub fn device_count(&self) -> usize {
        self.resistors.len()
            + self.capacitors.len()
            + self.inductors.len()
            + self.voltage_sources.len()
            + self.current_sources.len()
            + self.diodes.len()
            + self.bjts.len()
            + self.mosfets.len()
            + self.vcvs.len()
            + self.vccs.len()
            + self.cccs.len()
            + self.ccvs.len()
    }

    /// Create a triplet matrix for this circuit
    pub fn create_matrix(&self) -> TripletMatrix {
        let size = self.matrix_size();
        let mut m = TripletMatrix::new(size);
        m.nrows = size;
        m.ncols = size;
        m
    }

    /// Create RHS vector for this circuit
    pub fn create_rhs(&self) -> Vec<Value> {
        vec![0.0; self.matrix_size()]
    }

    /// Link all device stamps to a StaticMatrix for O(1) stamping
    /// Call this after build_matrix() to bake CSC indices into devices
    pub fn link_indices(&mut self, matrix: &StaticMatrix) {
        // Linear devices
        self.resistors.link_indices(matrix);
        self.capacitors.link_indices(matrix);
        let num_nodes = self.num_nodes;
        self.voltage_sources
            .link_indices(matrix, |br_ordinal| num_nodes + br_ordinal);

        // Nonlinear devices
        self.diodes.link_all(matrix);
        self.bjts.link_all(matrix);
        self.mosfets.link_all(matrix);
    }

    /// Stamp all linear devices for DC analysis using O(1) direct stamping
    pub fn stamp_dc_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.voltage_sources
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all(rhs);

        // Stamp controlled sources
        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
    }

    /// Stamp all devices with scaled source values (for source stepping)
    pub fn stamp_dc_direct_scaled(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        scale: Value,
    ) {
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.voltage_sources
            .stamp_all_direct_scaled(matrix, rhs, scale, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all_scaled(rhs, scale);
    }

    /// Stamp all linear devices for DC analysis
    pub fn stamp_dc(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        self.resistors.stamp_all(matrix);
        self.voltage_sources.stamp_all(matrix, rhs);
        self.current_sources.stamp_all(rhs);
    }

    /// Check if circuit has any nonlinear devices requiring Newton-Raphson
    pub fn has_nonlinear_devices(&self) -> bool {
        !self.diodes.is_empty() || !self.bjts.is_empty() || !self.mosfets.is_empty()
    }

    /// Update all nonlinear devices with current solution
    pub fn update_nonlinear(&mut self, voltages: &[Value]) {
        self.diodes.update_all(voltages);
        self.bjts.update_all(voltages);
        self.mosfets.update_all(voltages);
    }

    /// Stamp all nonlinear devices into matrix using O(1) direct indexing
    pub fn stamp_nonlinear(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        self.diodes.stamp_all_direct(matrix, rhs, voltages);
        self.bjts.stamp_all_direct(matrix, rhs, voltages);
        self.mosfets.stamp_all_direct(matrix, rhs, voltages);
    }

    /// Check if all nonlinear devices have converged
    pub fn nonlinear_converged(&self, tolerance: Value) -> bool {
        self.diodes.all_converged(tolerance)
            && self.bjts.all_converged(tolerance)
            && self.mosfets.all_converged(tolerance)
    }

    //=========================================================================
    // XSPICE Code Model Interface
    //=========================================================================

    /// Check if circuit has any XSPICE code model instances
    #[inline]
    pub fn has_xspice_devices(&self) -> bool {
        !self.xspice_instances.is_empty()
    }

    /// Evaluate all XSPICE code model instances
    ///
    /// This calls each XspiceInstance::evaluate() with the current simulation
    /// state, updating internal context and computing output contributions.
    ///
    /// # Arguments
    /// * `time` - Current simulation time
    /// * `voltages` - Current node voltage solution
    pub fn evaluate_xspice(&mut self, time: Value, voltages: &[Value]) {
        use crate::xspice::AnalysisType;

        for instance in &mut self.xspice_instances {
            // First, collect input values (immutable borrow of connections)
            let input_values: Vec<(usize, Value)> = instance
                .connections()
                .iter()
                .enumerate()
                .filter_map(|(port_idx, connection)| match connection {
                    crate::xspice::PortConnection::Analog(node) => {
                        let v = if *node == 0 {
                            0.0
                        } else {
                            voltages.get(*node - 1).copied().unwrap_or(0.0)
                        };
                        Some((port_idx, v))
                    }
                    crate::xspice::PortConnection::Differential(pos, neg) => {
                        let v_pos = if *pos == 0 {
                            0.0
                        } else {
                            voltages.get(*pos - 1).copied().unwrap_or(0.0)
                        };
                        let v_neg = if *neg == 0 {
                            0.0
                        } else {
                            voltages.get(*neg - 1).copied().unwrap_or(0.0)
                        };
                        Some((port_idx, v_pos - v_neg))
                    }
                    crate::xspice::PortConnection::Digital(_) => None,
                    _ => None,
                })
                .collect();

            // Now set input values (mutable borrow)
            for (port_idx, v) in input_values {
                instance.set_input_analog(port_idx, v);
            }

            // Evaluate the code model
            let analysis = AnalysisType::Transient;
            if let Err(e) = instance.evaluate(time, 0.0, analysis) {
                log::warn!("XSPICE evaluation error for {}: {}", instance.name, e);
            }
        }
    }

    /// Stamp XSPICE analog contributions into matrix and RHS
    ///
    /// After evaluation, analog code models produce conductance and current
    /// contributions that must be stamped into the MNA system.
    pub fn stamp_xspice(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for instance in &self.xspice_instances {
            // Get contributions from each output port
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                if let Some((conductance, current)) = instance.get_analog_contribution(port_idx) {
                    match connection {
                        crate::xspice::PortConnection::Analog(node) => {
                            if *node > 0 {
                                // Stamp diagonal conductance
                                matrix.add(*node - 1, *node - 1, conductance);
                                // Stamp RHS current
                                rhs[*node - 1] += current;
                            }
                        }
                        crate::xspice::PortConnection::Differential(pos, neg) => {
                            // Stamp differential conductance
                            if *pos > 0 {
                                matrix.add(*pos - 1, *pos - 1, conductance);
                                if *neg > 0 {
                                    matrix.add(*pos - 1, *neg - 1, -conductance);
                                }
                                rhs[*pos - 1] += current;
                            }
                            if *neg > 0 {
                                if *pos > 0 {
                                    matrix.add(*neg - 1, *pos - 1, -conductance);
                                }
                                matrix.add(*neg - 1, *neg - 1, conductance);
                                rhs[*neg - 1] -= current;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Accept current timestep for all XSPICE instances
    ///
    /// Called after a successful timestep to commit state changes.
    pub fn accept_xspice_timestep(&mut self) {
        for instance in &mut self.xspice_instances {
            instance.accept_timestep();
        }
    }

    /// Check if all XSPICE instances have converged
    pub fn xspice_converged(&self, tolerance: Value) -> bool {
        self.xspice_instances
            .iter()
            .all(|inst| inst.is_converged(tolerance))
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_creation() {
        let mut circuit = CircuitData::new();
        let n1 = circuit.get_or_create_node("1");
        let n2 = circuit.get_or_create_node("2");

        assert_eq!(n1, 1);
        assert_eq!(n2, 2);
        assert_eq!(circuit.num_nodes(), 2);
    }

    #[test]
    fn test_resistor_stamping() {
        let mut circuit = CircuitData::new();
        let n1 = circuit.get_or_create_node("1");
        let n2 = circuit.get_or_create_node("2");

        circuit.resistors.add("R1".to_string(), n1, n2, 1000.0);

        let mut matrix = circuit.create_matrix();
        circuit.resistors.stamp_all(&mut matrix);

        assert_eq!(matrix.nnz(), 4); // 4 stamp entries
    }

    #[test]
    fn test_ground_node() {
        let mut circuit = CircuitData::new();
        let gnd = circuit.get_or_create_node("0");
        let gnd2 = circuit.get_or_create_node("gnd");
        let gnd3 = circuit.get_or_create_node("GND");

        assert_eq!(gnd, 0);
        assert_eq!(gnd2, 0);
        assert_eq!(gnd3, 0);
    }
}
