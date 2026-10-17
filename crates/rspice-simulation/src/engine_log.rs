//! Bounded engine log capture scoped to the executing run.
//!
//! Hosts offer engine records through their logger and drain a queue or
//! forward observed lines. Sink guards restore prior capture on exit, including
//! unwind. Only engine records at Info and above are captured unless the run
//! asks for Debug; Trace, and records from threads the run did not hand work
//! to, remain outside that run.

use std::any::Any;
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rspice_app_types::diagnostics::LogSeverity;
use rspice_core::log_scope::{self, LogScope, LogScopeGuard};
use rspice_core::notice::{NoticeCollector, RecordedNotice};

mod forward;
pub(crate) use forward::RunLineForwarder;

/// Lines one run's Console log keeps before it starts counting instead.
///
/// A solver trace is unbounded — a stiff transient can ask for a line per
/// rejected step — and the Console is a session log a person reads, not a
/// transcript. Past this many lines the run counts what it withheld and says
/// so once, when the run ends.
const MAX_ENGINE_LOG_LINES: usize = 2_000;

/// One line the engine logged, as it crosses to the Console.
///
/// It carries the shared log severity rather than `log::Level` because the
/// mapping is a decision this module makes once ([`severity_for_level`]) and
/// because the line is serialized across the browser worker boundary, where a
/// `log::Level` would be a second vocabulary to keep in step.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EngineLogLine {
    pub severity: LogSeverity,
    pub message: String,
    /// Who in the run said it. A line that crosses the worker boundary
    /// without one is the engine's, which is all a line ever was before.
    #[serde(default)]
    pub origin: EngineLogOrigin,
    /// Its place among the lines the task's sink delivered, counted from
    /// zero: the number a notice the line states is recorded with
    /// ([`RecordedNotice::line`]). `None` for the closing count, which no
    /// sink delivered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u64>,
}

/// The part of a run a line came from, so the Console can say whose words
/// they are without parsing them.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EngineLogOrigin {
    /// The analysis itself: the engine, the solver and the device models.
    #[default]
    Engine,
    /// A digital testbench's own output: `$display`, `$finish` and the like.
    Hdl,
    /// The Verilog-A compiler, on the models this run elaborated.
    VerilogA,
}

/// One run's captured engine lines, waiting for the controller's next frame.
///
/// The same `Arc<Mutex<…>>` shape as the runner's live transient queue, for the
/// same reason: the solver thread writes it and the UI thread drains it, and a
/// poisoned lock recovers rather than taking the application down with it.
#[derive(Debug, Default)]
pub struct EngineLogQueue {
    lines: VecDeque<EngineLogLine>,
    /// What the run raised, in the shape the Problems page reads, delivered
    /// once when the run ends. Not bounded by the line cap: the notices are
    /// already one per condition, however often the run raised each.
    notices: Vec<RecordedNotice>,
    /// Lines kept so far in this run, which is not `lines.len()`: the
    /// controller drains every frame, so the queue's length says nothing about
    /// how much of the run has already been shown.
    kept: usize,
    withheld: usize,
    sealed: bool,
    suppressed_warnings: usize,
}

impl EngineLogQueue {
    /// Warnings this run did not show because their call site had already
    /// shown its [`last_warning`], for the run's summary to state.
    pub fn suppressed_warnings(&self) -> usize {
        self.suppressed_warnings
    }

    /// Lines past the bound this run counted and did not keep.
    pub fn withheld(&self) -> usize {
        self.withheld
    }

    /// Keep one line, or count it.
    ///
    /// Reached two ways: a native run's sink pushes what it captured, and the
    /// browser UI instance pushes what the worker posted across the contract.
    /// The worker bounds its run before posting and reports the rest through
    /// [`Self::seal_with`]; the bound applies here as well, so both arrive
    /// under the same one.
    pub fn push(&mut self, line: EngineLogLine) {
        if self.kept >= MAX_ENGINE_LOG_LINES {
            self.withheld = self.withheld.saturating_add(1);
            return;
        }
        self.kept = self.kept.saturating_add(1);
        self.lines.push_back(line);
    }

    /// Close the run's log, accounting for anything it withheld.
    ///
    /// Written by the sink guard on every exit path, so the count is exact and
    /// stated once. Announcing it at the first drain past the bound instead
    /// would have printed a number that was already stale, or one row per
    /// frame for the rest of the run.
    fn seal(&mut self) {
        if self.sealed || self.withheld == 0 {
            return;
        }
        self.sealed = true;
        self.lines.push_back(EngineLogLine {
            severity: LogSeverity::Info,
            message: format!(
                "\u{2026} {} further engine lines were not shown (the Console keeps the first \
                 {MAX_ENGINE_LOG_LINES} of a run)",
                self.withheld
            ),
            origin: EngineLogOrigin::Engine,
            ordinal: None,
        });
    }

