//! A per-thread tally of the work an edit does, so a test can hold a gesture
//! to the work each of its frames may do.
//!
//! Wall-clock budgets mean nothing in a debug build on a loaded machine, and
//! the work is invisible in the result: a drag that copies the document on
//! every move, or extracts connectivity over every sheet, draws exactly what
//! a lean one draws. Counting is how a test states it. The sites that count
//! are compiled only under the `work-observation` feature, which only test
//! builds enable, so a shipping build carries none of this. The tally is per
//! thread, so tests running side by side never see each other's work.

use std::cell::Cell;

/// The work counted on this thread since the last [`take`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorkTally {
    /// Whole-document snapshots: the copies an undo step is made of.
    pub document_snapshots: usize,
    /// Connectivity passes, and the objects of the drawings they were given.
    pub connectivity_passes: usize,
    pub connectivity_objects: usize,
    /// The inner work of those passes: each union of two nodes, each step up
    /// a union-find tree, and each candidate point a slanted segment tests.
    pub connectivity_steps: usize,
    /// Collinear-wire merges, and the wires each one sorted onto lines.
    pub collinear_merges: usize,
    pub collinear_wires_sorted: usize,
    /// Junction updates: orphaned dots taken out, implied ones placed.
    pub junction_updates: usize,
    /// Sheet-catalog lookups by cell-view key.
    pub sheet_catalog_lookups: usize,
    /// Reconciliations of the open cell view's sheet membership.
    pub sheet_reconciliations: usize,
}

impl WorkTally {
    const ZERO: Self = Self {
        document_snapshots: 0,
        connectivity_passes: 0,
        connectivity_objects: 0,
        connectivity_steps: 0,
        collinear_merges: 0,
        collinear_wires_sorted: 0,
        junction_updates: 0,
        sheet_catalog_lookups: 0,
        sheet_reconciliations: 0,
    };
}

thread_local! {
    static TALLY: Cell<WorkTally> = const { Cell::new(WorkTally::ZERO) };
}

/// Add work to this thread's tally.
pub fn record(work: impl FnOnce(&mut WorkTally)) {
    TALLY.with(|tally| {
        let mut counted = tally.get();
        work(&mut counted);
        tally.set(counted);
    });
}

/// The work counted on this thread since the last take, leaving the tally
/// at zero.
#[must_use]
pub fn take() -> WorkTally {
    TALLY.with(Cell::take)
}
