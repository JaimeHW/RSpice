//! `.DCMATCH`: DC mismatch variance and ranked contributors.
//!
//! # What is computed
//!
//! At the nominal DC operating point — every statistical variable at the value
//! the deterministic run uses — the output's variance is
//!
//! ```text
//! sigma_out^2 = sum over mismatch (d out / d p_i * sigma_i)^2
//!             + sum over process  (d out / d P_j * sigma_j)^2
//! ```
//!
//! A mismatch variable is drawn once per instance, so each `(instance,
//! variable)` pair is its own independent term. A process variable is one
//! variable the whole design reads, so it contributes a single term whose
//! derivative is taken with every instance displaced together — that
//! perfect correlation is the entire difference between the two scopes, and
//! it is why the same declared spread produces a different total.
//!
//! # How the derivative is taken
//!
//! By central difference with the step set to the variable's own standard
//! deviation. That is deliberate and not a convenience: the linearization a
//! mismatch analysis performs is the one that matters at the working scale,
//! and a step chosen orders of magnitude smaller would sit inside the
//! operating-point solver's own convergence noise, where the difference of
//! two solutions is dominated by how each one terminated. On a circuit linear
//! in the varied parameter the step size is irrelevant; on a nonlinear one
//! the error is second order in `sigma / (scale of the parameter)`, which is
//! the same order as the linearization the variance sum already assumes.
//!
//! Each displaced point is a full elaboration — a mismatch variable can reach
//! a model parameter, and the model is what the device evaluates — followed by
//! an operating point **warm started from the nominal solution**, complete
//! MNA vector included, so the Newton iteration resumes from the answer
//! rather than rediscovering it.
//!
//! # Where the numbers come from
//!
//! Every standard deviation comes from the deck's own Spectre `statistics`
//! block, resolved through the sampler's own expression path
//! ([`SpectreStatisticsPlan::scope_standard_deviations`]) so a spread cannot
//! mean one thing to a Monte Carlo trial and another here. A design that
//! declares no statistics is refused by name: there is no default spread to
//! fall back on, and inventing one would answer a question the deck did not
//! ask.
//!
//! [`SpectreStatisticsPlan::scope_standard_deviations`]: crate::netlist::SpectreStatisticsPlan::scope_standard_deviations

use std::collections::{BTreeMap, BTreeSet};

use super::core::DcOpStartup;
use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::dcmatch::{DcMatchContributor, DcMatchResult, DcMatchScope};
use crate::analysis::sensitivity::AcSensitivityOutput;
use crate::netlist::{
    DcMatchCard, FlattenerConfig, SpectreMismatchOverride, SpectreVariationScope,
    flatten_netlist_with_parameter_direction,
};
use crate::{Netlist, Value};

/// Instance label a design-wide process variable is reported under.
///
/// A process variable belongs to no instance: every instance reads the same
/// draw. Naming one of them would suggest the variable could be attributed to
/// it, and leaving the field empty would make the row unreadable.
const PROCESS_OWNER: &str = "(design)";

impl Engine {
    /// Run one authored `.DCMATCH` card.
    pub fn run_dc_match(
        &self,
        netlist: &Netlist,
        card: &DcMatchCard,
    ) -> Result<DcMatchResult, SimulationError> {
        self.run_dc_match_with_abort(netlist, card, &NoAbort)
    }

    /// Cancellable form of [`Self::run_dc_match`].
    ///
    /// Cost is two operating points per `(instance, variable)` mismatch pair
    /// plus two per process variable, each warm started from the nominal
    /// solution.
    pub fn run_dc_match_with_abort(
        &self,
        netlist: &Netlist,
        card: &DcMatchCard,
        abort: &dyn AbortSignal,
    ) -> Result<DcMatchResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        validate_card(card)?;
        if netlist.spectre_statistics.variations.is_empty() {
            return Err(SimulationError::Circuit(
                ".DCMATCH needs a `statistics { mismatch { vary ... } }` block; none is bound to \
                 this design"
                    .to_owned(),
            ));
        }