    /// Close a browser worker run's log with what the worker withheld.
    ///
    /// The worker applies the line bound before it posts, so the lines past
    /// it never reach this queue to be counted here; the worker's own count
    /// arrives once, when its run ends, and closes the log with the same row.
    pub fn seal_with(&mut self, seal: EngineLogSeal) {
        self.withheld = self.withheld.saturating_add(seal.withheld);
        self.suppressed_warnings = self
            .suppressed_warnings
            .saturating_add(seal.suppressed_warnings);
        self.seal();
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.notices.clear();
        self.kept = 0;
        self.withheld = 0;
        self.sealed = false;
        self.suppressed_warnings = 0;
    }

    pub fn drain(&mut self) -> Vec<EngineLogLine> {
        self.lines.drain(..).collect()
    }

    /// Keep what a finished run raised, for the controller to take with its
    /// result. Reached as lines are: from a native run's guard, and from the
    /// message a browser worker's run posts.
    pub fn record_notices(&mut self, notices: Vec<RecordedNotice>) {
        self.notices.extend(notices);
    }

    pub fn drain_notices(&mut self) -> Vec<RecordedNotice> {
        std::mem::take(&mut self.notices)
    }
}

/// What a browser worker's run withheld, sent once when its sink drops.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EngineLogSeal {
    /// Lines past the run's bound that the worker counted and did not post.
    pub withheld: usize,
    /// Warnings the run's per-site limit held back.
    pub suppressed_warnings: usize,
}

/// Where a browser worker run's [`EngineLogSeal`] goes: a posted message, as
/// each line is.
pub type EngineLogSealObserver = fn(EngineLogSeal);

/// Warning lines one call site may log in one run before the rest are counted
/// instead: Spectre's `maxwarns` default.
const WARNINGS_PER_SITE: usize = 5;

/// What a call site's last shown warning ends with, on the same line, so the
/// notice cannot be read as being about whatever another site logged next.
const FURTHER_WARNINGS_SUPPRESSED: &str = "further occurrences of this warning will be suppressed";

/// `text`, the last warning its call site shows in a run, with the notice.
pub fn last_warning(text: impl std::fmt::Display) -> String {
    format!("{text} ({FURTHER_WARNINGS_SUPPRESSED})")
}

/// Holds each call site to [`WARNINGS_PER_SITE`] warnings for as long as it
/// lives: one run's sink, or one command-line run.
///
/// Keyed by the site — the file and line of the `warn!` — rather than by the
/// text, because a clamp warning names the instance and the value, so no two
/// lines of a flood are worded alike. Only warnings are limited. An error is a
/// failure the run states every time, and Info and Debug are the run's account
/// of itself, bounded by the line cap.
#[derive(Debug, Default)]
pub struct WarningLimiter {
    sites: HashMap<(Cow<'static, str>, u32), usize>,
    suppressed: usize,
}

/// What [`WarningLimiter::judge`] decided for one record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningVerdict {
    /// Show the record.
    Show,
    /// Show the record as [`last_warning`]: its site shows no more this run.
    ShowLast,
    /// Show nothing; the record is counted.
    Withhold,
}

impl WarningLimiter {
    pub fn judge(&mut self, record: &log::Record<'_>) -> WarningVerdict {
        if record.level() != log::Level::Warn {
            return WarningVerdict::Show;
        }
        let file = match record.file_static() {
            Some(file) => Cow::Borrowed(file),
            None => Cow::Owned(record.file().unwrap_or(record.target()).to_owned()),
        };
        let count = self
            .sites
            .entry((file, record.line().unwrap_or(0)))
            .or_insert(0);
        *count = count.saturating_add(1);
        match (*count).cmp(&WARNINGS_PER_SITE) {
            std::cmp::Ordering::Less => WarningVerdict::Show,
            std::cmp::Ordering::Equal => WarningVerdict::ShowLast,
            std::cmp::Ordering::Greater => {
                self.suppressed = self.suppressed.saturating_add(1);
                WarningVerdict::Withhold
            }
        }
    }

    /// Warnings judged and not shown.
    pub fn suppressed(&self) -> usize {
        self.suppressed
    }
}

/// Where a captured line goes when the run has no queue to put it in.
///
/// The browser worker is a separate wasm instance with no shared memory, so its
/// run posts each line across the worker contract instead of pushing it onto a
/// queue the UI thread could lock. Same shape as the runner's transient-sample
/// observer, and for the same reason.
pub type EngineLogObserver = fn(&EngineLogLine);

/// Where a browser worker run's notices go: one message, when the run ends.
pub type EngineNoticeObserver = fn(&[RecordedNotice]);

/// Where one run's engine lines go, and how deep it asked to see.
#[derive(Clone)]
pub struct RunLogSink {
    queue: Option<Arc<Mutex<EngineLogQueue>>>,
    observer: Option<EngineLogObserver>,
    notice_observer: Option<EngineNoticeObserver>,
    seal_observer: Option<EngineLogSealObserver>,
    verbose: bool,
}

