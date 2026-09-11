use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use rspice_core::numerics::integration::IntegrationMethod;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const F0: f64 = 1.0e6;

#[test]
fn vbic_self_heated_delay_pss_matches_ngspice_and_retains_all_states() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    // ngspice 46: the NPN deck below, .tran .1n 20u 0 .1n, RELTOL=1e-7,
    // ABSTOL=1e-15, CHGTOL=1e-20, GMIN=0, TEMP=TNOM=27. Samples of the
    // settled 19--20 us cycle: [temperature rise K, XF1 A, XF2 A, I(VC), I(VB)].
    // Adjacent settled cycles agree within 1 nK and 0.1 pA.
    let reference = [
        [
            1.041077638037999e+00,
            6.148510238585312e-04,
            5.512131439845122e-04,
            -5.501013932401927e-04,
            -4.613003510449985e-05,
        ],
        [
            1.066520218984637e+00,
            1.030782930932765e-03,
            9.071921254690163e-04,
            -9.059728453870770e-04,
            -6.058080549928584e-05,
        ],
        [
            1.160121278249431e+00,
            1.525986690155254e-03,
            1.403162865590229e-03,
            -1.402446905055711e-03,
            -3.148136311295072e-05,
        ],
        [
            1.262109443709324e+00,
            1.666932333990812e-03,
            1.666945997940638e-03,
            -1.667459518545665e-03,
            2.758726760902989e-05,
        ],
        [
            1.289814930134844e+00,
            1.249361845072509e-03,
            1.383994125368064e-03,
            -1.385545316522035e-03,
            4.088852948242942e-05,
        ],
        [
            1.237627277822640e+00,
            7.230628060154461e-04,
            8.502299211956250e-04,
            -8.514960847744414e-04,
            1.688861056198725e-05,
        ],
        [
            1.155262105342785e+00,
            4.552662655006599e-04,
            5.094134103106141e-04,
            -5.097121812717492e-04,
            -2.633671005871926e-06,
        ],
        [
            1.080247271555298e+00,
            4.295708779729699e-04,
            4.236696989826053e-04,
            -4.230872781715032e-04,
            -2.176024389696356e-05,
        ],
    ];
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        let netlist = Netlist::parse(&format!(
            "VBIC coupled thermal/delay orbit\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1meg)\nQ1 c b 0 0 th qm\n.model qm {kind}(LEVEL=4 IS=1e-14 IBEI=1e-16 IBCI=1e-16 RCX=10 RCI=20 RBX=10 RBI=40 RE=1 RBP=10 RS=1 CJE=10p CJC=5p CJEP=3p CJCP=2p TF=10n TR=2n QCO=10f GAMM=1e-9 ISP=1e-16 WBE=.8 SELFT=1 RTH=1000 CTH=1n TD=100n)\n.options GMIN=0\n.temp 27\n.end",
            1.2*p, 0.65*p, 0.65*p, 0.02*p,
        )).unwrap();
        let mut config = SimulationConfig::default();
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let (analysis, state) = engine
            .run_pss_with_continuation_state(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(512)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-9),
            )
            .unwrap_or_else(|error| panic!("{kind}: {error}"));
        let names = ["th", "Q1.__xf1.internal", "Q1.__xf2.internal"];
        for (column, name) in names.into_iter().enumerate() {
            let node = analysis
                .result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            let tolerance = if column == 0 { 1e-4 } else { 3e-7 };
            for (phase, expected) in reference.iter().enumerate() {
                let actual = analysis.result.waveforms[node].values[phase * 64];
                assert!(
                    (actual - expected[column]).abs() < tolerance,
                    "{kind} PSS {name} phase={phase}: {actual} vs {}",
                    expected[column]
                );
            }
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&netlist, &state, 1e-6, 1e-9)
            .unwrap();
        let traces = [
            continued.try_voltage_waveform_named(names[0]).unwrap(),
            continued.try_voltage_waveform_named(names[1]).unwrap(),
            continued.try_voltage_waveform_named(names[2]).unwrap(),
            continued.try_branch_current_waveform_named("VC").unwrap(),
            continued.try_branch_current_waveform_named("VB").unwrap(),
        ];
        for (phase, expected) in reference.iter().enumerate() {
            let time = phase as f64 / (8.0 * F0);
            let hi = continued.time.partition_point(|&t| t < time);
            let lo = hi.saturating_sub(1);
            let fraction = if lo == hi {
                0.0
            } else {
                (time - continued.time[lo]) / (continued.time[hi] - continued.time[lo])
            };
            for (column, tolerance) in [1e-4, 3e-7, 3e-7, 3e-7, 5e-8].into_iter().enumerate() {
                let trace = traces[column];
                let sign = if column < 3 { 1.0 } else { p };
                let actual = sign * (trace[lo] + fraction * (trace[hi] - trace[lo]));
                assert!(
                    (actual - expected[column]).abs() < tolerance,
                    "{kind} continuation column={column} phase={phase}: {actual} vs {}",
                    expected[column]
                );
            }
        }
        let (direct, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.2e-6, 1e-9)
            .unwrap();
        for encoding in [
            TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Packed,
        ] {
            let restored =
                TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &restored, 1.2e-6, 1e-9)
                .unwrap();
            assert_eq!(resumed.time, direct.time);
            assert_eq!(resumed.voltages, direct.voltages);
            assert_eq!(resumed.branch_currents, direct.branch_currents);
        }
    }
}

