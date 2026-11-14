//! The members of a deck's Monte Carlo study or `.STEP` sweep, drawn as one
//! family on one strip.
//!
//! A study or sweep runs as one task per trial or point, and each keeps its
//! own result, so the run holds one analysis per member: a transient per
//! trial, and a transient, DC sweep, AC or noise analysis per point. Stacked
//! as analyses they would be one strip per member; drawn as a family they are
//! one strip, hosted by the first member, on which every member's curve of a
//! signal is a family trace of that signal — one Id–Vds curve per gate
//! voltage, one Bode curve per capacitor — and the strip's family overlay,
//! its envelope and its per-member hide controls all apply. Each curve of a
//! plain value is the member's own retained samples, shared rather than
//! copied; a decibel, phase or noise-density curve is the member's samples
//! through the same projection the host's curve takes.
//!
//! The runs of an imported LTspice `stepped` rawfile are members the same
//! way: each states its run on its import source, and
//! [`AnalysisResult::family_member`] answers for either origin.
//!
use std::collections::HashMap;
use std::sync::Arc;

use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::provenance::AnalysisResultFamilyMember;
use rspice_results::waveform::RetainedTrace;

use super::{FamilyTraceStyle, FamilyTraceVisibilityKey, ProjectionContext, StripTrace, TraceKind};
use super::{SweepShape, displayed_phase_series, shape_key, stable_hash};
use crate::derived::DerivedSeries;

/// How one analysis of the active run takes part in a member family.
pub(super) enum FamilyRole<'a> {
    /// The first member: its strip draws every member, itself included, in
    /// member order, as (analysis index, member) pairs.
    Host(Vec<(usize, &'a AnalysisResultFamilyMember)>),
    /// A later member, drawn on its host's strip.
    Member,
}

/// Whether a member of a `kind` analysis draws on its family's strip: one
/// whose strip plots curves over a swept axis, time, the swept source or
/// frequency.
const fn draws_as_family(kind: AnalysisType) -> bool {
    kind.is_time_domain()
        || matches!(
            kind,
            AnalysisType::DcSweep | AnalysisType::Ac | AnalysisType::Noise
        )
}

/// The member families among `analyses`, by analysis index.
///
/// A family is every retained analysis of one kind that names a member of
/// the same kind of family, trial or point; a run lowers at most one study
/// and one sweep. A member that failed keeps no waveforms and is simply
/// absent, never drawn as a flat line.
pub(super) fn member_families<A: AsRef<AnalysisResult<RetainedTrace>>>(
    analyses: &[A],
) -> HashMap<usize, FamilyRole<'_>> {
    let mut families: Vec<(AnalysisType, Vec<(usize, &AnalysisResultFamilyMember)>)> = Vec::new();
    for (index, analysis) in analyses.iter().map(AsRef::as_ref).enumerate() {
        let Some(member) = analysis
            .family_member()
            .filter(|_| draws_as_family(analysis.analysis_type) && !analysis.waveforms.is_empty())
        else {
            continue;
        };
        match families.iter_mut().find(|(kind, family)| {
            *kind == analysis.analysis_type && family[0].1.same_family_kind(member)
        }) {
            Some((_, family)) => family.push((index, member)),
            None => families.push((analysis.analysis_type, vec![(index, member)])),
        }
    }
    let mut roles = HashMap::new();
    for (_, mut family) in families {
        family.sort_by_key(|(_, member)| member.index());
        for &(index, _) in &family[1..] {
            roles.insert(index, FamilyRole::Member);
        }
        roles.insert(family[0].0, FamilyRole::Host(family));
    }
    roles
}

/// What the family's strip says it holds: the host's label without its own
/// member, then the count, `Transient · 3 trials`.
pub(super) fn family_subtitle(
    host_label: &str,
    members: &[(usize, &AnalysisResultFamilyMember)],
) -> String {
    let Some((_, first)) = members.first() else {
        return host_label.to_owned();
    };
    let noun = match first {
        AnalysisResultFamilyMember::MonteCarloTrial { .. } => "trial",
        AnalysisResultFamilyMember::StepPoint { .. } => "point",
    };
    let base = host_label
        .strip_suffix(&format!(" · {}", first.label()))
        .unwrap_or(host_label);
    let count = members.len();
    format!(
        "{base} · {count} {noun}{}",
        if count == 1 { "" } else { "s" }
    )
}

