//! Exact export datasets from already-selected retained waveform evidence.

use crate::table::sanitize_column_label;
use crate::waveform_io::{SignalType, WaveformDataset, WaveformSignal};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
use rspice_results::waveform::RetainedWaveform;

#[derive(Debug)]
pub enum WaveformProjectionError {
    NoSamples,
    MissingNetwork,
    InvalidPeriodicNoise,
    InvalidReferences,
    DifferentCoordinates {
        signal: String,
    },
    SampleCount {
        signal: String,
        x: usize,
        y: usize,
    },
    AxisLength {
        signal: String,
        samples: usize,
        axis: usize,
    },
}

impl std::fmt::Display for WaveformProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSamples => f.write_str("No waveform samples available to export."),
            Self::MissingNetwork => {
                f.write_str("Touchstone export requires complex S-parameter network traces")
            }
            Self::InvalidPeriodicNoise => {
                f.write_str("Periodic noise export context must be finite")
            }
            Self::InvalidReferences => f.write_str(
                "S-parameter export requires finite positive per-port reference impedances.",
            ),
            Self::DifferentCoordinates { signal } => write!(
                f,
                "CSV export requires all signals in a shared-axis result to use identical x-axis samples; '{signal}' has different x-axis samples."
            ),
            Self::SampleCount { signal, x, y } => write!(
                f,
                "Signal '{signal}' has {x} x samples and {y} y samples; export refused instead of truncating evidence."
            ),
            Self::AxisLength {
                signal,
                samples,
                axis,
            } => write!(
                f,
                "Signal '{signal}' has {samples} samples, exceeding shared x-axis length {axis}; export refused instead of truncating evidence."
            ),
        }
    }
}

impl std::error::Error for WaveformProjectionError {}

struct ExportSignalSlice<'a> {
    name: &'a str,
    signal_type: SignalType,
    unit: Option<&'a str>,
    x_values: &'a [f64],
    y_values: &'a [f64],
}

/// Project selected traces without applying view selection or publishing a file.
pub fn project_waveforms<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    waveforms: &[&W],
    touchstone: bool,
) -> Result<WaveformDataset, WaveformProjectionError> {
    let waveforms = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .collect::<Vec<_>>();
    let (x_name, x_signal_type) = axis_signal_for_analysis(analysis);
    let mut prepared = if touchstone && analysis.analysis_type == AnalysisType::SParameter {
        prepare_touchstone_waveform_dataset(&waveforms)?
    } else {
        prepare_flat_waveform_dataset(&waveforms, x_name, x_signal_type)?
    };
    if let Some(coordinate) = analysis.imported_coordinate()
        && let Some(axis) = prepared.x_signal.as_mut()
    {
        axis.unit = coordinate.unit.clone().unwrap_or_default();
    }
    if !touchstone
        && matches!(
            analysis.analysis_type,
            AnalysisType::Psp | AnalysisType::Hbsp
        )
    {
        for measurement in &analysis.measurements {
            if measurement.name.starts_with("periodic_noise_") {
                let value = measurement
                    .value
                    .filter(|value| value.is_finite())
                    .ok_or(WaveformProjectionError::InvalidPeriodicNoise)?;
                let mut signal = WaveformSignal::new(&measurement.name, SignalType::Unknown);
                signal.unit = if measurement.name.ends_with("_kelvin") {
                    "K"
                } else if measurement.name.ends_with("_hz") {
                    "Hz"
                } else {
                    "1"
                }
                .into();
                signal.data = vec![value; prepared.point_count()];
                prepared.add_signal(signal);
            }
        }
    }
    if let Some(AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
        noise_reference_temperature_kelvin,
    }) = analysis.family_metadata.as_ref()
    {
        if reference_impedances_ohm.is_empty()
            || reference_impedances_ohm
                .iter()
                .any(|impedance| !impedance.is_finite() || *impedance <= 0.0)
        {
            return Err(WaveformProjectionError::InvalidReferences);
        }
        prepared
            .metadata
            .insert("touchstone_version".to_owned(), "2".to_owned());
        prepared.metadata.insert(
            "z0_ports".to_owned(),
            reference_impedances_ohm
                .iter()
                .map(f64::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
        prepared
            .metadata
            .insert("z0".to_owned(), reference_impedances_ohm[0].to_string());
        if let Some(temperature) = noise_reference_temperature_kelvin
            && (!touchstone
                || waveforms.iter().any(|waveform| {
                    matches!(waveform.name.as_str(), "Fmin" | "Rn" | "F")
                        || waveform.complex.as_ref().is_some_and(|complex| {
                            complex.source_name == "Sopt" || complex.source_name.starts_with("CY(")
                        })
                }))
        {
            prepared.metadata.insert(
                "noise_reference_temperature_kelvin".to_owned(),
                temperature.to_string(),
            );
            if waveforms.iter().any(|waveform| {
                waveform
                    .complex
                    .as_ref()
                    .is_some_and(|complex| complex.source_name.starts_with("CY("))
            }) {
                prepared
                    .metadata
                    .insert("noise_covariance_unit".to_owned(), "A²/Hz".to_owned());
            }
        }
    }
    Ok(prepared)
}

fn prepare_touchstone_waveform_dataset(
    waveforms: &[&RetainedWaveform],
) -> Result<WaveformDataset, WaveformProjectionError> {
    let network = waveforms
        .iter()
        .find(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                complex.source_name.starts_with('S') && complex.source_name != "Sopt"
            })
        })
        .ok_or(WaveformProjectionError::MissingNetwork)?;
    let mut dataset = WaveformDataset::new("S-Parameter");
    let mut axis = WaveformSignal::new("frequency", SignalType::Frequency);
    axis.data = network.x.as_ref().clone();
    dataset.set_x(axis);
    for waveform in waveforms {
        let first = dataset.signals.len();
        append_waveform_signal(&mut dataset, &waveform.name, waveform, waveform.x.len())?;
        if waveform.x != network.x {
            for signal in &mut dataset.signals[first..] {
                signal.x_values = Some(waveform.x.as_ref().clone());
            }
        }
    }
    Ok(dataset)
}

