//! A segment read through one shared reader: exact ranges, tiles through the cache, the rows
//! around a cursor, and the cache's budget.

use super::*;
use rspice_results_store::{
    Axis, MemoryIo, NoBlockCache, RunStatus, SegmentWriter, Signal, SignalKind,
};

/// A segment of `rows` rows of x = row / 1000 and two real signals.
fn segment(rows: usize) -> (Vec<f64>, Vec<f64>, StoreReader) {
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![
            Signal::new("v(a)", "V", SignalKind::Real),
            Signal::new("v(b)", "V", SignalKind::Real),
        ],
    };
    let x: Vec<f64> = (0..rows).map(|row| row as f64 / 1000.0).collect();
    let values: Vec<f64> = x
        .iter()
        .flat_map(|&t| [(t * 7.0).sin(), (t * 3.0).cos() * 2.0])
        .collect();
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &RunMeta::default()).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let reader = StoreReader::open(Box::new(io)).unwrap();
    (x, values, reader)
}

#[test]
fn a_shared_reader_reads_exact_ranges_tiles_and_the_rows_around_a_cursor() {
    let (x, values, reader) = segment(70_000);
    assert_eq!(reader.rows(), 70_000);
    assert!(reader.digest().is_some());

    let mut out = vec![0.0; 9_000];
    reader.read(Some(1), 4_000, &mut out).unwrap();
    let expected: Vec<f64> = (4_000..13_000).map(|row| values[2 * row + 1]).collect();
    assert_eq!(out, expected);
    reader.read(None, 69_990, &mut out[..10]).unwrap();
    assert_eq!(&out[..10], &x[69_990..]);
    assert!(reader.read(None, 69_995, &mut out[..10]).is_err());

    // Slot 1 begins signal 1; through the cache, as without it, and again from the cache.
    let plain = reader
        .with_reader(|segment| segment.tile_in(1, 2_000..60_000, 3.0, 50.0, 300, &mut NoBlockCache))
        .unwrap();
    let cached = reader.tile(1, 2_000..60_000, 3.0, 50.0, 300).unwrap();
    assert_eq!(cached, plain);
    assert_eq!(
        reader.tile(1, 2_000..60_000, 3.0, 50.0, 300).unwrap(),
        plain
    );
    assert!(matches!(plain, Tile::Envelope { .. }));

    let at = |cursor: f64| reader.rows_around(0, 1_000..2_000, cursor).unwrap();
    let rows = |rows: Range<usize>| {
        (
            rows.start as u64,
            rows.map(|row| (x[row], values[2 * row]))
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(at(1.5), rows(1_499..1_503));
    assert_eq!(at(1.5004), rows(1_499..1_503));
    assert_eq!(at(0.2), rows(1_000..1_002));
    assert_eq!(at(9.0), rows(1_998..2_000));
    assert!(reader.rows_around(4, 0..10, 0.0).is_err());

    reader.close();
    assert!(reader.read(None, 0, &mut out[..1]).is_err());
}

#[test]
fn the_tile_cache_keeps_its_budget_by_dropping_the_least_recently_used_block() {
    let key = |first_row: u64| {
        (
            7,
            BlockKey {
                column: Some(0),
                level: 2,
                first_row,
            },
        )
    };
    let block = |value: f64| -> Arc<[f64]> { vec![value; 16].into() };
    let mut cache = TileCache::new(3 * 16 * 8);
    cache.put(key(0), block(0.0));
    cache.put(key(1), block(1.0));
    cache.put(key(2), block(2.0));
    assert_eq!(cache.bytes(), 3 * 16 * 8);
    // Using block 0 makes block 1 the oldest.
    assert!(cache.get(&key(0)).is_some());
    cache.put(key(3), block(3.0));
    assert_eq!(cache.bytes(), 3 * 16 * 8);
    assert!(cache.get(&key(1)).is_none());
    for kept in [0, 2, 3] {
        assert_eq!(cache.get(&key(kept)).unwrap()[0], kept as f64);
    }
    // A block larger than the whole budget is not kept, and keeps what is.
    cache.put(key(9), vec![0.0; 64].into());
    assert!(cache.get(&key(9)).is_none());
    assert_eq!(cache.bytes(), 3 * 16 * 8);
    cache.forget(7);
    assert_eq!(cache.bytes(), 0);
    assert!(cache.get(&key(0)).is_none());
}
