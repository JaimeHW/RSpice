//! Route selected trial populations against fully materialized per-point decks.
use super::*;
use crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointInput;

#[derive(Debug, Clone)]
pub(in crate::simulation) struct PreparedMonteCarloResume {
    population: [u8; 32],
    input: MonteCarloCheckpointInput,
}
impl PreparedMonteCarloResume {
    pub(in crate::simulation) fn from_input(
        input: MonteCarloCheckpointInput,
    ) -> Result<Self, String> {
        let population = input
            .decode()
            .map_err(|error| error.to_string())?
            .population_identity();
        Ok(Self { population, input })
    }
    #[cfg(test)]
    pub(in crate::simulation) fn input(&self) -> &MonteCarloCheckpointInput {
        &self.input
    }
}
impl PreparedTask {
    pub(in crate::simulation) fn with_monte_carlo_resumes(
        mut self,
        mut resumes: Vec<PreparedMonteCarloResume>,
    ) -> Self {
        resumes.sort_by_key(|resume| resume.population);
        self.monte_carlo_resumes = resumes.into();
        self.config_digest = self.payload_digest();
        self
    }
    #[cfg(test)]
    pub(in crate::simulation) fn monte_carlo_resumes(&self) -> &[PreparedMonteCarloResume] {
        &self.monte_carlo_resumes
    }
}

pub(super) fn digest_with_resumes(
    base: ContentDigest,
    resumes: &[PreparedMonteCarloResume],
) -> ContentDigest {
    if resumes.is_empty() {
        return base;
    }
    let mut writer = CanonicalWriter::new("rspice.monte-carlo-selected-populations/v1");
    writer.digest(base);
    writer.sequence(resumes.len());
    for resume in resumes {
        writer.digest(ContentDigest::from_bytes(resume.population));
        writer.digest(resume.input.digest());
    }
    writer.finish()
}

fn invalid(message: impl Into<String>) -> PreparationError {
    PreparationError::new(PreparationStage::AnalysisPlan, message)
}

pub(super) fn route_resumes(
    tasks: &mut [PreparedTask],
    source: &str,
) -> Result<(), PreparationError> {
    let mut selections: HashMap<AnalysisInstanceId, Arc<[PreparedMonteCarloResume]>> =
        HashMap::new();
    let mut required = HashSet::new();
    for task in tasks
        .iter()
        .filter(|task| !task.monte_carlo_resumes.is_empty())
    {
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
            if !required.insert((task.authored_instance_id, resume.population)) {
                return Err(invalid(
                    "Monte Carlo checkpoint selections contain an unpooled duplicate population",
                ));
            }
            bytes = bytes.saturating_add(resume.input.byte_len());
        }
        if bytes > rspice_core::ResourceLimits::default().max_external_data_bytes {
            return Err(invalid(
                "Selected Monte Carlo checkpoints exceed the combined byte limit",
            ));
        }
        selections.insert(task.authored_instance_id, task.monte_carlo_resumes.clone());
    }
    let mut used = HashSet::new();
    for task in tasks.iter_mut() {
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
        let base = options.study_base.as_ref().ok_or_else(|| {
            invalid("Checkpoint selections require a configured Monte Carlo base")
        })?;
        if options.mc_checkpoint.is_none() || candidates.is_some() && direct.is_some() {
            return Err(invalid(
                "Monte Carlo checkpoint request has missing or ambiguous resume policy",
            ));
        }
        let population =
            crate::simulation::runner::study::monte_carlo::prepared_population_identity(
                base,
                variation_source,
                options.mc_statistics.as_ref(),
                task.executable_netlist_override
                    .as_deref()
                    .unwrap_or(source),
                task.execution_environment.clone(),
            )
            .map_err(|error| {
                invalid(format!("{} checkpoint compatibility: {error}", task.label))
            })?;
        if let Some(candidates) = candidates {
            let selected = candidates
                .iter()
                .find(|candidate| candidate.population == population);
            if selected.is_some() {
                used.insert((task.authored_instance_id, population));
            }
            task.task
                .spec_options
                .mc_checkpoint
                .as_mut()
                .expect("checked policy")
                .resume = selected.map(|selected| selected.input.clone());
            task.monte_carlo_resumes = Arc::from([]);
            task.config_digest = task.payload_digest();
        } else if direct
            .expect("checked direct input")
            .decode()
            .map_err(|error| invalid(error.to_string()))?
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
            "{label}: a selected checkpoint matches no requested Run Set point. Restore its circuit, sampler and analysis settings, include that point, or clear the selection."
        )));
    }
    Ok(())
}
