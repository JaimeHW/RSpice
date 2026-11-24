//! The rows a running transient has written to its segment, as the waveforms its host draws
//! while it runs.
//!
//! The run's segment is opened again as it grows. Every chunk but the last is a frame the writer
//! never touches again, and the columns read those rows from the segment, its tiles from the
//! store's summaries as a finished run's are. The last chunk may be the writer's provisional
//! frame, which its next real frame overwrites in place, so its rows are read once as the segment
//! is opened and kept beside the columns: a later read never meets bytes the writer replaced.
//! Reading every column of it can take long enough for the writer to replace it under that one
//! read, at a million columns; such an opening finds nothing, and the next one reads the rows the
//! writer moved on to.

use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_results::store_reader::{SegmentBytes, StoreReader};
use rspice_results::stored_transient::{SlotTarget, slot_targets, transient_shell};
use rspice_results::waveform::{
    ColumnSource, ColumnTile, RetainedWaveform, RowsAround, StoredWindow, WaveformValues,
    WindowLead,
};
use rspice_results_store::{FileIo, SegmentReader, pixel_of};

use crate::results_store::StoredRun;

/// Every opening of a live segment takes the next number, so no two openings share an identity.
static OPENINGS: AtomicU64 = AtomicU64::new(1);

/// What a running transient's segment holds now.
pub struct LiveRows {
    /// Each saved node's and branch's waveform over the rows written, every one on one time axis.
    pub waveforms: Vec<RetainedWaveform>,
    /// The rows written.
    pub rows: u64,
    /// The time of the last row written.
    pub last_time: f64,
}

