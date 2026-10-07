//! Physical probe identities and signed/zero differential sensitivity outputs.
use rspice_core::analysis::{AcSensitivityOutput, SensitivityUnavailability};
use rspice_core::engine::SensitivityCardResult;
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, SignalUnit,
};
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

fn document(body: &str, probe: &str, sweep: &str) -> AnalysisResultDocument {
    let netlist = Netlist::parse(&format!("{body}.sens {probe} R1{sweep}\n.end\n")).unwrap();
    let id = AnalysisInstanceId::new(AnalysisKind::Sensitivity, 1);
    match Engine::default()
        .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
        .unwrap()
    {
        SensitivityCardResult::Dc(result) => {
            assert_eq!(
                result.output_unit,
                if probe.starts_with('I') {
                    SignalUnit::Ampere
                } else {
                    SignalUnit::Volt
                }
            );
            AnalysisResultDocument::from_sensitivity(id, &result).unwrap()
        }
        SensitivityCardResult::Ac(result) => {
            assert_eq!(
                result.output_unit,
                if probe.starts_with('I') {
                    SignalUnit::Ampere
                } else {
                    SignalUnit::Volt
                }
            );
            AnalysisResultDocument::from_ac_sensitivity(id, &result).unwrap()
        }
    }
    .build()
    .unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nominal_output_units_survive_voltage_and_current_documents() {
    for (probe, unit) in [
        ("V(out)", SignalUnit::Volt),
        ("V(0,out)", SignalUnit::Volt),
        ("I(V1)", SignalUnit::Ampere),
    ] {
        for sweep in ["", " AC LIN 2 1000 2000"] {
            let result = document(CIRCUIT, probe, sweep);
            if sweep.is_empty() {
                assert_eq!(result.scalars()[0].unit(), Some(&unit));
            } else {
                assert_eq!(result.signals()[0].descriptor().unit(), &unit);
            }
            assert_eq!(
                AnalysisResultDocument::from_json(&result.to_json().unwrap()).unwrap(),
                result
            );
        }
    }
    // The display spelling is not the authority for a physical unit.
    let result = rspice_core::analysis::SensitivityResult::new(
        "Voltage-looking label V(out)",
        2.0,
        SignalUnit::Ampere,
    );
    let result = AnalysisResultDocument::from_sensitivity(
        AnalysisInstanceId::new(AnalysisKind::Sensitivity, 1),
        &result,
    )
    .unwrap()
    .build()
    .unwrap();
    assert_eq!(result.scalars()[0].unit(), Some(&SignalUnit::Ampere));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn legacy_sensitivity_units_remain_unknown_without_changing_numbers() {
    for sweep in ["", " AC LIN 2 1000 2000"] {
        let original = document(CIRCUIT, "I(V1)", sweep);
        let mut raw = serde_json::to_value(&original).unwrap();
        let mut explicitly_typed_legacy = raw.clone();
        explicitly_typed_legacy["schemaVersion"] = 12.into();
        let preserved =
            AnalysisResultDocument::from_json(&explicitly_typed_legacy.to_string()).unwrap();
        assert_eq!(
            serde_json::to_value(preserved).unwrap(),
            explicitly_typed_legacy
        );
        let unit_path = if sweep.is_empty() {
            "/scalars/0/unit"
        } else {
            "/signals/0/descriptor/unit"
        };
        *raw.pointer_mut(unit_path).unwrap() = serde_json::json!({"unit": if sweep.is_empty() { "dimensionless" } else { "unspecified" }});
        for version in 5..=12 {
            raw["schemaVersion"] = version.into();
            let restored = AnalysisResultDocument::from_json(&raw.to_string()).unwrap();
            let mut expected = raw.clone();
            *expected.pointer_mut(unit_path).unwrap() = serde_json::json!({"unit":"unspecified"});
            assert_eq!(serde_json::to_value(&restored).unwrap(), expected);
            assert_eq!(
                AnalysisResultDocument::from_json(&restored.to_json().unwrap()).unwrap(),
                restored
            );
        }
        raw["schemaVersion"] = 13.into();
        assert!(
            AnalysisResultDocument::from_json(&raw.to_string())
                .unwrap_err()
                .to_string()
                .contains("sensitivity output unit")
        );
        raw["schemaVersion"] = 4.into();
        assert!(
            AnalysisResultDocument::from_json(&raw.to_string())
                .unwrap_err()
                .to_string()
                .contains("rerun")
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sensitivity_nominal_metadata_cannot_claim_invalid_or_missing_units() {
    for sweep in ["", " AC LIN 2 1000 2000"] {
        let original = serde_json::to_value(document(CIRCUIT, "V(out)", sweep)).unwrap();
        let unit_path = if sweep.is_empty() {
            "/scalars/0/unit"
        } else {
            "/signals/0/descriptor/unit"
        };
        for invalid in [
            serde_json::json!({"unit":"dimensionless"}),
            serde_json::json!({"unit":"unspecified"}),
            serde_json::json!({"unit":"ohm"}),
            serde_json::Value::Null,
        ] {
            let mut raw = original.clone();
            *raw.pointer_mut(unit_path).unwrap() = invalid;
            assert!(AnalysisResultDocument::from_json(&raw.to_string()).is_err());
        }
        let mut raw = original;
        raw[if sweep.is_empty() {
            "scalars"
        } else {
            "signals"
        }] = serde_json::json!([]);
        assert!(AnalysisResultDocument::from_json(&raw.to_string()).is_err());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn standalone_adjoint_retains_both_numeric_terminals_and_voltage_units() {
    use rspice_core::analysis::sensitivity::SensitivityAnalyzer;
    let mut dense =
        SensitivityAnalyzer::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![3.0, 1.0], vec![]);
    let sparse =
        SensitivityAnalyzer::with_precomputed_adjoint(vec![3.0, 1.0], vec![1.0, -1.0], vec![])
            .unwrap();
    for result in [
        dense.analyze(0, Some(1)).unwrap(),
        sparse.analyze_precomputed(0, Some(1)).unwrap(),
    ] {
        assert_eq!(result.output, "V(1,2)");
        assert_eq!(result.output_value, 2.0);
        assert_eq!(result.output_unit, SignalUnit::Volt);
    }
}
