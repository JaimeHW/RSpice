//! Transient checkpoint/restore: segmented runs must continue the same
//! trajectory the unsegmented run follows, file round-trips must be exact,
//! and mismatched state must be refused loudly.

use std::sync::Arc;

use rspice_core::abort_signal::ImmediateAbort;
use rspice_core::engine::ConvergenceConfig;
use rspice_core::engine::{
    Engine, SimulationConfig, SimulationError, SpiceDialect, TransientCheckpoint,
    TransientCheckpointEncoding, TransientStartupMode,
};
use rspice_core::netlist::Netlist;
use rspice_core::numerics::integration::{IntegrationMethod, TransientErrorControl};
use rspice_core::xspice::{register_data_file, unregister_data_file};

/// Sine-driven RC: smooth, source phase depends on absolute time, so a
/// resume that mishandles t0 or the capacitor history shows up immediately.
const DECK: &str = "\
* checkpoint bench: sine-driven rc
vin in 0 sin(0 1 1meg)
r1 in out 1k
c1 out 0 159.155p
.tran 1n 2u
.end
";

const XSPICE_GAIN_DECK: &str = "\
* checkpoint bench: stateless xspice gain
vin in 0 sin(0 1 1meg)
a1 in out amp
.model amp gain (gain=2)
rload out 0 1k
.tran 1n 40n
.end
";

const XSPICE_INTEGRATOR_DECK: &str = "\
* checkpoint bench: stateful xspice integrator
vin in 0 sin(0 1 1meg)
a1 in out integ
.model integ int (gain=1 out_lower_limit=-10 out_upper_limit=10)
rload out 0 1k
.tran 1n 40n
.end
";

#[test]
fn checkpoint_capability_preflight_honors_cancellation() {
    let netlist = Netlist::parse(DECK).expect("checkpoint fixture parses");
    let error = Engine::default()
        .preflight_transient_checkpoint_with_abort(&netlist, &ImmediateAbort)
        .expect_err("cancelled preflight must not elaborate or solve");
    assert!(matches!(error, SimulationError::Aborted));
}

const TAU_STEP: f64 = 1e-9;

fn interpolate(time: &[f64], values: &[f64], t: f64) -> f64 {
    let idx = time.partition_point(|x| *x < t);
    if idx == 0 {
        return values[0];
    }
    if idx >= time.len() {
        return *values.last().unwrap();
    }
    let (t0, t1) = (time[idx - 1], time[idx]);
    let (v0, v1) = (values[idx - 1], values[idx]);
    if t1 == t0 {
        v0
    } else {
        v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    }
}

fn out_index(result: &rspice_core::engine::TransientResult) -> usize {
    result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present")
}

fn assert_scheduled_xspice_deck_resumes_exactly(
    label: &str,
    deck: &str,
    tstop: f64,
    split: f64,
    step: f64,
) {
    let netlist = Netlist::parse(deck).unwrap_or_else(|err| panic!("{label} deck parses: {err}"));
    let engine = Engine::new(SimulationConfig::default());

    // Capture inside the baseline run so observing the checkpoint does not
    // introduce a new endpoint or perturb its adaptive accepted grid.
    let (full, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            tstop,
            step,
            TransientStartupMode::OperatingPoint,
            &[split],
        )
        .unwrap_or_else(|err| panic!("{label} scheduled baseline run completes: {err}"));
    assert_eq!(scheduled.len(), 1, "{label} emits one scheduled checkpoint");
    let packed = scheduled[0]
        .checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .unwrap_or_else(|err| panic!("{label} checkpoint packs: {err}"));
    let checkpoint = TransientCheckpoint::from_bytes(&packed)
        .unwrap_or_else(|err| panic!("{label} packed checkpoint parses: {err}"));
    let baseline_index = full
        .time
        .iter()
        .position(|time| time.to_bits() == checkpoint.time.to_bits())
        .unwrap_or_else(|| panic!("{label} checkpoint is an accepted baseline point"));
    let full_out = out_index(&full);
    let (second, _) = engine
        .run_tran_resume(&netlist, &checkpoint, tstop, step)
        .unwrap_or_else(|err| panic!("{label} resumed segment completes: {err}"));
    let second_out = out_index(&second);

    assert_eq!(
        second.time.len(),
        full.time.len() - baseline_index,
        "{label} resumed accepted-grid length differs from the baseline suffix"
    );
    for (row, (&actual, &expected)) in second
        .time
        .iter()
        .zip(&full.time[baseline_index..])
        .enumerate()
    {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "{label} accepted grid differs at suffix row {row}"
        );
    }
    for (row, (&actual, &expected)) in second.voltages[second_out]
        .iter()
        .zip(&full.voltages[full_out][baseline_index..])
        .enumerate()
    {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "{label} output differs at suffix row {row}: expected {expected:.17e}, got {actual:.17e}"
        );
    }
}

fn branch_index(result: &rspice_core::engine::TransientResult, name: &str) -> usize {
    result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("branch '{name}' present in {:?}", result.branch_names))
}

#[test]
fn uic_checkpoint_resume_preserves_startup_mode_and_floating_trajectory() {
    let netlist = Netlist::parse(
        "UIC checkpoint floating capacitor\n\
         I1 0 out 1m\n\
         C1 out 0 1u\n\
         .TRAN 0.5u 4u UIC\n\
         .END\n",
    )
    .expect("UIC checkpoint deck parses");
    let engine = Engine::default();

    let (first, checkpoint) = engine
        .run_tran_checkpointed_with_startup_mode(
            &netlist,
            2.0e-6,
            0.5e-6,
            TransientStartupMode::Uic,
        )
        .expect("UIC first segment completes without a DC operating point");
    assert_eq!(checkpoint.startup_mode(), Some(TransientStartupMode::Uic));
    assert!(checkpoint.to_text().contains("startup_mode uic\n"));

    let restored = TransientCheckpoint::from_text(&checkpoint.to_text())
        .expect("UIC checkpoint text round-trips");
    let (resumed, final_checkpoint) = engine
        .run_tran_resume(&netlist, &restored, 4.0e-6, 0.5e-6)
        .expect("UIC checkpoint resumes without reclassifying startup as an operating point");
    assert_eq!(
        final_checkpoint.startup_mode(),
        Some(TransientStartupMode::Uic)
    );

    let first_out = out_index(&first);
    let resumed_out = out_index(&resumed);
    assert_eq!(
        first.voltages[first_out].last().unwrap().to_bits(),
        resumed.voltages[resumed_out][0].to_bits(),
        "UIC resume must preserve the seam state exactly"
    );
    assert!(
        resumed.time.iter().all(|value| value.is_finite())
            && resumed.voltages[resumed_out]
                .iter()
                .all(|value| value.is_finite())
    );
}

#[test]
fn pem_resume_exposes_checkpointed_retained_resistance_at_the_seam() {
    let positive = "virtual://checkpoint/pem-retained-positive";
    let negative = "virtual://checkpoint/pem-retained-negative";
    register_data_file(positive, "0,1\n1,1\n").expect("register positive PEM table");
    register_data_file(negative, "0,1\n1,1\n").expect("register negative PEM table");
    let deck = format!(
        "PEM retained store checkpoint\n\
         V1 in 0 0.005\n\
         .model pem memristor level=4 fxpdata={positive} fxmdata={negative}\n\
         YMEMRISTOR mr1 in 0 pem xo=1\n\
         .tran 0.5n 2n\n\
         .end\n"
    );
    let netlist = Netlist::parse_validated(&deck).expect("PEM checkpoint deck validates");
    let config = SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::Trapezoidal,
        ..Default::default()
    };
    let engine = Engine::new(config);
    let (first, mut checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1.0e-9, 0.5e-9)
        .expect("PEM first segment completes");
    let retained = *first.store_traces[0]
        .values
        .last()
        .expect("first segment has a retained resistance");
    assert!(retained.is_finite() && retained != 0.0);

    let state_index = first
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("YMEMRISTOR!MR1_X"))
        .expect("PEM private state node is present");
    checkpoint.solution[state_index] = 0.0;
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2.0e-9, 0.5e-9)
        .expect("PEM resumed segment completes");
    assert_eq!(
        resumed.store_traces[0].values[0].to_bits(),
        retained.to_bits(),
        "the resumed seam sample must expose the checkpointed retained store"
    );

    unregister_data_file(positive).expect("unregister positive PEM table");
    unregister_data_file(negative).expect("unregister negative PEM table");
}

