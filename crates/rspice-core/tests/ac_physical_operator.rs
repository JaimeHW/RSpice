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

#[test]
fn nonzero_current_excitations_have_no_absolute_amplitude_cutoff() {
    for magnitude in [1e-14, 1e-15, 1e-16, 1e-30, 1e-300] {
        for phase in [0.0_f64, 37.0, -90.0] {
            let point = solve_one(&format!(
                "small current excitation\nI1 0 out DC 0 AC {magnitude} {phase}\nR1 out 0 1\n.end\n"
            ));
            let scaled = voltage(&point, "out") / magnitude;
            let expected = Complex64::from_polar(1.0, phase.to_radians());
            assert!(
                (scaled - expected).norm() < 1e-14,
                "magnitude={magnitude}, phase={phase}: {scaled}"
            );
        }
    }
}

#[test]
fn tied_current_terminals_cannot_erase_other_ac_excitations() {
    for terminals in ["out out", "0 0"] {
        let point = solve_one(&format!(
            "tied AC current terminals\nI1 0 out DC 0 AC 1 37\nI2 {terminals} DC 0 AC 1e100 37\nR1 out 0 1\n.end\n"
        ));
        let expected = Complex64::from_polar(1.0, 37.0_f64.to_radians());
        assert!((voltage(&point, "out") - expected).norm() < 1e-14);
    }
}

