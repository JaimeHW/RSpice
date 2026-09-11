//! Root-coordinate derivatives of qualified linear physical equations.
//! Field directions come from the actual binding and construction sites.

use super::*;
use crate::expr::Derivative;
use crate::netlist::expr::ComplexDirection;
use crate::netlist::{Element, ElementParameterDirection};

fn linear_element(kind: &ElementKind) -> bool {
    match kind {
        ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } => {
            (value_expr.is_some() || (value.is_finite() && *value > 0.0))
                && model.is_none()
                && instance_params.is_empty()
                && deferred_params.is_empty()
        }
        ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
            ..
        }
        | ElementKind::Inductor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
            ..
        } => {
            value.is_finite()
                && *value > 0.0
                && value_expr.is_none()
                && model.is_none()
                && instance_params.is_empty()
                && deferred_params.is_empty()
        }
        ElementKind::Vcvs { gain_expr, .. } | ElementKind::Cccs { gain_expr, .. } => {
            gain_expr.is_none()
        }
        ElementKind::Ccvs {
            transresistance_expr,
            ..
        } => transresistance_expr.is_none(),
        ElementKind::Vccs {
            transconductance_expr,
            multiplicity,
            ..
        } => {
            transconductance_expr.is_none()
                && multiplicity.value_expr.is_none()
                && multiplicity.value.is_finite()
        }
        ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
            matches!(
                spec,
                SourceSpec::Dc(_) | SourceSpec::Ac { .. } | SourceSpec::DcAc { .. }
            )
        }
        _ => false,
    }
}

fn times_phasor(direction: Derivative, value: Complex64) -> ComplexDirection {
    ComplexDirection {
        re: direction * value.re,
        im: direction * value.im,
    }
}

fn branch_row(circuit: &CircuitData, name: &str) -> Result<usize, SimulationError> {
    circuit
        .get_branch_by_name(name)
        .map(|ordinal| circuit.get_branch_matrix_index(ordinal) - 1)
        .ok_or_else(|| {
            SimulationError::Circuit(format!("Sensitivity branch '{name}' was not found"))
        })
}

fn node_index(circuit: &CircuitData, name: &str) -> Result<Option<usize>, SimulationError> {
    circuit
        .get_node_by_name(name)
        .map(|node| node.checked_sub(1))
        .ok_or_else(|| SimulationError::Circuit(format!("Sensitivity node '{name}' was not found")))
}

fn voltage(solution: &[Complex64], pair: [Option<usize>; 2]) -> Complex64 {
    pair[0].map_or(Complex64::new(0.0, 0.0), |index| solution[index])
        - pair[1].map_or(Complex64::new(0.0, 0.0), |index| solution[index])
}

/// Assemble db - dA*x without narrowing a field derivative before its owning
/// physical factors have been applied. The adjoint also remains outside this
/// conversion, so a very large forcing can yield a representable output.
fn add_element_direction(
    rhs: &mut [ComplexDirection],
    circuit: &CircuitData,
    element: &Element,
    direction: ElementParameterDirection,
    solution: &[Complex64],
    ac: bool,
    frequency: Value,
) -> Result<(), SimulationError> {
    let terminals = [
        node_index(circuit, &element.nodes[0])?,
        node_index(circuit, &element.nodes[1])?,
    ];
    let mut inject = |current: ComplexDirection| {
        if let Some(index) = terminals[0] {
            rhs[index] = rhs[index] - current;
        }
        if let Some(index) = terminals[1] {
            rhs[index] = rhs[index] + current;
        }
    };
    match (&element.kind, direction) {
        (ElementKind::Resistor { .. }, ElementParameterDirection::Passive { value, direction }) => {
            // R*x is the voltage equation when this resistor owns a current unknown.
            if circuit.get_branch_by_name(&element.name).is_some() {
                let row = branch_row(circuit, &element.name)?;
                rhs[row] = rhs[row] + times_phasor(direction, solution[row]);
            } else {
                inject(times_phasor(
                    -direction / value / value,
                    voltage(solution, terminals),
                ));
            }
        }
        (ElementKind::Capacitor { .. }, ElementParameterDirection::Passive { direction, .. }) => {
            if ac {
                let value = voltage(solution, terminals);
                inject(times_phasor(
                    direction * (2.0 * std::f64::consts::PI) * frequency,
                    Complex64::new(-value.im, value.re),
                ));
            }
        }
        (ElementKind::Inductor { .. }, ElementParameterDirection::Passive { direction, .. }) => {
            if ac {
                let row = branch_row(circuit, &element.name)?;
                let value = solution[row];
                rhs[row] = rhs[row]
                    + times_phasor(
                        direction * (2.0 * std::f64::consts::PI) * frequency,
                        Complex64::new(-value.im, value.re),
                    );
            }
        }
        (
            ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. },
            ElementParameterDirection::Gain(direction),
        ) => {
            let control = [
                node_index(circuit, &control_nodes.0)?,
                node_index(circuit, &control_nodes.1)?,
            ];
            let effect = times_phasor(direction, voltage(solution, control));
            if matches!(element.kind, ElementKind::Vcvs { .. }) {
                let row = branch_row(circuit, &element.name)?;
                rhs[row] = rhs[row] + effect;
            } else {
                inject(effect);
            }
        }
        (
            ElementKind::Cccs {
                control_element, ..
            }
            | ElementKind::Ccvs {
                control_element, ..
            },
            ElementParameterDirection::Gain(direction),
        ) => {
            let effect = times_phasor(direction, solution[branch_row(circuit, control_element)?]);
            if matches!(element.kind, ElementKind::Ccvs { .. }) {
                let row = branch_row(circuit, &element.name)?;
                rhs[row] = rhs[row] + effect;
            } else {
                inject(effect);
            }
        }
        (
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec),
            ElementParameterDirection::Source {
                dc,
                magnitude,
                phase,
            },
        ) => {
            let effect = if ac {
                let (amplitude, angle) = match spec {
                    SourceSpec::Ac { magnitude, phase }
                    | SourceSpec::DcAc {
                        ac_magnitude: magnitude,
                        ac_phase: phase,
                        ..
                    } => (*magnitude, *phase),
                    SourceSpec::Dc(_) => (0.0, 0.0),
                    _ => unreachable!("source qualification"),
                };
                let (sin, cos) = angle.sin_cos();
                ComplexDirection {
                    re: magnitude * cos - phase * amplitude * sin,
                    im: magnitude * sin + phase * amplitude * cos,
                }
            } else {
                ComplexDirection::from(dc)
            };
            if matches!(element.kind, ElementKind::VoltageSource(_)) {
                let row = branch_row(circuit, &element.name)?;
                rhs[row] = rhs[row] + effect;
            } else {
                inject(effect);
            }
        }
        _ => {
            return Err(SimulationError::Circuit(format!(
                "Incomplete parameter derivative for '{}'",
                element.name
            )));
        }
    }
    Ok(())
}

