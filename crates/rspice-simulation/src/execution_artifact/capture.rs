//! Capture dependency payloads from the exact task that produced a result.

use super::{
    ExecutionArtifactEnvelope, PeriodicOperatingEnvironment, ResolvedExecutionDependencies,
};
use crate::execution::AuthorizedTaskDispatch;
use crate::prepared_dependency::{ExecutionArtifactError, ExecutionArtifactKind};
use crate::results::SimulationResult;
use rspice_simulation_contract::analysis_spec::{AnalysisSpec, PssMethod};
use rspice_simulation_contract::config::AnalysisConfig;
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use std::sync::Arc;

/// Immutable producer context captured before dispatch consumes its task.
/// UI state and retained result documents cannot supply or replace its identity.
#[derive(Debug, Clone)]
pub struct PreparedArtifactProducer {
    snapshot: ContentDigest,
    instance: AnalysisInstanceId,
    revision: ObjectRevision,
    config_digest: ContentDigest,
    spec: AnalysisSpec,
    config: Option<AnalysisConfig>,
    source_basis: ContentDigest,
    source: Arc<str>,
    environment: Option<PeriodicOperatingEnvironment>,
    /// The times this transient schedules checkpoints at, in order.
    checkpoint_times: Vec<f64>,
}

impl PreparedArtifactProducer {
    pub(crate) fn new(
        task: &AuthorizedTaskDispatch,
        dependencies: &ResolvedExecutionDependencies,
    ) -> Result<Self, ExecutionArtifactError> {
        let environment = if matches!(
            task.spec(),
            AnalysisSpec::Qpss { .. }
                | AnalysisSpec::HarmonicBalance { .. }
                | AnalysisSpec::Pss {
                    method: PssMethod::Shooting,
                    ..
                }
        ) {
            Some(dependencies.dc_operating_point_seed()?.environment())
        } else {
            None
        };
        Ok(Self {
            snapshot: task.snapshot_digest(),
            instance: task.instance_id(),
            revision: task.source_revision(),
            config_digest: task.config_digest(),
            spec: task.spec().clone(),
            config: task.config().cloned(),
            source_basis: task.source_basis_digest(),
            source: Arc::clone(task.executable_netlist()),
            environment,
            checkpoint_times: task
                .spec_options()
                .tran_checkpoint
                .as_ref()
                .filter(|request| request.resume.is_none())
                .map(|request| request.times.clone())
                .unwrap_or_default(),
        })
    }

