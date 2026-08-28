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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct TransientTrajectoryArtifact {
    #[serde(with = "f64_bits_vec")]
    time: Vec<f64>,
    #[serde(with = "f64_bits_map")]
    waveforms: BTreeMap<String, Vec<f64>>,
}

mod f64_bits_vec {
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _};

    pub(super) fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            sequence.serialize_element(&format!("{:016x}", value.to_bits()))?;
        }
        sequence.end()
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|encoded| {
                u64::from_str_radix(&encoded, 16)
                    .map(f64::from_bits)
                    .map_err(|_| D::Error::custom("invalid exact f64 bit pattern"))
            })
            .collect()
    }
}

mod f64_bits_map {
    use std::collections::BTreeMap;

    use serde::ser::{SerializeMap, SerializeSeq};
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

    struct BitSlice<'a>(&'a [f64]);

    impl Serialize for BitSlice<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
            for value in self.0 {
                sequence.serialize_element(&format!("{:016x}", value.to_bits()))?;
            }
            sequence.end()
        }
    }

    pub(super) fn serialize<S>(
        waveforms: &BTreeMap<String, Vec<f64>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(waveforms.len()))?;
        for (name, values) in waveforms {
            map.serialize_entry(name, &BitSlice(values))?;
        }
        map.end()
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<String, Vec<f64>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        BTreeMap::<String, Vec<String>>::deserialize(deserializer)?
            .into_iter()
            .map(|(name, encoded_values)| {
                let values = encoded_values
                    .into_iter()
                    .map(|encoded| {
                        u64::from_str_radix(&encoded, 16)
                            .map(f64::from_bits)
                            .map_err(|_| D::Error::custom("invalid exact f64 bit pattern"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok((name, values))
            })
            .collect()
    }
}

impl TransientTrajectoryArtifact {
    pub(in crate::simulation) fn time(&self) -> &[f64] {
        &self.time
    }

    pub(in crate::simulation) fn waveform(&self, requested: &str) -> Option<&[f64]> {
        let requested = normalize_waveform_name(requested);
        self.waveforms
            .iter()
            .find(|(name, _)| normalize_waveform_name(name) == requested)
            .map(|(_, values)| values.as_slice())
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if self.time.len() < 3 {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory contains fewer than three samples".to_owned(),
            ));
        }
        if self
            .time
            .iter()
            .any(|sample| !sample.is_finite() || *sample < 0.0)
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory time axis contains a non-finite or negative sample"
                    .to_owned(),
            ));
        }
        if self.time.windows(2).any(|pair| pair[1] <= pair[0]) {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory time axis is not strictly increasing".to_owned(),
            ));
        }
        if self.waveforms.is_empty() {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory contains no waveforms".to_owned(),
            ));
        }
        for (name, values) in &self.waveforms {
            if name.trim().is_empty() {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "transient trajectory contains an unnamed waveform".to_owned(),
                ));
            }
            if values.len() != self.time.len() {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' has {} values for {} time samples",
                    values.len(),
                    self.time.len()
                )));
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' contains a non-finite sample"
                )));
            }
        }
        Ok(())
    }

    fn digest(&self) -> ContentDigest {
        let mut writer = CanonicalWriter::new("rspice.transient-trajectory-artifact/v1");
        writer.sequence(self.time.len());
        for value in &self.time {
            writer.f64(*value);
        }
        writer.sequence(self.waveforms.len());
        for (name, values) in &self.waveforms {
            writer.string(name);
            writer.sequence(values.len());
            for value in values {
                writer.f64(*value);
            }
        }
        writer.finish()
    }
}

/// Exact DC operating-point state consumed by one bound shooting-PSS task.
///
/// The payload retains the core MNA basis rather than presentation maps. Its
/// source identity covers the exact per-task executable deck (including any
/// process-corner materialization); voltage scaling and temperature are kept
/// alongside it so PSS can reproduce the OP engine environment exactly once.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct DcOperatingPointSeedArtifact {
    effective_source_content_digest: ContentDigest,
    temperature_celsius: f64,
    supply_voltage: Option<f64>,
    nominal_supply_voltage: Option<f64>,
    #[serde(default)]
    supply_source_names: Vec<String>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    #[serde(with = "f64_bits_vec")]
    solution: Vec<f64>,
}

impl DcOperatingPointSeedArtifact {
    pub(in crate::simulation) const fn effective_source_content_digest(&self) -> ContentDigest {
        self.effective_source_content_digest
    }

    pub(in crate::simulation) const fn temperature_celsius(&self) -> f64 {
        self.temperature_celsius
    }

    pub(in crate::simulation) const fn supply_voltage(&self) -> Option<f64> {
        self.supply_voltage
    }

    pub(in crate::simulation) const fn nominal_supply_voltage(&self) -> Option<f64> {
        self.nominal_supply_voltage
    }

    pub(in crate::simulation) fn supply_source_names(&self) -> &[String] {
        &self.supply_source_names
    }

    pub(in crate::simulation) fn core_seed(
        &self,
    ) -> Result<rspice_core::engine::PssDcOperatingPointSeed, ExecutionArtifactError> {
        rspice_core::engine::PssDcOperatingPointSeed::try_new(
            self.node_names.clone(),
            self.branch_names.clone(),
            self.solution.clone(),
        )
        .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if !self.temperature_celsius.is_finite() || self.temperature_celsius <= -273.15 {
            return Err(ExecutionArtifactError::InvalidPayload(
                "DC operating-point seed temperature must be finite and above absolute zero"
                    .to_owned(),
            ));
        }
        match (self.supply_voltage, self.nominal_supply_voltage) {
            (None, None) => {}
            (Some(supply), Some(nominal))
                if supply.is_finite() && supply > 0.0 && nominal.is_finite() && nominal > 0.0 => {}
            _ => {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "DC operating-point seed supply and nominal voltages must be paired positive finite values"
                        .to_owned(),
                ));
            }
        }
        if self.supply_voltage.is_some() && self.supply_source_names.is_empty() {
            return Err(ExecutionArtifactError::InvalidPayload(
                "DC operating-point seed supply requires explicitly bound source instances"
                    .to_owned(),
            ));
        }
        self.core_seed().map(|_| ())
    }

    fn digest(&self) -> ContentDigest {
        let mut writer = CanonicalWriter::new("rspice.dc-operating-point-seed-artifact/v1");
        writer.digest(self.effective_source_content_digest);
        writer.f64(self.temperature_celsius);
        writer.option(self.supply_voltage.as_ref(), |writer, value| {
            writer.f64(*value);
        });
        writer.option(self.nominal_supply_voltage.as_ref(), |writer, value| {
            writer.f64(*value);
        });
        writer.sequence(self.supply_source_names.len());
        for source in &self.supply_source_names {
            writer.string(source);
        }
        writer.sequence(self.node_names.len());
        for name in &self.node_names {
            writer.string(name);
        }
        writer.sequence(self.branch_names.len());
        for name in &self.branch_names {
            writer.string(name);
        }
        writer.sequence(self.solution.len());
        for value in &self.solution {
            writer.f64(*value);
        }
        writer.finish()
    }
}

/// Complete numerical shooting state consumed by PSS-dependent analyses.
/// The payload is immutable and its digest covers the orbit, monodromy,
/// Floquet data, and reactive phase-origin state bit-for-bit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct PeriodicStateArtifact {
    operating_point: Arc<rspice_core::engine::PssOperatingPoint>,
    result_floquet_real: Vec<f64>,
    result_floquet_imag: Vec<f64>,
    #[serde(default)]
    floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    #[serde(default)]
    floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    #[serde(default)]
    trivial_floquet_multiplier_index: Option<usize>,
    #[serde(default = "indeterminate_floquet_verdict")]
    floquet_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    #[serde(default)]
    floquet_authenticated: bool,
    analysis_floquet_real: Vec<f64>,
    analysis_floquet_imag: Vec<f64>,
    #[serde(default)]
    analysis_is_stable: bool,
}

