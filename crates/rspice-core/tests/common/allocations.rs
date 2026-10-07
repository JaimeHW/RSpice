//! Opt-in, per-thread allocation accounting for integration-test binaries.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static ALLOCATION_BYTES: Cell<Option<usize>> = const { Cell::new(None) };
}

struct CountingAllocator;

fn count(bytes: usize) {
    let _ = ALLOCATION_BYTES.try_with(|counter| {
        if let Some(total) = counter.get() {
            counter.set(Some(total.saturating_add(bytes)));
        }
    });
}

// SAFETY: Every operation is forwarded unchanged to System; accounting uses
// only nonallocating thread-local Cell operations.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count(size);
        unsafe { System.realloc(ptr, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// Count requested bytes, including reallocations, only during this operation.
pub fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            ALLOCATION_BYTES.with(|counter| counter.set(None));
        }
    }
    ALLOCATION_BYTES.with(|counter| {
        assert!(
            counter.get().is_none(),
            "allocation measurements cannot nest"
        );
        counter.set(Some(0));
    });
    let _reset = Reset;
    let result = operation();
    let bytes = ALLOCATION_BYTES.with(|counter| counter.get().unwrap());
    (result, bytes)
}
