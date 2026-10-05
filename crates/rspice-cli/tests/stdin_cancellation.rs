use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn open_idle_stdin_obeys_timeout_and_emits_one_json_error() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "--error-format",
            "json",
            "run",
            "-",
            "--timeout",
            "0.05",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Keep the producer alive without sending bytes or EOF.
    let producer = child.stdin.take().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let output = child.wait_with_output().unwrap();
            panic!("timeout failed to interrupt idle stdin: {output:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    drop(producer);
    assert_eq!(output.status.code(), Some(124), "{output:?}");
    assert!(output.stdout.is_empty());
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["exit_code"], 124);
}

#[test]
fn oversized_timeout_is_rejected_before_reading_stdin() {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "--error-format",
            "json",
            "run",
            "-",
            "--timeout",
            "1e100",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains("representable")
    );
}