// ngspice 46, identical NPN card below: TRAP, RELTOL=1e-9,
// ABSTOL=1e-18, CHGTOL=1e-22, GMIN=0, max step 0.1 ns.
// Linear interpolation at eight phases of the settled 19--20 us cycle:
// [intrinsic VBE, I(VC), I(VB)]. No private RSpice solver supplies this oracle.
const GP_PERIODIC_REFERENCE: [[f64; 3]; 8] = [
    [
        6.30578767042357602e-1,
        -2.98997251485324980e-4,
        -1.41771813619889610e-5,
    ],
    [
        6.46714289880719662e-1,
        -4.89399659214451318e-4,
        -1.89337801930940490e-5,
    ],
    [
        6.58914686737243738e-1,
        -6.93674739233193999e-4,
        -1.70222715296312860e-5,
    ],
    [
        6.60355903066060068e-1,
        -7.23571014206733447e-4,
        -7.55605874014792859e-6,
    ],
    [
        6.50928684442999894e-1,
        -5.57669368421028606e-4,
        2.57948906010152257e-6,
    ],
    [
        6.35716106317155916e-1,
        -3.55988591001042383e-4,
        5.87537526912878170e-6,
    ],
    [
        6.22950573595932755e-1,
        -2.36858621669185071e-4,
        1.77974652209541728e-6,
    ],
    [
        6.20579689520069921e-1,
        -2.17246220750473361e-4,
        -6.21136165965052330e-6,
    ],
];

#[test]
fn gummel_poon_nonlinear_pss_matches_ngspice_and_retains_its_orbit() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let reference = GP_PERIODIC_REFERENCE;
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        let netlist = Netlist::parse(&format!(
            "Nonlinear GP periodic reference\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1meg)\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-14 BF=100 VAF=50 IKF=1m RB=2k RBM=100 CJE=30p CJC=20p TF=2n XCJC=.4)\n.end\n",
            p*1.2, p*0.65, p*0.65, p*0.03,
        )).unwrap();
        let mut config = SimulationConfig::default();
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let (analysis, state) = engine
            .run_pss_with_continuation_state(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(512)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-10),
            )
            .unwrap_or_else(|error| panic!("{kind} periodic solve: {error}"));
        let node = analysis
            .result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("Q1.__bi.internal"))
            .unwrap();
        for (phase, expected) in reference.iter().enumerate() {
            let actual = p * analysis.result.waveforms[node].values[phase * 64];
            assert!(
                (actual - expected[0]).abs() < 4e-6,
                "{kind} PSS phase={phase}/8: {actual} vs {}",
                expected[0]
            );
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&netlist, &state, 1e-6, 1e-9)
            .unwrap();
        let traces = [
            continued
                .try_voltage_waveform_named("Q1.__bi.internal")
                .unwrap(),
            continued.try_branch_current_waveform_named("VC").unwrap(),
            continued.try_branch_current_waveform_named("VB").unwrap(),
        ];
        for (phase, expected) in reference.iter().enumerate() {
            let time = phase as f64 / (8.0 * F0);
            let hi = continued.time.partition_point(|&t| t < time);
            let lo = hi.saturating_sub(1);
            let fraction = if lo == hi {
                0.0
            } else {
                (time - continued.time[lo]) / (continued.time[hi] - continued.time[lo])
            };
            for (column, tolerance) in [4e-6, 5e-8, 5e-9].into_iter().enumerate() {
                let trace = traces[column];
                let actual = p * (trace[lo] + fraction * (trace[hi] - trace[lo]));
                assert!(
                    (actual - expected[column]).abs() < tolerance,
                    "{kind} continuation phase={phase}/8 column={column}: {actual} vs {}",
                    expected[column]
                );
            }
        }
        let (direct, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.2e-6, 1e-9)
            .unwrap();
        for encoding in [
            TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Packed,
        ] {
            let restored =
                TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &restored, 1.2e-6, 1e-9)
                .unwrap();
            assert_eq!(resumed.time, direct.time);
            assert_eq!(resumed.voltages, direct.voltages);
            assert_eq!(resumed.branch_currents, direct.branch_currents);
        }
    }
}

