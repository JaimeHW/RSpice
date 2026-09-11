mod parameter;
mod refinement;

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::sensitivity::{
    AcSensitivity, AcSensitivityOutput, AcSensitivityResult, ElementDesc, ElementType, Sensitivity,
    SensitivityAnalysisError, SensitivityAnalyzer, SensitivityResult, SensitivityUnavailability,
    SensitivityValue,
};
use crate::netlist::{ElementKind, SourceSpec};
use crate::solver::SimulationResult;
use crate::{CircuitData, Complex64, Netlist, Value};
use rspice_veriloga_runtime::arithmetic::ScaledValue;
use std::collections::{HashMap, HashSet};

fn finite_sensitivity(value: ScaledValue) -> Result<Value, SimulationError> {
    let result = value.binary64();
    if !result.is_finite() || (result == 0.0 && !value.is_zero()) {
        return Err(SimulationError::Circuit(
            "Sensitivity value exceeds finite representable precision".to_owned(),
        ));
    }
    Ok(result)
}

fn sensitivity_ratio(
    numerator: impl Iterator<Item = [ScaledValue; 3]> + Clone,
    denominator: impl Iterator<Item = [ScaledValue; 3]> + Clone,
) -> Result<Value, SimulationError> {
    let result = ScaledValue::sum_triple_products_ratio(numerator, denominator)
        .map_err(|error| SimulationError::Circuit(format!("Sensitivity arithmetic: {error:?}")))?;
    finite_sensitivity(result)
}

fn derived_sensitivity_ratio(
    numerator: impl Iterator<Item = [ScaledValue; 3]> + Clone,
    denominator: impl Iterator<Item = [ScaledValue; 3]> + Clone,
) -> Result<SensitivityValue<Value>, SimulationError> {
    ScaledValue::sum_triple_products_ratio(numerator, denominator)
        .map(SensitivityValue::from_scaled)
        .map_err(|error| SimulationError::Circuit(format!("Sensitivity arithmetic: {error:?}")))
}

/// Derivative of the quadratic interpolant at points[0], for either a
/// central or one-sided stencil. Nonuniform representable spacing is valid.
fn sensitivity_three_point(
    points: [Value; 3],
    values: [Value; 3],
) -> Result<Value, SimulationError> {
    if points.iter().chain(&values).any(|value| !value.is_finite())
        || points[0] == points[1]
        || points[0] == points[2]
        || points[1] == points[2]
    {
        return Err(SimulationError::Circuit(
            "Sensitivity stencil needs three distinct finite coordinates and finite samples"
                .to_owned(),
        ));
    }
    let one = ScaledValue::new(1.0);
    let difference = |left: Value, right: Value| {
        ScaledValue::product_sum(ScaledValue::new(left), one, ScaledValue::new(-right), one)
    };
    let a = difference(points[1], points[0]);
    let b = difference(points[2], points[0]);
    let distance = difference(points[2], points[1]);
    sensitivity_ratio(
        [
            [ScaledValue::new(values[1]), b, b],
            [ScaledValue::new(-values[0]), b, b],
            [ScaledValue::new(-values[2]), a, a],
            [ScaledValue::new(values[0]), a, a],
        ]
        .into_iter(),
        [[a, b, distance]].into_iter(),
    )
}

/// Fatal study failures stop before another perturbation is attempted;
/// recoverable failures can be retried at smaller steps. Only typed domain
/// violations can justify one-sided estimates in the refinement driver.
fn sensitivity_trial<T>(
    sample: Result<T, SimulationError>,
) -> Result<Result<T, SimulationError>, SimulationError> {
    match sample {
        Err(error)
            if !matches!(
                &error,
                SimulationError::Circuit(_)
                    | SimulationError::ParameterDomain(_)
                    | SimulationError::Netlist(_)
                    | SimulationError::Solver(_)
                    | SimulationError::ConvergenceFailed(_)
            ) =>
        {
            Err(error)
        }
        sample => Ok(sample),
    }
}

#[derive(Debug, Clone, Copy)]
enum AcSensitivityElementField {
    ResistorValue,
    CapacitorValue,
    InductorValue,
    JilesAthertonValue,
    VcvsGain,
    CccsGain,
    VccsTransconductance,
    CcvsTransresistance,
    SourceMultiplicity,
    BehavioralTc1,
    BehavioralTc2,
    TransmissionZ0,
    TransmissionDelay,
    TransmissionFrequency,
    TransmissionLength,
    Coupling,
}

#[derive(Debug, Clone)]
enum AcSensitivityLocation {
    ElementField {
        element_index: usize,
        field: AcSensitivityElementField,
    },
    ElementParameter {
        element_index: usize,
        parameter_index: usize,
    },
    ElementNamedParameter {
        element_index: usize,
        parameter: String,
    },
    ElementVectorParameter {
        element_index: usize,
        parameter_index: usize,
        entry_index: usize,
    },
    ElementNamedVectorParameter {
        element_index: usize,
        parameter: String,
        entry_index: usize,
        resolved_values: Vec<Value>,
    },
    SourceDc {
        element_index: usize,
    },
    SourceAcMagnitude {
        element_index: usize,
    },
    SourceAcPhaseDegrees {
        element_index: usize,
    },
    ModelParameter {
        model_index: usize,
        parameter: String,
    },
    ModelVectorParameter {
        model_index: usize,
        parameter_index: usize,
        entry_index: usize,
    },
    ModelNamedVectorParameter {
        model_index: usize,
        parameter: String,
        entry_index: usize,
        resolved_values: Vec<Value>,
    },
}

#[derive(Debug, Clone)]
struct AcSensitivityTarget {
    vector_name: String,
    element: String,
    element_type: ElementType,
    parameter: String,
    nominal_value: Value,
    location: AcSensitivityLocation,
}

impl Engine {
    pub(in crate::engine) fn collect_sensitivity_elements(
        circuit: &CircuitData,
    ) -> Vec<ElementDesc> {
        let mut elements = Vec::new();

        for (idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let name = circuit
                .resistors
                .names
                .get(idx)
                .cloned()
                .unwrap_or_else(|| format!("R{}", idx + 1));
            let g = circuit
                .resistors
                .small_signal_conductances
                .get(idx)
                .copied()
                .unwrap_or_else(|| {
                    circuit
                        .resistors
                        .conductances
                        .get(idx)
                        .copied()
                        .unwrap_or(0.0)
                });
            if !g.is_finite() || g == 0.0 {
                continue;
            }

            elements.push(ElementDesc::resistor(
                &name,
                Self::optional_system_index(stamp.pp.row),
                Self::optional_system_index(stamp.nn.row),
                1.0 / g,
            ));
        }

        for (idx, name) in circuit.resistor_branches.names.iter().enumerate() {
            let resistance = circuit.resistor_branches.small_signal_resistances[idx];
            let branch = circuit.resistor_branches.branch_indices[idx];
            if !resistance.is_finite() || branch == 0 {
                continue;
            }
            let mut element = ElementDesc::resistor(
                name,
                Self::optional_system_index(circuit.resistor_branches.node_pos[idx]),
                Self::optional_system_index(circuit.resistor_branches.node_neg[idx]),
                resistance,
            );
            element.branch_index = Some(circuit.get_branch_matrix_index(branch) - 1);
            elements.push(element);
        }

        for idx in 0..circuit.current_sources.names.len() {
            let name = circuit.current_sources.names[idx].clone();
            let value = circuit.current_sources.dc_values[idx];
            if !value.is_finite() {
                continue;
            }

            elements.push(ElementDesc::current_source(
                &name,
                Self::optional_system_index(circuit.current_sources.node_pos[idx]),
                Self::optional_system_index(circuit.current_sources.node_neg[idx]),
                value,
            ));
        }

        for idx in 0..circuit.voltage_sources.names.len() {
            let name = circuit.voltage_sources.names[idx].clone();
            let value = circuit.voltage_sources.dc_values[idx];
            let branch_ordinal = circuit.voltage_sources.branch_indices[idx];
            if !value.is_finite() || branch_ordinal == 0 {
                continue;
            }

            elements.push(ElementDesc::voltage_source(
                &name,
                Self::optional_system_index(circuit.voltage_sources.node_pos[idx]),
                Self::optional_system_index(circuit.voltage_sources.node_neg[idx]),
                circuit.get_branch_matrix_index(branch_ordinal) - 1,
                value,
            ));
        }

        elements
    }

    /// Run DC operating-point sensitivity using the linearized MNA system.
    pub fn run_sensitivity_linearized(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
    ) -> Result<SensitivityResult, SimulationError> {
        self.run_sensitivity_linearized_with_abort(netlist, output_pos, output_neg, &NoAbort)
    }

    /// Run adjoint DC sensitivity with cooperative cancellation of its
    /// operating-point solve.
    pub fn run_sensitivity_linearized_with_abort(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        abort: &dyn AbortSignal,
    ) -> Result<SensitivityResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if output_pos == 0 {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::warn_xspice_mif_analysis_boundary(
            &circuit,
            "Sensitivity",
            "linearized sensitivity reports native element/source derivatives and does not use ngspice MIF DEVsen* hooks",
        );
        Self::ensure_no_mixed_signal_analysis(&circuit, "sensitivity analysis")?;
        Self::validate_sensitivity_node("output", output_pos, circuit.num_nodes())?;
        if let Some(output_neg) = output_neg {
            Self::validate_sensitivity_node("reference", output_neg, circuit.num_nodes())?;
        }
        let matrix_size = circuit.matrix_size();
        engine.ensure_result_shape(matrix_size, matrix_size.saturating_mul(4).saturating_add(1))?;

        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit
            .prepare_behavioral_small_signal(&dc_solution)
            .map_err(SimulationError::Circuit)?;

        let mut small_signal =
            Self::try_build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, 0.0)?;
        let elements = Self::collect_sensitivity_elements(&circuit);
        if elements.is_empty() {
            return Err(SimulationError::Circuit(
                "Sensitivity analysis found no eligible linear elements or independent sources"
                    .to_string(),
            ));
        }
        engine.ensure_result_values(elements.len().saturating_mul(3).saturating_add(1))?;

        let output_index = output_pos - 1;
        let reference_index = output_neg.and_then(Self::optional_system_index);
        let mut observation = vec![Complex64::new(0.0, 0.0); matrix_size];
        observation[output_index] = Complex64::new(1.0, 0.0);
        if let Some(reference) = reference_index {
            observation[reference] -= Complex64::new(1.0, 0.0);
        }
        let mut complex_adjoint = Vec::with_capacity(matrix_size);
        small_signal
            .solve_transpose_into(&observation, &mut complex_adjoint)
            .map_err(SimulationError::Solver)?;
        let mut adjoint = Vec::with_capacity(matrix_size);
        for value in complex_adjoint {
            let imaginary_tolerance = 64.0 * Value::EPSILON * value.re.abs().max(1.0);
            if !value.re.is_finite()
                || !value.im.is_finite()
                || value.im.abs() > imaginary_tolerance
            {
                return Err(SimulationError::Circuit(
                    "DC sensitivity sparse adjoint produced a non-real or non-finite value"
                        .to_string(),
                ));
            }
            adjoint.push(value.re);
        }

