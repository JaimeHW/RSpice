//! The engine's own log lines, carried into the Console for the run that
//! produced them.
//!
//! `rspice-core` reports through the `log` facade — receipts at `info!`, solver
//! traces at `debug!` — and nothing used to carry those lines anywhere a reader
//! could see them. The desktop logger writes stderr, which a windowed
//! application does not show, and the Console's [`LogBuffer`] was written only
//! by explicit UI calls. So a transient-noise run could not state how many
//! device noise sources it injected or from which seed — numbers that exist
//! nowhere else — and the harmonic-balance and PSS forms could offer no Verbose
//! switch, because the output such a switch turns on had nowhere to appear.
//!
//! # What a sink is, and is not
//!
//! The sink is **run-scoped and thread-scoped**: [`install`] puts one on the
//! calling thread for as long as its guard lives, which is the duration of one
//! run. Two runs on two threads therefore never see each other's lines, and no
//! global sink exists for two runs to race for.
//!
//! The limitation that buys is deliberate and worth stating: a core line logged
//! on **any other thread is not captured**. The UI thread, a cloud executor,
//! and any rayon worker the engine may use inside a solve are all outside the
//! run's sink. A global sink would capture those, and would also hand one run
//! the other's lines, so this module does not reach for one.
//!
//! # What is captured
//!
//! Only records whose target is `rspice_core` (or a module inside it), because
//! the Console is reporting what the *engine* said. `Info` and above always;
//! `Debug` only when the run asked for verbose; `Trace` never. The engine's
//! message text is carried unchanged.
//!
//! # What it costs an ordinary run
//!
//! Nothing. [`admitted_max_level`] keeps the process log level at `Info` while
//! no verbose run is live, so `log::debug!` in the engine does not even reach
//! the logger, and the installed logger answers `enabled` as `false` for a core
//! `Debug` record on a thread with no verbose sink.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::{LogBuffer, LogSeverity, LogSource};

/// Lines one run's Console log keeps before it starts counting instead.
///
/// A solver trace is unbounded — a stiff transient can ask for a line per
/// rejected step — and the Console is a session log a person reads, not a
/// transcript. Past this many lines the run counts what it withheld and says
/// so once, when the run ends.
const MAX_ENGINE_LOG_LINES: usize = 2_000;

/// One line the engine logged, as it crosses to the Console.
///
/// It carries the Console's own severity rather than `log::Level` because the
/// mapping is a decision this module makes once ([`severity_for_level`]) and
/// because the line is serialized across the browser worker boundary, where a
/// `log::Level` would be a second vocabulary to keep in step.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct EngineLogLine {
    pub(crate) severity: LogSeverity,
    pub(crate) message: String,
}

/// One run's captured engine lines, waiting for the controller's next frame.
///
/// The same `Arc<Mutex<…>>` shape as the runner's live transient queue, for the
/// same reason: the solver thread writes it and the UI thread drains it, and a
/// poisoned lock recovers rather than taking the application down with it.
#[derive(Debug, Default)]
pub(crate) struct EngineLogQueue {
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
    pub(crate) fn push(&mut self, line: EngineLogLine) {
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
        });
    }

    pub(crate) fn clear(&mut self) {
        self.lines.clear();
        self.kept = 0;
        self.withheld = 0;
        self.sealed = false;
    }

    pub(crate) fn drain(&mut self) -> Vec<EngineLogLine> {
        self.lines.drain(..).collect()
    }
}

/// Where a captured line goes when the run has no queue to put it in.
///
/// The browser worker is a separate wasm instance with no shared memory, so its
/// run posts each line across the worker contract instead of pushing it onto a
/// queue the UI thread could lock. Same shape as the runner's transient-sample
/// observer, and for the same reason.
pub(crate) type EngineLogObserver = fn(&EngineLogLine);

