//! Struct-of-Arrays device storage for circuit elements.
//!
//! These containers own the per-device state used by DC, AC, and transient
//! stamping. Keeping them here leaves `circuit::mod` focused on topology,
//! branch allocation, and whole-circuit orchestration.

use super::{NodeId, TwoTerminalStamp, project_two_terminal_voltage};
use crate::Value;
use crate::analysis::{CompanionCoefficients, IntegrationMethod};
use crate::device::{Bjt, Diode, MatrixStamper, Mosfet, NonlinearConvergenceCriteria};
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock, RwLock};
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
    /// Voltage from 3 steps ago for ngspice-style charge truncation.
    pub v_prev_prev_prev: Vec<Value>,
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
        self.v_prev_prev_prev.push(0.0);
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
        self.v_prev_prev_prev.push(ic);
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
                self.v_prev_prev_prev[i] = v_dc;
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
            self.v_prev_prev_prev[i] = self.v_prev_prev[i];
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
        let period_was_omitted = period.is_nan();

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
        let per = if period_was_omitted {
            // Match ngspice's transient-context defaults for one-shot pulse
            // decks: omitted PER must not restart the waveform before the
            // default high interval has completed inside the active analysis.
            stop_default + tr + pw + tf
        } else if per.is_finite() && per > 0.0 {
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
            project_two_terminal_voltage(solution, np, nn, v_source);
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

    /// Maximum absolute change expected from time-varying current sources over [t0, t1].
    #[inline]
    pub fn max_expected_delta(&self, t0: Value, t1: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .filter_map(|spec| spec.as_ref())
            .map(|spec| {
                (VoltageSources::evaluate_source_at_time_with_context(spec, t1, context)
                    - VoltageSources::evaluate_source_at_time_with_context(spec, t0, context))
                .abs()
            })
            .fold(0.0, Value::max)
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
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
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
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
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
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
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
