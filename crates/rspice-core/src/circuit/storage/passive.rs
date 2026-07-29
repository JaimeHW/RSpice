//! Struct-of-Arrays storage for resistors and capacitors.
//!
//! [`Resistors`] and [`ResistorBranches`] cover the two ways a resistor can
//! enter the MNA system — folded into the conductance matrix, or given its
//! own branch unknown when a current probe or a zero-valued device requires
//! one. [`ThermalResistorState`] carries the temperature-dependent value.
//!
//! [`Capacitors`] holds companion-model state for transient integration,
//! including the [`SolutionDependentCapacitorState`] linearization used by
//! charge models whose capacitance depends on the solution.

use super::*;

#[inline]
fn solution_partial(partials: &[(usize, Value)], column: usize) -> Value {
    partials
        .iter()
        .filter_map(|(index, value)| (*index == column).then_some(*value))
        .sum()
}

/// Runtime state for an Xyce LEVEL=2 self-consistent thermal resistor.
///
/// Xyce keeps this electrothermal state outside the electrical MNA unknowns:
/// accepted electrical dissipation advances the temperature explicitly, and
/// the next electrical load evaluates dependent material expressions at that
/// temperature.
#[derive(Debug, Clone)]
pub struct ThermalResistorState {
    pub length: Value,
    pub area: Value,
    pub thermal_length: Value,
    pub thermal_area: Value,
    pub multiplicity: Value,
    pub scale: Value,
    pub temperature_celsius: Value,
    pub resistivity: Value,
    pub heat_capacity: Value,
    pub thermal_heat_capacity: Value,
    /// Nominal, unmultiplied resistance exposed by the `R` probe.
    pub reported_resistance: Value,
    /// Resistance/current values from the just-completed electrical load.
    /// Xyce advances temperature after that load, but its output lead current
    /// and `R` parameter still describe the load that produced the sample.
    pub output_resistance: Value,
    pub output_conductance: Value,
    /// Retained expression scope for temperature-dependent material values.
    pub base_context: crate::netlist::ParamContext,
    pub tnom_celsius: Value,
    pub model_params: Vec<(String, Value)>,
    pub model_expr_params: Vec<(String, String)>,
    pub instance_resistivity: Option<Value>,
    pub instance_heat_capacity: Option<Value>,
    pub instance_thermal_heat_capacity: Option<Value>,
}

impl ThermalResistorState {
    fn material_context(
        &self,
        temperature_celsius: Value,
    ) -> Result<crate::netlist::ParamContext, String> {
        let mut context = self.base_context.clone();
        context.set("TEMP", temperature_celsius);
        context.set("TEMPER", temperature_celsius);
        context.set("TNOM", self.tnom_celsius);
        context.set(
            "VT",
            crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(
                temperature_celsius,
            )),
        );
        crate::netlist::expr::materialize_available_parameter_expressions(&mut context);
        for (name, value) in &self.model_params {
            context.set(name, *value);
        }

