//! Capture compact ngspice reference tables for the oracle-free corpora.
//!
//! The normal test path never invokes ngspice: references are checked in as
//! ordinary `.oracle.out` files beside their decks and are consumed by the
//! existing ngspice table parser. The dedicated suffix cannot collide with
//! upstream `.out` logs already vendored in Paranoia. This module only owns
//! the explicit maintenance command that creates or verifies those files.

use super::*;
use crate::suites::ngspice::reference::live::{RawReferencePlot, parse_ngspice_raw_plots};
use crate::suites::ngspice::{ReferenceSeries, ReferenceTable, TestRunner};
use num_complex::Complex64;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const FORMAT_VERSION: u32 = 1;
const MAX_REFERENCE_ROWS: usize = 1_001;
const MAX_IMPLICIT_VARIABLES: usize = 64;
const NGSPICE_CAPTURE_THREADS: usize = 8;

#[derive(Debug, Clone, Default)]
pub struct OracleCaptureStatistics {
    pub discovered: usize,
    pub eligible: usize,
    pub written: usize,
    pub verified: usize,
    pub without_outputs: usize,
    pub failures: Vec<String>,
}

impl OracleCaptureStatistics {
    pub fn failed(&self) -> usize {
        self.failures.len()
    }
}

#[derive(Debug, Clone)]
struct OutputRequest {
    axis: Option<String>,
    expression: String,
}

/// Capture or verify every standalone deck in one corpus.
///
/// References use an adjacent `.oracle.out` convention and the same table
/// format as `tests/ngspice`, so the production comparison path can reuse its
/// mature parser and tolerance implementation. `update` must be explicit;
/// ordinary verification is read-only and fails on any drift.
pub fn capture_ngspice_oracles(
    corpus: ExecutionCorpus,
    tests_dir: &Path,
    ngspice_exe: &Path,
    case: Option<&str>,
    timeout_ms: u128,
    update: bool,
) -> Result<OracleCaptureStatistics, String> {
    let ngspice_exe = conventional_windows_path(
        ngspice_exe
            .canonicalize()
            .map_err(|err| format!("failed to resolve ngspice executable: {err}"))?,
    );
    if !ngspice_exe.is_file() {
        return Err(format!(
            "ngspice executable does not exist: {}",
            ngspice_exe.display()
        ));
    }
    validate_headless_ngspice_executable(&ngspice_exe)?;
    if timeout_ms == 0 {
        return Err("ngspice timeout must be greater than zero".to_string());
    }

    let version = ngspice_version(&ngspice_exe)?;
    let runner = ExecutionRunner::new(
        corpus,
        tests_dir,
        ExecutionConfig {
            max_time_per_deck_ms: timeout_ms,
            skip_extended: false,
            verbose: false,
        },
    );
    let requested_case = case.map(manifest::normalize_key);
    let decks = runner.discover();
    let mut stats = OracleCaptureStatistics {
        discovered: decks.len(),
        ..OracleCaptureStatistics::default()
    };

    for key in decks {
        if requested_case
            .as_deref()
            .is_some_and(|wanted| wanted != key)
        {
            continue;
        }
        // Include current gap/refusal cases in the capture audit. If ngspice
        // can run one, its oracle is checked in before RSpice support lands;
        // the execution manifest then remains the only temporary gap record.
        // Include-only files have no standalone simulation to reference.
        if runner.contract_for(&key) == ExecutionContract::LibraryFragment {
            continue;
        }
        stats.eligible += 1;
        let deck_path = runner.deck_path(&key);
        let reference_path = deck_path.with_extension("oracle.out");
        if let Some(reason) = oracle_capture_exclusion(corpus, &key) {
            if update && let Err(err) = remove_oracle_if_present(&reference_path) {
                stats.failures.push(format!(
                    "{key}: failed to remove stale oracle '{}': {err}",
                    reference_path.display()
                ));
                continue;
            }
            stats.without_outputs += 1;
            eprintln!("execution-only {key}: {reason}");
            continue;
        }
        match capture_one(&ngspice_exe, &version, &deck_path, timeout_ms) {
            Ok(None) => {
                if update && let Err(err) = remove_oracle_if_present(&reference_path) {
                    stats.failures.push(format!(
                        "{key}: failed to remove stale oracle '{}': {err}",
                        reference_path.display()
                    ));
                    continue;
                }
                stats.without_outputs += 1;
                eprintln!("execution-only {key}: ngspice produced no comparable output vectors");
            }
            Ok(Some(content)) => {
                if update {
                    if let Err(err) = fs::write(&reference_path, content.as_bytes()) {
                        stats.failures.push(format!(
                            "{key}: failed to write '{}': {err}",
                            reference_path.display()
                        ));
                    } else {
                        stats.written += 1;
                        eprintln!("captured {key}");
                    }
                } else {
                    match fs::read_to_string(&reference_path) {
                        Ok(existing) if existing == content => {
                            stats.verified += 1;
                        }
                        Ok(_) => stats.failures.push(format!(
                            "{key}: checked-in oracle differs from ngspice {version}"
                        )),
                        Err(err) => stats.failures.push(format!(
                            "{key}: missing oracle '{}': {err}",
                            reference_path.display()
                        )),
                    }
                }
            }
            Err(err) => {
                if update && let Err(remove_err) = remove_oracle_if_present(&reference_path) {
                    stats.failures.push(format!(
                        "{key}: {err}; also failed to remove stale oracle '{}': {remove_err}",
                        reference_path.display()
                    ));
                } else {
                    stats.failures.push(format!("{key}: {err}"));
                }
            }
        }
    }

    if let Some(case) = requested_case
        && stats.eligible == 0
    {
        return Err(format!(
            "case '{case}' was not discovered as a standalone {} deck",
            corpus.label()
        ));
    }
    Ok(stats)
}

