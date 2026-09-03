//! `.FOUR` is a post-process of an authored transient, never an independently
//! invented simulation.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
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
        "rspice_fourier_parent_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&path).expect("create Fourier test directory");
    TestDirectory(path)
}

fn harmonic_magnitude(result: &Value, harmonic: u64) -> f64 {
    result["harmonics"]
        .as_array()
        .expect("harmonics array")
        .iter()
        .find(|entry| entry["n"].as_u64() == Some(harmonic))
        .expect("requested harmonic")["magnitude"]
        .as_f64()
        .expect("finite harmonic magnitude")
}

#[test]
fn fourier_consumes_the_authored_transient_and_exports_currents_with_parent_identity() {
    let directory = test_dir("attached");
    let requested = directory.join("results.json");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            fixture("four_attached_tran.cir")
                .to_str()
                .expect("UTF-8 fixture path"),
            "--output",
            requested.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run attached Fourier fixture");
    assert!(
        output.status.success(),
        "attached Fourier run failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let fourier_path = directory.join("results.four-001.json");
    let transient_path = directory.join("results.tran-001.json");
    assert!(
        transient_path.exists(),
        "authored transient artifact is missing"
    );
    assert!(fourier_path.exists(), "Fourier artifact is missing");
    assert!(
        !requested.exists(),
        "multi-analysis base path was overwritten"
    );

    let document: Value =
        serde_json::from_slice(&std::fs::read(&fourier_path).expect("read Fourier JSON artifact"))
            .expect("parse Fourier JSON artifact");
    assert_eq!(document["analysis"], "fourier");
    assert_eq!(document["analysis_id"], "four-001");
    assert_eq!(document["parent_analysis_id"], "tran-001");
    let results = document["results"].as_array().expect("Fourier results");
    assert_eq!(results.len(), 2, "voltage or current output was dropped");
    assert_eq!(results[0]["physical_type"], "voltage");
    assert_eq!(results[1]["physical_type"], "current");
    let differential = harmonic_magnitude(&results[0], 1);
    let source_current = harmonic_magnitude(&results[1], 1);
    assert!(
        (differential - 0.5).abs() <= 5.0e-3,
        "V(in,mid) fundamental is {differential}, expected 0.5"
    );
    assert!(
        (source_current - 5.0e-4).abs() <= 5.0e-6,
        "I(V1) fundamental is {source_current}, expected 0.5 mA"
    );
}

#[test]
fn fourier_without_an_authored_transient_fails_without_an_artifact() {
    let directory = test_dir("missing_tran");
    let deck = directory.join("missing_tran.cir");
    let requested = directory.join("fourier.json");
    std::fs::write(
        &deck,
        "* missing parent transient\nV1 out 0 1\nR1 out 0 1k\n.FOUR 1k V(out) I(V1)\n.END\n",
    )
    .expect("write missing-parent deck");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--output",
            requested.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run missing-parent Fourier fixture");
    assert!(
        !output.status.success(),
        "missing parent transient was accepted"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("requires a completed authored .TRAN"),
        "missing-parent diagnostic was not explicit: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !requested.exists(),
        "failed Fourier request published an artifact"
    );
}
