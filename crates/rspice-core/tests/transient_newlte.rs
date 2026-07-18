//! End-to-end Xyce NEWLTE transient-control contracts.

use std::sync::Arc;

use rspice_core::analysis::IntegrationMethod;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, TransientLteReference};

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
