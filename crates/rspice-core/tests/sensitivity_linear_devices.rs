//! Independent physical oracles for native R, L and C field derivatives.
use rspice_core::engine::SensitivityCardResult;
use rspice_core::resource::ResourceKind;
use rspice_core::{ComplexValue, Engine, Netlist, NoAbort, SimulationConfig, SimulationError};

fn run(body: &str, probe: &str, filters: &str, sweep: &str) -> SensitivityCardResult {
    let netlist = Netlist::parse(&format!("{body}.sens {probe} {filters}{sweep}\n.end\n")).unwrap();
    let card = netlist
        .analyses
        .iter()
        .find(|card| {
            matches!(
                card,
                rspice_core::netlist::AnalysisCommand::Sensitivity { .. }
            )
        })
        .unwrap();
    let mut config = SimulationConfig::default();
    // Closed forms describe the authored circuit with no conditioning shunts.
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    Engine::try_new(config)
        .unwrap()
        .run_sensitivity_from_card_with_abort(&netlist, card, &NoAbort)
        .unwrap()
}

fn close(actual: ComplexValue, expected: ComplexValue) {
    if expected == ComplexValue::ZERO {
        assert_eq!(actual, expected);
    } else {
        assert!(
            ((actual - expected) / expected).norm() < 2e-9,
            "{actual} != {expected}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_reference_resistor_has_zero_differential_sensitivity() {
    let body = "Floating divider\nV1 20 10 DC 1 AC 1\nR0 10 0 1k\nR1 20 30 1k\nR2 30 10 1k\n";
    for sweep in ["", " AC LIN 2 1 1000"] {
        match run(body, "V(30,10)", "R0", sweep) {
            SensitivityCardResult::Dc(result) => {
                close(result.output_value.into(), 0.5.into());
                close(
                    result.get("R0").unwrap().absolute.into(),
                    ComplexValue::ZERO,
                );
            }
            SensitivityCardResult::Ac(result) => {
                for (&output, &derivative) in result
                    .output_values
                    .iter()
                    .zip(&result.get("R0").unwrap().absolute)
                {
                    close(output, 0.5.into());
                    close(derivative, ComplexValue::ZERO);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_small_physical_derivative_is_retained_below_probe_roundoff() {
    // d(Rweak || 1)/dRweak = 1/(Rweak+1)^2, about 1e-20 V/ohm.
    let body = "Weak resistor\nI1 0 out DC 1 AC 1\nRweak out 0 1e10\nRload out 0 1\n";
    let expected = 1.0 / (1e10_f64 + 1.0).powi(2);
    for sweep in ["", " AC LIN 2 1 1000"] {
        match run(body, "V(out)", "Rweak", sweep) {
            SensitivityCardResult::Dc(result) => {
                close(
                    result.get("Rweak").unwrap().absolute.into(),
                    expected.into(),
                );
            }
            SensitivityCardResult::Ac(result) => {
                for &derivative in &result.get("Rweak").unwrap().absolute {
                    close(derivative, expected.into());
                }
            }
        }
    }
}

fn rlc_response(frequency: f64, current: bool) -> (ComplexValue, [ComplexValue; 3]) {
    let jw = ComplexValue::new(0.0, std::f64::consts::TAU * frequency);
    let series = 100.0 + jw * 0.01;
    let load = 1e-6 + jw * 1e-6;
    let denominator = 1.0 + series * load;
    if current {
        (
            -load / denominator,
            [
                load * load / denominator.powi(2),
                jw * load * load / denominator.powi(2),
                -jw / denominator.powi(2),
            ],
        )
    } else {
        (
            1.0 / denominator,
            [
                -load / denominator.powi(2),
                -jw * load / denominator.powi(2),
                -jw * series / denominator.powi(2),
            ],
        )
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rlc_voltage_and_branch_current_follow_their_closed_form_derivatives() {
    let body = "RLC derivatives\nV1 in 0 DC 1 AC 1\nR1 in mid 100\nL1 mid out 10m\nC1 out 0 1u\nRload out 0 1meg\n";
    for (probe, current) in [("V(out)", false), ("I(V1)", true)] {
        let SensitivityCardResult::Dc(dc) = run(body, probe, "R1 L1 C1", "") else {
            panic!("expected DC")
        };
        let (value, derivatives) = rlc_response(0.0, current);
        close(dc.output_value.into(), value);
        for (name, expected) in ["R1", "L1", "C1"].into_iter().zip(derivatives) {
            close(dc.get(name).unwrap().absolute.into(), expected);
        }
        for frequency in [0.25, 123.0, 10000.0] {
            let SensitivityCardResult::Ac(ac) = run(
                body,
                probe,
                "R1 L1 C1",
                &format!(" AC LIN 1 {frequency} {frequency}"),
            ) else {
                panic!("expected AC")
            };
            let (value, derivatives) = rlc_response(frequency, current);
            close(ac.output_values[0], value);
            for (name, expected) in ["R1", "L1", "C1"].into_iter().zip(derivatives) {
                close(ac.get(name).unwrap().absolute[0], expected);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn branch_form_resistance_has_the_same_physical_derivative() {
    // The explicit storage threshold creates the resistor's current unknown:
    // I(R1)=1/R1 and dI(R1)/dR1=-1/R1^2, with its constitutive equation retained.
    let body =
        "Branch resistor\nV1 in 0 DC 1 AC 1\nR1 in 0 1k\n.options device zeroresistancetol=1k\n";
    for sweep in ["", " AC LIN 2 1 1000"] {
        match run(body, "I(R1)", "R1", sweep) {
            SensitivityCardResult::Dc(result) => {
                close(result.output_value.into(), 1e-3.into());
                close(result.get("R1").unwrap().absolute.into(), (-1e-6).into());
            }
            SensitivityCardResult::Ac(result) => {
                for (&output, &derivative) in result
                    .output_values
                    .iter()
                    .zip(&result.get("R1").unwrap().absolute)
                {
                    close(output, 1e-3.into());
                    close(derivative, (-1e-6).into());
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn temperature_dependent_resistors_keep_the_physical_field_scaling() {
    let body = "Temperature derivative\n.temp 127\n.options tnom=27\nV1 in 0 DC 1 AC 1\nR1 in out 1k tc1=0.001\nRload out 0 1k\n";
    let expected = -1.1 * 1000.0 / 2100.0_f64.powi(2);
    for sweep in ["", " AC LIN 1 1 1"] {
        match run(body, "V(out)", "R1:R", sweep) {
            SensitivityCardResult::Dc(result) => {
                close(result.sensitivities[0].absolute.into(), expected.into())
            }
            SensitivityCardResult::Ac(result) => {
                close(result.sensitivities[0].absolute[0], expected.into())
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn analytic_device_solves_consume_the_shared_run_budget() {
    for sweep in ["", " AC LIN 1 1 1"] {
        let netlist = Netlist::parse(&format!(
            "Run admission\nI1 0 out DC 1 AC 1\nR1 out 0 1\n.sens V(out) R1{sweep}\n.end\n"
        ))
        .unwrap();
        let mut config = SimulationConfig::default();
        config.resource_limits.max_batch_runs = 1;
        let error = Engine::try_new(config)
            .unwrap()
            .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap_err();
        assert!(
            matches!(error, SimulationError::ResourceLimit(limit) if limit.resource == ResourceKind::BatchRuns && limit.requested == 2)
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn terminal_meter_sensitivity_follows_its_materialized_connections() {
    use rspice_core::netlist::TerminalCurrentProbe;
    for sweep in ["", " AC LIN 1 1 1"] {
        let mut netlist = Netlist::parse(&format!("Meter derivative\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nRload out 0 1k\n.sens I(VMETER) R1{sweep}\n.end\n")).unwrap();
        netlist.add_terminal_current_probe(TerminalCurrentProbe {
            device: "R1".into(),
            terminal: 0,
            source_name: "VMETER".into(),
            node_name: "meter_private".into(),
        });
        let result = Engine::default()
            .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap();
        match result {
            SensitivityCardResult::Dc(result) => {
                close(result.get("R1").unwrap().absolute.into(), (-2.5e-7).into())
            }
            SensitivityCardResult::Ac(result) => {
                close(result.get("R1").unwrap().absolute[0], (-2.5e-7).into())
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn reduced_topology_is_not_differentiated_as_an_unreduced_device() {
    for sweep in ["", " AC LIN 1 1 1"] {
        let mut netlist = Netlist::parse(&format!("Reduced derivative\nV1 in 0 DC 1 AC 1\nR1 in out 1e-110\nRload out 0 1k\n.sens I(V1) R1{sweep}\n.end\n")).unwrap();
        netlist.options.topology_supernode = Some(true);
        netlist.options.device_zero_resistance_tol = Some(1e-100);
        let result = Engine::default()
            .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap();
        // Every accepted perturbation remains inside the explicitly reduced
        // topology: this field has no remaining equation in that circuit.
        match result {
            SensitivityCardResult::Dc(result) => close(
                result.get("R1").unwrap().absolute.into(),
                ComplexValue::ZERO,
            ),
            SensitivityCardResult::Ac(result) => {
                close(result.get("R1").unwrap().absolute[0], ComplexValue::ZERO)
            }
        }
    }
}
