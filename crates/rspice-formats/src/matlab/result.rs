//! MATLAB v5 projections from already-selected retained waveforms.

use super::MatVariable;
use super::publication::{MatNameAllocator, created_on, header_text, note_entry};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::waveform::RetainedWaveform;

use crate::waveform_io::result::{WaveformExportError, shared_coordinate};
use rspice_results::result_import::waveforms::WaveformImportLimits;

#[derive(Debug)]
pub enum MatlabProjectionError {
    UnsupportedAnalysis {
        analysis: AnalysisType,
        label: String,
    },
    Waveforms(WaveformExportError),
}

impl std::fmt::Display for MatlabProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedAnalysis { label, .. } => write!(
                f,
                "A MATLAB v5 file carries one sampled analysis: a transient on 'time', a DC sweep on \
                 'sweep' or an AC sweep on 'frequency'. '{label}' is none of those, and MAT has no header \
                 in which a variable could say what it is, so the file would reopen as an anonymous \
                 sweep. Export CSV, or an RSpice bundle, which carries this analysis whole."
            ),
            Self::Waveforms(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for MatlabProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Waveforms(source) => Some(source),
            _ => None,
        }
    }
}

/// One prepared file, and what the reader is owed about it.
#[derive(Debug)]
pub struct MatlabExport {
    pub header_text: String,
    pub variables: Vec<MatVariable>,
    /// The variable the coordinate was published under.
    pub coordinate: &'static str,
    pub rows: usize,
    /// Every published variable, mapped back to what it came from.
    pub note: String,
    /// The note did not fit the header's descriptive-text field.
    pub note_truncated: bool,
}

/// The variable name an analysis publishes its coordinate under.
///
/// These are exactly the names `result_import_adapters::parse_matlab_v5`
/// recognises a coordinate by, so a file written here reopens as the analysis
/// it was.
const fn coordinate_variable(analysis: AnalysisType) -> Option<&'static str> {
    match analysis {
        AnalysisType::Transient => Some("time"),
        AnalysisType::DcSweep => Some("sweep"),
        AnalysisType::Ac => Some("frequency"),
        _ => None,
    }
}

pub fn prepare_matlab<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    waveforms: &[&W],
    limits: WaveformImportLimits,
) -> Result<MatlabExport, MatlabProjectionError> {
    let Some(coordinate_name) = coordinate_variable(analysis.analysis_type) else {
        return Err(MatlabProjectionError::UnsupportedAnalysis {
            analysis: analysis.analysis_type,
            label: analysis.label.clone(),
        });
    };
    let coordinate =
        shared_coordinate(waveforms, limits).map_err(MatlabProjectionError::Waveforms)?;

    // The coordinate claims its name first: it is written first, and the
    // importer takes the first variable whose name it recognises.
    let mut names = MatNameAllocator::new(coordinate_name, waveforms.len());
    let rows = coordinate.len();
    let mut variables = vec![MatVariable {
        name: coordinate_name.to_owned(),
        real: coordinate.to_vec(),
        imag: None,
    }];
    let coordinate_source =
        crate::waveform_io::result::axis_signal_for_analysis_type(analysis.analysis_type).0;
    let mut entries = vec![note_entry(coordinate_name, coordinate_source, None)];
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        let (source, real, imag) = match &waveform.complex {
            Some(complex) => (
                complex.source_name.clone(),
                complex.real.as_ref().to_vec(),
                Some(complex.imag.as_ref().to_vec()),
            ),
            None => (waveform.name.clone(), waveform.y.as_ref().to_vec(), None),
        };
        let name = names.allocate(&source);
        entries.push(note_entry(&name, &source, waveform.unit.as_deref()));
        variables.push(MatVariable { name, real, imag });
    }

    // Every published variable is named, not just the first few: the unit is
    // recoverable from nowhere else, so a note that covered some of them
    // would be a note that quietly lost the rest.
    let note = entries.join("; ");
    let (header_text, note_truncated) = header_text(&created_on(analysis.timestamp), &note);
    Ok(MatlabExport {
        header_text,
        variables,
        coordinate: coordinate_name,
        rows,
        note,
        note_truncated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matlab::reader::{MatlabReadLimits, decode_matlab_v5};
    use crate::waveform_io::result::EXPORT_TEST_LIMITS;

    #[test]
    fn matlab_preserves_each_signal_representation_in_every_waveform_domain() {
        for kind in [
            AnalysisType::Transient,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
        ] {
            let analysis = AnalysisResult::new(1, kind, "Mixed representations", 0.0)
                .with_waveforms(vec![
                    RetainedWaveform::new("magnitude", vec![1.0, 2.0], vec![99.0, 99.0])
                        .with_complex_components("out", vec![-0.0, 4.0], vec![2.0, -1.0]),
                    RetainedWaveform::new("scalar", vec![1.0, 2.0], vec![3.0, -0.0]),
                ]);
            let export = prepare_matlab(
                &analysis,
                &analysis.waveforms.iter().collect::<Vec<_>>(),
                EXPORT_TEST_LIMITS,
            )
            .unwrap();
            let bytes =
                crate::matlab::write_mat_v5(&export.header_text, &export.variables).unwrap();
            let decoded = decode_matlab_v5(
                &bytes,
                MatlabReadLimits {
                    max_variables: 10,
                    min_rows: 1,
                    coordinate_names: &["time", "frequency", "sweep"],
                },
                "matlab-v5",
            )
            .unwrap();
            assert_eq!(decoded.signals.len(), 2);
            assert_eq!(decoded.signals[0].name, "out");
            assert_eq!(decoded.signals[0].real[0].to_bits(), (-0.0_f64).to_bits());
            assert_eq!(decoded.signals[0].real[1], 4.0);
            assert_eq!(
                decoded.signals[0].imag.as_deref(),
                Some([2.0, -1.0].as_slice())
            );
            assert_eq!(decoded.signals[1].name, "scalar");
            assert_eq!(decoded.signals[1].real[1].to_bits(), (-0.0_f64).to_bits());
            assert!(decoded.signals[1].imag.is_none());
        }
    }
}
