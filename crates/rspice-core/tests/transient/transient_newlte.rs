//! End-to-end Xyce NEWLTE transient-control contracts.

use std::sync::Arc;

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::numerics::integration::TransientLteReference;

fn rc_deck(options: &str) -> String {
    format!(
        "\
* adaptive NEWLTE RC contract
vzero in 0 0
r1 in out 1k
c1 out 0 1u ic=1
{options}
.tran 10u 5m uic
.end
"
    )
}

fn xyce_engine() -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::Trapezoidal,
        ..Default::default()
    })
}

#[test]
fn bjt_auxiliary_lte_domain_survives_checkpoint_encoding_and_rejects_mismatch() {
    use rspice_core::engine::{
        TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
    };

    for mode in 0..4 {
        let netlist = Netlist::parse(&format!(
            "BJT auxiliary domain continuation\n\
             VCC supply 0 5\nVIN drive 0 PULSE(0 1 1u .1u .1u 3u 10u)\n\
             RIN drive base 1k\nRLOAD supply collector 470\nCLOAD collector 0 1n\n\
             Q1 collector base 0 QB\n\
             .model QB NPN(IS=1e-14 BF=200 RB=10 IRB=.001 RBM=1 CJE=20p CJC=7p)\n\
             .options timeint NEWLTE={mode}\n.tran 0 8u\n.end\n"
        ))
        .unwrap();
        let engine = xyce_engine();
        let (full, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                8e-6,
                2e-7,
                TransientStartupMode::OperatingPoint,
                &[2.37e-6],
            )
            .unwrap();
        let source = &scheduled[0].checkpoint;
        let offset = full
            .time
            .iter()
            .position(|time| time.to_bits() == source.time.to_bits())
            .unwrap();
        for encoding in [
            TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Packed,
        ] {
            let checkpoint =
                TransientCheckpoint::from_bytes(&source.to_bytes(encoding).unwrap()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &checkpoint, 8e-6, 2e-7)
                .unwrap();
            assert_bit_exact(&resumed.time, &full.time[offset..], "BJT accepted times");
            assert_bit_exact(
                &resumed.step_sizes[1..],
                &full.step_sizes[offset + 1..],
                "BJT accepted steps",
            );
            for (actual, expected) in resumed
                .voltages
                .iter()
                .chain(&resumed.branch_currents)
                .zip(full.voltages.iter().chain(&full.branch_currents))
            {
                assert_bit_exact(actual, &expected[offset..], "BJT complete solution");
            }
        }
        let text = String::from_utf8(
            source
                .to_bytes(TransientCheckpointEncoding::Unpacked)
                .unwrap(),
        )
        .unwrap();
        let header = text
            .lines()
            .find(|line| line.starts_with("accepted_integration_lte "))
            .unwrap();
        let fields = header.split_whitespace().collect::<Vec<_>>();
        assert_eq!(fields[1], "2");
        assert_eq!(
            fields[15], "1",
            "the owner must declare exactly one RBI auxiliary"
        );
        let prefix = fields[..15].join(" ");
        for suffix in [
            format!("2 {0} {0}", fields[16]),
            "1 999999".to_string(),
            "9999999999999999999".to_string(),
        ] {
            assert!(
                TransientCheckpoint::from_bytes(
                    text.replace(header, &format!("{prefix} {suffix}"))
                        .as_bytes()
                )
                .is_err()
            );
        }
        for legacy in [false, true] {
            let mut changed = fields[..15]
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<_>>();
            if legacy {
                changed[1] = "1".to_string();
            } else {
                changed.push("0".to_string());
            }
            let changed = TransientCheckpoint::from_bytes(
                text.replace(header, &changed.join(" ")).as_bytes(),
            )
            .unwrap();
            let error = engine
                .run_tran_resume(&netlist, &changed, 8e-6, 2e-7)
                .unwrap_err();
            assert!(error.to_string().contains("auxiliary domain"), "{error}");
        }
    }
}

