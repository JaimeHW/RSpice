//! Event signals: round trips, determinism, crash prefixes and refusals.

use super::reader_tests::{RecordingIo, crash_states};
use super::support::{Rng, meta};
use crate::{
    Axis, EventChange, EventValue, Logic, MemoryIo, RunStatus, Schema, SegmentIo, SegmentReader,
    SegmentWriter, Signal, SignalKind, StoreError,
};

const LOGIC: u32 = 1;
const REAL: u32 = 2;
const BUS: u32 = 3;
const WIDE: u32 = 5;
const B128: u32 = 6;
const B1000: u32 = 7;
const EVENT_SIGNALS: [u32; 6] = [LOGIC, REAL, BUS, WIDE, B128, B1000];

/// An analog signal between event signals of every kind, so ids and slots differ.
fn event_schema() -> Schema {
    Schema {
        x: Axis {
            name: "time".into(),
            unit: "s".into(),
        },
        signals: vec![
            Signal::new("v(out)", "V", SignalKind::Real),
            Signal::new("clk", "", SignalKind::Logic),
            Signal::new("vref_evt", "V", SignalKind::RealEvent),
            Signal::new("addr[11:0]", "", SignalKind::Bus { width: 12 }),
            Signal::new("v(ac)", "V", SignalKind::Complex),
            Signal::new("data[63:0]", "", SignalKind::Bus { width: 64 }),
            Signal::new("key[127:0]", "", SignalKind::Bus { width: 128 }),
            Signal::new("frame[999:0]", "", SignalKind::Bus { width: 1000 }),
        ],
    }
}

/// What one step of a run pushes: a row, then the changes committed in it.
struct Step {
    x: f64,
    values: [f64; 3],
    changes: Vec<EventChange>,
}

/// Random bits of a `width`-bit bus; each byte is nonzero with probability one in `sparse`.
fn bits(rng: &mut Rng, width: u32, sparse: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = (0..width.div_ceil(8))
        .map(|_| {
            if rng.below(sparse) == 0 {
                rng.next_u64() as u8
            } else {
                0
            }
        })
        .collect();
    if !width.is_multiple_of(8) {
        *bytes.last_mut().unwrap() &= (1u8 << (width % 8)) - 1;
    }
    bytes
}

fn bus(rng: &mut Rng, width: u32) -> EventValue {
    let x = if rng.below(4) == 0 {
        bits(rng, width, 9)
    } else {
        bits(rng, width, usize::MAX)
    };
    let z = if rng.below(4) == 0 {
        bits(rng, width, 9)
    } else {
        bits(rng, width, usize::MAX)
    };
    EventValue::Bus {
        value: bits(rng, width, 1),
        x,
        z,
    }
}

fn change(rng: &mut Rng, signal: u32, time: f64, sequence: u32) -> EventChange {
    let value = match signal {
        LOGIC => EventValue::Logic([Logic::Zero, Logic::One, Logic::X, Logic::Z][rng.below(4)]),
        REAL => EventValue::Real(match rng.below(20) {
            0 => f64::NAN,
            1 => -0.0,
            2 => f64::from_bits(0x7ff8_0000_0000_beef),
            _ => rng.unit() * 3.3,
        }),
        BUS => bus(rng, 12),
        WIDE => bus(rng, 64),
        B128 => bus(rng, 128),
        _ => bus(rng, 1000),
    };
    EventChange {
        signal,
        time,
        sequence,
        value,
        strength: rng.below(8) as u8,
    }
}