/// The identity a member's curves share on the strip, the same for every
/// signal of that member.
fn member_key(member: &AnalysisResultFamilyMember) -> u64 {
    let trial = matches!(member, AnalysisResultFamilyMember::MonteCarloTrial { .. });
    stable_hash(&("transient-family-member", trial, member.index()))
}

/// Replace the host's signal traces with one family trace per member per
/// signal, members in order under each signal.
///
/// A curve of a member keeps the host's source identity (its waveform
/// index, name, unit and kind) so the signal's visibility, palette place and
/// selection read as one signal, and takes its samples from the member's own
/// waveform of that name, through the host curve's projection when it has
/// one: decibels, displayed phase or noise density.
pub(super) fn member_traces<A: AsRef<AnalysisResult<RetainedTrace>>>(
    host_traces: Vec<StripTrace>,
    members: &[(usize, &AnalysisResultFamilyMember)],
    analyses: &[A],
    derived: &mut DerivedSeries,
    context: &ProjectionContext<'_>,
) -> Vec<StripTrace> {
    let thin = matches!(
        members.first().map(|(_, member)| member),
        Some(AnalysisResultFamilyMember::MonteCarloTrial { .. })
    );
    let style = FamilyTraceStyle {
        // A trial is one draw of a population: every trial of a signal takes
        // the signal's hue, thinner, so the population reads as a band.
        width_points: thin.then_some(1.0),
        ..FamilyTraceStyle::default()
    };
    let mut traces = Vec::with_capacity(host_traces.len() * members.len());
    for trace in host_traces {
        if trace.family_group_ordinal.is_some() {
            traces.push(trace);
            continue;
        }
        for (ordinal, &(analysis_index, member)) in members.iter().enumerate() {
            let Some(analysis) = analyses.get(analysis_index).map(AsRef::as_ref) else {
                continue;
            };
            let Some(waveform) = analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == trace.source_waveform_name)
            else {
                continue;
            };
            let samples = match trace.kind {
                TraceKind::Real => waveform.complex.as_ref().map(|complex| &complex.real),
                TraceKind::Imaginary => waveform.complex.as_ref().map(|complex| &complex.imag),
                _ => Some(&waveform.y),
            };
            let Some(samples) = samples else {
                continue;
            };
            let presentation_key = member_key(member);
            let derived_key = stable_hash(&(
                context.analysis_key,
                &trace.source_waveform_name,
                trace.kind as u8,
            )) ^ context.selection_key
                ^ presentation_key.rotate_left(23);
            let y = match trace.kind {
                TraceKind::MagnitudeDb => derived.db(derived_key, samples),
                TraceKind::NoiseDensity => derived.noise_density_nv(derived_key, samples),
                TraceKind::PhaseDeg => {
                    displayed_phase_series(derived, derived_key, samples, context.phase_continuous)
                }
                TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => Arc::clone(samples),
            };
            let visibility_key = FamilyTraceVisibilityKey::new(
                context.analysis_key,
                presentation_key,
                &trace.source_waveform_name,
                trace.kind,
            );
            let x = Arc::clone(&waveform.x);
            let shape = derived.shape_or(
                shape_key(
                    context.analysis_key,
                    &trace.source_waveform_name,
                    trace.kind,
                    presentation_key,
                    context.run_id,
                    false,
                ),
                || SweepShape::of(&x),
            );
            traces.push(StripTrace {
                waveform_index: trace.waveform_index,
                source_waveform_name: trace.source_waveform_name.clone(),
                base_name: trace.base_name.clone(),
                name: format!("{} · {}", trace.base_name, member.label()),
                unit: trace.unit.clone(),
                signal_color: trace.signal_color,
                color: trace.signal_color,
                palette_slot: trace.palette_slot,
                x,
                y,
                shape,
                kind: trace.kind,
                visible: trace.visible && !context.hidden_family_traces.contains(&visibility_key),
                run_id: trace.run_id,
                overlay: false,
                presentation_key,
                family_group_ordinal: Some(ordinal),
                family_style: Some(style),
                family_visibility_key: Some(visibility_key),
            });
        }
    }
    traces
}

#[cfg(test)]
mod tests;
