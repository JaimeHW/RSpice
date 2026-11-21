use std::sync::{Arc, Mutex};

use rspice_core::abort_signal::{AbortSignal, TransientSample};
use rspice_results::sampling::SampleInterpolation;
use rspice_simulation_contract::config::{AnalysisConfig, TransientAnalysisConfig};

use super::*;
use crate::results::SimulationResult;

/// A step into an RC low-pass of a 1 ms time constant, solved in steps of at most 0.5 us, so the
/// run writes several chunks: V(out) = 1 - exp(-t / 1 ms).
const DECK: &str = "live segment
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 1u
.tran 0.5u 5m 0 0.5u
.end
";

const TAU: f64 = 1e-3;

/// Opens the run's live segment at chosen accepted points, as the host does while it runs.
struct Opener {
    slot: crate::results_store::LiveSegmentSlot,
    seen: Mutex<Vec<LiveRows>>,
}

impl AbortSignal for Opener {
    fn is_aborted(&self) -> bool {
        false
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        if !sample.points.is_multiple_of(2_500) {
            return;
        }
        let path = self.slot.lock().unwrap().clone();
        let path = path.expect("the running transient names its segment");
        if let Some(rows) = open_live_rows(&path).expect("a growing segment opens") {
            self.seen.lock().unwrap().push(rows);
        }
    }
}

fn values(values: &WaveformValues) -> Vec<f64> {
    values.read_whole().unwrap().into_owned()
}

#[test]
fn a_running_transient_is_drawn_from_the_rows_its_segment_holds() {
    let slot = crate::results_store::LiveSegmentSlot::default();
    let _installed = crate::results_store::install_live_segment(Arc::clone(&slot));
    let opener = Opener {
        slot: Arc::clone(&slot),
        seen: Mutex::new(Vec::new()),
    };
    let config = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 5e-3,
        step_time: 0.5e-6,
        start_time: 0.0,
        max_timestep: Some(0.5e-6),
        uic: false,
    });
    let result = super::super::super::EngineBridge::new()
        .run_with_abort(&config, DECK, &opener)
        .unwrap();
    assert!(
        slot.lock().unwrap().is_none(),
        "the slot is cleared once the run is done writing"
    );
    let SimulationResult::Transient {
        time, waveforms, ..
    } = result
    else {
        panic!("a transient");
    };
    let final_time = values(&time);
    let final_out = values(
        &waveforms
            .iter()
            .find(|(name, _)| {
                name.eq_ignore_ascii_case("out") || name.eq_ignore_ascii_case("V(out)")
            })
            .unwrap_or_else(|| panic!("V(out) among {:?}", waveforms.keys().collect::<Vec<_>>()))
            .1
            .y_values,
    );
    let seen = opener.seen.into_inner().unwrap();
    assert!(
        seen.iter().any(|live| live.rows > 0),
        "the rows are readable while the run writes them"
    );
    let mut previous = 0;
    for live in &seen {
        assert!(live.rows >= previous, "the segment only grows");
        previous = live.rows;
        let out = live
            .waveforms
            .iter()
            .find(|waveform| {
                waveform.name.eq_ignore_ascii_case("out")
                    || waveform.name.eq_ignore_ascii_case("V(out)")
            })
            .expect("each saved node is a waveform");
        assert_eq!(out.unit.as_deref(), Some("V"));
        assert!(out.x.is_stored() && out.y.is_stored());
        let x = values(&out.x);
        let y = values(&out.y);
        assert_eq!(x.len() as u64, live.rows);
        assert_eq!(x.last().copied(), Some(live.last_time));
        // The rows written so far are the run's own first rows, value for value.
        assert_eq!(x[..], final_time[..x.len()]);
        assert_eq!(y[..], final_out[..y.len()]);
        for (&t, &v) in x.iter().zip(&y).skip(1) {
            let expected = 1.0 - (-(t - 1e-9).max(0.0) / TAU).exp();
            assert!(
                (v - expected).abs() < 1e-4,
                "V(out) at {t}: {v} vs {expected}"
            );
        }
        // A view over every row reaches the last row written, which the segment's last chunk
        // holds: the charging curve's greatest value is its last.
        let (first, last) = (x[0], *x.last().unwrap());
        match out
            .y
            .tile_over(&out.x, first, last, 16)
            .expect("a stored column tiles")
        {
            ColumnTile::Envelope { max, .. } => {
                assert_eq!(max.last().copied(), y.last().copied());
            }
            ColumnTile::Samples { y: tiled, .. } => {
                assert_eq!(tiled.last().copied(), y.last().copied());
            }
        }
        // A cursor between the last two rows reads between them.
        if x.len() >= 2 {
            let n = x.len();
            let at = 0.5 * (x[n - 2] + x[n - 1]);
            let read = out
                .y
                .value_at(&out.x, at, SampleInterpolation::Linear)
                .expect("a stored column reads at a cursor");
            assert!((read - 0.5 * (y[n - 2] + y[n - 1])).abs() < 1e-12);
        }
    }
}

