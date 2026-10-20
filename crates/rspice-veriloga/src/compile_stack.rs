//! The stack the compiler's recursive phases run on.
//!
//! Parsing, semantic analysis, lowering and code generation recurse over the
//! syntax tree. The parser bounds that tree's depth
//! ([`crate::parser::MAX_STATEMENT_NESTING`] and its neighbours), but a tree at
//! those bounds still needs more stack than a caller's thread may have: a
//! spawned thread gets 2 MiB, a Windows main thread 1 MiB, and unoptimized
//! frames are several times larger than optimized ones. So the compiler runs
//! those phases on a thread of its own, sized for the bounds in every profile,
//! whatever thread called it.

use std::cell::Cell;
use std::sync::Mutex;

/// Address space reserved for a compile thread's stack. Pages are committed
/// only as they are touched, so a shallow model costs no more than it did on
/// the caller's stack. The built-in generator runs the same phases on the same
/// reservation.
const COMPILE_STACK_BYTES: usize = 256 * 1024 * 1024;

thread_local! {
    static ON_COMPILE_STACK: Cell<bool> = const { Cell::new(false) };
}

/// Run `work` on a compile thread, or inline when already on one.
///
/// A panic in `work` resumes on the calling thread. Where threads cannot be
/// spawned (a browser build has none), `work` runs inline on the caller's
/// stack, which the parser's bounds are chosen to fit.
pub(crate) fn run<T: Send>(work: impl FnOnce() -> T + Send) -> T {
    if ON_COMPILE_STACK.get() {
        return work();
    }
    let work = Mutex::new(Some(work));
    let take = || {
        work.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
            .expect("compile work runs once")
    };
    std::thread::scope(|scope| {
        let spawned = std::thread::Builder::new()
            .name("rspice-veriloga-compile".to_owned())
            .stack_size(COMPILE_STACK_BYTES)
            .spawn_scoped(scope, || {
                ON_COMPILE_STACK.set(true);
                take()()
            });
        match spawned {
            Ok(thread) => thread
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic)),
            Err(_) => take()(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_runs_once_on_one_compile_thread_and_nested_work_stays_on_it() {
        let outer = run(|| {
            let outer = std::thread::current().id();
            let inner = run(|| std::thread::current().id());
            (outer, inner)
        });
        assert_eq!(outer.0, outer.1);
        assert_ne!(outer.0, std::thread::current().id());
    }

    #[test]
    fn a_panic_in_the_work_resumes_on_the_caller() {
        let caught = std::panic::catch_unwind(|| run(|| panic!("compile work failed")));
        let message = caught.expect_err("the panic must reach the caller");
        assert_eq!(message.downcast_ref::<&str>(), Some(&"compile work failed"));
    }
}
