//! Public fail-closed contracts for transient-waveform Fourier analysis.

use std::f64::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};

use rspice_core::Netlist;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::fourier::{FourierAnalysis, FourierConfig, FourierError, FourierResult};
use rspice_core::netlist::ParseError;

fn analyzer(fundamental: f64, harmonics: usize) -> FourierAnalysis {
    FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(harmonics))
}

fn full_period_fixture() -> (Vec<f64>, Vec<f64>) {
    let time: Vec<_> = (0..=24).map(|index| index as f64 / 24.0).collect();
    let values = vec![1.0; time.len()];
    (time, values)
}

struct AbortAfterChecks(AtomicUsize);

impl AbortSignal for AbortAfterChecks {
    fn is_aborted(&self) -> bool {
        self.0
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |remaining| {
                remaining.checked_sub(1)
            })
            .is_err()
    }
}

#[test]
fn cooperative_abort_stops_fourier_qualification_before_completion() {
    let samples = 8_193usize;
    let time = (0..samples)
        .map(|index| index as f64 / (samples - 1) as f64)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|time| (2.0 * PI * time).sin())
        .collect::<Vec<_>>();
    let abort = AbortAfterChecks(AtomicUsize::new(5));

    assert!(matches!(
        analyzer(1.0, 16).analyze_with_abort(&time, &values, &abort),
        Err(FourierError::Aborted)
    ));
}

#[test]
fn empty_and_misaligned_waveforms_fail_closed() {
    let analysis = analyzer(1.0, 3);

    assert!(matches!(
        analysis.analyze(&[], &[]),
        Err(FourierError::EmptyWaveform)
    ));
    assert!(matches!(
        analysis.analyze(&[0.0, 0.5, 1.0], &[1.0, 1.0]),
        Err(FourierError::LengthMismatch {
            time_points: 3,
            values: 2
        })
    ));
}

#[test]
fn nonfinite_time_and_value_samples_fail_with_their_indices() {
    let analysis = analyzer(1.0, 3);

    for (index, bad) in [(0, f64::NAN), (1, f64::INFINITY), (2, f64::NEG_INFINITY)] {
        let mut time = [0.0, 0.5, 1.0];
        time[index] = bad;
        assert!(matches!(
            analysis.analyze(&time, &[1.0, 1.0, 1.0]),
            Err(FourierError::NonFiniteTime {
                index: actual,
                ..
            }) if actual == index
        ));

        let mut values = [1.0, 1.0, 1.0];
        values[index] = bad;
        assert!(matches!(
            analysis.analyze(&[0.0, 0.5, 1.0], &values),
            Err(FourierError::NonFiniteValue {
                index: actual,
                ..
            }) if actual == index
        ));
    }
}

#[test]
fn invalid_fundamental_frequencies_are_rejected_before_computation() {
    let (time, values) = full_period_fixture();
    for frequency in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(matches!(
            analyzer(frequency, 3).analyze(&time, &values),
            Err(FourierError::InvalidFundamentalFrequency { .. })
        ));
    }
}

#[test]
fn four_directive_rejects_invalid_fundamentals_at_the_authored_line() {
    for authored in ["0", "-1k", "1e309"] {
        let deck =
            format!("invalid Fourier fundamental\nV1 out 0 1\n.four {authored} V(out)\n.end\n");
        let error = Netlist::parse(&deck).expect_err("invalid .FOUR fundamental must not parse");
        assert!(
            matches!(
                error,
                ParseError::Syntax { line: 3, ref message }
                    if message.contains(".FOUR fundamental frequency")
                        && message.contains("positive and finite")
            ),
            "invalid `.FOUR {authored}` lost its line-aware diagnostic: {error}"
        );
    }
}

#[test]
fn empty_basis_and_zero_period_count_are_rejected() {
    let (time, values) = full_period_fixture();
    assert!(matches!(
        analyzer(1.0, 0).analyze(&time, &values),
        Err(FourierError::NoHarmonics)
    ));

    let mut config = FourierConfig::new(1.0).with_harmonics(3);
    config.num_periods = 0;
    assert!(matches!(
        FourierAnalysis::new(config).analyze(&time, &values),
        Err(FourierError::NoPeriods)
    ));
}

#[test]
fn unrepresentable_harmonic_capacity_is_rejected_before_waveform_work() {
    assert!(matches!(
        analyzer(1.0, usize::MAX).analyze(&[], &[]),
        Err(FourierError::HarmonicCapacity {
            num_harmonics: usize::MAX
        })
    ));
}

#[test]
fn insufficient_or_nonmonotone_sampling_is_rejected() {
    let analysis = analyzer(1.0, 3);
    assert!(matches!(
        analysis.analyze(&[0.0], &[1.0]),
        Err(FourierError::InsufficientSamples { samples: 1 })
    ));
    assert!(matches!(
        analysis.analyze(&[0.0, 0.5, 0.5], &[1.0, 1.0, 1.0]),
        Err(FourierError::NonIncreasingTime {
            index: 2,
            previous: 0.5,
            current: 0.5
        })
    ));
    assert!(matches!(
        analysis.analyze(&[0.0, 0.75, 0.5], &[1.0, 1.0, 1.0]),
        Err(FourierError::NonIncreasingTime {
            index: 2,
            previous: 0.75,
            current: 0.5
        })
    ));
    assert!(matches!(
        analysis.analyze(&[0.0, 0.25, 0.5], &[1.0, 1.0, 1.0]),
        Err(FourierError::InsufficientDuration { .. })
    ));
}

