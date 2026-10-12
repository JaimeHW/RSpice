//! Real disconnected pipes exercise command error paths without timing races.
mod common;

use std::path::Path;
use std::process::{Command, Stdio};

fn closed_stdout(directory: &Path, args: &[&str]) -> std::process::Output {
    let (reader, writer) = std::io::pipe().unwrap();
    drop(reader);
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json"])
        .args(args)
        .current_dir(directory)
        .stdout(writer)
        .stderr(Stdio::piped())
        .output()
        .unwrap()
}

fn assert_io_error(output: &std::process::Output) {
    assert_eq!(output.status.code(), Some(74), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["category"], "io");
    assert_eq!(error["error"]["path"], "stdout");
}

#[test]
fn inspection_and_check_reports_propagate_broken_pipes_in_both_formats() {
    let directory = common::test_dir("inspection_pipe");
    std::fs::write(
        directory.join("deck.cir"),
        "Output test\nV1 in 0 1\nR1 in 0 1k\n.end\n",
    )
    .unwrap();
    for args in [
        vec!["info", "deck.cir"],
        vec!["info", "deck.cir", "--json"],
        vec!["check", "deck.cir"],
        vec!["check", "deck.cir", "--json"],
    ] {
        assert_io_error(&closed_stdout(&directory, &args));
    }
    std::fs::write(directory.join("bad.cir"), "Bad deck\nR1 in\n.end\n").unwrap();
    assert_io_error(&closed_stdout(&directory, &["check", "bad.cir", "--json"]));
}

#[test]
fn health_and_completion_reports_propagate_broken_pipes() {
    let directory = common::test_dir("health_pipe");
    for args in [
        vec!["health", "--mode", "liveness"],
        vec!["health", "--json"],
        vec!["completions", "bash"],
        vec!["completions", "powershell"],
    ] {
        assert_io_error(&closed_stdout(&directory, &args));
    }
    std::fs::write(
        directory.join("tiny.toml"),
        "[resources]\nmax_netlist_bytes=1\n",
    )
    .unwrap();
    assert_io_error(&closed_stdout(
        &directory,
        &["--config", "tiny.toml", "health", "--json"],
    ));
}

#[test]
fn model_catalog_reports_propagate_broken_pipes() {
    let directory = common::test_dir("models_pipe");
    std::fs::write(
        directory.join("PACKS.tsv"),
        "ok\tbasic\tok\tpermissive\tMIT\t1\t\t1\t0\t1\t0\t1\t100\tdiode\tOK\n",
    )
    .unwrap();
    std::fs::write(
        directory.join("CATALOG.tsv"),
        "PART\tmodel\tdiode\tok\tparts.sp\t1\t0\ttop\n",
    )
    .unwrap();
    for query in [vec![], vec!["--part", "PART"], vec!["--search", "PART"]] {
        let mut args = vec!["models", "--models-dir", "."];
        args.extend(query);
        assert_io_error(&closed_stdout(&directory, &args));
    }
}

#[test]
fn simulation_output_failures_preserve_existing_artifacts() {
    let directory = common::test_dir("simulation_pipe");
    std::fs::write(
        directory.join("deck.cir"),
        "Output test\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
    )
    .unwrap();
    std::fs::write(directory.join("result.csv"), "Existing output").unwrap();
    assert_io_error(&closed_stdout(
        &directory,
        &["run", "deck.cir", "-f", "csv", "-o", "result.csv"],
    ));
    assert_eq!(
        std::fs::read_to_string(directory.join("result.csv")).unwrap(),
        "Existing output"
    );

    // A summary is published after the completed simulation's artifacts. Its
    // write failure is an I/O failure and must not erase those valid results.
    assert_io_error(&closed_stdout(
        &directory,
        &[
            "--quiet",
            "run",
            "deck.cir",
            "-f",
            "csv",
            "-o",
            "result.csv",
            "--summary",
            "-",
        ],
    ));
    assert!(
        std::fs::read_to_string(directory.join("result.csv"))
            .unwrap()
            .contains("V(IN),")
    );
}

