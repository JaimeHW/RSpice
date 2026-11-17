use std::sync::Arc;
use std::time::{Duration, SystemTime};

use rspice_results::store_reader::{SegmentBytes, StoreReader};
use rspice_results::waveform::{ColumnSource, StoredWindow, WaveformValues, WindowLead};
use rspice_results_store::{
    RunMeta, RunStatus, RunWriter, Schema, SegmentReader, Signal, SignalKind,
};

use super::*;

/// The free space a segment's disk watch reads, given the segment.
pub(crate) type SegmentFreeSpace = Box<dyn Fn(&Path) -> rspice_results_store::FreeSpace>;

/// Free space of `quota` bytes less what the segment holds, as a volume of that size would have.
pub(crate) fn quota(quota: u64) -> SegmentFreeSpace {
    Box::new(move |segment: &Path| {
        let segment = segment.to_path_buf();
        Arc::new(move || {
            Some(quota.saturating_sub(fs::metadata(&segment).map_or(0, |meta| meta.len())))
        })
    })
}

thread_local! {
    /// The free space a run on this thread's disk watch reads instead of its volume's.
    pub(crate) static FREE_SPACE: std::cell::RefCell<Option<SegmentFreeSpace>> =
        const { std::cell::RefCell::new(None) };
}

/// An empty folder of the test's own under the temporary folder.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "rspice-results-store-test-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_sweep_deletes_only_folders_no_live_process_holds() {
    let root = scratch("sweep");
    let folder = |name: &str| {
        let dir = root.join(name);
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("transient-0.rsr"), b"rows").unwrap();
        dir
    };
    // A live process: its lock is held.
    let live = folder("1-a-0");
    let held = File::create(lock_path(&live)).unwrap();
    held.try_lock().unwrap();
    // A process that ended: its lock is free.
    let ended = folder("2-b-0");
    File::create(lock_path(&ended)).unwrap();
    // A sweep that died between deleting a lock and its folder.
    let orphan = folder("3-c-0");

    let now = SystemTime::now();
    assert!(
        sweep(&root, now).is_empty(),
        "what was just made may be a process still making it"
    );
    let later = now + Duration::from_secs(120);
    let mut deleted = sweep(&root, later);
    deleted.sort();
    assert_eq!(deleted, vec![ended.clone(), orphan.clone()]);
    assert!(live.join("transient-0.rsr").exists() && lock_path(&live).exists());
    assert!(!ended.exists() && !lock_path(&ended).exists() && !orphan.exists());
    drop(held);
    assert_eq!(sweep(&root, later), vec![live.clone()]);
    assert!(!live.exists());
    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn a_process_holds_its_store_folder_and_a_sweep_keeps_it() {
    let (path, io) = create_segment().unwrap();
    drop(io);
    let dir = path.parent().unwrap().to_owned();
    let deleted = sweep(&stores_root(), SystemTime::now() + Duration::from_secs(120));
    assert!(!deleted.contains(&dir));
    assert!(path.exists());
    fs::remove_file(path).unwrap();
}

#[test]
fn a_stored_column_reads_any_window_across_chunks_and_its_segment_goes_with_the_last_reader() {
    let mut signal = Signal::new("out", "V", SignalKind::Real);
    signal.quantity = "voltage".to_owned();
    let schema = Schema {
        x: Default::default(),
        signals: vec![signal],
    };
    let (path, io) = create_segment().unwrap();
    let mut writer = RunWriter::create(io, &schema, &RunMeta::default()).unwrap();
    let rows = 10_000u64;
    for row in 0..rows {
        writer.push_row(row as f64, &[row as f64 * 0.5]).unwrap();
    }
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let reader = SegmentReader::open(Box::new(io.into_file()) as SegmentBytes).unwrap();
    assert!(
        reader.chunk_spans().len() > 2,
        "the rows span several chunks"
    );
    let run = Arc::new(StoredRun::temporary(StoreReader::new(reader), path.clone()));
    let time = StoredColumn::new(run.clone(), None, "time".to_owned());
    let out = StoredColumn::new(run.clone(), Some(0), "v(out)".to_owned());
    drop(run);
    let window = |first, rows| StoredWindow {
        first,
        rows,
        lead: WindowLead::None,
    };
    let axis = WaveformValues::stored(time, window(4000, 5000)).unwrap();
    let values = WaveformValues::stored(out.clone(), window(1, rows - 1)).unwrap();
    assert_eq!(axis.len(), 5000);
    assert!(
        axis.iter()
            .enumerate()
            .all(|(index, &x)| x == (4000 + index) as f64)
    );
    assert!(
        values
            .iter()
            .enumerate()
            .all(|(index, &y)| y == (1 + index) as f64 * 0.5)
    );
    let mut past = vec![0.0; 2];
    assert!(out.read(rows - 1, &mut past).is_err());
    drop((axis, values, out));
    assert!(!path.exists(), "the last column took its segment with it");
}

