//! BJT deck-level agreement with the Xyce and ngspice references: PSpice
//! NK/NKF high-current rolloff, and the OFF keyword's operating-point branch
//! selection.

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

/// Cross-coupled NPN pair with two stable DC operating points. Which one the
/// solver reports is exactly what an OFF keyword is there to decide.
fn bistable_deck(off_instance: &str) -> String {
    let annotate = |instance: &str| {
        if instance == off_instance {
            " OFF"
        } else {
            ""
        }
    };
    format!(
        "* cross-coupled bistable steered by the OFF keyword\n\
         vcc vcc 0 dc 5\n\
         rc1 vcc c1 1k\n\
         rc2 vcc c2 1k\n\
         rb1 c1 b2 10k\n\
         rb2 c2 b1 10k\n\
         q1 c1 b1 0 qmod{}\n\
         q2 c2 b2 0 qmod{}\n\
         .model qmod NPN (IS=1e-16 BF=100)\n\
         .op\n\
         .end\n",
        annotate("q1"),
        annotate("q2")
    )
}

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("bistable operating point converges");
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

#[test]
fn bjt_off_keyword_selects_the_bistable_operating_point_branch() {
    // ngspice-46 on the same deck: marking q1 OFF cuts it off and pulls c1 up
    // to 4.619879 V, marking q2 OFF gives the exact mirror at 0.07356493 V.
    // Without the keyword both references settle on the symmetric root, so a
    // simulator that ignores OFF returns that root all three times.
    let q1_off_c1 = op_voltage(&bistable_deck("q1"), "c1");
    let q2_off_c1 = op_voltage(&bistable_deck("q2"), "c1");

    for (label, got, expected) in [
        ("q1 OFF", q1_off_c1, 4.619_879_f64),
        ("q2 OFF", q2_off_c1, 0.073_564_93_f64),
    ] {
        let rel = (got - expected).abs() / expected.abs();
        assert!(
            rel < 1.0e-5,
            "{label} must select the ngspice branch: rspice={got:.9e} ngspice={expected:.9e} rel={rel:.3e}"
        );
    }

    // The two markings must land on opposite branches, not merely near a
    // reference value, so a future symmetric-root regression cannot pass.
    assert!(
        q1_off_c1 - q2_off_c1 > 4.0,
        "OFF on either device must select opposite branches, got {q1_off_c1:.9e} and {q2_off_c1:.9e}"
    );
}