fn oracle_capture_exclusion(corpus: ExecutionCorpus, key: &str) -> Option<&'static str> {
    (corpus == ExecutionCorpus::Iscas85 && key == "85/c7552/c7552_ann_oc.net").then_some(
        "the full ngspice transient exceeds one hour; retained as an extended execution/scale test",
    )
}

fn remove_oracle_if_present(path: &Path) -> std::io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

fn capture_one(
    ngspice_exe: &Path,
    version: &str,
    deck_path: &Path,
    timeout_ms: u128,
) -> Result<Option<String>, String> {
    let source =
        fs::read_to_string(deck_path).map_err(|err| format!("failed to read deck: {err}"))?;
    let expanded = Netlist::preprocess_includes(&source, deck_path)
        .map_err(|err| format!("failed to expand includes: {err}"))?;
    let input_hash = blake3::hash(expanded.as_bytes()).to_hex().to_string();
    let mut requests = output_requests(&expanded);
    if requests.is_empty() {
        requests = implicit_output_requests(&expanded);
    }
    let reference_source = reference_deck(&expanded, &requests);
    let plots = run_ngspice_raw(ngspice_exe, deck_path, &reference_source, timeout_ms)?;
    let tables = reference_tables(&plots, &requests);
    if tables.is_empty() {
        return Ok(None);
    }
    Ok(Some(serialize_reference(version, &input_hash, &tables)))
}

fn ngspice_version(ngspice_exe: &Path) -> Result<String, String> {
    let output = headless_command(ngspice_exe)
        .arg("--version")
        .output()
        .map_err(|err| format!("failed to query ngspice version: {err}"))?;
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    text.lines()
        .map(str::trim)
        .find(|line| line.to_ascii_lowercase().contains("ngspice-"))
        .map(|line| line.trim_matches('*').trim().to_string())
        .ok_or_else(|| "ngspice --version did not report a version".to_string())
}

fn run_ngspice_raw(
    ngspice_exe: &Path,
    deck_path: &Path,
    reference_source: &str,
    timeout_ms: u128,
) -> Result<Vec<RawReferencePlot>, String> {
    let temp = tempfile::Builder::new()
        .prefix("rspice-ngspice-oracle-")
        .tempdir()
        .map_err(|err| format!("failed to create oracle temporary directory: {err}"))?;
    let raw_path = temp.path().join("reference.raw");
    let reference_deck_path = temp.path().join("reference.cir");
    let stdout_path = temp.path().join("stdout.txt");
    let stderr_path = temp.path().join("stderr.txt");
    let stdout = fs::File::create(&stdout_path)
        .map_err(|err| format!("failed to create ngspice stdout file: {err}"))?;
    let stderr = fs::File::create(&stderr_path)
        .map_err(|err| format!("failed to create ngspice stderr file: {err}"))?;
    fs::write(&reference_deck_path, reference_source)
        .map_err(|err| format!("failed to create normalized reference deck: {err}"))?;
    let deck_path = conventional_windows_path(deck_path.to_path_buf());
    let reference_deck_path = conventional_windows_path(reference_deck_path);
    let parent = deck_path.parent().unwrap_or_else(|| Path::new("."));

    let mut child = headless_command(ngspice_exe)
        .arg("--batch")
        .arg("--no-spiceinit")
        .arg("-r")
        .arg(&raw_path)
        .arg(&reference_deck_path)
        .current_dir(parent)
        .env_remove("SPICE_SCRIPTS")
        .env_remove("ngspice_vpath")
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .map_err(|err| format!("failed to spawn ngspice: {err}"))?;

    let started = Instant::now();
    let timeout = Duration::from_millis(timeout_ms.min(u64::MAX as u128) as u64);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("ngspice timed out after {timeout_ms}ms"));
            }
            Ok(None) => thread::sleep(Duration::from_millis(50)),
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("failed to poll ngspice: {err}"));
            }
        }
    };
    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
    if !status.success() {
        return Err(format!(
            "ngspice exited with {status}; stdout: {}; stderr: {}",
            tail(&stdout),
            tail(&stderr)
        ));
    }
    let raw = fs::read(&raw_path).map_err(|err| {
        format!(
            "ngspice produced no readable rawfile: {err}; stdout: {}; stderr: {}",
            tail(&stdout),
            tail(&stderr)
        )
    })?;
    match parse_ngspice_raw_plots(&raw) {
        Ok(plots) => Ok(plots),
        Err(err) if std::env::var_os("RSPICE_KEEP_FAILED_ORACLE_RAW").is_some() => {
            let kept = temp.keep();
            Err(format!(
                "{err}; failed capture retained at {}",
                kept.display()
            ))
        }
        Err(err) => Err(err),
    }
}

