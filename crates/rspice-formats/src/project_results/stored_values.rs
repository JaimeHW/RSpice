//! A saved waveform's samples, or the window of a kept segment's column that holds them.
//!
//! A stored run's waveforms are written as [`ColumnReference`]s and its event histories as
//! references to signals, so a saved project, recovery or session holds no samples of a stored
//! run and writing it reads none: the segments are kept beside the file
//! ([`super::ProjectSimulationResults::stored_segments`]). Reading accepts either form.

use rspice_results::store_reader::{ColumnReference, SegmentDigest as Digest, writing_references};
use rspice_results::waveform::SharedWaveformValues;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::numeric::sample_serde;

/// An x axis: its values, which are never unavailable, or its stored window.
pub fn serialize_axis<S: Serializer>(
    values: &SharedWaveformValues,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match ColumnReference::of(values) {
        Some(reference) => reference.serialize(serializer),
        None => values.serialize(serializer),
    }
}

/// Samples, an unavailable one written as null, or their stored window.
pub fn serialize_samples<S: Serializer>(
    values: &SharedWaveformValues,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match ColumnReference::of(values) {
        Some(reference) => reference.serialize(serializer),
        None => sample_serde::serialize_shared(values, serializer),
    }
}

pub fn deserialize_axis<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<SharedWaveformValues, D::Error> {
    deserializer.deserialize_any(Values { nullable: false })
}

pub fn deserialize_samples<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<SharedWaveformValues, D::Error> {
    deserializer.deserialize_any(Values { nullable: true })
}

/// An analysis's payload, its stored event histories written as references.
pub fn serialize_payload<T: Serialize, S: Serializer>(
    payload: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    writing_references(|| payload.serialize(serializer))
}

struct Values {
    nullable: bool,
}

impl<'de> Visitor<'de> for Values {
    type Value = SharedWaveformValues;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("waveform samples, or the window of a results store's column")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        let seq = serde::de::value::SeqAccessDeserializer::new(seq);
        if self.nullable {
            sample_serde::deserialize(seq).map(SharedWaveformValues::from)
        } else {
            Vec::<f64>::deserialize(seq).map(SharedWaveformValues::from)
        }
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        ColumnReference::deserialize(serde::de::value::MapAccessDeserializer::new(map))?
            .values()
            .map_err(serde::de::Error::custom)
    }
}

/// The folder that keeps the segments the results saved in `file` read: `results/<file stem>`
/// beside it, so two projects in one folder never share, and never delete, each other's.
pub fn segment_folder(file: &std::path::Path) -> std::path::PathBuf {
    let stem = file
        .file_stem()
        .map_or_else(|| "project".into(), |stem| stem.to_os_string());
    file.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("results")
        .join(stem)
}

/// A segment saved results read: its logical digest, and the file it is read from now when that
/// file is still there to keep a copy of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredSegment {
    pub digest: Digest,
    pub source: Option<std::path::PathBuf>,
}

/// Each segment `analyses` read, once, in the order they read them.
fn segments_of<'a>(
    analyses: impl Iterator<Item = &'a super::ProjectAnalysisResult>,
) -> Vec<StoredSegment> {
    use rspice_results::analysis_payload::AnalysisResultPayload;
    let mut segments: Vec<StoredSegment> = Vec::new();
    let mut add = |identity: Option<([u8; 32], u64)>, store: String| {
        let Some((digest, _)) = identity else {
            return;
        };
        let digest = Digest(digest);
        if segments.iter().any(|segment| segment.digest == digest) {
            return;
        }
        let source = std::path::PathBuf::from(store);
        segments.push(StoredSegment {
            digest,
            source: source.is_file().then_some(source),
        });
    };
    for analysis in analyses {
        for waveform in &analysis.waveforms {
            let complex = waveform.complex.iter().flat_map(|c| [&c.real, &c.imag]);
            for values in [&waveform.x, &waveform.y].into_iter().chain(complex) {
                if let Some((source, _)) = values.stored_window() {
                    add(source.identity(), source.store());
                }
            }
        }
        if let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            ..
        }) = analysis.result_payload.as_ref()
        {
            let digital = digital_traces
                .iter()
                .map(|trace| trace.points.stored_source());
            let real = real_traces.iter().map(|trace| trace.points.stored_source());
            for source in digital.chain(real).flatten() {
                add(source.identity(), source.store());
            }
        }
    }
    segments
}

impl super::ProjectSimulationResultsData {
    /// Each segment these results read, once, in the order their runs read them.
    pub fn stored_segments(&self) -> Vec<StoredSegment> {
        segments_of(self.runs.iter().flat_map(|run| &run.analyses))
    }

    /// The labels of the runs that read a segment no store folder holds: results deleted or
    /// moved outside RSpice.
    pub fn runs_missing_results(&self) -> Vec<String> {
        self.runs
            .iter()
            .filter(|run| {
                segments_of(run.analyses.iter())
                    .iter()
                    .any(|segment| segment.source.is_none())
            })
            .map(|run| run.label.clone())
            .collect()
    }
}
