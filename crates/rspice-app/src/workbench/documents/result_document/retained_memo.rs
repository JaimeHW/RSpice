//! Source-aware memoization for read-only retained-evidence consumers.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use crate::state::{RunHistoryRevision, SimulationState};

/// A memo belongs to exactly one retained history revision and display version.
/// Holding the revision prevents a restored history from reusing its identity.
/// Read-only consumers check this boundary themselves, without depending on
/// frame preparation to clear answers computed from an earlier source.
#[derive(Debug, Clone)]
pub(super) struct RetainedMemo<K, V> {
    inner: RefCell<Entries<K, V>>,
}

#[derive(Debug, Clone)]
struct Entries<K, V> {
    source: Option<(RunHistoryRevision, u64)>,
    values: HashMap<K, V>,
}

impl<K, V> Default for RetainedMemo<K, V> {
    fn default() -> Self {
        Self {
            inner: RefCell::new(Entries {
                source: None,
                values: HashMap::new(),
            }),
        }
    }
}

impl<K: Eq + Hash, V: Clone> RetainedMemo<K, V> {
    /// Compute an absent answer from retained data; the computation must not
    /// recursively query this same memo. Old generations are released rather
    /// than accumulated as additional map keys.
    pub(super) fn get_or_insert_with(
        &self,
        simulation: &SimulationState,
        key: K,
        compute: impl FnOnce() -> V,
    ) -> V {
        let source = (simulation.runs.revision(), simulation.data_version);
        let mut entries = self.inner.borrow_mut();
        if entries.source.as_ref() != Some(&source) {
            entries.source = Some(source);
            entries.values.clear();
        }
        entries.values.entry(key).or_insert_with(compute).clone()
    }

    /// Release entries whose datasets are no longer retained, even when no
    /// reader asks another question of this memo after the removal.
    pub(super) fn retain(&mut self, keep: impl FnMut(&K, &mut V) -> bool) {
        self.inner.get_mut().values.retain(keep);
    }
}

#[cfg(test)]
mod tests;