fn validate_headless_ngspice_executable(path: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if !name.eq_ignore_ascii_case("ngspice_con.exe") {
            return Err(format!(
                "Windows oracle capture requires ngspice_con.exe; '{}' may open the ngspice GUI",
                path.display()
            ));
        }
    }
    #[cfg(not(windows))]
    let _ = path;
    Ok(())
}

fn headless_command(executable: &Path) -> Command {
    let mut command = Command::new(executable);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW: batch maintenance must never surface a console
        // or GUI-adjacent process window on the developer's desktop.
        command.creation_flags(0x0800_0000);
    }
    command
}

/// Build the deterministic deck whose analyses RSpice actually executes.
///
/// Paranoia's `.control` programs frequently destroy plots, overwrite the
/// command-line rawfile, invoke GUI programs, or repeat a run after `alter`.
/// RSpice deliberately promotes analysis commands from those blocks but does
/// not interpret the surrounding shell language. Capturing the same promoted
/// analyses gives both simulators the same circuit and keeps the oracle free
/// of workstation/UI side effects.
fn reference_deck(expanded_source: &str, requests: &[OutputRequest]) -> String {
    let mut body = Vec::<String>::new();
    let mut promoted = Vec::<String>::new();
    let mut netlist_analyses = Vec::<String>::new();
    let mut parameter_names = BTreeSet::<String>::new();
    let mut scalar_lets = BTreeMap::<String, String>::new();
    let mut in_control = false;

    for raw in expanded_source.lines() {
        let trimmed = raw.trim();
        let command = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_start_matches('.')
            .to_ascii_lowercase();
        if command == "control" {
            in_control = true;
            continue;
        }
        if command == "endc" {
            in_control = false;
            continue;
        }
        if in_control {
            if command == "let"
                && let Some((name, expression)) = control_scalar_let_assignment(trimmed)
            {
                let expression = expand_control_scalar_expression(&expression, &scalar_lets);
                scalar_lets.insert(name, expression);
            }
            if matches!(command.as_str(), "op" | "dc" | "ac" | "tran" | "sp") {
                let mut parts = trimmed.trim_start_matches('.').split_whitespace();
                let mut directive = format!(".{}", parts.next().unwrap_or_default());
                for part in parts {
                    directive.push(' ');
                    directive.push_str(&normalize_control_analysis_token(part, &scalar_lets));
                }
                if !promoted
                    .iter()
                    .any(|existing| normalize(existing) == normalize(&directive))
                {
                    promoted.push(directive);
                }
            }
            continue;
        }
        if command == "param" {
            remember_parameter_names(trimmed, &mut parameter_names);
            body.push(raw.to_string());
        } else if command == "csparam" {
            // `.CSPARAM` creates both a netlist parameter and a control
            // vector in ngspice. Once the control script is removed, retain
            // the netlist half explicitly; this is also how RSpice parses it.
            let rest = trimmed
                .split_once(char::is_whitespace)
                .map_or("", |(_, rest)| rest.trim());
            let name = rest.split_once('=').map(|(name, _)| normalize(name));
            if name
                .as_ref()
                .is_none_or(|name| !parameter_names.contains(name))
            {
                body.push(format!(".param {rest}"));
                remember_parameter_names(rest, &mut parameter_names);
            }
        } else {
            if matches!(command.as_str(), "op" | "dc" | "ac" | "tran" | "sp") {
                netlist_analyses.push(trimmed.to_string());
            }
            body.push(raw.to_string());
        }
    }

    let insertion = body
        .iter()
        .position(|line| line.trim().eq_ignore_ascii_case(".end"))
        .unwrap_or(body.len());
    let mut additions = promoted
        .into_iter()
        .filter(|directive| {
            !netlist_analyses
                .iter()
                .any(|existing| normalize(existing) == normalize(directive))
        })
        .collect::<Vec<_>>();
    additions.push(reference_save_directive(requests));
    additions.push(".control".to_string());
    additions.push(format!("set num_threads={NGSPICE_CAPTURE_THREADS}"));
    additions.push("run".to_string());
    additions.push(".endc".to_string());
    body.splice(insertion..insertion, additions);

    let mut source = body.join("\n");
    source.push('\n');
    if !body
        .iter()
        .any(|line| line.trim().eq_ignore_ascii_case(".end"))
    {
        source.push_str(".end\n");
    }
    source
}

