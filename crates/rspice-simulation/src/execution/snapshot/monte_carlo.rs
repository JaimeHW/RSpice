//! Route selected trial populations against fully materialized per-point decks.
use super::*;

impl PreparedTask {
    pub(crate) fn with_monte_carlo_resumes(
        mut self,
        mut resumes: Vec<PreparedMonteCarloResume>,
    ) -> Self {
        resumes.sort_by_key(PreparedMonteCarloResume::population_identity);
        self.monte_carlo_resumes = resumes.into();
        self.config_digest = self.payload_digest();
        self
    }
    /// Captured checkpoint inputs before routing to a materialized run point.
    pub fn monte_carlo_resumes(&self) -> &[PreparedMonteCarloResume] {
        &self.monte_carlo_resumes
    }
}

fn invalid(message: impl Into<String>) -> PreparationError {
    PreparationError::new(PreparationStage::AnalysisPlan, message)
}

pub(super) fn route_resumes(
    tasks: &mut [PreparedTask],
    source: &str,
    context: crate::engine_services::ServiceContext<'_>,
) -> Result<(), PreparationError> {
    let mut selections: HashMap<AnalysisInstanceId, Arc<[PreparedMonteCarloResume]>> =
        HashMap::new();
    let mut required = HashSet::new();
    for task in tasks
        .iter()
        .filter(|task| !task.monte_carlo_resumes.is_empty())
    {
        PreparationError::check_abort(context.abort)?;
        if let Some(existing) = selections.get(&task.authored_instance_id) {
            if !Arc::ptr_eq(existing, &task.monte_carlo_resumes) {
                return Err(invalid(
                    "Expanded Monte Carlo tasks disagree about selected checkpoints",
                ));
            }
            continue;
        }
        let mut bytes = 0usize;
        for resume in task.monte_carlo_resumes.iter() {
            PreparationError::check_abort(context.abort)?;
            if !required.insert((task.authored_instance_id, resume.population_identity())) {
                return Err(invalid(
                    "Monte Carlo checkpoint selections contain an unpooled duplicate population",
                ));
            }
            bytes = bytes.saturating_add(resume.input().byte_len());
        }
        PreparationError::check_limit(
            rspice_core::ResourceKind::ExternalDataBytes,
            bytes,
            context.limits.max_external_data_bytes,
        )?;
        selections.insert(task.authored_instance_id, task.monte_carlo_resumes.clone());
    }
    let mut used = HashSet::new();
    for task in tasks.iter_mut() {
        PreparationError::check_abort(context.abort)?;
        let options = &task.task.spec_options;
        let candidates = selections.get(&task.authored_instance_id);
        let direct = options
            .mc_checkpoint
            .as_ref()
            .and_then(|request| request.resume.as_ref());
        if candidates.is_none() && direct.is_none() {
            continue;
        }
        let AnalysisSpec::MonteCarlo {
            variation_source, ..
        } = task.task.spec
        else {
            return Err(invalid("Checkpoint selections require a Monte Carlo task"));
        };
        if options.mc_checkpoint.is_none() || candidates.is_some() && direct.is_some() {
            return Err(invalid(
                "Monte Carlo checkpoint request has missing or ambiguous resume policy",
            ));
        }
        let population = crate::study::monte_carlo::prepared_population_identity_with_context(
            options.study_base.as_ref(),
            options.mc_histogram_bins.unwrap_or(20),
            variation_source,
            options.mc_statistics.as_ref(),
            task.executable_netlist_override
                .as_deref()
                .unwrap_or(source),
            task.execution_environment.clone(),
            context,
        )
        .map_err(|error| {
            PreparationError::from_simulation(
                PreparationStage::AnalysisPlan,
                &format!("{} checkpoint compatibility", task.label),
                error,
            )
        })?;
        if let Some(candidates) = candidates {
            let selected = candidates
                .iter()
                .find(|candidate| candidate.population_identity() == population);
            if let Some(selected) = selected {
                selected
                    .input()
                    .decode_with_limits(context.limits, context.abort)
                    .map_err(|error| {
                        PreparationError::from_simulation(
                            PreparationStage::AnalysisPlan,
                            "Selected checkpoint",
                            error,
                        )
                    })?;
                used.insert((task.authored_instance_id, population));
            }
            task.task
                .spec_options
                .mc_checkpoint
                .as_mut()
                .expect("checked policy")
                .resume = selected.map(|selected| selected.input().clone());
            task.monte_carlo_resumes = Arc::from([]);
            task.config_digest = task.payload_digest();
        } else if direct
            .expect("checked direct input")
            .decode_with_limits(context.limits, context.abort)
            .map_err(|error| {
                PreparationError::from_simulation(
                    PreparationStage::AnalysisPlan,
                    "Checkpoint input",
                    error,
                )
            })?
            .population_identity()
            != population
        {
            return Err(invalid(format!(
                "{} checkpoint does not match the prepared trial population",
                task.label
            )));
        }
    }
    if let Some((instance, _)) = required.difference(&used).next() {
        let label = tasks
            .iter()
            .find(|task| task.authored_instance_id == *instance)
            .map_or("Monte Carlo", |task| task.label.as_str());
        return Err(invalid(format!(
            "{label}: a selected checkpoint matches no requested Run Set point under the current evaluator. Restore compatible circuit, sampler and analysis settings and include that point, or clear the selection to start a new population."
        )));
    }
    Ok(())
}
