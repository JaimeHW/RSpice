//! Authored transient post-processing, named by the canonical plan.
//!
//! `.FOUR` resolves its operands through the ordered transient output resolver
//! in [`crate::engine::evaluate_transient_fourier_results`], and the canonical
//! [`DeckPlan`] mints one `four-NNN` identity per authored operand. This module
//! is the single place that joins the two, so a frontend never has to pair a
//! spectrum with an identity — or invent a unit for it — on its own.

use crate::abort_signal::AbortSignal;
use crate::analysis::fourier::FourierResult;
use crate::engine::{SimulationError, TransientResult, evaluate_transient_fourier_results};
use crate::netlist::Netlist;
use crate::resource::ResourceLimits;

use super::plan::{AnalysisInstanceId, DeckPlan, PostProcessSource};
use super::schema::SignalUnit;

/// One authored `.FOUR` operand evaluated against a transient trajectory,
/// carrying exactly what [`AnalysisResultDocument::from_fourier`] needs.
///
/// [`AnalysisResultDocument::from_fourier`]: super::AnalysisResultDocument::from_fourier
#[derive(Debug, Clone, PartialEq)]
pub struct PlannedFourierResult {
    /// Canonical identity of this spectrum, such as `four-001`.
    pub analysis: AnalysisInstanceId,
    /// The transient this spectrum was computed from.
    pub parent: AnalysisInstanceId,
    /// Authored operand spelling, such as `V(out)`.
    pub output: String,
    /// Unit of the harmonic magnitudes and the DC component.
    pub output_unit: SignalUnit,
    /// The spectrum itself.
    pub result: FourierResult,
}

/// Declared unit of one resolved transient output column.
///
/// The resolver classifies every column as a voltage, a current, or a braced
/// parameter expression. A parameter expression has no unit the simulator
/// knows, which is [`SignalUnit::Unspecified`] and not dimensionless.
pub(crate) fn transient_output_unit(physical_type: &str) -> Result<SignalUnit, SimulationError> {
    match physical_type {
        "voltage" => Ok(SignalUnit::Volt),
        "current" => Ok(SignalUnit::Ampere),
        "parameter" => Ok(SignalUnit::Unspecified),
        other => Err(SimulationError::Circuit(format!(
            "transient output resolver reported unknown physical quantity '{other}'"
        ))),
    }
}

/// Evaluate every authored `.FOUR` card of a deck against one transient
/// result, binding each spectrum to the identity the plan minted for it.
///
/// `parent` is the transient instance `result` came from; only the plan's
/// `.FOUR` operands bound to that transient are evaluated, so a deck with two
/// `.TRAN` cards does not publish one card's spectra under the other's parent.
///
/// A resolved operand count that disagrees with the plan is a typed error: the
/// identities are assigned by authored operand order, and pairing them with a
/// different set of columns would publish a spectrum under another operand's
/// name.
pub fn evaluate_planned_fourier_with_abort(
    plan: &DeckPlan,
    netlist: &Netlist,
    parent: AnalysisInstanceId,
    result: &TransientResult,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<PlannedFourierResult>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let planned = plan
        .post_process_analyses()
        .iter()
        .filter(|post| {
            post.parent() == parent
                && matches!(post.source(), PostProcessSource::FourierOperand { .. })
        })
        .collect::<Vec<_>>();
    if planned.is_empty() {
        return Ok(Vec::new());
    }

    let evaluated = evaluate_transient_fourier_results(netlist, result, limits, abort)?;
    if evaluated.len() != planned.len() {
        return Err(SimulationError::Netlist(format!(
            "the deck plans {} .FOUR spectra for {parent} but the transient resolver produced {}",
            planned.len(),
            evaluated.len()
        )));
    }

    let mut results = Vec::new();
    results
        .try_reserve_exact(planned.len())
        .map_err(|_| SimulationError::Circuit("planned .FOUR spectra".to_owned()))?;
    for (post, spectrum) in planned.into_iter().zip(evaluated) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let PostProcessSource::FourierOperand {
            card_index,
            operand,
            output,
        } = post.source()
        else {
            return Err(SimulationError::Circuit(format!(
                "{} is not a planned .FOUR operand",
                post.id()
            )));
        };
        if *card_index != spectrum.card_index || output != &spectrum.output {
            return Err(SimulationError::Netlist(format!(
                "planned .FOUR operand {} of card {} names '{output}' but the resolver produced \
                 '{}' from card {}",
                operand + 1,
                card_index + 1,
                spectrum.output,
                spectrum.card_index + 1
            )));
        }
        results.push(PlannedFourierResult {
            analysis: post.id(),
            parent,
            output: spectrum.output,
            output_unit: transient_output_unit(spectrum.physical_type)?,
            result: spectrum.spectrum,
        });
    }
    Ok(results)
}
