//! Retained simulation history, retention policy and mutation tracking.
//!
//! A mutable borrow invalidates retained revisions before exposing any run,
//! analysis, or sample storage. Reads and unchanged clones preserve revisions.
//! Revision handles keep their allocation alive, so replacing or dropping a
//! history cannot recycle an identity still held by a snapshot cache.

use std::ops::{Deref, DerefMut};

use crate::run::{RunRetention, SimulationRun};
use crate::run_receipt::PreparedRunReceipt;
use rspice_app_types::product::{RunId, SimulationPlanId};

#[cfg(test)]
mod tests;

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

impl<R> RunHistory<R> {
    /// Index of the newest run with retained analyses.
    #[must_use]
    pub fn newest_retained_result_run_index<A>(&self) -> Option<usize>
    where
        R: AsRef<SimulationRun<A>>,
    {
        self.iter()
            .position(|run| !run.as_ref().analyses.is_empty())
    }

    /// Count datasets owned by this prepared plan.
    #[must_use]
    pub fn retained_plan_dataset_count<A>(&self, plan_id: SimulationPlanId) -> usize
    where
        R: AsRef<SimulationRun<A>>,
    {
        self.iter()
            .filter(|run| {
                let run = run.as_ref();
                run.prepared_receipt()
                    .and_then(PreparedRunReceipt::simulation_plan_id)
                    == Some(plan_id)
            })
            .count()
    }

    /// Count golden baselines owned by this prepared plan.
    #[must_use]
    pub fn pinned_plan_run_count<A>(&self, plan_id: SimulationPlanId) -> usize
    where
        R: AsRef<SimulationRun<A>>,
    {
        self.iter()
            .filter(|run| {
                let run = run.as_ref();
                run.retention().is_pinned()
                    && run
                        .prepared_receipt()
                        .and_then(PreparedRunReceipt::simulation_plan_id)
                        == Some(plan_id)
            })
            .count()
    }

    /// Prune oldest eligible runs, always keeping the newest and pinned baselines.
    /// Selection repair and dependent-view cleanup belong to the caller.
    pub fn prune_runs<A>(&mut self, limit: usize)
    where
        R: AsRef<SimulationRun<A>>,
    {
        let limit = limit.max(1);
        while self.len() > limit {
            // Oldest first among the pruneable, and never index 0: the head of
            // a newest-first history is the run that was just produced.
            let Some(oldest_pruneable) = self
                .iter()
                .rposition(|run| run.as_ref().retention().is_pruneable())
                .filter(|index| *index > 0)
            else {
                break;
            };
            self.deref_mut().remove(oldest_pruneable);
        }
    }

    /// Prune only this plan's oldest eligible datasets, preserving the selected run.
    pub fn prune_plan_runs<A>(
        &mut self,
        plan_id: SimulationPlanId,
        limit: usize,
        selected_run_id: Option<RunId>,
    ) where
        R: AsRef<SimulationRun<A>>,
    {
        let limit = limit.max(1);
        while self.retained_plan_dataset_count(plan_id) > limit {
            let Some(index) = self.iter().rposition(|run| {
                let run = run.as_ref();
                run.retention().is_pruneable()
                    && run
                        .prepared_receipt()
                        .and_then(PreparedRunReceipt::simulation_plan_id)
                        == Some(plan_id)
                    && Some(run.run_id) != selected_run_id
            }) else {
                break;
            };
            self.deref_mut().remove(index);
        }
    }

    /// Reclassify one stable run identity without pruning it.
    /// Mutable access keeps the same conservative revision invalidation as other edits.
    pub fn set_run_retention<A>(&mut self, run_id: RunId, retention: RunRetention) -> bool
    where
        R: AsRef<SimulationRun<A>> + AsMut<SimulationRun<A>>,
    {
        let Some(run) = self
            .deref_mut()
            .iter_mut()
            .find(|run| run.as_ref().run_id == run_id)
        else {
            return false;
        };
        run.as_mut().set_retention(retention);
        true
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
