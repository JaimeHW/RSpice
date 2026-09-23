//! The public physical-device path, with an independently known modulated orbit.
use super::*;
use crate::NoAbort;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicSampling, QuasiPeriodicTransform,
};
use crate::device::behavioral::{
    BehavioralBranchResolution, BehavioralCurrentSource, BehavioralSources,
};
use std::f64::consts::{SQRT_2, TAU};

#[test]
fn autonomous_qpss_native_equations_preserve_fixed_clocks_and_integrals() {
    let limits = ResourceLimits::default();
    let mut basis = QuasiPeriodicGridConfig::new(vec![1.2 / TAU, SQRT_2 / TAU], vec![2, 3]);
    basis.sampling = QuasiPeriodicSampling::Exact(vec![13, 25]);
    let grid = Arc::new(QuasiPeriodicGrid::new_with_abort(basis, &limits, &NoAbort).unwrap());
    let oscillator = QuasiPeriodicAutonomousConfig {
        tone: 0,
        phase_coordinate: 0,
        phase_tuple: vec![1, 0],
        minimum_amplitude: 0.1,
        max_relative_frequency_step: 0.2,
    };
    let config = QuasiPeriodicSolveConfig {
        relative_tolerance: 1e-8,
        ..Default::default()
    };
    for integral in [false, true] {
        let omega = if integral {
            "(1+.1*(sqrt(2)+sdt(-2*sin(sqrt(2)*time))))"
        } else {
            "(1+.1*sqrt(2)*cos(sqrt(2)*time))"
        };
        let mut sources = BehavioralSources::default();
        for (name, node, expression) in [
            ("BX", 1, format!("(1-v(x)^2-v(y)^2)*v(x)-{omega}*v(y)")),
            ("BY", 2, format!("(1-v(x)^2-v(y)^2)*v(y)+{omega}*v(x)")),
        ] {
            let mut source =
                BehavioralCurrentSource::new(name.into(), 0, node, &expression).unwrap();
            source
                .bind_references(
                    |name| match name {
                        "x" => Some(1),
                        "y" => Some(2),
                        _ => None,
                    },
                    |_| BehavioralBranchResolution::MissingDevice,
                )
                .unwrap();
            sources.current_sources.push(source);
        }
        let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 2);
        solver.add_capacitance(0, 0, 1.0);
        solver.add_capacitance(1, 1, 1.0);
        solver
            .set_quasi_periodic_behavioral_sources(&sources, &grid)
            .unwrap();
        let zero = vec![vec![Complex64::ZERO; grid.len()]; solver.unknowns()];
        let mut seed = zero.clone();
        let k = grid.index_of(&[1, 0]).unwrap();
        seed[0][k] = Complex64::from_polar(0.4, 0.2);
        seed[1][k] = Complex64::new(0.0, -1.0) * seed[0][k];
        for row in &mut seed {
            row[grid.len() - 1 - k] = row[k].conj();
        }
        let result = solver
            .solve_autonomous_quasi_periodic_with_abort(
                grid.clone(),
                &config,
                &oscillator,
                &zero,
                &seed,
                &limits,
                &NoAbort,
            )
            .unwrap();
        assert!((result.grid().config().frequencies_hz[0] - 1.0 / TAU).abs() < 1e-8);
        assert_eq!(result.grid().config().frequencies_hz[1], SQRT_2 / TAU);
        let mut transform =
            QuasiPeriodicTransform::new_with_abort(result.grid().clone(), &NoAbort).unwrap();
        for (row, spectrum) in result.spectra().iter().enumerate() {
            let values = transform
                .to_real_samples_with_abort(spectrum, &NoAbort)
                .unwrap();
            for (i, value) in values.iter().enumerate() {
                let phases = grid.phases(i).unwrap();
                let angle = phases[0] + 0.1 * phases[1].sin();
                let expected = match row {
                    0 => angle.cos(),
                    1 => angle.sin(),
                    _ => SQRT_2 * (phases[1].cos() - 1.0),
                };
                assert!(
                    (value - expected).abs() < 1e-5,
                    "integral={integral}, row={row}, sample={i}: {value} vs {expected}"
                );
            }
        }
        // A clock with this authored frequency must never be relabeled as a
        // free phase, even when buried inside an integral output expression.
        let driven = QuasiPeriodicAutonomousConfig {
            tone: 1,
            phase_tuple: vec![0, 1],
            ..oscillator.clone()
        };
        let error = solver
            .solve_autonomous_quasi_periodic_with_abort(
                grid.clone(),
                &config,
                &driven,
                &zero,
                &seed,
                &limits,
                &NoAbort,
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("authored behavioral or capacitance clock"),
            "{error}"
        );
    }
}
