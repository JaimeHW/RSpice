use rspice_core::Complex64;
use rspice_core::analysis::post_processing::{
    GroupDelayResult, SfdrResult, SnrResult, ThdResult, rms,
};
use std::f64::consts::TAU;

#[test]
fn noncoherent_pure_tone_does_not_acquire_fft_bin_distortion() {
    let samples: Vec<_> = (0..1024)
        .map(|i| (TAU * 10.5 * i as f64 / 1024.0).sin())
        .collect();
    let result = ThdResult::from_waveform(&samples, 1024.0, 10.5).unwrap();
    // Piecewise waveform integration has finite sampling error; at about
    // 98 points/period its distortion must stay below 0.02%, not the 3.57%
    // artificial distortion introduced by rounding to a DFT bin.
    assert!(result.thd_percent < 0.02, "THD = {}%", result.thd_percent);
    assert!((result.fundamental_amplitude - 1.0).abs() < 0.001);
    let refined: Vec<_> = (0..4096)
        .map(|i| (TAU * 10.5 * i as f64 / 4096.0).sin())
        .collect();
    let refined = ThdResult::from_waveform(&refined, 4096.0, 10.5).unwrap();
    assert!(refined.thd_percent < result.thd_percent / 8.0);
}

#[test]
fn waveform_thd_measures_known_distortion_and_rejects_short_records() {
    let samples: Vec<_> = (0..4096)
        .map(|i| {
            let angle = TAU * 10.5 * i as f64 / 4096.0;
            angle.sin() + 0.03 * (2.0 * angle).sin() + 0.04 * (3.0 * angle).sin()
        })
        .collect();
    let result = ThdResult::from_waveform(&samples, 4096.0, 10.5).unwrap();
    assert!((result.thd_percent - 5.0).abs() < 0.01);
    assert!(ThdResult::from_waveform(&samples[..10], 4096.0, 10.5).is_err());
    assert!(ThdResult::from_waveform(&samples, f64::NAN, 10.5).is_err());
}

#[test]
fn harmonic_thd_is_scale_invariant_and_rejects_undefined_ratios() {
    for scale in [1e-300, 1.0, 1e300] {
        let result =
            ThdResult::from_harmonics(&[scale, 0.03 * scale, 0.04 * scale], 100.0).unwrap();
        assert!((result.thd_percent - 5.0).abs() < 1e-12);
    }
    assert!(ThdResult::from_harmonics(&[0.0, 1.0], 100.0).is_err());
    assert!(ThdResult::from_harmonics(&[1.0, f64::NAN], 100.0).is_err());
    assert_eq!(
        ThdResult::from_harmonics(&[1.0, 0.0], 100.0)
            .unwrap()
            .thd_db,
        f64::NEG_INFINITY
    );
}

#[test]
fn sfdr_requires_measured_carrier_and_spur_and_explicit_full_scale_units() {
    let f = [0.0, 10.0, 11.0, 20.0];
    let db = [-10.0, -3.0, -40.0, -60.0];
    assert!(SfdrResult::from_spectrum(&f, &db, 100.0, 1.0).is_err());
    let result = SfdrResult::from_spectrum(&f, &db, 10.0, 1.0).unwrap();
    assert_eq!(result.signal_freq, 10.0);
    assert_eq!(result.spur_freq, 11.0);
    assert_eq!(result.sfdr_db, 37.0);
    assert_eq!(result.sfdr_dbfs, None);
    assert_eq!(
        SfdrResult::from_spectrum_dbfs(&f, &db, 10.0, 1.0)
            .unwrap()
            .sfdr_dbfs,
        Some(40.0)
    );
    assert!(SfdrResult::from_spectrum(&[10.0], &[-3.0], 10.0, 1.0).is_err());
    assert!(SfdrResult::from_spectrum(&f, &[0.0, -3.0, f64::NAN, -60.0], 10.0, 1.0).is_err());
}

#[test]
fn group_delay_resolves_a_pure_delay_across_phase_wraps() {
    let frequencies = [0.0, 1.0, 2.0, 3.0];
    let h = frequencies.map(|f| Complex64::from_polar(1.0, -TAU * f * 0.2));
    let result = GroupDelayResult::from_transfer_function(&frequencies, &h).unwrap();
    for delay in result.delays {
        assert!((delay - 0.2).abs() < 1e-14);
    }
    assert!(result.ripple < 1e-14);
    assert!(
        GroupDelayResult::from_phase_data(&[1.0, 2.0], &[0.0, 1e20])
            .unwrap()
            .average_delay
            .is_finite()
    );
    assert!(GroupDelayResult::from_phase_data(&[1.0, 1.0], &[0.0, 1.0]).is_err());
    assert!(
        GroupDelayResult::from_transfer_function(&[1.0, 2.0], &[Complex64::new(0.0, 0.0); 2])
            .is_err()
    );
}

#[test]
fn snr_rejects_invalid_powers_and_retains_extreme_dynamic_range() {
    for noise in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(SnrResult::from_powers(1.0, noise).is_err());
    }
    assert!(SnrResult::from_powers(0.0, 0.0).is_err());
    assert_eq!(
        SnrResult::from_powers(1.0, 0.0).unwrap().snr_db,
        f64::INFINITY
    );
    assert_eq!(
        SnrResult::from_powers(0.0, 1.0).unwrap().snr_db,
        f64::NEG_INFINITY
    );
    assert_eq!(
        SnrResult::from_powers(1e300, 1e-300).unwrap().snr_db,
        6000.0
    );
}

#[test]
fn sample_rms_does_not_overflow_or_underflow_when_squaring() {
    assert_eq!(rms(&[f64::MAX, -f64::MAX]).unwrap(), f64::MAX);
    assert_eq!(rms(&[1e-300, -1e-300]).unwrap(), 1e-300);
    assert!(rms(&[]).is_err());
    assert!(rms(&[f64::NAN]).is_err());
}
