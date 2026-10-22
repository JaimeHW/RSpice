//! Xyce Regression Corpus Tests
//!
//! The Xyce corpus is vendored under `tests/xyce`. These tests run the
//! Rust-native RSpice adapter, not the upstream Perl/Bash harness. Upstream
//! platform scripts are intentionally trimmed from this corpus.
//!
//! The harness is unconditional. It lives in `rspice-conformance`, which no
//! shipping crate depends on (enforced by `tools/ci/test_ci_configuration.py`),
//! so the GPL-3 corpus never reaches a product binary.

use rspice_conformance::suites::xyce::{
    XyceDeck, XyceDeckSection, XyceRunnerConfig, XyceTestResult, XyceTestRunner,
    XyceTransientGridAlignment, XyceValueMismatch,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const XYCE_CASE_RUNNER_EXE: &str = env!("CARGO_BIN_EXE_rspice-xyce-case-runner");
const XYCE_AGGREGATE_HARD_CASE_TIMEOUT_MS: u128 = 30_000;
const XYCE_CASE_RUNNER_RESULT_GRACE_MS: u128 = 1_000;
const XYCE_PROGRESS_INITIAL_DELAY: Duration = Duration::from_secs(2);
const XYCE_PROGRESS_INTERVAL: Duration = Duration::from_secs(10);
const XYCE_CLASSIFICATION_REPORT_ENV: &str = "RSPICE_XYCE_CLASSIFICATION_REPORT";
const XYCE_CONTRACT_MANIFEST_FILE: &str = "contract-manifest.tsv";
/// Contracts the watchdog here writes when the case runner itself failed. Such
/// a result says nothing about the deck's contract, so the contract manifest
/// skips it and `failed == 0` reports it.
const XYCE_HARNESS_FAILURE_CONTRACTS: [&str; 4] = [
    "case_runner_spawn_failure",
    "case_runner_result_failure",
    "hard_process_timeout",
    "case_runner_poll_failure",
];
/// Appended to a contract-manifest failure: what the pin is and how to move it.
const XYCE_CONTRACT_REBASELINE: &str = "\
The contract manifest pins, for every deck, whether it executes, is expected \
unsupported, or is upstream-excluded, and which contract judges it. To \
re-baseline after a deliberate change, capture a fresh census:\n  \
RSPICE_XYCE_CLASSIFICATION_REPORT=<report.tsv> cargo test --release -p rspice-conformance \\\n    \
--test xyce_regression test_full_xyce_suite_summary_accounts_for_every_deck\n\
then copy the report over tests/xyce/contract-manifest.tsv and say in the \
commit message why each changed row is right.";
const XYCE_GRID_ALIGNMENT_REPORT_ENV: &str = "RSPICE_XYCE_GRID_REPORT";
const XYCE_GRID_ALIGNMENT_MANIFEST_FILE: &str = "grid-alignment-manifest.tsv";

/// Appended to every grid-alignment failure. Says what moved and what the
/// deliberate response is, because the failing edit is almost never in this
/// file or in the deck it names.
const XYCE_GRID_ALIGNMENT_REBASELINE: &str = "\
Index alignment is the strictest claim this suite makes about a transient — \
reference row i against accepted step i, so the engine had to choose the same \
steps Xyce chose. Interpolating onto the reference times instead is a \
materially weaker claim, and nothing in a deck, a tolerance, or a contract has \
to change for a deck to slide from one to the other: it follows from the \
engine's accepted-step sequence alone. That is why it is pinned.\n\
To re-baseline, capture a fresh census and rewrite the manifest from it:\n  \
RSPICE_XYCE_GRID_REPORT=<report.tsv> cargo test --release -p rspice-conformance \\\n    \
--test xyce_regression test_full_xyce_suite_summary_accounts_for_every_deck\n\
then copy the report's `index_aligned` rows into \
tests/xyce/grid-alignment-manifest.tsv, keeping the header. Record in the \
commit message which engine change moved the grid, and why the weaker \
comparison is acceptable for every deck that lost the strict one — a manifest \
regenerated without that judgement is a gate that has been turned off.";

fn run_xyce_case_with_watchdog(
    test_dir: &Path,
    config: &XyceRunnerConfig,
    deck: &XyceDeck,
    index: usize,
    total: usize,
) -> XyceTestResult {
    let start = Instant::now();
    let hard_timeout_ms = xyce_hard_case_timeout_ms(config);
    let child_timeout_ms = hard_timeout_ms
        .saturating_sub(XYCE_CASE_RUNNER_RESULT_GRACE_MS)
        .max(1);
    let result_path = unique_xyce_case_result_path(&deck.path);
    let mut child = match Command::new(XYCE_CASE_RUNNER_EXE)
        .arg("--test-dir")
        .arg(test_dir)
        .arg("--circuit")
        .arg(&deck.path)
        .arg("--result")
        .arg(&result_path)
        .arg("--relative-tolerance")
        .arg(config.relative_tolerance.to_string())
        .arg("--absolute-tolerance")
        .arg(config.absolute_tolerance.to_string())
        .arg("--voltage-absolute-tolerance")
        .arg(config.voltage_absolute_tolerance.to_string())
        .arg("--power-absolute-tolerance")
        .arg(config.power_absolute_tolerance.to_string())
        .arg("--max-mismatches")
        .arg(config.max_mismatches.to_string())
        .arg("--max-time-per-test-ms")
        .arg(child_timeout_ms.to_string())
        .arg("--verbose")
        .arg(config.verbose.to_string())
        .stdout(if config.verbose {
            Stdio::inherit()
        } else {
            Stdio::null()
        })
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            return xyce_watchdog_error_result(
                deck,
                start.elapsed().as_millis(),
                "case_runner_spawn_failure",
                format!("Failed to spawn Xyce case runner: {err}"),
            );
        }
    };

    let timeout = Duration::from_millis(hard_timeout_ms.min(u64::MAX as u128) as u64);
    let mut next_progress = XYCE_PROGRESS_INITIAL_DELAY;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let decoded = match fs::read_to_string(&result_path) {
                    Ok(content) => decode_xyce_test_result(&content)
                        .map_err(|err| format!("Case runner wrote an unreadable result: {err}")),
                    Err(err) => Err(format!(
                        "Case runner exited with status {status}, but wrote no result: {err}"
                    )),
                };
                let _ = fs::remove_file(&result_path);
                return match decoded {
                    Ok(result) => result,
                    Err(err) => xyce_watchdog_error_result(
                        deck,
                        start.elapsed().as_millis(),
                        "case_runner_result_failure",
                        err,
                    ),
                };
            }
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&result_path);
                println!(
                    "Xyce progress {index}/{total}: {} exceeded hard timeout ({}ms)",
                    deck.relative_path, hard_timeout_ms
                );
                let _ = io::stdout().flush();
                return xyce_watchdog_error_result(
                    deck,
                    start.elapsed().as_millis(),
                    "hard_process_timeout",
                    format!("Deck exceeded hard process timeout ({}ms)", hard_timeout_ms),
                );
            }
            Ok(None) => {
                let elapsed = start.elapsed();
                if elapsed >= next_progress {
                    println!(
                        "Xyce progress {index}/{total}: {} still running after {:.1}s",
                        deck.relative_path,
                        elapsed.as_secs_f64()
                    );
                    let _ = io::stdout().flush();
                    next_progress += XYCE_PROGRESS_INTERVAL;
                }
                let remaining = timeout.saturating_sub(elapsed);
                thread::sleep(remaining.min(Duration::from_millis(50)));
            }
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&result_path);
                return xyce_watchdog_error_result(
                    deck,
                    start.elapsed().as_millis(),
                    "case_runner_poll_failure",
                    format!("Failed to poll Xyce case runner: {err}"),
                );
            }
        }
    }
}

fn xyce_hard_case_timeout_ms(config: &XyceRunnerConfig) -> u128 {
    let hard_cap_ms = std::env::var("RSPICE_XYCE_HARD_CASE_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u128>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(XYCE_AGGREGATE_HARD_CASE_TIMEOUT_MS);
    config.max_time_per_test_ms.max(1).min(hard_cap_ms)
}

fn xyce_watchdog_error_result(
    deck: &XyceDeck,
    duration_ms: u128,
    contract: &str,
    error: String,
) -> XyceTestResult {
    XyceTestResult {
        name: deck
            .path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        relative_path: deck.relative_path.clone(),
        passed: false,
        expected_unsupported: false,
        upstream_excluded: false,
        upstream_exclusion_source: None,
        error: Some(error),
        mismatches: Vec::new(),
        duration_ms,
        contract: contract.to_string(),
        transient_grid_alignment: None,
    }
}

fn unique_xyce_case_result_path(circuit_path: &Path) -> PathBuf {
    static RESULT_ID: AtomicU64 = AtomicU64::new(0);
    let stem = circuit_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let id = RESULT_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "rspice-xyce-{stem}-{}-{timestamp}-{id}.result",
        std::process::id()
    ))
}

fn write_xyce_classification_report_if_requested(
    results: &[XyceTestResult],
) -> io::Result<Option<PathBuf>> {
    let Some(path) = std::env::var_os(XYCE_CLASSIFICATION_REPORT_ENV).map(PathBuf::from) else {
        return Ok(None);
    };
    if path.as_os_str().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{XYCE_CLASSIFICATION_REPORT_ENV} must not be empty"),
        ));
    }
    write_xyce_classification_report(results, &path)?;
    Ok(Some(path))
}

/// How the suite disposed of one deck, as the contract manifest records it.
fn xyce_contract_outcome(result: &XyceTestResult) -> &'static str {
    if result.upstream_excluded {
        "upstream_excluded"
    } else if result.expected_unsupported {
        "expected_unsupported"
    } else {
        "executed"
    }
}

/// Dump `deck -> outcome, contract` for every deck in the format of
/// `tests/xyce/contract-manifest.tsv`, which is regenerated from this report.
fn write_xyce_classification_report(
    results: &[XyceTestResult],
    output_path: &Path,
) -> io::Result<()> {
    let mut rows = BTreeMap::new();
    let mut normalized_paths = BTreeSet::new();
    for result in results {
        validate_xyce_classification_field("relative_path", &result.relative_path)?;
        validate_xyce_classification_field("contract", &result.contract)?;
        if !normalized_paths.insert(result.relative_path.to_ascii_lowercase()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "classification report contains a case-insensitive duplicate: {}",
                    result.relative_path
                ),
            ));
        }
        rows.insert(
            result.relative_path.as_str(),
            (xyce_contract_outcome(result), result.contract.as_str()),
        );
    }

    let mut report = String::from("# relative_path\toutcome\tcontract\n");
    for (relative_path, (outcome, contract)) in rows {
        report.push_str(relative_path);
        report.push('\t');
        report.push_str(outcome);
        report.push('\t');
        report.push_str(contract);
        report.push('\n');
    }

    let parent = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(report.as_bytes())?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary
        .persist_noclobber(output_path)
        .map_err(|error| error.error)?;
    Ok(())
}

/// Dump `deck -> comparison geometry` for every deck that reached a transient
/// reference comparison, when `RSPICE_XYCE_GRID_REPORT` names a destination.
///
/// This is the manifest's generator. It runs before the suite's assertions so
/// a census is still captured on a run the manifest itself fails — otherwise
/// re-baselining would be impossible by construction.
fn write_xyce_grid_alignment_report_if_requested(
    results: &[XyceTestResult],
) -> io::Result<Option<PathBuf>> {
    let Some(path) = std::env::var_os(XYCE_GRID_ALIGNMENT_REPORT_ENV).map(PathBuf::from) else {
        return Ok(None);
    };
    if path.as_os_str().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{XYCE_GRID_ALIGNMENT_REPORT_ENV} must not be empty"),
        ));
    }
    write_xyce_grid_alignment_report(results, &path)?;
    Ok(Some(path))
}

