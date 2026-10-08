//! Borrowed native-bundle projections from selected retained waveforms.

use super::{NativeBundleDataset, NativeBundleSignal, NativeBundleSignalValues};
use crate::WaveformDomain;
use crate::waveform_io::result::{
    WaveformProjectionError, axis_signal_for_analysis, validate_shared_x_axis,
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
                NativeBundleSignal {
                    name: &complex.source_name,
                    unit: waveform.unit.as_deref(),
                    values: NativeBundleSignalValues::Complex {
                        real: complex.real.as_ref(),
                        imag: complex.imag.as_ref(),
                    },
                }
            } else {
                NativeBundleSignal {
                    name: &waveform.name,
                    unit: waveform.unit.as_deref(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_bundle::{
        NativeBundleKind, NativeBundleReadLimits, decode_native_bundle, encode_native_bundle,
    };
    use rspice_results::analysis_type::AnalysisType;

    #[test]
    fn retained_bundle_round_trip_preserves_declared_and_unstated_units() {
        for unit in [None, Some("mA")] {
            let mut real = RetainedWaveform::new("V(out)", vec![0.0, 1.0], vec![-0.0, 2.0]);
            let mut complex = RetainedWaveform::new("|I(V1)|", vec![0.0, 1.0], vec![1.0, 2.0])
                .with_complex_components("I(V1)", vec![-0.0, 2.0], vec![1.0, -1.0]);
            real.unit = unit.map(str::to_owned);
            complex.unit = unit.map(str::to_owned);
            let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Retained samples", 0.0)
                .with_waveforms(vec![real, complex]);
            let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
            let projected =
                project_native_bundle(&analysis, &waveforms, WaveformDomain::Transient).unwrap();
            for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
                let bytes = encode_native_bundle(kind, &projected, 1_000_000).unwrap();
                let decoded = decode_native_bundle(
                    &bytes,
                    kind,
                    NativeBundleReadLimits {
                        max_members: 10,
                        max_member_bytes: 1_000_000,
                        max_expanded_bytes: 1_000_000,
                    },
                )
                .unwrap();
                assert_eq!(decoded.signals[0].unit.as_deref(), unit);
                assert_eq!(decoded.signals[1].unit.as_deref(), unit);
                assert_eq!(decoded.signals[0].real[0].to_bits(), (-0.0_f64).to_bits());
                assert_eq!(
                    decoded.signals[1].imag.as_deref(),
                    Some([1.0, -1.0].as_slice())
                );
            }
        }
    }
}
