//! A saved waveform's samples, or the window of a kept segment's column that holds them.
//!
//! A stored run's waveforms are written as [`ColumnReference`]s and its event histories as
//! references to signals, so a saved project, recovery or session holds no samples of a stored
//! run and writing it reads none: the segments are kept beside the file
//! ([`super::ProjectSimulationResultsData::stored_segments`]). A file is written inside
//! [`saving_file`], which refuses a waveform that holds its samples in memory instead of writing
//! them: every run is in the store before its file is written. Outside it (a hardcopy worker's
//! snapshot, a command-line artifact) samples in memory are written as they are, and a snapshot
//! for a reader in another memory writes each stored window with the rows it reads
//! ([`super::carrying_rows`]).

use std::cell::{Cell, RefCell};

use rspice_results::store_reader::{ColumnReference, SegmentDigest as Digest, writing_references};
use rspice_results::waveform::SharedWaveformValues;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::carried_windows::{WindowWire, carried_values, carrying};
use crate::numeric::sample_serde;

thread_local! {
    static SAVING_FILE: Cell<bool> = const { Cell::new(false) };
    /// The x axis windows the run being read has met, each read as one waveform: every signal of
    /// a run names the same window of its segment's x column.
    static RUN_AXES: RefCell<Option<Vec<(ColumnReference, SharedWaveformValues)>>> =
        const { RefCell::new(None) };
}

/// What a file write that met samples in memory says.
pub const SAMPLES_IN_MEMORY: &str = "a waveform holds its samples in memory, and a saved file \
     names results store segments only: keep the run in the results store first";

/// Run `write` as a project, recovery or session file is written: every waveform and event
/// history is written as the window of the results store column or signal it reads, and one that
/// holds its samples in memory fails the write. A saved file holds no samples.
pub fn saving_file<T>(write: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            SAVING_FILE.with(|saving| saving.set(self.0));
        }
    }
    let _restore = Restore(SAVING_FILE.with(|saving| saving.replace(true)));
    writing_references(write)
}

fn saving() -> bool {
    SAVING_FILE.with(Cell::get)
}

/// An x axis: its stored window, or, outside [`saving_file`], its values.
pub fn serialize_axis<S: Serializer>(
    values: &SharedWaveformValues,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match ColumnReference::of(values) {
        Some(reference) => serialize_window(reference, values, serializer),
        None if saving() => Err(serde::ser::Error::custom(SAMPLES_IN_MEMORY)),
        None => values.serialize(serializer),
    }
}

/// Samples: their stored window, or, outside [`saving_file`], the samples with an unavailable one
/// written as null.
pub fn serialize_samples<S: Serializer>(
    values: &SharedWaveformValues,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match ColumnReference::of(values) {
        Some(reference) => serialize_window(reference, values, serializer),
        None if saving() => Err(serde::ser::Error::custom(SAMPLES_IN_MEMORY)),
        None => sample_serde::serialize_shared(values, serializer),
    }
}

/// A stored window: the window `reference` names, with the rows it reads when the snapshot is for
/// a reader in another memory.
fn serialize_window<S: Serializer>(
    reference: ColumnReference,
    values: &SharedWaveformValues,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if carrying() {
        WindowWire::carrying(reference, values)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    } else {
        reference.serialize(serializer)
    }
}

/// A run's analyses, every waveform among them that names an x axis window it has met reading
/// that window's one waveform: after a reopen the signals of a run share one axis, as they do
/// on the run that wrote them, and a reader of the whole axis holds one copy of it.
pub fn deserialize_run_analyses<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Vec<T>, D::Error> {
    struct Restore(Option<Vec<(ColumnReference, SharedWaveformValues)>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            let outer = self.0.take();
            RUN_AXES.with(|axes| *axes.borrow_mut() = outer);
        }
    }
    let _restore = Restore(RUN_AXES.with(|axes| axes.borrow_mut().replace(Vec::new())));
    Vec::<T>::deserialize(deserializer)
}

