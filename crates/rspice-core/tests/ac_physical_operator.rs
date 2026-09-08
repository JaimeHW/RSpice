//! Physical AC-operator regressions.
//!
//! These cases deliberately exercise weak, ideal, floating, and dependent
//! equations through the public engine boundary. A solver conditioning term
//! must never become a hidden shunt or series element that changes them.

use rspice_core::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::{Engine, SimulationConfig, SimulationErrorCode};
use rspice_core::netlist::Netlist;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("AC regression deck parses")
}

fn solve_one(deck: &str) -> AcResult {
    Engine::new(SimulationConfig::default())
        .run_ac(&parse(deck), &[1.0e3])
        .expect("physical AC operator solves")
        .pop()
        .expect("one requested AC point")
}

fn voltage(point: &AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing node {node:?} in {:?}", point.node_names));
    point.voltages[index]
}

fn branch_current(point: &AcResult, branch: &str) -> Complex64 {
    let index = point
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing branch {branch:?} in {:?}", point.branch_names));
    point.currents[index]
}

fn assert_solver_singular(deck: &str) {
    let error = Engine::new(SimulationConfig::default())
        .run_ac(&parse(deck), &[1.0e3])
        .expect_err("an underdetermined physical AC operator must not be regularized");
    assert_eq!(
        error.descriptor().code,
        SimulationErrorCode::SolverError,
        "singularity must cross the public API as a solver error: {error}"
    );
    assert!(
        error.to_string().to_ascii_lowercase().contains("singular"),
        "solver error must identify the singular operator: {error}"
    );
}

#[test]
fn vbic_overlap_capacitance_has_positive_admittance_for_both_polarities() {
    // With transport and junction capacitances negligible, Cbeo and the
    // authored resistor form an ordinary 1 ms RC low-pass for either BJT type.
    let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, 2.0 * std::f64::consts::PI);
    for kind in ["NPN", "PNP"] {
        let point = solve_one(&format!(
            "VBIC overlap-capacitance orientation\n\
             V1 in 0 DC 0 AC 1\nR1 in out 1k\nQ1 0 out 0 qmod\n\
             .model qmod {kind} LEVEL=4 IS=1e-30 IBEI=0 IBEN=0 IBCI=0 IBCN=0 ISP=0 CBEO=1u\n.end\n"
        ));
        let actual = voltage(&point, "out");
        assert!(
            (actual - expected).norm() < 1e-8,
            "{kind} overlap capacitor must produce the passive RC response: {actual:?}, expected {expected:?}"
        );
    }
}

#[test]
fn vbic13_delayed_avalanche_heat_and_clipped_temperature_match_xyce710() {
    for (level, kind, polarity, rise, drive_base, multiplier, expected) in [
        (
            11,
            "NPN",
            1.0,
            20.0,
            true,
            1,
            [
                Complex64::new(-0.0055299710825809055, 0.003995971898904793),
                Complex64::new(0.0002868746047409443, -0.00026608037701922105),
                Complex64::new(0.009743814554519954, -0.0070067899992358774),
            ],
        ),
        (
            12,
            "PNP",
            -1.0,
            74.0,
            false,
            3,
            [
                Complex64::new(3.36865683680846e-5, -2.340680248645959e-5),
                Complex64::new(-1.2487229592166488e-6, 1.2924528759025553e-6),
                Complex64::new(-0.002058422117792587, -0.0019261856424439548),
            ],
        ),
    ] {
        let substrate = if level == 12 { " 0" } else { "" };
        let netlist = parse(&format!(
            "Delayed VBIC avalanche and heat\nVc c 0 {}\nVb b 0 DC {} AC {}\nVth th 0 DC {rise} AC {}\nQ1 c b 0{substrate} th vm SW_ET=1 M={multiplier}\n\
             .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TAVC=0.01 TD=1n RTH=1000 TCRTH=0.005 TMAXCLIP=100 CTH=1p GMIN=1u TNOM=27)\n.temp 27\n.end\n",
            polarity * 1.8,
            polarity * 0.7,
            u8::from(drive_base),
            u8::from(!drive_base),
        ));
        let point = Engine::default()
            .run_ac(&netlist, &[1e8])
            .unwrap()
            .pop()
            .unwrap();
        for (branch, expected) in ["vc", "vb", "vth"].into_iter().zip(expected) {
            let actual = branch_current(&point, branch);
            assert!(
                (actual - expected).norm() < 2e-7 * expected.norm(),
                "{level} {kind} {branch}: {actual:?} != {expected:?}"
            );
        }
    }
}

