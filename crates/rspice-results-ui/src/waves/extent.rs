//! Derived geometry of a strip: how far its data reaches, and the envelope a
//! swept family draws around itself.
//!
//! Both answers cost a walk of every visible sample, and both were taken on
//! every frame — the shared X extent several times a frame, because the
//! overview lane, the axis and each pane all asked for it separately. Neither
//! can change without the strip models being rebuilt, so the extent is now
//! resolved once per rebuild — after the reader's visibility overrides have
//! been folded in — and the envelope is memoized against the same generation
//! of models the pane is drawing.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rspice_results::result_presentation::AnalysisPresentationKey;
use rspice_results::waveform::WaveformValues;
use rspice_ui_kit::plot::XScale;

use super::{StripModel, StripTrace, UnitPane, stable_hash};

/// Resolve every strip's X extent, once, against the traces it will draw.
///
/// This runs after the reader's presentation overrides have been folded into
/// `trace.visible`, never before: hiding a trace from the legend, the design
/// navigator or the inspector writes only the override map, so an extent
/// baked out of the raw data flags would keep the span of a trace nobody can
/// see — and would report a domain for a strip whose last trace was hidden.
pub fn resolve_x_ranges(models: &mut [StripModel]) {
    for model in models {
        if model.x_bounds.is_none() {
            model.x_bounds = super::stack::visible_x_bounds(model);
        }
        let data = x_range(model);
        // A running transient's strip spans the run from start to stop while it runs, so the
        // trace grows across a still axis rather than the axis stretching under it.
        model.x_range = match model.live_frame {
            Some((start, stop)) if model.x_scale == XScale::Linear => {
                Some(data.map_or((start, stop), |(x0, x1)| (x0.min(start), x1.max(stop))))
            }
            _ => data,
        };
    }
}

/// What the strips of a running transient keep in view as their runs grow.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiveExtent {
    /// The strips, by analysis, whose X window keeps their run's newest rows in view at the
    /// width it has. The reader chooses following; any zoom or pan of theirs ends it, until they
    /// choose it again, and so does the run's end, leaving the window where it was.
    pub follow: HashSet<AnalysisPresentationKey>,
}

impl LiveExtent {
    /// `view`'s width, ending at the run's newest time.
    pub fn followed(view: (f64, f64), leading: f64) -> (f64, f64) {
        (leading - (view.1 - view.0), leading)
    }
}

/// X range of a strip. Ordinary traces share one X series; family policies
/// intentionally project disjoint exact-row groups, so the range must cover
/// every visible group rather than assuming the first trace is authoritative.
///
/// Resolved once per strip model. The models are rebuilt whenever anything
/// this reads changes — the retained data, which traces are visible, the
/// phase projection — so the answer cannot outlive its inputs.
fn x_range(model: &StripModel) -> Option<(f64, f64)> {
    note_extent(0);
    let mut x0 = f64::INFINITY;
    let mut x1 = f64::NEG_INFINITY;
    // An event lane is drawn over its column's grid, so a strip showing only
    // lanes still has the extent of its run.
    let mut take = |x: f64| {
        if x.is_finite() && !(model.x_scale == XScale::Log10 && x <= 0.0) {
            x0 = x0.min(x);
            x1 = x1.max(x);
        }
    };
    let axes = model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .map(|trace| &trace.x)
        .chain(
            model
                .lane_traces
                .iter()
                .filter(|lane| lane.visible)
                .map(|lane| &lane.x),
        );
    for x in each_axis_once(axes) {
        // A stored run's time axis ascends, so its ends are its extent, read
        // without reading the axis; on a decade scale its least positive
        // sample is found by bisection.
        if x.is_ascending_axis()
            && let Some((first, last)) = x.ends()
        {
            if model.x_scale == XScale::Log10 {
                let positive = x.partition_point(|value| {
                    value.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
                });
                x.sample(positive).into_iter().for_each(&mut take);
            } else {
                take(first);
            }
            take(last);
            continue;
        }
        // Any other axis is held in memory, or read off the frame and taken
        // once it has arrived.
        if let Some(x) = x.fetch_whole().ready() {
            x.iter().copied().for_each(&mut take);
        }
    }
    if !x0.is_finite() || !x1.is_finite() {
        return None;
    }
    if x1 > x0 {
        return Some((x0, x1));
    }
    if model.x_scale == XScale::Log10 {
        Some((x0 / 10.0, x1 * 10.0))
    } else {
        Some((x0 - 1.0, x1 + 1.0))
    }
}

/// Each of `axes` once, in the order they come, reading none of them: the
/// traces of a run share its axis, which a strip reads once rather than once
/// a trace, however many handles its traces hold of it. Most traces share the
/// axis of the trace before them, and are passed over without a lookup.
pub(crate) fn each_axis_once<'a>(
    axes: impl IntoIterator<Item = &'a WaveformValues>,
) -> impl Iterator<Item = &'a WaveformValues> {
    let mut before: Option<&WaveformValues> = None;
    let mut seen = HashSet::new();
    axes.into_iter().filter(move |&x| {
        if before.is_some_and(|before| WaveformValues::ptr_eq(before, x)) {
            return false;
        }
        before = Some(x);
        seen.insert(x.sharing_key())
    })
}

pub use super::pane::FamilyEnvelopeSeries;

