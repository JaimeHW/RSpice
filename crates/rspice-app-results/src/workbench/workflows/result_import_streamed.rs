//! A binary rawfile imported as it is read: its rows go into a results store segment a block at
//! a time, checked as they pass, and no more of the file is held than one block. The review facts
//! are gathered on the way, so nothing reads the segment again before the review opens.
//!
//! Only a rawfile the streamed reader reads is imported this way: one ordinary binary plot. Every
//! other source is read whole, as before. The segment is the one the whole-file import keeps for
//! the same file, byte for byte.

use super::*;
use crate::state::run_store::{
    Axis, PushRows, RunMeta, Schema, Signal, SignalKind, StoreError, keep_segment,
};
use rspice_formats::spice_raw::stream::{RawRowBlock, RawStreamReader, RawStreamStart};
use rspice_results::result_import::waveforms::ImportedRowsCheck;
use rspice_results::store_reader::{Segment, SegmentColumn};
use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};
use std::cell::Cell;

/// The most of a file read before its header must have ended for its rows to be read as they
/// arrive.
const HEAD_BYTES: u64 = 1 << 20;

/// Bytes read between two feeds of the reader, two reports of progress and two looks at Cancel.
const FEED_BYTES: usize = 8 << 20;

/// The rawfile at `path` imported as it is read, when it is one plot of binary rows: `None` for
/// any other source, which the caller reads whole. Cancel, or a refusal, keeps nothing.
pub(super) fn stream_spice_raw(
    job: &JobContext,
    path: &Path,
    source_name: &str,
) -> Result<Option<StagedResultImport>, JobStop> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    let length = file
        .metadata()
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
        .len();
    let mut head = Vec::new();
    (&mut file)
        .take(HEAD_BYTES)
        .read_to_end(&mut head)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let format = ResultImportFormat::SpiceRaw;
    if head.is_empty() || identify_result_import_format(source_name, &head).ok() != Some(format) {
        return Ok(None);
    }
    let _source = ReadingSource::of(usize::try_from(length).unwrap_or(usize::MAX));
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_external_data_bytes = usize::MAX;
    limits.max_external_data_values = source_capacity();
    limits.max_result_values = source_capacity();
    let reader = match RawStreamReader::start(&head, length, limits)
        .map_err(|error| adapters::adapter_error(format, error))?
    {
        RawStreamStart::Streamed(reader) => reader,
        RawStreamStart::WholeFile(_) => return Ok(None),
    };
    let plot = reader.plot().clone();
    let analysis_type = imported_analysis_type(plot.domain);
    let names = plot
        .signals
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    let check = ImportedRowsCheck::new(
        format,
        analysis_type,
        plot.coordinate_name.clone(),
        &names,
        waveform_import_limits(),
    )?;
    // The coordinate is the segment's own axis when it ascends, as the whole-file import keeps
    // it; one that descends is a column of its own beside a row axis.
    let axis = match reader.first_coordinates(&head) {
        Some((first, second)) => second > first,
        None => plot.points == 1,
    };
    let mut stream = Stream {
        reader,
        check,
        facts: StreamFacts::new(plot.signals.len()),
        layout: Layout {
            axis,
            complex: plot.complex,
            signals: plot.signals.clone(),
        },
        rows: RowBuffer::default(),
        format,
    };
    let schema = stream.layout.schema();
    let meta = RunMeta {
        analysis: format!("Imported \u{00b7} {source_name}"),
        ..RunMeta::default()
    };
    job.step("Reading", length);
    let mut stopped = None;
    let kept = keep_segment(&schema, &meta, &mut |push: &mut PushRows<'_>| {
        stream
            .run(job, &mut file, path, &head, push)
            .map_err(|stop| {
                stopped = Some(stop);
                StoreError::Io {
                    kind: std::io::ErrorKind::Interrupted,
                    message: "the import stopped".to_owned(),
                }
            })
    });
    let segment = match kept {
        Ok(segment) => segment,
        Err(reason) => return Err(stopped.unwrap_or(JobStop::Failed(reason))),
    };
    let Stream {
        check,
        facts,
        layout,
        ..
    } = stream;
    let rows = check.finish()?;
    let parsed = ParsedResultDataset {
        source_format: format,
        analysis_type,
        coordinate_name: plot.coordinate_name,
        coordinate_unit: plot.coordinate_unit,
        sample_count: rows,
        waveforms: layout.traces(&segment, rows)?,
        family_metadata: None,
        delimiter: 0,
        notes: Vec::new(),
        event_payload: None,
        event_dump: None,
        plots: None,
    };
    Ok(Some(StagedResultImport {
        source_name: source_name.to_owned(),
        parsed,
        facts: facts.into_facts(rows),
    }))
}

