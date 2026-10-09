//! Portable accepted transient state shared by workers and retained projects.
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rspice_app_types::product::ContentDigest;
use rspice_core::engine::TransientCheckpoint;
use rspice_core::{AbortSignal, NoAbort, ResourceKind, ResourceLimitError, ResourceLimits};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use std::sync::Arc;

mod library;
pub use library::{ImportedTransientCheckpoint, TransientCheckpointLibrary};

#[derive(Debug, thiserror::Error)]
pub enum TransientCheckpointError {
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Engine(#[from] rspice_core::SimulationError),
}

pub fn checkpoint_digest(bytes: &[u8]) -> ContentDigest {
    rspice_app_types::canonical::content_digest("rspice.transient-checkpoint/v1", bytes)
}

pub fn validate_bytes_size(
    length: usize,
    limits: ResourceLimits,
) -> Result<(), TransientCheckpointError> {
    if length == 0 {
        return Err(TransientCheckpointError::Invalid(
            "Transient checkpoint is empty".into(),
        ));
    }
    check_limit(
        ResourceKind::ExternalDataBytes,
        length,
        limits.max_external_data_bytes,
    )
}

fn check_limit(
    resource: ResourceKind,
    requested: usize,
    limit: usize,
) -> Result<(), TransientCheckpointError> {
    if requested > limit {
        return Err(rspice_core::SimulationError::from(ResourceLimitError {
            resource,
            requested,
            limit,
        })
        .into());
    }
    Ok(())
}

/// Independently bound encoded bytes, canonical text, parsed backing storage and
/// retained scalar values before accepting a portable continuation image.
pub fn decode_bytes(
    bytes: &[u8],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<TransientCheckpoint, TransientCheckpointError> {
    if abort.is_aborted() {
        return Err(rspice_core::SimulationError::Aborted.into());
    }
    validate_bytes_size(bytes.len(), limits)?;
    let checkpoint = TransientCheckpoint::from_bytes_with_limit_and_abort(
        bytes,
        limits.max_external_data_bytes,
        abort,
    )?;
    checkpoint
        .capability()
        .require_resumable()
        .map_err(TransientCheckpointError::Invalid)?;
    check_limit(
        ResourceKind::ResultValues,
        checkpoint.retained_value_count(),
        limits.max_result_values,
    )?;
    Ok(checkpoint)
}

/// Validated immutable bytes. Clones share their storage; cached inspection
/// values are derived from the image and are never trusted from serialized data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientCheckpointEvidence {
    bytes: Arc<[u8]>,
    digest: ContentDigest,
    accepted_time_bits: u64,
}

impl TransientCheckpointEvidence {
    pub fn byte_limit() -> usize {
        ResourceLimits::default().max_external_data_bytes
    }

    pub fn from_bytes(bytes: Arc<[u8]>) -> Result<Self, TransientCheckpointError> {
        Self::from_bytes_with_limits(bytes, ResourceLimits::default(), &NoAbort)
    }

    pub fn from_bytes_with_limits(
        bytes: Arc<[u8]>,
        mut limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, TransientCheckpointError> {
        // A retained image must also be readable by the portable project
        // decoder, even when this execution permits larger private buffers.
        let portable = ResourceLimits::default();
        limits.max_external_data_bytes = limits
            .max_external_data_bytes
            .min(portable.max_external_data_bytes);
        limits.max_result_values = limits.max_result_values.min(portable.max_result_values);
        let checkpoint = decode_bytes(&bytes, limits, abort)?;
        Ok(Self {
            digest: checkpoint_digest(&bytes),
            accepted_time_bits: checkpoint.time.to_bits(),
            bytes,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn digest(&self) -> ContentDigest {
        self.digest
    }
    pub fn accepted_time(&self) -> f64 {
        f64::from_bits(self.accepted_time_bits)
    }

    pub fn validate_for<W>(
        &self,
        analysis: &crate::analysis_result::AnalysisResult<W>,
    ) -> Result<(), String> {
        if analysis.analysis_type != crate::analysis_type::AnalysisType::Transient
            || analysis.provenance.is_none()
            || analysis.import_source.is_some()
        {
            return Err(
                "Transient checkpoints require native prepared transient provenance".into(),
            );
        }
        // Source/configuration compatibility is checked against the fully
        // materialized receiving circuit before any continuation state installs.
        Ok(())
    }
}

impl Serialize for TransientCheckpointEvidence {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        struct Encoded<'a>(&'a [u8]);
        impl Serialize for Encoded<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(&base64::display::Base64Display::new(self.0, &STANDARD))
            }
        }
        let mut record = serializer.serialize_struct("TransientCheckpointEvidence", 2)?;
        record.serialize_field("digest", &self.digest)?;
        record.serialize_field("data", &Encoded(&self.bytes))?;
        record.end()
    }
}

impl<'de> Deserialize<'de> for TransientCheckpointEvidence {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Record {
            digest: ContentDigest,
            #[serde(deserialize_with = "decode_bounded_bytes")]
            data: Arc<[u8]>,
        }
        let record = Record::deserialize(deserializer)?;
        let evidence = Self::from_bytes(record.data).map_err(serde::de::Error::custom)?;
        if evidence.digest != record.digest {
            return Err(serde::de::Error::custom(
                "Transient checkpoint content identity does not match its bytes",
            ));
        }
        Ok(evidence)
    }
}

fn decode_bounded_bytes<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Arc<[u8]>, D::Error> {
    struct BytesVisitor;
    impl serde::de::Visitor<'_> for BytesVisitor {
        type Value = Arc<[u8]>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a bounded Base64 transient checkpoint")
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
            let limit = TransientCheckpointEvidence::byte_limit();
            if value.is_empty() || value.len() > limit.div_ceil(3).saturating_mul(4) {
                return Err(E::custom("Transient checkpoint exceeds its byte limit"));
            }
            let bytes = STANDARD.decode(value).map_err(E::custom)?;
            if bytes.len() > limit {
                return Err(E::custom("Transient checkpoint exceeds its byte limit"));
            }
            Ok(bytes.into())
        }
    }
    deserializer.deserialize_str(BytesVisitor)
}
