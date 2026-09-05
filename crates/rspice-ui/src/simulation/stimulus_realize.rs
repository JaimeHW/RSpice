//! One route from a source to its card, its spec and its waveform.
//!
//! Three implementations of the source families used to exist: the engine's,
//! the Component Properties preview's hand-rolled sampler, and the mockup's.
//! Two of them could be — and were — wrong: an omitted pulse width with
//! authored edges resolves to zero in the engine and to the whole stop time in
//! the other two, so a card that draws one triangle in a run drew a square wave
//! in the dialog and nobody could see the disagreement.
//!
//! This module is the only bridge, and it deliberately owns no semantics:
//!
//! - the card comes from the netlist generator, through the same positional
//!   lists a deck is written with;
//! - the spec comes from `rspice_core::netlist::parse_source_spec_text`, the
//!   parser the engine reads decks with;
//! - the samples come from `VoltageSources::evaluate_source_spec_at_time_with_dialect`,
//!   the evaluator a transient steps with.
//!
//! Everything here is plumbing between those three. The one judgement it does
//! make is where to *look* — the window and the sample count — and that is a
//! choice of x-range, never of value.

use rspice_core::circuit::VoltageSources;
use rspice_core::config::SpiceDialect;
use rspice_core::netlist::{ParamContext, SourceSpec};

use crate::state::stimulus_library::definition::StimulusDefinition;
use crate::state::{Component, ComponentType, Point};

/// The two nets a source is realized against when nobody is asking about a
/// particular sheet.
///
/// A preview cares about the waveform, not the topology, so the nets are named
/// rather than resolved. They appear in the card text and nowhere else.
pub(crate) const DETACHED_NETS: [&str; 2] = ["p", "n"];

/// The card this placed source emits, between these two nets.
///
/// The generator writes it, so this is the same text the deck carries for the
/// same instance — minus only the companion elements a source's parasitics add
/// beside it, which are separate devices.
pub(crate) fn source_card_text(
    component: &Component,
    nets: [&str; 2],
) -> Result<String, Vec<String>> {
    crate::simulation::netlist_gen::independent_source_card(
        component,
        nets,
        &component.spice_instance_name(),
    )
}

impl StimulusDefinition {
    /// A component carrying this definition's card and nothing else.
    ///
    /// It is never placed and its id means nothing; it exists so that a
    /// definition can be asked every question a placed source can be asked —
    /// its card, its engine-contract findings, its waveform — through the code
    /// that already answers those questions for instances. Realizing a
    /// definition is therefore never a second implementation of anything.
    #[must_use]
    pub fn transient_component(&self) -> Component {
        let mut component = Component::new(0, self.component_type(), Point::origin());
        component.name = self.name().to_owned();
        // Adoption is the copy, and it cannot refuse a component built from
        // this definition's own type; the fallback keeps the function total
        // rather than asserting.
        if self.adopt_onto(&mut component).is_err() {
            component.value = self.value.clone();
            component.params = self.params.clone();
        }
        component
    }

    /// The card this definition realizes to, between these two nets.
    ///
    /// The generator writes it, through the instance the definition would be
    /// adopted onto, so a library's realization line and a placed adopter's
    /// deck line cannot spell the same definition differently.
    pub fn card_text(&self, nets: [&str; 2]) -> Result<String, Vec<String>> {
        source_card_text(&self.transient_component(), nets)
    }
}

/// The engine's own reading of this source's card.
///
/// The card is written first and then read back, rather than the spec being
/// assembled from the property sheet: a preview that agreed with the sheet but
/// not with the text would be exactly the class of disagreement this module
/// exists to close.
///
/// The parse runs against an empty [`ParamContext`], so a field holding a
/// design variable does not resolve and is reported here rather than silently
/// taking some other scope's value — resolving design variables is the
/// netlister's business, at the point where a deck knows its own `.param`s.
pub(crate) fn source_spec(component: &Component) -> Result<SourceSpec, String> {
    let card = source_card_text(component, DETACHED_NETS).map_err(|errors| errors.join("; "))?;
    let specification = specification_of(&card).ok_or_else(|| {
        format!(
            "{} '{}' emitted a card with no source specification",
            component.kind.display_name(),
            component.name
        )
    })?;
    rspice_core::netlist::parse_source_spec_text(specification, 0, &ParamContext::new())
        .map_err(|error| error.to_string())
}

