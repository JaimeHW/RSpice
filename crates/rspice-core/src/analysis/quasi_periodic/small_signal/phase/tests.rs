use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicLinearMethod, QuasiPeriodicSampling, solve::Sample,
};
use std::f64::consts::{SQRT_2, TAU};

const CHARGE_NONLINEARITY: Value = 0.2;

struct Oscillator {
    floating: bool,
}
impl Circuit for Oscillator {
    fn unknowns(&self) -> usize {
        if self.floating { 3 } else { 2 }
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(vec![])
    }
    fn sample(&mut self, _: &[Value], _: bool) -> Result<Sample, Error> {
        unreachable!()
    }
    fn sample_at_phases(
        &mut self,
        x: &[Value],
        phases: &[Value],
        _: bool,
    ) -> Result<Sample, Error> {
        let a = 1.0 + 0.1 * phases[1].cos();
        // Q=(1+k*r^2)*x and F=-DQ*f, where f is the radial oscillator
        // vector field. This preserves its analytic orbit while exercising
        // sampled, off-diagonal nonlinear charge in the phase-rate column.
        let k = CHARGE_NONLINEARITY;
        let radius_squared = x[0] * x[0] + x[1] * x[1];
        let h = 1.0 + k * radius_squared;
        let g = a * (radius_squared - 1.0) * (1.0 + 3.0 * k * radius_squared);
        let derivative = 2.0 * a * (1.0 + 3.0 * k * (2.0 * radius_squared - 1.0));
        Ok(Sample {
            conductance: vec![
                (0, 0, g + derivative * x[0] * x[0] + 2.0 * k * x[0] * x[1]),
                (0, 1, h + derivative * x[0] * x[1] + 2.0 * k * x[1] * x[1]),
                (1, 0, -h + derivative * x[0] * x[1] - 2.0 * k * x[0] * x[0]),
                (1, 1, g + derivative * x[1] * x[1] - 2.0 * k * x[0] * x[1]),
            ],
            capacitance: vec![
                (0, 0, h + 2.0 * k * x[0] * x[0]),
                (0, 1, 2.0 * k * x[0] * x[1]),
                (1, 0, 2.0 * k * x[0] * x[1]),
                (1, 1, h + 2.0 * k * x[1] * x[1]),
            ],
            ..Default::default()
        })
    }
}

