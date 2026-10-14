//! Running the selection: a bounded pool of worker processes, a wall-clock
//! budget per run, and a fresh worker after any run that hangs or dies.
//!
//! # Why worker processes
//!
//! A test is arbitrary Verilog. RSpice may loop on it, recurse until the
//! stack is gone, or allocate without bound, and none of those can be
//! interrupted inside one process. So no test runs in the runner itself: each
//! run goes to a `worker` child of the runner binary, which reads one
//! [`CaseSpec`] per line on its standard input, compiles and runs it through
//! [`rspice_core::xspice::verilog::run_digital_testbench`], and answers with
//! one [`CaseResult`] line. The parent kills a worker whose run outlives its
//! budget and records `timeout`; a worker that dies mid-run is recorded as
//! `crash`. Either way the next run gets a fresh worker, and nothing can take
//! the run with it.
//!
//! A worker serves many runs rather than one because starting a process
//! costs tens of milliseconds on Windows and an ivtest run costs about one:
//! with a process per run, process start was nine tenths of the suite's wall
//! time.
//!
//! # The schedules
//!
//! Every test runs in source order. A test whose class executes (`passed`,
//! `gold`, `re`) also runs with the reverse schedule, for the racy rule
//! ([`super::judge::combine`]); the reverse verdict counts only when the
//! source compiled. Both runs are queued together, so the two schedules of a
//! slow test run side by side rather than one after the other.
//!
//! # Order
//!
//! Tests start slowest first, by the times the previous run on this machine
//! measured, so a long test does not start last and hold the run open.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs;
use std::io::{BufRead, BufReader, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use super::judge::{self, Expectation, Outcome, RunEnd};
use super::list::Check;
use super::manifest::Row;
use super::select::{Candidate, Class};

/// The schema of a [`CaseResult`].
pub const RESULT_SCHEMA: &str = "rspice-ivtest-case/1";

/// The prefix of a worker's answer line; any other line on its standard
/// output is not the protocol's and is ignored.
pub const RESULT_PREFIX: &str = "RSPICE-IVTEST-RESULT ";

/// The default budget for one run. The slowest selected test
/// (`multiply_large`, 65-bit multiplication in a loop) takes 6.7 s per run in
/// release on the development box; 30 s leaves a slower runner room and still
/// bounds a hang.
pub const DEFAULT_BUDGET: Duration = Duration::from_secs(30);

/// Which order same-tick activations start in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Schedule {
    Source,
    Reverse,
}

impl Schedule {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Reverse => "reverse",
        }
    }
}

/// What a worker is asked to run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSpec {
    /// The ivtest checkout: the working directory, and the `` `include ``
    /// search directory, as for Icarus run by the suite's own runner.
    pub root: PathBuf,
    /// The source, relative to `root`, spelled with `/` on every platform:
    /// RSpice names the file in its diagnostics, which the manifest records.
    pub source: PathBuf,
    pub plusargs: Vec<String>,
    /// `-gspecify`.
    pub specify: bool,
    /// `-Tmin`/`-Ttyp`/`-Tmax`, as `min`/`typ`/`max`.
    pub delay_select: Option<String>,
    /// Compile and run time zero only (compile-error and compile-only
    /// tests, which Icarus never executes).
    pub compile_only: bool,
    pub schedule: Schedule,
}

impl CaseSpec {
    pub fn for_candidate(root: &Path, candidate: &Candidate, schedule: Schedule) -> Self {
        let entry = &candidate.entry;
        let delay_select = entry
            .args
            .iter()
            .rev()
            .find_map(|arg| arg.strip_prefix("-T"))
            .map(str::to_owned);
        Self {
            root: root.to_path_buf(),
            source: PathBuf::from(entry.source.to_string_lossy().replace('\\', "/")),
            plusargs: entry.plusargs.clone(),
            specify: entry.args.iter().any(|arg| arg == "-gspecify"),
            delay_select,
            compile_only: !candidate.class.is_some_and(Class::runs),
            schedule,
        }
    }
}

/// What a worker reports for one run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseResult {
    pub schema: String,
    pub elapsed_s: f64,
    pub end: RunEnd,
}

impl CaseResult {
    pub fn to_line(&self) -> String {
        format!(
            "{RESULT_PREFIX}{}",
            serde_json::to_string(self).expect("a case result serializes")
        )
    }