#[test]
fn xyce_generic_switch_store_history_round_trips_across_resume() {
    let netlist = Netlist::parse_validated(
        "generic switch checkpoint history\n\
         V1 1 0 5\n\
         R2 2 0 100\n\
         SW1 1 2 SW OFF CONTROL={time/1n}\n\
         .MODEL SW SWITCH (ON=1 ONH=0.55 OFF=0 OFFH=0.25 RON=1 ROFF=100)\n\
         .TRAN 0 2n 0 1n\n\
         .END\n",
    )
    .expect("generic-switch checkpoint deck validates");
    let locked_grid = Arc::new(vec![
        0.0,
        0.2e-9,
        0.269_311_698e-9,
        0.274_889_451e-9,
        0.3e-9,
        0.6e-9,
        1.0e-9,
        2.0e-9,
    ]);
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(0.2e-9),
        locked_time_grid: Some(locked_grid),
        ..Default::default()
    });

    let full = engine
        .run_tran(&netlist, 2.0e-9, 1.0e-9)
        .expect("unsegmented generic-switch run completes");
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 0.274_889_451e-9, 1.0e-9)
        .expect("generic-switch first segment completes");
    assert!(
        checkpoint.to_text().contains("generic_switch_stores 1\n"),
        "checkpoint contains the generic-switch store-vector state"
    );
    let restored = TransientCheckpoint::from_text(&checkpoint.to_text())
        .expect("generic-switch checkpoint text round-trips");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &restored, 2.0e-9, 1.0e-9)
        .expect("generic-switch checkpoint resumes");

    let first_branch = branch_index(&first, "SW1");
    let resumed_branch = branch_index(&resumed, "SW1");
    assert_eq!(
        first.branch_currents[first_branch]
            .last()
            .expect("first segment has a seam current")
            .to_bits(),
        resumed.branch_currents[resumed_branch][0].to_bits(),
        "restored accepted conductance preserves the seam current bit-exactly"
    );

    let full_branch = branch_index(&full, "SW1");
    for time in [0.3e-9, 0.6e-9, 1.0e-9, 2.0e-9] {
        let expected = interpolate(&full.time, &full.branch_currents[full_branch], time);
        let actual = interpolate(
            &resumed.time,
            &resumed.branch_currents[resumed_branch],
            time,
        );
        assert!(
            (expected - actual).abs() <= 1.0e-12,
            "generic-switch resumed current at {time:e} must match the unsegmented trajectory: expected {expected:e}, got {actual:e}"
        );
    }
}

#[test]
fn segmented_run_continues_the_unsegmented_trajectory() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    // Reference: one unsegmented run to 2 µs.
    let full = engine
        .run_tran(&netlist, 2e-6, TAU_STEP)
        .expect("full run completes");
    let full_out = out_index(&full);

    // Segmented: run to 1 µs, checkpoint, resume to 2 µs.
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-6, TAU_STEP)
        .expect("first segment completes");
    assert!(
        (checkpoint.time - 1e-6).abs() < 1e-9,
        "checkpoint lands at the segment end, got {}",
        checkpoint.time
    );
    let (second, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-6, TAU_STEP)
        .expect("resumed segment completes");

    let second_out = out_index(&second);
    assert!(
        (second.time[0] - checkpoint.time).abs() < 1e-15,
        "resumed run starts at the checkpoint time"
    );

    // The resumed trajectory must track the unsegmented one. Compare at
    // sample times across the second segment; the restart introduces only
    // integration-tolerance-level differences.
    let mut worst = 0.0f64;
    for k in 1..=40 {
        let t = 1.0e-6 + (k as f64) * 24e-9;
        let v_full = interpolate(&full.time, &full.voltages[full_out], t);
        let v_seg = interpolate(&second.time, &second.voltages[second_out], t);
        worst = worst.max((v_full - v_seg).abs());
    }
    assert!(
        worst < 2e-3,
        "segmented trajectory must track the unsegmented run (worst |Δ| = {worst})"
    );

    // Continuity at the seam: the resumed first point equals the first
    // segment's last point exactly (same solution vector).
    let v_seam_first = *first.voltages[out_index(&first)].last().unwrap();
    let v_seam_second = second.voltages[second_out][0];
    assert_eq!(
        v_seam_first.to_bits(),
        v_seam_second.to_bits(),
        "seam state is carried bit-exactly"
    );
}

#[test]
fn fixed_gear2_resume_uses_be_before_real_history_bdf2() {
    const TAU: f64 = 1.0e-3;
    const PRE_RESUME_STEP: f64 = 100.0e-6;
    const SPLIT: f64 = 500.0e-6;
    const RESUME_STEP: f64 = 1.0e-3;

    let netlist = Netlist::parse(
        "\
* checkpoint fixed-Gear2 order restart
r1 out 0 1k
c1 out 0 1u ic=1
.tran 100u 3m uic
.end
",
    )
    .expect("fixed-Gear2 checkpoint deck parses");
    let first_engine = Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::Gear2,
        transient_initial_timestep: Some(PRE_RESUME_STEP),
        ..Default::default()
    });
    let (_, checkpoint) = first_engine
        .run_tran_checkpointed(&netlist, SPLIT, PRE_RESUME_STEP)
        .expect("fixed-Gear2 first segment completes");

    let resume_grid = Arc::new(vec![SPLIT, SPLIT + RESUME_STEP, SPLIT + 2.0 * RESUME_STEP]);
    let resume_engine = Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::Gear2,
        transient_initial_timestep: Some(RESUME_STEP),
        locked_time_grid: Some(resume_grid),
        ..Default::default()
    });
    let (resumed, _) = resume_engine
        .run_tran_resume(
            &netlist,
            &checkpoint,
            SPLIT + 2.0 * RESUME_STEP,
            RESUME_STEP,
        )
        .expect("fixed-Gear2 resumed segment completes");
    let trace = &resumed.voltages[out_index(&resumed)];

    assert_eq!(resumed.time.len(), 3, "resume grid has two real intervals");
    assert_eq!(resumed.time[0].to_bits(), checkpoint.time.to_bits());

    let first_dt = resumed.time[1] - resumed.time[0];
    let expected_be = trace[0] / (1.0 + first_dt / TAU);
    assert!(
        (trace[1] - expected_be).abs() <= 1.0e-12,
        "first resumed Gear2 interval must be backward Euler: expected {expected_be:e}, got {:e}",
        trace[1]
    );

    let second_dt = resumed.time[2] - resumed.time[1];
    let ratio = second_dt / first_dt;
    let a0 = (1.0 + 2.0 * ratio) / (1.0 + ratio);
    let a1 = 1.0 + ratio;
    let a2 = -(ratio * ratio) / (1.0 + ratio);
    let expected_bdf2 = (a1 * trace[1] + a2 * trace[0]) / (a0 + second_dt / TAU);
    assert!(
        (trace[2] - expected_bdf2).abs() <= 1.0e-12,
        "second resumed Gear2 interval must use the real first interval as BDF2 history: expected {expected_bdf2:e}, got {:e}",
        trace[2]
    );
}

