//! The decks a project's retained runs executed, as the file keeps them.
//!
//! A dataset explains what a run produced; its deck explains what the engine
//! was given to produce it, and until this was written the second half lived
//! only in the session that ran it. Reopening a project therefore reopened
//! every retained result with nothing to say about the source behind it.
//!
//! # One text, however many points solved it
//!
//! A PVT sweep hands most of its points the same source, and the session
//! archive shares one allocation between them. The file keeps the same shape:
//! distinct text in [`ProjectExecutedDecks::sources`], referenced by index
//! from each point. Writing it per point would multiply the largest text this
//! application holds by the point count.
//!
//! The two halves are one field because they are one fact with a
//! cross-reference invariant — a point naming a source the file does not
//! carry, and a source no point names, are both refusals — and a shape whose
//! halves can be written apart is a shape they can disagree in.
//!
//! # What is not written
//!
//! The sealed model sources each point carries in memory are absent, and are
//! re-read from the deck's own comments on load. The comment is what the
//! engine was given; a label persisted beside it could name a source the deck
//! does not actually contain.

use super::*;

/// The decks every retained run executed, deduplicated across the project.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProjectExecutedDecks {
    /// Distinct executed-deck text, written once and referenced by index.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
    /// The exact deck each retained run's engine read, per point.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runs: Vec<ProjectExecutedDeck>,
}

/// One retained run's executed decks, as the project file writes them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectExecutedDeck {
    /// The run sequence the session archive keys on, which is also the
    /// `runs[].id` this file persists.
    pub run_id: u64,
    pub points: Vec<ProjectExecutedDeckPoint>,
}

/// One dispatched task, and which distinct deck text its engine read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectExecutedDeckPoint {
    pub label: String,
    /// Index into [`ProjectExecutedDecks::sources`].
    pub source: usize,
}

impl ProjectExecutedDecks {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty() && self.runs.is_empty()
    }

    /// Project the session archive into the file's deduplicated shape.
    ///
    /// Only runs still in the history contribute: retention discards a dataset
    /// and its deck together, and a deck written for a run the file does not
    /// contain would reload as an artifact nothing can open.
    pub(super) fn from_state(state: &SimulationState) -> Self {
        let mut sources: Vec<String> = Vec::new();
        let mut runs: Vec<ProjectExecutedDeck> = Vec::new();
        for record in state.executed_decks.iter() {
            if !state.runs.iter().any(|run| run.id == record.run_id) {
                continue;
            }
            let points = record
                .points
                .iter()
                .map(|point| {
                    // Equality rather than allocation identity: the archive
                    // shares one allocation between the points that solved one
                    // source, and two runs' identical decks are separate
                    // allocations that would otherwise both be written.
                    let source = sources
                        .iter()
                        .position(|held| held.as_str() == point.deck.as_ref())
                        .unwrap_or_else(|| {
                            sources.push(point.deck.to_string());
                            sources.len() - 1
                        });
                    ProjectExecutedDeckPoint {
                        label: point.label.clone(),
                        source,
                    }
                })
                .collect();
            runs.push(ProjectExecutedDeck {
                run_id: record.run_id,
                points,
            });
        }
        Self { sources, runs }
    }

    /// Rebuild the session archive, sharing one allocation per distinct source
    /// exactly as the archive that wrote it did.
    ///
    /// The caps are re-applied by [`ExecutedDeckArchive::restore`]: a file
    /// carrying more than a session could have held is refused, not trimmed.
    pub(super) fn into_archive(self) -> Result<crate::state::ExecutedDeckArchive, String> {
        let shared: Vec<std::sync::Arc<str>> = self
            .sources
            .iter()
            .map(|source| std::sync::Arc::from(source.as_str()))
            .collect();
        let records = self
            .runs
            .iter()
            .map(|deck| {
                let points = deck
                    .points
                    .iter()
                    .map(|point| {
                        let source = shared.get(point.source).ok_or_else(|| {
                            format!(
                                "executed deck for run {} references source {} of {}",
                                deck.run_id,
                                point.source,
                                shared.len()
                            )
                        })?;
                        Ok(crate::state::ExecutedDeckPoint {
                            label: point.label.clone(),
                            model_sources: crate::state::sealed_model_sources(source),
                            deck: std::sync::Arc::clone(source),
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok(crate::state::ExecutedDeck {
                    run_id: deck.run_id,
                    points,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        crate::state::ExecutedDeckArchive::restore(records)
    }

    /// What a persisted executed-deck set has to be before it is installed.
    ///
    /// The size and count ceilings are re-applied by
    /// [`ExecutedDeckArchive::restore`] when the archive is actually built.
    /// What is checked here is the shape the file claims: that every deck
    /// belongs to a run this file also contains, that no run has two, and that
    /// the source table is exactly the set the points reference. An
    /// unreferenced source is not a harmless leftover — it is deck text riding
    /// along in a project that nothing accounts for and nothing shows.
    pub(super) fn validate(&self, run_sequences: &HashSet<u64>) -> Result<(), String> {
        let mut owners = HashSet::new();
        let mut referenced = HashSet::new();
        for deck in &self.runs {
            if !run_sequences.contains(&deck.run_id) {
                return Err(format!(
                    "executed deck names simulation run {} which is not in persisted history",
                    deck.run_id
                ));
            }
            if !owners.insert(deck.run_id) {
                return Err(format!(
                    "duplicate executed deck for simulation run {}",
                    deck.run_id
                ));
            }
            if deck.points.is_empty() {
                return Err(format!(
                    "executed deck for simulation run {} holds no point",
                    deck.run_id
                ));
            }
            for point in &deck.points {
                if point.source >= self.sources.len() {
                    return Err(format!(
                        "executed deck for simulation run {} references source {} of {}",
                        deck.run_id,
                        point.source,
                        self.sources.len()
                    ));
                }
                referenced.insert(point.source);
            }
        }
        if referenced.len() != self.sources.len() {
            return Err(format!(
                "{} of {} executed-deck sources are referenced by no retained point",
                self.sources.len() - referenced.len(),
                self.sources.len()
            ));
        }
        Ok(())
    }
}

/// Executed decks are retained only from schema v15 onward.
///
/// Nothing is reconstructed for an older file, and that is the whole rule: a
/// run recorded before the decks were kept executed a source nobody saved, and
/// deriving one from today's design would attribute a deck to a run that never
/// read it. A file claiming an older schema while carrying decks is refused
/// rather than trusted.
pub(super) fn reject_executed_decks_before_schema_v15(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    if !results.executed_decks.is_empty() {
        return Err(format!(
            "schema-v{source_schema} simulation results contain executed decks introduced by \
             schema v15"
        ));
    }
    Ok(())
}
