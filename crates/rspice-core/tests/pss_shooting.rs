//! Shooting-PSS validation against closed-form periodic steady states.
//!
//! These are the first analytic-truth gates for the shooting solver (the RF
//! roadmap's Tier-0 policy): a sine-driven RC has an exact sinusoidal steady
//! state, so the converged orbit, the periodicity residual, and the Floquet
//! multiplier are all checkable without any reference simulator.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6; // 1 MHz drive
const R: f64 = 1.0e3;
const C: f64 = 159.154943091895e-12; // RC corner ~ 1 MHz (w*RC = 1)

#[test]
fn gummel_poon_pss_private_charge_matches_ac_poles_and_floquet() {
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    let engine = Engine::new(config);
    let tau = 1e3 * 150e-12;
    let wt = std::f64::consts::TAU * F0 * tau;
    let amplitude = 0.01 / (1.0 + wt * wt).sqrt();
    let multiplier = (-1.0 / (F0 * tau)).exp();
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for (base, output) in [("RB=1k RBM=20", "Q1.__bi.internal"), ("RB=1k", "Q1.__bint")] {
            let netlist = Netlist::parse(&format!(
                "GP periodic charge\nV1 in 0 DC {} SIN({} {} 1meg) AC 1\nQ1 0 in 0 qm AREA=2 M=3\n.model qm {kind}(LEVEL=1 IS=0 {base} CJE=100p CJC=200p MJE=0 MJC=0 XCJC=.25)\n.end",
                -p, -p, 0.01*p,
            )).unwrap();
            let ac = engine.run_ac(&netlist, &[F0]).unwrap();
            let out = ac[0]
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(output))
                .unwrap();
            assert!((0.01 * ac[0].voltages[out].norm() / amplitude - 1.0).abs() < 1e-10);
            let input = ac[0]
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case("in"))
                .unwrap()
                + 1;
            let poles = engine
                .run_pz_ports(&netlist, input, None, out + 1, None, false, true, true)
                .unwrap();
            assert_eq!(poles.poles.len(), 1, "{kind} {base}: {:?}", poles.poles);
            assert!((poles.poles[0].re * tau + 1.0).abs() < 1e-9);
            assert!(poles.poles[0].im.abs() * tau < 1e-9);
            let mut previous_error = f64::INFINITY;
            for points in [256, 512] {
                let point = engine
                    .run_pss_operating_point_with_abort(
                        &netlist,
                        PssConfig::new(F0)
                            .with_points_per_period(points)
                            .with_tstab_periods(0)
                            .with_tolerance(1e-10),
                        &NoAbort,
                    )
                    .unwrap_or_else(|error| panic!("{kind} {base} N={points}: {error}"));
                assert_eq!(point.shooting_state_basis(), ["Q:Q1:qbe"]);
                let result = &point.analysis().result;
                let out = result
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(output))
                    .unwrap()
                    + 1;
                let error = (result.harmonics(out, 1)[1].magnitude / amplitude - 1.0).abs();
                assert!(
                    error < previous_error,
                    "{kind} {base} N={points}: {error} vs {previous_error}"
                );
                previous_error = error;
                assert!(error < 0.002);
                for (&time, &actual) in result.time.iter().zip(&result.waveforms[out - 1].values) {
                    let phase = std::f64::consts::TAU * F0 * time;
                    let expected =
                        -p + p * 0.01 * (phase.sin() - wt * phase.cos()) / (1.0 + wt * wt);
                    assert!(
                        (actual - expected).abs() < 0.002 * amplitude,
                        "{kind} {base} N={points} t={time}: {actual} vs {expected}"
                    );
                }
                assert_eq!(point.analysis().floquet_multipliers.len(), 1);
                assert!(
                    (point.analysis().floquet_multipliers[0].re / multiplier - 1.0).abs() < 0.001
                );
            }
        }
    }
}

#[test]
fn classic_jfet_pss_charge_matches_rc_ac_and_floquet_under_refinement() {
    for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
        let netlist = Netlist::parse(&format!(
            "JFET RC periodic charge\nV1 in 0 DC {} SIN({} {} 1meg) AC 1\nR1 in out 1k\nJ1 0 out 0 jm 2 M=3\n.model jm {kind}(BETA=1m VTO=-2 IS=0 CGS=100p CGD=50p M=0)\n.end\n",
            -polarity, -polarity, 0.01 * polarity,
        )).unwrap();
        let engine = Engine::default();
        let tau = 1e3 * 900e-12;
        let omega_tau = std::f64::consts::TAU * F0 * tau;
        let expected_amplitude = 0.01 / (1.0 + omega_tau * omega_tau).sqrt();
        let expected_multiplier = (-(1.0 / F0) / tau).exp();
        let ac = engine.run_ac(&netlist, &[F0]).unwrap();
        let out = ac[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((0.01 * ac[0].voltages[out].norm() / expected_amplitude - 1.0).abs() < 1e-8);
        let mut previous_error = f64::INFINITY;
        for points in [256, 512] {
            let point = engine
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(F0)
                        .with_points_per_period(points)
                        .with_tstab_periods(0)
                        .with_tolerance(1e-10),
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("{kind}, N={points}: {error}"));
            assert_eq!(point.shooting_state_basis(), ["J:J1:qgs"]);
            let result = &point.analysis().result;
            let out = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap()
                + 1;
            let error = (result.harmonics(out, 1)[1].magnitude / expected_amplitude - 1.0).abs();
            assert!(
                error < previous_error,
                "{kind}, N={points}: {error} vs {previous_error}"
            );
            previous_error = error;
            assert!(error < 0.002);
            for (&time, &actual) in result.time.iter().zip(&result.waveforms[out - 1].values) {
                let phase = std::f64::consts::TAU * F0 * time;
                let expected = -polarity
                    + polarity * 0.01 * (phase.sin() - omega_tau * phase.cos())
                        / (1.0 + omega_tau * omega_tau);
                assert!((actual - expected).abs() < 0.002 * expected_amplitude);
            }
            assert_eq!(point.analysis().floquet_multipliers.len(), 1);
            assert!(
                (point.analysis().floquet_multipliers[0].re / expected_multiplier - 1.0).abs()
                    < 0.001
            );
        }
    }
}

#[test]
fn classic_jfet_pss_prescribed_and_tied_charge_have_no_spurious_state() {
    let netlist = Netlist::parse("prescribed JFET charge\nV1 out 0 SIN(-1 0.1 1meg)\nJ1 0 out 0 jm\n.model jm NJF(IS=0 CGS=1n CGD=2n)\n.end\n").unwrap();
    let engine = Engine::default();
    let point = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(64)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    assert!(point.shooting_state().is_empty());
    engine
        .run_pss_with_continuation_state(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(64)
                .with_tstab_periods(0),
        )
        .expect("the complete accepted JFET state supports continuation");
    for (&time, &voltage) in point
        .analysis()
        .result
        .time
        .iter()
        .zip(&point.analysis().result.waveforms[0].values)
    {
        assert!((voltage - (-1.0 + 0.1 * (std::f64::consts::TAU * F0 * time).sin())).abs() < 1e-10);
    }
    let tied = Netlist::parse("tied JFET charge\nI1 0 out 1m\nR1 out 0 1k\nJ1 out out out jm\n.model jm NJF(CGS=1n CGD=2n)\n.end\n").unwrap();
    let error = engine
        .run_pss(&tied, PssConfig::new(F0))
        .unwrap_err()
        .to_string();
    assert!(error.contains("no charge or flux storage"), "{error}");
    let unadapted = Netlist::parse("JFET2 history\nV1 out 0 SIN(-1 0.1 1meg)\nJ1 0 out 0 jm\n.model jm NJF(LEVEL=2 CGS=1n CGD=2n)\n.end\n").unwrap();
    let error = engine
        .run_pss(&unadapted, PssConfig::new(F0))
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("non-classic JFET/MESFET charge and trap history"),
        "{error}"
    );
}

#[test]
fn classic_jfet_pss_nonlinear_orbit_matches_settled_ngspice46() {
    // ngspice-46, 2026-09-09: 1 ns TRAN steps, RELTOL=1e-6, settled
    // 20-21 us cycle. Phase zero uses the exact 21 us endpoint; other rows
    // interpolate at eighth-period offsets. Columns are gate/drain/source.
    let reference = [
        [
            -1.141_700_438_178_542_5,
            4.802_819_700_583_247,
            0.033_888_073_759_555_37,
        ],
        [
            -1.019_612_809_389_259_1,
            4.800_673_414_720_112,
            0.050_178_098_263_160_58,
        ],
        [
            -0.888_275_362_109_973,
            4.730_884_293_621_087,
            0.055_738_947_672_138_356,
        ],
        [
            -0.823_245_571_100_116_6,
            4.633_972_095_801_266_5,
            0.047_211_506_118_656_52,
        ],
        [
            -0.859_561_312_256_275_2,
            4.570_488_258_918_537,
            0.028_907_305_187_074_194,
        ],
        [
            -0.977_246_421_154_020_7,
            4.578_332_291_272_227,
            0.011_607_254_191_949_789,
        ],
        [
            -1.110_463_041_151_341_4,
            4.649_254_493_093_919_5,
            0.006_121_013_982_541_72,
        ],
        [
            -1.179_895_508_212_268,
            4.740_468_369_302_182_5,
            0.015_658_555_302_262_64,
        ],
    ];
    for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
        let netlist = Netlist::parse(&format!(
            "JFET nonlinear PSS oracle\nVDD supply 0 {}\nVIN in 0 DC {} SIN({} {} 1meg)\nRG in gate 1k\nRD supply drain 1k\nRS source 0 100\nJ1 drain gate source jm 1.5 M=2\n.model jm {kind}(BETA=1e-4 VTO=-2 LAMBDA=0.02 IS=1e-30 CGS=100p CGD=50p PB=1 FC=0.5 RD=20 RS=10 TNOM=50)\n.options TEMP=100\n.end\n",
            5.0 * polarity, -polarity, -polarity, 0.4 * polarity,
        )).unwrap();
        let engine = Engine::default();
        let mut previous_error = f64::INFINITY;
        for points in [256, 512] {
            let point = engine
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(F0)
                        .with_points_per_period(points)
                        .with_tstab_periods(0)
                        .with_tolerance(1e-10),
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("{kind}, N={points}: {error}"));
            assert_eq!(point.shooting_state_basis(), ["J:J1:qgs", "J:J1:qgd"]);
            let result = &point.analysis().result;
            let mut error = 0.0_f64;
            for (column, name) in ["gate", "drain", "source"].iter().enumerate() {
                let node = result
                    .node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .unwrap()
                    + 1;
                for (phase, expected) in reference.iter().enumerate() {
                    error = error.max(
                        (result.voltage_at(node, phase as f64 / 8.0 / F0)
                            - polarity * expected[column])
                            .abs(),
                    );
                }
            }
            eprintln!("{kind}, N={points}: max nonlinear JFET orbit error={error:e} V");
            assert!(error < previous_error);
            previous_error = error;
            if points == 512 {
                assert!(error < 2e-5);
            }
        }
    }
}

#[test]
fn tied_admittance_preserves_the_periodic_current_driven_voltage() {
    for device in [
        "R2 out out 1e-20",
        "C2 out out 1e6",
        "D2 out out dm\n.model dm D(IS=1e20 CJO=1e20)",
    ] {
        let netlist = Netlist::parse(&format!(
            "tied periodic admittance\nI1 0 in SIN(1 1 1meg)\nL1 in out 1u\nR1 out 0 1\n{device}\n.end\n"
        )).unwrap();
        let point = Engine::default()
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(64)
                    .with_tstab_periods(0),
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("{device}: {error}"));
        let result = &point.analysis().result;
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
            let expected = 1.0 + (std::f64::consts::TAU * F0 * time).sin();
            assert!(
                (actual - expected).abs() < 1e-10,
                "{device}, at {time}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn periodic_current_waveform_is_independent_of_its_dc_specification() {
    let netlist = Netlist::parse(
        "periodic current bias\nI1 0 in DC 1e100 SIN(1 1 1meg)\nI2 0 in 3\nI3 in in DC -1e100 SIN(1e100 1e100 1meg)\nL1 in out 1u\nR1 out 0 1\n.end\n",
    )
    .unwrap();
    let point = Engine::default()
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(64)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    let result = &point.analysis().result;
    let output = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
        let expected = 4.0 + (std::f64::consts::TAU * F0 * time).sin();
        assert!(
            (actual - expected).abs() < 1e-10,
            "at {time}: {actual} vs {expected}"
        );
    }
}

#[test]
fn nonlinear_vbic_charge_pss_matches_settled_reference_simulators() {
    use rspice_core::engine::SpiceDialect;
    // Live ngspice 46, 2026-09-08, the NPN LEVEL=4 deck below with
    // RELTOL=1e-7 VNTOL=1e-9 ABSTOL=1e-15 and .tran 0.2n 21u 19u 0.2n.
    // Collector voltage interpolated every 1/16 period in the settled 20–21 us
    // cycle. Xyce 7.10 LEVEL=12 agrees within 0.6 uV on the same mesh;
    // LEVEL=11 is a different three-terminal substrate topology.
    // Includes all seven intrinsic electrical nodes, nonlinear forward
    // and reverse diffusion, split depletion charge, epi and substrate charge.
    let four_terminal_reference = [
        1.3981978158,
        1.2433328436,
        1.0637934817,
        0.8830475405,
        0.7315884376,
        0.6402557219,
        0.6336042015,
        0.7244707505,
        0.9056791472,
        1.1402121144,
        1.3656502858,
        1.5258380514,
        1.6057943323,
        1.6220745688,
        1.5901940628,
        1.5152586291,
        1.3981978556,
    ];
    // Xyce 7.10 LEVEL=11, same settled mesh, 2026-09-08. The three-terminal
    // result differs by 5 mV from the four-terminal waveform, so a shared
    // topology cannot satisfy the 0.2 mV integration-error bound.
    let three_terminal_reference = [
        1.3954399520,
        1.2397637940,
        1.0597084600,
        0.8789465493,
        0.7281096900,
        0.6380552643,
        0.6332519076,
        0.7263252858,
        0.9096050192,
        1.1453053530,
        1.3704296340,
        1.5290815530,
        1.6072918720,
        1.6222274090,
        1.5893073480,
        1.5134234290,
        1.3954400000,
    ];
    for (dialect, level, reference) in [
        (SpiceDialect::Ngspice, 4, four_terminal_reference),
        (SpiceDialect::Xyce, 12, four_terminal_reference),
        (SpiceDialect::Xyce, 11, three_terminal_reference),
    ] {
        for (polarity, sign) in [("NPN", 1.0), ("PNP", -1.0)] {
            // Prescribe zero thermal rise for the electrical shooting oracle.
            let thermal_pin = if level == 12 { " 0" } else { "" };
            let netlist = Netlist::parse(&format!("VBIC PSS electrical oracle\nVcc supply 0 {}\nVb drive 0 SIN({} {} 1meg)\nRc supply c 1k\nRb drive b 100\nQ1 c b 0 0{thermal_pin} vm\n.model vm {polarity}(LEVEL={level} IS=1e-14 IBEI=1e-16 IBCI=1e-16 RCX=10 RCI=20 RBX=10 RBI=40 RE=1 RBP=10 RS=1 CJE=10p CJC=5p CJEP=3p CJCP=2p TF=10n TR=2n QCO=10f GAMM=1e-9 ISP=1e-16 WBE=0.8)\n.options RELTOL=1e-6 VNTOL=1e-8 ABSTOL=1e-14\n.temp 27\n.end\n", 2.0*sign, 0.65*sign, 0.02*sign)).unwrap();
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let point = engine
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(F0)
                        .with_points_per_period(256)
                        .with_tstab_periods(4)
                        .with_tolerance(1e-7),
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("{dialect:?} {polarity}: {error}"));
            assert!(
                point.shooting_state().len() >= 4,
                "internal charge states must participate in shooting"
            );
            let result = &point.analysis().result;
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("c"))
                .unwrap()
                + 1;
            let error = reference
                .iter()
                .enumerate()
                .map(|(index, &voltage)| {
                    (sign * result.voltage_at(output, index as f64 / 16.0 / F0) - voltage).abs()
                })
                .fold(0.0_f64, f64::max);
            assert!(
                error < 2e-4,
                "{dialect:?} LEVEL={level} {polarity}: maximum reference waveform error {error:e} V"
            );
        }
    }
}