fn indeterminate_floquet_verdict() -> rspice_core::analysis::FloquetStabilityVerdict {
    rspice_core::analysis::FloquetStabilityVerdict::Indeterminate
}

fn pss_floquet_contract_is_authenticated(
    result: &rspice_core::analysis::pss::PssResult,
    monodromy_order: usize,
) -> bool {
    if !result.has_consistent_floquet_contract() {
        return false;
    }
    match &result.floquet_evidence {
        rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } => {
            certificate.is_valid()
                && monodromy_order > 0
                && certificate.problem_order == monodromy_order
                && certificate.problem_order == result.floquet_multipliers.len()
        }
        rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes => {
            monodromy_order == 0
                && result.floquet_multipliers.is_empty()
                && result.floquet_orbit_kind == rspice_core::analysis::FloquetOrbitKind::Driven
        }
        _ => false,
    }
}

impl PeriodicStateArtifact {
    const MAX_NUMERIC_VALUES: usize = 16_777_216;

    pub(in crate::simulation) fn operating_point(&self) -> &rspice_core::engine::PssOperatingPoint {
        &self.operating_point
    }

    pub(in crate::simulation) fn validate_consumer_basis(
        &self,
        consumer: &str,
        fundamental_freq: f64,
        num_harmonics: usize,
        tolerance: f64,
        require_autonomous: bool,
    ) -> Result<(), ExecutionArtifactError> {
        let config = self.operating_point.config();
        let producer_frequency = if config.is_autonomous() {
            1.0 / config.period_guess
        } else {
            config.fundamental_freq
        };
        if producer_frequency.to_bits() != fundamental_freq.to_bits()
            || config.num_harmonics != num_harmonics
            || config.tolerance.to_bits() != tolerance.to_bits()
        {
            return Err(ExecutionArtifactError::ContractMismatch(format!(
                "{consumer} periodic basis ({fundamental_freq:.16e} Hz, {num_harmonics} harmonics, tolerance {tolerance:.16e}) does not exactly match producer PSS basis ({producer_frequency:.16e} Hz, {} harmonics, tolerance {:.16e})",
                config.num_harmonics, config.tolerance
            )));
        }
        validate_periodic_state_contract(
            consumer,
            PeriodicStateCapability {
                shooting: true,
                autonomous: config.is_autonomous(),
            },
            require_autonomous,
        )
        .map_err(ExecutionArtifactError::ContractMismatch)?;
        Ok(())
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        let analysis = self.operating_point.analysis();
        rspice_core::engine::PssOperatingPoint::try_from_parts(
            self.operating_point.config().clone(),
            analysis.clone(),
            self.operating_point.shooting_state().to_vec(),
        )
        .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;

        if self.floquet_evidence != analysis.result.floquet_evidence
            || self.floquet_orbit_kind != analysis.result.floquet_orbit_kind
            || self.trivial_floquet_multiplier_index
                != analysis.result.trivial_floquet_multiplier_index
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state Floquet evidence compatibility metadata does not match the canonical result"
                    .to_owned(),
            ));
        }
        let computed_verdict = analysis.result.stability_verdict();
        let computed_authenticated =
            pss_floquet_contract_is_authenticated(&analysis.result, analysis.monodromy.len());
        if self.floquet_verdict != computed_verdict {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state Floquet verdict compatibility metadata does not match the canonical result"
                    .to_owned(),
            ));
        }
        if self.floquet_authenticated != computed_authenticated || !computed_authenticated {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state payload lacks authenticated Floquet evidence".to_owned(),
            ));
        }
        if self.analysis_is_stable != analysis.is_stable
            || analysis.is_stable
                != (computed_verdict == rspice_core::analysis::FloquetStabilityVerdict::Stable)
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state stability compatibility flag does not match the canonical Floquet verdict"
                    .to_owned(),
            ));
        }

        validate_complex_cache(
            "PSS result Floquet",
            &analysis.result.floquet_multipliers,
            &self.result_floquet_real,
            &self.result_floquet_imag,
        )?;
        validate_complex_cache(
            "PSS analysis Floquet",
            &analysis.floquet_multipliers,
            &self.analysis_floquet_real,
            &self.analysis_floquet_imag,
        )?;

        let mut value_count = analysis.result.time.len();
        for waveform in &analysis.result.waveforms {
            value_count = value_count
                .checked_add(waveform.values.len())
                .ok_or_else(|| {
                    ExecutionArtifactError::InvalidPayload(
                        "periodic-state numeric payload size overflows this platform".to_owned(),
                    )
                })?;
        }
        for row in &analysis.monodromy {
            value_count = value_count.checked_add(row.len()).ok_or_else(|| {
                ExecutionArtifactError::InvalidPayload(
                    "periodic-state numeric payload size overflows this platform".to_owned(),
                )
            })?;
        }
        value_count = value_count
            .checked_add(analysis.result.floquet_multipliers.len().saturating_mul(2))
            .and_then(|count| {
                count.checked_add(analysis.floquet_multipliers.len().saturating_mul(2))
            })
            .and_then(|count| count.checked_add(self.operating_point.shooting_state().len()))
            .ok_or_else(|| {
                ExecutionArtifactError::InvalidPayload(
                    "periodic-state numeric payload size overflows this platform".to_owned(),
                )
            })?;
        if value_count > Self::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::InvalidPayload(format!(
                "periodic-state payload contains {value_count} numerical values, exceeding the authenticated transport limit {}",
                Self::MAX_NUMERIC_VALUES
            )));
        }
        Ok(())
    }

    fn digest(&self) -> ContentDigest {
        let analysis = self.operating_point.analysis();
        let config = self.operating_point.config();
        let result = &analysis.result;
        let mut writer = CanonicalWriter::new("rspice.periodic-state-artifact/v2");
        writer.f64(config.fundamental_freq);
        writer.usize(config.num_harmonics);
        writer.f64(config.tstab);
        writer.usize(config.max_iterations);
        writer.f64(config.tolerance);
        writer.f64(config.abstol);
        writer.bool(config.auto_period);
        writer.option(config.oscillator_node.as_deref(), |writer, value| {
            writer.string(value)
        });
        writer.f64(config.period_guess);
        writer.usize(config.tstab_periods);
        writer.f64(config.damping_factor);
        writer.f64(config.max_period_change);
        writer.option(config.integration_method.as_ref(), |writer, method| {
            writer.u8(match method {
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler => 0,
                rspice_core::numerics::integration::IntegrationMethod::Trapezoidal => 1,
                rspice_core::numerics::integration::IntegrationMethod::Gear2 => 2,
                rspice_core::numerics::integration::IntegrationMethod::TrapGear => 3,
            });
        });
        writer.usize(config.points_per_period);
        writer.bool(config.verbose);
        writer.f64(result.period);
        writer.f64(result.frequency);
        writer.usize(result.iterations);
        writer.f64(result.residual_norm);
        writer.sequence(result.time.len());
        for value in &result.time {
            writer.f64(*value);
        }
        writer.sequence(result.node_names.len());
        for (name, waveform) in result.node_names.iter().zip(&result.waveforms) {
            writer.string(name);
            writer.sequence(waveform.values.len());
            for value in &waveform.values {
                writer.f64(*value);
            }
        }
        writer.bool(result.period_detected);
        encode_complex_values(&mut writer, &result.floquet_multipliers);
        encode_floquet_evidence(&mut writer, &self.floquet_evidence);
        encode_floquet_orbit_kind(&mut writer, self.floquet_orbit_kind);
        writer.option(
            self.trivial_floquet_multiplier_index.as_ref(),
            |writer, index| writer.usize(*index),
        );
        encode_floquet_verdict(&mut writer, self.floquet_verdict);
        writer.bool(self.floquet_authenticated);
        writer.usize(analysis.iterations);
        writer.f64(analysis.final_residual);
        writer.f64(analysis.period);
        writer.sequence(analysis.monodromy.len());
        for row in &analysis.monodromy {
            writer.sequence(row.len());
            for value in row {
                writer.f64(*value);
            }
        }
        encode_complex_values(&mut writer, &analysis.floquet_multipliers);
        writer.bool(analysis.is_stable);
        writer.bool(self.analysis_is_stable);
        writer.sequence(self.operating_point.shooting_state().len());
        for value in self.operating_point.shooting_state() {
            writer.f64(*value);
        }
        writer.finish()
    }
}

