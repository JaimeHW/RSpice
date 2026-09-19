//! Public fail-closed contracts for transient-waveform Fourier analysis.

use std::f64::consts::PI;

use rspice_core::Netlist;
use rspice_core::abort_signal::CountingAbort;
use rspice_core::analysis::fourier::{FourierAnalysis, FourierConfig, FourierError, FourierResult};
use rspice_core::netlist::OutputSymbolKind::{Device, Node};
use rspice_core::netlist::{ParseError, validate_output_symbols};

fn analyzer(fundamental: f64, harmonics: usize) -> FourierAnalysis {
    FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(harmonics))
}

fn full_period_fixture() -> (Vec<f64>, Vec<f64>) {
    let time: Vec<_> = (0..=24).map(|index| index as f64 / 24.0).collect();
    let values = vec![1.0; time.len()];
    (time, values)
}

#[test]
fn fourier_and_pss_share_finite_scale_invariant_quadrature() {
    for frequency in [1e-300, 1.0, 1e300, 1e308] {
        let period = 1.0 / frequency;
        let time = (0..=128)
            .map(|index| (index as f64 / 128.0) * period)
            .collect::<Vec<_>>();
        for amplitude in [1e-300, 1.0, 1e300] {
            let values = (0..=128)
                .map(|index| amplitude * (0.25 + (2.0 * PI * index as f64 / 128.0).sin()))
                .collect::<Vec<_>>();
            let fourier = analyzer(frequency, 1).analyze(&time, &values).unwrap();
            let mut pss = rspice_core::analysis::PssResult::new(period, 1, time.len());
            pss.time = time.clone();
            pss.waveforms[0] = rspice_core::analysis::PeriodicWaveform::from_values(values);
            let periodic = pss.harmonics(1, 1);
            for harmonics in [&fourier.harmonics, &periodic] {
                assert!((harmonics[0].magnitude / amplitude - 0.25).abs() < 2e-14);
                assert!((harmonics[1].magnitude / amplitude - 1.0).abs() < 2e-14);
                assert!((harmonics[1].phase + 90.0).abs() < 2e-12);
            }
        }
    }
}

