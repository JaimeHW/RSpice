//! The view query: N pixels over [x0, x1] of one analog signal.
//!
//! A pixel's value is the min and max over the rows that fall in it. Rows are taken in units: the
//! buckets of the coarsest summary level with at least 2N buckets over the view, read straight from
//! the `SUMMARY` frames. A unit whose x span touches one or two pixels contributes its min and max
//! to them; a unit that spans three or more (sparse rows: large steps) is refined into the 16 units
//! of the level below, down to single rows. So a pixel's envelope holds every row whose x lies in
//! it, plus at most one unit's rows from a neighbour straddling its edge, and a view costs reads of
//! a few hundred summary blocks however long the run. When at most 2N rows fall in the view, the
//! rows themselves are returned.
//!
//! A view may be limited to a range of rows (a waveform's window of the run), and reads every
//! block through a [`BlockCache`]: a host that keeps the blocks reads none of them again for the
//! next view over the same rows.

use std::ops::Range;
use std::sync::Arc;

use crate::format::{LEVEL1_ROWS, bucket_rows, fold_max, fold_min, summary_frame_rows};
use crate::reader::{ChunkRef, SegmentReader};
use crate::writer::magnitude;
use crate::{SegmentIo, StoreError};

/// What [`SegmentReader::tile`] returns. A complex signal is drawn as |z|.
#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    /// At most two rows per pixel: the rows with x in the view, and the nearest row on either side
    /// of them so a line can be drawn to the view's edges.
    Samples {
        x: Vec<f64>,
        y: Vec<f64>,
        before: Option<(f64, f64)>,
        after: Option<(f64, f64)>,
    },
    /// Per pixel, the min and max of its rows (NaN when it has none, or only NaN values), and
    /// whether any row touched it. `level` is the summary level the units came from (0: rows).
    Envelope {
        level: u8,
        min: Vec<f64>,
        max: Vec<f64>,
        touched: Vec<bool>,
    },
}

/// The pixel holding `x`, for `x0 <= x <= x1`: `floor((x - x0) / (x1 - x0) * pixels)`, with `x1`
/// itself in the last pixel.
pub fn pixel_of(x: f64, x0: f64, x1: f64, pixels: usize) -> usize {
    if x1 <= x0 {
        return 0;
    }
    let pixel = ((x - x0) / (x1 - x0) * pixels as f64) as usize;
    pixel.min(pixels - 1)
}

/// How a unit landed.
#[derive(PartialEq, Eq)]
enum Fit {
    Outside,
    Done,
    Refine,
}

struct Pixels {
    x0: f64,
    x1: f64,
    min: Vec<f64>,
    max: Vec<f64>,
    touched: Vec<bool>,
}

impl Pixels {
    fn new(x0: f64, x1: f64, pixels: usize) -> Self {
        Self {
            x0,
            x1,
            min: vec![f64::NAN; pixels],
            max: vec![f64::NAN; pixels],
            touched: vec![false; pixels],
        }
    }

    /// Offer a unit spanning `lo ..= hi` in x with values `min ..= max`.
    fn unit(&mut self, lo: f64, hi: f64, min: f64, max: f64) -> Fit {
        if !(hi >= self.x0 && lo <= self.x1) {
            return Fit::Outside;
        }
        let pixels = self.min.len();
        let first = pixel_of(
            if lo > self.x0 { lo } else { self.x0 },
            self.x0,
            self.x1,
            pixels,
        );
        let last = pixel_of(
            if hi < self.x1 { hi } else { self.x1 },
            self.x0,
            self.x1,
            pixels,
        );
        if last - first >= 2 {
            return Fit::Refine;
        }
        for pixel in first..=last {
            self.min[pixel] = fold_min(self.min[pixel], min);
            self.max[pixel] = fold_max(self.max[pixel], max);
            self.touched[pixel] = true;
        }
        Fit::Done
    }
}

/// The signal a tile draws.
#[derive(Clone, Copy)]
struct Column {
    /// Position among the analog signals (its summaries' position).
    index: usize,
    slot: usize,
    complex: bool,
}

/// One block of decoded values a view reads, named by what it covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockKey {
    /// The analog signal's position among the segment's analog signals, or `None` for x.
    pub column: Option<u32>,
    /// 0: a chunk's rows (a complex signal's |z|); 1: a chunk's 64-row summary buckets; 2 and
    /// up: a `SUMMARY` frame's buckets. x of a summary is each bucket's (first, last) x; a value
    /// summary is each bucket's (min, max).
    pub level: u8,
    /// The first row the block covers: its chunk's or its frame's.
    pub first_row: u64,
}

