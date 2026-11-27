//! A CSV or TSV table of retained waveforms written as its samples are read.
//!
//! The table is the one [`project_waveforms`] and [`WaveformWriter`] make, byte for byte: the
//! projection is taken a chunk of rows at a time, each chunk's samples read a block at a time,
//! and the writer's header, rows and closing record are written as the chunks come. A stored
//! column of any length costs a chunk of memory and is never read whole.

use std::io::Write;
use std::ops::Range;

use super::export::{WaveformProjectionError, project_waveforms, validate_shared_x_axis};
use crate::waveform_io::{WaveformDataset, WaveformWriteError, WaveformWriter};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::waveform::{BLOCK_SAMPLES, RetainedWaveform, SharingKey, WaveformValues};

/// The rows projected and written at a time: one block of each column.
const CHUNK_ROWS: usize = BLOCK_SAMPLES;

/// What a written table holds, as its export reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelimitedTable {
    /// Its signal columns, the coordinate aside.
    pub signals: usize,
    /// Its rows.
    pub points: usize,
}

/// Why a table was not written, or stopped part way.
#[derive(Debug)]
pub enum DelimitedExportError {
    /// The traces do not make a table, as the projection refuses them.
    Projection(WaveformProjectionError),
    /// The table is not one the writer writes, as it refuses it.
    Table(WaveformWriteError),
    /// The destination refused a write.
    Write(std::io::Error),
}

impl std::fmt::Display for DelimitedExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Projection(error) => error.fmt(f),
            Self::Table(error) => error.fmt(f),
            Self::Write(error) => write!(f, "the table could not be written: {error}"),
        }
    }
}

impl std::error::Error for DelimitedExportError {}

/// Whether the selected `waveforms` of `analysis` make a table the writer writes, refused as
/// [`write_delimited_waveforms`] would refuse it before its first row: checked by their shapes
/// and their first row, so a stored column is not read.
pub fn check_delimited_waveforms<A: AsRef<RetainedWaveform>, W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<A>,
    waveforms: &[&W],
) -> Result<(), DelimitedExportError> {
    let rows = check_shape(waveforms)?;
    let dataset = chunk_dataset(analysis, waveforms, 0..rows.min(1))?;
    WaveformWriter::delimited_frame(&dataset, ',')
        .map(drop)
        .map_err(DelimitedExportError::Table)
}

/// The selected `waveforms` of `analysis` as the CSV (`delimiter` `,`) or TSV (`\t`) table
/// [`project_waveforms`] and [`WaveformWriter`] make of them, byte for byte, written to `out` a
/// chunk of rows at a time. A table they refuse is refused as they refuse it, before its first
/// row; an infinite sample found part way stops the table there, with the writer's reason.
pub fn write_delimited_waveforms<A: AsRef<RetainedWaveform>, W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<A>,
    waveforms: &[&W],
    delimiter: char,
    out: &mut dyn Write,
) -> Result<DelimitedTable, DelimitedExportError> {
    let rows = check_shape(waveforms)?;
    let mut closing = None;
    let mut signals = 0;
    let mut text = String::new();
    let mut first = 0_usize;
    loop {
        let end = rows.min(first.saturating_add(CHUNK_ROWS));
        let dataset = chunk_dataset(analysis, waveforms, first..end)?;
        text.clear();
        if closing.is_none() {
            let (head, record) = WaveformWriter::delimited_frame(&dataset, delimiter)
                .map_err(DelimitedExportError::Table)?;
            text.push_str(&head);
            closing = Some(record);
            signals = dataset.signal_count();
        } else {
            WaveformWriter::check_delimited(&dataset).map_err(DelimitedExportError::Table)?;
        }
        WaveformWriter::delimited_rows(&dataset, delimiter, &mut text);
        out.write_all(text.as_bytes())
            .map_err(DelimitedExportError::Write)?;
        first = end;
        if first >= rows {
            break;
        }
    }
    out.write_all(closing.unwrap_or_default().as_bytes())
        .map_err(DelimitedExportError::Write)?;
    Ok(DelimitedTable {
        signals,
        points: rows,
    })
}

/// The table's rows, once the traces are checked as the projection checks the whole of them:
/// one coordinate shared by every trace, and every component as long as it. A chunk of them
/// then projects as the whole would, so a table of the wrong shape is refused, in the
/// projection's words, before a row is written.
fn check_shape<W: AsRef<RetainedWaveform>>(
    waveforms: &[&W],
) -> Result<usize, DelimitedExportError> {
    let Some(reference) = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
    else {
        return Ok(0);
    };
    validate_shared_x_axis(waveforms, &reference.x).map_err(DelimitedExportError::Projection)?;
    for waveform in waveforms.iter().map(|waveform| (*waveform).as_ref()) {
        let rows = waveform.x.len();
        let parts = match &waveform.complex {
            Some(complex) => vec![
                (format!("Re({})", complex.source_name), &complex.real),
                (format!("Im({})", complex.source_name), &complex.imag),
            ],
            None => vec![(waveform.name.clone(), &waveform.y)],
        };
        for (signal, values) in parts {
            if values.len() != rows {
                return Err(DelimitedExportError::Projection(
                    WaveformProjectionError::SampleCount {
                        signal,
                        x: rows,
                        y: values.len(),
                    },
                ));
            }
        }
    }
    Ok(reference.x.len())
}

/// The projection of rows `rows` of the traces: each trace's columns cut to those rows, a
/// column the traces share cut once and shared, as the whole is.
fn chunk_dataset<A: AsRef<RetainedWaveform>, W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<A>,
    waveforms: &[&W],
    rows: Range<usize>,
) -> Result<WaveformDataset, DelimitedExportError> {
    let mut cut: Vec<(SharingKey, WaveformValues)> = Vec::new();
    let mut part = |signal: &str, values: &WaveformValues| {
        let key = values.sharing_key();
        if let Some((_, part)) = cut.iter().find(|(shared, _)| *shared == key) {
            return Ok(part.clone());
        }
        let end = rows.end.min(values.len());
        let mut samples = Vec::with_capacity(end.saturating_sub(rows.start));
        for block in values.blocks(rows.start.min(end)..end) {
            let block = block.map_err(|failure| {
                DelimitedExportError::Projection(WaveformProjectionError::Unreadable {
                    signal: signal.to_owned(),
                    reason: failure.to_string(),
                })
            })?;
            samples.extend_from_slice(&block);
        }
        let part = WaveformValues::from(samples);
        cut.push((key, part.clone()));
        Ok(part)
    };
    let chunk = waveforms
        .iter()
        .map(|waveform| {
            let waveform = (*waveform).as_ref();
            let mut chunk = waveform.clone();
            chunk.x = part(&waveform.name, &waveform.x)?;
            chunk.y = part(&waveform.name, &waveform.y)?;
            if let (Some(cut), Some(whole)) = (chunk.complex.as_mut(), waveform.complex.as_ref()) {
                cut.real = part(&whole.source_name, &whole.real)?;
                cut.imag = part(&whole.source_name, &whole.imag)?;
            }
            Ok(chunk)
        })
        .collect::<Result<Vec<_>, DelimitedExportError>>()?;
    let chunk = chunk.iter().collect::<Vec<_>>();
    project_waveforms(analysis, &chunk, false).map_err(DelimitedExportError::Projection)
}