fn write_xyce_grid_alignment_report(
    results: &[XyceTestResult],
    output_path: &Path,
) -> io::Result<()> {
    let mut rows = BTreeSet::new();
    for result in results {
        let Some(alignment) = result.transient_grid_alignment else {
            continue;
        };
        validate_xyce_classification_field("relative_path", &result.relative_path)?;
        if !rows.insert((result.relative_path.clone(), alignment.as_str())) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "grid-alignment report contains a duplicate deck: {}",
                    result.relative_path
                ),
            ));
        }
    }

    let mut report = String::from("# relative_path\tgrid_alignment\n");
    for (relative_path, alignment) in rows {
        report.push_str(&relative_path);
        report.push('\t');
        report.push_str(alignment);
        report.push('\n');
    }

    let parent = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(report.as_bytes())?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary
        .persist_noclobber(output_path)
        .map_err(|error| error.error)?;
    Ok(())
}

/// The decks entitled to index-aligned comparison, read from
/// `tests/xyce/grid-alignment-manifest.tsv`.
///
/// Parsing is strict — malformed, unsorted, duplicated, or non-`index_aligned`
/// rows are failures rather than warnings. A manifest that tolerates junk is a
/// manifest that can be half-edited without anyone noticing.
fn read_xyce_grid_alignment_manifest(root: &Path) -> BTreeSet<String> {
    let path = root.join(XYCE_GRID_ALIGNMENT_MANIFEST_FILE);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let expected_mode = XyceTransientGridAlignment::IndexAligned.as_str();
    let mut manifest: BTreeSet<String> = BTreeSet::new();
    let mut previous: Option<String> = None;
    for (index, raw_line) in content.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim_end_matches('\r');
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((relative_path, mode)) = line.split_once('\t') else {
            panic!(
                "{}:{line_number}: expected `<relative_path>\\t{expected_mode}`, got {line:?}",
                path.display()
            );
        };
        assert_eq!(
            mode,
            expected_mode,
            "{}:{line_number}: the grid-alignment manifest lists only the decks \
             entitled to index alignment, so `{expected_mode}` is the sole legal \
             mode; got {mode:?}",
            path.display()
        );
        assert!(
            !relative_path.is_empty() && !relative_path.contains('\\'),
            "{}:{line_number}: {relative_path:?} must be a nonempty `/`-separated \
             path relative to tests/xyce",
            path.display()
        );
        assert!(
            previous
                .as_deref()
                .is_none_or(|earlier| earlier < relative_path),
            "{}:{line_number}: rows must be sorted and unique; {relative_path:?} \
             does not follow {previous:?}",
            path.display()
        );
        previous = Some(relative_path.to_string());
        manifest.insert(relative_path.to_string());
    }
    manifest
}

/// Check the manifest against a census, in both directions.
///
/// A manifested deck that has fallen back to interpolation is the regression
/// this gate exists for. An unmanifested deck that has *started* comparing
/// index-aligned fails too: an allowlist that only ever grows stale stops
/// describing anything, and a deck that silently gained the strict mode is a
/// deck whose new strictness nobody signed off on either.
fn xyce_grid_alignment_failures(
    results: &[XyceTestResult],
    manifest: &BTreeSet<String>,
) -> Vec<String> {
    let mut failures = Vec::new();
    let mut seen = BTreeSet::new();
    for result in results {
        seen.insert(result.relative_path.as_str());
        let manifested = manifest.contains(&result.relative_path);
        match (manifested, result.transient_grid_alignment) {
            (true, Some(XyceTransientGridAlignment::IndexAligned)) => {}
            (false, None | Some(XyceTransientGridAlignment::Interpolated)) => {}
            (true, Some(XyceTransientGridAlignment::Interpolated)) => failures.push(format!(
                "{}: manifested as index-aligned, but its transient comparison fell \
                 back to interpolation.\n{XYCE_GRID_ALIGNMENT_REBASELINE}",
                result.relative_path
            )),
            (true, None) => failures.push(format!(
                "{}: manifested as index-aligned, but it no longer reaches a \
                 transient reference comparison at all (contract {:?}). Either the \
                 deck's contract changed or it stopped executing; both retire a \
                 pinned strict comparison.\n{XYCE_GRID_ALIGNMENT_REBASELINE}",
                result.relative_path, result.contract
            )),
            (false, Some(XyceTransientGridAlignment::IndexAligned)) => failures.push(format!(
                "{}: compares index-aligned but is absent from the manifest. A deck \
                 that gains the strict comparison must be recorded, or the manifest \
                 stops describing the suite.\n{XYCE_GRID_ALIGNMENT_REBASELINE}",
                result.relative_path
            )),
        }
    }
    for relative_path in manifest {
        if !seen.contains(relative_path.as_str()) {
            failures.push(format!(
                "{relative_path}: named by the grid-alignment manifest but not \
                 discovered in the corpus. Remove the row, or restore the deck.\n\
                 {XYCE_GRID_ALIGNMENT_REBASELINE}"
            ));
        }
    }
    failures
}

/// `deck -> (outcome, contract)` read from `tests/xyce/contract-manifest.tsv`.
///
/// Parsing is strict, like the grid-alignment manifest: malformed, unsorted,
/// or duplicated rows and unknown outcomes are failures.
fn read_xyce_contract_manifest(root: &Path) -> BTreeMap<String, (String, String)> {
    let path = root.join(XYCE_CONTRACT_MANIFEST_FILE);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let mut manifest = BTreeMap::new();
    let mut previous: Option<String> = None;
    for (index, raw_line) in content.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim_end_matches('\r');
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        let [relative_path, outcome, contract] = fields[..] else {
            panic!(
                "{}:{line_number}: expected `<relative_path>\\t<outcome>\\t<contract>`, got {line:?}",
                path.display()
            );
        };
        assert!(
            matches!(
                outcome,
                "executed" | "expected_unsupported" | "upstream_excluded"
            ),
            "{}:{line_number}: unknown outcome {outcome:?}",
            path.display()
        );
        assert!(
            !relative_path.is_empty() && !relative_path.contains('\\') && !contract.is_empty(),
            "{}:{line_number}: needs a `/`-separated deck path and a contract, got {line:?}",
            path.display()
        );
        assert!(
            previous
                .as_deref()
                .is_none_or(|earlier| earlier < relative_path),
            "{}:{line_number}: rows must be sorted and unique; {relative_path:?} \
             does not follow {previous:?}",
            path.display()
        );
        previous = Some(relative_path.to_string());
        manifest.insert(
            relative_path.to_string(),
            (outcome.to_string(), contract.to_string()),
        );
    }
    manifest
}

/// Check every deck's outcome and contract against the manifest, in both
/// directions. `failed == 0` cannot see a deck that slides from its own
/// contract to another passing one, or to expected-unsupported; this can.
fn xyce_contract_manifest_failures(
    results: &[XyceTestResult],
    manifest: &BTreeMap<String, (String, String)>,
) -> Vec<String> {
    let mut failures = Vec::new();
    let mut seen = BTreeSet::new();
    for result in results {
        seen.insert(result.relative_path.as_str());
        if XYCE_HARNESS_FAILURE_CONTRACTS.contains(&result.contract.as_str()) {
            continue;
        }
        let outcome = xyce_contract_outcome(result);
        match manifest.get(&result.relative_path) {
            Some((expected_outcome, expected_contract))
                if expected_outcome == outcome && *expected_contract == result.contract => {}
            Some((expected_outcome, expected_contract)) => failures.push(format!(
                "{}: manifested as {expected_outcome} under {expected_contract}, \
                 but ran as {outcome} under {}",
                result.relative_path, result.contract
            )),
            None => failures.push(format!(
                "{}: ran as {outcome} under {} but is absent from the contract manifest",
                result.relative_path, result.contract
            )),
        }
    }
    for relative_path in manifest.keys() {
        if !seen.contains(relative_path.as_str()) {
            failures.push(format!(
                "{relative_path}: named by the contract manifest but not discovered in the corpus"
            ));
        }
    }
    failures
}

fn validate_xyce_classification_field(name: &str, value: &str) -> io::Result<()> {
    if value.is_empty()
        || value
            .bytes()
            .any(|byte| matches!(byte, b'\t' | b'\r' | b'\n'))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("classification report {name} is not a nonempty TSV field: {value:?}"),
        ));
    }
    Ok(())
}

fn decode_xyce_test_result(encoded: &str) -> Result<XyceTestResult, String> {
    let mut fields = HashMap::new();
    for line in encoded.lines() {
        let (name, value) = line
            .split_once('=')
            .ok_or_else(|| format!("Malformed result field: {line:?}"))?;
        fields.insert(name.to_string(), unescape_xyce_result_field(value)?);
    }

    let name = take_xyce_result_field(&mut fields, "name")?;
    let relative_path = take_xyce_result_field(&mut fields, "relative_path")?;
    let contract = take_xyce_result_field(&mut fields, "contract")?;
    let passed = parse_xyce_result_field::<bool>(&mut fields, "passed")?;
    let expected_unsupported =
        parse_xyce_result_field::<bool>(&mut fields, "expected_unsupported")?;
    let upstream_excluded = parse_xyce_result_field::<bool>(&mut fields, "upstream_excluded")?;
    let upstream_exclusion_source =
        match take_xyce_result_field(&mut fields, "upstream_exclusion_source")? {
            value if value.is_empty() => None,
            value => Some(value),
        };
    let duration_ms = parse_xyce_result_field::<u128>(&mut fields, "duration_ms")?;
    let error = match take_xyce_result_field(&mut fields, "error")? {
        value if value.is_empty() => None,
        value => Some(value),
    };
    let transient_grid_alignment =
        match take_xyce_result_field(&mut fields, "transient_grid_alignment")? {
            value if value.is_empty() => None,
            value => Some(XyceTransientGridAlignment::parse(&value).ok_or_else(|| {
                format!("Invalid result field transient_grid_alignment={value:?}")
            })?),
        };
    let mismatch_count = parse_xyce_result_field::<usize>(&mut fields, "mismatch_count")?;
    let mut mismatches = Vec::with_capacity(mismatch_count);
    for index in 0..mismatch_count {
        let value = take_xyce_result_field(&mut fields, &format!("mismatch.{index}"))?;
        mismatches.push(decode_xyce_value_mismatch(&value)?);
    }

    Ok(XyceTestResult {
        name,
        relative_path,
        passed,
        expected_unsupported,
        upstream_excluded,
        upstream_exclusion_source,
        error,
        mismatches,
        duration_ms,
        contract,
        transient_grid_alignment,
    })
}

fn decode_xyce_value_mismatch(encoded: &str) -> Result<XyceValueMismatch, String> {
    let encoded = encoded
        .strip_prefix("row=")
        .ok_or_else(|| format!("Malformed mismatch row: {encoded:?}"))?;
    let (row, encoded) = encoded
        .split_once(" probe=")
        .ok_or_else(|| format!("Malformed mismatch probe: {encoded:?}"))?;
    let (probe, encoded) = encoded
        .split_once(" expected=")
        .ok_or_else(|| format!("Malformed mismatch expected value: {encoded:?}"))?;
    let (expected, encoded) = encoded
        .split_once(" actual=")
        .ok_or_else(|| format!("Malformed mismatch actual value: {encoded:?}"))?;
    let (actual, relative_error) = encoded
        .split_once(" relative_error=")
        .ok_or_else(|| format!("Malformed mismatch relative error: {encoded:?}"))?;

    Ok(XyceValueMismatch {
        row: row
            .parse()
            .map_err(|err| format!("Invalid mismatch row {row:?}: {err}"))?,
        probe: unescape_xyce_result_field(probe)?,
        expected: expected
            .parse()
            .map_err(|err| format!("Invalid mismatch expected value {expected:?}: {err}"))?,
        actual: actual
            .parse()
            .map_err(|err| format!("Invalid mismatch actual value {actual:?}: {err}"))?,
        relative_error: relative_error
            .parse()
            .map_err(|err| format!("Invalid mismatch relative error {relative_error:?}: {err}"))?,
    })
}

