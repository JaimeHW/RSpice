//! NumPy array projections from already-selected retained waveforms.

use super::{NamedArray, NumpyWriteError};
use crate::waveform_io::result::{WaveformExportError, check_export_shape, shared_coordinate};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::result_import::waveforms::WaveformImportLimits;
use rspice_results::waveform::RetainedWaveform;

pub type NumpyProjectionError = WaveformExportError;

#[derive(Debug)]
pub struct NumpySignal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
}

/// One rectangular table, with the complex signals still complex.
#[derive(Debug)]
pub struct NumpyExport {
    limits: WaveformImportLimits,
    pub coordinate_name: &'static str,
    pub coordinate: Vec<f64>,
    pub signals: Vec<NumpySignal>,
}

impl NumpyExport {
    pub fn is_complex(&self) -> bool {
        self.signals.iter().any(|signal| signal.imag.is_some())
    }

    pub fn columns(&self) -> usize {
        self.signals.len() + 1
    }
}

/// The coordinate name each analysis domain publishes under.
///
/// These three are exactly the spellings RSpice's own NPZ reader recognises as
/// a coordinate, and the ones it maps back onto an analysis domain. Publishing
/// any other name would produce an archive this product could not reopen.
fn coordinate_name(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Transient => "time",
        AnalysisType::Ac => "frequency",
        _ => "sweep",
    }
}

pub fn prepare_numpy<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    waveforms: &[&W],
    limits: WaveformImportLimits,
) -> Result<NumpyExport, NumpyProjectionError> {
    let coordinate = shared_coordinate(waveforms, limits)?;
    let mut signals = Vec::with_capacity(waveforms.len());
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        if let Some(complex) = &waveform.complex {
            signals.push(NumpySignal {
                name: complex.source_name.clone(),
                real: complex.real.as_ref().to_vec(),
                imag: Some(complex.imag.as_ref().to_vec()),
            });
        } else {
            signals.push(NumpySignal {
                name: waveform.name.clone(),
                real: waveform.y.as_ref().to_vec(),
                imag: None,
            });
        }
    }
    Ok(NumpyExport {
        limits,
        coordinate_name: coordinate_name(analysis.analysis_type),
        coordinate: coordinate.to_vec(),
        signals,
    })
}

fn borrowed_arrays(export: &NumpyExport) -> Vec<NamedArray<'_>> {
    export
        .signals
        .iter()
        .map(|signal| NamedArray {
            name: &signal.name,
            real: &signal.real,
            imag: signal.imag.as_deref(),
        })
        .collect()
}

/// One 2-D array, C order, coordinate first.
pub fn encode_npy(export: &NumpyExport) -> Result<Vec<u8>, NumpyWriteError> {
    // One complex column promotes the entire NPY matrix, including originally
    // real signals. Reopening retains three arrays per promoted signal.
    let retained_columns = export
        .signals
        .len()
        .checked_mul(if export.is_complex() { 3 } else { 1 })
        .and_then(|count| count.checked_add(1));
    check_export_shape(
        export.coordinate.len(),
        export.columns(),
        retained_columns,
        export.limits,
    )
    .map_err(NumpyWriteError::PublicationBounds)?;
    crate::numpy::matrix::encode_npy(&export.coordinate, &borrowed_arrays(export))
}

pub fn encode_npz(export: &NumpyExport) -> Result<Vec<u8>, NumpyWriteError> {
    let retained_columns = export.signals.iter().try_fold(1usize, |count, signal| {
        count.checked_add(if signal.imag.is_some() { 3 } else { 1 })
    });
    check_export_shape(
        export.coordinate.len(),
        export.columns(),
        retained_columns,
        export.limits,
    )
    .map_err(NumpyWriteError::PublicationBounds)?;
    crate::numpy::archive::encode_npz(
        export.coordinate_name,
        &export.coordinate,
        &borrowed_arrays(export),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waveform_io::result::EXPORT_TEST_LIMITS;

    #[test]
    fn complex_matrix_promotion_counts_every_imported_magnitude_and_component() {
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Mixed", 0.0)
            .with_waveforms(vec![
                RetainedWaveform::new("magnitude", vec![0.0, 1.0], vec![5.0, 5.0])
                    .with_complex_components("out", vec![3.0, 4.0], vec![4.0, 3.0]),
                RetainedWaveform::new("real", vec![0.0, 1.0], vec![-0.0, 2.0]),
            ]);
        for max_values in [10, 13, 14] {
            let limits = WaveformImportLimits {
                max_values,
                ..EXPORT_TEST_LIMITS
            };
            let export = prepare_numpy(
                &analysis,
                &analysis.waveforms.iter().collect::<Vec<_>>(),
                limits,
            )
            .unwrap();
            // NPZ retains each signal's own dtype and needs ten values.
            let archive = encode_npz(&export).unwrap();
            let decoded = crate::numpy::archive::decode_npz(
                &archive,
                crate::numpy::archive::NpzReadLimits {
                    max_members: 3,
                    max_expanded_bytes: 4096,
                    max_numeric_values: max_values,
                },
                &["time"],
                "numpy_npz",
            )
            .unwrap();
            assert_eq!(decoded.signals.len(), 2);
            assert_eq!(
                decoded.signals.iter().filter(|s| s.imag.is_some()).count(),
                1
            );
            // NPY's single complex dtype promotes the real signal as well.
            let matrix = encode_npy(&export);
            if max_values < 14 {
                assert!(
                    matrix
                        .map(|bytes| bytes.len())
                        .unwrap_err()
                        .to_string()
                        .contains("retains 14 numeric values")
                );
            } else {
                let array =
                    crate::numpy::reader::decode_npy(&matrix.unwrap(), max_values, "numpy_npy")
                        .unwrap();
                let decoded =
                    crate::numpy::reader::npy_matrix_to_dataset(array, 2, 3, "numpy_npy").unwrap();
                assert_eq!(decoded.coordinate, [0.0, 1.0]);
                assert_eq!(decoded.signals.len(), 2);
                assert!(decoded.signals.iter().all(|s| s.imag.is_some()));
                assert_eq!(decoded.signals[1].real[0].to_bits(), (-0.0_f64).to_bits());
            }
        }
    }
}