fn reference_save_directive(requests: &[OutputRequest]) -> String {
    let mut targets = Vec::<String>::new();
    let mut requires_all = requests.is_empty();
    for request in requests {
        if request.expression.eq_ignore_ascii_case("all") {
            continue;
        }
        let Some(target) = reference_save_target(&request.expression) else {
            requires_all = true;
            break;
        };
        if !targets
            .iter()
            .any(|existing| normalize(existing) == normalize(&target))
        {
            targets.push(target);
        }
    }
    if requires_all || targets.is_empty() {
        ".save all".to_string()
    } else {
        format!(".save {}", targets.join(" "))
    }
}

fn reference_save_target(expression: &str) -> Option<String> {
    let mut expression = expression.trim().trim_matches(',');
    if expression.is_empty() {
        return None;
    }
    if matches!(
        expression.to_ascii_lowercase().as_str(),
        "all" | "allv" | "alli"
    ) {
        return None;
    }
    // Control scripts qualify vectors with the plot which produced them
    // (`dc1.vc1#branch`). `.save` runs before those plot names exist and
    // therefore needs the underlying raw-vector spelling.
    if let Some((plot, raw_name)) = expression.split_once('.')
        && plot
            .chars()
            .last()
            .is_some_and(|character| character.is_ascii_digit())
    {
        expression = raw_name;
    }
    if expression.to_ascii_lowercase().ends_with("#branch") {
        return Some(expression.to_string());
    }
    if !expression.contains('(') {
        return expression
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | ':' | '#'))
            .then(|| format!("v({expression})"));
    }
    let (function, inner) = expression.split_once('(')?;
    let inner = inner.strip_suffix(')')?;
    if function.eq_ignore_ascii_case("abs") {
        return reference_save_target(inner);
    }
    let prefix = function.chars().next()?.to_ascii_lowercase();
    if !matches!(prefix, 'v' | 'i') || inner.is_empty() {
        return None;
    }
    Some(format!("{prefix}({inner})"))
}

fn control_scalar_let_assignment(line: &str) -> Option<(String, String)> {
    let (_, rest) = line.split_once(char::is_whitespace)?;
    let (name, expression) = rest.trim().split_once('=')?;
    let name = name.trim();
    let expression = expression.trim();
    if name.is_empty()
        || expression.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        || !expression.chars().all(|ch| {
            ch.is_ascii_alphanumeric()
                || ch.is_ascii_whitespace()
                || matches!(
                    ch,
                    '_' | '.' | '+' | '-' | '*' | '/' | '^' | '{' | '}' | '\''
                )
        })
    {
        return None;
    }
    let arithmetic = expression
        .chars()
        .any(|ch| matches!(ch, '+' | '-' | '*' | '/' | '^'));
    if !arithmetic && rspice_core::netlist::lexer::parse_spice_value(expression).is_err() {
        return None;
    }
    let expression = expression
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
        .or_else(|| {
            expression
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
        })
        .unwrap_or(expression);
    Some((name.to_ascii_uppercase(), expression.to_string()))
}

fn expand_control_scalar_expression(
    expression: &str,
    scalar_lets: &BTreeMap<String, String>,
) -> String {
    let mut output = String::with_capacity(expression.len());
    let mut identifier = String::new();
    for character in expression.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            identifier.push(character);
            continue;
        }
        if !identifier.is_empty() {
            append_control_scalar_identifier(&mut output, &identifier, scalar_lets);
            identifier.clear();
        }
        output.push(character);
    }
    if !identifier.is_empty() {
        append_control_scalar_identifier(&mut output, &identifier, scalar_lets);
    }
    output
}

fn append_control_scalar_identifier(
    output: &mut String,
    identifier: &str,
    scalar_lets: &BTreeMap<String, String>,
) {
    if let Some(expression) = scalar_lets.get(&identifier.to_ascii_uppercase()) {
        output.push('(');
        output.push_str(expression);
        output.push(')');
    } else {
        output.push_str(identifier);
    }
}

fn remember_parameter_names(line: &str, names: &mut BTreeSet<String>) {
    let assignments = line
        .trim()
        .strip_prefix(".param")
        .or_else(|| line.trim().strip_prefix(".PARAM"))
        .unwrap_or(line);
    // The assignment operator is commonly surrounded by whitespace, so
    // splitting into whitespace fields misses `.param stop = 10n`.  Each
    // equals sign belongs to the identifier immediately before it; the text
    // after the preceding equals sign may contain the prior value.
    let assignment_count = assignments.matches('=').count();
    for prefix in assignments.split('=').take(assignment_count) {
        let Some(name) = prefix.split_whitespace().next_back() else {
            continue;
        };
        if name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.'))
        {
            names.insert(normalize(name));
        }
    }
}

fn normalize_control_analysis_token(token: &str, scalar_lets: &BTreeMap<String, String>) -> String {
    if let Some(name) = token.strip_prefix("$&").filter(|name| {
        !name.is_empty()
            && name
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.'))
    }) {
        scalar_lets.get(&name.to_ascii_uppercase()).map_or_else(
            || name.to_string(),
            |expression| format!("{{{expression}}}"),
        )
    } else {
        token.to_string()
    }
}

