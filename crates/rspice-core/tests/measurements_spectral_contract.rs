//! External public-API contracts for waveform FFT, dominant-frequency, and THD helpers.

use std::f64::consts::PI;

use rspice_core::analysis::measurements::Waveform;

fn uniform_time(sample_count: usize, sample_rate: f64) -> Vec<f64> {
    (0..sample_count)
        .map(|index| index as f64 / sample_rate)
        .collect()
}

fn fft_waveform(sample_count: usize, sample_rate: f64, value: impl Fn(usize) -> f64) -> Waveform {
    let time = uniform_time(sample_count, sample_rate);
    let values: Vec<_> = (0..sample_count).map(value).collect();
    Waveform::new(&time, &values).expect("finite uniform FFT fixture is valid")
}

fn periodic_waveform(
    fundamental: f64,
    sample_rate: f64,
    periods: usize,
    value: impl Fn(f64) -> f64,
) -> Waveform {
    let intervals_per_period = sample_rate / fundamental;
    assert_eq!(
        intervals_per_period,
        intervals_per_period.round(),
        "fixture requires an integer number of intervals per period"
    );
    let interval_count = intervals_per_period as usize * periods;
    let time: Vec<_> = (0..=interval_count)
        .map(|index| index as f64 / sample_rate)
        .collect();
    let values: Vec<_> = time.iter().copied().map(value).collect();
    Waveform::new(&time, &values).expect("finite periodic THD fixture is valid")
}

fn assert_close(actual: f64, expected: f64, absolute: f64, relative: f64) {
    let tolerance = absolute.max(relative * expected.abs());
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual:.17e}, expected={expected:.17e}, tolerance={tolerance:.3e}"
    );
}

fn db(amplitude: f64) -> f64 {
    20.0 * amplitude.log10()
}

#[test]
fn fft_reports_analytic_dc_and_one_sided_peak_amplitudes() {
    const N: usize = 64;
    let waveform = fft_waveform(N, N as f64, |index| {
        let angle = 2.0 * PI * index as f64 / N as f64;
        1.25 + 2.0 * (5.0 * angle + 0.3).cos() + 0.5 * (10.0 * angle - 0.7).cos()
    });
    let (frequencies, magnitudes_db) = waveform.fft().expect("analytic FFT qualifies");

    assert_eq!(frequencies.len(), N / 2 + 1);
    assert_eq!(magnitudes_db.len(), frequencies.len());
    assert_close(frequencies[5], 5.0, 1.0e-12, 0.0);
    assert_close(frequencies[10], 10.0, 1.0e-12, 0.0);
    assert_close(magnitudes_db[0], db(1.25), 2.0e-10, 0.0);
    assert_close(magnitudes_db[5], db(2.0), 2.0e-10, 0.0);
    assert_close(magnitudes_db[10], db(0.5), 2.0e-10, 0.0);
}

#[test]
fn arbitrary_length_fft_keeps_original_bin_count_and_amplitude() {
    const N: usize = 15;
    const SAMPLE_RATE: f64 = 150.0;
    const BIN: usize = 3;
    const AMPLITUDE: f64 = 1.75;
    let waveform = fft_waveform(N, SAMPLE_RATE, |index| {
        AMPLITUDE * (2.0 * PI * BIN as f64 * index as f64 / N as f64).cos()
    });
    let (frequencies, magnitudes_db) = waveform.fft().expect("arbitrary-N FFT qualifies");

    assert_eq!(frequencies.len(), N / 2 + 1);
    assert_eq!(magnitudes_db.len(), N / 2 + 1);
    assert_close(
        frequencies[BIN],
        BIN as f64 * SAMPLE_RATE / N as f64,
        1.0e-12,
        0.0,
    );
    assert_close(magnitudes_db[BIN], db(AMPLITUDE), 2.0e-10, 0.0);
}

#[test]
fn odd_length_final_positive_bin_is_doubled_like_an_interior_bin() {
    const N: usize = 15;
    const SAMPLE_RATE: f64 = 150.0;
    const BIN: usize = N / 2;
    const AMPLITUDE: f64 = 0.75;
    let waveform = fft_waveform(N, SAMPLE_RATE, |index| {
        AMPLITUDE * (2.0 * PI * BIN as f64 * index as f64 / N as f64).cos()
    });
    let (frequencies, magnitudes_db) = waveform
        .fft()
        .expect("odd-length top positive bin qualifies");

    assert_eq!(frequencies.len(), BIN + 1);
    assert_close(
        frequencies[BIN],
        BIN as f64 * SAMPLE_RATE / N as f64,
        1.0e-12,
        0.0,
    );
    assert_close(magnitudes_db[BIN], db(AMPLITUDE), 2.0e-10, 0.0);
}

