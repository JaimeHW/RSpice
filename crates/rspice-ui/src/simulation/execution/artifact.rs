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
    FourierTransientRequirement, TransientCapability, validate_fourier_transient_contract,
};
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::SimulationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::simulation) enum ExecutionArtifactKind {
    TransientTrajectory,
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

    pub(super) fn encode(&self, writer: &mut CanonicalWriter) {
        writer.u8(match self.kind {
            ExecutionArtifactKind::TransientTrajectory => 0,
        });
        writer.uuid(self.producer_instance_id.as_uuid());
        writer.u64(self.producer_source_revision.get());
        writer.digest(self.producer_config_digest);
    }
}

pub(in crate::simulation) fn validate_prepared_dependency_contract(
    consumer: &AnalysisSpec,
    producer: &AnalysisSpec,
) -> Result<(), ExecutionArtifactError> {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct ExecutionArtifactEnvelope {
    snapshot_digest: ContentDigest,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
    kind: ExecutionArtifactKind,
    payload_digest: ContentDigest,
    trajectory: Arc<TransientTrajectoryArtifact>,
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
            trajectory: Arc::new(trajectory),
        }))
    }

    pub(in crate::simulation) fn trajectory(&self) -> &TransientTrajectoryArtifact {
        &self.trajectory
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
        self.trajectory.validate()?;
        let actual_digest = self.trajectory.digest();
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
        let expected_count = usize::from(matches!(spec, AnalysisSpec::Fourier { .. }));
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
        if binding.kind != ExecutionArtifactKind::TransientTrajectory {
            return Err(ExecutionArtifactError::ContractMismatch(
                "Fourier requires a transient-trajectory artifact".to_owned(),
            ));
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
        Ok(self.artifacts[0].trajectory())
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
                let time = push_transfer_slice(&mut buffers, &artifact.trajectory.time);
                let waveforms = artifact
                    .trajectory
                    .waveforms
                    .iter()
                    .map(|(name, values)| (name.clone(), push_transfer_slice(&mut buffers, values)))
                    .collect();
                ExecutionArtifactTransferMetadata {
                    snapshot_digest: artifact.snapshot_digest,
                    producer_instance_id: artifact.producer_instance_id,
                    producer_source_revision: artifact.producer_source_revision,
                    producer_config_digest: artifact.producer_config_digest,
                    kind: artifact.kind,
                    payload_digest: artifact.payload_digest,
                    trajectory: TransientTrajectoryTransferMetadata { time, waveforms },
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
                let time = take_transfer_buffer(&mut buffers, artifact.trajectory.time)?;
                let waveforms = artifact
                    .trajectory
                    .waveforms
                    .into_iter()
                    .map(|(name, reference)| {
                        take_transfer_buffer(&mut buffers, reference).map(|values| (name, values))
                    })
                    .collect::<Result<_, _>>()?;
                let trajectory = TransientTrajectoryArtifact { time, waveforms };
                trajectory.validate()?;
                Ok(ExecutionArtifactEnvelope {
                    snapshot_digest: artifact.snapshot_digest,
                    producer_instance_id: artifact.producer_instance_id,
                    producer_source_revision: artifact.producer_source_revision,
                    producer_config_digest: artifact.producer_config_digest,
                    kind: artifact.kind,
                    payload_digest: artifact.payload_digest,
                    trajectory: Arc::new(trajectory),
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
struct ExecutionArtifactTransferMetadata {
    snapshot_digest: ContentDigest,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
    kind: ExecutionArtifactKind,
    payload_digest: ContentDigest,
    trajectory: TransientTrajectoryTransferMetadata,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    let slot = buffers.get_mut(reference.buffer).ok_or_else(|| {
        ExecutionArtifactError::Transport(format!(
            "dependency transfer references missing buffer {}",
            reference.buffer
        ))
    })?;
    let values = slot.take().ok_or_else(|| {
        ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} is referenced more than once",
            reference.buffer
        ))
    })?;
    if values.len() != reference.len {
        return Err(ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} has length {}, expected {}",
            reference.buffer,
            values.len(),
            reference.len
        )));
    }
    Ok(values)
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
mod tests {
    use super::*;
    use crate::simulation::results::WaveformData;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn transient() -> SimulationResult {
        let time = vec![0.0, 0.5, 1.0];
        SimulationResult::Transient {
            time: time.clone(),
            waveforms: HashMap::from([
                (
                    "V(out)".to_owned(),
                    WaveformData::new_time_domain("V(out)", time.clone(), vec![0.0, 1.0, 0.0]),
                ),
                (
                    "V(unused)".to_owned(),
                    WaveformData::new_time_domain("V(unused)", time, vec![4.0, 5.0, 6.0]),
                ),
            ]),
            measurements: Vec::new(),
        }
    }

