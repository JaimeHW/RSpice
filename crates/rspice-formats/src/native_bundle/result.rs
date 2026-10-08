//! Borrowed native-bundle projections from selected retained waveforms.

use super::{NativeBundleDataset, NativeBundleSignal, NativeBundleSignalValues};
use crate::WaveformDomain;
use crate::waveform_io::result::{
    WaveformProjectionError, axis_signal_for_analysis, complex_signal_type,
    signal_type_from_waveform_name, validate_shared_x_axis,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::waveform::RetainedWaveform;

/// Project selected traces for a domain already admitted by the caller.
pub fn project_native_bundle<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    waveforms: &[&'a W],
    native_analysis: WaveformDomain,
) -> Result<NativeBundleDataset<'a>, WaveformProjectionError> {
    let reference = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or(WaveformProjectionError::NoSamples)?
        .x
        .as_ref();
    validate_shared_x_axis(waveforms, reference)?;
    let (coordinate_name, _) = axis_signal_for_analysis(analysis);
    let signals = waveforms
        .iter()
        .map(|waveform| {
            let waveform = (*waveform).as_ref();
            if let Some(complex) = &waveform.complex {
                let signal_type = complex_signal_type(&complex.source_name, true);
                NativeBundleSignal {
                    name: &complex.source_name,
                    unit: waveform
                        .unit
                        .as_deref()
                        .or_else(|| nonempty_unit(signal_type.default_unit())),
                    values: NativeBundleSignalValues::Complex {
                        real: complex.real.as_ref(),
                        imag: complex.imag.as_ref(),
                    },
                }
            } else {
                let signal_type = signal_type_from_waveform_name(&waveform.name);
                NativeBundleSignal {
                    name: &waveform.name,
                    unit: waveform
                        .unit
                        .as_deref()
                        .or_else(|| nonempty_unit(signal_type.default_unit())),
                    values: NativeBundleSignalValues::Real(waveform.y.as_ref()),
                }
            }
        })
        .collect();
    Ok(NativeBundleDataset {
        analysis: native_analysis,
        coordinate_name,
        coordinate_unit: analysis.waveform_coordinate_unit(),
        coordinate: reference,
        signals,
    })
}

fn nonempty_unit(unit: &'static str) -> Option<&'static str> {
    (!unit.is_empty()).then_some(unit)
}
