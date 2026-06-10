//! VBIC excess-phase transient validated against the official ngspice-46
//! binary on a solvable circuit.
//!
//! The shipped vbic suite exercises the excess-phase network (`TD > 0`) in
//! exactly one transient deck — diffamp — whose topology no modern VBIC
//! implementation can solve (ngspice-46 and current HEAD both fail its
//! operating point outright with a singular matrix). That leaves the xf
//! delay states' time integration without regression coverage, so this test
//! pins it against a checked-in oracle table instead: a single
//! common-emitter stage carrying the full diffamp N1 model card (Kull epi
//! resistance, avalanche, parasitic transistor, and `TD=2e-11`), driven at
//! 1 GHz where the 20 ps two-pole delay network shifts the output by a
//! measurable phase. The official ngspice-46 release solves this deck
//! cleanly; its `.print tran v(c)` table is the fixture
//! (`testdata/vbic_xf_ce_ngspice46.dat`).
//!
//! At capture, RSpice matched the oracle to a worst case of 368 uV on the
//! ~100 mV output swing (0.37%), with a 228 uV mean. The gates below keep
//! comfortable margin over those figures while still failing on any real
//! regression of the promoted xf rows, their charge companions, or the
//! delayed-transport coupling.

use rspice_core::Netlist;
use rspice_core::engine::{Engine, SimulationConfig};

const ORACLE_TABLE: &str = include_str!("testdata/vbic_xf_ce_ngspice46.dat");

const TESTBENCH_DECK: &str = "\
VBIC excess-phase transient testbench (diffamp N1 card, stable CE stage)

V1 VCC 0 3.3
VIN B 0 DC 0.8 SIN(0.8 0.05 1G 0 0)
RC VCC C 1k
RE E 0 100
Q1 C B E 0 N1

.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300

.TRAN 0.01n 5n
.END
";

/// Worst-case deviation allowed against the oracle, in volts. Twice the
/// captured 368 uV worst case, and still under 0.8% of the output swing.
const MAX_ABS_TOLERANCE: f64 = 7.5e-4;

/// Mean deviation allowed against the oracle, in volts.
const MEAN_ABS_TOLERANCE: f64 = 4.0e-4;

fn oracle_samples() -> Vec<(f64, f64)> {
    ORACLE_TABLE
        .lines()
        .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let mut fields = line.split_whitespace();
            let time: f64 = fields
                .next()
                .expect("oracle row missing time")
                .parse()
                .expect("oracle time parse");
            let voltage: f64 = fields
                .next()
                .expect("oracle row missing voltage")
                .parse()
                .expect("oracle voltage parse");
            (time, voltage)
        })
        .collect()
}

/// Linear interpolation of `(time, value)` samples at `t`; the sample times
/// are strictly increasing and `t` lies within their span.
fn interpolate(samples_t: &[f64], samples_v: &[f64], t: f64) -> f64 {
    match samples_t.binary_search_by(|probe| probe.partial_cmp(&t).unwrap()) {
        Ok(exact) => samples_v[exact],
        Err(0) => samples_v[0],
        Err(after) if after >= samples_t.len() => *samples_v.last().unwrap(),
        Err(after) => {
            let (t0, t1) = (samples_t[after - 1], samples_t[after]);
            let (v0, v1) = (samples_v[after - 1], samples_v[after]);
            v0 + (v1 - v0) * (t - t0) / (t1 - t0)
        }
    }
}

#[test]
fn excess_phase_ce_stage_matches_ngspice46_oracle() {
    let oracle = oracle_samples();
    assert!(
        oracle.len() > 500,
        "oracle fixture unexpectedly short: {} rows",
        oracle.len()
    );

    let netlist = Netlist::parse(TESTBENCH_DECK).expect("parse testbench");
    let engine = Engine::new(SimulationConfig::default());
    // tstop and max_step per the deck's `.TRAN 0.01n 5n` (tstep caps the
    // step), giving a grid dense enough that linear interpolation onto the
    // oracle's print times adds no measurable error at 1 GHz.
    let result = engine
        .run_tran(&netlist, 5e-9, 1e-11)
        .expect("transient must complete: the xf network is active and the stage is stable");

    let c_index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("c"))
        .expect("output node C present");
    let ours_v = &result.voltages[c_index];
    assert_eq!(ours_v.len(), result.time.len());
    assert!(
        result.time.len() > 100,
        "implausibly few accepted points: {}",
        result.time.len()
    );

    let mut max_abs: f64 = 0.0;
    let mut max_abs_time = 0.0;
    let mut sum_abs = 0.0;
    for &(t, oracle_v) in &oracle {
        let v = interpolate(&result.time, ours_v, t);
        let diff = (v - oracle_v).abs();
        sum_abs += diff;
        if diff > max_abs {
            max_abs = diff;
            max_abs_time = t;
        }
    }
    let mean_abs = sum_abs / oracle.len() as f64;

    assert!(
        max_abs < MAX_ABS_TOLERANCE,
        "worst deviation {max_abs:.3e} V at t={max_abs_time:.3e} exceeds {MAX_ABS_TOLERANCE:.1e}"
    );
    assert!(
        mean_abs < MEAN_ABS_TOLERANCE,
        "mean deviation {mean_abs:.3e} V exceeds {MEAN_ABS_TOLERANCE:.1e}"
    );
}
