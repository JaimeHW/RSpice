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
//! [`AnalysisResult::family_member`] answers for either origin. So are the
//! points one authored analysis was solved at across PVT corners: five
//! corners of an AC analysis are five Bode curves on one strip.
//!
//! A swept family's members are told apart the way ADS sequences a swept
//! dataset's traces: the first swept quantity picks a member's colour, the
//! second its dash, the third its marker, and the legend names each member
//! by its point, `V(out) · r=2k`. A Monte Carlo population keeps its
//! signal's hue, thin, so it reads as a band.
//!
use std::collections::HashMap;

use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_projection::FamilyColorStyle;
use rspice_results::provenance::AnalysisResultFamilyMember;
use rspice_results::sweep_table::{point_axes, point_name};
use rspice_results::visualization_document::AccessibleColorPalette;
use rspice_results::waveform::RetainedTrace;

use super::{FamilyTraceStyle, FamilyTraceVisibilityKey, ProjectionContext, StripTrace, TraceKind};
use super::{curve_shape, displayed_phase_series, shape_key, stable_hash};
use crate::derived::DerivedSeries;

/// One member of a family, as its host's strip draws it.
#[derive(Debug, Clone)]
pub(super) struct MemberSlot {
    /// The member's analysis in the run.
    pub analysis: usize,
    /// How the legend names the member: `r=2k`, `trial 3`, `ff, temp=125 °C`.
    pub label: String,
    /// The identity every curve of the member shares on the strip.
    pub key: u64,
    /// How the member's curves are told apart from its siblings'.
    pub style: FamilyTraceStyle,
}

/// What a family's members are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberKind {
    /// Draws of a Monte Carlo population.
    Trial,
    /// Points of a `.STEP` sweep.
    Point,
    /// Points one authored analysis was solved at across PVT corners.
    Corner,
}

/// A family of members drawn on one strip.
#[derive(Debug, Clone)]
pub(super) struct Family {
    kind: MemberKind,
    /// The members in family order, the host first.
    pub members: Vec<MemberSlot>,
}

