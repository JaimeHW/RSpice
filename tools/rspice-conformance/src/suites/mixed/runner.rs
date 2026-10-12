//! `run`: every selected cell in a child process, compared and classified,
//! then held against (or recorded into) `outcomes.tsv`. Also `capture`, the
//! maintenance command that (re)captures checked-in oracle references.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::case::{CaseResult, Observation};
use super::classify::{CaseEnd, Classification, QuantityOutcome, QuantityVerdict, classify};
use super::compare::{Expected, Tolerance, Verdict, compare, lattice, ratio};
use super::design::{Design, Quantity, analysis_kind, discover, integrity};
use super::measure::{MeasureError, Measured, evaluate, statistic};
use super::outcomes::{self, Drift, Row};
use super::{Outcome, Route, analytic, events, oracle, probes};

/// What `run` was asked to do.
#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    /// Design ids (`C1`) or slugs; empty means every design and probe.
    pub designs: Vec<String>,
    pub variant: Option<String>,
    pub analysis: Option<String>,
    pub record: bool,
    pub extended: bool,
    /// An external producer replacing the `case` child (P4 routes).
    pub producer: Option<String>,
    /// This binary, for spawning `case` children.
    pub bin: PathBuf,
    /// Where children write their result files.
    pub work: PathBuf,
}

/// One classified cell.
#[derive(Debug, Clone)]
pub struct CellReport {
    pub design: String,
    pub row: Row,
    pub dispatch: String,
    pub elapsed_s: f64,
}

/// One design's run.
#[derive(Debug, Clone)]
pub struct DesignReport {
    pub design: String,
    pub cells: Vec<CellReport>,
    pub drift: Vec<Drift>,
    pub findings: Vec<String>,
}

pub fn selected_designs(designs: &[String]) -> Result<Vec<Design>, String> {
    let mut selected = Vec::new();
    for dir in discover(&super::corpus_root())? {
        let design = Design::load(&dir)?;
        if designs.is_empty()
            || designs.iter().any(|name| {
                name.eq_ignore_ascii_case(design.id()) || name.eq_ignore_ascii_case(&design.slug)
            })
        {
            selected.push(design);
        }
    }
    if selected.is_empty() && !designs.is_empty() {
        return Err(format!("no design matches {designs:?}"));
    }
    Ok(selected)
}

/// Run the selected cells and ratchet or record their outcomes.
pub fn run(options: &RunOptions) -> Result<Vec<DesignReport>, String> {
    fs::create_dir_all(&options.work).map_err(|error| error.to_string())?;
    let mut reports = Vec::new();
    for design in selected_designs(&options.designs)? {
        let mut findings = integrity(&design);
        findings.retain(|finding| !finding.contains("outcomes.tsv"));
        if !findings.is_empty() {
            reports.push(DesignReport {
                design: design.id().to_owned(),
                cells: Vec::new(),
                drift: Vec::new(),
                findings,
            });
            continue;
        }
        let mut cells = Vec::new();
        for (variant, analysis) in design.cells() {
            if options
                .variant
                .as_ref()
                .is_some_and(|wanted| *wanted != variant)
                || options
                    .analysis
                    .as_ref()
                    .is_some_and(|wanted| *wanted != analysis)
            {
                continue;
            }
            let entry = design.analysis(&analysis).expect("declared");
            if entry.extended && !options.extended {
                continue;
            }
            let started = Instant::now();
            let budget = Duration::from_secs(entry.timeout_s);
            let end = match missing_feature(&design) {
                Some(feature) => CaseEnd::Completed(Box::new(
                    CaseResult::new(Route::compiled().token(), "none").failed(
                        super::case::ErrorRecord {
                            category: "harness".into(),
                            code: Some("no-feature".into()),
                            detail: format!("this build lacks rspice-core feature {feature}"),
                            display: format!("no-feature: {feature}"),
                            ..Default::default()
                        },
                    ),
                )),
                None => run_child(options, &design, &variant, &analysis, false, budget),
            };
            let (classification, dispatch, route) =
                evaluate_cell(options, &design, &variant, &analysis, &end, budget);
            eprintln!(
                "mixed-corpus: {} {variant} {analysis} {route}: {} — {} ({:.1} s, {dispatch})",
                design.id(),
                classification.outcome,
                classification.detail,
                started.elapsed().as_secs_f64()
            );
            cells.push(CellReport {
                design: design.id().to_owned(),
                row: Row {
                    variant,
                    analysis,
                    route,
                    outcome: classification.outcome,
                    detail: classification.detail,
                },
                dispatch,
                elapsed_s: started.elapsed().as_secs_f64(),
            });
        }
        let path = design.dir.join("outcomes.tsv");
        let recorded = match fs::read_to_string(&path) {
            Ok(text) => outcomes::parse(&text)?,
            Err(_) => Vec::new(),
        };
        let observed: Vec<Row> = cells.iter().map(|cell| cell.row.clone()).collect();
        let drift = if options.record {
            let merged = outcomes::record(&recorded, &observed, &design.cells());
            fs::write(&path, outcomes::render(&merged)).map_err(|error| error.to_string())?;
            Vec::new()
        } else {
            outcomes::ratchet(&recorded, &observed)
        };
        reports.push(DesignReport {
            design: design.id().to_owned(),
            cells,
            drift,
            findings: Vec::new(),
        });
    }
    Ok(reports)
}

