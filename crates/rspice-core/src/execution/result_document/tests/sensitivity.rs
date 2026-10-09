use super::*;
use crate::analysis::sensitivity::{
    AcSensitivity, AcSensitivityResult, SensitivityUnavailability as Reason,
    SensitivityValue as Sample,
};
use serde_json::{Value, json};

fn ac(
    nominal: f64,
    output: Complex64,
    absolute: Complex64,
    derived: (Sample<Complex64>, Sample<f64>, Sample<f64>),
) -> AcSensitivityResult {
    AcSensitivityResult {
        output: "V(out)".into(),
        output_unit: SignalUnit::Volt,
        frequencies: vec![1000.0],
        output_values: vec![output],
        sensitivities: vec![AcSensitivity {
            vector_name: "R1".into(),
            element: "R1".into(),
            element_type: ElementType::Resistor,
            parameter: "R".into(),
            nominal_value: nominal,
            absolute: vec![absolute],
            normalized: vec![derived.0],
            magnitude: vec![derived.1],
            phase: vec![derived.2],
        }],
    }
}

fn document(result: &AcSensitivityResult) -> AnalysisResultDocument {
    AnalysisResultDocument::from_ac_sensitivity(instance(AnalysisKind::Sensitivity), result)
        .unwrap()
        .build()
        .unwrap()
}

#[test]
fn dc_sensitivity_rejects_contradictory_normalized_values_and_duplicate_vectors() {
    let document = document_for(AnalysisResultKind::Sensitivity);
    let original: Value = serde_json::from_str(&document.to_json().unwrap()).unwrap();
    for case in [
        "normalized",
        "zero",
        "tiny",
        "range",
        "nominal",
        "absolute",
        "output",
        "duplicate",
    ] {
        let mut raw = original.clone();
        let entry = &mut raw["payload"]["entries"][0];
        match case {
            "normalized" => entry["normalized"] = json!(123.0),
            "zero" => entry["normalized"] = json!(0.0),
            "tiny" => entry["normalized"] = json!(1e-300),
            "range" => entry["normalized"] = json!({"unavailable":"out-of-range"}),
            "nominal" => entry["nominalValue"] = json!(2000.0),
            "absolute" => entry["absolute"] = json!(1.0),
            "output" => raw["scalars"][0]["value"]["value"] = json!(1.0),
            "duplicate" => {
                let mut second = entry.clone();
                second["vectorName"] = json!("r1");
                raw["payload"]["entries"]
                    .as_array_mut()
                    .unwrap()
                    .push(second);
            }
            _ => panic!("unknown case"),
        }
        assert!(
            AnalysisResultDocument::from_json(&raw.to_string()).is_err(),
            "{case}"
        );
    }
    let mut source = sensitivity_result();
    source.sensitivities[0].normalized = Sample::Available(123.0);
    assert!(
        AnalysisResultDocument::from_sensitivity(instance(AnalysisKind::Sensitivity), &source)
            .unwrap()
            .build()
            .is_err()
    );
}

#[test]
fn ac_sensitivity_checks_each_derived_component_against_the_retained_output() {
    // (2+i)/(3+4i) = 0.4-0.2i; d|output| = (6+4)/5 = 2.
    let result = ac(
        10.0,
        Complex64::new(3.0, 4.0),
        Complex64::new(2.0, 1.0),
        (
            Sample::Available(Complex64::new(4.0, -2.0)),
            Sample::Available(2.0),
            Sample::Available(-0.2),
        ),
    );
    let original: Value = serde_json::from_str(&document(&result).to_json().unwrap()).unwrap();
    for case in [
        "normalized-real",
        "normalized-imag",
        "magnitude",
        "phase",
        "normalized-range",
        "magnitude-range",
        "phase-range",
        "nominal",
        "absolute",
        "output",
        "missing-output",
        "duplicate",
    ] {
        let mut raw = original.clone();
        let entry = &mut raw["payload"]["acEntries"][0];
        match case {
            "normalized-real" => entry["normalized"][0]["real"] = json!(123.0),
            "normalized-imag" => entry["normalized"][0]["imaginary"] = json!(2.0),
            "magnitude" => entry["magnitude"][0] = json!(-2.0),
            "phase" => entry["phase"][0] = json!(0.0),
            "normalized-range" | "magnitude-range" | "phase-range" => {
                entry[case.strip_suffix("-range").unwrap()][0] =
                    json!({"unavailable":"out-of-range"})
            }
            "nominal" => entry["nominalValue"] = json!(20.0),
            "absolute" => entry["absolute"][0]["real"] = json!(1.0),
            "output" => raw["signals"][0]["values"]["samples"][0]["real"] = json!(2.0),
            "missing-output" => raw["signals"][0]["values"]["samples"][0] = Value::Null,
            "duplicate" => {
                let mut second = entry.clone();
                second["vectorName"] = json!("r1");
                raw["payload"]["acEntries"]
                    .as_array_mut()
                    .unwrap()
                    .push(second);
            }
            _ => panic!("unknown case"),
        }
        assert!(
            AnalysisResultDocument::from_json(&raw.to_string()).is_err(),
            "{case}"
        );
    }
    let mut bad = result;
    bad.sensitivities[0].phase[0] = Sample::Available(123.0);
    assert!(
        AnalysisResultDocument::from_ac_sensitivity(instance(AnalysisKind::Sensitivity), &bad)
            .unwrap()
            .build()
            .is_err()
    );
}

