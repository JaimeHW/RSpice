//! Qualification of the real integration body before widening public PTF admission.
use super::*;
use crate::SimulationConfig;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

struct Progress {
    began: std::time::Instant,
    next_report: AtomicU64,
    accepted_time: AtomicU64,
    points: AtomicUsize,
    initial: std::sync::Mutex<Vec<(Vec<Value>, Vec<Value>)>>,
}

impl AbortSignal for Progress {
    fn is_aborted(&self) -> bool {
        let elapsed = self.began.elapsed().as_millis() as u64;
        if elapsed >= self.next_report.load(Ordering::Relaxed) {
            self.next_report.store(elapsed + 5000, Ordering::Relaxed);
            eprintln!(
                "physical dispatch progress: wall_ms={elapsed}, accepted_time={:.17e}, points={}",
                f64::from_bits(self.accepted_time.load(Ordering::Relaxed)),
                self.points.load(Ordering::Relaxed)
            );
        }
        false
    }

    fn observe_transient_sample(&self, sample: crate::abort_signal::TransientSample<'_>) {
        if sample.time.len() == 1 {
            self.initial.lock().unwrap().push((
                sample
                    .node_voltages
                    .iter()
                    .map(|v| v.first().copied().unwrap_or(Value::NAN))
                    .collect(),
                sample
                    .branch_currents
                    .iter()
                    .map(|v| v.first().copied().unwrap_or(Value::NAN))
                    .collect(),
            ));
        }
        self.accepted_time.store(
            sample.time.last().copied().unwrap_or(0.0).to_bits(),
            Ordering::Relaxed,
        );
        self.points.store(sample.time.len(), Ordering::Relaxed);
    }
}

fn run(text: &str, stop: Value, max_step: Value) -> TransientResult {
    run_with_checkpoint(text, stop, max_step, None, &[]).0
}

fn run_with_checkpoint(
    text: &str,
    stop: Value,
    max_step: Value,
    resume: Option<&TransientCheckpoint>,
    scheduled: &[Value],
) -> (TransientResult, Vec<ScheduledTransientCheckpoint>) {
    run_with_configuration(
        text,
        stop,
        max_step,
        resume,
        scheduled,
        SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce),
    )
}