/// The first `design.toml` feature this build did not compile, if any. Each
/// is checked with `cfg!` on the forwarded feature; an unknown name counts as
/// missing, so a typo cannot pass as present.
fn missing_feature(design: &Design) -> Option<String> {
    const COMPILED: [(&str, bool); 3] = [
        (
            "veriloga-model-juncap200",
            cfg!(feature = "veriloga-model-juncap200"),
        ),
        (
            "veriloga-model-diode-cmc",
            cfg!(feature = "veriloga-model-diode-cmc"),
        ),
        (
            "veriloga-builtins-noise",
            cfg!(feature = "veriloga-builtins-noise"),
        ),
    ];
    design
        .file
        .features
        .iter()
        .find(|feature| {
            !COMPILED
                .iter()
                .any(|(name, compiled)| *compiled && name == feature)
        })
        .cloned()
}

/// Spawn the case child (or the external producer) and wait for its result.
fn run_child(
    options: &RunOptions,
    design: &Design,
    variant: &str,
    analysis: &str,
    o5: bool,
    budget: Duration,
) -> CaseEnd {
    let out = options.work.join(format!(
        "{}-{variant}-{analysis}{}.json",
        design.slug,
        if o5 { "-o5" } else { "" }
    ));
    let _ = fs::remove_file(&out);
    let mut command = match (&options.producer, o5) {
        (Some(producer), false) => {
            let mut words = producer.split_whitespace();
            let Some(program) = words.next() else {
                return CaseEnd::Crash("empty producer command".into());
            };
            let mut command = Command::new(program);
            command.args(words);
            command
        }
        _ => {
            let mut command = Command::new(&options.bin);
            command.arg("case");
            command
        }
    };
    command
        .arg(&design.dir)
        .arg(variant)
        .arg(analysis)
        .arg(&out)
        .arg("--budget-s")
        .arg(budget.as_secs().to_string());
    if o5 {
        command.arg("--o5");
    }
    let mut child = match command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => return CaseEnd::Crash(format!("spawn: {error}")),
    };
    let stderr = child.stderr.take();
    let reader = std::thread::spawn(move || {
        let mut text = String::new();
        if let Some(mut stderr) = stderr {
            let _ = std::io::Read::read_to_string(&mut stderr, &mut text);
        }
        text
    });
    let hard = budget + Duration::from_secs(60);
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if started.elapsed() > hard => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(100)),
            Err(_) => break None,
        }
    };
    let stderr = reader.join().unwrap_or_default();
    let Some(status) = status else {
        return CaseEnd::Timeout(hard.as_secs_f64());
    };
    match CaseResult::read(&out) {
        Ok(result) => {
            if result
                .error
                .as_ref()
                .is_some_and(|error| error.category == "timeout")
            {
                return CaseEnd::Timeout(budget.as_secs_f64());
            }
            CaseEnd::Completed(Box::new(result))
        }
        Err(error) => {
            let tail: String = stderr.lines().rev().take(6).collect::<Vec<_>>().join(" | ");
            CaseEnd::Crash(format!("{status}; {error}; stderr: {tail}"))
        }
    }
}