        let analyzer =
            SensitivityAnalyzer::with_precomputed_adjoint(dc_solution, adjoint, elements).ok_or(
                SimulationError::Solver(crate::solver::SolverError::SingularMatrix),
            )?;
        analyzer
            .analyze_precomputed_with_abort(output_index, reference_index, abort)
            .map_err(|error| match error {
                SensitivityAnalysisError::Aborted => SimulationError::Aborted,
            })?
            .ok_or(SimulationError::Solver(
                crate::solver::SolverError::SingularMatrix,
            ))
    }

    fn validate_sensitivity_node(
        role: &str,
        node: usize,
        num_nodes: usize,
    ) -> Result<(), SimulationError> {
        if node > num_nodes {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity {role} node {node} is outside circuit node range 0..={num_nodes}"
            )));
        }
        Ok(())
    }

    /// Clone a netlist with parameter overrides applied.
    ///
    /// Returns the materialized netlist and the number of substitutions made, so a
    /// caller can tell "applied nothing" from "applied everything".
    pub fn create_perturbed_netlist_multi(
        netlist: &Netlist,
        overrides: &[(String, Value)],
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi_with_abort(netlist, overrides, &NoAbort)
    }

    pub(crate) fn create_perturbed_netlist_multi_with_abort(
        netlist: &Netlist,
        overrides: &[(String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi_with_limits_and_abort(
            netlist,
            overrides,
            crate::resource::ResourceLimits::default(),
            abort,
        )
    }

    pub(crate) fn create_perturbed_netlist_with_limits_and_abort(
        netlist: &Netlist,
        param_name: &str,
        param_value: Value,
        resource_limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi_with_limits_and_abort(
            netlist,
            &[(param_name.to_ascii_uppercase(), param_value)],
            resource_limits,
            abort,
        )
    }

    pub(crate) fn create_perturbed_netlist_multi_with_limits_and_abort(
        netlist: &Netlist,
        overrides: &[(String, Value)],
        resource_limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::replay_parameter_overrides(netlist, overrides, None, resource_limits, abort)
    }

    fn replay_parameter_overrides(
        netlist: &Netlist,
        overrides: &[(String, Value)],
        direction: Option<&str>,
        resource_limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut override_map: HashMap<String, Value> = HashMap::new();
        for (index, (name, value)) in overrides.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "Parameter override '{name}' must be finite"
                )));
            }
            override_map.insert(name.to_ascii_uppercase(), *value);
        }

        let mut ordered_overrides: Vec<(String, Value)> = override_map.into_iter().collect();
        ordered_overrides.sort_by(|a, b| a.0.cmp(&b.0));

        let mut param_overrides = Vec::new();
        let mut device_overrides = Vec::new();
        for (index, (name, value)) in ordered_overrides.into_iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Some((device_name, param_name)) = Self::split_device_parameter_override(&name) {
                device_overrides.push((device_name, param_name, value));
            } else {
                param_overrides.push((name, value));
            }
        }

        let mut perturbed = netlist.clone();
        let mut retained_parameters = netlist.ast_overlay.parameters.clone();
        retained_parameters.extend(param_overrides.iter().cloned());
        let effective_overrides: Vec<_> = retained_parameters
            .iter()
            .map(|(name, value)| crate::netlist::ParameterOverride {
                name: name.clone(),
                value: *value,
                global: !netlist.params.has_parameter_binding(name)
                    && netlist.params.has_any_parameter_binding(name),
                direction: direction.is_some_and(|target| name.eq_ignore_ascii_case(target)),
            })
            .collect();
        for (index, parameter) in effective_overrides.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if parameter.global {
                perturbed
                    .params
                    .set_global(&parameter.name, parameter.value);
            } else {
                perturbed.params.set(&parameter.name, parameter.value);
            }
        }
        perturbed.ast_overlay.parameters = retained_parameters.clone();
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::NetlistBytes,
            perturbed.retained_source_bytes(),
            resource_limits.max_netlist_bytes,
        )?;
        let applied_device_overrides = Self::apply_device_parameter_overrides_with_abort(
            &mut perturbed,
            &device_overrides,
            abort,
        )?;

        // Identity comes from the resolved environment. Source occurrences
        // cannot distinguish dependencies, included definitions or zero influence.
        let defined_parameters = param_overrides
            .iter()
            .filter(|(name, _)| netlist.params.has_any_parameter_binding(name))
            .count();
        let Some(source) = &netlist.source_text else {
            return Ok((perturbed, defined_parameters + applied_device_overrides));
        };
        let parse_options = crate::netlist::NetlistParseOptions {
            statistical_mode: netlist.params.statistical_mode(),
            statistical_seed: Some(netlist.params.random().seed()),
            expression_dialect: netlist.params.expression_dialect(),
            parameter_redefinition_policy: netlist.params.parameter_redefinition_policy(),
            parameter_redefinition_diagnostic_policy: netlist
                .params
                .parameter_redefinition_diagnostic_policy(),
            resource_limits,
        };
        let mut reparsed = netlist
            .replay_root_source_with_parameter_overrides_and_abort(
                source,
                parse_options,
                &effective_overrides,
                abort,
            )
            .map_err(|error| match error {
                crate::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
                crate::netlist::ParseWithAbortError::Parse(
                    crate::netlist::ParseError::ResourceLimit(error),
                ) => SimulationError::ResourceLimit(error),
                crate::netlist::ParseWithAbortError::Parse(error) => {
                    SimulationError::Netlist(format!(
                        "Failed to reparse netlist for parameter override set {:?}: {}",
                        param_overrides, error
                    ))
                }
            })?;
        Self::reapply_ast_overlay_with_abort(&mut reparsed, &netlist.ast_overlay, abort)?;
        reparsed.ast_overlay.parameters = retained_parameters;
        let applied_device_overrides = Self::apply_device_parameter_overrides_with_abort(
            &mut reparsed,
            &device_overrides,
            abort,
        )?;

        Ok((reparsed, defined_parameters + applied_device_overrides))
    }

    fn split_device_parameter_override(name: &str) -> Option<(String, String)> {
        let (device_name, param_name) = name.rsplit_once(':')?;
        let device_name = device_name.trim();
        let param_name = param_name.trim();
        (!device_name.is_empty() && !param_name.is_empty())
            .then(|| (device_name.to_string(), param_name.to_string()))
    }

    fn apply_device_parameter_overrides_with_abort(
        netlist: &mut Netlist,
        overrides: &[(String, String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<usize, SimulationError> {
        let mut applied = 0;
        for (index, (device_name, param_name, value)) in overrides.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let element = netlist
                .elements
                .iter_mut()
                .find(|element| element.name.eq_ignore_ascii_case(device_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP DATA target '{}:{}' not found in netlist",
                        device_name, param_name
                    ))
                })?;
            let canonical_parameter =
                Self::canonical_device_parameter(&element.kind, Some(param_name));
            Self::apply_device_step_value(&mut element.kind, Some(&canonical_parameter), *value)?;
            netlist.ast_overlay.device_parameters.insert(
                (
                    element.name.to_ascii_uppercase(),
                    canonical_parameter.to_ascii_uppercase(),
                ),
                *value,
            );
            applied += 1;
        }
        Ok(applied)
    }

    fn reapply_ast_overlay_with_abort(
        netlist: &mut Netlist,
        overlay: &crate::netlist::NetlistAstOverlay,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for (index, ((device_name, parameter_name), value)) in
            overlay.device_parameters.iter().enumerate()
        {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let element = netlist
                .elements
                .iter_mut()
                .find(|element| element.name.eq_ignore_ascii_case(device_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "retained device override '{}:{}' no longer resolves after source replay",
                        device_name, parameter_name
                    ))
                })?;
            Self::apply_device_step_value(&mut element.kind, Some(parameter_name), *value)?;
        }
        netlist.ast_overlay = overlay.clone();
        Ok(())
    }

    fn validate_parameter_sensitivity_inputs(
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<(), SimulationError> {
        if !param_value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity param_value must be finite, got {param_value}"
            )));
        }
        if let Some(delta) = delta
            && (!delta.is_finite() || delta <= 0.0)
        {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity delta must be a positive finite number, got {delta}"
            )));
        }
        Ok(())
    }

    fn sensitivity_step(
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<Value, SimulationError> {
        Self::validate_parameter_sensitivity_inputs(param_value, delta)?;
        // A dimensionful floor can exceed the entire nominal value (for
        // example a 100 fF capacitance) and cross its physical domain.
        let h = delta.unwrap_or_else(|| Self::relative_sensitivity_step(param_value, 1e-12));
        let lower = param_value - h;
        let upper = param_value + h;
        if !(lower.is_finite() && lower < param_value || upper.is_finite() && upper > param_value) {
            return Err(SimulationError::Circuit(
                "Sensitivity perturbations must be distinct, finite, representable values around the nominal parameter".to_owned(),
            ));
        }
        Ok(h)
    }

    fn relative_sensitivity_step(nominal: Value, zero_scale: Value) -> Value {
        if nominal == 0.0 {
            zero_scale
        } else {
            let magnitude = nominal.abs();
            let upward = magnitude.next_up();
            let ulp = if upward.is_finite() {
                upward - magnitude
            } else {
                magnitude - magnitude.next_down()
            };
            // Leave enough distinct coordinates for a coarse and refined
            // stencil, including at subnormal and maximum finite values.
            (magnitude * 1e-3).max(8.0 * ulp)
        }
    }

    /// Run sensitivity analysis
    ///
    /// Computes dVout/dparam using captured analytic derivatives when available,
    /// or finite-difference refinement otherwise.
    /// Useful for design optimization and tolerance analysis.
    /// A defined parameter may have zero influence on the selected output.
    /// `delta` is an initial step; calibration may enlarge it to resolve probe changes.
    pub fn run_sensitivity(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<Value, SimulationError> {
        self.run_sensitivity_with_abort(
            netlist,
            output_node,
            param_name,
            param_value,
            delta,
            &NoAbort,
        )
    }

    /// Run parameter DC sensitivity with cooperative cancellation.
    #[allow(clippy::too_many_arguments)]
    pub fn run_sensitivity_with_abort(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<Value, SimulationError> {
        self.run_output_sensitivity_with_abort(
            netlist,
            AcSensitivityOutput::Voltage {
                positive: output_node,
                negative: None,
            },
            param_name,
            param_value,
            delta,
            &mut 0,
            abort,
        )
    }

    /// Differentiate a DC voltage or branch-current probe with respect to an authored design parameter.
    /// Qualified linear circuits use captured field derivatives and a sparse
    /// adjoint. Other circuits replay expressions through the refinement driver.
    /// `runs` includes prior study runs and each nominal or refinement run.
    /// `delta` sets the initial step when refinement is needed.
    #[allow(clippy::too_many_arguments)]
    pub fn run_output_sensitivity_with_abort(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
        runs: &mut usize,
        abort: &dyn AbortSignal,
    ) -> Result<Value, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Self::validate_parameter_sensitivity_inputs(param_value, delta)?;
        if let Some((_, derivative)) = self.linear_parameter_sensitivity(
            netlist,
            &output,
            param_name,
            param_value,
            None,
            runs,
            abort,
        )? {
            return Ok(derivative[0].re);
        }
        let h = Self::sensitivity_step(param_value, delta)?;
        *runs = runs.saturating_add(1);
        self.ensure_batch_runs(*runs)?;
        let evaluate = |candidate| {
            let (perturbed, references) = Self::create_perturbed_netlist_with_limits_and_abort(
                netlist,
                param_name,
                candidate,
                self.config.resource_limits,
                abort,
            )?;
            if references == 0 {
                return Err(SimulationError::Circuit(format!(
                    "Parameter '{param_name}' is not bound to any netlist expression"
                )));
            }
            let result = self.run_dc_op_with_abort(&perturbed, abort)?;
            let value = Self::dc_sensitivity_output_value(&result, &output)?;
            Ok(vec![Complex64::new(value, 0.0)])
        };
        let nominal = evaluate(param_value)?;
        let derivative =
            self.refine_sensitivity(param_name, param_value, h, &nominal, runs, abort, evaluate)?;
        Ok(derivative[0].re)
    }

    /// Run AC sensitivity analysis for a parameter across frequencies.
    ///
    /// Differentiates the complex output voltage, then projects that derivative
    /// onto the nominal output phasor to compute d|Vout|/dp. A zero nominal
    /// output with a nonzero complex derivative has no magnitude derivative;
    /// this numeric-only API returns a diagnostic in that case.
    pub fn run_sensitivity_ac(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        frequencies: &[Value],
        delta: Option<Value>,
    ) -> Result<Vec<Value>, SimulationError> {
        self.run_sensitivity_ac_with_abort(
            netlist,
            output_node,
            param_name,
            param_value,
            frequencies,
            delta,
            &NoAbort,
        )
    }

    /// Run parameter AC sensitivity with cooperative cancellation.
    #[allow(clippy::too_many_arguments)]
    pub fn run_sensitivity_ac_with_abort(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        frequencies: &[Value],
        delta: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        self.run_output_sensitivity_ac_with_abort(
            netlist,
            AcSensitivityOutput::Voltage {
                positive: output_node,
                negative: None,
            },
            param_name,
            param_value,
            frequencies,
            delta,
            &mut 0,
            abort,
        )?
        .into_iter()
        .zip(frequencies)
        .map(|(value, frequency)| match value {
            SensitivityValue::Available(value) => Ok(value),
            SensitivityValue::Unavailable { unavailable } => Err(SimulationError::Circuit(format!(
                "AC output-magnitude sensitivity to parameter '{param_name}' at {frequency} Hz is unavailable ({})",
                unavailable.as_str()
            ))),
        })
        .collect()
    }

    /// Differentiate AC probe magnitude with respect to an authored design parameter.
    /// Undefined or unrepresentable magnitudes retain their explicit reason per point.
    /// Qualified linear circuits use captured field derivatives and a sparse
    /// adjoint. Other circuits replay expressions through the refinement driver.
    /// `runs` includes prior study runs and each nominal or refinement run.
    /// `delta` sets the initial step when refinement is needed.
    #[allow(clippy::too_many_arguments)]
    pub fn run_output_sensitivity_ac_with_abort(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        param_name: &str,
        param_value: Value,
        frequencies: &[Value],
        delta: Option<Value>,
        runs: &mut usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<SensitivityValue<Value>>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Self::validate_parameter_sensitivity_inputs(param_value, delta)?;
        super::ac::validate_ac_frequencies(frequencies)?;
        self.ensure_analysis_points(frequencies.len())?;
        if let Some((nominal, derivatives)) = self.linear_parameter_sensitivity(
            netlist,
            &output,
            param_name,
            param_value,
            Some(frequencies),
            runs,
            abort,
        )? {
            return Self::project_parameter_magnitude(&nominal, derivatives, abort);
        }
        let h = Self::sensitivity_step(param_value, delta)?;
        *runs = runs.saturating_add(1);
        self.ensure_batch_runs(*runs)?;
        // Replay at every coordinate, including the requested nominal value:
        // it need not equal the value originally authored in the netlist.
        // Retain only this probe between runs, not every node's AC traces.
        let evaluate = |candidate| {
            let (perturbed, references) = Self::create_perturbed_netlist_with_limits_and_abort(
                netlist,
                param_name,
                candidate,
                self.config.resource_limits,
                abort,
            )?;
            if references == 0 {
                return Err(SimulationError::Circuit(format!(
                    "Parameter '{param_name}' is not bound to any netlist expression"
                )));
            }
            let results = self.run_ac_with_abort(&perturbed, frequencies, abort)?;
            Self::ac_sensitivity_outputs(&results, &output, frequencies, abort)
        };
        let nominal = evaluate(param_value)?;
        let derivatives =
            self.refine_sensitivity(param_name, param_value, h, &nominal, runs, abort, evaluate)?;
        Self::project_parameter_magnitude(&nominal, derivatives, abort)
    }

    fn project_parameter_magnitude(
        nominal: &[Complex64],
        derivatives: Vec<Complex64>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<SensitivityValue<Value>>, SimulationError> {
        nominal
            .iter()
            .zip(derivatives)
            .map(|(&value, derivative)| {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                Ok(SensitivityValue::magnitude(value, derivative))
            })
            .collect()
    }

    fn flattened_sensitivity_netlist(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Netlist, SimulationError> {
        let flattened = crate::netlist::flatten_netlist_with_models_config_with_abort(
            netlist,
            crate::netlist::FlattenerConfig {
                max_depth: self.config.resource_limits.max_hierarchy_depth,
                max_elements: self.config.resource_limits.max_flattened_elements,
                ..crate::netlist::FlattenerConfig::default()
            },
            abort,
        )
        .map_err(|error| match error {
            crate::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
            crate::netlist::ParseWithAbortError::Parse(
                crate::netlist::ParseError::ResourceLimit(error),
            ) => SimulationError::ResourceLimit(error),
            crate::netlist::ParseWithAbortError::Parse(error) => {
                SimulationError::Netlist(error.to_string())
            }
        })?;
        let mut flat = netlist.clone();
        flat.elements = flattened.elements;
        flat.subcircuits.clear();
        flat.models.extend(flattened.scoped_models);
        flat.initial_conditions
            .extend(flattened.scoped_initial_conditions);
        flat.node_sets.extend(flattened.scoped_node_sets);
        // Parameter perturbations below edit the resolved AST directly. A
        // retained source would cause generic override helpers to reparse the
        // original hierarchy and discard those edits.
        flat.source_text = None;
        flat.source_path = None;
        Ok(flat)
    }

    fn source_has_explicit_ac(spec: &SourceSpec) -> bool {
        match spec {
            SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
                Self::source_has_explicit_ac(inner)
            }
            SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::AcTransient { .. }
            | SourceSpec::DcAcTransient { .. } => true,
            SourceSpec::DcTransient { transient, .. } => Self::source_has_explicit_ac(transient),
            _ => false,
        }
    }

    fn sensitivity_element_type(kind: &ElementKind) -> ElementType {
        match kind {
            ElementKind::Resistor { .. } => ElementType::Resistor,
            ElementKind::Capacitor { .. } => ElementType::Capacitor,
            ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. } => {
                ElementType::Inductor
            }
            ElementKind::VoltageSource(_)
            | ElementKind::VoltageSourceDeferred(_)
            | ElementKind::RfPortDeferred { .. } => ElementType::VoltageSource,
            ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => {
                ElementType::CurrentSource
            }
            ElementKind::Diode { .. } => ElementType::Diode,
            ElementKind::Bjt { .. } => ElementType::Bjt,
            ElementKind::Mosfet { .. } => ElementType::Mosfet,
            ElementKind::Jfet { .. } => ElementType::Jfet,
            ElementKind::Mesfet { .. } => ElementType::Mesfet,
            ElementKind::XyceMemristor { .. } => ElementType::Other,
            ElementKind::Vccs { .. } => ElementType::Transconductance,
            ElementKind::Ccvs { .. } => ElementType::Transresistance,
            ElementKind::Vcvs { .. } | ElementKind::Cccs { .. } => ElementType::Other,
            ElementKind::BehavioralVoltage { .. } | ElementKind::BehavioralCurrent { .. } => {
                ElementType::BehavioralSource
            }
            ElementKind::VSwitch { .. }
            | ElementKind::ISwitch { .. }
            | ElementKind::GenericSwitch { .. } => ElementType::Switch,
            ElementKind::TransmissionLine { .. } => ElementType::TransmissionLine,
            ElementKind::Coupling { .. } => ElementType::Coupling,
            ElementKind::Xspice { .. } => ElementType::Xspice,
            ElementKind::Subcircuit { .. } | ElementKind::PspiceChebyshev { .. } => {
                ElementType::Other
            }
        }
    }

    fn sensitivity_instance_params(kind: &ElementKind) -> Option<&[(String, Value)]> {
        match kind {
            ElementKind::Resistor {
                instance_params, ..
            }
            | ElementKind::Capacitor {
                instance_params, ..
            }
            | ElementKind::Inductor {
                instance_params, ..
            }
            | ElementKind::Diode {
                instance_params, ..
            }
            | ElementKind::Bjt {
                instance_params, ..
            }
            | ElementKind::Mosfet {
                instance_params, ..
            }
            | ElementKind::Jfet {
                instance_params, ..
            }
            | ElementKind::Mesfet {
                instance_params, ..
            } => Some(instance_params),
            ElementKind::Xspice { params, .. } => Some(params),
            _ => None,
        }
    }

    fn sensitivity_model_name(kind: &ElementKind) -> Option<&str> {
        match kind {
            ElementKind::Resistor { model, .. }
            | ElementKind::Capacitor { model, .. }
            | ElementKind::Inductor { model, .. }
            | ElementKind::TransmissionLine { model, .. } => model.as_deref(),
            ElementKind::JilesAthertonInductor { model, .. }
            | ElementKind::Diode { model, .. }
            | ElementKind::Bjt { model, .. }
            | ElementKind::Mosfet { model, .. }
            | ElementKind::Jfet { model, .. }
            | ElementKind::Mesfet { model, .. }
            | ElementKind::VSwitch { model, .. }
            | ElementKind::ISwitch { model, .. }
            | ElementKind::GenericSwitch { model, .. }
            | ElementKind::Xspice { model, .. } => Some(model),
            _ => None,
        }
    }

    fn is_discrete_sensitivity_parameter(parameter: &str) -> bool {
        let upper = parameter.trim().to_ascii_uppercase();
        matches!(
            upper.as_str(),
            "LEVEL"
                | "VERSION"
                | "TYPE"
                | "POLARITY"
                | "PARAMCHK"
                | "BINUNIT"
                | "OFF"
                | "ON"
                | "SELECT"
                | "METHOD"
        ) || upper.ends_with("MOD")
            || upper.ends_with("MODE")
            || upper.ends_with("FLAG")
    }

    fn is_continuous_sensitivity_parameter(
        parameter: &str,
        specs: Option<&[crate::xspice::ParamSpec]>,
        vector: bool,
    ) -> bool {
        if let Some(spec) = specs.and_then(|specs| {
            specs
                .iter()
                .find(|spec| spec.name.eq_ignore_ascii_case(parameter))
        }) {
            return spec.param_type
                == if vector {
                    crate::xspice::ParamType::RealVector
                } else {
                    crate::xspice::ParamType::Real
                };
        }
        // Native device families do not yet all publish parameter descriptors.
        // A declared code-model type always takes precedence over this fallback.
        !Self::is_discrete_sensitivity_parameter(parameter)
    }

    fn add_ac_sensitivity_target(
        targets: &mut Vec<AcSensitivityTarget>,
        seen: &mut HashSet<String>,
        target: AcSensitivityTarget,
    ) {
        if target.nominal_value.is_finite() && seen.insert(target.vector_name.to_ascii_uppercase())
        {
            targets.push(target);
        }
    }

    fn resolved_model_scalar_params(
        netlist: &Netlist,
        model: &crate::netlist::ModelDef,
    ) -> Result<Vec<(String, Value)>, SimulationError> {
        Self::resolved_expression_params(
            &netlist.params,
            &model.params,
            &model.expr_params,
            &format!("model '{}'", model.name),
        )
    }

    fn resolved_expression_params(
        base_context: &crate::netlist::ParamContext,
        numeric: &[(String, Value)],
        expressions: &[(String, String)],
        owner: &str,
    ) -> Result<Vec<(String, Value)>, SimulationError> {
        let mut resolved = Vec::new();
        let mut seen = HashSet::new();
        let mut context = base_context.clone();
        for (name, value) in numeric {
            context.set(name, *value);
            if seen.insert(name.to_ascii_uppercase()) {
                resolved.push((name.clone(), *value));
            }
        }

        let mut pending = expressions.to_vec();
        while !pending.is_empty() {
            let mut next = Vec::new();
            let mut progressed = false;
            for (name, expression) in pending {
                match crate::netlist::expr::eval_expression(&expression, &context) {
                    Ok(value) if value.is_finite() => {
                        context.set(&name, value);
                        if seen.insert(name.to_ascii_uppercase()) {
                            resolved.push((name, value));
                        }
                        progressed = true;
                    }
                    Ok(value) => {
                        return Err(SimulationError::Circuit(format!(
                            "AC sensitivity {owner} parameter '{name}' resolved to non-finite value {value}"
                        )));
                    }
                    Err(_) => next.push((name, expression)),
                }
            }
            if !progressed {
                let unresolved = next
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(SimulationError::Circuit(format!(
                    "AC sensitivity could not resolve {owner} parameter(s): {unresolved}"
                )));
            }
            pending = next;
        }
        Ok(resolved)
    }

    fn resolved_vector_expression_params(
        base_context: &crate::netlist::ParamContext,
        scalar_params: &[(String, Value)],
        vectors: &[(String, Vec<String>)],
        owner: &str,
    ) -> Result<Vec<(String, Vec<Value>)>, SimulationError> {
        let mut context = base_context.clone();
        for (name, value) in scalar_params {
            context.set(name, *value);
        }
        vectors
            .iter()
            .map(|(name, expressions)| {
                let values = expressions
                    .iter()
                    .map(|expression| {
                        crate::netlist::expr::eval_expression(expression, &context).map_err(
                            |error| {
                                SimulationError::Circuit(format!(
                                    "AC sensitivity could not resolve {owner} vector parameter '{name}' entry '{expression}': {error}"
                                ))
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if let Some(value) = values.iter().find(|value| !value.is_finite()) {
                    return Err(SimulationError::Circuit(format!(
                        "AC sensitivity {owner} vector parameter '{name}' resolved to non-finite value {value}"
                    )));
                }
                Ok((name.clone(), values))
            })
            .collect()
    }

    fn collect_ac_sensitivity_targets(
        netlist: &Netlist,
        resource_limits: crate::resource::ResourceLimits,
    ) -> Result<Vec<AcSensitivityTarget>, SimulationError> {
        let mut targets = Vec::new();
        let mut seen = HashSet::new();
        let mut referenced_models = HashSet::new();
        // Use the same builtin catalog as circuit construction, and only pay
        // for it when an A-device is present. Keep resolved aliases for the
        // model-card pass as well as direct instance parameters.
        let xspice_registry = netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Xspice { .. }))
            .then(crate::xspice::CodeModelRegistry::with_builtins);
        let mut xspice_models = HashMap::new();

        for (element_index, element) in netlist.elements.iter().enumerate() {
            let name = element.name.clone();
            let element_type = Self::sensitivity_element_type(&element.kind);
            if let Some(model) = Self::sensitivity_model_name(&element.kind) {
                referenced_models.insert(model.to_ascii_uppercase());
            }
            let code_model = if let ElementKind::Xspice { model, .. } = &element.kind {
                let model_type = netlist
                    .models
                    .iter()
                    .find(|definition| definition.name.eq_ignore_ascii_case(model))
                    .map_or(model.as_str(), |definition| definition.model_type.as_str());
                let resolved = xspice_registry
                    .as_ref()
                    .and_then(|registry| registry.get(model_type));
                if let Some(resolved) = &resolved {
                    xspice_models
                        .insert(model.to_ascii_uppercase(), std::sync::Arc::clone(resolved));
                }
                resolved
            } else {
                None
            };
            let parameter_specs = code_model.as_ref().map(|model| model.parameters());

            let mut primary_aliases: &[&str] = &[];
            let mut add_field =
                |field: AcSensitivityElementField, parameter: &str, nominal_value: Value| {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: name.clone(),
                            element: name.clone(),
                            element_type,
                            parameter: parameter.to_string(),
                            nominal_value,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field,
                            },
                        },
                    );
                };

            match &element.kind {
                ElementKind::Resistor { value, .. } if value.is_finite() => {
                    primary_aliases = &["R", "RES", "RESISTANCE", "VALUE"];
                    add_field(AcSensitivityElementField::ResistorValue, "R", *value);
                }
                ElementKind::Capacitor { value, .. } if value.is_finite() => {
                    primary_aliases = &["C", "CAP", "CAPACITANCE", "VALUE"];
                    add_field(AcSensitivityElementField::CapacitorValue, "C", *value);
                }
                ElementKind::Inductor { value, .. } if value.is_finite() => {
                    primary_aliases = &["L", "IND", "INDUCTANCE", "VALUE"];
                    add_field(AcSensitivityElementField::InductorValue, "L", *value);
                }
                ElementKind::JilesAthertonInductor { value, .. } => {
                    add_field(AcSensitivityElementField::JilesAthertonValue, "L", *value);
                }
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: name.clone(),
                            element: name.clone(),
                            element_type,
                            parameter: "DC".to_string(),
                            nominal_value: crate::engine::extract_dc_value_with_limits(
                                spec,
                                resource_limits,
                            ),
                            location: AcSensitivityLocation::SourceDc { element_index },
                        },
                    );
                    if Self::source_has_explicit_ac(spec) {
                        let (magnitude, phase_radians) = crate::engine::extract_ac_value(spec);
                        for (suffix, parameter, nominal_value, location) in [
                            (
                                "AC_MAG",
                                "AC_MAG",
                                magnitude,
                                AcSensitivityLocation::SourceAcMagnitude { element_index },
                            ),
                            (
                                "AC_PHASE",
                                "AC_PHASE",
                                phase_radians.to_degrees(),
                                AcSensitivityLocation::SourceAcPhaseDegrees { element_index },
                            ),
                        ] {
                            Self::add_ac_sensitivity_target(
                                &mut targets,
                                &mut seen,
                                AcSensitivityTarget {
                                    vector_name: format!("{name}_{suffix}"),
                                    element: name.clone(),
                                    element_type,
                                    parameter: parameter.to_string(),
                                    nominal_value,
                                    location,
                                },
                            );
                        }
                    }
                }
                ElementKind::Vcvs { gain, .. } => {
                    add_field(AcSensitivityElementField::VcvsGain, "GAIN", *gain);
                }
                ElementKind::Cccs { gain, .. } => {
                    add_field(AcSensitivityElementField::CccsGain, "GAIN", *gain);
                }
                ElementKind::Vccs {
                    transconductance,
                    multiplicity,
                    ..
                } => {
                    add_field(
                        AcSensitivityElementField::VccsTransconductance,
                        "GM",
                        *transconductance,
                    );
                    add_field(
                        AcSensitivityElementField::SourceMultiplicity,
                        "M",
                        multiplicity.value,
                    );
                }
                ElementKind::Ccvs {
                    transresistance, ..
                } => add_field(
                    AcSensitivityElementField::CcvsTransresistance,
                    "RM",
                    *transresistance,
                ),
                ElementKind::BehavioralVoltage {
                    tc1,
                    tc2,
                    multiplicity,
                    ..
                }
                | ElementKind::BehavioralCurrent {
                    tc1,
                    tc2,
                    multiplicity,
                    ..
                } => {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_M"),
                            element: name.clone(),
                            element_type,
                            parameter: "M".to_string(),
                            nominal_value: multiplicity.value,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field: AcSensitivityElementField::SourceMultiplicity,
                            },
                        },
                    );
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_TC1"),
                            element: name.clone(),
                            element_type,
                            parameter: "TC1".to_string(),
                            nominal_value: *tc1,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field: AcSensitivityElementField::BehavioralTc1,
                            },
                        },
                    );
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_TC2"),
                            element: name.clone(),
                            element_type,
                            parameter: "TC2".to_string(),
                            nominal_value: *tc2,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field: AcSensitivityElementField::BehavioralTc2,
                            },
                        },
                    );
                }
                ElementKind::TransmissionLine {
                    z0, td, freq, nl, ..
                } => {
                    for (field, parameter, nominal) in [
                        (AcSensitivityElementField::TransmissionZ0, "Z0", *z0),
                        (AcSensitivityElementField::TransmissionDelay, "TD", *td),
                        (
                            AcSensitivityElementField::TransmissionFrequency,
                            "FREQ",
                            *freq,
                        ),
                        (AcSensitivityElementField::TransmissionLength, "NL", *nl),
                    ] {
                        if let Some(nominal) = nominal {
                            Self::add_ac_sensitivity_target(
                                &mut targets,
                                &mut seen,
                                AcSensitivityTarget {
                                    vector_name: format!("{name}_{parameter}"),
                                    element: name.clone(),
                                    element_type,
                                    parameter: parameter.to_string(),
                                    nominal_value: nominal,
                                    location: AcSensitivityLocation::ElementField {
                                        element_index,
                                        field,
                                    },
                                },
                            );
                        }
                    }
                }
                ElementKind::Coupling { coefficient, .. } => {
                    add_field(AcSensitivityElementField::Coupling, "K", *coefficient);
                }
                _ => {}
            }

            if let Some(parameters) = Self::sensitivity_instance_params(&element.kind) {
                for (parameter_index, (parameter, nominal_value)) in parameters.iter().enumerate() {
                    if !Self::is_continuous_sensitivity_parameter(parameter, parameter_specs, false)
                        || (primary_aliases
                            .iter()
                            .any(|alias| parameter.eq_ignore_ascii_case(alias))
                            && matches!(
                                element.kind,
                                ElementKind::Resistor { value, .. }
                                    | ElementKind::Capacitor { value, .. }
                                    | ElementKind::Inductor { value, .. }
                                    if value.is_finite()
                            ))
                    {
                        continue;
                    }
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_{}", parameter.to_ascii_uppercase()),
                            element: name.clone(),
                            element_type,
                            parameter: parameter.to_ascii_uppercase(),
                            nominal_value: *nominal_value,
                            location: AcSensitivityLocation::ElementParameter {
                                element_index,
                                parameter_index,
                            },
                        },
                    );
                }
            }

            if let ElementKind::Xspice {
                params,
                expr_params,
                real_vector_params,
                real_vector_expr_params,
                ..
            } = &element.kind
            {
                let resolved_scalars = Self::resolved_expression_params(
                    &netlist.params,
                    params,
                    expr_params,
                    &format!("XSPICE instance '{name}'"),
                )?;
                for (parameter, nominal_value) in &resolved_scalars {
                    if !Self::is_continuous_sensitivity_parameter(parameter, parameter_specs, false)
                    {
                        continue;
                    }
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_{}", parameter.to_ascii_uppercase()),
                            element: name.clone(),
                            element_type,
                            parameter: parameter.to_ascii_uppercase(),
                            nominal_value: *nominal_value,
                            location: AcSensitivityLocation::ElementNamedParameter {
                                element_index,
                                parameter: parameter.clone(),
                            },
                        },
                    );
                }
                for (parameter_index, (parameter, values)) in real_vector_params.iter().enumerate()
                {
                    if !Self::is_continuous_sensitivity_parameter(parameter, parameter_specs, true)
                    {
                        continue;
                    }
                    for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                        Self::add_ac_sensitivity_target(
                            &mut targets,
                            &mut seen,
                            AcSensitivityTarget {
                                vector_name: format!(
                                    "{name}_{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                element: name.clone(),
                                element_type,
                                parameter: format!(
                                    "{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                nominal_value,
                                location: AcSensitivityLocation::ElementVectorParameter {
                                    element_index,
                                    parameter_index,
                                    entry_index,
                                },
                            },
                        );
                    }
                }
                for (parameter, values) in Self::resolved_vector_expression_params(
                    &netlist.params,
                    &resolved_scalars,
                    real_vector_expr_params,
                    &format!("XSPICE instance '{name}'"),
                )? {
                    if !Self::is_continuous_sensitivity_parameter(&parameter, parameter_specs, true)
                    {
                        continue;
                    }
                    for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                        Self::add_ac_sensitivity_target(
                            &mut targets,
                            &mut seen,
                            AcSensitivityTarget {
                                vector_name: format!(
                                    "{name}_{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                element: name.clone(),
                                element_type,
                                parameter: format!(
                                    "{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                nominal_value,
                                location: AcSensitivityLocation::ElementNamedVectorParameter {
                                    element_index,
                                    parameter: parameter.clone(),
                                    entry_index,
                                    resolved_values: values.clone(),
                                },
                            },
                        );
                    }
                }
            }
        }

        for (model_index, model) in netlist.models.iter().enumerate() {
            if !referenced_models.contains(&model.name.to_ascii_uppercase()) {
                continue;
            }
            let parameter_specs = xspice_models
                .get(&model.name.to_ascii_uppercase())
                .map(|code_model| code_model.parameters());
            for (parameter, nominal_value) in Self::resolved_model_scalar_params(netlist, model)? {
                if !Self::is_continuous_sensitivity_parameter(&parameter, parameter_specs, false) {
                    continue;
                }
                Self::add_ac_sensitivity_target(
                    &mut targets,
                    &mut seen,
                    AcSensitivityTarget {
                        vector_name: format!("{}:{}", model.name, parameter.to_ascii_uppercase()),
                        element: model.name.clone(),
                        element_type: ElementType::Model,
                        parameter: parameter.to_ascii_uppercase(),
                        nominal_value,
                        location: AcSensitivityLocation::ModelParameter {
                            model_index,
                            parameter,
                        },
                    },
                );
            }
            for (parameter_index, (parameter, values)) in
                model.real_vector_params.iter().enumerate()
            {
                if !Self::is_continuous_sensitivity_parameter(parameter, parameter_specs, true) {
                    continue;
                }
                for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!(
                                "{}:{}[{entry_index}]",
                                model.name,
                                parameter.to_ascii_uppercase()
                            ),
                            element: model.name.clone(),
                            element_type: ElementType::Model,
                            parameter: format!("{}[{entry_index}]", parameter.to_ascii_uppercase()),
                            nominal_value,
                            location: AcSensitivityLocation::ModelVectorParameter {
                                model_index,
                                parameter_index,
                                entry_index,
                            },
                        },
                    );
                }
            }
            let resolved_scalars = Self::resolved_model_scalar_params(netlist, model)?;
            for (parameter, values) in Self::resolved_vector_expression_params(
                &netlist.params,
                &resolved_scalars,
                &model.real_vector_expr_params,
                &format!("model '{}'", model.name),
            )? {
                if !Self::is_continuous_sensitivity_parameter(&parameter, parameter_specs, true) {
                    continue;
                }
                for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!(
                                "{}:{}[{entry_index}]",
                                model.name,
                                parameter.to_ascii_uppercase()
                            ),
                            element: model.name.clone(),
                            element_type: ElementType::Model,
                            parameter: format!("{}[{entry_index}]", parameter.to_ascii_uppercase()),
                            nominal_value,
                            location: AcSensitivityLocation::ModelNamedVectorParameter {
                                model_index,
                                parameter: parameter.clone(),
                                entry_index,
                                resolved_values: values.clone(),
                            },
                        },
                    );
                }
            }
        }

        targets.sort_by(|left, right| {
            left.vector_name
                .to_ascii_uppercase()
                .cmp(&right.vector_name.to_ascii_uppercase())
        });
        Ok(targets)
    }

    fn update_primary_instance_aliases(
        parameters: &mut [(String, Value)],
        aliases: &[&str],
        value: Value,
    ) {
        for (name, existing) in parameters {
            if aliases.iter().any(|alias| name.eq_ignore_ascii_case(alias)) {
                *existing = value;
            }
        }
    }

    fn set_source_dc_for_sensitivity(spec: &mut SourceSpec, value: Value) {
        let owned = std::mem::replace(spec, SourceSpec::Dc(0.0));
        *spec = owned.with_dc_value(value);
    }

    fn set_source_ac_for_sensitivity(
        spec: &mut SourceSpec,
        magnitude: Value,
        phase_radians: Value,
    ) {
        let owned = std::mem::replace(spec, SourceSpec::Dc(0.0));
        *spec = owned.with_ac(magnitude, phase_radians);
    }

    fn apply_ac_sensitivity_target(
        netlist: &mut Netlist,
        target: &AcSensitivityTarget,
        value: Value,
    ) -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity perturbation for '{}' is non-finite: {value}",
                target.vector_name
            )));
        }
        match &target.location {
            AcSensitivityLocation::ElementField {
                element_index,
                field,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                match (field, &mut element.kind) {
                    (
                        AcSensitivityElementField::ResistorValue,
                        ElementKind::Resistor {
                            value: nominal,
                            value_expr,
                            instance_params,
                            ..
                        },
                    ) => {
                        *nominal = value;
                        *value_expr = None;
                        Self::update_primary_instance_aliases(
                            instance_params,
                            &["R", "RES", "RESISTANCE", "VALUE"],
                            value,
                        );
                    }
                    (
                        AcSensitivityElementField::CapacitorValue,
                        ElementKind::Capacitor {
                            value: nominal,
                            value_expr,
                            instance_params,
                            ..
                        },
                    ) => {
                        *nominal = value;
                        *value_expr = None;
                        Self::update_primary_instance_aliases(
                            instance_params,
                            &["C", "CAP", "CAPACITANCE", "VALUE"],
                            value,
                        );
                    }
                    (
                        AcSensitivityElementField::InductorValue,
                        ElementKind::Inductor {
                            value: nominal,
                            value_expr,
                            instance_params,
                            ..
                        },
                    ) => {
                        *nominal = value;
                        *value_expr = None;
                        Self::update_primary_instance_aliases(
                            instance_params,
                            &["L", "IND", "INDUCTANCE", "VALUE"],
                            value,
                        );
                    }
                    (
                        AcSensitivityElementField::JilesAthertonValue,
                        ElementKind::JilesAthertonInductor { value: nominal, .. },
                    ) => *nominal = value,
                    (
                        AcSensitivityElementField::VcvsGain,
                        ElementKind::Vcvs {
                            gain, gain_expr, ..
                        },
                    )
                    | (
                        AcSensitivityElementField::CccsGain,
                        ElementKind::Cccs {
                            gain, gain_expr, ..
                        },
                    ) => {
                        *gain = value;
                        *gain_expr = None;
                    }
                    (
                        AcSensitivityElementField::VccsTransconductance,
                        ElementKind::Vccs {
                            transconductance,
                            transconductance_expr,
                            ..
                        },
                    ) => {
                        *transconductance = value;
                        *transconductance_expr = None;
                    }
                    (
                        AcSensitivityElementField::CcvsTransresistance,
                        ElementKind::Ccvs {
                            transresistance,
                            transresistance_expr,
                            ..
                        },
                    ) => {
                        *transresistance = value;
                        *transresistance_expr = None;
                    }
                    (
                        AcSensitivityElementField::SourceMultiplicity,
                        ElementKind::Vccs { multiplicity, .. }
                        | ElementKind::BehavioralVoltage { multiplicity, .. }
                        | ElementKind::BehavioralCurrent { multiplicity, .. },
                    ) => {
                        multiplicity.value = value;
                        multiplicity.value_expr = None;
                        multiplicity.given = true;
                    }
                    (
                        AcSensitivityElementField::BehavioralTc1,
                        ElementKind::BehavioralVoltage { tc1, .. }
                        | ElementKind::BehavioralCurrent { tc1, .. },
                    ) => *tc1 = value,
                    (
                        AcSensitivityElementField::BehavioralTc2,
                        ElementKind::BehavioralVoltage { tc2, .. }
                        | ElementKind::BehavioralCurrent { tc2, .. },
                    ) => *tc2 = value,
                    (
                        AcSensitivityElementField::TransmissionZ0,
                        ElementKind::TransmissionLine { z0, .. },
                    ) => *z0 = Some(value),
                    (
                        AcSensitivityElementField::TransmissionDelay,
                        ElementKind::TransmissionLine { td, .. },
                    ) => *td = Some(value),
                    (
                        AcSensitivityElementField::TransmissionFrequency,
                        ElementKind::TransmissionLine { freq, .. },
                    ) => *freq = Some(value),
                    (
                        AcSensitivityElementField::TransmissionLength,
                        ElementKind::TransmissionLine { nl, .. },
                    ) => *nl = Some(value),
                    (
                        AcSensitivityElementField::Coupling,
                        ElementKind::Coupling { coefficient, .. },
                    ) => *coefficient = value,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "Sensitivity target '{}' no longer matches its element kind",
                            target.vector_name
                        )));
                    }
                }
            }
            AcSensitivityLocation::ElementParameter {
                element_index,
                parameter_index,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let parameters = match &mut element.kind {
                    ElementKind::Resistor {
                        instance_params, ..
                    }
                    | ElementKind::Capacitor {
                        instance_params, ..
                    }
                    | ElementKind::Inductor {
                        instance_params, ..
                    }
                    | ElementKind::Diode {
                        instance_params, ..
                    }
                    | ElementKind::Bjt {
                        instance_params, ..
                    }
                    | ElementKind::Mosfet {
                        instance_params, ..
                    }
                    | ElementKind::Jfet {
                        instance_params, ..
                    }
                    | ElementKind::Mesfet {
                        instance_params, ..
                    } => instance_params,
                    ElementKind::Xspice { params, .. } => params,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "Sensitivity target '{}' has no scalar instance parameters",
                            target.vector_name
                        )));
                    }
                };
                let (_, nominal) = parameters.get_mut(*parameter_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing parameter index {}",
                        target.vector_name, parameter_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::ElementNamedParameter {
                element_index,
                parameter,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let ElementKind::Xspice {
                    params,
                    expr_params,
                    ..
                } = &mut element.kind
                else {
                    return Err(SimulationError::Circuit(format!(
                        "Sensitivity target '{}' is not a named XSPICE parameter",
                        target.vector_name
                    )));
                };
                expr_params.retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                if let Some((_, nominal)) = params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    *nominal = value;
                } else {
                    params.push((parameter.to_ascii_uppercase(), value));
                }
            }
            AcSensitivityLocation::ElementVectorParameter {
                element_index,
                parameter_index,
                entry_index,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let ElementKind::Xspice {
                    real_vector_params, ..
                } = &mut element.kind
                else {
                    return Err(SimulationError::Circuit(format!(
                        "Sensitivity target '{}' is not an XSPICE vector parameter",
                        target.vector_name
                    )));
                };
                let (_, values) = real_vector_params.get_mut(*parameter_index).ok_or_else(
                    || {
                        SimulationError::Circuit(format!(
                            "Sensitivity target '{}' references missing vector parameter index {}",
                            target.vector_name, parameter_index
                        ))
                    },
                )?;
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::ElementNamedVectorParameter {
                element_index,
                parameter,
                entry_index,
                resolved_values,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let ElementKind::Xspice {
                    real_vector_params,
                    real_vector_expr_params,
                    ..
                } = &mut element.kind
                else {
                    return Err(SimulationError::Circuit(format!(
                        "Sensitivity target '{}' is not a named XSPICE vector parameter",
                        target.vector_name
                    )));
                };
                real_vector_expr_params.retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                let values = if let Some((_, values)) = real_vector_params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    values
                } else {
                    real_vector_params
                        .push((parameter.to_ascii_uppercase(), resolved_values.clone()));
                    &mut real_vector_params
                        .last_mut()
                        .expect("just inserted vector parameter")
                        .1
                };
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::SourceDc { element_index }
            | AcSensitivityLocation::SourceAcMagnitude { element_index }
            | AcSensitivityLocation::SourceAcPhaseDegrees { element_index } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing source index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let spec = match &mut element.kind {
                    ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => spec,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "Sensitivity target '{}' is not an independent source",
                            target.vector_name
                        )));
                    }
                };
                match target.location {
                    AcSensitivityLocation::SourceDc { .. } => {
                        Self::set_source_dc_for_sensitivity(spec, value);
                    }
                    AcSensitivityLocation::SourceAcMagnitude { .. } => {
                        let (_, phase) = crate::engine::extract_ac_value(spec);
                        Self::set_source_ac_for_sensitivity(spec, value, phase);
                    }
                    AcSensitivityLocation::SourceAcPhaseDegrees { .. } => {
                        let (magnitude, _) = crate::engine::extract_ac_value(spec);
                        Self::set_source_ac_for_sensitivity(spec, magnitude, value.to_radians());
                    }
                    _ => unreachable!("matched source sensitivity location"),
                }
            }
            AcSensitivityLocation::ModelParameter {
                model_index,
                parameter,
            } => {
                let model = netlist.models.get_mut(*model_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model index {}",
                        target.vector_name, model_index
                    ))
                })?;
                model
                    .expr_params
                    .retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                if let Some((_, nominal)) = model
                    .params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    *nominal = value;
                } else {
                    model.params.push((parameter.to_ascii_uppercase(), value));
                }
            }
            AcSensitivityLocation::ModelVectorParameter {
                model_index,
                parameter_index,
                entry_index,
            } => {
                let model = netlist.models.get_mut(*model_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model index {}",
                        target.vector_name, model_index
                    ))
                })?;
                let (_, values) = model
                    .real_vector_params
                    .get_mut(*parameter_index)
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Sensitivity target '{}' references missing model vector index {}",
                            target.vector_name, parameter_index
                        ))
                    })?;
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::ModelNamedVectorParameter {
                model_index,
                parameter,
                entry_index,
                resolved_values,
            } => {
                let model = netlist.models.get_mut(*model_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model index {}",
                        target.vector_name, model_index
                    ))
                })?;
                model
                    .real_vector_expr_params
                    .retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                let values = if let Some((_, values)) = model
                    .real_vector_params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    values
                } else {
                    model
                        .real_vector_params
                        .push((parameter.to_ascii_uppercase(), resolved_values.clone()));
                    &mut model
                        .real_vector_params
                        .last_mut()
                        .expect("just inserted model vector parameter")
                        .1
                };
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
        }
        Ok(())
    }

    fn sensitivity_glob_matches(pattern: &str, candidate: &str) -> bool {
        let pattern = pattern.to_ascii_uppercase();
        let candidate = candidate.to_ascii_uppercase();
        let pattern = pattern.as_bytes();
        let candidate = candidate.as_bytes();
        let (mut p, mut c) = (0usize, 0usize);
        let mut star = None;
        while c < candidate.len() {
            if p < pattern.len() && (pattern[p] == b'?' || pattern[p] == candidate[c]) {
                p += 1;
                c += 1;
            } else if p < pattern.len() && pattern[p] == b'*' {
                star = Some((p, c));
                p += 1;
            } else if let Some((star_p, star_c)) = star {
                star = Some((star_p, star_c + 1));
                p = star_p + 1;
                c = star_c + 1;
            } else {
                return false;
            }
        }
        while p < pattern.len() && pattern[p] == b'*' {
            p += 1;
        }
        p == pattern.len()
    }

    fn sensitivity_target_selected(target: &AcSensitivityTarget, filters: &[String]) -> bool {
        filters.is_empty()
            || filters.iter().any(|filter| {
                Self::sensitivity_glob_matches(filter, &target.vector_name)
                    || Self::sensitivity_glob_matches(filter, &target.element)
                    || Self::sensitivity_glob_matches(
                        filter,
                        &format!("{}:{}", target.element, target.parameter),
                    )
            })
    }

    fn complete_sensitivity_step(target: &AcSensitivityTarget) -> Value {
        let parameter = target.parameter.to_ascii_uppercase();
        let absolute_floor = if parameter.contains("PHASE") {
            1.0e-3
        } else if matches!(
            parameter.as_str(),
            "C" | "CAP" | "CAPACITANCE" | "CJ" | "CJO" | "CGSO" | "CGDO" | "CGBO"
        ) || parameter.starts_with('C')
            && (parameter.contains('J') || parameter.contains("CAP"))
        {
            1.0e-18
        } else if matches!(parameter.as_str(), "L" | "IND" | "INDUCTANCE") {
            1.0e-15
        } else if matches!(parameter.as_str(), "DC" | "AC_MAG") {
            1.0e-9
        } else {
            1.0e-12
        };
        Self::relative_sensitivity_step(target.nominal_value, absolute_floor)
    }

    fn ac_sensitivity_output_value(
        result: &crate::analysis::AcResult,
        output: &AcSensitivityOutput,
    ) -> Result<Complex64, SimulationError> {
        let voltage = |node: usize| -> Result<Complex64, SimulationError> {
            if node == 0 {
                return Ok(Complex64::new(0.0, 0.0));
            }
            result.voltages.get(node - 1).copied().ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "Sensitivity output node {node} is outside circuit node range 0..={}",
                    result.voltages.len()
                ))
            })
        };
        match output {
            AcSensitivityOutput::Voltage { positive, negative } => {
                Ok(voltage(*positive)? - voltage(negative.unwrap_or(0))?)
            }
            AcSensitivityOutput::BranchCurrent(element) => result
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(element))
                .and_then(|index| result.currents.get(index).copied())
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity branch-current output I({element}) is unavailable; the element must own an AC MNA branch"
                    ))
                }),
        }
    }

    fn ac_sensitivity_outputs(
        results: &[crate::analysis::AcResult],
        output: &AcSensitivityOutput,
        expected_frequencies: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, SimulationError> {
        if results.len() != expected_frequencies.len() {
            return Err(SimulationError::Circuit(format!(
                "AC sensitivity produced {} samples for a {}-point frequency grid",
                results.len(),
                expected_frequencies.len()
            )));
        }
        results
            .iter()
            .zip(expected_frequencies)
            .map(|(result, expected)| {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let tolerance = expected.abs().max(1.0) * 1.0e-12;
                if (result.frequency - expected).abs() > tolerance {
                    return Err(SimulationError::Circuit(format!(
                        "AC sensitivity frequency mismatch: expected {expected}, got {}",
                        result.frequency
                    )));
                }
                Self::ac_sensitivity_output_value(result, output)
            })
            .collect()
    }

    fn complete_ac_sensitivity_trace(
        target: &AcSensitivityTarget,
        nominal_output: &[Complex64],
        derivative: Vec<Complex64>,
        abort: &dyn AbortSignal,
    ) -> Result<AcSensitivity, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if nominal_output.len() != derivative.len() || !target.nominal_value.is_finite() {
            return Err(SimulationError::Circuit(
                "AC sensitivity trace shape or parameter is invalid".to_owned(),
            ));
        }
        let mut normalized = Vec::with_capacity(derivative.len());
        let mut magnitude = Vec::with_capacity(derivative.len());
        let mut phase = Vec::with_capacity(derivative.len());
        for (&output, &sensitivity) in nominal_output.iter().zip(&derivative) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if [output.re, output.im, sensitivity.re, sensitivity.im]
                .iter()
                .any(|value| !value.is_finite())
            {
                return Err(SimulationError::Circuit(format!(
                    "AC sensitivity '{}' contains a non-finite output or derivative",
                    target.vector_name
                )));
            }
            magnitude.push(SensitivityValue::magnitude(output, sensitivity));
            let scale = output.re.abs().max(output.im.abs());
            if scale != 0.0 {
                let one = ScaledValue::new(1.0);
                let re = ScaledValue::new(output.re);
                let im = ScaledValue::new(output.im);
                let dr = ScaledValue::new(sensitivity.re);
                let di = ScaledValue::new(sensitivity.im);
                let parameter = ScaledValue::new(target.nominal_value);
                let norm_squared = [[re, re, one], [im, im, one]];
                normalized.push(
                    derived_sensitivity_ratio(
                        [[re, dr, parameter], [im, di, parameter]].into_iter(),
                        norm_squared.into_iter(),
                    )?
                    .zip(derived_sensitivity_ratio(
                        [[re, di, parameter], [im.negated(), dr, parameter]].into_iter(),
                        norm_squared.into_iter(),
                    )?)
                    .map(|(re, im)| Complex64::new(re, im)),
                );
                phase.push(derived_sensitivity_ratio(
                    [[re, di, one], [im.negated(), dr, one]].into_iter(),
                    norm_squared.into_iter(),
                )?);
            } else {
                normalized.push(SensitivityValue::unavailable(
                    SensitivityUnavailability::ZeroOutput,
                ));
                phase.push(SensitivityValue::unavailable(
                    SensitivityUnavailability::ZeroOutput,
                ));
            }
        }
        Ok(AcSensitivity {
            vector_name: target.vector_name.clone(),
            element: target.element.clone(),
            element_type: target.element_type,
            parameter: target.parameter.clone(),
            nominal_value: target.nominal_value,
            absolute: derivative,
            normalized,
            magnitude,
            phase,
        })
    }

    fn dc_sensitivity_output_value(
        result: &SimulationResult,
        output: &AcSensitivityOutput,
    ) -> Result<Value, SimulationError> {
        match output {
            AcSensitivityOutput::Voltage { positive, negative } => {
                let positive_value = result.try_voltage(*positive).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity output node {positive} is outside circuit node range 0..={}",
                        result.node_voltages.len().saturating_sub(1)
                    ))
                })?;
                let negative_value = negative
                    .map(|node| {
                        result.try_voltage(node).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "Sensitivity reference node {node} is outside circuit node range 0..={}",
                                result.node_voltages.len().saturating_sub(1)
                            ))
                        })
                    })
                    .transpose()?
                    .unwrap_or(0.0);
                Ok(positive_value - negative_value)
            }
            AcSensitivityOutput::BranchCurrent(element) => {
                let matches = result
                    .branch_names
                    .iter()
                    .enumerate()
                    .filter(|(_, name)| name.eq_ignore_ascii_case(element))
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();
                match matches.as_slice() {
                    [] => Err(SimulationError::Circuit(format!(
                        "DC sensitivity branch-current output '{element}' was not found"
                    ))),
                    [index] => result.branch_currents.get(*index).copied().ok_or_else(|| {
                        SimulationError::Circuit(
                            "DC sensitivity branch-current result is malformed".to_string(),
                        )
                    }),
                    _ => Err(SimulationError::Circuit(format!(
                        "DC sensitivity branch-current output '{element}' is ambiguous"
                    ))),
                }
            }
        }
    }

    fn dc_sensitivity_target_active(target: &AcSensitivityTarget) -> bool {
        !matches!(
            target.location,
            AcSensitivityLocation::SourceAcMagnitude { .. }
                | AcSensitivityLocation::SourceAcPhaseDegrees { .. }
        )
    }

    fn validate_complete_dc_sensitivity_coverage(netlist: &Netlist) -> Result<(), SimulationError> {
        let unsupported = netlist
            .elements
            .iter()
            .filter_map(|element| {
                matches!(&element.kind, ElementKind::XyceMemristor { .. })
                    .then_some(element.name.as_str())
            })
            .collect::<Vec<_>>();
        if unsupported.is_empty() {
            return Ok(());
        }

        Err(SimulationError::Circuit(format!(
            "Complete DC sensitivity does not yet support native Xyce memristor instance/model parameters (device{}: {}). The analysis was rejected to avoid returning an incomplete sensitivity result.",
            if unsupported.len() == 1 { "" } else { "s" },
            unsupported.join(", ")
        )))
    }

    /// Run complete netlist-wide DC sensitivity for every eligible real
    /// parameter in the flattened circuit. Unlike the legacy adjoint helper,
    /// this covers nonlinear devices, models, hierarchy, branch-current
    /// outputs, and SPICE device filters.
    pub fn run_sensitivity_dc_complete(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        filters: &[String],
    ) -> Result<SensitivityResult, SimulationError> {
        self.run_sensitivity_dc_complete_with_abort(netlist, output, filters, &NoAbort)
    }

    /// Complete DC sensitivity with cooperative cancellation.
    pub fn run_sensitivity_dc_complete_with_abort(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        filters: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<SensitivityResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let AcSensitivityOutput::Voltage { positive: 0, .. } = output {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }
        if let AcSensitivityOutput::Voltage {
            positive,
            negative: Some(negative),
        } = output
            && positive == negative
        {
            return Err(SimulationError::Circuit(
                "Sensitivity output and reference nodes must differ".to_string(),
            ));
        }

        let flat = self.flattened_sensitivity_netlist(netlist, abort)?;
        Self::validate_complete_dc_sensitivity_coverage(&flat)?;
        let targets = Self::collect_ac_sensitivity_targets(&flat, self.config.resource_limits)?
            .into_iter()
            .filter(Self::dc_sensitivity_target_active)
            .filter(|target| Self::sensitivity_target_selected(target, filters))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            let detail = if filters.is_empty() {
                "the flattened circuit has no eligible real-valued DC parameters".to_string()
            } else {
                format!("no DC parameter matched filter(s) {}", filters.join(", "))
            };
            return Err(SimulationError::Circuit(format!(
                "DC sensitivity cannot run: {detail}"
            )));
        }
        self.ensure_batch_runs(1)?;
        self.ensure_result_values(targets.len().saturating_mul(3).saturating_add(1))?;

        let nominal_result = self.run_dc_op_with_abort(&flat, abort)?;
        let nominal_output = Self::dc_sensitivity_output_value(&nominal_result, &output)?;

        let output_name = match &output {
            AcSensitivityOutput::Voltage { positive, negative } => negative.map_or_else(
                || format!("V({positive})"),
                |negative| format!("V({positive},{negative})"),
            ),
            AcSensitivityOutput::BranchCurrent(element) => format!("I({element})"),
        };
        let mut result = SensitivityResult::new(&output_name, nominal_output);
        result.sensitivities.reserve(targets.len());

        let mut runs = 1;
        for target in targets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let h = Self::complete_sensitivity_step(&target);
            let derivative = self.refine_sensitivity(
                &target.vector_name,
                target.nominal_value,
                h,
                &[Complex64::new(nominal_output, 0.0)],
                &mut runs,
                abort,
                |candidate| {
                    let mut perturbed = flat.clone();
                    Self::apply_ac_sensitivity_target(&mut perturbed, &target, candidate)?;
                    let result = self.run_dc_op_with_abort(&perturbed, abort)?;
                    Ok(vec![Complex64::new(
                        Self::dc_sensitivity_output_value(&result, &output)?,
                        0.0,
                    )])
                },
            )?[0]
                .re;
            if !derivative.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "DC sensitivity '{}' produced a non-finite derivative",
                    target.vector_name
                )));
            }
            result.add(Sensitivity::new_named(
                &target.vector_name,
                &target.element,
                target.element_type,
                &target.parameter,
                target.nominal_value,
                derivative,
                nominal_output,
            ));
        }

        Ok(result)
    }

    /// Run complete AC sensitivity for every eligible real-valued parameter
    /// in the flattened netlist. The returned derivatives are complex and
    /// unnormalized, matching SPICE `.SENS AC` semantics; normalized,
    /// magnitude, and phase derivatives are retained alongside them.
    pub fn run_sensitivity_ac_complete(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        frequencies: &[Value],
        filters: &[String],
    ) -> Result<AcSensitivityResult, SimulationError> {
        self.run_sensitivity_ac_complete_with_abort(netlist, output, frequencies, filters, &NoAbort)
    }

    /// Complete AC sensitivity with cooperative cancellation.
    pub fn run_sensitivity_ac_complete_with_abort(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        frequencies: &[Value],
        filters: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<AcSensitivityResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if frequencies.is_empty()
            || frequencies
                .iter()
                .any(|frequency| !frequency.is_finite() || *frequency < 0.0)
        {
            return Err(SimulationError::Circuit(
                "AC sensitivity frequencies must be a non-empty list of finite, non-negative values"
                    .to_string(),
            ));
        }
        self.ensure_analysis_points(frequencies.len())?;
        if let AcSensitivityOutput::Voltage { positive: 0, .. } = output {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }

        let flat = self.flattened_sensitivity_netlist(netlist, abort)?;
        let targets = Self::collect_ac_sensitivity_targets(&flat, self.config.resource_limits)?
            .into_iter()
            .filter(|target| Self::sensitivity_target_selected(target, filters))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            let detail = if filters.is_empty() {
                "the flattened circuit has no eligible real-valued parameters".to_string()
            } else {
                format!("no parameter matched filter(s) {}", filters.join(", "))
            };
            return Err(SimulationError::Circuit(format!(
                "AC sensitivity cannot run: {detail}"
            )));
        }
        self.ensure_batch_runs(1)?;
        // Count numerical slots, including unavailable derived samples and
        // each retained nominal parameter; a complex sample occupies two.
        self.ensure_result_values(
            frequencies
                .len()
                .saturating_mul(targets.len().saturating_mul(6).saturating_add(3))
                .saturating_add(targets.len()),
        )?;

        let nominal_results = self.run_ac_with_abort(&flat, frequencies, abort)?;
        let nominal_output =
            Self::ac_sensitivity_outputs(&nominal_results, &output, frequencies, abort)?;

        let output_name = match &output {
            AcSensitivityOutput::Voltage { positive, negative } => negative.map_or_else(
                || format!("V({positive})"),
                |negative| format!("V({positive},{negative})"),
            ),
            AcSensitivityOutput::BranchCurrent(element) => format!("I({element})"),
        };
        let mut sensitivities = Vec::with_capacity(targets.len());
        let mut runs = 1;
        for target in targets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let h = Self::complete_sensitivity_step(&target);
            let derivative = self.refine_sensitivity(
                &target.vector_name,
                target.nominal_value,
                h,
                &nominal_output,
                &mut runs,
                abort,
                |candidate| {
                    let mut perturbed = flat.clone();
                    Self::apply_ac_sensitivity_target(&mut perturbed, &target, candidate)?;
                    let result = self.run_ac_with_abort(&perturbed, frequencies, abort)?;
                    Self::ac_sensitivity_outputs(&result, &output, frequencies, abort)
                },
            )?;
            sensitivities.push(Self::complete_ac_sensitivity_trace(
                &target,
                &nominal_output,
                derivative,
                abort,
            )?);
        }

        Ok(AcSensitivityResult {
            output: output_name,
            frequencies: frequencies.to_vec(),
            output_values: nominal_output,
            sensitivities,
        })
    }

    /// Run one authored `.SENS` card.
    ///
    /// The card names its output probe the way the deck does and may request
    /// an AC sweep. This resolves the probe against the elaborated circuit,
    /// builds the sweep grid from the card's own frequency specification, and
    /// selects the DC or AC driver accordingly. Which of the two a `.SENS AC`
    /// card selects is analysis semantics, so it is decided here rather than
    /// on each frontend.
    pub fn run_sensitivity_from_card_with_abort(
        &self,
        netlist: &Netlist,
        card: &crate::netlist::AnalysisCommand,
        abort: &dyn AbortSignal,
    ) -> Result<SensitivityCardResult, SimulationError> {
        use crate::netlist::AnalysisCommand;

        let AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } = card
        else {
            return Err(SimulationError::Netlist(
                "run_sensitivity_from_card_with_abort was given a card that is not .SENS"
                    .to_owned(),
            ));
        };
        let output = if *output_is_current {
            AcSensitivityOutput::BranchCurrent(output_node.clone())
        } else {
            let resolver = super::NodeResolver::build_with_abort(self, netlist, abort)?;
            AcSensitivityOutput::Voltage {
                positive: resolver.resolve(output_node, ".SENS output")?,
                negative: resolver
                    .resolve_reference(reference_node.as_deref(), ".SENS reference")?,
            }
        };
        match ac_sweep {
            None => self
                .run_sensitivity_dc_complete_with_abort(netlist, output, filters, abort)
                .map(SensitivityCardResult::Dc),
            Some(sweep) => {
                let frequencies = super::sp::card_frequency_grid(
                    sweep.variation,
                    sweep.points,
                    sweep.start_freq,
                    sweep.stop_freq,
                    abort,
                )?;
                self.run_sensitivity_ac_complete_with_abort(
                    netlist,
                    output,
                    &frequencies,
                    filters,
                    abort,
                )
                .map(SensitivityCardResult::Ac)
            }
        }
    }
}

