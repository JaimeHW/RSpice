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

        // Phase 1 (serial): draw every run's variations from the seeded
        // stream and build the perturbed netlists. Sampling order is
        // byte-identical to the historical serial implementation, so a
        // given seed reproduces the same runs regardless of how phase 2
        // schedules them.
        let mut rng = Xorshift128Plus::new(seed);
        let mut run_netlists = Vec::with_capacity(num_runs);
        for _run in 0..num_runs {
            let netlist_for_run = if monte_params.is_empty() {
                netlist.clone()
            } else {
                let overrides: Vec<(String, Value)> = monte_params
                    .iter()
                    .map(|(name, nominal)| {
                        let varied =
                            Self::sample_monte_carlo_value(&mut rng, *nominal, distribution);
                        (name.clone(), varied)
                    })
                    .collect();
                let (perturbed, _) = Self::create_perturbed_netlist_multi(netlist, &overrides)?;
                perturbed
            };
            run_netlists.push(netlist_for_run);
        }

        // Phase 2 (parallel): solve the independent runs across worker
        // threads, each with its own engine. Index-addressed slots keep
        // results in run order, so statistics match a serial sweep exactly;
        // failed runs are skipped just as before.
        let workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(num_runs.max(1));
        let mut run_outcomes: Vec<Option<(Vec<Value>, Vec<String>)>> = Vec::new();
        if workers <= 1 {
            for run_netlist in &run_netlists {
                run_outcomes.push(
                    self.run_dc_op(run_netlist)
                        .ok()
                        .map(|result| (result.node_voltages, result.node_names)),
                );
            }
        } else {
            use std::sync::Mutex;
            use std::sync::atomic::{AtomicUsize, Ordering};

            let next = AtomicUsize::new(0);
            let slots: Vec<Mutex<Option<(Vec<Value>, Vec<String>)>>> =
                (0..run_netlists.len()).map(|_| Mutex::new(None)).collect();
            let config = self.config().clone();

            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| {
                        let engine = Self::new(config.clone());
                        loop {
                            let index = next.fetch_add(1, Ordering::SeqCst);
                            if index >= run_netlists.len() {
                                break;
                            }
                            if let Ok(result) = engine.run_dc_op(&run_netlists[index]) {
                                *slots[index].lock().expect("mc slot") =
                                    Some((result.node_voltages, result.node_names));
                            }
                        }
                    });
                }
            });

            run_outcomes = slots
                .into_iter()
                .map(|slot| slot.into_inner().expect("mc slot lock"))
                .collect();
        }

        let mut results = Vec::with_capacity(num_runs);
        let mut first_node_names: Option<Vec<String>> = None;
        for (node_voltages, node_names) in run_outcomes.into_iter().flatten() {
            if first_node_names.is_none() {
                first_node_names = Some(node_names);
            }
            results.push(node_voltages);
        }

        // Compute statistics for each non-ground node.
        // node_voltages[0] is always ground.
        let max_node_id = results
            .first()
            .map(|r| r.len().saturating_sub(1))
            .unwrap_or(0);
        let mut variables: HashMap<String, VariableStatistics> = HashMap::new();

        for node_id in 1..=max_node_id {
            let samples: Vec<Value> = results
                .iter()
                .filter_map(|r| r.get(node_id).copied())
                .collect();

            if !samples.is_empty() {
                let numeric_name = format!("V({})", node_id);
                let numeric_label = numeric_name.clone();
                let stats = VariableStatistics::from_samples(&numeric_name, samples.clone(), 20);
                variables.insert(numeric_name, stats);

                if let Some(node_names) = &first_node_names
                    && let Some(node_name) = node_names.get(node_id)
                {
                    let named_key = format!("V({})", node_name);
                    if named_key != numeric_label {
                        let alias_stats = VariableStatistics::from_samples(&named_key, samples, 20);
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
