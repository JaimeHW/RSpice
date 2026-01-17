//! Device Storage Containers (Struct-of-Arrays Design)
//!
//! This module provides cache-efficient storage for circuit elements using
//! Struct-of-Arrays (SoA) layout. This design enables:
//! - Better cache locality during iteration
//! - Efficient SIMD vectorization potential
//! - Pre-indexed matrix stamping for O(1) hot-path access

use super::stamps::{NodeId, TwoTerminalStamp};
use crate::Value;
use crate::device::{Bjt, Diode, MatrixStamper, Mosfet};
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};

//=============================================================================
// Linear Device Storage
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
    /// Equivalent current source
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
        self.v_prev.push(ic);
        self.v_prev_prev.push(ic);
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

    /// Stamp all capacitors for transient analysis
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value], dt: Value) {
        for (i, stamp) in self.stamps.iter().enumerate() {
            let geq = 2.0 * self.capacitances[i] / dt;
            stamp.stamp_conductance(matrix, geq);

            let i_eq = self.i_eq[i];
            if stamp.pp.row != 0 {
                rhs[stamp.pp.row - 1] -= i_eq;
            }
            if stamp.nn.row != 0 {
                rhs[stamp.nn.row - 1] += i_eq;
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

            if np > 0 {
                self.csc_indices[i][0] = matrix.get_index(br - 1, np - 1);
                self.csc_indices[i][1] = matrix.get_index(np - 1, br - 1);
            }
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
}

/// Current source storage (SoA)
#[derive(Debug, Default)]
pub struct CurrentSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub dc_values: Vec<Value>,
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
    /// Current from 2 steps ago for Gear2/BDF2
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
        self.i_prev.push(ic);
        self.i_prev_prev.push(ic);
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

    /// Stamp all inductors for transient analysis
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value], dt: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];
            let req = self.req(i, dt);
            let veq = req * self.i_prev[i] + self.v_prev[i];

            if np > 0 && br > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if nn > 0 && br > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            if br > 0 {
                matrix.push(br - 1, br - 1, -req);
                rhs[br - 1] = veq;
            }
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

            let l = self.inductances[i];
            self.i_prev[i] = self.i_prev[i] + (dt / (2.0 * l)) * (v + self.v_prev[i]);
            self.v_prev[i] = v;

            if br > 0 {
                if let Some(&i_br) = currents.get(br - 1) {
                    self.i_prev[i] = i_br;
                }
            }
        }
    }
}

//=============================================================================
// Nonlinear Device Storage
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

    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

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

    pub fn all_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

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

    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

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

    pub fn all_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

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

    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

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

    pub fn all_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

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
