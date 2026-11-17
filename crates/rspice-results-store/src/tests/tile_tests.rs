//! Tiles against a brute-force pass over the raw rows.

use super::support::{Rng, meta, mixed_schema};
use crate::format::{bucket_rows, fold_max, fold_min};
use crate::writer::magnitude;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use crate::{
    BlockCache, BlockKey, MemoryIo, NoBlockCache, RunStatus, SegmentReader, SegmentWriter, Tile,
    pixel_of,
};

/// Rows with uneven steps (including jumps hundreds of steps wide, where units span many pixels),
/// stretches of NaN, and a complex signal.
fn rows(count: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let mut rng = Rng(seed);
    let (mut x, mut values) = (Vec::with_capacity(count), Vec::with_capacity(count * 4));
    let mut t = 0.0;
    for row in 0..count {
        t += if rng.below(5000) == 0 {
            400.0 * rng.unit()
        } else {
            0.2 + rng.unit()
        };
        x.push(t);
        let gap = (row / 500) % 23 == 7;
        for slot in 0..4 {
            let value = if (gap && slot != 3) || rng.below(3000) == 0 {
                f64::NAN
            } else {
                (t * 0.01 * (slot + 1) as f64).sin() * 10.0 + rng.unit() - 0.5
            };
            values.push(value);
        }
    }
    (x, values)
}

/// The drawn value of `signal` (0, 1 real; 2 complex) in each row.
fn drawn(values: &[f64], signal: usize) -> Vec<f64> {
    values
        .chunks_exact(4)
        .map(|row| match signal {
            2 => magnitude(row[2], row[3]),
            _ => row[signal],
        })
        .collect()
}

struct Brute<'a> {
    x: &'a [f64],
    y: &'a [f64],
    x0: f64,
    x1: f64,
    min: Vec<f64>,
    max: Vec<f64>,
    touched: Vec<bool>,
}

impl Brute<'_> {
    /// The tile's definition, from the raw rows: units of `level`, refined below where they span
    /// three or more pixels or lie only partly in `r0 .. r1`.
    fn visit(&mut self, level: u8, r0: usize, r1: usize) {
        let width = if level == 0 {
            1
        } else {
            bucket_rows(level) as usize
        };
        let mut start = r0;
        while start < r1 {
            let aligned = start / width * width;
            let whole = (aligned + width).min(self.x.len());
            let end = whole.min(r1);
            let partial = start != aligned || end != whole;
            let (mut lo, mut hi, mut min, mut max) = (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
            for row in start..end {
                lo = fold_min(lo, self.x[row]);
                hi = fold_max(hi, self.x[row]);
                min = fold_min(min, self.y[row]);
                max = fold_max(max, self.y[row]);
            }
            if hi >= self.x0 && lo <= self.x1 {
                let n = self.min.len();
                let first = pixel_of(lo.max(self.x0), self.x0, self.x1, n);
                let last = pixel_of(hi.min(self.x1), self.x0, self.x1, n);
                if level > 0 && (partial || last - first >= 2) {
                    self.visit(level - 1, start, end);
                } else {
                    for pixel in first..=last {
                        self.min[pixel] = fold_min(self.min[pixel], min);
                        self.max[pixel] = fold_max(self.max[pixel], max);
                        self.touched[pixel] = true;
                    }
                }
            }
            start = end;
        }
    }
}

fn same(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(a, b)| a == b || (a.is_nan() && b.is_nan()))
}

/// One view a test asks for: the rows it is limited to, the signal, and the pixels over x.
struct Ask {
    rows: Range<usize>,
    signal: usize,
    x0: f64,
    x1: f64,
    pixels: usize,
}

