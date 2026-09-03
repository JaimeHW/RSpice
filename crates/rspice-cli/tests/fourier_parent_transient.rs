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

/// Real samples of one named series in a typed result document.
fn samples(document: &Value, name: &str) -> Vec<f64> {
    document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|signal| signal["descriptor"]["canonicalName"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("missing series '{name}' in {document:#}"))["values"]["samples"]
        .as_array()
        .expect("real samples")
        .iter()
        .map(|sample| sample.as_f64().expect("finite sample"))
        .collect()
}

/// The magnitude the document publishes for one harmonic index.
///
/// The harmonic index is the document's own axis, so the row is selected by
/// the coordinate rather than by position.
fn harmonic_magnitude(document: &Value, harmonic: i64) -> f64 {
    let index = document["axes"]
        .as_array()
        .expect("document axes")
        .iter()
        .find(|axis| axis["name"].as_str() == Some("harmonic"))
        .expect("harmonic axis")["values"]["values"]
        .as_array()
        .expect("harmonic coordinates")
        .iter()
        .position(|value| value.as_i64() == Some(harmonic))
        .unwrap_or_else(|| panic!("harmonic {harmonic} is not on the axis of {document:#}"));
    samples(document, "harmonic_magnitude")[index]
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("read JSON artifact"))
        .expect("parse JSON artifact")
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

    // The core evaluates one spectrum per resolved `.FOUR` operand and the
    // shared result document names one spectrum, so each operand is its own
    // analysis instance with its own artifact.
    let voltage_path = directory.join("results.four-001.json");
    let current_path = directory.join("results.four-002.json");
    let transient_path = directory.join("results.tran-001.json");
    assert!(
        transient_path.exists(),
        "authored transient artifact is missing"
    );
    assert!(voltage_path.exists(), "voltage Fourier artifact is missing");
    assert!(current_path.exists(), "current Fourier artifact is missing");
    assert!(
        !requested.exists(),
        "multi-analysis base path was overwritten"
    );

    let voltage = read_json(&voltage_path);
    let current = read_json(&current_path);
    for (document, tag) in [(&voltage, "four-001"), (&current, "four-002")] {
        assert_eq!(document["resultKind"], "fourier");
        assert_eq!(document["analysis"]["tag"], tag);
        assert_eq!(
            document["parentAnalysis"]["tag"], "tran-001",
            "a Fourier spectrum must name the transient it post-processed"
        );
    }
    assert_eq!(voltage["payload"]["output"], "V(IN,MID)");
    assert_eq!(current["payload"]["output"], "I(V1)");
    assert_eq!(voltage["signals"][0]["descriptor"]["unit"]["unit"], "volt");
    assert_eq!(
        current["signals"][0]["descriptor"]["unit"]["unit"],
        "ampere"
    );
    let differential = harmonic_magnitude(&voltage, 1);
    let source_current = harmonic_magnitude(&current, 1);
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

/// A deck with two transients gets its `.FOUR` identities and its parent
/// bindings from the canonical plan, not from the order the CLI happened to
/// publish spectra in: the second card post-processes the second transient.
#[test]
fn each_fourier_card_is_named_and_parented_by_the_canonical_plan() {
    let directory = test_dir("two_parents");
    let deck = directory.join("two_parents.cir");
    let requested = directory.join("results.json");
    std::fs::write(
        &deck,
        "* one .FOUR card per authored transient\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in mid 1k\n\
         R2 mid 0 1k\n\
         .TRAN 2u 4m\n\
         .FOUR 1k V(in,mid)\n\
         .TRAN 2u 4m\n\
         .FOUR 1k I(V1)\n\
         .END\n",
    )
    .expect("write two-transient deck");
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
        .expect("run two-transient Fourier deck");
    assert!(
        output.status.success(),
        "two-transient Fourier run failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let first = read_json(&directory.join("results.four-001.json"));
    let second = read_json(&directory.join("results.four-002.json"));
    assert_eq!(first["payload"]["output"], "V(IN,MID)");
    assert_eq!(first["parentAnalysis"]["tag"], "tran-001");
    assert_eq!(second["payload"]["output"], "I(V1)");
    assert_eq!(
        second["parentAnalysis"]["tag"], "tran-002",
        "the second .FOUR card post-processes the second authored transient"
    );
}