/// `rows` steps with events on every signal, a burst of more than two frames of one signal inside
/// one chunk, and `tail` changes after the last row (an events-only stretch).
fn run(rows: usize, tail: usize, seed: u64) -> (Vec<Step>, Vec<EventChange>) {
    let mut rng = Rng(seed);
    let mut steps = Vec::with_capacity(rows);
    let mut t = 0.0;
    for row in 0..rows {
        t += 1e-9 * (0.5 + rng.unit());
        let mut changes = Vec::new();
        let mut sequence = 0;
        for signal in EVENT_SIGNALS {
            if rng.below(3) == 0 {
                changes.push(change(&mut rng, signal, t, sequence));
                sequence += 1;
            }
        }
        if row == rows / 3 {
            for burst in 0..9000 {
                changes.push(change(&mut rng, LOGIC, t, sequence + burst));
            }
        }
        steps.push(Step {
            x: t,
            values: [(t * 1e6).sin(), (t * 2e6).cos(), rng.unit()],
            changes,
        });
    }
    let mut after = Vec::with_capacity(tail);
    for at in 0..tail {
        t += 1e-9;
        after.push(change(&mut rng, [LOGIC, BUS][at % 2], t, 0));
    }
    (steps, after)
}

type Key = (u32, u64, u32, Vec<u8>, u8);

fn key(change: &EventChange) -> Key {
    let value = match &change.value {
        EventValue::Logic(logic) => vec![*logic as u8],
        EventValue::Real(value) => value.to_bits().to_le_bytes().to_vec(),
        EventValue::Bus { value, x, z } => [value.as_slice(), x, z].concat(),
    };
    (
        change.signal,
        change.time.to_bits(),
        change.sequence,
        value,
        change.strength,
    )
}

fn stream(steps: &[Step], after: &[EventChange], signal: u32) -> Vec<Key> {
    steps
        .iter()
        .flat_map(|step| step.changes.iter())
        .chain(after)
        .filter(|change| change.signal == signal)
        .map(key)
        .collect()
}

/// Write `steps` and `after`, pushing each step's changes `split` at a time and syncing every
/// `sync` rows.
fn write<I: SegmentIo>(
    mut writer: SegmentWriter<I>,
    steps: &[Step],
    after: &[EventChange],
    split: usize,
    sync: usize,
) -> (crate::StoreReceipt, I) {
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
    writer.finish(RunStatus::Completed).unwrap()
}

fn writer(chunk_rows: usize) -> SegmentWriter<MemoryIo> {
    SegmentWriter::with_chunk_rows(MemoryIo::new(), &event_schema(), &meta(), chunk_rows).unwrap()
}

fn keys(changes: &[EventChange]) -> Vec<Key> {
    changes.iter().map(key).collect()
}

#[test]
fn event_changes_read_back_with_x_z_and_strength() {
    let (steps, after) = run(1000, 5000, 11);
    let (receipt, io) = write(writer(64), &steps, &after, usize::MAX, usize::MAX);
    let total: usize = steps.iter().map(|step| step.changes.len()).sum::<usize>() + after.len();
    assert_eq!(receipt.events, total as u64);
    let mut reader = SegmentReader::open(io).unwrap();
    assert_eq!(reader.rows(), 1000);
    for signal in EVENT_SIGNALS {
        let written = stream(&steps, &after, signal);
        let read = reader
            .events(signal as usize, f64::NEG_INFINITY, f64::INFINITY)
            .unwrap();
        assert_eq!(keys(&read), written, "signal {signal}");
        assert_eq!(
            reader.event_count(signal as usize).unwrap(),
            written.len() as u64
        );
        // A window inside the run returns exactly the changes timed in it.
        let (t0, t1) = (steps[200].x, steps[700].x);
        let window: Vec<Key> = written
            .iter()
            .filter(|key| (t0..=t1).contains(&f64::from_bits(key.1)))
            .cloned()
            .collect();
        assert_eq!(
            keys(&reader.events(signal as usize, t0, t1).unwrap()),
            window
        );
    }
    assert_eq!(reader.digest().unwrap(), receipt.digest);
    assert!(reader.events(0, 0.0, 1.0).is_err());
    assert!(reader.event_count(4).is_err());
    assert!(reader.events(99, 0.0, 1.0).is_err());
}

