//! The store adapter: a finished analysis keeps its samples in a results store segment, and its
//! waveforms read windows of it.
//!
//! Every analysis reaches the history with its waveforms in memory, except a desktop transient
//! on its default outputs, which writes its own segment as it solves. Before an analysis is kept,
//! each waveform holding a series in memory is written to a segment of this process's store, one
//! segment per distinct x axis, and its series become windows of the segment's columns. A saved
//! project or session then names the segment by its digest and holds none of the samples.
//!
//! An x axis that ascends strictly is the segment's own axis, so a view tiles the waveforms on it
//! from the segment's summaries. Any other axis (a DC sweep that runs down, a nested sweep that
//! repeats) is a column of its own beside an axis of row numbers, and is read as values.
//!
//! The browser build has no store folder: its segments live in memory, read the same way, and
//! do not outlive the page.

use std::borrow::Cow;
use std::sync::Arc;

use rspice_results::analysis_result::AnalysisResult as RetainedAnalysisResult;
use rspice_results::store_reader::{Segment, SegmentColumn};
#[cfg(any(test, target_arch = "wasm32"))]
use rspice_results::store_reader::{SegmentBytes, StoreReader};
use rspice_results::waveform::{RetainedWaveform, StoredWindow, WaveformValues, WindowLead};
use rspice_results_store::{Axis, RunMeta, Schema, Signal, SignalKind, StoreError};
#[cfg(any(test, target_arch = "wasm32"))]
use rspice_results_store::{MemoryIo, SegmentReader, SegmentWriter};

/// Rows written per call to the segment writer.
const ROWS_PER_PUSH: usize = 4096;

/// Which series of a waveform a column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Y,
    Real,
    Imag,
}

/// The waveforms that share one x axis, and the series of theirs one segment keeps.
struct Group {
    x: WaveformValues,
    /// (waveform index, series) per value column, in column order.
    columns: Vec<(usize, Part)>,
}

/// Pushes rows (their x, then their value columns row-major) into a segment's writer.
type PushRows<'a> = dyn FnMut(&[f64], &[f64]) -> Result<(), StoreError> + 'a;

/// Fills a segment through the [`PushRows`] it is handed.
type FillSegment<'a> = dyn FnMut(&mut PushRows<'_>) -> Result<(), StoreError> + 'a;

/// Makes a finished segment of a schema from the rows its filler pushes.
type MakeSegment = fn(&Schema, &RunMeta, &mut FillSegment<'_>) -> Result<Arc<Segment>, String>;

/// Whether any series of `waveform` is held in memory.
fn holds_memory(waveform: &RetainedWaveform) -> bool {
    let complex = waveform
        .complex
        .iter()
        .flat_map(|complex| [&complex.real, &complex.imag]);
    [&waveform.x, &waveform.y]
        .into_iter()
        .chain(complex)
        .any(|values| !values.is_stored())
}

/// Whether any waveform of `analysis` holds a series in memory.
pub fn holds_samples_in_memory<W: AsRef<RetainedWaveform>>(
    analysis: &RetainedAnalysisResult<W>,
) -> bool {
    analysis
        .waveforms
        .iter()
        .any(|waveform| holds_memory(waveform.as_ref()))
}

fn series(waveform: &RetainedWaveform, part: Part) -> Option<&WaveformValues> {
    match part {
        Part::Y => Some(&waveform.y),
        Part::Real => waveform.complex.as_ref().map(|complex| &complex.real),
        Part::Imag => waveform.complex.as_ref().map(|complex| &complex.imag),
    }
}

fn series_mut(waveform: &mut RetainedWaveform, part: Part) -> Option<&mut WaveformValues> {
    match part {
        Part::Y => Some(&mut waveform.y),
        Part::Real => waveform.complex.as_mut().map(|complex| &mut complex.real),
        Part::Imag => waveform.complex.as_mut().map(|complex| &mut complex.imag),
    }
}

/// Whether `left` and `right` are one x axis: the same samples, compared without reading a stored
/// window.
fn same_axis(left: &WaveformValues, right: &WaveformValues) -> bool {
    if WaveformValues::ptr_eq(left, right) {
        return true;
    }
    if left.is_stored() || right.is_stored() || left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right.iter())
        .all(|(left, right)| left.to_bits() == right.to_bits())
}

/// The samples of `values`, read once.
fn samples(values: &WaveformValues) -> Result<Cow<'_, [f64]>, String> {
    values
        .read_range(0..values.len())
        .map_err(|failure| failure.to_string())
}

/// Write every series `analysis` holds in memory into results store segments, and make its
/// waveforms read them. An analysis whose waveforms are all stored is left as it is. On failure
/// nothing changes: the waveforms keep their samples in memory.
pub fn store_analysis<W>(analysis: &mut RetainedAnalysisResult<W>) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    store_analysis_with(analysis, process_segment)
}

/// [`store_analysis`] into segments held in memory, as the browser build keeps them.
#[cfg(test)]
pub fn store_analysis_in_memory<W>(analysis: &mut RetainedAnalysisResult<W>) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    store_analysis_with(analysis, memory_segment)
}

