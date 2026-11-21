//! A transient stored in a segment on disk, drawn, measured, stepped through and exported from
//! the store and never read whole.
//!
//! The suite runs a twin of 256 MiB; the run of 10 GiB, five times the reader's memory budget,
//! is ignored and run by hand. Each writes its segment under the target directory, reports what
//! it took, and deletes it.

use super::frame_cost::{FrameCost, frame, new_context};
use super::lanes::place_cursors;
use super::*;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use rspice_results::feature_search::Direction;
use rspice_results::measurements::{IntervalStatistics, measure_interval, measure_interval_values};
use rspice_results::store_reader::{Segment, SegmentColumn, StoreReader};
use rspice_results::waveform::column_work::ColumnWork;
use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};
use rspice_results_store::{
    Axis, FileIo, RunMeta, RunStatus, Schema, SegmentWriter, Signal, SignalKind, available_bytes,
};
use rspice_results_ui::waves::navigation::CursorJump;

/// Bytes of samples a row holds: its time and one signal.
const ROW_BYTES: u64 = 16;
/// Rows pushed to the segment at a time.
const CHUNK_ROWS: u64 = 1 << 20;
/// The time between rows.
const STEP_S: f64 = 1.0e-9;
/// The rows a cursor steps through.
const STEPS: u64 = 200;
/// The rows exported around the cursor.
const EXPORT_ROWS: u64 = 100_000;

/// Sample `row` of a run of `rows`: 64 periods of a sine over the run, with noise of a tenth of
/// its amplitude drawn from the row.
fn sample(row: u64, rows: u64) -> f64 {
    let mut z = row.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let noise = (z >> 11) as f64 / (1_u64 << 53) as f64 - 0.5;
    let phase = std::f64::consts::TAU * 64.0 * row as f64 / rows as f64;
    0.1f64.mul_add(noise, phase.sin())
}

/// A folder of the target directory, deleted with what it holds when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        // The test binary runs from `<target>/debug/deps`.
        let target = std::env::current_exe()
            .expect("the test binary has a path")
            .ancestors()
            .nth(3)
            .expect("the test binary is in the target directory")
            .to_path_buf();
        let folder = target
            .join("stored-run-tests")
            .join(format!("{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("the folder is made");
        Self(folder)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A transient of `rows` rows written to a segment in `folder`, read back from it: its time
/// axis and one signal, `V(s0)`. The segment is deleted with the last column that reads it.
fn stored_run(folder: &Path, rows: u64) -> AnalysisResult {
    let path = folder.join("transient-0.rsr");
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![Signal::new("V(s0)", "V", SignalKind::Real)],
    };
    let io = FileIo::create(&path).expect("the segment is created");
    let mut writer =
        SegmentWriter::create(io, &schema, &RunMeta::default()).expect("the segment starts");
    let (mut x, mut y) = (Vec::new(), Vec::new());
    for first in (0..rows).step_by(CHUNK_ROWS as usize) {
        let end = (first + CHUNK_ROWS).min(rows);
        x.clear();
        y.clear();
        x.extend((first..end).map(|row| row as f64 * STEP_S));
        y.extend((first..end).map(|row| sample(row, rows)));
        writer.push_rows(&x, &y).expect("the rows are written");
    }
    let (_, io) = writer.finish(RunStatus::Completed).expect("the run ends");
    drop(io);
    let reader = StoreReader::open_file(&path).expect("the segment opens");
    let segment = Segment::temporary(reader, path).shared();
    let window = StoredWindow {
        first: 0,
        rows,
        lead: WindowLead::None,
    };
    let column = |slot: Option<usize>, name: &str| {
        let source = SegmentColumn::new(std::sync::Arc::clone(&segment), slot, name.to_owned());
        WaveformValues::stored(source, window).expect("the window fits the segment")
    };
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new(
            "V(s0)",
            column(None, "time"),
            column(Some(0), "V(s0)"),
            "#00aaff",
        )
        .with_unit("V"),
    ])
}