#[test]
fn xyce_scheduled_checkpoint_restores_the_post_breakpoint_step_proposal() {
    let netlist = Netlist::parse(
        "scheduled checkpoint preserves Xyce continuation sizing\n\
         .tran 0 50u\n\
         V1 in 0 pulse(0 1 0 1u 1u 5u 10u)\n\
         R1 in 0 1k\n\
         .end\n",
    )
    .expect("scheduled-continuation deck parses");
    let mut convergence_config = ConvergenceConfig::robust();
    convergence_config.voltage_reltol = 1.0e-4;
    let engine = Engine::new(SimulationConfig {
        max_iterations: 1200,
        convergence_config,
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::TrapGear,
        transient_error_control: TransientErrorControl::NonlinearIterations,
        transient_initial_timestep: Some(1.0e-10),
        ..Default::default()
    });
    // 23 us is inside the 21-26 us source-breakpoint span. Re-anchoring the
    // span ceiling here would be observably tighter than restoring the active
    // ceiling established at 21 us.
    let seam = 23.0e-6;
    let stop = 50.0e-6;
    let max_step = 5.0e-6;
    let (full, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            stop,
            max_step,
            TransientStartupMode::OperatingPoint,
            &[seam, stop],
        )
        .expect("continuous run and scheduled checkpoint complete");
    assert_eq!(scheduled.len(), 2);
    assert!(
        (scheduled[1].checkpoint.time - stop).abs() <= 1.0e-20,
        "the scheduled endpoint must land within Xyce's breakpoint tolerance"
    );
    assert!(
        scheduled[1]
            .checkpoint
            .to_text()
            .contains("integration_continuation breakpoint-restart\n"),
        "a scheduled TSTOP checkpoint is an endpoint restart, not in-flight continuation"
    );
    let checkpoint = TransientCheckpoint::from_text(&scheduled[0].checkpoint.to_text())
        .expect("scheduled continuation state round-trips");
    assert!(
        full.time
            .iter()
            .any(|time| time.to_bits() == checkpoint.time.to_bits()),
        "scheduled checkpoint is an accepted baseline point"
    );
    let checkpoint_text = checkpoint.to_text();
    let continuation = checkpoint_text
        .lines()
        .find_map(|line| line.strip_prefix("integration_continuation proposed "))
        .expect("an in-flight scheduled checkpoint carries proposed continuation state");
    let mut continuation = continuation.split_whitespace();
    let proposal = continuation
        .next()
        .expect("continuation carries a next-step proposal")
        .parse::<f64>()
        .expect("continuation proposal is numeric");
    let span_ceiling = continuation
        .next()
        .expect("continuation carries an active span ceiling")
        .parse::<f64>()
        .expect("active span ceiling is numeric");
    let controller_max_step = continuation
        .next()
        .expect("continuation carries its effective controller maximum")
        .parse::<f64>()
        .expect("effective controller maximum is numeric");
    let analysis_first_step_pending = continuation
        .next()
        .expect("continuation carries the global analysis phase");
    let breakpoint_restart_pending = continuation
        .next()
        .expect("continuation carries the breakpoint-restart phase");
    assert!(continuation.next().is_none());
    assert!(proposal.is_finite() && proposal > 0.0);
    assert!(span_ceiling.is_finite() && span_ceiling > 0.0);
    assert!(controller_max_step.is_finite() && controller_max_step > 0.0);
    assert_eq!(analysis_first_step_pending, "0");
    assert_eq!(breakpoint_restart_pending, "0");
    assert!(proposal <= controller_max_step);
    assert_ne!(
        proposal.to_bits(),
        1.0e-10_f64.to_bits(),
        "the fixture must distinguish saved continuation from fresh Xyce startup sizing"
    );
    let reanchored_ceiling = (26.0e-6 - checkpoint.time) / 10.0;
    assert!(
        proposal > reanchored_ceiling && span_ceiling > reanchored_ceiling,
        "the fixture must expose interior-span re-anchoring: proposal={proposal:e}, saved ceiling={span_ceiling:e}, artificial ceiling={reanchored_ceiling:e}"
    );

    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, stop, max_step)
        .expect("scheduled checkpoint resumes");
    assert_eq!(resumed.time[0].to_bits(), checkpoint.time.to_bits());
    assert_eq!(
        resumed.time[1].to_bits(),
        (checkpoint.time + proposal).to_bits(),
        "resume must use the post-accept proposal rather than recomputing startup sizing"
    );
}

#[test]
fn xyce_source_breakpoint_checkpoint_restores_the_global_controller_phase() {
    let netlist = Netlist::parse(
        "source-breakpoint checkpoint preserves Xyce controller phase\n\
         .tran 0 50u\n\
         V1 in 0 pulse(0 1 0 1u 1u 5u 10u)\n\
         R1 in 0 1k\n\
         .end\n",
    )
    .expect("source-breakpoint continuation deck parses");
    let mut convergence_config = ConvergenceConfig::robust();
    convergence_config.voltage_reltol = 1.0e-4;
    let engine = Engine::new(SimulationConfig {
        max_iterations: 1200,
        convergence_config,
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::TrapGear,
        transient_error_control: TransientErrorControl::LocalTruncation,
        transient_initial_timestep: Some(1.0e-10),
        ..Default::default()
    });
    let seam = 21.0e-6;
    let stop = 50.0e-6;
    let max_step = 5.0e-6;
    let (full, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            stop,
            max_step,
            TransientStartupMode::OperatingPoint,
            &[seam, stop],
        )
        .expect("continuous source-breakpoint run completes");
    let source_checkpoint = &scheduled
        .first()
        .expect("source-breakpoint checkpoint is captured")
        .checkpoint;
    let baseline_index = full
        .time
        .iter()
        .position(|time| time.to_bits() == source_checkpoint.time.to_bits())
        .expect("source checkpoint is an accepted baseline point");
    let expected = full
        .time
        .get(baseline_index..baseline_index + 7)
        .expect("baseline has six accepted intervals after the source breakpoint");
    let checkpoint_text = source_checkpoint.to_text();
    let continuation = checkpoint_text
        .lines()
        .find_map(|line| line.strip_prefix("integration_continuation proposed "))
        .expect("source breakpoint carries an in-flight proposal")
        .split_whitespace()
        .collect::<Vec<_>>();
    assert_eq!(continuation.len(), 5);
    assert_eq!(continuation[3], "0", "the global first step is complete");
    assert_eq!(
        continuation[4], "1",
        "the first departure from the source breakpoint remains pending"
    );

    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let encoded = source_checkpoint
            .to_bytes(encoding)
            .unwrap_or_else(|error| panic!("{encoding:?} checkpoint encodes: {error}"));
        let checkpoint = TransientCheckpoint::from_bytes(&encoded)
            .unwrap_or_else(|error| panic!("{encoding:?} checkpoint decodes: {error}"));
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, stop, max_step)
            .unwrap_or_else(|error| panic!("{encoding:?} checkpoint resumes: {error}"));
        assert!(
            resumed.time.len() >= 7,
            "{encoding:?} resume must accept six intervals after the seam"
        );
        for (offset, (&actual, &baseline)) in resumed.time[..7].iter().zip(expected).enumerate() {
            assert_eq!(
                actual.to_bits(),
                baseline.to_bits(),
                "{encoding:?} accepted grid differs at source-breakpoint suffix row {offset}: baseline={baseline:.17e}, resumed={actual:.17e}"
            );
        }
    }
}

#[test]
fn xyce_source_breakpoint_restart_preserves_reactive_bjt_trajectory() {
    fn assert_exact_suffix(label: &str, actual: &[f64], baseline: &[f64]) {
        assert_eq!(
            actual.len(),
            baseline.len(),
            "{label} length differs: baseline={}, resumed={}",
            baseline.len(),
            actual.len()
        );
        for (row, (&actual, &baseline)) in actual.iter().zip(baseline).enumerate() {
            assert_eq!(
                actual.to_bits(),
                baseline.to_bits(),
                "{label} differs at suffix row {row}: baseline={baseline:.17e} ({:#018x}), resumed={actual:.17e} ({:#018x})",
                baseline.to_bits(),
                actual.to_bits()
            );
        }
    }

    let netlist = Netlist::parse(
        "source-breakpoint restart preserves reactive BJT trajectory\n\
         .tran 0 30u\n\
         VCC vcc 0 5\n\
         VDRIVE drive 0 pulse(0 1 0 1u 1u 4u 10u)\n\
         RB drive base 1k\n\
         RC vcc collector 470\n\
         Q1 collector base 0 QBENCH\n\
         CLOAD collector 0 1n\n\
         .model QBENCH NPN(Is=14.34f Bf=255.9 Vaf=74.03 Rb=10 Cjc=7.306p \
                         Mjc=.3416 Vjc=.75 Cje=22.01p Mje=.377 Vje=.75 \
                         Tr=46.91n Tf=411.1p Itf=.6 Vtf=1.7 Xtf=3)\n\
         .end\n",
    )
    .expect("reactive BJT checkpoint deck parses");
    let mut convergence_config = ConvergenceConfig::robust();
    convergence_config.voltage_reltol = 1.0e-4;
    let engine = Engine::new(SimulationConfig {
        max_iterations: 1200,
        convergence_config,
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::TrapGear,
        transient_error_control: TransientErrorControl::LocalTruncation,
        transient_initial_timestep: Some(1.0e-10),
        temperature: 300.15,
        ..Default::default()
    });
    let seam = 21.0e-6;
    let stop = 30.0e-6;
    let max_step = 500.0e-9;
    let (full, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            stop,
            max_step,
            TransientStartupMode::OperatingPoint,
            &[seam],
        )
        .expect("continuous reactive BJT run completes");
    let source_checkpoint = &scheduled
        .first()
        .expect("reactive BJT source-breakpoint checkpoint is captured")
        .checkpoint;
    let baseline_index = full
        .time
        .iter()
        .position(|time| time.to_bits() == source_checkpoint.time.to_bits())
        .expect("reactive BJT checkpoint is an accepted baseline point");
    let expected_time = &full.time[baseline_index..];

    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let encoded = source_checkpoint
            .to_bytes(encoding)
            .unwrap_or_else(|error| panic!("{encoding:?} checkpoint encodes: {error}"));
        let checkpoint = TransientCheckpoint::from_bytes(&encoded)
            .unwrap_or_else(|error| panic!("{encoding:?} checkpoint decodes: {error}"));
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, stop, max_step)
            .unwrap_or_else(|error| panic!("{encoding:?} checkpoint resumes: {error}"));
        assert_exact_suffix(
            &format!("{encoding:?} accepted grid"),
            &resumed.time,
            expected_time,
        );
        for (node, (actual, baseline)) in resumed.voltages.iter().zip(&full.voltages).enumerate() {
            assert_exact_suffix(
                &format!("{encoding:?} node {node}"),
                actual,
                &baseline[baseline_index..],
            );
        }
        for (branch, (actual, baseline)) in resumed
            .branch_currents
            .iter()
            .zip(&full.branch_currents)
            .enumerate()
        {
            assert_exact_suffix(
                &format!("{encoding:?} branch {branch}"),
                actual,
                &baseline[baseline_index..],
            );
        }
    }
}