    pub fn from_line(line: &str) -> Option<Result<Self, String>> {
        let json = line.trim_end().strip_prefix(RESULT_PREFIX)?;
        Some(
            serde_json::from_str::<Self>(json)
                .map_err(|error| format!("not a case result: {error}"))
                .and_then(|result| {
                    if result.schema == RESULT_SCHEMA {
                        Ok(result)
                    } else {
                        Err(format!(
                            "schema {}, expected {RESULT_SCHEMA}",
                            result.schema
                        ))
                    }
                }),
        )
    }
}

/// Run one case in this process. A panic is reported as a crash rather than
/// ending the worker.
pub fn execute(spec: &CaseSpec) -> CaseResult {
    let started = Instant::now();
    let end = std::panic::catch_unwind(|| execute_end(spec)).unwrap_or_else(|panic| {
        let message = panic
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| panic.downcast_ref::<&str>().map(|text| (*text).to_owned()))
            .unwrap_or_else(|| "a panic with no message".to_owned());
        RunEnd::Crash {
            message: format!("panicked: {}", judge::clip(&message)),
        }
    });
    CaseResult {
        schema: RESULT_SCHEMA.to_owned(),
        elapsed_s: started.elapsed().as_secs_f64(),
        end,
    }
}

/// The worker loop: one spec per input line, one result line per spec.
pub fn serve(input: impl BufRead, mut output: impl std::io::Write) -> Result<(), String> {
    for line in input.lines() {
        let line = line.map_err(|error| error.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let spec: CaseSpec =
            serde_json::from_str(&line).map_err(|error| format!("not a case spec: {error}"))?;
        writeln!(output, "{}", execute(&spec).to_line()).map_err(|error| error.to_string())?;
        output.flush().map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[cfg(feature = "verilog-digital")]
fn execute_end(spec: &CaseSpec) -> RunEnd {
    use rspice_core::config::{HdlDelaySelect, HdlStream};
    use rspice_core::xspice::verilog::{
        DigitalRunError, HdlSchedule, HdlTestbenchEnd, HdlTestbenchOptions, run_digital_testbench,
    };

    let path = spec.root.join(&spec.source);
    let text = match fs::read(&path) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(error) => {
            return RunEnd::Crash {
                message: format!("cannot read {}: {error}", spec.source.display()),
            };
        }
    };
    let delay_select = match spec.delay_select.as_deref() {
        Some("min") => HdlDelaySelect::Min,
        Some("max") => HdlDelaySelect::Max,
        _ => HdlDelaySelect::Typ,
    };
    // Icarus runs every test from the checkout's root; the design's files
    // stay in memory, so parallel runs never share one on disk.
    let options = HdlTestbenchOptions {
        run_directory: Some(spec.root.clone()),
        plusargs: spec.plusargs.clone(),
        delay_select,
        specify: spec.specify,
        include_dirs: vec![spec.root.clone()],
        schedule: match spec.schedule {
            Schedule::Source => HdlSchedule::Source,
            Schedule::Reverse => HdlSchedule::ReverseActive,
        },
        tick_ceiling: spec.compile_only.then_some(0),
        ..HdlTestbenchOptions::default()
    };
    match run_digital_testbench(&[(spec.source.clone(), text)], &options) {
        Ok(report) => {
            let mut stdout = String::new();
            let mut files: Vec<judge::ProducedFile> = Vec::new();
            for record in &report.records {
                match &record.stream {
                    HdlStream::Stdout => stdout.push_str(&record.text),
                    HdlStream::File { path, .. } => {
                        match files.iter_mut().find(|file| file.path == *path) {
                            Some(file) => file.text.push_str(&record.text),
                            None => files.push(judge::ProducedFile {
                                path: path.clone(),
                                text: record.text.clone(),
                            }),
                        }
                    }
                    HdlStream::Diagnostic { .. } | HdlStream::Finish { .. } => {}
                }
            }
            let end = match report.end {
                HdlTestbenchEnd::Finished => "finish",
                HdlTestbenchEnd::Stopped => "stop",
                HdlTestbenchEnd::Fatal => "fatal",
                HdlTestbenchEnd::Quiescent => "quiescence",
                HdlTestbenchEnd::TickCeiling => "tick-ceiling",
            };
            RunEnd::Completed {
                end: end.to_owned(),
                stdout,
                files,
            }
        }
        Err(DigitalRunError::Unsupported(refusal)) => RunEnd::Refused {
            token: refusal.capability.to_owned(),
            message: judge::clip(&refusal.detail),
        },
        Err(DigitalRunError::Compile { detail }) => judge::compile_failure(&detail),
        Err(DigitalRunError::MixedSignalModule { .. }) => RunEnd::Refused {
            token: "mixed-signal-module".to_owned(),
            message: "the module has analog equations".to_owned(),
        },
        Err(error @ DigitalRunError::NoDigitalContent { .. }) => RunEnd::NoContent {
            message: error.to_string(),
        },
        Err(error) => {
            let message = error.to_string();
            // A system call whose host behaviour is not implemented is a
            // named refusal, not a failure.
            match message.split_once(" cannot run here: ") {
                Some((_, detail)) => RunEnd::Refused {
                    token: detail.split('`').nth(1).unwrap_or("system-call").to_owned(),
                    message: judge::clip(detail),
                },
                None => judge::run_failure(message),
            }
        }
    }
}

#[cfg(not(feature = "verilog-digital"))]
fn execute_end(_spec: &CaseSpec) -> RunEnd {
    RunEnd::Crash {
        message: "this build has no digital Verilog execution; enable the `verilog-digital` \
                  feature"
            .to_owned(),
    }
}

/// What `run` was asked to do.
#[derive(Debug, Clone)]
pub struct RunOptions {
    /// The ivtest checkout.
    pub root: PathBuf,
    /// This binary, for spawning `worker` children.
    pub bin: PathBuf,
    /// The most workers at once; 0 means [`default_jobs`].
    pub jobs: usize,
    /// One run's wall-clock budget.
    pub budget: Duration,
    /// Where each test's last wall time is kept, to start the slowest first.
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

/// The default `--jobs`: half the hardware threads, between 2 and 16. A
/// worker is one thread and a few tens of megabytes.
pub fn default_jobs() -> usize {
    let threads = std::thread::available_parallelism().map_or(1, usize::from);
    (threads / 2).clamp(2, 16)
}

/// One test's result.
#[derive(Debug, Clone)]
pub struct TestReport {
    pub row: Row,
    /// The longer of its runs' wall times.
    pub elapsed_s: f64,
    /// Whether either run hit the budget.
    pub timed_out: bool,
}

/// One queued run: a test and a schedule.
#[derive(Debug, Clone, Copy)]
struct Task {
    test: usize,
    schedule: Schedule,
}

/// The runs of one test, until all are in.
#[derive(Debug, Default)]
struct Pending {
    source: Option<(RunEnd, f64)>,
    reverse: Option<(RunEnd, f64)>,
}

/// Run the given candidates and classify each. Excluded candidates are
/// classified without running. `progress` sees each report as it lands.
pub fn run(
    options: &RunOptions,
    candidates: &[Candidate],
    mut progress: impl FnMut(&TestReport),
) -> Result<Vec<TestReport>, String> {
    let previous = options
        .timings
        .as_deref()
        .map(load_timings)
        .unwrap_or_default();
    let mut reports: Vec<Option<TestReport>> = vec![None; candidates.len()];
    let mut order = Vec::new();
    for (index, candidate) in candidates.iter().enumerate() {
        match &candidate.exclusion {
            Some(reason) => reports[index] = Some(excluded(candidate, reason)),
            None => order.push(index),
        }
    }
    // Slowest first; an untimed test goes ahead of the timed ones.
    order.sort_by(|&a, &b| {
        let time = |index: usize| previous.get(&candidates[index].entry.name).copied();
        match (time(a), time(b)) {
            (None, None) => a.cmp(&b),
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(x), Some(y)) => y.total_cmp(&x).then(a.cmp(&b)),
        }
    });
    let mut queue = VecDeque::new();
    for &test in &order {
        queue.push_back(Task {
            test,
            schedule: Schedule::Source,
        });
        if candidates[test].class.is_some_and(Class::runs) {
            queue.push_back(Task {
                test,
                schedule: Schedule::Reverse,
            });
        }
    }
    let workers = options.jobs().max(1).min(queue.len());
    let queue = Arc::new(Mutex::new(queue));
    let mut pending: Vec<Pending> = (0..candidates.len()).map(|_| Pending::default()).collect();
    let (sender, receiver) = mpsc::channel::<(Task, RunEnd, f64)>();
    let mut failure = None;
    std::thread::scope(|scope| {
        for _ in 0..workers {
            let sender = sender.clone();
            let queue = Arc::clone(&queue);
            scope.spawn(move || {
                let mut worker: Option<Worker> = None;
                loop {
                    let next = queue.lock().map(|mut queue| queue.pop_front());
                    let Ok(Some(task)) = next else {
                        break;
                    };
                    let spec = CaseSpec::for_candidate(
                        &options.root,
                        &candidates[task.test],
                        task.schedule,
                    );
                    let (end, elapsed) = run_on_worker(&mut worker, options, &spec);
                    if sender.send((task, end, elapsed)).is_err() {
                        break;
                    }
                }
                if let Some(worker) = worker {
                    worker.finish();
                }
            });
        }
        drop(sender);
        for (task, end, elapsed) in receiver {
            let slot = &mut pending[task.test];
            match task.schedule {
                Schedule::Source => slot.source = Some((end, elapsed)),
                Schedule::Reverse => slot.reverse = Some((end, elapsed)),
            }
            let candidate = &candidates[task.test];
            let runs = candidate.class.is_some_and(Class::runs);
            if slot.source.is_none() || (runs && slot.reverse.is_none()) {
                continue;
            }
            let slot = std::mem::take(slot);
            match classify(&options.root, candidate, slot) {
                Ok(report) => {
                    progress(&report);
                    reports[task.test] = Some(report);
                }
                Err(error) => {
                    failure.get_or_insert(error);
                }
            }
        }
    });
    if let Some(error) = failure {
        return Err(error);
    }
    if let Some(path) = &options.timings {
        let measured = reports
            .iter()
            .flatten()
            .filter(|report| report.elapsed_s > 0.0)
            .map(|report| (report.row.name.clone(), report.elapsed_s));
        store_timings(path, previous, measured);
    }
    reports
        .into_iter()
        .map(|report| report.ok_or_else(|| "a test produced no report".to_owned()))
        .collect()
}

fn excluded(candidate: &Candidate, reason: &str) -> TestReport {
    TestReport {
        row: Row {
            name: candidate.entry.name.clone(),
            list: candidate.entry.list,
            class: candidate.class,
            lane: None,
            outcome: Outcome::Excluded(reason.to_owned()),
            detail: String::new(),
        },
        elapsed_s: 0.0,
        timed_out: false,
    }
}

/// Judge a test's runs and combine the schedules.
fn classify(root: &Path, candidate: &Candidate, runs: Pending) -> Result<TestReport, String> {
    let class = candidate
        .class
        .ok_or_else(|| format!("{} was run without a class", candidate.entry.name))?;
    let (source, source_s) = runs
        .source
        .ok_or_else(|| format!("{} has no source-order run", candidate.entry.name))?;
    let (reverse_s, reverse) = match runs.reverse {
        Some((end, seconds)) => (seconds, Some(end)),
        None => (0.0, None),
    };
    let timed_out =
        matches!(source, RunEnd::Timeout { .. }) || matches!(reverse, Some(RunEnd::Timeout { .. }));
    let (outcome, detail) = match expectation(root, candidate) {
        Err(detail) => (Outcome::Fail, detail),
        Ok(expectation) => {
            let first = judge::judge(class, &expectation, &source);
            // The reverse run counts only when the source compiled: a source
            // that did not has the same verdict on any schedule.
            let second = reverse
                .filter(|_| source.compiled())
                .map(|reverse| judge::judge(class, &expectation, &reverse));
            judge::combine(first, second)
        }
    };
    let detail = match &candidate.gap {
        Some(gap) => format!("{gap}; {detail}"),
        None => detail,
    };
    Ok(TestReport {
        row: Row {
            name: candidate.entry.name.clone(),
            list: candidate.entry.list,
            class: Some(class),
            lane: Some(candidate.lane),
            outcome,
            detail,
        },
        elapsed_s: source_s.max(reverse_s),
        timed_out,
    })
}

/// What a test's run is judged against, with its gold text read from the
/// checkout.
pub(super) fn expectation(root: &Path, candidate: &Candidate) -> Result<Expectation, String> {
    if !candidate.class.is_some_and(Class::runs) {
        return Ok(Expectation::None);
    }
    let read = |relative: &Path, absent_is_empty: bool| match fs::read(root.join(relative)) {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).into_owned()),
        Err(_) if absent_is_empty => Ok(String::new()),
        Err(error) => Err(format!(
            "cannot read the gold file {}: {error}",
            relative.display()
        )),
    };
    Ok(match &candidate.entry.check {
        Check::PassedLine { strict } => Expectation::PassedLine { strict: *strict },
        Check::Gold { path, unordered } => Expectation::Gold {
            text: read(path, false)?,
            unordered: *unordered,
        },
        Check::VvpGold { stem } => Expectation::Gold {
            text: read(
                &Path::new("gold").join(format!("{stem}-vvp-stdout.gold")),
                true,
            )?,
            unordered: false,
        },
        Check::Diff {
            produced,
            gold,
            skip,
        } => Expectation::Diff {
            produced: produced.clone(),
            gold: read(gold, false)?,
            skip: *skip,
        },
    })
}