#[test]
fn prescribed_vbic_voltages_have_no_free_charge_modes() {
    for (polarity, sign) in [("NPN", 1.0), ("PNP", -1.0)] {
        let netlist = Netlist::parse(&format!("prescribed VBIC charges\nVc c 0 {sign}\nVb b 0 SIN({} {} 1meg)\nQ1 c b 0 vm\n.model vm {polarity}(LEVEL=4 IS=1e-14 CJE=100p CJC=20p TF=1n RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0)\n.end\n", sign * 0.65, sign * 0.02)).unwrap();
        let point = Engine::default()
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(64)
                    .with_tstab_periods(0),
                &NoAbort,
            )
            .unwrap();
        assert!(point.shooting_state().is_empty());
        assert!(point.analysis().monodromy.is_empty());
        let result = &point.analysis().result;
        let base = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("b"))
            .unwrap();
        for (&time, &voltage) in result.time.iter().zip(&result.waveforms[base].values) {
            assert!(
                (voltage - sign * (0.65 + 0.02 * (std::f64::consts::TAU * F0 * time).sin())).abs()
                    < 1e-10
            );
        }
    }
}

#[test]
fn discontinuous_drive_preserves_the_complete_rlc_orbit() {
    // Independent two-state solution of C*v'=(u-v)/R1-i and L*i'=v-R2*i.
    // Each half-cycle has constant forcing, so its exact state transition is
    // exp(A*t). Solve periodic closure at the rising edge before phase shift.
    let capacitance = 159e-12;
    let inductance = 10e-6;
    let load = 2e3;
    let a: f64 = -1.0 / (R * capacitance);
    let b: f64 = -1.0 / capacitance;
    let c: f64 = 1.0 / inductance;
    let d: f64 = -load / inductance;
    let middle = (a + d) / 2.0;
    let radius = (((a - d) / 2.0).powi(2) + b * c).sqrt();
    let slow = middle + radius;
    let fast = middle - radius;
    let transition = |time: f64| {
        let es = (slow * time).exp();
        let ef = (fast * time).exp();
        let divided = (es - ef) / (slow - fast);
        [
            [ef + (a - fast) * divided, b * divided],
            [c * divided, ef + (d - fast) * divided],
        ]
    };
    let multiply = |matrix: [[f64; 2]; 2], state: [f64; 2]| {
        matrix.map(|row| row[0] * state[0] + row[1] * state[1])
    };
    let half_period = 0.5 / F0;
    let e = transition(half_period);
    let high = [load / (R + load), 1.0 / (R + load)];
    let rhs = multiply(e, high);
    let determinant = (1.0 + e[0][0]) * (1.0 + e[1][1]) - e[0][1] * e[1][0];
    let rising = [
        ((1.0 + e[1][1]) * rhs[0] - e[0][1] * rhs[1]) / determinant,
        ((1.0 + e[0][0]) * rhs[1] - e[1][0] * rhs[0]) / determinant,
    ];
    let exact = |time: f64| {
        let phase_time = (time + 0.1 / (std::f64::consts::TAU * F0)).rem_euclid(1.0 / F0);
        if phase_time < half_period {
            let transient = multiply(
                transition(phase_time),
                [rising[0] - high[0], rising[1] - high[1]],
            );
            [high[0] + transient[0], high[1] + transient[1]]
        } else {
            multiply(
                transition(phase_time - half_period),
                [high[0] - rising[0], high[1] - rising[1]],
            )
        }
    };
    // Impedance scaling leaves the voltage ODE unchanged while moving the
    // winding-current coordinate across six orders of magnitude.
    for impedance_scale in [0.001, 1.0, 1000.0] {
        let netlist = Netlist::parse(&format!("discontinuous RLC orbit\nB1 in 0 V=if(sin(2*pi*1meg*time+0.1)>0,1,0)\nR1 in out {:.17e}\nC1 out 0 {:.17e}\nL1 out load {:.17e}\nR2 load 0 {:.17e}\n.options RELTOL=1e-6 VNTOL=1e-8\n.end\n", R * impedance_scale, capacitance / impedance_scale, inductance * impedance_scale, load * impedance_scale)).unwrap();
        let analysis = Engine::default()
            .run_pss(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(256)
                    .with_tstab_periods(0),
            )
            .expect("source edges must preserve the winding correction below one current ULP");
        let result = &analysis.result;
        for (name, coordinate, scale, tolerance) in [("out", 0, 1.0, 1e-5), ("load", 1, load, 1e-5)]
        {
            let node = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            let waveform = &result.waveforms[node];
            let mut max_error: f64 = 0.0;
            for (&time, &actual) in result.time.iter().zip(&waveform.values) {
                let expected = exact(time)[coordinate] * scale;
                max_error = max_error.max((actual - expected).abs());
            }
            assert!(
                max_error < tolerance,
                "{name}, impedance scale={impedance_scale}: {max_error:e}, {} samples",
                result.time.len()
            );
            assert!((waveform.dc(&result.time, result.period) - 1.0 / 3.0).abs() < tolerance);
        }
    }
}

