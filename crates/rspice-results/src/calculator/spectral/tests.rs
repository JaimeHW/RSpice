//! Closed-form oracles for the spectral scalars, through the parser and
//! evaluator: coherent records of known tones, whose ratios are the ratios
//! of their amplitudes.
//!
//! The FFT sheet's engine refines each peak it measures by a parabola
//! through its own and its neighbours' levels in dB. On a coherent record
//! the neighbours are rounding noise, and the refinement moves a peak by a
//! few ten-thousandths of a dB, so the dB readings are held to 0.005 dB and
//! THD to 5 parts in 10 000 of itself.

use std::f64::consts::PI;

use crate::calculator::evaluator::evaluate;
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;
use crate::calculator::value::{CalcValue, RealValue};
use crate::waveform::RetainedWaveform;

fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn measure(waveforms: &[RetainedWaveform], expression: &str) -> Result<f64, String> {
    let parsed = try_parse(expression, plain_number).map_err(|error| error.to_string())?;
    match evaluate(&parsed, &WaveformsContext::new(waveforms)).map_err(|error| error.to_string())? {
        CalcValue::Real(RealValue::Scalar(value)) => Ok(value),
        other => Err(format!("{expression} is not a scalar: {other:?}")),
    }
}

#[track_caller]
fn close(waveforms: &[RetainedWaveform], expression: &str, expected: f64, tolerance: f64) {
    let actual = measure(waveforms, expression)
        .unwrap_or_else(|error| panic!("{expression} failed: {error}"));
    assert!(
        (actual - expected).abs() <= tolerance,
        "{expression}: expected {expected}, got {actual} (tolerance {tolerance})"
    );
}

/// The record's window: 1 ms, read at 1024 points, the trace drawn eight
/// times as finely so every point the record reads is one of its samples.
const SPAN: f64 = 1e-3;
const POINTS: usize = 1024;
/// The carrier: 8 periods in the window, 8 kHz.
const CARRIER_CYCLES: f64 = 8.0;
const DB_TOLERANCE: f64 = 0.005;
const THD_RELATIVE_TOLERANCE: f64 = 5e-4;

/// A trace of tones, each (cycles in the window, amplitude), over the window
/// and a little past its stop, as a transient runs past the window it is
/// read over.
fn tones(name: &str, tones: &[(f64, f64)]) -> RetainedWaveform {
    let step = SPAN / (8 * POINTS) as f64;
    let x: Vec<f64> = (0..=8 * POINTS + 64).map(|i| i as f64 * step).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|&t| {
            tones
                .iter()
                .map(|&(cycles, amplitude)| amplitude * (2.0 * PI * cycles * t / SPAN).sin())
                .sum::<f64>()
        })
        .collect();
    RetainedWaveform::new(name, x, y)
}

fn db(ratio: f64) -> f64 {
    20.0 * ratio.log10()
}

/// A carrier and one spur that is no harmonic of it: the spur is the
/// largest other bin, so SFDR is their ratio, and the only noise, so SNR and
/// SINAD are too; ENOB follows from SINAD.
#[test]
fn a_two_tone_records_sfdr_snr_and_enob_are_its_amplitude_ratio() {
    let waveforms = [tones("V(a)", &[(CARRIER_CYCLES, 1.0), (37.0, 1e-3)])];
    let window = format!("0, {SPAN}, {POINTS}");
    let ratio = db(1e3);
    close(
        &waveforms,
        &format!("sfdr(V(a), {window})"),
        ratio,
        DB_TOLERANCE,
    );
    close(
        &waveforms,
        &format!("snr(V(a), {window})"),
        ratio,
        DB_TOLERANCE,
    );
    close(
        &waveforms,
        &format!("sinad(V(a), {window})"),
        ratio,
        DB_TOLERANCE,
    );
    close(
        &waveforms,
        &format!("enob(V(a), {window})"),
        (ratio - 1.76) / 6.02,
        DB_TOLERANCE / 6.02,
    );
    close(&waveforms, &format!("thd(V(a), {window})"), 0.0, 1e-9);
}

