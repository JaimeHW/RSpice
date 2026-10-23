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

/// A stray positional is not a plusarg.
#[test]
fn a_stray_positional_is_not_a_plusarg() {
    let directory = common::test_dir("hdl_stray_positional");
    hdl_console_deck(&directory, "");
    let output = rspice_in(&directory, &["--quiet", "run", "talker.cir", "N=4"]);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
}

/// IEEE 1364-2005 section 15: `--hdl-timing-checks` and `.options
/// hdl_timing_checks` run a deck's HDL timing checks or leave them out. `d`
/// changes 1 before `c` rises at 2, inside the setup limit of 3, so the
/// check's notifier goes from `x` to 0 at 2 (section 15.5); left out, it
/// never changes. The values are the LRM's closed form.
#[test]
fn hdl_timing_checks_runs_or_leaves_out_timing_checks() {
    let directory = common::test_dir("hdl_timing_checks");
    std::fs::write(
        directory.join("stamp.va"),
        "`timescale 1ns/1ns\n\
         module chk(input d, input c);\n\
         \x20 reg ntf;\n\
         \x20 specify $setup(d, posedge c, 3, ntf); endspecify\n\
         \x20 always @(ntf) $display(\"ntf=%b t=%0t\", ntf, $time);\n\
         endmodule\n\
         module stamp(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 reg d, c;\n\
         \x20 chk u(d, c);\n\
         \x20 initial begin d = 0; c = 0; #1 d = 1; #1 c = 1; end\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .unwrap();
    let deck = |options: &str| {
        std::fs::write(
            directory.join("stamp.cir"),
            format!(
                "* timing checks\nx1 p 0 stamp\nrp p 0 1k\n.va \"stamp.va\" module=stamp\n{options}.tran 1n 6n\n.end\n"
            ),
        )
        .unwrap();
    };
    for (options, args, checked) in [
        ("", &[][..], true),
        ("", &["--hdl-timing-checks", "no"][..], false),
        (".options hdl_timing_checks=0\n", &[][..], false),
    ] {
        deck(options);
        let mut command = vec!["--quiet", "run", "stamp.cir"];
        command.extend(args);
        let output = rspice_in(&directory, &command);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{options}{args:?}: {output:?}"
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            stdout.contains("ntf=0 t=2\n"),
            checked,
            "{options}{args:?}: {stdout}"
        );
    }
}

/// IEEE 1364-2005 section 5.3: `--hdl-delay-select` and `.options
/// hdl_delay_select` choose which value of each min:typ:max expression a
/// deck's HDL module elaborates with, the command line over the deck; each
/// selection compiles the same file anew rather than reusing another's.
#[test]
fn hdl_delay_select_chooses_the_min_typ_or_max_value() {
    let directory = common::test_dir("hdl_delay_select");
    std::fs::write(
        directory.join("stamp.va"),
        "`timescale 1ns/1ns\n\
         module stamp(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 initial #(1:2:3) $display(\"t=%0t\", $time);\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .unwrap();
    let deck = |options: &str| {
        std::fs::write(
            directory.join("stamp.cir"),
            format!(
                "* min:typ:max\nx1 p 0 stamp\nrp p 0 1k\n.va \"stamp.va\" stamp\n{options}.tran 1n 5n\n.end\n"
            ),
        )
        .unwrap();
    };
    for (options, args, expected) in [
        ("", &[][..], "t=2\n"),
        ("", &["--hdl-delay-select", "max"][..], "t=3\n"),
        (".options hdl_delay_select=min\n", &[][..], "t=1\n"),
        (
            ".options hdl_delay_select=min\n",
            &["--hdl-delay-select", "max"][..],
            "t=3\n",
        ),
    ] {
        deck(options);
        let mut command = vec!["--quiet", "run", "stamp.cir"];
        command.extend(args);
        let output = rspice_in(&directory, &command);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{options}{args:?}: {output:?}"
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(expected), "{options}{args:?}: {stdout}");
    }
}

/// IEEE 1364-2005 section 14: `--hdl-path-delays` and `.options
/// hdl_path_delays` keep a deck's HDL module paths or leave them out. The
/// path delays `y` 3 after `a`; left out, `y` follows `a` at once.
#[test]
fn hdl_path_delays_keeps_or_leaves_out_module_paths() {
    let directory = common::test_dir("hdl_path_delays");
    std::fs::write(
        directory.join("stamp.va"),
        "`timescale 1ns/1ns\n\
         module buf_p(output y, input a);\n\
         \x20 assign y = a;\n\
         \x20 specify (a => y) = 3; endspecify\n\
         endmodule\n\
         module stamp(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 reg a;\n\
         \x20 wire y;\n\
         \x20 buf_p u(y, a);\n\
         \x20 initial begin a = 0; #1 a = 1; end\n\
         \x20 always @(y) $display(\"y=%b t=%0t\", y, $time);\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .unwrap();
    let deck = |options: &str| {
        std::fs::write(
            directory.join("stamp.cir"),
            format!(
                "* path delays\nx1 p 0 stamp\nrp p 0 1k\n.va \"stamp.va\" module=stamp\n{options}.tran 1n 6n\n.end\n"
            ),
        )
        .unwrap();
    };
    for (options, args, expected) in [
        ("", &[][..], "y=1 t=4\n"),
        ("", &["--hdl-path-delays", "no"][..], "y=1 t=1\n"),
        (".options hdl_path_delays=0\n", &[][..], "y=1 t=1\n"),
    ] {
        deck(options);
        let mut command = vec!["--quiet", "run", "stamp.cir"];
        command.extend(args);
        let output = rspice_in(&directory, &command);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{options}{args:?}: {output:?}"
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(expected), "{options}{args:?}: {stdout}");
    }
}

