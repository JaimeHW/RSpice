//! An analysis kept in results store segments: each waveform holding a series in memory is
//! written to a segment, one segment per distinct x axis, and its series become windows of the
//! segment's columns. The desktop writes those segments into its store folder and the browser
//! through its store worker; both group the waveforms here, so a run keeps the same segments on
//! either.
//!
//! An x axis that ascends strictly is the segment's own axis, so a view tiles the waveforms on it
//! from the segment's summaries. Any other axis (a DC sweep that runs down, a nested sweep that
//! repeats) is a column of its own beside an axis of row numbers, and is read as values.

use std::borrow::Cow;
use std::sync::Arc;

pub use rspice_results_store::{Axis, RunMeta, Schema, Signal, SignalKind, StoreError};

use crate::analysis_result::AnalysisResult;
use crate::store_reader::{Segment, SegmentColumn};
use crate::waveform::{RetainedWaveform, StoredWindow, WaveformValues, WindowLead};

/// The most rows pushed into a segment at once.
pub const ROWS_PER_PUSH: usize = 4096;

/// The most values one push carries across a segment's columns, 8 MiB of them. A block of rows
/// stays that size however many columns the segment has, so a run of a million signals is pushed
/// a row at a time, not [`ROWS_PER_PUSH`] rows of every signal at once (32 GB).
const VALUES_PER_PUSH: usize = 1 << 20;

/// Pushes rows (their x, then their value columns row-major) into a segment's writer.
pub type PushRows<'a> = dyn FnMut(&[f64], &[f64]) -> Result<(), StoreError> + 'a;

/// Fills a segment through the [`PushRows`] it is handed.
pub type FillSegment<'a> = dyn FnMut(&mut PushRows<'_>) -> Result<(), StoreError> + 'a;

/// Makes a finished segment of a schema and run metadata from the rows its filler pushes.
pub type MakeSegment<'a> =
    dyn FnMut(&Schema, &RunMeta, &mut FillSegment<'_>) -> Result<Arc<Segment>, String> + 'a;

/// Which series of a waveform a column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Y,
    Real,
    Imag,
}

/// The waveforms of an analysis that share one x axis: the series of theirs one segment keeps,
/// and that segment's shape.
#[derive(Debug)]
pub struct SegmentGroup {
    x: WaveformValues,
    /// (waveform index, series) per value column, in column order.
    columns: Vec<(usize, Part)>,
    /// The x axis ascends strictly, so it is the segment's own axis.
    axis: bool,
    schema: Schema,
    meta: RunMeta,
}

impl SegmentGroup {
    /// The shape of the segment that keeps this group.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// The run metadata the segment states.
    pub fn meta(&self) -> &RunMeta {
        &self.meta
    }

    /// The rows the segment keeps.
    pub fn rows(&self) -> usize {
        self.x.len()
    }

    /// Whether the x axis is the segment's own axis (ascending strictly), not a column beside
    /// row numbers.
    pub fn axis(&self) -> bool {
        self.axis
    }

    /// Push every row of this group, at most [`ROWS_PER_PUSH`] at a time and no more than
    /// [`VALUES_PER_PUSH`] values, reading each series once.
    pub fn fill<W: AsRef<RetainedWaveform>>(
        &self,
        waveforms: &[W],
        push: &mut PushRows<'_>,
    ) -> Result<(), StoreError> {
        let failed = |message: String| StoreError::Io {
            kind: std::io::ErrorKind::InvalidData,
            message,
        };
        let rows = self.rows();
        let x = samples(&self.x).map_err(failed)?;
        let mut columns: Vec<Cow<'_, [f64]>> = Vec::with_capacity(self.columns.len() + 1);
        if !self.axis {
            columns.push(Cow::Borrowed(&x[..]));
        }
        for &(index, part) in &self.columns {
            let waveform = waveforms[index].as_ref();
            let values = series(waveform, part).ok_or_else(|| {
                failed(format!(
                    "waveform '{}' lost its complex samples",
                    waveform.name
                ))
            })?;
            if values.len() != rows {
                return Err(failed(format!(
                    "waveform '{}' holds {} samples on an axis of {rows}",
                    waveform.name,
                    values.len()
                )));
            }
            columns.push(samples(values).map_err(failed)?);
        }
        let block = (VALUES_PER_PUSH / columns.len().max(1)).clamp(1, ROWS_PER_PUSH);
        let mut row_values = Vec::with_capacity(columns.len() * block.min(rows));
        let mut row_x = Vec::with_capacity(block.min(rows));
        let mut first = 0;
        while first < rows {
            let last = (first + block).min(rows);
            row_x.clear();
            row_values.clear();
            for row in first..last {
                row_x.push(if self.axis { x[row] } else { row as f64 });
                row_values.extend(columns.iter().map(|column| column[row]));
            }
            push(&row_x, &row_values)?;
            first = last;
        }
        Ok(())
    }
}