/// Where one run's engine lines go, and how deep it asked to see.
#[derive(Clone)]
pub(crate) struct RunLogSink {
    queue: Option<Arc<Mutex<EngineLogQueue>>>,
    observer: Option<EngineLogObserver>,
    verbose: bool,
}

impl RunLogSink {
    /// A native run's sink: the queue the controller drains each frame.
    pub(crate) fn queued(queue: Arc<Mutex<EngineLogQueue>>, verbose: bool) -> Self {
        Self {
            queue: Some(queue),
            observer: None,
            verbose,
        }
    }

    /// A browser worker run's sink: one posted message per line.
    ///
    /// Compiled only where something can be observing — the worker image, and
    /// the tests that stand in for it — because a desktop build that carried a
    /// constructor nothing calls is a shipping twin of one that does.
    #[cfg(any(all(target_arch = "wasm32", feature = "browser-worker"), test))]
    pub(crate) fn observed(observer: EngineLogObserver, verbose: bool) -> Self {
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
pub(crate) fn lock_queue(
    queue: &Arc<Mutex<EngineLogQueue>>,
) -> std::sync::MutexGuard<'_, EngineLogQueue> {
    match queue.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

thread_local! {
    /// The run executing on this thread, if one is.
    static RUN_SINK: RefCell<Option<RunLogSink>> = const { RefCell::new(None) };
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
pub(crate) fn install(sink: RunLogSink) -> RunSinkGuard {
    let verbose = sink.verbose;
    let previous = RUN_SINK
        .try_with(|cell| cell.borrow_mut().replace(sink))
        .unwrap_or(None);
    if verbose {
        VERBOSE_RUNS.fetch_add(1, Ordering::SeqCst);
        apply_max_level();
    }
    RunSinkGuard { previous, verbose }
}

/// What removes a run's sink, whatever happens to the run.
pub(crate) struct RunSinkGuard {
    previous: Option<RunLogSink>,
    verbose: bool,
}

impl Drop for RunSinkGuard {
    fn drop(&mut self) {
        let sink = RUN_SINK
            .try_with(|cell| std::mem::replace(&mut *cell.borrow_mut(), self.previous.take()))
            .unwrap_or(None);
        if let Some(sink) = sink {
            sink.seal();
        }
        if self.verbose {
            VERBOSE_RUNS.fetch_sub(1, Ordering::SeqCst);
            apply_max_level();
        }
    }
}

/// Whether the target names the engine rather than the application.
fn is_core_target(target: &str) -> bool {
    target == "rspice_core" || target.starts_with("rspice_core::")
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
pub(crate) fn admits(metadata: &log::Metadata<'_>) -> bool {
    if !is_core_target(metadata.target()) {
        return false;
    }
    let Some(severity) = severity_for_level(metadata.level()) else {
        return false;
    };
    RUN_SINK
        .try_with(|cell| {
            cell.borrow()
                .as_ref()
                .is_some_and(|sink| sink.admits(severity))
        })
        .unwrap_or(false)
}

/// Offer one record to this thread's run, if it has one that wants it.
pub(crate) fn offer(record: &log::Record<'_>) {
    if !is_core_target(record.target()) {
        return;
    }
    let Some(severity) = severity_for_level(record.level()) else {
        return;
    };
    let _ = RUN_SINK.try_with(|cell| {
        let sink = cell.borrow();
        let Some(sink) = sink.as_ref() else {
            return;
        };
        if !sink.admits(severity) {
            return;
        }
        sink.deliver(EngineLogLine {
            severity,
            message: record.args().to_string(),
        });
    });
}

/// Write one run's drained lines into the Console.
///
/// The engine's text is carried unchanged: the `ENG` tag is what distinguishes
/// an engine line from the studio's own, so nothing is prefixed to say it
/// again.
pub(crate) fn publish(lines: Vec<EngineLogLine>, buffer: &mut LogBuffer) {
    for line in lines {
        buffer.log(line.severity, LogSource::Engine, line.message, None);
    }
}

/// The process log level the stderr filter and the Console sink together need.
///
/// `Info`, so an engine receipt reaches a sink at all, and `Debug` only while
/// some run has asked for verbose. That is what keeps an ordinary run's cost at
/// zero: the `log!` macro compares against this level before it builds a
/// record, so the engine's `debug!` sites are not merely filtered out — they
/// are not entered.
pub(crate) fn admitted_max_level(
    stderr: log::LevelFilter,
    verbose_runs: usize,
) -> log::LevelFilter {
    let console = if verbose_runs > 0 {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    stderr.max(console)
}

/// Record what the installed logger prints to stderr, and apply the result.
pub(crate) fn note_stderr_level(level: log::LevelFilter) {
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

/// The desktop application's logger: stderr as before, plus the run's Console.
///
/// Two readers, and they want different things. Stderr wants what
/// `workbench::logging::native_default_filter` has always given it — warnings
/// and errors, no routine startup chatter, no graphics-backend probes. The
/// Console wants the *running analysis's* own account of itself, which stderr
/// never showed a windowed application's reader anyway.
///
/// Neither can change what the other sees: the stderr decision is the inner
/// logger's, unchanged, and the Console's is the run sink's.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct StudioLogger {
    inner: env_logger::Logger,
}

#[cfg(not(target_arch = "wasm32"))]
impl StudioLogger {
    /// Wrap the logger `env_logger` built, whatever filter it was built from.
    pub(crate) fn new(inner: env_logger::Logger) -> Self {
        Self { inner }
    }

    /// Whether this record reaches stderr.
    ///
    /// The inner logger's own answer, named so the decision has one reader and
    /// one test: adding the Console must not move a single line of what the
    /// terminal prints.
    pub(crate) fn prints_to_stderr(&self, record: &log::Record<'_>) -> bool {
        self.inner.matches(record)
    }

    /// The level the stderr filter alone requires.
    pub(crate) fn stderr_level(&self) -> log::LevelFilter {
        self.inner.filter()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl log::Log for StudioLogger {
    /// True when either reader wants the record.
    ///
    /// The second half is what makes `log_enabled!` honest inside the engine: a
    /// core `debug!` is enabled exactly when some verbose run on this thread
    /// would capture it, and on every ordinary run it is not.
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        self.inner.enabled(metadata) || admits(metadata)
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.prints_to_stderr(record) {
            self.inner.log(record);
        }
        offer(record);
    }

    fn flush(&self) {
        self.inner.flush();
    }
}

/// Install [`StudioLogger`] as the process logger.
///
/// The process log level is the sink's business rather than the stderr
/// filter's alone — an engine receipt has to be admitted even when stderr is
/// quiet — so it is taken from [`admitted_max_level`] and raised again for the
/// duration of a verbose run.
///
/// A logger installs once per process. A second call leaves the first in place
/// rather than failing a launch over it.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn install_studio_logger(inner: env_logger::Logger) {
    let logger = StudioLogger::new(inner);
    let stderr_level = logger.stderr_level();
    if log::set_boxed_logger(Box::new(logger)).is_ok() {
        note_stderr_level(stderr_level);
    }
}

/// The one [`StudioLogger`] the library test binary installs.
///
/// `log::set_boxed_logger` succeeds once per process and the library tests are
/// one process, so every test that needs the real `log!` path — here and in the
/// layers that execute a run — comes through this. Each such test scopes its
/// assertions to its own run sink, which is per-thread, so tests running in
/// parallel cannot see each other's lines.
///
/// Its filter is written out rather than read from `RSPICE_LOG`, because the
/// environment tests in `workbench::logging` mutate that variable: whichever
/// test installed the logger first would otherwise decide what every other
/// test's stderr half does.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn studio_logger_for_tests() -> &'static StudioLogger {
    static LOGGER: std::sync::OnceLock<&'static StudioLogger> = std::sync::OnceLock::new();
    LOGGER.get_or_init(|| {
        let mut builder = env_logger::Builder::new();
        builder.parse_filters(TEST_STDERR_FILTER);
        let logger: &'static StudioLogger = Box::leak(Box::new(StudioLogger::new(builder.build())));
        let stderr_level = logger.stderr_level();
        // `set_logger` over the leaked reference, not `set_boxed_logger` over a
        // second copy: the tests assert against the very logger the process
        // installed.
        let _ = log::set_logger(logger);
        note_stderr_level(stderr_level);
        logger
    })
}

/// The filter the test logger's stderr half is held to.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) const TEST_STDERR_FILTER: &str = "warn,rspice_core=warn";

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
    #[cfg(not(target_arch = "wasm32"))]
    use log::{Level, Record};

    fn core_record(level: log::Level, message: &str) -> EngineLogLine {
        let severity = severity_for_level(level).expect("the level reaches the Console");
        EngineLogLine {
            severity,
            message: message.to_owned(),
        }
    }

    /// Offer a record the way the installed logger does, without needing the
    /// process-wide logger: `offer` *is* the capture rule.
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
    fn a_captured_info_line_becomes_an_engine_row_in_the_console() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            offer_core(
                log::Level::Info,
                "Transient noise: 2 device noise source(s)",
            );
        }

        let lines = lock_queue(&queue).drain();
        assert_eq!(
            lines,
            vec![core_record(
                log::Level::Info,
                "Transient noise: 2 device noise source(s)"
            )]
        );

        let mut buffer = LogBuffer::default();
        publish(lines, &mut buffer);
        let rows: Vec<_> = buffer.entries_by_source(LogSource::Engine).collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].severity, LogSeverity::Info);
        assert_eq!(
            rows[0].message, "Transient noise: 2 device noise source(s)",
            "the engine's text is carried unchanged"
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
        for target in ["rspice_ui", "rspice_veriloga", "rspice_corella", "wgpu_hal"] {
            offer(
                &log::Record::builder()
                    .level(log::Level::Info)
                    .target(target)
                    .args(format_args!("not the engine"))
                    .build(),
            );
        }
        assert!(lock_queue(&queue).drain().is_empty());
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_core_info_line_logged_on_the_run_thread_reaches_the_console_as_an_engine_row() {
        studio_logger_for_tests();
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _run = install(RunLogSink::queued(Arc::clone(&queue), false));
            log::info!(
                target: "rspice_core::engine::transient",
                "Transient noise: {} device noise source(s) injected from seed {}",
                3,
                7
            );
        }

        let mut buffer = LogBuffer::default();
        crate::diagnostics::engine_log::publish(lock_queue(&queue).drain(), &mut buffer);
        let rows: Vec<_> = buffer.entries_by_source(LogSource::Engine).collect();
        assert_eq!(rows.len(), 1, "one engine row: {rows:?}");
        assert_eq!(rows[0].severity, LogSeverity::Info);
        assert_eq!(
            rows[0].message,
            "Transient noise: 3 device noise source(s) injected from seed 7"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_core_line_logged_off_the_run_thread_is_not_captured() {
        studio_logger_for_tests();
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        let held = Arc::clone(&queue);
        {
            let _run = install(RunLogSink::queued(queue, true));
            std::thread::spawn(|| {
                log::info!(target: "rspice_core", "logged by a thread running no analysis");
                log::error!(target: "rspice_core", "and an error from the same thread");
            })
            .join()
            .expect("the logging thread finishes");
        }
        assert!(
            lock_queue(&held).drain().is_empty(),
            "only the run's own thread feeds its Console log"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn two_concurrent_runs_never_see_each_others_engine_lines() {
        studio_logger_for_tests();
        let first = Arc::new(Mutex::new(EngineLogQueue::default()));
        let second = Arc::new(Mutex::new(EngineLogQueue::default()));
        let both_installed = Arc::new(std::sync::Barrier::new(2));
        let both_logged = Arc::new(std::sync::Barrier::new(2));

        let threads: Vec<_> = [("first run", &first), ("second run", &second)]
            .into_iter()
            .map(|(name, queue)| {
                let queue = Arc::clone(queue);
                let both_installed = Arc::clone(&both_installed);
                let both_logged = Arc::clone(&both_logged);
                std::thread::spawn(move || {
                    let _run = install(RunLogSink::queued(queue, true));
                    both_installed.wait();
                    log::debug!(target: "rspice_core::engine::pss", "{name}");
                    both_logged.wait();
                })
            })
            .collect();
        for thread in threads {
            thread.join().expect("both runs finish");
        }

        let messages = |queue: &Arc<Mutex<EngineLogQueue>>| {
            lock_queue(queue)
                .drain()
                .into_iter()
                .map(|line| line.message)
                .collect::<Vec<_>>()
        };
        assert_eq!(messages(&first), vec!["first run".to_owned()]);
        assert_eq!(messages(&second), vec!["second run".to_owned()]);
    }

    /// The engine's `debug!` sites cost an ordinary run nothing.
    ///
    /// Asserted on the installed logger's own answer rather than on the process
    /// level, which another test's verbose run may legitimately have raised:
    /// the answer below is per-thread and therefore exact whatever else is
    /// running.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn an_ordinary_run_never_enables_core_debug_formatting() {
        let logger = studio_logger_for_tests();
        let core_debug = log::Metadata::builder()
            .level(Level::Debug)
            .target("rspice_core::analysis::harmonic_balance")
            .build();

        assert!(
            !log::Log::enabled(logger, &core_debug),
            "no run is installed on this thread"
        );

        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _run = install(RunLogSink::queued(Arc::clone(&queue), false));
            assert!(
                !log::Log::enabled(logger, &core_debug),
                "an ordinary run does not ask for the solver trace"
            );
            log::debug!(target: "rspice_core::analysis::harmonic_balance", "HB Newton step");
            assert!(lock_queue(&queue).drain().is_empty());
        }
        {
            let _run = install(RunLogSink::queued(Arc::clone(&queue), true));
            assert!(
                log::Log::enabled(logger, &core_debug),
                "a verbose run is exactly when the trace is worth formatting"
            );
        }
    }

    /// Adding the Console sink moved no line of what the terminal prints.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_stderr_filter_is_unchanged_by_the_console_sink() {
        let logger = studio_logger_for_tests();
        let reference = {
            let mut builder = env_logger::Builder::new();
            builder.parse_filters(TEST_STDERR_FILTER);
            builder.build()
        };

        let cases = [
            ("rspice_core", Level::Info),
            ("rspice_core::engine::transient", Level::Debug),
            ("rspice_core", Level::Warn),
            ("rspice_ui", Level::Info),
            ("eframe::native", Level::Warn),
            ("wgpu_hal::vulkan::instance", Level::Warn),
        ];
        for (target, level) in cases {
            let record = Record::builder()
                .level(level)
                .target(target)
                .args(format_args!("record under test"))
                .build();
            assert_eq!(
                logger.prints_to_stderr(&record),
                reference.matches(&record),
                "{target} at {level} changed what stderr prints"
            );
        }

        // And a core Info record a sink *does* capture is still not printed,
        // because the filter above says warn.
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        let _run = install(RunLogSink::queued(Arc::clone(&queue), true));
        let receipt = Record::builder()
            .level(Level::Info)
            .target("rspice_core::engine::transient")
            .args(format_args!("Transient noise: 1 device noise source(s)"))
            .build();
        assert!(!logger.prints_to_stderr(&receipt));
        log::Log::log(logger, &receipt);
        assert_eq!(
            lock_queue(&queue).drain().len(),
            1,
            "the Console captured the record stderr refused"
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