/// Design P2 section 4.3: a design's relative file names resolve against the
/// deck's directory, or `--hdl-run-dir`; a write that would leave that
/// directory fails as `$fopen` failures do (0, with a located warning) unless
/// the run passes `--hdl-unrestricted-io`.
#[test]
fn hdl_files_resolve_against_the_run_directory_and_stay_inside_it() {
    let root = common::test_dir("hdl_files");
    let deck = root.join("deck");
    std::fs::create_dir_all(deck.join("sub")).unwrap();
    std::fs::write(
        deck.join("writer.va"),
        "`timescale 1ns/1ns\n\
         module writer(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 integer fd, up;\n\
         \x20 initial begin\n\
         \x20   fd = $fopen(\"out.txt\", \"w\");\n\
         \x20   $fdisplay(fd, \"hello %0d\", 5);\n\
         \x20   $fclose(fd);\n\
         \x20   up = $fopen(\"../escape.txt\", \"w\");\n\
         \x20   $display(\"escape=%0d\", up != 0);\n\
         \x20   if (up != 0) $fclose(up);\n\
         \x20 end\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .unwrap();
    std::fs::write(
        deck.join("writer.cir"),
        "* HDL files\nx1 p 0 writer\nrp p 0 1k\n.va \"writer.va\" writer\n.tran 1n 5n\n.end\n",
    )
    .unwrap();
    // From the root: the deck's directory is the run directory.
    let output = rspice_in(&root, &["--quiet", "run", "deck/writer.cir"]);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        std::fs::read_to_string(deck.join("out.txt")).unwrap(),
        "hello 5\n"
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("escape=0\n"));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("outside the run directory") && stderr.contains("writer.va:10"),
        "{stderr}"
    );
    assert!(!root.join("escape.txt").exists());

    let output = rspice_in(
        &root,
        &[
            "--quiet",
            "run",
            "deck/writer.cir",
            "--hdl-run-dir",
            "deck/sub",
        ],
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        std::fs::read_to_string(deck.join("sub").join("out.txt")).unwrap(),
        "hello 5\n"
    );
    // `deck/sub/../escape.txt` is `deck/escape.txt`, outside `deck/sub`.
    assert!(!deck.join("escape.txt").exists());

    let output = rspice_in(
        &root,
        &["--quiet", "run", "deck/writer.cir", "--hdl-unrestricted-io"],
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("escape=1\n"));
    assert!(root.join("escape.txt").exists());
}

/// A write to the standard-error descriptor (IEEE 1364-2005 section 17.2.1)
/// reaches stderr and the transcript, never stdout; the standard-output
/// descriptor's write stays on stdout.
#[test]
fn hdl_writes_to_the_stderr_descriptor_reach_stderr() {
    let directory = common::test_dir("hdl_stderr");
    std::fs::write(
        directory.join("teller.va"),
        "`timescale 1ns/1ns\n\
         module teller(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 initial begin\n\
         \x20   $fdisplay(32'h8000_0001, \"to stdout\");\n\
         \x20   $fdisplay(32'h8000_0002, \"to stderr %0d\", 7);\n\
         \x20   $fwrite(32'h8000_0002, \"no newline\");\n\
         \x20 end\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .unwrap();
    std::fs::write(
        directory.join("teller.cir"),
        "* HDL stderr\nx1 p 0 teller\nrp p 0 1k\n.va \"teller.va\" teller\n.tran 1n 5n\n.end\n",
    )
    .unwrap();
    let output = rspice_in(
        &directory,
        &["run", "teller.cir", "-f", "csv", "-o", "out.csv"],
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("to stdout\n"), "{stdout}");
    assert!(!stdout.contains("to stderr"), "{stdout}");
    assert!(stderr.contains("to stderr 7\nno newline"), "{stderr}");
    let transcript = std::fs::read_to_string(directory.join("out.hdl.log")).unwrap();
    assert_eq!(transcript, "to stdout\nto stderr 7\nno newline");
}

/// One call site warns five times, the fifth saying the rest are suppressed,
/// however often the deck trips it.
#[test]
fn a_repeated_warning_is_shown_five_times_then_suppressed() {
    let directory = common::test_dir("warning_limit");
    let saves = ".save\n".repeat(9);
    std::fs::write(
        directory.join("saves.cir"),
        format!("* bare saves\nV1 in 0 1\nR1 in 0 1k\n{saves}.op\n.end\n"),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["run", "saves.cir"])
        .current_dir(&directory)
        .env_remove("RUST_LOG")
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr
            .matches("save/print directive without output signals ignored")
            .count(),
        5,
        "{stderr}"
    );
    assert_eq!(
        stderr
            .matches(
                "save/print directive without output signals ignored (further occurrences of \
                 this warning will be suppressed)"
            )
            .count(),
        1,
        "{stderr}"
    );
}
