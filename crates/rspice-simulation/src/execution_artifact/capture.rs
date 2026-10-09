//! Capture dependency payloads from the exact task that produced a result.

use super::{
    ExecutionArtifactEnvelope, PeriodicOperatingEnvironment, ResolvedExecutionDependencies,
};
use crate::execution::AuthorizedTaskDispatch;
use crate::prepared_dependency::{ExecutionArtifactError, ExecutionArtifactKind};
use crate::results::SimulationResult;
use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use rspice_simulation_contract::analysis_spec::{AnalysisSpec, PssMethod};
use rspice_simulation_contract::config::AnalysisConfig;
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
        let mut kind = None;
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
