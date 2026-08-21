//! The exact deck every point of a completed run executed.
//!
//! A prepared run seals one source per point before dispatch and the engine
//! reads exactly that. Nothing kept it. The run-level source sat on the
//! controller until the next batch replaced it; the per-point sources were
//! consumed with the tasks that carried them; and only a *manual deck* run
//! left anything a reader could reopen — the editor buffer, which is the deck
//! somebody typed and not the deck the engine was handed.
//!
//! So a corner run could be inspected in every way except the one that
//! settles an argument: what did this point actually solve.
//!
//! # Retained, bounded, and never durable
//!
//! Decks are large and a PVT sweep has one per point, so this is a session
//! archive with a ceiling on both counts — [`MAX_RETAINED_RUNS`] runs and
//! [`MAX_RETAINED_BYTES`] of distinct text — and the oldest run is dropped
//! first. Points that solved the run-level source verbatim share one
//! allocation, which is the common case and costs nothing to keep.
//!
//! Nothing here is written to a project file. A deck is derivable from a
//! project revision and a run receipt, and a saved copy would be a second
//! authority on what executed — one that could disagree with the first.
//!
//! # An absent deck says so
//!
//! A run whose deck was evicted, or that ran before this session started, has
//! no entry. Every reader is therefore an `Option`, and the surfaces state the
//! absence rather than falling back to the working deck, which is a different
//! document and would be a lie told confidently.

use std::collections::VecDeque;
use std::sync::Arc;

use crate::state::model_library::SEALED_MODEL_SOURCE_MARKER;

/// How many runs' decks one session keeps.
///
/// Four, because the question this answers is nearly always about the run just
/// finished or the one before it, and a deck is the largest text this
/// application holds per point.
const MAX_RETAINED_RUNS: usize = 4;

/// The total distinct deck text one session keeps, across every retained run.
const MAX_RETAINED_BYTES: usize = 8 * 1024 * 1024;

/// One point of a run, and the source its engine read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedDeckPoint {
    /// The task label the run dispatched this point under.
    pub label: String,
    /// The exact bytes handed to the engine, shared with every other point
    /// that solved the same source.
    pub deck: Arc<str>,
    /// What the deck's own sealed-source comments say the model blocks in it
    /// came from, in the order they appear.
    ///
    /// Read back out of the deck rather than carried beside it: the comment is
    /// what the engine was given, so a label here can never describe a source
    /// the deck does not actually contain.
    pub model_sources: Vec<String>,
}

/// Every deck one run executed, in the order its tasks were dispatched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedDeck {
    pub run_id: u64,
    pub points: Vec<ExecutedDeckPoint>,
}

impl ExecutedDeck {
    /// Distinct deck text this record holds, counting shared sources once.
    fn bytes(&self) -> usize {
        let mut seen: Vec<*const u8> = Vec::new();
        let mut total = 0;
        for point in &self.points {
            let identity = std::ptr::from_ref::<str>(point.deck.as_ref()).cast::<u8>();
            if seen.contains(&identity) {
                continue;
            }
            seen.push(identity);
            total += point.deck.len();
        }
        total
    }

    /// The point at an index, or the first one when there is no choice to make.
    pub fn point(&self, index: usize) -> Option<&ExecutedDeckPoint> {
        self.points.get(index).or_else(|| self.points.first())
    }

    /// Every model source any point of this run was given, once each.
    ///
    /// Order is first appearance, so a reader sees the run's model sources in
    /// the order the run's own decks state them.
    pub fn model_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = Vec::new();
        for point in &self.points {
            for source in &point.model_sources {
                if !sources.iter().any(|held| held == source) {
                    sources.push(source.clone());
                }
            }
        }
        sources
    }
}

/// The executed decks this session still holds, oldest first.
#[derive(Debug, Clone, Default)]
pub struct ExecutedDeckArchive {
    runs: VecDeque<ExecutedDeck>,
}