fn take_xyce_result_field(
    fields: &mut HashMap<String, String>,
    name: &str,
) -> Result<String, String> {
    fields
        .remove(name)
        .ok_or_else(|| format!("Missing result field {name:?}"))
}

fn parse_xyce_result_field<T>(fields: &mut HashMap<String, String>, name: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = take_xyce_result_field(fields, name)?;
    value
        .parse()
        .map_err(|err| format!("Invalid result field {name}={value:?}: {err}"))
}

fn unescape_xyce_result_field(value: &str) -> Result<String, String> {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        match chars.next() {
            Some('\\') => output.push('\\'),
            Some('r') => output.push('\r'),
            Some('n') => output.push('\n'),
            Some(other) => return Err(format!("Unknown result escape \\\\{other}")),
            None => return Err("Trailing backslash in result field".to_string()),
        }
    }
    Ok(output)
}

fn xyce_runner_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn lock_xyce_runner() -> MutexGuard<'static, ()> {
    // Assertions run while the serialization guard is held. Recover after a
    // failed assertion so one fixture reports one failure instead of poisoning
    // every later Xyce test in the process.
    xyce_runner_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn get_xyce_tests_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .join("tests")
        .join("xyce")
        .canonicalize()
        .expect("tests/xyce must exist")
}

fn all_circuit_paths(root: &Path) -> BTreeSet<String> {
    fn visit(root: &Path, dir: &Path, paths: &mut BTreeSet<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, paths);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
            {
                paths.insert(
                    path.strip_prefix(root)
                        .expect("path under root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let mut paths = BTreeSet::new();
    visit(root, root, &mut paths);
    paths
}

fn removed_upstream_harness_artifact_paths(root: &Path) -> Vec<String> {
    fn visit(root: &Path, dir: &Path, paths: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, paths);
                continue;
            }

            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let is_removed_artifact = matches!(
                extension.to_ascii_lowercase().as_str(),
                "sh" | "pl" | "pm" | "py" | "tags"
            ) || name.eq_ignore_ascii_case("tags")
                || name.eq_ignore_ascii_case("exclude")
                || name.eq_ignore_ascii_case("run")
                || name.eq_ignore_ascii_case("run_xyce_regression")
                || name.eq_ignore_ascii_case("run_xyce_regressionMP")
                || (name.starts_with("Manifest") && extension.eq_ignore_ascii_case("txt"));

            if is_removed_artifact {
                paths.push(
                    path.strip_prefix(root)
                        .expect("path under root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let mut paths = Vec::new();
    visit(root, root, &mut paths);
    paths.sort();
    paths
}

fn harness_manifest_entries(root: &Path) -> BTreeSet<String> {
    let path = root.join("RSPICE-HARNESS-MANIFEST.tsv");
    let content = fs::read_to_string(&path).expect("read RSpice Xyce harness manifest");
    let mut entries = BTreeSet::new();
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (relative_path, contract) = line
            .split_once('\t')
            .expect("manifest rows use tab-separated path and contract");
        assert_eq!(
            contract, "requires_upstream_wrapper",
            "unexpected Xyce harness manifest contract in {line:?}"
        );
        assert!(
            entries.insert(relative_path.to_string()),
            "duplicate Xyce harness manifest path {relative_path}"
        );
    }
    entries
}

#[test]
fn test_xyce_classification_report_is_deterministic_and_fail_closed() {
    let deck = |name: &str| format!("Netlists/ContractFixture/{name}.cir");
    let results = [
        xyce_contract_fixture(&deck("c"), "executed", "static_prn_transient"),
        xyce_contract_fixture(&deck("a"), "upstream_excluded", "upstream_excluded"),
        xyce_contract_fixture(&deck("b"), "expected_unsupported", "unsupported_fixture"),
    ];
    let directory = tempfile::tempdir().expect("create classification report directory");
    let first = directory.path().join("first.tsv");
    let second = directory.path().join("second.tsv");

    write_xyce_classification_report(&results, &first).expect("write first report");
    write_xyce_classification_report(&results, &second).expect("write second report");
    let first_bytes = fs::read(&first).expect("read first report");
    assert_eq!(first_bytes, fs::read(&second).expect("read second report"));
    assert_eq!(
        String::from_utf8(first_bytes).expect("report is UTF-8"),
        format!(
            "# relative_path\toutcome\tcontract\n\
             {}\tupstream_excluded\tupstream_excluded\n\
             {}\texpected_unsupported\tunsupported_fixture\n\
             {}\texecuted\tstatic_prn_transient\n",
            deck("a"),
            deck("b"),
            deck("c")
        )
    );

    let overwrite = write_xyce_classification_report(&results, &first)
        .expect_err("classification reports must never overwrite an existing census");
    assert_eq!(overwrite.kind(), io::ErrorKind::AlreadyExists);

    let mut duplicate = results;
    duplicate[0].relative_path = duplicate[1].relative_path.to_ascii_uppercase();
    let duplicate_error =
        write_xyce_classification_report(&duplicate, &directory.path().join("duplicate.tsv"))
            .expect_err("case-insensitive duplicate inventory must fail");
    assert_eq!(duplicate_error.kind(), io::ErrorKind::InvalidData);
}

/// A result whose manifest outcome and contract are the given ones.
fn xyce_contract_fixture(relative_path: &str, outcome: &str, contract: &str) -> XyceTestResult {
    XyceTestResult {
        name: "fixture".to_string(),
        relative_path: relative_path.to_string(),
        passed: true,
        expected_unsupported: outcome == "expected_unsupported",
        upstream_excluded: outcome == "upstream_excluded",
        upstream_exclusion_source: None,
        error: None,
        mismatches: Vec::new(),
        duration_ms: 1,
        contract: contract.to_string(),
        transient_grid_alignment: None,
    }
}

#[test]
fn xyce_contract_manifest_names_exactly_the_discovered_decks() {
    let root = get_xyce_tests_dir();
    let manifest = read_xyce_contract_manifest(&root);
    let discovered = XyceTestRunner::new(&root, XyceRunnerConfig::default())
        .discover_tests()
        .into_iter()
        .map(|deck| deck.relative_path)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        manifest.keys().cloned().collect::<BTreeSet<_>>(),
        discovered,
        "the contract manifest must name every discovered deck and nothing else.\n\
         {XYCE_CONTRACT_REBASELINE}"
    );
}

#[test]
fn xyce_contract_manifest_is_enforced_in_both_directions() {
    let deck = |name: &str| format!("Netlists/ContractFixture/{name}.cir");
    let manifest = BTreeMap::from([(
        deck("pinned"),
        ("executed".to_string(), "fixture_contract".to_string()),
    )]);
    let pinned = xyce_contract_fixture(&deck("pinned"), "executed", "fixture_contract");
    assert!(xyce_contract_manifest_failures(std::slice::from_ref(&pinned), &manifest).is_empty());
    // A case-runner failure carries the watchdog's contract, not the deck's.
    let timed_out = xyce_contract_fixture(&deck("pinned"), "executed", "hard_process_timeout");
    assert!(xyce_contract_manifest_failures(&[timed_out], &manifest).is_empty());

    for moved in [
        xyce_contract_fixture(&deck("pinned"), "executed", "generic_contract"),
        xyce_contract_fixture(&deck("pinned"), "expected_unsupported", "fixture_contract"),
    ] {
        let failures = xyce_contract_manifest_failures(&[moved], &manifest);
        assert_eq!(failures.len(), 1);
        assert!(
            failures[0].starts_with(&format!("{}: manifested as executed", deck("pinned"))),
            "{}",
            failures[0]
        );
    }

    let newcomer = xyce_contract_fixture(&deck("newcomer"), "executed", "fixture_contract");
    let failures = xyce_contract_manifest_failures(&[pinned, newcomer], &manifest);
    assert_eq!(failures.len(), 1);
    assert!(
        failures[0].contains("absent from the contract manifest"),
        "{}",
        failures[0]
    );

    let vanished = xyce_contract_manifest_failures(&[], &manifest);
    assert_eq!(vanished.len(), 1);
    assert!(
        vanished[0].contains("not discovered in the corpus"),
        "{}",
        vanished[0]
    );
}

#[test]
fn xyce_grid_alignment_manifest_parses_and_names_real_decks() {
    let root = get_xyce_tests_dir();
    let manifest = read_xyce_grid_alignment_manifest(&root);
    assert!(
        !manifest.is_empty(),
        "the grid-alignment manifest is empty: either every deck lost the strict \
         transient comparison, or the file was truncated. Both need a human.\n\
         {XYCE_GRID_ALIGNMENT_REBASELINE}"
    );
    for relative_path in &manifest {
        assert!(
            relative_path.ends_with(".cir"),
            "{relative_path} is not a deck path"
        );
        assert!(
            root.join(relative_path).is_file(),
            "{relative_path} is manifested as index-aligned but is not in the corpus"
        );
    }
}

#[test]
fn xyce_grid_alignment_manifest_is_enforced_in_both_directions() {
    let result = |relative_path: &str, alignment| XyceTestResult {
        name: "fixture".to_string(),
        relative_path: relative_path.to_string(),
        passed: true,
        expected_unsupported: false,
        upstream_excluded: false,
        upstream_exclusion_source: None,
        error: None,
        mismatches: Vec::new(),
        duration_ms: 1,
        contract: "fixture".to_string(),
        transient_grid_alignment: alignment,
    };
    // Built rather than written out: `test_xyce_hard_coded_deck_paths_use_
    // exact_vendored_case` scans this file's string literals for deck paths
    // and requires each to exist in the corpus. These are fixtures.
    let deck = |name: &str| format!("Netlists/GridFixture/{name}.cir");
    let manifest = BTreeSet::from([deck("pinned")]);

    assert!(
        xyce_grid_alignment_failures(
            &[
                result(
                    &deck("pinned"),
                    Some(XyceTransientGridAlignment::IndexAligned)
                ),
                result(
                    &deck("loose"),
                    Some(XyceTransientGridAlignment::Interpolated)
                ),
                result(&deck("no_transient"), None),
            ],
            &manifest
        )
        .is_empty(),
        "a census matching the manifest must not fail"
    );

    let weakened = xyce_grid_alignment_failures(
        &[result(
            &deck("pinned"),
            Some(XyceTransientGridAlignment::Interpolated),
        )],
        &manifest,
    );
    assert_eq!(weakened.len(), 1);
    assert!(
        weakened[0].starts_with(&format!("{}: manifested as index-aligned", deck("pinned"))),
        "{}",
        weakened[0]
    );
    assert!(weakened[0].contains(XYCE_GRID_ALIGNMENT_REPORT_ENV));

    let retired = xyce_grid_alignment_failures(&[result(&deck("pinned"), None)], &manifest);
    assert_eq!(retired.len(), 1);
    assert!(
        retired[0].contains("no longer reaches a transient reference comparison"),
        "{}",
        retired[0]
    );

    let unmanifested = xyce_grid_alignment_failures(
        &[
            result(
                &deck("pinned"),
                Some(XyceTransientGridAlignment::IndexAligned),
            ),
            result(
                &deck("newcomer"),
                Some(XyceTransientGridAlignment::IndexAligned),
            ),
        ],
        &manifest,
    );
    assert_eq!(unmanifested.len(), 1);
    assert!(
        unmanifested[0].starts_with(&format!("{}: compares index-aligned", deck("newcomer"))),
        "{}",
        unmanifested[0]
    );

    let vanished = xyce_grid_alignment_failures(&[], &manifest);
    assert_eq!(vanished.len(), 1);
    assert!(
        vanished[0].contains("not discovered in the corpus"),
        "{}",
        vanished[0]
    );
}

#[test]
fn xyce_grid_alignment_report_is_sorted_and_fail_closed() {
    let result = |relative_path: &str, alignment| XyceTestResult {
        name: "fixture".to_string(),
        relative_path: relative_path.to_string(),
        passed: true,
        expected_unsupported: false,
        upstream_excluded: false,
        upstream_exclusion_source: None,
        error: None,
        mismatches: Vec::new(),
        duration_ms: 1,
        contract: "fixture".to_string(),
        transient_grid_alignment: alignment,
    };
    let deck = |name: &str| format!("Netlists/GridFixture/{name}.cir");
    let results = [
        result(&deck("z"), Some(XyceTransientGridAlignment::Interpolated)),
        result(&deck("a"), Some(XyceTransientGridAlignment::IndexAligned)),
        result(&deck("skipped"), None),
    ];

    let directory = tempfile::tempdir().expect("create grid-alignment report directory");
    let path = directory.path().join("grid.tsv");
    write_xyce_grid_alignment_report(&results, &path).expect("write grid-alignment report");
    assert_eq!(
        fs::read_to_string(&path).expect("read grid-alignment report"),
        format!(
            "# relative_path\tgrid_alignment\n{}\tindex_aligned\n{}\tinterpolated\n",
            deck("a"),
            deck("z")
        )
    );

    let overwrite = write_xyce_grid_alignment_report(&results, &path)
        .expect_err("grid-alignment reports must never overwrite an existing census");
    assert_eq!(overwrite.kind(), io::ErrorKind::AlreadyExists);

    let duplicate = [
        result(&deck("a"), Some(XyceTransientGridAlignment::IndexAligned)),
        result(&deck("a"), Some(XyceTransientGridAlignment::IndexAligned)),
    ];
    let duplicate_error =
        write_xyce_grid_alignment_report(&duplicate, &directory.path().join("duplicate.tsv"))
            .expect_err("a deck may appear in the census once");
    assert_eq!(duplicate_error.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn test_xyce_corpus_root_is_scoped_under_tests_xyce() {
    let root = get_xyce_tests_dir();
    let normalized = root.to_string_lossy().replace('\\', "/");

    assert!(
        normalized.ends_with("/tests/xyce"),
        "Xyce regression root must be scoped to tests/xyce, got {}",
        root.display()
    );
    assert!(root.join("Netlists").is_dir(), "Netlists directory missing");
    assert!(
        root.join("OutputData").is_dir(),
        "OutputData directory missing"
    );
    assert!(
        !root.join("TestScripts").exists(),
        "upstream TestScripts runner directory must not be vendored into the RSpice corpus"
    );
    assert!(root.join("COPYING").is_file(), "Xyce COPYING file missing");
    assert!(
        root.join("RSPICE-HARNESS-MANIFEST.tsv").is_file(),
        "RSpice Xyce harness manifest missing"
    );
    assert!(
        root.join("RSPICE-UPSTREAM-EXCLUSIONS.tsv").is_file(),
        "RSpice Xyce upstream-exclusion manifest missing"
    );
    assert!(
        root.join("RSPICE-VENDORING.md").is_file(),
        "RSpice vendoring notes missing"
    );
}

#[test]
fn test_xyce_discovery_covers_every_vendored_circuit() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    let discovered = runner
        .discover_tests()
        .into_iter()
        .map(|deck| deck.relative_path)
        .collect::<BTreeSet<_>>();
    let filesystem = all_circuit_paths(&root);

    assert_eq!(
        discovered, filesystem,
        "Every vendored Xyce .cir deck must be discoverable by the harness"
    );
    assert!(
        discovered.iter().any(|path| path.starts_with("Netlists/")),
        "Xyce Netlists corpus must contain simulator decks"
    );
    assert!(
        discovered.iter().all(|path| path.starts_with("Netlists/")),
        "Only runtime simulator decks should remain in the Xyce .cir corpus"
    );
}

#[test]
fn test_xyce_hard_coded_deck_paths_use_exact_vendored_case() {
    let root = get_xyce_tests_dir();
    let filesystem = all_circuit_paths(&root);
    let referenced = include_str!("xyce_regression.rs")
        .split('"')
        .filter(|fragment| fragment.starts_with("Netlists/") && fragment.ends_with(".cir"))
        // A `format!` template is not a hard-coded path. It names a family of
        // decks whose members cannot be resolved here, so each one was
        // reported as a deck the corpus is missing — four of them, which is
        // what this assertion had been failing on rather than on any real
        // drift.
        .filter(|fragment| !fragment.contains(['{', '}']))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let missing = referenced.difference(&filesystem).collect::<Vec<_>>();

    // The filters above decide what this guard looks at, so it has to say it
    // still looks at something. A guard that quietly narrows to nothing
    // passes forever and reports drift never.
    assert!(
        referenced.len() > 500,
        "the deck-path scan found only {} literals; it can no longer see the paths it exists to check",
        referenced.len()
    );
    assert!(
        missing.is_empty(),
        "hard-coded Xyce deck paths must match the vendored path and case exactly: {missing:#?}"
    );
}

#[test]
fn test_xyce_corpus_omits_upstream_platform_harness_artifacts() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let artifacts = removed_upstream_harness_artifact_paths(&root);

    assert!(
        artifacts.is_empty(),
        "Xyce corpus contains upstream platform harness artifacts that RSpice does not execute: {artifacts:#?}"
    );
    assert!(
        !runner.executes_upstream_scripts(),
        "RSpice Xyce tests must not depend on Perl/Bash harness execution"
    );
}

#[test]
fn test_xyce_wrapper_manifest_covers_trimmed_sidecar_contracts() {
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let entries = harness_manifest_entries(&root);

    assert!(
        entries.len() > 1000,
        "wrapper manifest should preserve the removed Xyce .cir.sh sidecar contracts"
    );
    for relative_path in &entries {
        assert!(
            relative_path.starts_with("Netlists/") && relative_path.ends_with(".cir"),
            "wrapper manifest path must name a retained Netlists .cir deck: {relative_path}"
        );
        let deck_path = root.join(relative_path);
        assert!(
            deck_path.is_file(),
            "wrapper manifest path points at a missing deck: {relative_path}"
        );
        let file_name = deck_path
            .file_name()
            .and_then(|value| value.to_str())
            .expect("deck has file name");
        assert!(
            !deck_path.with_file_name(format!("{file_name}.sh")).exists(),
            "wrapper sidecar should be trimmed, not retained: {relative_path}.sh"
        );
        assert!(
            runner.requires_upstream_wrapper(relative_path),
            "runner did not load wrapper manifest entry for {relative_path}"
        );
    }
    assert!(
        !runner.requires_upstream_wrapper("Netlists/RESISTOR/resistor.cir"),
        "plain resistor smoke deck should not require removed wrapper semantics"
    );
}

#[test]
fn test_xyce_bug_568_stepped_lossless_transmission_line_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let config = XyceRunnerConfig {
        max_time_per_test_ms: 180_000,
        // This certification oracle records Xyce Release 2.1 in its README,
        // before derivative-guarded TRA interpolation replaced the legacy
        // three-point quadratic behavior.
        xyce_tra_interpolation: rspice_core::XyceTraInterpolation::LegacyQuadratic,
        ..Default::default()
    };
    let runner = XyceTestRunner::new(&root, config);
    let relative = "Netlists/Certification_Tests/BUG_568/bug_568.cir";

    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported,
        "{relative} should rebuild lossless-line parameters and delay history per step, got {result:?}"
    );
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug546_passive_temperature_analytic_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/BUG_546_SON/tempcap.cir",
        "Netlists/Certification_Tests/BUG_546_SON/tempcap_instance.cir",
        "Netlists/Certification_Tests/BUG_546_SON/tempcap_instance2.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(relative),
            "{relative} must remain an ordinary upstream deck"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should pass its independently derived passive-temperature RC oracle, got {result:?}"
        );
        assert_eq!(
            result.contract, "analytic_passive_temperature_rc_tran",
            "{relative} should report the strict BUG546 analytic contract"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_transient_analysis_expression_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/ISSUE_160/baseline.cir",
        "Netlists/Certification_Tests/ISSUE_160/baseline2.cir",
        "Netlists/Certification_Tests/ISSUE_160/baseline3.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass as the independently simulated numeric member of its transient-analysis expression family, got {result:?}"
        );
        assert_eq!(
            result.contract, "transient_analysis_expression_family_baseline",
            "{relative} should report the strict baseline-family contract"
        );
        assert!(result.mismatches.is_empty());
    }

    for relative in [
        "Netlists/Certification_Tests/ISSUE_160/tranExpr.cir",
        "Netlists/Certification_Tests/ISSUE_160/tranExpr2.cir",
        "Netlists/Certification_Tests/ISSUE_160/tranExpr3.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should reproduce its independently simulated numeric counterpart through parameterized .TRAN fields, got {result:?}"
        );
        assert_eq!(
            result.contract, "transient_analysis_expression_family_wrapper",
            "{relative} should report the strict wrapper-family contract"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_dc_analysis_expression_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/Certification_Tests/ISSUE_536/dcBaseline1.cir",
        "Netlists/Certification_Tests/ISSUE_536/dcDecBaseline.cir",
        "Netlists/Certification_Tests/ISSUE_536/dcListBaseline.cir",
        "Netlists/Certification_Tests/ISSUE_536/dcOctBaseline.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass as the independently simulated numeric member of its DC-analysis expression family, got {result:?}"
        );
        assert_eq!(
            result.contract, "dc_analysis_expression_family_baseline",
            "{relative} should report the strict baseline-family contract"
        );
        assert!(result.mismatches.is_empty());
    }

    for relative in [
        "Netlists/Certification_Tests/ISSUE_536/dcExpr1.cir",
        "Netlists/Certification_Tests/ISSUE_536/dcDecExpr.cir",
        "Netlists/Certification_Tests/ISSUE_536/dcListExpr.cir",
        "Netlists/Certification_Tests/ISSUE_536/dcOctExpr.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should reproduce its independently simulated numeric counterpart through parameterized .DC fields, got {result:?}"
        );
        assert_eq!(
            result.contract, "dc_analysis_expression_family_wrapper",
            "{relative} should report the strict wrapper-family contract"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_ac_analysis_expression_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/ISSUE_543/acBaseline1.cir",
            "ac_analysis_expression_family_baseline",
        ),
        (
            "Netlists/Certification_Tests/ISSUE_543/acExpr1.cir",
            "ac_analysis_expression_family_wrapper",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should pass its independently simulated AC-analysis expression oracle, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_delimited_expression_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_1358/bug_1358_braces.cir",
            "delimited_expression_family_baseline",
        ),
        (
            "Netlists/Certification_Tests/BUG_1358/bug_1358_quotes.cir",
            "delimited_expression_family_wrapper",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should reproduce its quote/braces counterpart through independently simulated exact default PRN output, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_age_cap_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        ("Netlists/AGECAP/agecap_master.cir", "age_cap_family_anchor"),
        ("Netlists/AGECAP/agecap.cir", "age_cap_family_aged_baseline"),
        (
            "Netlists/AGECAP/agecap_ref.cir",
            "age_cap_family_equivalent_member",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should reproduce its AGE/D-equivalent representation through independently simulated exact default PRN output, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_params1_parameter_equivalence_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/PARAMS1/params_a.cir",
            "params1_parameter_equivalence_wrapper_owner",
            true,
        ),
        (
            "Netlists/PARAMS1/params_a0.cir",
            "params1_parameter_equivalence_literal_baseline",
            false,
        ),
        (
            "Netlists/PARAMS1/params_a1.cir",
            "params1_parameter_equivalence_parameterized_member",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy strict literal/parameterized transient equivalence, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(
            result.mismatches.is_empty(),
            "{relative} should satisfy the Release 7.10 interpolated RMS comparison"
        );
    }
}

