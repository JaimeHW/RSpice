//! Opt-in audit for checked-in ngspice `.out` oracles against a live local build.
//!
//! This test is ignored by default and never participates in ordinary regression
//! behavior. To run it, set `NGSPICE_SOURCE_ROOT` and `NGSPICE_EXE`, then invoke
//! the ignored `audit_checked_out_against_live_ngspice` test with `--nocapture`.
//! It defaults to `resistance/res_simple.cir`; use
//! `NGSPICE_ORACLE_AUDIT_CASES`, `NGSPICE_ORACLE_AUDIT_SUITES`, or
//! `NGSPICE_ORACLE_AUDIT_SUITES=all` to widen the audit.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const DEFAULT_CASE: &str = "resistance/res_simple.cir";
const DEFAULT_TIMEOUT_MS: u128 = 30_000;
const MAX_DIFFS: usize = 5;

// Mirrors the local ngspice `tests/bin/check.sh` filter. Keep this private to
// the opt-in audit so normal RSpice regression comparisons remain unchanged.
const CHECK_FILTER_PATTERNS: &[&str] = &[
    "SPARSE",
    "KLU",
    "CPU",
    "Dynamic",
    "Note",
    "Circuit",
    "Trying",
    "Reference",
    "Date",
    "Doing",
    "---",
    "v-sweep",
    "time",
    "est",
    "Error",
    "Warning",
    "Data",
    "Index",
    "trans",
    "acan",
    "oise",
    "nalysis",
    "ole",
    "Total",
    "memory",
    "urrent",
    "Got",
    "Added",
    "BSIM",
    "bsim",
    "B4SOI",
    "b4soi",
    "codemodel",
    "Operating",
];

struct AuditConfig {
    source_root: PathBuf,
    ngspice_exe: PathBuf,
    cases: Vec<String>,
    timeout_ms: u128,
    fail_on_drift: bool,
}

#[derive(Default)]
struct AuditReport {
    total: usize,
    passed: usize,
    drifted: usize,
    skipped: usize,
    errors: usize,
}

struct LiveOutput {
    stdout: String,
    stderr: String,
    duration_ms: u128,
    status: String,
}

#[derive(Debug)]
struct Comparison {
    passed: bool,
    expected_row_counts: Vec<usize>,
    live_row_counts: Vec<usize>,
    expected_filtered_lines: usize,
    live_filtered_lines: usize,
    diffs: Vec<DiffLine>,
}

#[derive(Debug)]
struct DiffLine {
    line_index: usize,
    expected: Option<String>,
    live: Option<String>,
}

#[test]
#[ignore = "requires NGSPICE_SOURCE_ROOT and NGSPICE_EXE; report-only unless NGSPICE_ORACLE_AUDIT_FAIL_ON_DRIFT=1"]
fn audit_checked_out_against_live_ngspice() {
    let Some(config) = audit_config_from_env() else {
        return;
    };

    let report = run_audit(&config);
    assert_eq!(
        report.errors, 0,
        "ngspice oracle audit had {} execution/configuration error(s)",
        report.errors
    );
    if config.fail_on_drift {
        assert_eq!(
            report.drifted, 0,
            "ngspice oracle audit found {} drifted checked-in .out file(s)",
            report.drifted
        );
    }
}

