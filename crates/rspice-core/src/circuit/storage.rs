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

use crate::device::NonlinearDevice;

/// Macro to generate nonlinear device storage containers.
///
/// This reduces boilerplate while maintaining type safety and an explicit API.
/// Each generated struct provides the standard Newton-Raphson iteration interface:
/// - `new()`, `add()`, `len()`, `is_empty()` - basic container operations
/// - `update_all()` - update device operating points from node voltages
/// - `stamp_all()` - stamp linearized contributions into matrix/rhs
/// - `all_converged()` - check if all devices have converged
/// - `link_all()` - link CSC indices for direct stamping
/// - `stamp_all_direct()` - O(1) stamping using pre-linked indices
macro_rules! define_nonlinear_storage {
    (
        $(#[$meta:meta])*
        $name:ident, $device:ty
    ) => {
        $(#[$meta])*
        #[derive(Debug, Default)]
        pub struct $name {
            pub devices: Vec<$device>,
        }

        impl $name {
            #[inline]
            pub fn new() -> Self {
                Self::default()
            }

            #[inline]
            pub fn add(&mut self, device: $device) {
                self.devices.push(device);
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.devices.len()
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.devices.is_empty()
            }

            pub fn update_all(&mut self, voltages: &[Value]) {
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
                for d in &self.devices {
                    d.stamp_nonlinear(voltages, matrix, rhs);
                }
            }

            pub fn all_converged(&self, tolerance: Value) -> bool {
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
    };
}

// Generate storage containers for all nonlinear device types
// NOTE: Diodes, Mosfets, and BJTs have custom implementations below for SIMD batch processing

// NOTE: Mosfets, BJTs, and Diodes have custom implementations below to support SIMD batch processing

//=============================================================================
// Custom Diode Storage with SIMD Batch Support
//=============================================================================

/// Diode storage for nonlinear Newton-Raphson iteration.
///
/// When compiled with the `simd` feature, this struct maintains a batch
/// representation for SIMD-accelerated evaluation on circuits with many diodes.
#[derive(Debug, Default)]
pub struct Diodes {
    /// Individual diode devices
    pub devices: Vec<Diode>,

    /// Batch representation for SIMD acceleration
    #[cfg(feature = "simd")]
    batch: Option<crate::device::batch::BatchDiodes>,
}

impl Diodes {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add(&mut self, device: Diode) {
        self.devices.push(device);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all devices from node voltages.
    pub fn update_all(&mut self, voltages: &[Value]) {
        #[cfg(feature = "simd")]
        if let Some(ref mut batch) = self.batch {
            batch.gather_voltages(voltages);
            batch.evaluate();
            return;
        }

        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all devices into matrix and RHS.
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        // Batch processing isn't used here because stamp_all uses the generic
        // MatrixStamper trait. Use stamp_all_direct for batch processing.
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all devices have converged.
    pub fn all_converged(&self, tolerance: Value) -> bool {
        #[cfg(feature = "simd")]
        if let Some(ref batch) = self.batch {
            return batch.all_converged(tolerance);
        }

        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all devices to the sparse matrix for O(1) stamping.
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        // Link individual devices
        for d in &mut self.devices {
            d.link(matrix);
        }

        // Build and link batch representation if we have enough diodes
        #[cfg(feature = "simd")]
        self.build_batch(matrix);
    }

    /// Build the batch representation from individual devices.
    #[cfg(feature = "simd")]
    fn build_batch(&mut self, matrix: &StaticMatrix) {
        // Only use batch for 4+ diodes (minimum for SIMD benefit)
        if self.devices.len() < 4 {
            self.batch = None;
            return;
        }

        let mut batch = crate::device::batch::BatchDiodes::with_capacity(self.devices.len());

        for d in &self.devices {
            batch.add(d.node_anode, d.node_cathode, d.is, d.n, d.vt);
        }

        // Link batch to matrix
        batch.link(matrix);

        self.batch = Some(batch);
    }

    /// Stamp all devices using O(1) direct indexing.
    ///
    /// When SIMD batch processing is available and enabled, this uses
    /// SIMD-accelerated evaluation for the I-V calculations.
    pub fn stamp_all_direct(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        #[cfg(feature = "simd")]
        if let Some(ref mut batch) = self.batch {
            // Use batch SIMD processing
            batch.gather_voltages(voltages);
            batch.evaluate();
            batch.stamp(matrix, rhs);
            return;
        }

        // Fallback to individual device stamping
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

//=============================================================================
// Custom MOSFET Storage with SIMD Batch Support
//=============================================================================

/// MOSFET storage for nonlinear Newton-Raphson iteration.
///
/// When compiled with the `simd` feature, this struct maintains a batch
/// representation for SIMD-accelerated evaluation on circuits with many MOSFETs.
#[derive(Debug, Default)]
pub struct Mosfets {
    /// Individual MOSFET devices
    pub devices: Vec<Mosfet>,

    /// Batch representation for Level 1 MOSFETs
    #[cfg(feature = "simd")]
    batch_level1: Option<crate::device::batch_mosfet::BatchMosfets>,

    /// Batch representation for Level 6 MOSFETs
    #[cfg(feature = "simd")]
    batch_level6: Option<crate::device::batch_mosfet_level6::BatchMosfetsLevel6>,

    /// Indices of devices handled by batch (for fallback stamping)
    #[cfg(feature = "simd")]
    batched_indices: Vec<usize>,
}

impl Mosfets {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add(&mut self, device: Mosfet) {
        self.devices.push(device);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all devices from node voltages.
    pub fn update_all(&mut self, voltages: &[Value]) {
        #[cfg(feature = "simd")]
        {
            if let Some(ref mut batch) = self.batch_level1 {
                batch.gather_voltages(voltages);
                batch.evaluate();
            }
            if let Some(ref mut batch) = self.batch_level6 {
                batch.gather_voltages(voltages);
                batch.evaluate();
            }
            // Still need to update non-batched devices
            for (i, d) in self.devices.iter_mut().enumerate() {
                if !self.batched_indices.contains(&i) {
                    d.update(voltages);
                }
            }
            return;
        }

        #[cfg(not(feature = "simd"))]
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all devices into matrix and RHS.
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all devices have converged.
    pub fn all_converged(&self, tolerance: Value) -> bool {
        #[cfg(feature = "simd")]
        {
            if let Some(ref batch) = self.batch_level1 {
                if !batch.all_converged(tolerance) {
                    return false;
                }
            }
            if let Some(ref batch) = self.batch_level6 {
                if !batch.all_converged(tolerance) {
                    return false;
                }
            }
            // Check non-batched devices
            for (i, d) in self.devices.iter().enumerate() {
                if !self.batched_indices.contains(&i) && !d.is_converged(tolerance) {
                    return false;
                }
            }
            return true;
        }

        #[cfg(not(feature = "simd"))]
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all devices to the sparse matrix for O(1) stamping.
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        // Link individual devices
        for d in &mut self.devices {
            d.link(matrix);
        }

        // Build batches for supported levels
        #[cfg(feature = "simd")]
        self.build_batches(matrix);
    }

    /// Build batch representations for supported MOSFET levels.
    #[cfg(feature = "simd")]
    fn build_batches(&mut self, matrix: &StaticMatrix) {
        self.batched_indices.clear();

        // Count devices by level
        let level1_count = self.devices.iter().filter(|d| d.level == 1).count();
        let level6_count = self.devices.iter().filter(|d| d.level == 6).count();

        // Build Level 1 batch
        if level1_count >= 4 {
            let mut batch = crate::device::batch_mosfet::BatchMosfets::with_capacity(level1_count);
            for (i, d) in self.devices.iter().enumerate() {
                if d.level == 1 {
                    batch.add(
                        d.node_drain,
                        d.node_gate,
                        d.node_source,
                        d.node_bulk,
                        d.mos_type,
                        d.beta(),
                        d.vto,
                        d.gamma,
                        d.phi,
                        d.lambda,
                    );
                    self.batched_indices.push(i);
                }
            }
            batch.link(matrix);
            self.batch_level1 = Some(batch);
        } else {
            self.batch_level1 = None;
        }

        // Build Level 6 batch
        if level6_count >= 4 {
            let mut batch =
                crate::device::batch_mosfet_level6::BatchMosfetsLevel6::with_capacity(level6_count);
            for (i, d) in self.devices.iter().enumerate() {
                if d.level == 6 {
                    batch.add(
                        d.node_drain,
                        d.node_gate,
                        d.node_source,
                        d.node_bulk,
                        d.mos_type,
                        d.wl_ratio(),
                        d.vto,
                        d.gamma,
                        d.phi,
                        d.kc,
                        d.nc,
                        d.kv,
                        d.nv,
                        d.lambda0,
                        d.lambda1,
                    );
                    self.batched_indices.push(i);
                }
            }
            batch.link(matrix);
            self.batch_level6 = Some(batch);
        } else {
            self.batch_level6 = None;
        }
    }

    /// Stamp all devices using O(1) direct indexing.
    pub fn stamp_all_direct(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        #[cfg(feature = "simd")]
        {
            // Batch process Level 1
            if let Some(ref mut batch) = self.batch_level1 {
                batch.gather_voltages(voltages);
                batch.evaluate();
                batch.stamp(matrix, rhs);
            }
            // Batch process Level 6
            if let Some(ref mut batch) = self.batch_level6 {
                batch.gather_voltages(voltages);
                batch.evaluate();
                batch.stamp(matrix, rhs);
            }
            // Stamp non-batched devices individually
            for (i, d) in self.devices.iter().enumerate() {
                if !self.batched_indices.contains(&i) {
                    d.stamp_direct(matrix, rhs, voltages);
                }
            }
            return;
        }

        #[cfg(not(feature = "simd"))]
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

//=============================================================================
// Custom BJT Storage with SIMD Batch Support
//=============================================================================

/// BJT storage for nonlinear Newton-Raphson iteration.
///
/// When compiled with the `simd` feature, maintains a batch representation
/// for SIMD-accelerated evaluation.
#[derive(Debug, Default)]
pub struct Bjts {
    /// Individual BJT devices
    pub devices: Vec<Bjt>,

    /// Batch representation for SIMD acceleration
    #[cfg(feature = "simd")]
    batch: Option<crate::device::batch_bjt::BatchBjts>,
}

impl Bjts {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add(&mut self, device: Bjt) {
        self.devices.push(device);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all devices from node voltages.
    pub fn update_all(&mut self, voltages: &[Value]) {
        #[cfg(feature = "simd")]
        if let Some(ref mut batch) = self.batch {
            batch.gather_voltages(voltages);
            batch.evaluate();
            return;
        }

        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all devices into matrix and RHS.
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all devices have converged.
    pub fn all_converged(&self, tolerance: Value) -> bool {
        #[cfg(feature = "simd")]
        if let Some(ref batch) = self.batch {
            return batch.all_converged(tolerance);
        }

        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all devices to the sparse matrix.
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }

        #[cfg(feature = "simd")]
        self.build_batch(matrix);
    }

    /// Build batch representation.
    #[cfg(feature = "simd")]
    fn build_batch(&mut self, matrix: &StaticMatrix) {
        if self.devices.len() < 4 {
            self.batch = None;
            return;
        }

        let mut batch = crate::device::batch_bjt::BatchBjts::with_capacity(self.devices.len());

        for d in &self.devices {
            batch.add(
                d.node_collector,
                d.node_base,
                d.node_emitter,
                d.bjt_type,
                d.is,
                d.nf,
                d.nr,
                d.vt,
                d.bf,
                d.br,
                d.vaf,
                d.ikf,
            );
        }

        batch.link(matrix);
        self.batch = Some(batch);
    }

    /// Stamp using direct indexing.
    pub fn stamp_all_direct(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        #[cfg(feature = "simd")]
        if let Some(ref mut batch) = self.batch {
            batch.gather_voltages(voltages);
            batch.evaluate();
            batch.stamp(matrix, rhs);
            return;
        }

        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

//=============================================================================
// Custom JFET Storage with SIMD Batch Support
//=============================================================================

use crate::device::Jfet;

/// JFET storage for nonlinear Newton-Raphson iteration.
///
/// When compiled with the `simd` feature, maintains a batch representation
/// for SIMD-accelerated evaluation.
#[derive(Debug, Default)]
pub struct Jfets {
    /// Individual JFET devices
    pub devices: Vec<Jfet>,

    /// Batch representation for SIMD acceleration
    #[cfg(feature = "simd")]
    batch: Option<crate::device::batch_jfet::BatchJfets>,
}

impl Jfets {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add(&mut self, device: Jfet) {
        self.devices.push(device);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all devices from node voltages.
    pub fn update_all(&mut self, voltages: &[Value]) {
        #[cfg(feature = "simd")]
        if let Some(ref mut batch) = self.batch {
            batch.gather_voltages(voltages);
            batch.evaluate();
            return;
        }

        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all devices into matrix and RHS.
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all devices have converged.
    pub fn all_converged(&self, tolerance: Value) -> bool {
        #[cfg(feature = "simd")]
        if let Some(ref batch) = self.batch {
            return batch.all_converged(tolerance);
        }

        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all devices to the sparse matrix.
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }

        #[cfg(feature = "simd")]
        self.build_batch(matrix);
    }

    /// Build batch representation.
    #[cfg(feature = "simd")]
    fn build_batch(&mut self, matrix: &StaticMatrix) {
        if self.devices.len() < 4 {
            self.batch = None;
            return;
        }

        let mut batch = crate::device::batch_jfet::BatchJfets::with_capacity(self.devices.len());
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;

        for d in &self.devices {
            let vt = K_BOLTZMANN * d.params.tnom / Q_ELECTRON;
            batch.add(
                d.drain,
                d.gate,
                d.source,
                d.jfet_type,
                d.params.vto,
                d.params.beta,
                d.params.lambda,
                d.params.eta,
                d.params.sigma0,
                d.params.is,
                d.params.n * vt,
                d.m * d.area,
            );
        }

        batch.link(matrix);
        self.batch = Some(batch);
    }

    /// Stamp using direct indexing.
    pub fn stamp_all_direct(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        #[cfg(feature = "simd")]
        if let Some(ref mut batch) = self.batch {
            batch.gather_voltages(voltages);
            batch.evaluate();
            batch.stamp(matrix, rhs);
            return;
        }

        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

//=============================================================================
// VDMOS Storage (Individual Processing - Thermal Model)
//=============================================================================

use crate::device::Vdmos;

/// VDMOS storage for nonlinear Newton-Raphson iteration.
///
/// VDMOS devices use individual processing due to the complexity of the
/// thermal network and body diode recovery models which have device-specific
/// state that doesn't benefit from SIMD vectorization.
#[derive(Debug, Default)]
pub struct Vdmoss {
    /// Individual VDMOS devices
    pub devices: Vec<Vdmos>,
}

impl Vdmoss {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add(&mut self, device: Vdmos) {
        self.devices.push(device);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all devices from node voltages.
    pub fn update_all(&mut self, voltages: &[Value]) {
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all devices into matrix and RHS.
    pub fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all devices have converged.
    pub fn all_converged(&self, tolerance: Value) -> bool {
        self.devices.iter().all(|d| d.is_converged(tolerance))
    }

    /// Link all devices to the sparse matrix.
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp using direct indexing.
    pub fn stamp_all_direct(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}