/// The process's peak working set: the most physical memory it has held at once.
#[cfg(windows)]
fn peak_working_set() -> Option<u64> {
    use std::ffi::c_void;

    /// `PROCESS_MEMORY_COUNTERS`.
    #[repr(C)]
    #[derive(Default)]
    struct Counters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set_size: usize,
        working_set_size: usize,
        quota_peak_paged_pool_usage: usize,
        quota_paged_pool_usage: usize,
        quota_peak_non_paged_pool_usage: usize,
        quota_non_paged_pool_usage: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut c_void;
        fn K32GetProcessMemoryInfo(process: *mut c_void, counters: *mut Counters, cb: u32) -> i32;
    }

    let cb = u32::try_from(std::mem::size_of::<Counters>()).ok()?;
    let mut counters = Counters {
        cb,
        ..Counters::default()
    };
    // SAFETY: the current process's pseudo-handle needs no closing, and `counters` is a
    // `PROCESS_MEMORY_COUNTERS` of the `cb` bytes it is said to be.
    let read = unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, cb) };
    (read != 0).then_some(counters.peak_working_set_size as u64)
}

/// The process's peak resident set: the most physical memory it has held at once.
#[cfg(target_os = "linux")]
fn peak_working_set() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|line| line.starts_with("VmHWM:"))?;
    let kib: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kib * 1024)
}

#[cfg(not(any(windows, target_os = "linux")))]
fn peak_working_set() -> Option<u64> {
    None
}

/// One frame of the strip and how long it took to build and tessellate.
struct TimedFrame {
    cost: FrameCost,
    took: Duration,
}

/// [`frame`], timed.
fn timed_frame(ctx: &egui::Context, state: &mut ResultsTestHost) -> TimedFrame {
    let started = Instant::now();
    let cost = frame(ctx, state, Vec::new());
    TimedFrame {
        cost,
        took: started.elapsed(),
    }
}

/// What a stored run of `rows` rows took to write, draw, measure, step through and export.
struct RunCost {
    write: Duration,
    open: TimedFrame,
    /// Frames that each pan a hundredth-width window a fiftieth of the span on.
    pans: Vec<TimedFrame>,
    /// Frames that each zoom in tenfold about one point, from the whole run to a window of a
    /// few hundred rows.
    zooms: Vec<TimedFrame>,
    measure: Duration,
    measured: IntervalStatistics,
    /// The slowest of the cursor's steps.
    step: Duration,
    export: Duration,
    exported_bytes: usize,
    columns: ColumnWork,
}

impl RunCost {
    fn frames(&self) -> impl Iterator<Item = &TimedFrame> {
        std::iter::once(&self.open)
            .chain(&self.pans)
            .chain(&self.zooms)
    }

    /// The slowest frame after the one that opened the run.
    fn worst_frame(&self) -> Duration {
        self.pans
            .iter()
            .chain(&self.zooms)
            .map(|frame| frame.took)
            .max()
            .unwrap_or_default()
    }

    fn report(&self, name: &str, rows: u64) {
        let millis = |duration: Duration| duration.as_secs_f64() * 1e3;
        let mut totals: Vec<Duration> = self.frames().map(|frame| frame.took).collect();
        totals.sort_unstable();
        let peak = peak_working_set().map_or_else(
            || "unmeasured".to_owned(),
            |bytes| format!("{:.0} MiB", bytes as f64 / f64::from(1 << 20)),
        );
        let text = format!(
            "{name}: {rows} rows, {:.2} GiB of samples\n  written in {:.1} s | opened in \
             {:.1} ms | pan and zoom frames median {:.1} ms, worst {:.1} ms\n  measured in \
             {:.1} ms ({:?}) | slowest step {:.2} ms | {EXPORT_ROWS} rows exported in {:.1} ms \
             ({} bytes)\n  whole-column work {:?} | peak working set {peak}",
            (rows * ROW_BYTES) as f64 / f64::from(1 << 30),
            self.write.as_secs_f64(),
            millis(self.open.took),
            millis(totals[totals.len() / 2]),
            millis(self.worst_frame()),
            millis(self.measure),
            self.measured,
            millis(self.step),
            millis(self.export),
            self.exported_bytes,
            self.columns,
        );
        // Through a locked handle: the crate denies the printing macros.
        writeln!(std::io::stdout().lock(), "{text}").expect("the report is written");
    }
}