fn store_analysis_with<W>(
    analysis: &mut RetainedAnalysisResult<W>,
    make: MakeSegment,
) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    let mut groups: Vec<Group> = Vec::new();
    for (index, waveform) in analysis.waveforms.iter().enumerate() {
        let waveform = waveform.as_ref();
        if !holds_memory(waveform) {
            continue;
        }
        let group = match groups
            .iter()
            .position(|group| same_axis(&group.x, &waveform.x))
        {
            Some(group) => group,
            None => {
                groups.push(Group {
                    x: waveform.x.clone(),
                    columns: Vec::new(),
                });
                groups.len() - 1
            }
        };
        let parts: &[Part] = if waveform.complex.is_some() {
            &[Part::Y, Part::Real, Part::Imag]
        } else {
            &[Part::Y]
        };
        groups[group]
            .columns
            .extend(parts.iter().map(|&part| (index, part)));
    }
    let mut kept = Vec::with_capacity(groups.len());
    for group in &groups {
        kept.push(keep_group(
            &analysis.label,
            &analysis.waveforms,
            group,
            make,
        )?);
    }
    for (group, (x, columns)) in groups.iter().zip(kept) {
        for (&(index, part), values) in group.columns.iter().zip(columns) {
            let waveform = analysis.waveforms[index].as_mut();
            waveform.x = x.clone();
            if let Some(series) = series_mut(waveform, part) {
                *series = values;
            }
        }
    }
    Ok(())
}

/// One group's segment: its x axis and each of its columns, as stored windows.
fn keep_group<W: AsRef<RetainedWaveform>>(
    label: &str,
    waveforms: &[W],
    group: &Group,
    make: MakeSegment,
) -> Result<(WaveformValues, Vec<WaveformValues>), String> {
    let rows = group.x.len();
    let x = samples(&group.x)?;
    let axis = x.iter().all(|x| x.is_finite()) && x.windows(2).all(|pair| pair[0] < pair[1]);
    let mut columns: Vec<Cow<'_, [f64]>> = Vec::with_capacity(group.columns.len() + 1);
    let mut signals = Vec::with_capacity(group.columns.len() + 1);
    if !axis {
        signals.push(Signal::new("x", "", SignalKind::Real));
        columns.push(Cow::Borrowed(&x[..]));
    }
    for &(index, part) in &group.columns {
        let waveform = waveforms[index].as_ref();
        let values = series(waveform, part)
            .ok_or_else(|| format!("waveform '{}' lost its complex samples", waveform.name))?;
        if values.len() != rows {
            return Err(format!(
                "waveform '{}' holds {} samples on an axis of {rows}",
                waveform.name,
                values.len()
            ));
        }
        let name = match part {
            Part::Y => waveform.name.clone(),
            Part::Real => format!("re({})", waveform.name),
            Part::Imag => format!("im({})", waveform.name),
        };
        signals.push(Signal::new(
            name,
            waveform.unit.clone().unwrap_or_default(),
            SignalKind::Real,
        ));
        columns.push(samples(values)?);
    }
    let schema = Schema {
        x: Axis {
            name: if axis { "x" } else { "row" }.to_owned(),
            unit: String::new(),
        },
        signals,
    };
    let meta = RunMeta {
        analysis: label.to_owned(),
        ..RunMeta::default()
    };
    let segment = make(&schema, &meta, &mut |push: &mut PushRows<'_>| {
        let mut row_values = Vec::with_capacity(columns.len() * ROWS_PER_PUSH);
        let mut row_x = Vec::with_capacity(ROWS_PER_PUSH);
        let mut first = 0;
        while first < rows {
            let last = (first + ROWS_PER_PUSH).min(rows);
            row_x.clear();
            row_values.clear();
            for row in first..last {
                row_x.push(if axis { x[row] } else { row as f64 });
                row_values.extend(columns.iter().map(|column| column[row]));
            }
            push(&row_x, &row_values)?;
            first = last;
        }
        Ok(())
    })?;
    let window = StoredWindow {
        first: 0,
        rows: rows as u64,
        lead: WindowLead::None,
    };
    let stored = |slot: Option<usize>, name: String| {
        WaveformValues::stored(SegmentColumn::new(Arc::clone(&segment), slot, name), window)
    };
    let (x, offset) = if axis {
        (stored(None, "x".to_owned())?, 0)
    } else {
        (stored(Some(0), "x".to_owned())?, 1)
    };
    let values = schema.signals[offset..]
        .iter()
        .enumerate()
        .map(|(slot, signal)| stored(Some(slot + offset), signal.name.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((x, values))
}

fn kept_failure(error: impl std::fmt::Display) -> String {
    format!("the results store could not keep the samples: {error}")
}

/// A finished segment of this process's store folder.
#[cfg(not(target_arch = "wasm32"))]
fn process_segment(
    schema: &Schema,
    meta: &RunMeta,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    rspice_simulation::results_store::write_finished_segment(schema, meta, |writer| {
        fill(&mut |x, values| writer.push_rows(x, values))
    })
    .map_err(kept_failure)
}

/// The browser has no store folder: its segments are held in memory.
#[cfg(target_arch = "wasm32")]
fn process_segment(
    schema: &Schema,
    meta: &RunMeta,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    memory_segment(schema, meta, fill)
}

/// A finished segment held in memory.
#[cfg(any(test, target_arch = "wasm32"))]
fn memory_segment(
    schema: &Schema,
    meta: &RunMeta,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    let mut writer = SegmentWriter::create(MemoryIo::new(), schema, meta).map_err(kept_failure)?;
    fill(&mut |x, values| writer.push_rows(x, values)).map_err(kept_failure)?;
    let (_, io) = writer
        .finish(rspice_results_store::RunStatus::Completed)
        .map_err(kept_failure)?;
    let reader = SegmentReader::open(Box::new(io) as SegmentBytes).map_err(kept_failure)?;
    Ok(Segment::kept(StoreReader::new(reader), std::path::PathBuf::new()).shared())
}

#[cfg(test)]
mod tests;