fn output_requests(source: &str) -> Vec<OutputRequest> {
    let mut requests = Vec::new();
    for line in folded_lines(source) {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let Some(command) = parts.first() else {
            continue;
        };
        let command = command.trim_start_matches('.').to_ascii_lowercase();
        match command.as_str() {
            "print" if parts.len() >= 2 => {
                let axis = parts
                    .get(1)
                    .and_then(|analysis| axis_for_analysis(analysis));
                let first_expression = usize::from(axis.is_some()) + 1;
                for expression in &parts[first_expression..] {
                    push_request(&mut requests, axis.clone(), expression);
                }
            }
            "plot" if parts.len() >= 2 => {
                // Netlist `.plot tran ...` carries an analysis keyword;
                // control-language `plot v(out) ...` does not.  Treating the
                // first control expression as an analysis name silently
                // discarded the primary requested waveform.
                let axis = line
                    .starts_with('.')
                    .then(|| {
                        parts
                            .get(1)
                            .and_then(|analysis| axis_for_analysis(analysis))
                    })
                    .flatten();
                let first_expression = usize::from(axis.is_some()) + 1;
                for expression in &parts[first_expression..] {
                    push_request(&mut requests, axis.clone(), expression);
                }
            }
            "save" if parts.len() >= 2 => {
                for expression in &parts[1..] {
                    push_request(&mut requests, None, expression);
                }
            }
            _ => {}
        }
    }
    requests
}

fn implicit_output_requests(source: &str) -> Vec<OutputRequest> {
    let Ok(netlist) = Netlist::parse(source) else {
        return Vec::new();
    };
    let mut nodes = BTreeSet::<String>::new();
    for element in &netlist.elements {
        for node in &element.nodes {
            if !matches!(normalize(node).as_str(), "0" | "gnd" | "ground") {
                nodes.insert(node.clone());
            }
        }
    }
    nodes
        .into_iter()
        .take(MAX_IMPLICIT_VARIABLES)
        .map(|node| OutputRequest {
            axis: None,
            expression: format!("v({node})"),
        })
        .collect()
}

fn folded_lines(source: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for raw in source.lines() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('*') {
            continue;
        }
        let raw = raw.split_once('$').map_or(raw, |(head, _)| head).trim();
        if let Some(continuation) = raw.strip_prefix('+') {
            if let Some(last) = lines.last_mut() {
                last.push(' ');
                last.push_str(continuation.trim());
            }
        } else {
            lines.push(raw.to_string());
        }
    }
    lines
}

fn push_request(requests: &mut Vec<OutputRequest>, axis: Option<String>, expression: &str) {
    let expression = expression.trim_matches(',').trim();
    if expression.is_empty()
        || expression.contains('=')
        || matches!(
            expression.to_ascii_lowercase().as_str(),
            "xlimit" | "ylimit" | "xlog" | "ylog" | "linear" | "loglog"
        )
    {
        return;
    }
    if !requests.iter().any(|request| {
        request.axis == axis && normalize(&request.expression) == normalize(expression)
    }) {
        requests.push(OutputRequest {
            axis,
            expression: expression.to_string(),
        });
    }
}

fn axis_for_analysis(analysis: &str) -> Option<String> {
    match analysis
        .trim_start_matches('.')
        .to_ascii_lowercase()
        .as_str()
    {
        "tran" => Some("time".to_string()),
        "dc" => Some("v-sweep".to_string()),
        "ac" | "noise" => Some("frequency".to_string()),
        "op" => Some("op".to_string()),
        _ => None,
    }
}

fn reference_tables(plots: &[RawReferencePlot], requests: &[OutputRequest]) -> Vec<ReferenceTable> {
    plots
        .iter()
        // The execution runner exposes the full spectral NoiseResult grid;
        // integrated-noise scalars are a separate API and have no frequency
        // coordinate.  Do not merge their synthetic x=0 table into the
        // spectrum table.
        .filter(|plot| {
            !plot
                .plotname
                .to_ascii_lowercase()
                .contains("integrated noise")
        })
        .filter_map(|plot| reference_table(plot, requests))
        .collect()
}