#[test]
fn fft_does_not_double_dc_or_nyquist() {
    const N: usize = 16;
    let waveform = fft_waveform(N, N as f64, |index| {
        2.0 + if index % 2 == 0 { 0.75 } else { -0.75 }
    });
    let (_, magnitudes_db) = waveform.fft().expect("DC/Nyquist FFT qualifies");

    assert_close(magnitudes_db[0], db(2.0), 2.0e-10, 0.0);
    assert_close(magnitudes_db[N / 2], db(0.75), 2.0e-10, 0.0);
}

#[test]
fn fft_scales_extreme_finite_nyquist_without_transform_overflow() {
    const N: usize = 16;
    let waveform = fft_waveform(N, N as f64, |index| {
        if index % 2 == 0 { f64::MAX } else { -f64::MAX }
    });
    let (_, magnitudes_db) = waveform
        .fft()
        .expect("finite full-scale Nyquist waveform must remain representable");

    let nyquist_db = magnitudes_db[N / 2];
    assert!(
        nyquist_db.is_finite(),
        "Nyquist magnitude overflowed despite a finite authored amplitude: {nyquist_db}"
    );
    assert_close(nyquist_db, db(f64::MAX), 2.0e-10, 0.0);
}

#[test]
fn zero_and_subnormal_scale_spectra_are_not_floored() {
    const N: usize = 16;
    let zero = fft_waveform(N, N as f64, |_| 0.0);
    let (_, zero_db) = zero.fft().expect("zero FFT is valid");
    assert!(
        zero_db.iter().all(|value| *value == f64::NEG_INFINITY),
        "zero spectrum must remain exact negative infinity in dB: {zero_db:?}"
    );

    let tiny = fft_waveform(N, N as f64, |index| {
        1.0e-200 * (2.0 * PI * 2.0 * index as f64 / N as f64).cos()
    });
    let (_, tiny_db) = tiny.fft().expect("tiny finite FFT is valid");
    assert_close(tiny_db[2], -4_000.0, 1.0e-9, 0.0);
    assert_eq!(
        tiny.dominant_frequency()
            .expect("tiny finite spectrum qualifies"),
        Some(2.0)
    );
}

#[test]
fn fft_rejects_obvious_nonuniformity_but_accepts_roundoff_level_uniformity() {
    const N: usize = 16;
    let mut nonuniform_time = uniform_time(N, N as f64);
    nonuniform_time[5] += 1.0 / 64.0;
    let nonuniform_values: Vec<_> = nonuniform_time
        .iter()
        .map(|time| (2.0 * PI * 2.0 * time).cos())
        .collect();
    let nonuniform = Waveform::new(&nonuniform_time, &nonuniform_values)
        .expect("nonuniform data remains valid for nonspectral measurements");
    assert!(nonuniform.fft().is_err());
    assert!(nonuniform.dominant_frequency().is_err());

    let mut time = Vec::with_capacity(32);
    let mut current = 0.0;
    for _ in 0..32 {
        time.push(current);
        current += 0.1;
    }
    let values: Vec<_> = (0..32)
        .map(|index| (2.0 * PI * 4.0 * index as f64 / 32.0).cos())
        .collect();
    let roundoff = Waveform::new(&time, &values).expect("roundoff-level grid is valid");
    assert!(roundoff.fft().is_ok());
    let dominant = roundoff
        .dominant_frequency()
        .expect("roundoff-level uniform grid qualifies")
        .expect("fixture has non-DC energy");
    assert_close(dominant, 1.25, 1.0e-12, 0.0);
}

#[test]
fn fft_accepts_long_mathematically_uniform_decimal_grid() {
    const SAMPLE_COUNT: usize = 10_000;
    let time: Vec<_> = (0..SAMPLE_COUNT)
        .map(|index| index as f64 / 1_000.0)
        .collect();
    let values = vec![0.0; SAMPLE_COUNT];
    let waveform = Waveform::new(&time, &values).expect("long finite grid is a valid waveform");

    let (frequencies, magnitudes_db) = waveform
        .fft()
        .expect("roundoff accumulated across a long uniform grid must qualify");
    assert_eq!(frequencies.len(), SAMPLE_COUNT / 2 + 1);
    assert!(
        magnitudes_db
            .iter()
            .all(|magnitude| *magnitude == f64::NEG_INFINITY),
        "zero-valued long-grid spectrum must remain exactly zero"
    );
}