fn check(
    reader: &mut SegmentReader<MemoryIo>,
    x: &[f64],
    values: &[f64],
    cache: &mut dyn BlockCache,
    ask: Ask,
) -> u8 {
    let Ask {
        rows: range,
        signal,
        x0,
        x1,
        pixels,
    } = ask;
    let y = drawn(values, signal);
    let tile = reader
        .tile_in(
            signal,
            range.start as u64..range.end as u64,
            x0,
            x1,
            pixels,
            cache,
        )
        .unwrap();
    match tile {
        Tile::Samples {
            x: xs,
            y: ys,
            before,
            after,
        } => {
            let rows: Vec<usize> = range
                .clone()
                .filter(|&row| x[row] >= x0 && x[row] <= x1)
                .collect();
            assert!(rows.len() <= 2 * pixels);
            assert!(same(
                &xs,
                &rows.iter().map(|&row| x[row]).collect::<Vec<_>>()
            ));
            assert!(same(
                &ys,
                &rows.iter().map(|&row| y[row]).collect::<Vec<_>>()
            ));
            let first_after = rows.last().map_or_else(
                || range.start + x[range.clone()].partition_point(|&v| v <= x1),
                |&row| row + 1,
            );
            let last_before = rows
                .first()
                .map_or_else(
                    || (range.start + x[range.clone()].partition_point(|&v| v < x0)).checked_sub(1),
                    |&row| row.checked_sub(1),
                )
                .filter(|&row| row >= range.start);
            let expect = |row: Option<usize>| {
                row.filter(|&row| row < range.end)
                    .map(|row| (x[row], y[row]))
            };
            assert_eq!(
                before.map(|(a, b)| (a.to_bits(), b.to_bits())),
                expect(last_before).map(|(a, b)| (a.to_bits(), b.to_bits()))
            );
            assert_eq!(
                after.map(|(a, b)| (a.to_bits(), b.to_bits())),
                expect(Some(first_after)).map(|(a, b)| (a.to_bits(), b.to_bits()))
            );
            u8::MAX
        }
        Tile::Envelope {
            level,
            min,
            max,
            touched,
        } => {
            let mut brute = Brute {
                x,
                y: &y,
                x0,
                x1,
                min: vec![f64::NAN; pixels],
                max: vec![f64::NAN; pixels],
                touched: vec![false; pixels],
            };
            brute.visit(level, range.start, range.end);
            assert!(
                same(&min, &brute.min),
                "min, level {level}, [{x0}, {x1}] / {pixels}"
            );
            assert!(
                same(&max, &brute.max),
                "max, level {level}, [{x0}, {x1}] / {pixels}"
            );
            assert_eq!(touched, brute.touched);
            level
        }
    }
}

#[test]
fn tiles_match_a_brute_force_pass_at_every_level_and_at_the_edges() {
    let schema = mixed_schema();
    let (x, values) = rows(300_000, 21);
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &meta()).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let mut reader = SegmentReader::open(io).unwrap();
    let (first, last) = (x[0], *x.last().unwrap());
    let mut levels = std::collections::BTreeSet::new();
    for signal in [0, 1, 2] {
        for (x0, x1) in [
            (first, last),
            (first - 50.0, last + 50.0),
            (x[1000], x[250_000]),
            (x[12_345], x[12_345 + 40_000]),
            (x[77_000] + 0.1, x[77_900] - 0.1),
            (x[5000], x[5010]),
            (x[200], x[200]),
            (last + 1.0, last + 2.0),
            (first - 2.0, first - 1.0),
        ] {
            for pixels in [1, 2, 7, 64, 333, 2000] {
                let ask = Ask {
                    rows: 0..x.len(),
                    signal,
                    x0,
                    x1,
                    pixels,
                };
                levels.insert(check(&mut reader, &x, &values, &mut NoBlockCache, ask));
            }
        }
    }
    // Rows, summaries from chunks, and two pyramid levels were all exercised.
    for level in [0, 1, 2, 3, u8::MAX] {
        assert!(
            levels.contains(&level),
            "level {level} not reached: {levels:?}"
        );
    }
}

#[test]
fn bad_tile_requests_are_refused() {
    let schema = mixed_schema();
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &meta()).unwrap();
    writer.push_row(0.0, &[1.0; 4]).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let mut reader = SegmentReader::open(io).unwrap();
    assert!(reader.tile(0, 0.0, 1.0, 0).is_err());
    assert!(reader.tile(0, 1.0, 0.0, 10).is_err());
    assert!(reader.tile(9, 0.0, 1.0, 10).is_err());
    assert!(reader.tile(0, f64::NAN, 1.0, 10).is_err());
}