    /// Capture only evidence required by authenticated pending consumers.
    /// Consumers from another snapshot and mismatched producer bindings fail
    /// before any result payload is copied or published.
    pub fn capture<'a>(
        &self,
        result: &SimulationResult,
        consumers: impl IntoIterator<Item = &'a AuthorizedTaskDispatch>,
    ) -> Result<Option<ExecutionArtifactEnvelope>, ExecutionArtifactError> {
        self.capture_with_transient_checkpoints(result, &[], consumers)
    }

    /// [`Self::capture`], with the checkpoint images this transient published
    /// as it ran, in publication order
    /// ([`SimulationRunner::take_task_transient_checkpoints`](crate::runner::SimulationRunner::take_task_transient_checkpoints)).
    /// A resume bound to this transient takes the image captured at the time
    /// it names; one the run never reached is refused by that time.
    pub fn capture_with_transient_checkpoints<'a>(
        &self,
        result: &SimulationResult,
        checkpoints: &[Arc<[u8]>],
        consumers: impl IntoIterator<Item = &'a AuthorizedTaskDispatch>,
    ) -> Result<Option<ExecutionArtifactEnvelope>, ExecutionArtifactError> {
        let mut kind = None;
        let mut resume_times = Vec::new();
        let mut waveforms = Vec::new();
        let mut carry_spectra = false;
        for consumer in consumers {
            if consumer.snapshot_digest() != self.snapshot {
                return Err(ExecutionArtifactError::StaleSnapshot {
                    expected: self.snapshot,
                    actual: consumer.snapshot_digest(),
                });
            }
            for binding in consumer.dependency_bindings() {
                if binding.producer_instance_id() != self.instance {
                    continue;
                }
                if binding.producer_source_revision() != self.revision
                    || binding.producer_config_digest() != self.config_digest
                {
                    return Err(invalid(
                        "Artifact consumer does not bind this exact producer",
                    ));
                }
                if kind.is_some_and(|kind| kind != binding.kind()) {
                    return Err(invalid(
                        "One producer cannot supply incompatible artifact kinds",
                    ));
                }
                kind = Some(binding.kind());
                match consumer.spec() {
                    AnalysisSpec::Fourier {
                        output_node,
                        output_ref,
                        additional_outputs,
                        ..
                    } => {
                        waveforms.push(output_node.clone());
                        append_reference(&mut waveforms, output_ref);
                        for output in additional_outputs {
                            let (node, reference) =
                                rspice_simulation_contract::fourier_output::split_fourier_output(
                                    output,
                                )
                                .map_err(invalid)?;
                            waveforms.push(node);
                            append_reference(&mut waveforms, &reference);
                        }
                    }
                    AnalysisSpec::Fft { .. } => carry_spectra = true,
                    AnalysisSpec::Transient { .. } => {
                        if let Some(time) = consumer.spec_options().resume_at {
                            resume_times.push(time);
                        }
                    }
                    _ => {}
                }
            }
        }
        let Some(kind) = kind else {
            return Ok(None);
        };
        match (kind, &self.spec) {
            (ExecutionArtifactKind::TransientTrajectory, AnalysisSpec::Transient { .. }) => {
                ExecutionArtifactEnvelope::from_transient_result(
                    self.snapshot,
                    self.instance,
                    self.revision,
                    self.config_digest,
                    result,
                    &waveforms,
                    carry_spectra,
                )?
                .ok_or_else(|| invalid("Transient producer returned a non-transient result"))
                .map(Some)
            }
            (ExecutionArtifactKind::PeriodicState, AnalysisSpec::Pss { .. }) => {
                ExecutionArtifactEnvelope::from_periodic_result_with_environment(
                    self.snapshot,
                    self.instance,
                    self.revision,
                    self.config_digest,
                    &self.spec,
                    result,
                    self.environment.clone(),
                )
            }
            (ExecutionArtifactKind::HbState, AnalysisSpec::HarmonicBalance { .. }) => {
                ExecutionArtifactEnvelope::from_hb_result_with_environment(
                    self.snapshot,
                    self.instance,
                    self.revision,
                    self.config_digest,
                    &self.spec,
                    Some(&self.source),
                    result,
                    self.environment.clone(),
                )
            }
            (ExecutionArtifactKind::QpssState, AnalysisSpec::Qpss { .. }) => {
                ExecutionArtifactEnvelope::from_qpss_result_with_environment(
                    self.snapshot,
                    self.instance,
                    self.revision,
                    self.config_digest,
                    &self.spec,
                    result,
                    self.environment.clone(),
                )
            }
            (
                ExecutionArtifactKind::DcOperatingPointSeed,
                AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. },
            ) => {
                let Some(AnalysisConfig::DcOp(config)) = &self.config else {
                    return Err(invalid(
                        "Operating-point producer has no prepared OP configuration",
                    ));
                };
                ExecutionArtifactEnvelope::from_dc_operating_point_result(
                    self.snapshot,
                    self.instance,
                    self.revision,
                    self.config_digest,
                    self.source_basis,
                    config,
                    result,
                )
            }
            (ExecutionArtifactKind::TransientCheckpoint, AnalysisSpec::Transient { .. }) => {
                let mut images = Vec::with_capacity(resume_times.len());
                resume_times.sort_by(f64::total_cmp);
                resume_times.dedup_by(|a, b| a.to_bits() == b.to_bits());
                for time in resume_times {
                    let position = self
                        .checkpoint_times
                        .iter()
                        .position(|scheduled| scheduled.to_bits() == time.to_bits())
                        .ok_or_else(|| {
                            invalid(format!(
                                "a resume at {time:e} s names no checkpoint this transient schedules"
                            ))
                        })?;
                    let image = checkpoints.get(position).ok_or_else(|| {
                        invalid(format!(
                            "this transient published no checkpoint at {time:e} s for the resume bound to it: it ended before that time, or its images were not retained"
                        ))
                    })?;
                    images.push((time, Arc::clone(image)));
                }
                Ok(Some(ExecutionArtifactEnvelope::from_transient_checkpoints(
                    self.snapshot,
                    self.instance,
                    self.revision,
                    self.config_digest,
                    super::TransientCheckpointArtifact::new(images)?,
                )))
            }
            _ => Err(invalid(
                "Prepared producer cannot supply the requested artifact kind",
            )),
        }
    }
}

fn append_reference(waveforms: &mut Vec<String>, reference: &str) {
    if !reference.trim().is_empty() && !reference.trim().eq_ignore_ascii_case("0") {
        waveforms.push(reference.to_owned());
    }
}

fn invalid(message: impl Into<String>) -> ExecutionArtifactError {
    ExecutionArtifactError::InvalidPayload(message.into())
}
