//! Checked selection and immutable inputs for resuming retained trial populations.

use super::MonteCarloCheckpointInput;
use rspice_app_types::canonical::CanonicalWriter;
use rspice_app_types::product::ContentDigest;

#[derive(Debug, Clone)]
pub struct PreparedMonteCarloResume {
    population: [u8; 32],
    input: MonteCarloCheckpointInput,
}
impl PreparedMonteCarloResume {
    pub fn from_input(input: MonteCarloCheckpointInput) -> Result<Self, String> {
        let population = input
            .decode()
            .map_err(|error| error.to_string())?
            .population_identity();
        Ok(Self { population, input })
    }
    pub fn population_identity(&self) -> [u8; 32] {
        self.population
    }
    pub fn input(&self) -> &MonteCarloCheckpointInput {
        &self.input
    }
}
pub fn digest_with_resumes(
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

/// Resolve selected history into checked populations before freezing a task.
/// Different populations stay separate; matching trial journals pool exactly.
pub fn resume_inputs_from_config<'a, W: 'a>(
    retained: impl Iterator<Item = &'a rspice_results::analysis_result::AnalysisResult<W>> + Clone,
    imported: &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    config: Option<&rspice_simulation_contract::mc_checkpoint::McCheckpointConfig>,
) -> Result<Vec<PreparedMonteCarloResume>, String> {
    use rspice_results::monte_carlo_checkpoint::StudyMonteCarloCheckpoint;
    let Some(config) = config else {
        return Ok(Vec::new());
    };
    let limits = rspice_core::ResourceLimits::default();
    let mut selected = Vec::new();
    let mut bytes = 0usize;
    let mut seen = std::collections::HashSet::new();
    for digest in &config.resume {
        if !seen.insert(*digest) {
            return Err("A Monte Carlo checkpoint is selected more than once".into());
        }
        let history = retained.clone().find_map(|analysis| {
            analysis
                .monte_carlo_checkpoint
                .as_ref()
                .filter(|checkpoint| checkpoint.digest() == *digest)
                .map(|checkpoint| (analysis, checkpoint))
        });
        let (analysis, evidence) = if let Some((analysis, evidence)) = history {
            (Some(analysis), evidence)
        } else {
            (None, imported.get(*digest)
                .ok_or("A selected Monte Carlo checkpoint is no longer retained. Clear its selection, restore the run or import the checkpoint file before preparing again.")?)
        };
        bytes = bytes.saturating_add(evidence.bytes().len());
        if bytes > limits.max_external_data_bytes {
            return Err("Selected Monte Carlo checkpoints exceed the combined byte limit".into());
        }
        selected.push((analysis, evidence));
    }
    let mut populations: std::collections::BTreeMap<[u8; 32], StudyMonteCarloCheckpoint> =
        Default::default();
    for (analysis, evidence) in selected {
        if let Some(analysis) = analysis {
            evidence.validate_for(analysis.into())?;
        }
        let checkpoint = StudyMonteCarloCheckpoint::from_bytes_with_limits(
            evidence.bytes(),
            limits,
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())?;
        if let Some(pooled) = populations.get_mut(&checkpoint.population_identity()) {
            pooled
                .merge_with_limits(&checkpoint, limits, &rspice_core::NoAbort)
                .map_err(|error| {
                    format!("Selected Monte Carlo checkpoints cannot be pooled: {error}")
                })?;
        } else {
            populations.insert(checkpoint.population_identity(), checkpoint);
        }
    }
    populations
        .into_values()
        .map(|checkpoint| {
            let bytes = checkpoint
                .to_bytes_with_limits(limits, &rspice_core::NoAbort)
                .map_err(|error| error.to_string())?;
            let input =
                MonteCarloCheckpointInput::from_bytes(bytes).map_err(|error| error.to_string())?;
            PreparedMonteCarloResume::from_input(input)
        })
        .collect()
}