/// The observation a cell's quantities read: the result of its own card.
pub fn primary_observation<'a>(result: &'a CaseResult, analysis: &str) -> Option<&'a Observation> {
    let kind = analysis_kind(analysis).unwrap_or(analysis);
    let wanted = match kind {
        "tran" | "trnoise" | "pss" => "tran",
        "mc-tran" => "mc",
        "step-tran" => "step",
        "pnoise" => "noise",
        "pac" => "ac",
        other => other,
    };
    result
        .results
        .iter()
        .rev()
        .find(|observation| observation.kind == wanted)
        .or_else(|| result.results.last())
}

fn evaluate_cell(
    options: &RunOptions,
    design: &Design,
    variant: &str,
    analysis: &str,
    end: &CaseEnd,
    budget: Duration,
) -> (Classification, String, Route) {
    let kind = analysis_kind(analysis).unwrap_or(analysis);
    let instances = deck_instances(&design.deck_path(variant, analysis));
    let (dispatch, route) = match end {
        CaseEnd::Completed(result) => (
            result.dispatch.clone(),
            Route::from_token(&result.route).unwrap_or(Route::compiled()),
        ),
        _ => ("none".to_owned(), Route::compiled()),
    };
    let mut verdicts = Vec::new();
    if let CaseEnd::Completed(result) = end
        && result.outcome == "completed"
        && let Some(observation) = primary_observation(result, analysis)
    {
        let mut o5_cache: Option<Result<Observation, String>> = None;
        for quantity in design.quantities_for(variant, analysis) {
            let outcome = evaluate_quantity(
                options,
                design,
                quantity,
                variant,
                analysis,
                observation,
                budget,
                &mut o5_cache,
            );
            verdicts.push(QuantityVerdict {
                quantity: quantity.id.clone(),
                oracle: quantity.oracle.clone(),
                outcome,
            });
        }
    }
    (classify(end, kind, &instances, &verdicts), dispatch, route)
}

/// Element names (lower case) of a deck and everything it includes.
fn deck_instances(deck: &Path) -> Vec<String> {
    let Ok(source) = fs::read_to_string(deck) else {
        return Vec::new();
    };
    let directory = deck.parent().unwrap_or(Path::new("."));
    let expanded = rspice_core::netlist::IncludeProcessor::new(directory)
        .expand_content(&source, deck)
        .unwrap_or(source);
    let mut names = BTreeSet::new();
    for line in expanded.lines().skip(1) {
        let trimmed = line.trim();
        let Some(first) = trimmed.chars().next() else {
            continue;
        };
        if matches!(first, '.' | '*' | '+' | '$' | ';') {
            continue;
        }
        if let Some(name) = trimmed.split_whitespace().next() {
            names.insert(name.to_ascii_lowercase());
        }
    }
    names.into_iter().collect()
}

