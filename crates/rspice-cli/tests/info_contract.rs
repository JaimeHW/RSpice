//! Inspection flags and machine output must describe the same parsed deck.
mod common;

use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn info_reports_closed_stdout_as_io_failure_in_both_formats() {
    for json in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
        command.args(["--quiet", "--error-format", "json", "info", "-"]);
        if json {
            command.arg("--json");
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        // Close the reader before delivering the deck, so the command cannot
        // race ahead and write successfully before its pipe closes.
        drop(child.stdout.take());
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"Closed pipe\nV1 in 0 1\nR1 in 0 1k\n.end\n")
            .unwrap();
        let result = child.wait_with_output().unwrap();
        assert_eq!(result.status.code(), Some(74), "json={json}: {result:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
        assert_eq!(diagnostic["error"]["category"], "io");
    }
}
