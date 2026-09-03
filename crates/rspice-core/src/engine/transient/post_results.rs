//! Typed transient post-processing over the exact accepted trajectory.
//!
//! `.FFT` is evaluated inside the transient driver, before any decimation,
//! and lands in [`TransientResult::fft_results`]. `.FOUR` and `.MEASURE` used
//! to be evaluated by each frontend against whatever result it happened to
//! hold, which for a compressed run was a decimated expansion. This module is
//! the single place that evaluates all three against the accepted trajectory,
//! so a compressed and an uncompressed run publish the same numbers.

use super::super::{
    SimulationError, TransientFourierResult, TransientPostResults, TransientResult,
};
use crate::Netlist;
use crate::abort_signal::AbortSignal;
use crate::analysis::{
    FourierAnalysis, FourierConfig, FourierError, evaluate_tran_four_output_requests_with_abort,
    evaluate_tran_measurements_with_abort,
};
use crate::netlist::AnalysisCommand;
use crate::resource::ResourceLimits;

/// Evaluate every authored transient post-process request against one accepted
/// transient trajectory.
///
/// The `.FFT` spectra are taken from `result`, which the transient driver
/// computed before any output projection or waveform decimation. `.FOUR` and
/// transient `.MEASURE` are evaluated here against the same accepted samples.
///
/// An unresolvable `.FOUR` operand or an out-of-range Fourier request is a
/// typed error: it is an authored-input defect, not a reason to publish a
/// silently missing spectrum. A failing `.MEASURE` keeps its own per-statement
/// failure record, which is that family's contract.
pub fn evaluate_transient_post_results(
    netlist: &Netlist,
    result: &TransientResult,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<TransientPostResults, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let fourier = evaluate_transient_fourier(netlist, result, limits, abort)?;
    let measurements = evaluate_tran_measurements_with_abort(netlist, result, abort)?;
    Ok(TransientPostResults {
        fft: result.fft_results.clone(),
        fourier,
        measurements,
    })
}

fn evaluate_transient_fourier(
    netlist: &Netlist,
    result: &TransientResult,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<TransientFourierResult>, SimulationError> {
    let cards = netlist
        .analyses
        .iter()
        .filter_map(|analysis| match analysis {
            AnalysisCommand::Four {
                fundamental,
                num_harmonics,
                ..
            } => Some((*fundamental, *num_harmonics)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut results = Vec::new();
    for (card_index, (fundamental, harmonic_count)) in cards.into_iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(SimulationError::Netlist(format!(
                ".FOUR request {} has an invalid fundamental frequency {fundamental}",
                card_index + 1
            )));
        }
        if harmonic_count == 0 {
            return Err(SimulationError::Netlist(format!(
                ".FOUR request {} must analyze at least one harmonic",
                card_index + 1
            )));
        }
        let columns = evaluate_tran_four_output_requests_with_abort(
            netlist, result, card_index, limits, abort,
        )?;
        let analysis =
            FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(harmonic_count));
        for (output, physical_type, waveform) in columns {
            let spectrum = analysis
                .analyze_with_abort(&result.time, &waveform, abort)
                .map_err(|error| match error {
                    FourierError::Aborted => SimulationError::Aborted,
                    error => SimulationError::Circuit(format!(
                        ".FOUR request {} output `{output}` could not be analyzed: {error}",
                        card_index + 1
                    )),
                })?;
            results.push(TransientFourierResult {
                card_index,
                output,
                physical_type,
                fundamental,
                harmonic_count,
                spectrum,
            });
        }
    }
    Ok(results)
}
