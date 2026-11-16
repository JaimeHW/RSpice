//! The store as an engine result sink: what the engine hands it is what the segment reads back.

use rspice_core_base::result_sink::{
    self as sink, ResultAxis, ResultSchema, ResultSignal, ResultSignalKind, ResultSink, SinkStop,
};

use crate::{
    EventChange, EventValue, Logic, MemoryIo, RunStatus, SegmentReader, SegmentState, StoreError,
    StoreSink, sink_stop,
};

fn schema() -> ResultSchema {
    ResultSchema {
        x: ResultAxis {
            name: "time".into(),
            unit: "s".into(),
        },
        signals: vec![
            ResultSignal::new("out", "V", ResultSignalKind::Real),
            ResultSignal::new("d", "", ResultSignalKind::Logic),
            ResultSignal::new("i(v1)", "A", ResultSignalKind::Real),
        ],
    }
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn rows_events_and_an_amended_row_zero_read_back_from_the_segment() {
    let mut store = StoreSink::new(MemoryIo::new());
    let sink: &mut dyn ResultSink = &mut store;
    let meta = sink::RunMeta {
        analysis: "tran".into(),
        ..sink::RunMeta::default()
    };
    sink.begin(&schema(), &meta).unwrap();
    let rows = 3000;
    let x: Vec<f64> = (0..rows).map(|row| row as f64 * 1e-9).collect();
    let mut values = Vec::with_capacity(rows * 2);
    for row in 0..rows {
        let value = match row % 7 {
            3 => f64::NAN,
            5 => -0.0,
            _ => (row as f64 * 0.01).sin(),
        };
        values.extend([value, -(row as f64)]);
    }
    sink.row(x[0], &values[..2]).unwrap();
    sink.amend_open_row(0, 1, 2.5e-3).unwrap();
    values[1] = 2.5e-3;
    for row in 1..rows {
        sink.row(x[row], &values[row * 2..row * 2 + 2]).unwrap();
    }
    sink.events(&[
        sink::EventChange {
            signal: 1,
            time: 0.0,
            sequence: 0,
            value: sink::EventValue::Logic(sink::Logic::Zero),
            strength: 6,
        },
        sink::EventChange {
            signal: 1,
            time: 1.5e-9,
            sequence: 0,
            value: sink::EventValue::Logic(sink::Logic::One),
            strength: 6,
        },
    ])
    .unwrap();
    let receipt = sink.end(sink::RunStatus::Completed).unwrap();
    assert_eq!((receipt.rows, receipt.events), (rows as u64, 2));
    let store_receipt = store.receipt().cloned().unwrap();
    assert_eq!(receipt.digest, Some(store_receipt.digest.0));

    let mut reader = SegmentReader::open(store.into_io().unwrap()).unwrap();
    assert_eq!(
        reader.state(),
        &SegmentState::Complete {
            status: RunStatus::Completed,
            digest: store_receipt.digest,
        }
    );
    assert_eq!(reader.run_meta().analysis, "tran");
    assert_eq!(bits(&reader.read_x(0, rows).unwrap()), bits(&x));
    for slot in 0..2 {
        let column: Vec<f64> = values.iter().skip(slot).step_by(2).copied().collect();
        assert_eq!(
            bits(&reader.read_slot(slot, 0, rows).unwrap()),
            bits(&column)
        );
    }
    assert_eq!(
        reader.events(1, f64::NEG_INFINITY, f64::INFINITY).unwrap(),
        vec![
            EventChange {
                signal: 1,
                time: 0.0,
                sequence: 0,
                value: EventValue::Logic(Logic::Zero),
                strength: 6,
            },
            EventChange {
                signal: 1,
                time: 1.5e-9,
                sequence: 0,
                value: EventValue::Logic(Logic::One),
                strength: 6,
            },
        ]
    );
}

#[test]
fn a_run_that_never_began_writes_nothing_refuses_to_end_and_keeps_its_io() {
    let mut store = StoreSink::new(MemoryIo::new());
    assert!(matches!(store.row(0.0, &[1.0]), Err(SinkStop::Io(_))));
    assert!(matches!(
        store.end(sink::RunStatus::Completed),
        Err(SinkStop::Io(_))
    ));
    assert!(store.into_io().unwrap().bytes().is_empty());
}

#[test]
fn storage_and_memory_exhaustion_stay_typed_as_sink_stops() {
    assert_eq!(
        sink_stop(StoreError::DiskFull),
        SinkStop::DiskFull {
            needed: None,
            available: None
        }
    );
    assert_eq!(
        sink_stop(StoreError::MemoryFull),
        SinkStop::MemoryFull { bytes: None }
    );
    assert_eq!(
        sink_stop(StoreError::RowWidth {
            expected: 2,
            found: 1
        }),
        SinkStop::Io("a result row carries 1 values; the schema has 2 value slots".into())
    );
}

/// A resumed run appended to the segment of the run it continues: the run repeats its start row
/// and the changes there, which the segment already holds, and the segment it finishes is the
/// uninterrupted run's.
#[test]
fn a_resumed_run_appends_to_the_segment_it_continues() {
    let meta = sink::RunMeta::default();
    let x: Vec<f64> = (0..2000).map(|row| row as f64 * 1e-9).collect();
    let row = |at: usize| [(at as f64 * 0.01).sin(), at as f64];
    let change = |at: usize| sink::EventChange {
        signal: 1,
        time: x[at],
        sequence: 0,
        value: sink::EventValue::Logic(if at.is_multiple_of(2) {
            sink::Logic::One
        } else {
            sink::Logic::Zero
        }),
        strength: 0,
    };
    let run = |sink: &mut dyn ResultSink, rows: std::ops::Range<usize>, begin: bool| {
        if begin {
            sink.begin(&schema(), &meta).unwrap();
        }
        for at in rows {
            sink.row(x[at], &row(at)).unwrap();
            if at.is_multiple_of(37) {
                sink.events(&[change(at)]).unwrap();
            }
        }
        sink.end(sink::RunStatus::Completed).unwrap()
    };
    let mut whole = StoreSink::new(MemoryIo::new());
    let whole_receipt = run(&mut whole, 0..x.len(), true);
    let mut first = StoreSink::new(MemoryIo::new());
    run(&mut first, 0..1500, true);
    let through = 1110;
    let mut resumed = StoreSink::appending(first.into_io().unwrap(), x[through]).unwrap();
    let receipt = run(&mut resumed, through..x.len(), true);
    assert_eq!(receipt, whole_receipt);
    assert!(resumed.into_io().unwrap().bytes() == whole.into_io().unwrap().bytes());

    // A resumed run that records other signals is refused before its first row.
    let mut first = StoreSink::new(MemoryIo::new());
    run(&mut first, 0..1500, true);
    let mut other = StoreSink::appending(first.into_io().unwrap(), x[through]).unwrap();
    let mut signals = schema();
    signals.signals.pop();
    assert!(matches!(
        ResultSink::begin(&mut other, &signals, &meta),
        Err(SinkStop::Io(_))
    ));
    assert!(matches!(
        StoreSink::appending(MemoryIo::new(), 0.0),
        Err(StoreError::Corrupt { .. })
    ));
}

/// A segment in a file reopens writable for the run that resumes it, and the file it finishes is
/// the uninterrupted run's.
#[cfg(any(unix, windows))]
#[test]
fn a_segment_file_reopens_writable_for_the_run_that_resumes_it() {
    use crate::FileIo;
    let meta = sink::RunMeta::default();
    let x: Vec<f64> = (0..1200).map(|row| row as f64 * 1e-9).collect();
    let run = |sink: &mut dyn ResultSink, rows: std::ops::Range<usize>| {
        sink.begin(&schema(), &meta).unwrap();
        for at in rows {
            sink.row(x[at], &[(at as f64 * 0.01).cos(), at as f64])
                .unwrap();
        }
        sink.end(sink::RunStatus::Completed).unwrap()
    };
    let dir = std::env::temp_dir().join(format!(
        "rspice-store-reopen-writable-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("000000.rsr");
    let mut whole = StoreSink::new(MemoryIo::new());
    let whole_receipt = run(&mut whole, 0..x.len());
    let mut first = StoreSink::new(FileIo::create(&path).unwrap());
    run(&mut first, 0..700);
    drop(first);
    // A segment opened only for reading cannot take the resumed rows.
    assert!(StoreSink::appending(FileIo::open(&path).unwrap(), x[699]).is_err());
    let mut resumed = StoreSink::appending(FileIo::open_writable(&path).unwrap(), x[699]).unwrap();
    assert_eq!(run(&mut resumed, 699..x.len()), whole_receipt);
    drop(resumed);
    assert!(std::fs::read(&path).unwrap() == whole.into_io().unwrap().bytes());
    let _ = std::fs::remove_dir_all(&dir);
}