impl Engine {
    /// None means that some owning fields or structural effects still require
    /// the existing replay/refinement path; absence is never proof of zero.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn linear_parameter_sensitivity(
        &self,
        netlist: &Netlist,
        output: &AcSensitivityOutput,
        parameter: &str,
        value: Value,
        frequencies: Option<&[Value]>,
        runs: &mut usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<(Vec<Complex64>, Vec<Complex64>)>, SimulationError> {
        if netlist.source_text.is_none()
            || !netlist.params.has_any_parameter_binding(parameter)
            || !netlist.ast_overlay.device_parameters.is_empty()
            || !netlist.spectre_statistics.variations.is_empty()
            || !netlist.elements.iter().all(|element| {
                linear_element(&element.kind)
                    || matches!(element.kind, ElementKind::Subcircuit { .. })
            })
        {
            return Ok(None);
        }
        let (directed, _) = Self::replay_parameter_overrides(
            netlist,
            &[(parameter.to_ascii_uppercase(), value)],
            Some(parameter),
            self.config.resource_limits,
            abort,
        )?;
        let Some(capture) = &directed.parameter_direction else {
            return Ok(None);
        };
        if capture.has_uncaptured_dependencies {
            return Ok(None);
        }
        let engine = self.resolved_for_netlist(&directed);
        let run_scope = crate::abort_signal::ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        Self::ensure_model_run_active(abort)?;
        let mut circuit = engine.build_circuit_with_abort(&directed, abort)?;
        // An enforced IC owns a DC voltage constraint whose direction has not
        // been captured. A transient-only seed adds no such physical equation.
        if circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .any(Option::is_some)
        {
            return Ok(None);
        }
        let Some(capture) = circuit.parameter_direction.take() else {
            return Ok(None);
        };
        if capture.has_uncaptured_dependencies
            || !capture.owners.iter().all(|element| {
                linear_element(&element.kind) && capture.elements.contains_key(&element.name)
            })
        {
            return Ok(None);
        }
        *runs = runs.saturating_add(1);
        self.ensure_batch_runs(*runs)?;
        let mut prepared = engine.prepare_ac_circuit(&directed, circuit, abort)?;
        let circuit = &prepared.circuit;
        let size = circuit.matrix_size();
        let points = frequencies.unwrap_or(&[0.0]);
        engine.ensure_result_values(
            size.saturating_mul(12)
                .saturating_add(points.len().saturating_mul(4)),
        )?;
        let mut observation = vec![Complex64::new(0.0, 0.0); size];
        match output {
            AcSensitivityOutput::Voltage { positive, negative } => {
                for (node, sign) in [(*positive, 1.0), (negative.unwrap_or(0), -1.0)] {
                    Self::validate_sensitivity_node("output", node, circuit.num_nodes())?;
                    if node != 0 {
                        observation[node - 1].re += sign;
                    }
                }
            }
            AcSensitivityOutput::BranchCurrent(name) => {
                observation[branch_row(circuit, name)?].re = 1.0
            }
        }
        let mut matrix = Self::try_build_small_signal_ac_matrix(
            circuit,
            &prepared.matrix,
            &prepared.linearization.bias,
            0.0,
        )?;
        let mut nominal = Vec::with_capacity(points.len());
        let mut derivatives = Vec::with_capacity(points.len());
        let mut adjoint = Vec::with_capacity(size);
        let mut rhs = vec![ComplexDirection::zero(); size];
        for (point, &frequency) in points.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let solution = if frequencies.is_some() {
                prepared.linearization.prepare_frequency(
                    &mut prepared.circuit,
                    &mut matrix,
                    frequency,
                    point + 1 == points.len(),
                    abort,
                )?;
                prepared
                    .linearization
                    .solve(&mut matrix, &prepared.excitation, abort)?
            } else {
                prepared
                    .linearization
                    .bias
                    .iter()
                    .map(|&value| Complex64::new(value, 0.0))
                    .collect()
            };
            matrix
                .solve_transpose_into(&observation, &mut adjoint)
                .map_err(SimulationError::Solver)?;
            rhs.fill(ComplexDirection::zero());
            for (index, element) in capture.owners.iter().enumerate() {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                add_element_direction(
                    &mut rhs,
                    &prepared.circuit,
                    element,
                    capture.elements[&element.name],
                    &solution,
                    frequencies.is_some(),
                    frequency,
                )?;
            }
            if let (Some(direction), Some(resistance)) = (capture.rshunt, directed.options.rshunt)
                && direction != 0.0
            {
                let conductance_direction = -direction / resistance / resistance;
                for row in 0..prepared.circuit.num_nodes() {
                    if row.is_multiple_of(64) && abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    if !prepared.circuit.is_non_electrical_state_matrix_index(row) {
                        rhs[row] = rhs[row] - times_phasor(conductance_direction, solution[row]);
                    }
                }
            }
            if frequencies.is_some()
                && let Some(direction) = capture.cshunt
            {
                let susceptance_direction = direction * std::f64::consts::TAU * frequency;
                for (index, &row) in capture.cshunt_rows.iter().enumerate() {
                    if index.is_multiple_of(64) && abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    let value = solution[row];
                    rhs[row] = rhs[row]
                        - times_phasor(susceptance_direction, Complex64::new(-value.im, value.re));
                }
            }
            let mut derivative = ComplexDirection::zero();
            for (forcing, weight) in rhs.iter().zip(&adjoint) {
                derivative = derivative
                    + ComplexDirection {
                        re: forcing.re * weight.re - forcing.im * weight.im,
                        im: forcing.re * weight.im + forcing.im * weight.re,
                    };
            }
            let result = derivative.binary64();
            if !result.re.is_finite()
                || !result.im.is_finite()
                || (result.re == 0.0 && derivative.re != 0.0)
                || (result.im == 0.0 && derivative.im != 0.0)
            {
                return Err(SimulationError::Circuit(format!(
                    "Sensitivity to '{parameter}' exceeds finite representable precision"
                )));
            }
            nominal.push(
                solution
                    .iter()
                    .zip(&observation)
                    .map(|(value, weight)| value * weight)
                    .sum(),
            );
            derivatives.push(Complex64::new(result.re, result.im));
        }
        Ok(Some((nominal, derivatives)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ExpressionDialect;

    fn parse(source: &str, dialect: ExpressionDialect) -> Netlist {
        Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: dialect,
                parameter_redefinition_policy: crate::netlist::ParameterRedefinitionPolicy::UseLast,
                parameter_redefinition_diagnostic_policy:
                    crate::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
                ..Default::default()
            },
        )
        .unwrap()
    }

