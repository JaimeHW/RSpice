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
