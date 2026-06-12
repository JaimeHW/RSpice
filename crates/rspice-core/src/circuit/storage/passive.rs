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

/// Capacitor storage (SoA)
#[derive(Debug, Default, Clone)]
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
