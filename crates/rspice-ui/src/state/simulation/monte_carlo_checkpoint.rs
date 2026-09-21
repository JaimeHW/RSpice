//! Durable trial journals, independent of a completed statistical result.

use super::{AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType};
use crate::product::ContentDigest;
use crate::simulation::runner::monte_carlo_checkpoint::checkpoint_digest;
use crate::simulation::runner::study::monte_carlo::checkpoint::StudyMonteCarloCheckpoint;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rspice_core::{NoAbort, ResourceLimits};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use std::sync::Arc;

/// Exact portable bytes shared by retained snapshots. Construction validates
/// both the numerical rows and their measurement verdicts; callers cannot
/// mutate either the bytes or the cached identities after validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonteCarloCheckpointEvidence {
    bytes: Arc<[u8]>,
    digest: ContentDigest,
    population: [u8; 32],
    completed_trials: usize,
}

impl MonteCarloCheckpointEvidence {
    pub(crate) fn from_bytes(bytes: Arc<[u8]>) -> Result<Self, String> {
        let checkpoint = StudyMonteCarloCheckpoint::from_bytes_with_limits(
            &bytes,
            ResourceLimits::default(),
            &NoAbort,
        )
        .map_err(|error| error.to_string())?;
        if checkpoint.completed_trials() == 0 {
            return Err("a retained Monte Carlo checkpoint must contain committed trials".into());
        }
        Ok(Self {
            digest: checkpoint_digest(&bytes),
            population: checkpoint.population_identity(),
            completed_trials: checkpoint.completed_trials(),
            bytes,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub fn population_identity(&self) -> [u8; 32] {
        self.population
    }

    pub fn completed_trials(&self) -> usize {
        self.completed_trials
    }

    pub(crate) fn validate_for(&self, analysis: &AnalysisResult) -> Result<(), String> {
        if analysis.analysis_type != AnalysisType::MonteCarlo
            || analysis.provenance.is_none()
            || analysis.import_source.is_some()
        {
            return Err(
                "Monte Carlo checkpoints require native prepared Monte Carlo provenance".into(),
            );
        }
        if analysis.success {
            let Some(AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements,
                ..
            }) = &analysis.family_metadata
            else {
                return Err(
                    "a completed checkpointed analysis requires its Monte Carlo trial roster"
                        .into(),
                );
            };
            StudyMonteCarloCheckpoint::from_bytes_with_limits(
                &self.bytes,
                ResourceLimits::default(),
                &NoAbort,
            )
            .map_err(|error| error.to_string())?
            .validate_retained_members(member_measurements)?;
        }
        Ok(())
    }
}

impl Serialize for MonteCarloCheckpointEvidence {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        struct Encoded<'a>(&'a [u8]);
        impl Serialize for Encoded<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(&base64::display::Base64Display::new(self.0, &STANDARD))
            }
        }
        let mut record = serializer.serialize_struct("MonteCarloCheckpointEvidence", 2)?;
        record.serialize_field("digest", &self.digest)?;
        record.serialize_field("data", &Encoded(&self.bytes))?;
        record.end()
    }
}

impl<'de> Deserialize<'de> for MonteCarloCheckpointEvidence {
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
        if record.digest != evidence.digest {
            return Err(serde::de::Error::custom(
                "Monte Carlo checkpoint content identity does not match its bytes",
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
            formatter.write_str("a bounded Base64 Monte Carlo checkpoint")
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
            let limit = ResourceLimits::default().max_external_data_bytes;
            // Check encoded size before allocating the decoded binary. The
            // project loader separately bounds the entire input document.
            if value.is_empty() || value.len() > limit.div_ceil(3).saturating_mul(4) {
                return Err(E::custom("Monte Carlo checkpoint exceeds its byte limit"));
            }
            let bytes = STANDARD.decode(value).map_err(E::custom)?;
            if bytes.len() > limit {
                return Err(E::custom("Monte Carlo checkpoint exceeds its byte limit"));
            }
            Ok(bytes.into())
        }
    }
    deserializer.deserialize_str(BytesVisitor)
}