#[test]
fn event_bytes_do_not_depend_on_how_changes_are_pushed_or_synced() {
    let (steps, after) = run(700, 300, 12);
    let (reference, io) = write(writer(64), &steps, &after, usize::MAX, usize::MAX);
    let reference_bytes = io.into_bytes();
    for (split, sync) in [(1, usize::MAX), (3, 1), (usize::MAX, 17), (2, 64)] {
        let (receipt, io) = write(writer(64), &steps, &after, split, sync);
        assert_eq!(receipt.digest, reference.digest);
        assert!(
            io.bytes() == reference_bytes.as_slice(),
            "split {split}, sync every {sync}"
        );
    }
}

#[test]
fn the_digest_does_not_depend_on_codec_or_chunk_rows() {
    let (steps, after) = run(3000, 200, 13);
    let schema = event_schema();
    let mut digests = Vec::new();
    let mut sizes = Vec::new();
    for (chunk_rows, compress) in [(64, true), (64, false), (128, true), (4096, false)] {
        let writer =
            SegmentWriter::with_options(MemoryIo::new(), &schema, &meta(), chunk_rows, compress)
                .unwrap();
        let (receipt, io) = write(writer, &steps, &after, usize::MAX, usize::MAX);
        sizes.push(receipt.bytes);
        let mut reader = SegmentReader::open(io).unwrap();
        assert_eq!(reader.digest().unwrap(), receipt.digest);
        digests.push(receipt.digest);
    }
    let (receipt, _) = write(
        SegmentWriter::create(MemoryIo::new(), &schema, &meta()).unwrap(),
        &steps,
        &after,
        5,
        100,
    );
    digests.push(receipt.digest);
    assert!(
        digests.iter().all(|digest| *digest == digests[0]),
        "{digests:?}"
    );
    assert!(
        sizes[0] < sizes[1],
        "compressed {} vs plain {}",
        sizes[0],
        sizes[1]
    );
}

/// What a run wrote, flattened once for the prefix checks: x per row and each signal's changes.
struct Written {
    x: Vec<u64>,
    streams: Vec<(u32, Vec<Key>)>,
}

impl Written {
    fn of(steps: &[Step], after: &[EventChange]) -> Self {
        Self {
            x: steps.iter().map(|step| step.x.to_bits()).collect(),
            streams: EVENT_SIGNALS
                .into_iter()
                .map(|signal| (signal, stream(steps, after, signal)))
                .collect(),
        }
    }

    fn total(&self) -> usize {
        self.streams.iter().map(|(_, stream)| stream.len()).sum()
    }

    /// Open `bytes` and check that the readable rows and each signal's readable changes are
    /// prefixes of what was written. Returns the readable rows, then the readable changes per
    /// signal; the values are decoded only when those counts differ from `previous`.
    fn prefix(&self, bytes: &[u8], previous: Option<&Vec<u64>>) -> Option<Vec<u64>> {
        let mut reader = SegmentReader::open(MemoryIo::from_bytes(bytes.to_vec())).ok()?;
        let mut counts = vec![reader.rows()];
        for (signal, _) in &self.streams {
            counts.push(reader.event_count(*signal as usize).unwrap());
        }
        if previous == Some(&counts) {
            return Some(counts);
        }
        let rows = counts[0] as usize;
        assert!(rows <= self.x.len());
        let x: Vec<u64> = reader
            .read_x(0, rows)
            .unwrap()
            .iter()
            .map(|x| x.to_bits())
            .collect();
        assert_eq!(x, self.x[..rows]);
        for (at, (signal, written)) in self.streams.iter().enumerate() {
            let read = keys(
                &reader
                    .events(*signal as usize, f64::NEG_INFINITY, f64::INFINITY)
                    .unwrap(),
            );
            assert_eq!(read.len() as u64, counts[at + 1]);
            assert!(read.len() <= written.len());
            assert_eq!(read, written[..read.len()], "signal {signal}");
        }
        Some(counts)
    }
}

fn changes_in(counts: &[u64]) -> usize {
    counts[1..].iter().sum::<u64>() as usize
}