#[test]
fn fft_rejects_finite_increasing_timestamps_with_overflowing_span() {
    let time = [
        -f64::MAX,
        -1.5e308,
        -1.0e308,
        -1.0,
        0.0,
        1.0,
        1.0e308,
        1.5e308,
        f64::MAX,
    ];
    assert!(time.iter().all(|timestamp| timestamp.is_finite()));
    assert!(time.windows(2).all(|pair| pair[1] > pair[0]));
    assert!((time[time.len() - 1] - time[0]).is_infinite());
    let values = [0.0; 9];
    let waveform = Waveform::new(&time, &values)
        .expect("finite strictly increasing endpoints form a public waveform");

    assert!(
        waveform.fft().is_err(),
        "overflowing total duration must fail FFT qualification"
    );
}

#[test]
fn fft_rejects_grid_when_large_time_origin_obscures_nominal_interval() {
    const SAMPLE_COUNT: usize = 64;
    const ORIGIN: f64 = 1.0e12;
    // At this origin the interval is exactly eight timestamp ULPs: the stored
    // grid looks perfectly uniform, but its time resolution is materially too
    // coarse to authenticate a spectral sample rate.
    const NOMINAL_INTERVAL: f64 = 0.000_976_562_5;
    let time: Vec<_> = (0..SAMPLE_COUNT)
        .map(|index| ORIGIN + index as f64 * NOMINAL_INTERVAL)
        .collect();
    assert!(
        time.windows(2).all(|pair| pair[1] > pair[0]),
        "fixture timestamps must remain strictly increasing"
    );
    assert!(
        time.windows(2)
            .all(|pair| pair[1] - pair[0] == NOMINAL_INTERVAL),
        "fixture must have exactly equal stored intervals"
    );
    let values = vec![0.0; SAMPLE_COUNT];
    let waveform = Waveform::new(&time, &values).expect("large-origin record is a valid waveform");

    assert!(
        waveform.fft().is_err(),
        "timestamp quantization material to the nominal interval must not be authenticated as uniform"
    );
}

#[test]
fn dominant_frequency_is_none_without_ac_energy_and_uses_lower_bin_for_ties() {
    let zero = fft_waveform(16, 16.0, |_| 0.0);
    let constant = fft_waveform(16, 16.0, |_| 3.0);
    assert_eq!(zero.dominant_frequency().expect("zero FFT qualifies"), None);
    assert_eq!(
        constant
            .dominant_frequency()
            .expect("constant FFT qualifies"),
        None
    );

    let strongest = fft_waveform(16, 16.0, |index| {
        let angle = 2.0 * PI * index as f64 / 16.0;
        0.25 * (2.0 * angle).cos() + (5.0 * angle).cos()
    });
    assert_eq!(
        strongest
            .dominant_frequency()
            .expect("finite spectrum qualifies"),
        Some(5.0)
    );

    // This exact impulse pair has identical DFT magnitudes at positive bins
    // one and three, so deterministic tie-breaking must retain the lower bin.
    let tie_values = [2.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0];
    let tie_time = uniform_time(tie_values.len(), tie_values.len() as f64);
    let tie = Waveform::new(&tie_time, &tie_values).expect("tie fixture is valid");
    assert_eq!(
        tie.dominant_frequency().expect("tie spectrum qualifies"),
        Some(1.0)
    );
}

#[test]
fn dominant_frequency_preserves_one_ulp_ac_variation_above_large_dc() {
    const N: usize = 16;
    const DC: f64 = 4_503_599_627_370_496.0; // 2^52; next_up is exactly DC + 1.
    assert_eq!(DC.next_up(), DC + 1.0);
    let waveform = fft_waveform(
        N,
        N as f64,
        |index| {
            if index % 2 == 0 { DC } else { DC + 1.0 }
        },
    );

    assert_eq!(
        waveform
            .dominant_frequency()
            .expect("one-ULP AC component remains representable"),
        Some(N as f64 / 2.0),
        "mean removal must not erase the Nyquist variation"
    );
}

#[test]
fn spectral_methods_reject_records_below_the_minimum_fft_size() {
    let short = fft_waveform(7, 8.0, |index| index as f64);
    assert!(short.fft().is_err());
    assert!(short.dominant_frequency().is_err());
    assert!(short.thd(1.0, 3).is_err());
}

#[test]
fn thd_rejects_invalid_requests_duration_and_incomplete_bandwidth() {
    let qualified = periodic_waveform(10.0, 480.0, 1, |time| (2.0 * PI * 10.0 * time).cos());
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(qualified.thd(invalid, 3).is_err());
    }
    assert!(qualified.thd(10.0, 0).is_err());
    assert!(qualified.thd(10.0, 1).is_err());

    let short_time: Vec<_> = (0..=24).map(|index| index as f64 / 480.0).collect();
    let short_values: Vec<_> = short_time
        .iter()
        .map(|time| (2.0 * PI * 10.0 * time).cos())
        .collect();
    let short = Waveform::new(&short_time, &short_values).expect("short record is finite");
    assert!(short.thd(10.0, 3).is_err());

    let undersampled = periodic_waveform(10.0, 240.0, 1, |time| (2.0 * PI * 10.0 * time).cos());
    assert!(undersampled.thd(10.0, 4).is_err());
}

