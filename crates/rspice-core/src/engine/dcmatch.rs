//! `.DCMATCH`: DC mismatch variance and ranked contributors.
//!
//! # What is computed
//!
//! At the nominal DC operating point — every statistical variable at the value
//! the deterministic run uses — write `c_i = d out / d p_i * sigma_i` for the
//! signed displacement one standard deviation of variable `i` produces. The
//! output's variance is the quadratic form
//!
//! ```text
//! sigma_out^2 = c^T R c = sum_i c_i^2 + 2 * sum_{i<j} R_ij c_i c_j
//! ```
//!
//! taken over each group of variables that is drawn together, where `R` is
//! the correlation the design's own `statistics` block declares between them.
//! A design that declares no `correlate` statement has `R = I`, the cross
//! terms vanish, and this is the sum of squares the analysis has always
//! formed, without building a matrix. Products and sums retain their exponent
//! until standard deviations and shares are ready to publish.
//!
//! A mismatch variable is drawn once per instance, so each instance is its own
//! group: its variables are correlated with each other as declared and with no
//! other instance's, and the groups' quadratic forms add. A process variable
//! is one variable the whole design reads, so the process scope is a single
//! group whose derivative is taken with every instance displaced together —
//! that perfect correlation across instances is the entire difference between
//! the two scopes, and it is why the same declared spread produces a different
//! total.
//!
//! `R` is the population Pearson correlation matrix. Without bounds it is
//! the sampler target; with bounds it is the conditional correlation computed
//! jointly with the conditional standard deviations from the statistics plan
//! ([`SpectreStatisticsPlan::scope_moments_with_abort`]) and validated by the
//! call Monte Carlo makes, not the Gaussian-copula latent matrix the sampler
//! factorizes to produce a draw: a variance is a second-moment statement about
//! the variables themselves, not about the normal scores behind them. For a
//! non-Gaussian variable `sigma_i` is that distribution's own standard
//! deviation and the formula stays exact to first order; nothing else about it
//! changes.
//!
//! Each contributor's share of the variance is its Euler allocation
//! `c_i * (R c)_i / sigma_out^2`. The shares sum to exactly one and reduce to
//! `c_i^2 / sigma_out^2` when `R = I`, so an uncorrelated design reports the
//! usual independent-variance fractions. A share **can** be negative: a variable whose
//! correlated partner cancels it removes variance from the total, and that is
//! what the design says. The sign is kept, and the ranking compares
//! magnitudes.
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
//! ([`SpectreStatisticsPlan::scope_moments_with_abort`]) so a spread cannot
//! mean one thing to a Monte Carlo trial and another here. A design that
//! declares no statistics is refused by name: there is no default spread to
//! fall back on, and inventing one would answer a question the deck did not
//! ask.
//!
//! [`SpectreStatisticsPlan::scope_moments_with_abort`]: crate::netlist::SpectreStatisticsPlan::scope_moments_with_abort

use std::collections::{BTreeMap, BTreeSet};

mod moments;
use moments::{assemble, central_difference, contributor};

use super::core::DcOpStartup;
use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::dcmatch::{DcMatchContributor, DcMatchResult, DcMatchScope};
use crate::analysis::sensitivity::AcSensitivityOutput;
use crate::netlist::{
    DcMatchCard, FlattenerConfig, SpectreCorrelationMatrix, SpectreMismatchOverride,
    SpectreScopeMoments, SpectreVariationScope, StatisticalMomentOptions,
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
        self.run_dc_match_with_moment_options_and_abort(netlist, card, card.moments, abort)
    }

    /// DC mismatch with an explicit integration policy for bounded statistics.
    /// The derivatives remain at the nominal operating point. Conditional
    /// covariance is integrated separately, without Monte Carlo circuit solves.
    pub fn run_dc_match_with_moment_options_and_abort(
        &self,
        netlist: &Netlist,
        card: &DcMatchCard,
        moments: StatisticalMomentOptions,
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
        let compute_moments = |scope| {
            base.spectre_statistics
                .scope_moments_with_abort(
                    scope,
                    &base.params,
                    &nominal_process,
                    moments,
                    self.config.resource_limits,
                    abort,
                )
                .map_err(|error| match error {
                    crate::netlist::SpectreStatisticsError::Aborted => SimulationError::Aborted,
                    error => SimulationError::Circuit(error.to_string()),
                })
        };
        let mismatch = if card.mismatch {
            compute_moments(SpectreVariationScope::Mismatch)?
        } else {
            SpectreScopeMoments::default()
        };
        let process = if card.process {
            compute_moments(SpectreVariationScope::Process)?
        } else {
            SpectreScopeMoments::default()
        };
        if mismatch.sigmas.is_empty() && process.sigmas.is_empty() {
            return Err(SimulationError::Circuit(format!(
                ".DCMATCH has nothing to vary: the card selects {}, and this design's \
                 `statistics` block declares no variation there",
                requested_scopes(card)
            )));
        }
        if mismatch.evaluated_points + process.evaluated_points > 0 {
            log::info!(
                ".DCMATCH conditional moments: {} integration points, maximum relative error estimate {:.3e}",
                mismatch.evaluated_points + process.evaluated_points,
                mismatch
                    .relative_error_estimate
                    .max(process.relative_error_estimate)
            );
        }
        let mismatch_sigmas = mismatch.sigmas;
        let process_sigmas = process.sigmas;
        let correlations = ScopeCorrelations {
            mismatch: scope_correlation(
                &base,
                SpectreVariationScope::Mismatch,
                mismatch_sigmas
                    .iter()
                    .map(|sigma| sigma.parameter.clone())
                    .collect(),
                mismatch.correlation,
            )?,
            process: scope_correlation(
                &base,
                SpectreVariationScope::Process,
                process_sigmas
                    .iter()
                    .map(|sigma| sigma.parameter.clone())
                    .collect(),
                process.correlation,
            )?,
        };

        // Ranking and correlated variance need all candidate rows before the
        // authored report limit can be applied. Bound that retained numeric
        // state before allocating it or running any displaced operating points.
        let active_mismatch = mismatch_sigmas
            .iter()
            .filter(|sigma| sigma.standard_deviation != 0.0)
            .count();
        let instances = if active_mismatch == 0 {
            Vec::new()
        } else {
            self.mismatch_instances(&base, abort)?
        };
        let candidate_count = process_sigmas
            .iter()
            .filter(|sigma| sigma.standard_deviation != 0.0)
            .count()
            .saturating_add(instances.len().saturating_mul(active_mismatch));
        self.ensure_result_values(DcMatchResult::value_count_for_contributors(candidate_count))?;
        let mut contributors = Vec::new();
        contributors
            .try_reserve_exact(candidate_count)
            .map_err(|source| SimulationError::Allocation {
                object: "DC mismatch contributors",
                source,
            })?;
        let output = self.resolve_probe(&base, card, abort)?;
        let nominal = self.run_dc_op_state_with_startup_and_abort(
            &base,
            DcOpStartup::Automatic { use_hints: true },
            abort,
        )?;
        let nominal_value = Self::dc_sensitivity_output_value(&nominal.result, &output)?;

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
                central_difference(up, down, sigma.standard_deviation)?,
            )?);
        }

        if !mismatch_sigmas.is_empty() {
            for instance in instances {
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
                        central_difference(up, down, sigma.standard_deviation)?,
                    )?);
                }
            }
        }

        let label = probe_label(card);
        log::debug!(
            ".DCMATCH {label}: {} contributor(s), {warm_start_solves} warm operating point(s), \
             {warm_start_iterations} Newton assembly/assemblies in total",
            contributors.len()
        );
        assemble(
            card,
            label,
            nominal_value,
            contributors,
            &correlations,
            abort,
        )
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

