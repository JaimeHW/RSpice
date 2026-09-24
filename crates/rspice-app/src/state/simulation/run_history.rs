//! Mutation tracking for retained simulation history.
//!
//! A mutable borrow invalidates retained revisions before exposing any run,
//! analysis, or sample storage. Reads and unchanged clones preserve revisions.
//! Revision handles keep their allocation alive, so replacing or dropping a
//! history cannot recycle an identity still held by a snapshot cache.

use std::ops::{Deref, DerefMut};
#[cfg(test)]
use std::sync::Arc;

use super::SimulationRun;

pub(crate) type RunHistoryRevision = crate::source_revision::SourceRevision;

/// Retained runs with a revision that covers every mutable collection access.
#[derive(Debug, Clone)]
pub struct RunHistory {
    runs: Vec<SimulationRun>,
    revision: RunHistoryRevision,
}

impl RunHistory {
    pub(crate) fn revision(&self) -> RunHistoryRevision {
        self.revision.clone()
    }
}

impl Default for RunHistory {
    fn default() -> Self {
        Vec::new().into()
    }
}

impl From<Vec<SimulationRun>> for RunHistory {
    fn from(runs: Vec<SimulationRun>) -> Self {
        Self {
            runs,
            revision: RunHistoryRevision::default(),
        }
    }
}

impl FromIterator<SimulationRun> for RunHistory {
    fn from_iter<T: IntoIterator<Item = SimulationRun>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl Deref for RunHistory {
    type Target = Vec<SimulationRun>;

    fn deref(&self) -> &Self::Target {
        &self.runs
    }
}

impl DerefMut for RunHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Only a retained revision needs a new identity. Consecutive writes
        // without an intervening reader can reuse this unobserved allocation.
        self.revision.advance();
        &mut self.runs
    }
}

impl IntoIterator for RunHistory {
    type Item = SimulationRun;
    type IntoIter = std::vec::IntoIter<SimulationRun>;

    fn into_iter(self) -> Self::IntoIter {
        self.runs.into_iter()
    }
}

impl<'a> IntoIterator for &'a RunHistory {
    type Item = &'a SimulationRun;
    type IntoIter = std::slice::Iter<'a, SimulationRun>;

    fn into_iter(self) -> Self::IntoIter {
        self.runs.iter()
    }
}

impl<'a> IntoIterator for &'a mut RunHistory {
    type Item = &'a mut SimulationRun;
    type IntoIter = std::slice::IterMut<'a, SimulationRun>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn history() -> RunHistory {
        vec![SimulationRun::new(1), SimulationRun::new(2)].into()
    }

    #[test]
    fn history_reads_and_clones_preserve_revision_but_forked_edits_do_not() {
        let original = history();
        let revision = original.revision();
        let mut fork = original.clone();
        assert_eq!(fork.revision(), revision);
        assert_eq!(original.len(), 2);
        assert_eq!(
            original.iter().map(|run| run.id).collect::<Vec<_>>(),
            [1, 2]
        );
        for run in &original {
            assert!(run.id > 0);
        }
        assert_eq!(original.revision(), revision);
        fork[0].label = "Edited".to_owned();
        assert_ne!(fork.revision(), revision);
        assert_eq!(original.revision(), revision);
        assert_ne!(original[0].label, "Edited");
    }

    #[test]
    fn every_mutable_history_access_invalidates_a_retained_revision() {
        let edits: [fn(&mut RunHistory); 8] = [
            |runs| runs.push(SimulationRun::new(3)),
            |runs| {
                runs.pop();
            },
            |runs| runs.swap(0, 1),
            |runs| runs.get_mut(0).unwrap().success = false,
            |runs| runs.iter_mut().next().unwrap().label.clear(),
            |runs| {
                for run in runs {
                    run.label.clear();
                }
            },
            |runs| runs.retain(|run| run.id == 1),
            |runs| runs.clear(),
        ];
        for edit in edits {
            let mut runs = history();
            let revision = runs.revision();
            edit(&mut runs);
            assert_ne!(runs.revision(), revision);
        }
    }

    #[test]
    fn nested_sample_edits_invalidate_history_without_a_manual_version_bump() {
        let mut runs = history();
        runs[0].add_analysis(
            super::super::AnalysisResult::new(
                1,
                super::super::AnalysisType::Transient,
                "Transient",
            )
            .with_waveforms(vec![super::super::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![1.0, 2.0],
                "#ffffff",
            )]),
        );
        let revision = runs.revision();
        Arc::make_mut(&mut runs[0].analyses[0].waveforms[0].y)[1] = 3.0;
        assert_ne!(runs.revision(), revision);
    }

    #[test]
    fn replacing_a_history_cannot_reuse_a_retained_revision() {
        let mut runs = history();
        let revision = runs.revision();
        let retained = std::mem::take(&mut runs);
        assert_ne!(runs.revision(), revision);
        assert_eq!(retained.revision(), revision);
        runs = history();
        assert_ne!(runs.revision(), revision);
        runs = retained;
        assert_eq!(runs.revision(), revision);
    }
}
