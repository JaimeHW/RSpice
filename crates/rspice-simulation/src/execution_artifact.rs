//! Typed, authenticated handoff of numerical results between prepared tasks.
//!
//! Execution artifacts are deliberately batch-local. They bind an immutable
//! payload to the exact producer task and prepared snapshot that created it;
//! neither retained UI results nor a same-kind task may satisfy a dependency.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::execution_options::SpecExecutionOptions;
use crate::prepared_dependency::ExecutionArtifactError;
use crate::prepared_dependency::ExecutionArtifactKind;
#[cfg(test)]
use crate::prepared_dependency::validate_prepared_dependency_contract_with_options;
use crate::results::SimulationResult;
use rspice_app_types::canonical::CanonicalWriter;
use rspice_app_types::product::AnalysisInstanceId;
use rspice_app_types::product::ContentDigest;
use rspice_app_types::product::ObjectRevision;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::analysis_spec::PssMethod;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedDependencyBinding {
    kind: ExecutionArtifactKind,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
}

impl PreparedDependencyBinding {
    pub const fn transient_trajectory(
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

    pub const fn periodic_state(
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

    pub const fn dc_operating_point_seed(
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

    pub const fn hb_state(
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

    pub const fn qpss_state(
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) -> Self {
        Self {
            kind: ExecutionArtifactKind::QpssState,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
        }
    }

    pub const fn kind(&self) -> ExecutionArtifactKind {
        self.kind
    }

    pub const fn producer_instance_id(&self) -> AnalysisInstanceId {
        self.producer_instance_id
    }

    pub const fn producer_source_revision(&self) -> ObjectRevision {
        self.producer_source_revision
    }

    pub const fn producer_config_digest(&self) -> ContentDigest {
        self.producer_config_digest
    }

    pub fn rebind_producer(
        &mut self,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
    ) {
        self.producer_instance_id = producer_instance_id;
        self.producer_source_revision = producer_source_revision;
        self.producer_config_digest = producer_config_digest;
    }

    pub fn encode(&self, writer: &mut CanonicalWriter) {
        writer.u8(match self.kind {
            ExecutionArtifactKind::TransientTrajectory => 0,
            ExecutionArtifactKind::PeriodicState => 1,
            ExecutionArtifactKind::DcOperatingPointSeed => 2,
            ExecutionArtifactKind::HbState => 3,
            ExecutionArtifactKind::QpssState => 4,
        });
        writer.uuid(self.producer_instance_id.as_uuid());
        writer.u64(self.producer_source_revision.get());
        writer.digest(self.producer_config_digest);
    }
}

#[cfg(test)]
pub(crate) fn validate_prepared_dependency_contract(
    consumer: &AnalysisSpec,
    producer: &AnalysisSpec,
) -> Result<(), ExecutionArtifactError> {
    validate_prepared_dependency_contract_with_options(
        consumer,
        &SpecExecutionOptions::default(),
        producer,
    )
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
        integration_method,
        tstab,
        max_iterations,
        abstol,
        damping,
        max_period_change,
        verbose,
        tone_sources,
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

    // Resolved by the runner's own builder, not by a second copy of it here.
    // This arm used to rebuild the configuration field by field, including its
    // own literal Newton-iteration limit, so every control the request learned
    // was a field the two could disagree about — and a disagreement refuses a
    // converged periodic state as unauthenticated. The HB arm below has always
    // delegated for the same reason.
    let expected = crate::periodic::build_core_pss_config(&crate::periodic::PssRunConfig {
        fundamental_freq: *fundamental_freq,
        tone_sources: tone_sources.clone(),
        tstab_periods: *tstab_periods,
        points_per_period: *points_per_period,
        num_harmonics: *num_harmonics,
        tolerance: *tolerance,
        oscillator_mode: *oscillator_mode,
        oscillator_node: oscillator_node.clone(),
        integration_method: integration_method
            .map(rspice_simulation_contract::options::IntegrationMethod::core),
        tstab: *tstab,
        max_iterations: *max_iterations,
        abstol: *abstol,
        damping: *damping,
        max_period_change: *max_period_change,
        verbose: *verbose,
    });

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
    producer_source: Option<&str>,
    environment: Option<&PeriodicOperatingEnvironment>,
    actual: &rspice_core::analysis::HbConfig,
) -> Result<(), ExecutionArtifactError> {
    let AnalysisSpec::HarmonicBalance {
        tones,
        reltol,
        abstol,
        max_iterations,
        damping,
        min_damping,
        oversample,
        collocation_points,
        max_mixing_order,
        use_krylov,
        gmres_restart,
        source_stepping,
        use_exact_jacobian,
        verbose,
    } = producer_spec
    else {
        return Err(ExecutionArtifactError::ContractMismatch(
            "HB-state artifact producer is not a Harmonic Balance analysis".to_owned(),
        ));
    };
    let run_config = crate::periodic::HbRunConfig {
        tones: tones
            .iter()
            .map(|tone| crate::periodic::HbToneRunConfig {
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
        min_damping: *min_damping,
        oversample: *oversample,
        collocation_points: *collocation_points,
        max_mixing_order: *max_mixing_order,
        use_krylov: *use_krylov,
        gmres_restart: *gmres_restart,
        source_stepping: *source_stepping,
        use_exact_jacobian: *use_exact_jacobian,
        verbose: *verbose,
    };
    let expected =
        crate::periodic::build_core_hb_config(&run_config, &rspice_core::abort_signal::NoAbort)
            .map_err(|error| ExecutionArtifactError::ContractMismatch(error.to_string()))?;
    // Resolve from the host's frozen deck, never from worker-returned settings.
    // OP temperature also governs expressions in the producer's option cards.
    let source = producer_source.ok_or_else(|| {
        ExecutionArtifactError::ContractMismatch(
            "HB-state artifact has no frozen producer source for numerical-option validation"
                .into(),
        )
    })?;
    let source = match environment {
        Some(environment) => crate::netlist_preparation::source_with_run_temperature_with_abort(
            source,
            environment.temperature_celsius(),
            &rspice_core::NoAbort,
        )
        .map_err(|error| ExecutionArtifactError::ContractMismatch(error.to_string()))?,
        None => source.to_owned(),
    };
    let netlist = crate::netlist_preparation::parse_runner_netlist_with_abort(
        &source,
        None,
        &rspice_core::NoAbort,
    )
    .map_err(|error| ExecutionArtifactError::ContractMismatch(error.to_string()))?;
    let expected = rspice_core::Engine::default()
        .hb_config_for_netlist(&netlist, expected)
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

pub use payload::*;
