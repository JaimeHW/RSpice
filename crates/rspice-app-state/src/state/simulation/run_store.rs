//! The store adapter: a finished analysis keeps its samples in a results store segment, and its
//! waveforms read windows of it.
//!
//! Every analysis reaches the history with its waveforms in memory, except a desktop transient
//! on its default outputs, which writes its own segment as it solves. Before an analysis is kept,
//! each waveform holding a series in memory is written to a segment of this process's store, one
//! segment per distinct x axis ([`rspice_results::stored_analysis`]), and its series become
//! windows of the segment's columns. A saved project or session then names the segment by its
//! digest and holds none of the samples.
//!
//! The browser build has no store folder: its segments live in the page's memory, read the same
//! way, and each is kept in the open project's store in the browser's own storage, from which a
//! reload loads it again ([`browser`]).

use std::sync::Arc;

use crate::product::ContentDigest;
use rspice_results::analysis_result::AnalysisResult as RetainedAnalysisResult;
use rspice_results::result_digest::ResultDigestEncoding;
use rspice_results::store_reader::{Segment, SegmentBytes, StoreReader};
/// How the filler [`keep_segment`] is handed pushes the segment's rows.
pub use rspice_results::stored_analysis::{FillSegment, PushRows};
use rspice_results::waveform::RetainedWaveform;
/// The store's words for a segment's shape, so a caller of [`keep_segment`] names its schema
/// without reaching the store itself.
pub use rspice_results_store::{Axis, RunMeta, Schema, Signal, SignalKind, StoreError};
use rspice_results_store::{MemoryIo, RunStatus, SegmentReader, SegmentWriter};