#[test]
fn fourier_dc_retains_small_terms_between_canceling_large_contributions() {
    let time = (0..=128)
        .map(|index| index as f64 / 128.0)
        .collect::<Vec<_>>();
    for (large, small) in [(1e16, 1.0), (1e300, 1e-300)] {
        let mut values = vec![small; time.len()];
        values[0] = 0.0;
        values[1] = large;
        values[127] = -large;
        values[128] = 0.0;
        let result = analyzer(1.0, 1).analyze(&time, &values).unwrap();
        // Each of the 125 remaining interior knots has trapezoidal weight 1/128.
        assert!(
            (result.dc_component / small - 125.0 / 128.0).abs() < 2e-15,
            "{}",
            result.dc_component
        );
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
    let abort = CountingAbort::new(5);

    assert!(matches!(
        analyzer(1.0, 16).analyze_with_abort(&time, &values, &abort),
        Err(FourierError::Aborted)
    ));
    assert_eq!(abort.observed_at(), Some(6));
    assert_eq!(
        abort.polls_after_abort(),
        0,
        "qualification must return at the poll that reported cancellation"
    );
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
fn unresolved_four_symbols_are_reported_in_authored_operand_order() {
    // Xyce builds one operator per `.FOUR` operand in sequence and reports
    // each failure as it reaches it, so its diagnostic order is the authored
    // order. `P` and `W` are operators RSpice does not classify as probes;
    // that must not push them behind the operands it does classify.
    let deck = "ill-formed .FOUR outputs\n\
                VS 1 0 SIN(0 1.0 1KHZ 0 0)\n\
                R1 1 0 100\n\
                .TRAN 0 1ms\n\
                .FOUR 1KHZ I(BogoDevice1) P(BogoDevice2) W(BogoDevice3) V(2) N(3)\n\
                .END\n";
    let netlist = Netlist::parse(deck).expect("the deck itself is well formed");
    let error = validate_output_symbols(&netlist)
        .expect_err("every `.FOUR` operand names a symbol the circuit does not define");
    let ParseError::OutputSymbolValidation(error) = error else {
        panic!("expected the typed output-symbol validation failure, got {error}");
    };
    let reported = error
        .unresolved
        .iter()
        .map(|symbol| {
            (
                symbol.operator.to_ascii_uppercase(),
                symbol.symbol.clone(),
                symbol.kind,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        reported,
        vec![
            ("I".to_string(), "BogoDevice1".to_string(), Device),
            ("P".to_string(), "BogoDevice2".to_string(), Device),
            ("W".to_string(), "BogoDevice3".to_string(), Device),
            ("V".to_string(), "2".to_string(), Node),
            ("N".to_string(), "3".to_string(), Node),
        ]
    );
    assert!(
        error
            .unresolved
            .iter()
            .all(|symbol| symbol.origin.line == 5),
        "every unresolved `.FOUR` symbol keeps its authored card line"
    );
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

//=============================================================================
// The window the card names
//=============================================================================

/// A configuration integrating `periods` whole fundamental periods.
fn windowed(fundamental: f64, harmonics: usize, periods: usize) -> FourierConfig {
    let mut config = FourierConfig::new(fundamental).with_harmonics(harmonics);
    config.num_periods = periods;
    config
}

/// Oracle 1. A waveform built from a constant, a fundamental sine and a third
/// harmonic cosine decomposes back into exactly those terms, and the answer
/// does not depend on how many whole periods of it are integrated.
///
/// Tolerance 1e-12 on the magnitudes: the trapezoidal rule applied to a
/// trigonometric polynomial over a whole number of periods on a uniform grid
/// of 256 points per period is exact up to rounding for every harmonic below
/// 128, so what is asserted here is accumulated floating-point error and
/// nothing else. Phases are asserted to 1e-10 degrees because a coefficient
/// error of `e` shows up in the angle as `e/|c| * 180/pi`, two orders larger;
/// a wrong phase reference would be off by 90 degrees or more.
#[test]
fn a_sine_with_a_third_harmonic_has_the_coefficients_it_was_built_from() {
    const FUNDAMENTAL: f64 = 1.0;
    const PER_PERIOD: usize = 256;
    const RECORD_PERIODS: usize = 8;
    const OFFSET: f64 = 0.75;
    const AMPLITUDE: f64 = 2.0;
    const PHASE: f64 = 0.3;
    const THIRD: f64 = 0.5;

    let samples = RECORD_PERIODS * PER_PERIOD;
    let time = (0..=samples)
        .map(|index| index as f64 / PER_PERIOD as f64)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|&t| {
            OFFSET
                + AMPLITUDE * (2.0 * PI * FUNDAMENTAL * t + PHASE).sin()
                + THIRD * (2.0 * PI * 3.0 * FUNDAMENTAL * t).cos()
        })
        .collect::<Vec<_>>();

    // The record ends a whole number of periods after it starts, so every
    // window below begins on a period boundary and shares one cosine
    // reference: a sine of phase `PHASE` is a cosine of `PHASE - pi/2`.
    let expected_fundamental_phase = (PHASE - std::f64::consts::FRAC_PI_2).to_degrees();
    for periods in [1, 4] {
        let result = FourierAnalysis::new(windowed(FUNDAMENTAL, 5, periods))
            .analyze(&time, &values)
            .unwrap_or_else(|error| panic!("{periods} period(s) must decompose: {error}"));
        assert!(
            (result.dc_component - OFFSET).abs() < 1e-12,
            "{periods}: DC {} != {OFFSET}",
            result.dc_component
        );
        let first = result.harmonic(1).expect("fundamental");
        assert!(
            (first.magnitude - AMPLITUDE).abs() < 1e-12,
            "{periods}: |c1| {} != {AMPLITUDE}",
            first.magnitude
        );
        assert!(
            (first.phase - expected_fundamental_phase).abs() < 1e-10,
            "{periods}: arg c1 {} != {expected_fundamental_phase}",
            first.phase
        );
        let third = result.harmonic(3).expect("third harmonic");
        assert!(
            (third.magnitude - THIRD).abs() < 1e-12,
            "{periods}: |c3| {} != {THIRD}",
            third.magnitude
        );
        assert!(
            third.phase.abs() < 1e-10,
            "{periods}: arg c3 {}",
            third.phase
        );
        for absent in [2, 4, 5] {
            let component = result.harmonic(absent).expect("requested harmonic");
            assert!(
                component.magnitude < 1e-12,
                "{periods}: harmonic {absent} is not in the waveform, got {}",
                component.magnitude
            );
        }
        let thd = result.thd.expect("a non-zero fundamental defines THD");
        assert!(
            (thd - 100.0 * THIRD / AMPLITUDE).abs() < 1e-10,
            "{periods}: THD {thd}"
        );
    }
}

/// Oracle 2. A half-wave rectified sine of amplitude `A` has the series
/// `A/pi + (A/2) sin(wt) - (2A/pi) sum_m cos(2 m wt)/(4 m^2 - 1)`: every odd
/// harmonic above the fundamental is absent, and THD follows from those terms.
///
/// Tolerance 1e-7: the integrand's derivative jumps by `2 pi A` where the
/// rectifier turns off, and Euler-Maclaurin leaves that single kink as the
/// whole error of the composite rule, `(h^2/12) 2 pi A` per coefficient, about
/// 6e-9 at 16384 samples per period. Every term asserted below differs from
/// its neighbours by more than 1e-2.
#[test]
fn a_half_wave_rectified_sine_has_its_closed_form_series() {
    const FUNDAMENTAL: f64 = 1.0;
    const PER_PERIOD: usize = 16_384;
    const AMPLITUDE: f64 = 1.5;
    const HARMONICS: usize = 9;
    const TOLERANCE: f64 = 1e-7;

    let samples = 2 * PER_PERIOD;
    let time = (0..=samples)
        .map(|index| index as f64 / PER_PERIOD as f64)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|&t| AMPLITUDE * (2.0 * PI * FUNDAMENTAL * t).sin().max(0.0))
        .collect::<Vec<_>>();

    let result = FourierAnalysis::new(windowed(FUNDAMENTAL, HARMONICS, 1))
        .analyze(&time, &values)
        .expect("a rectified sine has a Fourier series");

    let closed_form = |n: usize| -> f64 {
        match n {
            0 => AMPLITUDE / PI,
            1 => AMPLITUDE / 2.0,
            even if even.is_multiple_of(2) => {
                let m = (even / 2) as f64;
                2.0 * AMPLITUDE / (PI * (4.0 * m * m - 1.0))
            }
            _ => 0.0,
        }
    };

    assert!(
        (result.dc_component - closed_form(0)).abs() < TOLERANCE,
        "DC {} != A/pi {}",
        result.dc_component,
        closed_form(0)
    );
    for n in 1..=HARMONICS {
        let component = result.harmonic(n).expect("requested harmonic");
        assert!(
            (component.magnitude - closed_form(n)).abs() < TOLERANCE,
            "harmonic {n}: {} != {}",
            component.magnitude,
            closed_form(n)
        );
    }
    // The window starts on a zero crossing of the generating sine, so the
    // fundamental is a pure sine and every even term is a negated cosine.
    let fundamental_phase = result.harmonic(1).expect("fundamental").phase;
    assert!(
        (fundamental_phase + 90.0).abs() < 1e-5,
        "arg c1 {fundamental_phase}"
    );
    for m in 1..=4 {
        let phase = result.harmonic(2 * m).expect("even harmonic").phase;
        assert!(
            (phase.abs() - 180.0).abs() < 1e-5,
            "arg c{} = {phase}",
            2 * m
        );
    }

    let harmonic_norm = (2..=HARMONICS)
        .map(|n| closed_form(n) * closed_form(n))
        .sum::<f64>()
        .sqrt();
    let expected_thd = 100.0 * harmonic_norm / closed_form(1);
    let thd = result.thd.expect("THD is defined");
    assert!(
        (thd - expected_thd).abs() < 1e-5,
        "THD {thd} != {expected_thd}"
    );
}

/// Oracle 3. `exp(-t/tau)` has mean
/// `tau (exp(-t0/tau) - exp(-t1/tau)) / (t1 - t0)` over `[t0, t1]`, so the DC
/// term alone says which interval was integrated. Three windows over one
/// record: two ending where the card says, one ending at the record.
///
/// Tolerance 1e-8: the composite trapezoid on a smooth exponential errs by
/// `(h^2/12)(f'(t0) - f'(t1))/(t1 - t0)`, about 1.6e-10 here, while the three
/// windows are 0.027 apart.
#[test]
fn the_window_ends_where_the_card_says() {
    const TAU: f64 = 1e-3;
    const FUNDAMENTAL: f64 = 1_000.0;
    const STEP: f64 = 2.5e-7;
    const SAMPLES: usize = 20_000;

    let time = (0..=SAMPLES)
        .map(|index| index as f64 * STEP)
        .collect::<Vec<_>>();
    let values = time.iter().map(|&t| (-t / TAU).exp()).collect::<Vec<_>>();
    let mean =
        |start: f64, stop: f64| TAU * ((-start / TAU).exp() - (-stop / TAU).exp()) / (stop - start);

    for (stop, periods, start) in [
        (Some(4e-3), 1, 3e-3),
        (Some(4e-3), 2, 2e-3),
        (None, 1, 4e-3),
    ] {
        let mut config = windowed(FUNDAMENTAL, 4, periods);
        config.window_stop = stop;
        let end = stop.unwrap_or(SAMPLES as f64 * STEP);
        let result = FourierAnalysis::new(config)
            .analyze(&time, &values)
            .unwrap_or_else(|error| panic!("{stop:?}/{periods}: {error}"));
        let expected = mean(start, end);
        assert!(
            (result.dc_component - expected).abs() < 1e-8,
            "{stop:?}/{periods}: DC {} != {expected}",
            result.dc_component
        );
    }
}

/// Oracle 4. `x = t` has mean `(t0 + t1)/2` over any interval, and the
/// trapezoidal rule is exact for it on any partition. With both window edges
/// falling strictly between samples of a non-uniform grid, that mean is the
/// witness that neither edge was snapped to a neighbouring sample: snapping
/// either would move the answer by about a sample spacing, 1e-2 relative,
/// while the assertion below holds to 1e-12.
#[test]
fn both_window_edges_are_interpolated_between_samples() {
    const FUNDAMENTAL: f64 = 16.0;
    const PERIODS: usize = 3;
    const STOP: f64 = 0.5013;

    let mut time = vec![0.0_f64];
    let mut coarse = false;
    while *time.last().expect("seeded") < 0.6 {
        let next = time.last().expect("seeded") + if coarse { 0.005 } else { 0.002 };
        coarse = !coarse;
        time.push(next);
    }
    let values = time.clone();

    let start = STOP - PERIODS as f64 / FUNDAMENTAL;
    for edge in [start, STOP] {
        let above = time.partition_point(|&sample| sample < edge);
        assert!(
            above > 0 && above < time.len() && time[above] > edge && time[above - 1] < edge,
            "{edge} must fall strictly between two samples, not on one"
        );
    }

    let mut config = windowed(FUNDAMENTAL, 1, PERIODS);
    config.window_stop = Some(STOP);
    let result = FourierAnalysis::new(config)
        .analyze(&time, &values)
        .expect("a ramp over an interpolated window has a mean");
    let expected = 0.5 * (start + STOP);
    assert!(
        (result.dc_component - expected).abs() < 1e-12,
        "DC {} != (t0 + t1)/2 = {expected}",
        result.dc_component
    );
}

/// One period of a 1 kHz sine sampled at 1 us, long enough for four.
fn four_millisecond_sine() -> (Vec<f64>, Vec<f64>) {
    let time = (0..=4_000)
        .map(|index| index as f64 * 1e-6)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|&t| (2.0 * PI * 1_000.0 * t).sin())
        .collect::<Vec<_>>();
    (time, values)
}

/// Oracle 5a. A window that reaches back past the authored earliest start is
/// refused, naming both times, rather than quietly shortened: shortening it
/// would integrate a fraction of a period and bias every coefficient.
#[test]
fn a_window_that_starts_before_from_is_refused_by_name() {
    let (time, values) = four_millisecond_sine();

    let mut config = windowed(1_000.0, 4, 3);
    config.window_stop = Some(4e-3);
    config.earliest_start = Some(2e-3);
    let error = FourierAnalysis::new(config)
        .analyze(&time, &values)
        .expect_err("a window reaching back past FROM must fail closed");
    let rendered = error.to_string();
    assert!(
        matches!(
            error,
            FourierError::WindowStartsBeforeEarliestStart { start, earliest }
                if (start - 1e-3).abs() < 1e-12 && (earliest - 2e-3).abs() < 1e-12
        ),
        "{rendered}"
    );
    assert!(
        rendered.contains("before FROM = 0.002 s")
            && rendered.contains("lower PERIODS or FROM, or raise TO"),
        "{rendered}"
    );

    // The same guard with a window that clears it runs.
    let mut config = windowed(1_000.0, 4, 2);
    config.window_stop = Some(4e-3);
    config.earliest_start = Some(2e-3);
    FourierAnalysis::new(config)
        .analyze(&time, &values)
        .expect("a window that starts exactly at FROM is admissible");
}

/// Oracle 5b. A window end the run never reached is refused, naming the record
/// it would have had to lie inside, rather than silently becoming the last
/// accepted time and reporting a different spectrum.
#[test]
fn a_window_end_outside_the_record_is_refused_by_name() {
    let (time, values) = four_millisecond_sine();

    for stop in [5e-3, f64::NAN, 0.0] {
        let mut config = windowed(1_000.0, 4, 1);
        config.window_stop = Some(stop);
        let error = FourierAnalysis::new(config)
            .analyze(&time, &values)
            .expect_err("a window end outside the record must fail closed");
        assert!(
            matches!(error, FourierError::WindowStopOutsideRecord { .. }),
            "{stop}: {error}"
        );
        assert!(
            error.to_string().contains("no later than the last at"),
            "{stop}: {error}"
        );
    }

    // An end one rounding step past the record's own last time is that time: a
    // deck writing `TO=4m` beside `.TRAN ... 4m` asks for the record it got.
    let last = time[time.len() - 1];
    let mut config = windowed(1_000.0, 4, 1);
    config.window_stop = Some(last + last * f64::EPSILON);
    FourierAnalysis::new(config)
        .analyze(&time, &values)
        .expect("TO at the record's own end is the record's end");
}
