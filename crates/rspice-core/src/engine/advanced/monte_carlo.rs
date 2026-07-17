use super::*;

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

        let mut all_eligible_params: Vec<(String, Value)> = netlist
            .params
            .all_params()
            .into_iter()
            .filter(|(_, value)| value.is_finite() && value.abs() > 0.0)
            .collect();
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

        let materialize_run =
            |run_index: usize| -> Result<std::borrow::Cow<'_, Netlist>, SimulationError> {
                if monte_params.is_empty() {
                    return Ok(std::borrow::Cow::Borrowed(netlist));
                }
                let overrides = monte_params
                    .iter()
                    .zip(&run_variations[run_index])
                    .map(|((name, _), value)| (name.clone(), *value))
                    .collect::<Vec<_>>();
                let (perturbed, _) = Self::create_perturbed_netlist_multi(netlist, &overrides)?;
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
            let config = self.config().clone();

            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| {
                        let engine = Self::new(config.clone());
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
        let mut results = Vec::with_capacity(num_runs);
        let mut first_node_names: Option<Vec<String>> = None;
        let mut retained_values = 0usize;
        for (node_voltages, node_names) in run_outcomes.into_iter().flatten() {
            if first_node_names.is_none() {
                first_node_names = Some(node_names);
            }
            retained_values = retained_values.saturating_add(node_voltages.len());
            self.ensure_result_values(retained_values)?;
            results.push(node_voltages);
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
            let samples: Vec<Value> = results
                .iter()
                .filter_map(|r| r.get(node_id).copied())
                .collect();

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
            all_converged: results.len() == num_runs,
            num_failures: num_runs - results.len(),
        })
    }

    pub(in crate::engine::advanced) fn sample_monte_carlo_value(
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