/// Where a view keeps the blocks it decoded, so the next view over the same rows reads none of
/// them again. Its host bounds what it keeps; a block it dropped is read again when asked for.
pub trait BlockCache {
    /// The block `key` names, if it is kept.
    fn get(&mut self, key: &BlockKey) -> Option<Arc<[f64]>>;
    /// Keep `block` as `key`.
    fn put(&mut self, key: BlockKey, block: Arc<[f64]>);
}

/// A block of decoded values, shared with the cache that keeps it.
type Block = Arc<[f64]>;

/// A cache that keeps nothing: every view reads its blocks.
pub struct NoBlockCache;

impl BlockCache for NoBlockCache {
    fn get(&mut self, _: &BlockKey) -> Option<Arc<[f64]>> {
        None
    }

    fn put(&mut self, _: BlockKey, _: Arc<[f64]>) {}
}

impl<I: SegmentIo> SegmentReader<I> {
    /// Per-pixel min/max (or the rows themselves) of analog signal `signal` over `x0 ..= x1`.
    pub fn tile(
        &mut self,
        signal: usize,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Result<Tile, StoreError> {
        let rows = 0..self.rows();
        self.tile_in(signal, rows, x0, x1, pixels, &mut NoBlockCache)
    }

    /// [`Self::tile`] of the rows in `rows` alone, reading each block through `cache`.
    pub fn tile_in(
        &mut self,
        signal: usize,
        rows: Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
        cache: &mut dyn BlockCache,
    ) -> Result<Tile, StoreError> {
        if pixels == 0 || !(x0.is_finite() && x1.is_finite() && x0 <= x1) {
            return Err(StoreError::OutOfRange(format!(
                "a {pixels}-pixel view over [{x0}, {x1}]"
            )));
        }
        let column = self.drawn_column(signal)?;
        let rows = rows.start.min(self.rows())..rows.end.min(self.rows());
        let estimate: u64 = self
            .chunks()
            .iter()
            .filter(|chunk| chunk.x_max >= x0 && chunk.x_min <= x1)
            .map(|chunk| {
                rows.end
                    .min(chunk.end_row())
                    .saturating_sub(rows.start.max(chunk.first_row))
            })
            .sum();
        let wanted = 2 * pixels as u64;
        let level = (1..=self.top_level())
            .rev()
            .find(|&level| estimate >= wanted * bucket_rows(level))
            .unwrap_or(0);
        if level == 0 {
            return self.raw_tile(column, rows, x0, x1, pixels, cache);
        }
        let mut view = Pixels::new(x0, x1, pixels);
        self.units(level, rows.start, rows.end, column, &mut view, cache)?;
        Ok(Tile::Envelope {
            level,
            min: view.min,
            max: view.max,
            touched: view.touched,
        })
    }

    /// The rows of `rows` around `x` in an ascending x, each as (x, drawn value), with the
    /// index of the first: up to two at or before `x` and up to two after it, consecutive, so a
    /// cursor reads `x` from them as it would from the whole column (a cubic reads two either
    /// side).
    pub fn rows_around(
        &mut self,
        signal: usize,
        rows: Range<u64>,
        x: f64,
        cache: &mut dyn BlockCache,
    ) -> Result<(u64, Vec<(f64, f64)>), StoreError> {
        let column = self.drawn_column(signal)?;
        let rows = rows.start.min(self.rows())..rows.end.min(self.rows());
        if rows.is_empty() || x.is_nan() {
            return Ok((rows.start, Vec::new()));
        }
        // The first chunk reaching `x`; the last row at or before it is there or the chunk before.
        let first = self.chunk_containing(rows.start);
        let last = self.chunk_containing(rows.end - 1);
        let at = first + self.chunks()[first..=last].partition_point(|chunk| chunk.x_max < x);
        let mut below = None;
        for index in [at.min(last), at.saturating_sub(1).max(first)] {
            let chunk = self.chunks()[index];
            let (xs, _) = self.drawn_blocks(&chunk, column, cache)?;
            let from = rows.start.max(chunk.first_row) - chunk.first_row;
            let to = rows.end.min(chunk.end_row()) - chunk.first_row;
            let count = xs[from as usize..to as usize].partition_point(|&row_x| row_x <= x);
            if count > 0 {
                below = Some(chunk.first_row + from + count as u64 - 1);
                break;
            }
        }
        let (start, end) = match below {
            Some(row) => (
                row.saturating_sub(1).max(rows.start),
                (row + 3).min(rows.end),
            ),
            None => (rows.start, (rows.start + 2).min(rows.end)),
        };
        let around = (start..end)
            .map(|row| self.drawn_row(row, column, cache))
            .collect::<Result<_, _>>()?;
        Ok((start, around))
    }

    /// The analog signal `signal` as a tile draws it.
    fn drawn_column(&self, signal: usize) -> Result<Column, StoreError> {
        let slot = self.schema().slot_of(signal).ok_or_else(|| {
            StoreError::OutOfRange(format!("signal {signal} is not an analog signal"))
        })?;
        let index = self
            .analog()
            .iter()
            .position(|column| column.slot == slot)
            .unwrap_or_default();
        Ok(Column {
            index,
            slot,
            complex: self.analog()[index].complex,
        })
    }

    /// The block `key`, from `cache` or by `read`, which fills the vector it is given.
    fn cached(
        &mut self,
        cache: &mut dyn BlockCache,
        key: BlockKey,
        read: impl FnOnce(&mut Self, &mut Vec<f64>) -> Result<(), StoreError>,
    ) -> Result<Arc<[f64]>, StoreError> {
        if let Some(block) = cache.get(&key) {
            return Ok(block);
        }
        let mut values = Vec::new();
        read(self, &mut values)?;
        let block: Arc<[f64]> = values.into();
        cache.put(key, Arc::clone(&block));
        Ok(block)
    }

    /// x and the drawn value (|z| for a complex signal) of every row of `chunk`.
    fn drawn_blocks(
        &mut self,
        chunk: &ChunkRef,
        column: Column,
        cache: &mut dyn BlockCache,
    ) -> Result<(Block, Block), StoreError> {
        let rows = chunk.rows as usize;
        let at = |column: Option<u32>| BlockKey {
            column,
            level: 0,
            first_row: chunk.first_row,
        };
        let x = self.cached(cache, at(None), |reader, out| {
            out.resize(rows, 0.0);
            reader.decode_block(chunk, 0, out)
        })?;
        let y = self.cached(cache, at(Some(column.index as u32)), |reader, out| {
            out.resize(rows, 0.0);
            reader.decode_block(chunk, column.slot + 1, out)?;
            if column.complex {
                let mut im = vec![0.0; rows];
                reader.decode_block(chunk, column.slot + 2, &mut im)?;
                for (re, im) in out.iter_mut().zip(&im) {
                    *re = magnitude(*re, *im);
                }
            }
            Ok(())
        })?;
        Ok((x, y))
    }

    /// The row at `row`: (x, drawn value).
    fn drawn_row(
        &mut self,
        row: u64,
        column: Column,
        cache: &mut dyn BlockCache,
    ) -> Result<(f64, f64), StoreError> {
        let chunk = self.chunks()[self.chunk_containing(row)];
        let (x, y) = self.drawn_blocks(&chunk, column, cache)?;
        let at = (row - chunk.first_row) as usize;
        Ok((x[at], y[at]))
    }

    /// Few enough rows to decode: exact samples, or an exact per-row envelope.
    fn raw_tile(
        &mut self,
        column: Column,
        rows: Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
        cache: &mut dyn BlockCache,
    ) -> Result<Tile, StoreError> {
        let (mut xs, mut ys, mut kept) = (Vec::new(), Vec::new(), Vec::new());
        let mut last_before = None;
        for index in 0..self.chunks().len() {
            let chunk = self.chunks()[index];
            let from = rows.start.max(chunk.first_row);
            let to = rows.end.min(chunk.end_row());
            if from >= to {
                continue;
            }
            if chunk.x_max < x0 {
                last_before = Some(to - 1);
                continue;
            }
            if chunk.x_min > x1 {
                continue;
            }
            let (x, y) = self.drawn_blocks(&chunk, column, cache)?;
            for row in from..to {
                let at = (row - chunk.first_row) as usize;
                if x[at] >= x0 && x[at] <= x1 {
                    xs.push(x[at]);
                    ys.push(y[at]);
                    kept.push(row);
                } else if x[at] < x0 {
                    last_before = Some(row);
                }
            }
        }
        if xs.len() > 2 * pixels {
            let mut view = Pixels::new(x0, x1, pixels);
            for (&x, &y) in xs.iter().zip(&ys) {
                view.unit(x, x, y, y);
            }
            return Ok(Tile::Envelope {
                level: 0,
                min: view.min,
                max: view.max,
                touched: view.touched,
            });
        }
        let before_row = match kept.first() {
            Some(&first) => first.checked_sub(1).filter(|&row| row >= rows.start),
            None => last_before,
        };
        let after_row = match kept.last() {
            Some(&last) => Some(last + 1),
            None => Some(last_before.map_or(rows.start, |row| row + 1)),
        }
        .filter(|&row| row < rows.end);
        let before = before_row
            .map(|row| self.drawn_row(row, column, cache))
            .transpose()?;
        let after = after_row
            .map(|row| self.drawn_row(row, column, cache))
            .transpose()?;
        Ok(Tile::Samples {
            x: xs,
            y: ys,
            before,
            after,
        })
    }

    /// Offer rows `r0 .. r1` to the view as units of `level`, refining where a unit is too wide.
    fn units(
        &mut self,
        level: u8,
        r0: u64,
        r1: u64,
        column: Column,
        view: &mut Pixels,
        cache: &mut dyn BlockCache,
    ) -> Result<(), StoreError> {
        if r0 >= r1 {
            return Ok(());
        }
        let value_key = |level: u8, first_row: u64| BlockKey {
            column: Some(column.index as u32),
            level,
            first_row,
        };
        let x_key = |level: u8, first_row: u64| BlockKey {
            column: None,
            level,
            first_row,
        };
        match level {
            0 => {
                let mut index = self.chunk_containing(r0);
                while index < self.chunks().len() && self.chunks()[index].first_row < r1 {
                    let chunk = self.chunks()[index];
                    index += 1;
                    if !(chunk.x_max >= view.x0 && chunk.x_min <= view.x1) {
                        continue;
                    }
                    let (x, y) = self.drawn_blocks(&chunk, column, cache)?;
                    let from = r0.max(chunk.first_row) - chunk.first_row;
                    let to = r1.min(chunk.end_row()) - chunk.first_row;
                    for at in from as usize..to as usize {
                        view.unit(x[at], x[at], y[at], y[at]);
                    }
                }
            }
            1 => {
                let blocks = self.slots() + 1;
                let mut index = self.chunk_containing(r0);
                while index < self.chunks().len() && self.chunks()[index].first_row < r1 {
                    let chunk = self.chunks()[index];
                    index += 1;
                    if !(chunk.x_max >= view.x0 && chunk.x_min <= view.x1) {
                        continue;
                    }
                    let buckets = (chunk.rows as usize).div_ceil(LEVEL1_ROWS);
                    let at = chunk.offset + 16 + 24 + 12 * blocks as u64;
                    let xs = self.cached(cache, x_key(1, chunk.first_row), |reader, out| {
                        reader.read_f64s(at, 2 * buckets, out)
                    })?;
                    let values =
                        self.cached(cache, value_key(1, chunk.first_row), |reader, out| {
                            reader.read_f64s(
                                at + (16 * buckets * (1 + column.index)) as u64,
                                2 * buckets,
                                out,
                            )
                        })?;
                    for bucket in 0..buckets {
                        let first = chunk.first_row + (bucket * LEVEL1_ROWS) as u64;
                        let end = (first + LEVEL1_ROWS as u64).min(chunk.end_row());
                        if end <= r0 || first >= r1 {
                            continue;
                        }
                        // A bucket only partly in the rows is refined, so no row outside them
                        // reaches the view.
                        let (lo, hi) = (xs[2 * bucket], xs[2 * bucket + 1]);
                        if first < r0
                            || end > r1
                            || view.unit(lo, hi, values[2 * bucket], values[2 * bucket + 1])
                                == Fit::Refine
                        {
                            self.units(0, first.max(r0), end.min(r1), column, view, cache)?;
                        }
                    }
                }
            }
            _ => {
                let frame_rows = summary_frame_rows(level);
                let width = bucket_rows(level);
                let frames = self.summaries(level);
                let covered = frames
                    .last()
                    .map_or(0, |frame| frame.first_row + frame.rows);
                let first_frame = (r0 / frame_rows) as usize;
                let mut ordinal = first_frame;
                while ordinal < self.summaries(level).len() && (ordinal as u64 * frame_rows) < r1 {
                    let frame = self.summaries(level)[ordinal];
                    ordinal += 1;
                    if !(frame.x_max >= view.x0 && frame.x_min <= view.x1) {
                        continue;
                    }
                    let buckets = frame.buckets as usize;
                    let at = frame.offset + 16 + 32;
                    let xs = self.cached(cache, x_key(level, frame.first_row), |reader, out| {
                        reader.read_f64s(at, 2 * buckets, out)
                    })?;
                    let values =
                        self.cached(cache, value_key(level, frame.first_row), |reader, out| {
                            reader.read_f64s(
                                at + (16 * buckets * (1 + column.index)) as u64,
                                2 * buckets,
                                out,
                            )
                        })?;
                    for bucket in 0..buckets {
                        let first = frame.first_row + bucket as u64 * width;
                        let end = (first + width).min(frame.first_row + frame.rows);
                        if end <= r0 || first >= r1 {
                            continue;
                        }
                        let (lo, hi) = (xs[2 * bucket], xs[2 * bucket + 1]);
                        if first < r0
                            || end > r1
                            || view.unit(lo, hi, values[2 * bucket], values[2 * bucket + 1])
                                == Fit::Refine
                        {
                            self.units(level - 1, first.max(r0), end.min(r1), column, view, cache)?;
                        }
                    }
                }
                if covered < r1 {
                    self.units(level - 1, r0.max(covered), r1, column, view, cache)?;
                }
            }
        }
        Ok(())
    }
}
