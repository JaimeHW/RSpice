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

use crate::ui::plot::XScale;

use super::super::ResultsState;
use super::super::frame_work::{self, DatasetWalk};
use super::{StripModel, StripTrace, UnitPane, stable_hash};

/// Resolve every strip's X extent, once, against the traces it will draw.
///
/// This runs after the reader's presentation overrides have been folded into
/// `trace.visible`, never before: hiding a trace from the legend, the design
/// navigator or the inspector writes only the override map, so an extent
/// baked out of the raw data flags would keep the span of a trace nobody can
/// see — and would report a domain for a strip whose last trace was hidden.
pub(super) fn resolve_x_ranges(models: &mut [StripModel]) {
    for model in models {
        model.x_range = x_range(model);
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
    frame_work::note(DatasetWalk::WaveXRange);
    let mut x0 = f64::INFINITY;
    let mut x1 = f64::NEG_INFINITY;
    for x in model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .flat_map(|trace| trace.x.iter().copied())
        .filter(|value| value.is_finite())
    {
        if model.x_scale == XScale::Log10 && x <= 0.0 {
            continue;
        }
        x0 = x0.min(x);
        x1 = x1.max(x);
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

/// Whether a strip's sample grid is finite and non-decreasing.
///
/// Resolved with the model because it is the precondition for answering
/// "which retained sample is this cursor on" by bisection instead of by
/// reading every coordinate. A grid that fails it keeps the linear scan;
/// a parametric sweep is under no obligation to be monotonic.
pub(super) fn grid_is_ascending(model: &StripModel) -> bool {
    model.sample_grid().is_some_and(|grid| {
        let mut previous = f64::NEG_INFINITY;
        grid.iter().all(|value| {
            let ordered = value.is_finite() && *value >= previous;
            previous = *value;
            ordered
        })
    })
}

#[derive(Debug)]
pub(super) struct FamilyEnvelopeSeries {
    pub(super) x: Vec<f64>,
    pub(super) minimum: Vec<f64>,
    pub(super) maximum: Vec<f64>,
    pub(super) color: egui::Color32,
    pub(super) minimum_cache_key: u64,
    pub(super) maximum_cache_key: u64,
}

/// One pane's family envelopes.
#[derive(Debug)]
pub(in crate::workbench::documents::result_document) struct FamilyEnvelopePlan {
    series: Vec<FamilyEnvelopeSeries>,
}

impl FamilyEnvelopePlan {
    pub(super) fn series(&self) -> &[FamilyEnvelopeSeries] {
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
pub(in crate::workbench::documents::result_document) struct FamilyEnvelopeCache {
    generation: u64,
    plans: HashMap<u64, Arc<FamilyEnvelopePlan>>,
}

impl FamilyEnvelopeCache {
    /// Whether the cache holds no envelope at all.
    #[cfg(test)]
    pub(in crate::workbench::documents::result_document) fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// How many envelopes are held, for a test that pins the cache to one
    /// generation of models rather than letting it grow across them.
    #[cfg(test)]
    pub(in crate::workbench::documents::result_document) fn entry_count(&self) -> usize {
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
/// The walk is over every sample of every family member, collected into a
/// map keyed by exact X coordinate and then sorted — for a swept family of a
/// million points that is the most expensive thing the wave stack does, and
/// the sheet bar asks for it a second time just to decide whether to offer
/// the control that draws it. Held per pane, because every pane of a strip
/// asks on the same frame; see [`FamilyEnvelopeCache`].
pub(super) fn family_envelopes(
    results: &mut ResultsState,
    models_generation: u64,
    model: &StripModel,
    pane: &UnitPane,
) -> Arc<FamilyEnvelopePlan> {
    let key = envelope_key(models_generation, model, pane);
    let cache = &mut results.plans.envelopes;
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
    frame_work::note(DatasetWalk::WaveEnvelope);
    let mut groups = HashMap::<(String, u8), Vec<&StripTrace>>::new();
    for trace in pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .filter(|trace| trace.visible && !trace.overlay && trace.family_group_ordinal.is_some())
    {
        groups
            .entry((trace.source_waveform_name.clone(), trace.kind as u8))
            .or_default()
            .push(trace);
    }

    let mut envelopes = Vec::new();
    for ((source_name, kind), traces) in groups {
        let family_groups = traces
            .iter()
            .map(|trace| trace.presentation_key)
            .collect::<HashSet<_>>();
        if family_groups.len() < 2 {
            continue;
        }

        let mut points = HashMap::<u64, (f64, f64, f64, usize)>::new();
        for trace in &traces {
            for (&x, &y) in trace.x.iter().zip(trace.y.iter()) {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                points
                    .entry(x.to_bits())
                    .and_modify(|(_, minimum, maximum, count)| {
                        *minimum = minimum.min(y);
                        *maximum = maximum.max(y);
                        *count += 1;
                    })
                    .or_insert((x, y, y, 1));
            }
        }
        let mut points = points
            .into_values()
            .filter(|(_, _, _, count)| *count >= 2)
            .collect::<Vec<_>>();
        points.sort_by(|left, right| left.0.total_cmp(&right.0));
        if points.is_empty() {
            continue;
        }

        let presentation_keys = traces
            .iter()
            .map(|trace| trace.presentation_key)
            .collect::<Vec<_>>();
        let identity = stable_hash(&(model.analysis_key, source_name, kind, presentation_keys));
        envelopes.push(FamilyEnvelopeSeries {
            x: points.iter().map(|point| point.0).collect(),
            minimum: points.iter().map(|point| point.1).collect(),
            maximum: points.iter().map(|point| point.2).collect(),
            color: traces[0].signal_color.gamma_multiply(0.78),
            minimum_cache_key: identity ^ 0x1357_9BDF_2468_ACE0,
            maximum_cache_key: identity ^ 0x0246_8ACE_1357_9BDF,
        });
    }
    envelopes
}
