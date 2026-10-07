//! Public frequency-grid errors retain allocator causes across analysis boundaries.
use std::collections::TryReserveError;
use std::error::Error;

use rspice_core::analysis::pac::PacSweepType;
use rspice_core::analysis::pxf::{PxfConfig, PxfError, PxfSweepType};
use rspice_core::analysis::stb::{StbAnalysisError, StbConfig, StbSweepType};
use rspice_core::analysis::{FrequencyGridError, PacConfig};
use rspice_core::netlist::FreqVariation;
use rspice_core::{ResourceKind, SimulationError, SimulationErrorCategory, SimulationErrorCode};

fn point_limited_engine(limit: usize) -> rspice_core::Engine {
    let mut config = rspice_core::SimulationConfig::default();
    config.resource_limits.max_analysis_points = limit;
    rspice_core::Engine::new(config)
}

fn assert_point_limit(error: SimulationError, limit: usize) {
    let SimulationError::ResourceLimit(error) = error else {
        panic!("point policy must reject the grid before allocation or construction: {error}");
    };
    assert_eq!(error.resource, ResourceKind::AnalysisPoints);
    assert_eq!((error.requested, error.limit), (limit + 1, limit));
}

fn run_authored_grid(
    engine: &rspice_core::Engine,
    netlist: &rspice_core::Netlist,
    card: &rspice_core::netlist::AnalysisCommand,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<f64>, SimulationError> {
    match card {
        rspice_core::netlist::AnalysisCommand::Sp { .. } => engine
            .run_sp_with_abort(netlist, card, abort)
            .map(|result| result.scattering.frequencies().to_vec()),
        _ => engine
            .run_sensitivity_from_card_with_abort(netlist, card, abort)
            .map(|result| match result {
                rspice_core::engine::SensitivityCardResult::Ac(result) => result.frequencies,
                _ => panic!("the card requests an AC sensitivity sweep"),
            }),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn authored_grids_apply_point_policy_before_capacity_and_circuit_construction() {
    use rspice_core::netlist::AnalysisCommand;
    for command in [".sens V(out) R1 AC LIN 3 0 1", ".sp LIN 3 0 1 PORT1=(out)"] {
        // No circuit is needed to reject the grid. The capacity is impossible
        // on both 32- and 64-bit hosts, so this cannot cause a giant allocation.
        let netlist =
            rspice_core::Netlist::parse(&format!("Grid budget\n{command}\n.end\n")).unwrap();
        let mut card = netlist.analyses[0].clone();
        match &mut card {
            AnalysisCommand::Sensitivity {
                ac_sweep: Some(sweep),
                ..
            } => sweep.points = usize::MAX / 2,
            AnalysisCommand::Sp { points, .. } => *points = usize::MAX / 2,
            _ => unreachable!(),
        }
        assert_point_limit(
            run_authored_grid(
                &point_limited_engine(2),
                &netlist,
                &card,
                &rspice_core::NoAbort,
            )
            .unwrap_err(),
            2,
        );
        match &mut card {
            AnalysisCommand::Sensitivity {
                ac_sweep: Some(sweep),
                ..
            } => sweep.start_freq = -1.0,
            AnalysisCommand::Sp { start_freq, .. } => *start_freq = -1.0,
            _ => unreachable!(),
        }
        let error = run_authored_grid(
            &point_limited_engine(2),
            &netlist,
            &card,
            &rspice_core::NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(error, SimulationError::Netlist(message) if message == FrequencyGridError::InvalidStartFrequency.to_string())
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn authored_grids_budget_actual_lin_dec_and_oct_points_and_remain_cancellable() {
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};
    for (sweep, expected) in [
        ("LIN 2 1 9", vec![1.0]),
        ("LIN 3 1 3", vec![1.0, 2.0, 3.0]),
        ("DEC 1 1 100", vec![1.0, 10.0, 100.0]),
        ("OCT 1 1 4", vec![1.0, 2.0, 4.0]),
    ] {
        for command in [
            format!(".sens V(out) R1 AC {sweep}"),
            format!(".sp {sweep} PORT1=(out)"),
        ] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "Grid budget\nI1 0 out DC 1m AC 1\nR1 out 0 1k\n{command}\n.end\n"
            ))
            .unwrap();
            let card = &netlist.analyses[0];
            let actual = run_authored_grid(
                &point_limited_engine(expected.len()),
                &netlist,
                card,
                &rspice_core::NoAbort,
            )
            .unwrap();
            assert_eq!(actual.len(), expected.len());
            for (actual, expected) in actual.iter().zip(&expected) {
                assert!((actual / expected - 1.0).abs() < 1e-14);
            }
            assert_point_limit(
                run_authored_grid(
                    &point_limited_engine(expected.len() - 1),
                    &netlist,
                    card,
                    &rspice_core::NoAbort,
                )
                .unwrap_err(),
                expected.len() - 1,
            );
            assert!(matches!(
                run_authored_grid(&point_limited_engine(0), &netlist, card, &ImmediateAbort),
                Err(SimulationError::Aborted)
            ));
            let abort = CountingAbort::new(1);
            assert!(matches!(
                run_authored_grid(&point_limited_engine(10), &netlist, card, &abort),
                Err(SimulationError::Aborted)
            ));
            assert_eq!(abort.polls_after_abort(), 0);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_pnoise_carriers_apply_the_callers_offset_budget() {
    use rspice_core::analysis::{PssConfig, harmonic_balance::HbConfig};
    use rspice_core::engine::PeriodicNoiseResult;
    use rspice_core::{Engine, Netlist, NoAbort};

    let netlist = Netlist::parse("PNOISE budget\nV1 in 0 DC 0\nR1 in out 1k\nC1 out 0 1n\n.pnoise LIN 3 1 2 OUT=out INPUT=V1 MAXSIDEBAND=1\n.end\n").unwrap();
    let rspice_core::netlist::AnalysisCommand::Pnoise(mut card) = netlist.analyses[0].clone()
    else {
        panic!("PNOISE card")
    };
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(2))
        .unwrap();
    let pss = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(1e3)
                .with_harmonics(2)
                .with_points_per_period(32)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    for use_hb in [true, false] {
        // A zero origin lets the huge LIN request keep advancing on both
        // pointer widths, so it reaches policy admission instead of roundoff.
        card.sweep.start_freq = 0.0;
        card.sweep.stop_freq = 1.0;
        let run = |card: &rspice_core::netlist::PnoiseCard,
                   limit,
                   abort: &dyn rspice_core::AbortSignal| {
            let limited = point_limited_engine(limit);
            if use_hb {
                limited.run_pnoise_card_from_hb_with_abort(
                    &netlist,
                    card,
                    &hb.operating_point,
                    abort,
                )
            } else {
                limited.run_pnoise_card_from_pss_with_abort(&netlist, card, &pss, abort)
            }
        };
        card.sweep.points = usize::MAX / 2;
        assert_point_limit(run(&card, 2, &NoAbort).unwrap_err(), 2);
        assert!(matches!(
            run(&card, 2, &rspice_core::abort_signal::ImmediateAbort),
            Err(SimulationError::Aborted)
        ));
        let abort = rspice_core::abort_signal::CountingAbort::new(1);
        assert!(matches!(
            run(&card, 2, &abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
        card.sweep.start_freq = -1.0;
        assert!(matches!(
            run(&card, 2, &NoAbort),
            Err(SimulationError::Netlist(_))
        ));
        card.sweep.points = 2;
        card.sweep.start_freq = 1.0;
        card.sweep.stop_freq = 2.0;
        // Retained-carrier integration and sidebands also consume the point
        // ceiling. Allow those while checking LIN 2 still emits one offset.
        let PeriodicNoiseResult::Driven { result, .. } = run(&card, 128, &NoAbort).unwrap() else {
            panic!("driven carrier")
        };
        assert_eq!(result.frequencies, vec![1.0]);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn linear_unit_steps_retain_every_frequency_in_bounded_and_unbounded_grids() {
    use rspice_core::analysis::ac::{
        try_ac_sweep_frequencies_bounded_with_abort, try_ac_sweep_frequencies_with_abort,
        try_ac_sweep_point_count_bounded_with_abort,
    };
    for start in [0.0, 1.0, 10.0] {
        for points in [3, 5, 257] {
            let stop = start + (points - 1) as f64;
            let expected = (0..points).map(|i| start + i as f64).collect::<Vec<_>>();
            let unbounded = try_ac_sweep_frequencies_with_abort(
                FreqVariation::Lin,
                points,
                start,
                stop,
                &rspice_core::NoAbort,
            )
            .unwrap();
            let bounded = try_ac_sweep_frequencies_bounded_with_abort(
                FreqVariation::Lin,
                points,
                start,
                stop,
                points,
                &rspice_core::NoAbort,
            )
            .unwrap();
            assert_eq!(unbounded, expected);
            assert_eq!(bounded, expected);
            assert_eq!(
                try_ac_sweep_point_count_bounded_with_abort(
                    FreqVariation::Lin,
                    points,
                    start,
                    stop,
                    points,
                    &rspice_core::NoAbort,
                )
                .unwrap(),
                points
            );
            assert_eq!(
                try_ac_sweep_frequencies_bounded_with_abort(
                    FreqVariation::Lin,
                    points,
                    start,
                    stop,
                    points - 1,
                    &rspice_core::NoAbort,
                ),
                Err(FrequencyGridError::LimitExceeded {
                    requested: points,
                    limit: points - 1
                })
            );
        }
    }
}

fn assert_allocator_cause(error: FrequencyGridError, requested: usize) {
    let FrequencyGridError::Allocation {
        requested: actual, ..
    } = &error
    else {
        panic!("{error}");
    };
    assert_eq!(*actual, requested);
    let cause = error
        .source()
        .unwrap()
        .downcast_ref::<TryReserveError>()
        .unwrap()
        .clone();
    let converted = SimulationError::from(error.clone());
    let descriptor = converted.descriptor();
    assert_eq!(descriptor.category, SimulationErrorCategory::ResourceLimit);
    assert_eq!(descriptor.code, SimulationErrorCode::AllocationFailed);
    let SimulationError::Allocation { source, .. } = converted else {
        panic!("allocator category lost");
    };
    assert_eq!(source, cause);
    assert_eq!(error, error.clone());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ac_and_periodic_grids_preserve_capacity_refusal_without_allocating_giant_buffers() {
    // Both capacities exceed the platform's isize-backed Vec allocation range.
    // These failures are deterministic capacity refusals, not simulated OOMs.
    let count = usize::MAX / 2;
    let error = rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
        FreqVariation::Lin,
        count,
        0.0,
        1.0,
        &rspice_core::NoAbort,
    )
    .unwrap_err();
    assert_allocator_cause(error, count);
    let error = PacConfig::new()
        .with_sweep(1.0, 2.0, usize::MAX)
        .with_sweep_type(PacSweepType::Linear)
        .frequency_points()
        .unwrap_err();
    assert_allocator_cause(error, usize::MAX);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stb_grid_keeps_the_frequency_error_and_its_allocator_source_chain() {
    let error = StbConfig::new()
        .with_sweep(1.0, 2.0, usize::MAX)
        .with_sweep_type(StbSweepType::Linear)
        .frequency_points()
        .unwrap_err();
    let grid = error
        .source()
        .unwrap()
        .downcast_ref::<FrequencyGridError>()
        .unwrap();
    assert!(grid.source().unwrap().is::<TryReserveError>());
    let StbAnalysisError::FrequencyGrid(grid) = error else {
        panic!("grid cause lost");
    };
    assert_allocator_cause(grid, usize::MAX);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn pxf_grid_keeps_the_frequency_error_and_its_allocator_source_chain() {
    let error = PxfConfig::new()
        .with_sweep(1.0, 2.0, usize::MAX)
        .with_sweep_type(PxfSweepType::Linear)
        .frequency_points()
        .unwrap_err();
    let grid = error
        .source()
        .unwrap()
        .downcast_ref::<FrequencyGridError>()
        .unwrap();
    assert!(grid.source().unwrap().is::<TryReserveError>());
    let PxfError::FrequencyGrid(grid) = error else {
        panic!("grid cause lost");
    };
    assert_allocator_cause(grid, usize::MAX);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cancellation_policy_limits_and_invalid_input_remain_distinct() {
    assert!(matches!(
        SimulationError::from(FrequencyGridError::Aborted),
        SimulationError::Aborted
    ));
    let error = SimulationError::from(FrequencyGridError::LimitExceeded {
        requested: 7,
        limit: 6,
    });
    let SimulationError::ResourceLimit(limit) = error else {
        panic!("policy limit lost");
    };
    assert_eq!(limit.resource, ResourceKind::AnalysisPoints);
    assert_eq!((limit.requested, limit.limit), (7, 6));
    let invalid = FrequencyGridError::InvalidStartFrequency;
    assert!(invalid.source().is_none());
    assert!(matches!(
        SimulationError::from(invalid),
        SimulationError::Circuit(_)
    ));
}
