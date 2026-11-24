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

/// Threads that read one segment file at once each get what one thread reading it alone gets:
/// exact ranges, tiles through the shared cache and the rows around a cursor, of every signal.
#[cfg(any(unix, windows))]
#[test]
fn threads_reading_one_segment_at_once_read_what_one_thread_reads() {
    use rspice_results_store::FileIo;

    const SIGNALS: usize = 48;
    const ROWS: usize = 20_000;
    let root = std::env::temp_dir().join(format!("rspice-results-threads-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: (0..SIGNALS)
            .map(|signal| Signal::new(format!("v({signal})"), "V", SignalKind::Real))
            .collect(),
    };
    let x: Vec<f64> = (0..ROWS).map(|row| row as f64 / 1000.0).collect();
    let values: Vec<f64> = x
        .iter()
        .flat_map(|&t| (0..SIGNALS).map(move |signal| (t * (signal + 1) as f64).sin()))
        .collect();
    let path = root.join("transient-0.rsr");
    let mut writer =
        SegmentWriter::create(FileIo::create(&path).unwrap(), &schema, &RunMeta::default())
            .unwrap();
    writer.push_rows(&x, &values).unwrap();
    writer.finish(RunStatus::Completed).unwrap();

    // Every read of every signal, as one thread answers it from a reader of its own.
    type Answer = (Vec<f64>, Tile, Tile, (u64, Vec<(f64, f64)>));
    let read_all = |reader: &StoreReader, signal: usize| -> Answer {
        let mut range = vec![0.0; 3_000];
        reader
            .read(Some(signal), (signal * 300) as u64, &mut range)
            .unwrap();
        let whole = reader.tile(signal, 0..ROWS as u64, 0.0, 20.0, 400).unwrap();
        let narrow = reader.tile(signal, 1_000..9_000, 2.5, 2.9, 300).unwrap();
        let around = reader
            .rows_around(signal, 0..ROWS as u64, signal as f64 * 0.37)
            .unwrap();
        (range, whole, narrow, around)
    };
    let alone = StoreReader::open_file(&path).unwrap();
    let expected: Vec<Answer> = (0..SIGNALS)
        .map(|signal| read_all(&alone, signal))
        .collect();
    for (signal, (range, ..)) in expected.iter().enumerate() {
        let first = signal * 300;
        let wanted: Vec<f64> = (first..first + 3_000)
            .map(|row| values[row * SIGNALS + signal])
            .collect();
        assert_eq!(*range, wanted, "signal {signal} reads back as written");
    }

    // A second reader, so every thread misses the cache at first and fills it beside the others.
    let shared = StoreReader::open_file(&path).unwrap();
    let threads = 8;
    let answers: Vec<Vec<(usize, Answer)>> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|thread| {
                let shared = &shared;
                let read_all = &read_all;
                scope.spawn(move || {
                    // Each thread walks the signals from a place of its own.
                    (0..SIGNALS)
                        .map(|step| {
                            let signal = (step + thread * 7) % SIGNALS;
                            (signal, read_all(shared, signal))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    });
    for (thread, answers) in answers.iter().enumerate() {
        for (signal, answer) in answers {
            assert_eq!(
                *answer, expected[*signal],
                "thread {thread} read signal {signal} as one thread does"
            );
        }
    }
    alone.close();
    shared.close();
    let _ = std::fs::remove_dir_all(&root);
}

/// Two open segments of one digest (two runs that solved the same rows) each serve a reader of
/// it: the one opened second closing leaves the first found by its digest.
#[cfg(any(unix, windows))]
#[test]
fn a_digest_two_open_segments_share_is_found_while_either_is_open() {
    let open = || Segment::kept(segment(1_234).2, std::path::PathBuf::new()).shared();
    let first = open();
    let digest = first
        .reader()
        .digest()
        .expect("a finished segment states its digest");
    let second = open();
    assert!(second.reader().digest() == Some(digest));
    drop(second);
    let found = kept_segment(digest).expect("the first segment is still open");
    assert!(Arc::ptr_eq(&found, &first));
}

/// A saved result names a stored run's waveform and event history by its segment's digest and
/// reads them back from the folder that keeps the segment, the same values and the same
/// identity. A segment no folder holds reads as unavailable and names its file, keeping the
/// identity the result was saved with, so the run's views stay bound to it.
#[cfg(any(unix, windows))]
#[test]
fn a_saved_reference_reads_its_kept_segment_and_a_missing_one_names_its_file() {
    use crate::events::RealEventPointEvidence;
    use crate::events::points::EventPoints;
    use crate::waveform::{StoredWindow, WaveformValues, WindowLead};
    use rspice_results_store::{EventChange, EventValue, FileIo, PlaceWays, place_segment};

    let root =
        std::env::temp_dir().join(format!("rspice-results-references-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![
            Signal::new("v(a)", "V", SignalKind::Real),
            Signal::new("e(ctl)", "V", SignalKind::RealEvent),
        ],
    };
    let x: Vec<f64> = (0..1_000).map(|row| row as f64 / 1000.0).collect();
    let values: Vec<f64> = x.iter().map(|&t| (t * 7.0).sin()).collect();
    let run_path = root.join("transient-0.rsr");
    let mut writer = SegmentWriter::create(
        FileIo::create(&run_path).unwrap(),
        &schema,
        &RunMeta::default(),
    )
    .unwrap();
    writer.push_rows(&x, &values).unwrap();
    let changes: Vec<EventChange> = (0..10)
        .map(|i| EventChange {
            signal: 1,
            time: i as f64 * 0.1,
            sequence: i,
            value: EventValue::Real(f64::from(i) * 0.5),
            strength: 0,
        })
        .collect();
    writer.push_events(&changes).unwrap();
    let (receipt, _) = writer.finish(RunStatus::Completed).unwrap();

    let segment = Arc::new(Segment::temporary(
        StoreReader::open_file(&run_path).unwrap(),
        run_path,
    ));
    let window = StoredWindow {
        first: 100,
        rows: 800,
        lead: WindowLead::Value(0.0995),
    };
    let stored = WaveformValues::stored(
        SegmentColumn::new(segment.clone(), Some(0), "v(a)".to_owned()),
        window,
    )
    .unwrap();
    let history: EventPoints<RealEventPointEvidence> = EventPoints::stored(
        SegmentEvents::new(segment.clone(), 1, "e(ctl)".to_owned()),
        0.25,
    )
    .unwrap();
    // The whole column stored has the identity the same column held in memory has.
    let whole = WaveformValues::stored(
        SegmentColumn::new(segment.clone(), Some(0), "v(a)".to_owned()),
        StoredWindow {
            first: 0,
            rows: segment.rows(),
            lead: WindowLead::None,
        },
    )
    .unwrap();
    let held = WaveformValues::from(whole.samples(..).collect::<Vec<_>>());
    assert_eq!(whole.samples_identity(), held.samples_identity());
    let column = ColumnReference::of(&stored).unwrap();
    assert_eq!(column.segment, receipt.digest.to_string());
    assert!(
        column.column_digest.is_some(),
        "a complete segment states its columns' digests"
    );
    let written = writing_references(|| serde_json::to_string(&history).unwrap());
    assert!(written.contains(&receipt.digest.to_string()), "{written}");
    assert_eq!(
        serde_json::to_string(&history)
            .unwrap()
            .matches("time_s")
            .count(),
        7,
        "outside a save, a history is written as its points"
    );

    let folder = root.join("results").join("project");
    place_segment(
        segment.path(),
        &folder,
        &receipt.digest,
        PlaceWays::ALL,
        &std::sync::atomic::AtomicBool::new(false),
        &mut |_| {},
    )
    .unwrap();
    keep_segments_in(&folder);
    let read_back = column.values().unwrap();
    assert_eq!(read_back, stored);
    assert_eq!(
        read_back.stored_window().unwrap().0.identity(),
        stored.stored_window().unwrap().0.identity()
    );
    let events: EventPoints<RealEventPointEvidence> = serde_json::from_str(&written).unwrap();
    assert_eq!(events.len(), 7);
    assert_eq!(*events, *history);
    assert_eq!(events.stored_identity(), history.stored_identity());

    drop((read_back, events));
    let gone = "0".repeat(64);
    let missing = ColumnReference {
        segment: gone.clone(),
        ..column.clone()
    }
    .values()
    .unwrap();
    assert_eq!(missing.len(), stored.len());
    let failure = missing
        .read_range(0..missing.len())
        .expect_err("a missing segment reads nothing");
    assert!(
        failure.to_string().contains(&format!("{gone}.rsr")),
        "{failure}"
    );
    assert!(
        failure
            .to_string()
            .contains("deleted or moved outside RSpice"),
        "{failure}"
    );
    assert_eq!(
        missing.stored_window().unwrap().0.identity(),
        Some(([0; 32], column.column))
    );
    // A result whose segment went missing keeps the identity its samples had.
    assert_eq!(missing.samples_identity(), stored.samples_identity());
    std::fs::remove_dir_all(&root).unwrap();
}

/// A file under a kept segment's name is the segment only when its stored bits hash to the
/// digest: a damaged one reads as missing, never as the run's numbers, and the intact file put
/// back is found again.
#[test]
fn a_kept_file_that_does_not_hold_its_digest_reads_as_missing() {
    use std::time::{Duration, SystemTime};

    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![Signal::new("v(kept)", "V", SignalKind::Real)],
    };
    let x: Vec<f64> = (0..3_000).map(|row| row as f64 * 1e-6 + 0.25).collect();
    let values: Vec<f64> = x.iter().map(|&t| (t * 11.0).sin()).collect();
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &RunMeta::default()).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    let bytes = io.into_bytes();

    let folder =
        std::env::temp_dir().join(format!("rspice-results-damaged-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join(kept_segment_name(&receipt.digest));
    let put = |bytes: &[u8], seconds: u64| {
        std::fs::write(&path, bytes).unwrap();
        std::fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000 + seconds))
            .unwrap();
    };
    let mut damaged = bytes.clone();
    damaged[bytes.len() / 2] ^= 0x01;
    put(&damaged, 0);
    assert!(rspice_results_store::verify_segment(&path, &receipt.digest).is_err());
    keep_segments_in(&folder);
    assert!(
        kept_segment(receipt.digest).is_none(),
        "a damaged file is not the segment"
    );

    put(&bytes, 1);
    let segment = kept_segment(receipt.digest).expect("the intact file is the segment");
    assert_eq!(segment.path(), path.as_path());
    drop(segment);
    std::fs::remove_dir_all(&folder).unwrap();
}

/// A segment's bytes that are not here yet until `arrived`: a read before then fails as a
/// browser page's read of a segment its worker holds does while its bytes are on their way.
struct Arriving {
    bytes: MemoryIo,
    arrived: Arc<std::sync::atomic::AtomicBool>,
}

impl rspice_results_store::SegmentIo for Arriving {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        if !self.arrived.load(std::sync::atomic::Ordering::Acquire) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                "on their way",
            ));
        }
        self.bytes.read_at(offset, buf)
    }
    fn write_at(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        self.bytes.write_at(offset, data)
    }
    fn truncate(&mut self, len: u64) -> std::io::Result<()> {
        self.bytes.truncate(len)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.bytes.flush()
    }
    fn size(&self) -> std::io::Result<u64> {
        self.bytes.size()
    }
}

