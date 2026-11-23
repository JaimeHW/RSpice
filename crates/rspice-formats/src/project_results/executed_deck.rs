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
    pub sources: Vec<ExecutedDeckSource>,
    /// The exact deck each retained run's engine read, per point.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runs: Vec<ProjectExecutedDeck>,
}

/// One distinct executed deck's text, shared with the session archive that
/// holds it, and its content digest: what tells two sources apart, and what
/// the result fingerprint reads in place of the text.
///
/// Written as the text alone; the digest is read again from the text on load.
#[derive(Debug, Clone)]
pub struct ExecutedDeckSource {
    text: std::sync::Arc<str>,
    digest: ContentDigest,
}

impl ExecutedDeckSource {
    /// A source over `text`, digested here.
    pub fn new(text: impl Into<std::sync::Arc<str>>) -> Self {
        let text = text.into();
        let digest = rspice_results::executed_deck::deck_digest(&text);
        Self { text, digest }
    }

    /// The deck text.
    #[must_use]
    pub const fn text(&self) -> &std::sync::Arc<str> {
        &self.text
    }

    /// The content digest of [`Self::text`].
    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    /// This source as the result fingerprint reads it: named by its digest.
    pub(super) fn named_by_digest(&self) -> Self {
        Self {
            text: std::sync::Arc::from(format!("sha256:{}", self.digest)),
            digest: self.digest,
        }
    }
}

/// Equal digests are equal text; comparing them reads neither.
impl PartialEq for ExecutedDeckSource {
    fn eq(&self, other: &Self) -> bool {
        self.digest == other.digest
    }
}

impl Eq for ExecutedDeckSource {}

impl Serialize for ExecutedDeckSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.text)
    }
}

impl<'de> Deserialize<'de> for ExecutedDeckSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(Self::new)
    }
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
    pub fn from_archive(
        archive: &rspice_results::executed_deck::ExecutedDeckArchive,
        retained_runs: impl Iterator<Item = u64> + Clone,
    ) -> Self {
        let mut sources: Vec<ExecutedDeckSource> = Vec::new();
        let mut runs: Vec<ProjectExecutedDeck> = Vec::new();
        for record in archive.iter() {
            if !retained_runs.clone().any(|id| id == record.run_id) {
                continue;
            }
            let points = record
                .points
                .iter()
                .map(|point| {
                    // Content rather than allocation identity, read by digest:
                    // the archive shares one allocation between the points
                    // that solved one source, and two runs' identical decks
                    // are separate allocations that would otherwise both be
                    // written. The text is shared, never copied.
                    let source = sources
                        .iter()
                        .position(|held| held.digest == point.digest())
                        .unwrap_or_else(|| {
                            sources.push(ExecutedDeckSource {
                                text: std::sync::Arc::clone(&point.deck),
                                digest: point.digest(),
                            });
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
    /// exactly as the archive that wrote it did, and its digest.
    ///
    /// The caps are re-applied by [`ExecutedDeckArchive::restore`]: a file
    /// carrying more than a session could have held is refused, not trimmed.
    pub fn into_archive(
        self,
    ) -> Result<rspice_results::executed_deck::ExecutedDeckArchive, String> {
        let records = self
            .runs
            .iter()
            .map(|deck| {
                let points = deck
                    .points
                    .iter()
                    .map(|point| {
                        let source = self.sources.get(point.source).ok_or_else(|| {
                            format!(
                                "executed deck for run {} references source {} of {}",
                                deck.run_id,
                                point.source,
                                self.sources.len()
                            )
                        })?;
                        Ok(rspice_results::executed_deck::ExecutedDeckPoint::digested(
                            point.label.clone(),
                            std::sync::Arc::clone(&source.text),
                            rspice_model_library::sealed_model_sources(&source.text),
                            source.digest,
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok(rspice_results::executed_deck::ExecutedDeck {
                    run_id: deck.run_id,
                    points,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        rspice_results::executed_deck::ExecutedDeckArchive::restore(records)
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
