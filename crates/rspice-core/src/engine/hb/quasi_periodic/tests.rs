//! Netlist-to-retained-state checks with independent circuit/source oracles.
use super::*;
use crate::abort_signal::CountingAbort;
use crate::analysis::quasi_periodic::QuasiPeriodicSampling;
use crate::config::SimulationConfig;

fn config(harmonics: usize) -> QpssConfig {
    let mut config = QpssConfig::new(
        vec![1e3, std::f64::consts::SQRT_2 * 1e3],
        vec![harmonics; 2],
    );
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![32; 2]);
    config.solver.relative_tolerance = 1e-8;
    config.solver.current_absolute_tolerance = 1e-13;
    config.solver.voltage_absolute_tolerance = 1e-11;
    config
}

fn coefficient(point: &QpssOperatingPoint, node: &str, tuple: &[i32]) -> Complex64 {
    let grid = QuasiPeriodicGrid::new_with_abort(
        point.config().grid.clone(),
        &crate::ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    let row = point
        .node_names()
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap();
    point.spectra()[row][grid.index_of(tuple).unwrap()]
}

fn close(actual: Complex64, expected: Complex64, tolerance: Value) {
    assert!(
        (actual - expected).norm() <= tolerance,
        "{actual:?} != {expected:?}, tolerance {tolerance:e}"
    );
}

#[test]
fn qpss_engine_executes_two_tone_rlc_with_exact_branches_and_dc_initialization() {
    let netlist = Netlist::parse("independent tones\nV1 a 0 SIN(0 .4 1k 0 0 30)\nV2 b 0 SIN(0 .2 1414.213562373095 0 0 -40)\nR1 a out 1k\nR2 b out 2k\nR3 out 0 2k\nC1 out 0 100n\nL1 out 0 5m\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let direct = engine.run_qpss(&netlist, config(1)).unwrap();
    let mut seeded_config = config(1);
    seeded_config.initial_state = QpssInitialState::DcOperatingPoint;
    let seeded = engine.run_qpss(&netlist, seeded_config).unwrap();
    for (tuple, frequency, amplitude, phase, resistance) in [
        ([1, 0], 1e3, 0.4, 30.0_f64.to_radians(), 1e3),
        (
            [0, 1],
            std::f64::consts::SQRT_2 * 1e3,
            0.2,
            -40.0_f64.to_radians(),
            2e3,
        ),
    ] {
        let omega = std::f64::consts::TAU * frequency;
        let input = Complex64::from_polar(amplitude / 2.0, phase - std::f64::consts::FRAC_PI_2);
        let expected =
            input / resistance / Complex64::new(0.002, omega * 100e-9 - 1.0 / (omega * 5e-3));
        close(coefficient(&direct, "out", &tuple), expected, 1e-10);
        close(coefficient(&seeded, "out", &tuple), expected, 1e-10);
    }
    close(
        coefficient(&direct, "out", &[1, -1]),
        Complex64::ZERO,
        1e-12,
    );
    assert_eq!(direct.branch_names().len(), 3);
    assert!(direct.normalized_residual() <= 1.0);
    engine
        .validate_qpss_operating_point_with_abort(&netlist, &seeded, &NoAbort)
        .unwrap();
}

#[test]
fn qpss_engine_ac_tone_bindings_and_retained_payload_are_authenticated() {
    let deck = "explicit tone\nVdrive out 0 DC 1 AC 2 90\nIprobe out 0 AC 100\nR1 out 0 1k\n.end\n";
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let mut request = config(1);
    request.source_tones.push(QpssSourceTone {
        source: "Vdrive".into(),
        tone: 1,
    });
    let point = engine.run_qpss(&netlist, request.clone()).unwrap();
    close(
        coefficient(&point, "out", &[0, 0]),
        Complex64::new(1.0, 0.0),
        1e-12,
    );
    close(
        coefficient(&point, "out", &[0, 1]),
        Complex64::new(0.0, 1.0),
        1e-12,
    );
    close(coefficient(&point, "out", &[1, 0]), Complex64::ZERO, 1e-12);
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let branch = point.node_names().len()
        + point
            .branch_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("Vdrive"))
            .unwrap();
    close(
        point.spectra()[branch][grid.index_of(&[0, 1]).unwrap()],
        Complex64::new(0.0, -1e-3),
        1e-13,
    );
    let json = serde_json::to_string(&point).unwrap();
    let restored: QpssOperatingPoint = serde_json::from_str(&json).unwrap();
    assert_eq!(point, restored);
    engine
        .validate_qpss_operating_point_with_abort(&netlist, &restored, &NoAbort)
        .unwrap();
    let mut altered: serde_json::Value = serde_json::from_str(&json).unwrap();
    altered["spectra"][0][0][0] = serde_json::json!(0.125);
    let altered: QpssOperatingPoint = serde_json::from_value(altered).unwrap();
    assert!(
        engine
            .validate_qpss_operating_point_with_abort(&netlist, &altered, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("identity")
    );
    let changed = Netlist::parse(&deck.replace("DC 1", "DC 2")).unwrap();
    assert!(
        engine
            .validate_qpss_operating_point_with_abort(&changed, &point, &NoAbort)
            .is_err()
    );
    let mut oversized: serde_json::Value = serde_json::from_str(&json).unwrap();
    oversized["spectra"][0]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!([0.0, 0.0]));
    let oversized: QpssOperatingPoint = serde_json::from_value(oversized).unwrap();
    assert!(
        engine
            .validate_qpss_operating_point_with_abort(&netlist, &oversized, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("tone lattice")
    );
    request.source_tones[0].source = "missing".into();
    assert!(
        engine
            .run_qpss(&netlist, request)
            .unwrap_err()
            .to_string()
            .contains("does not exist")
    );
}

#[test]
fn qpss_engine_pulse_and_repeating_pwl_use_continuous_fourier_coefficients() {
    let deck = "narrow pulse and triangle\nV1 pulse 0 PULSE(0 1 10u 0 0 .2u 1m)\nR1 pulse 0 1k\nV2 triangle 0 PWL(0 0 0.3535533905932738m 1 0.7071067811865476m 0) R=0 TD=20u\nR2 triangle 0 1k\n.end\n";
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let point = engine.run_qpss(&netlist, config(2)).unwrap();
    close(
        coefficient(&point, "pulse", &[0, 0]),
        Complex64::new(0.0002, 0.0),
        1e-13,
    );
    for order in [1, 2] {
        let omega = std::f64::consts::TAU * 1e3 * order as Value;
        let half = omega * 0.2e-6 * 0.5;
        let expected = Complex64::from_polar(0.0002 * half.sin() / half, -omega * 10.1e-6);
        close(coefficient(&point, "pulse", &[order, 0]), expected, 1e-12);
    }
    close(
        coefficient(&point, "triangle", &[0, 0]),
        Complex64::new(0.5, 0.0),
        1e-12,
    );
    let expected = Complex64::from_polar(
        -2.0 / std::f64::consts::PI.powi(2),
        -std::f64::consts::TAU * std::f64::consts::SQRT_2 * 1e3 * 20e-6,
    );
    close(coefficient(&point, "triangle", &[0, 1]), expected, 1e-12);
    close(
        coefficient(&point, "triangle", &[0, 2]),
        Complex64::ZERO,
        1e-12,
    );
}

#[test]
fn qpss_engine_modulated_sources_keep_independent_carrier_and_modulation_phases() {
    let deck = "modulation\nV1 am 0 AM(.3 .7 .2 1414.213562373095 1k 20u 50 30)\nR1 am 0 1k\nV2 fm 0 SFFM(.1 .4 1k .1 1414.213562373095 20u 50 30)\nR2 fm 0 1k\n.end\n";
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let point = engine.run_qpss(&netlist, config(2)).unwrap();
    let pc = 30.0_f64.to_radians() - std::f64::consts::TAU * 1e3 * 20e-6;
    let pm = 50.0_f64.to_radians() - std::f64::consts::TAU * std::f64::consts::SQRT_2 * 1e3 * 20e-6;
    close(
        coefficient(&point, "am", &[1, 0]),
        Complex64::from_polar(0.35, pc - std::f64::consts::FRAC_PI_2),
        1e-12,
    );
    close(
        coefficient(&point, "am", &[1, -1]),
        Complex64::from_polar(0.05, pc - pm),
        1e-12,
    );
    close(
        coefficient(&point, "am", &[1, 1]),
        Complex64::from_polar(-0.05, pc + pm),
        1e-12,
    );
    for tuple in [[1, 0], [1, 1], [1, -1], [1, 2]] {
        let mut expected = Complex64::ZERO;
        for a in 0..64 {
            for b in 0..64 {
                let p = std::f64::consts::TAU * a as Value / 64.0;
                let q = std::f64::consts::TAU * b as Value / 64.0;
                let value = 0.1 + 0.4 * (p + pc + 0.1 * (q + pm).sin()).sin();
                expected += Complex64::from_polar(
                    value / 4096.0,
                    -(tuple[0] as Value * p + tuple[1] as Value * q),
                );
            }
        }
        close(coefficient(&point, "fm", &tuple), expected, 1e-12);
    }
}

#[test]
fn qpss_engine_native_bjt_uses_physical_mixing_and_complete_branch_state() {
    let netlist = Netlist::parse("native BJT\nVc collector 0 2\nVb1 base1 0 SIN(.6 .01 1k)\nVb2 base base1 SIN(0 .008 1414.213562373095)\nQ1 collector base 0 NMOD\n.model NMOD NPN(IS=1e-15 BF=100 TF=1n CJE=1p CJC=.5p)\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let mut request = config(1);
    request.initial_state = QpssInitialState::DcOperatingPoint;
    let point = engine.run_qpss(&netlist, request).unwrap();
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let row = point.node_names().len()
        + point
            .branch_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("Vc"))
            .unwrap();
    assert!(point.spectra()[row][grid.index_of(&[1, 1]).unwrap()].norm() > 1e-9);
    assert!(point.spectra()[row][grid.dc_index()].re < -1e-6);
    assert_eq!(
        point.spectra().len(),
        point.node_names().len() + point.branch_names().len()
    );
}

#[test]
fn qpss_engine_checks_resources_cancellation_and_nonstationary_sources_before_publication() {
    let netlist = Netlist::parse("bounded\nV1 out 0 SIN(0 .1 1k)\nR1 out 0 1k\n.end\n").unwrap();
    let mut simulation = SimulationConfig::default();
    simulation.resource_limits.max_matrix_unknowns = 17;
    let limited = Engine::new(simulation);
    assert!(matches!(
        limited.run_qpss(&netlist, config(1)),
        Err(SimulationError::ResourceLimit(_))
    ));
    let engine = Engine::new(SimulationConfig::default());
    let abort = CountingAbort::new(40);
    assert!(matches!(
        engine.run_qpss_with_abort(&netlist, config(1), &abort),
        Err(SimulationError::Aborted)
    ));
    assert_eq!(abort.polls_after_abort(), 0);
    let damped = Netlist::parse("damped\nV1 out 0 SIN(0 .1 1k 0 1)\nR1 out 0 1k\n.end\n").unwrap();
    assert!(
        engine
            .run_qpss(&damped, config(1))
            .unwrap_err()
            .to_string()
            .contains("damped SIN")
    );
    let unknown_clock =
        Netlist::parse("unknown clock\nV1 out 0 SIN(0 .1 17k)\nR1 out 0 1k\n.end\n").unwrap();
    assert!(
        engine
            .run_qpss(&unknown_clock, config(1))
            .unwrap_err()
            .to_string()
            .contains("absent")
    );
}