#[test]
fn sample_rate_must_resolve_the_highest_requested_harmonic() {
    let time = [0.0, 0.25, 0.5, 0.75, 1.0];
    let values = [1.0, 0.0, -1.0, 0.0, 1.0];

    assert!(matches!(
        analyzer(1.0, 2).analyze(&time, &values),
        Err(FourierError::InsufficientSampleRate {
            harmonic: 2,
            interval,
            maximum
        }) if (interval - 0.25).abs() < f64::EPSILON
            && (maximum - 0.0625).abs() < f64::EPSILON
    ));
}

#[test]
fn nonfinite_computed_coefficients_are_errors_not_placeholder_results() {
    let (time, _) = full_period_fixture();
    // A full-scale square wave has a fundamental coefficient 4*MAX/pi,
    // which is not representable even though every authored sample is.
    let values: Vec<_> = time
        .iter()
        .map(|sample| {
            if (2.0 * PI * sample).cos() >= 0.0 {
                f64::MAX
            } else {
                -f64::MAX
            }
        })
        .collect();
    assert!(matches!(
        analyzer(1.0, 3).analyze(&time, &values),
        Err(FourierError::NonFiniteCoefficient { .. })
    ));
}

#[test]
fn zero_waveform_retains_a_zero_spectrum_with_undefined_thd() {
    const SAMPLES: usize = 100;
    let time: Vec<_> = (0..=SAMPLES)
        .map(|index| index as f64 / SAMPLES as f64)
        .collect();
    let values = vec![0.0; time.len()];

    let result = analyzer(1.0, 4)
        .analyze(&time, &values)
        .expect("a finite zero waveform has a valid zero spectrum");
    assert_eq!(result.thd, None);
    assert!(result.harmonics.iter().all(|component| {
        component.magnitude == 0.0 && component.frequency.is_finite() && component.phase.is_finite()
    }));
}

#[test]
fn thd_remains_scale_invariant_below_the_old_absolute_cutoff() {
    const FUNDAMENTAL: f64 = 1_000.0;
    const SAMPLES_PER_PERIOD: usize = 1_000;
    const PERIODS: usize = 2;
    let period = 1.0 / FUNDAMENTAL;
    let sample_count = PERIODS * SAMPLES_PER_PERIOD + 1;
    let mut time = Vec::with_capacity(sample_count);
    let mut values = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let t = index as f64 * period / SAMPLES_PER_PERIOD as f64;
        let phase = 2.0 * PI * FUNDAMENTAL * t;
        time.push(t);
        values.push(1.0e-20 * phase.cos() + 2.5e-21 * (2.0 * phase).cos());
    }

    let result = analyzer(FUNDAMENTAL, 4)
        .analyze(&time, &values)
        .expect("tiny but finite nonzero harmonics remain analyzable");
    assert_eq!(
        result.thd.map(|thd| (thd - 25.0).abs() < 1.0e-9),
        Some(true)
    );
}

#[test]
fn known_dc_fundamental_and_second_harmonic_oracle_remains_accurate() {
    const FUNDAMENTAL: f64 = 1_000.0;
    const SAMPLES_PER_PERIOD: usize = 1_000;
    const PERIODS: usize = 2;
    let period = 1.0 / FUNDAMENTAL;
    let sample_count = PERIODS * SAMPLES_PER_PERIOD + 1;
    let mut time = Vec::with_capacity(sample_count);
    let mut values = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let t = index as f64 * period / SAMPLES_PER_PERIOD as f64;
        let phase = 2.0 * PI * FUNDAMENTAL * t;
        time.push(t);
        values.push(
            1.25 + 2.0 * (phase + 30_f64.to_radians()).cos()
                + 0.5 * (2.0 * phase - 45_f64.to_radians()).cos(),
        );
    }

    let result: FourierResult = analyzer(FUNDAMENTAL, 4)
        .analyze(&time, &values)
        .expect("finite known-harmonic waveform has a Fourier decomposition");
    let fundamental = result
        .fundamental()
        .expect("fundamental component retained");
    let second = result.harmonic(2).expect("second harmonic retained");

    assert!((result.dc_component - 1.25).abs() < 1.0e-10);
    assert!((fundamental.magnitude - 2.0).abs() < 1.0e-10);
    assert!((fundamental.phase - 30.0).abs() < 1.0e-9);
    assert!((second.magnitude - 0.5).abs() < 1.0e-10);
    assert!((second.phase + 45.0).abs() < 1.0e-9);
    assert_eq!(
        result.thd.map(|thd| (thd - 25.0).abs() < 1.0e-9),
        Some(true)
    );
    assert!(result.harmonics.iter().all(|component| {
        component.frequency.is_finite()
            && component.magnitude.is_finite()
            && component.phase.is_finite()
    }));
}
