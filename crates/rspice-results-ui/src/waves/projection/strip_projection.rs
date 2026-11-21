//! What a strip draws its complex signals as — their level in decibels, their
//! magnitude, their phase, their real or imaginary part, or their group
//! delay — chosen strip by strip, as ViVA's strips and ADS's plots choose a
//! frequency response's projection plot by plot.
//!
//! A strip names each curve by signal and form, so a strip's projection is
//! the form it lists its complex signals in: choosing one rewrites them, and
//! the stack draws what the strips list. A form the analysis' own projection
//! did not make is made here from the signal's retained complex samples, and
//! a complex signal the strips list in such a form is drawn only in the forms
//! they list, not again on a strip of its default form.

use std::collections::HashMap;
use std::f64::consts::{PI, TAU};

use egui::Ui;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::run::SimulationRun;
use rspice_results::visualization_document::{
    MAX_STRIP_AXES, SignalForm, Strip, StripAxis, StripMember,
};
use rspice_results::waveform::RetainedTrace;

use super::super::{
    AnalysisPresentationKey, SharedWaveformValues, StripModel, StripTrace, TraceKind, signal_form,
    signal_unit, strip_layout,
};

/// A complex signal of an analysis: the retained magnitude that carries its
/// complex samples, and its retained phase when there is one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexSource {
    /// The signal's own name: `V(out)`.
    pub name: String,
    /// The retained magnitude carrying the complex samples: `|V(out)|`.
    pub magnitude: String,
    /// The retained phase: `phase(V(out))`.
    pub phase: Option<String>,
    /// The unit the magnitude states for its samples.
    pub unit: Option<String>,
}

impl ComplexSource {
    /// Whether a curve listed by `name` is this signal.
    fn names(&self, name: &str) -> bool {
        self.magnitude == name || self.phase.as_deref() == Some(name)
    }
}

/// The complex signals of a frequency response `analysis`.
pub(crate) fn complex_sources(analysis: &AnalysisResult<RetainedTrace>) -> Vec<ComplexSource> {
    if !analysis.analysis_type.uses_complex_bode_projection() {
        return Vec::new();
    }
    analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            let complex = waveform.complex.as_ref()?;
            let phase = format!("phase({})", complex.source_name);
            Some(ComplexSource {
                name: complex.source_name.clone(),
                magnitude: waveform.name.clone(),
                phase: analysis
                    .waveforms
                    .iter()
                    .any(|candidate| candidate.name == phase)
                    .then_some(phase),
                unit: waveform.unit.clone(),
            })
        })
        .collect()
}

/// What a strip draws its complex signals as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripProjection {
    Decibels,
    Magnitude,
    Phase,
    Real,
    Imaginary,
    GroupDelay,
}

impl StripProjection {
    pub const ALL: [Self; 6] = [
        Self::Decibels,
        Self::Magnitude,
        Self::Phase,
        Self::Real,
        Self::Imaginary,
        Self::GroupDelay,
    ];

    /// The words the strip menu offers it by.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Decibels => "dB",
            Self::Magnitude => "Magnitude",
            Self::Phase => "Phase",
            Self::Real => "Real",
            Self::Imaginary => "Imaginary",
            Self::GroupDelay => "Group delay",
        }
    }

    const fn form(self) -> SignalForm {
        match self {
            Self::Decibels => SignalForm::Magnitude,
            Self::Magnitude => SignalForm::Value,
            Self::Phase => SignalForm::Phase,
            Self::Real => SignalForm::Real,
            Self::Imaginary => SignalForm::Imaginary,
            Self::GroupDelay => SignalForm::GroupDelay,
        }
    }

    const fn kind(self) -> TraceKind {
        match self {
            Self::Decibels => TraceKind::MagnitudeDb,
            Self::Magnitude => TraceKind::Value,
            Self::Phase => TraceKind::PhaseDeg,
            Self::Real => TraceKind::Real,
            Self::Imaginary => TraceKind::Imaginary,
            Self::GroupDelay => TraceKind::GroupDelay,
        }
    }

    /// The projection a complex signal listed in `form` is drawn in.
    const fn of(form: SignalForm) -> Option<Self> {
        match form {
            SignalForm::Magnitude => Some(Self::Decibels),
            SignalForm::Value => Some(Self::Magnitude),
            SignalForm::Phase => Some(Self::Phase),
            SignalForm::Real => Some(Self::Real),
            SignalForm::Imaginary => Some(Self::Imaginary),
            SignalForm::GroupDelay => Some(Self::GroupDelay),
            SignalForm::NoiseDensity => None,
        }
    }

    /// What a strip lists `source` by in this projection: its retained
    /// phase for the phase, when it has one, else its magnitude.
    fn member(self, source: &ComplexSource) -> StripMember {
        let name = match (self, &source.phase) {
            (Self::Phase, Some(phase)) => phase.clone(),
            _ => source.magnitude.clone(),
        };
        StripMember::Signal {
            name,
            form: self.form(),
        }
    }

    /// The curve's name in the legend.
    fn curve_name(self, source: &ComplexSource) -> String {
        match self {
            Self::Decibels | Self::Magnitude => source.magnitude.clone(),
            Self::Phase => format!("phase({})", source.name),
            Self::Real => format!("re({})", source.name),
            Self::Imaginary => format!("im({})", source.name),
            Self::GroupDelay => format!("groupDelay({})", source.name),
        }
    }
}

