//! Observation cards a task's own deck carries, and no other task's.
//!
//! Layer: execution preparation. A `.fft` card is not an observation of a
//! transient — the engine makes every requested sample time a solver stop, so
//! the card changes the accepted step grid, and a card whose `STOP` is past a
//! transient's stop time *fails* that transient. Both facts say the same
//! thing: a card may only ever be in the deck of the transient it is bound
//! to, and it is part of that transient's identity.
//!
//! This module is the whole of that seam. It attaches each FFT analysis's card
//! to its bound transient before dependency bindings capture the producer's
//! digest, splices the cards into that transient's own deck after per-point
//! expansion has chosen it, and refuses a prepared FFT task whose card its
//! producer does not carry.

use super::{
    AnalysisSpec, CanonicalWriter, ContentDigest, PreparationError, PreparationStage, PreparedTask,
    splice_before_terminal_end_card,
};

impl PreparedTask {
    /// Declare the observation cards this task's deck carries.
    ///
    /// Re-derives the config digest, because the cards are part of what this
    /// task solves. A task with no cards keeps the digest it already had, bit
    /// for bit, so a transient with no bound FFT is the same request it was.
    pub(in crate::simulation) fn with_bound_observation_cards(&mut self, cards: Vec<String>) {
        self.bound_observation_cards = cards;
        self.config_digest = self.payload_digest();
    }

    pub(in crate::simulation) fn bound_observation_cards(&self) -> &[String] {
        &self.bound_observation_cards
    }
}

/// Wrap `base` with the cards a task carries, or return it untouched.
///
/// A conditional wrapper, not a field: the empty case must add no bytes, so
/// every prepared transient already on disk keeps its identity.
pub(in crate::simulation) fn digest_with_cards(
    base: ContentDigest,
    cards: &[String],
) -> ContentDigest {
    if cards.is_empty() {
        return base;
    }
    let mut writer = CanonicalWriter::new("rspice.analysis-bound-observation-cards/v1");
    writer.digest(base);
    writer.sequence(cards.len());
    for card in cards {
        writer.string(card);
    }
    writer.finish()
}

/// Give each Transient task the cards of the FFT tasks bound to it.
///
/// Queue order, so two FFT instances on one transient produce a stable deck.
/// Called before dependency bindings are taken: a bound card changes the
/// producer's config digest, and a binding freezes that digest.
pub(in crate::simulation) fn attach_bound_observation_cards(tasks: &mut [PreparedTask]) {
    let mut carried: Vec<(usize, String)> = Vec::new();
    for task in tasks.iter() {
        let AnalysisSpec::Fft { .. } = task.queued_analysis().spec else {
            continue;
        };
        let card = task.queued_analysis().analysis_line.clone();
        for dependency in task.dependencies() {
            if let Some(producer) = tasks.iter().position(|candidate| {
                candidate.instance_id() == *dependency
                    && matches!(
                        candidate.queued_analysis().spec,
                        AnalysisSpec::Transient { .. }
                    )
            }) {
                carried.push((producer, card.clone()));
                break;
            }
        }
    }
    for (producer, card) in carried {
        let mut cards = tasks[producer].bound_observation_cards().to_vec();
        cards.push(card);
        tasks[producer].with_bound_observation_cards(cards);
    }
}

/// Splice each carrying task's cards into that task's deck, and give every
/// FFT task the final deck of the producer that computed its spectrum.
///
/// The FFT task inherits the producer's deck rather than the run-level one so
/// the executed-deck archive shows, for the recorded spectrum, the deck that
/// actually produced it.
pub(in crate::simulation) fn splice_bound_observation_cards(
    tasks: &mut [PreparedTask],
    run_level_deck: &str,
) {
    let mut producer_decks: Vec<(crate::product::AnalysisInstanceId, String)> = Vec::new();
    for task in tasks.iter_mut() {
        if task.bound_observation_cards().is_empty() {
            continue;
        }
        let block = task.bound_observation_cards().join("\n");
        let deck = task
            .executable_netlist_override
            .as_deref()
            .unwrap_or(run_level_deck);
        let spliced = splice_before_terminal_end_card(deck, &block);
        task.executable_netlist_override = Some(spliced.clone());
        producer_decks.push((task.instance_id(), spliced));
    }
    if producer_decks.is_empty() {
        return;
    }
    for task in tasks.iter_mut() {
        if !matches!(task.queued_analysis().spec, AnalysisSpec::Fft { .. }) {
            continue;
        }
        let inherited = task.dependencies().iter().find_map(|dependency| {
            producer_decks
                .iter()
                .find(|(producer, _)| producer == dependency)
                .map(|(_, deck)| deck.clone())
        });
        if let Some(deck) = inherited {
            task.executable_netlist_override = Some(deck);
        }
    }
}

/// Refuse a prepared FFT task whose producer does not carry its card.
///
/// This is what makes "one solve" checkable rather than asserted: the FFT task
/// publishes a spectrum the engine computed, so the card that computed it must
/// be in the deck of the transient it bound.
pub(in crate::simulation) fn validate_bound_observation_cards(
    tasks: &[PreparedTask],
) -> Result<(), PreparationError> {
    for task in tasks {
        if !matches!(task.queued_analysis().spec, AnalysisSpec::Fft { .. }) {
            continue;
        }
        let card = &task.queued_analysis().analysis_line;
        let carried = task.dependencies().iter().any(|dependency| {
            tasks.iter().any(|candidate| {
                candidate.instance_id() == *dependency
                    && candidate
                        .bound_observation_cards()
                        .iter()
                        .any(|held| held == card)
            })
        });
        if !carried {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Prepared FFT analysis {} is not carried by its transient producer",
                    task.instance_id()
                ),
            ));
        }
    }
    Ok(())
}
