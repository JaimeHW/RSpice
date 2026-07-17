//! Production lifecycle checks for Xyce ADDRESISTORS derived-copy artifacts.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn join(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }
}

impl AsRef<Path> for TestDirectory {
    fn as_ref(&self) -> &Path {
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
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "rspice_addresistors_artifact_{}_{}_{}",
        std::process::id(),
        tag,
        id
    ));
    std::fs::create_dir_all(&directory).expect("create isolated test directory");
    TestDirectory(directory)
}

fn artifact_path(deck: &Path) -> PathBuf {
    let mut path = deck.as_os_str().to_os_string();
    path.push("_xyce.cir");
    PathBuf::from(path)
}

fn run_file(deck: &Path, extra: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.arg("--quiet").arg("run").arg(deck).args(extra);
    command.output().expect("run rspice")
}

#[test]
fn file_run_writes_exact_atomic_xyce_sibling_and_summary_entry() {
    let directory = test_dir("exact");
    let deck = directory.join("leaf.input.cir");
    std::fs::write(
        &deck,
        "artifact\r\n.PREPROCESS\r\n+ ADDRESISTORS ONETERMINAL 2k\r\nV1 leaf 0 1\r\n.END\r\n",
    )
    .expect("write input deck");
    let summary = directory.join("summary.json");
    let output = run_file(&deck, &["--summary", summary.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let artifact = artifact_path(&deck);
    assert_eq!(
        std::fs::read_to_string(&artifact).expect("read derived artifact"),
        "artifact\r\n* .PREPROCESS\r\n* + ADDRESISTORS ONETERMINAL 2k\r\nV1 leaf 0 1\r\nRONETERM1 LEAF 0 2k\r\n.END\r\n"
    );
    let summary_text = std::fs::read_to_string(summary).expect("read run summary");
    let summary_json: serde_json::Value =
        serde_json::from_str(&summary_text).expect("parse run summary");
    assert!(
        summary_json["outputs"]
            .as_array()
            .expect("outputs array")
            .iter()
            .any(|value| value.as_str() == Some(artifact.to_str().unwrap()))
    );

    // A second successful run replaces an ordinary prior artifact instead of
    // failing or exposing a partially rewritten file.
    std::fs::write(&artifact, "stale").expect("seed stale artifact");
    let second = run_file(&deck, &[]);
    assert!(
        second.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(
        std::fs::read_to_string(&artifact)
            .expect("read replacement")
            .ends_with("RONETERM1 LEAF 0 2k\r\n.END\r\n")
    );
}

#[test]
fn zero_candidate_policy_still_writes_a_valid_copy() {
    let directory = test_dir("zero");
    let deck = directory.join("zero.sp");
    std::fs::write(
        &deck,
        "zero\n.PREPROCESS ADDRESISTORS NODCPATH {UNUSED+}\nV1 in 0 1\nR1 in out 1k\n.END\n",
    )
    .expect("write zero-candidate deck");
    let output = run_file(&deck, &[]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(artifact_path(&deck)).expect("read zero-copy artifact"),
        "zero\n* .PREPROCESS ADDRESISTORS NODCPATH {UNUSED+}\nV1 in 0 1\nR1 in out 1k\n.END\n"
    );
}

#[test]
fn hierarchical_artifact_uses_xyce_separator_and_replays_as_the_same_node() {
    let directory = test_dir("hierarchy");
    let deck = directory.join("hierarchy.cir");
    std::fs::write(
        &deck,
        "hierarchy\n\
         .PREPROCESS ADDRESISTORS ONETERMINAL 2k\n\
         .SUBCKT CELL a\n\
         C1 a leaf 1u IC=0\n\
         .ENDS CELL\n\
         V1 in 0 1\n\
         X1 in CELL\n\
         .END\n",
    )
    .expect("write hierarchical deck");

    let output = run_file(&deck, &[]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let artifact = artifact_path(&deck);
    let artifact_source = std::fs::read_to_string(&artifact).expect("read hierarchy artifact");
    assert!(
        artifact_source.contains("RONETERM1 X1:LEAF 0 2k\n"),
        "artifact did not use canonical Xyce hierarchy: {artifact_source}"
    );
    assert!(!artifact_source.contains("RONETERM1 X1.LEAF"));

    let replay = run_file(&artifact, &[]);
    assert!(
        replay.status.success(),
        "canonical Xyce artifact must replay successfully; stderr: {}",
        String::from_utf8_lossy(&replay.stderr)
    );
}

#[test]
fn destination_failure_is_safe_and_leaves_no_temporary_file() {
    let directory = test_dir("failure");
    let deck = directory.join("blocked.cir");
    std::fs::write(
        &deck,
        "blocked\n.PREPROCESS ADDRESISTORS ONETERMINAL 1k\nV1 leaf 0 1\n.END\n",
    )
    .expect("write blocked deck");
    let artifact = artifact_path(&deck);
    std::fs::create_dir(&artifact).expect("create blocking destination directory");

    let output = run_file(&deck, &[]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("Failed to write Xyce ADDRESISTORS derived netlist"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(artifact.is_dir());
    let temporary_prefix = format!("{}.tmp.", artifact.file_name().unwrap().to_string_lossy());
    assert!(
        std::fs::read_dir(&directory)
            .expect("list fixture")
            .all(|entry| !entry
                .expect("directory entry")
                .file_name()
                .to_string_lossy()
                .starts_with(&temporary_prefix))
    );
}

#[test]
fn stdin_and_multi_run_addresistors_are_rejected_before_artifact_creation() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn stdin run");
    child
        .stdin
        .as_mut()
        .expect("stdin pipe")
        .write_all(b"stdin\n.PREPROCESS ADDRESISTORS ONETERMINAL 1k\nV1 leaf 0 1\n.END\n")
        .expect("write stdin deck");
    let stdin_output = child.wait_with_output().expect("wait for stdin run");
    assert!(!stdin_output.status.success());
    assert!(
        String::from_utf8_lossy(&stdin_output.stderr).contains("file-backed input"),
        "stderr: {}",
        String::from_utf8_lossy(&stdin_output.stderr)
    );

    let directory = test_dir("multi");
    let deck = directory.join("multi.cir");
    std::fs::write(
        &deck,
        "multi\n.PREPROCESS ADDRESISTORS ONETERMINAL 1k\nV1 leaf 0 1\n.ALTER second\nV1 leaf 0 2\n.END\n",
    )
    .expect("write multi-run deck");
    let multi_output = run_file(&deck, &["--jobs", "2"]);
    assert!(!multi_output.status.success());
    assert!(
        String::from_utf8_lossy(&multi_output.stderr).contains("ambiguous in a multi-run"),
        "stderr: {}",
        String::from_utf8_lossy(&multi_output.stderr)
    );
    assert!(!artifact_path(&deck).exists());
}

#[cfg(unix)]
#[test]
fn artifact_destination_symlink_is_never_followed() {
    use std::os::unix::fs::symlink;

    let directory = test_dir("symlink");
    let deck = directory.join("symlink.cir");
    std::fs::write(
        &deck,
        "symlink\n.PREPROCESS ADDRESISTORS ONETERMINAL 1k\nV1 leaf 0 1\n.END\n",
    )
    .expect("write symlink deck");
    let victim = directory.join("victim.cir");
    std::fs::write(&victim, "untouched").expect("write victim");
    symlink(&victim, artifact_path(&deck)).expect("create artifact symlink");

    let output = run_file(&deck, &[]);
    assert!(!output.status.success());
    assert_eq!(std::fs::read_to_string(victim).unwrap(), "untouched");
}

#[cfg(windows)]
#[test]
fn artifact_destination_windows_symlink_is_never_followed_when_available() {
    use std::os::windows::fs::symlink_file;

    let directory = test_dir("windows_symlink");
    let deck = directory.join("symlink.cir");
    std::fs::write(
        &deck,
        "symlink\n.PREPROCESS ADDRESISTORS ONETERMINAL 1k\nV1 leaf 0 1\n.END\n",
    )
    .expect("write symlink deck");
    let victim = directory.join("victim.cir");
    std::fs::write(&victim, "untouched").expect("write victim");
    let artifact = artifact_path(&deck);
    if let Err(error) = symlink_file(&victim, &artifact) {
        eprintln!("Windows symlink privilege unavailable; skipping runtime assertion: {error}");
        return;
    }

    let output = run_file(&deck, &[]);
    assert!(!output.status.success());
    assert_eq!(std::fs::read_to_string(victim).unwrap(), "untouched");
    assert!(
        std::fs::symlink_metadata(artifact)
            .expect("artifact symlink remains an untouched directory entry")
            .file_type()
            .is_symlink()
    );
}
