//! Large values let go of on the editor's thread, freed on a thread of their
//! own.
//!
//! The index of a large deck holds millions of small allocations, and
//! freeing them takes hundreds of milliseconds. A newer index replaces it in
//! the slot that keeps it, while a frame's cache, an inspector's counts or a
//! hover may still hold it; whichever lets go last frees it, and a frame
//! that does stalls for as long. So a type that holds such contents hands
//! them, in its `Drop`, to [`free_away`]: whoever lets go of the value last,
//! the contents are freed on a thread kept for freeing, never on the thread
//! that let go. Where there are no threads they are freed in place.

/// Hand what `value` holds to the freeing thread, leaving `value` empty:
/// what a large type's `Drop` calls. On the freeing thread itself, and where
/// no thread could be started, nothing is taken, and the contents are freed
/// in place as the value's own fields drop.
pub fn free_away<T: Default + Send + 'static>(value: &mut T) {
    #[cfg(not(target_arch = "wasm32"))]
    thread::free_away(value);
    #[cfg(target_arch = "wasm32")]
    let _ = value;
}

#[cfg(not(target_arch = "wasm32"))]
mod thread {
    use std::cell::Cell;
    use std::sync::{OnceLock, mpsc};

    type Freed = Box<dyn Send>;

    thread_local! {
        /// Whether this thread is the freeing thread.
        static FREEING: Cell<bool> = const { Cell::new(false) };
    }

    /// The freeing thread's queue, started the first time a value is freed;
    /// `None` when no thread could be started.
    fn queue() -> Option<&'static mpsc::Sender<Freed>> {
        static QUEUE: OnceLock<Option<mpsc::Sender<Freed>>> = OnceLock::new();
        QUEUE
            .get_or_init(|| {
                let (sender, receiver) = mpsc::channel::<Freed>();
                std::thread::Builder::new()
                    .name("freeing".to_owned())
                    .spawn(move || {
                        FREEING.set(true);
                        for freed in receiver {
                            drop(freed);
                        }
                    })
                    .ok()
                    .map(|_| sender)
            })
            .as_ref()
    }

    pub(super) fn free_away<T: Default + Send + 'static>(value: &mut T) {
        if FREEING.get() {
            return;
        }
        let Some(queue) = queue() else {
            return;
        };
        let freed: Freed = Box::new(std::mem::take(value));
        // The thread ends only with the process; were it gone, the contents
        // come back with the error and are freed here.
        if let Err(mpsc::SendError(freed)) = queue.send(freed) {
            drop(freed);
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use super::free_away;

    /// Says, as it is freed, the thread it was freed on.
    #[derive(Default)]
    struct Contents(Option<mpsc::Sender<Option<String>>>);

    impl Drop for Contents {
        fn drop(&mut self) {
            if let Some(said) = self.0.take() {
                let thread = std::thread::current().name().map(str::to_owned);
                let _ = said.send(thread);
            }
        }
    }

    /// A value whose contents are freed away, as an index's are.
    struct Large(Contents);

    impl Drop for Large {
        fn drop(&mut self) {
            free_away(&mut self.0);
        }
    }

    /// A value let go of here is freed whole on the freeing thread, and
    /// nothing of it is freed here.
    #[test]
    fn a_value_let_go_of_is_freed_on_the_freeing_thread() {
        let (said, freed) = mpsc::channel();
        drop(Large(Contents(Some(said))));
        let thread = freed
            .recv_timeout(Duration::from_secs(10))
            .expect("the contents are freed");
        assert_eq!(thread.as_deref(), Some("freeing"));
        assert!(
            freed.recv_timeout(Duration::from_millis(50)).is_err(),
            "the contents are freed once"
        );
    }
}