/// The specification part of a card: everything after the instance name and its
/// two node names, which is what the engine's parser expects.
fn specification_of(card: &str) -> Option<&str> {
    let mut rest = card.trim_start();
    for _ in 0..3 {
        let (_, tail) = rest.split_once(char::is_whitespace)?;
        rest = tail.trim_start();
    }
    (!rest.is_empty()).then_some(rest)
}

/// Where a preview looks, and how finely.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PreviewWindow {
    /// First sample time, in seconds.
    pub start: f64,
    /// Last sample time, in seconds.
    pub stop: f64,
    /// How many samples, inclusive of both ends. Fewer than two is no curve.
    pub samples: usize,
}

/// The transient the preview evaluates against.
///
/// Every omitted waveform field resolves against a stop time — an omitted pulse
/// period becomes TSTOP, an omitted SIN frequency becomes 1/TSTOP — so a
/// preview cannot show a source without naming which transient it is showing it
/// under, and the card states this in its title.
///
/// The engine's own fallbacks when it is handed neither
/// (`evaluate_source_spec_at_time_with_dialect`: TSTEP 1e-12, TSTOP 1e99) are
/// fail-safes for an evaluator, not a window anyone can draw: a hundred
/// picoseconds of a 1e99-second sweep is a flat line for every family there is.
/// So a caller with no transient analysis in the plan passes the plan's own
/// default transient instead and says so, which is what a run would use if the
/// user added one without changing a field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PreviewTiming {
    /// TSTEP, in seconds.
    pub tstep: f64,
    /// TSTOP, in seconds.
    pub tstop: f64,
    /// Whether these came from a transient analysis the plan actually holds.
    pub from_analysis: bool,
}

impl Default for PreviewTiming {
    /// The plan's own default transient: `TranSetup::default()`'s `1m` stop and
    /// `10n` step, which is what a run would use if someone added a transient
    /// without changing a field. Named here rather than imported because the
    /// setup type lives in the application shell, above this layer; the two are
    /// pinned together by a test beside the shell's own copy.
    fn default() -> Self {
        Self {
            tstep: 1e-8,
            tstop: 1e-3,
            from_analysis: false,
        }
    }
}

impl PreviewTiming {
    /// The window a preview of this timing looks through.
    #[must_use]
    pub fn window(&self, samples: usize) -> PreviewWindow {
        PreviewWindow {
            start: 0.0,
            stop: if self.tstop.is_finite() && self.tstop > 0.0 {
                self.tstop
            } else {
                1.0
            },
            samples,
        }
    }

    /// How the preview names the transient it is showing, for the card's title.
    #[must_use]
    pub fn caption(&self) -> String {
        let stop = crate::state::format_engineering(self.tstop);
        let step = crate::state::format_engineering(self.tstep);
        if self.from_analysis {
            format!("TRAN {stop}s · TSTEP {step}s")
        } else {
            format!("TRAN {stop}s · TSTEP {step}s · plan default")
        }
    }
}

/// The dialect a project's sources are read under.
///
/// Every netlist flavour a project offers maps to `BestAvailable` or `Ngspice`
/// and nothing selects `Xyce`, so a preview that asked the project which one to
/// use would be offering a choice the project cannot make.
pub(crate) const PREVIEW_DIALECT: SpiceDialect = SpiceDialect::BestAvailable;

