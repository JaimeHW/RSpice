//! The run a thread's log records belong to, carried onto the engine's own
//! worker threads.
//!
//! A host that shows a run its own log keeps something on the thread executing
//! that run and reads it back from its logger. What it keeps is the host's
//! business, so the engine holds it opaquely. What the engine does know is when
//! it hands part of a run to another thread — a Monte Carlo worker, a chunk of
//! a parallel frequency sweep — and there it captures the scope and enters it
//! on the worker. A record logged on a thread that never entered a scope
//! belongs to no run.
//!
//! The run's [`NoticeCollector`](crate::notice::NoticeCollector) travels in the
//! same scope, so a notice raised on a worker reaches the run that spawned it,
//! labelled with the same point as the worker's lines. The scope carries it
//! opaquely, as it carries the host's state: what a notice is belongs to the
//! notice module, which reads down into this one.
//!
//! A site that logs the warning stating a notice learns which line of the
//! run's log the host kept for it ([`warn_line!`]), and raises the notice with
//! that line, so the two are linked by the site and never by text.
//!
//! A host reads a record's target through [`canonical_target`], which names a
//! module of any `rspice_core_<name>` crate as `rspice_core`'s.

use std::any::Any;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// The engine's name for a log target: `rspice_core::rest` for a target in
/// any `rspice_core_<name>` crate, and every other target as it is.
///
/// A module keeps its path from the crate root whichever crate of the
/// engine's family holds it, so this is the name the module has in
/// `rspice_core`. Hosts route, filter and print engine records by it.
pub fn canonical_target(target: &str) -> Cow<'_, str> {
    let Some(rest) = target.strip_prefix("rspice_core_") else {
        return Cow::Borrowed(target);
    };
    let (name, path) = match rest.split_once("::") {
        Some((name, path)) => (name, Some(path)),
        None => (rest, None),
    };
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Cow::Borrowed(target);
    }
    match path {
        Some(path) => Cow::Owned(format!("rspice_core::{path}")),
        None => Cow::Borrowed("rspice_core"),
    }
}

/// What a host keeps on the thread executing one run, and the point of the
/// run that thread is solving, when that is one of several.
#[derive(Clone)]
pub struct LogScope {
    owner: Arc<dyn Any + Send + Sync>,
    point: Option<Arc<str>>,
    /// Lines the run states once however often it reaches them, shared by
    /// every thread the run hands work to.
    said_once: Arc<Mutex<HashSet<String>>>,
    /// Where the run records what it raised, shared the same way. Set and
    /// read only by the notice module's `LogScope::collecting` and
    /// `LogScope::notices`.
    pub(crate) notices: Option<Arc<dyn Any + Send + Sync>>,
}

impl LogScope {
    pub fn new(owner: Arc<dyn Any + Send + Sync>) -> Self {
        Self {
            owner,
            point: None,
            said_once: Arc::default(),
            notices: None,
        }
    }

    /// The host's own state for the run, to downcast to what it stored.
    pub fn owner(&self) -> &(dyn Any + Send + Sync) {
        &*self.owner
    }

    /// The point of the run being solved, such as `sample 12`, for a host to
    /// say which of a run's points a line came from.
    pub fn point(&self) -> Option<&str> {
        self.point.as_deref()
    }

    /// The same run, at one of its points.
    #[must_use]
    pub fn at_point(self, point: impl Into<Arc<str>>) -> Self {
        Self {
            point: Some(point.into()),
            ..self
        }
    }
}

thread_local! {
    static CURRENT: RefCell<Option<LogScope>> = const { RefCell::new(None) };
}

/// The scope this thread is executing in, to enter on a thread it hands work to.
pub fn capture() -> Option<LogScope> {
    CURRENT
        .try_with(|current| current.borrow().clone())
        .ok()
        .flatten()
}

/// Read this thread's scope in place, without cloning it.
pub(crate) fn with_current<R>(read: impl FnOnce(&LogScope) -> R) -> Option<R> {
    CURRENT
        .try_with(|current| current.borrow().as_ref().map(read))
        .ok()
        .flatten()
}