#[test]
fn nonlinear_descriptor_preserves_physical_transient_continuation() {
    let deck=Netlist::parse("Implicit nonlinear continuation\nV1 src 0 SIN(0.6 0.1 1)\nR1 src in 100\nR2 in 0 200\nD1 in 0 DM\n.model DM D(IS=1e-12)\nE1 out 0 in 0 2\nCout out 0 1u\n.end\n").unwrap();
    let engine = Engine::default();
    let (analysis, state) = engine
        .run_pss_with_continuation_state(
            &deck,
            PssConfig::new(1.0)
                .with_points_per_period(128)
                .with_tstab_periods(0),
        )
        .unwrap();
    assert!(analysis.monodromy.is_empty());
    let (continued, _) = engine
        .run_tran_from_pss_state(&deck, &state, 0.1, 0.001)
        .unwrap();
    let node = |name: &str| {
        continued
            .node_names
            .iter()
            .position(|entry| entry.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let branch = |name: &str| {
        continued
            .branch_names
            .iter()
            .position(|entry| entry.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let first = analysis
        .result
        .node_names
        .iter()
        .position(|entry| entry.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(
        (continued.voltages[node("out")][0] - analysis.result.waveforms[first].values[0]).abs()
            < 2e-11
    );
    for (index, &time) in continued.time.iter().enumerate() {
        let input = continued.voltages[node("in")][index];
        let output = continued.voltages[node("out")][index];
        assert!((output - 2.0 * input).abs() < 2e-11, "t={time}");
        assert!(
            (continued.branch_currents[branch("E1")][index]
                + continued.branch_currents[branch("Cout")][index])
                .abs()
                < 2e-11
        );
        let source = 0.6 + 0.1 * (std::f64::consts::TAU * time).sin();
        assert!(
            ((source - input) / 100.0
                - input / 200.0
                - continued.branch_currents[branch("D1")][index])
                .abs()
                < 2e-10
        );
    }
}

#[test]
fn coupled_descriptor_winding_state_survives_transient_continuation() {
    for source in [
        "V1 in 0 SIN(0.7 1 1 0 0 37)",
        "B1 in 0 V=0.7+sin(2*pi*time+37*pi/180)",
    ] {
        let deck = Netlist::parse(&format!("Coupled flux continuation\n{source}\nL1 in mid 0.1\nR1 mid 0 1\nH1 out 0 L1 2\nCout out 0 0.2\n.end\n")).unwrap();
        let engine = Engine::default();
        let (analysis, state) = engine
            .run_pss_with_continuation_state(
                &deck,
                PssConfig::new(1.0)
                    .with_points_per_period(1024)
                    .with_tstab_periods(0),
            )
            .unwrap();
        assert_eq!(analysis.monodromy.len(), 1);
        let (continued, _) = engine
            .run_tran_from_pss_state(&deck, &state, 0.1, 0.001)
            .unwrap();
        let output = continued
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let branch = |name: &str| {
            continued
                .branch_names
                .iter()
                .position(|entry| entry.eq_ignore_ascii_case(name))
                .unwrap()
        };
        for (index, &time) in continued.time.iter().enumerate() {
            let omega = std::f64::consts::TAU;
            let phase = omega * time + 37_f64.to_radians();
            let lag = omega * 0.1;
            let expected = 0.7 + (phase.sin() - lag * phase.cos()) / (1.0 + lag * lag);
            let current = continued.branch_currents[branch("L1")][index];
            assert!(
                (current - expected).abs() < 2e-5,
                "t={time}: {current} vs {expected}"
            );
            assert!((continued.voltages[output][index] - 2.0 * current).abs() < 2e-12);
            assert!(
                (continued.branch_currents[branch("H1")][index]
                    + continued.branch_currents[branch("Cout")][index])
                    .abs()
                    < 1e-10
                        * continued.branch_currents[branch("H1")][index]
                            .abs()
                            .max(1.0)
            );
        }
    }
}

#[test]
fn vcvs_dependent_charge_survives_transient_continuation() {
    let deck = Netlist::parse("Controlled charge continuation\nI1 0 in SIN(0 1 1)\nR1 in 0 1\nC1 in 0 0.1\nE1 out 0 in 0 2\nC2 out 0 0.2\n.end\n").unwrap();
    let engine = Engine::default();
    let (analysis, state) = engine
        .run_pss_with_continuation_state(
            &deck,
            PssConfig::new(1.0)
                .with_points_per_period(1024)
                .with_tstab_periods(0),
        )
        .unwrap();
    assert_eq!(analysis.monodromy.len(), 1);
    let (continued, _) = engine
        .run_tran_from_pss_state(&deck, &state, 0.1, 0.001)
        .unwrap();
    let node = |name: &str| {
        continued
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let branch = |name: &str| {
        continued
            .branch_names
            .iter()
            .position(|branch| branch.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let omega = std::f64::consts::TAU;
    for (index, &time) in continued.time.iter().enumerate() {
        let vin = continued.voltages[node("in")][index];
        let vout = continued.voltages[node("out")][index];
        let expected = ((omega * time).sin() - 0.1 * omega * (omega * time).cos())
            / (1.0 + (0.1 * omega).powi(2));
        assert!(
            (vin - expected).abs() < 3e-5,
            "t={time}: {vin} vs {expected}"
        );
        assert!((vout - 2.0 * vin).abs() < 2e-12);
        assert!(
            (continued.branch_currents[branch("E1")][index]
                + continued.branch_currents[branch("C2")][index])
                .abs()
                < 2e-12
        );
    }
}

#[test]
fn classic_jfet_pss_continues_the_analytic_orbit_and_persists_its_history() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
        let netlist = Netlist::parse(&format!(
            "JFET periodic continuation\nV1 in 0 DC {} SIN({} {} 1meg)\nR1 in out 1k\nJ1 0 out 0 jm 2 M=3\n.model jm {kind}(IS=0 CGS=100p CGD=50p M=0)\n.end\n",
            -polarity, -polarity, 0.01 * polarity,
        )).unwrap();
        let engine = Engine::default();
        let (_, state) = engine
            .run_pss_with_continuation_state(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(256)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-10),
            )
            .unwrap();
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&netlist, &state, 1e-6, 2e-9)
            .unwrap();
        let output = continued.try_voltage_waveform_named("out").unwrap();
        let wc = std::f64::consts::TAU * F0 * 1e3 * 900e-12;
        for (&time, &actual) in continued.time.iter().zip(output) {
            let phase = std::f64::consts::TAU * F0 * time;
            let expected =
                -polarity + polarity * 0.01 * (phase.sin() - wc * phase.cos()) / (1.0 + wc * wc);
            assert!(
                (actual - expected).abs() < 4e-6,
                "{kind}: t={time:e}, {actual} vs {expected}"
            );
        }
        let (direct, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.2e-6, 2e-9)
            .unwrap();
        for encoding in [
            TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Packed,
        ] {
            let restored =
                TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &restored, 1.2e-6, 2e-9)
                .unwrap();
            assert_eq!(resumed.time, direct.time);
            assert_eq!(resumed.voltages, direct.voltages);
            assert_eq!(resumed.branch_currents, direct.branch_currents);
        }
    }
}

#[test]
fn vbic_periodic_charge_and_continuation_match_analytic_rc() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    for (polarity, level) in [("NPN", 4), ("PNP", 4), ("NPN", 11), ("PNP", 11)] {
        let netlist = Netlist::parse(&format!("VBIC periodic charge\nV1 in 0 SIN(0 0.1 1meg)\nR1 in out 1k\nQ1 0 out 0 0 vm\n.model vm {polarity}(LEVEL={level} IS=1e-40 IBEI=1e-40 IBCI=1e-40 CJE=100p CJC=20p MJE=0 MJC=0 TF=0 TR=0 CBEO=30p CBCO=9p RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 CJEP=0 CJCP=0 CCSO=0 QCO=0 GAMM=0 ISP=0)\n.end\n")).unwrap();
        let engine = Engine::default();
        let (analysis, state) = engine
            .run_pss_with_continuation_state(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(64)
                    .with_tstab_periods(0),
            )
            .unwrap_or_else(|error| panic!("{polarity}: {error}"));
        let node = |names: &[String]| {
            names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap()
        };
        let exact = |time: f64| {
            let phase = std::f64::consts::TAU * F0 * time;
            let wc = std::f64::consts::TAU * F0 * 1e3 * 159e-12;
            0.1 * (phase.sin() - wc * phase.cos()) / (1.0 + wc * wc)
        };
        for (&time, &value) in analysis
            .result
            .time
            .iter()
            .zip(&analysis.result.waveforms[node(&analysis.result.node_names)].values)
        {
            assert!(
                (value - exact(time)).abs() < 5e-5,
                "{polarity} periodic t={time:e}: {value} != {}",
                exact(time)
            );
        }
        // VBIC 1.3's resistance floors retain separate BE/BC charge modes.
        // Independent 65-digit poles of the resulting passive RC network;
        // compare the actual BE-first/TRAP period map, including its small
        // fast-mode remnants, rather than assuming ideal shorts.
        let modes = if level == 11 { 3 } else { 1 };
        assert_eq!(analysis.monodromy.len(), modes);
        let mut multipliers = analysis.floquet_multipliers.clone();
        multipliers.sort_by(|a, b| b.norm().total_cmp(&a.norm()));
        assert_eq!(multipliers.len(), modes);
        let poles = if level == 11 {
            vec![
                -6.289298324578039e6,
                -1.1834562758609295e13,
                -2.1530841221033938e13,
            ]
        } else {
            vec![-1.0 / (1e3 * 159e-12)]
        };
        let mut expected: Vec<f64> =
            poles
                .into_iter()
                .map(|pole| {
                    analysis.result.time.windows(2).enumerate().fold(
                        1.0,
                        |value, (index, times)| {
                            let z = pole * (times[1] - times[0]);
                            value
                                * if index == 0 {
                                    1.0 / (1.0 - z)
                                } else {
                                    (1.0 + z / 2.0) / (1.0 - z / 2.0)
                                }
                        },
                    )
                })
                .collect();
        expected.sort_by(|a, b| b.abs().total_cmp(&a.abs()));
        for (actual, expected) in multipliers.iter().zip(expected) {
            assert!(
                (actual - expected).norm() < 2e-5,
                "{polarity} LEVEL={level}: {actual:?} != {expected:e}"
            );
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&netlist, &state, 1e-6, 2e-9)
            .unwrap();
        for (&time, &value) in continued
            .time
            .iter()
            .zip(&continued.voltages[node(&continued.node_names)])
        {
            assert!(
                (value - exact(time)).abs() < 5e-5,
                "{polarity} continuation t={time:e}: {value} != {}",
                exact(time)
            );
        }
        let (direct, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.2e-6, 2e-9)
            .unwrap();
        for encoding in [
            TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Packed,
        ] {
            let restored =
                TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &restored, 1.2e-6, 2e-9)
                .unwrap();
            assert_eq!(direct.time, resumed.time);
            assert_eq!(direct.voltages, resumed.voltages);
            assert_eq!(direct.branch_currents, resumed.branch_currents);
        }
    }
}

#[test]
fn refined_pss_preserves_default_pulse_edges_through_checkpoint_resume() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let netlist = Netlist::parse("refined source defaults\nVp pulse 0 PULSE(0 1 0 0 0 0.4u 1u)\nRp pulse 0 1k\nB1 in 0 V=sin(2*pi*64meg*time)^4\nR1 in out 1k\nC1 out 0 159.154943091895p\n.end\n").unwrap();
    let engine = Engine::default();
    let config = PssConfig::new(F0)
        .with_points_per_period(32)
        .with_tstab_periods(0);
    let (analysis, state) = engine
        .run_pss_with_continuation_state(&netlist, config)
        .unwrap();
    assert!(analysis.result.time.len() > 513);
    let expected = |time: f64| {
        let phase = time.rem_euclid(1e-6);
        let edge = 1e-6 / 32.0;
        if phase < edge {
            phase / edge
        } else if phase < edge + 0.4e-6 {
            1.0
        } else if phase < 2.0 * edge + 0.4e-6 {
            (2.0 * edge + 0.4e-6 - phase) / edge
        } else {
            0.0
        }
    };
    let pulse = analysis
        .result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("pulse"))
        .unwrap();
    for (&time, &voltage) in analysis
        .result
        .time
        .iter()
        .zip(&analysis.result.waveforms[pulse].values)
    {
        assert!(
            (voltage - expected(time)).abs() < 1e-10,
            "orbit t={time:e}: {voltage}"
        );
    }
    let (_, checkpoint) = engine
        .run_tran_from_pss_state(&netlist, &state, 1.2e-6, 2e-10)
        .unwrap();
    let (direct, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2.1e-6, 2e-10)
        .unwrap();
    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let restored =
            TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &restored, 2.1e-6, 2e-10)
            .unwrap();
        assert_eq!(resumed.time, direct.time);
        assert_eq!(resumed.voltages, direct.voltages);
        assert_eq!(resumed.branch_currents, direct.branch_currents);
        let pulse = resumed
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("pulse"))
            .unwrap();
        for (&time, &voltage) in resumed.time.iter().zip(&resumed.voltages[pulse]) {
            assert!(
                (voltage - expected(time)).abs() < 1e-10,
                "{encoding:?}, t={time:e}: {voltage}"
            );
        }
    }
}