/// A restart may widen its own maximum step. The captured cap bounded the
/// steps of the segment that is already over; it is not seam state, and Xyce
/// likewise recomputes the working cap from the restart deck rather than
/// comparing it against the restart file. The resumed segment must therefore
/// take steps its own cap allows, not steps the captured cap allowed.
#[test]
fn resume_honors_its_own_maximum_step_instead_of_the_captured_one() {
    const CAPTURED_STEP: f64 = 10.0e-6;
    const RESUME_STEP: f64 = 200.0e-6;
    const SPLIT: f64 = 500.0e-6;
    const STOP: f64 = 3.0e-3;

    let netlist = Netlist::parse(
        "\
* checkpoint restart widens its maximum step
r1 out 0 1k
c1 out 0 1u ic=1
.tran 100u 3m uic
.end
",
    )
    .expect("widened-cap checkpoint deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, CAPTURED_STEP)
        .expect("first segment completes under the fine cap");

    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, STOP, RESUME_STEP)
        .expect("a resumed segment may choose a coarser maximum step");

    assert_eq!(resumed.time[0].to_bits(), checkpoint.time.to_bits());
    let widest = resumed
        .time
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .fold(0.0_f64, f64::max);
    assert!(
        widest > CAPTURED_STEP,
        "the captured cap must not clamp the resumed segment: widest resumed step {widest:e}s \
         did not exceed the captured cap {CAPTURED_STEP:e}s"
    );
    assert!(
        widest <= RESUME_STEP * (1.0 + 1.0e-12),
        "the resumed segment's own cap still binds: widest resumed step {widest:e}s exceeds \
         {RESUME_STEP:e}s"
    );
}

#[test]
fn checkpoint_file_round_trip_resumes_identically() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-6, TAU_STEP)
        .expect("segment completes");

    let directory = std::env::temp_dir().join(format!(
        "rspice-checkpoint-roundtrip-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos()
    ));
    std::fs::create_dir(&directory).expect("checkpoint directory");
    let path = directory.join("state.ckpt");
    std::fs::write(&path, b"obsolete partial checkpoint").expect("seed old checkpoint");
    checkpoint
        .save(&path)
        .expect("checkpoint atomically replaces old state");
    let loaded = TransientCheckpoint::load(&path).expect("checkpoint loads");
    assert_eq!(
        std::fs::read_dir(&directory)
            .expect("read checkpoint directory")
            .count(),
        1,
        "committed checkpoint must not leave a temporary sibling"
    );
    assert_eq!(checkpoint, loaded, "file round-trip is exact");

    let (from_memory, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 1.5e-6, TAU_STEP)
        .expect("resume from memory");
    let (from_file, _) = engine
        .run_tran_resume(&netlist, &loaded, 1.5e-6, TAU_STEP)
        .expect("resume from file");

    assert_eq!(from_memory.time.len(), from_file.time.len());
    let out = out_index(&from_memory);
    assert!(
        from_memory.voltages[out]
            .iter()
            .zip(&from_file.voltages[out_index(&from_file)])
            .all(|(a, b)| a.to_bits() == b.to_bits()),
        "file-loaded checkpoint resumes bit-identically"
    );
    std::fs::remove_dir_all(directory).expect("remove checkpoint directory");
}

#[test]
fn xyce_signal_history_modes_survive_segmented_disk_checkpoints() {
    for selector in [2, 3] {
        let deck = format!(
            "\
* Xyce NEWLTE signal-history checkpoint bench
vzero in 0 0
r1 in out 1k
c1 out 0 100n ic=1
.options timeint reltol=1e-5 abstol=1e-7 newlte={selector}
.tran 10u 1m uic
.end
"
        );
        let netlist = Netlist::parse(&deck).expect("NEWLTE checkpoint deck parses");
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        });

        let full = engine
            .run_tran(&netlist, 1.0e-3, 20.0e-6)
            .expect("unsegmented NEWLTE run completes");
        let full_out = out_index(&full);
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 0.4e-3, 20.0e-6)
            .expect("NEWLTE first segment completes");

        let checkpoint_text = checkpoint.to_text();
        assert!(
            checkpoint_text.contains(&format!("lte_reference_mode {selector}\n")),
            "checkpoint records the resolved NEWLTE={selector} provenance"
        );
        let local_count = checkpoint_text
            .lines()
            .find_map(|line| line.strip_prefix("lte_signal_local "))
            .expect("checkpoint contains the signal-local vector header")
            .parse::<usize>()
            .expect("signal-local count is numeric");
        if selector == 2 {
            assert_eq!(local_count, 0, "NEWLTE=2 stores one global reference");
        } else {
            assert!(local_count > 0, "NEWLTE=3 stores per-variable references");
        }

        let path = std::env::temp_dir().join(format!(
            "rspice_newlte{selector}_checkpoint_{}.ckpt",
            std::process::id()
        ));
        checkpoint.save(&path).expect("NEWLTE checkpoint saves");
        let loaded = TransientCheckpoint::load(&path).expect("NEWLTE checkpoint loads");
        let _ = std::fs::remove_file(&path);
        assert_eq!(checkpoint, loaded, "NEWLTE checkpoint round-trip is exact");

        let (memory_resume, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.0e-3, 20.0e-6)
            .expect("NEWLTE checkpoint resumes from memory");
        let (disk_resume, _) = engine
            .run_tran_resume(&netlist, &loaded, 1.0e-3, 20.0e-6)
            .expect("NEWLTE checkpoint resumes from disk");
        assert_eq!(memory_resume.time, disk_resume.time);
        assert!(
            memory_resume.voltages[out_index(&memory_resume)]
                .iter()
                .zip(&disk_resume.voltages[out_index(&disk_resume)])
                .all(|(memory, disk)| memory.to_bits() == disk.to_bits()),
            "NEWLTE={selector} disk resume is bit-identical to memory resume"
        );

        let mut worst = 0.0_f64;
        let resumed_out = out_index(&memory_resume);
        for sample in 1..=20 {
            let time = 0.4e-3 + sample as f64 * 30.0e-6;
            let expected = interpolate(&full.time, &full.voltages[full_out], time);
            let actual = interpolate(
                &memory_resume.time,
                &memory_resume.voltages[resumed_out],
                time,
            );
            worst = worst.max((expected - actual).abs());
        }
        assert!(
            worst < 5.0e-3,
            "NEWLTE={selector} segmented trajectory tracks the full run (worst |delta|={worst})"
        );
    }
}

#[test]
fn mismatched_netlist_is_refused() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let other = Netlist::parse(
        "\
* different circuit
vin in 0 dc 1
r1 in out 2k
c1 out 0 100p
.tran 1n 1u
.end
",
    )
    .expect("other deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 0.5e-6, TAU_STEP)
        .expect("segment completes");

    let err = engine
        .run_tran_resume(&other, &checkpoint, 1e-6, TAU_STEP)
        .expect_err("mismatched netlist must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("different netlist"),
        "diagnostic names the problem: {message}"
    );
}