/// The rows the run writing the segment at `path` has written so far; `None` while it holds no
/// row, or while the writer replaced its last chunk under the read.
pub fn open_live_rows(path: &Path) -> Result<Option<LiveRows>, String> {
    let failed = |error: rspice_results_store::StoreError| format!("{}: {error}", path.display());
    let io = FileIo::open(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let reader = SegmentReader::open(Box::new(io) as SegmentBytes).map_err(failed)?;
    let rows = reader.rows();
    let Some((last_chunk, tail)) = reader.chunk_spans().enumerate().last() else {
        return Ok(None);
    };
    let tail_first = tail.0;
    // A last chunk that fails its check because the writer replaced it since the opening found
    // it is no damage: its rows are read at the next opening.
    let unread = |error| {
        if tail_replaced(path, rows, tail) {
            Ok(None)
        } else {
            Err(failed(error))
        }
    };
    if rows == 0 {
        return Ok(None);
    }
    let shell = transient_shell(reader.schema())?;
    let targets = slot_targets(&reader, &shell)?;
    let mut tail_x = Vec::new();
    if let Err(error) = reader.read_chunk_x(last_chunk, &mut tail_x) {
        return unread(error);
    }
    let mut columns = Vec::new();
    for (slot, &target) in targets.iter().enumerate() {
        let (name, unit) = match target {
            SlotTarget::Voltage(node) => match super::node_waveform(&shell, node) {
                Some((name, unit)) => (name, unit.to_owned()),
                None => continue,
            },
            SlotTarget::Current(branch) => (
                super::branch_waveform_name(&shell.branch_names[branch]),
                "A".to_owned(),
            ),
            _ => continue,
        };
        let mut values = Vec::new();
        if let Err(error) = reader.read_chunk_slot(last_chunk, slot, &mut values) {
            return unread(error);
        }
        columns.push((slot, name, unit, values));
    }
    let last_time = tail_x.last().copied().unwrap_or(f64::NAN);
    let opening = OPENINGS.fetch_add(1, Ordering::Relaxed);
    let segment = Arc::new(StoredRun::kept(
        StoreReader::new(reader),
        path.to_path_buf(),
    ));
    let window = StoredWindow {
        first: 0,
        rows,
        lead: WindowLead::None,
    };
    let tail_x = Arc::new(tail_x);
    let x = WaveformValues::stored(
        Arc::new(LiveColumn {
            segment: Arc::clone(&segment),
            slot: None,
            name: "time".to_owned(),
            tail_first,
            tail: tail_x.as_ref().clone(),
            tail_x: None,
            opening,
        }),
        window,
    )?;
    let mut waveforms = Vec::with_capacity(columns.len());
    for (slot, name, unit, values) in columns {
        let y = WaveformValues::stored(
            Arc::new(LiveColumn {
                segment: Arc::clone(&segment),
                slot: Some(slot),
                name: name.clone(),
                tail_first,
                tail: values,
                tail_x: Some(Arc::clone(&tail_x)),
                opening,
            }),
            window,
        )?;
        waveforms.push(RetainedWaveform::new(name, x.clone(), y).with_unit(unit));
    }
    Ok(Some(LiveRows {
        waveforms,
        rows,
        last_time,
    }))
}

/// Whether the segment at `path` no longer holds `rows` rows ending in the chunk `tail` (its first
/// row and its rows), as an opening found it: the writer wrote on and replaced that chunk since.
fn tail_replaced(path: &Path, rows: u64, tail: (u64, usize)) -> bool {
    let Ok(io) = FileIo::open(path) else {
        return false;
    };
    SegmentReader::open(Box::new(io) as SegmentBytes)
        .is_ok_and(|reader| reader.rows() != rows || reader.chunk_spans().last() != Some(tail))
}

/// One column of a running transient's segment: its rows before the last chunk read from the
/// segment, those of the last chunk from the copy taken as it was opened.
struct LiveColumn {
    segment: Arc<StoredRun>,
    /// The value slot, or `None` for the time axis.
    slot: Option<usize>,
    name: String,
    /// The first row of the last chunk.
    tail_first: u64,
    /// The column's rows from `tail_first` on.
    tail: Vec<f64>,
    /// The time axis's rows from `tail_first` on, for a value column.
    tail_x: Option<Arc<Vec<f64>>>,
    opening: u64,
}

impl LiveColumn {
    fn tail_x(&self) -> &[f64] {
        self.tail_x.as_deref().map_or(&self.tail, Vec::as_slice)
    }

    /// `(x, value)` of rows `rows`, read from the segment before the last chunk and from the copy
    /// after.
    fn gather(&self, rows: Range<u64>) -> Result<Vec<(f64, f64)>, String> {
        let stored = rows.start..rows.end.min(self.tail_first).max(rows.start);
        let mut out = Vec::with_capacity((rows.end - rows.start) as usize);
        if !stored.is_empty() {
            let count = (stored.end - stored.start) as usize;
            let mut x = vec![0.0; count];
            let mut y = vec![0.0; count];
            self.segment
                .read(None, stored.start, &mut x)
                .map_err(|error| error.to_string())?;
            self.segment
                .read(self.slot, stored.start, &mut y)
                .map_err(|error| error.to_string())?;
            out.extend(x.into_iter().zip(y));
        }
        let from = rows.start.max(self.tail_first);
        for row in from..rows.end {
            let index = (row - self.tail_first) as usize;
            out.push((self.tail_x()[index], self.tail[index]));
        }
        Ok(out)
    }
}

impl ColumnSource for LiveColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        self.segment.path().display().to_string()
    }

    fn rows(&self) -> u64 {
        self.tail_first + self.tail.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        let end = first + out.len() as u64;
        if end > self.rows() {
            return Err(format!(
                "rows {first} .. {end} of a running transient with {} rows",
                self.rows()
            ));
        }
        let stored = (self.tail_first.min(end).saturating_sub(first)) as usize;
        if stored > 0 {
            self.segment
                .read(self.slot, first, &mut out[..stored])
                .map_err(|error| error.to_string())?;
        }
        let from = first.max(self.tail_first);
        for (offset, row) in (from..end).enumerate() {
            out[stored + offset] = self.tail[(row - self.tail_first) as usize];
        }
        Ok(())
    }

    /// The time axis: the engine accepts each point after the one before it.
    fn ascending(&self) -> bool {
        self.slot.is_none()
    }

    /// Unique to this opening: a later opening holds more rows under another identity.
    fn identity(&self) -> Option<([u8; 32], u64)> {
        let mut digest = [0u8; 32];
        digest[..12].copy_from_slice(b"rspice-live\0");
        digest[12..20].copy_from_slice(&self.opening.to_le_bytes());
        let index = self.slot.map_or(0, |slot| slot as u64 + 1);
        Some((digest, index))
    }

    /// A stand-in unique to this opening and column: a running transient's column has no digest
    /// until its run ends, and digesting the rows it holds so far would read them.
    fn column_digest(&self) -> Option<[u8; 32]> {
        let (mut digest, index) = self.identity()?;
        digest[20..28].copy_from_slice(&index.to_le_bytes());
        Some(digest)
    }

    fn tile(
        &self,
        rows: Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Option<Result<ColumnTile, String>> {
        let slot = self.slot?;
        let stored = rows.start..rows.end.min(self.tail_first).max(rows.start);
        let tile = if stored.is_empty() {
            ColumnTile::Samples {
                x: Vec::new(),
                y: Vec::new(),
                before: None,
                after: None,
            }
        } else {
            match self.segment.reader().tile(slot, stored, x0, x1, pixels) {
                Ok(tile) => ColumnTile::from(tile),
                Err(error) => return Some(Err(error.to_string())),
            }
        };
        let from = (rows.start.max(self.tail_first) - self.tail_first) as usize;
        let to = (rows.end.max(self.tail_first) - self.tail_first) as usize;
        let to = to.min(self.tail.len());
        let from = from.min(to);
        Some(Ok(fold_rows(
            tile,
            &self.tail_x()[from..to],
            &self.tail[from..to],
            x0,
            x1,
            pixels,
        )))
    }

    fn rows_around(&self, rows: Range<u64>, x: f64) -> Option<Result<RowsAround, String>> {
        let slot = self.slot?;
        let tail_x = self.tail_x();
        let in_tail = rows.end > self.tail_first
            && tail_x
                .get((rows.start.max(self.tail_first) - self.tail_first) as usize)
                .is_some_and(|first| x >= *first);
        // The rows at or before `x`.
        let at_or_before = if in_tail || rows.start >= self.tail_first {
            let from = (rows.start.max(self.tail_first) - self.tail_first) as usize;
            let to = ((rows.end - self.tail_first) as usize).min(tail_x.len());
            self.tail_first + from as u64 + tail_x[from..to].partition_point(|row| *row <= x) as u64
        } else {
            let stored = rows.start..rows.end.min(self.tail_first);
            match self.segment.reader().rows_around(slot, stored, x) {
                Ok((first, found)) => {
                    first + found.iter().take_while(|(row, _)| *row <= x).count() as u64
                }
                Err(error) => return Some(Err(error.to_string())),
            }
        };
        let first = at_or_before.saturating_sub(2).max(rows.start);
        let last = (at_or_before + 2).min(rows.end);
        Some(self.gather(first..last).map(|found| (first, found)))
    }
}

