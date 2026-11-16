use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use super::events_tests::{Step, event_schema, run};
use super::support::{meta, mixed_schema, rows};
use crate::{
    EventChange, MemoryIo, RunStatus, SegmentIo, SegmentReader, SegmentState, SegmentWriter,
    StoreError, StoreReceipt,
};

/// A segment in memory the test keeps a handle on, logging every write and truncation.
#[derive(Clone, Default)]
struct SharedIo {
    bytes: Rc<RefCell<MemoryIo>>,
    log: Rc<RefCell<Vec<Op>>>,
}

#[derive(Clone)]
enum Op {
    Write(u64, Vec<u8>),
    Truncate(u64),
}

impl SharedIo {
    fn holding(bytes: &[u8]) -> Self {
        Self {
            bytes: Rc::new(RefCell::new(MemoryIo::from_bytes(bytes.to_vec()))),
            log: Rc::default(),
        }
    }

    fn bytes(&self) -> Vec<u8> {
        self.bytes.borrow().bytes().to_vec()
    }
}

impl SegmentIo for SharedIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        self.bytes.borrow().read_at(offset, buf)
    }
    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        self.log.borrow_mut().push(Op::Write(offset, data.to_vec()));
        self.bytes.borrow_mut().write_at(offset, data)
    }
    fn truncate(&mut self, len: u64) -> io::Result<()> {
        self.log.borrow_mut().push(Op::Truncate(len));
        self.bytes.borrow_mut().truncate(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn size(&self) -> io::Result<u64> {
        self.bytes.borrow().size()
    }
}

/// Push `steps` (each row, then its changes) into `writer`.
fn push<I: SegmentIo>(writer: &mut SegmentWriter<I>, steps: &[Step]) {
    for step in steps {
        writer.push_row(step.x, &step.values).unwrap();
        writer.push_events(&step.changes).unwrap();
    }
}

fn whole_run(steps: &[Step], chunk_rows: usize) -> (StoreReceipt, Vec<u8>) {
    let mut writer =
        SegmentWriter::with_chunk_rows(MemoryIo::new(), &event_schema(), &meta(), chunk_rows)
            .unwrap();
    push(&mut writer, steps);
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    (receipt, io.into_bytes())
}

/// A run that went on past row `through` and ended there, as a checkpointed run leaves its store.
fn ran_past(steps: &[Step], chunk_rows: usize) -> Vec<u8> {
    whole_run(steps, chunk_rows).1
}

/// Reopen `bytes` after row `through` of `steps`, push the rest and finish.
fn resumed(
    bytes: &[u8],
    steps: &[Step],
    through: usize,
    chunk_rows: usize,
) -> (StoreReceipt, Vec<u8>) {
    let mut writer = SegmentWriter::reopen_with_chunk_rows(
        MemoryIo::from_bytes(bytes.to_vec()),
        steps[through].x,
        Some(chunk_rows),
    )
    .unwrap();
    assert_eq!(writer.rows(), through as u64 + 1);
    push(&mut writer, &steps[through + 1..]);
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    (receipt, io.into_bytes())
}

fn changes_through(steps: &[Step], through: usize) -> usize {
    steps[..=through]
        .iter()
        .map(|step| step.changes.len())
        .sum()
}

/// A resume keeps the rows and changes up to its row and appends the rest after them: the segment
/// it finishes is the uninterrupted run's, byte for byte, whatever the run that wrote it did after
/// that row. The rows tried sit mid-chunk, at a chunk's last row, at the first row, and at a row
/// whose 9,000 changes filled event frames before its chunk was written.
#[test]
fn a_resumed_segment_is_the_uninterrupted_runs_byte_for_byte() {
    let (steps, _) = run(3000, 0, 21);
    let (whole, whole_bytes) = whole_run(&steps, 64);
    for (through, ran) in [
        (1234, 2000),
        (1279, 3000),
        (0, 500),
        (1000, 1100),
        (2999, 3000),
    ] {
        let past = ran_past(&steps[..ran], 64);
        let (receipt, bytes) = resumed(&past, &steps, through, 64);
        assert_eq!(receipt, whole, "resumed after row {through}");
        assert!(bytes == whole_bytes, "resumed after row {through}");
    }
}

/// The same for a long run whose summary pyramid has written frames at several levels: the
/// pyramid at the cut is rebuilt from what the segment stores.
#[test]
fn a_resumed_segment_continues_its_summaries() {
    let slots = mixed_schema().slots();
    let (x, values) = rows(150_000, slots, 5);
    let write = |range: std::ops::Range<usize>, mut writer: SegmentWriter<MemoryIo>| {
        for row in range {
            writer
                .push_row(x[row], &values[row * slots..(row + 1) * slots])
                .unwrap();
        }
        writer.finish(RunStatus::Completed).unwrap()
    };
    let fresh =
        || SegmentWriter::with_chunk_rows(MemoryIo::new(), &mixed_schema(), &meta(), 256).unwrap();
    let (whole, whole_io) = write(0..x.len(), fresh());
    let (_, past) = write(0..140_000, fresh());
    let through = 131_171;
    let writer = SegmentWriter::reopen_with_chunk_rows(past, x[through], Some(256)).unwrap();
    let (receipt, io) = write(through + 1..x.len(), writer);
    assert_eq!(receipt, whole);
    assert!(io.bytes() == whole_io.bytes());
}

/// A resume reads its row back from a segment the run left without `END`, and from one already
/// resumed once.
#[test]
fn a_resume_continues_a_recovered_or_resumed_segment() {
    let (steps, _) = run(3000, 0, 22);
    let (whole, whole_bytes) = whole_run(&steps, 64);
    let past = ran_past(&steps[..2600], 64);
    // A crash cut the run's segment short of `END`.
    let torn = &past[..past.len() * 9 / 10];
    let reader = SegmentReader::open(MemoryIo::from_bytes(torn.to_vec())).unwrap();
    assert_eq!(*reader.state(), SegmentState::Recovered);
    let (receipt, bytes) = resumed(torn, &steps, 1500, 64);
    assert_eq!(receipt, whole);
    assert!(bytes == whole_bytes);
    // Resumed at 1500 and run on to 2600, then resumed again at 2200.
    let (_, again) = resumed(&past, &steps[..2600], 1500, 64);
    let (receipt, bytes) = resumed(&again, &steps, 2200, 64);
    assert_eq!(receipt, whole);
    assert!(bytes == whole_bytes);
}

/// A point the segment holds no row at is refused, and the segment is left as it was.
#[test]
fn a_resume_from_a_point_with_no_row_leaves_the_segment_alone() {
    let (steps, _) = run(600, 0, 23);
    let past = ran_past(&steps, 64);
    let io = SharedIo::holding(&past);
    for x in [steps[300].x * (1.0 + f64::EPSILON), steps[599].x * 2.0, 0.0] {
        let refused = SegmentWriter::reopen(io.clone(), x).err();
        assert_eq!(
            refused,
            Some(StoreError::NoRowAt {
                x_bits: x.to_bits()
            })
        );
    }
    assert!(io.bytes() == past);
    assert!(io.log.borrow().is_empty());
}

/// A crash at any write or truncation of a resume (each write also torn in half) leaves a segment
/// that opens, reads to its last whole chunk, and that a second resume from the same row extends
/// to the uninterrupted run, unless the tear lands in the one provisional write that carries the
/// kept rows; then the second resume finds no row there and says so.
#[test]
fn a_crash_while_resuming_leaves_a_segment_a_second_resume_extends() {
    let (steps, _) = run(1600, 0, 24);
    let (whole, _) = whole_run(&steps, 64);
    let past = ran_past(&steps, 64);
    let through = 1000;
    let io = SharedIo::holding(&past);
    let writer =
        SegmentWriter::reopen_with_chunk_rows(io.clone(), steps[through].x, Some(64)).unwrap();
    drop(writer);
    let log = io.log.borrow().clone();
    assert!(!log.is_empty());
    let mut states = Vec::new();
    let mut image = MemoryIo::from_bytes(past.clone());
    for op in &log {
        match op {
            Op::Write(offset, data) => {
                let mut torn = image.clone();
                torn.write_at(*offset, &data[..data.len() / 2]).unwrap();
                states.push((true, torn.into_bytes()));
                image.write_at(*offset, data).unwrap();
            }
            Op::Truncate(len) => image.truncate(*len).unwrap(),
        }
        states.push((false, image.bytes().to_vec()));
    }
    let mut lost = 0;
    for (index, (torn, bytes)) in states.iter().enumerate() {
        let reader = SegmentReader::open(MemoryIo::from_bytes(bytes.clone()))
            .unwrap_or_else(|error| panic!("state {index} does not open: {error}"));
        assert!(reader.rows() > 0, "state {index}");
        match SegmentWriter::reopen_with_chunk_rows(
            MemoryIo::from_bytes(bytes.clone()),
            steps[through].x,
            Some(64),
        ) {
            Ok(mut writer) => {
                push(&mut writer, &steps[through + 1..]);
                let (receipt, _) = writer.finish(RunStatus::Completed).unwrap();
                assert_eq!(receipt, whole, "state {index}");
            }
            Err(StoreError::NoRowAt { .. }) if *torn => lost += 1,
            Err(error) => panic!("state {index}: {error}"),
        }
    }
    assert!(lost <= 1, "{lost} torn states lost the resume's row");
    let kept: usize = changes_through(&steps, through);
    assert!(kept > 0);
}

/// Changes a resumed run hands over at or before its row are the ones the segment kept.
#[test]
fn a_resume_keeps_exactly_the_changes_up_to_its_row() {
    let (steps, _) = run(1200, 0, 25);
    let past = ran_past(&steps, 64);
    let through = 700;
    let writer = SegmentWriter::reopen_with_chunk_rows(
        MemoryIo::from_bytes(past),
        steps[through].x,
        Some(64),
    )
    .unwrap();
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    assert_eq!(receipt.rows, through as u64 + 1);
    assert_eq!(receipt.events, changes_through(&steps, through) as u64);
    let mut reader = SegmentReader::open(io).unwrap();
    let expected: Vec<&EventChange> = steps[..=through]
        .iter()
        .flat_map(|step| step.changes.iter())
        .filter(|change| change.signal == 1)
        .collect();
    let read = reader.events(1, f64::NEG_INFINITY, f64::INFINITY).unwrap();
    assert_eq!(read.len(), expected.len());
    assert!(
        read.iter()
            .zip(expected)
            .all(|(read, expected)| read == expected)
    );
    assert_eq!(reader.digest().unwrap(), receipt.digest);
}
