//! Borrowed native-bundle projections from selected retained waveforms.

use super::{
    NativeBundleDataset, NativeBundleMember, NativeBundleSignal, NativeBundleSignalValues,
    NativeBundleStepValue,
};
use crate::WaveformDomain;
use crate::waveform_io::result::{
    WaveformExportError, WaveformProjectionError, axis_signal_for_analysis, export_samples,
    validate_shared_x_axis,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::provenance::{AnalysisResultFamilyMember, AnalysisResultStepCoordinate};
use rspice_results::waveform::RetainedWaveform;

impl NativeBundleMember {
    /// The bundle's record of a result's family member.
    #[must_use]
    pub fn of(member: &AnalysisResultFamilyMember) -> Self {
        match member {
            AnalysisResultFamilyMember::MonteCarloTrial { index } => {
                Self::MonteCarloTrial { index: *index }
            }
            AnalysisResultFamilyMember::StepPoint { index, coordinate } => Self::StepPoint {
                index: *index,
                coordinate: coordinate
                    .iter()
                    .map(|axis| NativeBundleStepValue {
                        name: axis.name.clone(),
                        value: axis.value,
                    })
                    .collect(),
            },
        }
    }

    /// The family member a result read from the bundle is.
    #[must_use]
    pub fn into_family_member(self) -> AnalysisResultFamilyMember {
        match self {
            Self::MonteCarloTrial { index } => {
                AnalysisResultFamilyMember::MonteCarloTrial { index }
            }
            Self::StepPoint { index, coordinate } => AnalysisResultFamilyMember::StepPoint {
                index,
                coordinate: coordinate
                    .into_iter()
                    .map(|axis| AnalysisResultStepCoordinate {
                        name: axis.name,
                        value: axis.value,
                    })
                    .collect(),
            },
        }
    }
}

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
        .ok_or(WaveformProjectionError::NoSamples)?;
    validate_shared_x_axis(waveforms, &reference.x)?;
    let whole = |signal: &str, values: &'a rspice_results::waveform::WaveformValues| {
        export_samples(signal, values).map_err(|error| match error {
            WaveformExportError::Unreadable { signal, reason } => {
                WaveformProjectionError::Unreadable { signal, reason }
            }
            _ => WaveformProjectionError::NoSamples,
        })
    };
    let coordinate = whole(&reference.name, &reference.x)?;
    let (coordinate_name, _) = axis_signal_for_analysis(analysis);
    let signals = waveforms
        .iter()
        .map(|waveform| {
            let waveform = (*waveform).as_ref();
            Ok(if let Some(complex) = &waveform.complex {
                NativeBundleSignal {
                    name: &complex.source_name,
                    unit: waveform.unit.as_deref(),
                    values: NativeBundleSignalValues::Complex {
                        real: whole(&complex.source_name, &complex.real)?,
                        imag: whole(&complex.source_name, &complex.imag)?,
                    },
                }
            } else {
                NativeBundleSignal {
                    name: &waveform.name,
                    unit: waveform.unit.as_deref(),
                    values: NativeBundleSignalValues::Real(whole(&waveform.name, &waveform.y)?),
                }
            })
        })
        .collect::<Result<_, WaveformProjectionError>>()?;
    Ok(NativeBundleDataset {
        analysis: native_analysis,
        coordinate_name,
        coordinate_unit: analysis.waveform_coordinate_unit(),
        coordinate,
        signals,
        member: analysis.family_member().map(NativeBundleMember::of),
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
                        max_rows: 1_000_000,
                        max_columns: 1_024,
                        max_numeric_values: 1_000_000 / size_of::<f64>(),
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

    /// One member of a `.step` sweep or Monte Carlo study keeps which member it is, with its
    /// parameter values, through either bundle; a result that is no member writes none.
    #[test]
    fn a_family_member_round_trips_through_both_bundles() {
        use crate::native_bundle::decode_native_bundle_with_member;
        let limits = NativeBundleReadLimits {
            max_members: 10,
            max_member_bytes: 1_000_000,
            max_expanded_bytes: 1_000_000,
            max_rows: 1_000_000,
            max_columns: 1_024,
            max_numeric_values: 1_000_000 / size_of::<f64>(),
        };
        let step = AnalysisResultFamilyMember::StepPoint {
            index: 2,
            coordinate: vec![
                AnalysisResultStepCoordinate {
                    name: "r".to_owned(),
                    value: 2000.0,
                },
                AnalysisResultStepCoordinate {
                    name: "temp".to_owned(),
                    value: -40.0,
                },
            ],
        };
        let trial = AnalysisResultFamilyMember::MonteCarloTrial { index: 7 };
        for member in [Some(step), Some(trial), None] {
            let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0)
                .with_waveforms(vec![RetainedWaveform::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![0.5, 1.5],
                )]);
            if let Some(member) = &member {
                analysis.import_source = Some(rspice_results::result_import::ResultImportSource {
                    source_name: "family.raw".to_owned(),
                    format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
                    coordinate: None,
                    family_member: Some(member.clone()),
                    event_dump: None,
                });
            }
            let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
            let projected =
                project_native_bundle(&analysis, &waveforms, WaveformDomain::Transient).unwrap();
            for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
                let bytes = encode_native_bundle(kind, &projected, 1_000_000).unwrap();
                let (decoded, read) =
                    decode_native_bundle_with_member(&bytes, kind, limits).unwrap();
                assert_eq!(
                    read.map(NativeBundleMember::into_family_member),
                    member,
                    "{kind:?}"
                );
                assert_eq!(decoded.signals[0].real, vec![0.5, 1.5]);
            }
        }
    }
}
