//! A `.STEP` deck publishes its coordinate set as one transaction.
//!
//! The plan's output contract is that a failed or cancelled run leaves either
//! the old complete artifact or no artifact. For a swept deck that means the
//! destination directory holds the complete coordinate set plus the manifest
//! that names it, or none of it — never the prefix that finished before the
//! timeout fired.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

/// Coordinates of the swept deck below.
const COORDINATES: usize = 16;

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice_step_set_{}_{}_{}",
            std::process::id(),
            tag,
            serial
        ));
        std::fs::create_dir_all(&path).expect("create step-set test directory");
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

/// A deck whose coordinates each cost real solver time, so a timeout can land
/// between two of them instead of before the first.
fn write_swept_deck(directory: &Path) -> PathBuf {
    let mut source = String::from("* stepped resistive chain\n.param rval=1k\nV1 n0 0 1\n");
    for index in 0..2_000 {
        source.push_str(&format!("R{index} n{index} n{} {{rval}}\n", index + 1));
    }
    source.push_str("RTAIL n2000 0 1k\n.step param rval list");
    for coordinate in 0..COORDINATES {
        source.push_str(&format!(" {}", 1_000 + coordinate * 100));
    }
    source.push_str("\n.op\n.end\n");
    let path = directory.join("swept_chain.sp");
    std::fs::write(&path, source).expect("write swept deck");
    path
}

fn run(deck: &Path, output: &Path, timeout: Option<f64>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args([
        "run",
        deck.to_str().expect("UTF-8 deck path"),
        "--output",
        output.to_str().expect("UTF-8 output path"),
        "--format",
        "json",
    ]);
    if let Some(seconds) = timeout {
        command.args(["--timeout", &format!("{seconds}")]);
    }
    command.output().expect("run swept deck")
}

