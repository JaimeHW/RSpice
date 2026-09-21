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
type LegacyMeasurementPersistenceState = (
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

/// Unit metadata is an optional triple; each absent symbol means explicitly
/// unknown, whereas an absent triple is a historical/untyped measurement.
type MeasurementUnitsState = Option<(Option<String>, Option<String>, Option<String>)>;
#[derive(Debug, Clone, pyo3::FromPyObject, pyo3::IntoPyObject)]
pub(crate) enum CompressedMeasurementPersistenceState {
    #[pyo3(transparent)]
    WithUnits((LegacyMeasurementPersistenceState, MeasurementUnitsState)),
    #[pyo3(transparent)]
    Legacy(LegacyMeasurementPersistenceState),
}

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
    let legacy = (
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
    );
    let symbol = |unit: &rspice_core::analysis::MeasurementUnit| unit.symbol().map(str::to_owned);
    match &result.units {
        Some(units) => CompressedMeasurementPersistenceState::WithUnits((
            legacy,
            Some((
                symbol(&units.value),
                symbol(&units.raw_value),
                symbol(&units.axis),
            )),
        )),
        None => CompressedMeasurementPersistenceState::Legacy(legacy),
    }
}

pub(crate) fn rebuild_measurement(
    state: CompressedMeasurementPersistenceState,
) -> PyResult<rspice_core::MeasureResult> {
    let (legacy, units) = match state {
        CompressedMeasurementPersistenceState::Legacy(legacy) => (legacy, None),
        CompressedMeasurementPersistenceState::WithUnits((legacy, units)) => (legacy, units),
    };
    let unit = |symbol: Option<String>| -> PyResult<rspice_core::analysis::MeasurementUnit> {
        symbol.map_or(
            Ok(rspice_core::analysis::MeasurementUnit::Unknown),
            |symbol| {
                rspice_core::analysis::MeasurementUnit::known(&symbol)
                    .map_err(crate::errors::value_error)
            },
        )
    };
    let units = units
        .map(|(value, raw_value, axis)| -> PyResult<_> {
            Ok(rspice_core::analysis::MeasurementUnits {
                value: unit(value)?,
                raw_value: unit(raw_value)?,
                axis: unit(axis)?,
            })
        })
        .transpose()?;
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
    ) = legacy;
    Ok(rspice_core::MeasureResult {
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
        units,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};

    #[test]
    fn measurement_units_preserve_python_tuple_shapes_and_physical_metadata() {
        Python::initialize();
        Python::attach(|py| {
            let mut result = rspice_core::MeasureResult::success("peak_at", 2.0);
            result.raw_value = Some(0.25);
            for typed in [false, true] {
                if typed {
                    result.units = Some(MeasurementUnits {
                        value: MeasurementUnit::Known("s".into()),
                        raw_value: MeasurementUnit::Known("V".into()),
                        axis: MeasurementUnit::Known("s".into()),
                    });
                }
                let object = measurement_persistence_state(&result)
                    .into_pyobject(py)
                    .unwrap();
                assert_eq!(object.len().unwrap(), if typed { 2 } else { 10 });
                let decoded = object
                    .extract::<CompressedMeasurementPersistenceState>()
                    .unwrap();
                assert_eq!(rebuild_measurement(decoded).unwrap(), result);
            }
            result.units.as_mut().unwrap().value = MeasurementUnit::Known("bogus".into());
            assert!(rebuild_measurement(measurement_persistence_state(&result)).is_err());
        });
    }
}