thread_local! {
    /// What the host said it kept for the record this thread is logging,
    /// until [`stating`] takes it.
    static LOGGED: Cell<Option<u64>> = const { Cell::new(None) };
}

/// The line of the run's log that states a notice: the one the host kept for
/// the warning the notice's site logged just before raising it, numbered as
/// the host numbers its lines.
///
/// Only [`stating`] makes one that names a line, around the one warning
/// [`warn_line!`] logs, so a notice is never handed a line some other site
/// logged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
#[doc(hidden)]
pub struct LogLine(Option<u64>);

impl LogLine {
    /// What a notice raised without a warning of its own carries.
    #[doc(hidden)]
    pub const NONE: Self = Self(None);

    /// The line, if the host kept one.
    pub(crate) fn line(self) -> Option<u64> {
        self.0
    }
}

/// Say which line of the run's log the host kept for the record this thread
/// is logging: `Some(line)`, numbered as the host numbers its lines, or
/// `None` for a record it withheld or will not let state a notice.
///
/// A host that keeps the run's log answers this from its logger, on the
/// logging thread, for the engine records it is offered. Only a warning the
/// engine logs through [`warn_line!`] reads the answer.
pub fn logged(line: Option<u64>) {
    let _ = LOGGED.try_with(|logged| logged.set(line));
}

/// Run `log`, which logs one warning, and return the line the host kept for
/// it.
///
/// Whatever the thread was told before is cleared first and taken after, so
/// the line is this warning's or none: none when the host withheld it, keeps
/// no log, or was never offered it.
#[doc(hidden)]
pub fn stating(log: impl FnOnce()) -> LogLine {
    let _ = LOGGED.try_with(|logged| logged.set(None));
    log();
    LogLine(LOGGED.try_with(Cell::take).ok().flatten())
}

/// `log::warn!`, returning the [`LogLine`] the host kept for that warning,
/// for the site to raise the notice the warning states with.
///
/// A macro and not a function, so the record keeps its own call site: the
/// host limits how many warnings each site shows in a run.
///
/// Exported so that every crate of the engine family logs through the one
/// definition; callers name it as `log_scope::warn_line!`.
#[macro_export]
#[doc(hidden)]
macro_rules! __rspice_warn_line {
    ($($arg:tt)+) => {
        $crate::log_scope::stating(|| ::log::warn!($($arg)+))
    };
}
#[doc(hidden)]
pub use __rspice_warn_line as warn_line;

/// Execute in `scope` on this thread until the guard drops.
///
/// The guard restores whatever scope was there before, on every exit path,
/// including a panic unwinding out of the solver. Entering `None` leaves the
/// thread outside any run for the guard's life.
#[must_use]
pub fn enter(scope: Option<LogScope>) -> LogScopeGuard {
    let previous = CURRENT
        .try_with(|current| std::mem::replace(&mut *current.borrow_mut(), scope))
        .ok()
        .flatten();
    LogScopeGuard {
        previous,
        _thread_bound: std::marker::PhantomData,
    }
}

/// What restores a thread's previous scope.
pub struct LogScopeGuard {
    previous: Option<LogScope>,
    // Dropping a guard restores thread-local state, so it must stay on its thread.
    _thread_bound: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl Drop for LogScopeGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        let _ = CURRENT.try_with(|current| *current.borrow_mut() = previous);
    }
}

/// Enter this thread's own scope again, at one point of its run.
///
/// For the code that solves one point of many, on whichever thread it runs: a
/// thread outside any run stays outside.
#[must_use]
#[doc(hidden)]
pub fn enter_point(point: impl FnOnce() -> String) -> Option<LogScopeGuard> {
    capture().map(|scope| enter(Some(scope.at_point(point()))))
}