/// What one authored `.SENS` card produced.
///
/// A bare `.SENS` is a DC study of the converged operating point; `.SENS ... AC`
/// is a frequency-domain study whose derivatives are complex. They are
/// different result shapes, not two encodings of one, so the card runner names
/// which it produced instead of flattening either into the other.
#[derive(Debug, Clone)]
pub enum SensitivityCardResult {
    /// A `.SENS` card with no `AC` clause.
    Dc(SensitivityResult),
    /// A `.SENS ... AC` card.
    Ac(AcSensitivityResult),
}

#[cfg(test)]
mod tests {
    use super::super::super::Engine;
    use super::sensitivity_three_point;

    #[test]
    fn sensitivity_parameter_metadata_overrides_name_heuristics() {
        use crate::xspice::ParamSpec;
        let specs = [
            ParamSpec::real("mode", 1.0),
            ParamSpec::boolean("fraction", true),
            ParamSpec::integer("span", 3),
            ParamSpec::integer_vector("indices", vec![1]),
            ParamSpec::real_vector("values", vec![1.0]),
        ];
        assert!(Engine::is_continuous_sensitivity_parameter(
            "MODE",
            Some(&specs),
            false
        ));
        for name in ["fraction", "span", "indices"] {
            for vector in [false, true] {
                assert!(!Engine::is_continuous_sensitivity_parameter(
                    name,
                    Some(&specs),
                    vector
                ));
            }
        }
        assert!(Engine::is_continuous_sensitivity_parameter(
            "values",
            Some(&specs),
            true
        ));
        assert!(!Engine::is_continuous_sensitivity_parameter(
            "values",
            Some(&specs),
            false
        ));
    }