#[test]
fn legacy_lte_record_without_auxiliaries_preserves_exact_continuation() {
    use rspice_core::engine::{
        TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
    };
    let netlist = Netlist::parse(&rc_deck(".options timeint NEWLTE=2")).unwrap();
    let engine = xyce_engine();
    let (full, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            5e-3,
            1e-4,
            TransientStartupMode::Uic,
            &[1.37e-3],
        )
        .unwrap();
    let source = &scheduled[0].checkpoint;
    let text = String::from_utf8(
        source
            .to_bytes(TransientCheckpointEncoding::Unpacked)
            .unwrap(),
    )
    .unwrap();
    let header = text
        .lines()
        .find(|line| line.starts_with("accepted_integration_lte "))
        .unwrap();
    let mut fields = header.split_whitespace().collect::<Vec<_>>();
    assert_eq!(fields.pop(), Some("0"));
    fields[1] = "1";
    let legacy =
        TransientCheckpoint::from_bytes(text.replace(header, &fields.join(" ")).as_bytes())
            .unwrap();
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &legacy, 5e-3, 1e-4)
        .unwrap();
    let offset = full
        .time
        .iter()
        .position(|time| time.to_bits() == source.time.to_bits())
        .unwrap();
    assert_bit_exact(&resumed.time, &full.time[offset..], "legacy accepted times");
    assert_bit_exact(
        out_trace(&resumed),
        &out_trace(&full)[offset..],
        "legacy RC trajectory",
    );
}

fn out_trace(result: &rspice_core::engine::TransientResult) -> &[f64] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node is present");
    &result.voltages[index]
}

fn assert_bit_exact(left: &[f64], right: &[f64], label: &str) {
    assert_eq!(left.len(), right.len(), "{label} lengths differ");
    assert!(
        left.iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits()),
        "{label} values differ"
    );
}

#[test]
fn omitted_xyce_newlte_matches_explicit_point_global_mode() {
    let omitted = Netlist::parse(&rc_deck(".options timeint reltol=1e-4 abstol=1e-6"))
        .expect("omitted NEWLTE deck parses");
    let explicit = Netlist::parse(&rc_deck(
        ".options timeint reltol=1e-4 abstol=1e-6 newlte=1",
    ))
    .expect("explicit NEWLTE=1 deck parses");

    let omitted_result = xyce_engine()
        .run_tran(&omitted, 5.0e-3, 1.0e-3)
        .expect("omitted NEWLTE run completes");
    let explicit_result = xyce_engine()
        .run_tran(&explicit, 5.0e-3, 1.0e-3)
        .expect("explicit NEWLTE=1 run completes");

    assert_bit_exact(
        &omitted_result.time,
        &explicit_result.time,
        "omitted/explicit NEWLTE time grid",
    );
    assert_bit_exact(
        out_trace(&omitted_result),
        out_trace(&explicit_result),
        "omitted/explicit NEWLTE waveform",
    );
}

#[test]
fn tighter_timeint_tolerances_materially_refine_the_adaptive_grid() {
    let loose = Netlist::parse(&rc_deck(
        ".options timeint reltol=1e-2 abstol=1e-4 newlte=2",
    ))
    .expect("loose NEWLTE deck parses");
    let tight = Netlist::parse(&rc_deck(
        ".options timeint reltol=1e-5 abstol=1e-7 newlte=2",
    ))
    .expect("tight NEWLTE deck parses");

    let loose_result = xyce_engine()
        .run_tran(&loose, 5.0e-3, 1.0e-3)
        .expect("loose NEWLTE run completes");
    let tight_result = xyce_engine()
        .run_tran(&tight, 5.0e-3, 1.0e-3)
        .expect("tight NEWLTE run completes");

    assert!(
        tight_result.time.len() > loose_result.time.len(),
        "tight TIMEINT tolerances must add adaptive points (tight={}, loose={})",
        tight_result.time.len(),
        loose_result.time.len()
    );

    let max_error = |result: &rspice_core::engine::TransientResult| {
        result
            .time
            .iter()
            .zip(out_trace(result))
            .map(|(time, actual)| (actual - (-time / 1.0e-3).exp()).abs())
            .fold(0.0_f64, f64::max)
    };
    assert!(
        max_error(&tight_result) <= max_error(&loose_result),
        "tight TIMEINT tolerances must not increase the RC analytic error"
    );
}

#[test]
fn locked_grid_times_are_invariant_to_newlte_mode_and_tolerances() {
    let grid = Arc::new(vec![0.0, 10.0e-6, 30.0e-6, 100.0e-6, 300.0e-6, 1.0e-3]);
    let run = |options: &str| {
        let netlist = Netlist::parse(&rc_deck(options)).expect("locked-grid deck parses");
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            integration_method: IntegrationMethod::Trapezoidal,
            locked_time_grid: Some(Arc::clone(&grid)),
            ..Default::default()
        })
        .run_tran(&netlist, 1.0e-3, 1.0e-3)
        .expect("locked-grid NEWLTE run completes")
    };

    let point_local = run(".options timeint reltol=1e-2 abstol=1e-4 newlte=0");
    let signal_local = run(".options timeint reltol=1e-8 abstol=1e-12 newlte=3");

    assert_bit_exact(&point_local.time, grid.as_slice(), "locked reference grid");
    assert_bit_exact(
        &point_local.time,
        &signal_local.time,
        "locked NEWLTE time grids",
    );
    assert!(
        out_trace(&point_local)
            .iter()
            .all(|value| value.is_finite())
    );
    assert!(
        out_trace(&signal_local)
            .iter()
            .all(|value| value.is_finite())
    );
}

