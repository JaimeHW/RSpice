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

impl<I: SegmentIo> SegmentReader<I> {
    /// Per-pixel min/max (or the rows themselves) of analog signal `signal` over `x0 ..= x1`.
    pub fn tile(
        &mut self,
        signal: usize,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Result<Tile, StoreError> {
        if pixels == 0 || !(x0.is_finite() && x1.is_finite() && x0 <= x1) {
            return Err(StoreError::OutOfRange(format!(
                "a {pixels}-pixel view over [{x0}, {x1}]"
            )));
        }
        let slot = self.schema().slot_of(signal).ok_or_else(|| {
            StoreError::OutOfRange(format!("signal {signal} is not an analog signal"))
        })?;
        let index = self
            .analog()
            .iter()
            .position(|column| column.slot == slot)
            .unwrap_or_default();
        let column = Column {
            index,
            slot,
            complex: self.analog()[index].complex,
        };

        let in_view = |chunk: &ChunkRef| chunk.x_max >= x0 && chunk.x_min <= x1;
        let estimate: u64 = self
            .chunks()
            .iter()
            .filter(|chunk| in_view(chunk))
            .map(|chunk| u64::from(chunk.rows))
            .sum();
        let wanted = 2 * pixels as u64;
        let level = (1..=self.top_level())
            .rev()
            .find(|&level| estimate >= wanted * bucket_rows(level))
            .unwrap_or(0);
        if level == 0 {
            return self.raw_tile(column, x0, x1, pixels);
        }
        let mut view = Pixels::new(x0, x1, pixels);
        self.units(level, 0, self.rows(), column, &mut view)?;
        Ok(Tile::Envelope {
            level,
            min: view.min,
            max: view.max,
            touched: view.touched,
        })
    }

    /// x and the drawn value (|z| for a complex signal) of every row of `chunk`.
    fn decode_drawn(
        &mut self,
        chunk: &ChunkRef,
        column: Column,
        x: &mut Vec<f64>,
        y: &mut Vec<f64>,
    ) -> Result<(), StoreError> {
        let rows = chunk.rows as usize;
        x.resize(rows, 0.0);
        y.resize(rows, 0.0);
        self.decode_block(chunk, 0, x)?;
        self.decode_block(chunk, column.slot + 1, y)?;
        if column.complex {
            let mut im = vec![0.0; rows];
            self.decode_block(chunk, column.slot + 2, &mut im)?;
            for (re, im) in y.iter_mut().zip(&im) {
                *re = magnitude(*re, *im);
            }
        }
        Ok(())
    }

    /// The row at `row`: (x, drawn value).
    fn drawn_row(&mut self, row: u64, column: Column) -> Result<(f64, f64), StoreError> {
        let chunk = self.chunks()[self.chunk_containing(row)];
        let (mut x, mut y) = (Vec::new(), Vec::new());
        self.decode_drawn(&chunk, column, &mut x, &mut y)?;
        let at = (row - chunk.first_row) as usize;
        Ok((x[at], y[at]))
    }

    /// Few enough rows to decode: exact samples, or an exact per-row envelope.
    fn raw_tile(
        &mut self,
        column: Column,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Result<Tile, StoreError> {
        let (mut xs, mut ys, mut rows) = (Vec::new(), Vec::new(), Vec::new());
        let (mut x, mut y) = (Vec::new(), Vec::new());
        let mut last_before = None;
        for index in 0..self.chunks().len() {
            let chunk = self.chunks()[index];
            if chunk.x_max < x0 {
                last_before = Some(chunk.end_row() - 1);
                continue;
            }
            if chunk.x_min > x1 {
                continue;
            }
            self.decode_drawn(&chunk, column, &mut x, &mut y)?;
            for (at, (&x, &y)) in x.iter().zip(&y).enumerate() {
                if x >= x0 && x <= x1 {
                    xs.push(x);
                    ys.push(y);
                    rows.push(chunk.first_row + at as u64);
                } else if x < x0 {
                    last_before = Some(chunk.first_row + at as u64);
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
        let before_row = match rows.first() {
            Some(&first) => first.checked_sub(1),
            None => last_before,
        };
        let after_row = match rows.last() {
            Some(&last) => Some(last + 1),
            None => last_before.map_or(Some(0), |row| Some(row + 1)),
        }
        .filter(|&row| row < self.rows());
        let before = before_row
            .map(|row| self.drawn_row(row, column))
            .transpose()?;
        let after = after_row
            .map(|row| self.drawn_row(row, column))
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
    ) -> Result<(), StoreError> {
        if r0 >= r1 {
            return Ok(());
        }
        match level {
            0 => {
                let (mut x, mut y) = (Vec::new(), Vec::new());
                let mut index = self.chunk_containing(r0);
                while index < self.chunks().len() && self.chunks()[index].first_row < r1 {
                    let chunk = self.chunks()[index];
                    index += 1;
                    if !(chunk.x_max >= view.x0 && chunk.x_min <= view.x1) {
                        continue;
                    }
                    self.decode_drawn(&chunk, column, &mut x, &mut y)?;
                    let from = r0.max(chunk.first_row) - chunk.first_row;
                    let to = r1.min(chunk.end_row()) - chunk.first_row;
                    for at in from as usize..to as usize {
                        view.unit(x[at], x[at], y[at], y[at]);
                    }
                }
            }
            1 => {
                let (mut xs, mut values) = (Vec::new(), Vec::new());
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
                    self.read_f64s(at, 2 * buckets, &mut xs)?;
                    self.read_f64s(
                        at + (16 * buckets * (1 + column.index)) as u64,
                        2 * buckets,
                        &mut values,
                    )?;
                    for bucket in 0..buckets {
                        let first = chunk.first_row + (bucket * LEVEL1_ROWS) as u64;
                        let end = (first + LEVEL1_ROWS as u64).min(chunk.end_row());
                        if end <= r0 || first >= r1 {
                            continue;
                        }
                        let (lo, hi) = (xs[2 * bucket], xs[2 * bucket + 1]);
                        if view.unit(lo, hi, values[2 * bucket], values[2 * bucket + 1])
                            == Fit::Refine
                        {
                            self.units(0, first.max(r0), end.min(r1), column, view)?;
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
                let (mut xs, mut values) = (Vec::new(), Vec::new());
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
                    self.read_f64s(at, 2 * buckets, &mut xs)?;
                    self.read_f64s(
                        at + (16 * buckets * (1 + column.index)) as u64,
                        2 * buckets,
                        &mut values,
                    )?;
                    for bucket in 0..buckets {
                        let first = frame.first_row + bucket as u64 * width;
                        let end = (first + width).min(frame.first_row + frame.rows);
                        if end <= r0 || first >= r1 {
                            continue;
                        }
                        let (lo, hi) = (xs[2 * bucket], xs[2 * bucket + 1]);
                        if view.unit(lo, hi, values[2 * bucket], values[2 * bucket + 1])
                            == Fit::Refine
                        {
                            self.units(level - 1, first.max(r0), end.min(r1), column, view)?;
                        }
                    }
                }
                if covered < r1 {
                    self.units(level - 1, r0.max(covered), r1, column, view)?;
                }
            }
        }
        Ok(())
    }
}