/// A `worker` child and the lines it answers with.
struct Worker {
    child: Child,
    stdin: ChildStdin,
    lines: mpsc::Receiver<String>,
}

/// How a run on a worker ended when it produced no result.
enum Lost {
    Timeout,
    Crash(String),
}

impl Worker {
    fn spawn(options: &RunOptions) -> Result<Self, String> {
        let mut child = Command::new(&options.bin)
            .arg("worker")
            .current_dir(&options.root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("cannot start a worker: {error}"))?;
        let stdin = child
            .stdin
            .take()
            .ok_or("the worker has no standard input")?;
        let stdout = child
            .stdout
            .take()
            .ok_or("the worker has no standard output")?;
        let (sender, lines) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if sender.send(line).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            child,
            stdin,
            lines,
        })
    }

    /// Send one spec and wait up to `budget` for its result.
    fn run(&mut self, spec: &CaseSpec, budget: Duration) -> Result<CaseResult, Lost> {
        let line = serde_json::to_string(spec).map_err(|error| Lost::Crash(error.to_string()))?;
        writeln!(self.stdin, "{line}")
            .and_then(|()| self.stdin.flush())
            .map_err(|error| Lost::Crash(format!("the worker stopped reading: {error}")))?;
        let deadline = Instant::now() + budget;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            match self.lines.recv_timeout(left) {
                Ok(line) => match CaseResult::from_line(&line) {
                    Some(Ok(result)) => return Ok(result),
                    Some(Err(error)) => return Err(Lost::Crash(error)),
                    None => continue,
                },
                Err(RecvTimeoutError::Timeout) => return Err(Lost::Timeout),
                Err(RecvTimeoutError::Disconnected) => {
                    let status = self
                        .child
                        .wait()
                        .map_or_else(|error| error.to_string(), |status| status.to_string());
                    return Err(Lost::Crash(format!("the worker died ({status})")));
                }
            }
        }
    }

    fn kill(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }

    /// Close its input, so it exits, and reap it.
    fn finish(self) {
        let Self {
            mut child, stdin, ..
        } = self;
        drop(stdin);
        let _ = child.wait();
    }
}