#[test]
fn test_xyce_naked_algebra_parameter_equivalence_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/PARSER/nakedAlgebra.cir",
            "naked_algebra_parameter_equivalence_wrapper_owner",
            true,
        ),
        (
            "Netlists/PARSER/nakedAlgebraBaseline.cir",
            "naked_algebra_parameter_equivalence_braced_baseline",
            false,
        ),
        (
            "Netlists/PARSER/nakedAlgebraGlobal.cir",
            "naked_algebra_parameter_equivalence_global_member",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy strict braced/mixed local/global parameter equivalence, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(
            result.mismatches.is_empty(),
            "{relative} should satisfy the Release 7.10 interpolated RMS comparison"
        );
    }
}

#[test]
fn test_xyce_bug1826_thermal_parameter_scope_equivalence_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, wrapper_origin) in [
        (
            "Netlists/Certification_Tests/BUG_1826/linear_simple.cir",
            "bug1826_thermal_parameter_scope_wrapper_owner",
            true,
        ),
        (
            "Netlists/Certification_Tests/BUG_1826/linear_simple_global.cir",
            "bug1826_thermal_parameter_scope_global_baseline",
            false,
        ),
        (
            "Netlists/Certification_Tests/BUG_1826/linear_simple_param.cir",
            "bug1826_thermal_parameter_scope_local_member",
            false,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper_origin,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy strict global/local thermal-parameter equivalence, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(
            result.mismatches.is_empty(),
            "{relative} should satisfy the directional Release 7.10 interpolated RMS comparison"
        );
    }
}