/// This spec's waveform, as the engine steps it.
///
/// One point per sample, `(time, value)`, evaluated by the transient's own
/// evaluator — including every substitution it makes for an omitted field, so
/// an edges-only `PULSE(0 5 0 1n 1n)` shows the single zero-width pulse the
/// ngspice-46/47 rule produces rather than the square wave its labels suggest,
/// and an `SFFM` authored with `FC=0` shows the engine's `5 / TSTOP` carrier.
pub(crate) fn evaluate_waveform(
    spec: &SourceSpec,
    window: PreviewWindow,
    tstep: f64,
    tstop: f64,
    dialect: SpiceDialect,
) -> Vec<(f64, f64)> {
    if window.samples < 2 || !window.start.is_finite() || !window.stop.is_finite() {
        return Vec::new();
    }
    let span = window.stop - window.start;
    (0..window.samples)
        .map(|index| {
            let time = window.start + span * index as f64 / (window.samples - 1) as f64;
            (
                time,
                VoltageSources::evaluate_source_spec_at_time_with_dialect(
                    spec, time, tstep, tstop, dialect,
                ),
            )
        })
        .collect()
}

/// The numbers the preview card reads off its own curve.
///
/// Derived from the samples the engine produced, never from a second reading of
/// the card: an axis label that disagreed with the line above it would be the
/// same defect in miniature.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct WaveformReadouts {
    /// Lowest sampled value.
    pub minimum: f64,
    /// Highest sampled value.
    pub maximum: f64,
    /// Halfway between them, which is where the middle gridline sits.
    pub midpoint: f64,
}

impl WaveformReadouts {
    /// Read the curve, or `None` when there is no curve to read.
    #[must_use]
    pub fn of(samples: &[(f64, f64)]) -> Option<Self> {
        if samples.len() < 2 {
            return None;
        }
        let (minimum, maximum) = samples.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY),
            |(minimum, maximum), (_, value)| (minimum.min(*value), maximum.max(*value)),
        );
        if !minimum.is_finite() || !maximum.is_finite() {
            return None;
        }
        Some(Self {
            minimum,
            maximum,
            midpoint: (maximum + minimum) * 0.5,
        })
    }

    /// The vertical distance the curve covers.
    #[must_use]
    pub fn span(&self) -> f64 {
        (self.maximum - self.minimum).abs()
    }
}

/// Why the engine cannot draw this spec, or `None` when it can.
///
/// Two families have no waveform at this boundary, and both are the engine's
/// own answer rather than a limitation of the preview:
///
/// - `PWL FILE=` is read from the path the card names, and when that read fails
///   the evaluator logs and returns the value offset — a flat line that looks
///   like a waveform and is not one. A definition's retained copy of the table
///   deliberately does not rescue this: the loader takes a path, not bytes, so
///   bytes the app is holding are not something it can be asked to step
///   through. The retained copy is what makes a project self-contained when it
///   is reopened somewhere the file exists again.
/// - `TRNOISE` evaluates to exactly 0 here. The noise train is expanded into a
///   seeded PWL sample series when the transient's circuit is built, so it is
///   not a function of time the spec can be asked for, which is the same reason
///   the operating point sees zero from it.
pub(crate) fn preview_defect(spec: &SourceSpec) -> Option<String> {
    match transient_part(spec) {
        SourceSpec::PwlFile { path, .. } if !std::path::Path::new(path).is_file() => Some(format!(
            "This preview needs the data file '{path}', which is not readable here."
        )),
        SourceSpec::TrNoise { .. } => Some(
            "A noise source has no waveform until a run builds it: the engine expands TRNOISE \
             into a seeded sample train when the transient starts, and the source contributes \
             exactly its DC level until then."
                .to_owned(),
        ),
        _ => None,
    }
}

/// The time-domain half of a spec, past the DC, AC, port and distortion
/// annotations that wrap it.
fn transient_part(spec: &SourceSpec) -> &SourceSpec {
    match spec {
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            transient_part(inner)
        }
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            transient_part(transient)
        }
        other => other,
    }
}

/// Whether this component type is one the bridge can realize at all.
///
/// Every independent source is, and nothing else on the sheet has a waveform:
/// a behavioural source's expression and a controlled source's gain are not
/// `SourceSpec`s and the engine does not read them through this parser.
#[must_use]
pub(crate) fn is_independent_source(kind: ComponentType) -> bool {
    crate::state::stimulus_library::definition::StimulusFamily::of(kind).is_some()
}

#[cfg(test)]
mod tests;
