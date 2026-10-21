//! Observation cards a task's own deck carries, and no other task's.
//!
//! Layer: execution preparation. A `.fft` card is not an observation of a
//! transient — the engine makes every requested sample time a solver stop, so
//! the card changes the accepted step grid, and a card whose `STOP` is past a
//! transient's stop time *fails* that transient. Both facts say the same
//! thing: a card may only ever be in the deck of the transient it is bound
//! to, and it is part of that transient's identity.
//!
//! A small-signal card at a transient time (`ACTIMES=`) is the same kind of
//! card. Spectre runs it on the transient itself: the time is a breakpoint of
//! the one run, which captures its state there for the card. So the card rides
//! in its transient's deck too, and is part of that transient's identity.
//!
//! This module is the whole of that seam. It attaches each FFT analysis's card,
//! and each state card that shares its transient, to the bound transient before
//! dependency bindings capture the producer's digest, splices the cards into
//! that transient's own deck after per-point expansion has chosen it, and
//! refuses a prepared FFT task whose card its producer does not carry.

use super::{
    AnalysisSpec, CanonicalWriter, ContentDigest, PreparationError, PreparationStage, PreparedTask,
};
use crate::netlist_preparation::splice_before_terminal_end_card;

impl PreparedTask {
    /// Declare the observation cards this task's deck carries.
    ///
    /// Re-derives the config digest, because the cards are part of what this
    /// task solves. A task with no cards keeps the digest it already had, bit
    /// for bit, so a transient with no bound FFT is the same request it was.
    pub(crate) fn with_bound_observation_cards(&mut self, cards: Vec<String>) {
        self.bound_observation_cards = cards;
        self.config_digest = self.payload_digest();
    }

    /// Observation cards bound to this producer when the plan was prepared.
    pub fn bound_observation_cards(&self) -> &[String] {
        &self.bound_observation_cards
    }
}

/// Wrap `base` with the cards a task carries, or return it untouched.
///
/// A conditional wrapper, not a field: the empty case must add no bytes, so
/// every prepared transient already on disk keeps its identity.
pub(crate) fn digest_with_cards(base: ContentDigest, cards: &[String]) -> ContentDigest {
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
pub(crate) fn attach_bound_observation_cards(
    tasks: &mut [PreparedTask],
    abort: &dyn rspice_core::abort_signal::AbortSignal,
) -> crate::error::ServiceRunResult<()> {
    use crate::error::ensure_not_aborted;
    let mut carried = std::collections::BTreeMap::<usize, Vec<String>>::new();
    let mut transient_positions = std::collections::HashMap::new();
    for (index, task) in tasks.iter().enumerate() {
        ensure_not_aborted(abort)?;
        if matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }) {
            transient_positions.insert(task.instance_id(), index);
        }
    }
    for task in tasks.iter() {
        ensure_not_aborted(abort)?;
        let AnalysisSpec::Fft { .. } = task.queued_analysis().spec else {
            continue;
        };
        let card = task.queued_analysis().analysis_line.clone();
        for dependency in task.dependencies() {
            ensure_not_aborted(abort)?;
            if let Some(producer) = transient_positions.get(dependency) {
                carried.entry(*producer).or_default().push(card.clone());
                break;
            }
        }
    }
    for (producer, additional) in carried {
        ensure_not_aborted(abort)?;
        let mut cards = tasks[producer].bound_observation_cards().to_vec();
        cards.extend(additional);
        tasks[producer].with_bound_observation_cards(cards);
    }
    Ok(())
}

/// A small-signal task that reads its state from the run of the transient
/// it is bound to, at its card's one time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BoundStateReader {
    pub(crate) reader: rspice_app_types::product::AnalysisInstanceId,
    pub(crate) time_s: f64,
}