    #[test]
    fn sensitivity_xspice_target_collection_excludes_discrete_and_nonnumeric_channels() {
        for alias in [false, true] {
            let model = if alias { "demo" } else { "print_param_types" };
            let card = if alias {
                ".model demo print_param_types(integer=3 real=2 string=123 complex=4 integer_array=[1 2] real_array=[1 2])\n"
            } else {
                ""
            };
            let netlist = Netlist::parse(&format!(
                "Parameter types\nV1 in 0 1\nA1 [in] {model} integer=3 real=2 string=123 complex=4 integer_array=[1 2] real_array=[1 2]\n{card}.end\n"
            )).unwrap();
            let targets = Engine::collect_ac_sensitivity_targets(
                &netlist,
                crate::resource::ResourceLimits::default(),
            )
            .unwrap();
            let mut names = targets
                .iter()
                .map(|target| target.vector_name.to_ascii_uppercase())
                .collect::<Vec<_>>();
            names.sort();
            let mut expected = vec!["A1_REAL", "A1_REAL_ARRAY[0]", "A1_REAL_ARRAY[1]", "V1"];
            if alias {
                expected.extend(["DEMO:REAL", "DEMO:REAL_ARRAY[0]", "DEMO:REAL_ARRAY[1]"]);
            }
            expected.sort();
            assert_eq!(names, expected);
        }
    }
    use crate::analysis::AcSensitivityOutput;
    use crate::netlist::AnalysisCommand;
    use crate::netlist::{StepCommand, StepSweep, StepTarget};
    use crate::{Complex64, Netlist};