#[allow(clippy::too_many_arguments)]
fn evaluate_quantity(
    options: &RunOptions,
    design: &Design,
    quantity: &Quantity,
    variant: &str,
    analysis: &str,
    rspice: &Observation,
    budget: Duration,
    o5_cache: &mut Option<Result<Observation, String>>,
) -> QuantityOutcome {
    let tolerance = Tolerance::of(quantity);
    let kind = quantity.measure.kind.as_str();
    // Probe assertions read the observation directly.
    if kind == "assert" {
        return match probes::assert(quantity, rspice) {
            Ok(verdict) => QuantityOutcome::Compared(verdict),
            Err(error) if error.starts_with("absent") => QuantityOutcome::Missing(error),
            Err(error) => QuantityOutcome::Unmeasurable(error),
        };
    }
    // A captured reference names signals as its own deck does: one variant's
    // reference may serve another (C1's `xs` O1 tables serve `vams`), so the
    // oracle side is measured with the reference's variant spellings.
    let oracle_variant = quantity
        .reference
        .as_ref()
        .and_then(|reference| reference.for_variant(variant))
        .and_then(|reference| {
            let name = Path::new(&reference).file_name()?.to_str()?.to_owned();
            let token = name.split('.').next()?.to_owned();
            design
                .file
                .variants
                .iter()
                .any(|declared| declared.name == token)
                .then_some(token)
        })
        .unwrap_or_else(|| variant.to_owned());
    // The oracle's observation, when the oracle is a producer.
    let oracle_observation = match quantity.oracle.as_str() {
        "O6" | "O7" => None,
        "O5" => {
            let cached = o5_cache.get_or_insert_with(|| {
                match run_child(options, design, variant, analysis, true, budget) {
                    CaseEnd::Completed(result) if result.outcome == "completed" => {
                        primary_observation(&result, analysis)
                            .cloned()
                            .ok_or_else(|| "O5 run produced no observation".to_owned())
                    }
                    CaseEnd::Completed(result) => Err(format!(
                        "O5 run failed: {}",
                        result.error.map(|error| error.display).unwrap_or_default()
                    )),
                    CaseEnd::Crash(why) => Err(format!("O5 run crashed: {why}")),
                    CaseEnd::Timeout(seconds) => Err(format!("O5 run timed out after {seconds} s")),
                }
            });
            match cached {
                Ok(observation) => Some(observation.clone()),
                Err(error) => return QuantityOutcome::OracleError(error.clone()),
            }
        }
        "O3" => match o3_observation(design, quantity, variant, rspice) {
            Ok(observation) => Some(observation),
            Err(outcome) => return outcome,
        },
        _ => {
            let Some(reference) = quantity
                .reference
                .as_ref()
                .and_then(|reference| reference.for_variant(variant))
            else {
                return QuantityOutcome::OracleError("no reference named".into());
            };
            match oracle::load_reference(&design.dir, &reference) {
                Ok(Some(oracle::LoadedReference::Observed(observation))) => Some(*observation),
                Ok(Some(oracle::LoadedReference::Refused(reason))) => {
                    return QuantityOutcome::OracleAbsent(format!(
                        "{reference}: oracle refused: {reason}"
                    ));
                }
                Ok(None) => {
                    let why = format!("{reference} not captured");
                    return if quantity.oracle == "O2" && !oracle::oracles_required() {
                        QuantityOutcome::OracleAbsent(why)
                    } else if quantity.oracle == "O2" {
                        QuantityOutcome::OracleError(why)
                    } else {
                        QuantityOutcome::OracleAbsent(why)
                    };
                }
                Err(error) => return QuantityOutcome::OracleError(error),
            }
        }
    };
    match kind {
        "stat" => stat_quantity(
            quantity,
            variant,
            &oracle_variant,
            rspice,
            oracle_observation.as_ref(),
        ),
        "per_trial" => per_trial_quantity(quantity, variant, rspice, tolerance),
        _ => {
            let actual = match evaluate(&quantity.measure, variant, rspice) {
                Ok(actual) => actual,
                Err(error) => return measure_failure(error),
            };
            if let (Some(lattice_spec), Measured::Events(traces)) =
                (&quantity.measure.lattice, &actual)
            {
                let verdict = lattice(
                    traces,
                    lattice_spec.t0,
                    lattice_spec.period,
                    tolerance.edge_abs,
                );
                if !verdict.pass || oracle_observation.is_none() {
                    return QuantityOutcome::Compared(verdict);
                }
            }
            let expected = match &oracle_observation {
                Some(observation) => {
                    match evaluate(&quantity.measure, &oracle_variant, observation) {
                        Ok(measured) => Expected::Measured(measured),
                        Err(error) => {
                            return QuantityOutcome::OracleError(format!("oracle side: {error}"));
                        }
                    }
                }
                None if quantity.oracle == "O7" => Expected::Structural,
                None => match analytic_expectation(quantity, variant, rspice, None) {
                    Ok(expected) => expected,
                    Err(error) => return QuantityOutcome::OracleError(error),
                },
            };
            QuantityOutcome::Compared(compare(&actual, &expected, tolerance))
        }
    }
}

