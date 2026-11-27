//! What a strip draws its complex signals as — their level in decibels, their
//! magnitude, their phase, their real or imaginary part, or their group
//! delay — and its noise spectra as — their amplitude density or their power
//! density — chosen strip by strip, as ViVA's strips and ADS's plots choose a
//! frequency response's projection plot by plot.
//!
//! A strip names each curve by signal and form, so a strip's projection is
//! the form it lists its signals in: choosing one rewrites them, and the
//! stack draws what the strips list. A form the analysis' own projection did
//! not make is made here from the signal's retained samples, for every curve
//! the stack draws of it — each member of a sweep's family and each overlay
//! run's — and a signal the strips list in such a form is drawn only in the
//! forms they list, not again on a strip of its default form.

use std::collections::HashMap;
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

use egui::Ui;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::family_projection::SourceSampleSelection;
use rspice_results::run::SimulationRun;
use rspice_results::visualization_document::{
    MAX_STRIP_AXES, SignalForm, Strip, StripAxis, StripMember,
};
use rspice_results::waveform::RetainedTrace;

use super::super::{
    AnalysisPresentationKey, FamilyTraceVisibilityKey, SharedWaveformValues, StripModel,
    StripTrace, TraceKind, signal_form, signal_unit, strip_layout,
};
use super::member_family::{Family, FamilyRole, member_families};

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

/// A signal a strip can show in more than one form: a complex response, or
/// a noise spectrum by the name of its retained power density.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Projectable<'m> {
    Complex(&'m ComplexSource),
    Noise(&'m str),
}

impl<'m> Projectable<'m> {
    /// The retained column the signal's samples are read from.
    fn column(self) -> &'m str {
        match self {
            Self::Complex(source) => &source.magnitude,
            Self::Noise(name) => name,
        }
    }

    /// Whether a curve listed by `name` is this signal.
    fn names(self, name: &str) -> bool {
        match self {
            Self::Complex(source) => source.names(name),
            Self::Noise(column) => column == name,
        }
    }

    /// Whether the signal can be shown in `projection`.
    fn takes(self, projection: StripProjection) -> bool {
        matches!(self, Self::Noise(_)) == projection.is_noise()
    }
}

/// What a strip draws its signals as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripProjection {
    Decibels,
    Magnitude,
    Phase,
    Real,
    Imaginary,
    GroupDelay,
    /// A noise spectrum's amplitude density, per root hertz.
    Density,
    /// A noise spectrum's power density, per hertz: the square of its
    /// amplitude density.
    PowerDensity,
}

