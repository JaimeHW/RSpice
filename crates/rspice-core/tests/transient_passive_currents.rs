use rspice_core::analysis::IntegrationMethod;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

#[test]
fn transient_records_linear_resistor_branch_current_waveforms() {
    let deck = "\
* linear resistor transient branch currents
v1 in 0 dc 5
r1 in mid 1k
r2 mid 0 1k
.tran 1u 3u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-6, 1.0e-6)
        .expect("transient solves");

    let r1 = result
        .try_branch_current_waveform_named("r1")
        .expect("R1 branch current waveform exists");
    let r2 = result
        .try_branch_current_waveform_named("r2")
        .expect("R2 branch current waveform exists");

    assert_eq!(r1.len(), result.time.len());
    assert_eq!(r2.len(), result.time.len());

    for (&i_r1, &i_r2) in r1.iter().zip(r2.iter()) {
        assert!(
            (i_r1 - 2.5e-3).abs() < 1.0e-12,
            "R1 current should be positive from IN to MID, got {i_r1}"
        );
        assert!(
            (i_r2 - 2.5e-3).abs() < 1.0e-12,
            "R2 current should be positive from MID to ground, got {i_r2}"
        );
    }
}

#[test]
fn transient_records_linear_capacitor_branch_current_waveforms() {
    let deck = "\
* linear capacitor transient branch currents
v1 supply 0 dc 0
r1 n supply 1k
c1 n 0 1u ic=1
.options timeint method=trap
.tran 1u 5u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 5.0e-6, 1.0e-6)
        .expect("transient solves");

    let v_n = result
        .try_voltage_waveform_named("n")
        .expect("node N waveform exists");
    let c1 = result
        .try_branch_current_waveform_named("c1")
        .expect("C1 branch current waveform exists");

    assert_eq!(c1.len(), result.time.len());
    for (&time, (&voltage, &current)) in result.time.iter().zip(v_n.iter().zip(c1.iter())) {
        let expected_kcl_current = -voltage / 1000.0;
        assert!(
            (current - expected_kcl_current).abs() < 1.0e-10,
            "C1 current should satisfy KCL from N to ground, got {current} expected {expected_kcl_current} at {time}"
        );
    }
}

#[test]
fn transient_records_linear_capacitor_ic_startup_branch_current() {
    let deck = "\
* linear capacitor IC startup branch current
v1 supply 0 dc 0
r1 n supply 1k
c1 n 0 40u ic=1
.options timeint method=trap
.tran 0.5u 1u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::Trapezoidal,
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 1.0e-6, 0.5e-6)
    .expect("transient solves");

    let c1 = result
        .try_branch_current_waveform_named("c1")
        .expect("C1 branch current waveform exists");
    assert!(
        (c1[0] + 1.0e-3).abs() < 1.0e-8,
        "C1 initial branch current should balance the IC discharge path, got {}",
        c1[0]
    );
}

#[test]
fn transient_records_behavioral_current_source_waveforms() {
    let deck = "\
* behavioral current source branch current
b1 n 0 i={time}
r1 n 0 1
.tran 1u 3u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-6, 1.0e-6)
        .expect("transient solves");

    let b1 = result
        .try_branch_current_waveform_named("b1")
        .expect("B1 branch current waveform exists");

    assert_eq!(b1.len(), result.time.len());
    for (&time, &current) in result.time.iter().zip(b1.iter()) {
        assert!(
            (current - time).abs() < 1.0e-15,
            "B1 current should equal TIME, got {current} at {time}"
        );
    }
}

#[test]
fn transient_records_time_dependent_resistor_branch_current_waveforms() {
    let deck = "\
* time-dependent resistor branch current
v1 in 0 dc 1
v2 out 0 dc 0
r1 in out {10 + 1000*time}
.tran 1u 3u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-6, 1.0e-6)
        .expect("transient solves");

    let r1 = result
        .try_branch_current_waveform_named("r1")
        .expect("R1 branch current waveform exists");

    assert_eq!(r1.len(), result.time.len());
    for (&time, &current) in result.time.iter().zip(r1.iter()) {
        let expected = 1.0 / (10.0 + 1000.0 * time);
        assert!(
            (current - expected).abs() < 1.0e-12,
            "R1 current should be 1/R(TIME), got {current} expected {expected} at {time}"
        );
    }
}