/// Write, draw, measure, step through and export a stored run of `rows` rows, in a folder the
/// target directory's volume keeps `reserve` bytes free beside. A run `beyond_budget` is refused
/// when it is asked for whole.
fn exercise(name: &str, rows: u64, reserve: u64, beyond_budget: bool) -> RunCost {
    let scratch = Scratch::new(name);
    let free = available_bytes(&scratch.0).expect("the target's volume reports its free space");
    let needed = rows * ROW_BYTES + reserve;
    assert!(
        free >= needed,
        "{name} needs {needed} bytes free on the target's volume, which has {free}"
    );
    let started = Instant::now();
    let analysis = stored_run(&scratch.0, rows);
    let write = started.elapsed();
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(analysis);
    let columns = ColumnWork::read();

    // Open, then pan and zoom.
    let ctx = new_context();
    let open = timed_frame(&ctx, &mut state);
    let model = &state
        .results
        .results
        .models
        .last_built()
        .expect("the first frame built the strip")[0];
    let analysis = model.analysis_key;
    let (x0, x1) = model.x_range.expect("the strip has an X extent");
    let span = x1 - x0;
    let view = |state: &mut ResultsTestHost, range: (f64, f64)| {
        set_shared_x_view(&mut state.results.results.session, analysis, Some(range));
        timed_frame(&ctx, state)
    };
    let pans = (0..20)
        .map(|index| {
            let left = (f64::from(index) * 0.02).mul_add(span, x0);
            view(&mut state, (left, 0.01f64.mul_add(span, left)))
        })
        .collect();
    let anchor = 0.37f64.mul_add(span, x0);
    let zooms = (0..8)
        .map(|power| {
            let half = 0.5 * span / 10f64.powi(power);
            view(&mut state, (anchor - half, anchor + half))
        })
        .collect();

    let waveform = state
        .model
        .simulation
        .active_run()
        .expect("the run is active")
        .analyses[0]
        .waveforms[0]
        .clone();
    let (x, y) = (&waveform.as_ref().x, &waveform.as_ref().y);

    // Measure the middle half.
    let window = (0.25f64.mul_add(span, x0), 0.75f64.mul_add(span, x0));
    let started = Instant::now();
    let measured = measure_interval_values(x, y, 0..x.len(), Some(window))
        .expect("the stored run is measured");
    let measure = started.elapsed();

    // Step the cursor through rows from the middle.
    let middle = rows / 2;
    place_cursors(&mut state, Some(middle as f64 * STEP_S), None);
    let tokens = Tokens::default();
    let mut step = Duration::ZERO;
    for taken in 1..=STEPS {
        let started = Instant::now();
        let at = jump_cursor(
            &mut state.results_parts(),
            &tokens,
            CursorJump::Sample(Direction::Next),
        )
        .unwrap_or_else(|refusal| panic!("step {taken}: {refusal}"))
        .expect("a step lands on the frame");
        step = step.max(started.elapsed());
        assert_eq!(
            at.to_bits(),
            ((middle + taken) as f64 * STEP_S).to_bits(),
            "step {taken} lands on the next row"
        );
    }

    // Export the rows around the cursor, read from the store by window.
    let (source, _) = y.stored_window().expect("the signal is stored");
    let (axis, _) = x.stored_window().expect("the axis is stored");
    let around = StoredWindow {
        first: middle,
        rows: EXPORT_ROWS,
        lead: WindowLead::None,
    };
    let part = |source: &std::sync::Arc<dyn rspice_results::waveform::ColumnSource>| {
        WaveformValues::stored(std::sync::Arc::clone(source), around).expect("the rows exist")
    };
    let exported = RetainedTrace::new("V(s0)", part(axis), part(source), "#00aaff");
    let started = Instant::now();
    let text = rspice_formats::result_csv::encode_signal_tsv(
        state
            .model
            .simulation
            .active_run()
            .expect("the run is active")
            .dataset_id,
        "TRAN",
        exported.as_ref(),
    )
    .expect("the rows are exported");
    let export = started.elapsed();
    assert_eq!(
        text.lines().filter(|line| !line.starts_with('#')).count(),
        EXPORT_ROWS as usize + 1,
        "a header and every row"
    );

    let columns = columns.since();
    if beyond_budget {
        assert!(
            x.read_whole().is_err() && y.read_whole().is_err(),
            "a column larger than the memory budget is refused whole"
        );
    }
    let cost = RunCost {
        write,
        open,
        pans,
        zooms,
        measure,
        measured,
        step,
        export,
        exported_bytes: text.len(),
        columns,
    };
    drop(state);
    cost
}

