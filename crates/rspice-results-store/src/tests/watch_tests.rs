//! The disk watch: a write that would eat the closing reserve is refused, a run whose storage
//! fills stops there typed with every whole chunk kept, and the preflight refuses a run whose
//! rows cannot fit before it begins. A run's storage budget counts what every segment of the run
//! holds and stops the run whose next write would pass it, the same way.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::engine::{Engine, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_core::result_sink::{
    ResultSchema, ResultSignal, ResultSignalKind, ResultSink, RunMeta as SinkMeta,
    RunStatus as SinkStatus, SinkStop,
};

use crate::{
    CLOSING_RESERVE, FileIo, FreeSpace, MemoryIo, RunStatus, SegmentIo, SegmentReader,
    SegmentState, StopReason, StorageBudget, StoreError, StoreSink, WatchedIo, WatchedSink,
};

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice-store-watch-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A volume of `quota` bytes that holds only `segment`.
fn quota(segment: &Path, quota: u64) -> FreeSpace {
    let segment = segment.to_path_buf();
    Arc::new(move || {
        let used = std::fs::metadata(&segment).map_or(0, |meta| meta.len());
        Some(quota.saturating_sub(used))
    })
}

#[test]
fn a_write_that_would_eat_the_closing_reserve_is_refused_and_the_closing_writes_are_not() {
    let dir = scratch("io");
    let segment = dir.join("segment");
    let mut io = WatchedIo::new(
        FileIo::create(&segment).unwrap(),
        quota(&segment, CLOSING_RESERVE + 100),
    );
    io.write_at(0, &[1; 100]).unwrap();
    // Rewriting what is there grows nothing.
    io.write_at(0, &[2; 100]).unwrap();
    let refused = io.write_at(100, &[3; 1]).unwrap_err();
    assert_eq!(refused.kind(), std::io::ErrorKind::StorageFull);
    // The writer closing the segment uses the reserve.
    io.write_at(100, &[4; 4096]).unwrap();
    assert_eq!(io.size().unwrap(), 4196);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_run_whose_storage_fills_stops_typed_with_its_whole_chunks_equal_to_the_run_in_memory() {
    let dir = scratch("run");
    let segment = dir.join("segment");
    let netlist = Netlist::parse(
        "rc\nv1 in 0 sin(0 1 100k)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 200u\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let expected = engine
        .run_tran_with_startup_mode_and_abort(
            &netlist,
            200e-6,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &rspice_core::NoAbort,
        )
        .unwrap();

    let mut sink = StoreSink::new(WatchedIo::new(
        FileIo::create(&segment).unwrap(),
        quota(&segment, CLOSING_RESERVE + 256 * 1024),
    ));
    let run = engine
        .run_tran_into_sink_with_startup_mode_and_abort(
            &netlist,
            200e-6,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &rspice_core::NoAbort,
            &mut sink,
        )
        .unwrap();
    assert!(
        matches!(run.stopped, Some(SinkStop::DiskFull { .. })),
        "{:?}",
        run.stopped
    );

    // The reserve left room to close the segment over its whole chunks.
    let mut reader = SegmentReader::open(FileIo::open(&segment).unwrap()).unwrap();
    assert!(matches!(
        reader.state(),
        SegmentState::Complete {
            status: RunStatus::Stopped(StopReason::DiskFull),
            ..
        }
    ));
    let rows = reader.rows() as usize;
    assert_eq!(run.receipt.rows, rows as u64);
    assert!(
        rows > 1000 && rows < expected.time.len(),
        "{rows} of {} rows kept",
        expected.time.len()
    );
    let bits = |values: &[f64]| {
        values
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        bits(&reader.read_x(0, rows).unwrap()),
        bits(&expected.time[..rows])
    );
    let out = expected
        .node_names
        .iter()
        .position(|name| name == "OUT")
        .unwrap();
    let slot = reader
        .schema()
        .signals
        .iter()
        .position(|signal| signal.name == "OUT")
        .unwrap();
    assert_eq!(
        bits(&reader.read_slot(slot, 0, rows).unwrap()),
        bits(&expected.voltages[out][..rows])
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_preflight_refuses_rows_that_cannot_fit_and_warns_of_rows_that_likely_do_not() {
    let dir = scratch("preflight");
    let segment = dir.join("segment");
    let mut schema = ResultSchema::default();
    schema
        .signals
        .push(ResultSignal::new("out", "V", ResultSignalKind::Real));
    // 1000 rows of x and one value need 1000 * 2 * 8 * 1.035 = 16560 bytes.
    let free = |bytes: u64| -> FreeSpace { Arc::new(move || Some(bytes)) };
    let watched = |free: FreeSpace, needed: f64, likely: f64| {
        // The writes themselves see a large volume: only the preflight's estimate is tested.
        let io = WatchedIo::new(
            FileIo::create(&segment).unwrap(),
            Arc::new(|| Some(1 << 40)),
        );
        WatchedSink::new(StoreSink::new(io), free, needed, likely)
    };
    let mut refused = watched(free(16_559), 1000.0, 1000.0);
    let stop = refused.begin(&schema, &SinkMeta::default()).unwrap_err();
    assert_eq!(
        stop,
        SinkStop::DiskFull {
            needed: Some(16_560),
            available: Some(16_559)
        }
    );
    assert_eq!(refused.refused(), Some((16_560, 16_559)));
    let warned = Arc::new(AtomicU64::new(0));
    let seen = warned.clone();
    let mut fits =
        watched(free(16_560), 1000.0, 2000.0).with_space_warning(Box::new(move |likely, _free| {
            seen.store(likely, Ordering::SeqCst)
        }));
    fits.begin(&schema, &SinkMeta::default()).unwrap();
    assert_eq!(fits.refused(), None);
    assert_eq!(
        warned.load(Ordering::SeqCst),
        WatchedSink::<FileIo>::bytes(2000.0, 1)
    );
    let (mut inner, _) = fits.into_parts();
    inner.end(SinkStatus::Completed).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}

/// A volume with room for anything.
fn roomy() -> FreeSpace {
    Arc::new(|| Some(1 << 40))
}

#[test]
fn a_storage_budget_counts_what_every_segment_holds_and_refuses_the_write_that_would_pass_it() {
    let budget = StorageBudget::new(1000);
    let watched = || WatchedIo::new(MemoryIo::new(), roomy()).with_budget(Some(budget.clone()));
    let (mut first, mut second) = (watched(), watched());
    first.write_at(0, &[1; 400]).unwrap();
    // Rewriting what is there grows nothing; a provisional tail cut back is given back.
    first.write_at(0, &[2; 400]).unwrap();
    first.write_at(400, &[3; 200]).unwrap();
    first.truncate(400).unwrap();
    assert_eq!(budget.written(), 400);
    second.write_at(0, &[4; 600]).unwrap();
    assert_eq!(budget.written(), 1000);
    let refused = StoreError::from(first.write_at(400, &[5; 1]).unwrap_err());
    assert_eq!(
        refused,
        StoreError::StorageBudget {
            budget: 1000,
            written: 1000
        }
    );
    assert_eq!(first.size().unwrap(), 400);
    // The writes closing the refused segment, and those of a segment its writer is closing, are
    // counted and never refused.
    first.write_at(400, &[6; 64]).unwrap();
    second.closing();
    second.write_at(600, &[7; 64]).unwrap();
    assert_eq!(budget.written(), 1128);
}

#[test]
fn a_run_its_storage_budget_stops_keeps_its_whole_chunks_and_says_the_budget() {
    let dir = scratch("budget");
    let netlist = Netlist::parse(
        "rc\nv1 in 0 sin(0 1 100k)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 200u\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let run_into = |segment: &Path, budget: &Arc<StorageBudget>, stop: f64| {
        let mut sink = StoreSink::new(
            WatchedIo::new(FileIo::create(segment).unwrap(), roomy())
                .with_budget(Some(budget.clone())),
        );
        let run = engine
            .run_tran_into_sink_with_startup_mode_and_abort(
                &netlist,
                stop,
                1e-9,
                TransientStartupMode::OperatingPoint,
                &rspice_core::NoAbort,
                &mut sink,
            )
            .unwrap();
        (run.stopped, run.receipt.rows)
    };
    // One budget for the run's two transients: the first fits, the second reaches it.
    let budget = StorageBudget::new(256 * 1024);
    let (first, second) = (dir.join("first"), dir.join("second"));
    let (stopped, _) = run_into(&first, &budget, 2e-6);
    assert_eq!(stopped, None);
    let after_first = budget.written();
    assert_eq!(after_first, std::fs::metadata(&first).unwrap().len());
    let (stopped, rows) = run_into(&second, &budget, 200e-6);
    let Some(SinkStop::StorageBudget {
        budget: limit,
        written,
    }) = stopped
    else {
        panic!("the budget stops the run: {stopped:?}");
    };
    assert_eq!(limit, 256 * 1024);
    assert!(written <= limit && written > after_first, "{written}");
    // What the run's segments hold is what the budget counted: the refused chunk is not kept,
    // the closing frames are.
    let held = std::fs::metadata(&first).unwrap().len() + std::fs::metadata(&second).unwrap().len();
    assert_eq!(budget.written(), held);
    let reader = SegmentReader::open(FileIo::open(&second).unwrap()).unwrap();
    assert!(matches!(
        reader.state(),
        SegmentState::Complete {
            status: RunStatus::Stopped(StopReason::StorageBudget),
            ..
        }
    ));
    assert_eq!(reader.rows(), rows);
    assert!(rows > 1000 && rows < 200_000, "{rows} rows kept");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A volume of `capacity` bytes that holds only this segment, in memory: a write or an extension
/// past the capacity is refused as storage full, as a real volume refuses it.
struct Volume {
    io: MemoryIo,
    capacity: u64,
    used: Arc<AtomicU64>,
}

impl SegmentIo for Volume {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.io.read_at(offset, buf)
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        let end = offset + data.len() as u64;
        if end > self.capacity && end > self.io.size()? {
            return Err(std::io::ErrorKind::StorageFull.into());
        }
        self.io.write_at(offset, data)?;
        self.used.store(self.io.size()?, Ordering::Release);
        Ok(())
    }

    fn truncate(&mut self, len: u64) -> std::io::Result<()> {
        if len > self.capacity {
            return Err(std::io::ErrorKind::StorageFull.into());
        }
        self.io.truncate(len)?;
        self.used.store(len, Ordering::Release);
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<u64> {
        self.io.size()
    }
}

/// A watched segment on a volume of `capacity` bytes; `query` says whether the host reports the
/// free space.
fn on_volume(capacity: u64, query: bool) -> WatchedIo<Volume> {
    let used = Arc::new(AtomicU64::new(0));
    let seen = used.clone();
    let free: FreeSpace = if query {
        Arc::new(move || Some(capacity.saturating_sub(seen.load(Ordering::Acquire))))
    } else {
        Arc::new(|| None)
    };
    WatchedIo::new(
        Volume {
            io: MemoryIo::new(),
            capacity,
            used,
        },
        free,
    )
}

/// Push rows (and, with `events`, one change per event signal per chunk) until the volume
/// refuses one, then close the segment as stopped: it must end with `END`, its status and a
/// digest its bits hash to, over every whole chunk it holds.
fn fill(io: WatchedIo<Volume>, schema: &crate::Schema, events: u32) -> (u64, Vec<u8>) {
    use crate::{EventChange, EventValue, Logic, RunMeta, SegmentWriter};
    let mut writer =
        SegmentWriter::with_options(io, schema, &RunMeta::default(), 64, true).unwrap();
    let mut row = 0_u64;
    let refused = loop {
        let x = row as f64 * 1e-9;
        if events > 0 && row.is_multiple_of(64) {
            let changes: Vec<EventChange> = (1..=events)
                .map(|signal| EventChange {
                    signal,
                    time: x,
                    sequence: (row / 64) as u32,
                    value: EventValue::Logic(if (row / 64 + u64::from(signal)).is_multiple_of(2) {
                        Logic::Zero
                    } else {
                        Logic::One
                    }),
                    strength: 0,
                })
                .collect();
            if let Err(error) = writer.push_events(&changes) {
                break error;
            }
        }
        if let Err(error) = writer.push_row(x, &[(row % 7) as f64]) {
            break error;
        }
        row += 1;
    };
    assert_eq!(refused, StoreError::DiskFull);
    let (receipt, io) = writer
        .finish(RunStatus::Stopped(StopReason::DiskFull))
        .unwrap();
    let bytes = io.into_inner().io.into_bytes();
    assert_eq!(
        bytes.len() as u64,
        receipt.bytes,
        "nothing held past the tail"
    );
    let mut reader = SegmentReader::open(MemoryIo::from_bytes(bytes.clone())).unwrap();
    let SegmentState::Complete { status, digest } = reader.state().clone() else {
        panic!("END was written");
    };
    assert_eq!(status, RunStatus::Stopped(StopReason::DiskFull));
    assert_eq!(reader.rows(), receipt.rows);
    assert_eq!(reader.digest().unwrap(), digest);
    (receipt.chunks, bytes)
}

fn one_signal(events: u32) -> crate::Schema {
    let mut signals = vec![crate::Signal::new("v(out)", "V", crate::SignalKind::Real)];
    signals.extend(
        (0..events).map(|bit| crate::Signal::new(format!("d{bit}"), "", crate::SignalKind::Logic)),
    );
    crate::Schema {
        x: crate::Axis {
            name: "time".into(),
            unit: "s".into(),
        },
        signals,
    }
}

/// `END` copies every index entry, so a segment of many chunks needs far more than a fixed
/// reserve to close: a one-signal run of more than 25,000 chunks that fills its volume still ends
/// with `END`.
#[test]
fn a_run_of_twenty_five_thousand_chunks_that_fills_its_volume_still_writes_end() {
    let (chunks, _) = fill(on_volume(26 << 20, true), &one_signal(0), 0);
    assert!(chunks > 25_000, "{chunks} chunks");
}

/// Every event signal's last changes and every `EVENTS` frame's index entry are part of closing:
/// a run of 300 event signals that fills its volume still ends with `END`.
#[test]
fn a_run_of_three_hundred_event_signals_that_fills_its_volume_still_writes_end() {
    let (_, bytes) = fill(on_volume(40 << 20, true), &one_signal(300), 300);
    let frames = super::support::walk(&bytes)
        .iter()
        .filter(|frame| frame.kind == crate::format::FrameKind::Events)
        .count();
    assert!(frames > 25_000, "{frames} EVENTS frames");
}

/// A host that cannot say what is free still keeps room to close: the segment holds it past its
/// end, and gives it back once `END` is written.
#[test]
fn a_run_on_a_host_that_cannot_report_free_space_still_writes_end() {
    let (chunks, _) = fill(on_volume(26 << 20, false), &one_signal(0), 0);
    assert!(chunks > 25_000, "{chunks} chunks");
}
