//! Bound allocation growth for long startup cards, without timing assumptions.
#![cfg(not(target_arch = "wasm32"))]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::fmt::Write;

thread_local! {
    static ALLOCATION_BYTES: Cell<Option<usize>> = const { Cell::new(None) };
}

struct CountingAllocator;

fn count(bytes: usize) {
    let _ = ALLOCATION_BYTES.try_with(|counter| {
        if let Some(total) = counter.get() {
            counter.set(Some(total + bytes));
        }
    });
}

// SAFETY: Every allocation operation is forwarded unchanged to System; the
// thread-local counter uses only nonallocating Cell operations.
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

fn source(count: usize, directive: &str, scoped: bool) -> String {
    let mut source = String::from("Long startup card\n");
    if scoped {
        source.push_str(".subckt cell p params: level=1\nRport p 0 1k\n");
    } else {
        source.push_str(".param level=1\n");
    }
    for index in 0..count {
        writeln!(source, "R{index:04} N{index:04} 0 1k").unwrap();
    }
    write!(source, "{directive}").unwrap();
    for index in 0..count {
        write!(source, " V(N{index:04})={{level}}").unwrap();
    }
    source.push('\n');
    if scoped {
        source.push_str(".ends\nX1 out cell\n");
    }
    source.push_str(".end\n");
    source
}

fn allocated(source: &str, count: usize) -> usize {
    ALLOCATION_BYTES.with(|counter| counter.set(Some(0)));
    let parsed = rspice_core::Netlist::parse(source);
    let bytes = ALLOCATION_BYTES.with(|counter| counter.replace(None).unwrap());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.startup_directives()[0].entries().len(), count);
    for entry in parsed.startup_directives()[0].entries() {
        assert_eq!(entry.voltage(), 1.0);
    }
    bytes
}

#[test]
fn four_times_more_startup_entries_use_near_linear_allocation_volume() {
    let mut measurements = Vec::new();
    for directive in [".IC", ".NODESET"] {
        for scoped in [false, true] {
            let small = allocated(&source(64, directive, scoped), 64);
            let large = allocated(&source(256, directive, scoped), 256);
            println!("{directive} scoped={scoped}: bytes64={small} bytes256={large}");
            measurements.push((directive, scoped, small, large));
        }
    }
    for (directive, scoped, small, large) in measurements {
        // Four times the input may change vector/hash-table capacity rounding.
        // A 6x ceiling allows that overhead while rejecting per-entry copies
        // of the entire card (quadratic growth approaches 16x).
        assert!(
            large <= 6 * small,
            "{directive} scoped={scoped}: {small} -> {large}"
        );
    }
}
