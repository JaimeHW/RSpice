//! The disk watch: a run whose storage fills stops there, typed, with every whole chunk kept.

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::Parser;
use rspice_core::engine::{Engine, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_core::result_sink::SinkStop;
use rspice_results_store::{
    FileIo, RunStatus, SegmentReader, SegmentState, StopReason, StoreDir, StoreSink,
};

use super::{CLOSING_RESERVE, FreeSpace, WatchedIo};
use crate::cli::error::ExitCode;
use crate::cli::{Cli, CliError, Commands, Config, FailureCategory};

thread_local! {
    /// The size of the volume the stores this thread's runs create live on, when a test sets it.
    pub(super) static QUOTA: Cell<Option<u64>> = const { Cell::new(None) };
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice-store-watch-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A volume of `quota` bytes that holds only `segment`.
pub(super) fn quota(segment: &Path, quota: u64) -> FreeSpace {
    let segment = segment.to_path_buf();
    Arc::new(move || {
        let used = std::fs::metadata(&segment).map_or(0, |meta| meta.len());
        Some(quota.saturating_sub(used))
    })
}

#[test]
fn a_write_that_would_eat_the_closing_reserve_is_refused_and_the_closing_writes_are_not() {
    use rspice_results_store::SegmentIo;
    let dir = scratch("io");
    let segment = dir.join("segment");
    let mut io = WatchedIo {
        io: FileIo::create(&segment).unwrap(),
        free: quota(&segment, CLOSING_RESERVE + 100),
        full: false,
    };
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

    let mut sink = StoreSink::new(WatchedIo {
        io: FileIo::create(&segment).unwrap(),
        free: quota(&segment, CLOSING_RESERVE + 256 * 1024),
        full: false,
    });
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
fn a_storage_full_stop_exits_86_and_reads_back_from_a_run_report() {
    let error = CliError::StorageFull {
        message: "stopped".to_owned(),
        store: Some(PathBuf::from("out.rsr")),
    };
    assert_eq!(error.category(), FailureCategory::StorageFull);
    assert_eq!(error.exit_code(), ExitCode::StoppedStorageFull);
    assert_eq!(ExitCode::StoppedStorageFull as u8, 86);
    let details = error.details();
    assert_eq!(details.category, "storage_full");
    assert_eq!(details.path.as_deref(), Some("out.rsr"));
    assert_eq!(
        FailureCategory::parse(details.category),
        Some(FailureCategory::StorageFull)
    );
    assert!(
        error
            .to_string()
            .starts_with("stopped: storage full, partial results kept")
    );
}

/// Run `deck` from the command line with `extra` arguments on a volume of `quota` bytes; returns
/// how the command ended and its run summary.
fn run_on_quota(
    dir: &Path,
    deck: &str,
    extra: &[&str],
    quota: u64,
) -> (CliError, serde_json::Value) {
    let input = dir.join("deck.cir");
    std::fs::write(&input, deck).unwrap();
    let summary = dir.join("summary.json");
    let cli = Cli::try_parse_from(
        [
            "rspice",
            "run",
            input.to_str().unwrap(),
            "--summary",
            summary.to_str().unwrap(),
        ]
        .into_iter()
        .chain(extra.iter().copied()),
    )
    .unwrap();
    let Commands::Run(args) = cli.command else {
        panic!("expected the run command");
    };
    QUOTA.with(|cell| cell.set(Some(quota)));
    let ended = crate::commands::run(args, &Config::default(), false, true);
    QUOTA.with(|cell| cell.set(None));
    let error = ended.expect_err("the run stops when its storage fills");
    let summary = serde_json::from_str(&std::fs::read_to_string(&summary).unwrap()).unwrap();
    (error, summary)
}

#[test]
fn a_command_line_run_whose_storage_fills_exports_what_it_kept_and_exits_86_saying_so() {
    let dir = scratch("cli");
    let output = dir.join("out.csv");
    let (error, summary) = run_on_quota(
        &dir,
        "rc\nv1 in 0 sin(0 1 4meg)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 200u\n.end\n",
        &["-o", output.to_str().unwrap(), "-f", "csv"],
        CLOSING_RESERVE + 256 * 1024,
    );
    assert_eq!(error.exit_code(), ExitCode::StoppedStorageFull, "{error}");

    // The store beside the output closed over its whole chunks.
    let store = dir.join("out.rsr");
    let mut reader =
        SegmentReader::open(FileIo::open(&store.join(StoreDir::segment_name(0))).unwrap()).unwrap();
    assert!(matches!(
        reader.state(),
        SegmentState::Complete {
            status: RunStatus::Stopped(StopReason::DiskFull),
            ..
        }
    ));
    let rows = reader.rows();
    assert!(rows > 1000, "{rows} rows kept");
    let last = reader.read_x(rows - 1, 1).unwrap()[0];

    // The table holds the kept rows, the last one at the last kept time.
    let table = std::fs::read_to_string(&output).unwrap();
    let lines: Vec<&str> = table
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();
    assert_eq!(
        lines.len() as u64,
        rows + 1,
        "a header and one line per kept row"
    );
    let last_line_time: f64 = lines[lines.len() - 1]
        .split(',')
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert_eq!(last_line_time, last);

    // The summary says the run stopped for storage, when, and what it kept.
    let run = &summary["runs"][0];
    assert_eq!(run["passed"], false);
    let message = run["error"].as_str().unwrap();
    assert!(message.starts_with("stopped: storage full"), "{message}");
    assert!(
        message.contains(&format!("with {rows} points kept")),
        "{message}"
    );
    assert_eq!(run["error_details"]["category"], "storage_full");
    let stores = summary["stores"].as_array().unwrap();
    let kept = stores
        .iter()
        .find(|entry| entry["path"].as_str() == store.to_str())
        .unwrap();
    assert_eq!(kept["stopped"]["reason"], "disk_full");
    assert_eq!(kept["stopped"]["rows_kept"], rows);
    assert_eq!(kept["stopped"]["at_x"].as_f64(), Some(last));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_disk_that_fills_before_the_first_row_is_the_same_typed_stop_with_nothing_kept() {
    let dir = scratch("first");
    let output = dir.join("out.raw");
    // Room for the run's estimate, but not for the segment's own header past the reserve.
    let (error, summary) = run_on_quota(
        &dir,
        "rc\nv1 in 0 sin(0 1 100k)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 200u\n.end\n",
        &["-o", output.to_str().unwrap()],
        CLOSING_RESERVE + 100,
    );
    assert_eq!(error.exit_code(), ExitCode::StoppedStorageFull, "{error}");
    let message = summary["runs"][0]["error"].as_str().unwrap();
    assert!(
        message.starts_with("stopped: storage full") && message.contains("with 0 points kept"),
        "{message}"
    );
    assert!(!output.exists() && !dir.join("out.rsr").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_completed_run_whose_output_fills_the_disk_keeps_its_whole_store_and_exits_86() {
    use crate::commands::publish::OUTPUT_QUOTA;
    let dir = scratch("output");
    let input = dir.join("deck.cir");
    std::fs::write(
        &input,
        "rc\nv1 in 0 sin(0 1 4meg)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 20u\n.end\n",
    )
    .unwrap();
    let output = dir.join("out.csv");
    let summary = dir.join("summary.json");
    // `--no-store` asks for the store to go once the output is written; it never is.
    let cli = Cli::try_parse_from([
        "rspice",
        "run",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "-f",
        "csv",
        "--no-store",
        "--summary",
        summary.to_str().unwrap(),
    ])
    .unwrap();
    let Commands::Run(args) = cli.command else {
        panic!("expected the run command");
    };
    // Room for the summary, not for the table.
    OUTPUT_QUOTA.with(|cell| cell.set(Some(64 * 1024)));
    let ended = crate::commands::run(args, &Config::default(), false, true);
    OUTPUT_QUOTA.with(|cell| cell.set(None));
    let error = ended.expect_err("the run stops when its output fills the disk");
    assert_eq!(error.exit_code(), ExitCode::StoppedStorageFull, "{error}");
    assert_eq!(error.category(), FailureCategory::StorageFull);
    // The table is published whole or not at all.
    assert!(!output.exists());

    // The store beside the output holds the whole completed run.
    let store = dir.join("out.rsr");
    let mut reader =
        SegmentReader::open(FileIo::open(&store.join(StoreDir::segment_name(0))).unwrap()).unwrap();
    assert!(matches!(
        reader.state(),
        SegmentState::Complete {
            status: RunStatus::Completed,
            ..
        }
    ));
    let rows = reader.rows();
    assert!(rows > 1000, "{rows} rows kept");
    let last = reader.read_x(rows - 1, 1).unwrap()[0];
    assert!((last - 20e-6).abs() < 1e-15, "the run ended at {last}");

    let summary: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&summary).unwrap()).unwrap();
    let run = &summary["runs"][0];
    assert_eq!(run["passed"], false);
    let message = run["error"].as_str().unwrap();
    assert!(
        message.starts_with("stopped: storage full")
            && message.contains("writing its output filled the disk")
            && message.contains(&format!("all {rows} points are kept")),
        "{message}"
    );
    assert_eq!(run["error_details"]["category"], "storage_full");
    let kept = summary["stores"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["path"].as_str() == store.to_str())
        .unwrap();
    assert_eq!(kept["stopped"]["reason"], "output_disk_full");
    assert_eq!(kept["stopped"]["rows_kept"], rows);
    assert_eq!(kept["stopped"]["at_x"].as_f64(), Some(last));
    let _ = std::fs::remove_dir_all(&dir);
}