    #[test]
    fn exact_binding_resolves_and_tampered_payload_fails_closed() {
        let producer = AnalysisInstanceId::new();
        let revision = ObjectRevision::new(3).unwrap();
        let binding =
            PreparedDependencyBinding::transient_trajectory(producer, revision, digest(2));
        let mut artifact = ExecutionArtifactEnvelope::from_transient_result(
            digest(1),
            producer,
            revision,
            digest(2),
            &transient(),
            &["out".to_owned()],
        )
        .unwrap()
        .unwrap();
        assert_eq!(artifact.trajectory.waveforms.len(), 1);
        assert!(artifact.trajectory.waveform("unused").is_none());
        let encoded = serde_json::to_string(&artifact).expect("artifact serializes");
        assert!(
            encoded.contains("\"3ff0000000000000\""),
            "exact floating-point bits must use a JavaScript-safe string encoding"
        );
        let decoded: ExecutionArtifactEnvelope =
            serde_json::from_str(&encoded).expect("artifact deserializes exactly");
        assert_eq!(decoded, artifact);
        let artifacts = HashMap::from([(producer, artifact.clone())]);
        ResolvedExecutionDependencies::resolve(digest(1), vec![binding.clone()], &artifacts)
            .expect("exact artifact resolves");

        Arc::make_mut(&mut artifact.trajectory)
            .waveforms
            .get_mut("V(out)")
            .unwrap()[1] = 2.0;
        let artifacts = HashMap::from([(producer, artifact)]);
        assert!(matches!(
            ResolvedExecutionDependencies::resolve(digest(1), vec![binding], &artifacts),
            Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
        ));
    }

    #[test]
    fn wrong_or_stale_producer_artifacts_are_rejected() {
        let producer = AnalysisInstanceId::new();
        let revision = ObjectRevision::new(5).unwrap();
        let binding =
            PreparedDependencyBinding::transient_trajectory(producer, revision, digest(3));
        let stale = ExecutionArtifactEnvelope::from_transient_result(
            digest(9),
            producer,
            revision,
            digest(3),
            &transient(),
            &["out".to_owned()],
        )
        .unwrap()
        .unwrap();
        let artifacts = HashMap::from([(producer, stale)]);
        assert!(matches!(
            ResolvedExecutionDependencies::resolve(digest(1), vec![binding], &artifacts),
            Err(ExecutionArtifactError::StaleSnapshot { .. })
        ));
    }

    #[test]
    fn fourier_contract_rejects_out_of_window_or_undersampled_transients() {
        let fourier = AnalysisSpec::Fourier {
            fundamental_freq: 2.0,
            num_harmonics: 4,
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        };
        let transient = |stop_time, step_time| AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        };

        validate_prepared_dependency_contract(&fourier, &transient(1.0, 0.005))
            .expect("compatible transient contract");
        let outside = validate_prepared_dependency_contract(&fourier, &transient(0.75, 0.005))
            .expect_err("producer must cover the complete Fourier window");
        assert!(outside.to_string().contains("outside"));
        let undersampled = validate_prepared_dependency_contract(&fourier, &transient(1.0, 0.05))
            .expect_err("producer sampling must cover the harmonic basis");
        assert!(undersampled.to_string().contains("too coarse"));
    }

    #[test]
    fn large_artifact_transfer_uses_constant_size_metadata_and_exact_buffers() {
        const SAMPLE_COUNT: usize = 65_536;

        let producer = AnalysisInstanceId::new();
        let revision = ObjectRevision::new(8).unwrap();
        let snapshot = digest(11);
        let config = digest(12);
        let binding = PreparedDependencyBinding::transient_trajectory(producer, revision, config);
        let time = (0..SAMPLE_COUNT)
            .map(|index| index as f64 * 1.0e-9)
            .collect::<Vec<_>>();
        let expected_time_sample = time[12_345].to_bits();
        let values = time
            .iter()
            .map(|time| (2.0 * std::f64::consts::PI * 1.0e6 * time).sin())
            .collect::<Vec<_>>();
        let result = SimulationResult::Transient {
            time: time.clone(),
            waveforms: HashMap::from([(
                "V(out)".to_owned(),
                WaveformData::new_time_domain("V(out)", time, values),
            )]),
            measurements: Vec::new(),
        };
        let artifact = ExecutionArtifactEnvelope::from_transient_result(
            snapshot,
            producer,
            revision,
            config,
            &result,
            &["out".to_owned()],
        )
        .unwrap()
        .unwrap();
        let resolved = ResolvedExecutionDependencies::resolve(
            snapshot,
            vec![binding],
            &HashMap::from([(producer, artifact)]),
        )
        .unwrap();

        let (borrowed_metadata, borrowed_buffers) = resolved.encode_transfer_borrowed().unwrap();
        let trajectory = resolved.transient_trajectory().unwrap();
        assert_eq!(borrowed_buffers.len(), 2);
        assert_eq!(borrowed_buffers[0].as_ptr(), trajectory.time().as_ptr());
        assert_eq!(
            borrowed_buffers[1].as_ptr(),
            trajectory.waveform("out").unwrap().as_ptr()
        );

        let (metadata, buffers) = resolved.encode_transfer().unwrap();
        assert_eq!(metadata, borrowed_metadata);
        assert!(
            metadata.len() < 4_096,
            "sample-independent transfer metadata unexpectedly grew to {} bytes",
            metadata.len()
        );
        assert_eq!(buffers.len(), 2);
        assert_eq!(buffers[0].len(), SAMPLE_COUNT);
        assert_eq!(buffers[1].len(), SAMPLE_COUNT);
        assert_eq!(buffers[0][12_345].to_bits(), expected_time_sample);

        let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers.clone())
            .expect("transfer buffers reconstruct exactly");
        assert_eq!(restored, resolved);

        let mut tampered = buffers;
        tampered[1][32_768] = -tampered[1][32_768];
        assert!(matches!(
            ResolvedExecutionDependencies::decode_transfer(&metadata, tampered),
            Err(ExecutionArtifactError::PayloadDigestMismatch { .. })
        ));
    }
}
