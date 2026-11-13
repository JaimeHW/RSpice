//! `.MEAS` DERIV (time-derivative at a point) and PARAM (expressions over
//! prior measurement results).

use rspice_core::analysis::MeasureEngine;
use rspice_core::netlist::Netlist;
use std::collections::HashMap;

fn engine_for(deck: &str) -> MeasureEngine {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let mut engine = MeasureEngine::new();
    for statement in &netlist.measurements {
        engine.add(statement.clone());
    }
    engine
}

#[test]
fn deriv_at_reads_the_segment_slope() {
    let deck = "\
* deriv parse
v1 a 0 dc 1
r1 a 0 1k
.tran 1u 10u
.meas tran slope deriv v(out) at=1.5u
.end
";
    let engine = engine_for(deck);
    // Piecewise-linear ramp: v = 2e6 * t between 1u and 2u.
    let time = [0.0, 1e-6, 2e-6, 3e-6];
    let out = [0.0, 0.0, 2.0, 2.0];
    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    signals.insert("V(OUT)".to_string(), &out);
    let results = engine.evaluate(&time, &signals);
    let slope = results[0].value.expect("deriv evaluates");
    assert!(
        (slope - 2e6).abs() / 2e6 < 1e-9,
        "segment slope between 1u and 2u is 2e6 V/s, got {slope}"
    );
}

#[test]
fn deriv_when_uses_the_crossing_time() {
    let deck = "\
* deriv when
v1 a 0 dc 1
r1 a 0 1k
.tran 1u 10u
.meas tran slope deriv v(out) when v(trig)=0.5
.end
";
    let engine = engine_for(deck);
    let time = [0.0, 1e-6, 2e-6, 3e-6];
    let out = [0.0, 0.0, 2.0, 2.0];
    let trig = [0.0, 0.0, 1.0, 1.0]; // crosses 0.5 between 1u and 2u
    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    signals.insert("V(OUT)".to_string(), &out);
    signals.insert("V(TRIG)".to_string(), &trig);
    let results = engine.evaluate(&time, &signals);
    let slope = results[0].value.expect("deriv evaluates");
    assert!(
        (slope - 2e6).abs() / 2e6 < 1e-9,
        "crossing lands in the ramp segment, got {slope}"
    );
}

#[test]
fn param_expression_combines_prior_results() {
    let deck = "\
* param expression
v1 a 0 dc 1
r1 a 0 1k
.tran 1u 10u
.meas tran vmax max v(out)
.meas tran vmin min v(out)
.meas tran swing param='vmax-vmin'
.meas tran margin param='swing*0.5'
.end
";
    let engine = engine_for(deck);
    let time = [0.0, 1e-6, 2e-6, 3e-6];
    let out = [0.5, 0.0, 2.0, 1.0];
    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    signals.insert("V(OUT)".to_string(), &out);
    let results = engine.evaluate(&time, &signals);
    let by_name: HashMap<&str, Option<f64>> =
        results.iter().map(|r| (r.name.as_str(), r.value)).collect();
    assert_eq!(by_name["SWING"], Some(2.0), "vmax-vmin = 2-0");
    assert_eq!(
        by_name["MARGIN"],
        Some(1.0),
        "a PARAM may reference an earlier PARAM"
    );
}

#[test]
fn param_referencing_unknown_result_fails_loudly() {
    let deck = "\
* bad param
v1 a 0 dc 1
r1 a 0 1k
.tran 1u 10u
.meas tran broken param='nonexistent*2'
.end
";
    let engine = engine_for(deck);
    let time = [0.0, 1e-6];
    let out = [0.0, 1.0];
    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    signals.insert("V(OUT)".to_string(), &out);
    let results = engine.evaluate(&time, &signals);
    assert!(results[0].value.is_none(), "unknown reference must fail");
    assert!(!results[0].passed);
}