fn encode_floquet_evidence(
    writer: &mut CanonicalWriter,
    evidence: &rspice_core::analysis::FloquetSpectrumEvidence,
) {
    match evidence {
        rspice_core::analysis::FloquetSpectrumEvidence::NotComputed => writer.u8(0),
        rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes => writer.u8(1),
        rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } => {
            writer.u8(2);
            writer.usize(certificate.problem_order);
            writer.f64(certificate.max_backward_error);
            writer.f64(certificate.qualification_tolerance);
        }
        rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown => writer.u8(3),
        _ => writer.u8(u8::MAX),
    }
}

fn encode_floquet_orbit_kind(
    writer: &mut CanonicalWriter,
    orbit_kind: rspice_core::analysis::FloquetOrbitKind,
) {
    writer.u8(match orbit_kind {
        rspice_core::analysis::FloquetOrbitKind::Driven => 0,
        rspice_core::analysis::FloquetOrbitKind::Autonomous => 1,
        _ => u8::MAX,
    });
}

fn encode_floquet_verdict(
    writer: &mut CanonicalWriter,
    verdict: rspice_core::analysis::FloquetStabilityVerdict,
) {
    writer.u8(match verdict {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => 0,
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => 1,
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => 2,
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => 3,
        _ => u8::MAX,
    });
}

fn validate_complex_cache(
    label: &str,
    values: &[num_complex::Complex64],
    real: &[f64],
    imaginary: &[f64],
) -> Result<(), ExecutionArtifactError> {
    if values.len() != real.len()
        || values.len() != imaginary.len()
        || values
            .iter()
            .zip(real)
            .zip(imaginary)
            .any(|((value, re), im)| {
                value.re.to_bits() != re.to_bits() || value.im.to_bits() != im.to_bits()
            })
    {
        return Err(ExecutionArtifactError::InvalidPayload(format!(
            "{label} transfer cache does not match the retained complex values"
        )));
    }
    Ok(())
}

fn split_complex_values(values: &[num_complex::Complex64]) -> (Vec<f64>, Vec<f64>) {
    values
        .iter()
        .map(|value| (value.re, value.im))
        .unzip::<_, _, Vec<_>, Vec<_>>()
}

#[cfg(any(target_arch = "wasm32", test))]
fn join_complex_values(
    label: &str,
    real: &[f64],
    imaginary: &[f64],
) -> Result<Vec<num_complex::Complex64>, ExecutionArtifactError> {
    if real.len() != imaginary.len() {
        return Err(ExecutionArtifactError::InvalidPayload(format!(
            "{label} real/imaginary vector lengths differ ({} versus {})",
            real.len(),
            imaginary.len()
        )));
    }
    Ok(real
        .iter()
        .copied()
        .zip(imaginary.iter().copied())
        .map(|(re, im)| num_complex::Complex64::new(re, im))
        .collect())
}

fn encode_complex_values(writer: &mut CanonicalWriter, values: &[num_complex::Complex64]) {
    writer.sequence(values.len());
    for value in values {
        writer.f64(value.re);
        writer.f64(value.im);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct HbStateArtifact {
    operating_point: Arc<rspice_core::engine::HbOperatingPoint>,
    spectral_real: Vec<Vec<f64>>,
    spectral_imaginary: Vec<Vec<f64>>,
}

impl HbStateArtifact {
    const MAX_NUMERIC_VALUES: usize = 16_777_216;

    pub(in crate::simulation) fn operating_point(&self) -> &rspice_core::engine::HbOperatingPoint {
        &self.operating_point
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        rspice_core::engine::HbOperatingPoint::try_from_parts(
            self.operating_point.config().clone(),
            self.operating_point.node_names().to_vec(),
            self.operating_point.spectral_state().to_vec(),
            self.operating_point.iterations(),
            self.operating_point.residual_norm(),
        )
        .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        if self.spectral_real.len() != self.operating_point.spectral_state().len()
            || self.spectral_imaginary.len() != self.operating_point.spectral_state().len()
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "HB-state transfer cache row count does not match the retained state".to_owned(),
            ));
        }
        let mut numeric_values = 0usize;
        for (index, coefficients) in self.operating_point.spectral_state().iter().enumerate() {
            let real = &self.spectral_real[index];
            let imaginary = &self.spectral_imaginary[index];
            validate_complex_cache("HB spectral row", coefficients, real, imaginary)?;
            numeric_values = numeric_values
                .checked_add(real.len().saturating_mul(2))
                .ok_or_else(|| {
                    ExecutionArtifactError::InvalidPayload(
                        "HB-state numeric payload size overflows this platform".to_owned(),
                    )
                })?;
        }
        if numeric_values > Self::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::InvalidPayload(format!(
                "HB-state payload contains {numeric_values} numerical values, exceeding the authenticated transport limit {}",
                Self::MAX_NUMERIC_VALUES
            )));
        }
        Ok(())
    }

    fn digest(&self) -> ContentDigest {
        let point = &self.operating_point;
        let config = point.config();
        let mut writer = CanonicalWriter::new("rspice.hb-state-artifact/v1");
        encode_hb_config(&mut writer, config);
        writer.usize(point.iterations());
        writer.f64(point.residual_norm());
        writer.sequence(point.node_names().len());
        for (node, spectrum) in point.node_names().iter().zip(point.spectral_state()) {
            writer.string(node);
            encode_complex_values(&mut writer, spectrum);
        }
        writer.finish()
    }
}