        // The nominal coordinate is the absence of one: with no coordinate the
        // elaborator samples nothing and every statistical variable keeps the
        // value the deterministic run uses.
        let mut base = netlist.clone();
        base.spectre_statistical_coordinate = None;
        base.spectre_mismatch_override = None;

        let nominal_process = BTreeMap::new();
        let mismatch_sigmas = if card.mismatch {
            base.spectre_statistics
                .scope_standard_deviations(
                    SpectreVariationScope::Mismatch,
                    &base.params,
                    &nominal_process,
                )
                .map_err(|error| SimulationError::Circuit(error.to_string()))?
        } else {
            Vec::new()
        };
        let process_sigmas = if card.process {
            base.spectre_statistics
                .scope_standard_deviations(
                    SpectreVariationScope::Process,
                    &base.params,
                    &nominal_process,
                )
                .map_err(|error| SimulationError::Circuit(error.to_string()))?
        } else {
            Vec::new()
        };
        if mismatch_sigmas.is_empty() && process_sigmas.is_empty() {
            return Err(SimulationError::Circuit(format!(
                ".DCMATCH has nothing to vary: the card selects {}, and this design's \
                 `statistics` block declares no variation there",
                requested_scopes(card)
            )));
        }

        let output = self.resolve_probe(&base, card, abort)?;
        let nominal = self.run_dc_op_state_with_startup_and_abort(
            &base,
            DcOpStartup::Automatic { use_hints: true },
            abort,
        )?;
        let nominal_value = Self::dc_sensitivity_output_value(&nominal.result, &output)?;

        let mut contributors = Vec::new();
        let mut warm_start_iterations = 0usize;
        let mut warm_start_solves = 0usize;

        for sigma in &process_sigmas {
            if sigma.standard_deviation == 0.0 {
                continue;
            }
            // The elaborator's own process route: a process sample reaches
            // every instance as a design parameter, so displacing the
            // parameter displaces them all at once.
            let displaced = |value: Value| {
                let mut perturbed = base.clone();
                perturbed.params.set(&sigma.parameter, value);
                perturbed
            };
            let up = self.displaced_output(
                &displaced(sigma.nominal + sigma.standard_deviation),
                &nominal.solution,
                &output,
                DcMatchScope::Process,
                PROCESS_OWNER,
                &sigma.parameter,
                abort,
                &mut warm_start_iterations,
                &mut warm_start_solves,
            )?;
            let down = self.displaced_output(
                &displaced(sigma.nominal - sigma.standard_deviation),
                &nominal.solution,
                &output,
                DcMatchScope::Process,
                PROCESS_OWNER,
                &sigma.parameter,
                abort,
                &mut warm_start_iterations,
                &mut warm_start_solves,
            )?;
            contributors.push(contributor(
                DcMatchScope::Process,
                PROCESS_OWNER.to_owned(),
                sigma.parameter.clone(),
                sigma.standard_deviation,
                central_difference(up, down, sigma.standard_deviation),
            ));
        }

        if !mismatch_sigmas.is_empty() {
            for instance in self.mismatch_instances(&base, abort)? {
                for sigma in &mismatch_sigmas {
                    if sigma.standard_deviation == 0.0 {
                        continue;
                    }
                    let displaced = |value: Value| {
                        let mut perturbed = base.clone();
                        perturbed.spectre_mismatch_override = Some(
                            SpectreMismatchOverride::single(&instance, &sigma.parameter, value),
                        );
                        perturbed
                    };
                    let up = self.displaced_output(
                        &displaced(sigma.nominal + sigma.standard_deviation),
                        &nominal.solution,
                        &output,
                        DcMatchScope::Mismatch,
                        &instance,
                        &sigma.parameter,
                        abort,
                        &mut warm_start_iterations,
                        &mut warm_start_solves,
                    )?;
                    let down = self.displaced_output(
                        &displaced(sigma.nominal - sigma.standard_deviation),
                        &nominal.solution,
                        &output,
                        DcMatchScope::Mismatch,
                        &instance,
                        &sigma.parameter,
                        abort,
                        &mut warm_start_iterations,
                        &mut warm_start_solves,
                    )?;
                    contributors.push(contributor(
                        DcMatchScope::Mismatch,
                        instance.clone(),
                        sigma.parameter.clone(),
                        sigma.standard_deviation,
                        central_difference(up, down, sigma.standard_deviation),
                    ));
                }
            }
        }