#[test]
fn resume_requires_a_later_stop_time() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-6, TAU_STEP)
        .expect("segment completes");

    let err = engine
        .run_tran_resume(&netlist, &checkpoint, 0.5e-6, TAU_STEP)
        .expect_err("earlier tstop must be refused");
    assert!(format!("{err}").contains("must exceed"));
}

#[test]
fn stateless_xspice_checkpoint_resume_tracks_unsegmented_gain() {
    assert_scheduled_xspice_deck_resumes_exactly(
        "stateless XSPICE gain",
        XSPICE_GAIN_DECK,
        40e-9,
        20e-9,
        TAU_STEP,
    );
}

#[test]
fn stateful_xspice_checkpoint_resume_tracks_unsegmented_integrator() {
    assert_scheduled_xspice_deck_resumes_exactly(
        "stateful XSPICE integrator",
        XSPICE_INTEGRATOR_DECK,
        40e-9,
        20e-9,
        TAU_STEP,
    );
}

#[test]
fn stateful_analog_xspice_checkpoint_resume_tracks_additional_models() {
    let cases = [
        (
            "d_dt",
            "\
* checkpoint bench: differentiator
vin in 0 sin(0 1 1meg)
a1 in out diff
.model diff d_dt (gain=1e-6 out_lower_limit=-10 out_upper_limit=10)
rload out 0 1k
.tran 0.5n 60n
.end
",
            60.0e-9,
            30.0e-9,
            0.5e-9,
        ),
        (
            "hyst",
            "\
* checkpoint bench: hysteresis
vin in 0 pulse(0 2 0 1n 1n 20n 40n)
a1 in out h
.model h hyst (in_low=0.5 in_high=1.5 hyst=0.1 out_lower_limit=0 out_upper_limit=5 input_domain=0.01)
rload out 0 1k
.tran 1n 90n
.end
",
            90.0e-9,
            45.0e-9,
            1.0e-9,
        ),
        (
            "slew",
            "\
* checkpoint bench: slew
vin in 0 pulse(0 1 0 1n 1n 20n 40n)
a1 in out sl
.model sl slew (rise_slope=1e8 fall_slope=1e8)
rload out 0 1k
.tran 1n 90n
.end
",
            90.0e-9,
            45.0e-9,
            1.0e-9,
        ),
        (
            "astate",
            "\
* checkpoint bench: analog state return
vin in 0 pulse(0 1 0 1n 1n 10n 20n)
a1 in out ast
.model ast astate (astate_no=1)
rload out 0 1k
.tran 1n 60n
.end
",
            60.0e-9,
            // `astate` returns an accepted-sample history value. Request the
            // checkpoint at its source edge so the exact continuation test
            // also covers a model-state transition boundary.
            31.0e-9,
            1.0e-9,
        ),
    ];

    for (label, deck, tstop, split, step) in cases {
        assert_scheduled_xspice_deck_resumes_exactly(label, deck, tstop, split, step);
    }
}