fn reference_table(plot: &RawReferencePlot, requests: &[OutputRequest]) -> Option<ReferenceTable> {
    let axis = plot_axis(plot);
    let axis_values = plot.data.first()?;
    if axis_values.is_empty() {
        return None;
    }
    let mut selected = BTreeMap::<String, Vec<f64>>::new();
    let relevant = requests
        .iter()
        .filter(|request| request.axis.as_deref().is_none_or(|value| value == axis))
        .collect::<Vec<_>>();
    for request in &relevant {
        let normalized = normalize(&request.expression);
        if matches!(normalized.as_str(), "all" | "allv") {
            continue;
        }
        if selected
            .keys()
            .any(|existing| normalize(existing) == normalized)
        {
            continue;
        }
        if let Some((output_name, values)) = resolve_raw_series(plot, &request.expression) {
            selected.insert(output_name, values);
        }
    }

    // `all` is a fallback request.  If the deck also names concrete outputs,
    // those are the portable contract; simulator-private saved nodes from an
    // integrated-noise or XSPICE plot must not poison the whole table.
    if selected.is_empty() {
        for (index, name) in plot.variables.iter().enumerate().skip(1) {
            if selected.len() >= MAX_IMPLICIT_VARIABLES {
                break;
            }
            // Implicit coverage is deliberately node-voltage coverage. Raw
            // XSPICE/internal branch names are simulator-private and often
            // have no RSpice result symbol; explicitly requested currents
            // were already selected above and remain fully compared.
            if !normalize(name).starts_with("v(") || !comparable_raw_variable(name) {
                continue;
            }
            let normalized = normalize(name);
            if selected
                .keys()
                .any(|existing| normalize(existing) == normalized)
            {
                continue;
            }
            if let Some(values) = plot.data.get(index) {
                selected.entry(name.clone()).or_insert_with(|| {
                    values
                        .iter()
                        .copied()
                        .map(if plot.is_complex { magnitude } else { real })
                        .collect()
                });
            }
        }
    }
    if selected.is_empty() {
        return None;
    }

    let indices = sample_indices(axis_values.len(), MAX_REFERENCE_ROWS)
        .into_iter()
        .filter(|index| {
            axis_values[*index].re.is_finite()
                && selected
                    .values()
                    .all(|values| values.get(*index).is_some_and(|value| value.is_finite()))
        })
        .collect::<Vec<_>>();
    if indices.is_empty() {
        return None;
    }
    let sampled_axis = indices
        .iter()
        .map(|index| axis_values[*index].re)
        .collect::<Vec<_>>();
    let variables = selected
        .into_iter()
        .filter_map(|(name, values)| {
            (values.len() == axis_values.len()).then(|| {
                let y = indices.iter().map(|index| values[*index]).collect();
                (
                    name,
                    ReferenceSeries {
                        x: sampled_axis.clone(),
                        y,
                    },
                )
            })
        })
        .collect();
    Some(ReferenceTable {
        x_name: axis,
        variables,
    })
}

fn resolve_raw_series(plot: &RawReferencePlot, expression: &str) -> Option<(String, Vec<f64>)> {
    if let Some((index, transform, output_name)) = resolve_raw_variable(plot, expression) {
        let values = plot.data.get(index)?;
        return Some((output_name, values.iter().copied().map(transform).collect()));
    }
    TestRunner::resolve_reference_series(expression, &|probe| {
        let (index, transform, _) = resolve_raw_variable(plot, probe)?;
        Some(
            plot.data
                .get(index)?
                .iter()
                .copied()
                .map(transform)
                .collect(),
        )
    })
    .map(|values| (expression.to_string(), values))
}

/// A resolved raw-plot column: its index, the scalar projection applied to it,
/// and the request spelling it answers.
type RawVariableBinding = (usize, fn(Complex64) -> f64, String);

fn resolve_raw_variable(plot: &RawReferencePlot, request: &str) -> Option<RawVariableBinding> {
    let requested = normalize(request);
    if let Some(index) = plot
        .variables
        .iter()
        .position(|candidate| normalize(candidate) == requested)
    {
        let transform =
            if plot.is_complex && (requested.starts_with("v(") || requested.starts_with("i(")) {
                magnitude
            } else {
                real
            };
        return Some((index, transform, request.to_string()));
    }
    // Control-language `save out` is ngspice shorthand for the node voltage
    // `v(out)`.  Preserve the canonical probe spelling in the oracle so the
    // RSpice result mapper can resolve it without guessing.
    if requested
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '#' | ':' | '+'))
    {
        let probe = format!("v({requested})");
        if let Some(index) = plot
            .variables
            .iter()
            .position(|candidate| normalize(candidate) == probe)
        {
            return Some((index, if plot.is_complex { magnitude } else { real }, probe));
        }
    }
    if let Some((_, raw_name)) = requested.split_once('.')
        && raw_name.ends_with("#branch")
        && let Some(index) = plot
            .variables
            .iter()
            .position(|candidate| normalize(candidate) == raw_name)
    {
        return Some((index, real, plot.variables[index].clone()));
    }
    let (function, inner) = requested.split_once('(')?;
    let inner = inner.strip_suffix(')')?;
    let prefix = function.chars().next()?;
    if !matches!(prefix, 'v' | 'i') {
        return None;
    }
    let base = format!("{prefix}({inner})");
    let index = plot
        .variables
        .iter()
        .position(|candidate| normalize(candidate) == base)
        .or_else(|| {
            (prefix == 'i').then(|| {
                let branch = format!("{inner}#branch");
                plot.variables
                    .iter()
                    .position(|candidate| normalize(candidate) == branch)
            })?
        })?;
    let transform = match function {
        "v" | "i" if plot.is_complex => magnitude,
        "v" | "i" | "vr" | "ir" => real,
        "vm" | "im" => magnitude,
        "vdb" | "idb" => decibels,
        "vp" | "ip" | "vph" | "iph" => phase,
        "vi" | "ii" => imaginary,
        _ => return None,
    };
    Some((index, transform, request.to_string()))
}