fn audit_config_from_env() -> Option<AuditConfig> {
    let source_root = path_env("NGSPICE_SOURCE_ROOT");
    let ngspice_exe = path_env("NGSPICE_EXE");
    let (source_root, ngspice_exe) = match (source_root, ngspice_exe) {
        (None, None) => {
            println!(
                "Skipping ngspice oracle audit; set NGSPICE_SOURCE_ROOT and NGSPICE_EXE to opt in."
            );
            return None;
        }
        (Some(_), None) => panic!("NGSPICE_SOURCE_ROOT is set but NGSPICE_EXE is missing"),
        (None, Some(_)) => panic!("NGSPICE_EXE is set but NGSPICE_SOURCE_ROOT is missing"),
        (Some(source_root), Some(ngspice_exe)) => {
            let ngspice_exe = prefer_windows_console_ngspice_exe(ngspice_exe)
                .unwrap_or_else(|err| panic!("{err}"));
            (source_root, ngspice_exe)
        }
    };

    assert!(
        source_root.join("tests").is_dir(),
        "NGSPICE_SOURCE_ROOT must point at a local ngspice source tree with tests/: {}",
        source_root.display()
    );
    assert!(
        ngspice_exe.is_file(),
        "NGSPICE_EXE must point at a freshly built local ngspice executable: {}",
        ngspice_exe.display()
    );

    let tests_dir = workspace_tests_dir();
    let cases = audit_cases_from_env(&tests_dir);
    assert!(
        !cases.is_empty(),
        "ngspice oracle audit discovered no cases; check NGSPICE_ORACLE_AUDIT_CASES/SUITES"
    );

    Some(AuditConfig {
        source_root,
        ngspice_exe,
        cases,
        timeout_ms: std::env::var("NGSPICE_ORACLE_AUDIT_TIMEOUT_MS")
            .ok()
            .and_then(|value| value.parse::<u128>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_TIMEOUT_MS),
        fail_on_drift: env_bool("NGSPICE_ORACLE_AUDIT_FAIL_ON_DRIFT"),
    })
}

fn path_env(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .filter(|value| !value.as_os_str().is_empty())
        .map(PathBuf::from)
}

fn prefer_windows_console_ngspice_exe(ngspice_exe: PathBuf) -> Result<PathBuf, String> {
    #[cfg(windows)]
    {
        let is_windows_gui_binary = ngspice_exe
            .file_name()
            .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("ngspice.exe"));
        if is_windows_gui_binary {
            let console_exe = ngspice_exe.with_file_name("ngspice_con.exe");
            if console_exe.is_file() {
                return Ok(console_exe);
            }
            return Err(format!(
                "NGSPICE_EXE points to Windows GUI ngspice.exe '{}', but sibling console binary '{}' is missing; set NGSPICE_EXE to ngspice_con.exe to avoid GUI popup dialogs",
                ngspice_exe.display(),
                console_exe.display()
            ));
        }
    }
    Ok(ngspice_exe)
}

fn workspace_tests_dir() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("could not find workspace root")
        .join("tests")
}

fn audit_cases_from_env(tests_dir: &Path) -> Vec<String> {
    let mut cases = if let Ok(raw_cases) = std::env::var("NGSPICE_ORACLE_AUDIT_CASES") {
        split_list(&raw_cases)
            .into_iter()
            .map(|case| normalize_case_path(&case))
            .collect::<Vec<_>>()
    } else if let Ok(raw_suites) = std::env::var("NGSPICE_ORACLE_AUDIT_SUITES") {
        cases_for_suites(tests_dir, &split_list(&raw_suites))
    } else {
        vec![DEFAULT_CASE.to_string()]
    };

    cases.retain(|case| case.to_ascii_lowercase().ends_with(".cir"));
    cases.sort();
    cases.dedup();

    if let Some(limit) = std::env::var("NGSPICE_ORACLE_AUDIT_MAX_CASES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
    {
        cases.truncate(limit);
    }

    cases
}

fn cases_for_suites(tests_dir: &Path, suites: &[String]) -> Vec<String> {
    let suite_names = if suites.iter().any(|suite| suite.eq_ignore_ascii_case("all")) {
        discover_suite_dirs(tests_dir)
    } else {
        suites
            .iter()
            .map(|suite| suite.trim_matches('/').to_string())
            .filter(|suite| !suite.is_empty())
            .collect()
    };

    suite_names
        .into_iter()
        .flat_map(|suite| discover_suite_cases(tests_dir, &suite))
        .collect()
}

fn split_list(raw: &str) -> Vec<String> {
    raw.split([',', ';'])
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn normalize_case_path(raw: &str) -> String {
    raw.replace('\\', "/")
        .trim()
        .trim_start_matches("./")
        .trim_start_matches("tests/")
        .to_string()
}

fn discover_suite_dirs(tests_dir: &Path) -> Vec<String> {
    let mut dirs = BTreeSet::new();
    for cir in all_circuit_paths(tests_dir) {
        if let Some(parent) = cir.parent()
            && let Ok(rel) = parent.strip_prefix(tests_dir)
            && !rel.as_os_str().is_empty()
        {
            dirs.insert(path_to_rel(rel));
        }
    }
    dirs.into_iter().collect()
}

fn discover_suite_cases(tests_dir: &Path, suite: &str) -> Vec<String> {
    let dir = tests_dir.join(PathBuf::from(suite));
    if !dir.is_dir() {
        return Vec::new();
    }

    let mut cases = BTreeSet::new();
    if let Some(from_makefile) = discover_makefile_cases(&dir) {
        cases.extend(
            from_makefile
                .into_iter()
                .filter_map(|path| path.strip_prefix(tests_dir).ok().map(path_to_rel)),
        );
    }

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
                && let Ok(rel) = path.strip_prefix(tests_dir)
            {
                cases.insert(path_to_rel(rel));
            }
        }
    }

    cases.into_iter().collect()
}