#[test]
fn event_driven_xspice_checkpoint_is_refused_during_preflight() {
    let uri = "virtual://transient_checkpoint/event_state_blocker";
    register_data_file(uri, "0 0s\n1n 1s\n").expect("register virtual d_source data");
    let deck = format!(
        "\
* xspice event checkpoint boundary
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"{uri}\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.tran 100p 2n
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("XSPICE event deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let capability = engine
        .preflight_transient_checkpoint(&netlist)
        .expect("checkpoint capability preflight elaborates the deck");
    assert!(!capability.is_resumable());
    assert!(capability.blockers().iter().any(|blocker| {
        blocker.source == rspice_core::engine::TransientCheckpointBlockerSource::ExtensionState
            && blocker.message.contains("event node values")
    }));
    let err = engine
        .run_tran_checkpointed(&netlist, 1e-9, 100e-12)
        .expect_err("event-driven XSPICE checkpoint must be refused before solver work");
    let _ = unregister_data_file(uri);
    let message = format!("{err}");
    assert!(
        message.contains("checkpoint capability preflight failed")
            && message.contains("event node values"),
        "diagnostic should explain the unsupported checkpoint before solving: {message}"
    );
}

#[test]
fn every_checkpoint_blocker_is_reported_under_exactly_one_source() {
    // The accepted-runtime inventory a checkpoint stores is one flat string
    // list — a resume compares it by equality — so the extension-owned
    // messages sit in it beside the device-integration ones. A capability
    // report is not that list. A frontend groups it by owner, so a message
    // arriving under two owners would read as two blocked subsystems, and an
    // XSPICE model would look like a defect in the integrator.
    let uri = "virtual://transient_checkpoint/blocker_source_partition";
    register_data_file(uri, "0 0s\n1n 1s\n").expect("register virtual d_source data");
    let deck = format!(
        "\
* one extension-owned blocker and one integration-owned blocker in one deck
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"{uri}\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
l1 out mid 1u
l2 mid 0 1u
k1 l1 l2 0.5
.tran 100p 2n
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("the mixed-blocker deck parses");
    let capability = Engine::new(SimulationConfig::default())
        .preflight_transient_checkpoint(&netlist)
        .expect("checkpoint capability preflight elaborates the deck");
    let _ = unregister_data_file(uri);

    let blockers = capability.blockers();
    for blocker in blockers {
        let sources: Vec<_> = blockers
            .iter()
            .filter(|other| other.message == blocker.message)
            .map(|other| other.source)
            .collect();
        assert_eq!(
            sources.len(),
            1,
            "'{}' must be reported under exactly one source, saw {sources:?}",
            blocker.message
        );
    }
    assert!(
        blockers.iter().any(|blocker| {
            blocker.source == rspice_core::engine::TransientCheckpointBlockerSource::ExtensionState
                && blocker.message.contains("event node values")
        }),
        "an XSPICE model's pending state is owned by an extension runtime: {blockers:?}"
    );
    assert!(
        blockers.iter().any(|blocker| {
            blocker.source
                == rspice_core::engine::TransientCheckpointBlockerSource::IntegrationRuntime
                && blocker.message.contains("coupled-inductor")
        }),
        "a coupled inductor's accepted history is owned by the integration runtime: {blockers:?}"
    );
}

#[cfg(feature = "veriloga-builtins")]
#[test]
fn generated_veriloga_checkpoint_preserves_reactive_history_and_provenance() {
    use rspice_core::device::veriloga_builtins::builtins;

    const STEP: f64 = 250.0e-12;
    const SPLIT_INDEX: usize = 80;
    const STOP_INDEX: usize = 160;
    const SPLIT: f64 = SPLIT_INDEX as f64 * STEP;
    const STOP: f64 = STOP_INDEX as f64 * STEP;

    assert!(
        builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case("DIODE_CMC")),
        "the canonical generated CMC diode must be present"
    );

    let netlist = Netlist::parse(
        "\
* generated Verilog-A checkpoint bench: junction-charge history
vin in 0 pulse(0 1 0 1n 1n 8n 20n)
r1 in out 1k
d1 out 0 dcmc
.model dcmc d level=2002
.tran 250p 40n
.end
",
    )
    .expect("generated CMC diode checkpoint deck parses");
    let engine = Engine::new(SimulationConfig {
        tolerance: 1.0e-13,
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(STEP),
        convergence_config: ConvergenceConfig::default()
            .with_voltage_tolerances(1.0e-12, 1.0e-14)
            .with_current_tolerance(1.0e-15)
            .with_residual_reltol(1.0e-12),
        locked_time_grid: Some(Arc::new(
            (0..=STOP_INDEX).map(|index| index as f64 * STEP).collect(),
        )),
        ..Default::default()
    });

    let full = engine
        .run_tran(&netlist, STOP, STEP)
        .expect("unsegmented generated CMC diode run completes");
    let full_out = out_index(&full);
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, STEP)
        .expect("generated CMC diode first segment completes");

    let checkpoint_text = checkpoint.to_text();
    assert!(
        checkpoint_text.contains("generated_veriloga_state_available 1\n"),
        "current checkpoints record generated-state availability"
    );
    assert!(
        checkpoint_text.contains("generated_veriloga_states 1\n"),
        "the generated diode instance is serialized"
    );
    assert!(
        checkpoint_text.contains("ddt_state 5\n"),
        "the CMC diode's five accepted ddt histories are serialized"
    );
    let ddt_rows = checkpoint_text
        .lines()
        .skip_while(|line| *line != "ddt_state 5")
        .skip(1)
        .take(5)
        .collect::<Vec<_>>();
    assert_eq!(ddt_rows.len(), 5);
    assert!(
        ddt_rows.iter().any(|row| {
            let fields = row.split_whitespace().collect::<Vec<_>>();
            fields.len() == 4
                && fields[3] == "1"
                && fields[..3]
                    .iter()
                    .any(|value| value.parse::<f64>().is_ok_and(|value| value != 0.0))
        }),
        "the accepted diode-charge history is initialized and nonzero at the checkpoint"
    );

    let checkpoint_path = std::env::temp_dir().join(format!(
        "rspice_generated_veriloga_checkpoint_{}.ckpt",
        std::process::id()
    ));
    checkpoint
        .save(&checkpoint_path)
        .expect("generated checkpoint saves");
    let disk_checkpoint =
        TransientCheckpoint::load(&checkpoint_path).expect("generated checkpoint loads");
    let _ = std::fs::remove_file(&checkpoint_path);
    assert_eq!(
        checkpoint, disk_checkpoint,
        "generated persistent state round-trips exactly through disk"
    );

    let (memory_resume, _) = engine
        .run_tran_resume(&netlist, &checkpoint, STOP, STEP)
        .expect("generated checkpoint resumes from memory");
    let (disk_resume, _) = engine
        .run_tran_resume(&netlist, &disk_checkpoint, STOP, STEP)
        .expect("generated checkpoint resumes from disk");
    assert_eq!(memory_resume.time, disk_resume.time);
    assert!(
        memory_resume.voltages[out_index(&memory_resume)]
            .iter()
            .zip(&disk_resume.voltages[out_index(&disk_resume)])
            .all(|(memory, disk)| memory.to_bits() == disk.to_bits()),
        "disk and in-memory generated-state resumes are bit-identical"
    );

    let resumed_out = out_index(&memory_resume);
    let mut worst = 0.0_f64;
    for sample in 1..=20 {
        let time = SPLIT + sample as f64 * 1.0e-9;
        let expected = interpolate(&full.time, &full.voltages[full_out], time);
        let actual = interpolate(
            &memory_resume.time,
            &memory_resume.voltages[resumed_out],
            time,
        );
        worst = worst.max((expected - actual).abs());
    }
    assert!(
        worst < 1.0e-12,
        "generated checkpoint continuation tracks the unsegmented trajectory (worst |delta|={worst})"
    );
    assert_eq!(
        first.voltages[out_index(&first)]
            .last()
            .expect("first segment has a seam sample")
            .to_bits(),
        memory_resume.voltages[resumed_out][0].to_bits(),
        "generated checkpoint carries the seam solution bit-exactly"
    );

    let generated_prefix = checkpoint_text
        .lines()
        .take_while(|line| !line.starts_with("generated_veriloga_state_available "))
        .collect::<Vec<_>>()
        .join("\n");
    let following_sections = checkpoint_text
        .lines()
        .skip_while(|line| !line.starts_with("runtime_veriloga_state_available "))
        .collect::<Vec<_>>()
        .join("\n");
    let upgraded_legacy_text = format!(
        "{generated_prefix}\ngenerated_veriloga_state_available 0\ngenerated_veriloga_states 0\n{following_sections}\n"
    );
    let legacy = TransientCheckpoint::from_text(&upgraded_legacy_text)
        .expect("the upgraded legacy checkpoint parses with unavailable generated state");
    let legacy_error = engine
        .run_tran_resume(&netlist, &legacy, STOP, STEP)
        .expect_err("legacy checkpoints cannot silently reset generated reactive history");
    assert!(
        legacy_error
            .to_string()
            .contains("does not contain generated Verilog-A persistent state"),
        "legacy refusal identifies missing generated persistent state: {legacy_error}"
    );

    let generated_header = checkpoint_text
        .lines()
        .find(|line| line.starts_with("generated_veriloga_state "))
        .expect("checkpoint contains generated instance provenance");
    let identity = generated_header
        .split_whitespace()
        .nth(3)
        .expect("generated checkpoint header contains a model identity");
    assert_ne!(identity, "0".repeat(64));
    let wrong_identity_text = checkpoint_text.replacen(identity, &"0".repeat(64), 1);
    let wrong_identity = TransientCheckpoint::from_text(&wrong_identity_text)
        .expect("syntactically valid checkpoint with stale model identity parses");
    let identity_error = engine
        .run_tran_resume(&netlist, &wrong_identity, STOP, STEP)
        .expect_err("checkpoint state from a different generated artifact must be refused");
    assert!(
        identity_error.to_string().contains("model identity"),
        "identity refusal identifies generated-model provenance: {identity_error}"
    );
}

#[cfg(feature = "veriloga-model-vbic13")]
#[test]
fn generated_vbic_initial_step_state_is_accepted_at_origin_and_survives_resume() {
    const STEP: f64 = 1.0e-12;
    const SPLIT: f64 = 2.0e-12;
    const STOP: f64 = 4.0e-12;
    let netlist = Netlist::parse(
        "\
* generated VBIC origin-state checkpoint bench
vcc c 0 1
vb b 0 0.65
q1 c b 0 qmod
.model qmod npn (level=11 is=1e-16 ibei=1e-18 ibci=1e-18 rcx=1)
.tran 1p 4p
.end
",
    )
    .expect("generated VBIC origin-state deck parses");
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(STEP),
        locked_time_grid: Some(Arc::new((0..=4).map(|index| index as f64 * STEP).collect())),
        ..Default::default()
    });

    engine
        .run_tran(&netlist, STOP, STEP)
        .expect("uninterrupted generated VBIC run completes");
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, STEP)
        .expect("generated VBIC first segment completes");
    let checkpoint_text = checkpoint.to_text();
    let mut lines = checkpoint_text.lines();
    let event_header = lines
        .find(|line| line.starts_with("event_state "))
        .expect("format-26 checkpoint contains generated event state");
    assert_eq!(event_header, "event_state 20");
    let accepted_event_state = lines
        .by_ref()
        .take(20)
        .map(|line| line.parse::<f64>().expect("numeric VBIC event state"))
        .collect::<Vec<_>>();
    assert_eq!(accepted_event_state.len(), 20);
    assert!(
        accepted_event_state.iter().any(|value| *value != 0.0),
        "VBIC initial_step assignments must be accepted at t=0 before the first positive step"
    );

    let rewrite_first_event_value = |replacement: &str| {
        let mut rewritten = String::with_capacity(checkpoint_text.len());
        let mut replace_next = false;
        for line in checkpoint_text.lines() {
            rewritten.push_str(if replace_next { replacement } else { line });
            rewritten.push('\n');
            replace_next = line == "event_state 20";
        }
        rewritten
    };
    let nan_error = TransientCheckpoint::from_text(&rewrite_first_event_value("NaN"))
        .expect_err("format-26 generated event state must reject NaN");
    assert!(
        nan_error.contains("NaN event state"),
        "NaN rejection identifies generated event state: {nan_error}"
    );
    TransientCheckpoint::from_text(&rewrite_first_event_value("inf"))
        .expect("format-26 event state permits infinity like runtime Verilog-A");

    let mut legacy_v25 = String::with_capacity(checkpoint_text.len());
    let mut skip_event_values = 0usize;
    for (index, line) in checkpoint_text.lines().enumerate() {
        if skip_event_values > 0 {
            skip_event_values -= 1;
            continue;
        }
        if line == "event_state 20" {
            skip_event_values = 20;
            continue;
        }
        let line = if index == 0 {
            "RSPICE-CHECKPOINT 25".to_string()
        } else if line.starts_with("generated_veriloga_state ") {
            let (prefix, _) = line
                .rsplit_once(' ')
                .expect("generated state header carries its version");
            format!("{prefix} 3")
        } else {
            line.to_string()
        };
        legacy_v25.push_str(&line);
        legacy_v25.push('\n');
    }
    let upgraded_v25 = TransientCheckpoint::from_text(&legacy_v25)
        .expect("v25 generated payload remains parseable without event-state rows");
    let upgraded_text = upgraded_v25.to_text();
    assert!(upgraded_text.contains("generated_veriloga_state_available 0\n"));
    assert!(upgraded_text.contains("generated_veriloga_states 0\n"));
    let legacy_error = engine
        .run_tran_resume(&netlist, &upgraded_v25, STOP, STEP)
        .expect_err("v25 cannot authoritatively resume a generated event-state model");
    assert!(
        legacy_error
            .to_string()
            .contains("does not contain generated Verilog-A persistent state"),
        "v25 resume fails closed with a precise diagnostic: {legacy_error}"
    );

    let serialized = TransientCheckpoint::from_text(&checkpoint_text)
        .expect("format-26 VBIC checkpoint round-trips");
    let (_, resumed_checkpoint) = engine
        .run_tran_resume(&netlist, &serialized, STOP, STEP)
        .unwrap_or_else(|error| {
            panic!(
                "generated VBIC resumes with accepted origin event state {accepted_event_state:?}: {error}"
            )
        });
    let resumed_text = resumed_checkpoint.to_text();
    let mut resumed_lines = resumed_text.lines();
    assert_eq!(
        resumed_lines.find(|line| line.starts_with("event_state ")),
        Some("event_state 20")
    );
    let resumed_event_state = resumed_lines
        .take(20)
        .map(|line| {
            line.parse::<f64>()
                .expect("numeric resumed VBIC event state")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        resumed_event_state, accepted_event_state,
        "resume must preserve accepted initial_step variables exactly"
    );
}