fn real(value: Complex64) -> f64 {
    value.re
}

fn imaginary(value: Complex64) -> f64 {
    value.im
}

fn magnitude(value: Complex64) -> f64 {
    value.norm()
}

fn decibels(value: Complex64) -> f64 {
    20.0 * value.norm().max(f64::MIN_POSITIVE).log10()
}

fn phase(value: Complex64) -> f64 {
    value.arg()
}

fn comparable_raw_variable(name: &str) -> bool {
    let name = normalize(name);
    (name.starts_with("v(") || name.starts_with("i(") || name.ends_with("#branch"))
        && !name.contains('@')
        && !name.contains("#ibranch_")
}

fn plot_axis(plot: &RawReferencePlot) -> String {
    let name = plot.plotname.to_ascii_lowercase();
    if name.contains("transient") {
        "time".to_string()
    } else if name.contains("dc transfer") {
        "v-sweep".to_string()
    } else if name.contains("ac analysis") || name.contains("noise") {
        "frequency".to_string()
    } else if name.contains("operating point") {
        "op".to_string()
    } else {
        plot.variables
            .first()
            .cloned()
            .unwrap_or_else(|| "index".to_string())
    }
}

fn sample_indices(length: usize, maximum: usize) -> Vec<usize> {
    if length <= maximum {
        return (0..length).collect();
    }
    let mut indices = BTreeSet::new();
    for sample in 0..maximum {
        indices.insert(sample * (length - 1) / (maximum - 1));
    }
    indices.into_iter().collect()
}

fn serialize_reference(version: &str, input_hash: &str, tables: &[ReferenceTable]) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "# RSPICE-NGSPICE-ORACLE {FORMAT_VERSION}");
    let _ = writeln!(output, "# ngspice: {version}");
    let _ = writeln!(output, "# input-blake3: {input_hash}");
    for table in tables {
        if table.x_name == "op" {
            let _ = writeln!(output, "Node Voltage");
            let _ = writeln!(output, "---- -------");
            for (name, series) in &table.variables {
                if let Some(value) = series.y.first() {
                    let _ = writeln!(output, "{name} {value:.17e}");
                }
            }
            let _ = writeln!(output);
            continue;
        }
        let variables = table.variables.keys().collect::<Vec<_>>();
        let _ = write!(output, "Index {}", table.x_name);
        for variable in &variables {
            let _ = write!(output, " {variable}");
        }
        let _ = writeln!(output);
        let rows = variables
            .iter()
            .filter_map(|variable| table.variables.get(*variable))
            .map(|series| series.x.len().min(series.y.len()))
            .min()
            .unwrap_or(0);
        for row in 0..rows {
            let x = table.variables[variables[0]].x[row];
            let _ = write!(output, "{row} {x:.17e}");
            for variable in &variables {
                let _ = write!(output, " {:.17e}", table.variables[*variable].y[row]);
            }
            let _ = writeln!(output);
        }
        let _ = writeln!(output);
    }
    output
}

