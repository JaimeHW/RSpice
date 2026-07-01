//! DC Analysis - operating point and sweep
//!
//! This module provides DC analysis functions:
//! - Operating point (DC OP) calculation
//! - DC sweep for I-V curve generation

#![allow(clippy::too_many_arguments)]

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::solver::{SimulationResult, StaticMatrix};
use crate::{CircuitData, Netlist, Value};

const DC_SWEEP_CONTINUATION_MAX_SUBDIVISIONS: usize = 128;

/// One accepted point from a DC sweep, including the solved node/branch result
/// and the per-device operating-point report cached at that bias.
#[derive(Debug, Clone)]
pub struct DcSweepPointResult {
    pub sweep_value: Value,
    pub result: SimulationResult,
    pub device_op_report: crate::circuit::DeviceOpReport,
}

enum DcSweepSource {
    Voltage {
        index: usize,
        original_value: Value,
        original_source_spec: Option<crate::netlist::SourceSpec>,
    },
    Current {
        index: usize,
        original_value: Value,
        original_source_spec: Option<crate::netlist::SourceSpec>,
    },
}

impl DcSweepSource {
    fn set_value(&self, circuit: &mut CircuitData, value: Value) {
        match self {
            Self::Voltage { index, .. } => {
                circuit.voltage_sources.dc_values[*index] = value;
            }
            Self::Current { index, .. } => {
                circuit.current_sources.dc_values[*index] = value;
            }
        }
    }

    fn restore(self, circuit: &mut CircuitData) {
        match self {
            Self::Voltage {
                index,
                original_value,
                original_source_spec,
            } => {
                circuit.voltage_sources.dc_values[index] = original_value;
                circuit.voltage_sources.source_specs[index] = original_source_spec;
            }
            Self::Current {
                index,
                original_value,
                original_source_spec,
            } => {
                circuit.current_sources.dc_values[index] = original_value;
                circuit.current_sources.source_specs[index] = original_source_spec;
            }
        }
    }
}

impl Engine {
    fn build_empty_dc_result() -> SimulationResult {
        let mut result = SimulationResult::new(0, 0);
        result.node_names = vec!["0".to_string()];
        result
    }

    fn solve_nonlinear_dc_sweep_target_with_substeps(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        sweep_source: &DcSweepSource,
        from_value: Value,
        to_value: Value,
        seed: &[Value],
        min_subdivisions: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, usize), SimulationError> {
        let span = to_value - from_value;
        if !span.is_finite() || span == 0.0 {
            sweep_source.set_value(circuit, to_value);
            return self
                .solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(seed), abort)
                .map(|solution| (solution, 1));
        }

        let start_state = circuit.nonlinear_state_snapshot();
        let mut subdivisions = min_subdivisions.max(2).next_power_of_two();
        let mut last_error = None;

        while subdivisions <= DC_SWEEP_CONTINUATION_MAX_SUBDIVISIONS {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            circuit.restore_nonlinear_state(start_state.clone());
            sweep_source.set_value(circuit, from_value);
            let mut solution = seed.to_vec();
            let mut accepted = true;

            for step_idx in 1..=subdivisions {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let alpha = step_idx as Value / subdivisions as Value;
                sweep_source.set_value(circuit, from_value + alpha * span);
                match self.solve_nonlinear_with_guess_and_abort(
                    circuit,
                    matrix,
                    Some(&solution),
                    abort,
                ) {
                    Ok(next_solution) => {
                        solution = next_solution;
                    }
                    Err(err) => {
                        last_error = Some(err);
                        accepted = false;
                        break;
                    }
                }
            }

            if accepted {
                return Ok((solution, subdivisions));
            }

            subdivisions *= 2;
        }

