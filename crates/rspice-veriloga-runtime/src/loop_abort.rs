//! The run's abort hook, as a Verilog-A loop sees it.
//!
//! A `while`, `for` or `repeat` in an analog block runs inside one device
//! evaluation, where the engine's own polls cannot reach it: a loop whose
//! condition never clears would hold the evaluation, and the run, forever.
//! Every runtime that executes such a loop (the interpreter, the native
//! backends) therefore counts its trips and, once every [`POLL_INTERVAL`] of
//! them, asks [`aborted`] whether the run has been told to stop. That is the
//! whole guard: a loop that terminates runs to completion however many trips it
//! takes, and one that does not is ended by the same Stop or time limit that
//! ends any other run.
//!
//! The hook is the run's own abort signal, installed for the thread that
//! evaluates devices by [`with_hook`] for exactly as long as the run borrows
//! it. A thread with no hook installed never aborts a loop.

use std::cell::Cell;
use std::ptr::NonNull;

/// Trips of one loop between two polls of the hook.
///
/// A poll is a thread-local read and one call through the hook, tens of
/// nanoseconds; the tightest loop body is a few. At this interval the poll is
/// below a part in ten thousand of the loop's time, and a body that takes a
/// microsecond still sees a Stop within a tenth of a second.
pub const POLL_INTERVAL: u32 = 1 << 16;

/// The hook with its borrow erased; see [`with_hook`] for why that is sound.
type ErasedHook = NonNull<dyn Fn() -> bool + Sync + 'static>;

thread_local! {
    static HOOK: Cell<Option<ErasedHook>> = const { Cell::new(None) };
}

/// Run `body` with `hook` as this thread's abort hook, then restore whatever
/// was installed before, also when `body` unwinds.
///
/// The hook answers `true` once the run should stop. Scopes nest: the
/// innermost one is the hook a loop polls.
pub fn with_hook<R>(hook: &(dyn Fn() -> bool + Sync), body: impl FnOnce() -> R) -> R {
    struct Restore(Option<ErasedHook>);
    impl Drop for Restore {
        fn drop(&mut self) {
            let previous = self.0;
            let _ = HOOK.try_with(|slot| slot.set(previous));
        }
    }
    let borrowed: NonNull<dyn Fn() -> bool + Sync + '_> = NonNull::from(hook);
    // SAFETY: only the borrow's lifetime is erased; the pointer's layout is
    // unchanged. The erased pointer is reachable through `HOOK` only until
    // `Restore` drops at the end of this call, on return or unwind, and `hook`
    // stays borrowed for that whole span.
    let erased = unsafe {
        std::mem::transmute::<NonNull<dyn Fn() -> bool + Sync + '_>, ErasedHook>(borrowed)
    };
    let previous = HOOK.with(|slot| slot.replace(Some(erased)));
    let _restore = Restore(previous);
    body()
}

/// Whether this thread's run has been told to stop. `false` with no hook
/// installed.
pub fn aborted() -> bool {
    let Some(hook) = HOOK.try_with(Cell::get).ok().flatten() else {
        return false;
    };
    // SAFETY: a hook in `HOOK` is one `with_hook` installed on this thread and
    // has not returned from, so the borrow behind it is live.
    let hook = unsafe { hook.as_ref() };
    hook()
}

#[cfg(test)]
mod tests {
    use super::{aborted, with_hook};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn a_thread_without_a_hook_never_aborts() {
        assert!(!aborted());
    }

    #[test]
    fn the_innermost_hook_answers_and_the_outer_one_returns_after_it() {
        let outer = AtomicBool::new(true);
        let inner = AtomicBool::new(false);
        with_hook(&|| outer.load(Ordering::Relaxed), || {
            assert!(aborted());
            with_hook(&|| inner.load(Ordering::Relaxed), || {
                assert!(!aborted());
                inner.store(true, Ordering::Relaxed);
                assert!(aborted());
            });
            outer.store(false, Ordering::Relaxed);
            assert!(!aborted());
        });
        assert!(!aborted());
    }

    #[test]
    fn an_unwinding_body_restores_the_previous_hook() {
        let stop = AtomicBool::new(true);
        let unwound = std::panic::catch_unwind(|| {
            with_hook(&|| stop.load(Ordering::Relaxed), || panic!("unwind"));
        });
        assert!(unwound.is_err());
        assert!(!aborted());
    }
}