#[test]
fn autonomous_qp_response_resolves_phase_pole_and_adjoint_below_carrier_ulp() {
    let limits = ResourceLimits::default();
    let mut config = QuasiPeriodicGridConfig::new(vec![1.0 / TAU, SQRT_2 / TAU], vec![1, 2]);
    config.sampling = QuasiPeriodicSampling::Exact(vec![9, 15]);
    let grid = Arc::new(QuasiPeriodicGrid::new_with_abort(config, &limits, &NoAbort).unwrap());
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    let phases: Vec<_> = (0..grid.sample_count())
        .map(|i| grid.phases(i).unwrap())
        .collect();
    let orbit: Vec<_> = (0..2)
        .map(|row| {
            transform
                .to_spectrum_with_abort(
                    &phases
                        .iter()
                        .map(|p| {
                            Complex64::new(if row == 0 { p[0].cos() } else { p[0].sin() }, 0.0)
                        })
                        .collect::<Vec<_>>(),
                    &NoAbort,
                )
                .unwrap()
        })
        .collect();
    for method in [
        QuasiPeriodicLinearMethod::Direct,
        QuasiPeriodicLinearMethod::Krylov,
    ] {
        let mut circuit = Oscillator { floating: false };
        let mut config = QuasiPeriodicAcConfig::default();
        config.linear.method = method;
        config.linear.restart = 40;
        let mut forward = Linearization::prepare(
            &mut circuit,
            grid.clone(),
            &orbit,
            &config,
            &limits,
            &NoAbort,
        )
        .unwrap();
        forward
            .prepare_autonomous(&circuit, &orbit, 0, 1e-10, &limits, &NoAbort)
            .unwrap();
        let mut adjoint = Linearization::prepare_adjoint(
            &mut circuit,
            grid.clone(),
            &orbit,
            &config.linear,
            &limits,
            &NoAbort,
        )
        .unwrap();
        adjoint
            .prepare_autonomous(&circuit, &orbit, 0, 1e-10, &limits, &NoAbort)
            .unwrap();
        let mut observation = vec![vec![Complex64::ZERO; grid.len()]; 2];
        observation[0][grid.index_of(&[1, 0]).unwrap()] = Complex64::new(0.7, -0.3);
        observation[1][grid.index_of(&[-1, 1]).unwrap()] = Complex64::new(-0.2, 0.4);
        for offset in [-0.037, 1e-24] {
            let jw = Complex64::new(0.0, TAU * offset);
            let adj = adjoint
                .solve_adjoint(&circuit, offset, &observation, &NoAbort)
                .unwrap();
            for radial in [false, true] {
                let mut rhs = vec![];
                let mut expected = vec![];
                for row in 0..2 {
                    let mut forcing = vec![];
                    let mut response = vec![];
                    for p in &phases {
                        let unit = if row == 0 { p[0].cos() } else { p[0].sin() };
                        let tangent = if row == 0 { -p[0].sin() } else { p[0].cos() };
                        if radial {
                            let amplitude = Complex64::new(0.3, 0.2) * (1.0 + 0.2 * p[1].cos());
                            let derivative =
                                Complex64::new(0.3, 0.2) * (-0.2 * SQRT_2 * p[1].sin());
                            forcing.push(
                                (1.0 + 3.0 * CHARGE_NONLINEARITY)
                                    * unit
                                    * ((2.0 * (1.0 + 0.1 * p[1].cos()) + jw) * amplitude
                                        + derivative),
                            );
                            response.push(unit * amplitude);
                        } else {
                            forcing.push(
                                (1.0 + CHARGE_NONLINEARITY) * Complex64::new(0.4, -0.2) * tangent,
                            );
                            response.push(Complex64::new(0.4, -0.2) * tangent / jw);
                        }
                    }
                    rhs.push(
                        transform
                            .to_spectrum_with_abort(&forcing, &NoAbort)
                            .unwrap(),
                    );
                    expected.push(
                        transform
                            .to_spectrum_with_abort(&response, &NoAbort)
                            .unwrap(),
                    );
                }
                // FFT roundoff in a purely radial source is a real phase
                // forcing at tiny offsets. Test its finite radial response at
                // a resolvable offset; the tangent case tests sub-ulp diffusion.
                if radial && offset.abs() < 1e-10 {
                    continue;
                }
                let result = forward.solve(&circuit, offset, &rhs, &NoAbort).unwrap();
                let scale = expected
                    .iter()
                    .flatten()
                    .map(|v| v.norm())
                    .fold(1.0_f64, Value::max);
                for (a, b) in result
                    .spectra
                    .iter()
                    .flatten()
                    .zip(expected.iter().flatten())
                {
                    assert!(
                        (*a - *b).norm() / scale < 2e-8,
                        "{method:?} {offset:e} radial={radial}: {a:?} != {b:?}"
                    );
                }
                let predicted: Complex64 = adj
                    .sensitivities
                    .iter()
                    .flatten()
                    .zip(rhs.iter().flatten())
                    .map(|(a, b)| a.conj() * b)
                    .sum();
                let actual: Complex64 = observation
                    .iter()
                    .flatten()
                    .zip(expected.iter().flatten())
                    .map(|(c, x)| c.conj() * x)
                    .sum();
                assert!(
                    (predicted - actual).norm() / scale < 2e-8,
                    "adjoint {method:?} {offset:e}"
                );
            }
        }
        assert!(
            forward
                .solve(&circuit, 0.0, &observation, &NoAbort)
                .unwrap_err()
                .to_string()
                .contains("nonzero")
        );
        assert!(
            forward
                .solve(&circuit, 0.1, &observation, &CountingAbort::new(1))
                .is_err()
        );
        let mut wrong = orbit.clone();
        wrong[0].iter_mut().for_each(|v| *v *= 1.1);
        let mut unresolved = Linearization::prepare(
            &mut circuit,
            grid.clone(),
            &wrong,
            &config,
            &limits,
            &NoAbort,
        )
        .unwrap();
        assert!(
            unresolved
                .prepare_autonomous(&circuit, &wrong, 0, 1e-10, &limits, &NoAbort)
                .is_err()
        );
        let mut constrained = limits;
        constrained.max_matrix_unknowns = 2 * grid.len();
        assert!(
            forward
                .prepare_autonomous(&circuit, &orbit, 0, 1e-10, &constrained, &NoAbort)
                .is_err()
        );
        let mut floating = Oscillator { floating: true };
        let mut floating_orbit = orbit.clone();
        floating_orbit.push(vec![Complex64::ZERO; grid.len()]);
        let mut singular = Linearization::prepare(
            &mut floating,
            grid.clone(),
            &floating_orbit,
            &config,
            &limits,
            &NoAbort,
        )
        .unwrap();
        singular
            .prepare_autonomous(&floating, &floating_orbit, 0, 1e-10, &limits, &NoAbort)
            .unwrap();
        assert!(
            singular
                .solve(
                    &floating,
                    0.01,
                    &vec![vec![Complex64::ZERO; grid.len()]; 3],
                    &NoAbort
                )
                .is_err()
        );
    }
}