/// The segments that keep the series an analysis holds in memory, one per distinct x axis.
#[derive(Debug, Default)]
pub struct StoredAnalysis {
    groups: Vec<SegmentGroup>,
}

impl StoredAnalysis {
    /// The segments `analysis` needs: none when every waveform is already a window of a segment
    /// that states its digest. An analysis that does not validate is refused, since validation
    /// never reads a stored series again.
    pub fn plan<W: AsRef<RetainedWaveform>>(analysis: &AnalysisResult<W>) -> Result<Self, String> {
        let mut groups: Vec<(WaveformValues, Vec<(usize, Part)>)> = Vec::new();
        for (index, waveform) in analysis.waveforms.iter().enumerate() {
            let waveform = waveform.as_ref();
            if !holds_memory(waveform) {
                continue;
            }
            let group = match groups.iter().position(|(x, _)| same_axis(x, &waveform.x)) {
                Some(group) => group,
                None => {
                    groups.push((waveform.x.clone(), Vec::new()));
                    groups.len() - 1
                }
            };
            let parts: &[Part] = if waveform.complex.is_some() {
                &[Part::Y, Part::Real, Part::Imag]
            } else {
                &[Part::Y]
            };
            groups[group]
                .1
                .extend(parts.iter().map(|&part| (index, part)));
        }
        if groups.is_empty() {
            return Ok(Self::default());
        }
        // A result is checked while its samples are in memory, and one that fails stays there,
        // so every save and restore refuses it.
        analysis
            .validate_retained_evidence()
            .map_err(|reason| format!("the results store keeps only valid results: {reason}"))?;
        let groups = groups
            .into_iter()
            .map(|(x, columns)| group(&analysis.label, &analysis.waveforms, x, columns))
            .collect::<Result<_, _>>()?;
        Ok(Self { groups })
    }

    /// The segments, in the order their waveforms first appear.
    pub fn groups(&self) -> &[SegmentGroup] {
        &self.groups
    }

    /// Whether the analysis needs no segment.
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Make every waveform of `analysis` read the segments that now keep it: `column(group,
    /// slot, name)` gives the window of the whole column `slot` (`None` for the segment's own
    /// axis) of group `group`'s segment.
    pub fn apply<W: AsMut<RetainedWaveform>>(
        &self,
        analysis: &mut AnalysisResult<W>,
        mut column: impl FnMut(usize, Option<usize>, String) -> Result<WaveformValues, String>,
    ) -> Result<(), String> {
        let mut kept = Vec::with_capacity(self.groups.len());
        for (index, group) in self.groups.iter().enumerate() {
            let (x, offset) = if group.axis {
                (column(index, None, "x".to_owned())?, 0)
            } else {
                (column(index, Some(0), "x".to_owned())?, 1)
            };
            let values = group.schema.signals[offset..]
                .iter()
                .enumerate()
                .map(|(slot, signal)| column(index, Some(slot + offset), signal.name.clone()))
                .collect::<Result<Vec<_>, _>>()?;
            kept.push((x, values));
        }
        for (group, (x, values)) in self.groups.iter().zip(kept) {
            for (&(index, part), values) in group.columns.iter().zip(values) {
                let waveform = analysis.waveforms[index].as_mut();
                waveform.x = x.clone();
                if let Some(series) = series_mut(waveform, part) {
                    *series = values;
                }
            }
        }
        Ok(())
    }
}

/// Write every series `analysis` holds in memory into results store segments, each made by
/// `make` from its shape and the rows it is filled with, and make its waveforms read them. An
/// analysis whose waveforms are all stored is left as it is, and one that does not validate is
/// refused. On failure nothing changes: the waveforms keep their samples in memory.
pub fn store_analysis<W>(
    analysis: &mut AnalysisResult<W>,
    make: &mut MakeSegment<'_>,
) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    let plan = StoredAnalysis::plan(analysis)?;
    let mut segments = Vec::with_capacity(plan.groups.len());
    for group in &plan.groups {
        let waveforms = &analysis.waveforms;
        segments.push(make(group.schema(), group.meta(), &mut |push| {
            group.fill(waveforms, push)
        })?);
    }
    plan.apply(analysis, |group, slot, name| {
        whole_column(&segments[group], slot, name, plan.groups[group].rows())
    })
}

