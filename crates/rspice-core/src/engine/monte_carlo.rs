use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::monte_carlo::{
    Distribution, MonteCarloResult, VariableStatistics, Xorshift128Plus,
};
use crate::netlist::{ElementKind, SourceSpec};
use crate::{Netlist, Value};
use std::collections::{HashMap, HashSet};

/// Exact operating environment applied to every Monte Carlo trial after any
/// parameter-driven source reparse.
#[derive(Debug, Clone, PartialEq)]
pub struct MonteCarloEnvironment {
    pub temperature_celsius: Value,
    pub supply_voltage: Option<Value>,
    pub nominal_supply_voltage: Option<Value>,
    /// Exact independent voltage-source instance names that form the supply
    /// domain. A voltage corner is never inferred from circuit topology: bias,
    /// reference, and stimulus sources may also be ground referenced.
    pub supply_source_names: Vec<String>,
}

/// Scale only the explicitly bound independent DC supply sources.
///
/// Selection is resolved completely before mutation so a stale binding cannot
/// leave a partially scaled circuit.
pub fn apply_supply_voltage_scale_with_abort(
    netlist: &mut Netlist,
    supply: Value,
    nominal: Value,
    source_names: &[String],
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !supply.is_finite() || supply <= 0.0 {
        return Err(SimulationError::Circuit(
            "Corner voltage must be a positive finite value".to_owned(),
        ));
    }
    if !nominal.is_finite() || nominal <= 0.0 {
        return Err(SimulationError::Circuit(
            "Corner nominal voltage must be a positive finite value".to_owned(),
        ));
    }
    if source_names.is_empty() {
        return Err(SimulationError::Circuit(
            "Supply scaling requires at least one explicitly bound independent voltage source"
                .to_owned(),
        ));
    }
    let mut normalized = std::collections::BTreeSet::new();
    let mut candidates = Vec::with_capacity(source_names.len());
    for source_name in source_names {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let trimmed = source_name.trim();
        if trimmed.is_empty()
            || trimmed != source_name
            || trimmed.chars().any(char::is_control)
            || !normalized.insert(trimmed.to_ascii_lowercase())
        {
            return Err(SimulationError::Circuit(format!(
                "Supply source binding {source_name:?} is empty, malformed, or duplicated"
            )));
        }
        let Some((index, element)) = netlist
            .elements
            .iter()
            .enumerate()
            .find(|(_, element)| element.name.eq_ignore_ascii_case(trimmed))
        else {
            return Err(SimulationError::Circuit(format!(
                "Bound supply source {trimmed:?} is not present in the executable netlist"
            )));
        };
        match &element.kind {
            ElementKind::VoltageSource(spec) if scalable_dc_value(spec).is_some() => {
                candidates.push(index);
            }
            ElementKind::VoltageSource(_) => {
                return Err(SimulationError::Circuit(format!(
                    "Bound supply source {trimmed:?} has no scalable independent DC value"
                )));
            }
            _ => {
                return Err(SimulationError::Circuit(format!(
                    "Bound supply source {trimmed:?} is not an independent voltage source"
                )));
            }
        }
    }
    let scale = supply / nominal;
    for index in candidates {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let ElementKind::VoltageSource(spec) = &mut netlist.elements[index].kind else {
            continue;
        };
        if let Some(dc) = scalable_dc_value(spec) {
            Engine::set_source_dc_value(spec, dc * scale)?;
        }
    }
    Ok(())
}