/// The waveform of the window `reference` names, the one the run being read already has for an
/// x axis window it met before; read from `held` when the snapshot carried the window's rows.
fn referenced_values(
    reference: ColumnReference,
    held: Option<Vec<f64>>,
) -> Result<SharedWaveformValues, String> {
    let axis = reference.column == 0;
    let met = || {
        RUN_AXES.with(|axes| {
            axes.borrow().as_ref().and_then(|axes| {
                axes.iter()
                    .find(|(met, _)| *met == reference)
                    .map(|(_, values)| values.clone())
            })
        })
    };
    if axis && let Some(values) = met() {
        return Ok(values);
    }
    let values = match held {
        Some(held) => carried_values(&reference, held)?,
        None => super::unread_segments::values(&reference)?,
    };
    if axis {
        RUN_AXES.with(|axes| {
            if let Some(axes) = axes.borrow_mut().as_mut() {
                axes.push((reference, values.clone()));
            }
        });
    }
    Ok(values)
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

/// An analysis's payload, its stored event histories written as references. A snapshot for a
/// reader in another memory writes their changes instead, as it carries a stored window's rows:
/// that reader holds none of the segments a reference names.
pub fn serialize_payload<T: Serialize, S: Serializer>(
    payload: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if carrying() {
        return payload.serialize(serializer);
    }
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
        let (reference, held) =
            WindowWire::deserialize(serde::de::value::MapAccessDeserializer::new(map))?
                .into_parts();
        // Rows read in place of their segment are not checked against its digest: only a
        // snapshot read as one carrying them may hold them, never a saved file.
        if held.is_some() && !carrying() {
            return Err(serde::de::Error::custom(
                "a saved result names results store windows and carries none of their rows",
            ));
        }
        referenced_values(reference, held).map_err(serde::de::Error::custom)
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

/// Whether the samples of a source with `identity` are in a kept segment still waiting for its
/// first check: not read yet, rather than unreadable.
fn being_checked(identity: Option<([u8; 32], u64)>) -> bool {
    identity.is_some_and(|(digest, _)| rspice_results::store_reader::checking(digest))
}

/// Why the first waveform or event history of `analyses` that cannot be read cannot be. One
/// still being checked is not unreadable.
fn unread_reason(analyses: &[super::ProjectAnalysisResult]) -> Option<String> {
    use rspice_results::analysis_payload::AnalysisResultPayload;
    for analysis in analyses {
        for waveform in &analysis.waveforms {
            let complex = waveform.complex.iter().flat_map(|c| [&c.real, &c.imag]);
            for values in [&waveform.x, &waveform.y].into_iter().chain(complex) {
                if let Some((source, _)) = values.stored_window()
                    && !being_checked(source.identity())
                    && let Err(reason) = source.read(0, &mut [])
                {
                    return Some(reason);
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
                if !being_checked(source.identity())
                    && let Err(reason) = source.before(f64::NEG_INFINITY)
                {
                    return Some(reason);
                }
            }
        }
    }
    None
}

impl super::ProjectSimulationRun {
    /// Each segment this run reads, once.
    pub fn stored_segments(&self) -> Vec<StoredSegment> {
        segments_of(self.analyses.iter())
    }
}

impl super::ProjectSimulationResultsData {
    /// Each segment these results read, once, in the order their runs read them.
    pub fn stored_segments(&self) -> Vec<StoredSegment> {
        segments_of(self.runs.iter().flat_map(|run| &run.analyses))
    }

    /// The runs whose results cannot be read, each with why: a kept segment gone from every
    /// folder, or one a folder holds damaged. Every other run reads its samples.
    pub fn runs_without_results(&self) -> Vec<(String, String)> {
        self.runs
            .iter()
            .filter_map(|run| {
                unread_reason(&run.analyses).map(|reason| (run.label.clone(), reason))
            })
            .collect()
    }
}
