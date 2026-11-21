//! A binary rawfile imported as it is read: its rows are checked as they pass and laid out as the
//! results store segment that keeps them takes them, and the review's facts are gathered on the
//! way, so nothing reads the segment again before the review opens.
//!
//! This is the import's engine, with no file, store or window of its own: the desktop's import job
//! feeds it a file and pushes its rows into a segment of its store folder, and the browser's import
//! worker feeds it a picked file and hands its rows to the store worker. The segment either keeps
//! is the one the whole-file import keeps for the same file, byte for byte.

use rspice_core::ResourceLimits;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::result_import::ResultImportFormat;
use rspice_results::result_import::waveforms::{ImportedRowsCheck, WaveformImportLimits};
use rspice_results::stored_analysis::{Axis, Schema, Signal, SignalKind};
use rspice_results::waveform::{RetainedTrace, WaveformValues};

use super::stream::{RawRowBlock, RawStreamReader, RawStreamStart, StreamedRawPlot};

/// Per signal, its first rows as (coordinate, shown value), the value `None` where not finite.
pub type FirstRows = Vec<Vec<(f64, Option<f64>)>>;

/// The analysis an imported plot of `domain` is.
pub fn imported_analysis_type(domain: crate::WaveformDomain) -> AnalysisType {
    match domain {
        crate::WaveformDomain::Transient => AnalysisType::Transient,
        crate::WaveformDomain::Ac => AnalysisType::Ac,
        crate::WaveformDomain::DcSweep => AnalysisType::DcSweep,
    }
}

/// How a rawfile is imported.
#[derive(Debug)]
pub enum RawImportStart {
    /// As it is read: feed [`RawImport::feed`] the bytes from [`RawImport::data_offset`] on.
    Streamed(Box<RawImport>),
    /// Whole, by the whole-file reader: it is not one ordinary binary plot.
    WholeFile,
}

/// A binary rawfile on its way into a segment.
#[derive(Debug)]
pub struct RawImport {
    reader: Box<RawStreamReader>,
    check: ImportedRowsCheck,
    facts: StreamFacts,
    layout: Layout,
    rows: RowBuffer,
    format: ResultImportFormat,
}

impl RawImport {
    /// How the rawfile whose first bytes are `head`, `length` bytes in all, is imported: read
    /// within `resource`, its rows checked against `limits`, the first `preview_rows` of each
    /// signal kept for the review.
    pub fn start(
        head: &[u8],
        length: u64,
        resource: ResourceLimits,
        limits: WaveformImportLimits,
        preview_rows: usize,
    ) -> Result<RawImportStart, String> {
        let format = ResultImportFormat::SpiceRaw;
        let reader = match RawStreamReader::start(head, length, resource)
            .map_err(|error| adapter_error(format, error))?
        {
            RawStreamStart::Streamed(reader) => reader,
            RawStreamStart::WholeFile(_) => return Ok(RawImportStart::WholeFile),
        };
        let plot = reader.plot().clone();
        let names = plot
            .signals
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>();
        let check = ImportedRowsCheck::new(
            format,
            imported_analysis_type(plot.domain),
            plot.coordinate_name.clone(),
            &names,
            limits,
        )?;
        // The coordinate is the segment's own axis when it ascends, as the whole-file import keeps
        // it; one that descends is a column of its own beside a row axis.
        let axis = match reader.first_coordinates(head) {
            Some((first, second)) => second > first,
            None => plot.points == 1,
        };
        Ok(RawImportStart::Streamed(Box::new(Self {
            reader,
            check,
            facts: StreamFacts::new(plot.signals.len(), preview_rows),
            layout: Layout {
                axis,
                complex: plot.complex,
                signals: plot.signals.clone(),
            },
            rows: RowBuffer::default(),
            format,
        })))
    }

    /// What the plot declares.
    pub fn plot(&self) -> &StreamedRawPlot {
        self.reader.plot()
    }

    /// Where the plot's rows start in the file.
    pub fn data_offset(&self) -> usize {
        self.reader.data_offset()
    }

    /// The shape of the segment that keeps the plot.
    pub fn schema(&self) -> Schema {
        self.layout.schema()
    }