fn encode_hb_config(writer: &mut CanonicalWriter, config: &rspice_core::analysis::HbConfig) {
    writer.f64(config.fundamental_freq);
    writer.usize(config.num_harmonics);
    writer.sequence(config.tones.len());
    for tone in &config.tones {
        writer.f64(tone.frequency);
        writer.usize(tone.num_harmonics);
        writer.string(&tone.name);
        writer.option(tone.source_name.as_deref(), |writer, source| {
            writer.string(source)
        });
    }
    writer.f64(config.tolerance);
    writer.f64(config.abstol);
    writer.usize(config.max_iterations);
    writer.f64(config.damping);
    writer.f64(config.min_damping);
    writer.usize(config.oversample_factor);
    writer.option(config.collocation_points.as_ref(), |writer, points| {
        writer.usize(*points)
    });
    writer.usize(config.max_mixing_order);
    writer.bool(config.use_krylov);
    writer.usize(config.gmres_restart);
    writer.bool(config.source_stepping);
    writer.bool(config.use_exact_jacobian);
    writer.bool(config.verbose);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ExecutionArtifactPayload {
    TransientTrajectory(Arc<TransientTrajectoryArtifact>),
    PeriodicState(Arc<PeriodicStateArtifact>),
    HbState(Arc<HbStateArtifact>),
    DcOperatingPointSeed(Arc<DcOperatingPointSeedArtifact>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct ExecutionArtifactEnvelope {
    snapshot_digest: ContentDigest,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
    kind: ExecutionArtifactKind,
    payload_digest: ContentDigest,
    payload: ExecutionArtifactPayload,
}

impl ExecutionArtifactEnvelope {
    pub(in crate::simulation) fn from_transient_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        result: &SimulationResult,
        required_waveforms: &[String],
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::Transient {
            time, waveforms, ..
        } = result
        else {
            return Ok(None);
        };
        if required_waveforms.is_empty() {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient artifact request contains no required waveforms".to_owned(),
            ));
        }
        let mut artifact_waveforms = BTreeMap::new();
        for (name, waveform) in waveforms {
            if !required_waveforms
                .iter()
                .any(|required| normalize_waveform_name(required) == normalize_waveform_name(name))
            {
                continue;
            }
            if waveform.is_complex || waveform.y_imag.is_some() {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' unexpectedly contains complex values"
                )));
            }
            if waveform.x_values.len() != time.len()
                || waveform
                    .x_values
                    .iter()
                    .zip(time)
                    .any(|(waveform_time, common_time)| {
                        waveform_time.to_bits() != common_time.to_bits()
                    })
            {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' does not use the result's canonical time axis"
                )));
            }
            if artifact_waveforms
                .insert(name.clone(), waveform.y_values.clone())
                .is_some()
            {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient trajectory repeats waveform '{name}'"
                )));
            }
        }
        for required in required_waveforms {
            if !artifact_waveforms
                .keys()
                .any(|name| normalize_waveform_name(name) == normalize_waveform_name(required))
            {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "required transient waveform '{required}' is absent from the producer result"
                )));
            }
        }
        let trajectory = TransientTrajectoryArtifact {
            time: time.clone(),
            waveforms: artifact_waveforms,
        };
        trajectory.validate()?;
        let payload_digest = trajectory.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::TransientTrajectory,
            payload_digest,
            payload: ExecutionArtifactPayload::TransientTrajectory(Arc::new(trajectory)),
        }))
    }

    pub(in crate::simulation) fn from_periodic_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::Transient { periodic_state, .. } = result else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "PSS producer returned a non-periodic result variant".to_owned(),
            ));
        };
        let operating_point = periodic_state.as_ref().ok_or_else(|| {
            ExecutionArtifactError::InvalidPayload(
                "PSS producer did not retain the required numerical periodic state".to_owned(),
            )
        })?;
        validate_periodic_producer_config(producer_spec, operating_point.config())?;
        let (result_floquet_real, result_floquet_imag) =
            split_complex_values(&operating_point.analysis().result.floquet_multipliers);
        let (analysis_floquet_real, analysis_floquet_imag) =
            split_complex_values(&operating_point.analysis().floquet_multipliers);
        let analysis = operating_point.analysis();
        let floquet_verdict = analysis.result.stability_verdict();
        let floquet_authenticated =
            pss_floquet_contract_is_authenticated(&analysis.result, analysis.monodromy.len());
        let periodic_state = PeriodicStateArtifact {
            operating_point: Arc::clone(operating_point),
            result_floquet_real,
            result_floquet_imag,
            floquet_evidence: analysis.result.floquet_evidence.clone(),
            floquet_orbit_kind: analysis.result.floquet_orbit_kind,
            trivial_floquet_multiplier_index: analysis.result.trivial_floquet_multiplier_index,
            floquet_verdict,
            floquet_authenticated,
            analysis_floquet_real,
            analysis_floquet_imag,
            analysis_is_stable: analysis.is_stable,
        };
        periodic_state.validate()?;
        let payload_digest = periodic_state.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::PeriodicState,
            payload_digest,
            payload: ExecutionArtifactPayload::PeriodicState(Arc::new(periodic_state)),
        }))
    }

    pub(in crate::simulation) fn from_hb_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::HarmonicBalance {
            operating_point, ..
        } = result
        else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "HB producer returned a non-HB result variant".to_owned(),
            ));
        };
        validate_hb_producer_config(producer_spec, operating_point.config())?;
        let (spectral_real, spectral_imaginary): (Vec<_>, Vec<_>) = operating_point
            .spectral_state()
            .iter()
            .map(|row| split_complex_values(row))
            .unzip();
        let state = HbStateArtifact {
            operating_point: Arc::clone(operating_point),
            spectral_real,
            spectral_imaginary,
        };
        state.validate()?;
        let payload_digest = state.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::HbState,
            payload_digest,
            payload: ExecutionArtifactPayload::HbState(Arc::new(state)),
        }))
    }

    pub(in crate::simulation) fn from_dc_operating_point_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        effective_source_content_digest: ContentDigest,
        prepared_config: &crate::simulation::dialog::OpConfig,
        result: &SimulationResult,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::DcOp(result) = result else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "operating-point producer returned a non-OP result variant".to_owned(),
            ));
        };
        if &result.configuration != prepared_config {
            return Err(ExecutionArtifactError::InvalidPayload(
                "worker-returned operating-point configuration does not match the authenticated prepared producer configuration"
                    .to_owned(),
            ));
        }
        let run_point = prepared_config.run_point.clone();
        let seed = DcOperatingPointSeedArtifact {
            effective_source_content_digest,
            temperature_celsius: prepared_config.temperature_celsius,
            supply_voltage: run_point.supply_voltage,
            nominal_supply_voltage: run_point.nominal_supply_voltage,
            supply_source_names: run_point.supply_source_names,
            node_names: result.mna_node_names.clone(),
            branch_names: result.mna_branch_names.clone(),
            solution: result.mna_solution.clone(),
        };
        seed.validate()?;
        let payload_digest = seed.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::DcOperatingPointSeed,
            payload_digest,
            payload: ExecutionArtifactPayload::DcOperatingPointSeed(Arc::new(seed)),
        }))
    }

    pub(in crate::simulation) fn trajectory(&self) -> Option<&TransientTrajectoryArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::TransientTrajectory(trajectory) => Some(trajectory),
            ExecutionArtifactPayload::PeriodicState(_)
            | ExecutionArtifactPayload::HbState(_)
            | ExecutionArtifactPayload::DcOperatingPointSeed(_) => None,
        }
    }

    pub(in crate::simulation) fn periodic_state(&self) -> Option<&PeriodicStateArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::PeriodicState(state) => Some(state),
            ExecutionArtifactPayload::TransientTrajectory(_)
            | ExecutionArtifactPayload::HbState(_)
            | ExecutionArtifactPayload::DcOperatingPointSeed(_) => None,
        }
    }

    pub(in crate::simulation) fn hb_state(&self) -> Option<&HbStateArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::HbState(state) => Some(state),
            ExecutionArtifactPayload::TransientTrajectory(_)
            | ExecutionArtifactPayload::PeriodicState(_)
            | ExecutionArtifactPayload::DcOperatingPointSeed(_) => None,
        }
    }

    pub(in crate::simulation) fn dc_operating_point_seed(
        &self,
    ) -> Option<&DcOperatingPointSeedArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::DcOperatingPointSeed(seed) => Some(seed),
            ExecutionArtifactPayload::TransientTrajectory(_)
            | ExecutionArtifactPayload::PeriodicState(_)
            | ExecutionArtifactPayload::HbState(_) => None,
        }
    }

    fn validate_against(
        &self,
        snapshot_digest: ContentDigest,
        binding: &PreparedDependencyBinding,
    ) -> Result<(), ExecutionArtifactError> {
        if self.snapshot_digest != snapshot_digest {
            return Err(ExecutionArtifactError::StaleSnapshot {
                expected: snapshot_digest,
                actual: self.snapshot_digest,
            });
        }
        if self.kind != binding.kind
            || self.producer_instance_id != binding.producer_instance_id
            || self.producer_source_revision != binding.producer_source_revision
            || self.producer_config_digest != binding.producer_config_digest
        {
            return Err(ExecutionArtifactError::ProducerMismatch {
                expected: binding.producer_instance_id,
                actual: self.producer_instance_id,
            });
        }
        let actual_digest = match self.kind {
            ExecutionArtifactKind::TransientTrajectory => {
                let ExecutionArtifactPayload::TransientTrajectory(trajectory) = &self.payload
                else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "transient artifact carries the wrong payload variant".to_owned(),
                    ));
                };
                trajectory.validate()?;
                trajectory.digest()
            }
            ExecutionArtifactKind::PeriodicState => {
                let ExecutionArtifactPayload::PeriodicState(periodic_state) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "periodic-state artifact carries the wrong payload variant".to_owned(),
                    ));
                };
                periodic_state.validate()?;
                periodic_state.digest()
            }
            ExecutionArtifactKind::HbState => {
                let ExecutionArtifactPayload::HbState(state) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "HB-state artifact carries the wrong payload variant".to_owned(),
                    ));
                };
                state.validate()?;
                state.digest()
            }
            ExecutionArtifactKind::DcOperatingPointSeed => {
                let ExecutionArtifactPayload::DcOperatingPointSeed(seed) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "DC operating-point seed artifact carries the wrong payload variant"
                            .to_owned(),
                    ));
                };
                seed.validate()?;
                seed.digest()
            }
        };
        if actual_digest != self.payload_digest {
            return Err(ExecutionArtifactError::PayloadDigestMismatch {
                expected: self.payload_digest,
                actual: actual_digest,
            });
        }
        Ok(())
    }
}

