use super::*;

fn evaluate(builder: &MeasurementBuilder, values: &[f64; 5]) -> rspice_core::MeasureResult {
    let card = builder.card("metric").unwrap();
    let source = format!(
        "Builder\n.param first=1 last=3 sample=2.5\nV1 out 0 1\nR1 out 0 1k\n{card}\n.end\n"
    );
    let netlist =
        rspice_core::Netlist::parse(&source).unwrap_or_else(|error| panic!("{card}: {error}"));
    let mut engine = rspice_core::analysis::MeasureEngine::new();
    for measurement in netlist.measurements {
        engine.add(measurement);
    }
    let signals = std::collections::HashMap::from([("V(out)".to_owned(), values.as_slice())]);
    engine
        .evaluate(&[0.0, 1.0, 2.0, 3.0, 4.0], &signals)
        .remove(0)
}

fn expect(builder: &MeasurementBuilder, values: &[f64; 5], expected: f64) {
    let result = evaluate(builder, values);
    assert!(result.passed, "{result:?}");
    assert!(
        (result.value.unwrap() - expected).abs() < 1e-12,
        "{result:?}; expected {expected}"
    );
}

#[test]
fn authored_plan_measurements_builder_executes_reductions_and_window_parameters() {
    let mut builder = MeasurementBuilder {
        from: "{first}".into(),
        to: "{last}".into(),
        ..Default::default()
    };
    for (operation, expected) in [
        ("AVG", 2.5),
        ("RMS", 2.5),
        ("MIN", 2.5),
        ("MAX", 2.5),
        ("PP", 0.0),
        ("INTEG", 5.0),
    ] {
        builder.operation = operation;
        expect(&builder, &[2.5; 5], expected);
    }
    builder.operation = "MIN_AT";
    expect(&builder, &[0.0, 1.0, 2.0, 3.0, 4.0], 1.0);
    builder.operation = "MAX_AT";
    expect(&builder, &[0.0, 1.0, 2.0, 3.0, 4.0], 3.0);
    builder.operation = "PARAM";
    builder.expression = "2*3".into();
    expect(&builder, &[2.5; 5], 6.0);
    builder.options = "GOAL=100 TOL=0.1".into();
    let failed_goal = evaluate(&builder, &[2.5; 5]);
    assert_eq!(failed_goal.value, Some(6.0));
    assert!(!failed_goal.passed);
}

#[test]
fn authored_plan_measurements_builder_executes_axis_crossings_and_delays() {
    let ramp = [0.0, 1.0, 2.0, 3.0, 4.0];
    let mut builder = MeasurementBuilder::default();
    builder.event.at = true;
    builder.event.value = "{sample}".into();
    builder.operation = "FIND";
    expect(&builder, &ramp, 2.5);
    builder.operation = "DERIV";
    expect(&builder, &ramp, 1.0);
    builder.event.at = false;
    builder.event.value = "1.5".into();
    builder.event.delay = "1".into();
    builder.operation = "FIND";
    expect(&builder, &ramp, 1.5);
    builder.operation = "WHEN";
    expect(&builder, &ramp, 1.5);
    builder.event.edge = "FALL";
    builder.event.occurrence = "LAST".into();
    builder.event.value = "0.5".into();
    expect(&builder, &[0.0, 1.0, 0.0, 1.0, 0.0], 3.5);
    builder.operation = "TRIG";
    builder.event.edge = "RISE";
    builder.event.occurrence = "1".into();
    builder.event.value = "1".into();
    builder.event.delay.clear();
    builder.target.value = "3".into();
    expect(&builder, &ramp, 2.0);
    builder.event.at = true;
    builder.target.at = true;
    expect(&builder, &ramp, 2.0);

    builder.options = "PRINT=ALL\n.end".into();
    assert!(builder.card("metric").is_err());
    builder.options.clear();
    assert!(builder.card("").is_err());
    assert!(builder.card("two names").is_err());
    builder.target.value.clear();
    assert!(builder.card("metric").is_err());
}