#[test]
fn authored_control_print_propagates_broken_pipe_without_publishing_partial_results() {
    let directory = common::test_dir("control_pipe");
    std::fs::write(
        directory.join("deck.cir"),
        "Output test\nV1 in 0 1\nR1 in 0 1k\n.control\nop\nprint v(in)\n.endc\n.end\n",
    )
    .unwrap();
    let output = closed_stdout(
        &directory,
        &[
            "--quiet",
            "run",
            "deck.cir",
            "-f",
            "json",
            "-o",
            "result.json",
        ],
    );
    assert_io_error(&output);
    assert!(!directory.join("result.op-001.json").exists());
    assert!(!directory.join("result.control-001.json").exists());
}

#[test]
fn help_and_version_propagate_stdout_failures() {
    let directory = common::test_dir("help_pipe");
    for args in [vec!["--help"], vec!["--version"], vec!["run", "--help"]] {
        assert_io_error(&closed_stdout(&directory, &args));
    }
}

#[test]
fn closed_stderr_preserves_the_primary_exit_status() {
    let directory = common::test_dir("diagnostic_pipe");
    std::fs::write(
        directory.join("warning.cir"),
        "Warning\n.option foobar=1\nR1 in 0 1k\n.end\n",
    )
    .unwrap();
    for (args, expected) in [
        (vec!["info", "missing.cir"], 66),
        (vec!["--error-format", "json", "info", "missing.cir"], 66),
        (vec!["invalid-command"], 2),
        (vec!["--error-format", "json", "invalid-command"], 2),
        (vec!["info", "warning.cir"], 0),
        (vec!["--error-format", "json", "info", "warning.cir"], 0),
    ] {
        let (reader, writer) = std::io::pipe().unwrap();
        drop(reader);
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(args)
            .current_dir(&directory)
            .stderr(writer)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(expected), "{output:?}");
    }

    let (reader, writer) = std::io::pipe().unwrap();
    drop(reader);
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "health", "--json"])
        .current_dir(&directory)
        .stdout(writer.try_clone().unwrap())
        .stderr(writer)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(74), "{output:?}");
}

#[test]
fn comparison_conversion_and_compilation_propagate_broken_pipes() {
    let directory = common::test_dir("artifact_pipe");
    std::fs::write(directory.join("result.csv"), "time,V(out)\n0,0\n1,1\n").unwrap();
    std::fs::write(directory.join("golden.csv"), "time,V(out)\n0,0\n1,1\n").unwrap();
    for json in [false, true] {
        let mut args = vec!["compare", "result.csv", "golden.csv"];
        if json {
            args.push("--json");
        }
        assert_io_error(&closed_stdout(&directory, &args));
    }
    std::fs::write(directory.join("converted.csv"), "Existing output").unwrap();
    assert_io_error(&closed_stdout(
        &directory,
        &["convert", "result.csv", "converted.csv", "--to", "csv"],
    ));
    assert_eq!(
        std::fs::read_to_string(directory.join("converted.csv")).unwrap(),
        "Existing output"
    );
    std::fs::write(directory.join("model.va"), "module resistor(p,n); inout p,n; electrical p,n; parameter real r=1000; analog I(p,n) <+ V(p,n)/r; endmodule\n").unwrap();
    assert_io_error(&closed_stdout(&directory, &["compile-va", "model.va"]));
}

// ---------------------------------------------------------------------------
// The HDL console (design P2 section 4.3)
// ---------------------------------------------------------------------------