#[test]
fn signal_global_history_relaxes_late_decay_relative_to_point_global() {
    let point = Netlist::parse(&rc_deck(
        ".options timeint reltol=1e-4 abstol=1e-8 newlte=1",
    ))
    .expect("point-global deck parses");
    let signal = Netlist::parse(&rc_deck(
        ".options timeint reltol=1e-4 abstol=1e-8 newlte=2",
    ))
    .expect("signal-global deck parses");

    let point_result = xyce_engine()
        .run_tran(&point, 5.0e-3, 1.0e-3)
        .expect("point-global run completes");
    let signal_result = xyce_engine()
        .run_tran(&signal, 5.0e-3, 1.0e-3)
        .expect("signal-global run completes");
    let late_points = |result: &rspice_core::engine::TransientResult| {
        result.time.iter().filter(|time| **time >= 3.0e-3).count()
    };

    assert!(
        late_points(&point_result) > late_points(&signal_result),
        "point-global weighting must refine the late decay after its reference shrinks"
    );
}

#[test]
fn native_omitted_mode_matches_explicit_predictor_local_policy() {
    let netlist = Netlist::parse(&rc_deck("")).expect("native RC deck parses");
    for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
        let implicit = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            ..Default::default()
        })
        .run_tran(&netlist, 1.0e-3, 100.0e-6)
        .expect("implicit native LTE run completes");
        let explicit = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            transient_lte_reference: Some(TransientLteReference::PredictorLocal),
            ..Default::default()
        })
        .run_tran(&netlist, 1.0e-3, 100.0e-6)
        .expect("explicit native LTE run completes");

        assert_bit_exact(&implicit.time, &explicit.time, "native LTE time grid");
        assert_bit_exact(
            out_trace(&implicit),
            out_trace(&explicit),
            "native LTE waveform",
        );
    }
}

#[test]
fn fixed_gear2_keeps_native_order_two_and_xyce_starts_at_order_one() {
    let netlist = Netlist::parse(&rc_deck("")).expect("Gear startup deck parses");
    let run = |dialect| {
        Engine::new(SimulationConfig {
            spice_dialect: dialect,
            integration_method: IntegrationMethod::Gear2,
            transient_initial_timestep: Some(1.0e-3),
            ..Default::default()
        })
        .run_tran(&netlist, 1.0e-3, 1.0e-3)
        .expect("fixed Gear startup run completes")
    };

    let native = run(SpiceDialect::BestAvailable);
    let xyce = run(SpiceDialect::Xyce);
    let native_first = out_trace(&native)[1];
    let xyce_first = out_trace(&xyce)[1];
    let xyce_first_dt = xyce.time[1] - xyce.time[0];
    let xyce_bdf1_expected = 1.0 / (1.0 + xyce_first_dt / 1.0e-3);

    assert!(
        (native_first - 0.6).abs() <= 1.0e-12,
        "native Gear2 must stamp BDF2, got {native_first:.16e}"
    );
    assert!(
        (xyce_first - xyce_bdf1_expected).abs() <= 1.0e-12,
        "Xyce Gear12 must start with BDF1 at its restart-bounded first step: expected {xyce_bdf1_expected:.16e}, got {xyce_first:.16e}"
    );
}

#[test]
fn xyce_uic_gear12_promotes_on_the_third_locked_step() {
    let netlist = Netlist::parse(&rc_deck(".options timeint reltol=1 abstol=1 newlte=1"))
        .expect("UIC Gear12 deck parses");
    let step = 100.0e-6;
    let grid = Arc::new(vec![0.0, step, 2.0 * step, 3.0 * step]);
    let result = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        integration_method: IntegrationMethod::Gear2,
        transient_initial_timestep: Some(step),
        locked_time_grid: Some(grid),
        ..Default::default()
    })
    .run_tran(&netlist, 3.0 * step, step)
    .expect("UIC Gear12 locked-grid run completes");

    let alpha = step / 1.0e-3;
    let backward_euler = |previous: f64| previous / (1.0 + alpha);
    let first = backward_euler(1.0);
    let second = backward_euler(first);
    let third_bdf2 = (2.0 * second - 0.5 * first) / (1.5 + alpha);
    let trace = out_trace(&result);

    assert!((trace[1] - first).abs() <= 1.0e-12);
    assert!((trace[2] - second).abs() <= 1.0e-12);
    assert!(
        (trace[3] - third_bdf2).abs() <= 1.0e-12,
        "Xyce UIC Gear12 must promote after two accepted order-one steps"
    );
}