/// How one analysis of the active run takes part in a member family.
pub(super) enum FamilyRole {
    /// The first member: its strip draws every member, itself included.
    Host(Family),
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
/// the same kind of family, trial or point, or every analysis one authored
/// analysis was solved into at PVT corners; a run lowers at most one study
/// and one sweep. A member that failed keeps no waveforms and is simply
/// absent, never drawn as a flat line.
pub(super) fn member_families<A: AsRef<AnalysisResult<RetainedTrace>>>(
    analyses: &[A],
) -> HashMap<usize, FamilyRole> {
    let mut families: Vec<(AnalysisType, Vec<(usize, &AnalysisResultFamilyMember)>)> = Vec::new();
    let mut corners: Vec<(AnalysisType, u64, Vec<usize>)> = Vec::new();
    for (index, analysis) in analyses.iter().map(AsRef::as_ref).enumerate() {
        if !draws_as_family(analysis.analysis_type) || analysis.waveforms.is_empty() {
            continue;
        }
        let Some(member) = analysis.family_member() else {
            if let Some(provenance) = analysis
                .provenance()
                .filter(|provenance| provenance.pvt_point().is_some())
            {
                let authored = stable_hash(&provenance.authored_source_instance_id());
                match corners
                    .iter_mut()
                    .find(|(kind, id, _)| *kind == analysis.analysis_type && *id == authored)
                {
                    Some((_, _, family)) => family.push(index),
                    None => corners.push((analysis.analysis_type, authored, vec![index])),
                }
            }
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
    let mut host = |family: Family| {
        for slot in &family.members[1..] {
            roles.insert(slot.analysis, FamilyRole::Member);
        }
        roles.insert(family.members[0].analysis, FamilyRole::Host(family));
    };
    for (_, mut family) in families {
        family.sort_by_key(|(_, member)| member.index());
        if matches!(
            family[0].1,
            AnalysisResultFamilyMember::MonteCarloTrial { .. }
        ) {
            // A trial is one draw of a population: every trial of a signal
            // takes the signal's hue, thinner, so the population reads as a
            // band.
            let style = FamilyTraceStyle {
                width_points: Some(1.0),
                ..FamilyTraceStyle::default()
            };
            host(Family {
                kind: MemberKind::Trial,
                members: family
                    .iter()
                    .map(|&(analysis, member)| MemberSlot {
                        analysis,
                        label: member.label(),
                        key: member_key(member),
                        style,
                    })
                    .collect(),
            });
        } else {
            let points: Vec<&AnalysisResult<RetainedTrace>> = family
                .iter()
                .map(|&(analysis, _)| analyses[analysis].as_ref())
                .collect();
            let styles = swept_styles(&points);
            host(Family {
                kind: MemberKind::Point,
                members: family
                    .iter()
                    .zip(styles)
                    .map(|(&(analysis, member), (style, _))| MemberSlot {
                        analysis,
                        label: member.label(),
                        key: member_key(member),
                        style,
                    })
                    .collect(),
            });
        }
    }
    for (_, authored, family) in corners {
        if family.len() < 2 {
            continue;
        }
        let points: Vec<&AnalysisResult<RetainedTrace>> = family
            .iter()
            .map(|&analysis| analyses[analysis].as_ref())
            .collect();
        let styles = swept_styles(&points);
        host(Family {
            kind: MemberKind::Corner,
            members: family
                .iter()
                .zip(styles)
                .enumerate()
                .map(|(row, (&analysis, (style, label)))| MemberSlot {
                    analysis,
                    label: if label.is_empty() {
                        format!("corner {}", row + 1)
                    } else {
                        label
                    },
                    key: stable_hash(&("pvt-family-member", authored, row)),
                    style,
                })
                .collect(),
        });
    }
    roles
}

/// Each point's style and name: its value of the first swept quantity picks
/// its colour, of the second its dash and of the third its marker, and it is
/// named by every quantity that differs between points.
fn swept_styles(points: &[&AnalysisResult<RetainedTrace>]) -> Vec<(FamilyTraceStyle, String)> {
    let axes = point_axes(points);
    (0..points.len())
        .map(|row| {
            let level = |axis: usize| axes.get(axis).and_then(|axis| axis.level_of(row));
            let style = FamilyTraceStyle {
                color: axes
                    .first()
                    .zip(level(0))
                    .map(|(axis, ordinal)| FamilyColorStyle {
                        palette: AccessibleColorPalette::TolBrightCategorical,
                        ordinal,
                        category_count: axis.levels().len(),
                    }),
                dash_ordinal: level(1),
                marker_ordinal: level(2),
                width_points: None,
            };
            (style, point_name(&axes, row))
        })
        .collect()
}

/// What the family's strip says it holds: the host's label without its own
/// member, then the count, `Transient · 3 trials`.
pub(super) fn family_subtitle(host_label: &str, family: &Family) -> String {
    let Some(first) = family.members.first() else {
        return host_label.to_owned();
    };
    let noun = match family.kind {
        MemberKind::Trial => "trial",
        MemberKind::Point => "point",
        MemberKind::Corner => "corner",
    };
    let base = host_label
        .strip_suffix(&format!(" · {}", first.label))
        .unwrap_or(host_label);
    let count = family.members.len();
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
    family: &Family,
    analyses: &[A],
    derived: &mut DerivedSeries,
    context: &ProjectionContext<'_>,
) -> Vec<StripTrace> {
    let members = &family.members;
    let mut traces = Vec::with_capacity(host_traces.len() * members.len());
    for trace in host_traces {
        if trace.family_group_ordinal.is_some() {
            traces.push(trace);
            continue;
        }
        for (ordinal, member) in members.iter().enumerate() {
            let Some(analysis) = analyses.get(member.analysis).map(AsRef::as_ref) else {
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
            let presentation_key = member.key;
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
                TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => samples.clone(),
                // A projection a strip asks for is made after the families.
                TraceKind::GroupDelay => continue,
            };
            let visibility_key = FamilyTraceVisibilityKey::new(
                context.analysis_key,
                presentation_key,
                &trace.source_waveform_name,
                trace.kind,
            );
            let x = waveform.x.clone();
            let shape = curve_shape(
                derived,
                shape_key(
                    context.analysis_key,
                    &trace.source_waveform_name,
                    trace.kind,
                    presentation_key,
                    context.run_id,
                    false,
                ),
                &x,
                true,
            );
            traces.push(StripTrace {
                waveform_index: trace.waveform_index,
                source_waveform_name: trace.source_waveform_name.clone(),
                base_name: trace.base_name.clone(),
                name: format!("{} · {}", trace.base_name, member.label),
                unit: trace.unit.clone(),
                signal_color: trace.signal_color,
                color: super::family_color(member.style, trace.signal_color),
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
                family_style: Some(member.style),
                family_visibility_key: Some(visibility_key),
                style: trace.style,
            });
        }
    }
    traces
}

#[cfg(test)]
mod tests;
