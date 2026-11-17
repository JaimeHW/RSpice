//! The disk watch: a run whose storage fills stops there, typed, with every whole chunk kept.

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::Parser;
use rspice_results_store::{
    CLOSING_RESERVE, FileIo, FreeSpace, RunStatus, SegmentReader, SegmentState, StopReason,
    StoreDir,
};

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

#[test]
fn a_transient_read_back_whole_is_bounded_by_its_store_not_the_result_value_cap() {
    let dir = scratch("cap");
    let input = dir.join("deck.cir");
    // A measure reads the run back whole, projected and measured in memory.
    std::fs::write(
        &input,
        "rc\nv1 in 0 sin(0 1 4meg)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 20u\n\
         .meas tran vmax max v(out)\n.end\n",
    )
    .unwrap();
    let output = dir.join("out.csv");
    let cli = Cli::try_parse_from([
        "rspice",
        "run",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "-f",
        "csv",
    ])
    .unwrap();
    let Commands::Run(args) = cli.command else {
        panic!("expected the run command");
    };
    let mut config = Config::default();
    config.resources.max_result_values = 1000;
    crate::commands::run(args, &config, false, true)
        .expect("a transient's rows are bounded by its store's disk");
    let lines = std::fs::read_to_string(&output)
        .unwrap()
        .lines()
        .filter(|line| !line.starts_with('#'))
        .count();
    assert!(lines > 1000, "{lines} lines");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A resumed run appends to the store beside its output; when its storage fills, that store keeps
/// the earlier segment's rows and the resumed rows that fit, closed as stopped.
#[test]
fn a_resume_whose_storage_fills_keeps_the_store_it_appended_to() {
    let dir = scratch("resume");
    let input = dir.join("deck.cir");
    std::fs::write(
        &input,
        "rc\nv1 in 0 sin(0 1 4meg)\nr1 in out 1k\nc1 out 0 1n\n.tran 1n 200u\n.end\n",
    )
    .unwrap();
    let (output, state) = (dir.join("out.csv"), dir.join("state.ckpt"));
    let args = |extra: &[&str]| {
        let cli = Cli::try_parse_from(
            [
                "rspice",
                "--quiet",
                "run",
                input.to_str().unwrap(),
                "-o",
                output.to_str().unwrap(),
                "-f",
                "csv",
            ]
            .into_iter()
            .chain(extra.iter().copied()),
        )
        .unwrap();
        let Commands::Run(args) = cli.command else {
            panic!("expected the run command");
        };
        args
    };
    crate::commands::run(
        args(&[
            "--tran-stop",
            "20u",
            "--checkpoint",
            state.to_str().unwrap(),
        ]),
        &Config::default(),
        false,
        true,
    )
    .expect("the first segment runs");
    let segment = dir.join("out.rsr").join(StoreDir::segment_name(0));
    let (first_rows, first_bytes) = {
        let mut reader = SegmentReader::open(FileIo::open(&segment).unwrap()).unwrap();
        let rows = reader.rows();
        assert_eq!(reader.read_x(rows - 1, 1).unwrap()[0], 20e-6);
        (rows, std::fs::metadata(&segment).unwrap().len())
    };

    QUOTA.with(|cell| cell.set(Some(first_bytes + CLOSING_RESERVE + 256 * 1024)));
    let ended = crate::commands::run(
        args(&["--resume", state.to_str().unwrap()]),
        &Config::default(),
        false,
        true,
    );
    QUOTA.with(|cell| cell.set(None));
    let error = ended.expect_err("the resumed run stops when its storage fills");
    assert_eq!(error.exit_code(), ExitCode::StoppedStorageFull, "{error}");

    let mut reader = SegmentReader::open(FileIo::open(&segment).unwrap()).unwrap();
    assert!(matches!(
        reader.state(),
        SegmentState::Complete {
            status: RunStatus::Stopped(StopReason::DiskFull),
            ..
        }
    ));
    let rows = reader.rows();
    assert!(rows > first_rows, "{rows} rows kept after {first_rows}");
    assert_eq!(reader.read_x(first_rows - 1, 1).unwrap()[0], 20e-6);
    let _ = std::fs::remove_dir_all(&dir);
}

/// A read-back the host cannot give memory for ends the run with the typed allocation failure
/// that names the bytes it needed, as a refusal while solving does, and not as a damaged store.
#[test]
fn a_read_back_the_host_cannot_hold_is_the_typed_allocation_failure() {
    let source = Vec::<u8>::new()
        .try_reserve(usize::MAX)
        .expect_err("no host has usize::MAX bytes to give");
    let error = super::store_error(
        Path::new("out.rsr"),
        rspice_results_store::StoreError::Allocation {
            object: "the rows read back from a results segment",
            bytes: 8_000_000_000,
            source,
        },
    );
    let details = error.details();
    assert_eq!(details.code, "allocation_failed", "{error}");
    assert_eq!(details.category, "resource_limit", "{error}");
    assert!(error.to_string().contains("8000000000 bytes"), "{error}");
}

/// A `.TRAN 0 1` names no print step: the run likely writes the rows its maximum step writes, a
/// count the preflight can make, and not the `u64::MAX` bytes a zero step gave. A count past
/// what the preflight can make is no estimate, and warns of nothing.
#[test]
fn a_transient_without_a_print_step_estimates_its_store_from_its_maximum_step() {
    let span = |tstep| super::StoreSpan {
        start: 0.0,
        tstop: 1.0,
        tstep,
        max_step: 1.0 / 1024.0,
    };
    assert_eq!(span(0.0).rows(), (1024.0, 1024.0));
    assert_eq!(span(1.0e-4).rows(), (1024.0, 10_000.0));
    assert_eq!(span(1.0).rows(), (1024.0, 1024.0));
    let likely = rspice_results_store::WatchedSink::<FileIo>::bytes(span(0.0).rows().1, 3);
    assert_eq!(likely, 33_915);
    let warning = super::space_warning(likely, 1_000).expect("a run that likely overfills warns");
    assert!(warning.contains("about 33915 bytes"), "{warning}");
    assert_eq!(super::space_warning(u64::MAX, 1_000), None);
}

/// A `D(node)` or `E(node)` column a cursor samples from its event trace is the projection's,
/// value for value: a change a last bit after a row's time counts at that row, a level holds
/// to the next change, an unknown level reads 0.5, and a real value reads 0.0 before its first
/// change.
#[test]
fn an_event_column_samples_its_trace_as_the_projection_does() {
    use rspice_core::analysis::transient::{
        DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint, TransientResult,
    };
    use rspice_core::execution::{HeldEvents, ProjectionValues, transient_projection_signals};
    use rspice_core::xspice::DigitalValue;

    let time = vec![0.0, 1.0e-9, 2.0e-9, 3.0e-9, 4.0e-9, 5.0e-9];
    let digital = |time, value| DigitalTracePoint { time, value };
    let real = |time, value| RealTracePoint { time, value };
    let result = TransientResult {
        instance_unknowns: Vec::new(),
        potential_units: Vec::new(),
        voltage_impulses: None,
        current_impulses: None,
        step_sizes: vec![0.0; time.len()],
        time,
        voltages: Vec::new(),
        branch_currents: Vec::new(),
        num_nodes: 0,
        node_names: Vec::new(),
        branch_names: Vec::new(),
        digital_traces: vec![DigitalTrace {
            node_name: "q".to_owned(),
            points: vec![
                digital(0.5e-9, DigitalValue::one()),
                digital(2.0e-9 + 5.0e-19, DigitalValue::zero()),
                digital(3.5e-9, DigitalValue::unknown()),
                digital(3.6e-9, DigitalValue::one()),
            ],
            hdl_time_precision: None,
        }],
        digital_buses: Vec::new(),
        real_traces: vec![RealTrace {
            node_name: "r".to_owned(),
            points: vec![real(1.5e-9, 2.5), real(4.0e-9, -1.0), real(9.0e-9, 7.0)],
        }],
        hdl_sources: Vec::new(),
        event_only_nodes: Vec::new(),
        device_op_traces: Vec::new(),
        store_traces: Vec::new(),
        fft_results: Vec::new(),
    };
    let events = HeldEvents {
        digital: &result.digital_traces,
        real: &result.real_traces,
    };
    let projected = transient_projection_signals(&result).expect("the result projects");
    for (name, column) in [
        ("D(q)", super::Column::Digital(0)),
        ("E(r)", super::Column::Real(0)),
    ] {
        assert_eq!(super::event_column(&result, name), Some(column));
        let signal = projected
            .iter()
            .find(|signal| signal.descriptor().display_name() == name)
            .expect("the projection has the column");
        let ProjectionValues::Real(expected) = signal.values() else {
            panic!("{name} is a real column");
        };
        let mut cursor = super::EventCursor::open(&events, column).expect("an event column");
        let sampled = result
            .time
            .iter()
            .map(|&time| cursor.at(time))
            .collect::<Result<Vec<_>, _>>()
            .expect("samples");
        assert_eq!(
            sampled
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            "{name}"
        );
    }
    assert_eq!(super::event_column(&result, "D(r)"), None);
    assert_eq!(super::event_column(&result, "V(q)"), None);
}