/// A rawfile on its way into a segment.
struct Stream {
    reader: Box<RawStreamReader>,
    check: ImportedRowsCheck,
    facts: StreamFacts,
    layout: Layout,
    rows: RowBuffer,
    format: ResultImportFormat,
}

impl Stream {
    /// Every row of the plot, checked and pushed into the segment: the rows in `head`, the
    /// file's first bytes, then the rest of `file` from where `head` ended.
    fn run(
        &mut self,
        job: &JobContext,
        file: &mut std::fs::File,
        path: &Path,
        head: &[u8],
        push: &mut PushRows<'_>,
    ) -> Result<(), JobStop> {
        let Self {
            reader,
            check,
            facts,
            layout,
            rows,
            format,
        } = self;
        let (layout, format) = (&*layout, *format);
        // A refusal of the rows is already in its own words; the reader's is the rawfile's.
        let refused = Cell::new(false);
        let explain = |error: String| {
            if refused.get() {
                error
            } else {
                adapters::adapter_error(format, error)
            }
        };
        let mut sink = |block: RawRowBlock<'_>| {
            let kept = check
                .rows(block.coordinate, block.real, block.imag)
                .and_then(|()| {
                    facts.see(&block);
                    rows.fill(layout, &block);
                    push(&rows.x, &rows.values).map_err(|error| {
                        format!("the results store could not keep the samples: {error}")
                    })
                });
            refused.set(kept.is_err());
            kept
        };
        let data = head.get(reader.data_offset()..).unwrap_or_default();
        reader.feed(data, &mut sink).map_err(explain)?;
        let mut read = head.len() as u64;
        job.done(read);
        let mut buffer = vec![0; FEED_BYTES];
        loop {
            job.check()?;
            let count = file
                .read(&mut buffer)
                .map_err(|error| format!("could not read {}: {error}", path.display()))?;
            if count == 0 {
                break;
            }
            reader.feed(&buffer[..count], &mut sink).map_err(explain)?;
            read += count as u64;
            job.done(read);
        }
        reader.finish(&mut sink).map_err(explain)?;
        Ok(())
    }
}

/// How the plot's columns sit in the segment: the layout the whole-file import keeps.
struct Layout {
    /// The coordinate is the segment's axis; otherwise it is the first column, beside row numbers.
    axis: bool,
    complex: bool,
    /// Each signal's name and unit, in file order.
    signals: Vec<(String, Option<String>)>,
}

impl Layout {
    /// The name a signal is shown by: a complex signal's magnitude is its trace.
    fn trace_name(&self, name: &str) -> String {
        if self.complex {
            format!("|{name}|")
        } else {
            name.to_owned()
        }
    }

    fn schema(&self) -> Schema {
        let mut signals = Vec::new();
        if !self.axis {
            signals.push(Signal::new("x", "", SignalKind::Real));
        }
        for (name, unit) in &self.signals {
            let unit = unit.clone().unwrap_or_default();
            let trace = self.trace_name(name);
            if self.complex {
                let (real, imag) = (format!("re({trace})"), format!("im({trace})"));
                signals.push(Signal::new(trace, unit.clone(), SignalKind::Real));
                signals.push(Signal::new(real, unit.clone(), SignalKind::Real));
                signals.push(Signal::new(imag, unit, SignalKind::Real));
            } else {
                signals.push(Signal::new(trace, unit, SignalKind::Real));
            }
        }
        Schema {
            x: Axis {
                name: if self.axis { "x" } else { "row" }.to_owned(),
                unit: String::new(),
            },
            signals,
        }
    }