/// The window of all `rows` rows of the column `slot` (`None` for the axis) of `segment`.
pub fn whole_column(
    segment: &Arc<Segment>,
    slot: Option<usize>,
    name: String,
    rows: usize,
) -> Result<WaveformValues, String> {
    let window = StoredWindow {
        first: 0,
        rows: rows as u64,
        lead: WindowLead::None,
    };
    WaveformValues::stored(SegmentColumn::new(Arc::clone(segment), slot, name), window)
}

/// One group's segment shape: its axis, read once to see whether it ascends strictly.
fn group<W: AsRef<RetainedWaveform>>(
    label: &str,
    waveforms: &[W],
    x: WaveformValues,
    columns: Vec<(usize, Part)>,
) -> Result<SegmentGroup, String> {
    let axis = {
        let x = samples(&x)?;
        x.iter().all(|x| x.is_finite()) && x.windows(2).all(|pair| pair[0] < pair[1])
    };
    let mut signals = Vec::with_capacity(columns.len() + 1);
    if !axis {
        signals.push(Signal::new("x", "", SignalKind::Real));
    }
    for &(index, part) in &columns {
        let waveform = waveforms[index].as_ref();
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
    }
    Ok(SegmentGroup {
        x,
        columns,
        axis,
        schema: Schema {
            x: Axis {
                name: if axis { "x" } else { "row" }.to_owned(),
                unit: String::new(),
            },
            signals,
        },
        meta: RunMeta {
            analysis: label.to_owned(),
            ..RunMeta::default()
        },
    })
}

/// Whether a saved file can name `values`: a window of a segment that states its digest. A
/// segment its run left without `END` states none, and is kept again as if in memory, read once.
fn named_by_store(values: &WaveformValues) -> bool {
    values
        .stored_window()
        .is_some_and(|(source, _)| source.identity().is_some())
}

/// Whether any series of `waveform` is held in memory, or in a segment that states no digest.
pub fn holds_memory(waveform: &RetainedWaveform) -> bool {
    let complex = waveform
        .complex
        .iter()
        .flat_map(|complex| [&complex.real, &complex.imag]);
    [&waveform.x, &waveform.y]
        .into_iter()
        .chain(complex)
        .any(|values| !named_by_store(values))
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
    let (Some(left), Some(right)) = (left.in_memory(), right.in_memory()) else {
        return false;
    };
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

/// The samples of `values`, read once.
fn samples(values: &WaveformValues) -> Result<Cow<'_, [f64]>, String> {
    values
        .read_range(0..values.len())
        .map_err(|failure| failure.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis_type::AnalysisType;

    /// A segment of many columns is pushed in blocks no larger than a push holds, every row
    /// once and in order.
    #[test]
    fn a_wide_segment_is_pushed_in_blocks_no_larger_than_a_push_holds() {
        const COLUMNS: usize = 1_000;
        const ROWS: usize = 1_100;
        let value = |column: usize, row: usize| (column * ROWS + row) as f64;
        let x: Vec<f64> = (0..ROWS).map(|row| row as f64).collect();
        let waveforms: Vec<RetainedWaveform> = (0..COLUMNS)
            .map(|column| {
                let y: Vec<f64> = (0..ROWS).map(|row| value(column, row)).collect();
                RetainedWaveform::new(format!("V(n{column})"), x.clone(), y)
            })
            .collect();
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(waveforms);
        let plan = StoredAnalysis::plan(&analysis).expect("the analysis is grouped");
        let [group] = plan.groups() else {
            panic!("one x axis is one segment");
        };
        let mut pushes = 0;
        let mut next_row = 0;
        group
            .fill(&analysis.waveforms, &mut |row_x: &[f64], values: &[f64]| {
                assert!(values.len() <= VALUES_PER_PUSH, "{} values", values.len());
                assert_eq!(values.len(), row_x.len() * COLUMNS);
                for (&at, row) in row_x.iter().zip(values.chunks(COLUMNS)) {
                    assert_eq!(at, next_row as f64);
                    assert!(
                        row.iter()
                            .enumerate()
                            .all(|(column, &held)| held == value(column, next_row))
                    );
                    next_row += 1;
                }
                pushes += 1;
                Ok::<(), StoreError>(())
            })
            .expect("every row is pushed");
        assert_eq!(next_row, ROWS, "every row, once");
        assert_eq!(pushes, 2, "a thousand and forty-eight rows, then the rest");
    }
}