/// The same low-pass solved in steps of 0.5 ns for 1 ms: two million points.
const LONG_DECK: &str = "live segment of two million points
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 1u
.tran 0.5n 1m 0 0.5n
.end
";

/// Set in the process the memory test runs alone in.
const ALONE: &str = "RSPICE_LIVE_SEGMENT_MEMORY_ALONE";

/// Draws the run's rows through tiles every 400,000 points, as the host's plot does, and keeps
/// the largest peak working set seen while it ran.
struct Drawer {
    slot: crate::results_store::LiveSegmentSlot,
    drawn: Mutex<Vec<(u64, f64)>>,
}

impl AbortSignal for Drawer {
    fn is_aborted(&self) -> bool {
        false
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        if !sample.points.is_multiple_of(400_000) {
            return;
        }
        let path = self.slot.lock().unwrap().clone();
        let path = path.expect("the running transient names its segment");
        let Some(live) = open_live_rows(&path).expect("a growing segment opens") else {
            return;
        };
        let out = live
            .waveforms
            .iter()
            .find(|waveform| {
                waveform.name.eq_ignore_ascii_case("out")
                    || waveform.name.eq_ignore_ascii_case("V(out)")
            })
            .expect("V(out) is a waveform");
        let (first, last) = out.x.ends().expect("rows were written");
        // Every row the segment holds, through the tiles of a wide plot: the charging curve
        // rises, so its least value is its first and its greatest its last, and a plot this
        // dense touches every pixel.
        match out.y.tile_over(&out.x, first, last, 2_048) {
            Some(ColumnTile::Envelope { min, max, touched }) => {
                assert!(touched.iter().all(|touched| *touched));
                let top = max.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let bottom = min.iter().copied().fold(f64::INFINITY, f64::min);
                let expected = 1.0 - (-(last - 1e-9).max(0.0) / TAU).exp();
                assert!((top - expected).abs() < 1e-4, "{top} vs {expected}");
                assert!(bottom.abs() < 1e-6);
            }
            other => panic!("a view of {} rows is an envelope: {other:?}", live.rows),
        }
        self.drawn
            .lock()
            .unwrap()
            .push((live.rows, peak_working_set().unwrap_or(0) as f64));
    }
}

#[test]
fn a_two_million_point_transient_is_drawn_from_its_segment_with_memory_flat() {
    if std::env::var_os(ALONE).is_none() {
        // Alone in a process of its own, so the peak working set is this run's and no other
        // test's.
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "engine_bridge::transient::live::tests::\
                 a_two_million_point_transient_is_drawn_from_its_segment_with_memory_flat",
                "--nocapture",
                "--test-threads",
                "1",
            ])
            .env(ALONE, "1")
            .output()
            .expect("the test binary runs again");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{stdout}");
        assert!(
            output.status.success() && stdout.contains("1 passed"),
            "{stdout}\n{stderr}"
        );
        return;
    }
    let slot = crate::results_store::LiveSegmentSlot::default();
    let _installed = crate::results_store::install_live_segment(Arc::clone(&slot));
    let drawer = Drawer {
        slot: Arc::clone(&slot),
        drawn: Mutex::new(Vec::new()),
    };
    let config = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 1e-3,
        step_time: 0.5e-9,
        start_time: 0.0,
        max_timestep: Some(0.5e-9),
        uic: false,
    });
    let before = peak_working_set().expect("the platform reports the peak working set");
    let result = super::super::super::EngineBridge::new()
        .run_with_abort(&config, LONG_DECK, &drawer)
        .unwrap();
    let after = peak_working_set().expect("the platform reports the peak working set");
    let SimulationResult::Transient { time, .. } = &result else {
        panic!("a transient");
    };
    let rows = time.len() as u64;
    assert!(rows >= 2_000_000, "{rows} rows");
    let drawn = drawer.drawn.into_inner().unwrap();
    assert!(drawn.len() >= 4, "drawn {} times", drawn.len());
    // Time and the three columns a row of this circuit holds, each eight bytes.
    let rows_bytes = rows * 4 * 8;
    let grew = after.saturating_sub(before);
    // Flat: from the first drawing to the run's end, the samples written grow the peak by
    // little, however many there are.
    let (first_rows, first_peak) = drawn[0];
    let written_since = (rows - first_rows) * 4 * 8;
    let grew_since = after.saturating_sub(first_peak as u64);
    println!(
        "{rows} rows ({} MiB of samples): peak working set {} MiB before the run, {} MiB after \
         it; at each drawing (rows, MiB) {:?}; grew {} MiB over the run and {} MiB over the \
         last {} MiB of samples",
        rows_bytes >> 20,
        before >> 20,
        after >> 20,
        drawn
            .iter()
            .map(|(rows, peak)| (*rows, (*peak as u64) >> 20))
            .collect::<Vec<_>>(),
        grew >> 20,
        grew_since >> 20,
        written_since >> 20,
    );
    assert!(
        grew_since < written_since / 8,
        "the peak grew {} MiB while the run wrote {} MiB of samples",
        grew_since >> 20,
        written_since >> 20
    );
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