        let label = probe_label(card);
        log::debug!(
            ".DCMATCH {label}: {} contributor(s), {warm_start_solves} warm operating point(s), \
             {warm_start_iterations} Newton assembly/assemblies in total",
            contributors.len()
        );
        Ok(assemble(card, label, nominal_value, contributors))
    }

    /// Resolve the card's probe against the elaborated design.
    fn resolve_probe(
        &self,
        netlist: &Netlist,
        card: &DcMatchCard,
        abort: &dyn AbortSignal,
    ) -> Result<AcSensitivityOutput, SimulationError> {
        if card.output_is_current {
            return Ok(AcSensitivityOutput::BranchCurrent(card.output_node.clone()));
        }
        let resolver = super::NodeResolver::build_with_abort(self, netlist, abort)?;
        Ok(AcSensitivityOutput::Voltage {
            positive: resolver.resolve(&card.output_node, ".DCMATCH output")?,
            negative: resolver
                .resolve_reference(card.reference_node.as_deref(), ".DCMATCH reference")?,
        })
    }

    /// Every distinct mismatch scope the elaborated design contains.
    ///
    /// The rule is the elaborator's own: all primitives inside one concrete
    /// subcircuit instance share a draw, and a top-level primitive is its own
    /// scope. Enumerated in canonical order so a report does not depend on
    /// element order.
    fn mismatch_instances(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<String>, SimulationError> {
        let (flattened, _) = flatten_netlist_with_parameter_direction(
            netlist,
            FlattenerConfig {
                max_depth: self.config.resource_limits.max_hierarchy_depth,
                max_elements: self.config.resource_limits.max_flattened_elements,
                ..FlattenerConfig::default()
            },
            abort,
        )
        .map_err(|error| match error {
            crate::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
            crate::netlist::ParseWithAbortError::Parse(
                crate::netlist::ParseError::ResourceLimit(error),
            ) => SimulationError::ResourceLimit(error),
            crate::netlist::ParseWithAbortError::Parse(error) => SimulationError::Netlist(format!(
                ".DCMATCH could not elaborate the design to enumerate its mismatch scopes: {error}"
            )),
        })?;
        Ok(flattened
            .elements
            .iter()
            .map(|element| {
                super::builder::spectre_mismatch_identity(&element.name).to_ascii_uppercase()
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect())
    }

    /// One displaced operating point, warm started from the nominal solution.
    #[allow(clippy::too_many_arguments)]
    fn displaced_output(
        &self,
        displaced: &Netlist,
        nominal_solution: &[Value],
        output: &AcSensitivityOutput,
        scope: DcMatchScope,
        instance: &str,
        parameter: &str,
        abort: &dyn AbortSignal,
        iterations: &mut usize,
        solves: &mut usize,
    ) -> Result<Value, SimulationError> {
        let state = self
            .run_dc_op_state_with_startup_and_abort(
                displaced,
                DcOpStartup::PreviousSolution(nominal_solution),
                abort,
            )
            .map_err(|error| match error {
                // Cancellation, resource exhaustion and a misconfigured engine
                // are not statements about this variable, so they keep their
                // own category.
                error @ (SimulationError::Aborted
                | SimulationError::TimeLimitExceeded
                | SimulationError::ResourceLimit(_)
                | SimulationError::Configuration(_)
                | SimulationError::ModelFinished(_)) => error,
                error => SimulationError::Circuit(format!(
                    ".DCMATCH could not re-solve the operating point with {} variable '{parameter}' \
                     of '{instance}' displaced by one standard deviation: {error}",
                    scope.tag()
                )),
            })?;
        *solves = solves.saturating_add(1);
        *iterations = iterations.saturating_add(self.convergence_quality().total_iterations);
        Self::dc_sensitivity_output_value(&state.result, output)
    }
}

/// Reject a card whose numeric fields the parser could not have produced.
///
/// The parser validates every authored field, so this only fires for a card
/// built in code. It is still checked: a non-finite multiplier would travel
/// all the way into a result document before anything noticed.
fn validate_card(card: &DcMatchCard) -> Result<(), SimulationError> {
    if !card.sigma_multiplier.is_finite() || card.sigma_multiplier <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            ".DCMATCH SIGMA must be a positive finite multiple, got {}",
            card.sigma_multiplier
        )));
    }
    if !card.threshold.is_finite() || !(0.0..=1.0).contains(&card.threshold) {
        return Err(SimulationError::Circuit(format!(
            ".DCMATCH THRESHOLD must be a variance share in [0, 1], got {}",
            card.threshold
        )));
    }
    if !card.mismatch && !card.process {
        return Err(SimulationError::Circuit(
            ".DCMATCH has nothing to vary: the card selects neither the mismatch nor the process \
             scope"
                .to_owned(),
        ));
    }
    Ok(())
}