/// Makes a finished segment of a schema from the rows its filler pushes.
type MakeSegment = fn(&Schema, &RunMeta, &mut FillSegment<'_>) -> Result<Arc<Segment>, String>;

/// Why the first waveform or event history of `analyses` that cannot be read cannot be: its
/// segment gone from every folder, or there and damaged. `None` when every one reads, or waits
/// only for its segment's first check ([`checking_progress`]).
pub fn unread_reason<'a, W: AsRef<RetainedWaveform> + 'a>(
    analyses: impl IntoIterator<Item = &'a RetainedAnalysisResult<W>>,
) -> Option<String> {
    use rspice_results::analysis_payload::AnalysisResultPayload;
    for analysis in analyses {
        for waveform in &analysis.waveforms {
            let waveform = waveform.as_ref();
            let complex = waveform
                .complex
                .iter()
                .flat_map(|complex| [&complex.real, &complex.imag]);
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

/// Whether the samples of a source with `identity` are in a kept segment still waiting for its
/// first check.
fn being_checked(identity: Option<([u8; 32], u64)>) -> bool {
    identity.is_some_and(|(digest, _)| rspice_results::store_reader::checking(digest))
}

/// How far the checks of the kept segments `analyses` read and still wait for got, together, as
/// work done of work known: what a run shows while it is being checked. `None` when none waits.
pub fn checking_progress<'a, W: AsRef<RetainedWaveform> + 'a>(
    analyses: impl IntoIterator<Item = &'a RetainedAnalysisResult<W>>,
) -> Option<(u64, u64)> {
    let mut digests = Vec::new();
    for analysis in analyses {
        for waveform in &analysis.waveforms {
            let waveform = waveform.as_ref();
            let complex = waveform
                .complex
                .iter()
                .flat_map(|complex| [&complex.real, &complex.imag]);
            for values in [&waveform.x, &waveform.y].into_iter().chain(complex) {
                if let Some((digest, _)) = values
                    .stored_window()
                    .and_then(|(source, _)| source.identity())
                    && !digests.contains(&digest)
                {
                    digests.push(digest);
                }
            }
        }
    }
    let checks: Vec<_> = digests
        .into_iter()
        .filter_map(rspice_results::store_reader::check_progress)
        .collect();
    (!checks.is_empty()).then(|| {
        checks.iter().fold((0, 0), |(done, total), check| {
            (done + check.0, total + check.1)
        })
    })
}

/// Whether any waveform of `analysis` holds a series in memory, or in a segment that states no
/// digest: one a saved file cannot name.
pub fn holds_samples_in_memory<W: AsRef<RetainedWaveform>>(
    analysis: &RetainedAnalysisResult<W>,
) -> bool {
    analysis
        .waveforms
        .iter()
        .any(|waveform| rspice_results::stored_analysis::holds_memory(waveform.as_ref()))
}

/// Write every series `analysis` holds in memory into results store segments, and make its
/// waveforms read them. An analysis whose waveforms are all stored is left as it is, and one that
/// does not validate is refused. On failure nothing changes: the waveforms keep their samples in
/// memory.
pub fn store_analysis<W>(analysis: &mut RetainedAnalysisResult<W>) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    store_analysis_ending(analysis, RunStatus::Completed)
}

/// [`store_analysis`] for an analysis whose run ended as `ended` says, which each segment's `END`
/// keeps: the transient its results storage stopped reads as stopped from the segment it is kept
/// in again, as from its own.
pub fn store_analysis_ending<W>(
    analysis: &mut RetainedAnalysisResult<W>,
    ended: RunStatus,
) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    rspice_results::stored_analysis::store_analysis(analysis, &mut |schema, meta, fill| {
        process_segment(schema, meta, &ended, fill)
    })
}

/// A finished segment kept as [`store_analysis`] keeps one, its rows pushed by `fill` as they
/// arrive: what an import that keeps its source as it reads it writes, holding no more of it than
/// one push. When `fill` fails nothing is kept.
pub fn keep_segment(
    schema: &Schema,
    meta: &RunMeta,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    process_segment(schema, meta, &RunStatus::Completed, fill)
}

/// `analysis` as a results store keeps it: each series it holds in memory written into a segment
/// held in memory, as a save writes it into the project's store. A digest taken of it is the one
/// the same samples have once they are stored, so a result's digest never depends on where its
/// samples are held. `None` when nothing of it is held in memory, when it is still running (a save
/// keeps none of a preview's samples), or when the store refuses it, which a save does too.
pub fn as_stored<W>(analysis: &RetainedAnalysisResult<W>) -> Option<RetainedAnalysisResult<W>>
where
    W: Clone + AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    if analysis.is_live_partial() || !holds_samples_in_memory(analysis) {
        return None;
    }
    let mut stored = analysis.clone();
    store_analysis_with(&mut stored, memory_segment).ok()?;
    Some(stored)
}

/// The digest of the ordered analyses `run` retained, a function of their samples and never of
/// where they are held: a run saved while it held samples in memory reopens with them in the store
/// and the same digest.
#[must_use]
pub fn dataset_digest<A, W>(run: &rspice_results::run::SimulationRun<A>) -> ContentDigest
where
    A: AsRef<RetainedAnalysisResult<W>>,
    W: Clone + AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    analyses_digest(&run.analyses)
}

/// [`dataset_digest`] of a run's analyses, however they are held: a copy of
/// them made apart from the run, of their waveforms alone, digests as the
/// run does.
#[must_use]
pub fn analyses_digest<A, W>(analyses: &[A]) -> ContentDigest
where
    A: AsRef<RetainedAnalysisResult<W>>,
    W: Clone + AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    rspice_results::result_digest::dataset_digest_of(
        analyses.iter().map(|analysis| {
            let analysis = analysis.as_ref();
            (analysis.id, analysis_digest(analysis))
        }),
        ResultDigestEncoding::CURRENT,
    )
}

