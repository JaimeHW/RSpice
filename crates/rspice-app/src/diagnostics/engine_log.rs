//! Engine log publication into the Console and desktop logger installation.

use super::{LogBuffer, LogSource};
use rspice_simulation::engine_log::EngineLogLine;
#[cfg(not(target_arch = "wasm32"))]
use rspice_simulation::engine_log::{admits, note_stderr_level, offer};

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
/// quiet — so it is taken from [`note_stderr_level`] and raised again for the
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::LogSeverity;
    #[cfg(not(target_arch = "wasm32"))]
    use log::{Level, Record};
    use rspice_simulation::engine_log::{EngineLogQueue, RunLogSink, install, lock_queue, offer};
    use std::sync::{Arc, Mutex};

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
            vec![EngineLogLine {
                severity: LogSeverity::Info,
                message: "Transient noise: 2 device noise source(s)".to_owned(),
            }]
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
}