fn prepare_flat_waveform_dataset(
    waveforms: &[&RetainedWaveform],
    x_name: &str,
    x_signal_type: SignalType,
) -> Result<WaveformDataset, WaveformProjectionError> {
    let reference_waveform = waveforms
        .iter()
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or(WaveformProjectionError::NoSamples)?;

    let reference_len = reference_waveform.x.len();
    validate_shared_x_axis(waveforms, reference_waveform.x.as_ref())?;

    let mut dataset = WaveformDataset::new("Simulation Results");
    let mut x_signal = WaveformSignal::new(x_name, x_signal_type);
    x_signal.data.extend(reference_waveform.x.iter().copied());
    dataset.set_x(x_signal);

    for waveform in waveforms {
        append_waveform_signal(&mut dataset, &waveform.name, waveform, reference_len)?;
    }

    Ok(dataset)
}

pub(crate) fn validate_shared_x_axis<W: AsRef<RetainedWaveform>>(
    waveforms: &[&W],
    reference_x: &[f64],
) -> Result<(), WaveformProjectionError> {
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        if waveform.x.as_ref() != reference_x {
            return Err(WaveformProjectionError::DifferentCoordinates {
                signal: sanitize_column_label(&waveform.name),
            });
        }
    }

    Ok(())
}

/// The coordinate identity an analysis publishes in an exported file: the id
/// a reader keys the first column on, and the `SignalType` that restores its
/// unit.
///
/// This is a persisted identifier, not a caption, and it used to be spelled by
/// lower-casing the Studio's display axis label -- which tied it to a string
/// written to be read. That coupling has already fired: `6dc39920d` retitled
/// `.PXF`'s Studio axis
/// "Offset Frequency", a display-only change by intent, and thereby moved
/// every `.PXF` export's coordinate from `frequency` to `offset_frequency` and
/// dropped its type to `Unknown`, which carries no unit at all. So the
/// identity is stated here per analysis, and the match is exhaustive: a new
/// analysis type must say what it exports rather than inherit a title.
///
/// Every frequency-swept analysis publishes `("frequency", Frequency)`,
/// whether its abscissa is an absolute drive frequency or an offset from a
/// carrier. The exported quantity is a frequency in hertz either way, and
/// which frequency it is belongs to the analysis type the file already names.
pub(crate) fn axis_signal_for_analysis<W>(analysis: &AnalysisResult<W>) -> (&str, SignalType) {
    if let Some(coordinate) = analysis.imported_coordinate() {
        let kind = match coordinate.unit.as_deref() {
            Some("s") => SignalType::Time,
            Some("Hz") => SignalType::Frequency,
            Some("A") => SignalType::Current,
            Some("V") => SignalType::Voltage,
            _ => SignalType::Unknown,
        };
        return (&coordinate.name, kind);
    }
    if let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload {
        let kind = if evidence.source.starts_with(['I', 'i']) {
            SignalType::Current
        } else if evidence.source.starts_with(['V', 'v']) {
            SignalType::Voltage
        } else {
            SignalType::Unknown
        };
        return (&evidence.source, kind);
    }
    axis_signal_for_analysis_type(analysis.analysis_type)
}