#[test]
fn autonomous_startup_kick_remains_quiet_on_the_orbit_and_reactivates_after_saved_continuation() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let netlist = Netlist::parse("oscillator startup continuation\nL1 osc 0 1u\nC1 osc 0 1u\nB1 osc 0 I=-0.05*v(osc)+0.025*v(osc)*v(osc)*v(osc)\nI1 0 kick PULSE(0 1 10u 10n 10n 1u 1)\nRkick kick osc 1\n.end\n").unwrap();
    let engine = Engine::default();
    let (analysis, state) = engine
        .run_pss_with_continuation_state(
            &netlist,
            PssConfig::autonomous()
                .with_period_guess(6.3e-6)
                .with_tstab_periods(30)
                .with_tolerance(1e-6)
                .with_max_iterations(60),
        )
        .unwrap();
    let node = |names: &[String], name: &str| {
        names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let osc = node(&analysis.result.node_names, "osc");
    let kick = node(&analysis.result.node_names, "kick");
    for (&vosc, &vkick) in analysis.result.waveforms[osc]
        .values
        .iter()
        .zip(&analysis.result.waveforms[kick].values)
    {
        assert!(
            (vkick - vosc).abs() < 1e-10,
            "startup must not drive the periodic orbit"
        );
    }
    let (_, checkpoint) = engine
        .run_tran_from_pss_state(&netlist, &state, 8e-6, 1e-8)
        .unwrap();
    let (direct, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 12e-6, 1e-8)
        .unwrap();
    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let restored =
            TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &restored, 12e-6, 1e-8)
            .unwrap();
        assert_eq!(resumed.time, direct.time);
        assert_eq!(resumed.voltages, direct.voltages);
        let osc = node(&resumed.node_names, "osc");
        let kick = node(&resumed.node_names, "kick");
        let mut plateau = 0;
        for ((&time, &vosc), &vkick) in resumed
            .time
            .iter()
            .zip(&resumed.voltages[osc])
            .zip(&resumed.voltages[kick])
        {
            if (10.1e-6..10.9e-6).contains(&time) {
                plateau += 1;
                assert!(
                    (vkick - vosc - 1.0).abs() < 1e-9,
                    "{encoding:?}, t={time:e}: startup source must reactivate at its authored time"
                );
            }
        }
        assert!(plateau > 10);
    }
}