/// Run one spec on this thread's worker, starting one if there is none, and
/// replacing it after a run it did not survive.
fn run_on_worker(
    worker: &mut Option<Worker>,
    options: &RunOptions,
    spec: &CaseSpec,
) -> (RunEnd, f64) {
    let started = Instant::now();
    let active = match worker {
        Some(active) => active,
        None => match Worker::spawn(options) {
            Ok(spawned) => worker.insert(spawned),
            Err(message) => return (RunEnd::Crash { message }, 0.0),
        },
    };
    match active.run(spec, options.budget) {
        Ok(result) => (result.end, result.elapsed_s),
        Err(lost) => {
            if let Some(dead) = worker.take() {
                dead.kill();
            }
            let end = match lost {
                Lost::Timeout => RunEnd::Timeout {
                    seconds: options.budget.as_secs_f64(),
                },
                Lost::Crash(message) => RunEnd::Crash { message },
            };
            (end, started.elapsed().as_secs_f64())
        }
    }
}

/// `name<TAB>seconds` per line; anything unreadable is ignored.
fn load_timings(path: &Path) -> HashMap<String, f64> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let (name, seconds) = line.split_once('\t')?;
            let seconds: f64 = seconds.parse().ok()?;
            seconds.is_finite().then(|| (name.to_owned(), seconds))
        })
        .collect()
}