/// A checkpoint taken under the CFG plan resumes the same trajectory, every
/// point of it, for a module whose declared variables are not all live.
///
/// The runtime Verilog-A payload carries the whole variable array by value, and
/// under the CFG plan an evaluation publishes only the slots that plan reads.
/// `unread` here is a declared name no contribution, condition or Jacobian
/// loads. Capture does not run the observation pass that would fill it — it
/// takes `&self` and so cannot — which is deliberate: a ten-second compile at
/// the first checkpoint of a large deck is not acceptable, and a value derived
/// after the fact is not accepted state. What the payload holds for such a slot
/// is what the evaluation left, and this is the pin that says a resume does not
/// depend on it. `stored` is the opposite case in the same module: a variable
/// the residual reads, which the reactive history depends on.
///
/// Compared the way this suite compares a resume — the seam bit for bit, the
/// segment against the unsegmented run — because a resumed adaptive run derives
/// its own step sequence and lands on different points. The tolerance is two
/// orders tighter than `segmented_run_continues_the_unsegmented_trajectory`'s
/// and the failure it guards is not subtle: `stored` is the module's entire
/// conductance, so a payload that lost it moves the node by volts.
#[cfg(feature = "veriloga")]
#[test]
fn runtime_veriloga_checkpoint_resumes_a_partly_live_variable_array_exactly() {
    use std::io::Write;

    let mut model = std::env::temp_dir();
    model.push(format!(
        "rspice_checkpoint_partial_liveness_{}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&model).expect("create model file");
    file.write_all(
        br#"
`include "disciplines.vams"
module va_partial_liveness(p, n);
    inout p, n;
    electrical p, n;
    parameter real cap = 1.0e-9;
    parameter real res = 1.0e3;
    real stored, unread;
    analog begin
        stored = V(p, n) / res;
        unread = stored * stored + 1.0;
        I(p, n) <+ stored + ddt(cap * V(p, n));
    end
endmodule
"#,
    )
    .expect("write model");

    let deck = format!(
        "* runtime Verilog-A checkpoint with a partly live variable array\n\
         vin in 0 sin(0 1 1meg)\n\
         rsrc in out 1k\n\
         x1 out 0 va_partial_liveness\n\
         .va \"{}\" va_partial_liveness\n\
         .tran 1n 2u\n\
         .end\n",
        model.display().to_string().replace('\\', "/")
    );
    let netlist = Netlist::parse(&deck).expect("parse partial-liveness deck");
    let engine = Engine::new(SimulationConfig::default());

    let continuous = engine
        .run_tran(&netlist, 2.0e-6, 1.0e-9)
        .expect("continuous partial-liveness run");
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1.0e-6, 1.0e-9)
        .expect("partial-liveness checkpoint segment solves");
    let serialized = TransientCheckpoint::from_text(&checkpoint.to_text())
        .expect("runtime Verilog-A state survives portable text");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &serialized, 2.0e-6, 1.0e-9)
        .expect("partial-liveness checkpoint resumes");

    let out = |result: &rspice_core::engine::TransientResult| -> usize {
        result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("out node is in the solved trajectory")
    };

    // The seam carries the whole solution across the capture, bit for bit.
    assert_eq!(
        first.voltages[out(&first)]
            .last()
            .expect("captured endpoint")
            .to_bits(),
        resumed.voltages[out(&resumed)][0].to_bits(),
        "the resumed run must start on the captured solution exactly"
    );

    // And the resumed segment stays on the unsegmented trajectory. A variable
    // the residual reads but the payload failed to carry would not perturb this
    // at the integrator's tolerance — `stored` is the whole conductance, so
    // losing it moves the node by volts.
    let continuous_out = out(&continuous);
    let resumed_out = out(&resumed);
    let mut worst = 0.0_f64;
    for step in 1..=40 {
        let time = 1.0e-6 + f64::from(step) * 24.0e-9;
        let expected = interpolate(&continuous.time, &continuous.voltages[continuous_out], time);
        let actual = interpolate(&resumed.time, &resumed.voltages[resumed_out], time);
        worst = worst.max((expected - actual).abs());
    }
    assert!(
        worst < 1.0e-5,
        "the resumed segment must track the unsegmented run (worst |Δ| = {worst})"
    );

    let _ = std::fs::remove_file(&model);
}

// ---------------------------------------------------------------------------
// A locked grid across a checkpoint seam.
//
// The checkpoint is cut at the literal stop time, and a grid is built from an
// expression: `1000.0 * 1e-9` is one ulp above `1.0e-6`, and an adaptive run's
// accepted times straddle it. The seam is therefore off the grid in both of the
// constructions a caller reaches for, which is where the resumed run's first
// interval stops being a step and starts being a rounding artifact.
// ---------------------------------------------------------------------------

/// The worst deviation of a resumed segment from the trajectory the
/// unsegmented run of the same configuration followed.
fn worst_resumed_deviation(
    continuous: &rspice_core::engine::TransientResult,
    resumed: &rspice_core::engine::TransientResult,
) -> f64 {
    let continuous_out = out_index(continuous);
    let resumed_out = out_index(resumed);
    resumed
        .time
        .iter()
        .enumerate()
        .map(|(index, &time)| {
            let expected =
                interpolate(&continuous.time, &continuous.voltages[continuous_out], time);
            (resumed.voltages[resumed_out][index] - expected).abs()
        })
        .fold(0.0_f64, f64::max)
}

/// ngspice derives `delmin` from the maximum step as `1e-11 * max_step`, and
/// that is the hard minimum the controller applies to every step it selects
/// for itself. At this suite's nanosecond ceiling it is 1e-20 s.
const NANOSECOND_CEILING_HARD_MIN_DT: f64 = 1.0e-9 * 1.0e-11;

#[test]
fn locked_grid_resume_folds_a_seam_adjacent_target_into_the_seam() {
    const STEP: f64 = 1.0e-9;
    const SPLIT: f64 = 1.0e-6;
    const STOP: f64 = 2.0e-6;

    let grid: Vec<f64> = (0..=2000).map(|index| index as f64 * STEP).collect();
    assert_ne!(
        grid[1000].to_bits(),
        SPLIT.to_bits(),
        "the premise: this grid's own 1 us point is one ulp above the literal stop time"
    );

    let netlist = Netlist::parse(DECK).expect("checkpoint bench parses");
    let engine = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..Default::default()
    });

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, STEP)
        .expect("locked first segment completes");
    assert_eq!(
        checkpoint.time.to_bits(),
        SPLIT.to_bits(),
        "the seam is the literal stop time, which is what puts it off this grid"
    );
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, STOP, STEP)
        .expect("locked resume completes");

    // The seam plus the thousand grid points above it -- not the thousand and
    // one that counting a one-ulp neighbour as a target of its own produces.
    assert_eq!(
        resumed.time.len(),
        1001,
        "the resumed run is the seam plus every grid point above it"
    );
    assert_eq!(
        resumed.time[1].to_bits(),
        grid[1001].to_bits(),
        "the first target after the seam is the next real grid point"
    );
    assert!(
        resumed.step_sizes[1] > 0.5e-9,
        "the first resumed interval is a step, not a rounding artifact: {}",
        resumed.step_sizes[1]
    );
    for (index, &step) in resumed.step_sizes.iter().enumerate().skip(1) {
        assert!(
            step >= NANOSECOND_CEILING_HARD_MIN_DT,
            "accepted step {index} of {step} is below the solver hard minimum {NANOSECOND_CEILING_HARD_MIN_DT}"
        );
    }
}

#[cfg(feature = "veriloga")]
#[test]
fn runtime_veriloga_locked_grid_resume_holds_the_unsegmented_trajectory() {
    use std::io::Write;

    const STEP: f64 = 1.0e-9;
    const SPLIT: f64 = 1.0e-6;
    const STOP: f64 = 2.0e-6;

    let mut model = std::env::temp_dir();
    model.push(format!("rspice_locked_grid_seam_{}.va", std::process::id()));
    let mut file = std::fs::File::create(&model).expect("create model file");
    file.write_all(
        br#"
`include "disciplines.vams"
module va_partial_liveness(p, n);
    inout p, n;
    electrical p, n;
    parameter real cap = 1.0e-9;
    parameter real res = 1.0e3;
    real stored, unread;
    analog begin
        stored = V(p, n) / res;
        unread = stored * stored + 1.0;
        I(p, n) <+ stored + ddt(cap * V(p, n));
    end
endmodule
"#,
    )
    .expect("write model");

    let deck = format!(
        "* runtime Verilog-A under a locked grid across a checkpoint seam\n\
         vin in 0 sin(0 1 1meg)\n\
         rsrc in out 1k\n\
         x1 out 0 va_partial_liveness\n\
         .va \"{}\" va_partial_liveness\n\
         .tran 1n 2u\n\
         .end\n",
        model.display().to_string().replace('\\', "/")
    );
    let netlist = Netlist::parse(&deck).expect("parse locked-grid seam deck");
    let engine = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(
            (0..=2000).map(|index| index as f64 * STEP).collect(),
        )),
        ..Default::default()
    });

    let continuous = engine
        .run_tran(&netlist, STOP, STEP)
        .expect("unsegmented locked run completes");
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, STEP)
        .expect("locked first segment completes");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, STOP, STEP)
        .expect("locked resume completes");

    // A `ddt` term dropped for one step is not a small error. The fixture is
    // then a bare 1 kohm at a source zero crossing, so `v(out)` restarts from
    // zero and the whole segment is displaced by an eighth of a volt.
    let worst = worst_resumed_deviation(&continuous, &resumed);
    assert!(
        worst < 1.0e-5,
        "the resumed segment must stay on the unsegmented trajectory (worst |dv| = {worst:.6e})"
    );

    let _ = std::fs::remove_file(&model);
}