fn normalize_waveform_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && (trimmed.starts_with("V(") || trimmed.starts_with("v("))
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct ResolvedExecutionDependencies {
    snapshot_digest: Option<ContentDigest>,
    bindings: Vec<PreparedDependencyBinding>,
    artifacts: Vec<ExecutionArtifactEnvelope>,
}

impl ResolvedExecutionDependencies {
    pub(in crate::simulation) fn resolve(
        snapshot_digest: ContentDigest,
        bindings: Vec<PreparedDependencyBinding>,
        artifacts: &HashMap<AnalysisInstanceId, ExecutionArtifactEnvelope>,
    ) -> Result<Self, ExecutionArtifactError> {
        if bindings.is_empty() {
            return Ok(Self::default());
        }
        let mut resolved = Vec::with_capacity(bindings.len());
        for binding in &bindings {
            let artifact = artifacts.get(&binding.producer_instance_id).ok_or(
                ExecutionArtifactError::Missing {
                    producer: binding.producer_instance_id,
                    kind: binding.kind,
                },
            )?;
            artifact.validate_against(snapshot_digest, binding)?;
            resolved.push(artifact.clone());
        }
        Ok(Self {
            snapshot_digest: Some(snapshot_digest),
            bindings,
            artifacts: resolved,
        })
    }