impl RunLogSink {
    /// A native run's sink: the queue the controller drains each frame.
    pub fn queued(queue: Arc<Mutex<EngineLogQueue>>, verbose: bool) -> Self {
        Self {
            queue: Some(queue),
            observer: None,
            notice_observer: None,
            seal_observer: None,
            verbose,
        }
    }

    /// A browser worker run's sink: one posted message per line.
    ///
    /// The worker host supplies its transport callback when it starts a run.
    #[cfg(any(target_arch = "wasm32", test))]
    pub fn observed(observer: EngineLogObserver, verbose: bool) -> Self {
        Self {
            queue: None,
            observer: Some(observer),
            notice_observer: None,
            seal_observer: None,
            verbose,
        }
    }

    /// Post the run's notices across the worker contract when it ends.
    #[cfg(any(target_arch = "wasm32", test))]
    #[must_use]
    pub fn observing_notices(self, observer: EngineNoticeObserver) -> Self {
        Self {
            notice_observer: Some(observer),
            ..self
        }
    }

    /// The same observed sink, reporting what it withheld when its run ends.
    #[cfg(any(target_arch = "wasm32", test))]
    #[must_use]
    pub fn sealed_by(self, seal_observer: EngineLogSealObserver) -> Self {
        Self {
            seal_observer: Some(seal_observer),
            ..self
        }
    }

    fn admits(&self, severity: LogSeverity) -> bool {
        severity != LogSeverity::Debug || self.verbose
    }

    fn deliver(&self, line: &EngineLogLine) {
        if let Some(queue) = &self.queue {
            lock_queue(queue).push(line.clone());
        }
        if let Some(observer) = self.observer {
            observer(line);
        }
    }

    fn count_suppressed_warning(&self) {
        if let Some(queue) = &self.queue {
            let mut queue = lock_queue(queue);
            queue.suppressed_warnings = queue.suppressed_warnings.saturating_add(1);
        }
    }

    fn deliver_notices(&self, notices: Vec<RecordedNotice>) {
        if notices.is_empty() {
            return;
        }
        if let Some(observer) = self.notice_observer {
            observer(&notices);
        }
        if let Some(queue) = &self.queue {
            lock_queue(queue).record_notices(notices);
        }
    }

    fn seal(&self) {
        if let Some(queue) = &self.queue {
            lock_queue(queue).seal();
        }
    }
}

/// What one run keeps in the engine's log scope while it executes: where its
/// lines go, and how many warnings each call site has spent.
///
/// The limiter lives here rather than in [`RunLogSink`], which a request
/// carries and may clone, so its count is exactly one installed run's.
struct ActiveRun {
    sink: RunLogSink,
    warnings: Mutex<WarningLimiter>,
    /// Lines offered to a sink with no queue. A queue applies the bound
    /// itself; a browser worker applies it here, before it formats and posts a
    /// line the UI would only have counted and dropped.
    unqueued: AtomicUsize,
    /// Lines delivered so far, on every thread of the run: the next line's
    /// [`EngineLogLine::ordinal`].
    delivered: AtomicU64,
}

impl ActiveRun {
    fn new(sink: RunLogSink) -> Self {
        Self {
            sink,
            warnings: Mutex::new(WarningLimiter::default()),
            unqueued: AtomicUsize::new(0),
            delivered: AtomicU64::new(0),
        }
    }

    /// Deliver the line `line` builds from its ordinal, building it only if
    /// it will be sent. Returns the ordinal, or `None` when it was not sent.
    fn deliver(&self, line: impl FnOnce(u64) -> EngineLogLine) -> Option<u64> {
        if self.sink.queue.is_none()
            && self.unqueued.fetch_add(1, Ordering::Relaxed) >= MAX_ENGINE_LOG_LINES
        {
            return None;
        }
        let ordinal = self.delivered.fetch_add(1, Ordering::Relaxed);
        self.sink.deliver(&line(ordinal));
        Some(ordinal)
    }

    /// Close the run's log: a queue states what it withheld, and a worker
    /// sends its count across so the UI's queue can.
    fn seal(&self) {
        self.sink.seal();
        let Some(seal_observer) = self.sink.seal_observer else {
            return;
        };
        let seal = EngineLogSeal {
            withheld: self
                .unqueued
                .load(Ordering::Relaxed)
                .saturating_sub(MAX_ENGINE_LOG_LINES),
            suppressed_warnings: self
                .warnings
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .suppressed(),
        };
        if seal != EngineLogSeal::default() {
            seal_observer(seal);
        }
    }

    fn judge(&self, record: &log::Record<'_>) -> WarningVerdict {
        let verdict = self
            .warnings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .judge(record);
        if verdict == WarningVerdict::Withhold {
            self.sink.count_suppressed_warning();
        }
        verdict
    }
}

/// The runner's own poison recovery, for the one lock this module owns.
pub fn lock_queue(queue: &Arc<Mutex<EngineLogQueue>>) -> std::sync::MutexGuard<'_, EngineLogQueue> {
    match queue.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// The sink of the run executing on this thread, if one is, and the point of