/// One scope's declared correlation, with the order its rows are in.
struct ScopeCorrelation {
    /// How many `correlate` statements of this scope the result applied.
    statements: usize,
    /// Canonical (upper-case) parameter names in the matrix's own row order —
    /// the order [`crate::netlist::SpectreStatisticsPlan::scope_moments_with_abort`]
    /// reports, which is the identity the sampler keys its draws by.
    order: Vec<String>,
    matrix: SpectreCorrelationMatrix,
}

impl ScopeCorrelation {
    /// Which row of the matrix one contributor's variable occupies.
    fn row(&self, scope: DcMatchScope, parameter: &str) -> Result<usize, SimulationError> {
        self.order
            .iter()
            .position(|name| name == parameter)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".DCMATCH formed a {} contributor for '{parameter}', which is not one of the \
                     variables that scope's correlation matrix is over; this is an internal error",
                    scope.tag()
                ))
            })
    }
}

/// What each scope declares about its own variables.
struct ScopeCorrelations {
    mismatch: Option<ScopeCorrelation>,
    process: Option<ScopeCorrelation>,
}

impl ScopeCorrelations {
    fn for_scope(&self, scope: DcMatchScope) -> Option<&ScopeCorrelation> {
        match scope {
            DcMatchScope::Mismatch => self.mismatch.as_ref(),
            DcMatchScope::Process => self.process.as_ref(),
        }
    }

    fn statements(&self, scope: DcMatchScope) -> usize {
        self.for_scope(scope)
            .map_or(0, |correlation| correlation.statements)
    }
}

/// Read one scope's `correlate` statements out of the design's statistics
/// plan, in the order the scope's contributions are formed.
///
/// `None` when the scope contributes nothing to this card, or when the design
/// declares no correlation for it — the uncorrelated path then runs untouched.
fn scope_correlation(
    netlist: &Netlist,
    scope: SpectreVariationScope,
    order: Vec<String>,
    matrix: Option<SpectreCorrelationMatrix>,
) -> Result<Option<ScopeCorrelation>, SimulationError> {
    if order.is_empty() {
        return Ok(None);
    }
    let Some(matrix) = matrix else {
        return Ok(None);
    };
    if matrix.values().len() != order.len() {
        return Err(SimulationError::Circuit(format!(
            ".DCMATCH read a {} correlation matrix of {} variable(s) for a scope that resolves \
             {} standard deviation(s); this is an internal error",
            scope_tag(scope),
            matrix.values().len(),
            order.len()
        )));
    }
    Ok(Some(ScopeCorrelation {
        statements: netlist
            .spectre_statistics
            .correlations
            .iter()
            .filter(|correlation| correlation.scope == scope)
            .count(),
        order,
        matrix,
    }))
}

/// The statistical scope's own tag, for a diagnostic that names it.
fn scope_tag(scope: SpectreVariationScope) -> &'static str {
    match scope {
        SpectreVariationScope::Process => DcMatchScope::Process.tag(),
        SpectreVariationScope::Mismatch => DcMatchScope::Mismatch.tag(),
    }
}

#[cfg(test)]
mod tests;