    pub(in crate::simulation) fn validate_for_spec(
        &self,
        spec: &AnalysisSpec,
    ) -> Result<(), ExecutionArtifactError> {
        let expected_kind = match spec {
            AnalysisSpec::Fourier { .. } => Some(ExecutionArtifactKind::TransientTrajectory),
            AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. } => {
                Some(ExecutionArtifactKind::HbState)
            }
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                ..
            } => Some(ExecutionArtifactKind::DcOperatingPointSeed),
            AnalysisSpec::Pac
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pstb
            | AnalysisSpec::Psp { .. } => Some(ExecutionArtifactKind::PeriodicState),
            _ => None,
        };
        let expected_count = usize::from(expected_kind.is_some());
        if self.bindings.len() != expected_count || self.artifacts.len() != expected_count {
            return Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} requires {expected_count} typed execution artifact(s), received {} binding(s) and {} artifact(s)",
                spec.run_type().display_name(),
                self.bindings.len(),
                self.artifacts.len()
            )));
        }
        if expected_count == 0 {
            if self.snapshot_digest.is_some() {
                return Err(ExecutionArtifactError::ContractMismatch(
                    "an artifact-free task carried an unexpected dependency snapshot identity"
                        .to_owned(),
                ));
            }
            return Ok(());
        }
        let snapshot_digest = self.snapshot_digest.ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "typed execution dependencies have no prepared snapshot identity".to_owned(),
            )
        })?;
        let binding = &self.bindings[0];
        if Some(binding.kind) != expected_kind {
            return Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} requires a {:?} artifact",
                spec.run_type().display_name(),
                expected_kind.expect("artifact-backed task has an expected kind")
            )));
        }
        self.artifacts[0].validate_against(snapshot_digest, binding)
    }

    pub(in crate::simulation) fn validate_for_config(&self) -> Result<(), ExecutionArtifactError> {
        if self.snapshot_digest.is_none() && self.bindings.is_empty() && self.artifacts.is_empty() {
            Ok(())
        } else {
            Err(ExecutionArtifactError::ContractMismatch(
                "configuration-backed task carried unexpected typed execution dependencies"
                    .to_owned(),
            ))
        }
    }

    pub(in crate::simulation) fn transient_trajectory(
        &self,
    ) -> Result<&TransientTrajectoryArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::TransientTrajectory
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one transient-trajectory artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].trajectory().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved transient dependency has no trajectory payload".to_owned(),
            )
        })
    }

    pub(in crate::simulation) fn periodic_state(
        &self,
    ) -> Result<&PeriodicStateArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::PeriodicState
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one periodic-state artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].periodic_state().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved periodic dependency has no numerical state payload".to_owned(),
            )
        })
    }

    pub(in crate::simulation) fn hb_state(
        &self,
    ) -> Result<&HbStateArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::HbState
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one HB-state artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].hb_state().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved HB dependency has no numerical state payload".to_owned(),
            )
        })
    }

    pub(in crate::simulation) fn dc_operating_point_seed(
        &self,
    ) -> Result<&DcOperatingPointSeedArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::DcOperatingPointSeed
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one DC operating-point seed artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].dc_operating_point_seed().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved operating-point dependency has no DC seed payload".to_owned(),
            )
        })
    }

    /// Split numerical payloads from request metadata for the browser worker.
    ///
    /// The metadata is deliberately a small JSON object containing only
    /// identities, digests, names, and buffer references. Floating-point
    /// samples travel separately as `Float64Array` transfer buffers, avoiding
    /// per-sample string/JavaScript-object expansion while retaining their
    /// exact IEEE-754 bit patterns.
    #[cfg(test)]
    pub(in crate::simulation) fn encode_transfer(
        &self,
    ) -> Result<(String, Vec<Vec<f64>>), ExecutionArtifactError> {
        let (encoded, buffers) = self.encode_transfer_borrowed()?;
        Ok((encoded, buffers.into_iter().map(<[f64]>::to_vec).collect()))
    }

    /// Encode transfer metadata while borrowing the numerical payloads.
    ///
    /// The browser sender uses this form to copy each retained artifact slice
    /// directly into its transferable `Float64Array`. Keeping the owned form
    /// above is useful for deterministic native transport round-trip tests,
    /// but must not introduce a second full payload allocation on the browser
    /// main thread.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(in crate::simulation) fn encode_transfer_borrowed(
        &self,
    ) -> Result<(String, Vec<&[f64]>), ExecutionArtifactError> {
        self.validate_transport_integrity()?;

        let mut buffers = Vec::new();
        let artifacts = self
            .artifacts
            .iter()
            .map(|artifact| {
                let payload = match &artifact.payload {
                    ExecutionArtifactPayload::DcOperatingPointSeed(seed) => {
                        let solution = push_transfer_slice(&mut buffers, &seed.solution);
                        ExecutionArtifactPayloadTransferMetadata::DcOperatingPointSeed(
                            DcOperatingPointSeedTransferMetadata {
                                effective_source_content_digest: seed
                                    .effective_source_content_digest,
                                temperature_celsius: seed.temperature_celsius,
                                supply_voltage: seed.supply_voltage,
                                nominal_supply_voltage: seed.nominal_supply_voltage,
                                supply_source_names: seed.supply_source_names.clone(),
                                node_names: seed.node_names.clone(),
                                branch_names: seed.branch_names.clone(),
                                solution,
                            },
                        )
                    }
                    ExecutionArtifactPayload::TransientTrajectory(trajectory) => {
                        let time = push_transfer_slice(&mut buffers, &trajectory.time);
                        let waveforms = trajectory
                            .waveforms
                            .iter()
                            .map(|(name, values)| {
                                (name.clone(), push_transfer_slice(&mut buffers, values))
                            })
                            .collect();
                        ExecutionArtifactPayloadTransferMetadata::TransientTrajectory(
                            TransientTrajectoryTransferMetadata { time, waveforms },
                        )
                    }
                    ExecutionArtifactPayload::PeriodicState(periodic) => {
                        let analysis = periodic.operating_point.analysis();
                        let config = periodic.operating_point.config();
                        let result = &analysis.result;
                        let time = push_transfer_slice(&mut buffers, &result.time);
                        let waveforms = result
                            .node_names
                            .iter()
                            .zip(&result.waveforms)
                            .map(|(node_name, waveform)| PeriodicWaveformTransferMetadata {
                                node_name: node_name.clone(),
                                values: push_transfer_slice(&mut buffers, &waveform.values),
                            })
                            .collect();
                        let result_floquet_real =
                            push_transfer_slice(&mut buffers, &periodic.result_floquet_real);
                        let result_floquet_imag =
                            push_transfer_slice(&mut buffers, &periodic.result_floquet_imag);
                        let monodromy = analysis
                            .monodromy
                            .iter()
                            .map(|row| push_transfer_slice(&mut buffers, row))
                            .collect();
                        let analysis_floquet_real =
                            push_transfer_slice(&mut buffers, &periodic.analysis_floquet_real);
                        let analysis_floquet_imag =
                            push_transfer_slice(&mut buffers, &periodic.analysis_floquet_imag);
                        let shooting_state = push_transfer_slice(
                            &mut buffers,
                            periodic.operating_point.shooting_state(),
                        );
                        ExecutionArtifactPayloadTransferMetadata::PeriodicState(
                            PeriodicStateTransferMetadata {
                                config_fundamental_freq: config.fundamental_freq,
                                config_num_harmonics: config.num_harmonics,
                                config_tstab: config.tstab,
                                config_max_iterations: config.max_iterations,
                                config_tolerance: config.tolerance,
                                config_abstol: config.abstol,
                                config_auto_period: config.auto_period,
                                config_oscillator_node: config.oscillator_node.clone(),
                                config_period_guess: config.period_guess,
                                config_tstab_periods: config.tstab_periods,
                                config_damping_factor: config.damping_factor,
                                config_max_period_change: config.max_period_change,
                                config_integration_method: config.integration_method.map(
                                    |method| match method {
                                        rspice_core::numerics::integration::IntegrationMethod::BackwardEuler => {
                                            0
                                        }
                                        rspice_core::numerics::integration::IntegrationMethod::Trapezoidal => 1,
                                        rspice_core::numerics::integration::IntegrationMethod::Gear2 => 2,
                                        rspice_core::numerics::integration::IntegrationMethod::TrapGear => 3,
                                    },
                                ),
                                config_points_per_period: config.points_per_period,
                                config_verbose: config.verbose,
                                result_period: result.period,
                                result_frequency: result.frequency,
                                result_iterations: result.iterations,
                                result_residual_norm: result.residual_norm,
                                time,
                                waveforms,
                                period_detected: result.period_detected,
                                result_floquet_real,
                                result_floquet_imag,
                                floquet_evidence: periodic.floquet_evidence.clone(),
                                floquet_orbit_kind: periodic.floquet_orbit_kind,
                                trivial_floquet_multiplier_index: periodic
                                    .trivial_floquet_multiplier_index,
                                floquet_verdict: periodic.floquet_verdict,
                                floquet_authenticated: periodic.floquet_authenticated,
                                analysis_iterations: analysis.iterations,
                                analysis_final_residual: analysis.final_residual,
                                analysis_period: analysis.period,
                                monodromy,
                                analysis_floquet_real,
                                analysis_floquet_imag,
                                is_stable: analysis.is_stable,
                                analysis_is_stable: periodic.analysis_is_stable,
                                shooting_state,
                            },
                        )
                    }
                    ExecutionArtifactPayload::HbState(state) => {
                        let spectra = state
                            .operating_point
                            .node_names()
                            .iter()
                            .enumerate()
                            .map(|(index, node_name)| HbSpectrumTransferMetadata {
                                node_name: node_name.clone(),
                                real: push_transfer_slice(
                                    &mut buffers,
                                    &state.spectral_real[index],
                                ),
                                imaginary: push_transfer_slice(
                                    &mut buffers,
                                    &state.spectral_imaginary[index],
                                ),
                            })
                            .collect();
                        ExecutionArtifactPayloadTransferMetadata::HbState(
                            HbStateTransferMetadata {
                                config: state.operating_point.config().clone(),
                                spectra,
                                iterations: state.operating_point.iterations(),
                                residual_norm: state.operating_point.residual_norm(),
                            },
                        )
                    }
                };
                ExecutionArtifactTransferMetadata {
                    snapshot_digest: artifact.snapshot_digest,
                    producer_instance_id: artifact.producer_instance_id,
                    producer_source_revision: artifact.producer_source_revision,
                    producer_config_digest: artifact.producer_config_digest,
                    kind: artifact.kind,
                    payload_digest: artifact.payload_digest,
                    payload,
                }
            })
            .collect();
        let metadata = ResolvedExecutionDependenciesTransferMetadata {
            snapshot_digest: self.snapshot_digest,
            bindings: self.bindings.clone(),
            artifacts,
        };
        let encoded = serde_json::to_string(&metadata).map_err(|error| {
            ExecutionArtifactError::Transport(format!(
                "could not encode dependency transfer metadata: {error}"
            ))
        })?;
        Ok((encoded, buffers))
    }

    /// Reconstruct and authenticate dependencies received from transfer
    /// buffers. Every buffer must be referenced exactly once and every
    /// reconstructed artifact must still match its prepared binding and
    /// content digest.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(in crate::simulation) fn decode_transfer(
        encoded: &str,
        buffers: Vec<Vec<f64>>,
    ) -> Result<Self, ExecutionArtifactError> {
        const MAX_METADATA_BYTES: usize = 4 * 1024 * 1024;
        const MAX_TRANSFER_BUFFERS: usize = 65_536;
        if encoded.len() > MAX_METADATA_BYTES {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency metadata contains {} bytes, exceeding the {MAX_METADATA_BYTES}-byte limit",
                encoded.len()
            )));
        }
        if buffers.len() > MAX_TRANSFER_BUFFERS {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer contains {} buffers, exceeding the {MAX_TRANSFER_BUFFERS}-buffer limit",
                buffers.len()
            )));
        }
        let numeric_values = buffers.iter().try_fold(0usize, |total, values| {
            total.checked_add(values.len()).ok_or_else(|| {
                ExecutionArtifactError::Transport(
                    "dependency transfer numeric size overflows this platform".to_owned(),
                )
            })
        })?;
        if numeric_values > PeriodicStateArtifact::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer contains {numeric_values} numerical values, exceeding the {}-value limit",
                PeriodicStateArtifact::MAX_NUMERIC_VALUES
            )));
        }
        let metadata: ResolvedExecutionDependenciesTransferMetadata = serde_json::from_str(encoded)
            .map_err(|error| {
                ExecutionArtifactError::Transport(format!(
                    "could not decode dependency transfer metadata: {error}"
                ))
            })?;
        let mut buffers = buffers.into_iter().map(Some).collect::<Vec<_>>();
        let artifacts = metadata
            .artifacts
            .into_iter()
            .map(|artifact| {
                let payload = match artifact.payload {
                    ExecutionArtifactPayloadTransferMetadata::DcOperatingPointSeed(metadata) => {
                        let seed = DcOperatingPointSeedArtifact {
                            effective_source_content_digest: metadata
                                .effective_source_content_digest,
                            temperature_celsius: metadata.temperature_celsius,
                            supply_voltage: metadata.supply_voltage,
                            nominal_supply_voltage: metadata.nominal_supply_voltage,
                            supply_source_names: metadata.supply_source_names,
                            node_names: metadata.node_names,
                            branch_names: metadata.branch_names,
                            solution: take_transfer_buffer(&mut buffers, metadata.solution)?,
                        };
                        seed.validate()?;
                        ExecutionArtifactPayload::DcOperatingPointSeed(Arc::new(seed))
                    }
                    ExecutionArtifactPayloadTransferMetadata::TransientTrajectory(metadata) => {
                        let time = take_transfer_buffer(&mut buffers, metadata.time)?;
                        let waveforms = metadata
                            .waveforms
                            .into_iter()
                            .map(|(name, reference)| {
                                take_transfer_buffer(&mut buffers, reference)
                                    .map(|values| (name, values))
                            })
                            .collect::<Result<_, _>>()?;
                        let trajectory = TransientTrajectoryArtifact { time, waveforms };
                        trajectory.validate()?;
                        ExecutionArtifactPayload::TransientTrajectory(Arc::new(trajectory))
                    }
                    ExecutionArtifactPayloadTransferMetadata::PeriodicState(metadata) => {
                        let time = take_transfer_buffer(&mut buffers, metadata.time)?;
                        let mut node_names = Vec::with_capacity(metadata.waveforms.len());
                        let mut waveforms = Vec::with_capacity(metadata.waveforms.len());
                        for waveform in metadata.waveforms {
                            node_names.push(waveform.node_name);
                            waveforms.push(
                                rspice_core::analysis::pss::PeriodicWaveform::from_values(
                                    take_transfer_buffer(&mut buffers, waveform.values)?,
                                ),
                            );
                        }
                        let result_floquet_real = take_transfer_buffer(
                            &mut buffers,
                            metadata.result_floquet_real,
                        )?;
                        let result_floquet_imag = take_transfer_buffer(
                            &mut buffers,
                            metadata.result_floquet_imag,
                        )?;
                        let result_floquet_multipliers = join_complex_values(
                            "PSS result Floquet",
                            &result_floquet_real,
                            &result_floquet_imag,
                        )?;
                        let monodromy = metadata
                            .monodromy
                            .into_iter()
                            .map(|reference| take_transfer_buffer(&mut buffers, reference))
                            .collect::<Result<Vec<_>, _>>()?;
                        let analysis_floquet_real = take_transfer_buffer(
                            &mut buffers,
                            metadata.analysis_floquet_real,
                        )?;
                        let analysis_floquet_imag = take_transfer_buffer(
                            &mut buffers,
                            metadata.analysis_floquet_imag,
                        )?;
                        let analysis_floquet_multipliers = join_complex_values(
                            "PSS analysis Floquet",
                            &analysis_floquet_real,
                            &analysis_floquet_imag,
                        )?;
                        let shooting_state =
                            take_transfer_buffer(&mut buffers, metadata.shooting_state)?;
                        let result = rspice_core::analysis::pss::PssResult {
                            period: metadata.result_period,
                            frequency: metadata.result_frequency,
                            iterations: metadata.result_iterations,
                            residual_norm: metadata.result_residual_norm,
                            time,
                            waveforms,
                            node_names,
                            period_detected: metadata.period_detected,
                            floquet_multipliers: result_floquet_multipliers,
                            floquet_evidence: metadata.floquet_evidence.clone(),
                            floquet_orbit_kind: metadata.floquet_orbit_kind,
                            trivial_floquet_multiplier_index: metadata
                                .trivial_floquet_multiplier_index,
                        };
                        let computed_verdict = result.stability_verdict();
                        let computed_authenticated =
                            pss_floquet_contract_is_authenticated(&result, monodromy.len());
                        if metadata.floquet_verdict != computed_verdict {
                            return Err(ExecutionArtifactError::InvalidPayload(
                                "periodic-state transfer Floquet verdict does not match its evidence"
                                    .to_owned(),
                            ));
                        }
                        if metadata.floquet_authenticated != computed_authenticated
                            || !computed_authenticated
                        {
                            return Err(ExecutionArtifactError::InvalidPayload(
                                "periodic-state transfer lacks authenticated Floquet evidence"
                                    .to_owned(),
                            ));
                        }
                        let analysis = rspice_core::engine::PssAnalysisResult {
                            result,
                            iterations: metadata.analysis_iterations,
                            final_residual: metadata.analysis_final_residual,
                            period: metadata.analysis_period,
                            monodromy,
                            floquet_multipliers: analysis_floquet_multipliers,
                            is_stable: metadata.is_stable,
                        };
                        let integration_method = metadata
                            .config_integration_method
                            .map(|tag| match tag {
                                0 => Ok(rspice_core::numerics::integration::IntegrationMethod::BackwardEuler),
                                1 => Ok(rspice_core::numerics::integration::IntegrationMethod::Trapezoidal),
                                2 => Ok(rspice_core::numerics::integration::IntegrationMethod::Gear2),
                                3 => Ok(rspice_core::numerics::integration::IntegrationMethod::TrapGear),
                                _ => Err(ExecutionArtifactError::Transport(format!(
                                    "periodic-state transfer has unknown integration-method tag {tag}"
                                ))),
                            })
                            .transpose()?;
                        let config = rspice_core::analysis::PssConfig {
                            fundamental_freq: metadata.config_fundamental_freq,
                            num_harmonics: metadata.config_num_harmonics,
                            tstab: metadata.config_tstab,
                            max_iterations: metadata.config_max_iterations,
                            tolerance: metadata.config_tolerance,
                            abstol: metadata.config_abstol,
                            auto_period: metadata.config_auto_period,
                            oscillator_node: metadata.config_oscillator_node,
                            period_guess: metadata.config_period_guess,
                            tstab_periods: metadata.config_tstab_periods,
                            damping_factor: metadata.config_damping_factor,
                            max_period_change: metadata.config_max_period_change,
                            integration_method,
                            points_per_period: metadata.config_points_per_period,
                            verbose: metadata.config_verbose,
                        };
                        let operating_point =
                            rspice_core::engine::PssOperatingPoint::try_from_parts(
                                config,
                                analysis,
                                shooting_state,
                            )
                            .map_err(|error| {
                                ExecutionArtifactError::InvalidPayload(error.to_string())
                            })?;
                        let periodic = PeriodicStateArtifact {
                            operating_point: Arc::new(operating_point),
                            result_floquet_real,
                            result_floquet_imag,
                            floquet_evidence: metadata.floquet_evidence,
                            floquet_orbit_kind: metadata.floquet_orbit_kind,
                            trivial_floquet_multiplier_index: metadata
                                .trivial_floquet_multiplier_index,
                            floquet_verdict: metadata.floquet_verdict,
                            floquet_authenticated: metadata.floquet_authenticated,
                            analysis_floquet_real,
                            analysis_floquet_imag,
                            analysis_is_stable: metadata.analysis_is_stable,
                        };
                        periodic.validate()?;
                        ExecutionArtifactPayload::PeriodicState(Arc::new(periodic))
                    }
                    ExecutionArtifactPayloadTransferMetadata::HbState(metadata) => {
                        let mut node_names = Vec::with_capacity(metadata.spectra.len());
                        let mut spectral_state = Vec::with_capacity(metadata.spectra.len());
                        let mut spectral_real = Vec::with_capacity(metadata.spectra.len());
                        let mut spectral_imaginary = Vec::with_capacity(metadata.spectra.len());
                        for spectrum in metadata.spectra {
                            node_names.push(spectrum.node_name);
                            let real = take_transfer_buffer(&mut buffers, spectrum.real)?;
                            let imaginary =
                                take_transfer_buffer(&mut buffers, spectrum.imaginary)?;
                            spectral_state.push(join_complex_values(
                                "HB spectral row",
                                &real,
                                &imaginary,
                            )?);
                            spectral_real.push(real);
                            spectral_imaginary.push(imaginary);
                        }
                        let operating_point = rspice_core::engine::HbOperatingPoint::try_from_parts(
                            metadata.config,
                            node_names,
                            spectral_state,
                            metadata.iterations,
                            metadata.residual_norm,
                        )
                        .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
                        let state = HbStateArtifact {
                            operating_point: Arc::new(operating_point),
                            spectral_real,
                            spectral_imaginary,
                        };
                        state.validate()?;
                        ExecutionArtifactPayload::HbState(Arc::new(state))
                    }
                };
                Ok(ExecutionArtifactEnvelope {
                    snapshot_digest: artifact.snapshot_digest,
                    producer_instance_id: artifact.producer_instance_id,
                    producer_source_revision: artifact.producer_source_revision,
                    producer_config_digest: artifact.producer_config_digest,
                    kind: artifact.kind,
                    payload_digest: artifact.payload_digest,
                    payload,
                })
            })
            .collect::<Result<Vec<_>, ExecutionArtifactError>>()?;
        if let Some(unused) = buffers.iter().position(Option::is_some) {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer buffer {unused} is unreferenced"
            )));
        }

        let resolved = Self {
            snapshot_digest: metadata.snapshot_digest,
            bindings: metadata.bindings,
            artifacts,
        };
        resolved.validate_transport_integrity()?;
        Ok(resolved)
    }

    #[cfg(any(target_arch = "wasm32", test))]
    fn validate_transport_integrity(&self) -> Result<(), ExecutionArtifactError> {
        if self.bindings.is_empty() && self.artifacts.is_empty() {
            return if self.snapshot_digest.is_none() {
                Ok(())
            } else {
                Err(ExecutionArtifactError::Transport(
                    "artifact-free dependencies carry a snapshot identity".to_owned(),
                ))
            };
        }
        let snapshot_digest = self.snapshot_digest.ok_or_else(|| {
            ExecutionArtifactError::Transport(
                "dependency transfer has no prepared snapshot identity".to_owned(),
            )
        })?;
        if self.bindings.len() != self.artifacts.len() {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer contains {} bindings and {} artifacts",
                self.bindings.len(),
                self.artifacts.len()
            )));
        }
        for (binding, artifact) in self.bindings.iter().zip(&self.artifacts) {
            artifact.validate_against(snapshot_digest, binding)?;
        }
        Ok(())
    }
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct TransferBufferRef {
    buffer: usize,
    len: usize,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TransientTrajectoryTransferMetadata {
    time: TransferBufferRef,
    waveforms: BTreeMap<String, TransferBufferRef>,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PeriodicWaveformTransferMetadata {
    node_name: String,
    values: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PeriodicStateTransferMetadata {
    config_fundamental_freq: f64,
    config_num_harmonics: usize,
    config_tstab: f64,
    config_max_iterations: usize,
    config_tolerance: f64,
    config_abstol: f64,
    config_auto_period: bool,
    config_oscillator_node: Option<String>,
    config_period_guess: f64,
    config_tstab_periods: usize,
    config_damping_factor: f64,
    config_max_period_change: f64,
    config_integration_method: Option<u8>,
    config_points_per_period: usize,
    config_verbose: bool,
    result_period: f64,
    result_frequency: f64,
    result_iterations: usize,
    result_residual_norm: f64,
    time: TransferBufferRef,
    waveforms: Vec<PeriodicWaveformTransferMetadata>,
    period_detected: bool,
    result_floquet_real: TransferBufferRef,
    result_floquet_imag: TransferBufferRef,
    #[serde(default)]
    floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    #[serde(default)]
    floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    #[serde(default)]
    trivial_floquet_multiplier_index: Option<usize>,
    #[serde(default = "indeterminate_floquet_verdict")]
    floquet_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    #[serde(default)]
    floquet_authenticated: bool,
    analysis_iterations: usize,
    analysis_final_residual: f64,
    analysis_period: f64,
    monodromy: Vec<TransferBufferRef>,
    analysis_floquet_real: TransferBufferRef,
    analysis_floquet_imag: TransferBufferRef,
    is_stable: bool,
    #[serde(default)]
    analysis_is_stable: bool,
    shooting_state: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HbSpectrumTransferMetadata {
    node_name: String,
    real: TransferBufferRef,
    imaginary: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HbStateTransferMetadata {
    config: rspice_core::analysis::HbConfig,
    spectra: Vec<HbSpectrumTransferMetadata>,
    iterations: usize,
    residual_norm: f64,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DcOperatingPointSeedTransferMetadata {
    effective_source_content_digest: ContentDigest,
    temperature_celsius: f64,
    supply_voltage: Option<f64>,
    nominal_supply_voltage: Option<f64>,
    #[serde(default)]
    supply_source_names: Vec<String>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    solution: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ExecutionArtifactPayloadTransferMetadata {
    TransientTrajectory(TransientTrajectoryTransferMetadata),
    PeriodicState(PeriodicStateTransferMetadata),
    HbState(HbStateTransferMetadata),
    DcOperatingPointSeed(DcOperatingPointSeedTransferMetadata),
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExecutionArtifactTransferMetadata {
    snapshot_digest: ContentDigest,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
    kind: ExecutionArtifactKind,
    payload_digest: ContentDigest,
    payload: ExecutionArtifactPayloadTransferMetadata,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResolvedExecutionDependenciesTransferMetadata {
    snapshot_digest: Option<ContentDigest>,
    bindings: Vec<PreparedDependencyBinding>,
    artifacts: Vec<ExecutionArtifactTransferMetadata>,
}

#[cfg(any(target_arch = "wasm32", test))]
fn push_transfer_slice<'a>(buffers: &mut Vec<&'a [f64]>, values: &'a [f64]) -> TransferBufferRef {
    let reference = TransferBufferRef {
        buffer: buffers.len(),
        len: values.len(),
    };
    buffers.push(values);
    reference
}

#[cfg(any(target_arch = "wasm32", test))]
fn take_transfer_buffer(
    buffers: &mut [Option<Vec<f64>>],
    reference: TransferBufferRef,
) -> Result<Vec<f64>, ExecutionArtifactError> {
    if reference.len > PeriodicStateArtifact::MAX_NUMERIC_VALUES {
        return Err(ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} declares {} values, exceeding the {}-value limit",
            reference.buffer,
            reference.len,
            PeriodicStateArtifact::MAX_NUMERIC_VALUES
        )));
    }
    let slot = buffers.get_mut(reference.buffer).ok_or_else(|| {
        ExecutionArtifactError::Transport(format!(
            "dependency transfer references missing buffer {}",
            reference.buffer
        ))
    })?;
    let actual_len = slot.as_ref().map(Vec::len).ok_or_else(|| {
        ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} is referenced more than once",
            reference.buffer
        ))
    })?;
    if actual_len != reference.len {
        return Err(ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} has length {}, expected {}",
            reference.buffer, actual_len, reference.len
        )));
    }
    Ok(slot.take().expect("validated occupied transfer slot"))
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(in crate::simulation) enum ExecutionArtifactError {
    #[error("missing {kind:?} artifact from bound prerequisite {producer}")]
    Missing {
        producer: AnalysisInstanceId,
        kind: ExecutionArtifactKind,
    },
    #[error("dependency artifact belongs to producer {actual}, expected {expected}")]
    ProducerMismatch {
        expected: AnalysisInstanceId,
        actual: AnalysisInstanceId,
    },
    #[error("dependency artifact belongs to stale snapshot {actual}, expected {expected}")]
    StaleSnapshot {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("dependency artifact payload digest is {actual}, expected {expected}")]
    PayloadDigestMismatch {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("invalid dependency artifact payload: {0}")]
    InvalidPayload(String),
    #[error("invalid typed dependency contract: {0}")]
    ContractMismatch(String),
    #[cfg(any(target_arch = "wasm32", test))]
    #[error("invalid dependency artifact transfer: {0}")]
    Transport(String),
}

#[cfg(test)]
mod tests;