        circuit.restore_nonlinear_state(start_state);
        sweep_source.set_value(circuit, from_value);
        Err(last_error.unwrap_or(SimulationError::ConvergenceFailed(
            self.config.max_iterations,
        )))
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&self, netlist: &Netlist) -> Result<SimulationResult, SimulationError> {
        self.run_dc_op_with_report(netlist)
            .map(|(result, _)| result)
    }

    /// Run DC operating point analysis and return the per-device
    /// operating-point report alongside the node solution.
    ///
    /// The report carries each semiconductor device's bias point and
    /// small-signal parameters (id/gm/gds/region for MOSFETs, ic/beta/gm for
    /// BJTs, vd/id/gd for diodes) as cached by the converged Newton solve.
    pub fn run_dc_op_with_report(
        &self,
        netlist: &Netlist,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        let engine = self.resolved_for_netlist(netlist);

        // Build circuit from netlist
        let mut circuit = engine.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Ok((
                Self::build_empty_dc_result(),
                crate::circuit::DeviceOpReport::default(),
            ));
        }

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;

        // Link phase: bake CSC indices into device storage for O(1) stamping
        circuit.link_indices(&matrix);

        let mut matrix = matrix;

        let solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }

        // Build result
        let mut result = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());

        // Populate node names from circuit (results include actual net names)
        let sorted_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();
        result.node_names = std::iter::once("0".to_string()) // Ground is node 0
            .chain(sorted_names)
            .collect();
        result.branch_names = branch_names;

        for (i, &v) in solution.iter().enumerate() {
            if i < circuit.num_nodes() {
                result.node_voltages[i + 1] = v; // +1 because node 0 is ground
            } else {
                result.branch_currents[i - circuit.num_nodes()] = v;
            }
        }

        Ok((result, circuit.device_op_report()))
    }

    /// Run DC sweep analysis
    ///
    /// Sweeps one source through a range of values, solving DC at each point.
    /// Returns a vector of (sweep_value, solution) pairs.
    pub fn run_dc_sweep(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_dc_sweep_with_abort(netlist, source_name, start, stop, step, &NoAbort)
    }

    /// Two-source DC sweep: the first (inner) source sweeps fully at every
    /// value of the second (outer) source, ngspice-style; results are the
    /// inner sweeps concatenated in outer order, each point tagged with the
    /// inner sweep value. With no second sweep this is a plain DC sweep.
    ///
    /// The outer source must be a top-level independent source (or `TEMP`).
    pub fn run_dc_sweep2_with_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        sweep2: Option<&crate::netlist::DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_dc_sweep2_with_report_and_abort(
            netlist,
            source_name,
            start,
            stop,
            step,
            sweep2,
            abort,
        )
        .map(|points| {
            points
                .into_iter()
                .map(|point| (point.sweep_value, point.result))
                .collect()
        })
    }

    /// Two-source DC sweep that preserves per-point device operating reports.
    pub fn run_dc_sweep2_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        sweep2: Option<&crate::netlist::DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let primary = crate::netlist::DcSweepSpec::linear(start, stop, step);
        self.run_dc_sweep2_spec_with_report_and_abort(netlist, source_name, &primary, sweep2, abort)
    }

    /// Two-source DC sweep using the full `.DC` sweep specification for
    /// non-linear sweep modes such as LIST, DEC, and OCT.
    pub fn run_dc_sweep2_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        primary: &crate::netlist::DcSweepSpec,
        sweep2: Option<&crate::netlist::DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let Some(sweep2) = sweep2 else {
            return self.run_dc_sweep_spec_with_report_and_abort(
                netlist,
                source_name,
                primary,
                abort,
            );
        };

        let outer_points = sweep2.spec().points();
        if outer_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid second-source sweep parameters".to_string(),
            ));
        }

        let outer_is_temp = sweep2.source.eq_ignore_ascii_case("TEMP")
            || sweep2.source.eq_ignore_ascii_case("TEMPER");
        let outer_is_parameter = Self::netlist_has_numeric_parameter(netlist, &sweep2.source);

        let mut results = Vec::new();
        let mut any_outer_parameter_binding = false;
        for &outer_value in &outer_points {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let swept = if outer_is_temp {
                let mut swept = netlist.clone();
                swept.options.temp = Some(outer_value);
                swept.params.set("TEMP", outer_value);
                swept.params.set("TEMPER", outer_value);
                swept.params.set(
                    "VT",
                    crate::constants::thermal_voltage(
                        crate::analysis::temperature::celsius_to_kelvin(outer_value),
                    ),
                );
                swept
            } else if outer_is_parameter {
                let (swept, bindings) =
                    Self::create_perturbed_netlist(netlist, &sweep2.source, outer_value)?;
                any_outer_parameter_binding |= bindings > 0;
                swept
            } else {
                let mut swept = netlist.clone();
                Self::override_independent_source_dc(&mut swept, &sweep2.source, outer_value)?;
                swept
            };
            let inner =
                self.run_dc_sweep_spec_with_report_and_abort(&swept, source_name, primary, abort)?;
            results.extend(inner);
        }
        if outer_is_parameter && netlist.source_text.is_some() && !any_outer_parameter_binding {
            return Err(SimulationError::Circuit(format!(
                "Second DC sweep parameter '{}' is not bound to any netlist expression",
                sweep2.source
            )));
        }
        Ok(results)
    }

    /// Set the DC operating value of a named top-level independent source.
    fn override_independent_source_dc(
        netlist: &mut Netlist,
        source_name: &str,
        value: Value,
    ) -> Result<(), SimulationError> {
        use crate::netlist::ElementKind;
        for element in &mut netlist.elements {
            if element.name.eq_ignore_ascii_case(source_name) {
                return match &mut element.kind {
                    ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                        *spec = spec.clone().with_dc_value(value);
                        Ok(())
                    }
                    _ => Err(SimulationError::Circuit(format!(
                        "Second DC sweep source '{}' must be an independent source",
                        source_name
                    ))),
                };
            }
        }
        Err(SimulationError::Circuit(format!(
            "Second DC sweep source not found: {}",
            source_name
        )))
    }

    pub fn run_dc_sweep_with_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_dc_sweep_with_report_and_abort(netlist, source_name, start, stop, step, abort)
            .map(|points| {
                points
                    .into_iter()
                    .map(|point| (point.sweep_value, point.result))
                    .collect()
            })
    }

    /// Run a DC sweep and return per-point device operating reports alongside
    /// the solved node/branch vectors.
    pub fn run_dc_sweep_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let spec = crate::netlist::DcSweepSpec::linear(start, stop, step);
        self.run_dc_sweep_spec_with_report_and_abort(netlist, source_name, &spec, abort)
    }

    /// Run a DC sweep from an already parsed sweep specification.
    pub fn run_dc_sweep_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        spec: &crate::netlist::DcSweepSpec,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        let sweep_points = spec.points();

        if sweep_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid sweep parameters".to_string(),
            ));
        }

        if source_name.eq_ignore_ascii_case("TEMP") || source_name.eq_ignore_ascii_case("TEMPER") {
            let mut results = Vec::with_capacity(sweep_points.len());
            for &sweep_value in &sweep_points {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let mut swept = netlist.clone();
                swept.options.temp = Some(sweep_value);
                swept.params.set("TEMP", sweep_value);
                swept.params.set("TEMPER", sweep_value);
                swept.params.set(
                    "VT",
                    crate::constants::thermal_voltage(
                        crate::analysis::temperature::celsius_to_kelvin(sweep_value),
                    ),
                );
                let (result, device_op_report) = self.run_dc_op_with_report(&swept)?;
                results.push(DcSweepPointResult {
                    sweep_value,
                    result,
                    device_op_report,
                });
            }
            return Ok(results);
        }

        if Self::netlist_has_numeric_parameter(netlist, source_name) {
            return self.run_dc_parameter_sweep_spec_with_report_and_abort(
                netlist,
                source_name,
                &sweep_points,
                abort,
            );
        }

        // Build circuit once
        let mut circuit = engine.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Ok(sweep_points
                .into_iter()
                .map(|value| DcSweepPointResult {
                    sweep_value: value,
                    result: Self::build_empty_dc_result(),
                    device_op_report: crate::circuit::DeviceOpReport::default(),
                })
                .collect());
        }

        // Find source index (case-insensitive comparison - SPICE standard)
        let source_name_upper = source_name.to_uppercase();
        let sweep_source = if let Some(index) = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.to_uppercase() == source_name_upper)
        {
            // Store original source state so the sweep is reversible even if a point fails.
            DcSweepSource::Voltage {
                index,
                original_value: circuit.voltage_sources.dc_values[index],
                original_source_spec: circuit.voltage_sources.source_specs[index].take(),
            }
        } else if let Some(index) = circuit
            .current_sources
            .names
            .iter()
            .position(|n| n.to_uppercase() == source_name_upper)
        {
            DcSweepSource::Current {
                index,
                original_value: circuit.current_sources.dc_values[index],
                original_source_spec: circuit.current_sources.source_specs[index].take(),
            }
        } else {
            return Err(SimulationError::Circuit(format!(
                "Source not found: {}",
                source_name
            )));
        };

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let mut matrix = matrix;

        let sorted_node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

        let node_hints = self.collect_node_voltage_hints(netlist, &circuit);

        let sweep_result = (|| -> Result<Vec<DcSweepPointResult>, SimulationError> {
            let mut results = Vec::with_capacity(sweep_points.len());

            // Use previous solution as initial guess for next point.
            // For the first point, apply .NODESET/.IC hints if present.
            let mut prev_solution: Option<Vec<Value>> = None;
            let mut prev_sweep_value: Option<Value> = None;
            let mut dc_sweep_subdivisions = 2;

            for &sweep_value in &sweep_points {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // Update source value.
                sweep_source.set_value(&mut circuit, sweep_value);

                // Solve DC at this point
                // Key optimization: use previous solution as initial guess for faster convergence
                let solution =
                    if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
                        if let Some(seed) = prev_solution.as_deref() {
                            let previous_value = prev_sweep_value.unwrap_or(sweep_value);
                            let start_state = circuit.nonlinear_state_snapshot();
                            match engine.solve_nonlinear_with_guess_and_abort(
                                &mut circuit,
                                &mut matrix,
                                Some(seed),
                                abort,
                            ) {
                                Ok(solution) => {
                                    dc_sweep_subdivisions = 2;
                                    solution
                                }
                                Err(_) => {
                                    circuit.restore_nonlinear_state(start_state.clone());
                                    sweep_source.set_value(&mut circuit, previous_value);
                                    match engine.solve_nonlinear_dc_sweep_target_with_substeps(
                                        &mut circuit,
                                        &mut matrix,
                                        &sweep_source,
                                        previous_value,
                                        sweep_value,
                                        seed,
                                        dc_sweep_subdivisions,
                                        abort,
                                    ) {
                                        Ok((solution, subdivisions)) => {
                                            dc_sweep_subdivisions = subdivisions;
                                            solution
                                        }
                                        Err(substep_error) => {
                                            if abort.is_aborted() {
                                                return Err(substep_error);
                                            }

                                            circuit.restore_nonlinear_state(start_state);
                                            sweep_source.set_value(&mut circuit, sweep_value);
                                            let fresh_attempt = if node_hints.is_empty() {
                                                engine.solve_nonlinear_with_node_hints_and_abort(
                                                    &mut circuit,
                                                    &mut matrix,
                                                    &[],
                                                    abort,
                                                )
                                            } else {
                                                engine.solve_nonlinear_with_node_hints_and_abort(
                                                    &mut circuit,
                                                    &mut matrix,
                                                    &node_hints,
                                                    abort,
                                                )
                                            };
                                            if let Ok(solution) = fresh_attempt {
                                                dc_sweep_subdivisions = 2;
                                                solution
                                            } else {
                                                return Err(substep_error);
                                            }
                                        }
                                    }
                                }
                            }
                        } else if node_hints.is_empty() {
                            engine.solve_nonlinear_with_node_hints_and_abort(
                                &mut circuit,
                                &mut matrix,
                                &[],
                                abort,
                            )?
                        } else {
                            engine.solve_nonlinear_with_node_hints_and_abort(
                                &mut circuit,
                                &mut matrix,
                                &node_hints,
                                abort,
                            )?
                        }
                    } else {
                        if abort.is_aborted() {
                            return Err(SimulationError::Aborted);
                        }
                        engine.solve_linear(&circuit, &mut matrix)?
                    };
                if let Some(message) = circuit.take_xspice_evaluation_error() {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE evaluation failed: {message}"
                    )));
                }

                // Build result
                let mut result = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());
                result.node_names = std::iter::once("0".to_string())
                    .chain(sorted_node_names.iter().cloned())
                    .collect();
                result.branch_names = branch_names.clone();
                for (i, &v) in solution.iter().enumerate() {
                    if i < circuit.num_nodes() {
                        result.node_voltages[i + 1] = v;
                    } else {
                        result.branch_currents[i - circuit.num_nodes()] = v;
                    }
                }

                results.push(DcSweepPointResult {
                    sweep_value,
                    result,
                    device_op_report: circuit.device_op_report(),
                });
                prev_solution = Some(solution);
                prev_sweep_value = Some(sweep_value);
            }

            Ok(results)
        })();

        sweep_source.restore(&mut circuit);

        sweep_result
    }

    fn run_dc_parameter_sweep_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        param_name: &str,
        sweep_points: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let mut results = Vec::with_capacity(sweep_points.len());
        let mut any_binding = false;

        for &sweep_value in sweep_points {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (swept, bindings) =
                Self::create_perturbed_netlist(netlist, param_name, sweep_value)?;
            any_binding |= bindings > 0;
            let (result, device_op_report) = self.run_dc_op_with_report(&swept).map_err(|err| {
                SimulationError::Circuit(format!(
                    "DC parameter sweep {} = {} failed: {}",
                    param_name, sweep_value, err
                ))
            })?;
            results.push(DcSweepPointResult {
                sweep_value,
                result,
                device_op_report,
            });
        }

        if netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "DC sweep parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        Ok(results)
    }

    fn netlist_has_numeric_parameter(netlist: &Netlist, param_name: &str) -> bool {
        netlist
            .params
            .all_params()
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(param_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Netlist;

    fn missing_pwl_path(name: &str) -> String {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        std::env::temp_dir()
            .join(format!(
                "rspice-missing-{name}-{}-{unique}.csv",
                std::process::id()
            ))
            .to_string_lossy()
            .replace('\\', "/")
    }

    #[test]
    fn dc_op_rejects_missing_pwl_file_source() {
        let path = missing_pwl_path("dc");
        let deck = format!(
            "missing PWL file\n\
             V1 in 0 PWL FILE=\"{path}\"\n\
             R1 in 0 1k\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let err = Engine::default()
            .run_dc_op(&netlist)
            .expect_err("missing PWL file must fail before DC solve");

        assert!(
            err.to_string().contains("failed to load PWL file"),
            "unexpected error: {err}"
        );
        assert!(err.to_string().contains(&path));
    }

    #[test]
    fn xspice_integrator_initial_condition_is_initialized_before_dc_op() {
        let deck = r#"
* xspice integrator initial condition
V1 in 0 0
A1 in out integrator out_ic=5 out_lower_limit=-10 out_upper_limit=10
Rload out 0 1k
.op
.end
"#;
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("dc operating point should solve");
        let out_idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap_or_else(|| panic!("out node present in {:?}", result.node_names));

        assert!(
            (result.node_voltages[out_idx] - 5.0).abs() < 1e-9,
            "expected integrator out_ic=5 to drive V(out), got {}",
            result.node_voltages[out_idx]
        );
    }

    #[test]
    fn dc_sweep_supports_independent_current_sources() {
        let deck = "\
current source dc sweep
I1 in 0 0
R1 in 0 1k
.dc I1 -1m 1m 1m
.print dc V(in)
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "I1", -1.0e-3, 1.0e-3, 1.0e-3)
            .expect("current-source DC sweep solves");

        assert_eq!(results.len(), 3);
        for ((actual_sweep, result), (expected_sweep, expected_voltage)) in
            results
                .iter()
                .zip([(-1.0e-3, 1.0), (0.0, 0.0), (1.0e-3, -1.0)])
        {
            assert!(
                (actual_sweep - expected_sweep).abs() < 1.0e-15,
                "unexpected sweep point {actual_sweep}, expected {expected_sweep}"
            );
            let actual_voltage = result
                .try_voltage_named("in")
                .expect("swept node voltage is present");
            assert!(
                (actual_voltage - expected_voltage).abs() < 1.0e-9,
                "unexpected V(in) at I1={actual_sweep}: {actual_voltage}, expected {expected_voltage}"
            );
        }
    }

    #[test]
    fn dc_sweep_supports_nonlinear_independent_current_sources() {
        let deck = "\
current source nonlinear dc sweep
I1 in 0 0
D1 0 in DMOD
.model DMOD D(IS=1e-14 N=1)
.dc I1 1n 1u 4.995e-7
.print dc V(in)
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "I1", 1.0e-9, 1.0e-6, 4.995e-7)
            .expect("current-source nonlinear DC sweep solves");

        assert_eq!(results.len(), 3);
        let voltages = results
            .iter()
            .map(|(_, result)| result.try_voltage_named("in").expect("node is present"))
            .collect::<Vec<_>>();
        assert!(
            voltages.windows(2).all(|pair| pair[0] > pair[1]),
            "diode voltage should become more negative as source current increases: {voltages:?}"
        );
    }

    #[test]
    fn dc_sweep_supports_dependent_parameter_sources() {
        let deck = "\
parameter dc sweep
.param testnorm={0.5K}
.param r1value={testnorm*2.0}
R2 1 0 7k
R1 1 2 {r1value}
V1 2 0 1000
.dc testnorm 0.5k 0.7k 0.1k
.print dc testnorm V(1)
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "testnorm", 0.5e3, 0.7e3, 0.1e3)
            .expect("parameter DC sweep solves");

        assert_eq!(results.len(), 3);
        for ((actual_sweep, result), (expected_sweep, expected_voltage)) in results.iter().zip([
            (500.0, 875.0),
            (600.0, 853.6585365853658),
            (700.0, 833.3333333333334),
        ]) {
            assert!(
                (actual_sweep - expected_sweep).abs() < 1.0e-12,
                "unexpected sweep point {actual_sweep}, expected {expected_sweep}"
            );
            let actual_voltage = result
                .try_voltage_named("1")
                .expect("swept node voltage is present");
            assert!(
                (actual_voltage - expected_voltage).abs() < 1.0e-9,
                "unexpected V(1) at testnorm={actual_sweep}: {actual_voltage}, expected {expected_voltage}"
            );
        }
    }
}