/// Keeps every block a view puts, and counts the blocks it had to read.
#[derive(Default)]
struct Kept {
    blocks: HashMap<BlockKey, Arc<[f64]>>,
    reads: usize,
}

impl BlockCache for Kept {
    fn get(&mut self, key: &BlockKey) -> Option<Arc<[f64]>> {
        self.blocks.get(key).cloned()
    }

    fn put(&mut self, key: BlockKey, block: Arc<[f64]>) {
        self.reads += 1;
        self.blocks.insert(key, block);
    }
}

#[test]
fn a_tile_of_some_rows_is_the_brute_force_over_them_and_reads_no_kept_block_again() {
    let schema = mixed_schema();
    let (x, values) = rows(120_000, 33);
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &meta()).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let mut reader = SegmentReader::open(io).unwrap();
    let mut cache = Kept::default();
    let mut levels = std::collections::BTreeSet::new();
    for rows in [
        0..120_000,
        10_000..100_000,
        4_097..4_200,
        119_990..120_000,
        500..500,
    ] {
        for signal in [0, 2] {
            for (x0, x1) in [
                (x[0], x[119_999]),
                (x[3_000], x[60_000]),
                (x[4_100], x[4_150]),
                (x[119_995] - 1.0, x[119_999] + 1.0),
            ] {
                for pixels in [3, 400] {
                    let ask = || Ask {
                        rows: rows.clone(),
                        signal,
                        x0,
                        x1,
                        pixels,
                    };
                    levels.insert(check(&mut reader, &x, &values, &mut cache, ask()));
                    let reads = cache.reads;
                    check(&mut reader, &x, &values, &mut cache, ask());
                    assert_eq!(cache.reads, reads, "a repeated view read a kept block");
                }
            }
        }
    }
    for level in [0, 1, 2, u8::MAX] {
        assert!(
            levels.contains(&level),
            "level {level} not reached: {levels:?}"
        );
    }
}

#[test]
fn the_rows_around_a_cursor_are_two_at_or_before_it_and_two_after() {
    let schema = mixed_schema();
    let (x, values) = rows(20_000, 5);
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &meta()).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let mut reader = SegmentReader::open(io).unwrap();
    let mut rng = Rng(9);
    for (r0, r1) in [(0, 20_000), (4_000, 9_000), (8_191, 8_193), (300, 300)] {
        for signal in [1, 2] {
            let y = drawn(&values, signal);
            let row = |row: usize| (x[row], y[row]);
            let mut cursors = vec![x[0] - 5.0, x[19_999] + 5.0, x[r0.min(19_999)], x[8_192]];
            cursors.extend((0..40).map(|_| x[0] + rng.unit() * (x[19_999] - x[0])));
            for cursor in cursors {
                let found = reader
                    .rows_around(signal, r0 as u64..r1 as u64, cursor, &mut NoBlockCache)
                    .unwrap();
                let below = (r0..r1).rev().find(|&at| x[at] <= cursor);
                let (start, end) = match below {
                    Some(at) => (at.saturating_sub(1).max(r0), (at + 3).min(r1)),
                    None => (r0, (r0 + 2).min(r1)),
                };
                let bits = |rows: &[(f64, f64)]| {
                    rows.iter()
                        .map(|(a, b)| (a.to_bits(), b.to_bits()))
                        .collect::<Vec<_>>()
                };
                let expect: Vec<(f64, f64)> = (start..end).map(row).collect();
                assert_eq!(
                    (found.0, bits(&found.1)),
                    (start as u64, bits(&expect)),
                    "{cursor} in {r0}..{r1}"
                );
            }
        }
    }
    assert!(
        reader
            .rows_around(3, 0..10, 0.0, &mut NoBlockCache)
            .is_err()
    );
}
