//! Root-coordinate derivatives of qualified linear physical equations.
//! Field directions come from the actual parser binding sites, not finite probes.

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
        }
        | ElementKind::Capacitor {
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
        } => transconductance_expr.is_none() && !multiplicity.given,
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
        (ElementKind::Resistor { value, .. }, ElementParameterDirection::Passive(direction)) => {
            // R*x is the voltage equation when this resistor owns a current unknown.
            if circuit.get_branch_by_name(&element.name).is_some() {
                let row = branch_row(circuit, &element.name)?;
                rhs[row] = rhs[row] + times_phasor(direction, solution[row]);
            } else {
                inject(times_phasor(
                    -direction / *value / *value,
                    voltage(solution, terminals),
                ));
            }
        }
        (ElementKind::Capacitor { .. }, ElementParameterDirection::Passive(direction)) => {
            if ac {
                let value = voltage(solution, terminals);
                inject(times_phasor(
                    direction * (2.0 * std::f64::consts::PI) * frequency,
                    Complex64::new(-value.im, value.re),
                ));
            }
        }
        (ElementKind::Inductor { .. }, ElementParameterDirection::Passive(direction)) => {
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
            || !netlist
                .elements
                .iter()
                .all(|element| linear_element(&element.kind))
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
        if capture.has_conditionals
            || !directed.elements.iter().all(|element| {
                linear_element(&element.kind) && capture.elements.contains_key(&element.name)
            })
        {
            return Ok(None);
        }
        *runs = runs.saturating_add(1);
        self.ensure_batch_runs(*runs)?;
        let engine = self.resolved_for_netlist(&directed);
        let run_scope = crate::abort_signal::ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        Self::ensure_model_run_active(abort)?;
        let mut prepared = engine.prepare_ac_analysis(&directed, abort)?;
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
            for (index, element) in directed.elements.iter().enumerate() {
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
                    .unwrap()[0];
                assert_eq!(runs, 1, "must use one nominal sweep: {body}");
                assert!(
                    (ac - expected_ac).abs() <= expected_ac.abs() * 2e-12,
                    "{dialect:?}: {body}: AC {ac:e} != {expected_ac:e}"
                );
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
            "V1 in 0 1\nG1 out 0 in 0 1 M={1+p}\nR1 out 0 1",
            "V1 out 0 1\n.if (p == 0)\nR1 out 0 1\n.endif",
            ".subckt cell a\nR1 a 0 1\n.ends\nV1 out 0 1\nX1 out cell",
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