#[test]
fn thd_accepts_dense_nonuniform_full_period_that_fft_rejects() {
    const F0: f64 = 10.0;
    const INTERVALS: usize = 64;
    let base_interval = 1.0 / (F0 * INTERVALS as f64);
    let mut time = Vec::with_capacity(INTERVALS + 1);
    time.push(0.0);
    for index in 0..INTERVALS {
        let factor = if index % 2 == 0 { 0.75 } else { 1.25 };
        time.push(time[index] + factor * base_interval);
    }
    assert_close(
        *time.last().expect("fixture is nonempty"),
        1.0 / F0,
        1.0e-15,
        0.0,
    );
    let values: Vec<_> = time
        .iter()
        .map(|time| {
            let phase = 2.0 * PI * F0 * time;
            2.0 * phase.cos() + 0.5 * (2.0 * phase + 0.4).cos()
        })
        .collect();
    let waveform = Waveform::new(&time, &values).expect("strictly increasing waveform is valid");

    assert!(
        waveform.fft().is_err(),
        "FFT must reject the deliberately nonuniform grid"
    );
    let thd = waveform
        .thd(F0, 2)
        .expect("qualified Fourier integration accepts nonuniform sampling")
        .expect("nonzero fundamental defines THD");
    // The alternating intervals are symmetric over the complete period, so
    // trapezoidal integration preserves these coherent harmonic coefficients
    // to floating-point roundoff despite rejecting the FFT's uniform-grid
    // contract.
    assert_close(thd, 25.0, 1.0e-10, 0.0);
}

#[test]
fn thd_includes_requested_hd2_and_hd3_and_is_scale_invariant() {
    const F0: f64 = 10.0;
    const FS: f64 = 480.0;
    let signal = |time: f64| {
        let phase = 2.0 * PI * F0 * time;
        2.0 * (phase + 0.2).cos()
            + 0.5 * (2.0 * phase - 0.4).cos()
            + 0.25 * (3.0 * phase + 0.7).cos()
    };
    let waveform = periodic_waveform(F0, FS, 2, signal);
    let hd2 = waveform
        .thd(F0, 2)
        .expect("HD2 request qualifies")
        .expect("nonzero fundamental defines THD");
    let hd3 = waveform
        .thd(F0, 3)
        .expect("HD3 request qualifies")
        .expect("nonzero fundamental defines THD");
    assert_close(hd2, 25.0, 1.0e-9, 1.0e-10);
    assert_close(hd3, 100.0 * 0.5_f64.hypot(0.25) / 2.0, 1.0e-9, 1.0e-10);

    let tiny = periodic_waveform(F0, FS, 2, |time| 1.0e-200 * signal(time));
    let tiny_thd = tiny
        .thd(F0, 3)
        .expect("tiny finite harmonics qualify")
        .expect("tiny nonzero fundamental defines THD");
    assert_close(tiny_thd, hd3, 1.0e-9, 1.0e-10);
}

#[test]
fn thd_honors_explicit_f0_even_when_a_harmonic_is_stronger() {
    const F0: f64 = 10.0;
    let waveform = periodic_waveform(F0, 480.0, 2, |time| {
        let phase = 2.0 * PI * F0 * time;
        0.2 * phase.cos() + (2.0 * phase).cos()
    });
    let thd = waveform
        .thd(F0, 2)
        .expect("explicit-f0 request qualifies")
        .expect("nonzero explicit fundamental defines THD");
    assert_close(thd, 500.0, 1.0e-8, 1.0e-10);
}

#[test]
fn thd_distinguishes_zero_fundamental_from_zero_distortion() {
    const F0: f64 = 10.0;
    const FS: f64 = 480.0;
    let zero = periodic_waveform(F0, FS, 1, |_| 0.0);
    assert_eq!(zero.thd(F0, 3).expect("zero waveform qualifies"), None);

    let pure = periodic_waveform(F0, FS, 2, |time| (2.0 * PI * F0 * time + 0.37).cos());
    let thd = pure
        .thd(F0, 3)
        .expect("pure fundamental qualifies")
        .expect("nonzero pure fundamental defines THD");
    assert!(
        thd <= 1.0e-10,
        "pure fundamental should have numerical-zero THD, got {thd:.12e}%"
    );
}