/// One pane's family envelopes.
#[derive(Debug)]
pub struct FamilyEnvelopePlan {
    series: Vec<FamilyEnvelopeSeries>,
}

impl FamilyEnvelopePlan {
    pub fn series(&self) -> &[FamilyEnvelopeSeries] {
        &self.series
    }
}

/// Every pane's envelopes for one generation of strip models.
///
/// A single slot was one pane too few. Volts and amps do not share a Y
/// scale, so a strip probing both is a two-pane strip, and on one frame pane
/// 0 built its envelope, pane 1 evicted it, and the sheet bar's own gate —
/// which asks the same question about the active pane, to decide whether to
/// offer the control — evicted that. Three walks of every sample of every
/// family member, every frame, for a picture that had not changed.
///
/// The generation is what makes the map safe to hold: the models are rebuilt
/// whenever anything an envelope reads changes, so a new generation empties
/// the map rather than being folded into keys that would accumulate.
#[derive(Debug, Clone, Default)]
pub struct FamilyEnvelopeCache {
    generation: u64,
    plans: HashMap<u64, Arc<FamilyEnvelopePlan>>,
}

impl FamilyEnvelopeCache {
    /// Whether the cache holds no envelope at all.
    #[cfg(any(test, feature = "test-support"))]
    pub fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// How many envelopes are held, for a test that pins the cache to one
    /// generation of models rather than letting it grow across them.
    #[cfg(any(test, feature = "test-support"))]
    pub fn entry_count(&self) -> usize {
        self.plans.len()
    }
}

/// Everything the envelope was derived from: the generation of strip models,
/// the strip, and exactly which traces the pane carries.
fn envelope_key(models_generation: u64, model: &StripModel, pane: &UnitPane) -> u64 {
    stable_hash(&(
        models_generation,
        model.analysis_key,
        pane.unit,
        &pane.traces,
    ))
}

/// The envelope series for one pane, computed at most once per generation.
///
/// A plan names each family the pane carries and holds its members; the
/// samples are folded per screen column only when the pane draws, against
/// the view it draws in (see [`FamilyEnvelopeSeries::columns`]). The sheet
/// bar asks for the plan too, to decide whether to offer the control that
/// draws it. Held per pane, because every pane of a strip asks on the same
/// frame; see [`FamilyEnvelopeCache`].
pub fn family_envelopes(
    cache: &mut FamilyEnvelopeCache,
    models_generation: u64,
    model: &StripModel,
    pane: &UnitPane,
) -> Arc<FamilyEnvelopePlan> {
    let key = envelope_key(models_generation, model, pane);
    if cache.generation != models_generation {
        cache.generation = models_generation;
        cache.plans.clear();
    }
    if let Some(plan) = cache.plans.get(&key) {
        return Arc::clone(plan);
    }
    let built = Arc::new(FamilyEnvelopePlan {
        series: family_envelope_series(model, pane),
    });
    cache.plans.insert(key, Arc::clone(&built));
    built
}

fn family_envelope_series(model: &StripModel, pane: &UnitPane) -> Vec<FamilyEnvelopeSeries> {
    note_extent(1);
    let mut groups = HashMap::<(String, u8), Vec<(usize, &StripTrace)>>::new();
    for (index, trace) in pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index).map(|trace| (*index, trace)))
        .filter(|(_, trace)| {
            trace.visible && !trace.overlay && trace.family_group_ordinal.is_some()
        })
    {
        groups
            .entry((trace.source_waveform_name.clone(), trace.kind as u8))
            .or_default()
            .push((index, trace));
    }

    let mut envelopes = Vec::new();
    for traces in groups.into_values() {
        let family_groups = traces
            .iter()
            .map(|(_, trace)| trace.presentation_key)
            .collect::<HashSet<_>>();
        if family_groups.len() < 2 {
            continue;
        }
        envelopes.push(FamilyEnvelopeSeries {
            members: traces.iter().map(|(index, _)| *index).collect(),
        });
    }
    envelopes
}

/// Report a walk of a pane's traces for a summary of them, its extremes or
/// its legend's order, which a frame finds again while the strip stands.
#[inline]
pub(crate) fn note_pane_summary() {
    note_extent(2);
}

// Dataset-walk observations used by the owner and application regression gates.
#[inline]
fn note_extent(_index: usize) {
    #[cfg(any(test, feature = "test-support"))]
    EXTENT_WALKS.with(|counts| {
        let mut next = counts.get();
        next[_index] += 1;
        counts.set(next);
    });
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static EXTENT_WALKS: std::cell::Cell<[u64; 3]> = const { std::cell::Cell::new([0; 3]) };
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentWork {
    pub x_ranges: u64,
    pub envelopes: u64,
    /// Walks of a pane's traces for its extremes or its legend's order.
    pub pane_summaries: u64,
}

#[cfg(any(test, feature = "test-support"))]
impl ExtentWork {
    pub fn read() -> Self {
        let [x_ranges, envelopes, pane_summaries] = EXTENT_WALKS.with(std::cell::Cell::get);
        Self {
            x_ranges,
            envelopes,
            pane_summaries,
        }
    }

    pub fn reset() -> Self {
        EXTENT_WALKS.with(|counts| counts.set([0; 3]));
        Self::read()
    }
}

#[cfg(test)]
mod tests;