/// A window whose rows are on their way reads as pending, keeps no failure and reports none, and
/// reads its samples once they are here: a pending read never leaves a waveform unavailable.
#[test]
fn a_window_whose_rows_are_on_their_way_reads_them_once_they_arrive() {
    use crate::waveform::{
        PENDING_READ, StoredWindow, WaveformValues, WindowLead, take_column_read_failures,
    };
    let (x, values, reader) = segment(5_000);
    drop(reader);
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
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &RunMeta::default()).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (_, written) = writer.finish(RunStatus::Completed).unwrap();
    let bytes = written.into_bytes();
    let arrived = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let io = Arriving {
        bytes: MemoryIo::from_bytes(bytes),
        arrived: Arc::clone(&arrived),
    };
    let reader = StoreReader::open(Box::new(io)).unwrap();
    let segment = Arc::new(Segment::kept(
        reader,
        std::path::PathBuf::from("arriving.rseg"),
    ));
    arrived.store(false, std::sync::atomic::Ordering::Release);
    let window = |slot: Option<usize>, lead: WindowLead| {
        WaveformValues::stored(
            SegmentColumn::new(Arc::clone(&segment), slot, format!("arriving {slot:?}")),
            StoredWindow {
                first: 100,
                rows: 1_000,
                lead,
            },
        )
        .unwrap()
    };
    let axis = window(None, WindowLead::None);
    let wave = window(Some(1), WindowLead::Interpolated(0.5));
    let _ = take_column_read_failures();

    let failure = wave
        .read_range(0..10)
        .expect_err("its rows are on their way");
    assert_eq!(failure.cause, PENDING_READ);
    assert!(wave.sample(3).is_some_and(f64::is_nan));
    assert_eq!(axis.ends(), None);
    assert!(
        !take_column_read_failures()
            .iter()
            .any(|failure| failure.column.starts_with("arriving"))
    );

    arrived.store(true, std::sync::atomic::Ordering::Release);
    let lead = values[199] + 0.5 * (values[201] - values[199]);
    let read = wave.read_range(0..4).expect("its rows are here");
    assert_eq!(read[..], [lead, values[201], values[203], values[205]]);
    assert_eq!(axis.ends(), Some((x[100], x[1_099])));
    assert_eq!(wave.sample(3), Some(values[205]));
    assert!(
        !take_column_read_failures()
            .iter()
            .any(|failure| failure.column.starts_with("arriving"))
    );
}

/// Each column a saved result names is named for the signal whose first value slot it reads:
/// past an event signal, which takes no slot, and never for a complex signal's second slot.
#[test]
fn a_segments_columns_are_named_by_the_signal_each_slot_starts() {
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![
            Signal::new("a", "V", SignalKind::Real),
            Signal::new("d", "", SignalKind::Logic),
            Signal::new("z", "V", SignalKind::Complex),
            Signal::new("b", "A", SignalKind::Real),
        ],
    };
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &RunMeta::default()).unwrap();
    writer.push_rows(&[0.0, 1.0], &[0.0; 8]).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let segment = Segment::kept(
        StoreReader::open(Box::new(io)).unwrap(),
        std::path::PathBuf::new(),
    );
    let names = (0..5)
        .map(|slot| segment.slot_name(slot))
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            Some("a".to_owned()),
            Some("z".to_owned()),
            None,
            Some("b".to_owned()),
            None
        ]
    );
}
