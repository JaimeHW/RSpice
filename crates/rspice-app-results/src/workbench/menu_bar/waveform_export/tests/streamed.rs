//! A rawfile written as it is read: on a Results job, from a run kept in a results store, staged
//! beside its destination and put in its place only when it ends.
//!
//! The suite exports a run of 64 MiB of samples; the run of 10 GiB, past what the computer can
//! hold, is ignored and run by hand. Each writes its segment and its export under the target
//! directory and deletes them.

use super::*;

use std::path::Path;
use std::time::{Duration, Instant};

use crate::workbench::results_job::cancel_jobs_past_for_tests;
use crate::workbench::workflows::export_workflow::{
    NativeExportWorkflowIo, ObservedExportDestination,
};
use rspice_results::store_reader::{Segment, SegmentColumn, StoreReader};
use rspice_results::waveform::column_work::ColumnWork;
use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};
use rspice_results_store::{
    Axis, FileIo, RunMeta, RunStatus, Schema, SegmentWriter, Signal, SignalKind, available_bytes,
};

/// Index of the binary rawfile in the stored export choice.
const RAW_BINARY: usize = 10;
/// Rows pushed to the segment at a time.
const CHUNK_ROWS: u64 = 1 << 20;
/// The time between rows.
const STEP_S: f64 = 1.0e-9;

/// Sample `row`: a sine with noise drawn from the row, so no two blocks are alike.
fn sample(row: u64) -> f64 {
    let mut z = row.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let noise = (z >> 11) as f64 / (1_u64 << 53) as f64 - 0.5;
    0.1f64.mul_add(noise, (row as f64 * 1.0e-5).sin())
}