#[test]
fn waveform_bias_is_retained_when_linearizing_a_nonlinear_ac_circuit() {
    for waveform in [
        "SIN(.65 .05 1meg)",
        "PULSE(.65 .7 0 1n 1n 5n 10n)",
        "PWL(0 .65 20n .7)",
    ] {
        let deck = |dc: &str| {
            format!(
                "biased diode\nV1 in 0 {waveform} AC 1 {dc}\nR1 in out 100\nD1 out 0 dm\n.model dm D IS=1e-14\n.end\n"
            )
        };
        let implicit = solve_one(&deck(""));
        let explicit = solve_one(&deck("DC .65"));
        let expected = voltage(&explicit, "out");
        assert!(expected.norm() < 0.8, "diode must load the biased circuit");
        assert!(
            (voltage(&implicit, "out") - expected).norm() < 1e-12,
            "{waveform}"
        );
    }
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
fn vbic13_extreme_early_thermal_slope_preserves_dc_and_ac_currents() {
    // Independent 80-digit DC/derivative evaluation and AC solve. The
    // VEF=5 case has a 9.26e-16 V RBI drop: deriving its current by subtracting
    // two rounded node voltages loses the thermal derivative's accuracy.
    for (vef, coefficient, collector, base) in [
        (
            1e15,
            -0.049_999_999_999_999_99,
            Complex64::new(11073210790.749592, -8091064247.664278),
            Complex64::new(91165705.28041226, 9220857782.629837),
        ),
        (
            5.0,
            -0.0499999999995,
            Complex64::new(0.0010482816638976908, -0.0007608625817640124),
            Complex64::new(4.054942295825502e-6, 0.000859797326861502),
        ),
        (
            5.0,
            -0.049_999_999_999_999_99,
            Complex64::new(0.0010482816655048935, -0.0007608625829294785),
            Complex64::new(4.0549423005197475e-6, 0.0008597973281960521),
        ),
    ] {
        for level in [11, 12] {
            for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
                let substrate = if level == 12 { " 0" } else { "" };
                let deck = format!(
                    "* VBIC extreme Early slope\nVc c 0 {}\nVb b 0 {}\nVth th 0 DC 20 AC 1\nQ1 c b 0{substrate} th vm SW_ET=0 M=3\n\
                .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 ISP=0 IBEIP=0 VEF={vef} VER=3 TCVEF={coefficient:.18} TCVER=-0.02 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p CJE=1p CJC=1p TF=1n TR=2n QTF=0.3 TD=1n)\n.temp 27\n.options gmin=0\n.end\n",
                    polarity * 0.6,
                    polarity * 0.7
                );
                let mut config = SimulationConfig::default();
                config.convergence_config.gmin_target = 0.0;
                let points = Engine::new(config)
                    .run_ac_with_abort(&parse(&deck), &[1e8], &rspice_core::abort_signal::NoAbort)
                    .unwrap();
                for (branch, expected) in [
                    ("vc", polarity * collector),
                    ("vb", polarity * base),
                    ("vth", Complex64::new(-0.003, -0.0018849555921538759)),
                ] {
                    let actual = branch_current(&points[0], branch);
                    assert!(
                        (actual - expected).norm() < 2e-6 * expected.norm(),
                        "VEF={vef} LEVEL={level} {kind} {branch}: {actual:?} != {expected:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn vbic13_early_voltage_cutoff_and_thermal_derivatives_match_xyce710() {
    // Xyce 7.10 differentiates the inactive reciprocal branch at exactly
    // zero Early voltage; a temperature probe must not turn it back on.
    for (level, rise, tolerance, expected) in [
        (
            11,
            20.0,
            2e-7,
            [
                Complex64::new(-8.663038807350953e-6, 6.275760707091533e-6),
                Complex64::new(4.317831416641038e-7, -4.178850145275066e-7),
                Complex64::new(-0.0008111544184809163, -0.000639322846682267),
            ],
        ),
        (
            12,
            20.0,
            2e-7,
            [
                Complex64::new(-8.6629450754821e-6, 6.275692403772111e-6),
                Complex64::new(4.3177660764799645e-7, -4.178801743108376e-7),
                Complex64::new(-0.0008111542711349811, -0.0006393229527764979),
            ],
        ),
        (
            11,
            19.99999,
            2e-6,
            [
                Complex64::new(5.5688894178522316e-5, -4.029266538390741e-5),
                Complex64::new(-3.828244129547701e-6, 2.6916652185934493e-6),
                Complex64::new(-0.0009240107317768997, -0.0005576729057381911),
            ],
        ),
    ] {
        let substrate = if level == 12 { " 0" } else { "" };
        let netlist = parse(&format!(
            "VBIC Early voltage cutoff\nVc c 0 1.8\nVb b 0 0.7\nVth th 0 DC {rise} AC 1\nQ1 c b 0{substrate} th vm SW_ET=1\n\
             .model vm NPN(LEVEL={level} VEF=5 VER=3 TCVEF=-0.05 TCVER=-0.05 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TAVC=0.01 TD=1n RTH=1000 TCRTH=0.005 TMAXCLIP=100 CTH=1p GMIN=1u TNOM=27)\n.temp 27\n.end\n"
        ));
        let point = Engine::default()
            .run_ac(&netlist, &[1e8])
            .unwrap()
            .pop()
            .unwrap();
        for (branch, expected) in ["vc", "vb", "vth"].into_iter().zip(expected) {
            let actual = branch_current(&point, branch);
            assert!(
                (actual - expected).norm() < tolerance * expected.norm(),
                "{level} rise={rise} {branch}: {actual:?} != {expected:?}"
            );
        }
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
                Complex64::new(-0.0037142943763058055, 0.00268656085433121),
                Complex64::new(0.00016865091916015706, -0.00017907067371343544),
                Complex64::new(0.006562379107882372, -0.004710659636530188),
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
                Complex64::new(3.458083802410476e-5, -2.3971443485324468e-5),
                Complex64::new(-1.2882306894777684e-6, 1.321168637960378e-6),
                Complex64::new(-0.002056839994804803, -0.001927181953865983),
            ],
        ),
    ] {
        let substrate = if level == 12 { " 0" } else { "" };
        let netlist = parse(&format!(
            "Delayed VBIC avalanche and heat\nVc c 0 {}\nVb b 0 DC {} AC {}\nVth th 0 DC {rise} AC {}\nQ1 c b 0{substrate} th vm SW_ET=1 M={multiplier}\n\
             .model vm {kind}(LEVEL={level} VEF=5 VER=3 TCVEF=0.05 TCVER=-0.02 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TAVC=0.01 TD=1n RTH=1000 TCRTH=0.005 TMAXCLIP=100 CTH=1p GMIN=1u TNOM=27)\n.temp 27\n.end\n",
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
fn vbic_tnf_thermal_derivatives_match_xyce710() {
    for (tnf, collector, base, expected) in [
        (
            -0.001,
            -0.5,
            0.1,
            [
                Complex64::new(2.8129342311083413e-7, 3.1081440800174e-7),
                Complex64::new(-3.378869297428202e-8, -3.1098483294827837e-7),
                Complex64::new(-0.0029998561029066935, -0.0018849556961110332),
            ],
        ),
        (
            0.001,
            1.8,
            0.7,
            [
                Complex64::new(-1.084013990699429e-6, 7.885275583603683e-7),
                Complex64::new(-8.501427197947536e-7, -9.013856542434545e-7),
                Complex64::new(-0.0029974552824991404, -0.0018863758535638618),
            ],
        ),
    ] {
        for level in [11, 12] {
            let substrate = if level == 12 { " 0" } else { "" };
            for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
                let netlist = parse(&format!(
                    "VBIC TNF thermal derivative\nVc c 0 {}\nVb b 0 {}\nVth th 0 DC 20 AC 1\nQ1 c b 0{substrate} th vm SW_ET=1 M=3 TRISE=20\n\
                     .model vm {kind}(LEVEL={level} IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF={tnf} XISR=1.8 DEAR=0.1 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p TD=1n TF=1n TR=2n)\n.temp 27\n.options gmin=0\n.end\n",
                    p * collector,
                    p * base
                ));
                let point = Engine::default()
                    .run_ac(&netlist, &[1e8])
                    .unwrap()
                    .pop()
                    .unwrap();
                for ((branch, polarity), expected) in [("Vc", p), ("Vb", p), ("Vth", 1.0)]
                    .into_iter()
                    .zip(expected)
                {
                    let actual = branch_current(&point, branch);
                    assert!(
                        (actual - polarity * expected).norm() < 2e-6 * expected.norm(),
                        "LEVEL={level} {kind} TNF={tnf} {branch}: {actual:?} != {:?}",
                        polarity * expected
                    );
                }
            }
        }
    }
}

#[test]
fn vbic13_reverse_charge_and_delay_match_xyce710() {
    // Independent 100 MHz Xyce 7.10 currents with signed transport,
    // a local temperature rise and both forms of the active qb floor.
    for (qbm, expected) in [
        (
            0,
            [
                Complex64::new(-1.2335268095787588e-6, 1.4925871005433984e-6),
                Complex64::new(-2.5912605394751154e-11, -4.08970566366307e-6),
            ],
        ),
        (
            1,
            [
                Complex64::new(-1.2339551736312433e-6, 1.4936538192949223e-6),
                Complex64::new(-2.595701431573616e-11, -4.092821900053193e-6),
            ],
        ),
    ] {
        for level in [11, 12] {
            let substrate = if level == 12 { " 0" } else { "" };
            for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
                let netlist = parse(&format!(
                    "VBIC reverse charge and delay\nVc c 0 {p}\nVb b 0 DC {} AC 1\nVth th 0 20\nQ1 c b 0{substrate} th vm SW_ET=0 M=3\n\
                     .model vm {kind}(LEVEL={level} IS=1e-8 ISRR=0.7 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=10 RS=10 GMIN=0 TNOM=27 IKF=1e-10 IKR=1e-10 QBM={qbm} NKF=0.4 VEF=3 VER=4 TF=2n TR=1n TD=1n QTF=0.5 XTF=10 ITF=1e-3)\n.temp 27\n.options gmin=0\n.end\n",
                    -0.1 * p
                ));
                let point = Engine::default()
                    .run_ac(&netlist, &[1e8])
                    .unwrap()
                    .pop()
                    .unwrap();
                for (branch, expected) in ["Vc", "Vb"].into_iter().zip(expected) {
                    let actual = branch_current(&point, branch);
                    assert!(
                        (actual - expected).norm() < 2e-6 * expected.norm(),
                        "LEVEL={level} {kind} QBM={qbm} {branch}: {actual:?} != {expected:?}"
                    );
                }
            }
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
fn vbic_zero_parasitic_resistance_preserves_its_junction_admittance() {
    // Older VBIC collapses RBP=0. VBIC 1.3 retains RBX, RBP, and RCX
    // at 1mOhm each in series with the BEP diode and charge.
    let vt = 1.380_662e-23 * 300.15 / 1.602_189e-19;
    let conductance = 1e-15 * (0.2_f64 / vt).exp() / vt;
    let capacitance = 1e-8 / (1.0_f64 - 0.2 / 0.75).powf(0.33);
    let expected = Complex64::new(conductance, std::f64::consts::TAU * 1e3 * capacitance);
    for level in [4, 11, 12] {
        let expected = if level >= 11 {
            expected / (1.0 + 3e-3 * expected)
        } else {
            expected
        };
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
            assert_eq!(
                point
                    .node_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case("Q1.__bp.internal")),
                level >= 11,
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
    // With constant base charge and VTF=0, Qbe = TF*(1+2*expLin(0))*If/qb.
    // Include the 1mOhm emitter and two base resistance floors in the
    // operating bias and small-signal degeneration.
    let vt = 1.380_662e-23 * 300.15 / 1.602_189e-19;
    let qb = 0.5 * (((1.0_f64 - 1e-4).powi(2) + 1e-8).sqrt() + 1.0 - 1e-4) + 1e-4;
    let mut vbe = 0.7_f64;
    for _ in 0..4 {
        vbe = 0.7 - 1e-3 * 1e-16 * (vbe / vt).exp() / qb;
    }
    let gm = 1e-16 * (vbe / vt).exp() / (vt * qb);
    let tf = 1e-9 * (1.0 + 2.0 * 0.1 * (1.0 - 0.1_f64.ln()));
    let yq = Complex64::new(0.0, std::f64::consts::TAU * 1e3 * gm * tf);
    let expected = -yq / (1.0 + 1e-3 * gm + 1e-3 * (2.0 + 1.0 / qb) * yq);
    for level in [11, 12] {
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            let point = solve_one(&format!(
                "VBIC13 limited diffusion charge\nVc c 0 {}\nVb b 0 DC {} AC 1\nQ1 c b 0{substrate} vm SW_ET=0\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=0 IBCI=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TF=1n XTF=2 MAXEXP=0.1 TNOM=27)\n.temp 27\n.end\n",
                polarity * 1.8,
                polarity * 0.7,
            ));
            assert!((branch_current(&point, "vb").im - expected.im).abs() < 1e-8 * expected.norm());
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

#[test]
fn vbic13_pnjmaxi_all_junctions_match_xyce710_dc_and_ac() {
    // Independent Xyce 7.10: different saturation currents in every junction,
    // active IKF/IKR/IKP rolloff, M=3, TRISE and a driven thermal terminal.
    for (level, expected_dc, expected_ac) in [
        (
            11,
            [
                0.0006780911623027872,
                -0.0008715674576252999,
                0.0,
                -0.0590799710590033,
            ],
            [
                Complex64::new(0.0011866741027589229, 0.002958788230962209),
                Complex64::new(-0.0017459812824673904, -0.00296024496477361),
                Complex64::new(0.0, 0.0),
                Complex64::new(-0.00032629244303326096, -0.0018838180623807796),
            ],
        ),
        (
            12,
            [
                0.0010942286843618383,
                -0.0008668472207152475,
                -0.0004208624407979479,
                -0.0585801295584971,
            ],
            [
                Complex64::new(0.0012095912965754297, 0.0029581042907695734),
                Complex64::new(-0.0017953891091435636, -0.0029600985063590974),
                Complex64::new(2.65031740606827e-05, 5.37512843421898e-07),
                Complex64::new(-0.0003084636587386225, -0.0018841134824596002),
            ],
        ),
    ] {
        let substrate = if level == 12 { " s" } else { "" };
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let netlist = parse(&format!(
                "VBIC PNJMAXI all junctions\nVc c 0 {}\nVb b 0 DC {} AC {p}\nVs s 0 {p}\nVth th 0 DC 20 AC 1\nQ1 c b 0{substrate} th vm SW_ET=1 M=3 TRISE=20\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 XISR=1.8 DEAR=0.1 IBEI=1e-17 IBEN=1e-12 NEN=1.5 IBCI=2e-17 IBCN=2e-12 NCN=1.5 IBEIP=3e-17 IBENP=3e-12 IBCIP=4e-17 IBCNP=4e-12 NCNP=1.5 ISP=2e-16 WSP=0.4 WBE=0.6 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=1 RS=1 GMIN=0 TNOM=27 PNJMAXI=1e-6 RTH=1000 CTH=1p TD=1n TF=1n TR=2n IKF=1e-7 IKR=2e-7 IKP=2e-7 NKF=0.4)\n.temp 27\n.options gmin=0\n.end\n",
                -0.2 * p,
                0.9 * p,
            ));
            let engine = Engine::default();
            let dc = engine.run_dc_op(&netlist).unwrap();
            let ac = engine.run_ac(&netlist, &[1e8]).unwrap().pop().unwrap();
            for (i, (branch, sign)) in [("Vc", p), ("Vb", p), ("Vs", p), ("Vth", 1.0)]
                .into_iter()
                .enumerate()
            {
                let current = dc.branch_current_named(branch).unwrap();
                assert!(
                    (current - sign * expected_dc[i]).abs() < 2e-6 * expected_dc[i].abs() + 1e-14,
                    "LEVEL={level} {kind} DC {branch}: {current}"
                );
                let current = branch_current(&ac, branch);
                assert!(
                    (current - sign * expected_ac[i]).norm() < 2e-6 * expected_ac[i].norm() + 1e-14,
                    "LEVEL={level} {kind} AC {branch}: {current}"
                );
            }
        }
    }
}
