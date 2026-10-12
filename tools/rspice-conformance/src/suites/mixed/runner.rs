//! `run`: every selected cell in a child process, compared and classified,
//! then held against (or recorded into) `outcomes.tsv`. Also `capture`, the
//! maintenance command that (re)captures checked-in oracle references.
//!
//! Cells run concurrently, at most `--jobs` children at a time, slowest
//! first. Reports and records are assembled in declared order whatever the
//! completion order, so `outcomes.tsv` and the summary do not depend on N.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
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
    /// The most cells running at once; 0 means [`default_jobs`].
    pub jobs: usize,
    /// Where each cell's last wall time on this machine is kept, to start the
    /// slowest cells first. A scheduling hint only; `None` keeps none.
    pub timings: Option<PathBuf>,
}

impl RunOptions {
    pub fn jobs(&self) -> usize {
        if self.jobs == 0 {
            default_jobs()
        } else {
            self.jobs
        }
    }
}

/// The default `--jobs`: a quarter of the hardware threads, between 2 and 8.
///
/// Measured on the 28-thread, 31.7 GB development box (2026-10-12): a Monte
/// Carlo cell runs its trials in parallel, about one core per trial (C4 ports
/// mc-tran 9.8 cores and 0.50 GB, C2 xs mc-tran 8.0 cores and 0.64 GB); every
/// other cell is one core and at most 72 MB. Slowest first starts the two long
/// MC cells together, so N children want about 2 × 9 + (N − 2) cores: 23 of
/// 28 at N = 7, leaving room for the other work on the box, in under 2 GB.
pub fn default_jobs() -> usize {
    let threads = std::thread::available_parallelism().map_or(1, usize::from);
    (threads / 4).clamp(2, 8)
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

/// One selected cell: its design (an index into the selection) and budget.
#[derive(Debug, Clone)]
struct Task {
    design: usize,
    variant: String,
    analysis: String,
    timeout_s: u64,
}

/// Run the selected cells and ratchet or record their outcomes.
///
/// Every selected cell of every selected design goes on one queue that
/// [`RunOptions::jobs`] workers drain, each running one cell (its `case`
/// child, then any O5 child or O3 replay) at a time. A design is ratcheted or
/// recorded as soon as its last cell finishes, from its cells in declared
/// order.
pub fn run(options: &RunOptions) -> Result<Vec<DesignReport>, String> {
    fs::create_dir_all(&options.work).map_err(|error| error.to_string())?;
    let designs = selected_designs(&options.designs)?;
    let mut reports: Vec<Option<DesignReport>> = Vec::with_capacity(designs.len());
    let mut tasks = Vec::new();
    let mut pending = vec![0usize; designs.len()];
    for (index, design) in designs.iter().enumerate() {
        let mut findings = integrity(design);
        findings.retain(|finding| !finding.contains("outcomes.tsv"));
        if !findings.is_empty() {
            reports.push(Some(DesignReport {
                design: design.id().to_owned(),
                cells: Vec::new(),
                drift: Vec::new(),
                findings,
            }));
            continue;
        }
        reports.push(None);
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
            pending[index] += 1;
            tasks.push(Task {
                design: index,
                variant,
                analysis,
                timeout_s: entry.timeout_s,
            });
        }
    }
    // A design none of whose cells is selected is held (or re-rendered) now.
    for (index, design) in designs.iter().enumerate() {
        if reports[index].is_none() && pending[index] == 0 {
            reports[index] = Some(finish(options, design, Vec::new())?);
        }
    }
    let previous = options
        .timings
        .as_deref()
        .map(load_timings)
        .unwrap_or_default();
    let key = |task: &Task| {
        (
            designs[task.design].slug.clone(),
            task.variant.clone(),
            task.analysis.clone(),
        )
    };
    let order = slowest_first(&tasks, |task| previous.get(&key(task)).copied());
    let mut assembly = Assembly {
        pending,
        finished: vec![Vec::new(); designs.len()],
    };
    let mut measured = Vec::new();
    let outcome = execute(
        &tasks,
        &order,
        options.jobs(),
        |task| run_cell(options, &designs[task.design], task),
        |index, cell| {
            let design = tasks[index].design;
            measured.push((key(&tasks[index]), cell.elapsed_s));
            if let Some(cells) = assembly.complete(design, index, cell) {
                reports[design] = Some(finish(options, &designs[design], cells)?);
            }
            Ok(())
        },
    );
    if let Some(path) = &options.timings {
        store_timings(path, previous, measured);
    }
    outcome?;
    Ok(reports
        .into_iter()
        .map(|report| report.expect("every design finished"))
        .collect())
}

/// Finished cells, held per design until its last one is in.
struct Assembly {
    /// Cells still running or queued, per design.
    pending: Vec<usize>,
    /// Finished cells with their task index, in completion order.
    finished: Vec<Vec<(usize, CellReport)>>,
}

