//! Advanced CLI analyses must propagate process cancellation into core and
//! must not publish result artifacts after cancellation.

use std::path::{Path, PathBuf};
use std::process::Command;

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_advanced_cancel_{}_{}_{tag}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        if path.exists() {
            std::fs::remove_dir_all(&path).expect("remove stale test directory");
        }
        std::fs::create_dir_all(&path).expect("create test directory");
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

#[test]
fn monte_carlo_timeout_is_typed_prompt_and_does_not_publish_an_artifact() {
    let directory = TestDirectory::new("monte_carlo");
    let deck = directory.path().join("long_monte_carlo.sp");
    let artifact = directory.path().join("monte_carlo.json");
    std::fs::write(
        &deck,
        "* cancellable advanced analysis\n\
         V1 in 0 5\n\
         R1 in n1 {rvar}\n\
         R2 n1 n2 1k\n\
         R3 n2 n3 1k\n\
         R4 n3 n4 1k\n\
         R5 n4 n5 1k\n\
         R6 n5 n6 1k\n\
         R7 n6 n7 1k\n\
         R8 n7 n8 1k\n\
         R9 n8 n9 1k\n\
         R10 n9 n10 1k\n\
         R11 n10 0 1k\n\
         .param rvar=1k\n\
         .op\n\
         .end\n",
    )
    .expect("write Monte Carlo fixture");

    let started = std::time::Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--monte-carlo",
            "10000",
            "--seed",
            "7",
            "--mc-spread",
            "0.1",
            "--timeout",
            "0.10",
            "-o",
            artifact.to_str().expect("UTF-8 artifact path"),
            "-f",
            "json",
        ])
        .output()
        .expect("run rspice");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(124),
        "advanced timeout must retain the typed timeout exit code\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("timed out") || stderr.contains("Timeout"),
        "timeout diagnostic must remain explicit:\n{stderr}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "advanced cancellation must stop promptly"
    );
    assert!(
        !artifact.exists(),
        "a cancelled analysis must not publish {}",
        artifact.display()
    );
}