#[test]
fn a_locked_target_inside_the_hard_minimum_folds_into_the_accepted_point() {
    // A microsecond ceiling puts the solver hard minimum at 1e-17 s. The
    // crowded point below sits above the normalizer's duplicate margin (64
    // ulps of 1 us, about 1.4e-20) and below that minimum, so only a floor on
    // the locked step itself can reject it.
    const MAX_STEP: f64 = 1.0e-6;
    const STOP: f64 = 2.0e-6;
    const HARD_MIN_DT: f64 = MAX_STEP * 1.0e-11;

    let mut grid: Vec<f64> = (0..=20).map(|index| index as f64 * 1.0e-7).collect();
    let crowded = grid[10] + 1.0e-18;
    assert!(crowded > grid[10] && crowded - grid[10] < HARD_MIN_DT);
    grid.insert(11, crowded);

    let netlist = Netlist::parse(DECK).expect("checkpoint bench parses");
    let result = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..Default::default()
    })
    .run_tran(&netlist, STOP, MAX_STEP)
    .expect("locked run over a crowded grid completes");

    assert!(
        result
            .time
            .iter()
            .all(|time| time.to_bits() != crowded.to_bits()),
        "a target the clock cannot separate from the accepted point is not a step of its own"
    );
    for (index, &step) in result.step_sizes.iter().enumerate().skip(1) {
        assert!(
            step >= HARD_MIN_DT,
            "accepted step {index} of {step} is below the solver hard minimum {HARD_MIN_DT}"
        );
    }
}

#[cfg(feature = "veriloga")]
#[test]
fn runtime_veriloga_refuses_a_locked_step_below_the_integration_floor() {
    use std::io::Write;

    // At picosecond times the solver hard minimum is 1e-23 s, so a grid may
    // legitimately prescribe a 1e-21 s step and the engine will try to take
    // it. The companion rule cannot build `1/dt` from that interval. Silently
    // returning inactive coefficients would evaluate the module with no `ddt`
    // term at all -- a charge-storing device abruptly becoming a resistor,
    // converged and plausible and wrong.
    const MAX_STEP: f64 = 1.0e-12;
    const STOP: f64 = 3.0e-12;

    let mut model = std::env::temp_dir();
    model.push(format!(
        "rspice_locked_grid_floor_{}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&model).expect("create model file");
    file.write_all(
        br#"
`include "disciplines.vams"
module va_reactive(p, n);
    inout p, n;
    electrical p, n;
    parameter real cap = 1.0e-9;
    parameter real res = 1.0e3;
    analog I(p, n) <+ V(p, n) / res + ddt(cap * V(p, n));
endmodule
"#,
    )
    .expect("write model");

    let deck = format!(
        "* runtime Verilog-A under a locked grid finer than the companion rule\n\
         vin in 0 sin(0 1 1meg)\n\
         rsrc in out 1k\n\
         x1 out 0 va_reactive\n\
         .va \"{}\" va_reactive\n\
         .tran 1p 3p\n\
         .end\n",
        model.display().to_string().replace('\\', "/")
    );
    let crowded = 1.0e-12 + 1.0e-21;
    let sub_floor_dt = crowded - 1.0e-12;
    let netlist = Netlist::parse(&deck).expect("parse sub-floor deck");
    let error = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(vec![1.0e-12, crowded, 2.0e-12, 3.0e-12])),
        ..Default::default()
    })
    .run_tran(&netlist, STOP, MAX_STEP)
    .expect_err("a step the companion rule cannot integrate must be refused");

    let text = error.to_string();
    assert!(
        text.contains("Verilog-A devices cannot advance")
            && text.contains(&format!("{sub_floor_dt:.16e}"))
            && text.contains(&format!("needs at least {:.16e}s", 1.0e-20)),
        "the refusal must name the timestep ({sub_floor_dt:.16e}) and the floor, got: {text}"
    );

    let _ = std::fs::remove_file(&model);
}

#[test]
fn locked_paired_schedule_resume_integrates_the_seam_interval() {
    const STEP: f64 = 1.0e-9;
    const SPLIT: f64 = 1.0e-6;
    const STOP: f64 = 2.0e-6;

    let netlist = Netlist::parse(DECK).expect("checkpoint bench parses");
    // The documented use of the paired form: replay a reference run's accepted
    // times *and* the intervals it chose to reach them.
    let reference = Engine::default()
        .run_tran(&netlist, STOP, STEP)
        .expect("adaptive reference run completes");
    assert!(
        !reference
            .time
            .iter()
            .any(|time| time.to_bits() == SPLIT.to_bits()),
        "the premise: an adaptive run straddles the literal stop time"
    );
    let first_above_seam = reference
        .time
        .iter()
        .copied()
        .find(|&time| time > SPLIT)
        .expect("the reference run continues past the seam");

    let engine = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(reference.time.clone())),
        locked_time_step_sizes: Some(Arc::new(reference.step_sizes.clone())),
        ..Default::default()
    });
    let continuous = engine
        .run_tran(&netlist, STOP, STEP)
        .expect("unsegmented replay completes");
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, STEP)
        .expect("replayed first segment completes");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, STOP, STEP)
        .expect("replayed resume completes");

    // The recorded interval reached that target from the reference run's own
    // previous point, which is not where this run stands. Using it anyway
    // integrates over a wider interval than the gap and then relabels the
    // result to the target's time: the recorded times look perfect and the
    // solution is wrong by millivolts, for the rest of the segment.
    let gap = first_above_seam - SPLIT;
    assert!(
        (resumed.step_sizes[1] - gap).abs() <= 1.0e-18,
        "the first resumed interval must be the gap from the seam ({gap:.6e}), got {:.6e}",
        resumed.step_sizes[1]
    );
    let worst = worst_resumed_deviation(&continuous, &resumed);
    assert!(
        worst < 1.0e-4,
        "the resumed segment must stay on the unsegmented trajectory (worst |dv| = {worst:.6e})"
    );
}
