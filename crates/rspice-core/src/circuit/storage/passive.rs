use super::*;

/// Resistor storage (SoA layout for cache efficiency)
#[derive(Debug, Default, Clone)]
pub struct Resistors {
    /// Device names
    pub names: Vec<String>,
    /// Pre-computed stamp locations
    pub stamps: Vec<TwoTerminalStamp>,
    /// Conductance values (1/R)
    pub conductances: Vec<Value>,
    /// Small-signal conductances used by AC/PZ/noise analyses.
    pub small_signal_conductances: Vec<Value>,
    /// Per-instance thermal-noise temperature offsets in kelvin (ngspice
    /// `dtemp` semantics: noise runs at the analysis temperature plus this).
    pub noise_temperature_offsets: Vec<Value>,
    /// Per-instance noise enable (ngspice `noisy`, default on): a quiet
    /// resistor produces neither thermal nor flicker noise.
    pub noisy: Vec<bool>,
    /// Per-instance flicker noise as `(coefficient, AF, EF)` for a density
    /// of `coefficient·|I|^AF / f^EF`, with the model KF, multiplicity
    /// folding, and effective noise area pre-folded into the coefficient
    /// (resnoise.c semantics).
    pub flicker: Vec<Option<(Value, Value, Value)>>,
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
        self.noise_temperature_offsets.push(0.0);
        self.noisy.push(true);
        self.flicker.push(None);
    }

    /// Set the noise enable of the most recently added resistor.
    pub fn set_last_noisy(&mut self, noisy: bool) {
        if let Some(slot) = self.noisy.last_mut() {
            *slot = noisy;
        }
    }

    /// Set the flicker-noise terms of the most recently added resistor.
    pub fn set_last_flicker_noise(&mut self, coefficient: Value, af: Value, ef: Value) {
        if let Some(slot) = self.flicker.last_mut() {
            *slot = Some((coefficient, af, ef));
        }
    }

    /// Set the thermal-noise temperature offset of the most recently added
    /// resistor (the builder applies instance TEMP/DTEMP right after `add`).
    pub fn set_last_noise_temperature_offset(&mut self, offset_kelvin: Value) {
        if let Some(slot) = self.noise_temperature_offsets.last_mut() {
            *slot = offset_kelvin;
        }
    }

    /// Thermal-noise temperature offset (kelvin) for a resistor index.
    #[inline]
    pub fn noise_temperature_offset(&self, index: usize) -> Value {
        self.noise_temperature_offsets
            .get(index)
            .copied()
            .unwrap_or(0.0)
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

/// Resistors stored in MNA branch form:
/// `V(node_pos)-V(node_neg)-R*I(branch)=0`.
///
/// This form is used for zero and Xyce-near-zero resistances where nodal
/// conductance stamping would either be singular (`R=0`) or numerically
/// explosive while the branch current is still an observable.
#[derive(Debug, Default, Clone)]
pub struct ResistorBranches {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub resistances: Vec<Value>,
    pub small_signal_resistances: Vec<Value>,
    /// Pre-baked CSC indices: [br->np, np->br, br->nn, nn->br, br->br].
    csc_indices: Vec<[Option<CscIndex>; 5]>,
}

impl ResistorBranches {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        resistance: Value,
        small_signal_resistance: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.resistances.push(resistance);
        self.small_signal_resistances.push(small_signal_resistance);
        self.csc_indices.push([None; 5]);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

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
            self.csc_indices[i][4] = matrix.get_index(br - 1, br - 1);
        }
    }

    #[inline]
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
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
            if let Some(idx) = self.csc_indices[i][4] {
                matrix.stamp_direct(idx, -self.resistances[i]);
            }
            rhs[br - 1] = 0.0;
        }
    }

    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value], num_nodes: usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = num_nodes + self.branch_indices[i];

            if np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            matrix.push(br - 1, br - 1, -self.resistances[i]);
            rhs[br - 1] = 0.0;
        }
    }

    pub fn enforce_voltage_constraints(&self, solution: &mut [Value], num_nodes: usize) -> bool {
        let mut changed = false;
        for i in 0..self.names.len() {
            let branch_idx = num_nodes + self.branch_indices[i] - 1;
            let target_voltage = solution
                .get(branch_idx)
                .copied()
                .filter(|current| current.is_finite())
                .map(|current| self.resistances[i] * current)
                .filter(|voltage| voltage.is_finite())
                .unwrap_or(0.0);
            changed |= project_two_terminal_voltage(
                solution,
                self.node_pos[i],
                self.node_neg[i],
                target_voltage,
            );
        }
        changed
    }
}

