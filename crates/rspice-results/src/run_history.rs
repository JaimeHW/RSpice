//! Mutation tracking for retained simulation history.
//!
//! A mutable borrow invalidates retained revisions before exposing any run,
//! analysis, or sample storage. Reads and unchanged clones preserve revisions.
//! Revision handles keep their allocation alive, so replacing or dropping a
//! history cannot recycle an identity still held by a snapshot cache.

use std::ops::{Deref, DerefMut};

use crate::run::SimulationRun;

mod queries;
pub use queries::{has_retained_op_state, newest_retained_op_state};

pub type RunHistoryRevision = rspice_app_types::source_revision::SourceRevision;

/// Retained runs with a revision that covers every mutable collection access.
#[derive(Debug, Clone)]
pub struct RunHistory<R = SimulationRun> {
    runs: Vec<R>,
    revision: RunHistoryRevision,
}

impl<R> RunHistory<R> {
    pub fn revision(&self) -> RunHistoryRevision {
        self.revision.clone()
    }
}

impl<R> Default for RunHistory<R> {
    fn default() -> Self {
        Vec::new().into()
    }
}

impl<R> From<Vec<R>> for RunHistory<R> {
    fn from(runs: Vec<R>) -> Self {
        Self {
            runs,
            revision: RunHistoryRevision::default(),
        }
    }
}

impl<R> FromIterator<R> for RunHistory<R> {
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl<R> Deref for RunHistory<R> {
    type Target = Vec<R>;

    fn deref(&self) -> &Self::Target {
        &self.runs
    }
}

impl<R> DerefMut for RunHistory<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Only a retained revision needs a new identity. Consecutive writes
        // without an intervening reader can reuse this unobserved allocation.
        self.revision.advance();
        &mut self.runs
    }
}

impl<R> IntoIterator for RunHistory<R> {
    type Item = R;
    type IntoIter = std::vec::IntoIter<R>;

    fn into_iter(self) -> Self::IntoIter {
        self.runs.into_iter()
    }
}

impl<'a, R> IntoIterator for &'a RunHistory<R> {
    type Item = &'a R;
    type IntoIter = std::slice::Iter<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        self.runs.iter()
    }
}

impl<'a, R> IntoIterator for &'a mut RunHistory<R> {
    type Item = &'a mut R;
    type IntoIter = std::slice::IterMut<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().iter_mut()
    }
}