impl Assembly {
    /// Take one finished cell; when it is its design's last, release all of
    /// that design's cells in declared (task) order.
    fn complete(
        &mut self,
        design: usize,
        index: usize,
        cell: CellReport,
    ) -> Option<Vec<CellReport>> {
        self.finished[design].push((index, cell));
        self.pending[design] -= 1;
        if self.pending[design] > 0 {
            return None;
        }
        let mut cells = std::mem::take(&mut self.finished[design]);
        cells.sort_by_key(|(index, _)| *index);
        Some(cells.into_iter().map(|(_, cell)| cell).collect())
    }
}

/// Ratchet `cells` against the design's `outcomes.tsv`, or record them into
/// it under the table's lock.
fn finish(
    options: &RunOptions,
    design: &Design,
    cells: Vec<CellReport>,
) -> Result<DesignReport, String> {
    let path = design.dir.join("outcomes.tsv");
    let observed: Vec<Row> = cells.iter().map(|cell| cell.row.clone()).collect();
    let drift = if options.record {
        outcomes::record_locked(&path, &observed, &design.cells())?;
        Vec::new()
    } else {
        outcomes::ratchet(&outcomes::read_locked(&path)?, &observed)
    };
    Ok(DesignReport {
        design: design.id().to_owned(),
        cells,
        drift,
        findings: Vec::new(),
    })
}

/// Run one cell, classify it, and print its line as soon as it is done.
fn run_cell(options: &RunOptions, design: &Design, task: &Task) -> CellReport {
    let (variant, analysis) = (task.variant.as_str(), task.analysis.as_str());
    let started = Instant::now();
    let budget = Duration::from_secs(task.timeout_s);
    let end = match missing_feature(design) {
        Some(feature) => CaseEnd::Completed(Box::new(
            CaseResult::new(Route::compiled().token(), "none").failed(super::case::ErrorRecord {
                category: "harness".into(),
                code: Some("no-feature".into()),
                detail: format!("this build lacks rspice-core feature {feature}"),
                display: format!("no-feature: {feature}"),
                ..Default::default()
            }),
        )),
        None => run_child(options, design, variant, analysis, false, budget),
    };
    let (classification, dispatch, route) =
        evaluate_cell(options, design, variant, analysis, &end, budget);
    let detail = outcomes::normalize_paths(
        &classification.detail,
        &super::corpus_root().display().to_string(),
        &design.dir.display().to_string(),
    );
    let elapsed_s = started.elapsed().as_secs_f64();
    eprintln!(
        "mixed-corpus: {} {variant} {analysis} {route}: {} — {detail} ({elapsed_s:.1} s, {dispatch})",
        design.id(),
        classification.outcome,
    );
    CellReport {
        design: design.id().to_owned(),
        row: Row {
            variant: variant.to_owned(),
            analysis: analysis.to_owned(),
            route,
            outcome: classification.outcome,
            detail,
        },
        dispatch,
        elapsed_s,
    }
}

/// Run `work` on every task, `jobs` at a time, taking tasks in `order`, and
/// hand each result to `done` on this thread as it completes. An error from
/// `done` stops workers taking new tasks; the ones running finish first.
fn execute<T: Sync, R: Send>(
    tasks: &[T],
    order: &[usize],
    jobs: usize,
    work: impl Fn(&T) -> R + Sync,
    mut done: impl FnMut(usize, R) -> Result<(), String>,
) -> Result<(), String> {
    let next = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let (sender, receiver) = mpsc::channel();
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1).min(order.len()) {
            let sender = sender.clone();
            let (next, stop, work) = (&next, &stop, &work);
            scope.spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    let Some(&index) = order.get(next.fetch_add(1, Ordering::Relaxed)) else {
                        break;
                    };
                    if sender.send((index, work(&tasks[index]))).is_err() {
                        break;
                    }
                }
            });
        }
        drop(sender);
        let mut outcome = Ok(());
        for (index, result) in receiver {
            if outcome.is_ok()
                && let Err(error) = done(index, result)
            {
                stop.store(true, Ordering::Relaxed);
                outcome = Err(error);
            }
        }
        outcome
    })
}

/// The order cells start in: by their last wall time on this machine,
/// slowest first, so the run is not left waiting on a slow cell that started
/// last. A cell with no recorded time goes ahead of the timed ones, largest
/// declared budget first, as the likeliest straggler. Ties keep the declared
/// order.
fn slowest_first(tasks: &[Task], previous: impl Fn(&Task) -> Option<f64>) -> Vec<usize> {
    let keys: Vec<(bool, f64)> = tasks
        .iter()
        .map(|task| match previous(task) {
            Some(seconds) => (false, seconds),
            None => (true, task.timeout_s as f64),
        })
        .collect();
    let mut order: Vec<usize> = (0..tasks.len()).collect();
    order.sort_by(|&a, &b| {
        keys[b]
            .0
            .cmp(&keys[a].0)
            .then(keys[b].1.total_cmp(&keys[a].1))
    });
    order
}

