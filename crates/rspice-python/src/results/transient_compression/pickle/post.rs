//! Post-processed products computed before waveform decimation.
//!
//! `.FOUR` spectra and transient `.MEASURE` results are computed on the exact
//! accepted trajectory, so they are persisted field for field: decimation may
//! approximate a waveform, but it may not approximate the evidence derived
//! from the undecimated one.

use super::*;

/// One `.FOUR` operand result: card index, output, physical type, authored
/// fundamental and harmonic count, then the spectrum itself.
pub(crate) type CompressedFourierPersistenceState = (
    usize,
    String,
    String,
    f64,
    usize,
    f64,
    f64,
    Option<f64>,
    Vec<(usize, f64, f64, f64)>,
);
/// One transient `.MEASURE` result, field for field.
pub(crate) type CompressedMeasurementPersistenceState = (
    String,
    Option<f64>,
    Option<f64>,
    Option<String>,
    bool,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    bool,
    Option<f64>,
);

pub(crate) fn fourier_persistence_state(
    result: &rspice_core::engine::TransientFourierResult,
) -> CompressedFourierPersistenceState {
    (
        result.card_index,
        result.output.clone(),
        result.physical_type.to_string(),
        result.fundamental,
        result.harmonic_count,
        result.spectrum.fundamental_freq,
        result.spectrum.dc_component,
        result.spectrum.thd,
        result
            .spectrum
            .harmonics
            .iter()
            .map(|harmonic| {
                (
                    harmonic.harmonic_number,
                    harmonic.frequency,
                    harmonic.magnitude,
                    harmonic.phase,
                )
            })
            .collect(),
    )
}

pub(crate) fn rebuild_fourier(
    state: CompressedFourierPersistenceState,
) -> PyResult<rspice_core::engine::TransientFourierResult> {
    let (
        card_index,
        output,
        physical_type,
        fundamental,
        harmonic_count,
        fundamental_freq,
        dc_component,
        thd,
        harmonics,
    ) = state;
    let physical_type = match physical_type.as_str() {
        "voltage" => "voltage",
        "current" => "current",
        "parameter" => "parameter",
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient Fourier physical type '{physical_type}'"
            )));
        }
    };
    Ok(rspice_core::engine::TransientFourierResult {
        card_index,
        output,
        physical_type,
        fundamental,
        harmonic_count,
        spectrum: rspice_core::analysis::FourierResult {
            fundamental_freq,
            dc_component,
            harmonics: harmonics
                .into_iter()
                .map(|(harmonic_number, frequency, magnitude, phase)| {
                    rspice_core::analysis::HarmonicComponent {
                        harmonic_number,
                        frequency,
                        magnitude,
                        phase,
                    }
                })
                .collect(),
            thd,
        },
    })
}

pub(crate) fn measurement_persistence_state(
    result: &rspice_core::MeasureResult,
) -> CompressedMeasurementPersistenceState {
    (
        result.name.clone(),
        result.value,
        result.raw_value,
        result.error.clone(),
        result.passed,
        result.expected,
        result.tolerance,
        result.failure_limit,
        result.failure_limit_exceeded,
        result.event_axis,
    )
}

pub(crate) fn rebuild_measurement(
    state: CompressedMeasurementPersistenceState,
) -> rspice_core::MeasureResult {
    let (
        name,
        value,
        raw_value,
        error,
        passed,
        expected,
        tolerance,
        failure_limit,
        failure_limit_exceeded,
        event_axis,
    ) = state;
    rspice_core::MeasureResult {
        name,
        value,
        raw_value,
        error,
        passed,
        expected,
        tolerance,
        failure_limit,
        failure_limit_exceeded,
        event_axis,
    }
}