fn run_with_configuration(
    text: &str,
    stop: Value,
    max_step: Value,
    resume: Option<&TransientCheckpoint>,
    scheduled: &[Value],
    config: SimulationConfig,
) -> (TransientResult, Vec<ScheduledTransientCheckpoint>) {
    let progress = Progress {
        began: std::time::Instant::now(),
        next_report: AtomicU64::new(5000),
        accepted_time: AtomicU64::new(0),
        points: AtomicUsize::new(0),
        initial: std::sync::Mutex::new(Vec::new()),
    };
    let scope = crate::abort_signal::ModelRunSignal::if_needed(&progress);
    let abort: &dyn AbortSignal = scope.as_ref().map_or(&progress as &dyn AbortSignal, |s| s);
    let deck = Netlist::parse_with_options(
        text,
        crate::netlist::NetlistParseOptions {
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let base = Engine::new(config);
    let engine = base.resolved_for_netlist(&deck);
    let circuit = engine
        .build_transient_circuit_with_abort(&deck, stop, abort)
        .unwrap();
    assert!(
        circuit
            .bjts
            .devices
            .iter()
            .any(|bjt| bjt.legacy_excess_phase_delay() > 0.0)
    );
    let continuation = resume.and_then(|checkpoint| {
        checkpoint
            .validate_for_with_config(&deck, &engine.config)
            .unwrap();
        checkpoint.validate_recorded_integration_max_step().unwrap();
        checkpoint.validated_integration_continuation().unwrap()
    });
    let began = std::time::Instant::now();
    let outcome = engine.run_tran_admitted(
        &deck,
        &deck,
        TransientRunWindow {
            tstop: stop,
            max_step,
            startup_mode: TransientStartupMode::OperatingPoint,
        },
        abort,
        TransientResumePlan {
            resume,
            resume_validation: ResumeValidation::ExactNetlist,
            final_checkpoint_retention: FinalCheckpointRetention::Discarded,
            scheduled_checkpoint_times: scheduled,
        },
        PreparedTransientCircuit {
            circuit,
            modified_trapezoidal_coefficients: CompanionCoefficients::trapezoidal_with_xmu(0.5)
                .unwrap(),
            resume_continuation: continuation,
        },
    );
    eprintln!(
        "physical dispatch: elapsed={:?}, result={:?}",
        began.elapsed(),
        outcome
            .as_ref()
            .map(|(r, _, _)| (r.time.len(), r.time.last()))
    );
    let (result, _, checkpoints) = outcome.unwrap();
    let initial = progress.initial.lock().unwrap();
    assert_eq!(
        initial.len(),
        1,
        "one settled startup sample must be published"
    );
    for (recorded, observed) in [&result.voltages, &result.branch_currents]
        .into_iter()
        .zip([&initial[0].0, &initial[0].1])
    {
        assert_eq!(recorded.len(), observed.len());
        for (trace, &observed) in recorded.iter().zip(observed) {
            assert_eq!(
                trace.first().copied().unwrap_or(Value::NAN).to_bits(),
                observed.to_bits(),
                "live startup differs from retained waveform"
            );
        }
    }
    assert_eq!(result.time.last(), Some(&stop));
    assert!(result.time.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(
        result
            .voltages
            .iter()
            .chain(&result.branch_currents)
            .flatten()
            .all(|v| v.is_finite())
    );
    (result, checkpoints)
}

#[test]
fn physical_dispatch_source_corner_runs_actual_transient_body() {
    let result = run(
        "Physical GP source corner\nVb b 0 .6\nVc c 0 PWL(0 1 1n 2 3n 2)\nQ1 c b 0 qm\nC1 c 0 1p\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.tran .1n 3n\n.end\n",
        3e-9,
        1e-10,
    );
    assert!(result.time.contains(&1e-9));
    let voltage = result.try_voltage_waveform_named("c").unwrap();
    for (&t, &v) in result.time.iter().zip(voltage) {
        assert!((v - (1.0 + (t / 1e-9).min(1.0))).abs() < 1e-10);
    }
}

#[test]
fn physical_dispatch_bug805_full_original_alias_1() {
    run_original(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc1.cir"
        )),
        80e-6,
        1e-7,
    );
}

#[test]
fn physical_dispatch_bug805_full_original_alias_2() {
    run_original(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc2.cir"
        )),
        80e-6,
        1e-7,
    );
}

#[test]
fn physical_dispatch_bug805_full_original_alias_3() {
    run_original(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc3.cir"
        )),
        80e-6,
        1e-7,
    );
}

#[test]
fn physical_dispatch_source_jump_keeps_finite_currents_and_outgoing_startup() {
    let result = run(
        "Physical source jump and startup\nVc c 0 DC 0 PWL(0 1 1n 1 1n 2 3n 2)\nR1 c 0 1k\nC1 c 0 1p\nVb b 0 .6\nQ1 0 b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30)\n.tran .1n 3n\n.print tran v(c) i(Vc)\n.end\n",
        3e-9,
        1e-10,
    );
    let voltages = result.try_voltage_waveform_named("c").unwrap();
    let source = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("Vc"))
        .unwrap();
    let currents = &result.branch_currents[source];
    assert!(result.time.contains(&1e-9));
    for ((&time, &voltage), &current) in result.time.iter().zip(voltages).zip(currents) {
        let expected = if time < 1e-9 { 1.0 } else { 2.0 };
        assert!(
            (voltage - expected).abs() < 1e-10,
            "voltage at {time:e}: {voltage:e}"
        );
        assert!(
            (current + expected / 1000.0).abs() < 1e-10,
            "finite source current at {time:e}: {current:e}"
        );
    }
}

