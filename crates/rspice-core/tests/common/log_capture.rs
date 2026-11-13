//! The one `log` logger a test binary installs, shared by every file in it.
//!
//! `log` accepts a single logger per process, and one test binary runs the
//! tests of many files at once, so no file installs its own. A capture keeps
//! only the lines logged on its own thread: a test running beside it never
//! reaches it, whichever file that test is in.

use std::cell::RefCell;
use std::sync::{Mutex, Once, PoisonError};

thread_local! {
    /// The level this thread captures at and the lines it has kept so far.
    static CAPTURED: RefCell<Option<(log::LevelFilter, Vec<String>)>> =
        const { RefCell::new(None) };
}

struct ThreadCapture;

impl log::Log for ThreadCapture {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        CAPTURED.with(|captured| {
            captured
                .borrow()
                .as_ref()
                .is_some_and(|(level, _)| metadata.level() <= *level)
        })
    }

    fn log(&self, record: &log::Record<'_>) {
        CAPTURED.with(|captured| {
            if let Some((level, lines)) = captured.borrow_mut().as_mut()
                && record.level() <= *level
            {
                lines.push(record.args().to_string());
            }
        });
    }

    fn flush(&self) {}
}

static LOGGER: ThreadCapture = ThreadCapture;
static INSTALL: Once = Once::new();
/// Serializes raising the process-wide level, so two captures that start at
/// once cannot lower it under each other.
static MAX_LEVEL: Mutex<()> = Mutex::new(());

/// Run `body` with the lines it logs on this thread at `level` or above.
pub fn lines_logged<T>(level: log::LevelFilter, body: impl FnOnce() -> T) -> (T, Vec<String>) {
    INSTALL.call_once(|| log::set_logger(&LOGGER).expect("the capture logger installs"));
    {
        let _raising = MAX_LEVEL.lock().unwrap_or_else(PoisonError::into_inner);
        if log::max_level() < level {
            log::set_max_level(level);
        }
    }
    CAPTURED.with(|captured| *captured.borrow_mut() = Some((level, Vec::new())));
    let value = body();
    let lines = CAPTURED
        .with(|captured| captured.borrow_mut().take())
        .map(|(_, lines)| lines)
        .unwrap_or_default();
    (value, lines)
}