fn measure_failure(error: MeasureError) -> QuantityOutcome {
    match error {
        MeasureError::Missing(what) => QuantityOutcome::Missing(what),
        MeasureError::Invalid(what) => QuantityOutcome::Unmeasurable(what),
    }
}

fn analytic_expectation(
    quantity: &Quantity,
    variant: &str,
    rspice: &Observation,
    trial: Option<usize>,
) -> Result<Expected, String> {
    let name = quantity
        .expect
        .as_deref()
        .ok_or("O6 quantity without expect")?;
    let function = analytic::lookup(name).ok_or_else(|| format!("{name} is not registered"))?;
    function(&analytic::Context {
        quantity,
        variant,
        rspice,
        trial,
    })
}

/// A statistic over samples of the inner measure, or over trials.
fn stat_quantity(
    quantity: &Quantity,
    rspice_variant: &str,
    oracle_variant: &str,
    rspice: &Observation,
    oracle_observation: Option<&Observation>,
) -> QuantityOutcome {
    let Some(inner) = &quantity.of else {
        return QuantityOutcome::OracleError("stat without of".into());
    };
    let name = quantity.stat.as_deref().unwrap_or("mean");
    let over_trials = quantity.over.as_deref() == Some("trials");
    let sample = |observation: &Observation, variant: &str| -> Result<Vec<f64>, MeasureError> {
        if over_trials {
            observation
                .trials
                .iter()
                .map(|trial| match evaluate(inner, variant, trial)? {
                    Measured::Values(values) => values
                        .first()
                        .copied()
                        .ok_or(MeasureError::Invalid("empty trial value".into())),
                    _ => Err(MeasureError::Invalid("trial measure is not scalar".into())),
                })
                .collect()
        } else {
            match evaluate(inner, variant, observation)? {
                Measured::Values(values) => Ok(values),
                _ => Err(MeasureError::Invalid("stat needs scalar samples".into())),
            }
        }
    };
    let samples = match sample(rspice, rspice_variant) {
        Ok(samples) => samples,
        Err(error) => return measure_failure(error),
    };
    if let Some(expected) = quantity.trials
        && over_trials
        && samples.len() != expected
    {
        return QuantityOutcome::Compared(Verdict {
            pass: false,
            severity: f64::INFINITY,
            detail: format!("{} trials, expected {expected}", samples.len()),
        });
    }
    let value = match statistic(name, &samples) {
        Ok(value) => value,
        Err(error) => return measure_failure(error),
    };
    if let Some(interval) = quantity.interval {
        let mut verdict = compare(
            &Measured::Values(vec![value]),
            &Expected::Interval(interval),
            Tolerance::default(),
        );
        verdict.detail = format!("{name} over {} samples: {}", samples.len(), verdict.detail);
        return QuantityOutcome::Compared(verdict);
    }
    let (Some(bounds), Some(observation)) = (quantity.ratio, oracle_observation) else {
        return QuantityOutcome::OracleError(
            "stat needs an interval, or a ratio and an oracle".into(),
        );
    };
    let expected = match sample(observation, oracle_variant)
        .map_err(|error| error.to_string())
        .and_then(|samples| statistic(name, &samples).map_err(|error| error.to_string()))
    {
        Ok(expected) => expected,
        Err(error) => return QuantityOutcome::OracleError(format!("oracle side: {error}")),
    };
    QuantityOutcome::Compared(ratio(value, expected, bounds))
}