    /// The traces that read the segment's `rows` rows.
    fn traces(&self, segment: &Arc<Segment>, rows: usize) -> Result<Vec<RetainedTrace>, String> {
        let window = StoredWindow {
            first: 0,
            rows: rows as u64,
            lead: WindowLead::None,
        };
        let stored = |slot: Option<usize>, name: String| {
            WaveformValues::stored(SegmentColumn::new(Arc::clone(segment), slot, name), window)
        };
        let x = stored((!self.axis).then_some(0), "x".to_owned())?;
        let mut slot = usize::from(!self.axis);
        let mut traces = Vec::with_capacity(self.signals.len());
        for (name, unit) in &self.signals {
            let trace_name = self.trace_name(name);
            let mut trace =
                if self.complex {
                    let y = stored(Some(slot), trace_name.clone())?;
                    let real = stored(Some(slot + 1), format!("re({trace_name})"))?;
                    let imag = stored(Some(slot + 2), format!("im({trace_name})"))?;
                    slot += 3;
                    RetainedTrace::new(trace_name, x.clone(), y, String::new())
                        .with_complex_components(name.clone(), real, imag)
                } else {
                    let y = stored(Some(slot), trace_name.clone())?;
                    slot += 1;
                    RetainedTrace::new(trace_name, x.clone(), y, String::new())
                };
            if let Some(unit) = unit {
                trace = trace.with_unit(unit.clone());
            }
            traces.push(trace);
        }
        Ok(traces)
    }
}

/// One block's rows as the segment takes them: x, and each row's value slots in turn.
#[derive(Default)]
struct RowBuffer {
    x: Vec<f64>,
    values: Vec<f64>,
    /// The rows pushed before this block: the next row's number on a row axis.
    first: usize,
}

impl RowBuffer {
    fn fill(&mut self, layout: &Layout, block: &RawRowBlock<'_>) {
        self.x.clear();
        self.values.clear();
        for (row, &coordinate) in block.coordinate.iter().enumerate() {
            if layout.axis {
                self.x.push(coordinate);
            } else {
                self.x.push((self.first + row) as f64);
                self.values.push(coordinate);
            }
            for (signal, real) in block.real.iter().enumerate() {
                let real = real[row];
                match block.imag.get(signal) {
                    Some(imag) => {
                        let imag = imag[row];
                        self.values.extend([real.hypot(imag), real, imag]);
                    }
                    None => self.values.push(real),
                }
            }
        }
        self.first += block.coordinate.len();
    }
}

/// What the review pages show of the samples, gathered as the rows pass.
struct StreamFacts {
    /// Every coordinate so far is finite, positive and above the one before, as a frequency
    /// grid's must be.
    grid: bool,
    previous: Option<f64>,
    seen: usize,
    first_rows: Vec<Vec<(f64, Option<f64>)>>,
}

impl StreamFacts {
    fn new(signals: usize) -> Self {
        Self {
            grid: true,
            previous: None,
            seen: 0,
            first_rows: vec![Vec::new(); signals],
        }
    }

    fn see(&mut self, block: &RawRowBlock<'_>) {
        for &coordinate in block.coordinate {
            self.grid &= coordinate.is_finite()
                && coordinate > 0.0
                && self.previous.is_none_or(|previous| previous < coordinate);
            self.previous = Some(coordinate);
        }
        let wanted = RESULT_IMPORT_PREVIEW_ROWS
            .saturating_sub(self.seen)
            .min(block.coordinate.len());
        for row in 0..wanted {
            for (signal, first) in self.first_rows.iter_mut().enumerate() {
                let real = block.real[signal][row];
                let shown = match block.imag.get(signal) {
                    Some(imag) => real.hypot(imag[row]),
                    None => real,
                };
                first.push((
                    block.coordinate[row],
                    Some(shown).filter(|value| value.is_finite()),
                ));
            }
        }
        self.seen += block.coordinate.len();
    }

    fn into_facts(self, rows: usize) -> ResultImportSampleFacts {
        ResultImportSampleFacts {
            frequency_grid: vec![rows > 0 && self.grid; self.first_rows.len()],
            first_rows: self.first_rows,
        }
    }
}

#[cfg(test)]
#[path = "result_import_streamed/tests.rs"]
mod tests;
