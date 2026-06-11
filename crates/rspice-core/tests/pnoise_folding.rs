//! Periodic-noise validation: stationary parity and analytic folding.
//!
//! 1. With no large-signal drive the periodic operating point is the DC
//!    point and every modulated intensity is constant, so pnoise must
//!    reproduce the ordinary .noise analysis at the same frequencies.
//! 2. A resistor chopped by an ideal switch is the classic LTV noise
//!    problem: for memoryless modulation the output PSD of each stationary
//!    source is its intensity times the time-average squared transfer,
//!    computable in closed form from the two switch states.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const K_B: f64 = 1.380649e-23;
const T_REF: f64 = 300.15;

#[test]
fn pnoise_without_large_signal_drive_matches_stationary_noise() {
    // Forward-biased diode divider: thermal (R1) plus shot (D1) noise with
    // frequency shaping from the 1 nF capacitor.
    let deck = "\
* stationary parity network
v1 in 0 dc 2
r1 in mid 10k
d1 mid 0 dmod
c1 mid 0 1n
.model dmod D IS=1e-12 N=1.0 CJ0=0 TT=0 RS=0
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let offsets = [1.0e3, 1.0e5, 1.0e7];

    let pnoise = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, 6)
        .expect("pnoise completes");

    // Reference: the stationary noise analysis at the same frequencies.
    let dc = engine.run_dc_op(&netlist).expect("dc op");
    // run_dc_op node names include ground at index 0, so the position is
    // already the matrix node index run_noise_ports expects.
    let mid_idx = dc
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("mid"))
        .expect("mid node");
    let stationary = engine
        .run_noise_ports(&netlist, mid_idx, None, &offsets, T_REF)
        .expect("stationary noise completes");

    for (i, &freq) in offsets.iter().enumerate() {
        let folded = pnoise.output_noise[i];
        let reference = stationary[i].output_noise_density;
        assert!(
            (folded - reference).abs() < 0.03 * reference,
            "at {freq:.1e} Hz pnoise must match stationary noise: \
             {folded:.4e} vs {reference:.4e} V^2/Hz"
        );
    }
}

#[test]
fn chopped_resistor_noise_folds_to_the_time_average_transfer() {
    // 50% chopper between two 1k resistors. Closed form per source
    // (time-average squared transfer of the two switch states):
    //   R1 (source side): on Z = R1*(ron+R2)/(R1+ron+R2), off ~0
    //   R2 (output side): on Z = R2*(ron+R1)/(R1+ron+R2), off Z = R2
    // Switch ron thermal contributes ~4kT*0.25, negligible but modeled.
    let deck = "\
* chopped resistor noise
vlo ctl 0 sin(0 1 1meg)
r1 src 0 1k
s1 src out ctl 0 swmod
r2 out 0 1k
c1 out 0 1f
.model swmod sw vt=0 ron=1 roff=1e9 smooth=1m
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let result = engine
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, 12)
        .expect("pnoise completes");
    assert!(result.converged, "operating point must converge");

    let (r1, r2, ron) = (1000.0, 1000.0, 1.0);
    let loop_r = r1 + ron + r2;
    let z_r1_on = r1 * (ron + r2) / loop_r;
    let z_r2_on = r2 * (ron + r1) / loop_r;
    let z_sw_on = ron * r2 / loop_r; // parallel current source across ron

    let s_r1 = 4.0 * K_B * T_REF / r1 * 0.5 * z_r1_on * z_r1_on;
    let s_r2 = 4.0 * K_B * T_REF / r2 * 0.5 * (z_r2_on * z_r2_on + r2 * r2);
    let s_sw = 4.0 * K_B * T_REF / ron * 0.5 * z_sw_on * z_sw_on;
    let expected = s_r1 + s_r2 + s_sw;

    let got = result.output_noise[0];
    assert!(
        (got - expected).abs() < 0.04 * expected,
        "chopped-resistor output noise must fold to the time-average \
         transfer: got {got:.4e}, want {expected:.4e} V^2/Hz"
    );
}

/// Per-source contributions must decompose the total exactly (independent
/// sources), so the contributor list is a true breakdown rather than an
/// estimate.
#[test]
fn pnoise_contributors_sum_to_the_total() {
    let deck = "\
* contributor decomposition network
v1 in 0 dc 2
r1 in mid 10k
d1 mid 0 dmod
c1 mid 0 1n
.model dmod D IS=1e-12 N=1.0 CJ0=0 TT=0 RS=0
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let offsets = [1.0e4, 1.0e6];

    let result = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, 6)
        .expect("pnoise completes");

    assert!(
        !result.contributors.is_empty(),
        "thermal and shot contributors must be reported"
    );
    for (i, &total) in result.output_noise.iter().enumerate() {
        let sum: f64 = result.contributors.iter().map(|(_, psds)| psds[i]).sum();
        assert!(
            (sum - total).abs() <= 1e-12 * total.max(1e-300),
            "contributors must sum to the total at offset {}: {sum:.6e} vs {total:.6e}",
            offsets[i]
        );
    }
}
