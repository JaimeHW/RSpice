use super::*;
use std::collections::HashMap;

fn unit(symbol: &str) -> MeasurementUnit {
    MeasurementUnit::known(symbol).unwrap()
}

fn statements(cards: &str) -> Vec<crate::netlist::measure::MeasureStatement> {
    crate::Netlist::parse(&format!(
        "Measurement units\nV1 out 0 1\nR1 out 0 1k\n{cards}\n.end\n"
    ))
    .unwrap()
    .measurements
}

#[test]
fn measurement_unit_conversions_keep_dimensions_prefixes_and_offsets() {
    for (source, value, target, expected) in [
        ("V", 0.25, "mV", 250.0),
        ("A", 2e-6, "µA", 2.0),
        ("ohm", 1e6, "MΩ", 1.0),
        ("ohm", 1e-3, "mohm", 1.0),
        ("V^2/Hz", 1e-12, "(uV)^2/Hz", 1.0),
        ("V/sqrt(Hz)", 1e-9, "nV/sqrt(Hz)", 1.0),
        ("A*s", 1e-9, "nC", 1.0),
        ("V/A", 1000.0, "kohm", 1.0),
        ("V/s", 1e6, "V/us", 1.0),
        ("1", 0.5, "%", 50.0),
        ("rad", std::f64::consts::PI, "deg", 180.0),
        ("degC", 0.0, "K", 273.15),
        ("K", 300.0, "°C", 26.85),
    ] {
        let actual = unit(source).convert_value(value, target).unwrap();
        assert!(
            (actual - expected).abs() < 1e-12 * expected.abs().max(1.0),
            "{source} -> {target}: {actual}"
        );
    }
    assert!(unit("V").convert_value(1.0, "A").is_err());
    assert_eq!(unit("V").convert_value(0.35, "mV").unwrap(), 350.0);
    assert!(unit("dB").convert_value(1.0, "1").is_err());
    assert!(MeasurementUnit::Unknown.convert_value(1.0, "1").is_err());
    assert!(unit("V").convert_value(f64::INFINITY, "V").is_err());
    for invalid in [
        "",
        "mystery",
        "V/A junk",
        "V^0.3",
        "degC/s",
        "V^999",
        "V\n",
        &"(".repeat(257),
    ] {
        assert!(MeasurementUnit::known(invalid).is_err(), "{invalid:?}");
    }
    let serialized = serde_json::to_string(&unit("V/sqrt(Hz)")).unwrap();
    let restored: MeasurementUnit = serde_json::from_str(&serialized).unwrap();
    restored.validate().unwrap();
    assert_eq!(restored, unit("V/sqrt(Hz)"));
}

#[test]
fn measurement_units_follow_reductions_expressions_and_axis_projection() {
    let statements = statements(
        ".meas tran mean AVG V(out)\n\
         .meas tran charge INTEG I(V1)\n\
         .meas tran slope DERIV V(out) AT=1\n\
         .meas tran location MAX V(out) OUTPUT=TIME\n\
         .meas tran energy PARAM='mean*charge'\n\
         .meas tran ratio PARAM='mean/mean'\n\
         .meas tran elapsed PARAM='TIME'\n\
         .meas tran unknowable PARAM='mean*undeclared'\n\
         .meas tran loop_a PARAM='loop_b'\n\
         .meas tran loop_b PARAM='loop_a'",
    );
    let refs = statements.iter().collect::<Vec<_>>();
    let units = measurement_units(&refs, None, &HashMap::new());
    for (index, symbol) in ["V", "C", "V/s", "s", "J", "1", "s"].iter().enumerate() {
        assert_eq!(
            units[index].value.convert_value(1.0, symbol).unwrap(),
            1.0,
            "{index}: {:?}",
            units[index]
        );
    }
    assert_eq!(units[3].raw_value, unit("V"));
    assert_eq!(units[3].axis, unit("s"));
    assert!(
        units[7..]
            .iter()
            .all(|units| units.value == MeasurementUnit::Unknown)
    );

    // Compare with real evaluator output, including the pre-projection value.
    let mut engine = crate::MeasureEngine::new();
    for statement in statements.into_iter().take(7) {
        engine.add(statement);
    }
    let voltage = [0.0, 0.5, 1.0];
    let current = [0.001; 3];
    let signals = HashMap::from([
        ("V(OUT)".to_owned(), voltage.as_slice()),
        ("I(V1)".to_owned(), current.as_slice()),
    ]);
    let results = engine.evaluate(&[0.0, 1.0, 2.0], &signals);
    assert_eq!(
        results
            .iter()
            .map(|result| result.units.as_ref().unwrap())
            .collect::<Vec<_>>(),
        units[..7].iter().collect::<Vec<_>>()
    );
    assert_eq!(
        units[0]
            .value
            .convert_value(results[0].value.unwrap(), "mV")
            .unwrap(),
        500.0
    );
    assert_eq!(
        units[1]
            .value
            .convert_value(results[1].value.unwrap(), "mC")
            .unwrap(),
        2.0
    );
    assert_eq!(
        units[2]
            .value
            .convert_value(results[2].value.unwrap(), "mV/s")
            .unwrap(),
        500.0
    );
    assert_eq!(
        units[3]
            .value
            .convert_value(results[3].value.unwrap(), "ms")
            .unwrap(),
        2000.0
    );
    assert_eq!(
        units[3]
            .raw_value
            .convert_value(results[3].raw_value.unwrap(), "mV")
            .unwrap(),
        1000.0
    );
}

#[test]
fn measurement_units_use_producer_axis_and_noise_quantity() {
    let cards = statements(
        ".meas dc area INTEG I(V1)\n.meas dc slope DERIV V(out) AT=1\n.meas dc crossing WHEN V(out)=1",
    );
    let refs = cards.iter().collect::<Vec<_>>();
    let inferred = measurement_units(&refs, Some(&unit("V")), &HashMap::new());
    for (actual, expected) in inferred.iter().zip(["W", "1", "V"]) {
        assert_eq!(actual.value.convert_value(1.0, expected).unwrap(), 1.0);
    }
    assert_eq!(
        measurement_units(&refs, None, &HashMap::new())[0].value,
        MeasurementUnit::Unknown
    );
    let temperature = measurement_units(&refs, Some(&unit("degC")), &HashMap::new());
    assert_eq!(temperature[1].value.convert_value(1.0, "V/K").unwrap(), 1.0);
    assert_eq!(
        temperature[2].value.convert_value(0.0, "K").unwrap(),
        273.15
    );

    let cards = statements(
        ".meas noise density FIND ONOISE AT=1\n.meas noise total INTEG INOISE\n.meas noise rms PARAM='sqrt(DNI(R1))'\n.meas ac phase FIND VP(out) AT=1\n.meas ac gain FIND VDB(out) AT=1",
    );
    let refs = cards.iter().collect::<Vec<_>>();
    let inferred = measurement_units(
        &refs,
        None,
        &HashMap::from([
            ("ONOISE".into(), unit("V^2/Hz")),
            ("INOISE".into(), unit("A^2/Hz")),
            ("DNI(R1)".into(), unit("A^2/Hz")),
        ]),
    );
    for (actual, expected) in inferred
        .iter()
        .zip(["V^2/Hz", "A^2", "A/sqrt(Hz)", "deg", "dB"])
    {
        assert_eq!(
            actual.value.convert_value(1.0, expected).unwrap(),
            1.0,
            "{actual:?}"
        );
    }
}
