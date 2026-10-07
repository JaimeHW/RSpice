//! Physical probe identities and signed/zero differential sensitivity outputs.
use rspice_core::analysis::{AcSensitivityOutput, SensitivityUnavailability};
use rspice_core::engine::SensitivityCardResult;
use rspice_core::{ComplexValue, Engine, Netlist, NoAbort};

const CIRCUIT: &str =
    "Sensitivity probes\n.param p=1k\nV1 in 0 DC 1 AC 1\nR1 in out {p}\nR2 out 0 1k\nC1 out 0 1n\n";

fn close(actual: ComplexValue, expected: ComplexValue) {
    assert!(
        (actual - expected).norm() <= 2e-8 * expected.norm().max(1e-12),
        "{actual} != {expected}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn authored_voltage_sensitivity_retains_physical_names_in_both_domains() {
    for (body, probe) in [
        (CIRCUIT.to_owned(), "V(out)"),
        ("Numeric nodes\nV1 20 10 DC 1 AC 1\nR0 10 0 1k\nR1 20 30 1k\nR2 30 10 1k\n".into(), "V(30,10)"),
        ("Hierarchy\nV1 in 0 DC 1 AC 1\nX1 in cell\n.subckt cell in\nR1 in out 1k\nR2 out 0 1k\n.ends\n".into(), "V(X1.out)"),
    ] {
        for sweep in ["", " AC LIN 2 1000 2000"] {
            let netlist = Netlist::parse(&format!("{body}.sens {probe} *R1 {sweep}\n.end\n")).unwrap();
            let result = Engine::default().run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort).unwrap();
            let actual = match result { SensitivityCardResult::Dc(result) => result.output, SensitivityCardResult::Ac(result) => result.output };
            assert!(actual.eq_ignore_ascii_case(probe), "{actual} != {probe}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn grounded_positive_sensitivity_has_the_signed_rc_derivative() {
    let omega = std::f64::consts::TAU * 1000.0;
    let denominator = ComplexValue::new(2000.0, omega * 1e-3);
    let derivative = -1000.0 * ComplexValue::new(1.0, omega * 1e-6) / denominator.powi(2);
    for filter in ["R1", "PARAM:p"] {
        for sweep in ["", " AC LIN 1 1000 1000"] {
            let netlist =
                Netlist::parse(&format!("{CIRCUIT}.sens V(0,out) {filter}{sweep}\n.end\n"))
                    .unwrap();
            match Engine::default()
                .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
                .unwrap()
            {
                SensitivityCardResult::Dc(result) => {
                    assert!(result.output.eq_ignore_ascii_case("V(0,out)"));
                    close(result.output_value.into(), (-0.5).into());
                    close(result.sensitivities[0].absolute.into(), 0.00025.into());
                    close(
                        result.sensitivities[0].normalized.value().unwrap().into(),
                        (-0.5).into(),
                    );
                }
                SensitivityCardResult::Ac(result) => {
                    assert!(result.output.eq_ignore_ascii_case("V(0,out)"));
                    close(result.output_values[0], -1000.0 / denominator);
                    close(result.sensitivities[0].absolute[0], -derivative);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_probes_retain_zero_derivatives_and_undefined_normalization() {
    for probe in ["V(0)", "V(out,out)"] {
        for filter in ["R1", "PARAM:p"] {
            for sweep in ["", " AC LIN 1 1000 1000"] {
                let netlist =
                    Netlist::parse(&format!("{CIRCUIT}.sens {probe} {filter}{sweep}\n.end\n"))
                        .unwrap();
                match Engine::default()
                    .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
                    .unwrap()
                {
                    SensitivityCardResult::Dc(result) => {
                        assert_eq!(result.output_value, 0.0);
                        assert_eq!(result.sensitivities[0].absolute, 0.0);
                        assert_eq!(
                            result.sensitivities[0].normalized.reason(),
                            Some(SensitivityUnavailability::ZeroOutput)
                        );
                    }
                    SensitivityCardResult::Ac(result) => {
                        assert_eq!(result.output_values[0], ComplexValue::ZERO);
                        let trace = &result.sensitivities[0];
                        assert_eq!(trace.absolute[0], ComplexValue::ZERO);
                        assert_eq!(
                            trace.normalized[0].reason(),
                            Some(SensitivityUnavailability::ZeroOutput)
                        );
                        assert_eq!(
                            trace.phase[0].reason(),
                            Some(SensitivityUnavailability::ZeroOutput)
                        );
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sparse_adjoint_accepts_ground_and_reports_the_same_physical_probe() {
    let netlist = Netlist::parse(&format!("{CIRCUIT}.end\n")).unwrap();
    let engine = Engine::default();
    for (positive, negative, name, expected) in [
        (2, None, "V(out)", -0.00025),
        (0, Some(2), "V(0,out)", 0.00025),
        (0, None, "V(0)", 0.0),
        (2, Some(2), "V(out,out)", 0.0),
    ] {
        let result = engine
            .run_sensitivity_linearized(&netlist, positive, negative)
            .unwrap();
        assert!(
            result.output.eq_ignore_ascii_case(name),
            "{} != {name}",
            result.output
        );
        close(result.get("R1").unwrap().absolute.into(), expected.into());
    }
    for output in [
        AcSensitivityOutput::Voltage {
            positive: 99,
            negative: None,
        },
        AcSensitivityOutput::Voltage {
            positive: 0,
            negative: Some(99),
        },
    ] {
        assert!(
            engine
                .run_sensitivity_dc_complete(&netlist, output.clone(), &["R1".into()])
                .is_err()
        );
        assert!(
            engine
                .run_sensitivity_ac_complete(&netlist, output, &[1000.0], &["R1".into()])
                .is_err()
        );
    }
}