    /// Read `bytes`, the next of the plot's rows: each row is checked and laid out among the rows
    /// waiting to be pushed ([`Self::pending`]).
    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), String> {
        let Self {
            reader,
            check,
            facts,
            layout,
            rows,
            format,
        } = self;
        let mut refused = false;
        let mut sink = |block: RawRowBlock<'_>| {
            let kept = take_block(check, facts, layout, rows, &block);
            refused = kept.is_err();
            kept
        };
        let fed = reader.feed(bytes, &mut sink);
        explain(fed, refused, *format)
    }

    /// End the plot once every byte has been fed: its last rows join those waiting. A plot that
    /// ended before its declared rows is refused.
    pub fn finish(&mut self) -> Result<(), String> {
        let Self {
            reader,
            check,
            facts,
            layout,
            rows,
            format,
        } = self;
        let mut refused = false;
        let mut sink = |block: RawRowBlock<'_>| {
            let kept = take_block(check, facts, layout, rows, &block);
            refused = kept.is_err();
            kept
        };
        let finished = reader.finish(&mut sink).map(|_| ());
        explain(finished, refused, *format)
    }

    /// The rows read and not yet pushed: their x, then each row's value slots in turn.
    pub fn pending(&self) -> (&[f64], &[f64]) {
        (&self.rows.x, &self.rows.values)
    }

    /// Forget the rows [`Self::pending`] gave, once they are pushed.
    pub fn clear_pending(&mut self) {
        self.rows.x.clear();
        self.rows.values.clear();
    }

    /// The plot read to its end: its rows counted by the check, and what the review shows.
    pub fn imported(self) -> Result<RawImported, String> {
        let plot = self.reader.plot().clone();
        let rows = self.check.finish()?;
        let (frequency_grid, first_rows) = self.facts.into_facts(rows);
        Ok(RawImported {
            plot,
            rows,
            frequency_grid,
            first_rows,
            layout: self.layout,
        })
    }
}

/// A rawfile's plot read whole into a segment.
#[derive(Debug)]
pub struct RawImported {
    pub plot: StreamedRawPlot,
    pub rows: usize,
    /// Per signal: whether its coordinate is non-empty, finite, positive and strictly ascending.
    pub frequency_grid: Vec<bool>,
    pub first_rows: FirstRows,
    layout: Layout,
}

impl RawImported {
    /// The traces that read the segment: `column(slot, name)` gives the window of all its rows of
    /// the column `slot` (`None` for the segment's own axis).
    pub fn traces(
        &self,
        mut column: impl FnMut(Option<usize>, String) -> Result<WaveformValues, String>,
    ) -> Result<Vec<RetainedTrace>, String> {
        let layout = &self.layout;
        let x = column((!layout.axis).then_some(0), "x".to_owned())?;
        let mut slot = usize::from(!layout.axis);
        let mut traces = Vec::with_capacity(layout.signals.len());
        for (name, unit) in &layout.signals {
            let trace_name = layout.trace_name(name);
            let mut trace =
                if layout.complex {
                    let y = column(Some(slot), trace_name.clone())?;
                    let real = column(Some(slot + 1), format!("re({trace_name})"))?;
                    let imag = column(Some(slot + 2), format!("im({trace_name})"))?;
                    slot += 3;
                    RetainedTrace::new(trace_name, x.clone(), y, String::new())
                        .with_complex_components(name.clone(), real, imag)
                } else {
                    let y = column(Some(slot), trace_name.clone())?;
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

/// Check `block`, note what the review shows of it, and lay it out among the waiting rows.
fn take_block(
    check: &mut ImportedRowsCheck,
    facts: &mut StreamFacts,
    layout: &Layout,
    rows: &mut RowBuffer,
    block: &RawRowBlock<'_>,
) -> Result<(), String> {
    check.rows(block.coordinate, block.real, block.imag)?;
    facts.see(block);
    rows.fill(layout, block);
    Ok(())
}

/// A rawfile reader's failure in the import's words.
fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.canonical_id())
}

/// A refusal of the rows is already in its own words; the reader's is the rawfile's.
fn explain(
    result: Result<(), String>,
    refused: bool,
    format: ResultImportFormat,
) -> Result<(), String> {
    result.map_err(|error| {
        if refused {
            error
        } else {
            adapter_error(format, error)
        }
    })
}

/// How the plot's columns sit in the segment: the layout the whole-file import keeps.
#[derive(Debug)]
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
}

/// Rows as the segment takes them: x, and each row's value slots in turn.
#[derive(Debug, Default)]
struct RowBuffer {
    x: Vec<f64>,
    values: Vec<f64>,
    /// The rows laid out so far: the next row's number on a row axis.
    first: usize,
}

impl RowBuffer {
    fn fill(&mut self, layout: &Layout, block: &RawRowBlock<'_>) {
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
#[derive(Debug)]
struct StreamFacts {
    /// Every coordinate so far is finite, positive and above the one before, as a frequency
    /// grid's must be.
    grid: bool,
    previous: Option<f64>,
    seen: usize,
    preview_rows: usize,
    first_rows: FirstRows,
}

impl StreamFacts {
    fn new(signals: usize, preview_rows: usize) -> Self {
        Self {
            grid: true,
            previous: None,
            seen: 0,
            preview_rows,
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
        let wanted = self
            .preview_rows
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

    fn into_facts(self, rows: usize) -> (Vec<bool>, FirstRows) {
        (
            vec![rows > 0 && self.grid; self.first_rows.len()],
            self.first_rows,
        )
    }
}