#[test]
fn vbic13_intrinsic_avalanche_uses_delayed_forward_transport() {
    // Xyce 7.10 at 100 MHz: the BC avalanche current follows the
    // two-pole delay, just as the collector-emitter transport does.
    let expected = [
        Complex64::new(-0.001774651889771782, 0.0012868112960925493),
        Complex64::new(9.590581606050419e-5, -9.066589478545957e-5),
    ];
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let netlist = parse(&format!(
            "Delayed VBIC avalanche\nVc c 0 {}\nVb b 0 DC {} AC 1\nQ1 c b 0 vm SW_ET=0\n\
             .model vm {kind}(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TD=1n GMIN=1u TNOM=27)\n.temp 27\n.end\n",
            polarity * 1.8,
            polarity * 0.7,
        ));
        let point = Engine::default()
            .run_ac(&netlist, &[1e8])
            .unwrap()
            .pop()
            .unwrap();
        for (branch, expected) in ["vc", "vb"].into_iter().zip(expected) {
            let actual = branch_current(&point, branch);
            assert!(
                (actual - expected).norm() < 2e-7 * expected.norm(),
                "{kind} {branch}: {actual:?} != {expected:?}"
            );
        }
    }
}

#[test]
fn vbic_collapsed_parasitic_base_preserves_its_junction_admittance() {
    // RBP=0 joins BP to CX; it must retain both the BEP diode and charge.
    let vt = 1.380_662e-23 * 300.15 / 1.602_189e-19;
    let conductance = 1e-15 * (0.2_f64 / vt).exp() / vt;
    let capacitance = 1e-8 / (1.0_f64 - 0.2 / 0.75).powf(0.33);
    let expected = Complex64::new(conductance, std::f64::consts::TAU * 1e3 * capacitance);
    for level in [4, 11, 12] {
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 11 { "" } else { " 0" };
            let point = solve_one(&format!(
                "Collapsed VBIC parasitic junction\nVc c 0 {}\nVb b 0 DC {} AC 1\nQ1 c b 0{substrate} vm\n\
                 .model vm {kind}(LEVEL={level} IS=1e-40 IBEI=0 IBCI=0 ISP=0 IBEIP=1e-15 CJEP=10n PC=0.75 MC=0.33 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TNOM=27)\n.temp 27\n.options gmin=0\n.end\n",
                polarity * -0.1,
                polarity * 0.1,
            ));
            let base = branch_current(&point, "vb");
            let collector = branch_current(&point, "vc");
            assert!(
                (base + expected).norm() < 1e-8 * expected.norm(),
                "{level} {kind}: {base:?} != {:?}",
                -expected
            );
            assert!((collector - expected).norm() < 1e-8 * expected.norm());
            assert!(
                !point
                    .node_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case("Q1.__bp.internal"))
            );
        }
    }
}

#[test]
fn vbic13_extrinsic_avalanche_thermal_derivative_matches_xyce710() {
    for (level, kind, expected) in [
        (11, "NPN", [-1.108942174568846e-5, 3.050645763975771e-7]),
        (11, "PNP", [1.0242380896211347e-5, 4.874652997592307e-7]),
        (12, "NPN", [-1.1089201183715468e-5, 3.048295550222748e-7]),
        (12, "PNP", [1.0242370251186067e-5, 4.872579221019698e-7]),
    ] {
        let polarity = if kind == "PNP" { -1.0 } else { 1.0 };
        let substrate = if level == 12 { " 0" } else { "" };
        let point = solve_one(&format!(
            "VBIC13 avalanche thermal AC\nVc c 0 {}\nVb b 0 {}\nVth th 0 DC 20 AC 1\nQ1 c b 0{substrate} th vm SW_ET=0\n\
             .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVCX1=0.05 AVCX2=0.3 TAVCX=0.01 RTH=1000 GMIN=1e-6 TNOM=27)\n.temp 27\n.end\n",
            polarity * 1.8,
            polarity * 0.7,
        ));
        for (branch, expected) in ["vc", "vb"].into_iter().zip(expected) {
            let actual = branch_current(&point, branch);
            assert!(
                (actual.re - expected).abs() < 2e-7 * expected.abs(),
                "{level} {kind} {branch}: {actual:?} != {expected:e}"
            );
            assert_eq!(actual.im, 0.0);
        }
    }
}

#[test]
fn vbic13_maxexp_controls_forward_diffusion_capacitance() {
    // With unit base charge and VTF=0, Qbe = TF*(1+2*expLin(0))*If.
    // Driving the base directly exposes its differential capacitance.
    let vt = 1.380_662e-23 * 300.15 / 1.602_189e-19;
    for level in [11, 12] {
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            let point = solve_one(&format!(
                "VBIC13 limited diffusion charge\nVc c 0 {}\nVb b 0 DC {} AC 1\nQ1 c b 0{substrate} vm SW_ET=0\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=0 IBCI=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TF=1n XTF=2 MAXEXP=0.1 TNOM=27)\n.temp 27\n.end\n",
                polarity * 1.8,
                polarity * 0.7,
            ));
            let gm = 1e-16 * (0.7_f64 / vt).exp() / vt;
            let tf = 1e-9 * (1.0 + 2.0 * 0.1 * (1.0 - 0.1_f64.ln()));
            let expected = -std::f64::consts::TAU * 1e3 * gm * tf;
            assert!((branch_current(&point, "vb").im - expected).abs() < 1e-8 * expected.abs());
        }
    }
}

