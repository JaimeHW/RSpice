use std::panic::{AssertUnwindSafe, catch_unwind};

use num_complex::Complex64;
use rspice_core::analysis::{HbConfig, HbError, HbFft, HbSolver, HbSolverState};
use rspice_core::engine::HbOperatingPoint;
use rspice_core::{Engine, Netlist};

fn fixture() -> Netlist {
    Netlist::parse("HB config validation\nV1 out 0 1\nR1 out 0 1k\n.end").expect("fixture parses")
}

fn assert_engine_rejects_without_panicking(config: HbConfig, field: &str) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        Engine::default().run_hb(&fixture(), config)
    }));
    let error = result
        .expect("malformed HB configuration must never unwind")
        .expect_err("malformed HB configuration must return an error");
    assert!(
        error.to_string().contains(field),
        "expected field {field:?} in {error}"
    );
}

#[test]
fn malformed_collocation_grids_return_typed_errors_without_unwinding() {
    let mut undersized = HbConfig::new(1.0e6).with_harmonics(3);
    undersized.collocation_points = Some(5);
    assert_engine_rejects_without_panicking(undersized, "collocation_points");

    let mut even = HbConfig::new(1.0e6).with_harmonics(3);
    even.collocation_points = Some(8);
    assert_engine_rejects_without_panicking(even, "collocation_points");

    let checked_fft = catch_unwind(|| HbFft::try_with_size(3, 5));
    assert!(
        checked_fft
            .expect("checked FFT construction must never unwind")
            .is_err()
    );
}

#[test]
fn overflow_and_extreme_budgets_fail_before_allocation() {
    let mut harmonics = HbConfig::new(1.0e6);
    harmonics.num_harmonics = usize::MAX;
    assert_engine_rejects_without_panicking(harmonics, "num_harmonics");

    let mut oversample = HbConfig::new(1.0e6).with_harmonics(1);
    oversample.oversample_factor = usize::MAX;
    assert_engine_rejects_without_panicking(oversample, "oversample_factor");

    let mut iterations = HbConfig::new(1.0e6);
    iterations.max_iterations = usize::MAX;
    assert_engine_rejects_without_panicking(iterations, "max_iterations");

    let mut restart = HbConfig::new(1.0e6);
    restart.gmres_restart = usize::MAX;
    assert_engine_rejects_without_panicking(restart, "gmres_restart");

    let mut mixing = HbConfig::new(1.0e6);
    mixing.max_mixing_order = usize::MAX;
    assert_engine_rejects_without_panicking(mixing, "max_mixing_order");
}

#[test]
fn nonfinite_tolerances_and_bad_damping_return_field_specific_errors() {
    let mut tolerance = HbConfig::new(1.0e6);
    tolerance.tolerance = f64::NAN;
    assert_engine_rejects_without_panicking(tolerance, "tolerance");

    let mut abstol = HbConfig::new(1.0e6);
    abstol.abstol = -1.0;
    assert_engine_rejects_without_panicking(abstol, "abstol");

    let mut damping = HbConfig::new(1.0e6);
    damping.damping = f64::INFINITY;
    assert_engine_rejects_without_panicking(damping, "damping");

    let mut minimum = HbConfig::new(1.0e6);
    minimum.min_damping = 1.1;
    assert_engine_rejects_without_panicking(minimum, "min_damping");
}

#[test]
fn public_solver_constructor_is_fallible_and_legacy_constructor_is_fail_closed() {
    let mut invalid = HbConfig::new(1.0e6);
    invalid.collocation_points = Some(1);

    let checked = catch_unwind(AssertUnwindSafe(|| HbSolver::try_new(invalid.clone(), 1)))
        .expect("fallible solver construction must never unwind");
    assert!(matches!(checked, Err(HbError::InvalidConfig(_))));

    let mut legacy = catch_unwind(AssertUnwindSafe(|| HbSolver::new(invalid, 1)))
        .expect("legacy solver construction must retain, not panic on, validation failure");
    let mut state = HbSolverState::new(1, 1);
    assert!(matches!(
        legacy.solve_dc_operating_point(&mut state),
        Err(HbError::InvalidConfig(_))
    ));
}

#[test]
fn retained_operating_point_reconstruction_authenticates_full_config() {
    let mut invalid = HbConfig::new(1.0e6).with_harmonics(1);
    invalid.damping = 0.0;
    let spectrum = vec![vec![Complex64::new(0.0, 0.0); 2]];

    let result = catch_unwind(AssertUnwindSafe(|| {
        HbOperatingPoint::try_from_parts(invalid, vec!["out".to_owned()], spectrum, 1, 0.0)
    }));
    let error = result
        .expect("retained-state authentication must never unwind")
        .expect_err("malformed retained configuration must fail");
    assert!(error.to_string().contains("damping"));
}
