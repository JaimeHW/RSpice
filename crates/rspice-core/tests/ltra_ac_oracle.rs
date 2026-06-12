//! LTRA small-signal response validated against the official ngspice-46
//! binary.
//!
//! ngspice loads LTRA lines in AC with the exact analytic two-port
//! (ltraacld.c): `Y0(s)*V1 - I1 = e^{-lambda*l}*(Y0(s)*V2 + I2)` with
//! `Y0 = sqrt(sC/(R+sL))` and `lambda*l = sqrt(sC*(R+sL))` in total line
//! quantities. The native-LTRA branch rows must carry that load — the
//! checked-in transmission suite only exercises the lines in transient, so
//! this fixture pins the AC path: a 50 ohm source driving the txl/ltra
//! regression card (R=12.45, L=8.972n, C=0.468p, len=16) into 75 ohms,
//! swept 1 MHz to 10 GHz across the line's delay and skin-less RC rollover.
//!
//! The fixture table is the ngspice-46 batch `.print ac` output (phases in
//! radians). At capture, RSpice matched magnitudes within 4e-6 relative and
//! phases within 3e-6 radians across all 41 points.

use rspice_core::Netlist;
use rspice_core::engine::{Engine, SimulationConfig};

const ORACLE_TABLE: &str = include_str!("testdata/ltra_ac_ngspice46.dat");

const TESTBENCH_DECK: &str = "\
LTRA AC parity testbench
VIN in 0 DC 0 AC 1
RS in 1 50
o1 1 0 2 0 lline
RL 2 0 75
.AC DEC 10 1e6 10e9
.model lline ltra rel=1 r=12.45 g=0 l=8.972e-9 c=0.468e-12
+len=16 steplimit compactrel=1.0e-3 compactabs=1.0e-14
.end
";

/// Relative magnitude gate. Capture-time worst was 4e-6.
const MAG_REL_TOLERANCE: f64 = 1.0e-4;
/// Absolute phase gate in radians. Capture-time worst was 3e-6.
const PHASE_ABS_TOLERANCE: f64 = 1.0e-4;

struct OracleRow {
    frequency: f64,
    vm2: f64,
    vp2: f64,
    vm1: f64,
    vp1: f64,
}

fn oracle_rows() -> Vec<OracleRow> {
    ORACLE_TABLE
        .lines()
        .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let mut f = line.split_whitespace().map(|v| {
                v.parse::<f64>()
                    .unwrap_or_else(|e| panic!("oracle field '{v}': {e}"))
            });
            OracleRow {
                frequency: f.next().expect("frequency"),
                vm2: f.next().expect("vm2"),
                vp2: f.next().expect("vp2"),
                vm1: f.next().expect("vm1"),
                vp1: f.next().expect("vp1"),
            }
        })
        .collect()
}

#[test]
fn ltra_ac_two_port_matches_ngspice46_oracle() {
    let oracle = oracle_rows();
    assert_eq!(oracle.len(), 41, "expected the full 41-point sweep");

    let netlist = Netlist::parse(TESTBENCH_DECK).expect("parse testbench");
    let engine = Engine::new(SimulationConfig::default());
    let frequencies = rspice_core::analysis::ac::ac_sweep_frequencies(
        rspice_core::netlist::FreqVariation::Dec,
        10,
        1e6,
        10e9,
    );
    let results = engine
        .run_ac(&netlist, &frequencies)
        .expect("AC analysis runs");
    assert_eq!(results.len(), oracle.len(), "sweep grids must align");

    let node_index = |name: &str| -> usize {
        results[0]
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("node {name} present"))
    };
    let idx1 = node_index("1");
    let idx2 = node_index("2");

    for (point, row) in results.iter().zip(&oracle) {
        // The fixture prints frequencies at ngspice's 6-digit table
        // precision; the sweep itself is the same acan.c generator.
        let rel_f = (point.frequency - row.frequency).abs() / row.frequency;
        assert!(rel_f < 1e-5, "frequency grid mismatch at {}", row.frequency);

        for (idx, vm_ref, vp_ref, label) in [
            (idx2, row.vm2, row.vp2, "v(2)"),
            (idx1, row.vm1, row.vp1, "v(1)"),
        ] {
            let v = point.voltages[idx];
            let vm = v.norm();
            let vp = v.arg();
            let mag_err = (vm - vm_ref).abs() / vm_ref.abs().max(1e-30);
            let phase_err = (vp - vp_ref).abs();
            assert!(
                mag_err < MAG_REL_TOLERANCE,
                "{label} magnitude at {:.3e} Hz: ours {vm:.9e} vs ngspice-46 {vm_ref:.9e} (rel {mag_err:.3e})",
                row.frequency
            );
            assert!(
                phase_err < PHASE_ABS_TOLERANCE,
                "{label} phase at {:.3e} Hz: ours {vp:.9e} vs ngspice-46 {vp_ref:.9e} rad (abs {phase_err:.3e})",
                row.frequency
            );
        }
    }
}
