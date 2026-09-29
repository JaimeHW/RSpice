//! NumPy array projections from already-selected retained waveforms.

use super::{MAX_COLUMNS, NamedArray, NumpyWriteError};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::waveform::RetainedWaveform;

#[derive(Debug)]
pub enum NumpyProjectionError {
    NoSamples,
    DifferentCoordinates {
        signal: String,
    },
    SampleCount {
        signal: String,
        real: usize,
        imag: Option<usize>,
        coordinate: usize,
    },
    ColumnLimit {
        columns: usize,
    },
}

impl std::fmt::Display for NumpyProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSamples => f.write_str("No waveform samples available to export."),
            Self::DifferentCoordinates { signal } => write!(
                f,
                "A NumPy export is one rectangular table, so every column must stand on the same \
                 coordinate samples. '{signal}' carries its own x-axis samples. Export this result as \
                 CSV or an RSpice bundle instead."
            ),
            Self::SampleCount {
                signal,
                real,
                coordinate,
                ..
            } => write!(
                f,
                "'{signal}' has {real} samples against {coordinate} coordinate samples; the export is refused rather \
                 than padded or truncated."
            ),
            Self::ColumnLimit { columns } => write!(
                f,
                "This result has {columns} columns; RSpice reads at most {MAX_COLUMNS} from a NumPy source, \
                 so publishing it would produce a file this build could not reopen. Hide traces, or \
                 export an RSpice bundle."
            ),
        }
    }
}

impl std::error::Error for NumpyProjectionError {}

#[derive(Debug)]
pub struct NumpySignal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
}

/// One rectangular table, with the complex signals still complex.
#[derive(Debug)]
pub struct NumpyExport {
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
) -> Result<NumpyExport, NumpyProjectionError> {
    let reference = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or(NumpyProjectionError::NoSamples)?;
    let coordinate = reference.x.as_ref().to_vec();

    let mut signals = Vec::with_capacity(waveforms.len());
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        // A NumPy array is rectangular, so a column that does not stand on the
        // shared coordinate has no honest place in it.
        if waveform.x.as_ref() != coordinate.as_slice() {
            return Err(NumpyProjectionError::DifferentCoordinates {
                signal: waveform.name.clone(),
            });
        }
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
    if signals.is_empty() {
        return Err(NumpyProjectionError::NoSamples);
    }
    for signal in &signals {
        if signal.real.len() != coordinate.len()
            || signal
                .imag
                .as_ref()
                .is_some_and(|imag| imag.len() != coordinate.len())
        {
            return Err(NumpyProjectionError::SampleCount {
                signal: signal.name.clone(),
                real: signal.real.len(),
                imag: signal.imag.as_ref().map(Vec::len),
                coordinate: coordinate.len(),
            });
        }
    }
    let export = NumpyExport {
        coordinate_name: coordinate_name(analysis.analysis_type),
        coordinate,
        signals,
    };
    if export.columns() > MAX_COLUMNS {
        return Err(NumpyProjectionError::ColumnLimit {
            columns: export.columns(),
        });
    }
    Ok(export)
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
    crate::numpy::matrix::encode_npy(&export.coordinate, &borrowed_arrays(export))
}

pub fn encode_npz(export: &NumpyExport) -> Result<Vec<u8>, NumpyWriteError> {
    crate::numpy::archive::encode_npz(
        export.coordinate_name,
        &export.coordinate,
        &borrowed_arrays(export),
    )
}