#[test]
fn ac_sensitivity_keeps_mathematical_absence_and_extreme_finite_projections() {
    for result in [
        ac(
            1.0,
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            (
                Sample::unavailable(Reason::ZeroOutput),
                Sample::Available(0.0),
                Sample::unavailable(Reason::ZeroOutput),
            ),
        ),
        ac(
            1.0,
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
            (
                Sample::unavailable(Reason::ZeroOutput),
                Sample::unavailable(Reason::NondifferentiableMagnitude),
                Sample::unavailable(Reason::ZeroOutput),
            ),
        ),
        ac(
            1e200,
            Complex64::new(1e-200, 0.0),
            Complex64::new(1e200, 1e200),
            (
                Sample::unavailable(Reason::OutOfRange),
                Sample::Available(1e200),
                Sample::unavailable(Reason::OutOfRange),
            ),
        ),
        ac(
            1e-200,
            Complex64::new(1e200, 0.0),
            Complex64::new(1e-200, 1e-200),
            (
                Sample::unavailable(Reason::OutOfRange),
                Sample::Available(1e-200),
                Sample::unavailable(Reason::OutOfRange),
            ),
        ),
        ac(
            0.0,
            Complex64::new(1e-200, 0.0),
            Complex64::new(1e200, 1e200),
            (
                Sample::Available(Complex64::new(0.0, 0.0)),
                Sample::Available(1e200),
                Sample::unavailable(Reason::OutOfRange),
            ),
        ),
        ac(
            1.0,
            Complex64::new(1e300, 1e300),
            Complex64::new(1e300, 1e300),
            (
                Sample::Available(Complex64::new(1.0, 0.0)),
                Sample::Available(std::f64::consts::SQRT_2 * 1e300),
                Sample::Available(0.0),
            ),
        ),
    ] {
        let document = document(&result);
        assert_eq!(
            AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
            document
        );
        assert!(matches!(
            document.validate_with_abort(&ImmediateAbort),
            Err(ResultDocumentError::Aborted)
        ));
    }
}

#[test]
fn sensitivity_consistency_tolerance_has_no_absolute_floor() {
    for normalized in [1e-300, 1e300, f64::from_bits(1)] {
        let document = AnalysisResultDocument::from_parameter_sensitivity(
            instance(AnalysisKind::Sensitivity),
            "V(out)",
            1.0,
            SignalUnit::Volt,
            "p",
            1.0,
            normalized,
        )
        .unwrap()
        .build()
        .unwrap();
        let mut raw: Value = serde_json::from_str(&document.to_json().unwrap()).unwrap();
        raw["payload"]["entries"][0]["normalized"] = json!(0.0);
        assert!(AnalysisResultDocument::from_json(&raw.to_string()).is_err());
        if normalized != f64::from_bits(1) {
            raw["payload"]["entries"][0]["normalized"] = json!(normalized.next_up());
            assert!(AnalysisResultDocument::from_json(&raw.to_string()).is_ok());
            raw["payload"]["entries"][0]["normalized"] = json!(normalized * 1.01);
            assert!(AnalysisResultDocument::from_json(&raw.to_string()).is_err());
        }
    }
}
