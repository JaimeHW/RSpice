//! File admission is shared by conversion, comparison and reference promotion.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::Command;

fn reader(source: &Path, destination: &Path, operation: &str) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args(["--quiet", "--error-format", "json"]);
    if matches!(operation, "compare" | "bless") {
        command.arg("compare").arg(source).arg(destination);
        if operation == "bless" {
            command.arg("--bless");
        }
    } else {
        command
            .arg("convert")
            .arg(source)
            .arg(destination)
            .args(["--to", operation]);
    }
    command
}

#[test]
fn every_result_format_enforces_its_byte_limit_before_decoding() {
    let dir = test_dir("result_file_bytes");
    let grid = dir.join("grid.csv");
    std::fs::write(&grid, "time,D(clk)\n0,0\n1e-9,1\n").unwrap();
    let config = dir.join("limit.toml");
    std::fs::write(&config, "[resources]\nmax_external_data_bytes=16\n").unwrap();
    for (format, extension) in [
        ("raw", "raw"),
        ("ascii", "ascii"),
        ("csv", "csv"),
        ("tsv", "tsv"),
        ("json", "json"),
        ("hdf5", "h5"),
        ("vcd", "vcd"),
        ("touchstone", "s1p"),
    ] {
        let source = dir.join(format!("source.{extension}"));
        if format == "touchstone" {
            std::fs::write(&source, "# Hz S RI R 50\n1 0.5 0\n2 0.25 0\n").unwrap();
        } else {
            let output = reader(&grid, &source, format).output().unwrap();
            assert!(output.status.success(), "{format}: {output:?}");
        }
        #[cfg(unix)]
        {
            let link = dir.join(format!("linked.{extension}"));
            std::os::unix::fs::symlink(&source, &link).unwrap();
            let output = reader(&link, &dir.join("from-link.csv"), "csv")
                .output()
                .unwrap();
            assert!(output.status.success(), "{format}: {output:?}");
        }
        let original = std::fs::read(&source).unwrap();
        assert!(original.len() > 16);
        for operation in ["compare", "bless", "csv", "vcd"] {
            let destination = dir.join(format!("destination.{extension}"));
            std::fs::write(&destination, "preserve destination").unwrap();
            let output = reader(&source, &destination, operation)
                .arg("--config")
                .arg(&config)
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(75),
                "{format}, {operation}: {output:?}"
            );
            let report: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(report["error"]["code"], "resource_limit");
            assert_eq!(report["error"]["resource"], "external_data_bytes");
            assert_eq!(report["error"]["limit"], 16);
            assert_eq!(report["error"]["path"], source.to_str().unwrap());
            assert_eq!(
                std::fs::read_to_string(&destination).unwrap(),
                "preserve destination"
            );
            assert_eq!(std::fs::read(&source).unwrap(), original);
        }
    }
}

#[cfg(unix)]
#[test]
fn fifo_inputs_fail_promptly_without_waiting_for_a_writer() {
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let dir = test_dir("result_fifo");
    for extension in ["raw", "vcd", "csv", "tsv", "json", "h5", "s1p"] {
        let source = dir.join(format!("source.{extension}"));
        assert!(
            Command::new("mkfifo")
                .arg(&source)
                .status()
                .unwrap()
                .success()
        );
        for operation in ["compare", "bless", "csv", "vcd"] {
            let destination = dir.join(format!("destination.{extension}"));
            std::fs::write(&destination, "preserve destination").unwrap();
            let mut child = reader(&source, &destination, operation)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            let started = Instant::now();
            while child.try_wait().unwrap().is_none() {
                if started.elapsed() > Duration::from_secs(5) {
                    child.kill().unwrap();
                    let _ = child.wait();
                    panic!("{extension}, {operation}: blocked on a FIFO without a writer");
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            let output = child.wait_with_output().unwrap();
            assert_eq!(
                output.status.code(),
                Some(74),
                "{extension}, {operation}: {output:?}"
            );
            let report: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(report["error"]["code"], "input_read_error");
            assert_eq!(report["error"]["path"], source.to_str().unwrap());
            assert!(
                report["error"]["message"]
                    .as_str()
                    .unwrap()
                    .contains("regular file")
            );
            assert_eq!(
                std::fs::read_to_string(&destination).unwrap(),
                "preserve destination"
            );
        }
    }
}
