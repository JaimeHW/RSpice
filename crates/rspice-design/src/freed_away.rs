//! Large values let go of on the editor's thread, freed on a thread of their
//! own.
//!
//! The index of a large deck holds thousands of tables, and freeing them
//! takes tens of milliseconds. A newer index replaces it in the slot that
//! keeps it, while a frame's cache, an inspector's counts or a hover may
//! still hold it; whichever lets go last frees it, and a frame that does
//! stalls for as long. So a type that holds such contents hands them, in its
//! `Drop`, to [`free_away`]: whoever lets go of the value last, the contents
//! are freed on a thread kept for freeing, never on the thread that let go.
//! Where there are no threads they are freed in place.
//!
//! Freeing on another thread is not enough on its own. The system heap
//! takes freed memory back under the lock every allocation takes, and
//! taking back a hundred megabytes of tables at once holds that lock for
//! tens of milliseconds, while the frame waits on its next allocation. So
//! an index's tables are let go through [`free_away_with`]: page by page,
//! each table's whole pages handed back to the system before the heap frees
//! it, outside the heap's lock, and with a pause between pages so the
//! freeing never holds the heap for long.

/// Hand what `value` holds to the freeing thread, leaving `value` empty:
/// what a large type's `Drop` calls. On the freeing thread itself, and where
/// no thread could be started, nothing is taken, and the contents are freed
/// in place as the value's own fields drop.
pub fn free_away<T: Default + Send + 'static>(value: &mut T) {
    #[cfg(not(target_arch = "wasm32"))]
    thread::free_away(value, None);
    #[cfg(target_arch = "wasm32")]
    let _ = value;
}

/// [`free_away`], with `let_go` run on the contents on the freeing thread
/// before they are freed: it lets go of their large tables through the
/// [`Release`] it is handed. A value let go of on the freeing thread itself,
/// as the last share of another value's contents, is let go of there.
pub fn free_away_with<T: Default + Send + 'static>(
    value: &mut T,
    let_go: fn(&mut T, &mut Release),
) {
    #[cfg(not(target_arch = "wasm32"))]
    thread::free_away(value, Some(let_go));
    #[cfg(target_arch = "wasm32")]
    let _ = (value, let_go);
}

/// How the freeing thread lets go of a large value's tables: each table's
/// whole pages handed back to the system before the heap frees the table,
/// and a pause after each half millisecond of freeing.
pub struct Release {
    since: std::time::Instant,
}

/// How long the freeing thread frees before it pauses, and for how long.
const FREEING_BEFORE_A_PAUSE: std::time::Duration = std::time::Duration::from_micros(500);
const PAUSE: std::time::Duration = std::time::Duration::from_millis(1);

impl Release {
    #[cfg(not(target_arch = "wasm32"))]
    fn new() -> Self {
        Self {
            since: std::time::Instant::now(),
        }
    }
}

impl rspice_core_netlist::netlist::PageRelease for Release {
    fn release(&mut self, bytes: &mut [std::mem::MaybeUninit<u8>]) {
        hand_back(bytes);
    }

    fn page_freed(&mut self) {
        if self.since.elapsed() >= FREEING_BEFORE_A_PAUSE {
            std::thread::sleep(PAUSE);
            self.since = std::time::Instant::now();
        }
    }
}

/// Hand the whole pages of `bytes` back to the system, keeping them
/// committed: what they held is gone, and the heap that frees them next has
/// no page of memory left to give back while it holds its lock.
#[cfg(windows)]
fn hand_back(bytes: &mut [std::mem::MaybeUninit<u8>]) {
    const PAGE: usize = 4096;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn DiscardVirtualMemory(address: *mut std::ffi::c_void, size: usize) -> u32;
    }
    let start = bytes.as_mut_ptr() as usize;
    let first = start.next_multiple_of(PAGE);
    let end = (start + bytes.len()) / PAGE * PAGE;
    if end > first {
        // SAFETY: the pages lie inside `bytes`, which the caller holds
        // mutably and does not read again; discarding them keeps them
        // committed and only makes what they held undefined.
        unsafe {
            DiscardVirtualMemory(first as *mut std::ffi::c_void, end - first);
        }
    }
}

#[cfg(not(windows))]
fn hand_back(_: &mut [std::mem::MaybeUninit<u8>]) {}

#[cfg(not(target_arch = "wasm32"))]
mod thread {
    use std::cell::Cell;
    use std::sync::{OnceLock, mpsc};

    type Freed = Box<dyn FnOnce() + Send>;

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
                        for free in receiver {
                            free();
                        }
                    })
                    .ok()
                    .map(|_| sender)
            })
            .as_ref()
    }

    pub(super) fn free_away<T: Default + Send + 'static>(
        value: &mut T,
        let_go: Option<fn(&mut T, &mut super::Release)>,
    ) {
        if FREEING.get() {
            if let Some(let_go) = let_go {
                let_go(value, &mut super::Release::new());
            }
            return;
        }
        let Some(queue) = queue() else {
            return;
        };
        let mut contents = std::mem::take(value);
        let free: Freed = Box::new(move || {
            if let Some(let_go) = let_go {
                let_go(&mut contents, &mut super::Release::new());
            }
            drop(contents);
        });
        // The thread ends only with the process; were it gone, the contents
        // come back with the error and are freed here, whole.
        if let Err(mpsc::SendError(free)) = queue.send(free) {
            drop(free);
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use super::{free_away, free_away_with};

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

    /// Tables, each saying as it is freed the thread it was freed on.
    #[derive(Default)]
    struct Tables(Vec<Contents>);

    /// A value whose tables are let go of one at a time, as an index's are.
    struct Index(Tables);

    impl Drop for Index {
        fn drop(&mut self) {
            free_away_with(&mut self.0, |tables, release| {
                while let Some(table) = tables.0.pop() {
                    drop(table);
                    rspice_core_netlist::netlist::PageRelease::page_freed(release);
                }
            });
        }
    }

    /// Tables let go of one at a time are every one freed, on the freeing
    /// thread, and each once.
    #[test]
    fn tables_let_go_of_one_at_a_time_are_all_freed_on_the_freeing_thread() {
        let (said, freed) = mpsc::channel();
        let tables = (0..100).map(|_| Contents(Some(said.clone()))).collect();
        drop(said);
        drop(Index(Tables(tables)));
        let threads = (0..100)
            .map(|_| {
                freed
                    .recv_timeout(Duration::from_secs(10))
                    .expect("every table is freed")
            })
            .collect::<Vec<_>>();
        assert!(
            threads
                .iter()
                .all(|thread| thread.as_deref() == Some("freeing"))
        );
        assert!(
            freed.recv_timeout(Duration::from_millis(50)).is_err(),
            "each table is freed once"
        );
    }

    /// A table whose pages are handed back is still memory the heap holds:
    /// written again, it reads as written, and it frees.
    #[test]
    fn a_table_handed_back_is_memory_that_can_be_written_and_freed() {
        let mut table = Vec::<u8>::with_capacity(256 * 1024);
        super::hand_back(table.spare_capacity_mut());
        table.resize(256 * 1024, 7);
        assert!(table.iter().all(|byte| *byte == 7));
        drop(table);
    }
}