fn discover_makefile_cases(dir: &Path) -> Option<Vec<PathBuf>> {
    let content = fs::read_to_string(dir.join("Makefile.am")).ok()?;
    let mut in_tests_block = false;
    let mut tokens = Vec::new();

    for raw_line in content.lines() {
        let no_comment = raw_line
            .split_once('#')
            .map(|(head, _)| head)
            .unwrap_or(raw_line);
        let trimmed = no_comment.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !in_tests_block {
            let Some((lhs, rhs)) = trimmed.split_once('=') else {
                continue;
            };
            if lhs.trim() != "TESTS" {
                continue;
            }
            in_tests_block = true;
            let continuation = rhs.trim_end().ends_with('\\');
            tokens.extend(
                rhs.trim_end_matches('\\')
                    .split_whitespace()
                    .map(str::to_string),
            );
            if !continuation {
                break;
            }
            continue;
        }

        let continuation = trimmed.ends_with('\\');
        tokens.extend(
            trimmed
                .trim_end_matches('\\')
                .split_whitespace()
                .map(str::to_string),
        );
        if !continuation {
            break;
        }
    }

    let cases = tokens
        .into_iter()
        .filter(|token| !token.contains('$') && token.to_ascii_lowercase().ends_with(".cir"))
        .map(|token| dir.join(token))
        .filter(|path| path.exists())
        .collect::<Vec<_>>();
    (!cases.is_empty()).then_some(cases)
}

fn all_circuit_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
            {
                paths.push(path);
            }
        }
    }

    paths.sort();
    paths
}

