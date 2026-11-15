//! Reading back what the writer wrote, and what a crash at any point leaves readable.

use std::io;

use super::support::{meta, mixed_schema, rows};
use crate::format::POINTERS_AT;
use crate::{
    MemoryIo, RunStatus, SegmentIo, SegmentReader, SegmentState, SegmentWriter, StopReason,
};

/// A segment I/O that logs every mutation, so a test can rebuild the bytes a crash at any point
/// would leave, including a write torn halfway.
#[derive(Default)]
pub(crate) struct RecordingIo {
    current: MemoryIo,
    pub(crate) log: Vec<Op>,
}

#[derive(Clone)]
pub(crate) enum Op {
    Write(u64, Vec<u8>),
    Truncate(u64),
}

impl SegmentIo for RecordingIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        self.current.read_at(offset, buf)
    }
    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        self.log.push(Op::Write(offset, data.to_vec()));
        self.current.write_at(offset, data)
    }
    fn truncate(&mut self, len: u64) -> io::Result<()> {
        self.log.push(Op::Truncate(len));
        self.current.truncate(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn size(&self) -> io::Result<u64> {
        self.current.size()
    }
}

/// Every state a crash could leave: after each logged operation, and with each write half done.
pub(crate) fn crash_states(log: &[Op]) -> Vec<Vec<u8>> {
    let mut states = Vec::new();
    let mut io = MemoryIo::new();
    for op in log {
        match op {
            Op::Write(offset, data) => {
                let mut torn = io.clone();
                torn.write_at(*offset, &data[..data.len() / 2]).unwrap();
                states.push(torn.into_bytes());
                io.write_at(*offset, data).unwrap();
            }
            Op::Truncate(len) => io.truncate(*len).unwrap(),
        }
        states.push(io.bytes().to_vec());
    }
    states
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

/// Open `bytes` and check that every readable row is the row written at that position.
fn readable_prefix(bytes: &[u8], x: &[f64], values: &[f64], slots: usize) -> Option<u64> {
    let mut reader = SegmentReader::open(MemoryIo::from_bytes(bytes.to_vec())).ok()?;
    let rows = reader.rows();
    let count = rows as usize;
    assert!(
        count <= x.len(),
        "{count} readable rows of {} written",
        x.len()
    );
    assert_eq!(bits(&reader.read_x(0, count).unwrap()), bits(&x[..count]));
    for slot in 0..slots {
        let column: Vec<f64> = (0..count).map(|row| values[row * slots + slot]).collect();
        assert_eq!(
            bits(&reader.read_slot(slot, 0, count).unwrap()),
            bits(&column),
            "slot {slot}"
        );
    }
    Some(rows)
}

#[test]
fn every_row_reads_back_bit_for_bit_with_the_status_and_digest() {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(5000, slots, 1);
    let mut writer = SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), 64).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let status = RunStatus::Stopped(StopReason::Failed("timestep too small".into()));
    let (receipt, io) = writer.finish(status.clone()).unwrap();
    let bytes = io.bytes().to_vec();
    let mut reader = SegmentReader::open(io).unwrap();
    assert_eq!(
        reader.state(),
        &SegmentState::Complete {
            status,
            digest: receipt.digest
        }
    );
    assert_eq!(reader.schema(), &schema);
    assert_eq!(reader.run_meta(), &meta());
    assert_eq!(reader.rows(), 5000);
    assert_eq!(reader.digest().unwrap(), receipt.digest);
    assert_eq!(readable_prefix(&bytes, &x, &values, slots), Some(5000));
    // A window that crosses chunk boundaries.
    assert_eq!(
        bits(&reader.read_slot(2, 60, 200).unwrap()),
        bits(
            &values[60 * slots..260 * slots]
                .iter()
                .skip(2)
                .step_by(slots)
                .copied()
                .collect::<Vec<_>>()
        )
    );
    assert!(reader.read_x(4990, 11).is_err());
    assert_eq!(reader.x_range(), Some((x[0], x[4999])));
}

#[test]
fn a_segment_cut_at_any_byte_reads_as_a_prefix_of_its_rows() {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(300, slots, 2);
    let mut writer = SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), 64).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let bytes = io.into_bytes();
    let mut last = 0;
    let mut opened = 0;
    for cut in 0..=bytes.len() {
        if let Some(rows) = readable_prefix(&bytes[..cut], &x, &values, slots) {
            assert!(
                rows >= last,
                "cutting later never loses rows ({rows} < {last} at {cut})"
            );
            last = rows;
            opened += 1;
        } else {
            assert_eq!(
                opened, 0,
                "once the schema is whole every cut opens (failed at {cut})"
            );
        }
    }
    assert_eq!(last, 300);
}

#[test]
fn a_crash_at_any_write_including_during_syncs_reads_as_a_prefix() {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(700, slots, 3);
    let mut writer =
        SegmentWriter::with_chunk_rows(RecordingIo::default(), &schema, &meta(), 64).unwrap();
    for row in 0..700 {
        writer
            .push_row(x[row], &values[row * slots..(row + 1) * slots])
            .unwrap();
        if row % 37 == 5 {
            writer.sync().unwrap();
        }
    }
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let states = crash_states(&io.log);
    assert!(states.len() > 40);
    let mut most = 0;
    for state in &states {
        most = most.max(readable_prefix(state, &x, &values, slots).unwrap_or(0));
    }
    assert_eq!(most, 700);
    // A crash right after a sync keeps every row pushed before it.
    let mut writer = SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), 64).unwrap();
    writer.push_rows(&x[..100], &values[..100 * slots]).unwrap();
    writer.sync().unwrap();
    assert_eq!(
        readable_prefix(writer.io().bytes(), &x, &values, slots),
        Some(100)
    );
}

#[test]
fn a_damaged_superblock_pointer_falls_back_to_scanning() {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(4500, slots, 4);
    let mut writer = SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), 64).unwrap();
    writer.push_rows(&x, &values).unwrap();
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    let mut bytes = io.into_bytes();
    for garbage in [12345u64, 7, u64::MAX] {
        let at = POINTERS_AT as usize;
        bytes[at..at + 8].copy_from_slice(&garbage.to_le_bytes());
        bytes[at + 16..at + 24].copy_from_slice(&garbage.to_le_bytes());
        let reader = SegmentReader::open(MemoryIo::from_bytes(bytes.clone())).unwrap();
        assert_eq!(reader.rows(), 4500);
        assert_eq!(
            reader.state(),
            &SegmentState::Complete {
                status: RunStatus::Completed,
                digest: receipt.digest
            }
        );
    }
}

#[test]
fn foreign_bytes_are_refused() {
    assert!(SegmentReader::open(MemoryIo::from_bytes(vec![0; 10])).is_err());
    assert!(SegmentReader::open(MemoryIo::from_bytes(vec![0; 200])).is_err());
}