    #[test]
    fn parameter_replay_preserves_the_runtime_statistical_seed() {
        let source = "seeded sensitivity\n.options seed=7\n.param p=1k draw={aunif(0,1)}\nV1 in 0 1\nR1 in out {p}\nR2 out 0 {1k+draw}\n.end\n";
        let original = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                statistical_seed: Some(u64::MAX),
                ..Default::default()
            },
        )
        .unwrap();
        let (replayed, references) = Engine::create_perturbed_netlist_multi_with_abort(
            &original,
            &[("p".to_owned(), 2000.0)],
            &crate::abort_signal::NoAbort,
        )
        .unwrap();
        assert_eq!(references, 1);
        assert_eq!(replayed.params.get("p"), Some(2000.0));
        assert_eq!(replayed.params.get("draw"), original.params.get("draw"));
        assert_eq!(replayed.params.random().seed(), u64::MAX);
        assert_eq!(replayed.options.seed, Some(u64::MAX));
        assert!(original.source_text.as_deref().unwrap().contains("seed=7"));
    }

    #[test]
    fn complete_sensitivity_preserves_cancellation_during_perturbations() {
        use crate::abort_signal::CountingAbort;
        let netlist = Netlist::parse(
            "sensitivity cancellation\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
        )
        .unwrap();
        let engine = Engine::default();
        for ac in [false, true] {
            let run = |abort: &CountingAbort| {
                let output = AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                };
                if ac {
                    engine
                        .run_sensitivity_ac_complete_with_abort(
                            &netlist,
                            output,
                            &[1e3],
                            &[],
                            abort,
                        )
                        .map(|_| ())
                } else {
                    engine
                        .run_sensitivity_dc_complete_with_abort(&netlist, output, &[], abort)
                        .map(|_| ())
                }
            };
            let baseline = CountingAbort::new(usize::MAX);
            run(&baseline).unwrap();
            for slice in 1..=16 {
                let abort = CountingAbort::new((baseline.count() - 1) * slice / 16);
                let error = run(&abort).expect_err("analysis must stop after cancellation");
                assert!(
                    matches!(error, crate::SimulationError::Aborted),
                    "AC={ac}, slice={slice}: {error}"
                );
            }
        }
    }

    #[test]
    fn default_ac_sensitivity_resolves_femtofarad_capacitances() {
        let capacitance = 100e-15;
        let resistance = 1e3;
        let frequency = 1.0 / (std::f64::consts::TAU * resistance * capacitance);
        let netlist = Netlist::parse(
            "small capacitance\n.param cv=100f\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 {cv}\n.end\n",
        )
        .unwrap();
        let engine = Engine::default();
        let derivative = engine
            .run_sensitivity_ac(&netlist, 2, "cv", capacitance, &[frequency], None)
            .unwrap()[0];
        // |H| = 1 / sqrt(1 + (wRC)^2), evaluated at wRC = 1.
        let expected = -1.0 / (2.0_f64.sqrt().powi(3) * capacitance);
        assert!(
            (derivative / expected - 1.0).abs() < 2e-6,
            "{derivative} vs {expected}"
        );
    }

    #[test]
    fn sensitivity_postprocessing_preserves_cancellation_and_rejects_nonfinite_derivatives() {
        let netlist = Netlist::parse("Postprocessing\nR1 out 0 1\n.end\n").unwrap();
        let target = Engine::collect_ac_sensitivity_targets(
            &netlist,
            crate::resource::ResourceLimits::default(),
        )
        .unwrap()
        .remove(0);
        let one = Complex64::new(1.0, 0.0);
        let result = Engine::complete_ac_sensitivity_trace(
            &target,
            &[one],
            vec![Complex64::new(f64::NAN, 0.0)],
            &crate::abort_signal::NoAbort,
        );
        assert!(result.unwrap_err().to_string().contains("non-finite"));
        let abort = crate::abort_signal::CountingAbort::new(1);
        let result = Engine::complete_ac_sensitivity_trace(&target, &[one], vec![one], &abort);
        assert!(matches!(result, Err(crate::SimulationError::Aborted)));
        assert_eq!(abort.count(), 2);
    }

    #[test]
    fn sensitivity_stencils_preserve_nonuniform_coordinates_and_wide_cancellation() {
        for (points, values, expected) in [
            ([1.0, 2.0, 4.0], [1.0, 4.0, 16.0], 2.0),
            ([1.0, 0.0, -2.0], [1.0, 0.0, 4.0], 2.0),
            ([0.0, -1e308, 1e308], [0.0, -1e308, 1e308], 1.0),
            ([0.0, 1e-300, 2e-300], [1e308; 3], 0.0),
        ] {
            assert_eq!(sensitivity_three_point(points, values).unwrap(), expected);
        }
        assert_eq!(
            sensitivity_three_point([0.0, -1e308, 1e308], [0.0, -1e308, 1e308]).unwrap(),
            1.0
        );
        assert!(sensitivity_three_point([0.0, 1.0, 1.0], [0.0, 1.0, 1.0]).is_err());
        assert!(sensitivity_three_point([0.0, 5e307, 1e308], [0.0, 5e-309, 1e-308]).is_err());
    }

    #[test]
    fn sensitivity_scalar_accepts_finite_samples_with_an_overflowing_span() {
        let netlist =
            Netlist::parse("Wide scalar sensitivity\n.param p=0\nV1 out 0 {p}\n.end\n").unwrap();
        assert_eq!(
            Engine::default()
                .run_sensitivity(&netlist, 1, "p", 0.0, Some(1e308))
                .unwrap(),
            1.0
        );
    }

    #[test]
    fn sensitivity_availability_preserves_absolute_ac_derivatives() {
        use crate::analysis::{SensitivityUnavailability as Reason, SensitivityValue};
        let netlist =
            Netlist::parse("Zero output\nV1 out 0 DC 0 AC 0\nR1 out 0 1\n.end\n").unwrap();
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 1,
                    negative: None,
                },
                &[1.0],
                &[],
            )
            .unwrap();
        assert_eq!(result.output_values[0], Complex64::new(0.0, 0.0));
        for trace in &result.sensitivities {
            assert_eq!(trace.normalized[0].reason(), Some(Reason::ZeroOutput));
            assert_eq!(trace.phase[0].reason(), Some(Reason::ZeroOutput));
            if trace.absolute[0] == Complex64::new(0.0, 0.0) {
                assert_eq!(trace.magnitude[0].value(), Some(0.0));
            } else {
                assert_eq!(
                    trace.magnitude[0].reason(),
                    Some(Reason::NondifferentiableMagnitude)
                );
            }
        }
        let amplitude = result
            .sensitivities
            .iter()
            .find(|trace| trace.absolute[0] == Complex64::new(1.0, 0.0))
            .unwrap();
        assert_eq!(amplitude.magnitude[0].value(), None);

        let mut target = Engine::collect_ac_sensitivity_targets(
            &netlist,
            crate::resource::ResourceLimits::default(),
        )
        .unwrap()
        .remove(0);
        target.nominal_value = 1e200;
        let trace = Engine::complete_ac_sensitivity_trace(
            &target,
            &[Complex64::new(1e-200, 0.0)],
            vec![Complex64::new(1e200, 1e200)],
            &crate::abort_signal::NoAbort,
        )
        .unwrap();
        assert_eq!(trace.absolute[0], Complex64::new(1e200, 1e200));
        assert_eq!(trace.normalized[0].reason(), Some(Reason::OutOfRange));
        assert_eq!(trace.magnitude[0].value(), Some(1e200));
        assert_eq!(trace.phase[0].reason(), Some(Reason::OutOfRange));
        assert_eq!(
            SensitivityValue::decibels(Complex64::new(0.0, 0.0), amplitude.absolute[0]).reason(),
            Some(Reason::ZeroOutput)
        );
        for scale in [1e-300, 1.0, 1e308] {
            let value = Complex64::new(scale, scale);
            assert!(
                (SensitivityValue::decibels(value, value).value().unwrap()
                    / (20.0 / std::f64::consts::LN_10)
                    - 1.0)
                    .abs()
                    < 1e-14
            );
        }
    }

    #[test]
    fn complete_ac_sensitivity_preserves_derived_traces_at_extreme_scales() {
        let engine = Engine::default();
        for current in [1e-200, 1e200] {
            for phase in [0, 60] {
                let netlist = Netlist::parse(&format!(
                    "AC sensitivity scale\nI1 0 out AC {current:e} {phase}\nR1 out 0 1\n.end\n"
                ))
                .unwrap();
                let result = engine
                    .run_sensitivity_ac_complete(
                        &netlist,
                        AcSensitivityOutput::Voltage {
                            positive: 1,
                            negative: None,
                        },
                        &[1.0],
                        &["R1".into()],
                    )
                    .unwrap();
                let trace = result.get("R1").unwrap();
                assert!(
                    (trace.normalized[0].value().unwrap() - Complex64::new(1.0, 0.0)).norm()
                        < 2e-10,
                    "{trace:?}"
                );
                assert!(
                    (trace.magnitude[0].value().unwrap() / current - 1.0).abs() < 2e-10,
                    "{trace:?}"
                );
                assert!(trace.phase[0].value().unwrap().abs() < 2e-10, "{trace:?}");
            }
        }
    }

    #[test]
    fn complete_ac_sensitivity_one_sided_capacitance_keeps_finite_large_output_derivatives() {
        let netlist = Netlist::parse(
            "Boundary capacitance\nI1 0 out AC 1e308\nR1 out 0 1\nC1 out 0 0\n.end\n",
        )
        .unwrap();
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 1,
                    negative: None,
                },
                &[0.01],
                &["C1".into()],
            )
            .unwrap();
        let trace = result.get("C1").unwrap();
        let omega = std::f64::consts::TAU * 0.01;
        assert_eq!(trace.absolute[0].re, 0.0);
        assert!(
            (trace.absolute[0].im / (-omega * 1e308) - 1.0).abs() < 2e-12,
            "{trace:?}"
        );
        assert_eq!(
            trace.normalized[0].value().unwrap(),
            Complex64::new(0.0, 0.0)
        );
        assert_eq!(trace.magnitude[0].value().unwrap(), 0.0);
        assert!((trace.phase[0].value().unwrap() / -omega - 1.0).abs() < 2e-12);
    }

    #[test]
    fn sensitivity_refuses_unrepresentable_perturbations() {
        assert!(Engine::sensitivity_step(1.0, Some(f64::MIN_POSITIVE)).is_err());
        assert!(
            Engine::sensitivity_step(f64::MAX, None)
                .unwrap()
                .is_finite()
        );
        assert_eq!(
            Engine::sensitivity_step(0.0, Some(f64::MAX)).unwrap(),
            f64::MAX
        );
        assert!(Engine::sensitivity_step(f64::from_bits(2), None).unwrap() > 0.0);
    }

    #[test]
    fn parameter_replay_honors_mid_build_cancellation() {
        let netlist =
            Netlist::parse("override cancellation\n.param p=1\nR1 1 0 {p}\n.end\n").unwrap();
        let abort = crate::abort_signal::CountingAbort::new(1);
        let error = Engine::create_perturbed_netlist_multi_with_abort(
            &netlist,
            &[("P".to_string(), 2.0)],
            &abort,
        )
        .expect_err("parameter replay must poll while applying overrides");
        assert!(matches!(error, crate::SimulationError::Aborted));
        assert!(abort.count() >= 2);
    }

    #[test]
    fn parameter_replay_evaluates_dependencies_after_the_override() {
        use crate::netlist::expr::{
            ParameterRedefinitionDiagnosticPolicy, ParameterRedefinitionPolicy,
        };
        let source = "Parameter dependencies\n.param base=2 derived={3*base} factor=1\n\
            V1 in 0 1\nE1 out 0 in 0 {derived*factor}\n.end\n";
        for policy in [
            ParameterRedefinitionPolicy::UseFirst,
            ParameterRedefinitionPolicy::UseLast,
        ] {
            let netlist = Netlist::parse_with_options(
                source,
                crate::netlist::NetlistParseOptions {
                    parameter_redefinition_policy: policy,
                    parameter_redefinition_diagnostic_policy:
                        ParameterRedefinitionDiagnosticPolicy::Error,
                    ..Default::default()
                },
            )
            .unwrap();
            let (first, _) =
                Engine::create_perturbed_netlist_multi(&netlist, &[("base".into(), 3.0)]).unwrap();
            assert_eq!(first.params.get("derived"), Some(9.0));
            assert_eq!(first.source_text.as_deref(), Some(source));
            assert!(first.diagnostics.is_empty());
            let (second, _) =
                Engine::create_perturbed_netlist_multi(&first, &[("factor".into(), 2.0)]).unwrap();
            assert_eq!(second.params.get("derived"), Some(9.0));
            assert_eq!(second.params.get("factor"), Some(2.0));
            let result = Engine::default().run_dc_op(&second).unwrap();
            assert!((result.voltage(2) - 18.0).abs() < 1e-10);
        }
    }

    #[test]
    fn parameter_studies_use_defined_parameters_including_indirect_and_zero_influence() {
        let engine = Engine::default();
        for (source, expected) in [
            (
                "Aliases\n.param base=2 derived={3*base}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {derived}\n.end\n",
                3.0_f64,
            ),
            (
                "Deferred aliases\n.param base=2 derived={3*base}\nV1 in 0 DC 1 AC 1\nR1 in out {derived}\nR2 out 0 6\n.end\n",
                -0.125,
            ),
            (
                "Function dependency\n.param base=2 derived={3*base}\n.func gain(x) {derived*x}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {gain(2)}\n.end\n",
                6.0,
            ),
            (
                "Behavioral dependency\n.param base=2 derived={3*base}\nV1 in 0 DC 1 AC 1\nB1 out 0 V=derived*V(in)\n.end\n",
                3.0,
            ),
            (
                "Local shadow\n.param base=2 derived={3*base}\nV1 in 0 DC 1 AC 1\nX1 in out buffer\n.subckt buffer a b\n.param derived=7\nE1 b 0 a 0 {derived}\n.ends\n.end\n",
                0.0,
            ),
            (
                "Deferred replaced alias\n.param base=2 derived={3*base}\nV1 in 0 DC 1 AC 1\nB1 out 0 V={derived*V(in)}\n.param derived=7\n.end\n",
                0.0,
            ),
        ] {
            let netlist = Netlist::parse(source).unwrap();
            let dc = engine
                .run_sensitivity(&netlist, 2, "base", 2.0, None)
                .unwrap_or_else(|error| panic!("{source}: {error}"));
            let ac = engine
                .run_sensitivity_ac(&netlist, 2, "base", 2.0, &[1.0], None)
                .unwrap_or_else(|error| panic!("{source}: {error}"));
            let tolerance = expected.abs() * 1e-8;
            assert!((dc - expected).abs() <= tolerance, "{source}: {dc}");
            assert!((ac[0] - expected).abs() <= tolerance, "{source}: {ac:?}");
        }
    }

    #[test]
    fn parameter_studies_reject_undefined_node_and_function_argument_names() {
        let engine = Engine::default();
        for source in [
            "Node collision\nV1 base 0 DC 1 AC 1\nE1 out 0 base 0 7\n.end\n",
            "Formal collision\n.func gain(base) {3*base}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {gain(7)}\n.end\n",
        ] {
            for retain_source in [true, false] {
                let mut netlist = Netlist::parse(source).unwrap();
                if !retain_source {
                    netlist.source_text = None;
                }
                for error in [
                    engine
                        .run_sensitivity(&netlist, 2, "base", 2.0, None)
                        .unwrap_err(),
                    engine
                        .run_sensitivity_ac(&netlist, 2, "base", 2.0, &[1.0], None)
                        .unwrap_err(),
                ] {
                    assert!(error.to_string().contains("not bound"), "{source}: {error}");
                }
            }
        }
    }

    #[test]
    fn parameter_replay_enforces_retained_input_limits_and_finite_values() {
        use crate::resource::{ResourceKind, ResourceLimits};
        use crate::{NoAbort, Value};
        let source = "Replay limits\n.param p=1\nR1 1 0 {p}\n.end\n";
        for retain_source in [true, false] {
            let mut netlist = Netlist::parse(source).unwrap();
            // Cover retained source and programmatically materialized netlists.
            if !retain_source {
                netlist.source_text = None;
            }
            let bytes = netlist.retained_source_bytes() + "P".len() + std::mem::size_of::<Value>();
            let limits = ResourceLimits {
                max_netlist_bytes: bytes,
                ..Default::default()
            };
            let (perturbed, _) = Engine::create_perturbed_netlist_multi_with_limits_and_abort(
                &netlist,
                &[("p".into(), 2.0)],
                limits,
                &NoAbort,
            )
            .unwrap();
            let mut engine = Engine::default();
            engine.config.resource_limits = limits;
            engine.build_circuit(&perturbed).unwrap();
            engine.config.resource_limits.max_netlist_bytes = bytes - 1;
            assert!(matches!(engine.build_circuit(&perturbed),
                Err(crate::SimulationError::ResourceLimit(error)) if error.resource == ResourceKind::NetlistBytes));
            assert!(
                matches!(Engine::create_perturbed_netlist_multi_with_limits_and_abort(
                &netlist, &[("p".into(), 2.0)], engine.config.resource_limits, &NoAbort,
            ), Err(crate::SimulationError::ResourceLimit(error)) if error.resource == ResourceKind::NetlistBytes)
            );
            for invalid in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
                assert!(matches!(
                    Engine::create_perturbed_netlist_multi(&netlist, &[("p".into(), invalid)]),
                    Err(crate::SimulationError::Circuit(_))
                ));
            }
        }
    }

    #[test]
    fn parameter_replay_preserves_local_shadowing_and_global_namespace() {
        let source = "Local parameter\n.param gain=2\nV1 in 0 1\nX1 in out buffer\n\
            .subckt buffer a b\n.param gain=7\nE1 b 0 a 0 {gain}\n.ends\n.end\n";
        let netlist = Netlist::parse(source).unwrap();
        let (perturbed, _) =
            Engine::create_perturbed_netlist_multi(&netlist, &[("gain".into(), 3.0)]).unwrap();
        let result = Engine::default().run_dc_op(&perturbed).unwrap();
        assert!((result.voltage(2) - 7.0).abs() < 1e-10);
        let source =
            "Global parameter\n.GLOBAL_PARAM gain=2\nV1 in 0 1\nE1 out 0 in 0 {gain}\n.end\n";
        let netlist = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .unwrap();
        let (perturbed, _) =
            Engine::create_perturbed_netlist_multi(&netlist, &[("gain".into(), 3.0)]).unwrap();
        assert!(!perturbed.params.has_parameter_binding("gain"));
        assert_eq!(perturbed.params.get("gain"), Some(3.0));
        let result = Engine::default().run_dc_op(&perturbed).unwrap();
        assert!((result.voltage(2) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn parameter_replay_preserves_draws_from_overridden_defaults() {
        let netlist = Netlist::parse(
            "Overridden draw\n.options seed=7\n\
            .param chosen={aunif(0,1)} following={aunif(0,1)}\n\
            V1 out 0 {chosen+following}\n.end\n",
        )
        .unwrap();
        let (perturbed, _) =
            Engine::create_perturbed_netlist_multi(&netlist, &[("chosen".into(), 3.0)]).unwrap();
        assert_eq!(perturbed.params.get("chosen"), Some(3.0));
        assert_eq!(
            perturbed.params.get("following"),
            netlist.params.get("following")
        );
    }

    const PARAMETRIC_DIVIDER: &str = "\
Parametric divider
.param rval=1k
V1 1 0 10
R1 1 2 {rval}
R2 2 0 1k
.end
";

    const MODEL_STEP_DECK: &str = "\
Model step deck
V1 1 0 10
R1 1 2 RMOD L=10u W=1u
R2 2 0 1k
.model RMOD R RSH=100
.end
";

    /// Same divider but with `rval` referenced bare in the value position
    /// instead of inside a brace expression.
    const BARE_REFERENCE_DIVIDER: &str = "\
Parametric divider, bare reference
.param rval=1k
V1 1 0 10
R1 1 2 rval
R2 2 0 1k
.end
";

    const ORPHAN_PARAM_DIVIDER: &str = "\
Divider with an unreferenced parameter
.param rval=1k
.param orphan=42
V1 1 0 DC 10 AC 1
R1 1 2 {rval}
R2 2 0 1k
.end
";

    /// Analytic dV(2)/drval for the divider at rval=1k: -10*1k/(1k+1k)^2.
    const EXPECTED_DIVIDER_SENSITIVITY: f64 = -2.5e-3;

    const AC_DIVIDER: &str = "\
AC sensitivity divider
V1 in 0 DC 0 AC 1 0
R1 in out 1k
R2 out 0 1k
.end
";

    const DC_DIVIDER: &str = "\
DC sensitivity divider
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
";

    #[test]
    fn complete_dc_sensitivity_reports_device_and_source_derivatives() {
        let netlist = Netlist::parse(DC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[],
            )
            .expect("complete DC sensitivity runs");
        assert!(
            (result.output_value - 5.0).abs() < 1e-9,
            "output={}",
            result.output_value
        );
        let r1 = result.get("R1").expect("R1 sensitivity");
        let r2 = result.get("R2").expect("R2 sensitivity");
        let v1 = result.get("V1").expect("source sensitivity");
        assert!((r1.absolute + 2.5e-3).abs() < 1e-8);
        assert!((r2.absolute - 2.5e-3).abs() < 1e-8);
        assert!((v1.absolute - 0.5).abs() < 1e-9);
        assert!((r1.normalized.value().unwrap() + 0.5).abs() < 2e-7);
    }

    #[test]
    fn complete_dc_sensitivity_supports_branch_current_and_filters() {
        let netlist = Netlist::parse(DC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::BranchCurrent("v1".to_string()),
                &["R1".to_string()],
            )
            .expect("branch-current DC sensitivity runs");
        assert_eq!(result.output, "I(v1)");
        assert_eq!(result.len(), 1);
        assert!((result.output_value + 5.0e-3).abs() < 1e-12);
        assert!((result.sensitivities[0].absolute - 2.5e-6).abs() < 1e-11);
    }

    #[test]
    fn complete_dc_sensitivity_flattens_hierarchy_and_filters_parameters() {
        let netlist = Netlist::parse(
            "Hierarchical DC sensitivity\n\
V1 in 0 10\n\
XDIV in out DIVIDER\n\
.subckt DIVIDER input output\n\
RTOP input output 1k\n\
RBOT output 0 1k\n\
.ends\n\
.end\n",
        )
        .expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &["*RTOP".to_string()],
            )
            .expect("hierarchical filtered DC sensitivity runs");
        assert_eq!(result.len(), 1);
        assert!(
            result.sensitivities[0]
                .vector_name
                .to_ascii_uppercase()
                .ends_with("RTOP")
        );
        assert!((result.sensitivities[0].absolute + 2.5e-3).abs() < 1e-8);
    }

    #[test]
    fn complete_dc_sensitivity_rejects_team_memristor_instead_of_omitting_it() {
        let netlist = Netlist::parse(
            "TEAM DC sensitivity coverage\n\
V1 out 0 0.1\n\
.model team_model memristor level=2 ron=50 roff=1k xon=0 xoff=1\n\
YMEMRISTOR mr1 out 0 team_model ivrelation=0\n\
.end\n",
        )
        .expect("TEAM sensitivity deck parses");

        let message = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 1,
                    negative: None,
                },
                &[],
            )
            .expect_err("complete DC sensitivity must fail closed for TEAM")
            .to_string();

        assert!(message.contains("Complete DC sensitivity"), "{message}");
        assert!(message.contains("Xyce memristor"), "{message}");
        assert!(message.contains("YMEMRISTOR!MR1"), "{message}");
        assert!(
            message.contains("incomplete sensitivity result"),
            "{message}"
        );
    }

    #[test]
    fn complete_dc_sensitivity_detects_flattened_team_memristor_with_filters() {
        let netlist = Netlist::parse(
            "Hierarchical TEAM DC sensitivity coverage\n\
V1 in 0 0.1\n\
R1 in out 1k\n\
XMEM out 0 CELL\n\
.subckt CELL p n\n\
.model local_team memristor level=2 ron=50 roff=1k xon=0 xoff=1\n\
YMEMRISTOR state p n local_team ivrelation=0\n\
.ends\n\
.end\n",
        )
        .expect("hierarchical TEAM sensitivity deck parses");

        let message = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &["R1".to_string()],
            )
            .expect_err("filters must not permit a partially covered TEAM analysis")
            .to_string();

        assert!(message.contains("XMEM.YMEMRISTOR!STATE"), "{message}");
        assert!(message.contains("rejected"), "{message}");
    }

    #[test]
    fn complete_ac_sensitivity_reports_complex_device_and_source_derivatives() {
        let netlist = Netlist::parse(AC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[1.0, 1.0e3],
                &[],
            )
            .expect("complete AC sensitivity runs");

        assert_eq!(result.frequencies, vec![1.0, 1.0e3]);
        assert!((result.output_values[0].re - 0.5).abs() < 1e-12);
        let r1 = result.get("R1").expect("R1 primary sensitivity");
        let r2 = result.get("R2").expect("R2 primary sensitivity");
        assert!((r1.absolute[0].re + 2.5e-4).abs() < 1e-9);
        assert!((r2.absolute[0].re - 2.5e-4).abs() < 1e-9);
        assert!(r1.absolute[0].im.abs() < 1e-12);

        let ac_magnitude = result
            .get("V1_AC_MAG")
            .expect("source AC magnitude sensitivity");
        assert!((ac_magnitude.absolute[0].re - 0.5).abs() < 1e-9);
        let ac_phase = result
            .get("V1_AC_PHASE")
            .expect("source AC phase sensitivity");
        assert!((ac_phase.absolute[0].im - 0.5_f64.to_radians()).abs() < 1e-9);
    }

    #[test]
    fn complete_ac_sensitivity_filters_and_flattens_hierarchy() {
        let netlist = Netlist::parse(
            "Hierarchical sensitivity\n\
V1 in 0 AC 1\n\
XDIV in out DIVIDER\n\
.subckt DIVIDER input output\n\
RTOP input output 1k\n\
RBOT output 0 1k\n\
.ends\n\
.end\n",
        )
        .expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[1.0e3],
                &["*RTOP".to_string()],
            )
            .expect("hierarchical filtered sensitivity runs");
        assert_eq!(result.len(), 1);
        assert!(
            result.sensitivities[0]
                .vector_name
                .to_ascii_uppercase()
                .ends_with("RTOP")
        );
        assert!((result.sensitivities[0].absolute[0].re + 2.5e-4).abs() < 1e-9);
    }

    #[test]
    fn complete_ac_sensitivity_supports_branch_current_outputs() {
        let netlist = Netlist::parse(AC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::BranchCurrent("v1".to_string()),
                &[1.0e3],
                &["R1".to_string()],
            )
            .expect("branch current sensitivity runs");
        assert_eq!(result.output, "I(v1)");
        assert_eq!(result.len(), 1);
        assert!((result.sensitivities[0].absolute[0].re - 2.5e-7).abs() < 1e-12);
    }

    #[test]
    fn complete_ac_sensitivity_varies_expression_valued_model_parameters() {
        let netlist = Netlist::parse(
            "Model sensitivity\n\
.param sheet=100\n\
V1 in 0 AC 1\n\
R1 in out RMOD L=10u W=1u\n\
R2 out 0 1k\n\
.model RMOD R RSH={sheet}\n\
.end\n",
        )
        .expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[1.0e3],
                &["RMOD:*".to_string()],
            )
            .expect("model sensitivity runs");
        let rsh = result.get("RMOD:RSH").expect("RSH model sensitivity");
        assert_eq!(rsh.nominal_value, 100.0);
        assert!((rsh.absolute[0].re + 2.5e-3).abs() < 1e-8);
    }

    #[test]
    fn sens_parser_retains_current_output_filters_and_ac_sweep() {
        let netlist = Netlist::parse(
            "Sensitivity syntax\nV1 1 0 AC 1\nR1 1 0 1k\n.sens I(V1) R* RMOD:* AC DEC 10 1 1k\n.end\n",
        )
        .expect("deck parses");
        let AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } = &netlist.analyses[0]
        else {
            panic!("expected sensitivity command");
        };
        assert_eq!(output_node, "V1");
        assert!(reference_node.is_none());
        assert!(*output_is_current);
        assert_eq!(filters, &["R*", "RMOD:*"]);
        let sweep = ac_sweep.expect("AC sweep");
        assert_eq!(sweep.points, 10);
        assert_eq!(sweep.start_freq, 1.0);
        assert_eq!(sweep.stop_freq, 1.0e3);
    }

    /// An element name is not a parameter binding: before the fix, the
    /// `R1 1 2 {rval}` line itself made "R1" look referenced, and the run
    /// silently returned a sensitivity of 0.0 instead of erroring.
    #[test]
    fn sensitivity_rejects_element_name_lookalike() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity(&netlist, 2, "R1", 1000.0, None)
            .expect_err("element name must not count as a parameter binding");
        let msg = err.to_string();
        assert!(
            msg.contains("'R1'") && msg.contains("is not bound to any netlist expression"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn sensitivity_perturbs_brace_bound_parameter() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let dv = Engine::default()
            .run_sensitivity(&netlist, 2, "rval", 1000.0, None)
            .expect("rval is bound through a brace expression");
        assert!(
            (dv - EXPECTED_DIVIDER_SENSITIVITY).abs() < 1e-6,
            "dV(2)/drval = {dv}, expected ~{EXPECTED_DIVIDER_SENSITIVITY}"
        );
    }

    /// Bare identifiers in value positions are parameter references too;
    /// excluding the leading element-name token must not break them.
    #[test]
    fn sensitivity_perturbs_bare_value_position_parameter() {
        let netlist = Netlist::parse(BARE_REFERENCE_DIVIDER).expect("deck parses");
        let dv = Engine::default()
            .run_sensitivity(&netlist, 2, "rval", 1000.0, None)
            .expect("rval is bound through a bare value reference");
        assert!(
            (dv - EXPECTED_DIVIDER_SENSITIVITY).abs() < 1e-6,
            "dV(2)/drval = {dv}, expected ~{EXPECTED_DIVIDER_SENSITIVITY}"
        );
    }

    #[test]
    fn sensitivity_accepts_defined_parameters_without_output_dependence() {
        let netlist = Netlist::parse(ORPHAN_PARAM_DIVIDER).expect("deck parses");
        let dc = Engine::default()
            .run_sensitivity(&netlist, 2, "orphan", 42.0, None)
            .expect("a defined but unused parameter has zero derivative");
        let ac = Engine::default()
            .run_sensitivity_ac(&netlist, 2, "orphan", 42.0, &[1.0], None)
            .expect("a defined but unused parameter has zero AC derivative");
        assert_eq!(dc, 0.0);
        assert_eq!(ac, vec![0.0]);
    }

    #[test]
    fn sensitivity_rejects_invalid_output_node_without_panicking() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity(&netlist, 999, "rval", 1000.0, None)
            .expect_err("out-of-range sensitivity output node must raise");
        let msg = err.to_string();
        assert!(
            msg.contains("Sensitivity output node") && msg.contains("999"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn linearized_sensitivity_rejects_invalid_output_node_without_panicking() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity_linearized(&netlist, 999, None)
            .expect_err("out-of-range linearized sensitivity output node must raise");
        let msg = err.to_string();
        assert!(
            msg.contains("Sensitivity output node") && msg.contains("999"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn sparse_linearized_sensitivity_matches_divider_closed_form() {
        let netlist = Netlist::parse(
            "Sparse adjoint divider\n\
             V1 in 0 10\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
        )
        .expect("divider parses");
        let result = Engine::default()
            .run_sensitivity_linearized(&netlist, 2, None)
            .expect("sparse sensitivity succeeds");

        let close = |actual: f64, expected: f64| {
            assert!(
                (actual - expected).abs() <= 1.0e-12 * expected.abs().max(1.0),
                "actual={actual:.16e}, expected={expected:.16e}"
            );
        };
        close(result.output_value, 5.0);
        close(result.get("R1").expect("R1 sensitivity").absolute, -2.5e-3);
        close(result.get("R2").expect("R2 sensitivity").absolute, 2.5e-3);
        close(result.get("V1").expect("V1 sensitivity").absolute, 0.5);

        let differential = Engine::default()
            .run_sensitivity_linearized(&netlist, 2, Some(1))
            .expect("differential sparse sensitivity succeeds");
        close(differential.output_value, -5.0);
        close(
            differential
                .get("V1")
                .expect("differential V1 sensitivity")
                .absolute,
            -0.5,
        );
    }

    #[test]
    fn ac_sensitivity_rejects_invalid_output_node_without_silent_zero() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity_ac(&netlist, 999, "rval", 1000.0, &[1e3], None)
            .expect_err("out-of-range AC sensitivity output node must raise");
        let msg = err.to_string();
        assert!(
            msg.contains("Sensitivity output node") && msg.contains("999"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn sensitivity_rejects_invalid_numeric_inputs() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let engine = Engine::default();

        let err = engine
            .run_sensitivity(&netlist, 2, "rval", f64::NAN, None)
            .expect_err("non-finite sensitivity parameter values must be rejected");
        assert!(
            err.to_string().contains("param_value must be finite"),
            "unexpected error: {err}"
        );

        let err = engine
            .run_sensitivity(&netlist, 2, "rval", 1000.0, Some(0.0))
            .expect_err("zero sensitivity delta must be rejected");
        assert!(
            err.to_string()
                .contains("delta must be a positive finite number"),
            "unexpected error: {err}"
        );

        let err = engine
            .run_sensitivity_ac(&netlist, 2, "rval", f64::INFINITY, &[1e3], None)
            .expect_err("non-finite AC sensitivity parameter values must be rejected");
        assert!(
            err.to_string().contains("param_value must be finite"),
            "unexpected error: {err}"
        );

        let err = engine
            .run_sensitivity_ac(&netlist, 2, "rval", 1000.0, &[1e3], Some(-1.0))
            .expect_err("negative AC sensitivity delta must be rejected");
        assert!(
            err.to_string()
                .contains("delta must be a positive finite number"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ac_sensitivity_rejects_element_name_lookalike() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity_ac(&netlist, 2, "R1", 1000.0, &[1e3], None)
            .expect_err("element name must not count as a parameter binding");
        assert!(
            err.to_string()
                .contains("is not bound to any netlist expression"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn step_rejects_element_name_lookalike() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_step(&netlist, "R1", &[500.0, 2000.0])
            .expect_err("element name must not count as a parameter binding");
        assert!(
            err.to_string()
                .contains("is not bound to any netlist expression"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn step_rejects_empty_value_list() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_step(&netlist, "rval", &[])
            .expect_err("empty step sweep must not report success");
        assert!(
            err.to_string().contains("no sweep values"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn step_rejects_non_finite_value_list() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_step(&netlist, "rval", &[1000.0, f64::NAN])
            .expect_err("non-finite step sweep values must not enter the solver");
        assert!(
            err.to_string().contains("finite"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn model_step_rejects_empty_value_list() {
        let netlist = Netlist::parse(MODEL_STEP_DECK).expect("deck parses");
        let command = StepCommand {
            target: StepTarget::Model,
            name: "RMOD".to_string(),
            param_name: Some("RSH".to_string()),
            sweep: StepSweep::List(Vec::new()),
        };
        let err = Engine::default()
            .run_step_command(&netlist, &command, &[])
            .expect_err("empty model step sweep must not report success");
        assert!(
            err.to_string().contains("no sweep values"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn colon_qualified_device_step_target_resolves_to_model_when_no_device_matches() {
        let netlist = Netlist::parse(MODEL_STEP_DECK).expect("deck parses");
        let command = StepCommand {
            target: StepTarget::Device,
            name: "RMOD".to_string(),
            param_name: Some("RSH".to_string()),
            sweep: StepSweep::List(vec![100.0, 200.0]),
        };

        let results = Engine::default()
            .run_step_command(&netlist, &command, &[100.0, 200.0])
            .expect("device-style model:param step target should resolve to the model");

        assert_eq!(results.len(), 2);
        assert!((results[0].1.voltage(2) - 5.0).abs() < 1e-9);
        assert!((results[1].1.voltage(2) - (10.0 / 3.0)).abs() < 1e-9);
    }

    #[test]
    fn step_sweeps_a_genuinely_bound_parameter() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let results = Engine::default()
            .run_step(&netlist, "rval", &[1000.0, 4000.0])
            .expect("rval is bound");
        assert_eq!(results.len(), 2);
        // V(2) = 10 * 1k / (rval + 1k): 5 V at 1k, 2 V at 4k.
        assert!((results[0].1.voltage(2) - 5.0).abs() < 1e-9);
        assert!((results[1].1.voltage(2) - 2.0).abs() < 1e-9);
    }
}
