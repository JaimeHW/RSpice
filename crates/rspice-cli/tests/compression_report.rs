//! A compressed transient reports what it approximated, not just how much it
//! dropped.
//!
//! A compression ratio says how many samples were discarded. It says nothing
//! about whether discarding them was allowed, which is the only question that
//! decides whether the stored waveform is still evidence. The run therefore
//! has to print the worst error the compressor actually observed — which
//! signal, at what time, in absolute and relative terms, and how much of the
//! declared tolerance it consumed — alongside the ratio.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice_compression_report_{}_{}_{}",
            std::process::id(),
            tag,
            serial
        ));
        std::fs::create_dir(&path).expect("create compression-report test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// An RC low-pass driven well above its corner, sampled far more finely than
/// the compressor's tolerance requires, so samples are genuinely discarded and
/// the certificate names a real approximated sample rather than reporting
/// "none".
const SMOOTH_DECK: &str = "* compression certificate fixture\n\
     V1 in 0 SIN(0 1 1k)\n\
     R1 in out 1k\n\
     C1 out 0 1u\n\
     .tran 1u 2m\n\
     .end\n";

fn run_compressed(directory: &TestDirectory, tolerance: &str) -> (String, String) {
    let deck = directory.path().join("smooth.sp");
    let output = directory.path().join("smooth.csv");
    std::fs::write(&deck, SMOOTH_DECK).expect("write compression fixture");

    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--compress",
            "--compress-tol",
            tolerance,
            "--output",
            output.to_str().expect("UTF-8 output path"),
            "--format",
            "csv",
        ])
        .output()
        .expect("run compressed transient");

    let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
    assert!(
        result.status.success(),
        "compressed run failed:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    (stdout, stderr)
}

#[test]
fn a_compressed_run_reports_its_worst_error_beside_the_ratio() {
    let directory = TestDirectory::new("certificate");
    let (stdout, stderr) = run_compressed(&directory, "1e-4");

    assert!(
        stdout.contains("compression ratio:"),
        "the ratio is still reported:\n{stdout}\n{stderr}"
    );

    let line = stdout
        .lines()
        .find(|line| line.contains("Worst compression error:"))
        .unwrap_or_else(|| {
            panic!("no worst-error certificate beside the ratio:\n{stdout}\n{stderr}")
        });

    for expected in [
        "voltage",
        "at t=",
        "absolute",
        "relative",
        "tolerance",
        "% used",
    ] {
        assert!(
            line.contains(expected),
            "the certificate omits {expected:?}: {line}"
        );
    }
    assert!(
        line.contains("v("),
        "the certificate must name the signal it measured: {line}"
    );
}

#[test]
fn a_run_that_discarded_nothing_says_so_rather_than_omitting_the_certificate() {
    let directory = TestDirectory::new("lossless");
    // A tolerance far below the solver's own accuracy leaves every accepted
    // sample in place. Silence would be indistinguishable from a report that
    // was never computed, so the line still has to appear.
    let (stdout, stderr) = run_compressed(&directory, "1e-30");

    assert!(
        stdout.contains("Worst compression error: none"),
        "a lossless compression must still state that nothing was approximated:\n\
         {stdout}\n{stderr}"
    );
}