#[test]
fn source_defaults_survive_pss_continuation_and_persisted_transient_segments() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    for (dialect, nox) in [
        (SpiceDialect::Ngspice, false),
        (SpiceDialect::Xyce, false),
        (SpiceDialect::Xyce, true),
    ] {
        let source = if dialect == SpiceDialect::Xyce {
            "SFFM(0 1)"
        } else {
            "SIN(0 1 0)"
        };
        let netlist = Netlist::parse(&format!(
            "PSS source default continuation\nV1 in 0 {source}\nC1 in 0 1p\n.tran 5n 7u\n.end\n",
        ))
        .unwrap();
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            let engine = Engine::new(SimulationConfig {
                spice_dialect: dialect,
                transient_nonlinear_nox: Some(nox),
                integration_method: method,
                ..Default::default()
            });
            let (_, state) = engine
                .run_pss_with_continuation_state(
                    &netlist,
                    PssConfig::new(F0)
                        .with_points_per_period(128)
                        .with_tstab_periods(0),
                )
                .unwrap();
            let (continued, checkpoint) = engine
                .run_tran_from_pss_state(&netlist, &state, 1.31e-6, 3e-9)
                .unwrap();
            for encoding in [
                TransientCheckpointEncoding::Unpacked,
                TransientCheckpointEncoding::Packed,
            ] {
                let restored =
                    TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap())
                        .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&netlist, &restored, 2.57e-6, 2e-9)
                    .unwrap();
                let (direct, _) = engine
                    .run_tran_resume(&netlist, &checkpoint, 2.57e-6, 2e-9)
                    .unwrap();
                assert_eq!(resumed.time, direct.time);
                assert_eq!(resumed.voltages, direct.voltages);
                for result in [&continued, &resumed] {
                    let node = result
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case("in"))
                        .unwrap();
                    for (&time, &actual) in result.time.iter().zip(&result.voltages[node]) {
                        let expected = (std::f64::consts::TAU * F0 * time).sin();
                        assert!(
                            (actual - expected).abs() < 1e-11,
                            "{dialect:?}, {method:?}, {encoding:?}, t={time:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

fn envelope_startup_deck() -> Netlist {
    Netlist::parse(
        "* carrier plus a slower modulation source\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("envelope startup deck parses")
}

fn compact_pss_config() -> PssConfig {
    PssConfig::new(F0)
        .with_harmonics(4)
        .with_points_per_period(32)
        // Keep these continuation-contract tests on the deterministic
        // fixed-grid shooting path. Adaptive stabilization has its own guard
        // regression in the PSS unit tests.
        .with_tstab_periods(0)
        .with_tolerance(1.0e-6)
}

struct TemporaryFile(PathBuf);

impl TemporaryFile {
    fn new(label: &str, contents: &str) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-pss-continuation-{label}-{}-{id}.csv",
            std::process::id()
        ));
        std::fs::write(&path, contents).expect("temporary dependency is writable");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn replace(&self, contents: &str) {
        std::fs::write(&self.0, contents).expect("temporary dependency can be replaced");
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn frozen_modulation_source_is_authenticated_and_reactivated_at_time_zero() {
    let netlist = envelope_startup_deck();
    let engine = Engine::new(SimulationConfig::default());
    let (pss, state) = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["vMoD".to_string()],
        )
        .expect("frozen-source PSS produces a continuation state");

    assert_eq!(state.time_origin(), 0.0);
    assert!((state.period() - 1.0 / F0).abs() <= 4.0 * f64::EPSILON / F0);
    assert_eq!(pss.result.time.last().copied(), Some(state.period()));
    assert_eq!(state.frozen_sources(), &["VMOD".to_string()]);
    let pss_mod_index = pss
        .result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("mod"))
        .expect("mod node is present in PSS");
    assert!(
        pss.result.waveforms[pss_mod_index]
            .values
            .iter()
            .all(|value| value.abs() < 1.0e-12),
        "the selected modulation source must remain frozen throughout PSS"
    );

    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 600.0e-9, 10.0e-9)
        .expect("original modulation waveform reactivates from the authenticated state");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    let mod_index = transient
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("mod"))
        .expect("mod node is present");
    let mod_waveform = &transient.voltages[mod_index];
    assert!(mod_waveform.first().copied().unwrap_or_default().abs() < 1.0e-12);
    assert!(
        mod_waveform.iter().copied().fold(0.0_f64, f64::max) > 0.99,
        "the original PULSE source must be active after the PSS-to-transient seam"
    );
}

