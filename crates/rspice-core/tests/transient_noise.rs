//! TRNOISE end-to-end: transient noise sources through the full engine.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn run_noise_deck(seed_line: &str) -> (Vec<f64>, Vec<f64>) {
    let deck = format!(
        "\
* trnoise bench
{seed_line}
v1 in 0 trnoise(1m 1n 0 0)
r1 in out 1k
r2 out 0 1k
.tran 1n 2u
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 2e-6, 1e-9)
        .expect("transient with noise source runs");
    let out_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    (result.time.clone(), result.voltages[out_idx].clone())
}

#[test]
fn white_noise_reaches_the_output_with_expected_statistics() {
    let (time, v_out) = run_noise_deck(".options seed=11");

    // The divider halves the 1 mV RMS source noise. Points *between* noise
    // samples are linear interpolations of two Gaussians (reduced variance
    // by construction), so the sharp invariant is the variance measured at
    // the NT sample grid itself. Solver timepoints land on the grid because
    // every PWL vertex is a breakpoint.
    const NT: f64 = 1e-9;
    let on_grid: Vec<f64> = time
        .iter()
        .zip(&v_out)
        .filter(|(t, _)| {
            let k = (**t / NT).round();
            (**t - k * NT).abs() < 1e-15
        })
        .map(|(_, v)| *v)
        .collect();

    assert!(
        on_grid.len() >= 500,
        "solver must land on the noise sample grid (breakpoints honored); \
         only {} of {} points were on-grid",
        on_grid.len(),
        time.len()
    );

    let n = on_grid.len() as f64;
    let mean = on_grid.iter().sum::<f64>() / n;
    let var = on_grid.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let expected = (0.5e-3f64).powi(2);

    assert!(
        mean.abs() < 5e-5,
        "noise is zero-mean at the output, got mean {mean}"
    );
    assert!(
        (var - expected).abs() / expected < 0.12,
        "on-grid output variance within 12% of (NA/2)^2: got {var}, want {expected}"
    );
}

#[test]
fn runs_are_bit_identical_for_a_fixed_seed() {
    let (t1, v1) = run_noise_deck(".options seed=42");
    let (t2, v2) = run_noise_deck(".options seed=42");
    assert_eq!(t1.len(), t2.len(), "identical time grids");
    assert!(
        t1.iter().zip(&t2).all(|(a, b)| a == b) && v1.iter().zip(&v2).all(|(a, b)| a == b),
        "fixed seed must reproduce the run bit-identically"
    );
}

#[test]
fn different_seeds_give_different_sample_paths() {
    let (_, v1) = run_noise_deck(".options seed=1");
    let (_, v2) = run_noise_deck(".options seed=2");
    assert!(
        v1.iter().zip(&v2).any(|(a, b)| a != b),
        "different seeds must give different noise"
    );
}

#[test]
fn operating_point_sees_zero() {
    let deck = "\
* trnoise op
v1 in 0 trnoise(10m 1n 0 0)
r1 in out 1k
r2 out 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("op solves");
    for (name, idx) in op.node_names.iter().zip(0..) {
        if name.eq_ignore_ascii_case("in") || name.eq_ignore_ascii_case("out") {
            assert!(
                op.node_voltages[idx].abs() < 1e-12,
                "OP must see 0 from a zero-mean noise source at {name}"
            );
        }
    }
}

#[test]
fn rts_tail_is_rejected_with_a_clear_diagnostic() {
    let deck = "\
* trnoise rts
v1 in 0 trnoise(1m 1n 0 0 5m 10u 20u)
r1 in 0 1k
.tran 1n 1u
.end
";
    let err = Netlist::parse(deck).expect_err("RTS tail must be rejected");
    let msg = format!("{err}");
    assert!(
        msg.contains("RTS"),
        "diagnostic names the unsupported feature: {msg}"
    );
}
