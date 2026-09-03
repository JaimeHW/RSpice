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

/// Coordinates of the cancellable deck below.
const CANCELLABLE_COORDINATES: usize = 32;

/// How many budgets the cancellation search may try before giving up.
///
/// Each attempt halves the remaining interval in log space, and the interval
/// the deck offers spans more than a decade, so a handful of attempts is
/// generous even when the host stalls between two of them.
const CANCELLATION_ATTEMPTS: usize = 6;

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

/// A deck whose coordinates each cost real solver time, so a sweep of it can
/// be caught after it has staged a coordinate and before it has published any.
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

/// A deck built so that a `--timeout` has a wide interval to land in.
///
/// Each coordinate solves a four-thousand-point transient, which costs far
/// more than the planning and preflight that precede the first one, and there
/// are enough coordinates that the whole sweep costs an order of magnitude
/// more than one of them. The interval between "the first coordinate has
/// finished" and "the last coordinate has finished" is therefore a property
/// of the deck rather than of how fast the host happens to be running.
fn write_cancellable_deck(directory: &Path) -> PathBuf {
    let mut source = String::from(
        "* cancellable transient sweep\n.param rval=1k\nV1 n0 0 PULSE(0 1 0 1u 1u 500u 1m)\n",
    );
    for index in 0..6 {
        source.push_str(&format!("R{index} n{index} n{} {{rval}}\n", index + 1));
    }
    source.push_str("C1 n6 0 1n\nRTAIL n6 0 1k\n.print tran v(n6)\n.step param rval list");
    for coordinate in 0..CANCELLABLE_COORDINATES {
        source.push_str(&format!(" {}", 1_000 + coordinate * 10));
    }
    source.push_str("\n.tran 1u 4m\n.end\n");
    let path = directory.join("cancellable_sweep.sp");
    std::fs::write(&path, source).expect("write cancellable deck");
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
fn assert_complete_set_or_nothing(directory: &Path, coordinates: usize) -> bool {
    let published = published_artifacts(directory);
    if published.is_empty() {
        return false;
    }
    assert!(
        published.contains(&"results.run_set.json".to_owned()),
        "a published coordinate set has no manifest: {published:?}"
    );
    assert!(
        published.contains(&"results.step_schema.json".to_owned()),
        "a published coordinate set has no schema manifest: {published:?}"
    );
    // One artifact per coordinate, plus the schema manifest and the set
    // manifest, both of which are members of the same transaction.
    assert_eq!(
        published.len(),
        coordinates + 2,
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
    let deck = write_cancellable_deck(directory.path());
    let requested = directory.path().join("results.json");

    // What a complete sweep of this deck costs here, which is the only
    // budget known to be too large to cancel anything.
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
        assert_complete_set_or_nothing(directory.path(), CANCELLABLE_COORDINATES),
        "a successful sweep published nothing"
    );
    assert_eq!(
        exported_coordinate_count(&complete),
        CANCELLABLE_COORDINATES
    );
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(directory.path().join("results.run_set.json")).expect("read set manifest"),
    )
    .expect("parse set manifest");
    assert_eq!(manifest["kind"], "axis_coordinate_set");
    assert_eq!(manifest["coordinate_count"], CANCELLABLE_COORDINATES);
    let coordinates = manifest["coordinates"]
        .as_array()
        .expect("manifest coordinates");
    assert_eq!(coordinates.len(), CANCELLABLE_COORDINATES);
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

    // Cancel fresh sweeps until one is cancelled after a coordinate finished.
    // The budget is bracketed by what the child actually did rather than
    // extrapolated from a single measurement: a run cancelled before its
    // first coordinate finished raises the floor, a run that completed the
    // whole sweep lowers the ceiling, and the next budget is the geometric
    // middle of what is left. A loaded host moves both bounds together, so it
    // costs attempts and never a verdict — and every attempt, whichever way
    // it lands, is held to the whole-set-or-nothing contract.
    let mut floor = 0.0_f64;
    let mut ceiling = sweep_seconds;
    let mut budget = sweep_seconds / (CANCELLABLE_COORDINATES as f64).sqrt();
    for _ in 0..CANCELLATION_ATTEMPTS {
        let cancelled_directory = TestDirectory::new("cancelled");
        let cancelled_deck = write_cancellable_deck(cancelled_directory.path());
        let cancelled_output = cancelled_directory.path().join("results.json");
        let cancelled = run(&cancelled_deck, &cancelled_output, Some(budget));

        match cancelled.status.code() {
            Some(124) => {
                assert!(
                    !assert_complete_set_or_nothing(
                        cancelled_directory.path(),
                        CANCELLABLE_COORDINATES
                    ),
                    "a cancelled sweep published artifacts: {:?}",
                    published_artifacts(cancelled_directory.path())
                );
                assert!(
                    staging_files(cancelled_directory.path()).is_empty(),
                    "a cancelled sweep left staging files: {:?}",
                    staging_files(cancelled_directory.path())
                );
                if exported_coordinate_count(&cancelled) > 0 {
                    // The interesting case: coordinates completed and were
                    // discarded rather than published as a shorter sweep.
                    return;
                }
                floor = budget;
            }
            Some(0) => {
                assert!(
                    assert_complete_set_or_nothing(
                        cancelled_directory.path(),
                        CANCELLABLE_COORDINATES
                    ),
                    "a sweep that outran its budget published nothing"
                );
                ceiling = budget;
            }
            // A run stopped by its deadline reports the timeout, whatever it
            // was doing when the deadline arrived. Any other status means a
            // cancellation was reported as something else.
            other => panic!(
                "a {budget}s budget produced exit status {other:?} instead of 0 or 124:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&cancelled.stdout),
                String::from_utf8_lossy(&cancelled.stderr)
            ),
        }
        assert!(
            floor < ceiling,
            "the cancellation budget interval collapsed: [{floor}, {ceiling}]"
        );
        budget = if floor > 0.0 {
            (floor * ceiling).sqrt()
        } else {
            ceiling / (CANCELLABLE_COORDINATES as f64).sqrt()
        };
    }
    panic!(
        "no budget in {CANCELLATION_ATTEMPTS} attempts cancelled the sweep after a completed coordinate"
    );
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
