//! Circuit representation with Struct-of-Arrays design and pre-indexed matrix stamping
//!
//! This module provides high-performance circuit storage using data-oriented design:
//! - Struct-of-Arrays (SoA) for cache-friendly iteration
//! - Pre-computed matrix indices for zero-cost stamping in the hot loop
//! - Separation of topology (static) from values (mutable)

#![allow(clippy::too_many_arguments)]
use crate::Value;
use crate::analysis::{CompanionCoefficients, IntegrationMethod};
use crate::device::behavioral::BehavioralSources;
use crate::device::{Bjt, Cccs, Ccvs, Diode, MatrixStamper, Mosfet, Vccs, Vcvs};
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};
use crate::xspice::{CodeModelRegistry, XspiceInstance};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock, RwLock};
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
    /// Small-signal conductances used by AC/PZ/noise analyses.
    pub small_signal_conductances: Vec<Value>,
}

impl Resistors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, resistance: Value) {
        self.add_with_small_signal(name, node_pos, node_neg, resistance, resistance);
    }

    pub fn add_with_small_signal(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        resistance: Value,
        small_signal_resistance: Value,
    ) {
        self.names.push(name);
        self.stamps.push(TwoTerminalStamp::new(node_pos, node_neg));
        self.conductances.push(1.0 / resistance);
        self.small_signal_conductances
            .push(1.0 / small_signal_resistance);
    }

    #[inline]
    pub fn small_signal_conductance(&self, index: usize) -> Value {
        self.small_signal_conductances
            .get(index)
            .copied()
            .unwrap_or_else(|| self.conductances.get(index).copied().unwrap_or(0.0))
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

    /// Initialize capacitor voltages from DC solution
    ///
    /// This is critical for correct transient startup. Capacitors without explicit
    /// IC= values should start with the DC voltage across them. Otherwise, coupling
    /// capacitors cause massive startup current spikes.
    pub fn set_initial_voltages_from_dc(&mut self, solution: &[Value]) {
        for (i, stamp) in self.stamps.iter().enumerate() {
            // Only set if no explicit IC was provided
            if self.ic[i].is_none() {
                let v_pos = if stamp.pp.row != 0 {
                    solution[stamp.pp.row - 1]
                } else {
                    0.0
                };
                let v_neg = if stamp.nn.row != 0 {
                    solution[stamp.nn.row - 1]
                } else {
                    0.0
                };
                let v_dc = v_pos - v_neg;
                self.v_prev[i] = v_dc;
                self.v_prev_prev[i] = v_dc;
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
    /// Optional transient context used to resolve source defaults.
    transient_context: Option<TransientSourceContext>,
}

#[derive(Debug, Clone, Copy)]
struct TransientSourceContext {
    tstep: Value,
    tstop: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PwlCacheKey {
    path: String,
    time_scale_bits: u64,
    value_scale_bits: u64,
    time_offset_bits: u64,
    value_offset_bits: u64,
}

impl PwlCacheKey {
    fn new(
        path: &str,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
    ) -> Self {
        Self {
            path: path.to_string(),
            time_scale_bits: time_scale.to_bits(),
            value_scale_bits: value_scale.to_bits(),
            time_offset_bits: time_offset.to_bits(),
            value_offset_bits: value_offset.to_bits(),
        }
    }
}

fn pwl_waveform_cache()
-> &'static RwLock<HashMap<PwlCacheKey, Arc<crate::device::pwl_file::PwlWaveform>>> {
    static CACHE: OnceLock<
        RwLock<HashMap<PwlCacheKey, Arc<crate::device::pwl_file::PwlWaveform>>>,
    > = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn pwl_error_log_cache() -> &'static RwLock<HashSet<PwlCacheKey>> {
    static CACHE: OnceLock<RwLock<HashSet<PwlCacheKey>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashSet::new()))
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

    /// Set transient context used to resolve waveform defaults.
    pub fn set_transient_context(&mut self, tstep: Value, tstop: Value) {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        self.transient_context = Some(TransientSourceContext {
            tstep: step,
            tstop: stop,
        });
    }

    /// Clear transient context and use static waveform defaults.
    pub fn clear_transient_context(&mut self) {
        self.transient_context = None;
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
        let context = self.transient_context;
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);

            let v = match &self.source_specs[i] {
                Some(spec) => Self::evaluate_source_at_time_with_context(spec, time, context),
                None => self.dc_values[i], // DC only
            };

            rhs[br - 1] = v;
        }
    }

    /// Maximum absolute change expected from time-varying sources over [t0, t1].
    #[inline]
    pub fn max_expected_delta(&self, t0: Value, t1: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .filter_map(|spec| spec.as_ref())
            .map(|spec| {
                (Self::evaluate_source_at_time_with_context(spec, t1, context)
                    - Self::evaluate_source_at_time_with_context(spec, t0, context))
                .abs()
            })
            .fold(0.0, Value::max)
    }

    fn load_pwl_waveform_cached(
        path: &str,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
    ) -> Result<Arc<crate::device::pwl_file::PwlWaveform>, String> {
        let key = PwlCacheKey::new(path, time_scale, value_scale, time_offset, value_offset);

        if let Ok(cache) = pwl_waveform_cache().read()
            && let Some(wf) = cache.get(&key)
        {
            return Ok(Arc::clone(wf));
        }

        let waveform = crate::device::pwl_file::load_pwl_file(path)
            .map_err(|e| format!("failed to load PWL file '{}': {}", path, e))?
            .with_scaling(time_scale, value_scale, time_offset, value_offset);
        let waveform = Arc::new(waveform);

        if let Ok(mut cache) = pwl_waveform_cache().write() {
            let entry = cache.entry(key).or_insert_with(|| Arc::clone(&waveform));
            return Ok(Arc::clone(entry));
        }

        Ok(waveform)
    }

    fn log_pwl_error_once(key: PwlCacheKey, msg: &str) {
        if let Ok(mut logged) = pwl_error_log_cache().write() {
            if logged.insert(key) {
                log::warn!("{}", msg);
            }
            return;
        }
        log::warn!("{}", msg);
    }

    #[inline]
    fn pulse_step_default(context: Option<TransientSourceContext>) -> Value {
        context.map(|ctx| ctx.tstep).unwrap_or(1e-12).max(1e-18)
    }

    #[inline]
    fn pulse_stop_default(context: Option<TransientSourceContext>) -> Value {
        context.map(|ctx| ctx.tstop).unwrap_or(1e99).max(1e-18)
    }

    #[inline]
    fn resolve_pulse_timing(
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value, Value, Value, Value) {
        let step_default = Self::pulse_step_default(context);
        let stop_default = Self::pulse_stop_default(context);

        let td = if delay.is_finite() {
            delay.max(0.0)
        } else {
            0.0
        };
        let tr = if rise.is_nan() { step_default } else { rise };
        let tf = if fall.is_nan() { step_default } else { fall };
        let pw = if width.is_nan() { stop_default } else { width };
        let per = if period.is_nan() {
            stop_default
        } else {
            period
        };

        let tr = if tr.is_finite() && tr > 0.0 {
            tr
        } else {
            step_default
        };
        let tf = if tf.is_finite() && tf > 0.0 {
            tf
        } else {
            step_default
        };
        let pw = if pw.is_finite() && pw >= 0.0 {
            pw
        } else {
            stop_default
        };
        let per = if per.is_finite() && per > 0.0 {
            per
        } else {
            stop_default
        };

        (td, tr, tf, pw, per)
    }

    #[inline]
    fn evaluate_source_at_time_with_context(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        context: Option<TransientSourceContext>,
    ) -> Value {
        use crate::netlist::SourceSpec;
        use std::f64::consts::PI;

        match spec {
            SourceSpec::Dc(v) => *v,
            SourceSpec::Ac { .. } => 0.0, // AC sources are DC=0 in transient
            SourceSpec::DcAc { dc_value, .. } => *dc_value,
            SourceSpec::DcTransient { transient, .. } => {
                Self::evaluate_source_at_time_with_context(transient, time, context)
            }
            SourceSpec::DcAcTransient { transient, .. } => {
                Self::evaluate_source_at_time_with_context(transient, time, context)
            }
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
            } => {
                let (delay, rise, fall, width, period) =
                    Self::resolve_pulse_timing(*delay, *rise, *fall, *width, *period, context);
                if time < delay {
                    return *v1;
                }
                let t_rel = time - delay;
                let t = if period.is_finite() && period > 0.0 {
                    t_rel.rem_euclid(period)
                } else {
                    t_rel
                };
                if t < rise {
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
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
            } => {
                let key =
                    PwlCacheKey::new(path, *time_scale, *value_scale, *time_offset, *value_offset);
                match Self::load_pwl_waveform_cached(
                    path,
                    *time_scale,
                    *value_scale,
                    *time_offset,
                    *value_offset,
                ) {
                    Ok(waveform) => waveform.value_at(time),
                    Err(err) => {
                        Self::log_pwl_error_once(key, &err);
                        *value_offset
                    }
                }
            }
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

    /// Enforce voltage source constraints on solution vector after force-accept
    ///
    /// When Newton iteration fails to converge and we force-accept a solution,
    /// the voltage source node values may not satisfy V(n+) - V(n-) = Vs.
    /// This method corrects the solution vector to enforce this constraint
    /// for display purposes and to prevent drift.
    pub fn enforce_voltage_constraints(&self, solution: &mut [Value], time: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];

            // Get the source value at this time
            let v_source = match &self.source_specs[i] {
                Some(spec) => {
                    Self::evaluate_source_at_time_with_context(spec, time, self.transient_context)
                }
                None => self.dc_values[i],
            };

            // If negative node is ground (nn=0), positive node voltage = source voltage
            // If negative node is not ground, we can only correct if we know its voltage
            if nn == 0 && np > 0 {
                // V(np) - V(ground) = Vs  =>  V(np) = Vs
                if let Some(v) = solution.get_mut(np - 1) {
                    *v = v_source;
                }
            } else if np == 0 && nn > 0 {
                // V(ground) - V(nn) = Vs  =>  V(nn) = -Vs
                if let Some(v) = solution.get_mut(nn - 1) {
                    *v = -v_source;
                }
            }
            // For floating voltage sources (neither terminal grounded),
            // we cannot unambiguously correct the node voltages
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
    /// Full source specification for transient waveform evaluation
    pub source_specs: Vec<Option<crate::netlist::SourceSpec>>,
    /// Optional transient context used to resolve source defaults.
    transient_context: Option<TransientSourceContext>,
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
        self.source_specs.push(None);
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
        self.source_specs.push(None);
    }

    /// Add current source with AC and transient specification.
    pub fn add_with_ac_and_spec(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(source_spec);
    }

    /// Set transient context used to resolve waveform defaults.
    pub fn set_transient_context(&mut self, tstep: Value, tstop: Value) {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        self.transient_context = Some(TransientSourceContext {
            tstep: step,
            tstop: stop,
        });
    }

    /// Clear transient context and use static waveform defaults.
    pub fn clear_transient_context(&mut self) {
        self.transient_context = None;
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

    /// Update RHS contribution of time-varying current sources at transient time.
    ///
    /// `stamp_dc_direct` already stamped DC values, so this applies only the
    /// delta between waveform and DC.
    #[inline]
    pub fn update_transient_rhs(&self, rhs: &mut [Value], time: Value) {
        for i in 0..self.names.len() {
            let Some(spec) = self.source_specs[i].as_ref() else {
                continue;
            };

            let value = VoltageSources::evaluate_source_at_time_with_context(
                spec,
                time,
                self.transient_context,
            );
            let delta = value - self.dc_values[i];
            if delta == 0.0 {
                continue;
            }

            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            if np > 0 {
                rhs[np - 1] -= delta;
            }
            if nn > 0 {
                rhs[nn - 1] += delta;
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

    /// Stamp inductors for DC operating point.
    ///
    /// At DC, an ideal inductor is a short circuit:
    /// V(np) - V(nn) = 0 with unconstrained branch current.
    #[inline]
    pub fn stamp_dc_short_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            if np > 0 {
                matrix.add(br - 1, np - 1, 1.0);
                matrix.add(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(br - 1, nn - 1, -1.0);
                matrix.add(nn - 1, br - 1, -1.0);
            }

            rhs[br - 1] = 0.0;
        }
    }

    /// Stamp inductors for DC operating point (triplet path).
    #[inline]
    pub fn stamp_dc_short(&self, matrix: &mut TripletMatrix, rhs: &mut [Value], num_nodes: usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            if np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }

            rhs[br - 1] = 0.0;
        }
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
            // L: v = L*di/dt â†’ v_n+1 - v_eq = r_eq * i_n+1
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
            self.i_prev[i] += (dt / (2.0 * l)) * (v + self.v_prev[i]);
            self.v_prev[i] = v;

            // Also get branch current from solution for accuracy
            if br > 0
                && let Some(&i_br) = currents.get(br - 1)
            {
                self.i_prev[i] = i_br;
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
    pub(crate) couplings: Vec<crate::device::InductorCoupling>,
    pub(crate) coupled_inductor_pairs: Vec<CoupledInductorPairBinding>,
    pub(crate) multi_winding_transformers: Vec<MultiWindingTransformerBinding>,
    pub(crate) jiles_atherton_inductors: Vec<JilesAthertonBinding>,

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

impl CircuitData {
    pub fn new() -> Self {
        let mut node_map = HashMap::new();
        // Ground is always node 0
        node_map.insert("0".to_string(), 0);
        node_map.insert("gnd".to_string(), 0);
        node_map.insert("GND".to_string(), 0);

        Self {
            node_map,
            has_explicit_ground_reference: false,
            branch_names: HashMap::new(),
            branch_name_by_ordinal: Vec::new(),
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
            pending_iswitch: Vec::new(),
            // New device types
            vswitches: Vec::new(),
            iswitches: Vec::new(),
            tlines: Vec::new(),
            couplings: Vec::new(),
            coupled_inductor_pairs: Vec::new(),
            multi_winding_transformers: Vec::new(),
            jiles_atherton_inductors: Vec::new(),
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
        if Self::is_ground_name(name) {
            self.has_explicit_ground_reference = true;
            return 0;
        }

        if let Some(&id) = self.node_map.get(name) {
            return id;
        }

        self.num_nodes += 1;
        self.node_map.insert(name.to_string(), self.num_nodes);
        self.num_nodes
    }

    #[inline]
    fn is_ground_name(name: &str) -> bool {
        name == "0" || name.eq_ignore_ascii_case("gnd")
    }

    /// Look up an existing node ID by name.
    pub fn get_node_by_name(&self, name: &str) -> Option<NodeId> {
        self.node_map
            .get(name)
            .copied()
            .or_else(|| self.node_map.get(&name.to_lowercase()).copied())
            .or_else(|| self.node_map.get(&name.to_uppercase()).copied())
            .or_else(|| {
                self.node_map
                    .iter()
                    .find_map(|(candidate, &id)| candidate.eq_ignore_ascii_case(name).then_some(id))
            })
    }

    /// Resolve behavioral expression references against current node/branch maps.
    pub fn bind_behavioral_references(&mut self) -> Result<(), CircuitError> {
        let node_lookup = self.node_map.clone();
        let branch_lookup = self.branch_names.clone();
        let num_nodes = self.num_nodes;
        self.behavioral_sources
            .bind_references(
                |name: &str| {
                    node_lookup
                        .get(name)
                        .copied()
                        .or_else(|| node_lookup.get(&name.to_lowercase()).copied())
                        .or_else(|| node_lookup.get(&name.to_uppercase()).copied())
                        .or_else(|| {
                            node_lookup.iter().find_map(|(candidate, &id)| {
                                candidate.eq_ignore_ascii_case(name).then_some(id)
                            })
                        })
                },
                |name: &str| {
                    branch_lookup
                        .get(name)
                        .or_else(|| branch_lookup.get(&name.to_uppercase()))
                        .copied()
                        .map(|ordinal| num_nodes + ordinal - 1)
                },
            )
            .map_err(CircuitError::InvalidComponent)
    }

    /// Check whether the circuit explicitly referenced the SPICE ground node.
    pub fn has_explicit_ground_reference(&self) -> bool {
        self.has_explicit_ground_reference
    }

    /// Ensure a ground reference exists. If no explicit node "0" was specified,
    /// pick the first node connected to a voltage source's negative terminal
    /// as the reference.
    /// This should be called after all elements are added but before simulation.
    pub fn ensure_ground_reference(&mut self) {
        if self.has_explicit_ground_reference() {
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
        // Update node map
        for (_, id) in self.node_map.iter_mut() {
            *id = Self::remap_node_id(*id, old_node_id);
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
        Self::remap_node_slice(&mut self.voltage_sources.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.voltage_sources.node_neg, old_node_id);

        // Current sources
        Self::remap_node_slice(&mut self.current_sources.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.current_sources.node_neg, old_node_id);

        // Inductors
        Self::remap_node_slice(&mut self.inductors.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.inductors.node_neg, old_node_id);
        for diode in &mut self.diodes.devices {
            diode.node_anode = Self::remap_node_id(diode.node_anode, old_node_id);
            diode.node_cathode = Self::remap_node_id(diode.node_cathode, old_node_id);
        }
        for bjt in &mut self.bjts.devices {
            bjt.node_collector = Self::remap_node_id(bjt.node_collector, old_node_id);
            bjt.node_base = Self::remap_node_id(bjt.node_base, old_node_id);
            bjt.node_emitter = Self::remap_node_id(bjt.node_emitter, old_node_id);
            bjt.node_substrate = Self::remap_node_id(bjt.node_substrate, old_node_id);
        }
        for mosfet in &mut self.mosfets.devices {
            mosfet.node_drain = Self::remap_node_id(mosfet.node_drain, old_node_id);
            mosfet.node_gate = Self::remap_node_id(mosfet.node_gate, old_node_id);
            mosfet.node_source = Self::remap_node_id(mosfet.node_source, old_node_id);
            mosfet.node_bulk = Self::remap_node_id(mosfet.node_bulk, old_node_id);
        }
        for jfet in &mut self.jfets {
            jfet.drain = Self::remap_node_id(jfet.drain, old_node_id);
            jfet.gate = Self::remap_node_id(jfet.gate, old_node_id);
            jfet.source = Self::remap_node_id(jfet.source, old_node_id);
        }
        Self::remap_node_slice(&mut self.vcvs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.vcvs.node_neg, old_node_id);
        Self::remap_node_slice(&mut self.vcvs.ctrl_pos, old_node_id);
        Self::remap_node_slice(&mut self.vcvs.ctrl_neg, old_node_id);
        Self::remap_node_slice(&mut self.vccs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.vccs.node_neg, old_node_id);
        Self::remap_node_slice(&mut self.vccs.ctrl_pos, old_node_id);
        Self::remap_node_slice(&mut self.vccs.ctrl_neg, old_node_id);
        Self::remap_node_slice(&mut self.cccs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.cccs.node_neg, old_node_id);
        Self::remap_node_slice(&mut self.ccvs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.ccvs.node_neg, old_node_id);
        for switch in &mut self.vswitches {
            switch.node_pos = Self::remap_node_id(switch.node_pos, old_node_id);
            switch.node_neg = Self::remap_node_id(switch.node_neg, old_node_id);
            switch.ctrl_pos = Self::remap_node_id(switch.ctrl_pos, old_node_id);
            switch.ctrl_neg = Self::remap_node_id(switch.ctrl_neg, old_node_id);
        }
        for switch in &mut self.iswitches {
            switch.node_pos = Self::remap_node_id(switch.node_pos, old_node_id);
            switch.node_neg = Self::remap_node_id(switch.node_neg, old_node_id);
        }
        for tline in &mut self.tlines {
            tline.node1_pos = Self::remap_node_id(tline.node1_pos, old_node_id);
            tline.node1_neg = Self::remap_node_id(tline.node1_neg, old_node_id);
            tline.node2_pos = Self::remap_node_id(tline.node2_pos, old_node_id);
            tline.node2_neg = Self::remap_node_id(tline.node2_neg, old_node_id);
        }
        for binding in &mut self.coupled_inductor_pairs {
            binding.device.node1_pos = Self::remap_node_id(binding.device.node1_pos, old_node_id);
            binding.device.node1_neg = Self::remap_node_id(binding.device.node1_neg, old_node_id);
            binding.device.node2_pos = Self::remap_node_id(binding.device.node2_pos, old_node_id);
            binding.device.node2_neg = Self::remap_node_id(binding.device.node2_neg, old_node_id);
        }
        for binding in &mut self.multi_winding_transformers {
            for (pos, neg) in &mut binding.device.nodes {
                *pos = Self::remap_node_id(*pos, old_node_id);
                *neg = Self::remap_node_id(*neg, old_node_id);
            }
        }
        for binding in &mut self.jiles_atherton_inductors {
            binding.device.node_pos = Self::remap_node_id(binding.device.node_pos, old_node_id);
            binding.device.node_neg = Self::remap_node_id(binding.device.node_neg, old_node_id);
        }

        // Behavioral sources
        for source in &mut self.behavioral_sources.voltage_sources {
            source.node_pos = Self::remap_node_id(source.node_pos, old_node_id);
            source.node_neg = Self::remap_node_id(source.node_neg, old_node_id);
        }
        for source in &mut self.behavioral_sources.current_sources {
            source.node_pos = Self::remap_node_id(source.node_pos, old_node_id);
            source.node_neg = Self::remap_node_id(source.node_neg, old_node_id);
        }

        for instance in &mut self.xspice_instances {
            instance.remap_circuit_nodes(|node| Self::remap_node_id(node, old_node_id));
        }

        #[cfg(feature = "veriloga")]
        self.veriloga_devices
            .remap_circuit_nodes(|node| Self::remap_node_id(node, old_node_id));

        self.has_explicit_ground_reference = true;

        // Decrement num_nodes since one node is now ground
        if self.num_nodes > 0 {
            self.num_nodes -= 1;
        }
    }

    /// Helper to remap a two-terminal stamp with full shifting
    fn remap_stamp_full(stamp: &mut TwoTerminalStamp, old_id: NodeId) {
        stamp.pp.row = Self::remap_node_id(stamp.pp.row, old_id);
        stamp.pp.col = Self::remap_node_id(stamp.pp.col, old_id);
        stamp.pn.row = Self::remap_node_id(stamp.pn.row, old_id);
        stamp.pn.col = Self::remap_node_id(stamp.pn.col, old_id);
        stamp.np.row = Self::remap_node_id(stamp.np.row, old_id);
        stamp.np.col = Self::remap_node_id(stamp.np.col, old_id);
        stamp.nn.row = Self::remap_node_id(stamp.nn.row, old_id);
        stamp.nn.col = Self::remap_node_id(stamp.nn.col, old_id);
    }

    #[inline]
    fn remap_node_id(id: NodeId, old_id: NodeId) -> NodeId {
        if id == old_id {
            0
        } else if id > old_id {
            id - 1
        } else {
            id
        }
    }

    fn remap_node_slice(nodes: &mut [NodeId], old_id: NodeId) {
        for node in nodes {
            *node = Self::remap_node_id(*node, old_id);
        }
    }

    /// Allocate a branch current variable - returns branch ordinal (1-indexed)
    /// Note: The stored value is the branch ordinal, NOT the matrix index.
    /// Use get_branch_matrix_index() to get the actual matrix row/column.
    pub fn allocate_branch(&mut self) -> NodeId {
        self.num_branches += 1;
        self.branch_name_by_ordinal.push(None);
        self.num_branches // Return branch ordinal (1, 2, 3...)
    }

    /// Allocate a branch and register it with the given element name
    /// This allows CCCS/CCVS to look up control branches by name
    pub fn allocate_branch_named(&mut self, name: &str) -> NodeId {
        let branch = self.allocate_branch();
        if let Some(slot) = self.branch_name_by_ordinal.get_mut(branch - 1) {
            *slot = Some(name.to_string());
        }
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

    /// Return the set of branch-bearing element names that can be used as probes.
    pub fn branch_probe_names(&self) -> Vec<String> {
        let mut names = Vec::with_capacity(
            self.inductors.names.len()
                + self.voltage_sources.names.len()
                + self.ccvs.len()
                + self.behavioral_sources.voltage_sources.len(),
        );
        names.extend(self.inductors.names.iter().cloned());
        names.extend(self.voltage_sources.names.iter().cloned());
        names.extend(self.ccvs.names.iter().cloned());
        names.extend(
            self.behavioral_sources
                .voltage_sources
                .iter()
                .map(|source| source.name.clone()),
        );
        names
    }

    /// Return the canonical set of inductor probe names.
    pub fn inductor_probe_names(&self) -> Vec<String> {
        self.inductors.names.clone()
    }

    /// Resolve a probe name to the inductor state tracked during periodic RF analyses.
    pub fn resolve_inductor_probe(&self, probe_name: &str) -> Option<InductorProbeInfo> {
        let branch_ordinal = self.get_branch_by_name(probe_name)?;
        self.inductor_probe_for_branch(branch_ordinal)
    }

    /// Resolve an existing branch ordinal to the owning inductor probe metadata.
    pub fn inductor_probe_for_branch(&self, branch_ordinal: NodeId) -> Option<InductorProbeInfo> {
        let inductor_index = self
            .inductors
            .branch_indices
            .iter()
            .position(|branch| *branch == branch_ordinal)?;

        Some(InductorProbeInfo {
            canonical_name: self.inductors.names.get(inductor_index)?.clone(),
            branch_ordinal,
            state_index: self.capacitors.len() + inductor_index,
        })
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

    /// Register an ISWITCH element for pending control branch resolution.
    pub fn add_iswitch_pending(&mut self, iswitch_index: usize, control_element_name: String) {
        self.pending_iswitch
            .push((iswitch_index, control_element_name));
    }

    /// Register a Jiles-Atherton inductor runtime binding.
    pub fn add_jiles_atherton_inductor(
        &mut self,
        inductor_index: usize,
        branch_ordinal: NodeId,
        device: crate::device::passive::JilesAthertonInductor,
    ) {
        self.jiles_atherton_inductors.push(JilesAthertonBinding {
            inductor_index,
            branch_ordinal,
            device,
        });
    }

    /// Register a coupled inductor pair runtime binding.
    pub fn add_coupled_inductor_pair(
        &mut self,
        branch1_ordinal: NodeId,
        branch2_ordinal: NodeId,
        device: crate::device::CoupledInductorPair,
    ) {
        self.coupled_inductor_pairs
            .push(CoupledInductorPairBinding {
                branch1_ordinal,
                branch2_ordinal,
                device,
            });
    }

    /// Register a multi-winding transformer runtime binding.
    pub fn add_multi_winding_transformer(
        &mut self,
        branch_ordinals: Vec<NodeId>,
        device: crate::device::MultiWindingTransformer,
    ) {
        self.multi_winding_transformers
            .push(MultiWindingTransformerBinding {
                branch_ordinals,
                device,
            });
    }

    /// Resolve all pending CCCS/CCVS/ISWITCH control element references.
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

        // Resolve current-controlled switch control branches.
        // CurrentSwitch expects a matrix variable index (1-based) so convert
        // from branch ordinal after final node count is known.
        for (iswitch_idx, control_name) in self.pending_iswitch.drain(..).collect::<Vec<_>>() {
            let branch_ordinal = self.get_branch_by_name(&control_name).ok_or_else(|| {
                CircuitError::InvalidComponent(format!(
                    "ISWITCH control element not found: {}",
                    control_name
                ))
            })?;
            let branch_matrix_index = self.get_branch_matrix_index(branch_ordinal);
            if let Some(sw) = self.iswitches.get_mut(iswitch_idx) {
                sw.set_ctrl_branch(branch_matrix_index);
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

    /// Refresh effective inductance values for all Jiles-Atherton inductors.
    ///
    /// Call this with the latest solution vector before transient companion
    /// stamping so nonlinear core state updates feed into the MNA coefficients.
    pub fn refresh_jiles_atherton_inductances(&mut self, solution: &[Value]) {
        use crate::device::NonlinearDevice;

        let num_nodes = self.num_nodes;
        for idx in 0..self.jiles_atherton_inductors.len() {
            let (inductor_index, l_eff) = {
                let binding = &mut self.jiles_atherton_inductors[idx];
                let branch_matrix_index = num_nodes + binding.branch_ordinal;
                binding.device.set_branch_index(branch_matrix_index);
                binding.device.update(solution);
                (
                    binding.inductor_index,
                    binding.device.effective_inductance(),
                )
            };

            if let Some(slot) = self.inductors.inductances.get_mut(inductor_index)
                && l_eff.is_finite()
                && l_eff > 0.0
            {
                *slot = l_eff.max(1e-18);
            }
        }
    }

    /// Stamp coupled inductor companion models for transient analysis.
    pub fn stamp_coupled_inductor_pairs_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            binding
                .device
                .stamp_transient_companion(dt, coeff, &mut stamper, &mut []);
        }
    }

    /// Stamp multi-winding transformer companion models for transient analysis.
    pub fn stamp_multi_winding_transformers_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .stamp_transient_companion(dt, coeff, &mut stamper, &mut []);
        }
    }

    /// Update coupled inductor transient history from an accepted solution.
    pub fn update_coupled_inductor_pair_state(&mut self, solution: &[Value]) {
        for binding in &mut self.coupled_inductor_pairs {
            binding.device.update_state_from_solution(solution);
        }
    }

    /// Update multi-winding transformer transient history from an accepted solution.
    pub fn update_multi_winding_transformer_state(&mut self, solution: &[Value]) {
        for binding in &mut self.multi_winding_transformers {
            binding.device.update_state_from_solution(solution);
        }
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

    /// Get branch names sorted by their branch ordinal (1, 2, 3, ...).
    /// Returns a Vec where index i contains the canonical name of branch (i+1).
    pub fn branch_names_sorted(&self) -> Vec<String> {
        self.branch_name_by_ordinal
            .iter()
            .enumerate()
            .map(|(idx, name)| name.clone().unwrap_or_else(|| format!("BRANCH{}", idx + 1)))
            .collect()
    }

    /// Total device count (for parallel stamping threshold)
    pub fn device_count(&self) -> usize {
        let count = self.resistors.len()
            + self.capacitors.len()
            + self.inductors.len()
            + self.voltage_sources.len()
            + self.current_sources.len()
            + self.diodes.len()
            + self.bjts.len()
            + self.mosfets.len()
            + self.jfets.len()
            + self.vcvs.len()
            + self.vccs.len()
            + self.cccs.len()
            + self.ccvs.len()
            + self.coupled_inductor_pairs.len()
            + self.multi_winding_transformers.len()
            + self.jiles_atherton_inductors.len();
        #[cfg(feature = "veriloga")]
        {
            count + self.veriloga_device_count()
        }
        #[cfg(not(feature = "veriloga"))]
        {
            count
        }
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
        for jfet in &mut self.jfets {
            jfet.link(matrix);
        }
        for binding in &mut self.coupled_inductor_pairs {
            let branch1_matrix_index = self.num_nodes + binding.branch1_ordinal;
            let branch2_matrix_index = self.num_nodes + binding.branch2_ordinal;
            binding
                .device
                .set_branches(branch1_matrix_index, branch2_matrix_index);
        }
        for binding in &mut self.multi_winding_transformers {
            let branches: Vec<NodeId> = binding
                .branch_ordinals
                .iter()
                .map(|branch_ordinal| self.num_nodes + *branch_ordinal)
                .collect();
            binding.device.set_branches(branches);
        }
        for binding in &mut self.jiles_atherton_inductors {
            let branch_matrix_index = self.num_nodes + binding.branch_ordinal;
            binding.device.set_branch_index(branch_matrix_index);
        }
    }

    #[inline]
    fn stamp_tline_port_direct(
        matrix: &mut StaticMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        g: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -g);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -g);
            }
            matrix.add(node_neg - 1, node_neg - 1, g);
        }
    }

    #[inline]
    fn stamp_tline_port_triplet(
        matrix: &mut TripletMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        g: Value,
    ) {
        if node_pos > 0 {
            matrix.push(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.push(node_pos - 1, node_neg - 1, -g);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.push(node_neg - 1, node_pos - 1, -g);
            }
            matrix.push(node_neg - 1, node_neg - 1, g);
        }
    }

    #[inline]
    fn stamp_tlines_dc_direct(&self, matrix: &mut StaticMatrix) {
        for tl in &self.tlines {
            let g_series = tl.dc_series_conductance();
            // DC fallback: couple near/far conductors through equivalent series path.
            // This preserves operating-point continuity across the line and avoids
            // nonphysical port-to-ground shunts.
            Self::stamp_tline_port_direct(matrix, tl.node1_pos, tl.node2_pos, g_series);
            Self::stamp_tline_port_direct(matrix, tl.node1_neg, tl.node2_neg, g_series);
        }
    }

    #[inline]
    fn stamp_tlines_dc(&self, matrix: &mut TripletMatrix) {
        for tl in &self.tlines {
            let g_series = tl.dc_series_conductance();
            Self::stamp_tline_port_triplet(matrix, tl.node1_pos, tl.node2_pos, g_series);
            Self::stamp_tline_port_triplet(matrix, tl.node1_neg, tl.node2_neg, g_series);
        }
    }

    #[inline]
    fn stamp_coupled_inductors_dc_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            binding.device.stamp_dc_short(&mut stamper, &mut []);
        }
    }

    #[inline]
    fn stamp_coupled_inductors_dc(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        for binding in &self.coupled_inductor_pairs {
            let br1 = self.get_branch_matrix_index(binding.branch1_ordinal);
            let br2 = self.get_branch_matrix_index(binding.branch2_ordinal);
            let device = &binding.device;

            if device.node1_pos > 0 {
                matrix.push(br1 - 1, device.node1_pos - 1, 1.0);
                matrix.push(device.node1_pos - 1, br1 - 1, 1.0);
            }
            if device.node1_neg > 0 {
                matrix.push(br1 - 1, device.node1_neg - 1, -1.0);
                matrix.push(device.node1_neg - 1, br1 - 1, -1.0);
            }
            if device.node2_pos > 0 {
                matrix.push(br2 - 1, device.node2_pos - 1, 1.0);
                matrix.push(device.node2_pos - 1, br2 - 1, 1.0);
            }
            if device.node2_neg > 0 {
                matrix.push(br2 - 1, device.node2_neg - 1, -1.0);
                matrix.push(device.node2_neg - 1, br2 - 1, -1.0);
            }

            rhs[br1 - 1] = 0.0;
            rhs[br2 - 1] = 0.0;
        }
    }

    #[inline]
    fn stamp_multi_winding_transformers_dc_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.multi_winding_transformers {
            binding.device.stamp_dc_short(&mut stamper, &mut []);
        }
    }

    #[inline]
    fn stamp_multi_winding_transformers_dc(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        for binding in &self.multi_winding_transformers {
            for (winding_idx, &(pos, neg)) in binding.device.nodes.iter().enumerate() {
                let br = self.get_branch_matrix_index(binding.branch_ordinals[winding_idx]);
                if pos > 0 {
                    matrix.push(br - 1, pos - 1, 1.0);
                    matrix.push(pos - 1, br - 1, 1.0);
                }
                if neg > 0 {
                    matrix.push(br - 1, neg - 1, -1.0);
                    matrix.push(neg - 1, br - 1, -1.0);
                }
                rhs[br - 1] = 0.0;
            }
        }
    }

    /// Stamp all linear devices for DC analysis using O(1) direct stamping
    pub fn stamp_dc_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.inductors.stamp_dc_short_direct(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.voltage_sources
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all(rhs);

        // Stamp controlled sources
        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
        self.cccs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.ccvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);

        // Transmission-line DC fallback: couple near/far conductors via series path.
        self.stamp_tlines_dc_direct(matrix);
    }

    /// Stamp linear devices for transient Newton iterations.
    ///
    /// This intentionally excludes transmission-line DC fallback conductances:
    /// transient delay behavior is handled by dedicated tline companion stamps.
    pub fn stamp_transient_linear_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        self.resistors.stamp_all_direct(matrix);
        let num_nodes = self.num_nodes;
        self.inductors.stamp_dc_short_direct(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.voltage_sources
            .stamp_all_direct(matrix, rhs, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all(rhs);

        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
        self.cccs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.ccvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
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
        self.inductors.stamp_dc_short_direct(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc_direct(matrix, rhs);
        self.stamp_multi_winding_transformers_dc_direct(matrix, rhs);
        self.voltage_sources
            .stamp_all_direct_scaled(matrix, rhs, scale, |br_ordinal| num_nodes + br_ordinal);
        self.current_sources.stamp_all_scaled(rhs, scale);
        self.vcvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.vccs.stamp_all_direct(matrix);
        self.cccs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.ccvs
            .stamp_all_direct(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.stamp_tlines_dc_direct(matrix);
    }

    /// Stamp all linear devices for DC analysis
    pub fn stamp_dc(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        let num_nodes = self.num_nodes;
        self.resistors.stamp_all(matrix);
        self.inductors.stamp_dc_short(matrix, rhs, num_nodes);
        self.stamp_coupled_inductors_dc(matrix, rhs);
        self.stamp_multi_winding_transformers_dc(matrix, rhs);
        self.voltage_sources.stamp_all(matrix, rhs);
        self.current_sources.stamp_all(rhs);
        self.vcvs.stamp_all(matrix, num_nodes);
        self.vccs.stamp_all(matrix);
        self.cccs.stamp_all(matrix, num_nodes);
        self.ccvs.stamp_all(matrix, num_nodes);
        self.stamp_tlines_dc(matrix);
    }

    /// Check if circuit has any nonlinear devices requiring Newton-Raphson
    pub fn has_nonlinear_devices(&self) -> bool {
        !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.jfets.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || !self.behavioral_sources.is_empty()
            || self.has_xspice_devices()
            || {
                #[cfg(feature = "veriloga")]
                {
                    self.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    false
                }
            }
    }

    /// Check whether circuit contains strongly-coupled physical nonlinearities
    /// that benefit from conservative Newton damping (e.g., voltage limiting).
    #[inline]
    pub fn has_physical_nonlinear_devices(&self) -> bool {
        !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.jfets.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self.has_xspice_devices()
            || {
                #[cfg(feature = "veriloga")]
                {
                    self.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    false
                }
            }
    }

    /// Update all nonlinear devices with current solution
    pub fn update_nonlinear(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        self.diodes.update_all(voltages);
        self.bjts.update_all(voltages);
        self.mosfets.update_all(voltages);
        let mut order: Vec<usize> = (0..self.jfets.len()).collect();
        order.sort_by_key(|&idx| (self.jfets[idx].model_order(), idx));
        let mut hfet_inverse_latched = false;
        for idx in order {
            let jfet = &mut self.jfets[idx];
            let uses_hfet_legacy_inverse = jfet.uses_hfet_legacy_inverse_mode();
            jfet.set_hfet_legacy_inverse_active(uses_hfet_legacy_inverse && hfet_inverse_latched);
            jfet.update(voltages);
            if uses_hfet_legacy_inverse && jfet.internal_vds_limited_state() < 0.0 {
                hfet_inverse_latched = true;
            }
        }
        for vswitch in &mut self.vswitches {
            vswitch.update(voltages);
        }
        for iswitch in &mut self.iswitches {
            iswitch.update(voltages);
        }
        #[cfg(feature = "veriloga")]
        {
            self.veriloga_devices_mut().update_all_voltages(voltages);
        }
    }

    /// Stamp all nonlinear devices into matrix using O(1) direct indexing
    pub fn stamp_nonlinear(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        self.diodes.stamp_all_direct(matrix, rhs, voltages);
        self.bjts.stamp_all_direct(matrix, rhs, voltages);
        self.mosfets.stamp_all_direct(matrix, rhs, voltages);
        for jfet in &self.jfets {
            jfet.stamp_direct(matrix, rhs, voltages);
        }
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for vswitch in &self.vswitches {
            vswitch.stamp_nonlinear(voltages, &mut stamper, &mut []);
        }
        for iswitch in &self.iswitches {
            iswitch.stamp_nonlinear(voltages, &mut stamper, &mut []);
        }
        #[cfg(feature = "veriloga")]
        {
            let veriloga_devices = self.veriloga_devices_mut();
            veriloga_devices.update_all_voltages(voltages);
            veriloga_devices.stamp_all(
                voltages,
                |row, col, value| matrix.add(row, col, value),
                |index, value| {
                    if let Some(slot) = rhs.get_mut(index) {
                        *slot += value;
                    }
                },
            );
        }
    }

    /// Stamp behavioral sources with the given analysis time.
    pub fn stamp_behavioral(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) {
        self.behavioral_sources
            .stamp_all(matrix, rhs, solution, self.num_nodes, time);
    }

    /// Check if all nonlinear devices have converged
    pub fn nonlinear_converged(&self, tolerance: Value) -> bool {
        use crate::device::NonlinearDevice;
        self.diodes.all_converged(tolerance)
            && self.bjts.all_converged(tolerance)
            && self.mosfets.all_converged(tolerance)
            && self.jfets.iter().all(|jfet| jfet.is_converged(tolerance))
            && self.vswitches.iter().all(|sw| sw.is_converged(tolerance))
            && self.iswitches.iter().all(|sw| sw.is_converged(tolerance))
            && self.xspice_converged(tolerance)
    }

    //=========================================================================
    // XSPICE Code Model Interface
    //=========================================================================

    /// Check if circuit has any XSPICE code model instances
    #[inline]
    pub fn has_xspice_devices(&self) -> bool {
        !self.xspice_instances.is_empty()
    }

    #[cfg(feature = "veriloga")]
    #[inline]
    pub fn has_veriloga_devices(&self) -> bool {
        !self.veriloga_devices.is_empty()
    }

    #[cfg(feature = "veriloga")]
    #[inline]
    pub fn veriloga_device_count(&self) -> usize {
        self.veriloga_devices.len()
    }

    #[cfg(feature = "veriloga")]
    pub fn add_veriloga_device(&mut self, device: crate::device::veriloga::VerilogADevice) {
        self.veriloga_devices.add(device);
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn veriloga_devices(&self) -> &crate::device::veriloga::VerilogADevices {
        &self.veriloga_devices
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn veriloga_devices_mut(&mut self) -> &mut crate::device::veriloga::VerilogADevices {
        &mut self.veriloga_devices
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
        self.evaluate_xspice_with_analysis(
            time,
            0.0,
            voltages,
            crate::xspice::AnalysisType::Transient,
        );
    }

    /// Evaluate all XSPICE code model instances for transient with explicit timestep.
    pub fn evaluate_xspice_with_timestep(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
    ) {
        self.evaluate_xspice_with_analysis(
            time,
            timestep,
            voltages,
            crate::xspice::AnalysisType::Transient,
        );
    }

    /// Evaluate all XSPICE code model instances for the requested analysis type.
    pub fn evaluate_xspice_with_analysis(
        &mut self,
        time: Value,
        timestep: Value,
        voltages: &[Value],
        analysis: crate::xspice::AnalysisType,
    ) {
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
            if let Err(e) = instance.evaluate(time, timestep, analysis) {
                log::warn!("XSPICE evaluation error for {}: {}", instance.name, e);
            }
        }
    }

    /// Stamp XSPICE analog contributions into matrix and RHS
    ///
    /// After evaluation, analog code models produce conductance and current
    /// contributions that must be stamped into the MNA system.
    pub fn stamp_xspice(&mut self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        let num_nodes = self.num_nodes;

        #[inline]
        fn stamp_nodal_current_output(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            connection: &crate::xspice::PortConnection,
            conductance: Value,
            current: Value,
        ) {
            match connection {
                crate::xspice::PortConnection::Analog(node) => {
                    if *node > 0 {
                        matrix.add(*node - 1, *node - 1, conductance);
                        rhs[*node - 1] += current;
                    }
                }
                crate::xspice::PortConnection::Differential(pos, neg) => {
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

        for instance in &mut self.xspice_instances {
            let ports = instance.ports();
            // Get contributions from each output port
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                if let Some((conductance, current)) = instance.get_analog_contribution(port_idx) {
                    let Some(port) = ports.get(port_idx) else {
                        continue;
                    };
                    match port.default_type {
                        crate::xspice::PortType::Voltage
                        | crate::xspice::PortType::DifferentialVoltage => {
                            if let Some(branch_ordinal) = instance.branch_ordinal_at(port_idx) {
                                let br_mna = num_nodes + branch_ordinal;
                                let br = br_mna - 1;
                                match connection {
                                    crate::xspice::PortConnection::Analog(node) => {
                                        if *node > 0 {
                                            matrix.add(br, *node - 1, 1.0);
                                            matrix.add(*node - 1, br, 1.0);
                                        }
                                        rhs[br] += current;
                                    }
                                    crate::xspice::PortConnection::Differential(pos, neg) => {
                                        if *pos > 0 {
                                            matrix.add(br, *pos - 1, 1.0);
                                            matrix.add(*pos - 1, br, 1.0);
                                        }
                                        if *neg > 0 {
                                            matrix.add(br, *neg - 1, -1.0);
                                            matrix.add(*neg - 1, br, -1.0);
                                        }
                                        rhs[br] += current;
                                    }
                                    _ => {
                                        stamp_nodal_current_output(
                                            matrix,
                                            rhs,
                                            connection,
                                            conductance,
                                            current,
                                        );
                                    }
                                }
                            } else {
                                // Fallback for misconfigured instances: preserve behavior.
                                stamp_nodal_current_output(
                                    matrix,
                                    rhs,
                                    connection,
                                    conductance,
                                    current,
                                );
                            }
                        }
                        crate::xspice::PortType::Current => {
                            stamp_nodal_current_output(
                                matrix,
                                rhs,
                                connection,
                                conductance,
                                current,
                            );
                        }
                        _ => {}
                    }
                }
            }

            // Drain any explicit matrix/RHS stamps queued by the code model.
            for (row, col, value) in instance.take_deferred_stamps() {
                if row < rhs.len() && col < rhs.len() {
                    if matrix.get_index(row, col).is_some() {
                        matrix.add(row, col, value);
                    } else {
                        log::debug!(
                            "XSPICE deferred stamp ({}, {}) missing from matrix topology",
                            row,
                            col
                        );
                    }
                }
            }
            for (node, value) in instance.take_deferred_rhs() {
                if node < rhs.len() {
                    rhs[node] += value;
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
    use crate::device::{BehavioralVoltageSource, Bjt, Diode, Mosfet, VoltageSwitch};
    use crate::netlist::SourceSpec;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file_path(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rspice_{}_{}_{}.csv",
            name,
            std::process::id(),
            stamp
        ))
    }

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
    fn test_resistor_small_signal_override_is_tracked_separately() {
        let mut resistors = Resistors::new();
        resistors.add_with_small_signal("R1".to_string(), 1, 0, 10_000.0, 15_000.0);

        assert!((resistors.conductances[0] - 1.0 / 10_000.0).abs() < 1e-18);
        assert!(
            (resistors.small_signal_conductance(0) - 1.0 / 15_000.0).abs() < 1e-18,
            "AC/PZ/noise conductance should use the dedicated small-signal override"
        );
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
        assert!(circuit.has_explicit_ground_reference());
    }

    #[test]
    fn test_branch_probe_names_cover_supported_branch_families() {
        let mut circuit = CircuitData::new();
        let n1 = circuit.get_or_create_node("1");
        let n2 = circuit.get_or_create_node("2");

        let v_branch = circuit.allocate_branch_named("V1");
        let l_branch = circuit.allocate_branch_named("L1");
        let h_branch = circuit.allocate_branch_named("H1");
        let b_branch = circuit.allocate_branch_named("B1");

        circuit
            .voltage_sources
            .add("V1".to_string(), n1, 0, v_branch, 5.0);
        circuit
            .inductors
            .add("L1".to_string(), n1, n2, l_branch, 1e-9);
        circuit
            .ccvs
            .add("H1".to_string(), n2, 0, h_branch, v_branch, 10.0);
        circuit.behavioral_sources.add_voltage(
            BehavioralVoltageSource::new("B1".to_string(), n2, 0, b_branch, "1.0")
                .expect("behavioral source should parse"),
        );

        assert_eq!(
            circuit.branch_probe_names(),
            vec![
                "L1".to_string(),
                "V1".to_string(),
                "H1".to_string(),
                "B1".to_string(),
            ]
        );
    }

    #[test]
    fn test_resolve_inductor_probe_returns_canonical_name_and_state_index() {
        let mut circuit = CircuitData::new();
        let n1 = circuit.get_or_create_node("1");
        let n2 = circuit.get_or_create_node("2");

        circuit.capacitors.add("C1".to_string(), n1, 0, 1e-12);
        circuit.capacitors.add("C2".to_string(), n2, 0, 2e-12);

        let l_branch = circuit.allocate_branch_named("LPROBE");
        circuit
            .inductors
            .add("LPROBE".to_string(), n1, n2, l_branch, 1e-9);

        let probe = circuit
            .resolve_inductor_probe("lprobe")
            .expect("inductor probe should resolve case-insensitively");
        assert_eq!(probe.canonical_name, "LPROBE");
        assert_eq!(probe.branch_ordinal, l_branch);
        assert_eq!(probe.state_index, 2);
        assert!(circuit.resolve_inductor_probe("missing").is_none());
    }

    #[test]
    fn test_ensure_ground_reference_preserves_explicit_ground_on_nonlinear_path() {
        let mut circuit = CircuitData::new();
        let in_node = circuit.get_or_create_node("in");
        let ref_node = circuit.get_or_create_node("ref");
        let ground = circuit.get_or_create_node("0");
        let branch = circuit.allocate_branch_named("V1");

        circuit
            .voltage_sources
            .add("V1".to_string(), in_node, ref_node, branch, 1.0);
        circuit
            .diodes
            .add(Diode::new("D1".to_string(), ref_node, ground));

        circuit.ensure_ground_reference();

        assert_eq!(circuit.get_node_by_name("ref"), Some(ref_node));
        assert_eq!(circuit.get_node_by_name("in"), Some(in_node));
        assert_eq!(circuit.voltage_sources.node_neg[0], ref_node);
        assert_eq!(circuit.diodes.devices[0].node_cathode, 0);
        assert_eq!(circuit.num_nodes(), 2);
    }

    #[test]
    fn test_remap_node_to_ground_updates_diverse_device_families() {
        let mut circuit = CircuitData::new();
        let n1 = circuit.get_or_create_node("n1");
        let ref_node = circuit.get_or_create_node("ref");
        let ctrl = circuit.get_or_create_node("ctrl");
        let out = circuit.get_or_create_node("out");
        let aux = circuit.get_or_create_node("aux");

        let v_branch = circuit.allocate_branch_named("V1");
        let e_branch = circuit.allocate_branch_named("E1");
        let h_branch = circuit.allocate_branch_named("H1");
        let l_branch = circuit.allocate_branch_named("L1");

        circuit
            .voltage_sources
            .add("V1".to_string(), n1, ref_node, v_branch, 1.0);
        circuit
            .current_sources
            .add("I1".to_string(), ref_node, aux, 1.0);
        circuit
            .resistors
            .add("R1".to_string(), ref_node, out, 1_000.0);
        circuit
            .capacitors
            .add("C1".to_string(), out, ref_node, 1e-12);
        circuit
            .inductors
            .add("L1".to_string(), ref_node, aux, l_branch, 1e-9);
        circuit
            .diodes
            .add(Diode::new("D1".to_string(), ref_node, aux));
        circuit
            .bjts
            .add(Bjt::new_npn("Q1".to_string(), out, ctrl, ref_node));
        circuit
            .mosfets
            .add(Mosfet::new_nmos("M1".to_string(), out, ctrl, ref_node, aux));
        circuit
            .jfets
            .push(crate::device::Jfet::njf("J1", out, ctrl, ref_node));
        circuit
            .vcvs
            .add("E1".to_string(), out, ref_node, ctrl, aux, e_branch, 2.0);
        circuit
            .vccs
            .add("G1".to_string(), ref_node, out, ctrl, aux, 1e-3);
        circuit
            .cccs
            .add("F1".to_string(), ref_node, aux, v_branch, 5.0);
        circuit
            .ccvs
            .add("H1".to_string(), out, ref_node, h_branch, v_branch, 10.0);
        circuit.vswitches.push(VoltageSwitch::new(
            "S1".to_string(),
            ref_node,
            out,
            ctrl,
            aux,
        ));
        circuit.iswitches.push(crate::device::CurrentSwitch::new(
            "W1".to_string(),
            ref_node,
            aux,
            "V1".to_string(),
        ));
        circuit.tlines.push(crate::device::TransmissionLine::new(
            "T1".to_string(),
            ref_node,
            out,
            aux,
            ctrl,
            50.0,
            1e-9,
        ));

        circuit.remap_node_to_ground(ref_node);

        assert_eq!(circuit.get_node_by_name("ref"), Some(0));
        assert_eq!(circuit.get_node_by_name("ctrl"), Some(2));
        assert_eq!(circuit.get_node_by_name("out"), Some(3));
        assert_eq!(circuit.get_node_by_name("aux"), Some(4));

        assert_eq!(circuit.voltage_sources.node_neg[0], 0);
        assert_eq!(circuit.current_sources.node_pos[0], 0);
        assert_eq!(circuit.diodes.devices[0].node_anode, 0);
        assert_eq!(circuit.bjts.devices[0].node_emitter, 0);
        assert_eq!(circuit.mosfets.devices[0].node_source, 0);
        assert_eq!(circuit.jfets[0].source, 0);
        assert_eq!(circuit.vcvs.node_neg[0], 0);
        assert_eq!(circuit.vccs.node_pos[0], 0);
        assert_eq!(circuit.cccs.node_pos[0], 0);
        assert_eq!(circuit.ccvs.node_neg[0], 0);
        assert_eq!(circuit.vswitches[0].node_pos, 0);
        assert_eq!(circuit.iswitches[0].node_pos, 0);
        assert_eq!(circuit.tlines[0].node1_pos, 0);
        assert_eq!(circuit.num_nodes(), 4);
    }

    #[test]
    fn test_voltage_sources_max_expected_delta_with_sin() {
        let mut vs = VoltageSources::new();
        vs.add_with_ac_and_spec(
            "VIN".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::Sin {
                offset: 0.0,
                amplitude: 1.0,
                frequency: 1_000.0,
                delay: 0.0,
                damping: 0.0,
                phase: 0.0,
            }),
        );

        let delta = vs.max_expected_delta(0.0, 1e-6);
        assert!(delta > 1e-6, "expected non-zero transient source delta");
    }

    #[test]
    fn test_voltage_sources_max_expected_delta_ignores_dc() {
        let mut vs = VoltageSources::new();
        vs.add_with_ac_and_spec(
            "VDC".to_string(),
            1,
            0,
            1,
            5.0,
            0.0,
            0.0,
            Some(SourceSpec::Dc(5.0)),
        );

        let delta = vs.max_expected_delta(0.0, 1e-3);
        assert_eq!(delta, 0.0);
    }

    #[test]
    fn test_voltage_sources_pulse_defaults_follow_transient_context() {
        let mut vs = VoltageSources::new();
        vs.add_with_ac_and_spec(
            "VP".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::Pulse {
                v1: 0.0,
                v2: 1.0,
                delay: 0.0,
                rise: Value::NAN,
                fall: Value::NAN,
                width: Value::NAN,
                period: Value::NAN,
            }),
        );
        vs.set_transient_context(1e-6, 10e-6);

        let mut rhs = vec![0.0; 2];
        vs.update_transient_rhs(&mut rhs, 0.5e-6, |br_ordinal| 1 + br_ordinal);
        assert!(
            (rhs[1] - 0.5).abs() < 1e-6,
            "pulse rise default should follow transient step, got {}",
            rhs[1]
        );

        vs.update_transient_rhs(&mut rhs, 2.0e-6, |br_ordinal| 1 + br_ordinal);
        assert!(
            (rhs[1] - 1.0).abs() < 1e-9,
            "pulse width default should keep source high within tstop window, got {}",
            rhs[1]
        );
    }

    #[test]
    fn test_current_sources_update_transient_rhs_applies_waveform_delta() {
        let mut cs = CurrentSources::new();
        cs.add_with_ac_and_spec(
            "IP".to_string(),
            1,
            0,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::Pulse {
                v1: 0.0,
                v2: 1.0,
                delay: 0.0,
                rise: 1e-9,
                fall: 1e-9,
                width: 5e-9,
                period: 10e-9,
            }),
        );

        let mut rhs = vec![0.0; 2];
        cs.stamp_all(&mut rhs);
        cs.update_transient_rhs(&mut rhs, 2.0e-9);
        assert!(
            (rhs[0] + 1.0).abs() < 1e-9,
            "current pulse should inject 1A from node to ground, rhs={}",
            rhs[0]
        );
    }

    #[test]
    fn test_voltage_sources_evaluate_pwl_file_with_scaling() {
        let file_path = temp_file_path("pwl_scaled");
        fs::write(&file_path, "0,0\n1,1\n2,0\n").expect("should write PWL CSV");

        let mut vs = VoltageSources::new();
        vs.add_with_ac_and_spec(
            "VPWL".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::PwlFile {
                path: file_path.to_string_lossy().to_string(),
                time_scale: 1e-3,
                value_scale: 2.0,
                time_offset: 1e-3,
                value_offset: 0.5,
            }),
        );

        let mut rhs = vec![0.0; 2];
        vs.update_transient_rhs(&mut rhs, 2e-3, |br_ordinal| 1 + br_ordinal);

        // (t - toffset) / tscale = (2e-3 - 1e-3)/1e-3 = 1.0 -> base value 1.0
        // scaled and offset => 1.0 * 2.0 + 0.5 = 2.5
        assert!((rhs[1] - 2.5).abs() < 1e-12, "expected scaled PWL value");

        fs::remove_file(file_path).expect("should remove temp PWL CSV");
    }

    #[test]
    fn test_voltage_sources_pwl_file_missing_falls_back_to_value_offset() {
        let missing_path = temp_file_path("pwl_missing");
        let mut vs = VoltageSources::new();
        vs.add_with_ac_and_spec(
            "VPWL".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::PwlFile {
                path: missing_path.to_string_lossy().to_string(),
                time_scale: 1.0,
                value_scale: 1.0,
                time_offset: 0.0,
                value_offset: -0.75,
            }),
        );

        let mut rhs = vec![0.0; 2];
        vs.update_transient_rhs(&mut rhs, 10e-6, |br_ordinal| 1 + br_ordinal);
        assert!(
            (rhs[1] + 0.75).abs() < 1e-12,
            "missing PWL file should fall back to value_offset"
        );
    }
}