/// The inner measure per trial against the O6 expectation for that trial.
fn per_trial_quantity(
    quantity: &Quantity,
    variant: &str,
    rspice: &Observation,
    tolerance: Tolerance,
) -> QuantityOutcome {
    let Some(inner) = &quantity.of else {
        return QuantityOutcome::OracleError("per_trial without of".into());
    };
    if let Some(expected) = quantity.trials
        && rspice.trials.len() != expected
    {
        return QuantityOutcome::Compared(Verdict {
            pass: false,
            severity: f64::INFINITY,
            detail: format!("{} trials, expected {expected}", rspice.trials.len()),
        });
    }
    if rspice.trials.is_empty() {
        return QuantityOutcome::Missing("no trials".into());
    }
    let mut worst: Option<Verdict> = None;
    for (index, trial) in rspice.trials.iter().enumerate() {
        let actual = match evaluate(inner, variant, trial) {
            Ok(actual) => actual,
            Err(error) => return measure_failure(error),
        };
        let expected = match analytic_expectation(quantity, variant, trial, Some(index)) {
            Ok(expected) => expected,
            Err(error) => return QuantityOutcome::OracleError(error),
        };
        let mut verdict = compare(&actual, &expected, tolerance);
        verdict.detail = format!("trial {index}: {}", verdict.detail);
        if worst
            .as_ref()
            .is_none_or(|worst| verdict.severity > worst.severity)
        {
            worst = Some(verdict);
        }
    }
    QuantityOutcome::Compared(worst.expect("at least one trial"))
}

/// O3: replay RSpice's boundary inputs into Icarus and read the outputs back.
fn o3_observation(
    design: &Design,
    quantity: &Quantity,
    variant: &str,
    rspice: &Observation,
) -> Result<Observation, QuantityOutcome> {
    let tools = match oracle::icarus::discover() {
        Ok(tools) => tools,
        Err(absent) if oracle::oracles_required() => {
            return Err(QuantityOutcome::OracleError(absent.to_string()));
        }
        Err(absent) => return Err(QuantityOutcome::OracleAbsent(absent.to_string())),
    };
    let template_dir = design.dir.join("oracle").join(format!("{variant}.o3"));
    let template = fs::read_to_string(template_dir.join("tb.v"))
        .map_err(|error| QuantityOutcome::OracleError(format!("O3 template: {error}")))?;
    let inputs: Vec<String> = oracle::icarus::template_directive(&template, "rspice-o3-inputs")
        .unwrap_or_default()
        .split_whitespace()
        .map(str::to_ascii_lowercase)
        .collect();
    let sources: Vec<String> = oracle::icarus::template_directive(&template, "rspice-o3-sources")
        .unwrap_or_default()
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let tick = oracle::icarus::template_directive(&template, "rspice-o3-timescale")
        .and_then(|scale| match scale.as_str() {
            "1ps" => Some(1e-12),
            "1ns" => Some(1e-9),
            "1fs" => Some(1e-15),
            _ => None,
        })
        .unwrap_or(1e-12);
    let mut replay = BTreeMap::new();
    for net in &inputs {
        let trace = rspice.events.get(net).ok_or_else(|| {
            QuantityOutcome::Missing(format!("O3 input net {net} absent from the RSpice trace"))
        })?;
        replay.insert(net.clone(), trace.clone());
    }
    let testbench = oracle::icarus::render_testbench(&template, &replay, tick)
        .map_err(QuantityOutcome::OracleError)?;
    let vcd = oracle::icarus::run(&tools, &template_dir, &testbench, &sources, 600_000)
        .map_err(QuantityOutcome::OracleError)?;
    let traces = events::parse_vcd(&vcd).map_err(QuantityOutcome::OracleError)?;
    let _ = quantity;
    Ok(Observation {
        kind: "tran".into(),
        events: traces,
        ..Default::default()
    })
}

// ---------------------------------------------------------------------------
// capture
// ---------------------------------------------------------------------------

/// What `capture` did for one reference.
#[derive(Debug, Clone)]
pub struct CaptureReport {
    pub reference: String,
    pub status: String,
}

