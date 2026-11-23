//! A retained analysis's signals found by name in a lookup rather than a walk
//! of its columns. After a million-device run one analysis holds a million
//! signals: the data browser lists them as a catalogue, while a tick, a
//! strip's projection or a selection reads only the few it names.

use rspice_results::analysis_result::AnalysisResult;
use rspice_results::result_presentation::AnalysisPresentationKey;
use rspice_results::waveform::RetainedTrace;
use rspice_vocabulary::source_revision::SourceRevision;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, PoisonError};

/// One analysis's columns by name, and the columns its dataset plots.
#[derive(Debug, Default)]
pub struct SignalIndex {
    /// Each column's name hash and the column, ordered by hash.
    by_hash: Vec<(u64, usize)>,
    /// The columns the dataset itself plots, in column order.
    plotted: Vec<usize>,
}

fn name_hash(name: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
}

impl SignalIndex {
    /// The index of `analysis`, made in one walk of its columns.
    pub fn of(analysis: &AnalysisResult<RetainedTrace>) -> Self {
        let mut by_hash = Vec::with_capacity(analysis.waveforms.len());
        let mut plotted = Vec::new();
        for (column, waveform) in analysis.waveforms.iter().enumerate() {
            by_hash.push((name_hash(&waveform.name), column));
            if waveform.visible {
                plotted.push(column);
            }
        }
        by_hash.sort_unstable();
        Self { by_hash, plotted }
    }

    /// The columns of `analysis`, the analysis this indexes, named `name`, in
    /// column order.
    pub fn columns_named<'a>(
        &'a self,
        analysis: &'a AnalysisResult<RetainedTrace>,
        name: &'a str,
    ) -> impl Iterator<Item = usize> + 'a {
        let hash = name_hash(name);
        let start = self.by_hash.partition_point(|&(other, _)| other < hash);
        self.by_hash[start..]
            .iter()
            .take_while(move |&&(other, _)| other == hash)
            .map(|&(_, column)| column)
            .filter(move |&column| {
                analysis
                    .waveforms
                    .get(column)
                    .is_some_and(|waveform| waveform.name == name)
            })
    }

    /// The one column of `analysis` named `name`: none when no column has
    /// that name, or more than one has.
    pub fn column(&self, analysis: &AnalysisResult<RetainedTrace>, name: &str) -> Option<usize> {
        let mut named = self.columns_named(analysis, name);
        let column = named.next()?;
        named.next().is_none().then_some(column)
    }

    /// The columns the dataset plots, in column order.
    pub fn plotted(&self) -> &[usize] {
        &self.plotted
    }
}

/// The indices of the retained analyses, kept while the retained runs are
/// unchanged; each is made the first time something asks for it.
#[derive(Debug, Default)]
pub struct SignalIndices {
    held: Mutex<Option<IndicesAt>>,
}

/// Indices made from the retained runs at one revision.
type IndicesAt = (
    SourceRevision,
    HashMap<AnalysisPresentationKey, Arc<SignalIndex>>,
);

impl Clone for SignalIndices {
    /// A copy makes its own indices as it needs them.
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl SignalIndices {
    /// The index of `analysis`, which `key` names among the retained runs at
    /// `revision`. None while the analysis is still being run: its runs
    /// change every frame, and an index made each frame would cost more than
    /// the walk it saves.
    pub fn index(
        &self,
        revision: &SourceRevision,
        key: AnalysisPresentationKey,
        analysis: &AnalysisResult<RetainedTrace>,
    ) -> Option<Arc<SignalIndex>> {
        if analysis.is_live_partial() {
            return None;
        }
        let mut held = self.held.lock().unwrap_or_else(PoisonError::into_inner);
        if held.as_ref().is_none_or(|(at, _)| at != revision) {
            *held = Some((revision.clone(), HashMap::new()));
        }
        let (_, indices) = held.as_mut()?;
        Some(Arc::clone(
            indices
                .entry(key)
                .or_insert_with(|| Arc::new(SignalIndex::of(analysis))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::analysis_type::AnalysisType;

    fn analysis(names: &[(&str, bool)]) -> AnalysisResult<RetainedTrace> {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "tran", 0.0);
        analysis.waveforms = names
            .iter()
            .map(|(name, visible)| {
                let mut trace = RetainedTrace::new(*name, vec![0.0], vec![0.0], "");
                trace.visible = *visible;
                trace
            })
            .collect();
        analysis
    }

    #[test]
    fn a_name_finds_its_one_column_and_a_shared_name_finds_none() {
        let analysis = analysis(&[
            ("V(a)", true),
            ("V(b)", false),
            ("V(a)", false),
            ("I(r)", true),
        ]);
        let index = SignalIndex::of(&analysis);
        assert_eq!(index.column(&analysis, "V(b)"), Some(1));
        assert_eq!(index.column(&analysis, "I(r)"), Some(3));
        assert_eq!(
            index.column(&analysis, "V(a)"),
            None,
            "two columns share it"
        );
        assert_eq!(
            index.columns_named(&analysis, "V(a)").collect::<Vec<_>>(),
            vec![0, 2]
        );
        assert_eq!(index.column(&analysis, "V(c)"), None);
        assert_eq!(index.plotted(), &[0, 3]);
    }
}