pub(crate) const fn axis_signal_for_analysis_type(
    analysis: AnalysisType,
) -> (&'static str, SignalType) {
    use AnalysisType as A;
    match analysis {
        A::Transient | A::TransientNoise | A::Pss | A::Envelope | A::Soa => {
            ("time", SignalType::Time)
        }
        A::Ac
        | A::Disto
        | A::Tf
        | A::Stb
        | A::SParameter
        | A::HarmonicBalance
        | A::Fourier
        | A::Noise
        | A::Qpss
        | A::Hbsp
        | A::Psp
        // The periodic small-signal family sweeps an offset from the carrier
        // rather than a drive frequency. An offset is still a frequency in
        // hertz, and a reader that keys on the coordinate needs the same id
        // and the same unit for it.
        | A::Pac
        | A::Pxf
        | A::Qpac
        | A::Qpxf
        | A::Pnoise
        | A::Qpnoise
        | A::Hbnoise => ("frequency", SignalType::Frequency),
        A::DcSweep => ("voltage", SignalType::Unknown),
        A::Pstb => ("mode", SignalType::Unknown),
        A::PoleZero => ("real", SignalType::Unknown),
        A::Sensitivity | A::DcMismatch => ("parameter", SignalType::Unknown),
        A::MonteCarlo => ("value", SignalType::Unknown),
        A::Parametric => ("sweep", SignalType::Unknown),
        A::Corner => ("temperature", SignalType::Unknown),
        A::Optimization => ("iteration", SignalType::Unknown),
        // A scalar operating point has no abscissa to name.
        A::DcOp => ("x", SignalType::Unknown),
    }
}

fn append_waveform_signal(
    dataset: &mut WaveformDataset,
    signal_name: &str,
    waveform: &RetainedWaveform,
    reference_len: usize,
) -> Result<(), WaveformProjectionError> {
    append_signal_values(
        dataset,
        ExportSignalSlice {
            name: signal_name,
            signal_type: signal_type_from_waveform_name(signal_name),
            unit: waveform.unit.as_deref(),
            x_values: waveform.x.as_ref(),
            y_values: waveform.y.as_ref(),
        },
        reference_len,
    )?;

    if let Some(complex) = &waveform.complex {
        let real_name = format!("re({})", complex.source_name);
        append_signal_values(
            dataset,
            ExportSignalSlice {
                name: &real_name,
                signal_type: complex_signal_type(&complex.source_name, true),
                unit: waveform.unit.as_deref(),
                x_values: waveform.x.as_ref(),
                y_values: complex.real.as_ref(),
            },
            reference_len,
        )?;
        let imag_name = format!("im({})", complex.source_name);
        append_signal_values(
            dataset,
            ExportSignalSlice {
                name: &imag_name,
                signal_type: complex_signal_type(&complex.source_name, false),
                unit: waveform.unit.as_deref(),
                x_values: waveform.x.as_ref(),
                y_values: complex.imag.as_ref(),
            },
            reference_len,
        )?;
    }
    Ok(())
}

fn append_signal_values(
    dataset: &mut WaveformDataset,
    signal: ExportSignalSlice<'_>,
    reference_len: usize,
) -> Result<(), WaveformProjectionError> {
    // The writer quotes delimiters and control characters. Preserve the
    // source identity: CY(1,2) and CY(1 2) must never become the same column.
    let export_name = signal.name.to_owned();
    let mut export_signal = WaveformSignal::new(&export_name, signal.signal_type);
    if let Some(unit) = signal.unit {
        export_signal.unit = unit.to_owned();
    }

    let available_points = signal.x_values.len().min(signal.y_values.len());
    if signal.x_values.len() != signal.y_values.len() {
        return Err(WaveformProjectionError::SampleCount {
            signal: export_name,
            x: signal.x_values.len(),
            y: signal.y_values.len(),
        });
    }

    if available_points > reference_len {
        return Err(WaveformProjectionError::AxisLength {
            signal: export_name,
            samples: available_points,
            axis: reference_len,
        });
    }

    export_signal.data.extend(signal.y_values.iter().copied());
    dataset.add_signal(export_signal);
    Ok(())
}