fn path_to_rel(path: &Path) -> String {
    path.iter()
        .map(|segment| segment.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

fn env_bool(name: &str) -> bool {
    std::env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn run_audit(config: &AuditConfig) -> AuditReport {
    let tests_dir = workspace_tests_dir();
    let source_tests_dir = config.source_root.join("tests");
    let mut report = AuditReport::default();

    println!(
        "ngspice oracle audit: {} case(s), source={}, exe={}, timeout={}ms, fail_on_drift={}",
        config.cases.len(),
        config.source_root.display(),
        config.ngspice_exe.display(),
        config.timeout_ms,
        config.fail_on_drift
    );

    for rel in &config.cases {
        report.total += 1;
        let checked_cir = tests_dir.join(PathBuf::from(rel));
        let checked_out = checked_cir.with_extension("out");
        let source_cir = source_tests_dir.join(PathBuf::from(rel));
        let source_out = source_cir.with_extension("out");

        if !checked_out.exists() {
            report.skipped += 1;
            println!("ORACLE SKIP  {rel} -- no checked-in .out file");
            continue;
        }
        if !source_cir.exists() {
            report.errors += 1;
            println!(
                "ORACLE ERROR {rel} -- source deck is missing from {}",
                source_tests_dir.display()
            );
            continue;
        }

        let expected_bytes = match fs::read(&checked_out) {
            Ok(expected) => expected,
            Err(err) => {
                report.errors += 1;
                println!(
                    "ORACLE ERROR {rel} -- failed to read {}: {err}",
                    checked_out.display()
                );
                continue;
            }
        };

        if source_out.exists() {
            match fs::read(&source_out) {
                Ok(source_bytes) if source_bytes == expected_bytes => {}
                Ok(_) => {
                    report.errors += 1;
                    println!(
                        "ORACLE ERROR {rel} -- checked-in .out differs from authoritative source {}",
                        source_out.display()
                    );
                    continue;
                }
                Err(err) => {
                    report.errors += 1;
                    println!(
                        "ORACLE ERROR {rel} -- failed to read authoritative source {}: {err}",
                        source_out.display()
                    );
                    continue;
                }
            }
        }

        let expected = String::from_utf8_lossy(&expected_bytes);
        let live = match run_live_ngspice(config, &source_cir) {
            Ok(live) => live,
            Err(err) => {
                report.errors += 1;
                println!("ORACLE ERROR {rel} -- {err}");
                continue;
            }
        };

        let comparison = compare_outputs(&expected, &live.stdout);
        if comparison.passed {
            report.passed += 1;
            println!(
                "ORACLE PASS  {rel} -- live rows {}, {}ms",
                format_counts(&comparison.live_row_counts),
                live.duration_ms
            );
        } else {
            report.drifted += 1;
            print_drift(rel, &live, &comparison);
        }
    }

    println!(
        "ngspice oracle audit summary: {} total | {} passed | {} drifted | {} skipped | {} errors",
        report.total, report.passed, report.drifted, report.skipped, report.errors
    );
    report
}

fn run_live_ngspice(config: &AuditConfig, source_cir: &Path) -> Result<LiveOutput, String> {
    let start = Instant::now();
    let stdout_path = unique_temp_path(source_cir, "stdout");
    let stderr_path = unique_temp_path(source_cir, "stderr");
    let stdout_file = fs::File::create(&stdout_path).map_err(|err| {
        format!(
            "failed to create temporary stdout file '{}': {err}",
            stdout_path.display()
        )
    })?;
    let stderr_file = fs::File::create(&stderr_path).map_err(|err| {
        format!(
            "failed to create temporary stderr file '{}': {err}",
            stderr_path.display()
        )
    })?;

    let source_parent = source_cir.parent().unwrap_or(&config.source_root);
    let mut child = Command::new(&config.ngspice_exe)
        .arg("--batch")
        .arg(source_cir)
        .current_dir(source_parent)
        .env(
            "SPICE_SCRIPTS",
            config.source_root.join("tests").join("bin"),
        )
        .env("ngspice_vpath", source_parent)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|err| {
            format!(
                "failed to spawn local ngspice '{}': {err}",
                config.ngspice_exe.display()
            )
        })?;

    let timeout = Duration::from_millis(config.timeout_ms.min(u64::MAX as u128) as u64);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
                let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
                let _ = fs::remove_file(&stdout_path);
                let _ = fs::remove_file(&stderr_path);
                return Err(format!(
                    "ngspice exceeded oracle audit timeout ({}ms); stdout tail: {}; stderr tail: {}",
                    config.timeout_ms,
                    tail(&stdout),
                    tail(&stderr)
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(50)),
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&stdout_path);
                let _ = fs::remove_file(&stderr_path);
                return Err(format!("failed to poll local ngspice: {err}"));
            }
        }
    };

    let stdout = fs::read_to_string(&stdout_path).map_err(|err| {
        format!(
            "failed to read temporary stdout file '{}': {err}",
            stdout_path.display()
        )
    })?;
    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
    let _ = fs::remove_file(&stdout_path);
    let _ = fs::remove_file(&stderr_path);

    if !status.success() {
        return Err(format!(
            "ngspice exited with status {status}; stdout tail: {}; stderr tail: {}",
            tail(&stdout),
            tail(&stderr)
        ));
    }

    Ok(LiveOutput {
        stdout,
        stderr,
        duration_ms: start.elapsed().as_millis(),
        status: status.to_string(),
    })
}