#[test]
fn linear_rl_continuation_retains_the_exact_supported_inductor_path() {
    let netlist = Netlist::parse(
        "* stable driven RL circuit\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out 10\n\
         L1 out 0 1u\n\
         .end\n",
    )
    .expect("linear RL deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (pss, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .expect("ordinary R/L and independent sources have an exact continuation contract");

    assert_eq!(pss.result.time.last().copied(), Some(state.period()));
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect("the retained inductor history resumes in transient");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    assert_eq!(transient.time.last().copied(), Some(100.0e-9));
}

#[test]
fn continuation_artifact_rejects_different_netlist_or_engine_configuration() {
    let netlist = envelope_startup_deck();
    let engine = Engine::new(SimulationConfig::default());
    let (_, state) = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmod".to_string()],
        )
        .expect("continuation state");

    let changed_deck = Netlist::parse(
        "* changed carrier resistance\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1.1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("changed deck parses");
    let deck_error = engine
        .run_tran_from_pss_state(&changed_deck, &state, 100.0e-9, 10.0e-9)
        .expect_err("artifact must not cross semantic netlist identity");
    assert!(
        deck_error.to_string().contains("different netlist"),
        "unexpected identity error: {deck_error}"
    );

    let mut changed_config = SimulationConfig::default();
    changed_config.temperature += 10.0;
    let changed_engine = Engine::new(changed_config);
    let config_error = changed_engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect_err("artifact must not cross resolved simulation configuration identity");
    assert!(
        config_error
            .to_string()
            .contains("different resolved simulation configuration"),
        "unexpected configuration identity error: {config_error}"
    );
}

#[test]
fn continuation_artifact_authenticates_external_waveform_bytes() {
    let waveform = TemporaryFile::new("pwl", "0,0\n0.000001,1\n");
    let path = waveform.path().to_string_lossy().replace('\\', "/");
    let netlist = Netlist::parse(&format!(
        "* external modulation dependency\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PWL FILE=\"{path}\"\n\
         Rcarrier carrier out 1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n"
    ))
    .expect("PWL FILE deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, state) = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmod".to_string()],
        )
        .expect("the original external waveform snapshot is authenticated");

    waveform.replace("0,0\n0.000001,2\n");
    let error = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect_err("changed external waveform bytes must invalidate the artifact");
    assert!(
        error.to_string().contains("different netlist"),
        "unexpected external dependency identity error: {error}"
    );
}

#[test]
fn memoryless_diode_supports_periodic_transient_continuation() {
    let netlist = Netlist::parse(
        "memoryless diode\nV1 in 0 SIN(-1 0.01 1meg)\nR1 in out 1k\n\
         C1 out 0 100p\nD1 out 0 DMOD\n.model DMOD D(CJO=0 TT=0)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let (_, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .unwrap();
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 1e-6, 1e-8)
        .unwrap();
    assert!(
        transient
            .voltages
            .iter()
            .flatten()
            .all(|value| value.is_finite())
    );
}