/// Capacitor storage (SoA)
#[derive(Debug, Default, Clone)]
pub struct Capacitors {
    pub names: Vec<String>,
    /// Construction provenance aligned with `names` and the other SoA fields.
    ///
    /// `true` identifies a simulator-generated integration companion rather
    /// than a capacitor authored in the input netlist. Internal capacitors
    /// still participate in matrix stamping and state history, but callers
    /// can exclude them from authored-device introspection and public output.
    pub internal: Vec<bool>,
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
    /// Xyce-only MNA branch ordinal allocated for an `IC=` voltage
    /// constraint. Other dialects retain the IC for UIC history seeding but
    /// do not allocate or stamp this operating-point branch.
    pub ic_branch_indices: Vec<Option<NodeId>>,
    /// Pre-linked entries for the IC branch stamp:
    /// `[branch,pos; pos,branch; branch,neg; neg,branch; branch,branch]`.
    ic_branch_csc_indices: Vec<[Option<CscIndex>; 5]>,
}

impl Capacitors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, capacitance: Value) {
        self.names.push(name);
        self.internal.push(false);
        self.stamps.push(TwoTerminalStamp::new(node_pos, node_neg));
        self.capacitances.push(capacitance);
        self.v_prev.push(0.0);
        self.v_prev_prev.push(0.0);
        self.v_prev_prev_prev.push(0.0);
        self.i_prev.push(0.0); // Initial capacitor current is zero
        self.i_eq.push(0.0);
        self.ic.push(None);
        self.ic_branch_indices.push(None);
        self.ic_branch_csc_indices.push([None; 5]);
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
        self.internal.push(false);
        self.stamps.push(TwoTerminalStamp::new(node_pos, node_neg));
        self.capacitances.push(capacitance);
        self.v_prev.push(ic); // Initialize v_prev to IC
        self.v_prev_prev.push(ic); // Initialize v_prev_prev to IC as well
        self.v_prev_prev_prev.push(ic);
        self.i_prev.push(0.0); // Initial capacitor current is zero (DC steady state)
        self.i_eq.push(0.0);
        self.ic.push(Some(ic));
        self.ic_branch_indices.push(None);
        self.ic_branch_csc_indices.push([None; 5]);
    }

    /// Add a simulator-generated capacitor that owns private integration
    /// state. It remains in the canonical capacitor pipeline while carrying
    /// explicit provenance for public introspection and output filtering.
    pub fn add_internal(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        capacitance: Value,
    ) {
        self.add(name, node_pos, node_neg, capacitance);
        *self
            .internal
            .last_mut()
            .expect("capacitor provenance follows capacitor storage") = true;
    }

    /// Add a capacitor whose `IC=` is enforced as an ideal voltage source
    /// during Xyce operating-point solves.
    pub fn add_with_ic_branch(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        capacitance: Value,
        ic: Value,
        branch_ordinal: NodeId,
    ) {
        self.add_with_ic(name, node_pos, node_neg, capacitance, ic);
        *self
            .ic_branch_indices
            .last_mut()
            .expect("capacitor IC branch storage follows capacitor storage") = Some(branch_ordinal);
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

    /// Number of capacitors authored by the input netlist.
    pub fn authored_len(&self) -> usize {
        self.names
            .len()
            .saturating_sub(self.internal.iter().filter(|&&internal| internal).count())
    }

    /// Whether the capacitor at `index` is a simulator-generated companion.
    pub fn is_internal(&self, index: usize) -> bool {
        self.internal.get(index).copied().unwrap_or(false)
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Link all stamps to a StaticMatrix for O(1) access
    pub fn link_indices(
        &mut self,
        matrix: &StaticMatrix,
        get_branch_idx: impl Fn(NodeId) -> usize,
    ) {
        for (index, stamp) in self.stamps.iter_mut().enumerate() {
            stamp.link(matrix);

            let Some(branch_ordinal) = self.ic_branch_indices[index] else {
                self.ic_branch_csc_indices[index] = [None; 5];
                continue;
            };
            let branch = get_branch_idx(branch_ordinal);
            let pos = stamp.pp.row;
            let neg = stamp.nn.row;
            self.ic_branch_csc_indices[index] = [
                (pos > 0)
                    .then(|| matrix.get_index(branch - 1, pos - 1))
                    .flatten(),
                (pos > 0)
                    .then(|| matrix.get_index(pos - 1, branch - 1))
                    .flatten(),
                (neg > 0)
                    .then(|| matrix.get_index(branch - 1, neg - 1))
                    .flatten(),
                (neg > 0)
                    .then(|| matrix.get_index(neg - 1, branch - 1))
                    .flatten(),
                matrix.get_index(branch - 1, branch - 1),
            ];
        }
    }

    /// Stamp Xyce capacitor `IC=` constraints for an operating-point solve.
    /// The branch current participates in terminal KCL and the branch row
    /// enforces `V(pos) - V(neg) = IC`.
    #[inline]
    pub fn stamp_ic_operating_point_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
    ) {
        for index in 0..self.len() {
            let Some(branch_ordinal) = self.ic_branch_indices[index] else {
                continue;
            };
            let branch = num_nodes + branch_ordinal;
            let entries = self.ic_branch_csc_indices[index];
            if let Some(entry) = entries[0] {
                matrix.stamp_direct(entry, 1.0);
            }
            if let Some(entry) = entries[1] {
                matrix.stamp_direct(entry, 1.0);
            }
            if let Some(entry) = entries[2] {
                matrix.stamp_direct(entry, -1.0);
            }
            if let Some(entry) = entries[3] {
                matrix.stamp_direct(entry, -1.0);
            }
            rhs[branch - 1] = self.ic[index].unwrap_or(0.0);
        }
    }

    /// Triplet-matrix form of the Xyce operating-point IC constraint.
    #[inline]
    pub fn stamp_ic_operating_point(
        &self,
        matrix: &mut TripletMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
    ) {
        for index in 0..self.len() {
            let Some(branch_ordinal) = self.ic_branch_indices[index] else {
                continue;
            };
            let branch = num_nodes + branch_ordinal;
            let pos = self.stamps[index].pp.row;
            let neg = self.stamps[index].nn.row;
            if pos > 0 {
                matrix.push(branch - 1, pos - 1, 1.0);
                matrix.push(pos - 1, branch - 1, 1.0);
            }
            if neg > 0 {
                matrix.push(branch - 1, neg - 1, -1.0);
                matrix.push(neg - 1, branch - 1, -1.0);
            }
            rhs[branch - 1] = self.ic[index].unwrap_or(0.0);
        }
    }

    /// Project the physical small-signal lead current of every IC capacitor
    /// into its public branch-current slot. The OP-only branch equation is
    /// intentionally reused as the stable observable identity across AC,
    /// noise, and distortion results.
    pub fn project_complex_ic_branch_currents(
        &self,
        solution: &[Complex64],
        currents: &mut [Complex64],
        omega: Value,
    ) {
        for (index, branch_ordinal) in self.ic_branch_indices.iter().copied().enumerate() {
            let Some(branch_ordinal) = branch_ordinal else {
                continue;
            };
            let stamp = self.stamps[index];
            let v_pos = stamp
                .pp
                .row
                .checked_sub(1)
                .and_then(|slot| solution.get(slot))
                .copied()
                .unwrap_or_default();
            let v_neg = stamp
                .nn
                .row
                .checked_sub(1)
                .and_then(|slot| solution.get(slot))
                .copied()
                .unwrap_or_default();
            if let Some(current) = branch_ordinal
                .checked_sub(1)
                .and_then(|slot| currents.get_mut(slot))
            {
                *current = Complex64::new(0.0, omega) * self.capacitances[index] * (v_pos - v_neg);
            }
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
        num_nodes: usize,
    ) {
        for (i, stamp) in self.stamps.iter().enumerate() {
            // geq = coeff_g * C / dt
            let geq = coeff.capacitor_geq(self.capacitances[i], dt);

            if let Some(branch_ordinal) = self.ic_branch_indices[i] {
                // The Xyce IC branch is the capacitor's physical lead current,
                // so make it the terminal-KCL current instead of recovering an
                // observer from `geq*V - i_eq`.  The latter subtracts two very
                // large, nearly equal values at small timesteps and can lose
                // every useful digit of a modest lead current.
                //
                // For finite nonzero geq, scale the companion equation
                //
                //     I = geq*V - i_eq
                //
                // into `V - I/geq = i_eq/geq`.  This is algebraically
                // identical to the Norton companion but keeps both its matrix
                // coefficients and RHS well conditioned.  If reciprocal
                // scaling would overflow, retain the unscaled equation while
                // still using the branch current directly in KCL.
                let entries = self.ic_branch_csc_indices[i];
                if let Some(entry) = entries[1] {
                    matrix.stamp_direct(entry, 1.0);
                }
                if let Some(entry) = entries[3] {
                    matrix.stamp_direct(entry, -1.0);
                }

                // Scale only when the Norton conductance exceeds unity. For a
                // small conductance, `I - geq*V = -i_eq` is already the
                // better-conditioned row and avoids manufacturing a huge
                // `1/geq` coefficient.
                let use_scaled_row = geq.is_finite() && geq.abs() > 1.0 && coeff.coeff_g != 0.0;
                let scaled_rhs = if use_scaled_row {
                    // Form i_eq/geq directly. Cancelling C/dt analytically
                    // prevents an overflowing intermediate i_eq even when the
                    // normalized history is perfectly representable.
                    let mut history = (coeff.coeff_v_n / coeff.coeff_g) * self.v_prev[i];
                    if coeff.needs_two_history {
                        history += (coeff.coeff_v_n_minus_1 / coeff.coeff_g) * self.v_prev_prev[i];
                    }
                    if coeff.needs_current_history {
                        history += self.i_prev[i] / geq;
                    }
                    history
                } else {
                    Value::NAN
                };
                if use_scaled_row && scaled_rhs.is_finite() {
                    let reciprocal_geq = 1.0 / geq;
                    if let Some(entry) = entries[0] {
                        matrix.stamp_direct(entry, 1.0);
                    }
                    if let Some(entry) = entries[2] {
                        matrix.stamp_direct(entry, -1.0);
                    }
                    if let Some(entry) = entries[4] {
                        matrix.stamp_direct(entry, -reciprocal_geq);
                    }
                    rhs[num_nodes + branch_ordinal - 1] = scaled_rhs;
                } else {
                    let i_eq = coeff.capacitor_ieq(
                        self.capacitances[i],
                        dt,
                        self.v_prev[i],
                        self.v_prev_prev[i],
                        self.i_prev[i],
                    );
                    // This also gives a zero-capacitance device the exact
                    // `I = -i_eq` identity without dividing by zero. Invalid
                    // non-finite device data remains non-finite here and is
                    // rejected by the normal matrix-solve validation.
                    if let Some(entry) = entries[0] {
                        matrix.stamp_direct(entry, -geq);
                    }
                    if let Some(entry) = entries[2] {
                        matrix.stamp_direct(entry, geq);
                    }
                    if let Some(entry) = entries[4] {
                        matrix.stamp_direct(entry, 1.0);
                    }
                    rhs[num_nodes + branch_ordinal - 1] = -i_eq;
                }
            } else {
                // Compute the Norton history source only when this capacitor
                // actually uses the Norton terminal stamp.
                let i_eq = coeff.capacitor_ieq(
                    self.capacitances[i],
                    dt,
                    self.v_prev[i],
                    self.v_prev_prev[i],
                    self.i_prev[i],
                );
                stamp.stamp_direct(matrix, geq);
                if stamp.pp.row != 0 {
                    rhs[stamp.pp.row - 1] += i_eq;
                }
                if stamp.nn.row != 0 {
                    rhs[stamp.nn.row - 1] -= i_eq;
                }
            }
        }
    }

    /// Update capacitor state after a successful timestep using the exact
    /// coefficients that stamped that timestep.
    ///
    /// Adaptive integrators should prefer this entry point so the committed
    /// current history cannot be reconstructed with a different timestep
    /// ratio or integration order than the accepted linear system.
    /// Whole-circuit callers with explicit IC-current branches must instead
    /// commit those solved branch unknowns; this container-only helper has no
    /// node-count offset with which to locate them.
    pub fn update_state_with_coefficients(
        &mut self,
        solution: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
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

    /// Update capacitor state with explicit accepted-timestep history.
    ///
    /// `previous_accepted_dt` is the interval from the two solution points
    /// preceding this accepted step. Variable-step Gear2 uses it to construct
    /// its nonuniform BDF2 stencil. `None` deliberately restarts Gear2 at
    /// backward Euler order instead of inventing an equal previous step.
    pub fn update_state_with_previous_step(
        &mut self,
        solution: &[Value],
        dt: Value,
        method: IntegrationMethod,
        previous_accepted_dt: Option<Value>,
    ) {
        let coeff = match previous_accepted_dt {
            Some(previous_dt) => {
                CompanionCoefficients::for_method_with_previous_step(method, dt, previous_dt)
            }
            None if method == IntegrationMethod::Gear2 => CompanionCoefficients::backward_euler(),
            None => CompanionCoefficients::for_method(method),
        };
        self.update_state_with_coefficients(solution, dt, &coeff);
    }

    /// Update capacitor state assuming the current and previous timesteps are
    /// equal.
    ///
    /// This convenience is appropriate for fixed-grid integration. Adaptive
    /// Gear2 callers must use [`Self::update_state_with_previous_step`] or,
    /// preferably, [`Self::update_state_with_coefficients`].
    pub fn update_state_equal_step(
        &mut self,
        solution: &[Value],
        dt: Value,
        method: IntegrationMethod,
    ) {
        let coeff = CompanionCoefficients::for_method(method);
        self.update_state_with_coefficients(solution, dt, &coeff);
    }

    /// Legacy equal-step state update.
    ///
    /// This retains source compatibility, but its Gear2 behavior is explicitly
    /// fixed-grid only. Adaptive callers must provide accepted-step history.
    #[deprecated(
        note = "Gear2 here assumes equal timesteps; use update_state_with_previous_step or update_state_with_coefficients"
    )]
    pub fn update_state(&mut self, solution: &[Value], dt: Value, method: IntegrationMethod) {
        self.update_state_equal_step(solution, dt, method);
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

#[cfg(test)]
mod capacitor_state_tests {
    use super::*;

    fn assert_close(actual: Value, expected: Value) {
        let tolerance = 32.0 * Value::EPSILON * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected:.17e}, got {actual:.17e}"
        );
    }

    fn capacitor_with_history(
        capacitance: Value,
        previous_voltage: Value,
        older_voltage: Value,
    ) -> Capacitors {
        let mut capacitors = Capacitors::new();
        capacitors.add("C1".to_string(), 1, 0, capacitance);
        capacitors.v_prev[0] = previous_voltage;
        capacitors.v_prev_prev[0] = older_voltage;
        capacitors
    }

    #[test]
    fn variable_step_gear2_state_update_is_exact_for_an_affine_voltage() {
        let dt = 2.0;
        let previous_dt = 1.0;
        let slope = 3.0;
        let previous_voltage = 7.0;
        let older_voltage = previous_voltage - slope * previous_dt;
        let current_voltage = previous_voltage + slope * dt;
        let capacitance = 0.25;
        let mut capacitors = capacitor_with_history(capacitance, previous_voltage, older_voltage);

        capacitors.update_state_with_previous_step(
            &[current_voltage],
            dt,
            IntegrationMethod::Gear2,
            Some(previous_dt),
        );

        assert_close(capacitors.i_prev[0], capacitance * slope);
        assert_eq!(capacitors.v_prev[0], current_voltage);
        assert_eq!(capacitors.v_prev_prev[0], previous_voltage);
        assert_eq!(capacitors.v_prev_prev_prev[0], older_voltage);
    }

    #[test]
    fn gear2_without_previous_timestep_history_commits_with_backward_euler() {
        let dt = 2.0;
        let previous_voltage = 4.0;
        let current_voltage = 10.0;
        let capacitance = 0.5;
        // An intentionally unrelated older value proves that startup does not
        // manufacture an equal-step BDF2 stencil from invalid history.
        let mut capacitors = capacitor_with_history(capacitance, previous_voltage, -100.0);

        capacitors.update_state_with_previous_step(
            &[current_voltage],
            dt,
            IntegrationMethod::Gear2,
            None,
        );

        assert_close(
            capacitors.i_prev[0],
            capacitance * (current_voltage - previous_voltage) / dt,
        );
    }

    #[test]
    fn equal_step_gear2_convenience_has_explicit_fixed_grid_semantics() {
        let dt = 2.0;
        let slope = 3.0;
        let previous_voltage = 7.0;
        let older_voltage = previous_voltage - slope * dt;
        let current_voltage = previous_voltage + slope * dt;
        let capacitance = 0.25;
        let mut capacitors = capacitor_with_history(capacitance, previous_voltage, older_voltage);

        capacitors.update_state_equal_step(&[current_voltage], dt, IntegrationMethod::Gear2);

        assert_close(capacitors.i_prev[0], capacitance * slope);
    }

    #[test]
    fn ic_branch_companion_uses_physical_current_in_terminal_kcl() {
        let mut capacitors = Capacitors::new();
        capacitors.add_with_ic_branch("C1".to_string(), 1, 0, 1.0, 1.0, 1);

        // One node plus one capacitor-current branch, with every location
        // reserved just as Engine::build_matrix does for an IC capacitor.
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .expect("test matrix builds");
        capacitors.link_indices(&matrix, |branch_ordinal| 1 + branch_ordinal);

        // A one-ohm shunt makes KCL V + I(C1) = 0.  The tiny timestep
        // deliberately makes geq and i_eq about 1e10, the regime where an
        // observer equation would lose current precision by cancellation.
        matrix.add(0, 0, 1.0);
        let mut rhs = vec![0.0; 2];
        capacitors.stamp_transient_companion(
            &mut matrix,
            &mut rhs,
            1.0e-10,
            &CompanionCoefficients::backward_euler(),
            1,
        );
        let solution = matrix.solve(&rhs).expect("companion system solves");

        assert_close(solution[0] + solution[1], 0.0);
        assert!((solution[0] - 1.0).abs() < 2.0e-10);
        assert!((solution[1] + 1.0).abs() < 2.0e-10);
    }
}