#[test]
fn a_stored_waveform_tiles_fits_and_reads_cursors_from_its_store_as_from_its_values() {
    let schema = Schema {
        x: Default::default(),
        signals: vec![Signal::new("out", "V", SignalKind::Real)],
    };
    let (path, io) = create_segment().unwrap();
    let mut writer = RunWriter::create(io, &schema, &RunMeta::default()).unwrap();
    let rows = 60_000u64;
    let time = |row: u64| row as f64 * 1.0e-6;
    let value = |row: u64| (row as f64 * 2.0e-4).sin() * 3.0 - 1.0;
    for row in 0..rows {
        writer.push_row(time(row), &[value(row)]).unwrap();
    }
    let (_, io) = writer.finish(RunStatus::Completed).unwrap();
    let reader = SegmentReader::open(Box::new(io.into_file()) as SegmentBytes).unwrap();
    let run = Arc::new(StoredRun::temporary(StoreReader::new(reader), path));
    // An output window from 1.5 us past row 100: a lead between rows 100 and 101, as the
    // transient's own output start makes one.
    let start = time(100) + 0.5e-6;
    let window = |lead| StoredWindow {
        first: 101,
        rows: rows - 101,
        lead,
    };
    let axis = WaveformValues::stored(
        StoredColumn::new(run.clone(), None, "time".to_owned()),
        window(WindowLead::Value(start)),
    )
    .unwrap();
    let out = WaveformValues::stored(
        StoredColumn::new(run.clone(), Some(0), "out".to_owned()),
        window(WindowLead::Interpolated(0.5)),
    )
    .unwrap();
    assert!(axis.is_ascending_axis() && !out.is_ascending_axis());
    assert_eq!(axis.ends(), Some((start, time(rows - 1))));

    // What the same samples give read whole, from copies that do not share the windows.
    let (x, y) = (axis.to_vec(), out.to_vec());
    let linear = rspice_results::sampling::SampleInterpolation::Linear;
    let cubic = rspice_results::sampling::SampleInterpolation::MonotoneCubic;
    for at in [
        0.0,
        start,
        start + 0.2e-6,
        time(4_096) + 0.3e-6,
        0.03,
        time(rows - 1),
        1.0,
    ] {
        for interpolation in [linear, cubic] {
            let whole = rspice_results::sampling::sample_at_with(&x, &y, at, interpolation);
            let read = out.value_at(&axis, at, interpolation).unwrap();
            assert_eq!(read.to_bits(), whole.to_bits(), "{at} by {interpolation:?}");
        }
    }

    let extent = out.extent_over(&axis, None).unwrap();
    let finite = |values: &[f64]| {
        values
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
                (lo.min(v), hi.max(v))
            })
    };
    assert_eq!((extent.min, extent.max), finite(&y));
    let (x0, x1) = (0.01, 0.02);
    let inside: Vec<f64> = x
        .iter()
        .zip(&y)
        .filter(|(t, _)| (x0..=x1).contains(*t))
        .map(|(_, v)| *v)
        .collect();
    let windowed = out.extent_over(&axis, Some((x0, x1))).unwrap();
    assert_eq!((windowed.min, windowed.max), finite(&inside));

    // The lead is the first pixel's: a tile of the window's start holds it.
    let Some(rspice_results::waveform::ColumnTile::Samples { x: xs, y: ys, .. }) =
        out.tile_over(&axis, 0.0, time(103), 400)
    else {
        panic!("a few rows tile as samples");
    };
    assert_eq!(xs, &x[..4]);
    assert_eq!(ys, &y[..4]);
    assert!(out.tile_over(&axis, 0.0, 0.05, 400).is_some());
    // A window over other rows of the axis is not this waveform's axis: read the values instead.
    let other = WaveformValues::stored(
        StoredColumn::new(run, None, "time".to_owned()),
        StoredWindow {
            first: 0,
            rows: 10,
            lead: WindowLead::None,
        },
    )
    .unwrap();
    assert!(out.tile_over(&other, 0.0, 1.0, 10).is_none());
}
