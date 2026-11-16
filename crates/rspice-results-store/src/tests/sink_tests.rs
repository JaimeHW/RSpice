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