#[test]
fn source_intervals_reveal_pulses_between_both_initial_pss_grids() {
    let knots = [
        (0.0, 0.0),
        (4e-10, 0.0),
        (5e-10, 1.0),
        (1.5e-9, 1.0),
        (1.6e-9, 0.0),
        (1e-6, 0.0),
    ];
    let advance = |initial: f64, voltage: f64, slope: f64, dt: f64| {
        let exponent = -dt / (R * C);
        initial * exponent.exp() - voltage * exponent.exp_m1()
            + slope * (dt + R * C * exponent.exp_m1())
    };
    let traverse = |initial: f64, stop: f64| {
        let mut state = initial;
        for pair in knots.windows(2) {
            let duration: f64 = pair[1].0 - pair[0].0;
            let elapsed = (stop - pair[0].0).clamp(0.0, duration);
            state = advance(
                state,
                pair[0].1,
                (pair[1].1 - pair[0].1) / duration,
                elapsed,
            );
            if stop <= pair[1].0 {
                break;
            }
        }
        state
    };
    let initial = traverse(0.0, 1e-6) / -(-1e-6 / (R * C)).exp_m1();
    for (source, current_drive) in [
        ("V1 in 0 PULSE(0 1 400p 100p 100p 1n 1u)", false),
        (
            "V1 in 0 PWL(0 0 400p 0 500p 1 1.5n 1 1.6n 0 1u 0) R=0",
            false,
        ),
        (
            "B1 in 0 V=table(time%1u,0,0,400p,0,500p,1,1.5n,1,1.6n,0,1u,0)",
            false,
        ),
        ("B1 in 0 V=spice_pulse(0,1,400p,100p,100p,1n,1u)", false),
        ("I1 0 out PULSE(0 1m 400p 100p 100p 1n 1u)", true),
        (
            "B1 0 out I=1m*table(mod(time,1u),0,0,400p,0,500p,1,1.5n,1,1.6n,0,1u,0)",
            true,
        ),
    ] {
        let resistor = if current_drive {
            "R1 out 0 1k"
        } else {
            "R1 in out 1k"
        };
        let netlist = Netlist::parse(&format!(
            "narrow periodic pulse\n{source}\n{resistor}\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        let analysis = Engine::default()
            .run_pss(&netlist, PssConfig::new(F0).with_tstab_periods(0))
            .unwrap_or_else(|error| panic!("{source}: {error}"));
        let result = &analysis.result;
        let steps = result.time.len() - 1;
        assert!(
            steps < 2_048,
            "source corners should resolve the pulse locally: {source}: {steps}"
        );
        for &(corner, _) in &knots {
            assert!(
                result
                    .time
                    .iter()
                    .any(|&time| (time - corner).abs() <= 4.0 * f64::EPSILON * corner),
                "missing source corner {corner:e}"
            );
        }
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let values = &result.waveforms[output].values;
        let mean = result.waveforms[output].dc(&result.time, result.period);
        assert!(
            (mean - 0.0011).abs() < 2e-6,
            "{source}, steps={steps}: DC {mean}"
        );
        let peak = traverse(initial, 1.6e-9);
        for (&time, &actual) in result.time.iter().zip(values) {
            let expected = traverse(initial, time);
            assert!(
                (actual - expected).abs() < 1e-6 + 1e-3 * peak,
                "{source}, steps={steps}, t={time:e}: {actual:e} vs {expected:e}"
            );
        }
    }
}

#[test]
fn rounded_periods_preserve_and_qualify_adjacent_source_clocks() {
    use rspice_core::numerics::integration::IntegrationMethod;

    for (period_text, frequency, authored_period) in [
        ("100u", 1e4, 100.0 * 1e-6_f64),
        ("10u", 1e5, 10.0 * 1e-6_f64),
    ] {
        let period = 1.0 / frequency;
        assert_eq!(authored_period.next_up(), period);
        let width = 0.4 * period;
        for (source, integration_method) in [
            (
                format!("V1 in 0 PULSE(0 1 0 1n 1n {width:e} {period_text})"),
                None,
            ),
            (
                format!("B1 in 0 V=spice_pulse(0,1,0,1n,1n,{width:e},{period_text})"),
                Some(IntegrationMethod::BackwardEuler),
            ),
            (
                format!(
                    "B1 in 0 V=table(mod(time,{period_text}),0,0,1n,1,{:e},1,{:e},0,{period_text},0)",
                    width + 1e-9,
                    width + 2e-9,
                ),
                Some(IntegrationMethod::Gear2),
            ),
        ] {
            let capacitance = 1.0 / (std::f64::consts::TAU * R * frequency);
            let netlist = Netlist::parse(&format!(
                "rounded clock\n{source}\nR1 in out {R}\nC1 out 0 {capacitance:e}\n.end\n"
            ))
            .unwrap();
            let engine = Engine::default();
            let mut config = PssConfig::new(frequency).with_tstab_periods(0);
            config.integration_method = integration_method;
            let point = engine
                .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
                .unwrap_or_else(|error| panic!("{source}: {error}"));
            let result = &point.analysis().result;
            assert_eq!(*result.time.last().unwrap(), period);
            assert_eq!(result.time[result.time.len() - 2], authored_period);
            assert!(result.time.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(result.time.len() < 4_096, "{source}: {}", result.time.len());
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let mean = result.waveforms[output].dc(&result.time, period);
            let expected = (width + 1e-9) / authored_period;
            assert!(
                (mean - expected).abs() < 1e-4,
                "{source}: {mean} vs {expected}"
            );
            let restored = rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
                point.producer_identity().unwrap().clone(),
                point.config().clone(),
                point.analysis().clone(),
                point.shooting_state_basis().to_vec(),
                point.shooting_state().to_vec(),
            )
            .unwrap();
            assert_eq!(restored, point);
        }
    }
}

#[test]
fn unresolvable_source_intervals_cannot_pass_by_sharing_the_same_mesh() {
    let interval = 0.5_f64.next_up() - 0.5;
    let netlist = Netlist::parse(&format!(
        "physical response at the time precision floor\nV1 in 0 PULSE(0 1 0.5 {interval:e} {interval:e} {interval:e} 1)\nR1 in out 1\nC1 out 0 {interval:e}\n.end\n"
    ))
    .unwrap();
    let error = Engine::default()
        .run_pss(&netlist, PssConfig::new(1.0).with_tstab_periods(0))
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("alternate-method waveform error"),
        "{error}"
    );
}

#[test]
fn nonlinear_time_features_cannot_hide_between_shooting_grids() {
    check_nonlinear_time_features(0..=7);
}

#[test]
fn hyperbolic_time_features_preserve_the_complete_rc_waveform() {
    check_nonlinear_time_features(8..=14);
}

#[test]
fn circular_time_features_preserve_the_complete_rc_waveform() {
    check_nonlinear_time_features(15..=18);
}

#[test]
fn bounded_tangent_compositions_preserve_the_complete_rc_waveform() {
    check_bounded_compositions(0..=1);
}

#[test]
fn bounded_quotient_compositions_preserve_the_complete_rc_waveform() {
    check_bounded_compositions(2..=3);
}

#[test]
fn bounded_power_compositions_preserve_the_complete_rc_waveform() {
    check_bounded_compositions(4..=4);
}

#[test]
fn bounded_reciprocal_exponentials_preserve_the_complete_rc_waveform() {
    check_bounded_compositions(5..=6);
}

fn check_bounded_compositions(cases: std::ops::RangeInclusive<usize>) {
    let rate = std::f64::consts::TAU * 64.0 * F0;
    let period = 1.0 / (128.0 * F0);
    let tau = R * C;
    let bias = -std::f64::consts::FRAC_PI_2;
    let initial = bias - rate * tau + rate * period / -(-period / tau).exp_m1();
    let count = 131_072;
    // Integrate a smooth ramp between the one-sided limits at the tanh jump.
    let (tanh_reference, _) = periodic_rc_convolution(period, tau, count, |index| {
        if index == 0 {
            -1.0
        } else if index == count {
            1.0
        } else {
            (bias + std::f64::consts::PI * index as f64 / count as f64)
                .tan()
                .tanh()
        }
    });
    let (reciprocal_reference, _) = periodic_rc_convolution(period, tau, count, |index| {
        if index == 0 || index == count {
            std::f64::consts::FRAC_PI_2
        } else {
            (1.0 / (bias + std::f64::consts::PI * index as f64 / count as f64).cos()).atan()
        }
    });
    let (power_reference, power_mean) = periodic_rc_convolution(period, tau, count, |index| {
        if index == 0 || index == count {
            1.0
        } else {
            (bias + std::f64::consts::PI * index as f64 / count as f64)
                .tan()
                .powi(2)
                .tanh()
        }
    });
    let (exponential_reference, exponential_mean) =
        periodic_rc_convolution(period, tau, count, |index| {
            if index == 0 || index == count {
                0.0
            } else {
                (-1.0
                    / (bias + std::f64::consts::PI * index as f64 / count as f64)
                        .cos()
                        .powi(2))
                .exp()
            }
        });
    for (expression, kind) in [
        ("atan(tan(2*pi*64meg*time+0.1))", 0),
        ("tanh(tan(2*pi*64meg*time+0.1))", 1),
        ("atan(sin(2*pi*64meg*time+0.1)/cos(2*pi*64meg*time+0.1))", 0),
        ("atan(1/cos(2*pi*64meg*time+0.1))", 2),
        ("tanh(tan(2*pi*64meg*time+0.1)^2)", 3),
        ("exp(-1/cos(2*pi*64meg*time+0.1)^2)", 4),
        ("exp(-sqr(1/cos(2*pi*64meg*time+0.1)))", 4),
    ]
    .into_iter()
    .enumerate()
    .filter_map(|(index, case)| cases.contains(&index).then_some(case))
    {
        // The squared source now fits the ordinary result budget at tighter
        // accuracy; derivative probes must not retain discarded waveforms.
        let tolerance = if kind == 4 { 1e-5 } else { 1e-6 };
        let reltol = if kind == 3 { 1e-5 } else { 1e-4 };
        let netlist = Netlist::parse(&format!(
            "bounded tangent forcing\n.options reltol={reltol} vntol=1e-8\nB1 in 0 V={expression}\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        )).unwrap();
        let analysis = Engine::default()
            .run_pss_with_abort(&netlist, PssConfig::new(F0).with_tstab_periods(0), &NoAbort)
            .unwrap_or_else(|error| panic!("{expression}: {error}"));
        let result = &analysis.result;
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
            let elapsed = (rate * time + 0.1 - bias).rem_euclid(std::f64::consts::PI) / rate;
            let expected = if kind == 0 {
                let decay = (-elapsed / tau).exp_m1();
                initial * (1.0 + decay) - bias * decay + rate * (elapsed + tau * decay)
            } else {
                let reference = match kind {
                    1 => &tanh_reference,
                    2 => &reciprocal_reference,
                    3 => &power_reference,
                    _ => &exponential_reference,
                };
                let position = elapsed / period * count as f64;
                let left = (position as usize).min(count - 1);
                let interpolated = reference[left]
                    + (position - left as f64) * (reference[left + 1] - reference[left]);
                if kind == 2 {
                    // The reciprocal forcing changes sign each half-period.
                    // Convert the positive forcing's periodic solution into
                    // its anti-periodic solution using the exact RC decay.
                    let anti_initial =
                        -reference[0] * -(-period / tau).exp_m1() / (1.0 + (-period / tau).exp());
                    let positive =
                        interpolated + (anti_initial - reference[0]) * (-elapsed / tau).exp();
                    if (rate * time + 0.1 - bias).rem_euclid(std::f64::consts::TAU)
                        < std::f64::consts::PI
                    {
                        positive
                    } else {
                        -positive
                    }
                } else {
                    interpolated
                }
            };
            assert!(
                (actual - expected).abs() < tolerance,
                "{expression}, t={time:e}: {actual:e} versus {expected:e}"
            );
        }
        assert!(
            (result.waveforms[output].dc(&result.time, result.period)
                - match kind {
                    3 => power_mean,
                    4 => exponential_mean,
                    _ => 0.0,
                })
            .abs()
                < tolerance,
            "{expression}"
        );
    }
}

#[test]
fn polar_signed_zero_edges_preserve_the_analytic_rc_orbit() {
    let netlist = Netlist::parse(&format!(
        "signed-zero polar forcing\n.options reltol=1e-4\nB1 in 0 V=atan2(0*sin(2*pi*64meg*time+0.1),-1)\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
    )).unwrap();
    let config = PssConfig::new(F0).with_tstab_periods(0);
    let point = Engine::default()
        .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
        .unwrap();
    let result = &point.analysis().result;
    assert!(
        result.time.len() < 8192,
        "signed-zero edges must resolve locally: {}",
        result.time.len()
    );
    let output = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let tau = R * C;
    let period = 1.0 / (64.0 * F0);
    let amplitude = std::f64::consts::PI;
    let high = amplitude * (period / (4.0 * tau)).tanh();
    for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
        let phase = (std::f64::consts::TAU * time / period + 0.1).rem_euclid(std::f64::consts::TAU);
        let elapsed = phase / std::f64::consts::TAU * period;
        let expected = if elapsed < period / 2.0 {
            amplitude - (amplitude + high) * (-elapsed / tau).exp()
        } else {
            -amplitude + (amplitude + high) * (-(elapsed - period / 2.0) / tau).exp()
        };
        assert!(
            (actual - expected).abs() < 1e-5,
            "t={time:e}: {actual:e} versus {expected:e}"
        );
    }
    assert!(
        result.waveforms[output]
            .dc(&result.time, result.period)
            .abs()
            < 1e-6
    );
}

/// Exact linear-forcing RC integration, independent of the shooting stamps
/// and mesh. The caller supplies the two one-sided endpoint values at a jump.
fn periodic_rc_convolution(
    period: f64,
    tau: f64,
    count: usize,
    forcing: impl Fn(usize) -> f64,
) -> (Vec<f64>, f64) {
    let dt = period / count as f64;
    let decay = (-dt / tau).exp();
    let decay_minus_one = (-dt / tau).exp_m1();
    let linear_area = dt + tau * decay_minus_one;
    let mut reference = vec![0.0];
    let mut previous = forcing(0);
    let mut mean = 0.0;
    for index in 1..=count {
        let next = forcing(index);
        let state = reference[index - 1] * decay - previous * decay_minus_one
            + (next - previous) / dt * linear_area;
        reference.push(state);
        mean += 0.5 * (previous + next) / count as f64;
        previous = next;
    }
    let initial = reference[count] / -(-period / tau).exp_m1();
    for (index, value) in reference.iter_mut().enumerate() {
        *value += initial * (-(index as f64 * dt) / tau).exp();
    }
    (reference, mean)
}

fn check_nonlinear_time_features(kinds: std::ops::RangeInclusive<usize>) {
    let omega = std::f64::consts::TAU * F0 * 64.0;
    let source_period = 1.0 / (F0 * 64.0);
    let tau = R * C;
    let threshold: f64 = 0.9999;
    let angle = threshold.acos();
    let on = 2.0 * angle / omega;
    let off = source_period - on;
    let low = (-off / tau).exp() * -(-on / tau).exp_m1() / -(-source_period / tau).exp_m1();
    let high = low * (-on / tau).exp() - (-on / tau).exp_m1();
    let pulse = |time: f64| {
        let elapsed = (omega * time + 0.1 + angle).rem_euclid(std::f64::consts::TAU) / omega;
        if elapsed < on {
            low * (-elapsed / tau).exp() - (-elapsed / tau).exp_m1()
        } else {
            high * (-(elapsed - on) / tau).exp()
        }
    };
    for (expression, kind) in [
        ("if(cos(2*pi*64meg*time+0.1)>0.9999,1,0)", 0),
        ("exp(-10000*(1-cos(2*pi*64meg*time+0.1)))", 1),
        ("exp(-10000*(cos(2*pi*64meg*time+0.1)-0.25)^2)", 2),
        (
            "exp(-1000000*(cos(2*pi*64meg*time+0.1)+0.5*cos(2*pi*128meg*time+0.2)-0.25)^2)",
            3,
        ),
        (
            "exp(-1000000*((cos(2*pi*64meg*time+0.1)+0.5*cos(2*(2*pi*64meg*time+0.1)))/(sqr(sin(2*pi*64meg*time+0.1))+sqr(cos(2*pi*64meg*time+0.1)))-0.25)^2)",
            3,
        ),
        (
            "exp(-1000000*((1e-310*(cos(2*pi*64meg*time+0.1)+0.5*cos(2*(2*pi*64meg*time+0.1))))/(1e-310*(sin(2*pi*64meg*time+0.1)^2+cos(2*pi*64meg*time+0.1)^2))-0.25)^2)",
            3,
        ),
        (
            "exp(-1000000*(exp(cos(2*pi*64meg*time+0.1))+0.5*exp(cos(2*(2*pi*64meg*time+0.1)))-1.5)^2)",
            4,
        ),
        (
            "exp(-1000000*(ln(2+cos(2*pi*64meg*time+0.1))+0.5*ln(2+cos(2*(2*pi*64meg*time+0.1)))-1.5)^2)",
            5,
        ),
        (
            "exp(-1000000*(2.302585092994046*(log10(2+cos(2*pi*64meg*time+0.1))+0.5*log10(2+cos(2*(2*pi*64meg*time+0.1))))-1.5)^2)",
            5,
        ),
        (
            "exp(-1000000*(sqrt(2+cos(2*pi*64meg*time+0.1))+0.5*sqrt(2+cos(2*(2*pi*64meg*time+0.1)))-2.4)^2)",
            6,
        ),
        (
            "exp(-1000000*(abs(cos(2*pi*64meg*time+0.1))+0.5*abs(cos(2*(2*pi*64meg*time+0.1)))-0.75)^2)",
            7,
        ),
        (
            "exp(-1000000*(max(cos(2*pi*64meg*time+0.1),-cos(2*pi*64meg*time+0.1))+0.5*max(cos(2*(2*pi*64meg*time+0.1)),-cos(2*(2*pi*64meg*time+0.1)))-0.75)^2)",
            7,
        ),
        (
            "exp(-1000000*(min(cos(2*pi*64meg*time+0.1),-cos(2*pi*64meg*time+0.1),1)+0.5*min(cos(2*(2*pi*64meg*time+0.1)),-cos(2*(2*pi*64meg*time+0.1)),1)+0.75)^2)",
            7,
        ),
        (
            "exp(-1000000*(atan(cos(2*pi*64meg*time+0.1))+0.5*atan(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            8,
        ),
        (
            "exp(-1000000*(sinh(cos(2*pi*64meg*time+0.1))+0.5*sinh(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            9,
        ),
        (
            "exp(-1000000*(cosh(cos(2*pi*64meg*time+0.1))+0.5*cosh(cos(2*(2*pi*64meg*time+0.1)))-1.7)^2)",
            10,
        ),
        (
            "exp(-1000000*(tanh(cos(2*pi*64meg*time+0.1))+0.5*tanh(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            11,
        ),
        (
            "exp(-1000000*(asinh(cos(2*pi*64meg*time+0.1))+0.5*asinh(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            12,
        ),
        (
            "exp(-1000000*(acosh(2+cos(2*pi*64meg*time+0.1))+0.5*acosh(2+cos(2*(2*pi*64meg*time+0.1)))-1.5)^2)",
            13,
        ),
        (
            "exp(-1000000*(atanh(0.5*cos(2*pi*64meg*time+0.1))+0.5*atanh(0.5*cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            14,
        ),
        (
            "exp(-1000000*(asin(0.5*cos(2*pi*64meg*time+0.1))+0.5*asin(0.5*cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            15,
        ),
        (
            "exp(-1000000*(acos(0.5*cos(2*pi*64meg*time+0.1))+0.5*acos(0.5*cos(2*(2*pi*64meg*time+0.1)))-2.1)^2)",
            16,
        ),
        (
            "exp(-1000000*(tan(0.5*cos(2*pi*64meg*time+0.1))+0.5*tan(0.5*cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
            17,
        ),
        (
            "exp(-1000000*(atan2(cos(2*pi*64meg*time+0.1)+0.5*cos(2*(2*pi*64meg*time+0.1))-0.25,2))^2)",
            18,
        ),
    ] {
        if !kinds.contains(&kind) {
            continue;
        }
        // Independent linear RC convolution on one source cycle. This uses
        // exact integration of densely sampled linear forcing segments, not
        // the shooting companion, period map or feature mesh under test.
        let count = 131_072;
        let dt = source_period / count as f64;
        let forcing = |time: f64| {
            let cosine = (omega * time + 0.1).cos();
            if kind == 1 {
                (-10000.0 * (1.0 - cosine)).exp()
            } else if kind == 3 {
                (-1000000.0 * (cosine + 0.5 * (2.0 * (omega * time + 0.1)).cos() - 0.25).powi(2))
                    .exp()
            } else if kind == 4 {
                (-1000000.0
                    * (cosine.exp() + 0.5 * (2.0 * (omega * time + 0.1)).cos().exp() - 1.5).powi(2))
                .exp()
            } else if kind == 5 {
                (-1000000.0
                    * ((2.0 + cosine).ln() + 0.5 * (2.0 + (2.0 * (omega * time + 0.1)).cos()).ln()
                        - 1.5)
                        .powi(2))
                .exp()
            } else if kind == 6 {
                (-1000000.0
                    * ((2.0 + cosine).sqrt()
                        + 0.5 * (2.0 + (2.0 * (omega * time + 0.1)).cos()).sqrt()
                        - 2.4)
                        .powi(2))
                .exp()
            } else if kind == 7 {
                (-1000000.0
                    * (cosine.abs() + 0.5 * (2.0 * (omega * time + 0.1)).cos().abs() - 0.75)
                        .powi(2))
                .exp()
            } else if kind == 18 {
                (-1000000.0
                    * (cosine + 0.5 * (2.0 * (omega * time + 0.1)).cos() - 0.25)
                        .atan2(2.0)
                        .powi(2))
                .exp()
            } else if kind >= 8 {
                let (function, bias): (fn(f64) -> f64, f64) = match kind {
                    8 => (f64::atan, 0.25),
                    9 => (f64::sinh, 0.25),
                    10 => (f64::cosh, 1.7),
                    11 => (f64::tanh, 0.25),
                    12 => (f64::asinh, 0.25),
                    13 => (|x| (2.0 + x).acosh(), 1.5),
                    14 => (|x| (0.5 * x).atanh(), 0.25),
                    15 => (|x| (0.5 * x).asin(), 0.25),
                    16 => (|x| (0.5 * x).acos(), 2.1),
                    17 => (|x| (0.5 * x).tan(), 0.25),
                    _ => unreachable!(),
                };
                (-1000000.0
                    * (function(cosine) + 0.5 * function((2.0 * (omega * time + 0.1)).cos())
                        - bias)
                        .powi(2))
                .exp()
            } else {
                (-10000.0 * (cosine - 0.25).powi(2)).exp()
            }
        };
        let (reference, expected_dc) = if kind == 0 {
            (Vec::new(), angle / std::f64::consts::PI)
        } else {
            periodic_rc_convolution(source_period, tau, count, |index| {
                forcing(index as f64 * dt)
            })
        };
        let expected = |time: f64| {
            if kind == 0 {
                pulse(time)
            } else {
                let position = (time / source_period).rem_euclid(1.0) * count as f64;
                let left = (position as usize).min(count - 1);
                let fraction = position - left as f64;
                reference[left] + fraction * (reference[left + 1] - reference[left])
            }
        };
        let netlist = Netlist::parse(&format!(
            "nonlinear periodic feature\nB1 in 0 V={expression}\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        let point = Engine::default()
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0).with_tstab_periods(0),
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("{expression}: {error}"));
        let result = &point.analysis().result;
        assert!(
            result.time.len() <= if kind == 3 { 2_000_001 } else { 262_145 },
            "{expression}: {}",
            result.time.len()
        );
        if kind == 0 {
            assert!(
                result.time.len() < 2_048,
                "switching edges must resolve locally"
            );
        }
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let mean = result.waveforms[output].dc(&result.time, result.period);
        // The absolute-value case has a 43 mV DC level, for which the default
        // voltage criterion is about 44 uV. Require 10 uV against the oracle;
        // the smaller compound responses retain their 1 uV requirement.
        let tolerance = if (3..=6).contains(&kind) || (kind >= 8 && kind != 10) {
            1e-6
        } else {
            1e-5
        };
        assert!(
            (mean - expected_dc).abs() < tolerance,
            "{expression}: N={}, DC {mean:e} versus {expected_dc:e}",
            result.time.len()
        );
        for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
            assert!(
                (actual - expected(time)).abs() < tolerance,
                "{expression}, t={time:e}: {actual:e} versus {:e}",
                expected(time)
            );
        }
        if kind == 0 {
            let engine = Engine::default();
            let (_, state) = engine
                .run_pss_with_continuation_state(&netlist, PssConfig::new(F0).with_tstab_periods(0))
                .unwrap();
            let (continued, _) = engine
                .run_tran_from_pss_state(&netlist, &state, 1.0 / F0, 1.0 / (F0 * 256.0))
                .unwrap();
            let output = continued
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            for (&time, &actual) in continued.time.iter().zip(&continued.voltages[output]) {
                assert!(
                    (actual - pulse(time)).abs() < 1e-5,
                    "continued t={time:e}: {actual:e} versus {:e}",
                    pulse(time)
                );
            }
        }
    }
}