#[test]
fn charged_diode_continuation_preserves_the_periodic_orbit_from_time_zero() {
    let netlist = Netlist::parse(
        "charged diode continuation\nV1 in 0 SIN(-1 0.01 1meg)\nR1 in out 1k\n\
         D1 out 0 DMOD\n.model DMOD D(IS=1e-30 CJO=1n M=0 TT=0)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let (_, state) = engine
        .run_pss_with_continuation_state(
            &netlist,
            compact_pss_config()
                .with_points_per_period(512)
                .with_tolerance(1e-10),
        )
        .expect("PSS captures the charged diode's accepted state");
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 2e-6, 1e-6 / 1024.0)
        .expect("charged periodic history resumes through the ordinary transient engine");
    let ratio = std::f64::consts::TAU;
    let amplitude = 0.01 / (1.0 + ratio * ratio).sqrt();
    for (&time, &voltage) in transient
        .time
        .iter()
        .zip(transient.try_voltage_waveform_named("OUT").unwrap())
    {
        let expected = -1.0 + amplitude * (std::f64::consts::TAU * F0 * time - ratio.atan()).sin();
        assert!(
            (voltage - expected).abs() < 0.002 * amplitude,
            "charge continuation drift at {time:e}: {voltage:e} versus {expected:e}"
        );
    }
}

#[test]
fn continuation_fails_closed_for_unadvanced_dynamic_state_families() {
    let engine = Engine::new(SimulationConfig::default());

    let coupled = Netlist::parse(
        "* perfect coupling needs independent flux coordinates\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in p 10\n\
         L1 p 0 1u\n\
         L2 out 0 2u\n\
         K1 L1 L2 1\n\
         R2 out 0 100\n\
         .end\n",
    )
    .expect("coupled-inductor deck parses");
    let coupled_error = engine
        .run_pss_with_continuation_state(&coupled, compact_pss_config())
        .expect_err("singular magnetic flux must fail before the periodic solve");
    assert!(
        coupled_error
            .to_string()
            .contains("coupled-inductor flux constraints"),
        "unexpected coupled-inductor diagnostic: {coupled_error}"
    );

    let behavioral = Netlist::parse(
        "* behavioral accepted-step expression memory is not in shooting x\n\
         V1 in 0 SIN(0 1 1meg)\n\
         B1 out 0 V={SDT(V(in))}\n\
         R1 out 0 1k\n\
         C1 out 0 100p\n\
         .end\n",
    )
    .expect("behavioral deck parses");
    let behavioral_error = engine
        .run_pss_with_continuation_state(&behavioral, compact_pss_config())
        .expect_err("behavioral accepted-step memory must fail before solving");
    assert!(
        behavioral_error
            .to_string()
            .contains("behavioral-source accepted-step memory"),
        "unexpected behavioral-state diagnostic: {behavioral_error}"
    );

    let solution_dependent_capacitor = Netlist::parse_with_options(
        "* expression-valued capacitor charge is outside the shooting state\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out 1k\n\
         C1 out 0 C={100p*(1+0.1*V(out))}\n\
         .end\n",
        NetlistParseOptions {
            expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .expect("solution-dependent capacitor deck parses");
    let solution_dependent_error =
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_pss_with_continuation_state(&solution_dependent_capacitor, compact_pss_config())
            .expect_err("solution-dependent capacitor history must fail before solving");
    assert!(
        solution_dependent_error
            .to_string()
            .contains("solution-dependent capacitor charge/expression history"),
        "unexpected solution-dependent capacitor diagnostic: {solution_dependent_error}"
    );

    let thermal_resistor = Netlist::parse(
        "* electrothermal accepted temperature is outside the shooting state\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out RMOD L=1u A=1u\n\
         C1 out 0 100p\n\
         .MODEL RMOD R (LEVEL=2 RESISTIVITY=1 HEATCAPACITY=1)\n\
         .end\n",
    )
    .expect("thermal resistor deck parses");
    let thermal_error =
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_pss_with_continuation_state(&thermal_resistor, compact_pss_config())
            .expect_err("thermal accepted temperature must fail before solving");
    assert!(
        thermal_error
            .to_string()
            .contains("thermal resistor accepted temperature state"),
        "unexpected thermal resistor diagnostic: {thermal_error}"
    );
}

#[test]
fn stateless_behavioral_source_has_an_exact_pss_continuation_path() {
    let netlist = Netlist::parse(
        "* time-only behavioral source has no accepted expression memory\n\
         B1 drive 0 V=sin(2*pi*1meg*time)\n\
         R1 drive out 1k\n\
         C1 out 0 100p\n\
         .end\n",
    )
    .expect("stateless behavioral deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (pss, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .expect("stateless behavioral source produces an exact continuation state");

    assert_eq!(pss.result.time.last().copied(), Some(state.period()));
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect("stateless behavioral source resumes from the PSS state");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    assert_eq!(transient.time.last().copied(), Some(100.0e-9));
}

#[test]
fn pss_continuation_checkpoint_has_bit_exact_split_run_parity() {
    let netlist = Netlist::parse(
        "* deterministic PSS-to-TRAN split-run fixture\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .end\n",
    )
    .expect("split-run deck parses");
    let simulation = SimulationConfig {
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(100.0e-9),
        ..SimulationConfig::default()
    };
    let engine = Engine::new(simulation);
    let (_, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .expect("PSS continuation state");

    let (uninterrupted, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 200.0e-9, 100.0e-9)
        .expect("uninterrupted PSS continuation");
    let (_, seam_checkpoint) = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 100.0e-9)
        .expect("first split segment");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &seam_checkpoint, 200.0e-9, 100.0e-9)
        .expect("second split segment resumes");

    let seam = uninterrupted
        .time
        .iter()
        .position(|time| time.to_bits() == seam_checkpoint.time.to_bits())
        .expect("uninterrupted trajectory contains the split seam");
    assert_eq!(resumed.time, uninterrupted.time[seam..]);
    assert_eq!(resumed.step_sizes.first().copied(), Some(0.0));
    assert_eq!(
        resumed.step_sizes[1..],
        uninterrupted.step_sizes[seam + 1..]
    );
    assert_eq!(resumed.node_names, uninterrupted.node_names);
    for (resumed_waveform, uninterrupted_waveform) in
        resumed.voltages.iter().zip(&uninterrupted.voltages)
    {
        assert_eq!(resumed_waveform, &uninterrupted_waveform[seam..]);
    }
}

#[test]
fn frozen_source_contract_rejects_ambiguous_or_unknown_names() {
    let netlist = envelope_startup_deck();
    let engine = Engine::new(SimulationConfig::default());

    let duplicate = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmod".to_string(), "vMOD".to_string()],
        )
        .expect_err("case-insensitive duplicates must fail before solving");
    assert!(duplicate.to_string().contains("duplicate source 'vmod'"));

    let unknown = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmissing".to_string()],
        )
        .expect_err("unknown sources must fail closed");
    assert!(
        unknown
            .to_string()
            .contains("unknown independent source 'vmissing'")
    );
}