#[test]
fn test_xyce_bsrc_vccs_source_multiplicity_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let families: &[(&str, &[&str])] = &[
        ("BSRC", &["bsrc1_m", "bsrc2_m", "bsrc3_m", "bsrc4_m"]),
        (
            "VCCS",
            &[
                "vccs_m",
                "vccs_nl1_m",
                "vccs_nl2_m",
                "vccs_nl3_m",
                "vccs_nl4_m",
                "vccs_tran_m",
            ],
        ),
    ];
    let mut executed = 0usize;
    for (directory, stems) in families {
        for stem in *stems {
            for baseline in [false, true] {
                let file = if baseline {
                    format!("{stem}_baseline.cir")
                } else {
                    format!("{stem}.cir")
                };
                let relative = format!("Netlists/{directory}/{file}");
                assert_eq!(
                    runner.requires_upstream_wrapper(&relative),
                    !baseline,
                    "{relative} wrapper ownership changed"
                );
                let result = runner.run_test(root.join(&relative));
                assert!(
                    result.passed && !result.expected_unsupported && !result.upstream_excluded,
                    "{relative} should satisfy the typed source-M relational oracle, got {result:?}"
                );
                assert_eq!(
                    result.contract,
                    if baseline {
                        "source_multiplicity_family_baseline"
                    } else {
                        "source_multiplicity_family_wrapper"
                    }
                );
                assert!(
                    result.mismatches.is_empty(),
                    "{relative} should satisfy owner-GOODFILE/baseline-TESTFILE xyce_verify ordering"
                );
                executed += 1;
            }
        }
    }
    assert_eq!(executed, 20, "source-M family census changed");
}

#[test]
fn test_xyce_abm_splines_inline_lookup_order_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, owner) in [
        ("Netlists/ABM_SPLINES/akimaOutOfOrder.cir", true),
        ("Netlists/ABM_SPLINES/akimaOutOfOrder_baseline.cir", false),
        ("Netlists/ABM_SPLINES/tableOutOfOrder2.cir", true),
        ("Netlists/ABM_SPLINES/tableOutOfOrder2_baseline.cir", false),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            owner,
            "{relative} wrapper ownership changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy exact sorted/reverse-order lookup equivalence, got {result:?}"
        );
        assert_eq!(
            result.contract,
            if owner {
                "abm_splines_inline_lookup_order_wrapper_owner"
            } else {
                "abm_splines_inline_lookup_order_sorted_control"
            }
        );
        assert!(
            result.mismatches.is_empty(),
            "{relative} should reproduce byte-identical default PRN output"
        );
    }
}

#[test]
fn test_xyce_bug38_subckt_formal_parentheses_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_38_SON/bug_38_son.cir",
            "bug38_subckt_formal_parentheses_wrapper_owner",
        ),
        (
            "Netlists/Certification_Tests/BUG_38_SON/bug_38_son_p.cir",
            "bug38_subckt_formal_parentheses_control",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should execute independently through the exact BUG_38_SON diff-i relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(
            result.mismatches.is_empty(),
            "{relative} should serialize the same default PRN table modulo header case"
        );
    }
}

#[test]
fn test_xyce_bug39_generated_gaussian_mean_sigma_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_39_SON/agauss.cir",
            "bug39_agauss_generated_mean_sigma_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_39_SON/gauss.cir",
            "bug39_gauss_generated_mean_sigma_wrapper",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} must remain owned by its removed historical wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should independently execute the exact generated Gaussian population and satisfy the historical strict mean/sigma predicate, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            Some("Netlists/Certification_Tests/BUG_39_SON/exclude")
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug39_deterministic_expression_gold_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_39_SON/bug39_int.cir",
            "bug39_int_single_point_xyce_verify_dc_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_39_SON/bug39_limit.cir",
            "bug39_limit_nominal_single_point_xyce_verify_dc_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_39_SON/bug39_pow.cir",
            "bug39_pow_single_point_xyce_verify_dc_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_39_SON/bug39_sign.cir",
            "bug39_sign_single_point_xyce_verify_dc_wrapper",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} must remain owned by its removed historical wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should execute its exact deterministic one-point DC/gold contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(result.upstream_exclusion_source, None);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug864_unresolved_subcircuit_parameter_error_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_864_SON/bug_864_son.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG864 must remain owned by its removed historical wrapper"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG864 should reproduce its typed bounded unresolved-parameter failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "expected_failure_bug864_unresolved_subcircuit_parameter_build"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug1025_bounded_no_analysis_error_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1025/null.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG1025 must remain owned by its removed historical wrapper"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG1025 should reproduce its bounded typed no-analysis failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "expected_failure_bug1025_no_analysis_selection"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug636_incomplete_tran_error_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_636_SON/bug636.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG636 must retain its active wrapper ownership"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG636 should reproduce its ordered incomplete-.TRAN failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "expected_failure_bug636_incomplete_tran_parse"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug206_undefined_subcircuit_error_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_206/bug_206.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG206 must retain its active wrapper ownership"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG206 should reproduce its typed undefined-subcircuit failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "expected_failure_bug206_undefined_subcircuit_build"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug1116_missing_diode_model_error_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1116/bug_1116.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG1116 must retain its active wrapper ownership"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG1116 should reproduce its typed missing-diode-model failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "expected_failure_bug1116_missing_diode_model_parse"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_issue61_behavioral_lead_current_error_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/ISSUE_61/issue61.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "ISSUE61 must retain its active wrapper ownership"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "ISSUE61 should reproduce its typed behavioral-reference failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "expected_failure_issue61_behavioral_lead_current_reference_build"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug784_archived_duplicate_subcircuit_port_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_784/bug_784.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG784 must retain its archived wrapper ownership"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG784 should reproduce its typed duplicate-port failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "archived_expected_failure_bug784_duplicate_subcircuit_port"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug42_archived_deprecated_lib_alias_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_42_SON/bug42.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG42 must retain its archived wrapper ownership"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG42 should reproduce its typed deprecated-.LIB failure, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "archived_expected_failure_bug42_deprecated_lib_alias"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_diode_analytic_generated_gold_wrappers() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, expected_contract) in [
        (
            "Netlists/DIODE_ANALYTIC/forward_model.cir",
            "analytic_diode_forward_tran_wrapper",
        ),
        (
            "Netlists/DIODE_ANALYTIC/reverse_model.cir",
            "analytic_diode_reverse_tran_wrapper",
        ),
        (
            "Netlists/DIODE_ANALYTIC/breakdown_model.cir",
            "analytic_diode_breakdown_tran_wrapper",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} must remain owned by its removed historical wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce its Release-7.10 generated analytic diode oracle, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(result.upstream_exclusion_source, None);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_legacy_device_analytic_generated_gold_wrappers() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, expected_contract) in [
        (
            "Netlists/BJT_ANALYTIC/ramp_test1.cir",
            "analytic_bjt_level1_depletion_charge_tran_wrapper",
        ),
        (
            "Netlists/BJT_ANALYTIC/ramp_test2.cir",
            "analytic_bjt_level1_transit_charge_tran_wrapper",
        ),
        (
            "Netlists/NMOS_ANALYTIC/mosfet_level1.cir",
            "analytic_nmos_level1_regions_tran_wrapper",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} must remain owned by its removed historical wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce its Release-7.10 generated legacy-device oracle, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(result.upstream_exclusion_source, None);
        assert!(result.mismatches.is_empty());
        assert!(result.error.is_none());
    }
}