/// Harmonics of the carrier are distortion: THD is the root sum of their
/// squares over the carrier, SINAD its reciprocal in dB with no noise, and
/// SFDR the largest of them.
#[test]
fn a_distorted_carriers_thd_is_the_root_sum_square_of_its_harmonics() {
    let (third, fifth) = (0.01, 0.005);
    let waveforms = [tones(
        "V(d)",
        &[
            (CARRIER_CYCLES, 1.0),
            (3.0 * CARRIER_CYCLES, third),
            (5.0 * CARRIER_CYCLES, fifth),
        ],
    )];
    let window = format!("0, {SPAN}, {POINTS}");
    let distortion = third.hypot(fifth);
    close(
        &waveforms,
        &format!("thd(V(d), {window})"),
        100.0 * distortion,
        100.0 * distortion * THD_RELATIVE_TOLERANCE,
    );
    close(
        &waveforms,
        &format!("sinad(V(d), {window})"),
        -db(distortion),
        DB_TOLERANCE,
    );
    close(
        &waveforms,
        &format!("sfdr(V(d), {window})"),
        db(1.0 / third),
        DB_TOLERANCE,
    );
    // Counting only the second harmonic leaves the third and fifth to noise.
    let carrier = CARRIER_CYCLES / SPAN;
    close(
        &waveforms,
        &format!("snr(V(d), {window}, {carrier}, 2)"),
        -db(distortion),
        DB_TOLERANCE,
    );
}

/// The record runs from its start up to, not including, its stop, so a
/// window of whole periods is one period of a periodic record and a pure
/// tone leaks into no other bin: its SNR is limited only by rounding.
#[test]
fn a_window_of_whole_periods_is_coherent() {
    let waveforms = [tones("V(s)", &[(CARRIER_CYCLES, 1.0)])];
    let snr = measure(&waveforms, &format!("snr(V(s), 0, {SPAN}, {POINTS})")).unwrap();
    assert!(snr > 200.0, "a coherent record's SNR is {snr} dB");
}

/// A stated fundamental is ViVA's fifth argument: the reading is the same
/// when it names the largest tone, and refused when it names another.
#[test]
fn a_stated_fundamental_must_be_the_largest_tone() {
    let waveforms = [tones("V(d)", &[(CARRIER_CYCLES, 1.0), (24.0, 0.01)])];
    let window = format!("0, {SPAN}, {POINTS}");
    let found = measure(&waveforms, &format!("thd(V(d), {window})")).unwrap();
    let carrier = CARRIER_CYCLES / SPAN;
    let stated = measure(&waveforms, &format!("thd(V(d), {window}, {carrier})")).unwrap();
    assert_eq!(
        found.to_bits(),
        stated.to_bits(),
        "{found} against {stated}"
    );
    let error = measure(&waveforms, &format!("thd(V(d), {window}, 24000)")).unwrap_err();
    assert!(error.contains("not the stated fundamental"), "{error}");
}

/// What cannot be read is refused with the reason.
#[test]
fn a_record_that_cannot_be_read_is_refused() {
    let waveforms = [tones("V(s)", &[(CARRIER_CYCLES, 1.0)])];
    for (expression, reason) in [
        (format!("thd(V(s), 0, {SPAN}, 32)"), "take more samples"),
        (
            format!("thd(V(s), 0, {SPAN}, 8)"),
            "whole number of samples",
        ),
        (
            format!("thd(V(s), 0, 1, {POINTS})"),
            "reaches past the trace",
        ),
        (format!("thd(V(s), {SPAN}, 0, {POINTS})"), "to a later stop"),
        (
            format!("thd(V(s), 0, {SPAN}, {POINTS}, 0)"),
            "positive frequency",
        ),
        (
            format!("thd(V(s), 0, {SPAN}, {POINTS}, 8000, 0)"),
            "harmonic count",
        ),
    ] {
        let error = measure(&waveforms, &expression).unwrap_err();
        assert!(error.contains(reason), "{expression}: {error}");
    }
}