/// The complex signal `member` lists, and the projection it is drawn in.
fn listed_source<'m>(
    sources: &'m [ComplexSource],
    member: &StripMember,
) -> Option<(&'m ComplexSource, StripProjection)> {
    let StripMember::Signal { name, form } = member else {
        return None;
    };
    let source = sources.iter().find(|source| source.names(name))?;
    Some((source, StripProjection::of(*form)?))
}

/// The projection drawn strip `index` of `model` shows: the one every
/// complex signal on it is drawn in, or none when they differ or it holds
/// none.
pub fn drawn_projection(model: &StripModel, index: usize) -> Option<StripProjection> {
    let panes = model.unit_panes();
    let mut shown = panes
        .get(index)?
        .all_traces()
        .filter_map(|trace| model.trace_member(trace))
        .filter_map(|member| listed_source(&model.complex_sources, &member))
        .map(|(_, projection)| projection);
    let first = shown.next()?;
    shown.all(|projection| projection == first).then_some(first)
}

/// Where drawn strip `index` is among the strips `strips` writes down.
fn written_position(model: &StripModel, strips: &[Strip], index: usize) -> Option<usize> {
    let panes = model.unit_panes();
    let pane = panes.get(index)?;
    if pane.arranged {
        return strips.iter().position(|strip| strip.key == pane.unit);
    }
    if model.arrangement.is_empty() {
        return (index < strips.len()).then_some(index);
    }
    let first = pane
        .all_traces()
        .find_map(|trace| model.trace_member(trace))?;
    strips
        .iter()
        .position(|strip| strip.members().any(|member| *member == first))
}

/// The unit the stack reads `source` in under `projection`.
fn projected_unit(
    model: &StripModel,
    source: &ComplexSource,
    projection: StripProjection,
) -> String {
    signal_unit(
        &projection.curve_name(source),
        projection.kind(),
        source.unit.as_deref(),
        model.y_unit,
    )
    .to_owned()
}

/// The strips with drawn strip `index` showing its complex signals in
/// `projection`: on its first axes, one per unit, its other curves on the
/// axes after them.
pub fn set_projection(
    model: &StripModel,
    index: usize,
    projection: StripProjection,
) -> Result<Vec<Strip>, String> {
    let mut strips = strip_layout::layout(model);
    let position = written_position(model, &strips, index)
        .ok_or_else(|| "The strip is no longer drawn.".to_owned())?;
    let strip = &mut strips[position];
    let mut projected: Vec<(StripMember, String)> = Vec::new();
    let mut others: Vec<StripAxis> = Vec::new();
    for axis in &strip.axes {
        let mut rest = Vec::new();
        for member in &axis.members {
            match listed_source(&model.complex_sources, member) {
                Some((source, _)) => {
                    let member = projection.member(source);
                    if !projected.iter().any(|(held, _)| *held == member) {
                        projected.push((member, projected_unit(model, source, projection)));
                    }
                }
                None => rest.push(member.clone()),
            }
        }
        if !rest.is_empty() {
            others.push(StripAxis {
                unit: axis.unit.clone(),
                members: rest,
            });
        }
    }
    if projected.is_empty() {
        return Err("The strip draws no complex signal.".to_owned());
    }
    let mut axes: Vec<StripAxis> = Vec::new();
    let mut place =
        |unit: String, member: StripMember| match axes.iter_mut().find(|axis| axis.unit == unit) {
            Some(axis) => {
                if !axis.members.contains(&member) {
                    axis.members.push(member);
                }
            }
            None => axes.push(StripAxis {
                unit,
                members: vec![member],
            }),
        };
    for (member, unit) in projected {
        place(unit, member);
    }
    for axis in others {
        for member in axis.members {
            place(axis.unit.clone(), member);
        }
    }
    if axes.len() > MAX_STRIP_AXES {
        return Err(format!(
            "A strip reads against {MAX_STRIP_AXES} Y axes at most. Move a curve off it first."
        ));
    }
    strip.axes = axes;
    Ok(strips)
}