/// [`dataset_digest`] of `run`, the digest of its analysis at `known.0`
/// being `known.1`, as [`analysis_digest`] gives it: that analysis is not
/// read again, and the others are digested here.
#[must_use]
pub fn dataset_digest_knowing<A, W>(
    run: &rspice_results::run::SimulationRun<A>,
    known: (usize, ContentDigest),
) -> ContentDigest
where
    A: AsRef<RetainedAnalysisResult<W>>,
    W: Clone + AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    rspice_results::result_digest::dataset_digest_of(
        run.analyses.iter().enumerate().map(|(place, analysis)| {
            let analysis = analysis.as_ref();
            let digest = if place == known.0 {
                known.1
            } else {
                analysis_digest(analysis)
            };
            (analysis.id, digest)
        }),
        ResultDigestEncoding::CURRENT,
    )
}

/// The digest of one analysis that [`dataset_digest`] takes of it: a
/// function of its samples, and never of where they are held.
#[must_use]
pub fn analysis_digest<W>(analysis: &RetainedAnalysisResult<W>) -> ContentDigest
where
    W: Clone + AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    let stored = as_stored(analysis);
    stored
        .as_ref()
        .unwrap_or(analysis)
        .result_data_ref()
        .digest(ResultDigestEncoding::CURRENT)
}

/// [`store_analysis`] into segments held in memory, as the browser build keeps them.
#[cfg(test)]
pub fn store_analysis_in_memory<W>(analysis: &mut RetainedAnalysisResult<W>) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    store_analysis_with(analysis, memory_segment)
}

fn store_analysis_with<W>(
    analysis: &mut RetainedAnalysisResult<W>,
    make: MakeSegment,
) -> Result<(), String>
where
    W: AsRef<RetainedWaveform> + AsMut<RetainedWaveform>,
{
    rspice_results::stored_analysis::store_analysis(analysis, &mut |schema, meta, fill| {
        make(schema, meta, fill)
    })
}

fn kept_failure(error: impl std::fmt::Display) -> String {
    format!("the results store could not keep the samples: {error}")
}

/// A finished segment of this process's store folder, ending as `ended` says.
#[cfg(not(target_arch = "wasm32"))]
fn process_segment(
    schema: &Schema,
    meta: &RunMeta,
    ended: &RunStatus,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    rspice_simulation::results_store::write_finished_segment(
        schema,
        meta,
        ended.clone(),
        |writer| fill(&mut |x, values| writer.push_rows(x, values)),
    )
    .map_err(kept_failure)
}

/// The browser has no store folder: its segments are held in memory, and each is kept in the open
/// project's store in the browser's own storage.
#[cfg(target_arch = "wasm32")]
fn process_segment(
    schema: &Schema,
    meta: &RunMeta,
    ended: &RunStatus,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    let (segment, bytes) = memory_segment_sized(schema, meta, ended, fill)?;
    browser::keep(&segment, bytes);
    Ok(segment)
}

/// A finished segment held in memory.
fn memory_segment(
    schema: &Schema,
    meta: &RunMeta,
    fill: &mut FillSegment<'_>,
) -> Result<Arc<Segment>, String> {
    memory_segment_sized(schema, meta, &RunStatus::Completed, fill).map(|(segment, _)| segment)
}

/// A finished segment held in memory, ending as `ended` says, with its bytes.
fn memory_segment_sized(
    schema: &Schema,
    meta: &RunMeta,
    ended: &RunStatus,
    fill: &mut FillSegment<'_>,
) -> Result<(Arc<Segment>, u64), String> {
    use rspice_results_store::SegmentIo;
    let mut writer = SegmentWriter::create(MemoryIo::new(), schema, meta).map_err(kept_failure)?;
    fill(&mut |x, values| writer.push_rows(x, values)).map_err(kept_failure)?;
    let (_, io) = writer.finish(ended.clone()).map_err(kept_failure)?;
    let bytes = io.size().map_err(kept_failure)?;
    let reader = SegmentReader::open(Box::new(io) as SegmentBytes).map_err(kept_failure)?;
    Ok((
        Segment::kept(StoreReader::new(reader), std::path::PathBuf::new()).shared(),
        bytes,
    ))
}

#[cfg(target_arch = "wasm32")]
pub mod browser;
pub mod named;
#[cfg(test)]
mod tests;
