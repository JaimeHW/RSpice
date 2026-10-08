use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};

fn fixture() -> serde_json::Value {
    let points = [AcResult {
        frequency: 10.0,
        node_names: vec!["out".into()],
        branch_names: vec![],
        voltages: vec![Complex64::new(1.0, 0.0)],
        currents: vec![],
    }];
    let document =
        AnalysisResultDocument::from_ac(AnalysisInstanceId::new(AnalysisKind::Ac, 0), &points)
            .unwrap()
            .build()
            .unwrap();
    serde_json::to_value(document).unwrap()
}

fn substituted(pointer: &str, literal: &str) -> String {
    let mut value = fixture();
    *value.pointer_mut(pointer).unwrap() = serde_json::json!("NUMERIC_TOKEN");
    serde_json::to_string(&value)
        .unwrap()
        .replace("\"NUMERIC_TOKEN\"", literal)
}

#[test]
fn typed_coordinates_and_complex_components_refuse_numeric_information_loss() {
    for pointer in [
        "/axes/0/values/values/0",
        "/signals/0/values/samples/0/real",
        "/signals/0/values/samples/0/imaginary",
    ] {
        for (literal, diagnostic) in [
            ("1e-999", "underflow"),
            ("-1e-999", "underflow"),
            ("9007199254740993", "cannot be represented exactly"),
            ("18446744073709551617", "cannot be represented exactly"),
        ] {
            let content = substituted(pointer, literal);
            let error = AnalysisResultDocument::from_json(&content).unwrap_err();
            assert!(
                error.to_string().contains(diagnostic),
                "{pointer}/{literal}: {error}"
            );
        }
    }
}

#[test]
fn typed_json_preserves_signed_zero_subnormals_and_exact_large_integers() {
    for (literal, expected) in [
        ("-0e-999", -0.0_f64),
        ("5e-324", 5e-324_f64),
        ("-5e-324", -5e-324_f64),
        ("9007199254740994", 9007199254740994.0_f64),
        ("18446744073709551616", 18446744073709551616.0_f64),
    ] {
        let content = substituted("/signals/0/values/samples/0/real", literal);
        let document = AnalysisResultDocument::from_json(&content).unwrap();
        let actual = serde_json::to_value(document).unwrap();
        assert_eq!(
            actual["signals"][0]["values"]["samples"][0]["real"]
                .as_f64()
                .unwrap()
                .to_bits(),
            expected.to_bits()
        );
    }
}
