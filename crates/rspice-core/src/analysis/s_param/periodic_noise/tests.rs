use super::*;
use crate::abort_signal::NoAbort;

#[test]
fn periodic_wave_parameters_agree_with_series_resistor_current_noise() {
    let r = 50.0;
    let covariance = [
        [
            Complex64::new(4.0 / 9.0, 0.0),
            Complex64::new(-4.0 / 9.0, 0.0),
        ],
        [
            Complex64::new(-4.0 / 9.0, 0.0),
            Complex64::new(4.0 / 9.0, 0.0),
        ],
    ];
    let result = wave_noise_parameters(
        Complex64::new(1.0 / 3.0, 0.0),
        Complex64::new(2.0 / 3.0, 0.0),
        covariance,
        r,
    )
    .unwrap();
    assert!((result.noise_factor - 2.0).abs() < 1e-12);
    assert!((result.minimum_noise_factor - 1.0).abs() < 1e-7);
    assert!((result.noise_resistance - 50.0).abs() < 1e-12);
    assert!((result.optimum_source_reflection - Complex64::ONE).norm() < 1e-7);
}

#[test]
fn periodic_wave_noise_parameters_predict_complex_mismatched_source_noise() {
    let s11 = Complex64::new(0.2, 0.1);
    let s21 = Complex64::new(2.0, -0.5);
    let c = [
        [Complex64::new(3.0, 0.0), Complex64::new(0.3, 0.7)],
        [Complex64::new(0.3, -0.7), Complex64::new(2.0, 0.0)],
    ];
    let result = wave_noise_parameters(s11, s21, c, 75.0).unwrap();
    let direct_factor = |gamma: Complex64| {
        let h = s21 / (Complex64::ONE - s11 * gamma);
        let h_noise = h * gamma;
        let output_noise =
            h_noise.norm_sqr() * c[0][0].re + c[1][1].re + 2.0 * (h_noise * c[0][1]).re;
        1.0 + output_noise / (h.norm_sqr() * (1.0 - gamma.norm_sqr()))
    };
    assert!(
        (direct_factor(result.optimum_source_reflection) - result.minimum_noise_factor).abs()
            < 1e-12
    );
    for gamma in [
        Complex64::ZERO,
        Complex64::new(0.3, 0.4),
        Complex64::new(-0.6, 0.2),
        Complex64::new(0.1, -0.8),
    ] {
        let predicted = result.minimum_noise_factor
            + 4.0 * result.noise_resistance / 75.0
                * (gamma - result.optimum_source_reflection).norm_sqr()
                / ((1.0 - gamma.norm_sqr())
                    * (Complex64::ONE + result.optimum_source_reflection).norm_sqr());
        assert!((predicted - direct_factor(gamma)).abs() < 1e-12);
        assert!(predicted >= result.minimum_noise_factor);
    }
}

#[test]
fn periodic_noise_figure_retains_image_noise_and_explicit_termination_temperature() {
    let mut scattering = SMatrix::new(1e4, 6);
    scattering.set(5, 2, Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0));
    scattering.set(5, 1, Complex64::new(0.0, std::f64::consts::FRAC_1_SQRT_2));
    let intrinsic = vec![vec![Complex64::ZERO; 6]; 6];
    let mut reference = PeriodicPortNoiseReference {
        image_sideband: Some(-1),
        ..Default::default()
    };
    let run = |reference: &PeriodicPortNoiseReference| {
        derive_periodic_port_noise_with_abort(
            &scattering,
            &intrinsic,
            &[50.0, 75.0],
            -1,
            1,
            reference,
            crate::constants::K_BOLTZMANN,
            &NoAbort,
        )
        .unwrap()
    };
    let result = run(&reference);
    assert!((result.single_sideband.noise_factor - 2.0).abs() < 1e-12);
    assert!((result.double_sideband_noise_factor.unwrap() - 1.0).abs() < 1e-12);
    reference.image_sideband = None;
    reference.termination_temperature_kelvin = 0.0;
    assert_eq!(run(&reference).single_sideband.noise_factor, 1.0);
    reference.termination_temperature_kelvin = 580.0;
    assert!((run(&reference).single_sideband.noise_factor - 3.0).abs() < 1e-12);
}

#[test]
fn periodic_noise_reference_rejects_invalid_channels_and_preserves_noiseless_through() {
    let zero = [[Complex64::ZERO; 2]; 2];
    assert_eq!(
        wave_noise_parameters(Complex64::ZERO, Complex64::ONE, zero, 50.0)
            .unwrap()
            .noise_factor,
        1.0
    );
    assert!(wave_noise_parameters(Complex64::ZERO, Complex64::ZERO, zero, 50.0).is_none());
    let mut reference = PeriodicPortNoiseReference::default();
    assert!(reference.validate(Some(2), -1, 1).is_ok());
    reference.input_port = 0;
    assert!(reference.validate(Some(2), -1, 1).is_err());
    reference.input_port = 1;
    reference.output_port = 1;
    assert!(reference.validate(Some(2), -1, 1).is_err());
    reference.output_sideband = 1;
    assert!(reference.validate(Some(2), -1, 1).is_ok());
    reference.image_sideband = Some(1);
    assert!(reference.validate(Some(2), -1, 1).is_err());
}
