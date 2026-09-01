//! Typed, authenticated handoff of numerical results between prepared tasks.
//!
//! Execution artifacts are deliberately batch-local. They bind an immutable
//! payload to the exact producer task and prepared snapshot that created it;
//! neither retained UI results nor a same-kind task may satisfy a dependency.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::canonical::CanonicalWriter;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::simulation::dependency_contract::{
    FourierTransientRequirement, PeriodicStateCapability, TransientCapability,
    validate_fourier_transient_contract, validate_periodic_state_contract,
};
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::multi_run::PssMethod;
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::SpecExecutionOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::simulation) enum ExecutionArtifactKind {
    TransientTrajectory,
    PeriodicState,
    HbState,
    DcOperatingPointSeed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::simulation) struct PreparedDependencyBinding {
    kind: ExecutionArtifactKind,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
}

impl PreparedDependencyBinding {
    pub(in crate::simulation) const fn transient_trajectory(
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) -> Self {
        Self {
            kind: ExecutionArtifactKind::TransientTrajectory,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
        }
    }

    pub(in crate::simulation) const fn periodic_state(
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) -> Self {
        Self {
            kind: ExecutionArtifactKind::PeriodicState,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
        }
    }

    pub(in crate::simulation) const fn dc_operating_point_seed(
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) -> Self {
        Self {
            kind: ExecutionArtifactKind::DcOperatingPointSeed,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
        }
    }

    pub(in crate::simulation) const fn hb_state(
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) -> Self {
        Self {
            kind: ExecutionArtifactKind::HbState,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
        }
    }

    pub(in crate::simulation) const fn kind(&self) -> ExecutionArtifactKind {
        self.kind
    }

    pub(in crate::simulation) const fn producer_instance_id(&self) -> AnalysisInstanceId {
        self.producer_instance_id
    }

    pub(in crate::simulation) const fn producer_source_revision(&self) -> ObjectRevision {
        self.producer_source_revision
    }

    pub(in crate::simulation) const fn producer_config_digest(&self) -> ContentDigest {
        self.producer_config_digest
    }

    pub(in crate::simulation) fn rebind_producer(
        &mut self,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) {
        self.producer_instance_id = producer_instance_id;
        self.producer_source_revision = producer_source_revision;
        self.producer_config_digest = producer_config_digest;
    }

    pub(super) fn encode(&self, writer: &mut CanonicalWriter) {
        writer.u8(match self.kind {
            ExecutionArtifactKind::TransientTrajectory => 0,
            ExecutionArtifactKind::PeriodicState => 1,
            ExecutionArtifactKind::DcOperatingPointSeed => 2,
            ExecutionArtifactKind::HbState => 3,
        });
        writer.uuid(self.producer_instance_id.as_uuid());
        writer.u64(self.producer_source_revision.get());
        writer.digest(self.producer_config_digest);
    }
}

#[cfg(test)]
pub(in crate::simulation) fn validate_prepared_dependency_contract(
    consumer: &AnalysisSpec,
    producer: &AnalysisSpec,
) -> Result<(), ExecutionArtifactError> {
    validate_prepared_dependency_contract_with_options(
        consumer,
        &SpecExecutionOptions::default(),
        producer,
    )
}

pub(in crate::simulation) fn validate_prepared_dependency_contract_with_options(
    consumer: &AnalysisSpec,
    consumer_options: &SpecExecutionOptions,
    producer: &AnalysisSpec,
) -> Result<(), ExecutionArtifactError> {
    if matches!(
        consumer,
        AnalysisSpec::Pss {
            method: PssMethod::Shooting,
            ..
        }
    ) {
        return match producer {
            AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. } => Ok(()),
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "shooting PSS cannot consume a DC operating-point seed produced by {}",
                producer.run_type().display_name()
            ))),
        };
    }
    if matches!(consumer, AnalysisSpec::Pss { .. }) {
        return Err(ExecutionArtifactError::ContractMismatch(
            "legacy HB-PSS is not executable and cannot bind a DC operating-point seed".to_owned(),
        ));
    }
    if matches!(
        consumer,
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. }
    ) {
        return match producer {
            AnalysisSpec::HarmonicBalance { .. } => Ok(()),
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} cannot consume an HB state produced by {}",
                consumer.run_type().display_name(),
                producer.run_type().display_name()
            ))),
        };
    }
    if matches!(
        consumer,
        AnalysisSpec::PssSpectrum { .. }
            | AnalysisSpec::Pac
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pstb
            | AnalysisSpec::Psp { .. }
    ) {
        let require_autonomous = matches!(consumer, AnalysisSpec::Pnoise)
            && consumer_options.pnoise.as_ref().is_some_and(|config| {
                config.noise_ref == crate::services::simulation_runner::PnoiseReference::Phase
            });
        return match producer {
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                oscillator_mode,
                ..
            } => validate_periodic_state_contract(
                consumer.run_type().display_name(),
                PeriodicStateCapability {
                    shooting: true,
                    autonomous: *oscillator_mode,
                },
                require_autonomous,
            )
            .map_err(ExecutionArtifactError::ContractMismatch),
            AnalysisSpec::Pss {
                oscillator_mode, ..
            } => validate_periodic_state_contract(
                consumer.run_type().display_name(),
                PeriodicStateCapability {
                    shooting: false,
                    autonomous: *oscillator_mode,
                },
                require_autonomous,
            )
            .map_err(ExecutionArtifactError::ContractMismatch),
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} cannot consume a typed artifact produced by {}",
                consumer.run_type().display_name(),
                producer.run_type().display_name()
            ))),
        };
    }

    let (
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            start_time,
            stop_time,
            ..
        },
        AnalysisSpec::Transient {
            stop_time: transient_stop,
            step_time,
            start_time: transient_start,
            max_timestep,
            ..
        },
    ) = (consumer, producer)
    else {
        return Err(ExecutionArtifactError::ContractMismatch(format!(
            "{} cannot consume a typed artifact produced by {}",
            consumer.run_type().display_name(),
            producer.run_type().display_name()
        )));
    };

    let num_harmonics = u32::try_from(*num_harmonics).map_err(|_| {
        ExecutionArtifactError::ContractMismatch(
            "Fourier harmonic count exceeds the supported dependency contract".to_owned(),
        )
    })?;
    validate_fourier_transient_contract(
        FourierTransientRequirement {
            start_time: *start_time,
            stop_time: *stop_time,
            fundamental_freq: *fundamental_freq,
            num_harmonics,
        },
        TransientCapability {
            start_time: *transient_start,
            stop_time: *transient_stop,
            step_time: *step_time,
            max_timestep: *max_timestep,
        },
    )
    .map_err(ExecutionArtifactError::ContractMismatch)
}

