//! Public transient convergence contract: rejected Newton iterates must never
//! be committed as waveform points.

use std::sync::Arc;

use rspice_core::ConvergenceConfig;
use rspice_core::engine::{Engine, SimulationConfig, SimulationError, SpiceDialect};
use rspice_core::netlist::Netlist;

#[test]
fn active_current_sources_can_cancel_without_any_voltage_motion() {
    let netlist = Netlist::parse(
        "canceling sources\nI1 0 out PWL(0 0 1n 1 2n 1 3n 0)\nI2 out 0 PWL(0 0 1n 1 2n 1 3n 0)\nR1 out 0 1\n.end\n"
    ).unwrap();
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let result = engine.run_tran(&netlist, 4e-9, 1e-9).unwrap();
        assert_eq!(result.time.last().copied(), Some(4e-9));
        assert!(
            result
                .try_voltage_waveform_named("out")
                .unwrap()
                .iter()
                .all(|value| value.abs() < 1e-14)
        );
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}

#[test]
fn transient_current_values_do_not_cancel_against_their_dc_specification() {
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        for load in [
            "",
            "D1 0 out dm\n.model dm D(IS=1e-14)",
            "M1 out 0 0 0 mm\n.model mm NMOS(LEVEL=1 VTO=1)",
        ] {
            let netlist = Netlist::parse(&format!(
                "independent DC and transient current\nI1 0 out DC 1e100 PWL(0 1 1n 2 2n 2)\nI2 0 out 3\nR1 out 0 1\n{load}\n.end\n"
            )).unwrap();
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let result = engine.run_tran(&netlist, 2e-9, 1e-9).unwrap();
            assert_eq!(result.time.last().copied(), Some(2e-9));
            for (time, actual) in result
                .time
                .iter()
                .zip(result.try_voltage_waveform_named("out").unwrap())
            {
                let expected = 4.0 + (*time / 1e-9).min(1.0);
                assert!(
                    (actual - expected).abs() < 1e-10,
                    "{dialect:?}, {load}, at {time}: {actual} versus {expected}"
                );
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}
#[test]
fn current_shunts_are_qualified_by_kcl_instead_of_a_source_to_voltage_ratio() {
    let netlist =
        Netlist::parse("current shunt\nI1 0 out PWL(0 0 1n 1 2n 1 3n 0)\nR1 out 0 1u\n.end\n")
            .unwrap();
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let result = engine.run_tran(&netlist, 4e-9, 1e-9).unwrap();
        assert_eq!(result.time.last().copied(), Some(4e-9));
        for (time, actual) in result
            .time
            .iter()
            .zip(result.try_voltage_waveform_named("out").unwrap())
        {
            let current = if *time < 1e-9 {
                *time / 1e-9
            } else if *time <= 2e-9 {
                1.0
            } else if *time < 3e-9 {
                (3e-9 - time) / 1e-9
            } else {
                0.0
            };
            assert!(
                (actual - current * 1e-6).abs() < 1e-14,
                "{dialect:?} at {time}: {actual}"
            );
        }
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}

#[test]
fn finite_linear_voltage_and_branch_states_are_not_clipped_to_global_rails() {
    let netlist = Netlist::parse(
        "scaled divider\nV1 in 0 PWL(0 0 1n 1e100 2n 1e100)\nR1 in out 1\nR2 out 0 1\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let result = engine.run_tran(&netlist, 2e-9, 1e-9).unwrap();
    assert_eq!(result.time.last().copied(), Some(2e-9));
    for (time, actual) in result
        .time
        .iter()
        .zip(result.try_voltage_waveform_named("out").unwrap())
    {
        let expected = 0.5 * (*time / 1e-9).min(1.0);
        assert!(
            (actual / 1e100 - expected).abs() < 1e-12,
            "at {time}: {actual}"
        );
    }
    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
}

#[test]
fn finite_nonlinear_voltage_and_branch_states_are_not_clipped_to_global_rails() {
    for (bias, resistance) in [(5000.0_f64, 1000.0), (-5000.0, 1000.0), (5.0, 1e-14)] {
        let diode_nodes = if bias > 0.0 { "0 out" } else { "out 0" };
        let netlist = Netlist::parse(&format!(
            "reverse diode divider\nV1 in 0 PWL(0 {bias} 1n {end} 2n {end})\nR1 in out {resistance}\nR2 out 0 {resistance}\nD1 {diode_nodes} dm\n.model dm D(IS=1e-14)\n.end\n",
            end = 2.0 * bias,
        )).unwrap();
        for dialect in [
            SpiceDialect::BestAvailable,
            SpiceDialect::Ngspice,
            SpiceDialect::Xyce,
        ] {
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let dc = engine
                .run_dc_op(&netlist)
                .expect("finite reverse-bias operating point");
            assert!((dc.try_voltage_named("out").unwrap() / bias - 0.5).abs() < 1e-8);
            let result = engine
                .run_tran(&netlist, 2e-9, 1e-9)
                .unwrap_or_else(|error| {
                    panic!("{dialect:?}, bias={bias}, R={resistance}: {error}")
                });
            assert_eq!(result.time.last().copied(), Some(2e-9));
            for (time, actual) in result
                .time
                .iter()
                .zip(result.try_voltage_waveform_named("out").unwrap())
            {
                let expected = 0.5 * (1.0 + (*time / 1e-9).min(1.0));
                assert!(
                    (actual / bias - expected).abs() < 1e-8,
                    "{dialect:?}, at {time}: {actual}"
                );
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn forward_diode_solution_is_invariant_under_large_common_mode_shifts() {
    let grid = Arc::new(vec![0.0, 0.5e-9, 1e-9, 1.5e-9, 2e-9]);
    let mut reference: Option<Vec<f64>> = None;
    for common_mode in [0.0, 5000.0, -5000.0] {
        let netlist = Netlist::parse(&format!(
            "translated diode\nV0 base 0 {common_mode}\nV1 in base PWL(0 1 1n 2 2n 2)\nR1 in out 1k\nD1 out base dm\n.model dm D(IS=1e-14)\n.end\n"
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            locked_time_grid: Some(grid.clone()),
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                ..Default::default()
            },
            ..Default::default()
        });
        let result = engine.run_tran(&netlist, 2e-9, 0.5e-9).unwrap();
        assert_eq!(result.time, *grid);
        let junction: Vec<_> = result
            .try_voltage_waveform_named("out")
            .unwrap()
            .iter()
            .map(|value| value - common_mode)
            .collect();
        if let Some(expected) = &reference {
            for (actual, expected) in junction.iter().zip(expected) {
                assert!(
                    (actual - expected).abs() < 1e-8,
                    "common mode {common_mode}: {actual} versus {expected}"
                );
            }
        } else {
            reference = Some(junction);
        }
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}

const UNSATISFIABLE_TRANSIENT: &str = "\
* No real transient operating point: v^2 + v + 1 = 0
 B1 n 0 I={V(n)*V(n)+1}
 R1 n 0 1
 .tran 1u 2u uic
 .end
";

fn assert_rejected_newton_iterate_is_not_published(dialect: SpiceDialect) {
    let netlist = Netlist::parse(UNSATISFIABLE_TRANSIENT).expect("contract deck parses");
    let mut config = SimulationConfig::default().with_spice_dialect(dialect);
    config.transient_max_iterations = 2;
    config.transient_nonlinear_max_iterations = Some(2);
    config.transient_initial_timestep = Some(1.0e-6);
    config.min_timestep = 1.0e-6;
    config.max_timestep = 1.0e-6;
    let engine = Engine::new(config);

    let error = engine
        .run_tran(&netlist, 2.0e-6, 1.0e-6)
        .expect_err("an equation with no real root must fail transient convergence");
    assert!(
        matches!(error, SimulationError::ConvergenceFailed(_)),
        "unexpected error for {dialect:?}: {error}"
    );

    let quality = engine.convergence_quality();
    assert_eq!(quality.force_accepted_points, 0, "dialect={dialect:?}");
    assert!(
        quality.force_accepted_indices.is_empty(),
        "dialect={dialect:?}"
    );
}

#[test]
fn every_dialect_rejects_nonconverged_newton_points() {
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        assert_rejected_newton_iterate_is_not_published(dialect);
    }
}

fn gmin_rescue_fixture() -> (Engine, Netlist) {
    let netlist = Netlist::parse(
        "cubic continuation rescue\n\
         .options gmin=0\n\
         I1 0 n PULSE(0 -2 0 1f 1f 10n 20n)\n\
         B1 n 0 I={V(n)*V(n)*V(n)-2*V(n)}\n\
         .tran 0 1f uic\n\
         .end\n",
    )
    .expect("cubic continuation fixture parses");
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        transient_nonlinear_max_iterations: Some(8),
        convergence_config: ConvergenceConfig {
            gmin_initial: 10.0,
            gmin_target: 1.0e-15,
            junction_gmin_target: 0.0,
            ..Default::default()
        },
        locked_time_grid: Some(Arc::new(vec![0.0, 1.0e-15])),
        ..Default::default()
    });

    (engine, netlist)
}

#[test]
fn gmin_rescue_observes_cancellation_inside_continuation() {
    let (engine, netlist) = gmin_rescue_fixture();
    // This two-point fixture previously completed with just 51 polls, all
    // outside rescue. Cancellation at poll 65 requires checks in the walk.
    let abort = rspice_core::abort_signal::CountingAbort::new(64);
    let error = engine
        .run_tran_with_abort(&netlist, 1.0e-15, 1.0e-15, &abort)
        .expect_err("cancel a running GMIN continuation");
    assert!(matches!(error, SimulationError::Aborted), "{error}");
    assert_eq!(abort.observed_at(), Some(65));
    assert_eq!(abort.polls_after_abort(), 0);
    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
}

#[test]
fn successful_gmin_rescue_is_not_counted_as_a_rejected_timestep() {
    let (engine, netlist) = gmin_rescue_fixture();
    let result = engine
        .run_tran(&netlist, 1.0e-15, 1.0e-15)
        .expect("GMIN continuation recovers the nonlinear endpoint");
    assert_eq!(
        result.time.last().expect("terminal sample").to_bits(),
        1.0e-15_f64.to_bits()
    );
    assert!(
        result
            .voltages
            .iter()
            .flatten()
            .all(|value| value.is_finite()),
        "the rescued physical solution is finite"
    );

    let voltage = *result
        .try_voltage_waveform_named("n")
        .unwrap()
        .last()
        .unwrap();
    assert!(
        (voltage + 1.769_292_354_238_631_4).abs() < 1e-6,
        "cubic root: {voltage}"
    );
    let quality = engine.convergence_quality();
    assert_eq!(
        quality.timestep_reductions, 1,
        "only the first terminal Newton rejection reduces a timestep; the second plain-Newton failure is rescued in-step"
    );
    assert_eq!(quality.force_accepted_points, 0);
}

/// A force-accept arms the conservative step-recovery caps for two recovery
/// events, not for the rest of the run. The cooldown used to be decremented
/// only inside the Newton non-convergence arm, so a timepoint force-accepted
/// out of LTE exhaustion in a run that never failed Newton again held the
/// accepted-step growth limit at 1.5x all the way to `tstop`.
///
/// The deck reaches that path without a single Newton failure, and three things
/// put it there:
///
/// * the coupled inductor pair keeps the stressed node off ngspice device-local
///   truncation, so the generic voltage LTE is the acceptance authority there.
///   On any node a device limit covers, that limit rejects the candidate before
///   the voltage LTE is even formed, so this branch is unreachable;
/// * the 1 ps edge at t = 0.5 s leaves curvature on that node four decades
///   sharper than anything else in the run; and
/// * `reltol` 1e-9 with `abstol` 1e-12 puts the width that curvature demands
///   below this deck's integration floor of 1.0e-11 s. The tolerances are
///   load-bearing, not decoration: a predictor with an order is entitled to an
///   estimate that falls with a power of the width, so it always satisfies a
///   default `reltol` somewhere above the floor — at the defaults this same
///   deck refines to 4.4e-7 s and accepts.
///
/// The deck used to reach the floor at the default tolerances for a reason that
/// is no longer a rejection at all: after the edge's breakpoint the predictor
/// was restarted to a single point, and a constant cannot follow a ramp at any
/// width, so the demand was `abstol/slope`. R2.24 stopped forming a verdict
/// where the predictor is a constant, which is why the provocation here has to
/// be a curvature demand the tolerances cannot meet.
///
/// The diode keeps the deck nonlinear, which is what puts the growth limit on
/// the cooldown instead of on the strictly linear fast-recovery path.
#[test]
fn a_force_accepted_edge_does_not_cap_step_growth_for_the_rest_of_the_run() {
    let netlist = Netlist::parse(
        "force accepted edge\n.options reltol=1e-9 abstol=1e-12\nV1 in 0 PULSE(0 1 0.5 1p 1p 10 20)\nR1 in out 1k\nC1 out 0 1n\nD1 0 out dm\n.model dm D(IS=1e-14)\nR3 in p 1k\nL1 p 0 1\nL2 s 0 1\nK1 L1 L2 0.5\nR4 s 0 1k\n.end\n",
    )
    .unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let result = engine.run_tran(&netlist, 1.0, 1.0).unwrap();
    let quality = engine.convergence_quality();
    let last_force_accept = quality
        .force_accepted_indices
        .iter()
        .copied()
        .max()
        .expect("the 1 ps edge exhausts LTE recovery at the integration floor");
    // The provocation has to be the edge rather than anything else in a one
    // second run, and it has to be LTE exhaustion rather than a rescued Newton
    // failure - the arm this pins is the one the old cooldown never reached.
    let force_accepted_time = result.time[last_force_accept];
    assert!(
        (force_accepted_time - 0.5).abs() < 1.0e-6,
        "the force-accepted point is at {force_accepted_time} rather than on the 1 ps edge at 0.5 s"
    );
    assert_eq!(
        (quality.gmin_stepping_count, quality.source_stepping_count),
        (0, 0),
        "no convergence aid may run: a Newton failure would decay the cooldown \
         under the old rule too, and the case would stop discriminating"
    );
    let widths: Vec<f64> = result.time.windows(2).map(|w| w[1] - w[0]).collect();
    // Two accepted points after the last force-accept the budget is spent, so
    // the widest growth in the remaining tail is the ordinary 2x limit. While
    // the cooldown was stuck the same tail grew by exactly 1.5x per step.
    let widest_growth = widths
        .windows(2)
        .skip(last_force_accept + 2)
        .map(|w| w[1] / w[0])
        .fold(0.0_f64, f64::max);
    assert!(
        widest_growth > 1.9,
        "growth after the force-accepted edge peaked at {widest_growth}: the recovery caps never \
         disarmed (1.5 is the armed accepted-step growth limit)"
    );
}