#[test]
fn test_xyce_bug28_subcircuit_parameter_equivalence_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, expected_contract, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3.cir",
            "bug28son_subcircuit_parameter_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3_noparams.cir",
            "bug28son_subcircuit_parameter_literal_control",
            Some("Netlists/Certification_Tests/BUG_28_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3_globalp.cir",
            "bug28son_subcircuit_parameter_global_control",
            Some("Netlists/Certification_Tests/BUG_28_SON/exclude"),
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should execute the exact three-way BUG28 relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1398_inductor_model_equivalence_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, expected_contract, expected_wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1398/RLC.cir",
            "bug1398_inductor_model_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1398/RLC_simple.cir",
            "bug1398_inductor_model_literal_control",
            false,
            Some("Netlists/Certification_Tests/BUG_1398/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), expected_wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should execute the exact BUG1398 inductor-model relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1040_noop_operating_point_transient_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1040_SON/bug_1040_son.cir";
    assert!(runner.requires_upstream_wrapper(relative));
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG1040 should reproduce its exact NOOP/operating-point relation, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "bug1040_noop_operating_point_transient_wrapper"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug1162_inconsistent_dc_sweep_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, expected_contract, expected_wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1162_SON/bug_1162_son.cir",
            "bug1162_inconsistent_dc_sweep_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1162_SON/baseline.cir",
            "bug1162_inconsistent_dc_sweep_baseline_control",
            false,
            Some("Netlists/Certification_Tests/BUG_1162_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1162_SON/defective_lin.cir",
            "bug1162_inconsistent_dc_sweep_linear_control",
            false,
            Some("Netlists/Certification_Tests/BUG_1162_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1162_SON/defective_dec.cir",
            "bug1162_inconsistent_dc_sweep_decade_control",
            false,
            Some("Netlists/Certification_Tests/BUG_1162_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1162_SON/defective_oct.cir",
            "bug1162_inconsistent_dc_sweep_octave_control",
            false,
            Some("Netlists/Certification_Tests/BUG_1162_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), expected_wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should execute the exact BUG1162 one-point relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug271_tab_comment_rlc_success_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_271_SON/bug_271.cir";
    assert!(runner.requires_upstream_wrapper(relative));
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG271 should execute its exact tab-comment RLC success contract, got {result:?}"
    );
    assert_eq!(result.contract, "bug271_tab_comment_rlc_success_wrapper");
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug1661_globalnode_behavioral_expression_equality_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_1661/globalnode_expr_toplev.cir";
    assert!(runner.requires_upstream_wrapper(relative));
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG1661 should execute its exact global-node expression relation, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "bug1661_globalnode_behavioral_expression_equality_wrapper"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug519_binary_ascii_raw_column_equality_wrapper() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_519_SON/bug_519_SON.cir";
    assert!(runner.requires_upstream_wrapper(relative));
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG519 should execute its binary/ASCII RAW relation, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "bug519_binary_ascii_raw_column_equality_wrapper"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug48_level54_native_bsim4_success_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_48_SON/test.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG48 must remain owned by its removed historical wrapper"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG48 should execute its typed native BSIM4 success contract, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "bug48_level54_native_bsim4_success_wrapper"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug113_switch_initial_state_jacobian_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let owner = "Netlists/Certification_Tests/BUG_113/VSWITCH.cir";

    assert!(
        runner.requires_upstream_wrapper(owner),
        "BUG113 empty owner must remain owned by its removed historical wrapper"
    );
    assert_eq!(
        std::fs::metadata(root.join(owner))
            .expect("inspect BUG113 owner")
            .len(),
        0,
        "BUG113 owner must remain an exact zero-byte wrapper placeholder"
    );

    for (relative, expected_contract, expected_exclusion) in [
        (
            owner,
            "bug113_switch_initial_state_jacobian_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_113/VSWITCH_NOIC.cir",
            "bug113_switch_initial_state_noic_baseline",
            Some("Netlists/Certification_Tests/BUG_113/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_113/VSWITCH_ON_OFF.cir",
            "bug113_switch_initial_state_explicit_on_member",
            Some("Netlists/Certification_Tests/BUG_113/exclude"),
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} must reproduce the exact BUG113 3/2-Jacobian relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug159_bjt_tnom_default_equivalence_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let owner = "Netlists/Certification_Tests/BUG_159/bug_159.cir";

    assert!(
        runner.requires_upstream_wrapper(owner),
        "BUG159 empty owner must remain owned by its removed historical wrapper"
    );
    let owner_result = runner.run_test(root.join(owner));
    assert!(
        owner_result.passed
            && !owner_result.expected_unsupported
            && !owner_result.upstream_excluded,
        "BUG159 should execute both BJT TNOM workers and reproduce their exact PRN relation, got {owner_result:?}"
    );
    assert_eq!(
        owner_result.contract,
        "bug159_bjt_tnom_default_equivalence_wrapper"
    );
    assert_eq!(owner_result.upstream_exclusion_source, None);
    assert!(owner_result.mismatches.is_empty());

    for worker in [
        "Netlists/Certification_Tests/BUG_159/bug_159_1.cir",
        "Netlists/Certification_Tests/BUG_159/bug_159_2.cir",
    ] {
        assert!(
            !runner.requires_upstream_wrapper(worker),
            "BUG159 workers are invoked by the owner and must not become wrapper owners"
        );
        let result = runner.run_test(root.join(worker));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{worker} must retain its independently-qualified static PRN contract, got {result:?}"
        );
        assert_eq!(result.contract, "static_prn_dc");
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            Some("Netlists/Certification_Tests/BUG_159/exclude")
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug267_global_parameter_include_success_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_267/bug267.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG267 must remain owned by its removed historical wrapper"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG267 should execute its typed include/global-parameter DC contract, got {result:?}"
    );
    assert_eq!(
        result.contract,
        "bug267_global_parameter_include_success_wrapper"
    );
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug302_print_delimiter_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let relative = "Netlists/Certification_Tests/BUG_302/bug_302.cir";

    assert!(
        runner.requires_upstream_wrapper(relative),
        "BUG302 must remain owned by its removed historical wrapper"
    );
    let result = runner.run_test(root.join(relative));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG302 should execute its typed eight-member delimiter relation, got {result:?}"
    );
    assert_eq!(result.contract, "bug302_print_delimiter_relational_wrapper");
    assert_eq!(result.upstream_exclusion_source, None);
    assert!(result.mismatches.is_empty());
}

#[test]
fn test_xyce_bug1797_bsim3_level_alias_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1797/one-shot.cir",
            "bug1797_bsim3_level_alias_relational_wrapper",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1797/one-shot_lev9.cir",
            "bug1797_bsim3_level9_relational_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_1797/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1797/one-shot_lev49.cir",
            "bug1797_bsim3_level49_relational_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_1797/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the independently integrated BSIM3 selector relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug981_output_schedule_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_981_SON/bug981.cir",
            "bug981_outputtimepoints_breakpoints_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_981_SON/bug981A.cir",
            "bug981_outputtimepoints_breakpoints_relational_output_control",
            false,
            Some("Netlists/Certification_Tests/BUG_981_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_981_SON/bug981B.cir",
            "bug981_outputtimepoints_breakpoints_relational_breakpoint_control",
            false,
            Some("Netlists/Certification_Tests/BUG_981_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the exact adaptive output/breakpoint relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug706_rc_include_inline_release_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_706_SON/rc_simple_lib.cir",
            "bug706_rc_include_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_706_SON/rc_simple_xyce.cir",
            "bug706_rc_inline_release_reference",
            false,
            Some("Netlists/Certification_Tests/BUG_706_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the strict inline-good to include-test Release relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug706_each_role_fits_aggregate_process_watchdog() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let config = XyceRunnerConfig::default();
    let runner = XyceTestRunner::new(&root, config.clone());
    let relatives = [
        "Netlists/Certification_Tests/BUG_706_SON/rc_simple_lib.cir",
        "Netlists/Certification_Tests/BUG_706_SON/rc_simple_xyce.cir",
    ];
    let discovered = runner.discover_tests();

    for (index, relative) in relatives.into_iter().enumerate() {
        let deck = discovered
            .iter()
            .find(|deck| deck.relative_path == relative)
            .unwrap_or_else(|| panic!("discover BUG706 role {relative}"));
        let started = Instant::now();
        let result = run_xyce_case_with_watchdog(&root, &config, deck, index + 1, 2);
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} must complete its full paired relation under the aggregate watchdog: {result:?}"
        );
        assert!(
            started.elapsed() < Duration::from_millis(XYCE_AGGREGATE_HARD_CASE_TIMEOUT_MS as u64),
            "{relative} exceeded aggregate hard watchdog: {:?}",
            started.elapsed()
        );
    }
}

#[test]
fn test_xyce_bug986_erroption_breakpoint_source_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_986_SON/bug986.cir",
            "bug986_erroption_breakpoint_source_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_986_SON/bug986A.cir",
            "bug986_erroption_explicit_breakpoints_relational_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_986_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_986_SON/bug986B.cir",
            "bug986_erroption_disconnected_pwl_breakpoints_relational_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_986_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the strict ERROPTION breakpoint/PWL default-PRN relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug907_nested_library_exact_release_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_907_SON/test2.cir",
            "bug907_nested_library_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_907_SON/test2_nolib.cir",
            "bug907_flat_release_reference",
            false,
            Some("Netlists/Certification_Tests/BUG_907_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the strict flat-first nested-LIB PRN relation: {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
    let rlc = runner.run_test(root.join("Netlists/Certification_Tests/BUG_907_SON/rlc.cir"));
    assert!(
        rlc.passed && rlc.expected_unsupported && !rlc.upstream_excluded,
        "BUG907 recursive-include diagnostic owner must remain unqualified: {rlc:?}"
    );
    assert_eq!(rlc.contract, "unsupported_xyce_contract");
}

#[test]
fn test_xyce_bug1035_simple_rc_ac_data_release_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_data.cir",
            "bug1035_simple_rc_ac_data_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_baseline.cir",
            "bug1035_simple_rc_ac_data_baseline_reference",
            false,
            Some("Netlists/Certification_Tests/BUG_1035_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce baseline-first exact/fallback AC relation: {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
    let sibling =
        runner.run_test(root.join("Netlists/Certification_Tests/BUG_1035_SON/RC_simple.cir"));
    assert!(
        sibling.passed
            && sibling.contract == "wrapper_static_fd_prn_ac"
            && sibling.contract != "bug1035_simple_rc_ac_data_relational_wrapper_owner"
            && sibling.contract != "bug1035_simple_rc_ac_data_baseline_reference",
        "BUG1035 checked-gold sibling must retain its pre-existing independent contract: {sibling:?}"
    );
}

#[test]
fn test_xyce_issue202_redefined_parameter_mode_matrix() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/ISSUE_202/first_last.cir",
            "issue202_redefined_params_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/ISSUE_202/first.cir",
            "issue202_redefined_params_first_control",
            false,
            Some("Netlists/Certification_Tests/ISSUE_202/exclude"),
        ),
        (
            "Netlists/Certification_Tests/ISSUE_202/last.cir",
            "issue202_redefined_params_last_control",
            false,
            Some("Netlists/Certification_Tests/ISSUE_202/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the complete repeated-parameter mode matrix, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_issue451_hierarchical_node_operating_point_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/ISSUE_451/issue451.cir",
            "issue451_hierarchical_node_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/ISSUE_451/issue451_ref.cir",
            "issue451_hierarchical_node_explicit_reference",
            false,
            Some("Netlists/Certification_Tests/ISSUE_451/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the exact hierarchical-node operating-point relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_issue565566_step_data_pwl_table_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/ISSUE_565_566/issue565_566.cir",
            "issue565566_step_data_pwl_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/ISSUE_565_566/issue565_566ref.cir",
            "issue565566_step_data_table_reference",
            Some("Netlists/Certification_Tests/ISSUE_565_566/exclude"),
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            expected_exclusion.is_none(),
            "only the PWL deck owns the historical ISSUE565566 wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact three-row STEP DATA PWL/table relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1455_model_parameter_punctuation_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1455/bug_1455.cir",
            "bug1455_model_equals_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1455/bug_1455_neq.cir",
            "bug1455_model_whitespace_reference",
            false,
            Some("Netlists/Certification_Tests/BUG_1455/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the model-parameter punctuation relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_mosfet_parameter_alias_relations() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/MOSFET_ParamAliases/invert1.cir",
            "mosfet_param_alias_level1_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert1_alt.cir",
            "mosfet_param_alias_level1_alt_worker",
            false,
            Some("Netlists/MOSFET_ParamAliases/exclude"),
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert2.cir",
            "mosfet_param_alias_level2_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert2_alt.cir",
            "mosfet_param_alias_level2_alt_worker",
            false,
            Some("Netlists/MOSFET_ParamAliases/exclude"),
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert3.cir",
            "mosfet_param_alias_level3_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert3_alt.cir",
            "mosfet_param_alias_level3_alt_worker",
            false,
            Some("Netlists/MOSFET_ParamAliases/exclude"),
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert6.cir",
            "mosfet_param_alias_level6_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/MOSFET_ParamAliases/invert6_alt.cir",
            "mosfet_param_alias_level6_alt_worker",
            false,
            Some("Netlists/MOSFET_ParamAliases/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce its exact classic-MOS alias relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_tr_tran_short_analysis_alias_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/TR_TRAN/tr2.cir",
            "tr_tran_short_alias_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/TR_TRAN/tran2.cir",
            "tr_tran_long_form_reference",
            false,
            Some("Netlists/TR_TRAN/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the TR/TRAN transient alias relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug689_dos_unix_line_ending_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_689/dos-diode.cir",
            "bug689_crlf_line_endings_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_689/unix-diode.cir",
            "bug689_lf_line_endings_reference",
            false,
            Some("Netlists/Certification_Tests/BUG_689/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the DOS/Unix line-ending relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug806_dc_data_baseline_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_806_SON/dc_data.cir",
            "bug806_dc_data_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_806_SON/dc_baseline.cir",
            "bug806_dc_data_relational_wrapper_baseline",
            false,
            Some("Netlists/Certification_Tests/BUG_806_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the DC DATA/baseline relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1043_ac_data_parameter_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params.cir",
            "bug1043_ac_data_parameter_relational_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_expr.cir",
            "bug1043_ac_data_parameter_relational_expression_baseline",
            false,
            Some("Netlists/Certification_Tests/BUG_1043_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir",
            "wrapper_static_fd_prn_ac",
            true,
            None,
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            wrapper,
            "{relative} wrapper provenance changed"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy its BUG_1043 AC DATA parameter disposition, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1957_multiwinding_mutual_inductor_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1957/mutindlin_even.cir",
            "bug1957_even_mutual_inductor_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1957/mutindlin_even_ps.cir",
            "bug1957_even_multiwinding_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_1957/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1957/mutindlin_even_s3f5.cir",
            "bug1957_even_pairwise_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_1957/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1957/mutindlin_odd.cir",
            "bug1957_odd_mutual_inductor_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1957/mutindlin_odd_ps.cir",
            "bug1957_odd_multiwinding_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_1957/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1957/mutindlin_odd_s3f5.cir",
            "bug1957_odd_pairwise_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_1957/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the absolute and relational mutual-inductor oracles, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug805_legacy_bjt_model_alias_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_805/bug_805.cir",
            "bug805_bjt_model_alias_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_805/colpitts_osc1.cir",
            "bug805_bjt_model_canonical_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_805/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_805/colpitts_osc2.cir",
            "bug805_bjt_model_pspice_alias_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_805/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_805/colpitts_osc3.cir",
            "bug805_bjt_model_hspice_alias_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_805/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the native BJT alias relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug805_son_include_partition_mutual_inductor_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, contract, wrapper, exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_805_SON/bug805_all.cir",
            "bug805son_inline_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_805_SON/bug805_top.cir",
            "bug805son_single_include_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_805_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_805_SON/bug805_top2.cir",
            "bug805son_nested_include_worker",
            false,
            Some("Netlists/Certification_Tests/BUG_805_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the exact inline/include/nested-include PRN relation, got {result:?}"
        );
        assert_eq!(result.contract, contract);
        assert_eq!(result.upstream_exclusion_source.as_deref(), exclusion);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug352_diode_model_expression_equivalence_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_wrapper, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_352/BUG_352a.cir",
            "bug352_diode_model_expression_equivalence_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_352/BUG_352b.cir",
            "bug352_diode_model_expression_equivalence_literal_control",
            false,
            Some("Netlists/Certification_Tests/BUG_352/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), expected_wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the exact model-expression/literal PRN relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug306_numeric_string_timeint_method_release_relations() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_wrapper, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear.cir",
            "bug306_gear_numeric_method_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear_strings.cir",
            "bug306_gear_string_method_release_control",
            false,
            Some("Netlists/Certification_Tests/BUG_306_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap.cir",
            "bug306_trap_numeric_method_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap_strings.cir",
            "bug306_trap_string_method_release_control",
            false,
            Some("Netlists/Certification_Tests/BUG_306_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap_strings2.cir",
            "bug306_trapezoidal_string_method_release_control",
            false,
            Some("Netlists/Certification_Tests/BUG_306_SON/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), expected_wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should independently reproduce the Release-7.10 numeric/string TIMEINT METHOD relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug308_stepped_temp_output_framing_relation() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let owner = "Netlists/Certification_Tests/BUG_308_SON/comparator3.cir";
    assert!(runner.requires_upstream_wrapper(owner));
    let result = runner.run_test(root.join(owner));
    assert!(
        result.passed && !result.expected_unsupported && !result.upstream_excluded,
        "BUG308 owner should reproduce the Release stepped-TEMP/raw-framing relation: {result:?}"
    );
    assert_eq!(
        result.contract,
        "bug308_stepped_temp_output_framing_wrapper_owner"
    );
    assert!(result.error.is_none());
    assert!(result.mismatches.is_empty());

    for helper in ["comparator0.cir", "comparator.cir"] {
        let relative = format!("Netlists/Certification_Tests/BUG_308_SON/{helper}");
        assert!(!runner.requires_upstream_wrapper(&relative));
        let excluded = runner.run_test(root.join(&relative));
        assert!(excluded.passed && excluded.upstream_excluded);
        assert_eq!(excluded.contract, "upstream_excluded");
        assert_eq!(
            excluded.upstream_exclusion_source.as_deref(),
            Some("Netlists/Certification_Tests/BUG_308_SON/exclude")
        );
        assert!(
            excluded
                .error
                .as_deref()
                .is_some_and(|error| error.starts_with("UPSTREAM_EXCLUDED:"))
        );
    }
}

#[test]
fn test_xyce_bug307_subcircuit_model_scope_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_wrapper, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_307/bug_307_a.cir",
            "bug307_subcircuit_model_scope_wrapper_owner",
            true,
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_307/bug_307_b.cir",
            "bug307_subcircuit_model_scope_control",
            false,
            Some("Netlists/Certification_Tests/BUG_307/exclude"),
        ),
    ] {
        assert_eq!(runner.requires_upstream_wrapper(relative), expected_wrapper);
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should reproduce the exact scoped-model raw-PRN relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug402_temperature_option_scope_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_402_SON/bug402son.cir",
            "bug402_temperature_option_scope_relational_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_402_SON/bug402XyceTempOptions.cir",
            "bug402_temperature_option_scope_relational_xyce_reference",
            Some("Netlists/Certification_Tests/BUG_402_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_402_SON/bug402SpiceTempOptions.cir",
            "bug402_temperature_option_scope_relational_spice_member",
            Some("Netlists/Certification_Tests/BUG_402_SON/exclude"),
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should independently execute the exact BUG402 canonical-versus-legacy temperature-option relation, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion,
            "only the two historically excluded workers retain exclusion provenance"
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_switch_initial_state_case_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_1376/bug_1376.cir",
            "switch_state_case_family_anchor",
        ),
        (
            "Netlists/Certification_Tests/BUG_1376/upper_case.cir",
            "switch_state_case_family_uppercase_baseline",
        ),
        (
            "Netlists/Certification_Tests/BUG_1376/lower_case.cir",
            "switch_state_case_family_lowercase_member",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should reproduce its initial-state case counterpart through independently simulated exact default PRN output, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_subcircuit_parameter_resolution_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let cases = [
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558.cir",
            "subckt_parameter_resolution_family_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558_0.cir",
            "subckt_parameter_resolution_family_baseline",
        ),
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558_1.cir",
            "subckt_parameter_resolution_family_member",
        ),
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558_2.cir",
            "subckt_parameter_resolution_family_member",
        ),
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558_3.cir",
            "subckt_parameter_resolution_family_member",
        ),
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558_4.cir",
            "subckt_parameter_resolution_family_member",
        ),
        (
            "Netlists/Certification_Tests/BUG_1558/bug_1558_error.cir",
            "subckt_parameter_resolution_expected_error",
        ),
    ];

    for (relative, expected_contract) in cases {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy the complete subcircuit parameter-resolution family, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_nested_include_identity_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_1686/bug_1686.cir",
            "nested_include_identity_family_anchor",
        ),
        (
            "Netlists/Certification_Tests/BUG_1686/bug_1686a.cir",
            "nested_include_identity_family_repeated_target_baseline",
        ),
        (
            "Netlists/Certification_Tests/BUG_1686/bug_1686_dup.cir",
            "nested_include_identity_family_split_targets_member",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should preserve scope-qualified nested-include identity through independently simulated exact default PRN output, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_startup_diagnostic_wrappers_run_with_exact_typed_contracts() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Message/Input/IC_At_Missing_Node_Warning.cir",
            "expected_warning_ic_undefined_node_success",
        ),
        (
            "Netlists/Message/Input/IC_No_Args_Warning.cir",
            "expected_warning_ic_empty_success",
        ),
        (
            "Netlists/Message/Input/NODESET_At_Missing_Node_Warning.cir",
            "expected_warning_nodeset_undefined_node_success",
        ),
        (
            "Netlists/Message/Input/NODESET_No_Args_Warning.cir",
            "expected_warning_nodeset_empty_success",
        ),
        (
            "Netlists/Certification_Tests/BUG_667_SON/ic_in_subckt_warning.cir",
            "expected_warning_bug667_scoped_global_ic_success",
        ),
        (
            "Netlists/Message/Input/IC_And_NODESET_Specified.cir",
            "expected_failure_message_ic_nodeset_conflict_parse",
        ),
    ] {
        assert!(
            runner.requires_upstream_wrapper(relative),
            "{relative} must retain its removed upstream wrapper provenance"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported,
            "{relative} should satisfy its exact native startup diagnostic contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug667_direct_and_scoped_ic_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_667_SON/ic_in_subckt.cir",
            "bug667_ic_relational_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_667_SON/ic_not_in_subckt.cir",
            "bug667_ic_relational_wrapper_explicit_reference",
            Some("Netlists/Certification_Tests/BUG_667_SON/exclude"),
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            expected_exclusion.is_none(),
            "only the scoped-IC deck is the historical wrapper owner"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should execute the direct/scoped IC relation with a nonvacuous decay oracle, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug991_upstream_exclusion_is_typed_without_hiding_sibling_coverage() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let config = XyceRunnerConfig::default();
    let runner = XyceTestRunner::new(&root, config.clone());
    const EXCLUDED: &str = "Netlists/Certification_Tests/BUG_991_SON/BH_nogap_lv2.cir";
    let deck = runner
        .discover_tests()
        .into_iter()
        .find(|deck| deck.relative_path == EXCLUDED)
        .expect("discover BUG 991 excluded deck");
    let excluded = run_xyce_case_with_watchdog(&root, &config, &deck, 1, 1);
    assert!(
        excluded.passed,
        "upstream exclusion is accepted: {excluded:?}"
    );
    assert!(excluded.upstream_excluded);
    assert!(!excluded.expected_unsupported);
    assert_eq!(excluded.contract, "upstream_excluded");
    assert_eq!(
        excluded.upstream_exclusion_source.as_deref(),
        Some("Netlists/Certification_Tests/BUG_991_SON/exclude")
    );
    assert!(excluded.mismatches.is_empty());

    let mut family_results = vec![excluded];
    for sibling in ["BH_nogap.cir", "BH_test.cir", "BH_test_lv2.cir"] {
        let relative = format!("Netlists/Certification_Tests/BUG_991_SON/{sibling}");
        let result = runner.run_test(root.join(&relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "BUG 991 sibling {sibling} must retain native coverage: {result:?}"
        );
        assert_eq!(result.contract, "static_xyce_verify_prn_tran");
        assert!(result.upstream_exclusion_source.is_none());
        family_results.push(result);
    }

    let stats = XyceTestRunner::statistics(&family_results);
    assert_eq!(stats.total, 4);
    assert_eq!(stats.executed, 3);
    assert_eq!(stats.passed, 3);
    assert_eq!(stats.failed, 0);
    assert_eq!(stats.expected_unsupported, 0);
    assert_eq!(stats.upstream_excluded, 1);
}

#[test]
fn test_xyce_bug1190_mutual_inductor_parameter_alias_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_1190_SON/mutInd1.cir",
            "bug1190_mutual_inductor_parameter_alias_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/mutInd1_baseline.cir",
            "bug1190_mutual_inductor_parameter_alias_baseline",
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/mutInd2.cir",
            "bug1190_mutual_inductor_parameter_alias_wrapper",
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/mutInd2_baseline.cir",
            "bug1190_mutual_inductor_parameter_alias_baseline",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the complete BUG 1190 mutual-inductor parameter-alias contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1190_process_parameter_alias_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1190_SON/bsim3.cir",
            "bug1190_son_process_parameter_alias_relational_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/bsim3_modpar.cir",
            "bug1190_son_direct_model_parameter_relational_control",
            Some("Netlists/Certification_Tests/BUG_1190_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/bsim4.cir",
            "bug1190_son_process_parameter_alias_relational_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/bsim4_modpar.cir",
            "bug1190_son_direct_model_parameter_relational_control",
            Some("Netlists/Certification_Tests/BUG_1190_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/diode.cir",
            "bug1190_son_process_parameter_alias_relational_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1190_SON/diodeRef.cir",
            "bug1190_son_direct_model_parameter_relational_control",
            Some("Netlists/Certification_Tests/BUG_1190_SON/exclude"),
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact BUG 1190 process-parameter relational contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1284_transient_restart_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1284/bug_1284.cir",
            "bug1284_transient_restart_relational_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1284/bug_1284_baseline.cir",
            "bug1284_transient_restart_relational_worker",
            Some("Netlists/Certification_Tests/BUG_1284/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1284/bug_1284_first.cir",
            "bug1284_transient_restart_relational_worker",
            Some("Netlists/Certification_Tests/BUG_1284/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1284/bug_1284_restarted.cir",
            "bug1284_transient_restart_relational_worker",
            Some("Netlists/Certification_Tests/BUG_1284/exclude"),
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            expected_exclusion.is_none(),
            "only the zero-byte BUG1284 deck owns the removed wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact historical 20 ns and strengthened nonquiescent 5 ns restart relations, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug442_packed_unpacked_tline_restart_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_442/bug_442.cir",
            "bug442_packed_unpacked_tline_restart_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_442/bug_442_baseline.cir",
            "bug442_packed_unpacked_tline_restart_worker",
            Some("Netlists/Certification_Tests/BUG_442/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_442/bug_442_baseline_unpacked.cir",
            "bug442_packed_unpacked_tline_restart_worker",
            Some("Netlists/Certification_Tests/BUG_442/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_442/bug_442_restarted.cir",
            "bug442_packed_unpacked_tline_restart_worker",
            Some("Netlists/Certification_Tests/BUG_442/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_442/bug_442_restarted_unpacked.cir",
            "bug442_packed_unpacked_tline_restart_worker",
            Some("Netlists/Certification_Tests/BUG_442/exclude"),
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            expected_exclusion.is_none(),
            "only the zero-byte BUG442 owner owns the removed wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy both restart codecs and all three historical relations, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_classic_level1_mos_dtemp_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for polarity in ["nmos", "pmos"] {
        for level_tag in [1, 2, 3, 6] {
            for (suffix, expected_contract) in [
                ("dtemp", "classic_mos_level1_dtemp_relational_wrapper_owner"),
                (
                    "ref",
                    "classic_mos_level1_dtemp_relational_wrapper_reference",
                ),
            ] {
                let relative = format!("Netlists/DTEMP/{polarity}{level_tag}_{suffix}.cir");
                let result = runner.run_test(root.join(&relative));
                assert!(
                    result.passed && !result.expected_unsupported && !result.upstream_excluded,
                    "{relative} should satisfy the exact classic level-1 MOS TEMP/DTEMP relational contract, got {result:?}"
                );
                assert_eq!(result.contract, expected_contract);
                assert!(result.error.is_none());
                assert!(result.mismatches.is_empty());
            }
        }
    }
}

#[test]
fn test_xyce_legacy_bjt_dtemp_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/DTEMP/npn_dtemp.cir",
            "legacy_gummel_poon_bjt_dtemp_relational_wrapper_owner",
        ),
        (
            "Netlists/DTEMP/npn_ref.cir",
            "legacy_gummel_poon_bjt_dtemp_relational_wrapper_reference",
        ),
        (
            "Netlists/DTEMP/pnp_dtemp.cir",
            "legacy_gummel_poon_bjt_dtemp_relational_wrapper_owner",
        ),
        (
            "Netlists/DTEMP/pnp_ref.cir",
            "legacy_gummel_poon_bjt_dtemp_relational_wrapper_reference",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact native legacy BJT TEMP/DTEMP relational contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_sydney_level1_jfet_dtemp_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/DTEMP/njfet_dtemp.cir",
            "xyce_sydney_level1_jfet_dtemp_relational_wrapper_owner",
        ),
        (
            "Netlists/DTEMP/njfet_ref.cir",
            "xyce_sydney_level1_jfet_dtemp_relational_wrapper_reference",
        ),
        (
            "Netlists/DTEMP/pjfet_dtemp.cir",
            "xyce_sydney_level1_jfet_dtemp_relational_wrapper_owner",
        ),
        (
            "Netlists/DTEMP/pjfet_ref.cir",
            "xyce_sydney_level1_jfet_dtemp_relational_wrapper_reference",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact native Xyce Sydney level-1 JFET TEMP/DTEMP relational contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_level2_diode_dtemp_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/DTEMP/Level2_Diode_DTEMP.cir",
            "level2_diode_dtemp_relational_wrapper_owner",
        ),
        (
            "Netlists/DTEMP/Level2_Diode_Ref.cir",
            "level2_diode_dtemp_relational_wrapper_reference",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact native Level-2 diode TEMP/DTEMP relational contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_capacitor_dtemp_relational_oracles() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/DTEMP/cap_dtemp.cir",
            "capacitor_dtemp_relational_wrapper_owner",
        ),
        (
            "Netlists/DTEMP/cap_ref.cir",
            "capacitor_dtemp_relational_wrapper_reference",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact native capacitor TEMP/DTEMP relational contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1085_i0_user_function_relational_oracle() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract) in [
        (
            "Netlists/Certification_Tests/BUG_1085_SON/bug1085son.cir",
            "bug1085_user_function_i0_relational_wrapper_owner",
        ),
        (
            "Netlists/Certification_Tests/BUG_1085_SON/bug1085sonRef.cir",
            "bug1085_user_function_alpha_renamed_reference_baseline",
        ),
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact BUG_1085 I0 user-function relational contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug141_native_solver_backend_completion_family() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_141_SON/bug_141-empty.cir",
            "bug141_native_solver_backend_completion_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_141_SON/abs.cir",
            "bug141_native_solver_backend_completion_worker",
            Some("Netlists/Certification_Tests/BUG_141_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_141_SON/bug_138_1.cir",
            "bug141_native_solver_backend_completion_worker",
            Some("Netlists/Certification_Tests/BUG_141_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_141_SON/dig_count.cir",
            "bug141_native_solver_backend_completion_worker",
            Some("Netlists/Certification_Tests/BUG_141_SON/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_141_SON/poly.cir",
            "bug141_native_solver_backend_completion_worker",
            Some("Netlists/Certification_Tests/BUG_141_SON/exclude"),
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            expected_exclusion.is_none(),
            "only the comments-only BUG141 deck owns the historical wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should complete its explicitly selected KLU/Faer backend contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1692_expression_equivalence_family() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for (relative, expected_contract, expected_exclusion) in [
        (
            "Netlists/Certification_Tests/BUG_1692/bug_1692.cir",
            "bug1692_expression_equivalence_wrapper_owner",
            None,
        ),
        (
            "Netlists/Certification_Tests/BUG_1692/bug_1692_0.cir",
            "bug1692_expression_equivalence_worker",
            Some("Netlists/Certification_Tests/BUG_1692/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1692/bug_1692_1.cir",
            "bug1692_expression_equivalence_worker",
            Some("Netlists/Certification_Tests/BUG_1692/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1692/bug_1692_2.cir",
            "bug1692_expression_equivalence_worker",
            Some("Netlists/Certification_Tests/BUG_1692/exclude"),
        ),
        (
            "Netlists/Certification_Tests/BUG_1692/bug_1692_3.cir",
            "bug1692_expression_equivalence_worker",
            Some("Netlists/Certification_Tests/BUG_1692/exclude"),
        ),
    ] {
        assert_eq!(
            runner.requires_upstream_wrapper(relative),
            expected_exclusion.is_none(),
            "only the zero-byte BUG1692 deck owns the historical wrapper"
        );
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && !result.expected_unsupported && !result.upstream_excluded,
            "{relative} should satisfy the exact BUG1692 expression-equivalence contract, got {result:?}"
        );
        assert_eq!(result.contract, expected_contract);
        assert_eq!(
            result.upstream_exclusion_source.as_deref(),
            expected_exclusion
        );
        assert!(result.error.is_none());
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn test_xyce_bug1692_noworkee_remains_upstream_excluded() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
    let noworkee =
        runner.run_test(root.join("Netlists/Certification_Tests/BUG_1692/bug_1692_noworkee.cir"));
    assert!(
        noworkee.upstream_excluded && noworkee.passed,
        "the historical noworkee control must remain ordinarily upstream-excluded: {noworkee:?}"
    );
    assert_eq!(noworkee.contract, "upstream_excluded");
    assert!(
        noworkee
            .error
            .as_deref()
            .is_some_and(|error| error.starts_with("UPSTREAM_EXCLUDED:"))
    );
    assert_eq!(
        noworkee.upstream_exclusion_source.as_deref(),
        Some("Netlists/Certification_Tests/BUG_1692/exclude")
    );
}

// The aggregate replays every retained deck, so it runs only in release (the
// nightly and engine-release workflows). Each deck's numbers are held by
// `failed == 0`, its contract by the contract manifest.
#[cfg_attr(
    debug_assertions,
    ignore = "release-only full Xyce corpus; run with `cargo test --release -p rspice-conformance --test xyce_regression test_full_xyce_suite_summary_accounts_for_every_deck`"
)]
#[test]
fn test_full_xyce_suite_summary_accounts_for_every_deck() {
    let _xyce_runner_guard = lock_xyce_runner();
    let root = get_xyce_tests_dir();
    let config = XyceRunnerConfig::default();
    let runner = XyceTestRunner::new(&root, config.clone());

    let discovered = runner.discover_tests();
    let netlist_count = discovered
        .iter()
        .filter(|deck| deck.section == XyceDeckSection::Netlists)
        .count();
    let total = discovered.len();
    let results = discovered
        .iter()
        .enumerate()
        .map(|(index, deck)| run_xyce_case_with_watchdog(&root, &config, deck, index + 1, total))
        .collect::<Vec<_>>();
    XyceTestRunner::print_summary(&results);

    // Before any assertion: this census is what a re-baseline is written from,
    // and the most likely reason to want one is a run the manifest fails.
    if let Some(report_path) = write_xyce_grid_alignment_report_if_requested(&results)
        .expect("write requested Xyce grid-alignment report")
    {
        println!("Xyce grid-alignment report: {}", report_path.display());
    }
    if let Some(report_path) = write_xyce_classification_report_if_requested(&results)
        .expect("write requested Xyce classification report")
    {
        println!("Xyce classification report: {}", report_path.display());
    }

    let index_aligned = results
        .iter()
        .filter(|result| {
            result.transient_grid_alignment == Some(XyceTransientGridAlignment::IndexAligned)
        })
        .count();
    let interpolated = results
        .iter()
        .filter(|result| {
            result.transient_grid_alignment == Some(XyceTransientGridAlignment::Interpolated)
        })
        .count();
    println!(
        "Xyce transient grid alignment: {index_aligned} index-aligned, {interpolated} interpolated"
    );

    let stats = XyceTestRunner::statistics(&results);

    assert_eq!(
        stats.total,
        discovered.len(),
        "Full Xyce suite must produce exactly one result per discovered deck"
    );
    assert!(netlist_count > 0, "No Xyce Netlists decks discovered");
    assert!(
        stats.executed > 0,
        "At least one Xyce deck must run as a numeric comparison"
    );
    // Both manifests are checked before the pass/fail census, and deliberately.
    // A deck's contract is chosen before it runs, and a comparison's geometry is
    // decided by the two time axes alone, so both are just as valid on a deck
    // that then failed on values; checked second, a corpus that is red for any
    // unrelated reason would take the pins offline with it. Contracts go first
    // because a moved contract is the usual reason a grid pin breaks.
    let contract_failures =
        xyce_contract_manifest_failures(&results, &read_xyce_contract_manifest(&root));
    assert!(
        contract_failures.is_empty(),
        "Xyce contract manifest disagrees with the suite in {} place(s):\n{}\n\n{XYCE_CONTRACT_REBASELINE}",
        contract_failures.len(),
        contract_failures.join("\n")
    );
    let grid_alignment_failures =
        xyce_grid_alignment_failures(&results, &read_xyce_grid_alignment_manifest(&root));
    assert!(
        grid_alignment_failures.is_empty(),
        "Xyce grid-alignment manifest disagrees with the suite in {} place(s):\n\n{}",
        grid_alignment_failures.len(),
        grid_alignment_failures.join("\n\n")
    );
    assert_eq!(
        stats.failed, 0,
        "Xyce full suite has {} failing deck(s): {} executed pass, {} expected unsupported, {} upstream excluded",
        stats.failed, stats.passed, stats.expected_unsupported, stats.upstream_excluded
    );
}
