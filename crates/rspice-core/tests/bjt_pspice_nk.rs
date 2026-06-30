//! Xyce/ngspice-compatible BJT PSpice NK/NKF high-current rolloff checks.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn bjt_pspice_nk_deck() -> &'static str {
    "\
* Xyce BJT_PSPICE_NK forward Gummel deck
VBB  6 0 0V
VMON1 6 1 0
VMON2 6 2 0
Q1 2 1 0 NBJT
.MODEL NBJT NPN (
+ IS=2.96E-14
+ BF=233.8
+ NF=1
+ ISE=2.30E-14
+ NE=1.780
+ VAF=2
+ IKF=0.340
+ NK=0.9
+ BR=1
+ NR=1
+ ISC=0
+ NC=2.0
+ VAR=100
+ IKR=1e99
+ RB=1.3
+ RC=1.3
+ RE=0 )
.DC VBB 0.15 0.95 0.05
.PRINT DC V(6) I(VMON1) I(VMON2)
.END
"
}

fn bjt_pspice_nkf_deck() -> String {
    bjt_pspice_nk_deck().replace("NK=0.9", "NKF=0.9")
}

fn branch_index(result: &rspice_core::solver::SimulationResult, name: &str) -> usize {
    result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing {name} branch in {:?}", result.branch_names))
}

#[test]
fn bjt_pspice_nk_matches_xyce_forward_gummel_collector_current() {
    let netlist = Netlist::parse(bjt_pspice_nk_deck()).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep(&netlist, "vbb", 0.15, 0.95, 0.05)
        .expect("sweep converges");

    let vmon2 = branch_index(&results[0].1, "vmon2");
    let xyce710_tail = [
        (0.75, 6.836_429_12e-2),
        (0.80, 1.490_730_48e-1),
        (0.85, 1.978_536_01e-1),
        (0.90, 2.244_952_79e-1),
        (0.95, 2.426_359_14e-1),
    ];

    for &(bias, expected) in &xyce710_tail {
        let (_, result) = results
            .iter()
            .find(|(sweep, _)| (sweep - bias).abs() < 1e-12)
            .unwrap_or_else(|| panic!("missing sweep bias {bias:.2}"));
        let got = result.branch_currents[vmon2];
        let abs = (got - expected).abs();
        let rel = abs / expected.abs();
        assert!(
            rel < 2.0e-3 || abs < 5.0e-5,
            "BJT NK collector current at VBB={bias:.2}: rspice={got:.9e} xyce={expected:.9e} abs={abs:.3e} rel={rel:.3e}"
        );
    }
}

#[test]
fn bjt_pspice_nk_matches_xyce_forward_gummel_base_current() {
    let netlist = Netlist::parse(bjt_pspice_nk_deck()).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep(&netlist, "vbb", 0.15, 0.95, 0.05)
        .expect("sweep converges");

    let vmon1 = branch_index(&results[0].1, "vmon1");
    let xyce710_points = [
        (0.20, 2.038_331_06e-12),
        (0.35, 1.414_174_73e-10),
        (0.50, 3.268_393_12e-8),
        (0.75, 4.848_572_21e-4),
        (0.90, 3.231_205_34e-2),
    ];

    for &(bias, expected) in &xyce710_points {
        let (_, result) = results
            .iter()
            .find(|(sweep, _)| (sweep - bias).abs() < 1e-12)
            .unwrap_or_else(|| panic!("missing sweep bias {bias:.2}"));
        let got = result.branch_currents[vmon1];
        let abs = (got - expected).abs();
        let rel = abs / expected.abs();
        assert!(
            rel < 2.0e-3 || abs < 5.0e-15,
            "BJT NK base current at VBB={bias:.2}: rspice={got:.9e} xyce={expected:.9e} abs={abs:.3e} rel={rel:.3e}"
        );
    }
}

#[test]
fn bjt_pspice_nkf_alias_matches_xyce_forward_gummel_collector_current() {
    let deck = bjt_pspice_nkf_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep(&netlist, "vbb", 0.15, 0.95, 0.05)
        .expect("sweep converges");

    let vmon2 = branch_index(&results[0].1, "vmon2");
    let (_, result) = results
        .iter()
        .find(|(sweep, _)| (sweep - 0.90).abs() < 1e-12)
        .expect("VBB=0.90 row present");
    let got = result.branch_currents[vmon2];
    let expected = 2.244_952_79e-1;
    let rel = (got - expected).abs() / expected;
    assert!(
        rel < 2.0e-3,
        "BJT NKF alias collector current mismatch: rspice={got:.9e} xyce={expected:.9e} rel={rel:.3e}"
    );
}