type TimingKey = (String, String, String);

/// `slug variant analysis seconds` per line; anything unreadable is ignored.
fn load_timings(path: &Path) -> HashMap<TimingKey, f64> {
    let text = fs::read_to_string(path).unwrap_or_default();
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split('\t');
            let key = (
                fields.next()?.to_owned(),
                fields.next()?.to_owned(),
                fields.next()?.to_owned(),
            );
            let seconds: f64 = fields.next()?.parse().ok()?;
            seconds.is_finite().then_some((key, seconds))
        })
        .collect()
}

/// Merge this run's times over the previous ones and replace the file. A
/// concurrent runner's update may be lost; the file is only a hint.
fn store_timings(path: &Path, previous: HashMap<TimingKey, f64>, measured: Vec<(TimingKey, f64)>) {
    let merged: BTreeMap<TimingKey, f64> = previous.into_iter().chain(measured).collect();
    let mut text = String::new();
    for ((slug, variant, analysis), seconds) in merged {
        text.push_str(&format!("{slug}\t{variant}\t{analysis}\t{seconds:.3}\n"));
    }
    let staged = path.with_extension(format!("{}.tmp", std::process::id()));
    if fs::write(&staged, text).is_ok() && fs::rename(&staged, path).is_err() {
        let _ = fs::remove_file(&staged);
    }
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

/// A cell's result file. Unique per cell, so concurrent children never share
/// one: design slugs are unique directory names, and `.` separates the
/// fields because no variant or analysis id contains one (their decks are
/// named `<variant>.<analysis>.cir`). Each runner process has its own `work`.
fn work_file(work: &Path, design: &Design, variant: &str, analysis: &str, o5: bool) -> PathBuf {
    work.join(format!(
        "{}.{variant}.{analysis}{}.json",
        design.slug,
        if o5 { ".o5" } else { "" }
    ))
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
    let out = work_file(&options.work, design, variant, analysis, o5);
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
    let template_dir = design.o3_dir(variant);
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

/// (Re)capture every reference of `oracle` in the selected designs, `jobs`
/// at a time where [`captures_in_parallel`] allows. Without `update`, a
/// reference must reproduce byte for byte. Reports keep the declared order.
pub fn capture(
    designs: &[String],
    oracle_id: &str,
    update: bool,
    jobs: usize,
) -> Result<Vec<CaptureReport>, String> {
    let designs = selected_designs(designs)?;
    let mut work: Vec<(usize, String)> = Vec::new();
    for (index, design) in designs.iter().enumerate() {
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
        work.extend(references.into_iter().map(|reference| (index, reference)));
    }
    let jobs = if captures_in_parallel(oracle_id) {
        jobs
    } else {
        1
    };
    let order: Vec<usize> = (0..work.len()).collect();
    let mut statuses: Vec<Option<String>> = vec![None; work.len()];
    execute(
        &work,
        &order,
        jobs,
        |(design, reference)| match capture_one(&designs[*design], reference, oracle_id, update) {
            Ok(status) => status,
            Err(error) => format!("FAILED: {error}"),
        },
        |index, status| {
            statuses[index] = Some(status);
            Ok(())
        },
    )?;
    Ok(work
        .iter()
        .zip(statuses)
        .map(|((design, reference), status)| CaptureReport {
            reference: format!("{}/{reference}", designs[*design].slug),
            status: status.expect("every reference ran"),
        })
        .collect())
}

/// Whether one oracle's captures may run side by side. ngspice (O1, O5-O1)
/// and Xyce (O4, O5-O4) write their tables and logs into a fresh temporary
/// directory per run, and what they leave beside the deck is named per run
/// (ngspice's control deck) or per deck (Xyce's `.NOISE` side file), and every
/// reference has its own deck. O2 is serial: every reference of one variant
/// builds the same `oracle/<v>.o2/` Verilog image in place and deletes it
/// after its run, so two captures of one variant would race on it.
pub fn captures_in_parallel(oracle_id: &str) -> bool {
    matches!(oracle_id, "O1" | "O5-O1" | "O4" | "O5-O4")
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
            if !path.is_file() {
                findings.push(format!("{}: outcomes.tsv is missing", design.slug));
                continue;
            }
            match outcomes::read_locked(&path) {
                Ok(rows) => findings.extend(
                    outcomes::coverage(&rows, &design.cells())
                        .into_iter()
                        .map(|finding| format!("{}: {finding}", design.slug)),
                ),
                Err(error) => findings.push(format!("{}: {error}", design.slug)),
            }
        }
        assert!(
            findings.is_empty(),
            "corpus integrity:\n{}",
            findings.join("\n")
        );
    }

    #[test]
    fn every_cell_has_its_own_work_file() {
        let work = Path::new("work");
        let mut seen = BTreeSet::new();
        for design in selected_designs(&[]).expect("corpus loads") {
            for (variant, analysis) in design.cells() {
                assert!(
                    !variant.contains('.') && !analysis.contains('.'),
                    "{}: {variant} {analysis}",
                    design.slug
                );
                for o5 in [false, true] {
                    let path = work_file(work, &design, &variant, &analysis, o5);
                    assert!(seen.insert(path.clone()), "{} is shared", path.display());
                }
            }
        }
    }

    fn task(design: usize, variant: &str, timeout_s: u64) -> Task {
        Task {
            design,
            variant: variant.to_owned(),
            analysis: "tran".to_owned(),
            timeout_s,
        }
    }

    #[test]
    fn untimed_cells_start_first_then_the_slowest() {
        let tasks = vec![
            task(0, "fast", 300),
            task(0, "new-small", 300),
            task(0, "slow", 300),
            task(1, "new-big", 1800),
            task(1, "tie-a", 300),
            task(1, "tie-b", 300),
        ];
        let previous = |task: &Task| match task.variant.as_str() {
            "fast" => Some(0.5),
            "slow" => Some(600.0),
            "tie-a" | "tie-b" => Some(10.0),
            _ => None,
        };
        let order: Vec<&str> = slowest_first(&tasks, previous)
            .into_iter()
            .map(|index| tasks[index].variant.as_str())
            .collect();
        assert_eq!(
            order,
            ["new-big", "new-small", "slow", "tie-a", "tie-b", "fast"]
        );
    }

    /// Run fake cells through the real queue and assembly at `jobs`; each
    /// design's rendered table, and the peak number running at once.
    fn assemble(tasks: &[Task], designs: usize, jobs: usize) -> (Vec<String>, usize) {
        let running = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        let mut assembly = Assembly {
            pending: (0..designs)
                .map(|design| tasks.iter().filter(|task| task.design == design).count())
                .collect(),
            finished: vec![Vec::new(); designs],
        };
        let mut tables = vec![String::new(); designs];
        // Declared order, so the earliest cells, which run longest, finish last.
        let order: Vec<usize> = (0..tasks.len()).collect();
        execute(
            tasks,
            &order,
            jobs,
            |task| {
                let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(task.timeout_s));
                running.fetch_sub(1, Ordering::SeqCst);
                CellReport {
                    design: format!("D{}", task.design),
                    row: Row {
                        variant: task.variant.clone(),
                        analysis: task.analysis.clone(),
                        route: Route::Interp,
                        outcome: Outcome::Pass,
                        detail: format!("{} ms", task.timeout_s),
                    },
                    dispatch: "simulation".into(),
                    elapsed_s: 0.0,
                }
            },
            |index, cell| {
                let design = tasks[index].design;
                if let Some(cells) = assembly.complete(design, index, cell) {
                    let rows: Vec<Row> = cells.into_iter().map(|cell| cell.row).collect();
                    tables[design] = outcomes::render(&rows);
                }
                Ok(())
            },
        )
        .unwrap();
        (tables, peak.load(Ordering::SeqCst))
    }

    #[test]
    fn reports_are_identical_for_any_job_count() {
        let tasks: Vec<Task> = (0..12)
            .map(|index| task(index % 3, &format!("v{index}"), 2 * (12 - index as u64)))
            .collect();
        let (serial, peak) = assemble(&tasks, 3, 1);
        assert_eq!(peak, 1);
        assert!(
            serial[0].contains("v0\ttran\tinterp\tpass\t24 ms\nv3\t"),
            "{}",
            serial[0]
        );
        for jobs in [2, 5, 16] {
            let (parallel, peak) = assemble(&tasks, 3, jobs);
            assert_eq!(parallel, serial, "jobs {jobs}");
            assert!(peak > 1 && peak <= jobs, "jobs {jobs}: peak {peak}");
        }
    }

    #[test]
    fn an_assembly_error_stops_the_queue() {
        let tasks: Vec<Task> = (0..20)
            .map(|index| task(0, &format!("v{index}"), 20))
            .collect();
        let order: Vec<usize> = (0..tasks.len()).collect();
        let started = AtomicUsize::new(0);
        let outcome = execute(
            &tasks,
            &order,
            2,
            |task| {
                started.fetch_add(1, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(task.timeout_s));
            },
            |_, ()| Err("lock timed out".to_owned()),
        );
        assert_eq!(outcome, Err("lock timed out".to_owned()));
        assert!(started.load(Ordering::SeqCst) <= 4, "{started:?}");
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
