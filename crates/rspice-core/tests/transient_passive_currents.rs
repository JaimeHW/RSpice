use rspice_core::engine::Engine;
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
