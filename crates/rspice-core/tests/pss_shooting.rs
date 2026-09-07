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
        assert_eq!(stability.probe_state_index, 0);
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