impl ExecutedDeckArchive {
    /// Seals one run's decks, evicting oldest-first to stay inside both caps.
    ///
    /// A record with no points is not retained: an empty archive entry would
    /// report a run whose deck is held and then show nothing, which is worse
    /// than reporting that nothing is held.
    pub fn retain(&mut self, deck: ExecutedDeck) {
        if deck.points.is_empty() {
            return;
        }
        self.runs.retain(|held| held.run_id != deck.run_id);
        self.runs.push_back(deck);
        while self.runs.len() > MAX_RETAINED_RUNS {
            self.runs.pop_front();
        }
        // The newest run is never evicted for size: a single deck larger than
        // the whole budget is still the answer to the question being asked,
        // and dropping it would leave the archive empty at exactly the moment
        // a reader looked.
        while self.runs.len() > 1
            && self.runs.iter().map(ExecutedDeck::bytes).sum::<usize>() > MAX_RETAINED_BYTES
        {
            self.runs.pop_front();
        }
    }

    /// The decks one run executed, if this session still holds them.
    pub fn get(&self, run_id: u64) -> Option<&ExecutedDeck> {
        self.runs.iter().find(|deck| deck.run_id == run_id)
    }
}

/// The sealed model sources one deck's own comments name.
pub fn sealed_model_sources(deck: &str) -> Vec<String> {
    deck.lines()
        .filter_map(|line| line.trim_start().strip_prefix(SEALED_MODEL_SOURCE_MARKER))
        .map(|label| label.trim().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(label: &str, deck: &Arc<str>) -> ExecutedDeckPoint {
        ExecutedDeckPoint {
            label: label.to_owned(),
            deck: Arc::clone(deck),
            model_sources: sealed_model_sources(deck),
        }
    }

    fn deck_of(run_id: u64, text: &str, points: usize) -> ExecutedDeck {
        let deck: Arc<str> = Arc::from(text);
        ExecutedDeck {
            run_id,
            points: (0..points)
                .map(|index| point(&format!("point {index}"), &deck))
                .collect(),
        }
    }

    #[test]
    fn the_sealed_comments_name_the_model_sources_the_engine_was_given() {
        let deck = format!(
            "run deck\n{SEALED_MODEL_SOURCE_MARKER}pack rspice-opamps 2.1.0\n.model X D\n\
             * an ordinary comment\n  {SEALED_MODEL_SOURCE_MARKER}built into RSpice · foundation \n\
             .end\n"
        );
        assert_eq!(
            sealed_model_sources(&deck),
            vec![
                "pack rspice-opamps 2.1.0".to_owned(),
                "built into RSpice · foundation".to_owned(),
            ]
        );
        assert!(
            sealed_model_sources("deck\n.end\n").is_empty(),
            "a deck with no sealed block names no model source"
        );
    }

    #[test]
    fn points_that_solved_one_source_are_counted_and_held_once() {
        let record = deck_of(1, &"x".repeat(1000), 40);
        assert_eq!(
            record.bytes(),
            1000,
            "forty points over one shared source is one source"
        );
    }

    #[test]
    fn the_archive_keeps_the_newest_runs_and_states_what_it_dropped_by_absence() {
        let mut archive = ExecutedDeckArchive::default();
        for run_id in 1..=6 {
            archive.retain(deck_of(run_id, &format!("run {run_id}\n.end\n"), 2));
        }
        assert!(archive.get(6).is_some());
        assert!(archive.get(3).is_some());
        assert!(
            archive.get(2).is_none() && archive.get(1).is_none(),
            "the oldest runs are dropped, and absence is how that is reported"
        );
    }

    #[test]
    fn one_run_larger_than_the_whole_budget_is_still_the_run_that_is_kept() {
        let mut archive = ExecutedDeckArchive::default();
        archive.retain(deck_of(1, &"x".repeat(1024), 1));
        archive.retain(deck_of(2, &"y".repeat(MAX_RETAINED_BYTES + 1), 1));
        assert!(
            archive.get(2).is_some(),
            "the newest run is never evicted for size"
        );
        assert!(
            archive.get(1).is_none(),
            "and the older one made room for it"
        );
    }

    #[test]
    fn re_running_the_same_run_identity_replaces_rather_than_duplicates() {
        let mut archive = ExecutedDeckArchive::default();
        archive.retain(deck_of(7, "first\n.end\n", 1));
        archive.retain(deck_of(7, "second\n.end\n", 1));
        assert_eq!(archive.runs.len(), 1);
        assert_eq!(
            archive
                .get(7)
                .and_then(|deck| deck.point(0))
                .map(|point| point.deck.as_ref()),
            Some("second\n.end\n")
        );
    }
}
