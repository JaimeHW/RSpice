//! Frequency-domain CLI analyses must propagate the process abort into core
//! execution and must not publish a result artifact after cancellation.

mod common;

use common::test_dir;

use std::process::Command;

#[test]
fn noise_timeout_is_typed_prompt_and_does_not_publish_an_artifact() {
    let directory = test_dir("noise");
    let deck = directory.path().join("long_noise.sp");
    let artifact = directory.path().join("noise.csv");
    std::fs::write(
        &deck,
        "* cancellable frequency-domain analysis\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         R2 out 0 2k\n\
         .noise V(out) V1 lin 1500000 1 1meg\n\
         .end\n",
    )
    .expect("write noise fixture");

    let started = std::time::Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--timeout",
            "0.25",
            "-o",
            artifact.to_str().expect("UTF-8 artifact path"),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(124),
        "frequency timeout must retain the typed timeout exit code\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("timed out") || stderr.contains("Timeout"),
        "timeout diagnostic must remain explicit:\n{stderr}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "frequency cancellation must stop promptly"
    );
    assert!(
        !artifact.exists(),
        "a cancelled analysis must not publish {}",
        artifact.display()
    );
}
