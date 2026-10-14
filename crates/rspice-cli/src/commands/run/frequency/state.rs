//! Small-signal cards around a transient state (`ACTIMES=`/`CHECKPOINT=`).
//!
//! The card runs through the engine's state entry points, once per state,
//! and each state's result is published by the card's own finisher. Every
//! result names the `.TRAN` analysis whose trajectory holds its state as its
//! parent and carries the state's time, and its dataset is
//! `<card>@t=<time>`: the card's planned identity, qualified by the time.

use std::collections::HashSet;

use rspice_core::engine::{SmallSignalCardResult, StateLinearizedResult};
use rspice_core::execution::AnalysisInstanceId;
use rspice_core::netlist::{AnalysisCommand, SmallSignalStateSource};

use super::RunContext;
use crate::cli::CliError;

/// The transient state the result being published linearized around.
#[derive(Debug, Clone, Copy)]
pub(in crate::commands::run) struct StatePublication {
    /// The card's planned identity, which every state's result shares.
    pub(in crate::commands::run) analysis: AnalysisInstanceId,
    /// The `.TRAN` analysis whose trajectory holds the state.
    pub(in crate::commands::run) transient: AnalysisInstanceId,
    /// The state's time in seconds.
    pub(in crate::commands::run) time: f64,
}

impl StatePublication {
    /// The dataset this state's result is published as: `ac-001@t=5.5e-4`.
    pub(in crate::commands::run) fn dataset(&self) -> String {
        format!("{}@t={}", self.analysis.tag(), state_time_label(self.time))
    }
}

/// A state time to twelve significant digits with its trailing zeros
/// dropped, so a time the deck spelled `550u` reads `5.5e-4` although the
/// value it scales to is not exactly that decimal.
fn state_time_label(time: f64) -> String {
    let text = format!("{time:.11e}");
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text.as_str(), "0"));
    let mantissa = if mantissa.contains('.') {
        mantissa.trim_end_matches('0').trim_end_matches('.')
    } else {
        mantissa
    };
    format!("{mantissa}e{exponent}")
}

/// How each card names itself in a refusal.
fn card_label(card: &AnalysisCommand) -> &'static str {
    match card {
        AnalysisCommand::Ac { .. } => "AC",
        AnalysisCommand::Noise { .. } => "Noise",
        AnalysisCommand::Tf { .. } => "Transfer Function",
        AnalysisCommand::Sp { .. } => "S-Parameters",
        AnalysisCommand::Stb { .. } => "STB",
        AnalysisCommand::PoleZero { .. } => "Pole-Zero",
        AnalysisCommand::Sensitivity { .. } => "Sensitivity",
        AnalysisCommand::Disto { .. } => "DISTO",
        _ => "Small-signal",
    }
}

/// Run the small-signal card at `index` around the transient state it names
/// and publish one result per state.
pub(in crate::commands::run) fn run_at_transient_state(
    ctx: &RunContext<'_>,
    index: usize,
    card: &AnalysisCommand,
    state: &SmallSignalStateSource,
) -> Result<(), CliError> {
    let label = card_label(card);
    let results = match state {
        SmallSignalStateSource::TransientTimes(times) => {
            if !ctx.quiet {
                crate::console::line(format_args!(
                    "Running {label} analysis around the transient state at {} time(s)...",
                    times.len()
                ))?;
            }
            ctx.engine.run_small_signal_at_transient_times(
                ctx.netlist,
                index,
                times,
                &crate::abort::ProcessAbort,
            )
        }
        SmallSignalStateSource::CheckpointFile(name) => {
            if !ctx.quiet {
                crate::console::line(format_args!(
                    "Running {label} analysis around the state in checkpoint {name}..."
                ))?;
            }
            let checkpoint = super::super::restart::load_state_checkpoint(ctx, name)?;
            ctx.engine
                .run_small_signal_at_checkpoint(
                    ctx.netlist,
                    index,
                    &checkpoint,
                    &crate::abort::ProcessAbort,
                )
                .map(|result| vec![result])
        }
    }
    .map_err(|source| super::map_frequency_error(ctx, label, source))?;

    let tag =
        super::super::naming::analysis_output_tag(card).ok_or_else(|| CliError::InternalError {
            message: format!("{label} card has no output namespace"),
        })?;
    let analysis = ctx.take_state_card_identity(tag)?;
    let transient = ctx.state_transient_identity()?;
    let mut datasets = HashSet::with_capacity(results.len());
    for StateLinearizedResult { state_time, result } in &results {
        let state = StatePublication {
            analysis,
            transient,
            time: *state_time,
        };
        if !datasets.insert(state.dataset()) {
            return Err(CliError::SimulationError {
                message: format!(
                    "two of the card's state times name one dataset, {}; separate the times by \
                     more than twelve significant digits",
                    state.dataset()
                ),
                analysis: Some(label.to_string()),
            });
        }
        if !ctx.quiet {
            crate::console::line(format_args!(
                "  Around the state at t={} s:",
                state_time_label(*state_time)
            ))?;
        }
        ctx.publish_at_state(state, || publish(ctx, card, result))?;
    }
    Ok(())
}

/// Publish one state's result with the card's own finisher.
fn publish(
    ctx: &RunContext<'_>,
    card: &AnalysisCommand,
    result: &SmallSignalCardResult,
) -> Result<(), CliError> {
    match result {
        SmallSignalCardResult::Ac(points) => super::finish_ac_results(ctx, points),
        SmallSignalCardResult::Noise(points) => {
            let AnalysisCommand::Noise {
                output_node,
                reference_node,
                input_source,
                ..
            } = card
            else {
                return Err(CliError::InternalError {
                    message: "a noise result came from a card that is not .NOISE".to_string(),
                });
            };
            super::finish_noise_results(
                ctx,
                output_node,
                reference_node.as_deref(),
                input_source,
                points,
                true,
            )
        }
        SmallSignalCardResult::TransferFunction(result) => super::finish_tf_result(ctx, result),
        SmallSignalCardResult::SParameters(run) => {
            super::super::advanced::publish_sparam_run(ctx, run, "sp")
        }
        SmallSignalCardResult::Stability(result) => super::finish_stb_result(ctx, result),
        SmallSignalCardResult::PoleZero(result) => super::report_pz(ctx, result),
        SmallSignalCardResult::Sensitivity(result) => super::finish_sensitivity_result(ctx, result),
        SmallSignalCardResult::Distortion(result) => super::finish_disto_result(ctx, result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_state_time_reads_as_the_deck_spelled_it() {
        assert_eq!(state_time_label(550.0 * 1.0e-6), "5.5e-4");
        assert_eq!(state_time_label(5.5e-4), "5.5e-4");
        assert_eq!(state_time_label(25.0 * 1.0e-6), "2.5e-5");
        assert_eq!(state_time_label(1.0e-6), "1e-6");
        assert_eq!(state_time_label(0.0), "0e0");
        assert_eq!(state_time_label(1.234_567_890_123e-3), "1.23456789012e-3");
    }

    #[test]
    fn a_dataset_is_the_card_qualified_by_its_state_time() {
        use rspice_core::execution::AnalysisKind;
        let state = StatePublication {
            analysis: AnalysisInstanceId::new(AnalysisKind::Ac, 0),
            transient: AnalysisInstanceId::new(AnalysisKind::Tran, 0),
            time: 550.0 * 1.0e-6,
        };
        assert_eq!(state.dataset(), "ac-001@t=5.5e-4");
    }
}