/// Bind each state card to the transient whose state it reads, where the two
/// describe the same transient, so that transient runs once for both.
///
/// The card is spliced into the transient's deck, which lands the run on the
/// card's time; the run captures the state there and hands it to the card's
/// task (`engine_services::transient_state`). Called after the FFT cards are
/// attached, for the same reason, and with them known.
///
/// The same transient is the same deck under the same options: a card's deck
/// is its transient's `.TRAN` card followed by its own, so the two runs differ
/// only where the tasks do. A card keeps its own run when its transient carries
/// an FFT card, whose sample times are solver stops the card's run never has,
/// runs a checkpoint schedule, or runs under other options or another
/// environment; so does a `.SENS` card, whose state moves with each
/// perturbation, and a `CHECKPOINT=` card, which restores its state without a
/// transient. A time outside the transient, or one a hair from another card's,
/// stays the card's own too: the shared run must never fail where the card's
/// own would not.
pub(crate) fn attach_bound_state_cards(
    tasks: &mut [PreparedTask],
    abort: &dyn rspice_core::abort_signal::AbortSignal,
) -> crate::error::ServiceRunResult<()> {
    use crate::error::ensure_not_aborted;
    let transient_positions = tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| matches!(task.task.spec, AnalysisSpec::Transient { .. }))
        .map(|(index, task)| (task.instance_id(), index))
        .collect::<std::collections::HashMap<_, _>>();
    let mut carried = std::collections::BTreeMap::<usize, Vec<(String, BoundStateReader)>>::new();
    for task in tasks.iter() {
        ensure_not_aborted(abort)?;
        let Some(state) = task.transient_state else {
            continue;
        };
        if task.task.spec_options.tran_checkpoint.is_some()
            || matches!(task.task.spec, AnalysisSpec::Sensitivity { .. })
        {
            continue;
        }
        let Some(&producer) = transient_positions.get(&state.parent) else {
            continue;
        };
        let transient = &tasks[producer];
        let Some(stop_time) = shared_transient_stop(transient, task, state.time_s) else {
            continue;
        };
        let Some(card) = task
            .task
            .analysis_line
            .trim()
            .strip_prefix(transient.task.analysis_line.trim())
            .map(str::trim)
            .filter(|card| !card.is_empty())
        else {
            continue;
        };
        // Far wider than any breakpoint merge window, which is a fraction of
        // the maximum step, so every bound time is landed on as its own.
        let readers = carried.entry(producer).or_default();
        if readers.iter().any(|(_, held)| {
            held.time_s != state.time_s && (held.time_s - state.time_s).abs() <= 1.0e-9 * stop_time
        }) {
            continue;
        }
        readers.push((
            card.to_owned(),
            BoundStateReader {
                reader: task.instance_id(),
                time_s: state.time_s,
            },
        ));
    }
    for (producer, readers) in carried {
        ensure_not_aborted(abort)?;
        let mut cards = tasks[producer].bound_observation_cards().to_vec();
        cards.extend(readers.iter().map(|(card, _)| card.clone()));
        tasks[producer].with_bound_observation_cards(cards);
        tasks[producer].bound_state_readers =
            readers.into_iter().map(|(_, reader)| reader).collect();
    }
    Ok(())
}

/// The stop time of `transient` when `card`'s state transient is its run.
fn shared_transient_stop(
    transient: &PreparedTask,
    card: &PreparedTask,
    time_s: f64,
) -> Option<f64> {
    let AnalysisSpec::Transient { stop_time, .. } = transient.task.spec else {
        return None;
    };
    (transient.bound_observation_cards.is_empty()
        && transient.task.spec_options.tran_checkpoint.is_none()
        && transient.task.numeric_override == card.task.numeric_override
        && transient.execution_environment == card.execution_environment
        && time_s.is_finite()
        && (0.0..=stop_time).contains(&time_s))
    .then_some(stop_time)
}

/// Splice each carrying task's cards into that task's deck, and give every
/// FFT task the final deck of the producer that computed its spectrum.
///
/// The FFT task inherits the producer's deck rather than the run-level one so
/// the executed-deck archive shows, for the recorded spectrum, the deck that
/// actually produced it.
pub(crate) fn splice_bound_observation_cards(tasks: &mut [PreparedTask], run_level_deck: &str) {
    let mut producer_decks: Vec<(rspice_app_types::product::AnalysisInstanceId, String)> =
        Vec::new();
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
pub(crate) fn validate_bound_observation_cards(
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