/// The scopes the card asked for, for a refusal that names what was requested.
fn requested_scopes(card: &DcMatchCard) -> &'static str {
    match (card.mismatch, card.process) {
        (true, true) => "both the mismatch and the process scope",
        (true, false) => "the mismatch scope",
        (false, true) => "the process scope",
        (false, false) => "no scope",
    }
}

/// The probe as the deck spelled it.
fn probe_label(card: &DcMatchCard) -> String {
    if card.output_is_current {
        return format!("I({})", card.output_node);
    }
    match &card.reference_node {
        Some(reference) => format!("V({},{reference})", card.output_node),
        None => format!("V({})", card.output_node),
    }
}

/// `(up - down) / (2 * sigma)`.
fn central_difference(up: Value, down: Value, sigma: Value) -> Value {
    (up - down) / (2.0 * sigma)
}

/// One contributor with its share left unset; the total is not known yet.
fn contributor(
    scope: DcMatchScope,
    instance: String,
    parameter: String,
    sigma_parameter: Value,
    sensitivity: Value,
) -> DcMatchContributor {
    DcMatchContributor {
        instance,
        parameter,
        scope,
        sigma_parameter,
        sensitivity,
        contribution: sensitivity * sigma_parameter,
        share: 0.0,
    }
}

/// Sum the variances, rank the contributors and apply the card's limits.
fn assemble(
    card: &DcMatchCard,
    output: String,
    nominal_value: Value,
    mut contributors: Vec<DcMatchContributor>,
) -> DcMatchResult {
    let variance = |scope: DcMatchScope| -> Value {
        contributors
            .iter()
            .filter(|entry| entry.scope == scope)
            .map(|entry| entry.contribution * entry.contribution)
            .sum()
    };
    let mismatch_variance = variance(DcMatchScope::Mismatch);
    let process_variance = variance(DcMatchScope::Process);
    let total_variance = mismatch_variance + process_variance;
    if total_variance > 0.0 {
        for entry in &mut contributors {
            entry.share = entry.contribution * entry.contribution / total_variance;
        }
    }
    // Largest share first. Ties break on scope, instance and parameter so a
    // report is a function of the design rather than of evaluation order.
    contributors.sort_by(|left, right| {
        right
            .share
            .total_cmp(&left.share)
            .then_with(|| left.scope.tag().cmp(right.scope.tag()))
            .then_with(|| left.instance.cmp(&right.instance))
            .then_with(|| left.parameter.cmp(&right.parameter))
    });
    let evaluated_contributors = contributors.len();
    contributors.retain(|entry| entry.share >= card.threshold);
    if card.contributor_limit > 0 {
        contributors.truncate(card.contributor_limit);
    }
    DcMatchResult {
        output,
        nominal_value,
        sigma_multiplier: card.sigma_multiplier,
        sigma_total: libm::sqrt(total_variance),
        sigma_mismatch: libm::sqrt(mismatch_variance),
        sigma_process: libm::sqrt(process_variance),
        contributors,
        evaluated_contributors,
    }
}

#[cfg(test)]
mod tests;