#[test]
fn zero_ohm_charge_constraints_survive_transient_continuation() {
    let deck = Netlist::parse("Shorted charge continuation\nI1 0 out SIN(0.5 1 1 0 0 37)\nR1 out 0 0 AC=2\nC1 out 0 1u\nD1 out 0 DM\n.model DM D(IS=0 CJO=1n M=0)\n.end\n").unwrap();
    let engine = Engine::default();
    let (analysis, state) = engine
        .run_pss_with_continuation_state(
            &deck,
            PssConfig::new(1.0)
                .with_points_per_period(128)
                .with_tstab_periods(0),
        )
        .unwrap();
    assert!(analysis.monodromy.is_empty());
    let (continued, _) = engine
        .run_tran_from_pss_state(&deck, &state, 0.1, 0.01)
        .unwrap();
    assert!(continued.voltages[0].iter().all(|voltage| *voltage == 0.0));
    let branch = |name: &str| {
        continued
            .branch_names
            .iter()
            .position(|branch| branch.eq_ignore_ascii_case(name))
            .unwrap()
    };
    for name in ["C1", "D1"] {
        assert!(
            continued.branch_currents[branch(name)]
                .iter()
                .all(|current| *current == 0.0)
        );
    }
    for (&time, &current) in continued
        .time
        .iter()
        .zip(&continued.branch_currents[branch("R1")])
    {
        let expected = 0.5 + (std::f64::consts::TAU * time + 37.0_f64.to_radians()).sin();
        assert!(
            (current - expected).abs() < 2e-12,
            "t={time}: {current} vs {expected}"
        );
    }
}

#[test]
fn native_gp_hb_matches_independent_ngspice_current_and_voltage_orbit() {
    use num_complex::Complex64;
    use rspice_core::analysis::harmonic_balance::HbConfig;
    for (kind, p, krylov) in [("NPN", 1.0, false), ("PNP", -1.0, true)] {
        let netlist = Netlist::parse(&format!("Nonlinear GP HB reference\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1meg)\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-14 BF=100 VAF=50 IKF=1m RB=2k RBM=100 CJE=30p CJC=20p TF=2n XCJC=.4)\n.end\n", p*1.2, p*0.65, p*0.65, p*0.03)).unwrap();
        let mut simulation = SimulationConfig::default();
        simulation.convergence_config.gmin_target = 0.0;
        simulation.convergence_config.junction_gmin_target = 0.0;
        let mut config = HbConfig::new(F0).with_harmonics(15).with_tolerance(1e-9);
        config.abstol = 1e-14;
        config.use_krylov = krylov;
        let hb = Engine::new(simulation).run_hb(&netlist, config).unwrap();
        let voltage = &hb
            .result
            .spectral_voltages
            .iter()
            .find(|s| s.node_name.eq_ignore_ascii_case("Q1.__bi.internal"))
            .unwrap()
            .coefficients;
        let currents: Vec<_> = ["VC", "VB"]
            .map(|name| {
                &hb.result
                    .mna_branch_currents
                    .iter()
                    .find(|s| s.device_name.eq_ignore_ascii_case(name))
                    .unwrap()
                    .coefficients
            })
            .into_iter()
            .collect();
        for (phase, expected) in GP_PERIODIC_REFERENCE.iter().enumerate() {
            for ((spectrum, expected), tolerance) in [voltage, currents[0], currents[1]]
                .into_iter()
                .zip(expected)
                .zip([1e-6, 2e-8, 2e-9])
            {
                let actual = spectrum
                    .iter()
                    .enumerate()
                    .map(|(k, c)| {
                        (*c * Complex64::from_polar(
                            1.0,
                            std::f64::consts::TAU * k as f64 * phase as f64 / 8.0,
                        ))
                        .re
                    })
                    .sum::<f64>()
                    * p;
                assert!(
                    (actual - expected).abs() < tolerance,
                    "{kind} phase={phase}: {actual:e} vs {expected:e}"
                );
            }
        }
    }
}