fn scalable_dc_value(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(value) => Some(*value),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

impl Engine {
    /// Run Monte Carlo analysis
    ///
    /// Performs multiple simulation runs with random component variations.
    pub fn run_monte_carlo(
        &self,
        netlist: &Netlist,
        num_runs: usize,
        seed: u64,
    ) -> Result<MonteCarloResult, SimulationError> {
        self.run_monte_carlo_with_options(
            netlist,
            num_runs,
            seed,
            Distribution::Gaussian { sigma: 0.01 },
            None,
        )
    }

    /// Run Monte Carlo analysis with configurable distribution and parameter filter.
    pub fn run_monte_carlo_with_options(
        &self,
        netlist: &Netlist,
        num_runs: usize,
        seed: u64,
        distribution: Distribution,
        parameter_filter: Option<&[String]>,
    ) -> Result<MonteCarloResult, SimulationError> {
        self.run_monte_carlo_with_options_and_abort(
            netlist,
            num_runs,
            seed,
            distribution,
            parameter_filter,
            &NoAbort,
        )
    }

    /// Run Monte Carlo analysis with cooperative cancellation during deck
    /// generation and independent operating-point solves.
    pub fn run_monte_carlo_with_options_and_abort(
        &self,
        netlist: &Netlist,
        num_runs: usize,
        seed: u64,
        distribution: Distribution,
        parameter_filter: Option<&[String]>,
        abort: &dyn AbortSignal,
    ) -> Result<MonteCarloResult, SimulationError> {
        self.run_monte_carlo_with_options_environment_and_abort(
            netlist,
            num_runs,
            seed,
            distribution,
            parameter_filter,
            None,
            abort,
        )
    }

    /// Run Monte Carlo under one exact operating environment. Parameter
    /// perturbation reparses the authored source per trial, so the environment
    /// is deliberately applied after that reparse and before each solve.
    pub fn run_monte_carlo_with_options_environment_and_abort(
        &self,
        netlist: &Netlist,
        num_runs: usize,
        seed: u64,
        distribution: Distribution,
        parameter_filter: Option<&[String]>,
        environment: Option<MonteCarloEnvironment>,
        abort: &dyn AbortSignal,
    ) -> Result<MonteCarloResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.ensure_valid_configuration()?;
        if num_runs == 0 {
            return Err(SimulationError::Circuit(
                "Monte Carlo requires at least one run".to_string(),
            ));
        }
        self.ensure_batch_runs(num_runs)?;
        let spread = match distribution {
            Distribution::Gaussian { sigma } => sigma,
            Distribution::Uniform { tolerance } => tolerance,
            Distribution::WorstCase { tolerance } => tolerance,
        };
        if !spread.is_finite() || spread < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Monte Carlo spread must be finite and non-negative, got {}",
                spread
            )));
        }

        let normalized_filter: Option<HashSet<String>> = parameter_filter.and_then(|params| {
            let normalized: HashSet<String> = params
                .iter()
                .map(|p| p.trim().to_ascii_uppercase())
                .filter(|p| !p.is_empty())
                .collect();
            if normalized.is_empty() {
                None
            } else {
                Some(normalized)
            }
        });
        let has_spectre_statistics = !netlist.spectre_statistics.variations.is_empty();
        if has_spectre_statistics && normalized_filter.is_some() {
            return Err(SimulationError::Circuit(
                "A generic Monte Carlo parameter filter cannot be combined with native Spectre statistics; declare every varied parameter in the Spectre statistics block"
                    .to_owned(),
            ));
        }

        let mut all_eligible_params: Vec<(String, Value)> = if has_spectre_statistics {
            Vec::new()
        } else {
            netlist
                .params
                .all_params()
                .into_iter()
                .filter(|(_, value)| value.is_finite() && value.abs() > 0.0)
                .collect()
        };
        all_eligible_params.sort_by(|a, b| a.0.cmp(&b.0));

        if let Some(filter) = &normalized_filter {
            let available: HashSet<String> = all_eligible_params
                .iter()
                .map(|(name, _)| name.clone())
                .collect();
            let mut unknown: Vec<String> = filter
                .iter()
                .filter(|name| !available.contains(*name))
                .cloned()
                .collect();
            unknown.sort();
            if !unknown.is_empty() {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo parameter(s) not defined or not eligible: {}",
                    unknown.join(", ")
                )));
            }
        }

        let mut monte_params: Vec<(String, Value)> = all_eligible_params
            .into_iter()
            .filter(|(name, _)| {
                normalized_filter
                    .as_ref()
                    .map(|filter| filter.contains(name))
                    .unwrap_or(true)
            })
            .collect();

        if normalized_filter.is_some() && monte_params.is_empty() {
            return Err(SimulationError::Circuit(
                "Monte Carlo parameter filter did not match any eligible parameters".to_string(),
            ));
        }

        if let Some(source) = &netlist.source_text {
            let mut bound_params = Vec::new();
            let mut unbound_params = Vec::new();
            for (name, nominal) in std::mem::take(&mut monte_params) {
                if Self::source_references_param(source, &name) {
                    bound_params.push((name, nominal));
                } else {
                    unbound_params.push(name);
                }
            }

            if normalized_filter.is_some() && !unbound_params.is_empty() {
                unbound_params.sort();
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo parameter(s) are not bound to any netlist expression: {}",
                    unbound_params.join(", ")
                )));
            }
            if !bound_params.is_empty() {
                monte_params = bound_params;
            } else if !unbound_params.is_empty() {
                return Err(SimulationError::Circuit(
                    "Monte Carlo parameter set is not bound to any netlist expression".to_string(),
                ));
            }
        }

        // Phase 1 (serial): draw every run's compact variation vector from
        // the seeded stream. Sampling order remains byte-identical across
        // worker counts, but full netlists are no longer cloned and retained
        // for every run. Each worker materializes one deck immediately before
        // its solve and drops it immediately afterward.
        let mut rng = Xorshift128Plus::new(seed);
        let mut run_variations = if monte_params.is_empty() {
            Vec::new()
        } else {
            Vec::with_capacity(num_runs)
        };
        if !monte_params.is_empty() {
            self.ensure_result_shape(num_runs, monte_params.len())?;
            for _run in 0..num_runs {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let variations: Vec<Value> = monte_params
                    .iter()
                    .map(|(_, nominal)| {
                        Self::sample_monte_carlo_value(&mut rng, *nominal, distribution)
                    })
                    .collect();
                run_variations.push(variations);
            }
        }

        let resource_limits = self.config.resource_limits;
        let inherited_statistical_axes = netlist
            .spectre_statistical_coordinate
            .as_ref()
            .map(|coordinate| coordinate.axes.clone())
            .unwrap_or_default();
        let materialize_run =
            |run_index: usize| -> Result<std::borrow::Cow<'_, Netlist>, SimulationError> {
                if monte_params.is_empty() {
                    if environment.is_none() && !has_spectre_statistics {
                        return Ok(std::borrow::Cow::Borrowed(netlist));
                    }
                    let mut materialized = netlist.clone();
                    if let Some(environment) = environment.as_ref() {
                        Self::apply_monte_carlo_environment(&mut materialized, environment, abort)?;
                    }
                    if has_spectre_statistics {
                        let temperature_celsius = environment
                            .as_ref()
                            .map(|environment| environment.temperature_celsius)
                            .or(materialized.options.temp)
                            .unwrap_or_else(|| {
                                crate::constants::kelvin_to_celsius(self.config.temperature)
                            });
                        materialized.spectre_statistical_coordinate =
                            Some(crate::netlist::SpectreStatisticalCoordinate {
                                seed,
                                monte_carlo_run: run_index as u64,
                                temperature_celsius,
                                axes: inherited_statistical_axes.clone(),
                            });
                    }
                    return Ok(std::borrow::Cow::Owned(materialized));
                }
                let overrides = monte_params
                    .iter()
                    .zip(&run_variations[run_index])
                    .map(|((name, _), value)| (name.clone(), *value))
                    .collect::<Vec<_>>();
                let (mut perturbed, _) =
                    Self::create_perturbed_netlist_multi_with_limits_and_abort(
                        netlist,
                        &overrides,
                        resource_limits,
                        abort,
                    )?;
                if let Some(environment) = environment.as_ref() {
                    Self::apply_monte_carlo_environment(&mut perturbed, environment, abort)?;
                }
                if has_spectre_statistics {
                    let temperature_celsius = environment
                        .as_ref()
                        .map(|environment| environment.temperature_celsius)
                        .or(perturbed.options.temp)
                        .unwrap_or_else(|| {
                            crate::constants::kelvin_to_celsius(self.config.temperature)
                        });
                    perturbed.spectre_statistical_coordinate =
                        Some(crate::netlist::SpectreStatisticalCoordinate {
                            seed,
                            monte_carlo_run: run_index as u64,
                            temperature_celsius,
                            axes: inherited_statistical_axes.clone(),
                        });
                }
                Ok(std::borrow::Cow::Owned(perturbed))
            };

        // Phase 2 (parallel): solve the independent runs across worker
        // threads, each with its own engine. Index-addressed slots keep
        // results in run order, so statistics match a serial sweep exactly;
        // failed runs are skipped just as before.
        // (node voltages, node names) of a converged run; None = failed run.
        type RunResult = Option<(Vec<Value>, Vec<String>)>;
        type RunOutcome = Result<RunResult, SimulationError>;

        let workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(self.config.resource_limits.max_parallel_workers)
            .min(num_runs.max(1));
        let mut run_outcomes: Vec<RunOutcome> = Vec::new();
        if workers <= 1 {
            for run_index in 0..num_runs {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let run_netlist = materialize_run(run_index)?;
                let outcome = match self.run_dc_op_with_abort(&run_netlist, abort) {
                    Ok(result) => Ok(Some((result.node_voltages, result.node_names))),
                    Err(error @ SimulationError::Aborted)
                    | Err(error @ SimulationError::ResourceLimit(_))
                    | Err(error @ SimulationError::Configuration(_)) => Err(error),
                    Err(_) => Ok(None),
                };
                run_outcomes.push(outcome);
            }
        } else {
            use std::sync::Mutex;
            use std::sync::atomic::{AtomicUsize, Ordering};

            let next = AtomicUsize::new(0);
            let slots: Vec<Mutex<Option<RunOutcome>>> =
                (0..num_runs).map(|_| Mutex::new(None)).collect();
            let mut worker_config = self.config().clone();
            // Independent Monte Carlo runs already consume the complete
            // worker budget. Keep each child engine serial so a future
            // internally parallel DC path cannot multiply the thread count.
            worker_config.resource_limits.max_parallel_workers = 1;

            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| {
                        let engine = Self::new(worker_config.clone());
                        loop {
                            if abort.is_aborted() {
                                break;
                            }
                            let index = next.fetch_add(1, Ordering::SeqCst);
                            if index >= num_runs {
                                break;
                            }
                            let run_netlist = match materialize_run(index) {
                                Ok(run_netlist) => run_netlist,
                                Err(error) => {
                                    *slots[index].lock().expect("mc slot") = Some(Err(error));
                                    continue;
                                }
                            };
                            let outcome = match engine.run_dc_op_with_abort(&run_netlist, abort) {
                                Ok(result) => Ok(Some((result.node_voltages, result.node_names))),
                                Err(error @ SimulationError::Aborted)
                                | Err(error @ SimulationError::ResourceLimit(_))
                                | Err(error @ SimulationError::Configuration(_)) => Err(error),
                                Err(_) => Ok(None),
                            };
                            *slots[index].lock().expect("mc slot") = Some(outcome);
                        }
                    });
                }
            });

            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            run_outcomes = slots
                .into_iter()
                .map(|slot| {
                    slot.into_inner()
                        .expect("mc slot lock")
                        .expect("every Monte Carlo slot is processed without cancellation")
                })
                .collect();
        }

        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let run_outcomes = run_outcomes.into_iter().collect::<Result<Vec<_>, _>>()?;
        self.monte_carlo_result_from_trials(run_outcomes.into_iter().flatten(), num_runs)
    }

    fn apply_monte_carlo_environment(
        netlist: &mut Netlist,
        environment: &MonteCarloEnvironment,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !environment.temperature_celsius.is_finite()
            || crate::constants::celsius_to_kelvin(environment.temperature_celsius) <= 0.0
        {
            return Err(SimulationError::Circuit(
                "Monte Carlo environment temperature must be finite and above absolute zero"
                    .to_owned(),
            ));
        }
        netlist.options.temp = Some(environment.temperature_celsius);
        match (
            environment.supply_voltage,
            environment.nominal_supply_voltage,
        ) {
            (Some(supply), Some(nominal)) => {
                apply_supply_voltage_scale_with_abort(
                    netlist,
                    supply,
                    nominal,
                    &environment.supply_source_names,
                    abort,
                )?;
            }
            (None, None) => {}
            _ => {
                return Err(SimulationError::Circuit(
                    "Monte Carlo environment supply and nominal voltage must be provided together"
                        .to_owned(),
                ));
            }
        }
        Ok(())
    }

    /// Aggregate the converged trials into the reported distribution.
    ///
    /// Both Monte Carlo drivers reach this. They differ only in how a trial's
    /// deck is produced — one perturbs eligible parameter values numerically,
    /// the other redraws the deck's own statistical expressions from a fresh
    /// seed — so what a reported mean, sigma, or histogram means has one owner
    /// rather than one per driver.
    ///
    /// `requested_runs` is the number of trials asked for, which is what makes
    /// the failure count and the all-converged flag answerable here.
    pub fn monte_carlo_result_from_trials(
        &self,
        trials: impl IntoIterator<Item = (Vec<Value>, Vec<String>)>,
        requested_runs: usize,
    ) -> Result<MonteCarloResult, SimulationError> {
        if requested_runs == 0 {
            return Err(SimulationError::Circuit(
                "Monte Carlo requires at least one requested trial".to_owned(),
            ));
        }
        let mut results = Vec::with_capacity(requested_runs);
        let mut first_node_names: Option<Vec<String>> = None;
        let mut retained_values = 0usize;
        for (trial_index, (node_voltages, node_names)) in trials.into_iter().enumerate() {
            if node_names.is_empty() || node_names.len() != node_voltages.len() {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo trial {} returned {} node names for {} voltages",
                    trial_index + 1,
                    node_names.len(),
                    node_voltages.len()
                )));
            }
            if node_voltages.iter().any(|value| !value.is_finite()) {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo trial {} returned a non-finite node voltage",
                    trial_index + 1
                )));
            }
            let mut normalized_names = std::collections::HashSet::with_capacity(node_names.len());
            if node_names.iter().any(|name| {
                let normalized = name.trim().to_ascii_uppercase();
                normalized.is_empty() || !normalized_names.insert(normalized)
            }) {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo trial {} returned an empty or duplicate node identity",
                    trial_index + 1
                )));
            }
            if let Some(reference_names) = &first_node_names {
                if node_names.len() != reference_names.len()
                    || node_names
                        .iter()
                        .zip(reference_names)
                        .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
                {
                    return Err(SimulationError::Circuit(format!(
                        "Monte Carlo trial {} changed the solved node basis",
                        trial_index + 1
                    )));
                }
            } else {
                first_node_names = Some(node_names);
            }
            retained_values = retained_values.saturating_add(node_voltages.len());
            self.ensure_result_values(retained_values)?;
            results.push(node_voltages);
        }
        if results.is_empty() {
            return Err(SimulationError::Circuit(
                "Monte Carlo produced no converged trials".to_owned(),
            ));
        }

        // Compute statistics for each non-ground node.
        // node_voltages[0] is always ground.
        let max_node_id = results
            .first()
            .map(|r| r.len().saturating_sub(1))
            .unwrap_or(0);
        let mut variables: HashMap<String, VariableStatistics> = HashMap::new();
        let mut output_values = 0usize;

        let variable_value_count = |statistics: &VariableStatistics| {
            statistics
                .samples
                .len()
                .saturating_add(statistics.histogram.len())
                .saturating_add(statistics.bin_edges.len())
                .saturating_add(4)
        };

        for node_id in 1..=max_node_id {
            let samples: Vec<Value> = results.iter().map(|result| result[node_id]).collect();

            if !samples.is_empty() {
                let numeric_name = format!("V({})", node_id);
                let numeric_label = numeric_name.clone();
                let stats = VariableStatistics::from_samples(&numeric_name, samples.clone(), 20);
                output_values = output_values.saturating_add(variable_value_count(&stats));
                self.ensure_result_values(retained_values.saturating_add(output_values))?;
                variables.insert(numeric_name, stats);

                if let Some(node_names) = &first_node_names
                    && let Some(node_name) = node_names.get(node_id)
                {
                    let named_key = format!("V({})", node_name);
                    if named_key != numeric_label {
                        let alias_stats = VariableStatistics::from_samples(&named_key, samples, 20);
                        output_values =
                            output_values.saturating_add(variable_value_count(&alias_stats));
                        self.ensure_result_values(retained_values.saturating_add(output_values))?;
                        variables.insert(named_key, alias_stats);
                    }
                }
            }
        }

        Ok(MonteCarloResult {
            num_runs: results.len(),
            variables,
            all_converged: results.len() == requested_runs,
            num_failures: requested_runs.saturating_sub(results.len()),
        })
    }

    pub(in crate::engine) fn sample_monte_carlo_value(
        rng: &mut Xorshift128Plus,
        nominal: Value,
        distribution: Distribution,
    ) -> Value {
        let magnitude = nominal.abs();
        match distribution {
            Distribution::Gaussian { sigma } => {
                let sigma = sigma.abs();
                nominal + rng.next_gaussian() * magnitude * sigma
            }
            Distribution::Uniform { tolerance } => {
                let tolerance = tolerance.abs();
                let delta = magnitude * tolerance;
                nominal + (2.0 * rng.next_f64() - 1.0) * delta
            }
            Distribution::WorstCase { tolerance } => {
                let tolerance = tolerance.abs();
                let delta = magnitude * tolerance;
                let sign = if (rng.next_u64() & 1) == 0 { -1.0 } else { 1.0 };
                nominal + sign * delta
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimulationConfig;
    use crate::abort_signal::{AbortSignal, NoAbort};

    fn dc_sources(netlist: &Netlist) -> Vec<Value> {
        netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::VoltageSource(spec) => scalable_dc_value(spec),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn supply_scaling_changes_only_explicitly_bound_sources() {
        let mut netlist =
            Netlist::parse("scale\nVDD vdd 0 1\nVFLOAT a b 3\nR1 vdd 0 1k\nR2 a b 1k\n.end\n")
                .expect("deck parses");

        apply_supply_voltage_scale_with_abort(
            &mut netlist,
            2.0,
            1.0,
            &["VDD".to_owned()],
            &NoAbort,
        )
        .expect("scale applies");

        assert_eq!(dc_sources(&netlist), vec![2.0, 3.0]);
    }

    #[test]
    fn supply_scaling_supports_explicit_floating_supply_domains() {
        let mut netlist = Netlist::parse("scale\nV1 a b 1\nV2 c d 3\nR1 a b 1k\nR2 c d 1k\n.end\n")
            .expect("deck parses");

        apply_supply_voltage_scale_with_abort(
            &mut netlist,
            0.5,
            1.0,
            &["V1".to_owned(), "V2".to_owned()],
            &NoAbort,
        )
        .expect("bound scale applies");

        assert_eq!(dc_sources(&netlist), vec![0.5, 1.5]);
    }

    #[test]
    fn supply_scaling_refuses_an_unbound_or_stale_domain_without_mutation() {
        let mut netlist =
            Netlist::parse("scale\nVDD vdd 0 1\nVBIAS bias 0 0.5\n.end\n").expect("deck parses");
        let original = dc_sources(&netlist);

        let error = apply_supply_voltage_scale_with_abort(
            &mut netlist,
            0.9,
            1.0,
            &["VMISSING".to_owned()],
            &NoAbort,
        )
        .expect_err("stale binding is refused");

        assert!(error.to_string().contains("VMISSING"));
        assert_eq!(dc_sources(&netlist), original);
    }

    struct Aborted;

    impl AbortSignal for Aborted {
        fn is_aborted(&self) -> bool {
            true
        }
    }

    #[test]
    fn supply_scaling_preserves_typed_abort() {
        let mut netlist =
            Netlist::parse("scale\nV1 a 0 1\nR1 a 0 1k\n.end\n").expect("deck parses");

        assert!(matches!(
            apply_supply_voltage_scale_with_abort(
                &mut netlist,
                2.0,
                1.0,
                &["VDD".to_owned()],
                &Aborted,
            ),
            Err(SimulationError::Aborted)
        ));
    }

    #[test]
    fn native_spectre_monte_carlo_is_seeded_and_parallel_order_independent() {
        let plan = crate::netlist::SpectreStatisticsPlan {
            variations: vec![crate::netlist::SpectreVariation {
                line: 3,
                scope: crate::netlist::SpectreVariationScope::Process,
                parameter: "rtop".to_owned(),
                distribution: crate::netlist::SpectreDistribution::Gaussian,
                spread: crate::netlist::SpectreSpread::StandardDeviation("100".to_owned()),
                percent: false,
            }],
            correlations: vec![],
        };
        let deck = Netlist::parse(&format!(
            "native Spectre Monte Carlo\n.param rtop=1k\n.RSPICE_SPECTRE_STAT {}\nV1 in 0 1\nR1 in out {{rtop}}\nR2 out 0 1k\n.end\n",
            plan.encode_internal()
        ))
        .expect("statistical divider parses");

        let mut serial_config = SimulationConfig::default();
        serial_config.resource_limits.max_parallel_workers = 1;
        let mut parallel_config = serial_config.clone();
        parallel_config.resource_limits.max_parallel_workers = 4;
        let serial = Engine::new(serial_config)
            .run_monte_carlo(&deck, 24, 0x1234_5678)
            .expect("serial Monte Carlo converges");
        let replay = Engine::new(parallel_config.clone())
            .run_monte_carlo(&deck, 24, 0x1234_5678)
            .expect("parallel Monte Carlo converges");
        let mut unrelated = deck.clone();
        unrelated.params.set("UNRELATED_NOMINAL", 999.0);
        let unrelated_replay = Engine::new(parallel_config.clone())
            .run_monte_carlo(&unrelated, 24, 0x1234_5678)
            .expect("unrelated nominal parameter does not alter the run coordinate");
        let mut outer_axis_one = deck.clone();
        outer_axis_one.spectre_statistical_coordinate =
            Some(crate::netlist::SpectreStatisticalCoordinate {
                axes: vec![("outer_step".into(), 1.0)],
                ..Default::default()
            });
        let mut outer_axis_two = outer_axis_one.clone();
        outer_axis_two
            .spectre_statistical_coordinate
            .as_mut()
            .expect("outer coordinate exists")
            .axes[0]
            .1 = 2.0;
        let axis_one = Engine::new(parallel_config.clone())
            .run_monte_carlo(&outer_axis_one, 24, 0x1234_5678)
            .expect("first composed outer coordinate converges");
        let axis_two = Engine::new(parallel_config.clone())
            .run_monte_carlo(&outer_axis_two, 24, 0x1234_5678)
            .expect("second composed outer coordinate converges");
        let changed_seed = Engine::new(parallel_config)
            .run_monte_carlo(&deck, 24, 0x1234_5679)
            .expect("changed-seed Monte Carlo converges");

        let bits = |result: &MonteCarloResult| {
            result
                .variables
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
                .expect("named output node is reported")
                .1
                .samples
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        assert_eq!(serial.num_runs, 24);
        assert_eq!(serial.num_failures, 0);
        assert_eq!(bits(&serial), bits(&replay));
        assert_eq!(bits(&serial), bits(&unrelated_replay));
        assert_ne!(bits(&axis_one), bits(&axis_two));
        assert_ne!(bits(&serial), bits(&changed_seed));
    }
}
