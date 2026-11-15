//! The threaded run writer: same bytes as inline, backpressure, flushes, errors and abandonment.

use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::events_tests::{Step, event_schema, run};
use super::support::{meta, mixed_schema, rows};
use crate::{
    EventChange, EventValue, Logic, MemoryIo, RunStatus, RunWriter, SegmentIo, SegmentReader,
    SegmentState, SegmentWriter, StoreError,
};

/// A segment the test can read while the writer thread owns the I/O, optionally slow or with a
/// size past which every write reports a full disk.
#[derive(Clone, Default)]
struct SharedIo {
    bytes: Arc<Mutex<MemoryIo>>,
    delay: Duration,
    limit: Option<u64>,
}

impl SharedIo {
    fn snapshot(&self) -> Vec<u8> {
        self.bytes.lock().unwrap().bytes().to_vec()
    }
}

impl SegmentIo for SharedIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        self.bytes.lock().unwrap().read_at(offset, buf)
    }
    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        if self
            .limit
            .is_some_and(|limit| offset + data.len() as u64 > limit)
        {
            return Err(io::ErrorKind::StorageFull.into());
        }
        std::thread::sleep(self.delay);
        self.bytes.lock().unwrap().write_at(offset, data)
    }
    fn truncate(&mut self, len: u64) -> io::Result<()> {
        self.bytes.lock().unwrap().truncate(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn size(&self) -> io::Result<u64> {
        self.bytes.lock().unwrap().size()
    }
}

fn inline_bytes(steps: &[Step], after: &[EventChange], chunk_rows: usize) -> Vec<u8> {
    let mut writer =
        SegmentWriter::with_chunk_rows(MemoryIo::new(), &event_schema(), &meta(), chunk_rows)
            .unwrap();
    for step in steps {
        writer.push_row(step.x, &step.values).unwrap();
        writer.push_events(&step.changes).unwrap();
    }
    writer.push_events(after).unwrap();
    writer.finish(RunStatus::Completed).unwrap().1.into_bytes()
}

fn threaded<I: crate::RunIo>(
    io: I,
    steps: &[Step],
    after: &[EventChange],
    chunk_rows: usize,
    interval: Duration,
    split: usize,
    sync: usize,
) -> I {
    let mut writer =
        RunWriter::with_options(io, &event_schema(), &meta(), chunk_rows, interval).unwrap();
    for (row, step) in steps.iter().enumerate() {
        writer.push_row(step.x, &step.values).unwrap();
        for part in step.changes.chunks(split) {
            writer.push_events(part).unwrap();
        }
        if row % sync == sync - 1 {
            writer.sync().unwrap();
        }
    }
    for part in after.chunks(split) {
        writer.push_events(part).unwrap();
    }
    assert_eq!(writer.rows(), steps.len() as u64);
    writer.finish(RunStatus::Completed).unwrap().1
}

const NEVER: Duration = Duration::from_secs(3600);

#[test]
fn a_threaded_run_writes_the_bytes_an_inline_one_does() {
    let (steps, after) = run(3000, 5000, 21);
    let inline = inline_bytes(&steps, &after, 64);
    let plain = threaded(
        MemoryIo::new(),
        &steps,
        &after,
        64,
        NEVER,
        usize::MAX,
        usize::MAX,
    );
    assert!(plain.bytes() == inline.as_slice());
    // Explicit flushes and changes pushed one at a time.
    let synced = threaded(MemoryIo::new(), &steps, &after, 64, NEVER, 1, 17);
    assert!(synced.bytes() == inline.as_slice());
    // A flush requested after every push.
    let timed = threaded(
        MemoryIo::new(),
        &steps,
        &after,
        64,
        Duration::ZERO,
        3,
        usize::MAX,
    );
    assert!(timed.bytes() == inline.as_slice());
    // A writer slower than the producer: the producer waits for the free buffer.
    let slow = SharedIo {
        delay: Duration::from_micros(300),
        ..SharedIo::default()
    };
    let slow = threaded(slow, &steps, &after, 64, NEVER, usize::MAX, 50);
    assert!(slow.snapshot() == inline);

    // The default chunk rows, as a run would use them.
    let schema = mixed_schema();
    let (x, values) = rows(20_000, schema.slots(), 22);
    let mut inline = SegmentWriter::create(MemoryIo::new(), &schema, &meta()).unwrap();
    inline.push_rows(&x, &values).unwrap();
    let (expected, inline) = inline.finish(RunStatus::Completed).unwrap();
    let mut writer = RunWriter::create(MemoryIo::new(), &schema, &meta()).unwrap();
    for row in 0..x.len() {
        let slots = schema.slots();
        writer
            .push_row(x[row], &values[row * slots..(row + 1) * slots])
            .unwrap();
    }
    // Row 0 left the open chunk long ago.
    assert_eq!(
        writer.amend_open_row(0, 1, values[1]),
        Err(StoreError::RowNotOpen { row: 0, slot: 1 })
    );
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    assert_eq!(receipt, expected);
    assert!(io.bytes() == inline.bytes());
}

#[test]
fn a_timed_flush_makes_the_open_rows_durable() {
    let io = SharedIo::default();
    let (steps, _) = run(100, 0, 23);
    let mut writer = RunWriter::with_options(
        io.clone(),
        &event_schema(),
        &meta(),
        64,
        Duration::from_millis(20),
    )
    .unwrap();
    for step in &steps[..10] {
        writer.push_row(step.x, &step.values).unwrap();
    }
    std::thread::sleep(Duration::from_millis(60));
    // The first push after the interval hands the open chunk over to be written provisionally.
    writer.push_row(steps[10].x, &steps[10].values).unwrap();
    writer.amend_open_row(3, 0, 0.5).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    let rows = loop {
        let rows = SegmentReader::open(MemoryIo::from_bytes(io.snapshot()))
            .map(|reader| reader.rows())
            .unwrap_or(0);
        if rows >= 11 || Instant::now() > deadline {
            break rows;
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    assert_eq!(rows, 11);
    let (receipt, _) = writer.finish(RunStatus::Completed).unwrap();
    assert_eq!(receipt.rows, 11);
    let mut reader = SegmentReader::open(MemoryIo::from_bytes(io.snapshot())).unwrap();
    assert_eq!(reader.read_slot(0, 3, 1).unwrap(), [0.5]);
}

#[test]
fn a_full_disk_stops_the_writer_and_keeps_a_readable_prefix() {
    let io = SharedIo {
        limit: Some(40_000),
        ..SharedIo::default()
    };
    let (steps, _) = run(5000, 0, 24);
    let mut writer =
        RunWriter::with_options(io.clone(), &event_schema(), &meta(), 64, NEVER).unwrap();
    let mut failed = None;
    for step in &steps {
        let pushed = writer
            .push_row(step.x, &step.values)
            .and_then(|()| writer.push_events(&step.changes));
        if let Err(error) = pushed {
            failed = Some(error);
            break;
        }
    }
    assert_eq!(failed, Some(StoreError::DiskFull));
    assert_eq!(
        writer.push_row(0.0, &[0.0; 3]),
        Err(StoreError::WriterStopped)
    );
    assert_eq!(writer.sync(), Err(StoreError::WriterStopped));
    assert_eq!(
        writer.finish(RunStatus::Completed).err(),
        Some(StoreError::WriterStopped)
    );
    let reader = SegmentReader::open(MemoryIo::from_bytes(io.snapshot())).unwrap();
    assert_eq!(reader.state(), &SegmentState::Recovered);
    assert!(reader.rows() > 0 && reader.rows() < 5000);
}

#[test]
fn an_abandoned_writer_leaves_the_chunks_it_handed_over() {
    let io = SharedIo::default();
    let (steps, _) = run(200, 0, 25);
    let mut writer =
        RunWriter::with_options(io.clone(), &event_schema(), &meta(), 64, NEVER).unwrap();
    for step in &steps {
        writer.push_row(step.x, &step.values).unwrap();
    }
    let change = EventChange {
        signal: 0,
        time: 0.0,
        sequence: 0,
        value: EventValue::Logic(Logic::One),
        strength: 0,
    };
    assert!(matches!(
        writer.push_events(&[change]),
        Err(StoreError::EventSignal { .. })
    ));
    assert!(matches!(
        writer.push_row(0.0, &[0.0; 2]),
        Err(StoreError::RowWidth {
            expected: 3,
            found: 2
        })
    ));
    drop(writer);
    let reader = SegmentReader::open(MemoryIo::from_bytes(io.snapshot())).unwrap();
    assert_eq!(reader.state(), &SegmentState::Recovered);
    assert_eq!(reader.rows(), 192);
}