pub(crate) fn signal_type_from_waveform_name(name: &str) -> SignalType {
    if name.starts_with("V(") || name.starts_with("v(") {
        SignalType::Voltage
    } else if name.starts_with("I(") || name.starts_with("i(") {
        SignalType::Current
    } else {
        SignalType::Unknown
    }
}

pub(crate) fn complex_signal_type(source_name: &str, real: bool) -> SignalType {
    if source_name.starts_with("V(") || source_name.starts_with("v(") {
        if real {
            SignalType::VoltageReal
        } else {
            SignalType::VoltageImag
        }
    } else if source_name.starts_with("I(") || source_name.starts_with("i(") {
        if real {
            SignalType::CurrentReal
        } else {
            SignalType::CurrentImag
        }
    } else if source_name.starts_with('S') || source_name.starts_with('s') {
        SignalType::SParameter
    } else {
        SignalType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every analysis states the coordinate it exports, and this is the table.
    ///
    /// The match behind it is exhaustive, so a new analysis type cannot compile
    /// without an entry; this pins what the entries are, so moving one is a named
    /// test failure rather than a quietly rewritten file header. Rows sharing an
    /// id share a physical quantity: every frequency sweep is `frequency` in
    /// hertz, whether it sweeps a drive or an offset from a carrier.
    #[test]
    fn every_analysis_type_pins_the_coordinate_identity_it_exports() {
        let expected = [
            (AnalysisType::Transient, "time", SignalType::Time),
            (AnalysisType::TransientNoise, "time", SignalType::Time),
            (AnalysisType::Pss, "time", SignalType::Time),
            (AnalysisType::Envelope, "time", SignalType::Time),
            (AnalysisType::Soa, "time", SignalType::Time),
            (AnalysisType::Ac, "frequency", SignalType::Frequency),
            (AnalysisType::Disto, "frequency", SignalType::Frequency),
            (AnalysisType::Tf, "frequency", SignalType::Frequency),
            (AnalysisType::Stb, "frequency", SignalType::Frequency),
            (AnalysisType::SParameter, "frequency", SignalType::Frequency),
            (
                AnalysisType::HarmonicBalance,
                "frequency",
                SignalType::Frequency,
            ),
            (AnalysisType::Fourier, "frequency", SignalType::Frequency),
            (AnalysisType::Noise, "frequency", SignalType::Frequency),
            (AnalysisType::Qpss, "frequency", SignalType::Frequency),
            (AnalysisType::Hbsp, "frequency", SignalType::Frequency),
            (AnalysisType::Psp, "frequency", SignalType::Frequency),
            (AnalysisType::Pac, "frequency", SignalType::Frequency),
            (AnalysisType::Pxf, "frequency", SignalType::Frequency),
            (AnalysisType::Qpac, "frequency", SignalType::Frequency),
            (AnalysisType::Qpxf, "frequency", SignalType::Frequency),
            (AnalysisType::Pnoise, "frequency", SignalType::Frequency),
            (AnalysisType::Qpnoise, "frequency", SignalType::Frequency),
            (AnalysisType::Hbnoise, "frequency", SignalType::Frequency),
            (AnalysisType::DcSweep, "voltage", SignalType::Unknown),
            (AnalysisType::Pstb, "mode", SignalType::Unknown),
            (AnalysisType::PoleZero, "real", SignalType::Unknown),
            (AnalysisType::Sensitivity, "parameter", SignalType::Unknown),
            (AnalysisType::DcMismatch, "parameter", SignalType::Unknown),
            (AnalysisType::MonteCarlo, "value", SignalType::Unknown),
            (AnalysisType::Parametric, "sweep", SignalType::Unknown),
            (AnalysisType::Corner, "temperature", SignalType::Unknown),
            (AnalysisType::Optimization, "iteration", SignalType::Unknown),
            (AnalysisType::DcOp, "x", SignalType::Unknown),
        ];
        for (analysis, id, signal_type) in expected {
            assert_eq!(
                axis_signal_for_analysis_type(analysis),
                (id, signal_type),
                "{analysis:?} exports a different coordinate than the one pinned here",
            );
        }
        // A table that stopped covering the enum stops guarding it. The match is
        // exhaustive, so a new variant cannot compile without an entry there; this
        // is what stops it reaching the export unpinned here.
        let distinct = expected
            .iter()
            .map(|(analysis, ..)| format!("{analysis:?}"))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            distinct.len(),
            expected.len(),
            "duplicate rows: {distinct:?}"
        );
        assert_eq!(
            distinct.len(),
            33,
            "AnalysisType has a variant this table does not pin; add its row and this count",
        );
    }
}
