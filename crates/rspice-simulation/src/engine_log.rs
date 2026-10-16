//! Bounded engine log capture scoped to the executing run.
//!
//! Hosts offer engine records through their logger and drain a queue or
//! forward observed lines. Sink guards restore prior capture on exit, including
//! unwind. Only engine records at Info and above are captured unless the run
//! asks for Debug; Trace, and records from threads the run did not hand work
//! to, remain outside that run.

use std::any::Any;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rspice_app_types::diagnostics::LogSeverity;
use rspice_core::log_scope::{self, LogScope, LogScopeGuard};

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
    /// Lines kept so far in this run, which is not `lines.len()`: the
    /// controller drains every frame, so the queue's length says nothing about
    /// how much of the run has already been shown.
    kept: usize,
    withheld: usize,
    sealed: bool,
}

impl EngineLogQueue {
    /// Keep one line, or count it.
    ///
    /// Reached two ways: a native run's sink pushes what it captured, and the
    /// browser UI instance pushes what the worker posted across the contract.
    /// The bound is applied here so both arrive under the same one.
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
        });
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.kept = 0;
        self.withheld = 0;
        self.sealed = false;
    }

    pub fn drain(&mut self) -> Vec<EngineLogLine> {
        self.lines.drain(..).collect()
    }
}

/// Where a captured line goes when the run has no queue to put it in.
///
/// The browser worker is a separate wasm instance with no shared memory, so its
/// run posts each line across the worker contract instead of pushing it onto a
/// queue the UI thread could lock. Same shape as the runner's transient-sample
/// observer, and for the same reason.
pub type EngineLogObserver = fn(&EngineLogLine);

/// Where one run's engine lines go, and how deep it asked to see.
#[derive(Clone)]
pub struct RunLogSink {
    queue: Option<Arc<Mutex<EngineLogQueue>>>,
    observer: Option<EngineLogObserver>,
    verbose: bool,
}

impl RunLogSink {
    /// A native run's sink: the queue the controller drains each frame.
    pub fn queued(queue: Arc<Mutex<EngineLogQueue>>, verbose: bool) -> Self {
        Self {
            queue: Some(queue),
            observer: None,
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
            verbose,
        }
    }

    fn admits(&self, severity: LogSeverity) -> bool {
        severity != LogSeverity::Debug || self.verbose
    }

    fn deliver(&self, line: EngineLogLine) {
        if let Some(queue) = &self.queue {
            lock_queue(queue).push(line.clone());
        }
        if let Some(observer) = self.observer {
            observer(&line);
        }
    }

    fn seal(&self) {
        if let Some(queue) = &self.queue {
            lock_queue(queue).seal();
        }
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
fn with_current_run<R>(use_run: impl FnOnce(&RunLogSink, Option<&str>) -> R) -> Option<R> {
    let scope = log_scope::capture()?;
    let sink = scope.owner().downcast_ref::<RunLogSink>()?;
    Some(use_run(sink, scope.point()))
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
#[must_use]
pub fn install(sink: RunLogSink) -> RunSinkGuard {
    let verbose = sink.verbose;
    let sink = Arc::new(sink);
    let owner: Arc<dyn Any + Send + Sync> = Arc::clone(&sink) as _;
    let scope = log_scope::enter(Some(LogScope::new(owner)));
    if verbose {
        VERBOSE_RUNS.fetch_add(1, Ordering::SeqCst);
        apply_max_level();
    }
    RunSinkGuard {
        scope: Some(scope),
        sink,
        verbose,
    }
}

/// What removes a run's sink, whatever happens to the run.
///
/// Bound to its thread by the scope guard it holds.
pub struct RunSinkGuard {
    scope: Option<LogScopeGuard>,
    sink: Arc<RunLogSink>,
    verbose: bool,
}

impl Drop for RunSinkGuard {
    fn drop(&mut self) {
        // Leave the scope before sealing, so nothing this thread logs from
        // here on lands after the run's closing row. A worker the run spawned
        // has been joined by now: the run returned before its guard dropped.
        drop(self.scope.take());
        self.sink.seal();
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
    with_current_run(|sink, _| sink.admits(severity)).unwrap_or(false)
}

/// Offer one record to this thread's run, if it has one that wants it.
pub fn offer(record: &log::Record<'_>) {
    if !is_engine_target(record.target()) {
        return;
    }
    let Some(severity) = severity_for_level(record.level()) else {
        return;
    };
    with_current_run(|sink, point| {
        if !sink.admits(severity) {
            return;
        }
        let message = match point {
            Some(point) => format!("{point}: {}", record.args()),
            None => record.args().to_string(),
        };
        sink.deliver(EngineLogLine {
            severity,
            message,
            origin: EngineLogOrigin::Engine,
        });
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

    fn core_record(level: log::Level, message: &str) -> EngineLogLine {
        let severity = severity_for_level(level).expect("the level reaches the Console");
        EngineLogLine {
            severity,
            message: message.to_owned(),
            origin: EngineLogOrigin::Engine,
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
            vec![core_record(log::Level::Warn, message)]
        );
    }

    /// The sparse solver says when it changed factorization backends.
    #[test]
    fn a_sparse_solver_line_is_captured() {
        let message = "klu backend could not factor this system; using faer fallback";
        assert_eq!(
            captured_from("rspice_matrix::sparse", message),
            vec![core_record(log::Level::Warn, message)]
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
            vec![core_record(log::Level::Warn, message)]
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
            vec![core_record(log::Level::Info, "kept")]
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
                core_record(log::Level::Debug, "HB Newton step 1"),
                core_record(log::Level::Info, "also kept"),
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
            vec![core_record(log::Level::Info, "second run")]
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
            vec![core_record(log::Level::Debug, "first")]
        );
        assert_eq!(
            lock_queue(&second).drain(),
            vec![core_record(log::Level::Debug, "second")]
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
                core_record(log::Level::Warn, "sample 4: vds clamped to 5 V"),
                core_record(log::Level::Info, "the run's own line"),
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
            vec![core_record(log::Level::Info, "inner")]
        );
        assert_eq!(
            lock_queue(&outer).drain(),
            vec![core_record(log::Level::Info, "outer")]
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