fn normalize(value: &str) -> String {
    value
        .trim()
        .trim_matches(',')
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn tail(value: &str) -> String {
    const LIMIT: usize = 500;
    let value = value.trim();
    if value.len() <= LIMIT {
        value.to_string()
    } else {
        format!("...{}", &value[value.len() - LIMIT..])
    }
}

fn conventional_windows_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        let text = path.to_string_lossy();
        if let Some(rest) = text.strip_prefix(r"\\?\") {
            return PathBuf::from(rest);
        }
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn windows_capture_rejects_the_gui_ngspice_executable() {
        assert!(validate_headless_ngspice_executable(Path::new("ngspice_con.exe")).is_ok());
        let error = validate_headless_ngspice_executable(Path::new("ngspice.exe"))
            .expect_err("the GUI executable must fail closed");
        assert!(error.contains("may open the ngspice GUI"));
    }

    #[test]
    fn request_discovery_includes_print_plot_and_save_forms() {
        let requests = output_requests(
            ".print tran v(out)\n.plot ac vm(acout)\n.control\nsave v(saved)\nplot v(first) v(second)\nplot allv\n.endc\n",
        );
        assert!(requests.iter().any(|request| {
            request.axis.as_deref() == Some("time") && request.expression == "v(out)"
        }));
        assert!(
            requests
                .iter()
                .any(|request| request.expression == "v(saved)")
        );
        assert!(requests.iter().any(|request| {
            request.axis.as_deref() == Some("frequency") && request.expression == "vm(acout)"
        }));
        assert!(
            requests
                .iter()
                .any(|request| request.expression == "v(first)")
        );
        assert!(
            requests
                .iter()
                .any(|request| request.expression == "v(second)")
        );
        assert!(requests.iter().any(|request| request.expression == "allv"));
    }

    #[test]
    fn implicit_requests_are_bounded_node_voltages() {
        let requests = implicit_output_requests(
            "implicit outputs\nV1 input 0 1\nR1 input output 1k\nC1 output 0 1p\n.tran 1n 10n\n.end\n",
        );

        assert_eq!(requests.len(), 2);
        assert!(
            requests
                .iter()
                .any(|request| normalize(&request.expression) == "v(input)")
        );
        assert!(
            requests
                .iter()
                .any(|request| normalize(&request.expression) == "v(output)")
        );
    }

    #[test]
    fn pathological_iscas85_oracle_is_an_explicit_execution_only_case() {
        assert!(
            oracle_capture_exclusion(ExecutionCorpus::Iscas85, "85/c7552/c7552_ann_oc.net")
                .is_some()
        );
        assert!(
            oracle_capture_exclusion(ExecutionCorpus::Iscas85, "85/c7552/c7552_ann.net").is_none()
        );
    }

    #[test]
    fn sampling_preserves_both_endpoints() {
        let indices = sample_indices(10_000, 101);
        assert_eq!(indices.len(), 101);
        assert_eq!(indices.first(), Some(&0));
        assert_eq!(indices.last(), Some(&9_999));
    }

    #[test]
    fn reference_deck_promotes_analyses_and_drops_control_side_effects() {
        let source = reference_deck(
            "title\nV1 1 0 1\n.param stop = 10n\n.csparam stop='stop'\n.control\nlet step = stop/10\nlet waveform = v(1)\nalter V1 2\ntran $&step $&stop\ndestroy all\n.endc\n.end\n",
            &[],
        );
        assert!(!source.contains(".param step"));
        assert!(!source.contains(".param waveform"));
        assert!(source.contains(".tran {stop/10} stop"));
        assert!(source.contains(".save all"));
        assert!(source.contains("set num_threads=8"));
        assert!(source.contains("\nrun\n"));
        assert!(!source.contains(".param stop='stop'"));
        assert!(!source.contains("alter V1"));
        assert!(!source.contains("destroy all"));
        assert!(source.find(".tran").unwrap() < source.find(".end\n").unwrap());
    }

    #[test]
    fn reference_deck_inlines_each_control_scalar_reassignment() {
        let source = reference_deck(
            "title\nV1 1 0 1\n.control\nlet step = 1n\ntran $&step 10n\nlet step = 2n\ntran $&step 20n\n.endc\n.end\n",
            &[],
        );

        assert!(source.contains(".tran {1n} 10n"), "{source}");
        assert!(source.contains(".tran {2n} 20n"), "{source}");
        assert!(!source.contains(".param step"), "{source}");
    }

    #[test]
    fn reference_deck_deduplicates_matching_netlist_and_control_analyses() {
        let source = reference_deck(
            "title\nV1 out 0 1\n.tran 1n 10n\n.control\ntran 1n 10n\n.endc\n.end\n",
            &[],
        );

        assert_eq!(source.matches(".tran 1n 10n").count(), 1, "{source}");
    }

    #[test]
    fn reference_deck_saves_only_requested_raw_vectors_when_possible() {
        let requests = output_requests(".plot tran v(out) abs(i(vsense))\n");
        let source = reference_deck(
            "title\nV1 out 0 1\nVsense out load 0\nR1 load 0 1k\n.tran 1n 10n\n.end\n",
            &requests,
        );

        assert!(source.contains(".save v(out) i(vsense)"), "{source}");
        assert!(!source.contains(".save all"), "{source}");
    }

    #[test]
    fn reference_save_strips_plot_qualification_from_branch_vectors() {
        let requests = output_requests(".control\nplot dc1.vc1#branch dc2.vc1#branch\n.endc\n");
        let source = reference_deck("title\nV1 out 0 1\n.tran 1n 10n\n.end\n", &requests);

        assert!(source.contains(".save vc1#branch"), "{source}");
        assert!(!source.contains("v(dc1.vc1#branch)"), "{source}");
    }

    #[test]
    fn reference_tables_drop_non_finite_rows() {
        let plot = RawReferencePlot {
            plotname: "Transient Analysis".to_string(),
            variables: vec!["time".to_string(), "v(out)".to_string()],
            is_complex: false,
            data: vec![
                vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
                vec![Complex64::new(f64::NAN, 0.0), Complex64::new(2.0, 0.0)],
            ],
        };
        let requests = vec![OutputRequest {
            axis: Some("time".to_string()),
            expression: "v(out)".to_string(),
        }];

        let tables = reference_tables(&[plot], &requests);
        let series = &tables[0].variables["v(out)"];
        assert_eq!(series.x, vec![1.0]);
        assert_eq!(series.y, vec![2.0]);
    }
}