#[test]
fn combined_comparison_coordinates_preserve_the_complete_rc_waveform() {
    let omega = std::f64::consts::TAU * F0 * 64.0;
    let period = 1.0 / (F0 * 64.0);
    let tau = R * C;
    let threshold: f64 = 0.001;
    // cos(phi)+0.5*cos(2*phi)-0.25 = cos(phi)^2+cos(phi)-0.75.
    // Solve the four exact switching phases and integrate each constant
    // forcing segment analytically, independently of the event collector.
    let enter = ((1.0 + threshold).sqrt() - 0.5).acos();
    let leave = ((1.0 - threshold).sqrt() - 0.5).acos();
    let edges = [
        enter,
        leave,
        std::f64::consts::TAU - leave,
        std::f64::consts::TAU - enter,
    ]
    .map(|phase| (phase - 0.1) / omega);
    let response = |time: f64, mut state: f64| {
        let mut left = 0.0;
        let mut forcing = 0.0;
        for right in edges.into_iter().chain([time]) {
            let stop = right.min(time);
            let exponent = -(stop - left) / tau;
            state = state * exponent.exp() - forcing * exponent.exp_m1();
            if right >= time {
                break;
            }
            left = right;
            forcing = 1.0 - forcing;
        }
        state
    };
    let initial = response(period, 0.0) / -(-period / tau).exp_m1();
    let expected_dc = (leave - enter) / std::f64::consts::PI;
    for expression in [
        "abs(cos(2*pi*64meg*time+0.1)+0.5*cos(2*pi*128meg*time+0.2)-0.25)<0.001",
        "0.5*(1-pwrs(abs(cos(2*pi*64meg*time+0.1)+0.5*cos(2*pi*128meg*time+0.2)-0.25)-0.001,0))",
    ] {
        let netlist = Netlist::parse(&format!(
            "combined comparison clock\nB1 in 0 V={expression}\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        let analysis = Engine::default()
            .run_pss(&netlist, PssConfig::new(F0).with_tstab_periods(0))
            .unwrap();
        let result = &analysis.result;
        assert!(
            result.time.len() < 4096,
            "switching features must resolve locally"
        );
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let mean = result.waveforms[output].dc(&result.time, result.period);
        assert!(
            (mean - expected_dc).abs() < 1e-6,
            "DC {mean:e} versus {expected_dc:e}"
        );
        for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
            let expected = response(time.rem_euclid(period), initial);
            assert!(
                (actual - expected).abs() < 1e-6,
                "t={time:e}: {actual:e} versus {expected:e}"
            );
        }
    }
}

#[test]
fn behavioral_polynomial_harmonics_drive_an_accurate_refined_rc_orbit() {
    for (power, dc, coefficients) in [
        (2, 0.5, vec![-0.5]),
        (4, 0.375, vec![-0.5, 0.125]),
        (
            8,
            35.0 / 128.0,
            vec![-7.0 / 16.0, 7.0 / 32.0, -1.0 / 16.0, 1.0 / 128.0],
        ),
    ] {
        let netlist = Netlist::parse(&format!(
            "polynomial harmonics\nB1 in 0 V=sin(2*pi*64meg*time)^{power}\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        )).unwrap();
        let requested = PssConfig::new(F0).with_tstab_periods(0);
        let point = Engine::default()
            .run_pss_operating_point_with_abort(&netlist, requested.clone(), &NoAbort)
            .unwrap();
        assert_eq!(
            point.config(),
            &requested,
            "refinement must not rewrite source defaults"
        );
        let result = &point.analysis().result;
        let steps = result.time.len() - 1;
        assert!(steps > 2 * 64 * power);
        assert_eq!(point.spectral_harmonic_capacity(), steps / 2);
        let restored = rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
            point.producer_identity().unwrap().clone(),
            point.config().clone(),
            point.analysis().clone(),
            point.shooting_state_basis().to_vec(),
            point.shooting_state().to_vec(),
        )
        .unwrap();
        assert_eq!(restored, point);
        if power == 4 {
            let mut malformed = point.analysis().clone();
            malformed.result.time[1] *= 1.5;
            let error = rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
                point.producer_identity().unwrap().clone(),
                requested.clone(),
                malformed,
                point.shooting_state_basis().to_vec(),
                point.shooting_state().to_vec(),
            )
            .unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("authenticated producer identity"),
                "{error}"
            );
        }
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let values = &result.waveforms[output].values;
        let mean = values[..steps].iter().sum::<f64>() / steps as f64;
        assert!(
            (mean - dc).abs() < 1e-4,
            "power={power}, steps={steps}: DC={mean}, expected={dc}"
        );
        for (&time, &actual) in result.time.iter().zip(values) {
            let expected = dc
                + coefficients
                    .iter()
                    .enumerate()
                    .map(|(index, coefficient)| {
                        let omega = std::f64::consts::TAU * F0 * 128.0 * (index + 1) as f64;
                        let wrc = omega * R * C;
                        let phase = omega * time;
                        coefficient * (phase.cos() + wrc * phase.sin()) / (1.0 + wrc * wrc)
                    })
                    .sum::<f64>();
            assert!(
                (actual - expected).abs() < 1e-3 * dc + 1e-6,
                "power={power}, steps={steps}, t={time:e}: actual={actual}, expected={expected}"
            );
        }
    }
}