/// Merge this run's times over the previous ones. A scheduling hint only: a
/// concurrent runner's update may be lost.
fn store_timings(
    path: &Path,
    previous: HashMap<String, f64>,
    measured: impl Iterator<Item = (String, f64)>,
) {
    let merged: BTreeMap<String, f64> = previous.into_iter().chain(measured).collect();
    let text: String = merged
        .into_iter()
        .map(|(name, seconds)| format!("{name}\t{seconds:.3}\n"))
        .collect();
    let staged = path.with_extension(format!("{}.tmp", std::process::id()));
    if fs::write(&staged, text).is_ok() && fs::rename(&staged, path).is_err() {
        let _ = fs::remove_file(&staged);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::verilog::ivtest::list::{Entry, SourceList};
    use crate::suites::verilog::ivtest::select::Lane;

    fn candidate(kind: &str, args: &[&str], check: Check) -> Candidate {
        let entry = Entry {
            name: "t".to_owned(),
            list: SourceList::Vlg,
            kind: kind.to_owned(),
            args: args.iter().map(|arg| (*arg).to_owned()).collect(),
            plusargs: vec!["+x=1".to_owned()],
            runtime_flags: (Vec::new(), Vec::new()),
            // Joined as the list reader joins it: `\` on Windows.
            source: Path::new("ivltests").join("t.v"),
            module: None,
            check,
        };
        Candidate {
            class: Some(match kind {
                "CE" => Class::CompileError,
                "CO" => Class::CompileOnly,
                _ => Class::Passed,
            }),
            entry,
            lane: Lane::L1,
            exclusion: None,
            gap: None,
        }
    }

    #[test]
    fn a_case_spec_carries_the_mapped_options() {
        let root = Path::new("ivtest");
        let spec = CaseSpec::for_candidate(
            root,
            &candidate(
                "normal",
                &["-gspecify", "-Tmax"],
                Check::PassedLine { strict: false },
            ),
            Schedule::Reverse,
        );
        assert!(spec.specify);
        // The source is spelled with `/` everywhere, so RSpice's diagnostics
        // name it the same way on every platform.
        assert_eq!(spec.source.to_str(), Some("ivltests/t.v"));
        assert_eq!(spec.delay_select.as_deref(), Some("max"));
        assert_eq!(spec.plusargs, vec!["+x=1"]);
        assert!(!spec.compile_only);
        assert_eq!(spec.schedule, Schedule::Reverse);

        let ce = CaseSpec::for_candidate(
            root,
            &candidate("CE", &[], Check::PassedLine { strict: false }),
            Schedule::Source,
        );
        assert!(ce.compile_only);
        assert!(!ce.specify);
        assert_eq!(ce.delay_select, None);
    }

    #[test]
    fn result_lines_round_trip_and_other_lines_are_not_results() {
        let result = CaseResult {
            schema: RESULT_SCHEMA.to_owned(),
            elapsed_s: 0.25,
            end: RunEnd::Refused {
                token: "fork".to_owned(),
                message: "not yet supported".to_owned(),
            },
        };
        let line = result.to_line();
        assert_eq!(CaseResult::from_line(&line), Some(Ok(result)));
        assert_eq!(CaseResult::from_line("stray output"), None);
        let other = format!(
            "{RESULT_PREFIX}{}",
            r#"{"schema":"other","elapsed_s":0,"end":{"kind":"timeout","seconds":1}}"#
        );
        assert!(matches!(CaseResult::from_line(&other), Some(Err(_))));
    }

    #[test]
    fn a_worker_answers_each_spec_on_its_own_line() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let spec = CaseSpec {
            root: directory.path().to_path_buf(),
            source: PathBuf::from("absent.v"),
            plusargs: Vec::new(),
            specify: false,
            delay_select: None,
            compile_only: true,
            schedule: Schedule::Source,
        };
        let line = serde_json::to_string(&spec).expect("serializes");
        let input = format!("{line}\n\n{line}\n");
        let mut output = Vec::new();
        serve(input.as_bytes(), &mut output).expect("served");
        let output = String::from_utf8(output).expect("text");
        let results: Vec<CaseResult> = output
            .lines()
            .map(|line| {
                CaseResult::from_line(line)
                    .expect("a result line")
                    .expect("a result")
            })
            .collect();
        assert_eq!(results.len(), 2);
        // A source that cannot be read is the harness's failure, not a verdict.
        assert!(
            matches!(&results[0].end, RunEnd::Crash { .. }),
            "{results:?}"
        );
    }
}