impl StripProjection {
    /// Every projection, the complex ones and then the noise ones.
    pub const ALL: [Self; 8] = [
        Self::Decibels,
        Self::Magnitude,
        Self::Phase,
        Self::Real,
        Self::Imaginary,
        Self::GroupDelay,
        Self::Density,
        Self::PowerDensity,
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
            Self::Density => "Amplitude density",
            Self::PowerDensity => "Power density",
        }
    }

    /// Whether it shows a noise spectrum rather than a complex response.
    const fn is_noise(self) -> bool {
        matches!(self, Self::Density | Self::PowerDensity)
    }

    const fn form(self) -> SignalForm {
        match self {
            Self::Decibels => SignalForm::Magnitude,
            Self::Magnitude => SignalForm::Value,
            Self::Phase => SignalForm::Phase,
            Self::Real => SignalForm::Real,
            Self::Imaginary => SignalForm::Imaginary,
            Self::GroupDelay => SignalForm::GroupDelay,
            Self::Density => SignalForm::NoiseDensity,
            Self::PowerDensity => SignalForm::NoisePower,
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
            Self::Density => TraceKind::NoiseDensity,
            Self::PowerDensity => TraceKind::NoisePower,
        }
    }

    /// The projection a signal listed in `form` is drawn in.
    const fn of(form: SignalForm) -> Option<Self> {
        match form {
            SignalForm::Magnitude => Some(Self::Decibels),
            SignalForm::Value => Some(Self::Magnitude),
            SignalForm::Phase => Some(Self::Phase),
            SignalForm::Real => Some(Self::Real),
            SignalForm::Imaginary => Some(Self::Imaginary),
            SignalForm::GroupDelay => Some(Self::GroupDelay),
            SignalForm::NoiseDensity => Some(Self::Density),
            SignalForm::NoisePower => Some(Self::PowerDensity),
        }
    }

    /// What a strip lists `source` by in this projection: a complex signal
    /// by its retained phase for the phase, when it has one, else by its
    /// magnitude; a noise spectrum by its retained density.
    fn member(self, source: Projectable<'_>) -> StripMember {
        let name = match (self, source) {
            (
                Self::Phase,
                Projectable::Complex(ComplexSource {
                    phase: Some(phase), ..
                }),
            ) => phase.clone(),
            _ => source.column().to_owned(),
        };
        StripMember::Signal {
            name,
            form: self.form(),
        }
    }

    /// The curve's name in the legend.
    fn curve_name(self, source: Projectable<'_>) -> String {
        let Projectable::Complex(source) = source else {
            return source.column().to_owned();
        };
        match self {
            Self::Phase => format!("phase({})", source.name),
            Self::Real => format!("re({})", source.name),
            Self::Imaginary => format!("im({})", source.name),
            Self::GroupDelay => format!("groupDelay({})", source.name),
            Self::Decibels | Self::Magnitude | Self::Density | Self::PowerDensity => {
                source.magnitude.clone()
            }
        }
    }
}

/// The signal `member` lists, and the projection it is drawn in: a complex
/// signal of `sources`, or a noise spectrum.
fn listed_source<'m>(
    sources: &'m [ComplexSource],
    member: &'m StripMember,
) -> Option<(Projectable<'m>, StripProjection)> {
    let StripMember::Signal { name, form } = member else {
        return None;
    };
    let projection = StripProjection::of(*form)?;
    if projection.is_noise() {
        return Some((Projectable::Noise(name), projection));
    }
    let source = sources.iter().find(|source| source.names(name))?;
    Some((Projectable::Complex(source), projection))
}

/// The projection drawn strip `index` of `model` shows: the one every
/// projectable signal on it is drawn in, or none when they differ or it
/// holds none.
pub fn drawn_projection(model: &StripModel, index: usize) -> Option<StripProjection> {
    let panes = model.unit_panes();
    let mut shown = panes
        .get(index)?
        .all_traces()
        .filter_map(|trace| model.trace_member(trace))
        .filter_map(|member| {
            listed_source(&model.complex_sources, &member).map(|(_, projection)| projection)
        });
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
    source: Projectable<'_>,
    projection: StripProjection,
) -> String {
    let retained = match source {
        Projectable::Complex(source) => source.unit.as_deref(),
        Projectable::Noise(column) => model
            .traces
            .iter()
            .find(|trace| trace.source_waveform_name == column)
            .and_then(|trace| trace.unit.as_deref()),
    };
    signal_unit(
        &projection.curve_name(source),
        projection.kind(),
        retained,
        model.y_unit,
    )
    .to_owned()
}