#[test]
fn physical_dispatch_checkpoint_event_preserves_exact_suffix() {
    let text = "Physical GP restart\nVb b 0 .6\nVc c 0 PWL(0 1 1n 2 3n 2)\nQ1 c b 0 qm\nC1 c 0 1p\n.model qm NPN(IS=1e-16 TF=1n PTF=30 VAF=10 CJE=1p CJC=.2p)\n.tran .1n 3n\n.print tran v(c) i(Vc) i(Q1)\n.end\n";
    let delay = 1e-9 * (30.0_f64 * std::f64::consts::PI / 180.0);
    let (baseline, scheduled) = run_with_checkpoint(text, 3e-9, 1e-10, None, &[delay]);
    assert_eq!(scheduled.len(), 1);
    let captured = &scheduled[0].checkpoint;
    assert_eq!(captured.time.to_bits(), delay.to_bits());
    assert!(
        captured
            .accepted_junction_transient_history()
            .bjt_history
            .phase_outgoing_slopes[0]
            .is_some()
    );
    let seam = baseline
        .time
        .iter()
        .position(|t| t.to_bits() == captured.time.to_bits())
        .unwrap();
    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let restored =
            TransientCheckpoint::from_bytes(&captured.to_bytes(encoding).unwrap()).unwrap();
        let (resumed, _) = run_with_checkpoint(text, 3e-9, 1e-10, Some(&restored), &[]);
        assert_eq!(
            resumed.time.len(),
            baseline.time.len() - seam,
            "accepted grid length for {encoding:?}"
        );
        for (index, (&actual, &expected)) in
            resumed.time.iter().zip(&baseline.time[seam..]).enumerate()
        {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "time at suffix {index}, {encoding:?}"
            );
        }
        for (kind, actual_traces, expected_traces) in [
            ("voltage", &resumed.voltages, &baseline.voltages),
            (
                "current",
                &resumed.branch_currents,
                &baseline.branch_currents,
            ),
        ] {
            assert_eq!(actual_traces.len(), expected_traces.len());
            for (trace, (actual, expected)) in actual_traces.iter().zip(expected_traces).enumerate()
            {
                if expected.is_empty() {
                    assert!(actual.is_empty(), "absent {kind} trace {trace}");
                    continue;
                }
                assert_eq!(actual.len(), expected.len() - seam);
                for (index, (&actual, &expected)) in
                    actual.iter().zip(&expected[seam..]).enumerate()
                {
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "{kind} trace {trace} at suffix {index}: actual={actual:e}, expected={expected:e}, {encoding:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn physical_dispatch_authored_shunt_is_in_finite_event_current() {
    let result = run(
        "Physical shunt\nVc c 0 DC 0 PWL(0 1 1n 1 1n 2 3n 2)\nR1 c 0 1k\nC1 c 0 1p\nVb b 0 .6\nQ1 0 b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30)\n.options rshunt=1k\n.tran .1n 3n\n.print tran v(c) i(Vc)\n.end\n",
        3e-9,
        1e-10,
    );
    let source = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("Vc"))
        .unwrap();
    for (&time, &current) in result.time.iter().zip(&result.branch_currents[source]) {
        let expected = if time < 1e-9 { -0.002 } else { -0.004 };
        assert!(
            (current - expected).abs() < 1e-10,
            "finite shunted source current at {time:e}: {current:e}"
        );
    }
}

#[test]
fn physical_dispatch_nearby_event_trains_preserve_controller_proposal() {
    // Two distinct event trains separated by 3.6 ps. Clipping a proposal to
    // that short gap must not become a permanent new controller timestep.
    let result = run(
        "Nearby physical event trains\nVc c 0 PWL(0 1 .52n 1 .52n 2 50n 2)\nR1 c 0 1k\nC1 c 0 1p\nVb b 0 .6\nQ1 0 b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30)\n.tran 1n 50n\n.print tran v(c) i(Vc)\n.end\n",
        50e-9,
        1e-9,
    );
    let jump = 0.52 * 1e-9; // Preserve the authored suffix multiplication.
    assert!(result.time.contains(&jump));
    let voltage = result.try_voltage_waveform_named("c").unwrap();
    for (&time, &actual) in result.time.iter().zip(voltage) {
        let expected = if time < jump { 1.0 } else { 2.0 };
        assert!((actual - expected).abs() < 1e-10);
    }
    assert!(
        result.time.len() < 500,
        "nearby clocks manufactured {} integration points",
        result.time.len()
    );
}

#[test]
fn physical_dispatch_collector_matches_analytic_delayed_exponential() {
    let result = run(
        "Analytic GP delay\nVb b 0 SIN(.7 1u 1G)\nVc c 0 2\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 BR=1 TF=.25n PTF=90 TNOM=27)\n.options gmin=0 temp=27\n.tran 1p 6n\n.print tran v(b) i(Vc)\n.end\n",
        6e-9,
        1e-12,
    );
    let source = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("Vc"))
        .unwrap();
    let vt = crate::constants::XYCE_K_BOLTZMANN * 300.15 / crate::constants::XYCE_Q_ELECTRON;
    let delay = 0.25e-9 * (90.0 * std::f64::consts::PI / 180.0);
    let current = |time: Value| {
        let base = 0.7 + 1e-6 * (std::f64::consts::TAU * 1e9 * time.max(0.0)).sin();
        // No Early effect, high injection, series resistance or collector
        // storage: IC = delayed IS*expm1(VBE/VT) plus two reverse leakages.
        1e-16 * (base / vt).exp_m1() + 2e-16
    };
    let mut max_error = 0.0_f64;
    let mut zero_delay_error = 0.0_f64;
    for (&time, &actual) in result.time.iter().zip(&result.branch_currents[source]) {
        max_error = max_error.max((-actual - current(time - delay)).abs());
        zero_delay_error = zero_delay_error.max((-actual - current(time)).abs());
    }
    eprintln!("analytic GP delay: max_error={max_error:e}, zero_delay_error={zero_delay_error:e}");
    assert!(
        max_error < 1e-12,
        "delayed exponential error: {max_error:e}"
    );
    assert!(
        zero_delay_error > 1e-9,
        "oracle must distinguish omitted delay"
    );
}