#[test]
fn pss_refines_source_clocks_beyond_the_requested_grid_nyquist_limit() {
    use rspice_core::engine::PssDcOperatingPointSeed;
    for source in [
        "V1 in 0 SIN(0 1 128meg)",
        "V1 in 0 SIN(0 1 256meg)",
        "I1 0 in SIN(0 1m 128meg)",
        "B1 in 0 V=sin(2*pi*256meg*time)",
        "B1 0 in I=1m*cos(-2*pi*128meg*time+0.3)",
        "B1 in 0 V=spice_sin(0,1,128meg)",
        "V1 in 0 SFFM(0 1 256meg 0.3 1meg)",
        "V1 in 0 SFFM(0 1 1meg 0.3 128meg)",
        "V1 in 0 AM(0 1 1 64meg 64meg)",
        "B1 in 0 V=spice_sffm(0,1,1meg,0.3,128meg)",
        "B1 in 0 V=sin(2*pi*1meg*time+0.3*sin(2*pi*128meg*time))",
        "V1 in 0 DC 0 AC 1 portnum=1 z0=50 pwr=0.001 freq=128meg",
    ] {
        let netlist = Netlist::parse(&format!(
            "aliased PSS forcing\n{source}\nR1 in out {R}\nR2 in 0 1meg\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        let engine = Engine::default();
        let config = PssConfig::new(F0).with_tstab_periods(0);
        let circuit = engine.build_circuit(&netlist).unwrap();
        let seed = PssDcOperatingPointSeed::try_new(
            circuit.node_names_sorted(),
            circuit.branch_names_sorted(),
            vec![0.0; circuit.matrix_size()],
        )
        .unwrap();
        let selected = if source.starts_with('B') {
            Vec::new()
        } else {
            vec![source.split_whitespace().next().unwrap().to_owned()]
        };
        engine
            .validate_pss_source_contract_with_abort(&netlist, &selected, &config, &NoAbort)
            .unwrap_or_else(|error| panic!("{source}: {error}"));
        let mut runs = vec![engine.run_pss_with_abort(&netlist, config.clone(), &NoAbort)];
        // Exercise every entry point on one drive; the remaining cases cover
        // distinct waveform evaluators without repeating the same solver work.
        if source == "V1 in 0 SIN(0 1 128meg)" {
            runs.extend([
                engine.run_pss(&netlist, config.clone()),
                engine
                    .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
                    .map(|point| point.analysis().clone()),
                engine
                    .run_pss_with_continuation_state(&netlist, config.clone())
                    .map(|(analysis, _)| analysis),
                engine
                    .run_pss_with_frozen_source_continuation_state(&netlist, config.clone(), &[])
                    .map(|(analysis, _)| analysis),
                engine
                    .run_pss_operating_point_with_dc_seed_and_abort(
                        &netlist,
                        config.clone(),
                        &seed,
                        &NoAbort,
                    )
                    .map(|point| point.analysis().clone()),
            ]);
        }
        for run in runs {
            let analysis = run.unwrap_or_else(|error| panic!("{source}: {error}"));
            assert!(
                analysis.result.time.len() > config.points_per_period + 1,
                "{source}"
            );
            assert!(analysis.final_residual.is_finite());
        }
    }
}

#[test]
fn pss_sampling_preflight_preserves_sub_nyquist_and_inactive_tones() {
    for source in [
        "V1 in 0 SIN(0 1 127meg)",
        "V1 in 0 SIN(0 0 256meg)",
        "B1 in 0 V=sin(2*pi*127meg*time)",
        "B1 in 0 V=spice_sin(0,0,256meg)",
        "V1 in 0 SFFM(0 1 1meg 0 256meg)",
        "B1 in 0 V=spice_sffm(0,1,1meg,0,256meg)",
    ] {
        let netlist = Netlist::parse(&format!(
            "sampling boundary\n{source}\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        let selected = if source.starts_with('B') {
            vec![]
        } else {
            vec!["V1".to_owned()]
        };
        Engine::default()
            .validate_pss_source_contract_with_abort(
                &netlist,
                &selected,
                &PssConfig::new(F0),
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("{source}: {error}"));
    }
}

#[test]
fn resolved_high_harmonic_rc_orbits_converge_toward_the_analytic_waveform() {
    let harmonic = 128.0;
    let omega = std::f64::consts::TAU * F0 * harmonic;
    let wrc = omega * R * C;
    let gain = 1.0 / (1.0 + wrc * wrc).sqrt();
    for source in ["V1 in 0 SIN(0 1 128meg)", "B1 in 0 V=sin(2*pi*128meg*time)"] {
        let netlist = Netlist::parse(&format!(
            "resolved high harmonic\n{source}\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        for points in [4096, 8192] {
            let point = Engine::default()
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(F0)
                        .with_tstab_periods(0)
                        .with_points_per_period(points),
                    &NoAbort,
                )
                .unwrap();
            let analysis = point.analysis();
            assert!(
                analysis.iterations > 0,
                "the physical response must not alias to DC"
            );
            let result = &analysis.result;
            let actual_steps = result.time.len() - 1;
            assert!(actual_steps >= points);
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let error = result
                .time
                .iter()
                .zip(&result.waveforms[output].values)
                .map(|(&time, &actual)| {
                    let phase = omega * time;
                    let expected = (phase.sin() - wrc * phase.cos()) / (1.0 + wrc * wrc);
                    (actual - expected).abs() / gain
                })
                .fold(0.0, f64::max);
            // Mesh qualification must bound the complete physical waveform,
            // independently of the shooting seam and requested initial mesh.
            assert!(
                error < 1e-3 + 1e-6 / gain,
                "{source}, POINTS={points}, actual={actual_steps}: normalized error {error:e}"
            );
        }
    }
}

#[test]
fn prescribed_pwl_current_preserves_its_physical_voltage_under_time_scaling() {
    for period in [1e-6, 1e-18, 1e-30] {
        let netlist = Netlist::parse(&format!(
            "scaled PWL winding current\nI1 0 a PWL(0 0 {:e} 1m {period:e} 0) R=0\nL1 a out {:e}\nR1 out 0 1k\n.end\n",
            period / 2.0, 1000.0 * period,
        )).unwrap();
        let result = Engine::default()
            .run_pss(
                &netlist,
                PssConfig::new(1.0 / period)
                    .with_tstab_periods(0)
                    .with_points_per_period(128),
            )
            .unwrap_or_else(|error| panic!("period={period:e}: {error}"));
        let result = &result.result;
        let node = |name: &str| {
            result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap()
        };
        for (index, &time) in result.time.iter().enumerate() {
            let phase = if index == 128 { 0.0 } else { time / period };
            let current_voltage = if phase < 0.5 {
                2.0 * phase
            } else {
                2.0 * (1.0 - phase)
            };
            let winding_voltage = if phase < 0.5 { 2.0 } else { -2.0 };
            let output = result.waveforms[node("out")].values[index];
            let input = result.waveforms[node("a")].values[index];
            assert!(
                (output - current_voltage).abs() < 1e-12,
                "period={period:e}, phase={phase}: OUT={output}"
            );
            assert!(
                (input - output - winding_voltage).abs() < 1e-12,
                "period={period:e}, phase={phase}: winding={}",
                input - output
            );
        }
    }
}

#[test]
fn every_pss_entry_point_rejects_nonperiodic_forcing_even_when_endpoints_alias() {
    use rspice_core::engine::PssDcOperatingPointSeed;
    for source in [
        "V1 out 0 SIN(0 1 1.25meg)",
        "V1 out 0 SIN(0 1 1.5meg)",
        "V1 out 0 SIN(0 1 256.5meg)",
        "V1 out 0 SIN(0 1 1meg 0 1000)",
        "I1 0 out SIN(0 1m 1.5meg)",
        "B1 out 0 V=sin(2*pi*1.25meg*time)",
        "B1 out 0 V=sin(2*pi*1.5meg*time)",
        "B1 out 0 V=1meg*time",
        "B1 0 out I=1m*sin(2*pi*1.5meg*time)",
        "B1 out 0 V=spice_sin(0,1,1.5meg)",
        "V1 out 0 DC 0 AC 1 portnum=1 z0=50 pwr=0.001 freq=1.5meg",
    ] {
        let netlist = Netlist::parse(&format!(
            "nonperiodic forcing\n{source}\nR1 out 0 1k\nC1 out 0 1p\n.end\n"
        ))
        .unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let seed = PssDcOperatingPointSeed::try_new(
            circuit.node_names_sorted(),
            circuit.branch_names_sorted(),
            vec![0.0; circuit.matrix_size()],
        )
        .unwrap();
        let config = PssConfig::new(F0).with_tstab_periods(0);
        let errors = [
            engine.run_pss(&netlist, config.clone()).unwrap_err(),
            engine
                .run_pss_with_abort(&netlist, config.clone(), &NoAbort)
                .unwrap_err(),
            engine
                .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
                .unwrap_err(),
            engine
                .run_pss_operating_point_with_dc_seed_and_abort(
                    &netlist,
                    config.clone(),
                    &seed,
                    &NoAbort,
                )
                .unwrap_err(),
            engine
                .run_pss_with_continuation_state(&netlist, config.clone())
                .unwrap_err(),
            engine
                .run_pss_with_frozen_source_continuation_state(&netlist, config, &[])
                .unwrap_err(),
        ];
        for error in errors {
            assert!(
                error
                    .to_string()
                    .contains("analysis.pss.driven_source_waveform"),
                "{source}: {error}"
            );
        }
    }
}

#[test]
fn source_selection_and_solver_share_resolved_periodicity_for_all_waveform_routes() {
    use rspice_core::config::SpiceDialect;
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for source in [
            "V1 in 0 SIN(0 1 0)",
            "V1 in 0 SFFM(0 1 2meg 0.3 1meg)",
            "V1 in 0 PWL(0 0 0.5u 1 1u 0) R=0",
            "V1 in 0 DC 0 AC 1 portnum=1 z0=50 pwr=0.001 freq=2meg",
            "B1 in 0 V=sin(2*pi*1meg*time+0.3)",
            "B1 in 0 V=spice_sin(0,1,1meg,0,0,37)",
            "B1 in 0 V=spice_sffm(0,1,2meg,0.3,1meg)",
            "B1 in 0 V=spice_pulse(0,1,0,0.1u,0.1u,0.3u,1u)",
            "B1 in 0 V=table(time%1u,0,0,0.5u,1,1u,0)",
        ] {
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let netlist = Netlist::parse(&format!(
                "periodic forcing\n{source}\nR1 in out 1k\nC1 out 0 1n\n.end\n"
            ))
            .unwrap();
            let config = PssConfig::new(F0)
                .with_tstab_periods(0)
                .with_points_per_period(128);
            let selected = if source.starts_with('V') {
                vec!["V1".to_owned()]
            } else {
                Vec::new()
            };
            engine
                .validate_pss_source_contract_with_abort(&netlist, &selected, &config, &NoAbort)
                .unwrap_or_else(|error| panic!("{dialect:?}, {source}: {error}"));
            let analysis = engine
                .run_pss(&netlist, config)
                .unwrap_or_else(|error| panic!("{dialect:?}, {source}: {error}"));
            assert!(analysis.result.residual_norm.is_finite());
            for waveform in &analysis.result.waveforms {
                let first = waveform.values.first().unwrap();
                let last = waveform.values.last().unwrap();
                assert!(
                    (last - first).abs() < 1e-6,
                    "{dialect:?}, {source}: seam {}",
                    last - first
                );
            }
        }
    }
}

#[test]
fn autonomous_pss_rejects_a_drive_or_startup_kick_inside_the_orbit_window() {
    for source in [
        "I1 0 out SIN(0 1m 1meg)",
        "I1 0 out PULSE(0 1m 0.25u 1n 1n 0.1u 1)",
        "I1 0 out PULSE(0 1m 1u 1n 1n 0.1u 1)",
        "B1 0 out I=sin(2*pi*1meg*time)",
    ] {
        let netlist = Netlist::parse(&format!(
            "forced oscillator\n{source}\nR1 out 0 1k\nC1 out 0 1n\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_pss(
                &netlist,
                PssConfig::autonomous()
                    .with_period_guess(1e-6)
                    .with_tstab_periods(0),
            )
            .unwrap_err();
        assert!(error.to_string().contains("source"), "{source}: {error}");
        assert!(
            error.to_string().contains("analysis.pss."),
            "{source}: {error}"
        );
    }
}

#[test]
fn periodic_source_preflight_uses_the_authored_point_count_for_default_edges() {
    let engine = Engine::default();
    // The delay fits the repeated low segment only at the finer grid, whose
    // omitted native rise/fall defaults are shorter. The source is not
    // periodic from zero at 32 points, but is periodic at 512 points.
    let netlist = Netlist::parse("grid-dependent source defaults\nV1 in 0 PULSE(0 1 0.7u 0 0 0.28u 1u)\nR1 in out 1k\nC1 out 0 1n\n.end\n").unwrap();
    for (points, periodic) in [(32, false), (512, true)] {
        let config = PssConfig::new(F0)
            .with_harmonics(4)
            .with_points_per_period(points)
            .with_tstab_periods(0);
        let preflight = engine.validate_pss_source_contract_with_abort(
            &netlist,
            &["V1".to_owned()],
            &config,
            &NoAbort,
        );
        assert_eq!(preflight.is_ok(), periodic, "{points}: {preflight:?}");
        let solve = engine.run_pss(&netlist, config);
        assert_eq!(solve.is_ok(), periodic, "{points}: {solve:?}");
    }
}

#[test]
fn small_signal_shooting_closes_the_orbit_from_a_zero_initial_state() {
    for amplitude in [1.0_f64, 1e-3, 1e-6, 1e-7, 1e-9] {
        let netlist = Netlist::parse(&format!(
            "small-signal shooting\nV1 in 0 SIN(0 {amplitude} 1meg)\nR1 in out {R}\nC1 out 0 {C}\n.end\n"
        ))
        .unwrap();
        let result = Engine::default()
            .run_pss(
                &netlist,
                PssConfig::new(F0)
                    .with_tstab_periods(0)
                    .with_points_per_period(512),
            )
            .unwrap();
        let node = result
            .result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let voltage = &result.result.waveforms[node].values;
        assert!(result.iterations > 0, "amplitude={amplitude:e}");
        assert!((voltage[0] / amplitude + 0.5).abs() < 1e-6);
        assert!((voltage.last().unwrap() - voltage[0]).abs() < 1e-8 * amplitude);
        // Bound the existing O(dt^2) grid error, including the first BE
        // interval, independently of the stricter periodic seam check.
        let grid_bound = 0.5 * (std::f64::consts::TAU / 512.0).powi(2);
        for (&time, &actual) in result.result.time.iter().zip(voltage) {
            let angle = std::f64::consts::TAU * F0 * time;
            let expected = 0.5 * (angle.sin() - angle.cos());
            assert!(
                (actual / amplitude - expected).abs() < grid_bound,
                "amplitude={amplitude:e}, t={time:e}, normalized={:e}, expected={expected:e}",
                actual / amplitude,
            );
        }
    }
}

#[test]
fn pss_sources_use_the_configured_dialect_and_one_period_for_time_defaults() {
    use rspice_core::config::SpiceDialect;
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for current in [false, true] {
            for modulated in [false, true] {
                let source = match (current, modulated) {
                    (false, false) if dialect == SpiceDialect::Xyce => "SFFM(0 1)",
                    (true, false) if dialect == SpiceDialect::Xyce => "SFFM(0 1m)",
                    (false, false) => "SIN(0 1 0)",
                    (true, false) => "SIN(0 1m 0)",
                    (false, true) => "SFFM(0 1 1meg 5 1meg)",
                    (true, true) => "SFFM(0 1m 1meg 5 1meg)",
                };
                let deck = if current {
                    format!(
                        "PSS source context\nI1 0 a {source}\nL1 a out 100u\nR1 out 0 1k\n.end\n"
                    )
                } else {
                    format!("PSS source context\nV1 out 0 {source}\nC1 out 0 1p\n.end\n")
                };
                let netlist = Netlist::parse(&deck).unwrap();
                for stabilization in [0, 3] {
                    let analysis = engine
                        .run_pss(
                            &netlist,
                            PssConfig::new(F0)
                                .with_points_per_period(256)
                                .with_tstab_periods(stabilization),
                        )
                        .unwrap();
                    let result = &analysis.result;
                    let out = result
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case("out"))
                        .unwrap();
                    let mdi = if !modulated {
                        0.0
                    } else if dialect == SpiceDialect::Xyce {
                        5.0
                    } else {
                        1.0
                    };
                    for (&time, &actual) in result.time.iter().zip(&result.waveforms[out].values) {
                        let angle = std::f64::consts::TAU * F0 * time;
                        let expected = (angle + mdi * angle.sin()).sin();
                        assert!(
                            (actual - expected).abs() < 1e-11,
                            "{dialect:?}, current={current}, {source}, stabilization={stabilization}, t={time:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn xyce_sine_requires_a_frequency_and_preserves_an_authored_zero() {
    use rspice_core::config::SpiceDialect;
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let netlist =
        Netlist::parse("Xyce zero frequency\nV1 out 0 SIN(2 1 0 0 0 37)\nC1 out 0 1p\n.end\n")
            .unwrap();
    let pss = engine
        .run_pss(&netlist, PssConfig::new(F0).with_tstab_periods(0))
        .unwrap();
    let expected = 2.0 + 37.0_f64.to_radians().sin();
    for &actual in &pss.result.waveforms[0].values {
        assert!((actual - expected).abs() < 1e-12);
    }
    let transient = engine.run_tran(&netlist, 1e-6, 1e-9).unwrap();
    for &actual in &transient.voltages[0] {
        assert!((actual - expected).abs() < 1e-12);
    }
    for source in ["V1 out 0 SIN(0 1)", "I1 0 out DC 0 SIN(0 1)"] {
        let netlist = Netlist::parse(&format!(
            "Xyce required frequency\n{source}\nR1 out 0 1k\nC1 out 0 1p\n.end\n"
        ))
        .unwrap();
        let error = engine.build_circuit(&netlist).unwrap_err().to_string();
        assert!(error.contains("requires an authored frequency"), "{error}");
    }
}

#[test]
fn prescribed_dc_current_voltages_do_not_depend_on_the_companion_scale() {
    use rspice_core::numerics::integration::IntegrationMethod;
    let engine = Engine::default();
    for coupled in [false, true] {
        let secondary = if coupled {
            "I2 0 c DC 2m\nL2 c d 200u\nR2 d 0 100\nK1 L1 L2 0.6\n"
        } else {
            ""
        };
        let netlist = Netlist::parse(&format!(
            "prescribed DC flux scaling\nI1 0 a DC 1m\nL1 a b 100u\nR1 b 0 100\n{secondary}.end\n"
        ))
        .unwrap();
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            for frequency in [1e-3, 1e3, 1e9, 1e15, 1e21] {
                let mut config = PssConfig::new(frequency)
                    .with_tstab_periods(0)
                    .with_points_per_period(128);
                config.integration_method = Some(method);
                let analysis = engine.run_pss(&netlist, config).unwrap();
                for (node, name) in analysis.result.node_names.iter().enumerate() {
                    let expected =
                        if name.eq_ignore_ascii_case("a") || name.eq_ignore_ascii_case("b") {
                            0.1
                        } else {
                            0.2
                        };
                    for &actual in &analysis.result.waveforms[node].values {
                        assert!(
                            (actual - expected).abs() < 1e-12,
                            "coupled={coupled}, {method:?}, f={frequency:e}, {name}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn prescribed_current_ripple_voltage_is_independent_of_dc_bias() {
    use rspice_core::numerics::integration::IntegrationMethod;
    let omega = std::f64::consts::TAU * F0;
    for coupled in [false, true] {
        for bias in [1e-3, 1.0, 1e3, 1e6] {
            let secondary = if coupled {
                format!(
                    "I2 0 c SIN({} 2m 1meg 0 0 -23)\nL2 c d 200u\nR2 d 0 0.0001\nK1 L1 L2 0.6\n",
                    -0.5 * bias
                )
            } else {
                String::new()
            };
            let netlist = Netlist::parse(&format!(
                "prescribed ripple precision\nI1 0 a SIN({bias} 1m 1meg 0 0 37)\nL1 a b 100u\nR1 b 0 0.0001\n{secondary}.options reltol=1e-12 vntol=1e-12 abstol=1e-15\n.end\n"
            )).unwrap();
            for method in [
                IntegrationMethod::BackwardEuler,
                IntegrationMethod::Trapezoidal,
                IntegrationMethod::Gear2,
                IntegrationMethod::TrapGear,
            ] {
                let mut config = PssConfig::new(F0)
                    .with_tstab_periods(0)
                    .with_points_per_period(256);
                config.integration_method = Some(method);
                let analysis = Engine::default().run_pss(&netlist, config).unwrap();
                let result = &analysis.result;
                let node = |name: &str| {
                    result
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case(name))
                        .unwrap()
                };
                let mutual = if coupled {
                    0.6 * (100e-6_f64 * 200e-6).sqrt()
                } else {
                    0.0
                };
                for (sample, &time) in result.time.iter().enumerate() {
                    let rate1 = 1e-3 * omega * (omega * time + 37.0_f64.to_radians()).cos();
                    let rate2 = 2e-3 * omega * (omega * time - 23.0_f64.to_radians()).cos();
                    for (pos, neg, expected) in [
                        ("a", "b", 100e-6 * rate1 + mutual * rate2),
                        ("c", "d", 200e-6 * rate2 + mutual * rate1),
                    ]
                    .into_iter()
                    .take(if coupled { 2 } else { 1 })
                    {
                        let actual = result.waveforms[node(pos)].values[sample]
                            - result.waveforms[node(neg)].values[sample];
                        assert!(
                            (actual - expected).abs() < 1e-10,
                            "coupled={coupled}, bias={bias:e}, {method:?}, {pos}-{neg}, t={time:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_source_prescribes_its_series_winding_current() {
    use rspice_core::numerics::integration::IntegrationMethod;
    let engine = Engine::default();
    let phase = 37.0_f64.to_radians();
    let omega = std::f64::consts::TAU * F0;
    for (source, amplitude) in [("DC 1m", 0.0), ("SIN(1m 2m 1meg 0 0 37)", 2e-3)] {
        for reversed in [false, true] {
            let winding = if reversed { "b a" } else { "a b" };
            let netlist = Netlist::parse(&format!(
                "prescribed winding current\nI1 0 a {source}\nI2 0 a DC -0.5m\nL1 {winding} 100u\nR1 b 0 100\n.end\n"
            )).unwrap();
            for method in [
                IntegrationMethod::BackwardEuler,
                IntegrationMethod::Trapezoidal,
                IntegrationMethod::Gear2,
                IntegrationMethod::TrapGear,
            ] {
                for stabilization in [0, 2] {
                    let mut config = PssConfig::new(F0)
                        .with_tstab_periods(stabilization)
                        .with_points_per_period(256);
                    config.integration_method = Some(method);
                    let point = engine
                        .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
                        .unwrap_or_else(|error| panic!("{source}, {method:?}: {error}"));
                    assert!(point.shooting_state_basis().is_empty());
                    assert!(point.analysis().monodromy.is_empty());
                    let result = &point.analysis().result;
                    for (node, name) in result.node_names.iter().enumerate() {
                        for (&time, &actual) in
                            result.time.iter().zip(&result.waveforms[node].values)
                        {
                            let angle = omega * time + phase;
                            let current = 0.5e-3 + amplitude * angle.sin();
                            let expected = 100.0 * current
                                + if name.eq_ignore_ascii_case("a") {
                                    100e-6 * amplitude * omega * angle.cos()
                                } else {
                                    0.0
                                };
                            assert!(
                                (actual - expected).abs() < 1e-10,
                                "{source}, {method:?}, reversed={reversed}, {name} at {time:e}: {actual:e} vs {expected:e}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn prescribed_current_requires_a_continuous_periodic_drive() {
    let engine = Engine::default();
    for source in [
        "SIN(0 1m 1.25meg)",
        "SIN(0 1m 1meg 0 1000)",
        "EXP(0 1m 0.1u 0.2u 0.7u 0.1u)",
        "PWL(0 0 0.3u 1m 0.3u -1m 1u 0) R=0",
        "PWL(0 0 1u 1m) R=0",
        "PWL(0 0 0.5u 1m 1u 0)",
        "PULSE(0 1m 0 0.2u 0.2u 0.9u 1u)",
        "PULSE(0 1m 0 0.2u 0.2u 0.3u 1u 2)",
    ] {
        let netlist = Netlist::parse(&format!(
            "irregular prescribed current\nI1 0 a {source}\nL1 a b 100u\nR1 b 0 100\n.end\n"
        ))
        .unwrap();
        let error = engine
            .run_pss(&netlist, PssConfig::new(F0).with_tstab_periods(0))
            .unwrap_err();
        assert!(
            matches!(&error, SimulationError::Circuit(message) if message.contains("prescribed current 'I1'") && message.contains("continuous and periodic")),
            "{source}: {error}"
        );
    }
}

#[test]
fn piecewise_linear_prescribed_current_retains_the_outgoing_voltage_at_corners() {
    let engine = Engine::default();
    // A continuous triangular wave has finite, discontinuous inductor voltage.
    // PULSE and repeating PWL encode the same waveform by independent routes.
    for source in [
        "PWL(0 0.2m 0.5u 1.2m 1u 0.2m) R=0",
        "PULSE(0.2m 1.2m 0 0.5u 0.5u 0 1u)",
    ] {
        let netlist = Netlist::parse(&format!(
            "regular piecewise current\nI1 0 a {source}\nL1 a b 100u\nR1 b 0 100\n.end\n"
        ))
        .unwrap();
        let result = engine
            .run_pss(
                &netlist,
                PssConfig::new(F0)
                    .with_tstab_periods(0)
                    .with_points_per_period(128),
            )
            .unwrap()
            .result;
        for (node, name) in result.node_names.iter().enumerate() {
            for (&time, &actual) in result.time.iter().zip(&result.waveforms[node].values) {
                let rising = time < 0.5e-6 || time == 1e-6;
                let current = if time <= 0.5e-6 {
                    0.2e-3 + 2000.0 * time
                } else {
                    1.2e-3 - 2000.0 * (time - 0.5e-6)
                };
                let expected = 100.0 * current
                    + if name.eq_ignore_ascii_case("a") {
                        if rising { 0.2 } else { -0.2 }
                    } else {
                        0.0
                    };
                assert!(
                    (actual - expected).abs() < 1e-11,
                    "{source}, {name}, t={time:e}: {actual:e} vs {expected:e}"
                );
            }
        }
    }
}

#[test]
fn prescribed_current_drives_mutual_flux_without_adding_a_shooting_coordinate() {
    use num_complex::Complex64;
    use rspice_core::numerics::integration::IntegrationMethod;
    let omega = std::f64::consts::TAU * F0;
    let drive = Complex64::from_polar(2e-3, 37.0_f64.to_radians());
    for reversed in [false, true] {
        let winding = if reversed { "mid a" } else { "a mid" };
        let mutual = if reversed { -1.0 } else { 1.0 } * 0.6 * (100e-6_f64 * 200e-6).sqrt();
        let z_mutual = Complex64::new(0.0, omega * mutual);
        let i2 = -z_mutual * drive / Complex64::new(200.0, omega * 200e-6);
        let voltages = [
            ("mid", 100.0 * drive, 0.1),
            (
                "a",
                Complex64::new(100.0, omega * 100e-6) * drive + z_mutual * i2,
                0.1,
            ),
            ("b", -200.0 * i2, 0.0),
        ];
        let netlist = Netlist::parse(&format!(
            "prescribed mutual flux\nI1 0 a SIN(1m 2m 1meg 0 0 37)\nL1 {winding} 100u\nR1 mid 0 100\nL2 b 0 200u\nR2 b 0 200\nK1 L1 L2 0.6\n.end\n"
        )).unwrap();
        let engine = Engine::default();
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            let mut previous_error = f64::INFINITY;
            let mut previous_steps = 0;
            for points in [256, 512, 1024] {
                let mut config = PssConfig::new(F0)
                    .with_tstab_periods(if points == 512 { 2 } else { 0 })
                    .with_points_per_period(points)
                    .with_tolerance(1e-11);
                config.integration_method = Some(method);
                let point = engine
                    .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
                    .unwrap();
                assert_eq!(point.shooting_state_basis(), ["L:L2"]);
                let analysis = point.analysis();
                let mut error: f64 = 0.0;
                for (name, phasor, dc) in voltages {
                    let node = analysis
                        .result
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case(name))
                        .unwrap();
                    for (&time, &actual) in analysis
                        .result
                        .time
                        .iter()
                        .zip(&analysis.result.waveforms[node].values)
                    {
                        let expected = dc
                            + phasor.re * (omega * time).sin()
                            + phasor.im * (omega * time).cos();
                        error = error.max((actual - expected).abs() / phasor.norm());
                    }
                }
                let actual_steps = analysis.result.time.len() - 1;
                if actual_steps > previous_steps {
                    assert!(
                        error < 0.6 * previous_error,
                        "{method:?}, points={points}, actual={actual_steps}, error={error:e}, previous={previous_error:e}"
                    );
                } else {
                    assert!(error <= previous_error * 1.01 + 1e-9);
                }
                previous_error = error;
                previous_steps = actual_steps;
                if points == 1024 {
                    assert!(
                        error
                            < if method == IntegrationMethod::BackwardEuler {
                                0.004
                            } else {
                                0.0001
                            },
                        "{method:?}: {error:e}"
                    );
                    let multiplier = analysis.floquet_multipliers[0];
                    assert!(multiplier.im.abs() < 1e-10);
                    assert!((multiplier.re / (-1.0_f64).exp() - 1.0).abs() < 0.001);
                    let (_, state) = engine
                        .run_pss_with_continuation_state(&netlist, config)
                        .unwrap();
                    let (continued, _) = engine
                        .run_tran_from_pss_state(&netlist, &state, 1e-6, 1e-6 / 1024.0)
                        .unwrap();
                    for (name, phasor, dc) in voltages {
                        let node = continued
                            .node_names
                            .iter()
                            .position(|n| n.eq_ignore_ascii_case(name))
                            .unwrap();
                        for (&time, &actual) in continued.time.iter().zip(&continued.voltages[node])
                        {
                            let expected = dc
                                + phasor.re * (omega * time).sin()
                                + phasor.im * (omega * time).cos();
                            assert!(
                                (actual - expected).abs() < 0.007 * phasor.norm(),
                                "continuation {method:?}, {name}, t={time:e}: {actual:e} vs {expected:e}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn coupled_winding_history_preserves_the_transformer_orbit_and_continuation() {
    use num_complex::Complex64;
    let engine = Engine::default();
    for (coupling, secondary, orientation) in [
        (-0.6, "b 0", 1.0),
        (0.6, "0 b", -1.0),
        (0.6, "b 0", 1.0),
        (-0.6, "0 b", -1.0),
    ] {
        let netlist = Netlist::parse(&format!(
            "mutual flux history\nV1 in 0 SIN(0.5 1 1meg)\nR1 in a 50\nL1 a 0 100u\nL2 {secondary} 200u\nR2 b 0 100\nK1 L1 L2 {coupling}\n.end\n"
        )).unwrap();
        let omega = std::f64::consts::TAU * F0;
        let z1 = Complex64::new(50.0, omega * 100e-6);
        let z2 = Complex64::new(100.0, omega * 200e-6);
        let zm = Complex64::new(
            0.0,
            omega * coupling * orientation * (100e-6_f64 * 200e-6).sqrt(),
        );
        let determinant = z1 * z2 - zm * zm;
        let transfers = [
            ("a", 1.0 - 50.0 * z2 / determinant),
            ("b", 100.0 * zm / determinant),
        ];
        let mut previous_error = f64::INFINITY;
        for points in [256, 512, 1024] {
            let config = PssConfig::new(F0)
                .with_tstab_periods(if points == 512 { 2 } else { 0 })
                .with_points_per_period(points)
                .with_tolerance(1e-11);
            let (analysis, state) = engine
                .run_pss_with_continuation_state(&netlist, config)
                .unwrap();
            let mut error: f64 = 0.0;
            for (name, transfer) in transfers {
                let result = &analysis.result;
                let node = result
                    .node_names
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(name))
                    .unwrap();
                for (&time, &voltage) in result.time.iter().zip(&result.waveforms[node].values) {
                    let expected =
                        transfer.re * (omega * time).sin() + transfer.im * (omega * time).cos();
                    error = error.max((voltage - expected).abs() / transfer.norm());
                }
            }
            eprintln!("K={coupling}, points={points}, relative waveform error={error:e}");
            assert!(
                error < previous_error,
                "mutual-flux error must decrease with grid refinement"
            );
            previous_error = error;
            if points == 1024 {
                assert_eq!(analysis.monodromy.len(), 2);
                let mut multipliers = analysis
                    .floquet_multipliers
                    .iter()
                    .map(|value| {
                        assert!(value.im.abs() < 1e-10);
                        value.re
                    })
                    .collect::<Vec<_>>();
                multipliers.sort_by(f64::total_cmp);
                // Both R/L ratios equal 5e5 /s. In normalized winding
                // coordinates the inductance eigenvalues are 1 +/- |k|.
                for (actual, sign) in multipliers.into_iter().zip([-1.0, 1.0]) {
                    let expected = (-0.5 / (1.0 + sign * coupling.abs())).exp();
                    assert!(
                        (actual / expected - 1.0).abs() < 2e-6,
                        "decay multiplier {actual:e} versus {expected:e}"
                    );
                }
                assert!(
                    error < 0.00002,
                    "the coupled orbit must match its exact impedance matrix"
                );
                let (continued, _) = engine
                    .run_tran_from_pss_state(&netlist, &state, 2e-6, 1e-6 / 1024.0)
                    .unwrap();
                for (name, transfer) in transfers {
                    let node = continued
                        .node_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))
                        .unwrap();
                    for (&time, &voltage) in continued.time.iter().zip(&continued.voltages[node]) {
                        let expected =
                            transfer.re * (omega * time).sin() + transfer.im * (omega * time).cos();
                        assert!((voltage - expected).abs() < 0.002 * transfer.norm());
                    }
                }
            }
        }
    }
}

#[test]
fn three_coupled_windings_match_the_full_flux_matrix_and_its_modes() {
    use num_complex::Complex64;
    let netlist = Netlist::parse(
        "three-winding flux\nV1 in 0 SIN(0 1 1meg)\nR1 in a 50\nL1 a 0 100u\nL2 b 0 200u\nR2 b 0 100\nL3 c 0 400u\nR3 c 0 200\nK1 L1 L2 0.25\nK2 L2 L3 0.25\nK3 L1 L3 0.25\n.end\n",
    ).unwrap();
    let analysis = Engine::default()
        .run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_tstab_periods(0)
                .with_points_per_period(1024)
                .with_tolerance(1e-11),
        )
        .unwrap();
    let omega = std::f64::consts::TAU * F0;
    // With R_i/L_i = a and equal k, Z = sqrt(L) (d I + s 11^T) sqrt(L).
    // Sherman-Morrison gives the exact driven current in every winding.
    let d = Complex64::new(5e5, omega * 0.75);
    let s = Complex64::new(0.0, omega * 0.25);
    let i1 = (1.0 - s / (d + 3.0 * s)) / (100e-6 * d);
    let i2 = -s / ((100e-6_f64 * 200e-6).sqrt() * d * (d + 3.0 * s));
    let i3 = -s / ((100e-6_f64 * 400e-6).sqrt() * d * (d + 3.0 * s));
    for (name, transfer) in [
        ("a", 1.0 - 50.0 * i1),
        ("b", -100.0 * i2),
        ("c", -200.0 * i3),
    ] {
        let node = analysis
            .result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap();
        for (&time, &voltage) in analysis
            .result
            .time
            .iter()
            .zip(&analysis.result.waveforms[node].values)
        {
            let expected = transfer.re * (omega * time).sin() + transfer.im * (omega * time).cos();
            assert!(
                (voltage - expected).abs() < 2e-5 * transfer.norm(),
                "{name} at {time:e}: {voltage:e} vs {expected:e}"
            );
        }
    }
    assert_eq!(analysis.monodromy.len(), 3);
    let mut multipliers = analysis
        .floquet_multipliers
        .iter()
        .map(|value| {
            assert!(value.im.abs() < 1e-9);
            value.re
        })
        .collect::<Vec<_>>();
    multipliers.sort_by(f64::total_cmp);
    for (actual, flux_eigenvalue) in multipliers.into_iter().zip([0.75, 0.75, 1.5]) {
        let expected = (-0.5_f64 / flux_eigenvalue).exp();
        assert!((actual / expected - 1.0).abs() < 2e-6);
    }
}

#[test]
fn series_inductors_share_one_current_state_and_preserve_the_voltage_division() {
    for (first, second) in [
        ("out mid", "mid 0"),
        ("mid out", "mid 0"),
        ("out mid", "0 mid"),
    ] {
        let netlist = Netlist::parse(&format!(
        "series flux coordinates\nV1 in 0 SIN(0 1 1meg)\nR1 in out 1k\nL1 {first} 40u\nL2 {second} 60u\n.end\n"
    ))
    .unwrap();
        let point = Engine::default()
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_tstab_periods(0)
                    .with_points_per_period(512)
                    .with_tolerance(1e-9),
                &NoAbort,
            )
            .expect("series inductor currents satisfy KCL and have one free coordinate");
        assert_eq!(point.shooting_state().len(), 1);
        let expected_multiplier = (-10.0_f64).exp();
        assert!(
            (point.analysis().floquet_multipliers[0].re / expected_multiplier - 1.0).abs() < 0.001
        );
        let result = &point.analysis().result;
        let out = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        let mid = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("mid"))
            .unwrap();
        let ratio = std::f64::consts::TAU * F0 * 100e-6 / 1e3;
        let amplitude = ratio / (1.0 + ratio * ratio).sqrt();
        for (index, &time) in result.time.iter().enumerate() {
            let expected =
                amplitude * (std::f64::consts::TAU * F0 * time + (1.0 / ratio).atan()).sin();
            let voltage = result.waveforms[out].values[index];
            assert!((voltage - expected).abs() < 0.002 * amplitude);
            assert!(
                (result.waveforms[mid].values[index] - 0.6 * voltage).abs() < 1e-8,
                "the initial sample must retain inductive voltage division too"
            );
        }
        let card = rspice_core::netlist::PstbCard {
            probe_instance: "L2".to_owned(),
            max_harmonics: 4,
            num_multipliers: 1,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        };
        let stability = Engine::default()
            .run_pstb_card_from_pss_with_abort(&netlist, &card, &point, &NoAbort)
            .expect("a dependent series winding still names a physical current probe");
        assert_eq!(stability.probe_state_index, Some(0));
        assert_eq!(stability.probe_instance, "L2");
        assert_eq!(stability.probe_participation, [1.0]);
        let changed = Netlist::parse(&format!("different carrier\nV1 in 0 SIN(0 1 1meg)\nR1 in out 2k\nL1 {first} 40u\nL2 {second} 60u\n.end\n")).unwrap();
        let error = Engine::default()
            .run_pstb_card_from_pss_with_abort(&changed, &card, &point, &NoAbort)
            .unwrap_err();
        assert!(error.to_string().contains("semantic circuit identity"));
        let (_, state) = Engine::default()
            .run_pss_with_continuation_state(&netlist, point.config().clone())
            .unwrap();
        let (continued, _) = Engine::default()
            .run_tran_from_pss_state(&netlist, &state, 2e-6, 1e-6 / 1024.0)
            .unwrap();
        for (index, &time) in continued.time.iter().enumerate() {
            let expected =
                amplitude * (std::f64::consts::TAU * F0 * time + (1.0 / ratio).atan()).sin();
            assert!((continued.voltages[out][index] - expected).abs() < 0.002 * amplitude);
            assert!(
                (continued.voltages[mid][index] - 0.6 * continued.voltages[out][index]).abs()
                    < 1e-8
            );
        }
    }
}

#[test]
fn independent_charge_initialization_preserves_xyce_ic_branches_and_parallel_constraints() {
    use rspice_core::config::SpiceDialect;
    use rspice_core::engine::PssDcOperatingPointSeed;

    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    for capacitors in [
        "C1 out 0 1n IC=0.2",
        "C1 out 0 0.4n IC=0.2\nC2 0 out 0.6n IC=-0.2",
        "C1 out 0 0.4n\nC2 0 out 0.6n IC=-0.2",
    ] {
        let netlist = Netlist::parse(&format!(
            "IC charge basis\nV1 in 0 SIN(0 1 1meg)\nR1 in out 1k\n{capacitors}\n.end\n"
        ))
        .unwrap();
        // Supply the initial state explicitly to isolate period-map
        // initialization from the ordinary DC IC-constraint solver.
        let circuit = engine.build_circuit(&netlist).unwrap();
        let seed = PssDcOperatingPointSeed::try_new(
            circuit.node_names_sorted(),
            circuit.branch_names_sorted(),
            vec![0.0; circuit.matrix_size()],
        )
        .unwrap();
        let point = engine
            .run_pss_operating_point_with_dc_seed_and_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_tstab_periods(0)
                    .with_points_per_period(512)
                    .with_tolerance(1e-9),
                &seed,
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("{capacitors}: {error}"));
        assert_eq!(point.shooting_state().len(), 1);
        let result = &point.analysis().result;
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let ratio = std::f64::consts::TAU;
        let amplitude = 1.0 / (1.0 + ratio * ratio).sqrt();
        for (&time, &voltage) in result.time.iter().zip(&result.waveforms[output].values) {
            let expected = amplitude * (std::f64::consts::TAU * F0 * time - ratio.atan()).sin();
            assert!(
                (voltage - expected).abs() < 0.002 * amplitude,
                "{capacitors}, t={time:e}: got {voltage:e}, expected {expected:e}"
            );
        }
    }
}

#[test]
fn charged_diode_pss_matches_ac_transient_and_rc_theory_under_grid_refinement() {
    for (cjo, explicit_c, reverse) in [(1e-9, 1e-12, false), (2e-9, 0.0, true)] {
        let bias = if reverse { 1.0 } else { -1.0 };
        let terminals = if reverse { "0 out" } else { "out 0" };
        let netlist = Netlist::parse(&format!(
            "audited diode PSS charge\nV1 in 0 SIN({bias} 0.01 1meg) AC 1\n\
             R1 in out 1k\nCkeep out 0 {explicit_c:e}\nD1 {terminals} dm\n\
             .model dm D(IS=1e-30 CJO={cjo:e} M=0 TT=0)\n.end\n"
        ))
        .unwrap();
        let engine = Engine::default();
        let tau = R * (cjo + explicit_c);
        let ratio = std::f64::consts::TAU * F0 * tau;
        let expected_amplitude = 0.01 / (1.0 + ratio * ratio).sqrt();
        let expected_phase = -ratio.atan().to_degrees();
        let mut previous_amplitude_error = f64::INFINITY;
        let mut previous_phase_error = f64::INFINITY;
        for points in [256, 512, 1024] {
            let point = engine
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(F0)
                        .with_points_per_period(points)
                        .with_tstab_periods(0)
                        .with_tolerance(1e-10),
                    &NoAbort,
                )
                .expect("charged diode PSS must converge");
            assert_eq!(
                point.shooting_state().len(),
                1,
                "parallel charge branches share one voltage state"
            );
            let result = &point.analysis().result;
            let node = |name: &str| {
                result
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap()
                    + 1
            };
            let input = &result.harmonics(node("in"), 1)[1];
            let output = &result.harmonics(node("out"), 1)[1];
            let amplitude_error = (output.magnitude / expected_amplitude - 1.0).abs();
            let phase = (output.phase - input.phase + 180.0).rem_euclid(360.0) - 180.0;
            let phase_error = (phase - expected_phase).abs();
            eprintln!(
                "CJO={cjo:e}, C={explicit_c:e}, N={points}: amplitude={:.12e}, relative error={amplitude_error:e}, phase error={phase_error:e} deg",
                output.magnitude
            );
            assert!(
                amplitude_error < previous_amplitude_error,
                "amplitude must converge under refinement"
            );
            assert!(
                phase_error < previous_phase_error,
                "phase must converge under refinement"
            );
            previous_amplitude_error = amplitude_error;
            previous_phase_error = phase_error;
            if points == 1024 {
                assert!(amplitude_error <= 0.002);
                assert!(phase_error <= 0.02);
                let expected_multiplier = (-(1.0 / F0) / tau).exp();
                assert_eq!(point.analysis().floquet_multipliers.len(), 1);
                assert!(
                    (point.analysis().floquet_multipliers[0].re / expected_multiplier - 1.0).abs()
                        < 0.001
                );
            }
        }
        let ac = engine.run_ac(&netlist, &[F0]).unwrap().remove(0);
        let out = ac
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((0.01 * ac.voltages[out].norm() / expected_amplitude - 1.0).abs() < 1e-6);
        let transient = engine.run_tran(&netlist, 20e-6, 1e-6 / 1024.0).unwrap();
        let output = transient.try_voltage_waveform_named("OUT").unwrap();
        for (&time, &voltage) in transient
            .time
            .iter()
            .zip(output)
            .filter(|(time, _)| **time >= 19e-6)
        {
            let expected = bias
                + expected_amplitude * (std::f64::consts::TAU * F0 * time - ratio.atan()).sin();
            assert!((voltage - expected).abs() <= 0.002 * expected_amplitude);
        }
    }
}

#[test]
fn prescribed_diode_voltage_is_a_constraint_not_a_spurious_shooting_state() {
    let netlist = Netlist::parse("prescribed diode charge\nV1 out 0 SIN(-1 0.01 1meg)\nD1 out 0 dm\n.model dm D(IS=1e-30 CJO=1n M=0)\n.end\n").unwrap();
    let point = Engine::default()
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(64)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .expect("a prescribed reactive voltage needs no free shooting coordinate");
    assert!(point.shooting_state().is_empty());
    let card = rspice_core::netlist::PstbCard {
        probe_instance: "Lmissing".to_owned(),
        max_harmonics: 4,
        num_multipliers: 1,
        stability_threshold: 1.0 + 1e-6,
        detect_subharmonics: true,
        eigenvalue_tolerance: 1e-10,
    };
    let error = Engine::default()
        .run_pstb_card_from_pss_with_abort(&netlist, &card, &point, &NoAbort)
        .unwrap_err()
        .to_string();
    assert!(error.contains("no independent dynamic coordinate"));
    assert!(!error.contains("legacy"));
    let result = &point.analysis().result;
    for (&time, &voltage) in result.time.iter().zip(&result.waveforms[0].values) {
        assert!((voltage - (-1.0 + 0.01 * (std::f64::consts::TAU * F0 * time).sin())).abs() < 1e-9);
    }
}

#[test]
fn nonlinear_diode_charge_pss_matches_settled_ngspice46() {
    let netlist = Netlist::parse(
        "nonlinear charged diode PSS oracle\nV1 in 0 SIN(0.7 0.5 1meg)\n\
         R1 in out 100\nCkeep out 0 1n\nD1 out 0 dm\n\
         .model dm D(IS=1e-14 N=1.6 RS=1 CJO=100p VJ=0.7 M=0.5 TT=5n)\n.end\n",
    )
    .unwrap();
    // Live ngspice 46, 2026-09-07: the identical deck with .tran 0.5n 20u.
    // Linear interpolation at 1/16-period intervals in the settled 19–20 us
    // cycle. This covers depletion, diffusion and an internal series-R node.
    let reference = [
        4.628_742_814_191_617e-1,
        6.037_671_134_394_52e-1,
        7.568_202_648_753_487e-1,
        8.993_142_649_246_983e-1,
        1.005_292_659_521_489,
        1.046_226_789_508_697,
        1.036_616_990_752_226,
        9.933_852_452_288_828e-1,
        9.097_133_113_610_002e-1,
        7.848_911_278_674_935e-1,
        6.368_145_176_295_715e-1,
        4.913_580_983_703_467e-1,
        3.744_209_501_360_043e-1,
        3.069_170_541_699_974e-1,
        3.005_752_587_035_279e-1,
        3.558_969_590_533_483e-1,
        4.628_740_361_311_73e-1,
    ];
    let mut previous_error = f64::INFINITY;
    for points in [256, 512, 1024] {
        let point = Engine::default()
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(points)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-9),
                &NoAbort,
            )
            .expect("nonlinear charged diode PSS converges");
        assert_eq!(
            point.shooting_state().len(),
            2,
            "the series-R junction voltage is an independent state"
        );
        let result = &point.analysis().result;
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap()
            + 1;
        let error = reference
            .iter()
            .enumerate()
            .map(|(index, &voltage)| {
                (result.voltage_at(output, index as f64 / 16.0 / F0) - voltage).abs()
            })
            .fold(0.0_f64, f64::max);
        eprintln!("nonlinear diode N={points}: maximum ngspice waveform error={error:e} V");
        assert!(
            error < previous_error,
            "nonlinear waveform must converge under refinement"
        );
        previous_error = error;
        if points == 1024 {
            assert!(error < 2e-4);
        }
    }
}

fn run_rc_pss() -> rspice_core::engine::PssAnalysisResult {
    let deck = format!(
        "\
* sine-driven rc
v1 in 0 sin(0 1 {F0})
r1 in out {R}
c1 out 0 {C}
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(F0)
        .with_tstab_periods(8)
        .with_tolerance(1e-7);
    engine.run_pss(&netlist, config).expect("PSS converges")
}

#[test]
fn public_pss_resolves_deck_rshunt_before_circuit_construction() {
    let netlist = Netlist::parse(
        "resolved PSS RSHUNT\n\
         I1 0 out SIN(0 1m 1meg)\n\
         C1 out 0 159.154943091895p\n\
         .OPTIONS RSHUNT=1k\n\
         .END\n",
    )
    .expect("PSS option deck parses");
    let result = Engine::default()
        .run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_harmonics(4)
                .with_points_per_period(32)
                .with_tstab_periods(0)
                .with_tolerance(1.0e-7),
        )
        .expect("resolved RSHUNT PSS converges");
    let output = result
        .result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("output node exists");
    let fundamental = result.result.harmonics(output + 1, 1)[1].magnitude;
    let expected = 1.0 / 2.0_f64.sqrt();
    assert!(
        (fundamental / expected - 1.0).abs() < 0.03,
        "the resolved 1 kOhm shunt and omega*C=1 mS must produce a 1/sqrt(2) V peak; got {fundamental} V"
    );
}

#[test]
fn multi_state_driven_pss_exercises_preconditioned_newton_krylov() {
    let mut deck = format!("* twelve-state driven PSS\nVdrive in 0 SIN(0 1 {F0})\n");
    for index in 1..=12 {
        deck.push_str(&format!(
            "R{index} in n{index} {}\nC{index} n{index} 0 {}\n",
            R * (1.0 + index as f64 * 0.03),
            C * (1.0 + index as f64 * 0.02)
        ));
    }
    deck.push_str(".end\n");
    let netlist = Netlist::parse(&deck).expect("multi-state deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(32)
                .with_max_iterations(30)
                .with_damping(0.5)
                .with_tolerance(1e-6),
        )
        .expect("matrix-free shooting converges");

    assert!(
        result.iterations >= 2,
        "damping must require a Krylov-era step"
    );
    assert!(
        result.final_residual < 1e-5,
        "residual={}",
        result.final_residual
    );
    assert_eq!(result.monodromy.len(), 12);
    assert!(result.monodromy.iter().all(|row| row.len() == 12));
}

#[test]
fn adaptive_stabilization_with_nonzero_tstab_completes_for_linear_rc_and_rl() {
    let period = 1.0 / F0;
    let inductance = R / (std::f64::consts::TAU * F0);
    let decks = [
        format!(
            "* adaptive PSS stabilization RC\n\
             V1 in 0 SIN(0 1 {F0})\n\
             R1 in out {R}\n\
             C1 out 0 {C}\n\
             .end\n"
        ),
        format!(
            "* adaptive PSS stabilization RL\n\
             V1 in 0 SIN(0 1 {F0})\n\
             R1 in out {R}\n\
             L1 out 0 {inductance}\n\
             .end\n"
        ),
    ];

    for deck in decks {
        let netlist = Netlist::parse(&deck).expect("linear stabilization deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_pss(
                &netlist,
                PssConfig::new(F0)
                    .with_tstab(8.0 * period)
                    .with_tolerance(1.0e-7),
            )
            .expect("adaptive nonzero-tstab traversal reaches the shooting solve");

        assert_eq!(
            result.result.time.last().copied(),
            Some(period),
            "the converged fixed-grid orbit retains its exact endpoint"
        );
        assert!(
            result.final_residual < 1.0e-4,
            "linear periodic orbit closes after stabilization: {}",
            result.final_residual
        );
    }
}

#[test]
fn pss_rejects_zero_max_iterations_as_invalid_config() {
    let deck = format!(
        "\
* sine-driven rc
v1 in 0 sin(0 1 {F0})
r1 in out {R}
c1 out 0 {C}
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let err = engine
        .run_pss(&netlist, PssConfig::new(F0).with_max_iterations(0))
        .expect_err("zero max_iterations must be rejected");

    match err {
        SimulationError::Circuit(message) => {
            assert_eq!(message, "Invalid PSS config: max_iterations must be > 0");
        }
        other => panic!("expected invalid PSS config error, got {other:?}"),
    }
}

#[test]
fn pss_rejects_invalid_public_numeric_config_as_invalid_config() {
    let deck = format!(
        "\
* sine-driven rc
v1 in 0 sin(0 1 {F0})
r1 in out {R}
c1 out 0 {C}
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let invalid_cases = vec![
        (
            "non-finite fundamental",
            {
                let mut config = PssConfig::new(f64::NAN);
                config.period_guess = 1e-9;
                config
            },
            "Invalid PSS config: fundamental_freq must be finite and >= 0",
        ),
        (
            "negative fundamental",
            {
                let mut config = PssConfig::new(-F0);
                config.period_guess = 1e-9;
                config
            },
            "Invalid PSS config: fundamental_freq must be finite and >= 0",
        ),
        (
            "negative tstab",
            PssConfig::new(F0).with_tstab(-1e-6),
            "Invalid PSS config: tstab must be finite and >= 0",
        ),
        (
            "zero tolerance",
            PssConfig::new(F0).with_tolerance(0.0),
            "Invalid PSS config: tolerance must be finite and > 0",
        ),
        (
            "non-finite abstol",
            {
                let mut config = PssConfig::new(F0);
                config.abstol = f64::INFINITY;
                config
            },
            "Invalid PSS config: abstol must be finite and > 0",
        ),
        (
            "invalid period guess",
            {
                let mut config = PssConfig::autonomous();
                config.period_guess = 0.0;
                config
            },
            "Invalid PSS config: period_guess must be finite and > 0",
        ),
        (
            "out of range damping",
            {
                let mut config = PssConfig::new(F0);
                config.damping_factor = 1.5;
                config
            },
            "Invalid PSS config: damping_factor must be finite and in [0.1, 1.0]",
        ),
        (
            "invalid period change",
            {
                let mut config = PssConfig::autonomous();
                config.max_period_change = f64::NAN;
                config
            },
            "Invalid PSS config: max_period_change must be finite and > 0",
        ),
        (
            "invalid grid density",
            {
                let mut config = PssConfig::new(F0);
                config.points_per_period = 0;
                config
            },
            "Invalid PSS config: points_per_period must be >= 16",
        ),
    ];

    for (case, config, expected) in invalid_cases {
        let err = match engine.run_pss(&netlist, config) {
            Ok(_) => panic!("{case} must be rejected"),
            Err(err) => err,
        };

        match err {
            SimulationError::Circuit(message) => {
                assert_eq!(message, expected, "{case}");
            }
            other => panic!("{case}: expected invalid PSS config error, got {other:?}"),
        }
    }
}

#[test]
fn driven_pss_ignores_autonomous_period_controls() {
    let deck = format!(
        "\
* sine-driven rc
v1 in 0 sin(0 1 {F0})
r1 in out {R}
c1 out 0 {C}
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let mut config = PssConfig::new(F0)
        .with_tstab_periods(8)
        .with_tolerance(1e-7);
    config.period_guess = 0.0;
    config.max_period_change = f64::NAN;

    let result = engine
        .run_pss(&netlist, config)
        .expect("driven PSS must ignore autonomous-only period controls");

    assert!(
        result.final_residual < 1e-4,
        "periodicity residual must be small, got {}",
        result.final_residual
    );
}

#[test]
fn rc_steady_state_matches_the_analytic_solution() {
    let result = run_rc_pss();

    // Periodicity itself: the converged orbit closes.
    assert!(
        result.final_residual < 1e-4,
        "periodicity residual must be small, got {}",
        result.final_residual
    );

    // Closed form: |H| = 1/sqrt(1 + (wRC)^2) with wRC = 1 -> amplitude
    // 1/sqrt(2) = 0.7071 V on the capacitor.
    let pss = &result.result;
    let out_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap_or_else(|| panic!("out missing from PSS waveforms: {:?}", pss.node_names));

    let amplitude = pss.waveforms[out_idx]
        .values
        .iter()
        .fold(0.0f64, |acc, v| acc.max(v.abs()));
    let expected = std::f64::consts::FRAC_1_SQRT_2;
    assert!(
        (amplitude - expected).abs() / expected < 0.02,
        "capacitor amplitude within 2% of 1/sqrt(2): got {amplitude}"
    );
}

/// A shooting period is evidence about the authored circuit, so the
/// integrator may not stabilize each time point with an implicit nodal
/// conductance.  In this deliberately high-impedance RC, a 1 pS numerical
/// shunt would double the physical conductance and suppress the exact
/// 0.847 V response to about 0.477 V.
#[test]
fn high_impedance_pss_preserves_the_authored_parallel_rc_response() {
    const CURRENT: f64 = 1.0e-12;
    const RESISTANCE: f64 = 1.0e12;
    const CAPACITANCE: f64 = 1.0e-19;

    let deck = format!(
        "\
* high-impedance current-driven rc
i1 0 out sin(0 {CURRENT} {F0})
r1 out 0 {RESISTANCE}
c1 out 0 {CAPACITANCE}
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("high-impedance deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_tstab_periods(8)
                .with_points_per_period(256)
                .with_tolerance(1.0e-8),
        )
        .expect("high-impedance PSS converges without an artificial shunt");

    let pss = &result.result;
    let out_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    let values = &pss.waveforms[out_idx].values;
    let amplitude = 0.5
        * (values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - values.iter().copied().fold(f64::INFINITY, f64::min));
    let omega_rc = std::f64::consts::TAU * F0 * RESISTANCE * CAPACITANCE;
    let expected = CURRENT * RESISTANCE / (1.0 + omega_rc * omega_rc).sqrt();
    assert!(
        (amplitude - expected).abs() <= 0.02 * expected,
        "physical high-impedance response must be retained: got {amplitude:.6e}, want {expected:.6e}"
    );

    assert_eq!(result.floquet_multipliers.len(), 1);
    let expected_multiplier = (-(1.0 / F0) / (RESISTANCE * CAPACITANCE)).exp();
    let multiplier = result.floquet_multipliers[0];
    assert!(
        (multiplier.re - expected_multiplier).abs() <= 0.05 * expected_multiplier,
        "physical RC decay must determine the period map: got {:.6e}, want {expected_multiplier:.6e}",
        multiplier.re
    );
    assert!(multiplier.im.abs() <= 1.0e-8);
}

#[test]
fn rc_floquet_multiplier_matches_exp_minus_t_over_rc() {
    let result = run_rc_pss();

    assert_eq!(
        result.result.floquet_multipliers, result.floquet_multipliers,
        "nested and outer PSS results must retain the same qualified spectrum"
    );
    assert_eq!(result.is_stable, result.result.is_stable());
    assert!(
        !result.result.period_detected,
        "forced/driven PSS periods are not auto-detected"
    );

    // One reactive state: the single Floquet multiplier of a linear RC is
    // exactly exp(-T/RC), independent of the drive.
    assert_eq!(
        result.floquet_multipliers.len(),
        1,
        "one reactive state -> one multiplier"
    );
    let mu = result.floquet_multipliers[0].norm();
    let expected = (-(1.0 / F0) / (R * C)).exp();
    // The fixed-grid period map is smooth in the initial state, so the
    // central-difference monodromy reaches real derivative accuracy: demand
    // 1% on exp(-T/RC) ~ 1.87e-3, which the adaptive-grid forward
    // difference could never deliver.
    assert!(
        (mu - expected).abs() < 0.01 * expected,
        "Floquet multiplier within 1% of exp(-T/RC): got {mu}, want {expected}"
    );
    assert!(
        result.floquet_multipliers[0].im.abs() < 1e-6,
        "RC multiplier is real"
    );
}

/// A square-wave-driven RC has a closed-form periodic steady state: with
/// a = exp(-T/(2RC)), the capacitor rides exponential segments between
/// V_min = a/(1+a) and V_max = 1/(1+a). Landing on the PULSE edges requires
/// the PSS integrator to honor source breakpoints; without them the orbit
/// smears by an LTE-sized step at every edge.
#[test]
fn pulse_driven_rc_matches_the_closed_form_steady_state() {
    // T = 1us, RC = T/2 -> a = exp(-1).
    let deck = "\
* square-wave rc
v1 in 0 pulse(0 1 0 1n 1n 0.499u 1u)
r1 in out 1k
c1 out 0 0.5n
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(1.0e6)
        .with_tstab_periods(8)
        .with_tolerance(1e-7);
    let result = engine.run_pss(&netlist, config).expect("PSS converges");

    let pss = &result.result;
    let out_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    let values = &pss.waveforms[out_idx].values;
    let v_max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let v_min = values.iter().cloned().fold(f64::INFINITY, f64::min);

    let a = (-1.0f64).exp();
    let expected_max = 1.0 / (1.0 + a);
    let expected_min = a / (1.0 + a);

    assert!(
        (v_max - expected_max).abs() < 0.015 * expected_max,
        "steady-state peak must be 1/(1+e^-1): got {v_max:.5}, want {expected_max:.5}"
    );
    assert!(
        (v_min - expected_min).abs() < 0.015 * expected_min,
        "steady-state trough must be e^-1/(1+e^-1): got {v_min:.5}, want {expected_min:.5}"
    );
}

/// Autonomous shooting: a weakly nonlinear LC negative-resistance oscillator
/// (van der Pol form, eps = g1*sqrt(L/C) = 0.05) has period
/// T = 2*pi*sqrt(LC)*(1 + eps^2/16 + ...), within 0.02% of 2*pi*sqrt(LC),
/// and a describing-function amplitude sqrt(4*g1/(3*g3)). The period must
/// come out of the (n+1)-unknown Newton, not the coarse detector, and the
/// Floquet spectrum must carry the structural unity multiplier of an
/// autonomous orbit.
#[test]
fn lc_oscillator_period_solves_to_the_analytic_value() {
    // L = C = 1u -> sqrt(LC) = 1us, T0 = 6.28319us, sqrt(L/C) = 1 ohm.
    let deck = "* negative-resistance lc oscillator
l1 osc 0 1u
c1 osc 0 1u
b1 osc 0 i=-0.05*v(osc)+0.025*v(osc)*v(osc)*v(osc)
i1 0 osc pulse(0 1 10u 10n 10n 1u 1)
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::autonomous()
        .with_period_guess(6.3e-6)
        .with_tstab_periods(30)
        .with_tolerance(1e-6)
        .with_max_iterations(60);
    let result = engine.run_pss(&netlist, config).expect("PSS converges");
    assert!(
        result.result.period_detected,
        "autonomous PSS period provenance must be retained"
    );
    assert_eq!(result.is_stable, result.result.is_stable());

    let t0 = 2.0 * std::f64::consts::PI * 1.0e-6;
    let eps: f64 = 0.05;
    let t_expected = t0 * (1.0 + eps * eps / 16.0);
    assert!(
        (result.period - t_expected).abs() < 1e-3 * t_expected,
        "oscillator period must solve to the van der Pol value: got {:.6e}, want {:.6e}",
        result.period,
        t_expected
    );

    // Structural unity Floquet multiplier of the autonomous orbit.
    let unity_error = result
        .floquet_multipliers
        .iter()
        .map(|m| (m - num_complex::Complex64::new(1.0, 0.0)).norm())
        .fold(f64::INFINITY, f64::min);
    assert!(
        unity_error < 0.05,
        "autonomous orbit must carry a unity Floquet multiplier; nearest is {unity_error:.3} away; all: {:?}",
        result.floquet_multipliers
    );

    // Describing-function amplitude sqrt(4*0.05/(3*0.025)) = 1.633 V.
    let pss = &result.result;
    let osc_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("osc"))
        .expect("osc node present");
    let amplitude = pss.waveforms[osc_idx]
        .values
        .iter()
        .fold(0.0f64, |acc, v| acc.max(v.abs()));
    let a_expected = (4.0f64 * 0.05 / (3.0 * 0.025)).sqrt();
    assert!(
        (amplitude - a_expected).abs() < 0.04 * a_expected,
        "limit-cycle amplitude must match the describing function: got {amplitude:.4}, want {a_expected:.4}"
    );
}

#[test]
fn three_way_inductor_junction_uses_two_independent_currents() {
    use num_complex::Complex64;
    for coupling in [0.0, 0.4] {
        let netlist = Netlist::parse(&format!("inductive KCL constraint\nV1 in 0 SIN(0 1 1meg)\nR1 in a 1k\nL1 a mid 40u\nL2 mid b 60u\nL3 mid c 120u\nR2 b 0 1k\nR3 c 0 2k\nK1 L2 L3 {coupling}\n.end\n")).unwrap();
        let engine = Engine::default();
        let point = engine
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_tstab_periods(0)
                    .with_points_per_period(512)
                    .with_tolerance(1e-9),
                &NoAbort,
            )
            .expect("a three-way KCL junction has two free winding currents");
        assert_eq!(point.shooting_state_basis(), ["L:L1", "L:L2"]);
        let omega = std::f64::consts::TAU * F0;
        let mutual = coupling * (60e-6_f64 * 120e-6).sqrt();
        // KVL after eliminating I1 = I2 + I3 gives this exact two-loop matrix.
        let z22 = Complex64::new(2000.0, omega * 100e-6);
        let z33 = Complex64::new(3000.0, omega * 160e-6);
        let z23 = Complex64::new(1000.0, omega * (40e-6 + mutual));
        let determinant = z22 * z33 - z23 * z23;
        let i2 = (z33 - z23) / determinant;
        let i3 = (z22 - z23) / determinant;
        for (name, transfer) in [
            ("a", 1.0 - 1000.0 * (i2 + i3)),
            (
                "mid",
                Complex64::new(1000.0, omega * 60e-6) * i2
                    + Complex64::new(0.0, omega * mutual) * i3,
            ),
            ("b", 1000.0 * i2),
            ("c", 2000.0 * i3),
        ] {
            let result = &point.analysis().result;
            let node = result
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .unwrap();
            for (&time, &voltage) in result.time.iter().zip(&result.waveforms[node].values) {
                let expected =
                    transfer.re * (omega * time).sin() + transfer.im * (omega * time).cos();
                assert!(
                    (voltage - expected).abs() < 0.001 * transfer.norm(),
                    "k={coupling}, {name}, t={time:e}: {voltage:e} versus {expected:e}"
                );
            }
        }
        let card = rspice_core::netlist::PstbCard {
            probe_instance: "L3".to_owned(),
            max_harmonics: 4,
            num_multipliers: 2,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        };
        let stability = engine
            .run_pstb_card_from_pss_with_abort(&netlist, &card, &point, &NoAbort)
            .unwrap();
        assert_eq!(stability.probe_state_index, None);
        assert_eq!(stability.probe_state_projection, [(0, 1.0), (1, -1.0)]);
        for (mode, &participation) in stability
            .result
            .multipliers
            .iter()
            .zip(&stability.probe_participation)
        {
            let vector = mode.eigenvector.as_ref().unwrap();
            let expected = (vector[0] - vector[1]).norm()
                / 2.0_f64.sqrt()
                / vector[0].norm().hypot(vector[1].norm());
            assert!((participation - expected).abs() < 1e-14);
        }
    }
}

#[test]
fn coupled_series_windings_preserve_flux_and_internal_resistive_voltage_drops() {
    use num_complex::Complex64;
    for (second, orientation) in [("right 0", 1.0), ("0 right", -1.0)] {
        for coupling in [0.0, 0.9] {
            let netlist = Netlist::parse(&format!("series flux and internal resistor\nV1 in 0 SIN(0 1 1meg)\nR1 in out 100\nL1 out left 40u\nRmid left right 50\nL2 {second} 60u\nK1 L1 L2 {coupling}\n.end\n")).unwrap();
            let engine = Engine::default();
            let config = PssConfig::new(F0)
                .with_tstab_periods(0)
                .with_points_per_period(1024)
                .with_tolerance(1e-11);
            let point = engine
                .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
                .unwrap();
            assert_eq!(point.shooting_state().len(), 1);
            let mutual = orientation * coupling * (40e-6_f64 * 60e-6).sqrt();
            let total_l = 100e-6 + 2.0 * mutual;
            let omega = std::f64::consts::TAU * F0;
            let current = 1.0 / Complex64::new(150.0, omega * total_l);
            let right = Complex64::new(0.0, omega * (60e-6 + mutual)) * current;
            let transfers = [
                ("out", 1.0 - 100.0 * current),
                ("left", right + 50.0 * current),
                ("right", right),
            ];
            for (name, transfer) in transfers {
                let result = &point.analysis().result;
                let node = result
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap();
                for (&time, &voltage) in result.time.iter().zip(&result.waveforms[node].values) {
                    let expected =
                        transfer.re * (omega * time).sin() + transfer.im * (omega * time).cos();
                    assert!(
                        (voltage - expected).abs() < 0.0001 * transfer.norm(),
                        "{name}, k={coupling}, sign={orientation}, t={time:e}"
                    );
                }
            }
            let (_, state) = engine
                .run_pss_with_continuation_state(&netlist, config)
                .unwrap();
            let (continued, _) = engine
                .run_tran_from_pss_state(&netlist, &state, 1e-6, 1e-6 / 1024.0)
                .unwrap();
            for (name, transfer) in transfers {
                let node = continued
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap();
                for (&time, &voltage) in continued.time.iter().zip(&continued.voltages[node]) {
                    let expected =
                        transfer.re * (omega * time).sin() + transfer.im * (omega * time).cos();
                    assert!((voltage - expected).abs() < 0.0001 * transfer.norm());
                }
            }
        }
    }
}
