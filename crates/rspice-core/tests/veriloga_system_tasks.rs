//! Verilog-AMS system tasks and functions in the analog context (LRM 2.4
//! sections 9.4 to 9.13), run by the engine.

#![cfg(feature = "veriloga")]

#[path = "common/model_outcome.rs"]
mod model_outcome;

use model_outcome::RunWithOutcome;
use rspice_core::config::{HdlOutputCollector, HdlOutputRecord, HdlStream};
use rspice_core::{Engine, Netlist, NoAbort, SimulationOutcome};
use std::path::PathBuf;
use std::sync::Arc;

/// A scratch directory of its own for each test.
fn scratch(name: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!(
        "rspice_va_system_tasks_{name}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

/// Run a transient of `body` in a one-port module across a ramp source, in
/// `directory`, and return the outcome and every HDL record.
fn run(
    directory: &std::path::Path,
    body: &str,
    stop: f64,
) -> (
    Result<SimulationOutcome<rspice_core::engine::TransientResult>, String>,
    Vec<HdlOutputRecord>,
) {
    let model = directory.join("tasks.va");
    std::fs::write(
        &model,
        format!(
            "`include \"disciplines.vams\"\nmodule tasks(x); inout x; electrical x;\n\
             integer fd, rc, k, seed, n; real r;\nanalog begin\n{body}\nI(x) <+ V(x) / 1k;\nend\nendmodule\n"
        ),
    )
    .unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        format!(
            "* system tasks\nvx s 0 pwl(0 0 {stop} 1)\nr1 s x 1k\nxa x tasks\n.hdl \"tasks.va\"\n.end\n"
        ),
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.to_path_buf());
    let outcome = Engine::new(config)
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, stop, stop / 100.0, signal)
        })
        .map_err(|error| error.to_string());
    (outcome, collector.take())
}

fn stdout(records: &[HdlOutputRecord]) -> Vec<String> {
    records
        .iter()
        .filter(|record| record.stream == HdlStream::Stdout)
        .map(|record| record.text.clone())
        .collect()
}

/// `$debug` and `$fdisplay` to the standard output print on every evaluation
/// of an accepted point, and `$fdisplay` to the standard error lands there.
#[test]
fn debug_and_standard_stream_file_output_print_from_accepted_points() {
    let directory = scratch("streams");
    let (outcome, records) = run(
        &directory,
        "@(initial_step) begin $debug(\"debug\"); $fdisplay(32'h8000_0001, \"out %0d\", 7); \
         $fwrite(32'h8000_0002, \"err\"); end",
        1e-9,
    );
    outcome.unwrap();
    let printed = stdout(&records);
    assert!(printed.iter().any(|text| text == "debug\n"), "{printed:?}");
    assert!(printed.iter().any(|text| text == "out 7\n"), "{printed:?}");
    assert!(
        records
            .iter()
            .any(|record| record.stream == HdlStream::Stderr && record.text == "err"),
        "{records:?}"
    );
}

/// `$fatal` ends the run at the accepted point with its formatted message
/// and source line; `$stop` ends it as a stop (LRM 2.4 section 9.7).
#[test]
fn fatal_and_stop_end_the_run_with_their_source_line() {
    use rspice_core::ModelFinishTask;
    let directory = scratch("control");
    for (body, task, message) in [
        (
            "@(cross(V(x) - 0.25, +1)) $fatal(1, \"crossed at %g\", 0.25);",
            ModelFinishTask::Fatal,
            Some("crossed at 0.25"),
        ),
        (
            "@(cross(V(x) - 0.25, +1)) $stop;",
            ModelFinishTask::Stop,
            None,
        ),
    ] {
        let (outcome, records) = run(&directory, body, 10e-9);
        let SimulationOutcome::Finished { finish, .. } = outcome.unwrap() else {
            panic!("{body}: the run must end at the call");
        };
        assert_eq!(finish.task, task, "{body}");
        assert_eq!(finish.message.as_deref(), message, "{body}");
        let source = finish.source.expect("the call's source");
        assert!(source.file.ends_with("tasks.va"), "{source:?}");
        assert_eq!(source.line, 5);
        assert!(
            records
                .iter()
                .any(|record| matches!(record.stream, HdlStream::Finish { .. })),
            "{records:?}"
        );
    }
}