#[test]
fn physical_dispatch_native_nodal_floor_is_in_finite_event_current() {
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 1e-3;
    let (result, _) = run_with_configuration(
        "Physical native floor\nVc c 0 DC 0 PWL(0 1 1n 1 1n 2 3n 2)\nR1 c 0 1k\nC1 c 0 1p\nVb b 0 .6\nQ1 0 b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30)\n.options gmin=1m\n.tran .1n 3n\n.print tran v(c) i(Vc)\n.end\n",
        3e-9,
        1e-10,
        None,
        &[],
        config,
    );
    let source = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("Vc"))
        .unwrap();
    for (&time, &current) in result.time.iter().zip(&result.branch_currents[source]) {
        let expected = if time < 1e-9 { -0.002 } else { -0.004 };
        assert!(
            (current - expected).abs() < 1e-10,
            "native finite source current at {time:e}: {current:e}"
        );
    }
}

fn run_original(text: &str, stop: Value, max_step: Value) {
    let (baseline, scheduled) = run_with_checkpoint(text, stop, max_step, None, &[40e-6]);
    assert_eq!(scheduled.len(), 1);
    let checkpoint = &scheduled[0].checkpoint;
    let seam = baseline
        .time
        .iter()
        .position(|t| t.to_bits() == checkpoint.time.to_bits())
        .unwrap();
    let restored = TransientCheckpoint::from_bytes(
        &checkpoint
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap(),
    )
    .unwrap();
    let (resumed, _) = run_with_checkpoint(text, stop, max_step, Some(&restored), &[]);
    assert_eq!(
        resumed.time.len(),
        baseline.time.len() - seam,
        "original restart grid length"
    );
    for (index, (&actual, &expected)) in resumed.time.iter().zip(&baseline.time[seam..]).enumerate()
    {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "original restart time at suffix {index}"
        );
    }
    for (kind, actual_traces, expected_traces) in [
        ("voltage", &resumed.voltages, &baseline.voltages),
        (
            "current",
            &resumed.branch_currents,
            &baseline.branch_currents,
        ),
    ] {
        assert_eq!(actual_traces.len(), expected_traces.len());
        for (trace, (actual, expected)) in actual_traces.iter().zip(expected_traces).enumerate() {
            if expected.is_empty() {
                assert!(actual.is_empty());
                continue;
            }
            assert_eq!(actual.len(), expected.len() - seam);
            for (index, (&actual, &expected)) in actual.iter().zip(&expected[seam..]).enumerate() {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "original restart {kind} trace {trace}, suffix {index}: actual={actual:e}, expected={expected:e}"
                );
            }
        }
    }
}
