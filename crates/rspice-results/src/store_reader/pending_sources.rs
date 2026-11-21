//! The columns and event signals of a kept segment still waiting for its first check.
//!
//! A project opens without reading its kept segments whole: each one not checked yet is left for
//! a check of its own ([`super::opening_unchecked`]). Until that check ends, a result names the
//! segment's columns and signals through these sources. They keep the identity, the rows and the
//! changes the result was saved with, so a save keeps the segment and a run says it is being
//! checked, never that its samples are missing. Reading one joins the check ([`super::kept_segment`]
//! waits for the check under way, or runs it when none has started) and then reads the segment,
//! or says what is wrong with it: the file is read whole once, and its samples are never read as
//! empty. Inside [`super::opening_unchecked`] a read waits for nothing and says the store is still
//! being checked.

use std::ops::Range;
use std::sync::{Arc, OnceLock};

use rspice_results_store::{Digest, EventSpan};

use super::references::{segment_column, segment_events};
use super::segments::unread;
use super::{checks, kept_segment};
use crate::events::points::EventHistorySource;
use crate::waveform::{ColumnSource, ColumnTile, RowsAround};

/// Why a store being checked is not read inside a project open.
fn still_checking(digest: Digest) -> String {
    format!(
        "its results store {} is still being checked",
        rspice_results_store::kept_segment_name(&digest)
    )
}

/// What a source of a segment being checked reads once the check ended, opened the first time it
/// is asked for: the segment's own column or signal, or why there is none.
struct Opened<S: ?Sized> {
    digest: Digest,
    opened: OnceLock<Result<Arc<S>, String>>,
}

impl<S: ?Sized> Opened<S> {
    fn new(digest: Digest) -> Self {
        Self {
            digest,
            opened: OnceLock::new(),
        }
    }

    /// The source, opened through `open` once the check of its segment ended. Inside a project
    /// open nothing waits, and nothing is remembered.
    fn get(
        &self,
        open: impl FnOnce(Arc<super::Segment>) -> Result<Arc<S>, String>,
    ) -> Result<&Arc<S>, String> {
        if let Some(opened) = self.opened.get() {
            return opened.as_ref().map_err(Clone::clone);
        }
        if checks::deferring() {
            return Err(still_checking(self.digest));
        }
        self.opened
            .get_or_init(|| match kept_segment(self.digest) {
                Some(segment) => open(segment),
                None => Err(unread(self.digest)),
            })
            .as_ref()
            .map_err(Clone::clone)
    }
}

/// A column of a kept segment still waiting for its first check.
pub(super) struct CheckingColumn {
    column: u64,
    /// The rows the saved result stated.
    rows: u64,
    name: String,
    opened: Opened<dyn ColumnSource>,
}

impl CheckingColumn {
    pub(super) fn new(digest: Digest, column: u64, rows: u64, name: String) -> Arc<Self> {
        Arc::new(Self {
            column,
            rows,
            name,
            opened: Opened::new(digest),
        })
    }

    fn column(&self) -> Result<&Arc<dyn ColumnSource>, String> {
        let (digest, column, rows) = (self.opened.digest, self.column, self.rows);
        self.opened
            .get(|segment| segment_column(segment, digest, column, rows))
    }
}

impl ColumnSource for CheckingColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        rspice_results_store::kept_segment_name(&self.opened.digest)
    }

    fn rows(&self) -> u64 {
        self.rows
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.column()?.read(first, out)
    }

    fn ascending(&self) -> bool {
        self.column().is_ok_and(|column| column.ascending())
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((self.opened.digest.0, self.column))
    }

    fn tile(
        &self,
        rows: Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Option<Result<ColumnTile, String>> {
        match self.column() {
            Ok(column) => column.tile(rows, x0, x1, pixels),
            Err(reason) => Some(Err(reason)),
        }
    }

    fn rows_around(&self, rows: Range<u64>, x: f64) -> Option<Result<RowsAround, String>> {
        match self.column() {
            Ok(column) => column.rows_around(rows, x),
            Err(reason) => Some(Err(reason)),
        }
    }
}

/// An event signal of a kept segment still waiting for its first check.
pub(super) struct CheckingEvents {
    signal: u64,
    /// The changes the saved result stated.
    changes: u64,
    name: String,
    opened: Opened<dyn EventHistorySource>,
}

impl CheckingEvents {
    pub(super) fn new(digest: Digest, signal: u64, changes: u64, name: String) -> Arc<Self> {
        Arc::new(Self {
            signal,
            changes,
            name,
            opened: Opened::new(digest),
        })
    }

    fn events(&self) -> Result<&Arc<dyn EventHistorySource>, String> {
        let signal = self.signal;
        self.opened.get(|segment| segment_events(segment, signal))
    }
}

impl EventHistorySource for CheckingEvents {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        rspice_results_store::kept_segment_name(&self.opened.digest)
    }

    /// The changes the saved result stated: counting them reads nothing.
    fn count(&self) -> Result<u64, String> {
        Ok(self.changes)
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        self.events()?.before(t)
    }

    fn span(&self, t0: f64, t1: f64) -> Result<EventSpan, String> {
        self.events()?.span(t0, t1)
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((self.opened.digest.0, self.signal))
    }
}