/// A mixed deck whose HDL half prints, reads a plusarg, reports an error and
/// finishes at level 1.
fn hdl_console_deck(directory: &Path, options: &str) {
    std::fs::write(
        directory.join("talker.va"),
        "`timescale 1ns/1ns\n\
         module talker(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 integer value;\n\
         \x20 initial begin\n\
         \x20   value = 0;\n\
         \x20   #2 $display(\"checking\");\n\
         \x20   if ($value$plusargs(\"N=%d\", value)) $display(\"n=%0d\", value);\n\
         \x20   #3 $error(\"mismatch %0d\", 7);\n\
         \x20   #1 $finish(1);\n\
         \x20 end\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .unwrap();
    std::fs::write(
        directory.join("talker.cir"),
        format!(
            "* HDL console\nx1 p 0 talker\nrp p 0 1k\n.va \"talker.va\" talker\n{options}.tran 1n 20n\n.end\n"
        ),
    )
    .unwrap();
}

fn rspice_in(directory: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .current_dir(directory)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .output()
        .unwrap()
}

/// Design text goes to stdout as it is released; the severity report and the
/// level-1 finish notice go to stderr with their time, line and scope; the
/// command-line plusarg reaches `$value$plusargs`; and a run that publishes
/// a result also publishes the transcript beside it.
#[test]
fn hdl_text_reaches_stdout_and_reports_reach_stderr() {
    let directory = common::test_dir("hdl_console");
    hdl_console_deck(&directory, "");
    let output = rspice_in(
        &directory,
        &["run", "talker.cir", "-f", "csv", "-o", "out.csv", "+N=5"],
    );
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("checking\nn=5\n"), "{stdout}");
    assert!(
        stderr.contains("$error at 5 ns (") && stderr.contains("talker.va:10, x1): mismatch 7"),
        "{stderr}"
    );
    assert!(stderr.contains("finished by $finish at 6 ns ("), "{stderr}");
    let transcript = std::fs::read_to_string(directory.join("out.hdl.log")).unwrap();
    assert!(
        transcript.starts_with("checking\nn=5\n$error at 5 ns"),
        "{transcript}"
    );
}

/// When stdout carries the JSON summary, design text moves to stderr, and the
/// deck's plusargs come after the command line's.
#[test]
fn hdl_text_moves_to_stderr_when_stdout_is_a_payload() {
    let directory = common::test_dir("hdl_payload");
    hdl_console_deck(&directory, ".options hdl_plusargs='+N=9'\n");
    let output = rspice_in(
        &directory,
        &["--quiet", "run", "talker.cir", "--summary", "-"],
    );
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("stdout is the summary alone: {error}: {output:?}"));
    assert_eq!(summary["passed"], false);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("checking\nn=9\n"), "{stderr}");
    let output = rspice_in(&directory, &["--quiet", "run", "talker.cir", "+N=4"]);
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("n=4\n"),
        "the command line precedes the deck: {output:?}"
    );
}

/// HDL options whose behaviour has not landed are refused, saying plainly what
/// is not supported, rather than accepted and ignored; a stray positional is
/// not a plusarg.
#[test]
fn unsupported_hdl_options_are_refused_as_not_supported_yet() {
    let directory = common::test_dir("hdl_unlanded");
    hdl_console_deck(&directory, "");
    let refused = |output: &std::process::Output, feature: &str| {
        assert_eq!(output.status.code(), Some(78), "{output:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("is not supported yet"), "{stderr}");
        assert!(stderr.contains(feature), "{stderr}");
        assert!(!stderr.contains("lane"), "no plan identifier: {stderr}");
    };
    for (args, feature) in [
        (
            vec!["--hdl-delay-select", "max"],
            "min:typ:max delay selection",
        ),
        (vec!["--hdl-run-dir", "."], "HDL file I/O is not implemented"),
        (
            vec!["--hdl-unrestricted-io"],
            "HDL file I/O is not implemented",
        ),
    ] {
        let mut command = vec!["--quiet", "run", "talker.cir"];
        command.extend(args);
        refused(&rspice_in(&directory, &command), feature);
    }
    hdl_console_deck(&directory, ".options hdl_libmap=\"lib.map\"\n");
    refused(
        &rspice_in(&directory, &["--quiet", "run", "talker.cir"]),
        "library maps are not implemented",
    );
    let output = rspice_in(&directory, &["--quiet", "run", "talker.cir", "N=4"]);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
}