fn validate_periodic_producer_config(
    producer_spec: &AnalysisSpec,
    actual: &rspice_core::analysis::PssConfig,
) -> Result<(), ExecutionArtifactError> {
    let AnalysisSpec::Pss {
        fundamental_freq,
        num_harmonics,
        tolerance,
        method,
        oscillator_mode,
        oscillator_node,
        tstab_periods,
        points_per_period,
        tone_sources: _,
    } = producer_spec
    else {
        return Err(ExecutionArtifactError::ContractMismatch(
            "periodic-state artifact producer is not a PSS analysis".to_owned(),
        ));
    };
    if !matches!(method, PssMethod::Shooting) {
        return Err(ExecutionArtifactError::ContractMismatch(
            "periodic-state artifacts require a shooting-PSS producer".to_owned(),
        ));
    }

    let mut expected = if *oscillator_mode {
        rspice_core::analysis::PssConfig::autonomous().with_period_guess(1.0 / *fundamental_freq)
    } else {
        rspice_core::analysis::PssConfig::new(*fundamental_freq)
    }
    .with_harmonics((*num_harmonics).max(1))
    .with_tolerance(*tolerance)
    .with_max_iterations(100)
    .with_tstab_periods(*tstab_periods)
    .with_points_per_period(*points_per_period);
    if let Some(node) = oscillator_node.as_deref() {
        expected = expected.with_oscillator_node(node);
    }

    if actual != &expected {
        return Err(ExecutionArtifactError::ContractMismatch(
            "returned PSS numerical state was produced with a configuration that does not match the frozen producer specification"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_hb_producer_config(
    producer_spec: &AnalysisSpec,
    actual: &rspice_core::analysis::HbConfig,
) -> Result<(), ExecutionArtifactError> {
    let AnalysisSpec::HarmonicBalance {
        tones,
        reltol,
        abstol,
        max_iterations,
        damping,
        oversample,
        collocation_points,
        max_mixing_order,
        use_krylov,
        gmres_restart,
        source_stepping,
        verbose,
    } = producer_spec
    else {
        return Err(ExecutionArtifactError::ContractMismatch(
            "HB-state artifact producer is not a Harmonic Balance analysis".to_owned(),
        ));
    };
    let run_config = crate::services::simulation_runner::HbRunConfig {
        tones: tones
            .iter()
            .map(|tone| crate::services::simulation_runner::HbToneRunConfig {
                frequency: tone.frequency,
                harmonics: tone.harmonics,
                source: tone.source.clone(),
                name: tone.name.clone(),
            })
            .collect(),
        reltol: *reltol,
        abstol: *abstol,
        max_iterations: *max_iterations,
        damping: *damping,
        oversample: *oversample,
        collocation_points: *collocation_points,
        max_mixing_order: *max_mixing_order,
        use_krylov: *use_krylov,
        gmres_restart: *gmres_restart,
        source_stepping: *source_stepping,
        verbose: *verbose,
    };
    let expected = crate::services::simulation_runner::build_core_hb_config(
        &run_config,
        &rspice_core::abort_signal::NoAbort,
    )
    .map_err(|error| ExecutionArtifactError::ContractMismatch(error.to_string()))?;
    if actual != &expected {
        return Err(ExecutionArtifactError::ContractMismatch(
            "returned HB numerical state was produced with a configuration that does not match the frozen producer specification"
                .to_owned(),
        ));
    }
    Ok(())
}

mod payload;

pub(in crate::simulation) use payload::*;
