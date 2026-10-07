//! Public frequency-grid errors retain allocator causes across analysis boundaries.
use std::collections::TryReserveError;
use std::error::Error;

use rspice_core::analysis::pac::PacSweepType;
use rspice_core::analysis::pxf::{PxfConfig, PxfError, PxfSweepType};
use rspice_core::analysis::stb::{StbAnalysisError, StbConfig, StbSweepType};
use rspice_core::analysis::{FrequencyGridError, PacConfig};
use rspice_core::netlist::FreqVariation;
use rspice_core::{ResourceKind, SimulationError, SimulationErrorCategory, SimulationErrorCode};

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
