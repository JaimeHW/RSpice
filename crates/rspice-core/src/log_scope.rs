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

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// What a host keeps on the thread executing one run, and the point of the
/// run that thread is solving, when that is one of several.
#[derive(Clone)]
pub struct LogScope {
    owner: Arc<dyn Any + Send + Sync>,
    point: Option<Arc<str>>,
    /// Lines the run states once however often it reaches them, shared by
    /// every thread the run hands work to.
    said_once: Arc<Mutex<HashSet<String>>>,
}

impl LogScope {
    pub fn new(owner: Arc<dyn Any + Send + Sync>) -> Self {
        Self {
            owner,
            point: None,
            said_once: Arc::default(),
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
pub(crate) fn enter_point(point: impl FnOnce() -> String) -> Option<LogScopeGuard> {
    capture().map(|scope| enter(Some(scope.at_point(point()))))
}

/// Whether this thread's run has yet to say `line`, which it then has.
///
/// For a fact the engine reaches once per elaboration that a run may well
/// elaborate several times over — before its operating point, per sweep point,
/// per Monte Carlo sample — and that is worth reading once. A thread outside
/// any run remembers nothing, so there every call is the first.
pub(crate) fn first_saying(line: &str) -> bool {
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
    fn a_run_says_a_line_once_on_every_thread_it_hands_work_to() {
        assert!(first_saying("outside"));
        assert!(first_saying("outside"), "outside a run nothing is remembered");
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
        assert!(first_saying("Circuit: 2 nodes"), "the next run starts with nothing said");
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
        assert_eq!(capture().and_then(|scope| scope.point().map(str::to_owned)), None);
    }
}