/// The projection entries of drawn strip `index`'s menu, when it draws a
/// complex signal, and the arrangement the one chosen makes.
pub(crate) fn projection_menu(ui: &mut Ui, model: &StripModel, index: usize) -> Option<Vec<Strip>> {
    let panes = model.unit_panes();
    let draws_complex = panes.get(index)?.all_traces().any(|trace| {
        model
            .trace_member(trace)
            .is_some_and(|member| listed_source(&model.complex_sources, &member).is_some())
    });
    if !draws_complex {
        return None;
    }
    let shown = drawn_projection(model, index);
    let mut chosen = None;
    rspice_ui_kit::widgets::menu_button(ui, "Show as", |ui| {
        for projection in StripProjection::ALL {
            let selected = shown == Some(projection);
            match set_projection(model, index, projection) {
                Ok(strips) => {
                    if ui.radio(selected, projection.label()).clicked() && !selected {
                        chosen = Some(strips);
                        ui.close();
                    }
                }
                Err(reason) => {
                    ui.add_enabled(false, egui::RadioButton::new(selected, projection.label()))
                        .on_disabled_hover_text(reason);
                }
            }
        }
    });
    chosen
}

/// Every sample of `values`, once it has been read.
fn whole(values: &SharedWaveformValues) -> Option<Vec<f64>> {
    values.fetch_whole().ready().map(|held| held.to_vec())
}

/// The group delay of a response sampled at `frequency` in hertz: the slope
/// of its unwrapped phase against angular frequency, negated, in seconds.
/// Each sample takes the slope across its two neighbours, an end sample the
/// slope to its one; a pure delay reads its delay at every sample, whatever
/// the spacing.
pub(crate) fn group_delay(frequency: &[f64], real: &[f64], imaginary: &[f64]) -> Vec<f64> {
    let count = frequency.len().min(real.len()).min(imaginary.len());
    let mut phase = Vec::with_capacity(count);
    let mut previous: Option<f64> = None;
    for index in 0..count {
        let mut radians = imaginary[index].atan2(real[index]);
        if let Some(previous) = previous.filter(|_| radians.is_finite()) {
            radians += TAU * ((previous - radians) / TAU).round();
            if radians - previous > PI {
                radians -= TAU;
            }
        }
        if radians.is_finite() {
            previous = Some(radians);
        }
        phase.push(radians);
    }
    (0..count)
        .map(|index| {
            let (before, after) = (index.saturating_sub(1), (index + 1).min(count - 1));
            let span = frequency[after] - frequency[before];
            if before == after || span == 0.0 {
                return f64::NAN;
            }
            -(phase[after] - phase[before]) / (TAU * span)
        })
        .collect()
}

/// The curve `source` draws in `projection`, made from its retained
/// samples, standing where `base`, its curve in the analysis' own
/// projection, stands.
fn projected_trace(
    analysis: &AnalysisResult<RetainedTrace>,
    source: &ComplexSource,
    projection: StripProjection,
    phase_continuous: bool,
    base: &StripTrace,
) -> Option<StripTrace> {
    let (waveform_index, waveform) = analysis
        .waveforms
        .iter()
        .enumerate()
        .find(|(_, waveform)| waveform.name == source.magnitude)?;
    let complex = waveform.complex.as_ref()?;
    let y: SharedWaveformValues = match projection {
        StripProjection::Decibels => whole(&waveform.y)?
            .into_iter()
            .map(|magnitude| 20.0 * magnitude.log10())
            .collect::<Vec<_>>()
            .into(),
        StripProjection::Magnitude => waveform.y.clone(),
        StripProjection::Real => complex.real.clone(),
        StripProjection::Imaginary => complex.imag.clone(),
        StripProjection::Phase => {
            let (real, imaginary) = (whole(&complex.real)?, whole(&complex.imag)?);
            let wrapped: Vec<f64> = real
                .iter()
                .zip(&imaginary)
                .map(|(re, im)| im.atan2(*re).to_degrees())
                .collect();
            if phase_continuous {
                unwrap_degrees(&wrapped).into()
            } else {
                wrapped.into()
            }
        }
        StripProjection::GroupDelay => group_delay(
            &whole(&waveform.x)?,
            &whole(&complex.real)?,
            &whole(&complex.imag)?,
        )
        .into(),
    };
    let name = projection.curve_name(source);
    Some(StripTrace {
        waveform_index,
        source_waveform_name: match projection.member(source) {
            StripMember::Signal { name, .. } => name,
            StripMember::Expression { text } => text,
        },
        base_name: name.clone(),
        name,
        unit: waveform.unit.clone(),
        signal_color: base.signal_color,
        color: base.color,
        palette_slot: base.palette_slot,
        x: waveform.x.clone(),
        y,
        shape: std::sync::Arc::clone(&base.shape),
        kind: projection.kind(),
        visible: base.visible,
        run_id: base.run_id,
        overlay: false,
        presentation_key: base.presentation_key,
        family_group_ordinal: None,
        family_style: None,
        family_visibility_key: None,
        style: base.style,
    })
}