/// The strips with drawn strip `index` showing its signals in `projection`:
/// on its first axes, one per unit, its other curves on the axes after them.
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
                Some((source, _)) if source.takes(projection) => {
                    let member = projection.member(source);
                    if !projected.iter().any(|(held, _)| *held == member) {
                        projected.push((member, projected_unit(model, source, projection)));
                    }
                }
                _ => rest.push(member.clone()),
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
        return Err(if projection.is_noise() {
            "The strip draws no noise spectrum.".to_owned()
        } else {
            "The strip draws no complex signal.".to_owned()
        });
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
/// signal shown in more than one form, and the arrangement the one chosen
/// makes. A strip of complex signals offers their forms, a strip of noise
/// spectra theirs.
pub(crate) fn projection_menu(ui: &mut Ui, model: &StripModel, index: usize) -> Option<Vec<Strip>> {
    let panes = model.unit_panes();
    let members: Vec<StripMember> = panes
        .get(index)?
        .all_traces()
        .filter_map(|trace| model.trace_member(trace))
        .collect();
    let sources: Vec<Projectable<'_>> = members
        .iter()
        .filter_map(|member| listed_source(&model.complex_sources, member))
        .map(|(source, _)| source)
        .collect();
    let offered: Vec<StripProjection> = StripProjection::ALL
        .into_iter()
        .filter(|projection| sources.iter().any(|source| source.takes(*projection)))
        .collect();
    if offered.is_empty() {
        return None;
    }
    let shown = drawn_projection(model, index);
    let mut chosen = None;
    rspice_ui_kit::widgets::menu_button(ui, "Show as", |ui| {
        for projection in offered {
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

/// The samples `waveform`, the retained column of a signal, draws in
/// `projection`, at the rows `rows` names, or at every row.
fn projected_samples(
    waveform: &RetainedTrace,
    projection: StripProjection,
    phase_continuous: bool,
    rows: Option<&[usize]>,
) -> Option<SharedWaveformValues> {
    let at = |values: &SharedWaveformValues| -> Option<SharedWaveformValues> {
        let Some(rows) = rows else {
            return Some(values.clone());
        };
        let held = whole(values)?;
        rows.iter()
            .map(|&row| held.get(row).copied())
            .collect::<Option<Vec<f64>>>()
            .map(Into::into)
    };
    let read = |values: &SharedWaveformValues| whole(&at(values)?);
    let mapped = |values: &SharedWaveformValues, map: fn(f64) -> f64| {
        read(values).map(|samples| samples.into_iter().map(map).collect::<Vec<_>>().into())
    };
    let complex = waveform.complex.as_ref();
    match projection {
        StripProjection::Decibels => mapped(&waveform.y, |magnitude| 20.0 * magnitude.log10()),
        StripProjection::Magnitude | StripProjection::PowerDensity => at(&waveform.y),
        StripProjection::Real => at(&complex?.real),
        StripProjection::Imaginary => at(&complex?.imag),
        StripProjection::Phase => {
            let complex = complex?;
            let (real, imaginary) = (read(&complex.real)?, read(&complex.imag)?);
            let wrapped: Vec<f64> = real
                .iter()
                .zip(&imaginary)
                .map(|(re, im)| im.atan2(*re).to_degrees())
                .collect();
            Some(if phase_continuous {
                unwrap_degrees(&wrapped).into()
            } else {
                wrapped.into()
            })
        }
        StripProjection::GroupDelay => {
            let complex = complex?;
            Some(
                group_delay(
                    &read(&waveform.x)?,
                    &read(&complex.real)?,
                    &read(&complex.imag)?,
                )
                .into(),
            )
        }
        // In nV/√Hz, as the noise sheet reads a density; a negative PSD
        // sample clamps to zero rather than inventing a NaN.
        StripProjection::Density => mapped(&waveform.y, |psd| {
            if psd.is_finite() {
                1.0e9 * psd.max(0.0).sqrt()
            } else {
                f64::NAN
            }
        }),
    }
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

/// Where the curves of one strip model read their samples: the analysis of
/// the active run it draws, the members of its family, the overlay runs'
/// counterparts of each, and the rows a sample selection draws.
struct Readings<'r, A> {
    run: &'r SimulationRun<A>,
    overlay_runs: &'r [&'r SimulationRun<A>],
    host: &'r AnalysisResult<RetainedTrace>,
    analysis_index: usize,
    family: Option<&'r Family>,
    selection: Option<&'r SourceSampleSelection>,
    phase_continuous: bool,
}

impl<A: AsRef<AnalysisResult<RetainedTrace>>> Readings<'_, A> {
    /// The analysis of the active run `curve` reads: its family member's,
    /// or the strip's own.
    fn own(&self, curve: &StripTrace) -> Option<&AnalysisResult<RetainedTrace>> {
        let index = self
            .family
            .zip(curve.family_group_ordinal)
            .and_then(|(family, ordinal)| family.members.get(ordinal))
            .filter(|member| member.key == curve.presentation_key)
            .map_or(self.analysis_index, |member| member.analysis);
        self.run.analyses.get(index).map(AsRef::as_ref)
    }

    /// The analysis `curve` reads its samples from: its own, or an overlay
    /// curve's run's solve of it.
    fn source(&self, curve: &StripTrace) -> Option<&AnalysisResult<RetainedTrace>> {
        let own = self.own(curve)?;
        if !curve.overlay {
            return Some(own);
        }
        let overlay_run = self
            .overlay_runs
            .iter()
            .find(|run| run.id == curve.run_id)?;
        super::matching_overlay_analysis(own, overlay_run)
    }

    /// The rows of `analysis` `curve` draws, as the strip's projection
    /// selected them: its family group's under a sample selection, the
    /// selection's, or `Some(None)`, every row. `None` when the selection
    /// no longer resolves for it.
    fn rows(
        &self,
        curve: &StripTrace,
        analysis: &AnalysisResult<RetainedTrace>,
    ) -> Option<Option<Vec<usize>>> {
        let Some(selection) = self.selection else {
            return Some(None);
        };
        let group_rows = |groups: &[rspice_results::family_projection::FamilyRenderGroup]| {
            curve
                .family_group_ordinal
                .and_then(|ordinal| groups.get(ordinal))
                .filter(|group| group.stable_key == curve.presentation_key)
                .map(|group| group.source_indices.clone())
        };
        if curve.overlay {
            return match selection
                .overlay_render_plan(analysis.analysis_type, analysis.family_metadata.as_ref())
                .ok()?
            {
                Some(plan) => group_rows(plan.groups()).map(Some),
                None => Some(Some(selection.source_indices.clone())),
            };
        }
        match selection
            .family_render_plan()
            .map(|plan| plan.groups())
            .filter(|groups| !groups.is_empty())
        {
            Some(groups) => group_rows(groups).map(Some),
            None => Some(Some(selection.source_indices.clone())),
        }
    }

    /// `base`'s curve of `source` drawn in `projection` instead: from the
    /// same analysis and rows, standing where `base` stands, in its member's
    /// style or its run's.
    fn project(
        &self,
        analysis_key: AnalysisPresentationKey,
        source: Projectable<'_>,
        projection: StripProjection,
        base: &StripTrace,
    ) -> Option<StripTrace> {
        let analysis = self.source(base)?;
        let (index, waveform) = analysis
            .waveforms
            .iter()
            .enumerate()
            .find(|(_, waveform)| waveform.name == source.column())?;
        let rows = self.rows(base, analysis)?;
        let y = projected_samples(waveform, projection, self.phase_continuous, rows.as_deref())?;
        if y.len() != base.x.len() {
            return None;
        }
        // A curve of the active run keeps the strip's own source identity,
        // as a family member's does; an overlay's names its run's column.
        let (host_index, host_column) = self
            .host
            .waveforms
            .iter()
            .enumerate()
            .find(|(_, waveform)| waveform.name == source.column())?;
        let StripMember::Signal {
            name: source_waveform_name,
            ..
        } = projection.member(source)
        else {
            return None;
        };
        let kind = projection.kind();
        let base_name = projection.curve_name(source);
        Some(StripTrace {
            waveform_index: if base.overlay { index } else { host_index },
            name: match base.name.strip_prefix(base.base_name.as_str()) {
                Some(tag) => format!("{base_name}{tag}"),
                None => base_name.clone(),
            },
            base_name,
            unit: host_column.unit.clone(),
            signal_color: base.signal_color,
            color: base.color,
            palette_slot: base.palette_slot,
            x: base.x.clone(),
            y,
            shape: Arc::clone(&base.shape),
            kind,
            visible: base.visible,
            run_id: base.run_id,
            overlay: base.overlay,
            presentation_key: base.presentation_key,
            family_group_ordinal: base.family_group_ordinal,
            family_style: base.family_style,
            family_visibility_key: base.family_visibility_key.map(|_| {
                FamilyTraceVisibilityKey::new(
                    analysis_key,
                    base.presentation_key,
                    &source_waveform_name,
                    kind,
                )
            }),
            source_waveform_name,
            style: base.style,
        })
    }
}

/// Give each analysis' strips the curves they list in a projection the
/// analysis' own did not make, and let go of a projected signal's curves no
/// strip lists, so it is drawn only as the strips show it.
///
/// `display_runs` are the runs the stack draws, the active run first and
/// then its overlays, and `selection` the sample selection it draws them
/// under. Each projected curve is made from the retained samples the curve
/// it replaces drew: every member of a family keeps its member's style and
/// visibility, and every overlay its run's.
pub fn project_strips<A: AsRef<AnalysisResult<RetainedTrace>>, R: AsRef<SimulationRun<A>>>(
    models: &mut [StripModel],
    display_runs: &[&R],
    strips: &HashMap<AnalysisPresentationKey, Vec<Strip>>,
    selection: Option<&SourceSampleSelection>,
) {
    let Some((&run, overlay_runs)) = display_runs.split_first() else {
        return;
    };
    let run = run.as_ref();
    let overlay_runs: Vec<&SimulationRun<A>> = overlay_runs
        .iter()
        .map(|overlay| (*overlay).as_ref())
        .collect();
    let families = member_families(&run.analyses);
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
        // A selection slices one analysis's own rows; with one in force the
        // host is drawn as that analysis alone, as the projection drew it.
        let selection = selection.filter(|selection| {
            selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
        });
        let family = match families.get(&model.analysis_index) {
            Some(FamilyRole::Host(family)) if selection.is_none() => Some(family),
            _ => None,
        };
        let readings = Readings {
            run,
            overlay_runs: &overlay_runs,
            host: analysis,
            analysis_index: model.analysis_index,
            family,
            selection,
            phase_continuous: model.phase_continuous,
        };
        let sources = model.complex_sources.clone();
        let listed: Vec<&StripMember> = arranged.iter().flat_map(Strip::members).collect();
        let drawn: Vec<StripMember> = (0..model.signal_trace_count)
            .filter_map(|index| model.trace_member(index))
            .collect();
        let mut added: Vec<StripTrace> = Vec::new();
        let mut added_overlays: Vec<StripTrace> = Vec::new();
        let mut made: Vec<&StripMember> = Vec::new();
        let mut projected: Vec<Projectable<'_>> = Vec::new();
        for &member in &listed {
            if drawn.contains(member) || made.contains(&member) {
                continue;
            }
            let Some((source, projection)) = listed_source(&sources, member) else {
                continue;
            };
            if projection.member(source) != *member {
                continue;
            }
            // The curves the stack draws of the signal in one of its forms,
            // one for each member and each overlay run, each made again in
            // this projection.
            let Some(form) = model
                .traces
                .iter()
                .find(|trace| source.names(&trace.source_waveform_name))
            else {
                continue;
            };
            let curves: Vec<StripTrace> = model
                .traces
                .iter()
                .filter(|trace| {
                    trace.kind == form.kind
                        && trace.source_waveform_name == form.source_waveform_name
                })
                .filter_map(|base| readings.project(model.analysis_key, source, projection, base))
                .collect();
            if curves.is_empty() {
                continue;
            }
            for curve in curves {
                if curve.overlay {
                    added_overlays.push(curve);
                } else {
                    added.push(curve);
                }
            }
            made.push(member);
            projected.push(source);
        }
        if added.is_empty() && added_overlays.is_empty() {
            continue;
        }
        let signal_count = model.signal_trace_count;
        let mut signals = 0;
        let mut kept = Vec::with_capacity(model.traces.len() + added.len() + added_overlays.len());
        for (index, trace) in std::mem::take(&mut model.traces).into_iter().enumerate() {
            let let_go = projected
                .iter()
                .any(|source| source.names(&trace.source_waveform_name))
                && !listed.contains(&&model_member(&trace));
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
        kept.extend(added_overlays);
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