#[test]
fn a_cut_or_crash_leaves_a_prefix_of_each_event_stream() {
    let (mut steps, after) = run(200, 40, 14);
    // Shrink the burst so a byte-by-byte cut stays quick; keep one sealed frame of it.
    for step in &mut steps {
        step.changes.truncate(4200);
    }
    let written = Written::of(&steps, &after);
    let total = written.total();
    let (_, io) = write(writer(64), &steps, &after, usize::MAX, usize::MAX);
    let bytes = io.into_bytes();
    let mut last: Option<Vec<u64>> = None;
    for cut in 0..=bytes.len() {
        if let Some(counts) = written.prefix(&bytes[..cut], last.as_ref()) {
            if let Some(last) = &last {
                assert!(
                    counts.iter().zip(last).all(|(now, before)| now >= before),
                    "cutting at {cut} lost rows or changes"
                );
            }
            last = Some(counts);
        }
    }
    assert_eq!(last.map(|counts| changes_in(&counts)), Some(total));

    let recorder =
        SegmentWriter::with_chunk_rows(RecordingIo::default(), &event_schema(), &meta(), 64)
            .unwrap();
    let (_, io) = write(recorder, &steps, &after, 7, 23);
    let mut most = 0;
    for state in crash_states(&io.log) {
        let counts = written.prefix(&state, None);
        most = most.max(counts.map_or(0, |counts| changes_in(&counts)));
    }
    assert_eq!(most, total);

    // A sync makes the open buffers' changes durable.
    let mut writer = writer(64);
    for step in &steps[..10] {
        writer.push_row(step.x, &step.values).unwrap();
        writer.push_events(&step.changes).unwrap();
    }
    writer.sync().unwrap();
    let pushed: usize = steps[..10].iter().map(|step| step.changes.len()).sum();
    let counts = written.prefix(writer.io().bytes(), None).unwrap();
    assert_eq!((counts[0], changes_in(&counts)), (10, pushed));
}

#[test]
fn changes_that_do_not_fit_the_schema_are_refused_whole() {
    let mut writer = writer(64);
    writer.push_row(0.0, &[0.0; 3]).unwrap();
    let good = EventChange {
        signal: LOGIC,
        time: 0.0,
        sequence: 0,
        value: EventValue::Logic(Logic::One),
        strength: 6,
    };
    let refused = [
        EventChange {
            signal: 0,
            ..good.clone()
        },
        EventChange {
            signal: 4,
            ..good.clone()
        },
        EventChange {
            signal: 99,
            ..good.clone()
        },
        EventChange {
            value: EventValue::Real(1.0),
            ..good.clone()
        },
        // Bit 12 of a 12-bit bus.
        EventChange {
            signal: BUS,
            value: EventValue::Bus {
                value: vec![0, 0x10],
                x: vec![0, 0],
                z: vec![0, 0],
            },
            ..good.clone()
        },
        // Z masks one byte short of the bus.
        EventChange {
            signal: BUS,
            value: EventValue::Bus {
                value: vec![0, 0],
                x: vec![0, 0],
                z: vec![0],
            },
            ..good.clone()
        },
        // An X mask one byte longer than a 1000-bit bus.
        EventChange {
            signal: B1000,
            value: EventValue::Bus {
                value: vec![0; 125],
                x: vec![0; 126],
                z: vec![0; 125],
            },
            ..good.clone()
        },
    ];
    for bad in refused {
        assert!(matches!(
            writer.push_events(&[good.clone(), bad]),
            Err(StoreError::EventSignal { .. })
        ));
    }
    // A refusal stops nothing, and none of the refused calls' changes were kept.
    writer.push_events(std::slice::from_ref(&good)).unwrap();
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    assert_eq!(receipt.events, 1);
    let mut reader = SegmentReader::open(io).unwrap();
    assert_eq!(
        keys(&reader.events(1, -1.0, 1.0).unwrap()),
        vec![key(&good)]
    );

    // A bus has at least one bit and no upper bound.
    for (width, valid) in [(0, false), (65, true), (100_000, true)] {
        let mut schema = event_schema();
        schema.signals[BUS as usize].kind = SignalKind::Bus { width };
        let created = SegmentWriter::create(MemoryIo::new(), &schema, &meta());
        assert_eq!(created.is_ok(), valid, "width {width}");
    }
}