/// Asserts what every stored run holds to, whatever its size: nothing read whole, and every
/// frame drew.
fn assert_drawn_from_the_store(cost: &RunCost) {
    assert_eq!(
        cost.columns.stored_reads, 0,
        "a stored column was read whole: {:?}",
        cost.columns
    );
    assert!(
        cost.frames().all(|frame| frame.cost.vertices > 0),
        "every frame drew"
    );
}

/// The twin of the run larger than memory: 256 MiB of samples, measured the same from the store
/// as in memory, bit for bit.
#[test]
fn a_stored_run_is_drawn_measured_stepped_and_exported_without_a_whole_read() {
    const ROWS: u64 = (256 << 20) / ROW_BYTES;
    let cost = exercise("twin", ROWS, 2 * ROWS * ROW_BYTES, false);
    cost.report("stored run twin", ROWS);
    assert_drawn_from_the_store(&cost);

    let x: Vec<f64> = (0..ROWS).map(|row| row as f64 * STEP_S).collect();
    let y: Vec<f64> = (0..ROWS).map(|row| sample(row, ROWS)).collect();
    let span = x[x.len() - 1] - x[0];
    let window = (0.25f64.mul_add(span, x[0]), 0.75f64.mul_add(span, x[0]));
    let in_memory = measure_interval(&x, &y, Some(window)).expect("measured in memory");
    let bits = |statistics: IntervalStatistics| {
        [
            statistics.min,
            statistics.max,
            statistics.mean,
            statistics.rms,
        ]
        .map(f64::to_bits)
    };
    assert_eq!(
        bits(cost.measured),
        bits(in_memory),
        "from the store {:?}, in memory {in_memory:?}",
        cost.measured
    );
}

/// A run of 10 GiB of samples, five times the reader's memory budget: refused whole, drawn,
/// measured, stepped through and exported from the store with the process's memory well under
/// the run's size, every pan and zoom frame inside the frame budget of an unoptimized build. Run
/// alone, so the peak working set is this run's: `cargo test -p rspice-app-results --lib
/// a_run_larger_than_memory -- --ignored --test-threads=1 --nocapture`.
#[test]
#[ignore = "writes a 10 GiB segment; run by hand"]
fn a_run_larger_than_memory_is_drawn_measured_stepped_and_exported_from_the_store() {
    const ROWS: u64 = (10 << 30) / ROW_BYTES;
    /// What the process may hold at its peak: the reader's 1 GiB budget and the block cache's
    /// working set, beside the test host's own.
    const MEMORY_CAP: u64 = 2 << 30;
    /// A frame's build and tessellation in the unoptimized test build, which runs several times
    /// slower than a shipped one.
    const FRAME_BUDGET: Duration = Duration::from_millis(100);
    let cost = exercise("ten-gib", ROWS, 20 << 30, true);
    cost.report("stored run of 10 GiB", ROWS);
    assert_drawn_from_the_store(&cost);
    assert!(
        cost.worst_frame() < FRAME_BUDGET,
        "the slowest pan or zoom frame took {:?}",
        cost.worst_frame()
    );
    let peak = peak_working_set().expect("the platform reports the peak working set");
    assert!(
        peak < MEMORY_CAP,
        "the peak working set, {peak} bytes, passed the cap of {MEMORY_CAP}"
    );
}