/// (Re)capture every reference of `oracle` in the selected designs. Without
/// `update`, a reference must reproduce byte for byte.
pub fn capture(
    designs: &[String],
    oracle_id: &str,
    update: bool,
) -> Result<Vec<CaptureReport>, String> {
    let mut reports = Vec::new();
    for design in selected_designs(designs)? {
        let mut references: BTreeSet<String> = BTreeSet::new();
        for quantity in &design.tolerance.quantities {
            if quantity.oracle != oracle_id {
                continue;
            }
            if let Some(reference) = &quantity.reference {
                for variant in &quantity.variants {
                    if let Some(path) = reference.for_variant(variant) {
                        references.insert(path);
                    }
                }
            }
        }
        for reference in references {
            let status = match capture_one(&design, &reference, oracle_id, update) {
                Ok(status) => status,
                Err(error) => format!("FAILED: {error}"),
            };
            reports.push(CaptureReport {
                reference: format!("{}/{reference}", design.slug),
                status,
            });
        }
    }
    Ok(reports)
}

fn capture_one(
    design: &Design,
    reference: &str,
    oracle_id: &str,
    update: bool,
) -> Result<String, String> {
    let path = design.dir.join(reference);
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("bad reference path")?;
    let deck = oracle::deck_for_reference(&design.dir, name)?;
    let stem = name.trim_end_matches(".out");
    let mut parts = stem.splitn(3, '.');
    let _variant = parts.next();
    let analysis = parts
        .next()
        .ok_or("reference name lacks an analysis")?
        .to_owned();
    let timeout = Duration::from_secs(
        design
            .analysis(&analysis)
            .map(|entry| entry.timeout_s.max(600))
            .unwrap_or(1800),
    );
    let fingerprint = oracle::deck_fingerprint(&deck)?;
    let mut refused = None;
    let (producer, executable, version, sections, vcd) = match oracle_id {
        "O1" | "O5-O1" => {
            let executable = oracle::ngspice::discover().map_err(|absent| absent.to_string())?;
            let version = oracle::ngspice::version(&executable)?;
            let run = oracle::ngspice::run(&executable, &deck, timeout)?;
            refused = oracle::ngspice::refusal(&analysis, &run.log);
            let sections = if refused.is_some() {
                BTreeMap::new()
            } else {
                oracle::ngspice::sections(&analysis, &run.plots)?
            };
            (
                oracle::Producer::Ngspice,
                executable,
                version,
                sections,
                run.vcd,
            )
        }
        "O4" | "O5-O4" => {
            let executable = oracle::xyce::discover().map_err(|absent| absent.to_string())?;
            let version = oracle::xyce::version(&executable).unwrap_or_else(|_| "Xyce 7.10".into());
            let prn = oracle::xyce::run(&executable, &deck, timeout)?;
            let deck_source = fs::read_to_string(&deck).map_err(|error| error.to_string())?;
            let sections = oracle::xyce::sections(&analysis, &prn, &deck_source)?;
            (oracle::Producer::Xyce, executable, version, sections, None)
        }
        "O2" => {
            let tools = oracle::cosim::discover().map_err(|absent| absent.to_string())?;
            let version = oracle::ngspice::version(&tools.ngspice)?;
            let run = oracle::cosim::run(&tools, &deck, timeout)?;
            let sections = oracle::ngspice::sections(&analysis, &run.plots)?;
            (
                oracle::Producer::Cosim,
                tools.ngspice.clone(),
                version,
                sections,
                run.vcd,
            )
        }
        other => return Err(format!("oracle {other} is not captured")),
    };
    let mut header = vec![
        format!(
            "{}: {version}",
            match producer {
                oracle::Producer::Xyce => "xyce",
                _ => "ngspice",
            }
        ),
        format!(
            "executable-sha256: {}",
            oracle::executable_sha256(&executable)?
        ),
        oracle::binding_line(&fingerprint)
            .trim_start_matches("# ")
            .to_owned(),
    ];
    match &refused {
        Some(reason) => header.push(format!("{}{reason}", oracle::REFUSED_PREFIX)),
        None => oracle::validate(&sections)?,
    }
    let text = oracle::render(producer, &header, &sections);
    let events_path = path.with_file_name(name.replace(".out", ".events.tsv"));
    let events_text = vcd
        .map(|vcd| -> Result<String, String> {
            let traces = events::parse_vcd(&vcd)?;
            Ok(events::write_events_tsv(&header, &traces))
        })
        .transpose()?;
    // A deck whose oracle cannot reproduce itself says so; its tables are then
    // verified by their header (same executable, same deck) and section
    // layout, not byte for byte.
    let stochastic = fs::read_to_string(&deck)
        .ok()
        .and_then(|source| oracle::ngspice::directive(&source, "rspice-stochastic"));
    let mut changed = Vec::new();
    for (target, content) in [(path.clone(), Some(text)), (events_path, events_text)] {
        let Some(content) = content else { continue };
        let existing = fs::read_to_string(&target).ok();
        if existing.as_deref() == Some(content.as_str()) {
            continue;
        }
        if !update
            && stochastic.is_some()
            && existing
                .as_deref()
                .is_some_and(|existing| same_layout(existing, &content))
        {
            continue;
        }
        if update {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            fs::write(&target, content).map_err(|error| error.to_string())?;
        }
        changed.push(
            target
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_owned(),
        );
    }
    Ok(match (changed.is_empty(), update, &stochastic) {
        (true, false, Some(why)) => {
            format!("same header and layout; values not byte-compared (stochastic: {why})")
        }
        (true, _, _) => "identical".into(),
        (false, true, _) => format!("written {}", changed.join(", ")),
        (false, false, _) => format!("DIFFERS {} (re-run with --update)", changed.join(", ")),
    })
}