/// that run the thread is solving.
///
/// The sink sits in the engine's log scope rather than a thread-local of this
/// module's, because the engine hands parts of a run to threads it spawns — a
/// Monte Carlo worker, a chunk of a parallel sweep — and enters the scope on
/// each. A thread this module never saw is then still inside the run.
fn with_current_run<R>(use_run: impl FnOnce(&ActiveRun, Option<&str>) -> R) -> Option<R> {
    let scope = log_scope::capture()?;
    let run = scope.owner().downcast_ref::<ActiveRun>()?;
    Some(use_run(run, scope.point()))
}

/// Runs that asked for verbose and have not finished.
///
/// A count and not a flag: two verbose runs may overlap, and the first to
/// finish must not take the process log level back down under the second.
static VERBOSE_RUNS: AtomicUsize = AtomicUsize::new(0);

/// The level the installed logger's own filter needs, as a [`level_index`].
///
/// `Off` until a logger reports one, which is what a build with no studio
/// logger — the browser UI instance, a test binary — leaves it at.
static STDERR_LEVEL: AtomicUsize = AtomicUsize::new(0);

/// Installs `sink` on this thread until the returned guard is dropped.
///
/// The guard restores whatever sink was there before, on every exit path
/// including a panic unwinding out of the solver, and seals the run's queue as
/// it goes.
///
/// The run's notices are collected in the same scope, on every thread the run
/// hands work to, and delivered to the same sink when the guard drops: the
/// structured account of the advisories whose prose the lines carry.
#[must_use]
pub fn install(sink: RunLogSink) -> RunSinkGuard {
    let verbose = sink.verbose;
    let run = Arc::new(ActiveRun::new(sink));
    let owner: Arc<dyn Any + Send + Sync> = Arc::clone(&run) as _;
    let notices = NoticeCollector::new();
    let scope = log_scope::enter(Some(LogScope::new(owner).collecting(notices.clone())));
    if verbose {
        VERBOSE_RUNS.fetch_add(1, Ordering::SeqCst);
        apply_max_level();
    }
    RunSinkGuard {
        scope: Some(scope),
        run,
        verbose,
        notices,
    }
}

/// What removes a run's sink, whatever happens to the run.
///
/// Bound to its thread by the scope guard it holds.
pub struct RunSinkGuard {
    scope: Option<LogScopeGuard>,
    run: Arc<ActiveRun>,
    verbose: bool,
    notices: NoticeCollector,
}

impl Drop for RunSinkGuard {
    fn drop(&mut self) {
        // Leave the scope before sealing, so nothing this thread logs from
        // here on lands after the run's closing row. A worker the run spawned
        // has been joined by now: the run returned before its guard dropped,
        // so every notice it raised is in the collector.
        drop(self.scope.take());
        self.run.sink.deliver_notices(self.notices.take());
        self.run.seal();
        if self.verbose {
            VERBOSE_RUNS.fetch_sub(1, Ordering::SeqCst);
            apply_max_level();
        }
    }
}

/// The log targets whose code executes inside an analysis, and so speaks for
/// the run: the engine, the Verilog-A compiler and runtime it calls, the sparse
/// solver under it, and the services that drive an analysis from this crate.
///
/// The rest of this crate is the runner's own bookkeeping: lock recovery,
/// thread lifecycle and a `{:?}` dump of the request, none of which is the
/// run's account of itself.
const ENGINE_TARGETS: [&str; 4] = [
    "rspice_core",
    "rspice_veriloga",
    "rspice_matrix",
    "rspice_simulation::engine_services",
];

/// Whether the target is one of [`ENGINE_TARGETS`] or a module inside one.
fn is_engine_target(target: &str) -> bool {
    ENGINE_TARGETS.iter().any(|root| {
        target
            .strip_prefix(root)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with("::"))
    })
}

/// The Console severity one engine level reports as, or `None` for a level the
/// Console never shows.
fn severity_for_level(level: log::Level) -> Option<LogSeverity> {
    match level {
        log::Level::Error => Some(LogSeverity::Error),
        log::Level::Warn => Some(LogSeverity::Warning),
        log::Level::Info => Some(LogSeverity::Info),
        log::Level::Debug => Some(LogSeverity::Debug),
        // A trace record is development output, not a run's account of itself.
        log::Level::Trace => None,
    }
}

/// Whether this thread's run would capture a record with this metadata.
///
/// The installed logger answers `log::Log::enabled` with this, so
/// `log_enabled!` tells the engine the truth about whether formatting a
/// `debug!` would reach anything.
pub fn admits(metadata: &log::Metadata<'_>) -> bool {
    if !is_engine_target(metadata.target()) {
        return false;
    }
    let Some(severity) = severity_for_level(metadata.level()) else {
        return false;
    };
    with_current_run(|run, _| run.sink.admits(severity)).unwrap_or(false)
}