/// A folder of the target directory, deleted with what it holds when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        // The test binary runs from `<target>/debug/deps`.
        let folder = std::env::current_exe()
            .expect("the test binary has a path")
            .ancestors()
            .nth(3)
            .expect("the test binary is in the target directory")
            .join("streamed-export-tests")
            .join(format!("{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("the folder is made");
        Self(folder)
    }

    /// The names of the files the folder holds.
    fn files(&self) -> Vec<String> {
        let mut names = std::fs::read_dir(&self.0)
            .expect("the folder lists")
            .map(|entry| {
                entry
                    .expect("an entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();
        names
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A transient of `rows` rows written to a segment in `folder`, read back from it.
fn stored_transient(folder: &Path, rows: u64) -> AnalysisResult {
    let path = folder.join("transient.rsr");
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
        y.extend((first..end).map(sample));
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

/// Writes where the test says, as the native backend writes, and never opens a dialog.
struct FileExportIo(PathBuf);

impl ExportWorkflowIo for FileExportIo {
    fn show_save_dialog(&self, _: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        Ok(Some(self.0.clone()))
    }

    fn write_text_file(&self, path: &Path, _: &str) -> Result<(), String> {
        Err(format!("{} is written as it is read", path.display()))
    }

    fn observe_destination(&self, path: &Path) -> Result<ObservedExportDestination, String> {
        NativeExportWorkflowIo.observe_destination(path)
    }

    fn write_waveform_csv(
        &self,
        _: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        Err(format!("{} is written as it is read", path.display()))
    }

    fn writes_as_it_reads(&self) -> bool {
        true
    }
}

/// Start the binary rawfile export of what `state` shows, written at `path`.
fn start_raw_export(state: &mut ResultsTestHost, path: &Path) -> ResultExportJob {
    state
        .model
        .preferences
        .set_choice(
            crate::workbench::ChoicePreference::EngineeringExport,
            RAW_BINARY,
        )
        .expect("a rawfile preference");
    action_export_dataset_with_io(
        &mut state.results_parts(),
        &FileExportIo(path.to_path_buf()),
        &DatasetExportChoice::default(),
    )
    .unwrap_or_else(|| panic!("the export starts: {}", last_log_message(state)))
}

/// Wait for `export` to end and say how it ended.
fn finish(state: &mut ResultsTestHost, export: ResultExportJob, patience: Duration) {
    assert!(export.wait(patience), "the export ends");
    assert!(finish_export_job(&mut state.results_parts(), export).is_none());
}

/// Where a binary rawfile's values start: after the `Binary:` line.
fn values_start(file: &mut std::fs::File) -> u64 {
    use std::io::Read as _;
    let mut head = vec![0; 4096];
    let read = file.read(&mut head).expect("the header reads");
    let marker = b"Binary:\n";
    head[..read]
        .windows(marker.len())
        .position(|window| window == marker)
        .expect("a binary rawfile")
        .saturating_add(marker.len()) as u64
}

/// Row `row` of a binary rawfile of a time and one real signal, as written.
fn written_row(file: &mut std::fs::File, values: u64, row: u64) -> (f64, f64) {
    use std::io::{Read as _, Seek as _};
    let mut bytes = [0; 16];
    file.seek(std::io::SeekFrom::Start(values + row * 16))
        .expect("the row seeks");
    file.read_exact(&mut bytes).expect("the row reads");
    let value = |at: usize| f64::from_le_bytes(bytes[at..at + 8].try_into().expect("eight bytes"));
    (value(0), value(8))
}

/// A run kept in a results store exports as a rawfile written as it is read: on a job, a block of
/// each column at a time and never a column whole, every value bit for bit, and nothing staged
/// left beside it.
#[test]
fn a_stored_run_exports_as_a_rawfile_written_as_it_is_read() {
    // 64 MiB of samples: four mebirows of a time and one signal.
    const ROWS: u64 = 4 << 20;
    let scratch = Scratch::new("twin");
    let analysis = stored_transient(&scratch.0, ROWS);

    let work = ColumnWork::read();
    let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
    let facts = rspice_formats::spice_raw::result::write_result_raw(
        &analysis,
        &waveforms,
        &|_| false,
        rspice_core::io::RawFormat::Binary,
        crate::workbench::workflows::result_import_workflow::exported_waveform_limits(),
        &mut std::io::sink(),
    )
    .expect("the rawfile writes");
    assert_eq!((facts.points, facts.variables), (ROWS as usize, 2));
    assert_eq!(
        work.since().stored_reads,
        0,
        "no stored column is read whole"
    );

    let mut state = state_with_typed_result(analysis);
    let path = scratch.0.join("waveforms.raw");
    let export = start_raw_export(&mut state, &path);
    // The window's frames poll the export while it is written, and read no column.
    let work = ColumnWork::read();
    let started = Instant::now();
    let mut running = Some(export);
    while let Some(export) = running.take() {
        running = finish_export_job(&mut state.results_parts(), export);
        if running.is_some() {
            assert!(
                started.elapsed() < Duration::from_secs(600),
                "the export ends"
            );
            std::thread::sleep(Duration::from_millis(2));
        }
    }
    assert_eq!(work.since(), ColumnWork::default());
    assert!(
        last_log_message(&state).starts_with("Exported SPICE raw: "),
        "{}",
        last_log_message(&state)
    );
    assert_eq!(scratch.files(), ["transient.rsr", "waveforms.raw"]);

    let mut file = std::fs::File::open(&path).expect("the export opens");
    let values = values_start(&mut file);
    assert_eq!(
        file.metadata().expect("the export's size").len(),
        values + ROWS * 16
    );
    let mut header = vec![0; values as usize];
    {
        use std::io::{Read as _, Seek as _};
        file.rewind().expect("the export rewinds");
        file.read_exact(&mut header).expect("the header reads");
    }
    let header = String::from_utf8_lossy(&header);
    assert!(header.contains(&format!("No. Points: {ROWS}")), "{header}");
    assert!(header.contains("Flags: real forward double"), "{header}");
    let bytes = std::fs::read(&path).expect("the export reads");
    for (row, chunk) in bytes[values as usize..].chunks_exact(16).enumerate() {
        let row = row as u64;
        let time = f64::from_le_bytes(chunk[..8].try_into().expect("eight bytes"));
        let value = f64::from_le_bytes(chunk[8..].try_into().expect("eight bytes"));
        assert_eq!(
            (time.to_bits(), value.to_bits()),
            ((row as f64 * STEP_S).to_bits(), sample(row).to_bits()),
            "row {row}"
        );
    }
}

/// A cancelled export leaves the file it was writing as it was, and nothing staged beside it.
#[test]
fn a_cancelled_export_leaves_the_destination_as_it_was() {
    let scratch = Scratch::new("cancel");
    let mut state = state_with_typed_result(stored_transient(&scratch.0, 1 << 20));
    let path = scratch.0.join("waveforms.raw");
    std::fs::write(&path, b"the reader's own file").expect("the destination is written");

    cancel_jobs_past_for_tests(Some(4 << 20));
    let export = start_raw_export(&mut state, &path);
    cancel_jobs_past_for_tests(None);
    finish(&mut state, export, Duration::from_secs(120));

    assert_eq!(
        std::fs::read(&path).expect("the destination reads"),
        b"the reader's own file"
    );
    assert_eq!(scratch.files(), ["transient.rsr", "waveforms.raw"]);
    assert_eq!(
        refusal(&state),
        format!(
            "The SPICE raw export was cancelled; {} was left as it was.",
            path.display()
        )
    );
}

/// A run of 10 GiB of samples, past what the computer can hold, exports as a rawfile written as
/// it is read. Run by hand, alone; it needs about 16 GiB free on the target volume.
#[test]
#[ignore = "writes a 10 GiB segment and an 11 GB rawfile; run by hand"]
fn a_stored_run_larger_than_memory_exports_as_a_rawfile() {
    const ROWS: u64 = 671_088_640;
    let scratch = Scratch::new("beyond");
    let free = available_bytes(&scratch.0).expect("the target volume states its free space");
    assert!(
        free > 18 << 30,
        "{free} bytes free; the run needs about 16 GiB"
    );
    let started = Instant::now();
    let analysis = stored_transient(&scratch.0, ROWS);
    let written = started.elapsed();
    let mut state = state_with_typed_result(analysis);
    let path = scratch.0.join("waveforms.raw");

    let started = Instant::now();
    let export = start_raw_export(&mut state, &path);
    finish(&mut state, export, Duration::from_secs(3600));
    let exported = started.elapsed();
    assert!(
        last_log_message(&state).starts_with("Exported SPICE raw: "),
        "{}",
        last_log_message(&state)
    );

    let mut file = std::fs::File::open(&path).expect("the export opens");
    let values = values_start(&mut file);
    let size = file.metadata().expect("the export's size").len();
    assert_eq!(size, values + ROWS * 16);
    for row in [0, ROWS / 3, ROWS / 2 + 12_345, ROWS - 1] {
        let (time, value) = written_row(&mut file, values, row);
        assert_eq!(
            (time.to_bits(), value.to_bits()),
            ((row as f64 * STEP_S).to_bits(), sample(row).to_bits()),
            "row {row}"
        );
    }
    println!(
        "{ROWS} rows: segment written in {:.1} s; exported, {size} bytes, in {:.1} s",
        written.as_secs_f64(),
        exported.as_secs_f64()
    );
}