fn unique_temp_path(cir_path: &Path, suffix: &str) -> PathBuf {
    let stem = cir_path
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

    std::env::temp_dir().join(format!(
        "rspice-ngspice-oracle-{stem}-{suffix}-{}-{}.txt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos()
    ))
}

fn compare_outputs(expected: &str, live: &str) -> Comparison {
    let expected_lines = normalize_check_lines(expected);
    let live_lines = normalize_check_lines(live);
    let diffs = first_diffs(&expected_lines, &live_lines, MAX_DIFFS);

    Comparison {
        passed: diffs.is_empty(),
        expected_row_counts: extract_row_counts(expected),
        live_row_counts: extract_row_counts(live),
        expected_filtered_lines: expected_lines.len(),
        live_filtered_lines: live_lines.len(),
        diffs,
    }
}

fn normalize_check_lines(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|raw_line| {
            let trimmed = raw_line
                .trim_matches(|ch: char| ch.is_whitespace() || ch == '\u{000c}')
                .trim();
            if trimmed.is_empty() {
                return None;
            }
            let normalized = normalize_windows_exponents(trimmed);
            if line_is_filtered(&normalized) {
                return None;
            }
            Some(normalized)
        })
        .collect()
}

fn line_is_filtered(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("binary raw file") {
        return true;
    }
    if trimmed.starts_with("ngspice") && trimmed.contains("done") {
        return true;
    }
    CHECK_FILTER_PATTERNS
        .iter()
        .any(|pattern| line.contains(pattern))
}

fn normalize_windows_exponents(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(input.len());
    let mut idx = 0usize;

    while idx < chars.len() {
        let ch = chars[idx];
        if matches!(ch, 'e' | 'E')
            && idx > 0
            && (chars[idx - 1].is_ascii_digit() || chars[idx - 1] == '.')
        {
            output.push(ch);
            idx += 1;
            if idx < chars.len() && matches!(chars[idx], '+' | '-') {
                output.push(chars[idx]);
                idx += 1;
            }
            if idx + 2 < chars.len()
                && chars[idx] == '0'
                && chars[idx + 1].is_ascii_digit()
                && chars[idx + 2].is_ascii_digit()
            {
                idx += 1;
            }
            continue;
        }

        output.push(ch);
        idx += 1;
    }

    output
}

fn first_diffs(expected: &[String], live: &[String], max_diffs: usize) -> Vec<DiffLine> {
    let mut diffs = Vec::new();
    for idx in 0..expected.len().max(live.len()) {
        let expected_line = expected.get(idx);
        let live_line = live.get(idx);
        let equal = match (expected_line, live_line) {
            (Some(expected_line), Some(live_line)) => {
                line_key(expected_line) == line_key(live_line)
            }
            (None, None) => true,
            _ => false,
        };
        if equal {
            continue;
        }

        diffs.push(DiffLine {
            line_index: idx,
            expected: expected_line.cloned(),
            live: live_line.cloned(),
        });
        if diffs.len() >= max_diffs {
            break;
        }
    }
    diffs
}