/// Offer one record to this thread's run, if it has one that wants it.
///
/// Whatever becomes of a warning, the logging thread is told which line it
/// was kept as ([`log_scope::logged`]), so the site that logged it raises the
/// notice it states with that line, and with none when it was withheld.
pub fn offer(record: &log::Record<'_>) {
    if !is_engine_target(record.target()) {
        return;
    }
    let Some(severity) = severity_for_level(record.level()) else {
        return;
    };
    with_current_run(|run, point| {
        let ordinal = if !run.sink.admits(severity) {
            None
        } else {
            match run.judge(record) {
                WarningVerdict::Withhold => None,
                verdict => run.deliver(|ordinal| {
                    let message = match point {
                        Some(point) => format!("{point}: {}", record.args()),
                        None => record.args().to_string(),
                    };
                    EngineLogLine {
                        severity,
                        message: if verdict == WarningVerdict::ShowLast {
                            last_warning(message)
                        } else {
                            message
                        },
                        origin: EngineLogOrigin::Engine,
                        ordinal: Some(ordinal),
                    }
                }),
            }
        };
        // Only a warning states a notice.
        log_scope::logged(ordinal.filter(|_| severity == LogSeverity::Warning));
    });
}

/// The process log level the stderr filter and the Console sink together need.
///
/// `Info`, so an engine receipt reaches a sink at all, and `Debug` only while
/// some run has asked for verbose. That is what keeps an ordinary run's cost at
/// zero: the `log!` macro compares against this level before it builds a
/// record, so the engine's `debug!` sites are not merely filtered out — they
/// are not entered.
fn admitted_max_level(stderr: log::LevelFilter, verbose_runs: usize) -> log::LevelFilter {
    let console = if verbose_runs > 0 {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    stderr.max(console)
}

/// Record what the installed logger prints to stderr, and apply the result.
pub fn note_stderr_level(level: log::LevelFilter) {
    STDERR_LEVEL.store(level_index(level), Ordering::SeqCst);
    apply_max_level();
}

fn apply_max_level() {
    let stderr = level_of_index(STDERR_LEVEL.load(Ordering::SeqCst));
    let verbose_runs = VERBOSE_RUNS.load(Ordering::SeqCst);
    log::set_max_level(admitted_max_level(stderr, verbose_runs));
}

fn level_index(level: log::LevelFilter) -> usize {
    match level {
        log::LevelFilter::Off => 0,
        log::LevelFilter::Error => 1,
        log::LevelFilter::Warn => 2,
        log::LevelFilter::Info => 3,
        log::LevelFilter::Debug => 4,
        log::LevelFilter::Trace => 5,
    }
}

fn level_of_index(index: usize) -> log::LevelFilter {
    match index {
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        5 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The engine line the run delivered `ordinal`-th, from zero.
    fn core_record(ordinal: u64, level: log::Level, message: &str) -> EngineLogLine {
        let severity = severity_for_level(level).expect("the level reaches the Console");
        EngineLogLine {
            severity,
            message: message.to_owned(),
            origin: EngineLogOrigin::Engine,
            ordinal: Some(ordinal),
        }
    }

    /// A line posted without an origin is read as the engine's.
    #[test]
    fn a_line_without_an_origin_is_the_engines() {
        let line: EngineLogLine =
            serde_json::from_str(r#"{"severity":"Warning","message":"clamped"}"#)
                .expect("a line without an origin still reads");
        assert_eq!(line.origin, EngineLogOrigin::Engine);
        let hdl = EngineLogLine {
            origin: EngineLogOrigin::Hdl,
            ..line
        };
        let round_trip: EngineLogLine =
            serde_json::from_str(&serde_json::to_string(&hdl).expect("serializes"))
                .expect("reads back");
        assert_eq!(round_trip, hdl);
    }

    fn offer_core(level: log::Level, message: &str) {
        offer(
            &log::Record::builder()
                .level(level)
                .target("rspice_core::engine::transient")
                .args(format_args!("{message}"))
                .build(),
        );
    }

    #[test]
    fn a_core_line_offered_with_no_run_installed_is_not_captured() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        offer_core(log::Level::Info, "no run owns this line");
        assert!(lock_queue(&queue).drain().is_empty());
    }

    /// A line the application logged is not the engine's account of a run.
    #[test]
    fn a_line_from_outside_the_engine_is_not_captured() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        let _guard = install(RunLogSink::queued(Arc::clone(&queue), true));
        for target in [
            "rspice_ui",
            "rspice_app::controller",
            "rspice_corella",
            "rspice_veriloga_runtime",
            "rspice_simulation",
            "rspice_simulation::runner",
            "rspice_simulation::engine_services_extra",
            "wgpu_hal",
        ] {
            offer_from(target, log::Level::Info, "not the engine");
        }
        assert!(lock_queue(&queue).drain().is_empty());
    }

    fn offer_from(target: &str, level: log::Level, message: &str) {
        offer(
            &log::Record::builder()
                .level(level)
                .target(target)
                .args(format_args!("{message}"))
                .build(),
        );
    }

    fn captured_from(target: &str, message: &str) -> Vec<EngineLogLine> {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            assert!(admits(
                &log::Metadata::builder()
                    .level(log::Level::Warn)
                    .target(target)
                    .build()
            ));
            offer_from(target, log::Level::Warn, message);
        }
        lock_queue(&queue).drain()
    }

    /// The Verilog-A compiler reports a lost AC contribution while the run
    /// elaborates its models.
    #[test]
    fn a_verilog_a_compiler_line_is_captured() {
        let message = "ddt() on both sides of a product; reactive AC contribution omitted";
        assert_eq!(
            captured_from("rspice_veriloga::ir", message),
            vec![core_record(0, log::Level::Warn, message)]
        );
    }

    /// The sparse solver says when it changed factorization backends.
    #[test]
    fn a_sparse_solver_line_is_captured() {
        let message = "klu backend could not factor this system; using faer fallback";
        assert_eq!(
            captured_from("rspice_matrix::sparse", message),
            vec![core_record(0, log::Level::Warn, message)]
        );
    }

    /// The service driving a transient says when it carried on without an
    /// operating point.
    #[test]
    fn an_analysis_service_line_is_captured() {
        let message = "DC OP computation failed: singular matrix (continuing with transient if \
                       available)";
        assert_eq!(
            captured_from("rspice_simulation::engine_services::transient", message),
            vec![core_record(0, log::Level::Warn, message)]
        );
    }

    #[test]
    fn a_debug_line_is_captured_only_when_the_run_asked_for_verbose() {
        let quiet = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&quiet), false));
            assert!(!admits(
                &log::Metadata::builder()
                    .level(log::Level::Debug)
                    .target("rspice_core::analysis::harmonic_balance")
                    .build()
            ));
            offer_core(log::Level::Debug, "HB Newton step 1");
            offer_core(log::Level::Info, "kept");
        }
        assert_eq!(
            lock_queue(&quiet).drain(),
            vec![core_record(0, log::Level::Info, "kept")]
        );

        let verbose = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&verbose), true));
            assert!(admits(
                &log::Metadata::builder()
                    .level(log::Level::Debug)
                    .target("rspice_core::analysis::harmonic_balance")
                    .build()
            ));
            offer_core(log::Level::Debug, "HB Newton step 1");
            // Trace is never captured, verbose or not: it is development
            // output, not a run's account of itself.
            offer_core(log::Level::Info, "also kept");
            offer(
                &log::Record::builder()
                    .level(log::Level::Trace)
                    .target("rspice_core")
                    .args(format_args!("trace"))
                    .build(),
            );
        }
        assert_eq!(
            lock_queue(&verbose).drain(),
            vec![
                core_record(0, log::Level::Debug, "HB Newton step 1"),
                core_record(1, log::Level::Info, "also kept"),
            ]
        );
    }

    #[test]
    fn the_console_keeps_the_first_two_thousand_engine_lines_and_counts_the_rest() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            for index in 0..(MAX_ENGINE_LOG_LINES + 137) {
                offer_core(log::Level::Info, &format!("line {index}"));
            }
            // Drained mid-run, the way the controller drains every frame: the
            // bound is on what the run delivered, not on what is waiting.
            assert_eq!(lock_queue(&queue).drain().len(), MAX_ENGINE_LOG_LINES);
            offer_core(log::Level::Info, "still past the bound");
        }

        let tail = lock_queue(&queue).drain();
        assert_eq!(tail.len(), 1, "only the accounting row is left: {tail:?}");
        assert_eq!(
            tail[0].message,
            "\u{2026} 138 further engine lines were not shown (the Console keeps the first 2000 \
             of a run)"
        );
        assert_eq!(tail[0].severity, LogSeverity::Info);
    }

    #[test]
    fn a_sealed_queue_states_its_withheld_count_once() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            for index in 0..(MAX_ENGINE_LOG_LINES + 1) {
                offer_core(log::Level::Info, &format!("line {index}"));
            }
        }
        let mut guard = lock_queue(&queue);
        assert_eq!(guard.drain().len(), MAX_ENGINE_LOG_LINES + 1);
        guard.seal();
        assert!(
            guard.drain().is_empty(),
            "the accounting row is stated once per run"
        );
    }

    /// The next run of the same runner starts from an empty account.
    #[test]
    fn clearing_the_queue_restores_the_whole_bound() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            for index in 0..(MAX_ENGINE_LOG_LINES + 5) {
                offer_core(log::Level::Info, &format!("line {index}"));
            }
        }
        lock_queue(&queue).clear();
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            offer_core(log::Level::Info, "second run");
        }
        assert_eq!(
            lock_queue(&queue).drain(),
            vec![core_record(0, log::Level::Info, "second run")]
        );
    }

    #[test]
    fn two_runs_on_two_threads_keep_their_lines_apart() {
        let first = Arc::new(Mutex::new(EngineLogQueue::default()));
        let second = Arc::new(Mutex::new(EngineLogQueue::default()));
        let (start, run) = (
            Arc::new(std::sync::Barrier::new(2)),
            Arc::new(std::sync::Barrier::new(2)),
        );

        let threads: Vec<_> = [("first", &first), ("second", &second)]
            .into_iter()
            .map(|(name, queue)| {
                let queue = Arc::clone(queue);
                let start = Arc::clone(&start);
                let run = Arc::clone(&run);
                std::thread::spawn(move || {
                    let _guard = install(RunLogSink::queued(queue, true));
                    start.wait();
                    offer_core(log::Level::Debug, name);
                    run.wait();
                })
            })
            .collect();
        for thread in threads {
            thread.join().expect("both runs finish");
        }

        assert_eq!(
            lock_queue(&first).drain(),
            vec![core_record(0, log::Level::Debug, "first")]
        );
        assert_eq!(
            lock_queue(&second).drain(),
            vec![core_record(0, log::Level::Debug, "second")]
        );
    }

    /// A run on another thread leaves the offering thread's sink alone.
    #[test]
    fn a_line_offered_from_a_thread_with_no_run_reaches_no_queue() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        let held = Arc::clone(&queue);
        let _guard = install(RunLogSink::queued(queue, true));
        std::thread::spawn(|| {
            offer_core(log::Level::Info, "logged by a thread with no run");
        })
        .join()
        .expect("the offering thread finishes");
        assert!(lock_queue(&held).drain().is_empty());
    }

    fn warn_at(line: u32, level: log::Level, message: &str) {
        offer(
            &log::Record::builder()
                .level(level)
                .target("rspice_core::engine::transient")
                .file_static(Some("crates/rspice-core/src/engine/transient.rs"))
                .line(Some(line))
                .args(format_args!("{message}"))
                .build(),
        );
    }

    /// Spectre's `maxwarns`: five from a call site, the fifth saying it is the
    /// last, then a count the run's summary can state.
    #[test]
    fn a_call_site_shows_five_warnings_then_says_the_rest_are_suppressed() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            for index in 1..=8 {
                warn_at(100, log::Level::Warn, &format!("M{index}: vds clamped"));
            }
            // Another site keeps its own five.
            warn_at(200, log::Level::Warn, "R1: negative resistance");
            // A failure is stated every time.
            for _ in 0..7 {
                warn_at(300, log::Level::Error, "singular matrix");
            }
        }
        let mut queue = lock_queue(&queue);
        let messages: Vec<_> = queue.drain().into_iter().map(|line| line.message).collect();
        let mut expected: Vec<String> = (1..=4)
            .map(|index| format!("M{index}: vds clamped"))
            .collect();
        expected.push(
            "M5: vds clamped (further occurrences of this warning will be suppressed)".to_owned(),
        );
        expected.push("R1: negative resistance".to_owned());
        expected.extend(std::iter::repeat_n("singular matrix".to_owned(), 7));
        assert_eq!(messages, expected);
        assert_eq!(queue.suppressed_warnings(), 3);
        queue.clear();
        assert_eq!(queue.suppressed_warnings(), 0);
    }

    /// The limit is one run's: the next run of the same site starts again.
    #[test]
    fn each_run_starts_with_every_call_site_unspent() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        for run in 0..2 {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            for _ in 0..6 {
                warn_at(100, log::Level::Warn, &format!("run {run}"));
            }
        }
        let lines = lock_queue(&queue).drain();
        // Each run: five warnings, the fifth saying the sixth is held back.
        assert_eq!(lines.len(), 10, "{lines:?}");
        assert_eq!(lines[4].message, last_warning("run 0"));
        assert_eq!(lines[5].message, "run 1");
        assert_eq!(lock_queue(&queue).suppressed_warnings(), 2);
    }

    /// A worker the engine hands a sample to logs into the run that spawned
    /// it, and its lines say which sample they came from.
    #[test]
    fn a_line_from_a_worker_inside_the_run_is_captured_with_its_point() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            let captured = log_scope::capture();
            std::thread::scope(|threads| {
                threads.spawn(|| {
                    let _run = log_scope::enter(captured.clone());
                    let _sample =
                        log_scope::enter(log_scope::capture().map(|run| run.at_point("sample 4")));
                    offer_core(log::Level::Warn, "vds clamped to 5 V");
                });
            });
            offer_core(log::Level::Info, "the run's own line");
        }
        assert_eq!(
            lock_queue(&queue).drain(),
            vec![
                core_record(0, log::Level::Warn, "sample 4: vds clamped to 5 V"),
                core_record(1, log::Level::Info, "the run's own line"),
            ]
        );
    }

    #[test]
    fn a_run_sink_restores_the_one_it_displaced() {
        let outer = Arc::new(Mutex::new(EngineLogQueue::default()));
        let inner = Arc::new(Mutex::new(EngineLogQueue::default()));
        let _outer_guard = install(RunLogSink::queued(Arc::clone(&outer), false));
        {
            let _inner_guard = install(RunLogSink::queued(Arc::clone(&inner), false));
            offer_core(log::Level::Info, "inner");
        }
        offer_core(log::Level::Info, "outer");

        assert_eq!(
            lock_queue(&inner).drain(),
            vec![core_record(0, log::Level::Info, "inner")]
        );
        assert_eq!(
            lock_queue(&outer).drain(),
            vec![core_record(0, log::Level::Info, "outer")]
        );
    }

    /// An observed sink is the browser worker's: no queue, one call per line.
    #[test]
    fn an_observed_sink_reports_every_captured_line() {
        static OBSERVED: Mutex<Vec<String>> = Mutex::new(Vec::new());
        fn observe(line: &EngineLogLine) {
            OBSERVED
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push(line.message.clone());
        }

        {
            let _guard = install(RunLogSink::observed(observe, false));
            offer_core(log::Level::Info, "posted");
            offer_core(log::Level::Debug, "withheld");
        }
        let observed = OBSERVED
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        assert_eq!(observed, vec!["posted".to_owned()]);
    }

    /// The browser worker bounds a run before it posts, and says once, as
    /// the run ends, what it held back, so the UI's queue closes the run's log
    /// with the row a native run's queue writes.
    #[test]
    fn an_observed_sink_posts_the_bound_and_seals_with_the_rest() {
        static POSTED: AtomicUsize = AtomicUsize::new(0);
        static SEALS: Mutex<Vec<EngineLogSeal>> = Mutex::new(Vec::new());
        fn post(_: &EngineLogLine) {
            POSTED.fetch_add(1, Ordering::Relaxed);
        }
        fn seal(seal: EngineLogSeal) {
            SEALS
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(seal);
        }

        {
            let _guard = install(RunLogSink::observed(post, false).sealed_by(seal));
            for index in 0..(MAX_ENGINE_LOG_LINES - 2) {
                offer_core(log::Level::Info, &format!("line {index}"));
            }
            for index in 0..8 {
                warn_at(100, log::Level::Warn, &format!("M{index}: vds clamped"));
            }
        }
        // Five warnings shown make 2003 lines offered: 2000 posted.
        assert_eq!(POSTED.load(Ordering::Relaxed), MAX_ENGINE_LOG_LINES);
        let seals = SEALS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let expected = EngineLogSeal {
            withheld: 3,
            suppressed_warnings: 3,
        };
        assert_eq!(seals, vec![expected]);

        // The UI side: the worker's count closes its queue with the native row.
        let mut queue = EngineLogQueue::default();
        queue.seal_with(expected);
        let rows = queue.drain();
        assert_eq!(rows.len(), 1, "{rows:?}");
        assert_eq!(
            rows[0].message,
            "\u{2026} 3 further engine lines were not shown (the Console keeps the first 2000 \
             of a run)"
        );
        assert_eq!(queue.suppressed_warnings(), 3);
    }

    /// A run that withheld nothing sends no seal.
    #[test]
    fn an_observed_run_within_its_bound_sends_no_seal() {
        static SEALED: AtomicUsize = AtomicUsize::new(0);
        fn post(_: &EngineLogLine) {}
        fn seal(_: EngineLogSeal) {
            SEALED.fetch_add(1, Ordering::Relaxed);
        }
        {
            let _guard = install(RunLogSink::observed(post, false).sealed_by(seal));
            offer_core(log::Level::Info, "one line");
        }
        assert_eq!(SEALED.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn the_process_level_admits_an_engine_receipt_and_only_a_verbose_trace() {
        // Quiet stderr still has to admit Info, or the receipt never reaches a
        // sink at all.
        assert_eq!(
            admitted_max_level(log::LevelFilter::Warn, 0),
            log::LevelFilter::Info
        );
        assert_eq!(
            admitted_max_level(log::LevelFilter::Off, 0),
            log::LevelFilter::Info
        );
        // One verbose run raises it; the rest of the time `debug!` is not
        // entered at all.
        assert_eq!(
            admitted_max_level(log::LevelFilter::Warn, 1),
            log::LevelFilter::Debug
        );
        assert_eq!(
            admitted_max_level(log::LevelFilter::Warn, 2),
            log::LevelFilter::Debug
        );
        // A stderr filter that asks for more keeps it.
        assert_eq!(
            admitted_max_level(log::LevelFilter::Trace, 0),
            log::LevelFilter::Trace
        );
    }

    #[test]
    fn every_level_index_round_trips() {
        for level in [
            log::LevelFilter::Off,
            log::LevelFilter::Error,
            log::LevelFilter::Warn,
            log::LevelFilter::Info,
            log::LevelFilter::Debug,
            log::LevelFilter::Trace,
        ] {
            assert_eq!(level_of_index(level_index(level)), level);
        }
    }
}