/// `tile` with the rows `xs`, `ys` folded in: rows that come after every row `tile` was read
/// from. A view that would then hold more than two rows a pixel is an envelope.
fn fold_rows(
    tile: ColumnTile,
    xs: &[f64],
    ys: &[f64],
    x0: f64,
    x1: f64,
    pixels: usize,
) -> ColumnTile {
    if xs.is_empty() || pixels == 0 {
        return tile;
    }
    match tile {
        ColumnTile::Samples {
            mut x,
            mut y,
            mut before,
            mut after,
        } => {
            for (&row_x, &row_y) in xs.iter().zip(ys) {
                if row_x < x0 {
                    before = Some((row_x, row_y));
                } else if row_x <= x1 {
                    x.push(row_x);
                    y.push(row_y);
                } else if after.is_none() {
                    after = Some((row_x, row_y));
                }
            }
            if x.len() <= 2 * pixels {
                return ColumnTile::Samples {
                    x,
                    y,
                    before,
                    after,
                };
            }
            let envelope = ColumnTile::Envelope {
                min: vec![f64::NAN; pixels],
                max: vec![f64::NAN; pixels],
                touched: vec![false; pixels],
            };
            fold_rows(envelope, &x, &y, x0, x1, pixels)
        }
        ColumnTile::Envelope {
            mut min,
            mut max,
            mut touched,
        } => {
            for (&row_x, &row_y) in xs.iter().zip(ys) {
                if !(x0..=x1).contains(&row_x) {
                    continue;
                }
                let pixel = pixel_of(row_x, x0, x1, pixels);
                touched[pixel] = true;
                if row_y.is_nan() {
                    continue;
                }
                if min[pixel].is_nan() || row_y < min[pixel] {
                    min[pixel] = row_y;
                }
                if max[pixel].is_nan() || row_y > max[pixel] {
                    max[pixel] = row_y;
                }
            }
            ColumnTile::Envelope { min, max, touched }
        }
    }
}

#[cfg(test)]
mod tests;
