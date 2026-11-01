//! The CLI's transient step ceiling.
//!
//! Xyce never bounds the step by `.TRAN TSTEP`: an omitted `TMAX` is a tenth
//! of the analysis window. A deck run with `--spice-dialect xyce` therefore
//! takes the steps its error control asks for, not one per print step. No
//! dialect has a ceiling of its own beyond the `.TRAN` card's.

mod common;

use common::{csv_column, run_rspice, test_dir};

/// tau = 1 ms behind a 1 us ramp, printed every 1 us for 10 ms: ngspice's
/// ceiling (`min(TSTEP, window / 50)`) forces 10 000 steps. Xyce 7.10 takes
/// 60, with a worst error of 4.6e-4 V against the exact charge.
const RC_DECK: &str = "* RC charge\n\
     V1 in 0 PULSE(0 1 0 1u 1u 1 2)\n\
     R1 in out 1k\n\
     C1 out 0 1u\n\
     .TRAN 1u 10m\n\
     .PRINT TRAN V(out)\n\
     .END\n";

#[test]
fn xyce_dialect_does_not_bound_the_step_by_the_print_step() {
    let dir = test_dir("xyce-ceiling");
    let deck = dir.join("rc.cir");
    let csv = dir.join("rc.csv");
    std::fs::write(&deck, RC_DECK).expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        "--spice-dialect",
        "xyce",
        deck.to_str().unwrap(),
        "-o",
        csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(&csv).expect("read CSV");
    let time = csv_column(&text, "time");
    let out = csv_column(&text, "V(out)");
    assert!(
        time.len() < 100,
        "the Xyce dialect took {} steps where Xyce takes 60",
        time.len()
    );
    assert!((time[time.len() - 1] - 10.0e-3).abs() < 1.0e-12);
    let tau: f64 = 1.0e-3;
    let worst = time
        .iter()
        .zip(&out)
        .filter(|(t, _)| **t >= 1.0e-6)
        .map(|(t, v)| {
            // Exact response to the 1 us ramp, past its end.
            let ramp = tau / 1.0e-6 * ((1.0e-6 / tau).exp() - 1.0);
            (v - (1.0 - ramp * (-t / tau).exp())).abs()
        })
        .fold(0.0, f64::max);
    assert!(
        worst < 1.0e-3,
        "worst error {worst:e} V against the exact RC charge"
    );
}

/// tau = 1 s behind a 1 ms ramp, printed every 100 ms for 6 s. ngspice-46
/// takes 83 steps, 2.8e-4 V from the exact charge at worst; a fixed 1 ms
/// frontend ceiling forced 6000.
const SLOW_RC_DECK: &str = "* RC charge, seconds
     V1 in 0 PULSE(0 1 0 1m 1m 100 200)
     R1 in out 1k
     C1 out 0 1m
     .TRAN 0.1 6
     .PRINT TRAN V(out)
     .END
";

#[test]
fn no_default_ceiling_caps_a_seconds_scale_transient() {
    let dir = test_dir("default-ceiling");
    let deck = dir.join("rc.cir");
    let csv = dir.join("rc.csv");
    std::fs::write(&deck, SLOW_RC_DECK).expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        "--spice-dialect",
        "ngspice",
        deck.to_str().unwrap(),
        "-o",
        csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(&csv).expect("read CSV");
    let time = csv_column(&text, "time");
    let out = csv_column(&text, "V(out)");
    assert!(
        time.len() < 100,
        "the run took {} steps where ngspice-46 takes 83",
        time.len()
    );
    let tau: f64 = 1.0;
    let ramp = tau / 1.0e-3 * ((1.0e-3 / tau).exp() - 1.0);
    let worst = time
        .iter()
        .zip(&out)
        .filter(|(t, _)| **t >= 1.0e-3)
        .map(|(t, v)| (v - (1.0 - ramp * (-t / tau).exp())).abs())
        .fold(0.0, f64::max);
    assert!(
        worst < 1.0e-3,
        "worst error {worst:e} V against the exact RC charge"
    );
}