/// `degrees` with each jump of more than half a turn taken out.
fn unwrap_degrees(degrees: &[f64]) -> Vec<f64> {
    let mut previous: Option<f64> = None;
    degrees
        .iter()
        .map(|&value| {
            let Some(before) = previous.filter(|_| value.is_finite()) else {
                if value.is_finite() {
                    previous = Some(value);
                }
                return value;
            };
            let value = value + 360.0 * ((before - value) / 360.0).round();
            previous = Some(value);
            value
        })
        .collect()
}

/// Give each analysis' strips the curves they list in a projection the
/// analysis' own did not make, from the active run `run`'s retained
/// complex samples, and let go of a projected signal's curves no strip
/// lists, so it is drawn only as the strips show it.
pub fn project_strips<A: AsRef<AnalysisResult<RetainedTrace>>, R: AsRef<SimulationRun<A>>>(
    models: &mut [StripModel],
    run: &R,
    strips: &HashMap<AnalysisPresentationKey, Vec<Strip>>,
) {
    let run = run.as_ref();
    for model in models {
        let Some(arranged) = strips.get(&model.analysis_key) else {
            continue;
        };
        let Some(analysis) = run
            .analyses
            .get(model.analysis_index)
            .map(AsRef::as_ref)
            .filter(|analysis| {
                AnalysisPresentationKey::new(run.dataset_id, analysis) == model.analysis_key
            })
        else {
            continue;
        };
        let sources = model.complex_sources.clone();
        let listed: Vec<&StripMember> = arranged.iter().flat_map(Strip::members).collect();
        let drawn: Vec<StripMember> = (0..model.signal_trace_count)
            .filter_map(|index| model.trace_member(index))
            .collect();
        let mut added: Vec<StripTrace> = Vec::new();
        let mut projected: Vec<&str> = Vec::new();
        for &member in &listed {
            if drawn.contains(member) {
                continue;
            }
            let Some((source, projection)) = listed_source(&sources, member) else {
                continue;
            };
            if projection.member(source) != *member
                || added.iter().any(|trace| model_member(trace) == *member)
            {
                continue;
            }
            let Some(base) = model.traces[..model.signal_trace_count]
                .iter()
                .find(|trace| source.names(&trace.source_waveform_name))
            else {
                continue;
            };
            let Some(trace) =
                projected_trace(analysis, source, projection, model.phase_continuous, base)
            else {
                continue;
            };
            added.push(trace);
            projected.push(&source.magnitude);
        }
        if added.is_empty() {
            continue;
        }
        let signal_count = model.signal_trace_count;
        let mut signals = 0;
        let mut kept = Vec::with_capacity(model.traces.len() + added.len());
        for (index, trace) in std::mem::take(&mut model.traces).into_iter().enumerate() {
            let let_go = sources.iter().any(|source| {
                source.names(&trace.source_waveform_name)
                    && projected.contains(&source.magnitude.as_str())
            }) && !listed.contains(&&model_member(&trace));
            if let_go {
                continue;
            }
            if index < signal_count {
                signals += 1;
            }
            kept.push(trace);
        }
        let count = added.len();
        kept.splice(signals..signals, added);
        model.traces = kept;
        model.signal_trace_count = signals + count;
    }
}

/// What a strip lists `trace` by.
fn model_member(trace: &StripTrace) -> StripMember {
    StripMember::Signal {
        name: trace.source_waveform_name.clone(),
        form: signal_form(trace.kind),
    }
}

#[cfg(test)]
mod tests;