    #[test]
    fn linear_root_sensitivity_preserves_small_physical_directions() {
        let slope = 1e-8;
        let frequency = 1.0 / (2.0 * std::f64::consts::PI);
        let cases = [
            ("V1 in 0 DC 1 AC 1\nE1 out 0 in 0 {1+1e-8*p}", slope, slope),
            (
                "V1 in 0 DC 1 AC 1\nG1 out 0 in 0 {1+1e-8*p}\nR1 out 0 1",
                -slope,
                slope,
            ),
            (
                "V1 in 0 DC 1 AC 1\nR0 in 0 1\nF1 out 0 V1 {1+1e-8*p}\nR1 out 0 1",
                slope,
                slope,
            ),
            (
                "V1 in 0 DC 1 AC 1\nR0 in 0 1\nH1 out 0 V1 {1+1e-8*p}",
                -slope,
                slope,
            ),
            ("V1 out 0 DC {1+1e-8*p} AC {1+1e-8*p} 30", slope, slope),
            (
                "I1 0 out DC {1+1e-8*p} AC {1+1e-8*p} 30\nR1 out 0 1",
                slope,
                slope,
            ),
            (
                ".param r={1+1e-8*p}\nV1 in 0 DC 1 AC 1\nR1 in out r\nR2 out 0 1",
                -0.25 * slope,
                -0.25 * slope,
            ),
            (
                "V1 in 0 DC 1 AC 1\nR1 in out 1\nC1 out 0 {1+1e-8*p}",
                0.0,
                -slope / (2.0 * 2.0_f64.sqrt()),
            ),
            (
                "V1 in 0 DC 1 AC 1\nR1 in out 1\nL1 out 0 {1+1e-8*p}",
                0.0,
                slope / (2.0 * 2.0_f64.sqrt()),
            ),
            (
                "V1 a 0 AC 1 {1e-8*p}\nV2 out a AC 1 90",
                0.0,
                slope * std::f64::consts::PI / 180.0 / 2.0_f64.sqrt(),
            ),
            (
                ".param q={1e-8*p}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {1+q}\n.param q={9e-8*p}",
                slope,
                slope,
            ),
            (
                ".param TEMP={27+1e-8*p}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {1+VT}",
                slope * crate::constants::thermal_voltage(1.0),
                slope * crate::constants::thermal_voltage(1.0),
            ),
            (
                ".param q={-1-1e-8*p}\nI1 0 out DC 1 AC 1\nR0 out 0 1\nR1 out 0 -q",
                0.25 * slope,
                0.25 * slope,
            ),
            ("V1 out 0 DC 1 AC 1", 0.0, 0.0),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let engine = Engine::default();
            for (body, expected_dc, expected_ac) in cases {
                let source = format!("Physical parameter directions\n.param p=13\n{body}\n.end\n");
                let netlist = parse(&source, dialect);
                let node = engine
                    .build_circuit(&netlist)
                    .unwrap()
                    .get_node_by_name("out")
                    .unwrap();
                let output = AcSensitivityOutput::Voltage {
                    positive: node,
                    negative: None,
                };
                let mut runs = 0;
                let dc = engine
                    .run_output_sensitivity_with_abort(
                        &netlist,
                        output.clone(),
                        "p",
                        0.0,
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap();
                assert_eq!(runs, 1, "must use one nominal solve: {body}");
                assert!(
                    (dc - expected_dc).abs() <= expected_dc.abs() * 2e-12,
                    "{dialect:?}: {body}: DC {dc:e} != {expected_dc:e}"
                );
                runs = 0;
                let ac = engine
                    .run_output_sensitivity_ac_with_abort(
                        &netlist,
                        output,
                        "p",
                        0.0,
                        &[frequency],
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap()[0]
                    .value()
                    .unwrap();
                assert_eq!(runs, 1, "must use one nominal sweep: {body}");
                assert!(
                    (ac - expected_ac).abs() <= expected_ac.abs() * 2e-12,
                    "{dialect:?}: {body}: AC {ac:e} != {expected_ac:e}"
                );
            }
        }
    }

    #[test]
    fn capacitor_ic_sensitivity_qualifies_the_actual_constraint() {
        for spice_dialect in [crate::SpiceDialect::Ngspice, crate::SpiceDialect::Xyce] {
            let engine =
                Engine::new(crate::SimulationConfig::default().with_spice_dialect(spice_dialect));
            for nested in [false, true] {
                let body = "C1 out 0 {1+1e-8*p} IC={p}";
                let body = if nested {
                    format!("X1 out cell p={{p}}\n.subckt cell out p=99\n{body}\n.ends")
                } else {
                    body.into()
                };
                let netlist = parse(
                    &format!(
                        "Physical IC qualification\n.param p=0\nI1 0 out AC 1\nR1 out 0 1\n{body}\n.end"
                    ),
                    ExpressionDialect::Ngspice,
                );
                let mut runs = 0;
                let result = engine
                    .linear_parameter_sensitivity(
                        &netlist,
                        &AcSensitivityOutput::Voltage {
                            positive: 1,
                            negative: None,
                        },
                        "p",
                        0.0,
                        Some(&[1.0 / std::f64::consts::TAU]),
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap();
                if spice_dialect == crate::SpiceDialect::Xyce {
                    assert!(result.is_none());
                    assert_eq!(runs, 0);
                } else {
                    let (_, derivative) = result.expect("a seed adds no DC constraint");
                    assert!((derivative[0].re / -5e-9 - 1.0).abs() < 2e-12);
                    assert!(derivative[0].im.abs() < 1e-20);
                    assert_eq!(runs, 1);
                }
            }
        }
    }

    #[test]
    fn signed_resistor_parameters_use_the_instantiated_scope() {
        let engine = Engine::default();
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (sign, actual) in [("+", "1+1e-8*p"), ("-", "-1-1e-8*p")] {
                for gap in ["", " "] {
                    let netlist = parse(
                        &format!(
                            "Signed scoped resistor\n.param p=0\nI1 0 out 1\nR0 out 0 1\nX1 out cell q={{{actual}}}\n.subckt cell out q=99\nR1 out 0 {sign}{gap}q\n.ends\n.end"
                        ),
                        dialect,
                    );
                    let circuit = engine.build_circuit(&netlist).unwrap();
                    assert_eq!(
                        circuit.resistors.reported_resistances[1], 1.0,
                        "{dialect:?}: {sign}{gap}q"
                    );
                    let output = AcSensitivityOutput::Voltage {
                        positive: circuit.get_node_by_name("out").unwrap(),
                        negative: None,
                    };
                    let mut runs = 0;
                    let derivative = engine
                        .run_output_sensitivity_with_abort(
                            &netlist, output, "p", 0.0, None, &mut runs, &NoAbort,
                        )
                        .unwrap();
                    assert_eq!(runs, 1);
                    assert!(
                        (derivative / 2.5e-9 - 1.0).abs() < 2e-12,
                        "{dialect:?}: {sign}{gap}q: {derivative:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn passive_sensitivity_tracks_the_accepted_primary_assignment() {
        let engine = Engine::default();
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for device in ["R", "C", "L"] {
                for nested in [false, true] {
                    for (fields, relative_direction) in [
                        (format!("{device}=q"), 1e-8),
                        ("VALUE=q".into(), 1e-8),
                        (format!("3 {device}=q"), 1e-8),
                        ("q VALUE=2".into(), 0.0),
                        ("{q} 2".into(), 0.0),
                        ("3 +{q}".into(), 1e-8),
                        ("3 {q}".into(), 1e-8),
                        ("{1+abs(p)} VALUE=2".into(), 0.0),
                        ("n VALUE=2".into(), 0.0),
                        ("3 VALUE={1+1e-8*p}".into(), 1e-8),
                        ("3 VALUE=1+1e-8*p".into(), 1e-8),
                        ("2 VALUE=n VALUE=2".into(), 0.0),
                        ("3 VALUE={aunif(2,0.25)*(1+1e-8*p)}".into(), 1e-8),
                    ] {
                        let body = format!(
                            ".param q={{1+1e-8*p}} n={{1+abs(p)}}\n{device}1 out 0 {fields}"
                        );
                        let body = if nested {
                            format!("X1 out cell p={{p}}\n.subckt cell out p=99\n{body}\n.ends")
                        } else {
                            body
                        };
                        let netlist = Netlist::parse_with_options(
                            &format!("Accepted passive value\n.param p=0\nI1 0 out DC 1 AC 1\nR0 out 0 1\n{body}\n.end"),
                            crate::netlist::NetlistParseOptions {
                                expression_dialect: dialect,
                                statistical_seed: Some(749),
                                ..Default::default()
                            },
                        ).unwrap();
                        let circuit = engine.build_circuit(&netlist).unwrap();
                        let value = match device {
                            "R" => circuit.resistors.reported_resistances[1],
                            "C" => circuit.capacitors.capacitances[0],
                            _ => circuit.inductors.inductances[0],
                        };
                        let next_draw = netlist.params.random().next_uniform();
                        let direction = value * relative_direction;
                        let expected_dc = if device == "R" {
                            direction / (1.0 + value).powi(2)
                        } else {
                            0.0
                        };
                        let expected_ac = match device {
                            "R" => expected_dc,
                            "C" => -value * direction / (1.0 + value * value).powf(1.5),
                            _ => direction / (1.0 + value * value).powf(1.5),
                        };
                        let output = AcSensitivityOutput::Voltage {
                            positive: circuit.get_node_by_name("out").unwrap(),
                            negative: None,
                        };
                        for (frequencies, expected) in [
                            (None, expected_dc),
                            (Some([1.0 / std::f64::consts::TAU]), expected_ac),
                        ] {
                            let mut runs = 0;
                            let result = if let Some(frequencies) = frequencies {
                                engine
                                    .run_output_sensitivity_ac_with_abort(
                                        &netlist,
                                        output.clone(),
                                        "p",
                                        0.0,
                                        &frequencies,
                                        None,
                                        &mut runs,
                                        &NoAbort,
                                    )
                                    .unwrap()[0]
                                    .value()
                                    .unwrap()
                            } else {
                                engine
                                    .run_output_sensitivity_with_abort(
                                        &netlist,
                                        output.clone(),
                                        "p",
                                        0.0,
                                        None,
                                        &mut runs,
                                        &NoAbort,
                                    )
                                    .unwrap()
                            };
                            assert_eq!(runs, 1, "{dialect:?} {device} nested={nested}: {fields}");
                            assert!(
                                (result - expected).abs() <= expected.abs() * 2e-12,
                                "{dialect:?} {device} nested={nested}: {fields}: {result:e} != {expected:e}"
                            );
                        }
                        let (directed, _) = Engine::replay_parameter_overrides(
                            &netlist,
                            &[("P".into(), 0.0)],
                            Some("p"),
                            engine.config.resource_limits,
                            &NoAbort,
                        )
                        .unwrap();
                        let directed_circuit = engine.build_circuit(&directed).unwrap();
                        let actual = match device {
                            "R" => directed_circuit.resistors.reported_resistances[1],
                            "C" => directed_circuit.capacitors.capacitances[0],
                            _ => directed_circuit.inductors.inductances[0],
                        };
                        assert_eq!(actual, value);
                        assert_eq!(directed.params.random().next_uniform(), next_draw);
                    }
                }
                let netlist = parse(
                    &format!(
                        "Used nonsmooth primary\n.param p=0\nI1 0 out 1\nR0 out 0 1\n{device}1 out 0 VALUE={{1+abs(p)}}\n.end"
                    ),
                    dialect,
                );
                assert!(
                    engine
                        .run_sensitivity(&netlist, 1, "p", 0.0, None)
                        .unwrap_err()
                        .to_string()
                        .contains("derivative")
                );
            }
        }
    }

    #[test]
    fn vccs_sensitivity_includes_multiplicity_without_resampling() {
        let engine = Engine::default();
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for nested in [false, true] {
                for (gain, multiplicity, relative_direction) in [
                    ("2", "M={1+1e-8*p}", 1e-8),
                    ("{1+1e-8*p}", "M=3", 1e-8),
                    ("{2*(1+3e-8*p)}", "M={4*(1+5e-8*p)}", 8e-8),
                    ("2", "M=1+1e-8*p", 1e-8),
                    ("2", "M=m", 1e-8),
                    ("2", "M=+ m", 1e-8),
                    ("1", "M={1+1e-8*p} M=2", 0.0),
                    ("1", "M=2 M={1+1e-8*p}", 1e-8),
                    ("1", "M={1+abs(p)} M=1", 0.0),
                    ("1e-200", "M={1e200*(1+1e200*p)}", 1e200),
                    (
                        "{aunif(2,0.25)*(1+3e-8*p)}",
                        "M={aunif(4,0.5)*(1+5e-8*p)}",
                        8e-8,
                    ),
                ] {
                    let body =
                        format!(".param m={{1+1e-8*p}}\nG1 out 0 in 0 {gain} {multiplicity}");
                    let body = if nested {
                        format!("X1 in out cell p={{p}}\n.subckt cell in out p=99\n{body}\n.ends")
                    } else {
                        body
                    };
                    let netlist = Netlist::parse_with_options(
                        &format!("VCCS multiplicity sensitivity\n.param p=0\nV1 in 0 DC 1 AC 1\nR1 out 0 1\n{body}\n.end"),
                        crate::netlist::NetlistParseOptions {
                            expression_dialect: dialect,
                            statistical_seed: Some(632),
                            ..Default::default()
                        },
                    ).unwrap();
                    let circuit = engine.build_circuit(&netlist).unwrap();
                    let expected = circuit.vccs.transconductances[0] * relative_direction;
                    let next_draw = netlist.params.random().next_uniform();
                    let output = AcSensitivityOutput::Voltage {
                        positive: circuit.get_node_by_name("out").unwrap(),
                        negative: None,
                    };
                    let mut runs = 0;
                    let dc = engine
                        .run_output_sensitivity_with_abort(
                            &netlist,
                            output.clone(),
                            "p",
                            0.0,
                            None,
                            &mut runs,
                            &NoAbort,
                        )
                        .unwrap();
                    assert_eq!(
                        runs, 1,
                        "{dialect:?} nested={nested}: {gain} {multiplicity}"
                    );
                    assert!(
                        (dc + expected).abs() <= expected.abs() * 2e-12,
                        "DC {dc:e} != {:e}",
                        -expected
                    );
                    runs = 0;
                    let ac = engine
                        .run_output_sensitivity_ac_with_abort(
                            &netlist,
                            output,
                            "p",
                            0.0,
                            &[1.0],
                            None,
                            &mut runs,
                            &NoAbort,
                        )
                        .unwrap()[0]
                        .value()
                        .unwrap();
                    assert_eq!(runs, 1);
                    assert!(
                        (ac - expected).abs() <= expected.abs() * 2e-12,
                        "AC {ac:e} != {expected:e}"
                    );
                    let (directed, _) = Engine::replay_parameter_overrides(
                        &netlist,
                        &[("P".into(), 0.0)],
                        Some("p"),
                        engine.config.resource_limits,
                        &NoAbort,
                    )
                    .unwrap();
                    let directed_circuit = engine.build_circuit(&directed).unwrap();
                    assert_eq!(
                        directed_circuit.vccs.transconductances,
                        circuit.vccs.transconductances
                    );
                    assert_eq!(directed.params.random().next_uniform(), next_draw);
                }
            }
            let netlist = parse(
                "Nonsmooth multiplicity\n.param p=0\nV1 in 0 DC 1 AC 1\nG1 out 0 in 0 1 M={1+abs(p)}\nR1 out 0 1\n.end",
                dialect,
            );
            assert!(
                engine
                    .run_sensitivity(&netlist, 2, "p", 0.0, None)
                    .unwrap_err()
                    .to_string()
                    .contains("derivative")
            );
        }
        let zero_m = parse(
            "Zero multiplicity\n.param p=0\nV1 in 0 1\nG1 out 0 in 0 1 M={p}\nR1 out 0 1\n.end",
            ExpressionDialect::Ngspice,
        );
        assert_eq!(
            engine.run_sensitivity(&zero_m, 2, "p", 0.0, None).unwrap(),
            -1.0
        );
    }

    #[test]
    fn parameter_sensitivity_includes_physical_shunt_options() {
        let engine = Engine::default();
        let slope = 1e-8;
        let frequency = 1.0 / std::f64::consts::TAU;
        let cases = [
            (".options rshunt={1+1e-8*p}", slope, 0.0),
            (".options cshunt={1+1e-8*p}", 0.0, slope),
            (".options rshunt={1+1e-8*p} cshunt={1+1e-8*p}", slope, slope),
            (
                ".param q={1+1e-8*p}
.options rshunt=q
.param q={1+9e-8*p}",
                slope,
                0.0,
            ),
            (
                ".options rshunt={1+1e-8*p}
.options rshunt=1",
                0.0,
                0.0,
            ),
            (
                ".options rshunt={aunif(1,0.25)*(1+1e-8*p)} cshunt={aunif(1,0.25)*(1+1e-8*p)}",
                slope,
                slope,
            ),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (options, relative_r_direction, relative_c_direction) in cases {
                // Each isolated unit load has Y = 1 + 1/Rshunt + j*Cshunt at omega=1.
                // One load is inside a subcircuit, so its shunted node is private.
                let netlist = Netlist::parse_with_options(
                    &format!(
                        "Shunt directions
.param p=0
{options}
I1 0 out DC 1 AC 1
R1 out 0 1
X1 cell
.subckt cell
I2 0 internal DC 1 AC 1
R2 internal 0 1
.ends
.end"
                    ),
                    crate::netlist::NetlistParseOptions {
                        expression_dialect: dialect,
                        statistical_seed: Some(351),
                        parameter_redefinition_diagnostic_policy:
                            crate::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
                        ..Default::default()
                    },
                )
                .unwrap();
                let g = netlist.options.rshunt.map_or(0.0, |r| r.recip());
                let c = netlist.options.cshunt.unwrap_or(0.0);
                let dg = -relative_r_direction * g;
                let dc = relative_c_direction * c;
                let expected_dc = -dg / (1.0 + g).powi(2);
                let expected_ac = -((1.0 + g) * dg + c * dc) / (1.0 + g).hypot(c).powi(3);
                let circuit = engine.build_circuit(&netlist).unwrap();
                let next_draw = netlist.params.random().next_uniform();
                for name in ["out", "X1.internal"] {
                    let output = AcSensitivityOutput::Voltage {
                        positive: circuit.get_node_by_name(name).unwrap(),
                        negative: None,
                    };
                    let mut runs = 0;
                    let dc = engine
                        .run_output_sensitivity_with_abort(
                            &netlist,
                            output.clone(),
                            "p",
                            0.0,
                            None,
                            &mut runs,
                            &NoAbort,
                        )
                        .unwrap();
                    assert_eq!(runs, 1);
                    assert!(
                        (dc - expected_dc).abs() <= expected_dc.abs() * 2e-12,
                        "{dialect:?} {options} {name}: DC {dc:e} != {expected_dc:e}"
                    );
                    runs = 0;
                    let ac = engine
                        .run_output_sensitivity_ac_with_abort(
                            &netlist,
                            output,
                            "p",
                            0.0,
                            &[frequency],
                            None,
                            &mut runs,
                            &NoAbort,
                        )
                        .unwrap()[0]
                        .value()
                        .unwrap();
                    assert_eq!(runs, 1);
                    assert!(
                        (ac - expected_ac).abs() <= expected_ac.abs() * 2e-12,
                        "{dialect:?} {options} {name}: AC {ac:e} != {expected_ac:e}"
                    );
                }
                let (directed, _) = Engine::replay_parameter_overrides(
                    &netlist,
                    &[("P".into(), 0.0)],
                    Some("p"),
                    engine.config.resource_limits,
                    &NoAbort,
                )
                .unwrap();
                engine.build_circuit(&directed).unwrap();
                assert_eq!(directed.options.rshunt, netlist.options.rshunt);
                assert_eq!(directed.options.cshunt, netlist.options.cshunt);
                assert_eq!(directed.params.random().next_uniform(), next_draw);
            }
            let scoped_option = parse(
                "Scoped options
.param p=0
I1 0 out 1
X1 out cell
.subckt cell a
.options rshunt=1
R1 a 0 1
.ends
.end",
                dialect,
            );
            let mut runs = 0;
            assert!(
                engine
                    .linear_parameter_sensitivity(
                        &scoped_option,
                        &AcSensitivityOutput::Voltage {
                            positive: 1,
                            negative: None
                        },
                        "p",
                        0.0,
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap()
                    .is_none()
            );
            assert_eq!(runs, 0);
        }
    }

    #[test]
    fn analytic_parameter_sensitivity_does_not_require_finite_probes() {
        let engine = Engine::default();
        let output = AcSensitivityOutput::Voltage {
            positive: 2,
            negative: None,
        };
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let netlist = parse(
                "Analytic coordinate
.param p=1
V1 in 0 DC 1 AC 1
E1 out 0 in 0 {p/1e308}
.end",
                dialect,
            );
            for value in [1.0, f64::MAX, -f64::MAX] {
                let delta = Some(f64::MIN_POSITIVE);
                // These requested probe coordinates round to the same value.
                assert!(Engine::sensitivity_step(value, delta).is_err());
                let mut runs = 0;
                let dc = engine
                    .run_output_sensitivity_with_abort(
                        &netlist,
                        output.clone(),
                        "p",
                        value,
                        delta,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap();
                assert_eq!(runs, 1);
                assert!((dc / 1e-308 - 1.0).abs() < 2e-14, "{dialect:?}: {dc:e}");
                runs = 0;
                let ac = engine
                    .run_output_sensitivity_ac_with_abort(
                        &netlist,
                        output.clone(),
                        "p",
                        value,
                        &[1.0],
                        delta,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap();
                assert_eq!(runs, 1);
                assert!(
                    (ac[0].value().unwrap() / (value.signum() * 1e-308) - 1.0).abs() < 2e-14,
                    "{dialect:?}: {ac:?}"
                );
            }
            // Invalid input remains an error even when no probes would be used.
            for (value, delta, diagnostic) in [
                (f64::NAN, None, "param_value"),
                (f64::INFINITY, None, "param_value"),
                (f64::NEG_INFINITY, None, "param_value"),
                (1.0, Some(0.0), "delta"),
                (1.0, Some(-1.0), "delta"),
                (1.0, Some(f64::NAN), "delta"),
                (1.0, Some(f64::INFINITY), "delta"),
            ] {
                for ac in [false, true] {
                    let mut runs = 0;
                    let error = if ac {
                        engine
                            .run_output_sensitivity_ac_with_abort(
                                &netlist,
                                output.clone(),
                                "p",
                                value,
                                &[1.0],
                                delta,
                                &mut runs,
                                &NoAbort,
                            )
                            .unwrap_err()
                    } else {
                        engine
                            .run_output_sensitivity_with_abort(
                                &netlist,
                                output.clone(),
                                "p",
                                value,
                                delta,
                                &mut runs,
                                &NoAbort,
                            )
                            .unwrap_err()
                    };
                    assert_eq!(runs, 0);
                    assert!(error.to_string().contains(diagnostic), "{error}");
                }
            }
            // A nonlinear owner still requires distinct refinement coordinates.
            let nonlinear = parse(
                "Refinement coordinate
.param p=1
V1 in 0 DC 1 AC 1
B1 out 0 V={p*V(in)}
.end",
                dialect,
            );
            let mut runs = 0;
            let dc = engine
                .run_output_sensitivity_with_abort(
                    &nonlinear,
                    output.clone(),
                    "p",
                    1.0,
                    Some(f64::MIN_POSITIVE),
                    &mut runs,
                    &NoAbort,
                )
                .unwrap_err();
            let ac = engine
                .run_output_sensitivity_ac_with_abort(
                    &nonlinear,
                    output.clone(),
                    "p",
                    1.0,
                    &[1.0],
                    Some(f64::MIN_POSITIVE),
                    &mut runs,
                    &NoAbort,
                )
                .unwrap_err();
            assert_eq!(runs, 0);
            for error in [dc, ac] {
                assert!(error.to_string().contains("representable"), "{error}");
            }
        }
    }

    #[test]
    fn hierarchical_root_sensitivity_uses_flattened_physical_owners() {
        let slope = 1e-8;
        let frequency = 1.0 / (2.0 * std::f64::consts::PI);
        let cases = [
            ("E1 out 0 in 0 {gain(q)}", slope, slope),
            ("G1 out 0 in 0 {gain(q)}\nR1 out 0 1", -slope, slope),
            (
                "Vsense in n 0\nR0 n 0 1\nF1 out 0 Vsense {gain(q)}\nR1 out 0 1",
                -slope,
                slope,
            ),
            (
                "Vsense in n 0\nR0 n 0 1\nH1 out 0 Vsense {gain(q)}",
                slope,
                slope,
            ),
            (
                "R1 in out {gain(q)}\nR2 out 0 1",
                -0.25 * slope,
                -0.25 * slope,
            ),
            (
                "R1 in out 1\nC1 out 0 {gain(q)}",
                0.0,
                -slope / (2.0 * 2.0_f64.sqrt()),
            ),
            (
                "R1 in out 1\nL1 out 0 {gain(q)}",
                0.0,
                slope / (2.0 * 2.0_f64.sqrt()),
            ),
            ("V1 out 0 DC {gain(q)} AC {gain(q)} 30", slope, slope),
            (
                "I1 0 out DC {gain(q)} AC {gain(q)} 30\nR1 out 0 1",
                slope,
                slope,
            ),
            (
                "V1 n 0 AC 1 {1e-8*q}\nV2 out n AC 1 90",
                0.0,
                slope * std::f64::consts::PI / 180.0 / 2.0_f64.sqrt(),
            ),
            (
                ".param z={1+q*1j}\nE1 out 0 in 0 {1+1e-8*img(z)}",
                slope,
                slope,
            ),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (body, expected_dc, expected_ac) in cases {
                let netlist = parse(
                    &format!(
                        "Hierarchical parameter sensitivity\n.param p=13\nVdrive in 0 DC 1 AC 1\nXtop in out wrapper q={{p}}\n.subckt wrapper in out q=99\nXleaf in out cell q={{q}}\n.ends\n.subckt cell in out q=99\n.func gain(x) {{1+1e-8*x}}\n{body}\n.ends\n.end"
                    ),
                    dialect,
                );
                let engine = Engine::default();
                let output = AcSensitivityOutput::Voltage {
                    positive: engine
                        .build_circuit(&netlist)
                        .unwrap()
                        .get_node_by_name("out")
                        .unwrap(),
                    negative: None,
                };
                let mut runs = 0;
                let dc = engine
                    .run_output_sensitivity_with_abort(
                        &netlist,
                        output.clone(),
                        "p",
                        0.0,
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap();
                assert_eq!(runs, 1, "{dialect:?} DC {body}");
                assert!(
                    (dc - expected_dc).abs() <= expected_dc.abs() * 2e-12,
                    "{dialect:?} {body}: DC {dc:e} != {expected_dc:e}"
                );
                runs = 0;
                let ac = engine
                    .run_output_sensitivity_ac_with_abort(
                        &netlist,
                        output,
                        "p",
                        0.0,
                        &[frequency],
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap()[0]
                    .value()
                    .unwrap();
                assert_eq!(runs, 1, "{dialect:?} AC {body}");
                assert!(
                    (ac - expected_ac).abs() <= expected_ac.abs() * 2e-12,
                    "{dialect:?} {body}: AC {ac:e} != {expected_ac:e}"
                );
            }
        }
    }

    #[test]
    fn hierarchical_sensitivity_preserves_instance_directions_and_random_draws() {
        let engine = Engine::default();
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let netlist = Netlist::parse_with_options(
                "Scoped sample direction
.param p=0
Vdrive in 0 DC 1 AC 1
Xleft in mid cell q={2*p}
Xright mid out cell q={3*p}
Rload out 0 1
.subckt cell in out q=99
R1 in out {aunif(2,0.25)*(1+1e-8*q)}
.ends
.end",
                crate::netlist::NetlistParseOptions {
                    expression_dialect: dialect,
                    statistical_seed: Some(351),
                    ..Default::default()
                },
            )
            .unwrap();
            let circuit = engine.build_circuit(&netlist).unwrap();
            let next_draw = netlist.params.random().next_uniform();
            let resistance = |name: &str| {
                let index = circuit
                    .resistors
                    .names
                    .iter()
                    .position(|actual| actual.eq_ignore_ascii_case(name))
                    .unwrap();
                circuit.resistors.reported_resistances[index]
            };
            let (left, right) = (resistance("Xleft.R1"), resistance("Xright.R1"));
            let expected = -(2.0 * left + 3.0 * right) * 1e-8 / (1.0 + left + right).powi(2);
            for frequencies in [None, Some([1.0].as_slice())] {
                let mut runs = 0;
                let (nominal, derivative) = engine
                    .linear_parameter_sensitivity(
                        &netlist,
                        &AcSensitivityOutput::Voltage {
                            positive: circuit.get_node_by_name("out").unwrap(),
                            negative: None,
                        },
                        "p",
                        0.0,
                        frequencies,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(runs, 1);
                assert!((nominal[0].re * (1.0 + left + right) - 1.0).abs() < 1e-14);
                assert!((derivative[0].re / expected - 1.0).abs() < 2e-12);
            }
            // Capture performs exactly the normal two instance draws.
            let (directed, _) = Engine::replay_parameter_overrides(
                &netlist,
                &[("P".into(), 0.0)],
                Some("p"),
                engine.config.resource_limits,
                &NoAbort,
            )
            .unwrap();
            let directed_circuit = engine.build_circuit(&directed).unwrap();
            assert_eq!(
                directed_circuit.resistors.reported_resistances,
                circuit.resistors.reported_resistances
            );
            assert_eq!(directed.params.random().next_uniform(), next_draw);
            let current = AcSensitivityOutput::BranchCurrent("Vdrive".into());
            let dc = engine
                .run_output_sensitivity_with_abort(
                    &netlist, current, "p", 0.0, None, &mut 0, &NoAbort,
                )
                .unwrap();
            assert!((dc / -expected - 1.0).abs() < 2e-12);
        }
    }

    #[test]
    fn root_sensitivity_accepts_extreme_nonzero_parameter_divisors() {
        let engine = Engine::default();
        for expression in [
            "(1e-200+1e-208*p)/1e-200",
            "(1e200+1e192*p)/1e200",
            // sqrt(-1) is non-real; both components of the quotient matter.
            "abs((1e-200+sqrt(-1)*1e-200+1e-208*p)/(1e-200+sqrt(-1)*1e-200))",
        ] {
            let netlist = parse(
                &format!(
                    "Extreme quotient parameter\n.param p=0 gain={{{expression}}}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 gain\n.end"
                ),
                ExpressionDialect::Ngspice,
            );
            let output = AcSensitivityOutput::Voltage {
                positive: engine
                    .build_circuit(&netlist)
                    .unwrap()
                    .get_node_by_name("out")
                    .unwrap(),
                negative: None,
            };
            let expected = if expression.starts_with("abs") {
                0.5e-8
            } else {
                1e-8
            };
            for frequencies in [None, Some([1.0].as_slice())] {
                let mut runs = 0;
                let (nominal, derivative) = engine
                    .linear_parameter_sensitivity(
                        &netlist,
                        &output,
                        "p",
                        0.0,
                        frequencies,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(runs, 1);
                assert_eq!(nominal[0], Complex64::new(1.0, 0.0));
                assert!(
                    (derivative[0].re / expected - 1.0).abs() < 2e-14,
                    "{expression}: {:?}",
                    derivative
                );
                assert_eq!(derivative[0].im, 0.0);
            }
        }
    }

    #[test]
    fn deferred_resistor_sensitivity_preserves_binding_values_and_directions() {
        let cases = [
            ("", "1+1e-8*p", "", 1.0, 1e-8),
            (".param q={1e-8*p}", "1+q", ".param q={2e-8*p}", 1.0, 2e-8),
            (".func rval(x) {1+1e-8*x}", "rval(p)", "", 1.0, 1e-8),
            (".param z={sqrt(-1)}", "1+abs(z)*(2+1e-8*p)", "", 3.0, 1e-8),
            (".param TEMP={27+1e-8*p}", "1+TEMP", "", 28.0, 1e-8),
            // The eager parameter reader must still use its definition-time value.
            (".param r={1+1e-8*p}", "r", ".param r={7+9e-8*p}", 1.0, 1e-8),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let engine = Engine::default();
            for (before, expression, after, resistance, direction) in cases {
                let netlist = parse(
                    &format!(
                        "Deferred scalar resistor\n.param p=0\n{before}\nV1 in 0 DC 1 AC 1\nR1 in out {{{expression}}}\nR2 out 0 1\n{after}\n.end"
                    ),
                    dialect,
                );
                let circuit = engine.build_circuit(&netlist).unwrap();
                let r1 = circuit
                    .resistors
                    .names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("r1"))
                    .unwrap();
                assert_eq!(
                    circuit.resistors.reported_resistances[r1], resistance,
                    "{dialect:?}: {expression}"
                );
                let output = AcSensitivityOutput::Voltage {
                    positive: circuit.get_node_by_name("out").unwrap(),
                    negative: None,
                };
                let expected = -direction / (1.0 + resistance).powi(2);
                let mut runs = 0;
                let dc = engine
                    .run_output_sensitivity_with_abort(
                        &netlist,
                        output.clone(),
                        "p",
                        0.0,
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap();
                assert_eq!(runs, 1, "DC {expression}");
                assert!(
                    (dc / expected - 1.0).abs() < 2e-12,
                    "{dialect:?}: {expression}: DC {dc:e} != {expected:e}"
                );
                runs = 0;
                let ac = engine
                    .run_output_sensitivity_ac_with_abort(
                        &netlist,
                        output,
                        "p",
                        0.0,
                        &[1.0],
                        None,
                        &mut runs,
                        &NoAbort,
                    )
                    .unwrap()[0]
                    .value()
                    .unwrap();
                assert_eq!(runs, 1, "AC {expression}");
                assert!(
                    (ac / expected - 1.0).abs() < 2e-12,
                    "{dialect:?}: {expression}: AC {ac:e} != {expected:e}"
                );
            }
        }
    }

    #[test]
    fn deferred_resistor_directions_consume_each_random_draw_once() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for mode in [
                crate::netlist::StatisticalParamMode::Sample,
                crate::netlist::StatisticalParamMode::Nominal,
            ] {
                let netlist = Netlist::parse_with_options(
                    "Random resistor directions\n.param p=0\nV1 in 0 DC 1 AC 1\nR1 in out {aunif(2,0.25)*(1+1e-8*p)}\nR2 out 0 {aunif(3,0.5)+2e-8*p}\n.end",
                    crate::netlist::NetlistParseOptions {
                        expression_dialect: dialect,
                        statistical_mode: mode,
                        statistical_seed: Some(581),
                        ..Default::default()
                    },
                ).unwrap();
                let engine = Engine::default();
                let circuit = engine.build_circuit(&netlist).unwrap();
                let next_draw = netlist.params.random().next_uniform();
                let (directed, _) = Engine::replay_parameter_overrides(
                    &netlist,
                    &[("P".into(), 0.0)],
                    Some("p"),
                    engine.config.resource_limits,
                    &NoAbort,
                )
                .unwrap();
                let directed_circuit = engine.build_circuit(&directed).unwrap();
                assert_eq!(
                    directed_circuit.resistors.reported_resistances,
                    circuit.resistors.reported_resistances
                );
                assert_eq!(
                    directed.params.random().next_uniform(),
                    next_draw,
                    "{dialect:?} {mode:?}"
                );
                let resistance = |name: &str| {
                    let index = circuit
                        .resistors
                        .names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))
                        .unwrap();
                    circuit.resistors.reported_resistances[index]
                };
                let (r1, r2) = (resistance("r1"), resistance("r2"));
                let expected = (2e-8 * r1 - 1e-8 * r1 * r2) / (r1 + r2).powi(2);
                let output = AcSensitivityOutput::Voltage {
                    positive: circuit.get_node_by_name("out").unwrap(),
                    negative: None,
                };
                for frequencies in [None, Some([1.0].as_slice())] {
                    let mut runs = 0;
                    let (nominal, derivative) = engine
                        .linear_parameter_sensitivity(
                            &netlist,
                            &output,
                            "p",
                            0.0,
                            frequencies,
                            &mut runs,
                            &NoAbort,
                        )
                        .unwrap()
                        .unwrap();
                    assert_eq!(runs, 1);
                    assert!((nominal[0].re - r2 / (r1 + r2)).abs() < 2e-14);
                    assert!(
                        (derivative[0].re / expected - 1.0).abs() < 2e-12,
                        "{dialect:?} {mode:?}: {:?} != {expected:e}",
                        derivative
                    );
                }
            }
        }
    }

    #[test]
    fn linear_root_sensitivity_retains_extreme_scales_and_qualifies_coverage() {
        let engine = Engine::default();
        let netlist = parse(
            "Retained physical forcing\n.param p=0 q={1e200*p}\nV1 in 0 DC 1e-200 AC 1e-200\nE1 out 0 in 0 {1+1e200*q}\n.end",
            ExpressionDialect::Ngspice,
        );
        let output = AcSensitivityOutput::Voltage {
            positive: engine
                .build_circuit(&netlist)
                .unwrap()
                .get_node_by_name("out")
                .unwrap(),
            negative: None,
        };
        let result = engine
            .linear_parameter_sensitivity(
                &netlist,
                &output,
                "p",
                0.0,
                Some(&[1.0]),
                &mut 0,
                &NoAbort,
            )
            .unwrap()
            .unwrap();
        assert!(
            (result.1[0].re / 1e200 - 1.0).abs() < 2e-14,
            "{:?}",
            result.1
        );
        let dc = engine
            .linear_parameter_sensitivity(&netlist, &output, "p", 0.0, None, &mut 0, &NoAbort)
            .unwrap()
            .unwrap();
        assert!((dc.1[0].re / 1e200 - 1.0).abs() < 2e-14, "DC {:?}", dc.1);
        let netlist = parse(
            "Branch current direction\n.param p=0\nV1 out 0 DC {1+1e-8*p} AC {1+1e-8*p}\nR1 out 0 1\n.end",
            ExpressionDialect::Ngspice,
        );
        let output = AcSensitivityOutput::BranchCurrent("v1".into());
        let derivative = engine
            .run_output_sensitivity_with_abort(&netlist, output, "p", 0.0, None, &mut 0, &NoAbort)
            .unwrap();
        assert!((derivative / -1e-8 - 1.0).abs() < 2e-14);
        for expression in [
            "floor(p)",
            "ceil(p)",
            "round(p+0.5)",
            "int(p+1)",
            "uramp(p)",
            "limit(p,0,1)",
            "if(p>0,1,0)",
            "if(p,1,0)",
            "table(p,0,0,1,1)",
            "mod(p+1,1)",
        ] {
            let source = format!(
                "Nonsmooth field\n.param p=0\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {{1+{expression}}}\n.end"
            );
            let netlist = parse(&source, ExpressionDialect::Ngspice);
            let error = engine
                .run_sensitivity(&netlist, 2, "p", 0.0, None)
                .unwrap_err();
            assert!(
                error.to_string().contains("parameter derivative"),
                "{expression}: {error}"
            );
        }
        let netlist = parse(
            "Unused nonsmooth definition\n.param p=0 unused={abs(p)} alias=unused\nV1 out 0 DC 1 AC 1\n.end",
            ExpressionDialect::Ngspice,
        );
        assert_eq!(
            engine.run_sensitivity(&netlist, 1, "p", 0.0, None).unwrap(),
            0.0
        );
        assert_eq!(
            engine
                .run_sensitivity_ac(&netlist, 1, "p", 0.0, &[1.0], None)
                .unwrap(),
            vec![0.0]
        );
        let netlist = parse(
            "Used nonsmooth definition\n.param p=0 cusp={abs(p)} alias=cusp\nV1 out 0 DC {1+alias} AC 1\n.end",
            ExpressionDialect::Ngspice,
        );
        assert!(
            engine
                .run_sensitivity(&netlist, 1, "p", 0.0, None)
                .unwrap_err()
                .to_string()
                .contains("derivative")
        );
        // Missing field coverage and structural selectors cannot produce an analytic zero.
        for body in [
            "V1 out 0 1\nR1 out 0 1 M={1+p}",
            "V1 out 0 1\n.if (p == 0)\nR1 out 0 1\n.endif",
            ".subckt cell a\nR1 a 0 1\n.ends\nV1 out 0 1\nX1 out cell M={1+p}",
        ] {
            let netlist = parse(
                &format!("Qualification\n.param p=0\n{body}\n.end"),
                ExpressionDialect::Ngspice,
            );
            let mut runs = 0;
            assert!(
                engine
                    .linear_parameter_sensitivity(
                        &netlist,
                        &AcSensitivityOutput::Voltage {
                            positive: 1,
                            negative: None
                        },
                        "p",
                        0.0,
                        None,
                        &mut runs,
                        &NoAbort
                    )
                    .unwrap()
                    .is_none(),
                "{body}"
            );
            assert_eq!(runs, 0);
        }
    }
}
