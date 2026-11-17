use std::sync::Arc;
use std::time::{Duration, SystemTime};

use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};
use rspice_results_store::{RunMeta, RunStatus, RunWriter, Schema, Signal, SignalKind};

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
    let reader = SegmentReader::open(io.into_file()).unwrap();
    assert!(
        reader.chunk_spans().len() > 2,
        "the rows span several chunks"
    );
    let run = Arc::new(StoredRun::new(reader, path.clone()));
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
