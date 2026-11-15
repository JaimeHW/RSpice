use std::panic::{AssertUnwindSafe, catch_unwind};

use super::family_src::read_family_src;
use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::{DepletionCap, NonlinearDeviceInstance};
use rspice_core::analysis::{HbConfig, HbError, HbFft, HbSolver, HbSolverState};
use rspice_core::engine::HbOperatingPoint;
use rspice_core::{Engine, Netlist, NoAbort};

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
    iterations.max_iterations = 0;
    assert_engine_rejects_without_panicking(iterations, "max_iterations");

    let mut restart = HbConfig::new(1.0e6);
    restart.gmres_restart = usize::MAX;
    assert_engine_rejects_without_panicking(restart, "gmres_restart");

    let mut mixing = HbConfig::new(1.0e6);
    mixing.max_mixing_order = 0;
    assert_engine_rejects_without_panicking(mixing, "max_mixing_order");
}

/// An iteration limit and a mixing order are the user's own numbers. A
/// limit no solve reaches and an order a single tone never mixes to are
/// valid and change nothing; neither sizes any storage.
#[test]
fn an_unreachable_iteration_limit_and_mixing_order_are_accepted() {
    let mut config = HbConfig::new(1.0e6);
    config.max_iterations = usize::MAX;
    config.max_mixing_order = usize::MAX;
    let result = Engine::default()
        .run_hb(&fixture(), config)
        .expect("a linear divider converges long before any iteration limit");
    let dc = result.result.spectral_voltages[0].coefficients[0].re;
    assert!((dc - 1.0).abs() < 1.0e-12, "V(out) = {dc}");
}

/// A collocation grid takes its size from the configuration, as Xyce's
/// `NUMFREQ` and Spectre's `harms`/`oversample` do: neither tool caps it.
/// Over 2.1 million points (the old ceiling was 2 million), the transform
/// returns the closed-form spectrum of the square-law response
/// `cos(wt) - 0.1 cos(wt)^2 = -0.05 + cos(wt) - 0.05 cos(2wt)`.
#[test]
fn a_collocation_grid_above_two_million_points_transforms_to_the_closed_form() {
    let points = 3usize.pow(3) * 5usize.pow(7);
    assert!(points > 2_000_000 && points % 2 == 1);
    let mut fft = HbFft::try_with_size(4, points).expect("a grid this machine can hold plans");
    let waveform = (0..points)
        .map(|sample| {
            let phase = std::f64::consts::TAU * sample as f64 / points as f64;
            phase.cos() - 0.1 * phase.cos().powi(2)
        })
        .collect::<Vec<_>>();
    let spectrum = fft.to_frequency_domain(&waveform);
    let peak = |harmonic: usize| {
        if harmonic == 0 {
            spectrum[0]
        } else {
            spectrum[harmonic] * 2.0
        }
    };
    let expected = [-0.05, 1.0, -0.05, 0.0, 0.0];
    for (harmonic, want) in expected.into_iter().enumerate() {
        let got = peak(harmonic);
        assert!(
            (got - Complex64::new(want, 0.0)).norm() < 1.0e-9,
            "harmonic {harmonic}: {got} against {want}"
        );
    }
}

/// A grid no machine can hold is refused by the allocator, as a typed
/// failure, before any sample is stored or any transform planned.
#[test]
fn a_collocation_grid_no_machine_can_hold_is_a_typed_allocation_failure() {
    let points = (1usize << 61) + 1;
    let error = Engine::default()
        .run_hb(
            &fixture(),
            HbConfig::new(1.0e6)
                .with_harmonics(3)
                .with_collocation_points(points),
        )
        .expect_err("2^61 complex samples exceed every address space");
    assert!(
        matches!(error, rspice_core::SimulationError::Allocation { .. }),
        "an unholdable grid is an allocation failure, not an abort: {error:?}"
    );
    assert!(
        HbFft::try_with_size(3, points)
            .is_err_and(|error| matches!(error, HbError::Allocation { .. })),
        "the transform's own constructor refuses the same grid"
    );
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
        legacy.solve_dc_operating_point_with_abort(&mut state, &NoAbort),
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

#[test]
fn public_hb_solver_rejects_invalid_charge_parameters_before_evaluation() {
    let mut invalid_devices = Vec::new();

    let mut invalid_junction = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_junction.params.cap_a = DepletionCap::new(1.0e-12, 0.7, 1.01, 0.5);
    invalid_devices.push((invalid_junction, "grading coefficient"));

    let mut invalid_gate = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, 1.0e-3, 0.0);
    invalid_gate.params.cox_wl = -1.0e-15;
    invalid_devices.push((invalid_gate, "intrinsic gate capacitance"));

    let mut invalid_transit = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_transit.params.tt_f = f64::NAN;
    invalid_devices.push((invalid_transit, "transit time"));

    let invalid_diode = NonlinearDeviceInstance::diode(0, 0, -1.0, 1.0);
    invalid_devices.push((invalid_diode, "diode IS"));

    let invalid_mos = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, -1.0, 0.0);
    invalid_devices.push((invalid_mos, "MOS KP"));

    let invalid_jfet = NonlinearDeviceInstance::njfet(0, 0, 0, -2.0, -1.0, 0.0, 1.0e-14);
    invalid_devices.push((invalid_jfet, "JFET BETA"));

    let mut invalid_arity = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_arity.terminals.pop();
    invalid_devices.push((invalid_arity, "has 1 terminals, expected 2"));

    let invalid_index = NonlinearDeviceInstance::diode(2, 0, 1.0e-14, 1.0);
    invalid_devices.push((invalid_index, "node index 2 exceeds 1 nodes"));

    for (device, expected) in invalid_devices {
        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
        solver.add_nonlinear_device(device);
        let mut state = HbSolverState::new(1, 1);
        let error = solver
            .solve_dc_operating_point_with_abort(&mut state, &NoAbort)
            .expect_err("invalid public nonlinear-device parameters must fail before solving");
        assert!(matches!(error, HbError::InvalidCircuit(_)));
        assert!(
            error.to_string().contains(expected),
            "wrong public-solver parameter diagnostic: {error}"
        );
    }
}

#[test]
fn public_hb_surface_does_not_advertise_rejected_approximate_kernels() {
    let solver_source = read_family_src("analysis/harmonic_balance/solver.rs");
    let device_source = read_family_src("analysis/harmonic_balance/solver/devices.rs");
    let api_source = read_family_src("analysis/harmonic_balance/solver/nonlinear_api.rs");
    let combined = format!("{solver_source}\n{device_source}\n{api_source}");

    for stale_name in [
        "NpnBjt",
        "PnpBjt",
        "CurrentSwitch",
        "npn_bjt",
        "pnp_bjt",
        "current_switch",
        "add_current_switch",
    ] {
        assert!(
            !combined.contains(stale_name),
            "exact-HB public surface still advertises removed approximate kernel {stale_name}"
        );
    }
}