/// Whether this thread's run has yet to say `line`, which it then has.
///
/// For a fact the engine reaches once per elaboration that a run may well
/// elaborate several times over — before its operating point, per sweep point,
/// per Monte Carlo sample — and that is worth reading once. A thread outside
/// any run remembers nothing, so there every call is the first.
#[doc(hidden)]
pub fn first_saying(line: &str) -> bool {
    let Some(scope) = capture() else {
        return true;
    };
    scope
        .said_once
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(line.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_family_crates_target_reads_as_rspice_cores() {
        for (target, canonical) in [
            (
                "rspice_core_netlist::netlist::parser",
                "rspice_core::netlist::parser",
            ),
            ("rspice_core_netlist::library", "rspice_core::library"),
            ("rspice_core_base::expr", "rspice_core::expr"),
            (
                "rspice_core_devices::device::mosfet::bsim4v8::temp",
                "rspice_core::device::mosfet::bsim4v8::temp",
            ),
            ("rspice_core_devices", "rspice_core"),
        ] {
            assert_eq!(canonical_target(target), canonical, "{target}");
        }
        for target in [
            "rspice_core",
            "rspice_core::engine::transient",
            "rspice_corella",
            "rspice_core_",
            "rspice_core_::engine",
            "rspice_veriloga::ir",
            "rspice_simulation::engine_services",
            "",
        ] {
            assert!(
                matches!(canonical_target(target), Cow::Borrowed(same) if same == target),
                "{target} is not a family crate's and stays as it is"
            );
        }
    }

    #[test]
    fn a_run_says_a_line_once_on_every_thread_it_hands_work_to() {
        assert!(first_saying("outside"));
        assert!(
            first_saying("outside"),
            "outside a run nothing is remembered"
        );
        let _run = enter(Some(LogScope::new(Arc::new("run"))));
        assert!(first_saying("Circuit: 2 nodes"));
        let captured = capture();
        std::thread::scope(|threads| {
            threads.spawn(|| {
                let _scope = enter(captured.clone().map(|run| run.at_point("sample 2")));
                assert!(!first_saying("Circuit: 2 nodes"));
                assert!(first_saying("Circuit: 3 nodes"));
            });
        });
        assert!(!first_saying("Circuit: 3 nodes"));
        let _next = enter(Some(LogScope::new(Arc::new("next run"))));
        assert!(
            first_saying("Circuit: 2 nodes"),
            "the next run starts with nothing said"
        );
    }

    fn owner_name(scope: &LogScope) -> Option<&'static str> {
        scope.owner().downcast_ref::<&'static str>().copied()
    }

    #[test]
    fn a_thread_starts_outside_any_run() {
        std::thread::spawn(|| assert!(capture().is_none()))
            .join()
            .expect("the thread finishes");
    }

    #[test]
    fn a_guard_restores_the_scope_it_displaced() {
        let _outer = enter(Some(LogScope::new(Arc::new("outer"))));
        {
            let _inner = enter(Some(LogScope::new(Arc::new("inner"))));
            assert_eq!(capture().as_ref().and_then(owner_name), Some("inner"));
            {
                let _none = enter(None);
                assert!(capture().is_none());
            }
            assert_eq!(capture().as_ref().and_then(owner_name), Some("inner"));
        }
        assert_eq!(capture().as_ref().and_then(owner_name), Some("outer"));
    }

    #[test]
    fn a_captured_scope_entered_on_a_worker_is_the_same_run_there() {
        let _run = enter(Some(LogScope::new(Arc::new("run"))));
        let captured = capture();
        std::thread::scope(|threads| {
            threads.spawn(|| {
                assert!(capture().is_none(), "a worker inherits nothing by itself");
                let _scope = enter(captured.clone());
                assert_eq!(capture().as_ref().and_then(owner_name), Some("run"));
            });
        });
    }

    #[test]
    fn a_point_labels_the_run_until_its_guard_drops() {
        assert!(enter_point(|| "sample 1".to_owned()).is_none());
        let _run = enter(Some(LogScope::new(Arc::new("run"))));
        {
            let _point = enter_point(|| "sample 3".to_owned());
            let scope = capture().expect("the run is still entered");
            assert_eq!(scope.point(), Some("sample 3"));
            assert_eq!(owner_name(&scope), Some("run"));
        }
        assert_eq!(
            capture().and_then(|scope| scope.point().map(str::to_owned)),
            None
        );
    }
}