#[test]
fn vbic13_thermal_port_matches_temperature_dependent_rc_admittance() {
    // SW_ET=0 isolates the thermal R/C port. Its AC conductance is
    // d[theta/R(T)]/dtheta, not simply 1/R(T), and CTH has no hidden floor.
    for level in [11, 12] {
        let substrate = if level == 12 { " 0" } else { "" };
        for (rise, coefficient, effective_c, slope) in [
            (20.0, 0.005, 47.0, 1.0),
            (74.0, 0.005, 100.0 - (-2.0_f64).exp(), (-2.0_f64).exp()),
            (-78.0, 0.005, -50.0 + (-2.0_f64).exp(), (-2.0_f64).exp()),
            (20.0, -0.1, 47.0, 1.0),
        ] {
            for capacitance in [0.0, 1e-15, 2e-6] {
                let point = solve_one(&format!(
                    "VBIC13 thermal small-signal port\nVth th 0 DC {rise} AC 1\n\
                     Q1 0 0 0{substrate} th vm SW_ET=0 M=3\n\
                     .model vm NPN(LEVEL={level} RTH=1000 TCRTH={coefficient} CTH={capacitance} TMINCLIP=-50 TMAXCLIP=100 TNOM=27)\n.temp 27\n.end\n"
                ));
                let resistance = 1000.0 * (1.0 + coefficient * (effective_c - 27.0));
                let conductance = if resistance <= 1e-3 {
                    3e3
                } else {
                    3.0 / resistance
                        - rise * 3.0 * 1000.0 * coefficient * slope / resistance.powi(2)
                };
                let expected = -Complex64::new(
                    conductance,
                    2.0 * std::f64::consts::PI * 1e3 * capacitance * 3.0,
                );
                let actual = branch_current(&point, "Vth");
                assert!(
                    (actual - expected).norm() < 1e-10 * expected.norm().max(1e-6),
                    "LEVEL={level} rise={rise} TCRTH={coefficient} CTH={capacitance}: {actual:?} vs {expected:?}"
                );
            }
        }
    }
}

#[test]
fn one_ampere_through_one_hundred_teraohms_produces_one_hundred_teravolts() {
    let point = solve_one(
        "one-ampere weak-conductance transimpedance\n\
         I1 0 out DC 0 AC 1\n\
         R1 out 0 1e14\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    );
    let actual = voltage(&point, "out");
    let expected = Complex64::new(1.0e14, 0.0);
    let tolerance = 64.0 * f64::EPSILON * expected.norm();

    assert!(
        (actual - expected).norm() <= tolerance,
        "1 A through 1e14 ohm must produce 1e14 V, got {actual:?} (tolerance {tolerance:.3e})"
    );
}

#[test]
fn exact_and_near_zero_resistors_preserve_their_branch_constitutive_equation() {
    for resistance in ["0", "1e-101"] {
        let point = solve_one(&format!(
            "branch-form short AC constitutive equation\n\
             V1 a 0 DC 0 AC 1e14\n\
             RSHORT a b {resistance}\n\
             RLOAD b 0 1\n\
             .AC LIN 1 1k 1k\n\
             .END\n"
        ));
        let va = voltage(&point, "a");
        let vb = voltage(&point, "b");
        let current = branch_current(&point, "RSHORT");
        let resistance = resistance.parse::<f64>().unwrap();
        let constitutive_residual = va - vb - resistance * current;
        let voltage_tolerance = 2.0 * f64::EPSILON * va.norm().max(vb.norm()).max(1.0);
        let current_tolerance = 64.0 * f64::EPSILON * 1.0e14;

        assert!(
            constitutive_residual.norm() <= voltage_tolerance,
            "RSHORT={resistance:.1e} violates Va-Vb=R*I: Va={va:?}, Vb={vb:?}, I={current:?}, residual={constitutive_residual:?}"
        );
        assert!(
            (current - Complex64::new(1.0e14, 0.0)).norm() <= current_tolerance,
            "RSHORT={resistance:.1e} must carry the one-ohm load current, got {current:?}"
        );
    }
}

#[test]
fn genuinely_floating_differential_network_is_singular() {
    assert_solver_singular(
        "floating differential AC component\n\
         VREF reference 0 DC 0 AC 0\n\
         RREF reference 0 1k\n\
         VDIFF left right DC 0 AC 1\n\
         RDIFF left right 1k\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    );
}

#[test]
fn frequency_activated_behavioral_voltage_identity_is_singular() {
    // At DC, HERTZ is zero and BID prescribes V(out)=0, so the bias point is
    // well-defined. At an AC frequency the exact small-signal equation becomes
    // V(out)=V(out), leaving the behavioral branch current underdetermined.
    assert_solver_singular(
        "underdetermined behavioral AC identity\n\
         IEX 0 out DC 0 AC 1\n\
         BID out 0 V={V(out)*(HERTZ>0)}\n\
         RLOAD out 0 1k\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    );
}