#[test]
fn locked_xyce_dialect_predictor_local_gear2_remains_native_bdf2() {
    let netlist = Netlist::parse(&rc_deck("")).expect("mixed-policy Gear2 deck parses");
    let step = 1.0e-3;
    let result = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        transient_lte_reference: Some(TransientLteReference::PredictorLocal),
        integration_method: IntegrationMethod::Gear2,
        transient_initial_timestep: Some(step),
        locked_time_grid: Some(Arc::new(vec![0.0, step])),
        ..Default::default()
    })
    .run_tran(&netlist, step, step)
    .expect("mixed-policy locked Gear2 run completes");

    assert!(
        (out_trace(&result)[1] - 0.6).abs() <= 1.0e-12,
        "explicit PredictorLocal must retain native fixed-BDF2 semantics"
    );
}

#[test]
fn first_step_after_xyce_breakpoint_is_lte_controlled() {
    let deck = |reltol: &str, abstol: &str| {
        format!(
            "\
* breakpoint NEWLTE restart contract
vin in 0 pulse(0 1 100n 1n 1n 100n 300n)
r1 in out 1k
c1 out 0 10p
.options timeint reltol={reltol} abstol={abstol} newlte=1
.tran 1n 150n
.end
"
        )
    };
    let run = |reltol, abstol| {
        let netlist = Netlist::parse(&deck(reltol, abstol)).expect("breakpoint deck parses");
        xyce_engine()
            .run_tran(&netlist, 150.0e-9, 20.0e-9)
            .expect("breakpoint NEWLTE run completes")
    };
    let loose = run("1e-2", "1e-4");
    let tight = run("1e-7", "1e-10");
    let first_two_post_edge_steps = |result: &rspice_core::engine::TransientResult| {
        let edge = result
            .time
            .iter()
            .position(|time| (*time - 101.0e-9).abs() <= 1.0e-21)
            .expect("run lands on the completed rising-edge breakpoint");
        (
            result.time[edge + 1] - result.time[edge],
            result.time[edge + 2] - result.time[edge + 1],
        )
    };

    let (loose_first, loose_second) = first_two_post_edge_steps(&loose);
    let (tight_first, tight_second) = first_two_post_edge_steps(&tight);

    assert!(
        (loose_second / loose_first - 2.0).abs() <= 1.0e-8,
        "loose LTE control should permit Xyce's bounded 2x growth (first={loose_first:e}, second={loose_second:e})"
    );
    assert!(
        tight_second <= tight_first,
        "tight LTE control must not grow immediately after the first controlled post-breakpoint step; equality is valid when that first step already reached the precision floor (first={tight_first:e}, second={tight_second:e})"
    );
    assert!(
        tight_first < loose_first,
        "tight TIMEINT tolerances must refine the breakpoint restart trajectory"
    );
}

#[test]
fn newbpstepping_zero_bypasses_first_post_breakpoint_lte_rejection() {
    let deck = |new_bp_stepping: u8| {
        format!(
            "\
* legacy breakpoint acceptance contract
vin in 0 pulse(0 1 100n 1n 1n 100n 300n)
r1 in out 1k
c1 out 0 10p
.options timeint reltol=1e-7 abstol=1e-10 newlte=1 newbpstepping={new_bp_stepping}
.tran 1n 150n
.end
"
        )
    };
    let run = |new_bp_stepping| {
        let netlist =
            Netlist::parse(&deck(new_bp_stepping)).expect("NEWBPSTEPPING breakpoint deck parses");
        xyce_engine()
            .run_tran(&netlist, 150.0e-9, 20.0e-9)
            .expect("NEWBPSTEPPING breakpoint run completes")
    };
    let legacy = run(0);
    let modern = run(1);
    let first_post_edge_step = |result: &rspice_core::engine::TransientResult| {
        let edge = result
            .time
            .iter()
            .position(|time| (*time - 101.0e-9).abs() <= 1.0e-21)
            .expect("run lands on the completed rising-edge breakpoint");
        result.time[edge + 1] - result.time[edge]
    };

    let legacy_first = first_post_edge_step(&legacy);
    let modern_first = first_post_edge_step(&modern);
    assert!(
        legacy_first > modern_first,
        "NEWBPSTEPPING=0 must accept the first Newton-converged restart step without the LTE shrink used by NEWBPSTEPPING=1 (legacy={legacy_first:e}, modern={modern_first:e})"
    );
}