fn line_key(line: &str) -> String {
    line.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn extract_row_counts(content: &str) -> Vec<usize> {
    content
        .lines()
        .filter_map(|line| {
            if !line.to_ascii_lowercase().contains("no. of data rows") {
                return None;
            }
            line.split_once(':')
                .map(|(_, rhs)| rhs)
                .unwrap_or(line)
                .split_whitespace()
                .find_map(|token| {
                    token
                        .trim_matches(|ch: char| !ch.is_ascii_digit())
                        .parse::<usize>()
                        .ok()
                })
        })
        .collect()
}

fn print_drift(rel: &str, live: &LiveOutput, comparison: &Comparison) {
    println!(
        "ORACLE DRIFT {rel} -- expected rows {}, live rows {}, filtered lines {} -> {}, {}ms, status {}",
        format_counts(&comparison.expected_row_counts),
        format_counts(&comparison.live_row_counts),
        comparison.expected_filtered_lines,
        comparison.live_filtered_lines,
        live.duration_ms,
        live.status
    );
    if !live.stderr.trim().is_empty() {
        println!("  stderr tail: {}", tail(&live.stderr));
    }
    for diff in &comparison.diffs {
        println!("  diff filtered line {}", diff.line_index + 1);
        println!(
            "    expected: {}",
            diff.expected
                .as_deref()
                .map(|line| truncate(line, 160))
                .unwrap_or_else(|| "<missing>".to_string())
        );
        println!(
            "    live:     {}",
            diff.live
                .as_deref()
                .map(|line| truncate(line, 160))
                .unwrap_or_else(|| "<missing>".to_string())
        );
    }
}

fn format_counts(counts: &[usize]) -> String {
    if counts.is_empty() {
        "none".to_string()
    } else {
        counts
            .iter()
            .map(|count| count.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn tail(content: &str) -> String {
    let lines = content
        .lines()
        .rev()
        .take(3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" | ");
    truncate(lines.trim(), 240)
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut iter = value.chars();
    let truncated = iter.by_ref().take(max_chars).collect::<String>();
    if iter.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}

#[test]
#[cfg(windows)]
fn audit_windows_gui_ngspice_exe_prefers_sibling_console_binary() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("rspice_ngspice_audit_console_pick_{unique}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    let gui = dir.join("ngspice.exe");
    let console = dir.join("ngspice_con.exe");
    fs::write(&gui, b"gui").expect("write gui marker");
    fs::write(&console, b"console").expect("write console marker");

    let resolved = prefer_windows_console_ngspice_exe(gui.clone()).expect("console exists");

    let _ = fs::remove_dir_all(&dir);
    assert_eq!(resolved, console);
}

#[test]
#[cfg(windows)]
fn audit_windows_gui_ngspice_exe_without_sibling_console_is_rejected() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("rspice_ngspice_audit_console_missing_{unique}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    let gui = dir.join("ngspice.exe");
    fs::write(&gui, b"gui").expect("write gui marker");

    let err = prefer_windows_console_ngspice_exe(gui.clone())
        .expect_err("GUI ngspice.exe must not be used without ngspice_con.exe");

    let _ = fs::remove_dir_all(&dir);
    assert!(err.contains("ngspice_con.exe"), "{err}");
    assert!(err.contains(&gui.display().to_string()), "{err}");
}

#[test]
fn audit_filter_matches_ngspice_check_script_noise() {
    let content = "\
Circuit: demo
Note: local build metadata
Using KLU as Direct Linear Solver
No. of Data Rows : 1
Index   time            v(out)
0\t1.000000e-004\t2.000000e+003
";

    assert_eq!(
        normalize_check_lines(content),
        vec!["0\t1.000000e-04\t2.000000e+03"]
    );
}

#[test]
fn audit_compare_reports_row_and_filtered_line_drift() {
    let expected = "\
No. of Data Rows : 2
Index   time            v(out)
0\t0.000000e+00\t1.000000e+00
1\t1.000000e-09\t2.000000e+00
";
    let live = "\
No. of Data Rows : 1
Index   time            v(out)
0\t0.000000e+00\t1.000000e+00
";

    let comparison = compare_outputs(expected, live);

    assert!(!comparison.passed);
    assert_eq!(comparison.expected_row_counts, vec![2]);
    assert_eq!(comparison.live_row_counts, vec![1]);
    assert_eq!(comparison.expected_filtered_lines, 2);
    assert_eq!(comparison.live_filtered_lines, 1);
    assert_eq!(comparison.diffs.len(), 1);
    assert_eq!(comparison.diffs[0].line_index, 1);
}
