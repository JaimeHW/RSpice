//! The CLI has no execution route for the periodic large-signal family, so an
//! authored `.PSS`/`.PAC`/`.PNOISE`/`.ENVELOPE` card is refused with a typed
//! unsupported-capability error before anything is written.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

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
        "rspice_periodic_cards_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&path).expect("create test directory");
    TestDirectory(path)
}

const CIRCUIT: &str = "* periodic card refusal\n\
                       V1 in 0 SIN(0 1 1G)\n\
                       R1 in out 1k\n\
                       C1 out 0 1p\n";

fn run_deck(dir: &Path, cards: &str, extra: &[&str]) -> (Output, PathBuf) {
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, format!("{CIRCUIT}{cards}.END\n")).expect("write deck");
    let output_path = dir.join("result.csv");
    let mut args: Vec<String> = vec![
        "--quiet".to_string(),
        "--error-format".to_string(),
        "json".to_string(),
        "run".to_string(),
        deck.to_string_lossy().into_owned(),
        "-o".to_string(),
        output_path.to_string_lossy().into_owned(),
        "-f".to_string(),
        "csv".to_string(),
    ];
    args.extend(extra.iter().map(|arg| (*arg).to_string()));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(&args)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE")
        .env_remove("RUST_LOG")
        .output()
        .expect("run rspice");
    (output, output_path)
}

fn error_document(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stderr).unwrap_or_else(|error| {
        panic!(
            "stderr must be exactly one JSON document: {error}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn assert_typed_refusal(cards: &str, card: &str, analysis_id: &str, tag: &str) {
    let dir = test_dir(tag);
    let (output, artifact) = run_deck(&dir, cards, &[]);

    assert_eq!(
        output.status.code(),
        Some(65),
        "refusal must use the input-error exit status; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json = error_document(&output);
    assert_eq!(json["error"]["code"], "unsupported_deck_analysis");
    assert_eq!(json["error"]["category"], "unsupported_capability");
    assert_eq!(json["error"]["retryable"], false);
    assert_eq!(json["error"]["analysis"], analysis_id);
    let message = json["error"]["message"]
        .as_str()
        .expect("a refusal names the card");
    assert!(
        message.contains(card) && message.contains(analysis_id),
        "refusal must name the card and its instance: {message}"
    );
    assert!(
        !artifact.exists(),
        "a refused deck must publish no artifact, found {}",
        artifact.display()
    );
}

#[test]
fn authored_periodic_cards_are_refused_with_their_analysis_identity() {
    assert_typed_refusal(".PSS FUND=1G\n", ".PSS", "pss-001", "pss");
    assert_typed_refusal(
        ".HB 1G\n.PAC DEC 5 1k 1meg INPUT=V1 OUT=V(out)\n",
        ".PAC",
        "pac-001",
        "pac",
    );
    assert_typed_refusal(
        ".HB 1G\n.PNOISE DEC 5 1 1k OUT=V(out)\n",
        ".PNOISE",
        "pnoise-001",
        "pnoise",
    );
    assert_typed_refusal(
        ".HB 1G\n.ENVELOPE TSTOP=1u\n",
        ".ENVELOPE",
        "env-001",
        "envelope",
    );
}

#[test]
fn a_stepped_periodic_card_keeps_its_ordinal_in_the_refusal() {
    assert_typed_refusal(
        ".STEP PARAM rload LIST 1k 2k\n.OP\n.PSS FUND=1G\n.PSS FUND=2G\n",
        ".PSS",
        "pss-001",
        "stepped",
    );
}

#[test]
fn a_periodic_card_is_refused_before_a_preceding_transient_publishes() {
    let dir = test_dir("before_tran");
    let (output, artifact) = run_deck(&dir, ".TRAN 1n 10n\n.PSS FUND=1G\n", &[]);

    assert_eq!(output.status.code(), Some(65));
    assert_eq!(
        error_document(&output)["error"]["code"],
        "unsupported_deck_analysis"
    );
    assert!(
        !artifact.exists(),
        "the transient must not publish when a later card is refused"
    );
    let tagged = dir.join("result.tran.csv");
    assert!(!tagged.exists(), "no tagged transient artifact either");
}

#[test]
fn the_pss_flag_and_an_authored_pss_card_are_an_explicit_conflict() {
    let dir = test_dir("flag_conflict");
    let (output, artifact) = run_deck(&dir, ".PSS FUND=1G\n", &["--pss-freq", "1e9"]);

    assert_eq!(
        output.status.code(),
        Some(2),
        "a command-line/deck conflict is a usage error; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json = error_document(&output);
    assert_eq!(json["error"]["code"], "invalid_argument");
    let message = json["error"]["message"]
        .as_str()
        .expect("the conflict names both sources");
    assert!(
        message.contains("--pss-freq") && message.contains(".PSS"),
        "unexpected conflict message: {message}"
    );
    assert!(!artifact.exists(), "a refused deck publishes no artifact");
}

#[test]
fn the_pss_flag_still_runs_a_deck_without_an_authored_card() {
    let dir = test_dir("flag_only");
    let (output, artifact) = run_deck(&dir, ".OP\n", &["--pss-freq", "1e9"]);

    assert_eq!(
        output.status.code(),
        Some(0),
        "the flag route is unchanged; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        artifact.exists(),
        "the flag route still publishes its artifact"
    );
}