        let mut pending = self.model_expr_params.clone();
        while !pending.is_empty() {
            let mut progress = false;
            let mut unresolved = Vec::new();
            for (name, expression) in pending {
                match crate::netlist::expr::eval_expression(&expression, &context) {
                    Ok(value) => {
                        context.set(&name, value);
                        progress = true;
                    }
                    Err(_) => unresolved.push((name, expression)),
                }
            }
            if !progress {
                let names = unresolved
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(format!(
                    "thermal resistor material expressions could not be resolved: {names}"
                ));
            }
            pending = unresolved;
        }
        Ok(context)
    }

    fn model_value(&self, context: &crate::netlist::ParamContext, names: &[&str]) -> Option<Value> {
        names.iter().find_map(|candidate| {
            self.model_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
                .map(|(_, value)| *value)
                .or_else(|| {
                    self.model_expr_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
                        .and_then(|(_, expression)| {
                            crate::netlist::expr::eval_expression(expression, context).ok()
                        })
                })
        })
    }

    /// Re-evaluate dependent material values and the nominal resistance at a
    /// new Celsius temperature.
    pub fn update_material_at_temperature(
        &mut self,
        temperature_celsius: Value,
    ) -> Result<(), String> {
        if !temperature_celsius.is_finite() {
            return Err(format!(
                "thermal resistor temperature became non-finite: {temperature_celsius}"
            ));
        }
        let context = self.material_context(temperature_celsius)?;
        let resistivity = self
            .instance_resistivity
            .or_else(|| self.model_value(&context, &["RESISTIVITY"]))
            .ok_or_else(|| "thermal resistor requires RESISTIVITY".to_string())?;
        let heat_capacity = self
            .instance_heat_capacity
            .or_else(|| self.model_value(&context, &["HEATCAPACITY"]))
            .ok_or_else(|| "thermal resistor requires HEATCAPACITY".to_string())?;
        let thermal_heat_capacity = self
            .instance_thermal_heat_capacity
            .or_else(|| self.model_value(&context, &["THERMAL_HEATCAPACITY"]))
            .unwrap_or(heat_capacity);
        if !resistivity.is_finite()
            || resistivity <= 0.0
            || !heat_capacity.is_finite()
            || heat_capacity <= 0.0
            || !thermal_heat_capacity.is_finite()
            || thermal_heat_capacity <= 0.0
        {
            return Err(format!(
                "thermal resistor material values must be finite and positive (RESISTIVITY={resistivity}, HEATCAPACITY={heat_capacity}, THERMAL_HEATCAPACITY={thermal_heat_capacity})"
            ));
        }
        let resistance = resistivity * self.length / self.area;
        if !resistance.is_finite() || resistance <= 0.0 {
            return Err(format!(
                "thermal resistor material resistance is invalid: {resistance}"
            ));
        }
        self.temperature_celsius = temperature_celsius;
        self.resistivity = resistivity;
        self.heat_capacity = heat_capacity;
        self.thermal_heat_capacity = thermal_heat_capacity;
        self.reported_resistance = resistance;
        Ok(())
    }

    #[inline]
    pub fn electrical_resistance(&self) -> Value {
        self.reported_resistance * self.scale / self.multiplicity
    }

    /// Advance the explicit Xyce thermal state after an accepted electrical
    /// solution; the resulting material is used by the next electrical load.
    pub fn advance_after_accepted_step(
        &mut self,
        voltage: Value,
        conductance: Value,
        step_size: Value,
    ) -> Result<(), String> {
        if !step_size.is_finite() || step_size < 0.0 {
            return Err(format!(
                "thermal resistor accepted step size is invalid: {step_size}"
            ));
        }
        let current = voltage * conductance;
        self.output_resistance = self.reported_resistance;
        self.output_conductance = conductance;
        let dissipation = current * current * self.reported_resistance;
        let denominator = self.area * self.length * self.heat_capacity
            + self.thermal_area * self.thermal_length * self.thermal_heat_capacity;
        if !denominator.is_finite() || denominator <= 0.0 {
            return Err(format!(
                "thermal resistor heat-capacity denominator is invalid: {denominator}"
            ));
        }
        let temperature = self.temperature_celsius + dissipation * step_size / denominator;
        self.update_material_at_temperature(temperature)
    }
}

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
    /// Optional Xyce LEVEL=2 thermal state aligned with the resistor arrays.
    pub thermal: Vec<Option<ThermalResistorState>>,
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
        self.thermal.push(None);
    }

    /// Attach thermal state to the most recently added resistor.
    pub fn set_last_thermal(&mut self, state: ThermalResistorState) {
        let mut state = state;
        state.output_resistance = state.reported_resistance;
        state.output_conductance = 1.0 / state.electrical_resistance();
        if let Some(slot) = self.thermal.last_mut() {
            *slot = Some(state);
        }
    }

    #[inline]
    pub fn output_conductance(&self, index: usize) -> Value {
        self.thermal
            .get(index)
            .and_then(Option::as_ref)
            .map(|state| state.output_conductance)
            .unwrap_or_else(|| self.conductances.get(index).copied().unwrap_or(0.0))
    }

    /// Advance every self-consistent thermal resistor after an accepted
    /// transient solution and refresh the next-step conductances.
    pub fn advance_thermal_states(
        &mut self,
        solution: &[Value],
        step_size: Value,
    ) -> Result<(), String> {
        for index in 0..self.thermal.len() {
            let Some(state) = self.thermal[index].as_mut() else {
                continue;
            };
            let stamp = self.stamps[index];
            let node_voltage = |node: usize| {
                if node == 0 {
                    0.0
                } else {
                    solution.get(node - 1).copied().unwrap_or(0.0)
                }
            };
            let voltage = node_voltage(stamp.pp.row) - node_voltage(stamp.nn.row);
            let old_conductance = self.conductances[index];
            state.advance_after_accepted_step(voltage, old_conductance, step_size)?;
            let resistance = state.electrical_resistance();
            if !resistance.is_finite() || resistance <= 0.0 {
                return Err(format!(
                    "thermal resistor '{}' resolved to invalid electrical resistance {resistance}",
                    self.names[index]
                ));
            }
            self.conductances[index] = 1.0 / resistance;
        }
        Ok(())
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

/// Accepted charge/incremental-derivative history for one
/// solution-dependent capacitor.
#[derive(Debug, Clone)]
pub struct SolutionDependentCapacitorState {
    /// Effective capacitance at the previous accepted solution.
    pub c_prev: Value,
    /// Charge at the previous accepted solution.
    pub q_prev: Value,
    /// Charge at the accepted solution before `q_prev`.
    pub q_prev_prev: Value,
    /// `dC/dX` at the previous accepted solution.
    pub dcdx_prev: Vec<(usize, Value)>,
    /// `dQ/dX` at the previous accepted solution.
    pub dqdx_prev: Vec<(usize, Value)>,
}

impl Default for SolutionDependentCapacitorState {
    fn default() -> Self {
        Self {
            c_prev: Value::NAN,
            q_prev: Value::NAN,
            q_prev_prev: Value::NAN,
            dcdx_prev: Vec::new(),
            dqdx_prev: Vec::new(),
        }
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
    /// Optional compiled solution-dependent capacitance expressions aligned
    /// with `names` and the other capacitor storage fields.
    ///
    /// Static capacitors carry `None`; expression-valued capacitors retain
    /// their compiled evaluator here until a later circuit phase binds and
    /// evaluates it. Keeping this separate from the numeric capacitance
    /// preserves the existing companion-model storage layout.
    pub value_expressions: Vec<Option<SolutionDependentCapacitor>>,
    /// Accepted state aligned with `value_expressions`; static capacitors
    /// carry `None`.
    pub value_expression_states: Vec<Option<SolutionDependentCapacitorState>>,
    /// Last accepted effective capacitance for device operating-point output.
    pub effective_capacitances: Vec<Value>,
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
        self.value_expressions.push(None);
        self.value_expression_states.push(None);
        self.effective_capacitances.push(capacitance);
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
        self.value_expressions.push(None);
        self.value_expression_states.push(None);
        self.effective_capacitances.push(capacitance);
        self.v_prev.push(ic); // Initialize v_prev to IC
        self.v_prev_prev.push(ic); // Initialize v_prev_prev to IC as well
        self.v_prev_prev_prev.push(ic);
        self.i_prev.push(0.0); // Initial capacitor current is zero (DC steady state)
        self.i_eq.push(0.0);
        self.ic.push(Some(ic));
        self.ic_branch_indices.push(None);
        self.ic_branch_csc_indices.push([None; 5]);
    }

    /// Add a capacitor with a compiled solution-dependent capacitance
    /// expression. The numeric value is retained as the parse-time fallback
    /// until a circuit phase evaluates and validates the expression.
    pub fn add_with_value_expression(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        capacitance: Value,
        value_expression: SolutionDependentCapacitor,
    ) {
        self.add(name, node_pos, node_neg, capacitance);
        *self
            .value_expressions
            .last_mut()
            .expect("capacitor expression storage follows capacitor storage") =
            Some(value_expression);
        *self
            .value_expression_states
            .last_mut()
            .expect("capacitor expression state follows capacitor storage") =
            Some(SolutionDependentCapacitorState::default());
    }

    /// Bind all retained solution-dependent capacitance expressions to the
    /// circuit's node and branch maps.
    pub fn bind_value_expression_references<FN, FB>(
        &mut self,
        resolve_node: FN,
        resolve_branch: FB,
    ) -> Result<(), String>
    where
        FN: Fn(&str) -> Option<usize> + Copy,
        FB: Fn(&str) -> Option<usize> + Copy,
    {
        for expression in self.value_expressions.iter_mut().flatten() {
            expression.bind_references(resolve_node, resolve_branch)?;
        }
        Ok(())
    }

    /// Access a retained solution-dependent capacitance evaluator by storage
    /// index.
    pub fn value_expression(&self, index: usize) -> Option<&SolutionDependentCapacitor> {
        self.value_expressions.get(index).and_then(Option::as_ref)
    }

    /// Mutably access a retained solution-dependent capacitance evaluator by
    /// storage index for analysis-time evaluation and state updates.
    pub fn value_expression_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut SolutionDependentCapacitor> {
        self.value_expressions
            .get_mut(index)
            .and_then(Option::as_mut)
    }

    /// Evaluate a retained solution-dependent capacitance expression.
    pub fn evaluate_value_expression(
        &mut self,
        index: usize,
        solution: &[Value],
        time: Value,
    ) -> Option<Value> {
        self.value_expression_mut(index)
            .map(|expression| expression.evaluate(solution, time))
    }

    /// Evaluate the effective capacitance, applying the static instance/model
    /// scale stored in `capacitances` to a retained solution-dependent value.
    /// Static capacitors simply return their stored numeric capacitance.
    pub fn evaluate_effective_capacitance(
        &mut self,
        index: usize,
        solution: &[Value],
        time: Value,
    ) -> Option<Value> {
        let scale = *self.capacitances.get(index)?;
        Some(
            self.evaluate_value_expression(index, solution, time)
                .unwrap_or(scale)
                * if self.value_expression(index).is_some() {
                    scale
                } else {
                    1.0
                },
        )
    }

    /// Linearize an effective solution-dependent capacitance, including the
    /// static instance/model scale. Returns `None` for a static capacitor.
    pub fn linearize_effective_capacitance(
        &mut self,
        index: usize,
        solution: &[Value],
        time: Value,
    ) -> Option<SolutionDependentCapacitorLinearization> {
        let scale = *self.capacitances.get(index)?;
        let expression = self.value_expression_mut(index)?;
        let mut linearization = expression.linearize(solution, time);
        linearization.value *= scale;
        for (_, partial) in &mut linearization.partials {
            *partial *= scale;
        }
        Some(linearization)
    }

    /// Initialize solution-dependent charge history from an accepted DC
    /// solution. Xyce's DC charge remains `C(V)*V`; transient steps then
    /// integrate the effective capacitance between accepted points.
    pub fn initialize_solution_dependent_from_dc(&mut self, solution: &[Value], time: Value) {
        for index in 0..self.stamps.len() {
            if self
                .value_expressions
                .get(index)
                .and_then(Option::as_ref)
                .is_none()
            {
                continue;
            }
            let Some(linearization) = self.linearize_effective_capacitance(index, solution, time)
            else {
                continue;
            };
            let c = linearization.value;
            if !c.is_finite() || c < 0.0 {
                continue;
            }
            let stamp = self.stamps[index];
            let v_dc = if stamp.pp.row > 0 {
                solution.get(stamp.pp.row - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            } - if stamp.nn.row > 0 {
                solution.get(stamp.nn.row - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            };
            let dqdx = linearization
                .partials
                .iter()
                .map(|(column, partial)| (*column, *partial * v_dc))
                .collect();
            if let Some(state) = self
                .value_expression_states
                .get_mut(index)
                .and_then(Option::as_mut)
            {
                state.c_prev = c;
                state.q_prev = c * v_dc;
                state.q_prev_prev = state.q_prev;
                state.dcdx_prev = linearization.partials;
                state.dqdx_prev = dqdx;
                self.effective_capacitances[index] = c;
            }
        }
    }

    /// Stamp the DAE companion for every solution-dependent capacitor.
    ///
    /// The accepted state stores charge and the integrated external
    /// derivatives. Trial Newton points use Xyce's trapezoidal charge update
    /// `q = q_old + 0.5*(C_old+C_new)*dV`, then apply the selected integration
    /// coefficients to `q` and `dQ/dX`. Terminal derivatives are always
    /// replaced by `+/-C`; the integrated `dC/dX*dV` term is only used for
    /// non-terminal dependencies.
    pub fn stamp_solution_dependent_transient_companion(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        coeff: &CompanionCoefficients,
    ) -> Result<(), String> {
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!(
                "solution-dependent capacitor companion requires a finite positive dt, got {dt}"
            ));
        }
        let charge_factor = coeff.coeff_g / dt;
        if !charge_factor.is_finite() {
            return Err(format!(
                "solution-dependent capacitor companion produced invalid charge coefficient {charge_factor}"
            ));
        }

        for index in 0..self.stamps.len() {
            if self
                .value_expressions
                .get(index)
                .and_then(Option::as_ref)
                .is_none()
            {
                continue;
            }
            let linearization = self
                .linearize_effective_capacitance(index, solution, time)
                .ok_or_else(|| {
                    format!(
                        "solution-dependent capacitor '{}' has no evaluator",
                        self.names[index]
                    )
                })?;
            let capacitance = linearization.value;
            if !capacitance.is_finite() || capacitance < 0.0 {
                return Err(format!(
                    "solution-dependent capacitor '{}' evaluated to invalid capacitance {capacitance}",
                    self.names[index]
                ));
            }

            let stamp = self.stamps[index];
            let v_new = if stamp.pp.row > 0 {
                solution.get(stamp.pp.row - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            } - if stamp.nn.row > 0 {
                solution.get(stamp.nn.row - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            };
            let state = self
                .value_expression_states
                .get_mut(index)
                .and_then(Option::as_mut)
                .ok_or_else(|| {
                    format!(
                        "solution-dependent capacitor '{}' has no history state",
                        self.names[index]
                    )
                })?;

            // Be robust for direct CircuitData users that stamp without the
            // transient startup helper: seed a physically consistent state at
            // the first trial point rather than treating zero charge as real.
            if !state.c_prev.is_finite()
                || !state.q_prev.is_finite()
                || !state.q_prev_prev.is_finite()
            {
                state.c_prev = capacitance;
                state.q_prev = capacitance * self.v_prev[index];
                state.q_prev_prev = state.q_prev;
                state.dcdx_prev = linearization.partials.clone();
                state.dqdx_prev = linearization
                    .partials
                    .iter()
                    .map(|(column, partial)| (*column, *partial * self.v_prev[index]))
                    .collect();
            }

            let delta_v = v_new - self.v_prev[index];
            let charge = state.q_prev + 0.5 * (state.c_prev + capacitance) * delta_v;
            let mut dqd_x = Vec::with_capacity(linearization.partials.len());
            for (column, dcdx) in &linearization.partials {
                let old_dcdx = solution_partial(&state.dcdx_prev, *column);
                let old_dqdx = solution_partial(&state.dqdx_prev, *column);
                dqd_x.push((*column, old_dqdx + 0.5 * (old_dcdx + *dcdx) * delta_v));
            }
            for (column, old_dqdx) in &state.dqdx_prev {
                if !linearization
                    .partials
                    .iter()
                    .any(|(current_column, _)| current_column == column)
                {
                    let old_dcdx = solution_partial(&state.dcdx_prev, *column);
                    dqd_x.push((*column, *old_dqdx + 0.5 * old_dcdx * delta_v));
                }
            }

            let mut current = charge_factor * charge - coeff.coeff_v_n / dt * state.q_prev;
            if coeff.needs_two_history {
                current -= coeff.coeff_v_n_minus_1 / dt * state.q_prev_prev;
            }
            if coeff.needs_current_history {
                current -= self.i_prev[index];
            }

            let pos_col = (stamp.pp.row > 0).then(|| stamp.pp.row - 1);
            let neg_col = (stamp.nn.row > 0).then(|| stamp.nn.row - 1);
            let mut derivative_terms = Vec::with_capacity(dqd_x.len() + 2);
            if let Some(column) = pos_col {
                derivative_terms.push((column, capacitance));
            }
            if let Some(column) = neg_col {
                derivative_terms.push((column, -capacitance));
            }
            for (column, derivative) in dqd_x {
                if Some(column) == pos_col || Some(column) == neg_col {
                    continue;
                }
                derivative_terms.push((column, derivative));
            }

            let mut affine = current;
            for (column, derivative) in derivative_terms {
                let d_current = charge_factor * derivative;
                if !d_current.is_finite() {
                    return Err(format!(
                        "solution-dependent capacitor '{}' produced invalid dI/dX {d_current}",
                        self.names[index]
                    ));
                }
                if stamp.pp.row > 0 {
                    matrix.add(stamp.pp.row - 1, column, d_current);
                }
                if stamp.nn.row > 0 {
                    matrix.add(stamp.nn.row - 1, column, -d_current);
                }
                affine -= d_current * solution.get(column).copied().unwrap_or(0.0);
            }
            if stamp.pp.row > 0 {
                rhs[stamp.pp.row - 1] -= affine;
            }
            if stamp.nn.row > 0 {
                rhs[stamp.nn.row - 1] += affine;
            }
            self.effective_capacitances[index] = capacitance;
        }
        Ok(())
    }

    /// Commit solution-dependent charge and derivative history after an
    /// accepted transient step. The ordinary capacitor voltage/current
    /// histories are rotated here as well, so callers can skip these devices
    /// in the static companion updater.
    pub fn update_solution_dependent_state_with_coefficients(
        &mut self,
        solution: &[Value],
        accepted_time: Value,
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let charge_factor = coeff.coeff_g / dt;
        for index in 0..self.stamps.len() {
            if self
                .value_expressions
                .get(index)
                .and_then(Option::as_ref)
                .is_none()
            {
                continue;
            }
            let Some(linearization) =
                self.linearize_effective_capacitance(index, solution, accepted_time)
            else {
                continue;
            };
            let capacitance = linearization.value;
            if !capacitance.is_finite() || capacitance < 0.0 {
                continue;
            }
            let stamp = self.stamps[index];
            let v_new = if stamp.pp.row > 0 {
                solution.get(stamp.pp.row - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            } - if stamp.nn.row > 0 {
                solution.get(stamp.nn.row - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            };
            let state = self
                .value_expression_states
                .get_mut(index)
                .and_then(Option::as_mut)
                .expect("solution-dependent capacitor history follows storage");
            if !state.c_prev.is_finite()
                || !state.q_prev.is_finite()
                || !state.q_prev_prev.is_finite()
            {
                state.c_prev = capacitance;
                state.q_prev = capacitance * self.v_prev[index];
                state.q_prev_prev = state.q_prev;
                state.dcdx_prev = linearization.partials.clone();
                state.dqdx_prev = linearization
                    .partials
                    .iter()
                    .map(|(column, partial)| (*column, *partial * self.v_prev[index]))
                    .collect();
            }
            let delta_v = v_new - self.v_prev[index];
            let charge = state.q_prev + 0.5 * (state.c_prev + capacitance) * delta_v;
            let mut dqd_x = Vec::with_capacity(linearization.partials.len());
            for (column, dcdx) in &linearization.partials {
                let old_dcdx = solution_partial(&state.dcdx_prev, *column);
                let old_dqdx = solution_partial(&state.dqdx_prev, *column);
                dqd_x.push((*column, old_dqdx + 0.5 * (old_dcdx + *dcdx) * delta_v));
            }
            let mut current = charge_factor * charge - coeff.coeff_v_n / dt * state.q_prev;
            if coeff.needs_two_history {
                current -= coeff.coeff_v_n_minus_1 / dt * state.q_prev_prev;
            }
            if coeff.needs_current_history {
                current -= self.i_prev[index];
            }

            state.q_prev_prev = state.q_prev;
            state.q_prev = charge;
            state.c_prev = capacitance;
            state.dcdx_prev = linearization.partials;
            state.dqdx_prev = dqd_x;
            self.v_prev_prev_prev[index] = self.v_prev_prev[index];
            self.v_prev_prev[index] = self.v_prev[index];
            self.v_prev[index] = v_new;
            self.i_prev[index] = current;
            self.effective_capacitances[index] = capacitance;
            if let Some(expression) = self.value_expression_mut(index) {
                expression.accept_transient_step(solution, accepted_time);
            }
        }
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

    /// Whether any retained capacitor value must be evaluated from the trial
    /// circuit solution rather than treated as a fixed numeric value.
    pub fn has_solution_dependent_values(&self) -> bool {
        self.value_expressions.iter().any(Option::is_some)
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
            if self
                .value_expressions
                .get(i)
                .and_then(Option::as_ref)
                .is_some()
            {
                continue;
            }
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
            if self
                .value_expressions
                .get(i)
                .and_then(Option::as_ref)
                .is_some()
            {
                continue;
            }
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

    #[test]
    fn solution_dependent_value_expression_stays_aligned_with_capacitor_state() {
        let mut capacitors = Capacitors::new();
        capacitors.add("Cstatic".to_string(), 1, 0, 1.0e-12);
        let expression = SolutionDependentCapacitor::new("Cdynamic".to_string(), "V(ctrl)")
            .expect("solution-dependent capacitor expression parses");
        capacitors.add_with_value_expression("Cdynamic".to_string(), 2, 0, 2.0, expression);

        assert!(capacitors.value_expression(0).is_none());
        assert_eq!(
            capacitors
                .value_expression(1)
                .map(|expression| expression.name.as_str()),
            Some("Cdynamic")
        );

        capacitors
            .bind_value_expression_references(
                |name| (name.eq_ignore_ascii_case("ctrl")).then_some(1),
                |_| None,
            )
            .expect("capacitor expression references bind");
        assert_eq!(
            capacitors.evaluate_value_expression(1, &[2.5], 0.0),
            Some(2.5)
        );
        assert_eq!(
            capacitors.evaluate_effective_capacitance(1, &[2.5], 0.0),
            Some(5.0)
        );
        let linearization = capacitors
            .linearize_effective_capacitance(1, &[2.5], 0.0)
            .expect("dynamic capacitor linearization exists");
        assert_eq!(linearization.value, 5.0);
        assert_eq!(linearization.partials, vec![(0, 2.0)]);
    }
}
