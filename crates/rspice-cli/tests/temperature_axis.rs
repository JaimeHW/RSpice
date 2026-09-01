//! `.TEMP` is a run axis, not an independent operating-point analysis.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("audit_regressions")
        .join(name)
}

struct TestDirectory(PathBuf);

impl std::ops::Deref for TestDirectory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn test_dir(tag: &str) -> TestDirectory {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rspice_temperature_axis_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&path).expect("create temperature-axis test directory");
    TestDirectory(path)
}

fn run(deck: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--output",
            output.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run temperature deck")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("read JSON output"))
        .expect("parse JSON output")
}

fn signal_real_values(document: &Value, name: &str) -> Vec<f64> {
    let signal = document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|signal| {
            signal["name"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| panic!("missing '{name}' in {document:#}"));
    let values = signal["values"]
        .as_array()
        .or_else(|| signal["real"].as_array())
        .expect("real signal values");
    values
        .iter()
        .map(|value| value.as_f64().expect("finite signal value"))
        .collect()
}

#[test]
fn multi_temperature_wraps_every_authored_analysis_without_extra_runs() {
    let dir = test_dir("multi");
    let requested = dir.join("results.json");
    let output = run(&fixture("temp_wraps_tran_ac.cir"), &requested);
    assert!(
        output.status.success(),
        "temperature-axis deck failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!requested.exists(), "multi-run base path was overwritten");

    let expected = [
        dir.join("results.step_000001.tran.json"),
        dir.join("results.step_000001.ac.json"),
        dir.join("results.step_000002.tran.json"),
        dir.join("results.step_000002.ac.json"),
    ];
    let mut actual = std::fs::read_dir(&*dir)
        .expect("list temperature outputs")
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    actual.sort();
    let mut expected_sorted = expected.clone();
    expected_sorted.sort();
    assert_eq!(actual, expected_sorted, "unexpected nominal or OP artifact");

    for path in &expected {
        let document = read_json(path);
        let expected_analysis = if path.to_string_lossy().contains(".tran.") {
            "transient"
        } else {
            "ac"
        };
        assert_eq!(
            document["analysis"],
            expected_analysis,
            "{}",
            path.display()
        );
        assert!(!signal_real_values(&document, "V(out)").is_empty());
    }

    let cold_ac = read_json(&expected[1]);
    let hot_ac = read_json(&expected[3]);
    assert_ne!(
        signal_real_values(&cold_ac, "V(out)"),
        signal_real_values(&hot_ac, "V(out)"),
        "temperature coordinates produced identical AC data"
    );
}

#[test]
fn single_temperature_configures_transient_without_running_a_temp_op() {
    let dir = test_dir("single");
    let requested = dir.join("transient.json");
    let output = run(&fixture("temp_single_tran.cir"), &requested);
    assert!(
        output.status.success(),
        "single-temperature transient failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let files = std::fs::read_dir(&*dir)
        .expect("list outputs")
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    assert_eq!(files.as_slice(), std::slice::from_ref(&requested));
    assert_eq!(read_json(&requested)["analysis"], "transient");
}