/// Header lines, section labels, column headers and row counts agree.
fn same_layout(existing: &str, regenerated: &str) -> bool {
    let shape = |text: &str| -> Vec<String> {
        let mut shape = Vec::new();
        let mut rows = 0usize;
        for line in text.lines() {
            if line.starts_with('#') || line.starts_with("Index") {
                if rows > 0 {
                    shape.push(format!("rows {rows}"));
                    rows = 0;
                }
                shape.push(line.to_owned());
            } else {
                rows += 1;
            }
        }
        shape.push(format!("rows {rows}"));
        shape
    };
    shape(existing) == shape(regenerated)
}

/// A compact per-route count for the plan ledger (`interp 9P 4R 1W 2U`).
pub fn ledger_counts(rows: &[Row]) -> String {
    let mut counts: BTreeMap<Route, HashMap<Outcome, usize>> = BTreeMap::new();
    for row in rows {
        *counts
            .entry(row.route)
            .or_default()
            .entry(row.outcome)
            .or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(route, counts)| {
            let mut text = route.token().to_owned();
            for (outcome, letter) in [
                (Outcome::Pass, 'P'),
                (Outcome::Refused, 'R'),
                (Outcome::Fail, 'F'),
                (Outcome::Wrong, 'W'),
                (Outcome::Unverified, 'U'),
            ] {
                if let Some(count) = counts.get(&outcome) {
                    text.push_str(&format!(" {count}{letter}"));
                }
            }
            text
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_design_in_the_corpus_passes_its_integrity_checks() {
        let designs = selected_designs(&[]).expect("corpus loads");
        let mut findings = Vec::new();
        for design in &designs {
            findings.extend(integrity(design));
            let path = design.dir.join("outcomes.tsv");
            match fs::read_to_string(&path) {
                Ok(text) => match outcomes::parse(&text) {
                    Ok(rows) => findings.extend(
                        outcomes::coverage(&rows, &design.cells())
                            .into_iter()
                            .map(|finding| format!("{}: {finding}", design.slug)),
                    ),
                    Err(error) => findings.push(format!("{}: {error}", design.slug)),
                },
                Err(_) => findings.push(format!("{}: outcomes.tsv is missing", design.slug)),
            }
        }
        assert!(
            findings.is_empty(),
            "corpus integrity:\n{}",
            findings.join("\n")
        );
    }

    #[test]
    fn ledger_counts_group_by_route() {
        let row = |route, outcome| Row {
            variant: "v".into(),
            analysis: "a".into(),
            route,
            outcome,
            detail: String::new(),
        };
        let rows = vec![
            row(Route::Interp, Outcome::Pass),
            row(Route::Interp, Outcome::Pass),
            row(Route::Interp, Outcome::Refused),
            row(Route::Native, Outcome::Wrong),
        ];
        assert_eq!(ledger_counts(&rows), "interp 2P 1R; native 1W");
    }
}