fn names_in(directory: &Path) -> Vec<String> {
    std::fs::read_dir(directory)
        .expect("list step-set directory")
        .map(|entry| {
            entry
                .expect("step-set directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

/// Artifacts the sweep publishes: one per coordinate plus the set manifest.
fn published_artifacts(directory: &Path) -> Vec<String> {
    let mut published = names_in(directory)
        .into_iter()
        .filter(|name| name.starts_with("results.") && name.ends_with(".json"))
        .collect::<Vec<_>>();
    published.sort();
    published
}

fn staging_files(directory: &Path) -> Vec<String> {
    names_in(directory)
        .into_iter()
        .filter(|name| name.contains(rspice_output::STAGING_MARKER))
        .collect()
}

/// The contract itself: the sweep is either entirely present with its
/// manifest, or entirely absent.
fn assert_complete_set_or_nothing(directory: &Path) -> bool {
    let published = published_artifacts(directory);
    if published.is_empty() {
        return false;
    }
    assert!(
        published.contains(&"results.run_set.json".to_owned()),
        "a published coordinate set has no manifest: {published:?}"
    );
    assert_eq!(
        published.len(),
        COORDINATES + 1,
        "a partial coordinate set was published: {published:?}"
    );
    true
}

fn exported_coordinate_count(output: &Output) -> usize {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.contains("Results exported to:"))
        .count()
}

#[test]
fn a_timeout_between_coordinates_publishes_the_whole_set_or_nothing() {
    let directory = TestDirectory::new("timeout");
    let deck = write_swept_deck(directory.path());
    let requested = directory.path().join("results.json");

    // Measure the deck on this machine first, so the timeout below is a
    // fraction of a real sweep rather than a guessed wall-clock constant.
    let started = std::time::Instant::now();
    let complete = run(&deck, &requested, None);
    let sweep_seconds = started.elapsed().as_secs_f64();
    assert!(
        complete.status.success(),
        "swept deck failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&complete.stdout),
        String::from_utf8_lossy(&complete.stderr)
    );
    assert!(
        assert_complete_set_or_nothing(directory.path()),
        "a successful sweep published nothing"
    );
    assert_eq!(exported_coordinate_count(&complete), COORDINATES);
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(directory.path().join("results.run_set.json")).expect("read set manifest"),
    )
    .expect("parse set manifest");
    assert_eq!(manifest["kind"], "axis_coordinate_set");
    assert_eq!(manifest["coordinate_count"], COORDINATES);
    let coordinates = manifest["coordinates"]
        .as_array()
        .expect("manifest coordinates");
    assert_eq!(coordinates.len(), COORDINATES);
    for coordinate in coordinates {
        for artifact in coordinate["artifacts"]
            .as_array()
            .expect("coordinate artifacts")
        {
            let artifact = artifact.as_str().expect("artifact file name");
            assert!(
                directory.path().join(artifact).exists(),
                "manifest names a missing artifact: {artifact}"
            );
        }
    }

    // Cancel a fresh sweep partway through. Half of a measured sweep leaves
    // the planner and the first coordinates enough time to finish and the
    // rest of the set unreachable.
    for fraction in [0.5, 0.75] {
        let cancelled_directory = TestDirectory::new("cancelled");
        let cancelled_deck = write_swept_deck(cancelled_directory.path());
        let cancelled_output = cancelled_directory.path().join("results.json");
        let cancelled = run(
            &cancelled_deck,
            &cancelled_output,
            Some(sweep_seconds * fraction),
        );

        assert_eq!(
            cancelled.status.code(),
            Some(124),
            "a cancelled sweep must use the timeout exit code:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cancelled.stdout),
            String::from_utf8_lossy(&cancelled.stderr)
        );
        assert!(
            !assert_complete_set_or_nothing(cancelled_directory.path()),
            "a cancelled sweep published artifacts: {:?}",
            published_artifacts(cancelled_directory.path())
        );
        assert!(
            staging_files(cancelled_directory.path()).is_empty(),
            "a cancelled sweep left staging files: {:?}",
            staging_files(cancelled_directory.path())
        );

        if exported_coordinate_count(&cancelled) > 0 {
            // The interesting case: coordinates completed and were discarded
            // rather than published as a shorter sweep.
            return;
        }
    }
    panic!("no timeout fraction cancelled the sweep after a completed coordinate");
}

#[test]
fn a_killed_sweep_publishes_nothing_and_the_next_run_reclaims_its_stages() {
    let directory = TestDirectory::new("killed");
    let deck = write_swept_deck(directory.path());
    let requested = directory.path().join("results.json");

    let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--output",
            requested.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn swept deck");

    // Wait for the sweep to stage a coordinate, then kill it without giving
    // it a chance to clean up, the way a machine failure would.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while staging_files(directory.path()).is_empty() {
        assert!(
            std::time::Instant::now() < deadline,
            "the sweep staged no coordinate before the deadline"
        );
        assert!(
            child.try_wait().expect("poll swept deck").is_none(),
            "the sweep finished before it staged a coordinate"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    child.kill().expect("kill the swept deck");
    child.wait().expect("reap the swept deck");

    assert!(
        published_artifacts(directory.path()).is_empty(),
        "a killed sweep published artifacts: {:?}",
        published_artifacts(directory.path())
    );
    assert!(
        !staging_files(directory.path()).is_empty(),
        "the killed sweep left no staging file to reclaim"
    );

    // A later run into the same directory reclaims what the dead process left.
    let recovery_deck = directory.path().join("recovery.sp");
    std::fs::write(
        &recovery_deck,
        "* trivial recovery deck\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
    )
    .expect("write recovery deck");
    let recovery_output = directory.path().join("recovered.json");
    let recovery = run(&recovery_deck, &recovery_output, None);
    assert!(
        recovery.status.success(),
        "recovery run failed:\n{}",
        String::from_utf8_lossy(&recovery.stderr)
    );
    assert!(
        staging_files(directory.path()).is_empty(),
        "stale staging files survived a later run: {:?}",
        staging_files(directory.path())
    );
    assert!(
        recovery_output.exists(),
        "the recovery run published nothing"
    );
}
