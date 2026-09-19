//! Where the proof surface looks, what it marks, and what it states.
//!
//! Everything here is a choice of *x-range* or a restatement of a number the
//! engine resolved. Nothing evaluates a waveform: the trace comes from
//! [`sample_trace`], which asks the transient's own evaluator, and every
//! substituted field — a pulse width the card omitted, a sine frequency that
//! defaults to `1 / TSTOP`, an exponential time constant that defaults to
//! TSTEP — is read back out of `rspice_core`'s resolvers rather than worked
//! out again up here. The breakpoints a trace is measured at come from those
//! same resolvers, which is why they are stated here beside the guides that
//! label a handful of them.
//!
//! That is the whole discipline of this module. The mockup carries a
//! substitution table because a browser has no engine to ask; the application
//! has one, and a second table would be a second answer that nobody diffs
//! against the first until a run disagrees with the picture that sold it.
//!
//! The one judgement it does make is the fit span per family: how much of a
//! waveform you have to see before you have seen it. Three sine periods, four
//! edge times of a zero-width pulse, six time constants of an exponential
//! decay. Those are presentation, and they are derived from the resolved
//! timing so that a card whose period defaulted is framed by the period it
//! actually got.

use rspice_core::circuit::VoltageSources;
use rspice_core::netlist::SourceSpec;

use super::{
    DETACHED_NETS, PREVIEW_DIALECT, PreviewTiming, PreviewWindow, WaveformReadouts, WaveformTrace,
    preview_defect, sample_trace, source_spec, transient_part,
};
use crate::state::Component;
use crate::state::format_engineering_display;
use crate::state::stimulus_library::definition::{
    StimulusDefinition, StimulusFamily, StimulusKind,
};
use crate::ui::plot::tick_with_unit;

/// How many points the proof surface asks the engine for.
///
/// Finer than the widest stage the instrument fits in, so the curve is limited
/// by the stroke rather than by the sampling, and re-evaluated only when the
/// record, the span or the transient changes.
const PREVIEW_SAMPLES: usize = 480;

/// Which span the surface is looking through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpanChoice {
    /// The natural span of this shape.
    Fit,
    /// One fundamental period, where the family has one; the fit span
    /// otherwise, so a button that cannot answer never draws a blank plot.
    Period,
    /// The plan's transient window, which is what a run will show.
    Transient,
}

/// One labelled breakpoint the surface draws a rule at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Guide {
    /// Where it lands, in seconds from the start of the window.
    pub time: f64,
    /// What the card calls it.
    pub label: &'static str,
}

/// One authored PWL point, as the plot marks it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PwlMarker {
    /// Source time, including the card's TD.
    pub time: f64,
    /// Level at that time.
    pub value: f64,
    /// Row index in the authored table, so a marker and a row select together.
    pub index: usize,
}

/// Everything the proof surface needs about one definition, at one span.
///
/// Built in one pass because every part of it comes from the same parse: a
/// realization whose guides came from one reading of the card and whose curve
/// came from another is the disagreement this whole module exists to close.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StimulusRealization {
    /// The window's length in seconds.
    pub span: f64,
    /// The fundamental period, where this family has one.
    pub fundamental: Option<f64>,
    /// The waveform, as the engine steps it: a curve where the window can
    /// carry one, a measured envelope where it cannot.
    pub trace: WaveformTrace,
    /// Breakpoint rules, already clipped to the window.
    pub guides: Vec<Guide>,
    /// Authored PWL points inside the window.
    pub markers: Vec<PwlMarker>,
    /// Label and value pairs the strip states, in reading order.
    pub derived: Vec<(String, String)>,
    /// What the curve covers vertically.
    pub readouts: Option<WaveformReadouts>,
    /// Why there is no curve, when there is none.
    pub defect: Option<String>,
}

impl StimulusRealization {
    /// Realize one definition over one span, under this transient.
    pub fn of(record: &StimulusDefinition, choice: SpanChoice, timing: PreviewTiming) -> Self {
        let component = record.transient_component();
        let spec = match source_spec(&component) {
            Ok(spec) => spec,
            Err(reason) => {
                return Self {
                    span: timing.tstop,
                    fundamental: None,
                    trace: WaveformTrace::Curve(Vec::new()),
                    guides: Vec::new(),
                    markers: Vec::new(),
                    derived: unresolved_readouts(record),
                    readouts: None,
                    defect: Some(reason),
                };
            }
        };
        let transient = transient_part(&spec);
        let fundamental = fundamental_period(transient, timing);
        let span = match choice {
            SpanChoice::Transient => timing.tstop,
            SpanChoice::Period => fundamental.unwrap_or_else(|| fit_span(transient, timing)),
            SpanChoice::Fit => fit_span(transient, timing),
        };
        let span = if span.is_finite() && span > 0.0 {
            span
        } else {
            timing.tstop
        };
        let defect = preview_defect(&spec);
        let trace = if defect.is_some() {
            WaveformTrace::Curve(Vec::new())
        } else {
            sample_trace(
                &spec,
                PreviewWindow {
                    start: 0.0,
                    stop: span,
                    samples: PREVIEW_SAMPLES,
                },
                timing,
            )
        };
        let readouts = trace.readouts();
        // A band drawn over five hundred cycles has every breakpoint a family
        // names inside its first column, and five rules on one column is five
        // labels on top of each other. Keep what the window can tell apart.
        let resolution = matches!(trace, WaveformTrace::Envelope { .. })
            .then(|| span / (PREVIEW_SAMPLES - 1) as f64);
        let mut guides = guides(transient, timing);
        guides.retain(|guide| guide.time > 0.0 && guide.time <= span && guide.time.is_finite());
        collapse(&mut guides, resolution, |guide| guide.time);
        let mut markers = markers(transient);
        markers.retain(|marker| marker.time >= 0.0 && marker.time <= span);
        collapse(&mut markers, resolution, |marker| marker.time);
        Self {
            span,
            fundamental,
            trace,
            guides,
            markers,
            derived: readouts_of(record, transient, timing, span),
            readouts,
            defect,
        }
    }

    /// The card this definition realizes to, or the generator's refusals.
    ///
    /// Stated here so the realization band and the audit strip read one
    /// result rather than asking the generator twice and disagreeing about
    /// whether it refused.
    pub fn card(record: &StimulusDefinition) -> Result<String, Vec<String>> {
        record.card_text(DETACHED_NETS)
    }
}

/// Drop the entries a window cannot draw apart from the one before them.
///
/// `None` keeps every entry: only a window measured column by column packs
/// enough of them into one column for this to be the difference between a mark
/// and a smear.
fn collapse<T>(entries: &mut Vec<T>, resolution: Option<f64>, time: impl Fn(&T) -> f64) {
    let Some(resolution) = resolution.filter(|resolution| *resolution > 0.0) else {
        return;
    };
    let mut previous: Option<f64> = None;
    entries.retain(|entry| {
        let at = time(entry);
        if previous.is_some_and(|previous| (at - previous).abs() < resolution) {
            return false;
        }
        previous = Some(at);
        true
    });
}

/// The unit a definition of this kind is measured in.
fn unit(record: &StimulusDefinition) -> &'static str {
    match record.kind() {
        StimulusKind::Voltage => "V",
        StimulusKind::Current => "A",
    }
}

/// What a definition whose card could not be read can still say.
///
/// A DC level holding a design variable is the ordinary case: the netlister
/// resolves `{VSUP}` at elaboration, and a preview parsing against an empty
/// parameter scope cannot. Naming the variable is the only fact left, and it
/// is the one the reader wants.
fn unresolved_readouts(record: &StimulusDefinition) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    if let Some(variable) = design_variable(&record.value) {
        rows.push(("variable".to_owned(), variable));
    }
    rows
}

/// The period a preview may measure one column's extremes over, instead of the
/// whole of the time that column covers.
///
/// For every family that repeats exactly, this is the fundamental: the values a
/// strictly periodic waveform takes over one period are the values it takes
/// over any longer stretch, so a window holding five hundred cycles costs the
/// same to measure as one holding five.
///
/// `AM` and `SFFM` are the deliberate exception, and it is the carrier rather
/// than the fundamental they answer with. Their fundamental is the modulating
/// period, and measuring a column over one of those would flatten the
/// modulation into a single band; measuring it over one carrier period reports
/// the carrier's swing *at that column's time*, which is what makes the
/// modulation visible at all.
pub(super) fn resolution_period(spec: &SourceSpec, timing: PreviewTiming) -> Option<f64> {
    match spec {
        SourceSpec::Sffm {
            carrier_freq,
            modulation_index,
            signal_freq,
            ..
        } => {
            let (fc, _, _) =
                sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, timing);
            positive_period(1.0 / fc)
        }
        SourceSpec::Am {
            modulating_freq,
            carrier_freq,
            ..
        } => {
            let (_, fc) = am_frequencies(*modulating_freq, *carrier_freq, timing);
            positive_period(1.0 / fc)
        }
        other => fundamental_period(other, timing),
    }
}

/// A period a window can be divided by, or `None` for one that cannot.
fn positive_period(period: f64) -> Option<f64> {
    (period.is_finite() && period > 0.0).then_some(period)
}

/// The period this waveform repeats at, where it has one.
fn fundamental_period(spec: &SourceSpec, timing: PreviewTiming) -> Option<f64> {
    match spec {
        SourceSpec::Pulse { .. } => positive_period(pulse_timing(spec, timing)?.4),
        SourceSpec::Sin { frequency, .. } => {
            positive_period(1.0 / sin_frequency(*frequency, timing))
        }
        SourceSpec::Sffm {
            carrier_freq,
            modulation_index,
            signal_freq,
            ..
        } => {
            let (_, fm, _) =
                sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, timing);
            positive_period(1.0 / fm)
        }
        SourceSpec::Am {
            modulating_freq,
            carrier_freq,
            ..
        } => {
            let (fm, _) = am_frequencies(*modulating_freq, *carrier_freq, timing);
            positive_period(1.0 / fm)
        }
        SourceSpec::Pat { sample, data, .. } => {
            positive_period(pattern_bits(data).len() as f64 * resolved_sample(*sample, timing))
        }
        _ => None,
    }
}

/// A stored shape over its own fit span, at the density a list mini draws at.
///
/// A list row shows what a definition *is*, not what one run makes of it: a
/// supply brownout that takes forty milliseconds is a flat line over a
/// millisecond transient, and a row of flat lines tells a reader nothing about
/// which definition they want. The instrument's Fit view and this are the same
/// window, so the picture in the list is the picture the row opens.
///
/// The refusals are the instrument's too, carried through unrewritten.
pub(crate) fn shape_trace(
    component: &Component,
    samples: usize,
    timing: PreviewTiming,
) -> Result<WaveformTrace, String> {
    let spec = source_spec(component)?;
    if let Some(defect) = preview_defect(&spec) {
        return Err(defect);
    }
    let span = fit_span(transient_part(&spec), timing);
    let stop = if span.is_finite() && span > 0.0 {
        span
    } else {
        timing.tstop
    };
    Ok(sample_trace(
        &spec,
        PreviewWindow {
            start: 0.0,
            stop,
            samples,
        },
        timing,
    ))
}

/// The variable a `{NAME}` level names, if the text is one.
///
/// A level held by a design variable is the ordinary case rather than a
/// defect: the netlister resolves it at elaboration, and only a preview parsing
/// against an empty scope cannot. Surfaces that draw a mark where a curve would
/// be ask this, so that ordinary case never wears a warning.
pub(crate) fn design_variable(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let inner = trimmed.strip_prefix('{')?.strip_suffix('}')?.trim();
    (!inner.is_empty()).then(|| inner.to_owned())
}

/// Whether any field of this definition is held by a design variable.
///
/// Read off the authored text, because that is all there is at this boundary:
/// the parser refuses a braced field against an empty scope without saying
/// that a brace was why.
pub(crate) fn names_design_variable(record: &StimulusDefinition) -> bool {
    record.value.contains('{') || record.params.contains('{')
}

/// How much of this waveform you have to see before you have seen it.
fn fit_span(spec: &SourceSpec, timing: PreviewTiming) -> f64 {
    let stop = timing.tstop;
    match spec {
        SourceSpec::Pulse {
            width_defaults_to_zero,
            pulse_count,
            ..
        } => {
            let Some((delay, rise, fall, _, period)) = pulse_timing(spec, timing) else {
                return stop;
            };
            let lead = delay.max(0.0);
            if *width_defaults_to_zero {
                return lead + ((rise + fall) * 4.0).min(stop);
            }
            let cycles = if *pulse_count > 0.0 {
                pulse_count.min(3.0) + 0.3
            } else {
                2.0
            };
            lead + (cycles * period).min(stop)
        }
        SourceSpec::Sin {
            frequency, delay, ..
        } => delay.max(0.0) + 3.0 / sin_frequency(*frequency, timing),
        SourceSpec::Exp {
            td1,
            tau1,
            td2,
            tau2,
            ..
        } => {
            let (td1, tau1, td2, tau2) = exp_timing(*td1, *tau1, *td2, *tau2, timing);
            (td2 + 6.0 * tau2).max(td1 + 6.0 * tau1)
        }
        SourceSpec::Pwl {
            points,
            delay,
            repeat_from,
        } => {
            let Some((last, _)) = points.last() else {
                return stop;
            };
            let folds = repeat_from.is_some_and(|knot| knot < *last);
            (delay.max(0.0) + last) * if folds { 2.5 } else { 1.05 }
        }
        SourceSpec::Pat {
            delay,
            sample,
            data,
            repeat_count,
            ..
        } => {
            let pattern = pattern_bits(data).len().max(1) as f64 * resolved_sample(*sample, timing);
            let cycles = if *repeat_count < 0 {
                2.2
            } else {
                1.0 + f64::from(*repeat_count).clamp(0.0, 1.5)
            };
            delay.max(0.0) + pattern * cycles * 1.08
        }
        SourceSpec::Sffm {
            carrier_freq,
            modulation_index,
            signal_freq,
            delay,
            ..
        } => {
            let (_, fm, _) =
                sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, timing);
            delay.max(0.0) + 2.0 / fm
        }
        SourceSpec::Am {
            modulating_freq,
            carrier_freq,
            delay,
            ..
        } => {
            let (fm, _) = am_frequencies(*modulating_freq, *carrier_freq, timing);
            delay.max(0.0) + 2.5 / fm
        }
        SourceSpec::TrNoise { nt, .. } => {
            if nt.is_finite() && *nt > 0.0 {
                (nt * 240.0).min(stop)
            } else {
                stop
            }
        }
        SourceSpec::TrRandom {
            sample_interval,
            delay,
            ..
        } => delay.max(0.0) + resolved_sample(*sample_interval, timing) * 40.0,
        _ => stop,
    }
}

/// The breakpoints this family names, at the times the engine resolved.
fn guides(spec: &SourceSpec, timing: PreviewTiming) -> Vec<Guide> {
    let mut guides = Vec::new();
    match spec {
        SourceSpec::Pulse { pulse_count, .. } => {
            let Some((delay, rise, fall, width, period)) = pulse_timing(spec, timing) else {
                return guides;
            };
            guides.push(Guide {
                time: delay,
                label: "TD",
            });
            guides.push(Guide {
                time: delay + rise,
                label: "TR",
            });
            guides.push(Guide {
                time: delay + rise + width,
                label: "PW",
            });
            guides.push(Guide {
                time: delay + rise + width + fall,
                label: "TF",
            });
            guides.push(Guide {
                time: delay + period,
                label: "PER",
            });
            if *pulse_count > 0.0 {
                guides.push(Guide {
                    time: delay + pulse_count * period,
                    label: "NP·PER",
                });
            }
        }
        SourceSpec::Exp {
            td1,
            tau1,
            td2,
            tau2,
            ..
        } => {
            let (td1, _, td2, _) = exp_timing(*td1, *tau1, *td2, *tau2, timing);
            guides.push(Guide {
                time: td1,
                label: "TD1",
            });
            guides.push(Guide {
                time: td2,
                label: "TD2",
            });
        }
        SourceSpec::Sin { delay, .. }
        | SourceSpec::Sffm { delay, .. }
        | SourceSpec::Am { delay, .. }
        | SourceSpec::TrRandom { delay, .. } => guides.push(Guide {
            time: *delay,
            label: "TD",
        }),
        SourceSpec::Pwl {
            points,
            delay,
            repeat_from,
        } => {
            guides.push(Guide {
                time: *delay,
                label: "TD",
            });
            if let (Some(knot), Some((last, _))) = (repeat_from, points.last())
                && knot < last
            {
                guides.push(Guide {
                    time: delay + knot.max(points[0].0),
                    label: "R",
                });
            }
        }
        SourceSpec::Pat {
            delay,
            sample,
            data,
            ..
        } => {
            guides.push(Guide {
                time: *delay,
                label: "TD",
            });
            guides.push(Guide {
                time: delay + pattern_bits(data).len() as f64 * resolved_sample(*sample, timing),
                label: "PAT",
            });
        }
        _ => {}
    }
    guides
}

/// Everything the waveform's own timing says happens inside a window.
pub(super) struct Breakpoints {
    /// The times, clipped to the window, in no particular order.
    pub times: Vec<f64>,
    /// Whether this family has more corners in this window than the caller
    /// asked for. A window carrying more corners than a plot has columns
    /// cannot be drawn as a polyline at all, which is what makes it an
    /// envelope by construction rather than a curve with corners missing.
    pub saturated: bool,
}

/// Where this waveform changes direction or value inside `[from, to]`.
///
/// These are the engine's own resolved times — the edges a transient schedules
/// its own breakpoints at — and they exist so that a preview never has to guess
/// one. A 1 ns edge in a 100 µs window falls between two columns of any grid a
/// plot can afford, and the sample that lands nearest it draws a level the
/// source held for a nanosecond as if it had held it for a microsecond.
///
/// `budget` bounds the work and the answer both: a family with more corners
/// here than the budget reports `saturated`, and stops looking.
///
/// Two limits are deliberate. A `PWL FILE=` table's knots live behind the
/// engine's own loader cache and are not reachable from here, so that family
/// reports none. A repeating `PWL`'s later passes are not enumerated either:
/// where the seam falls is `rspice_core`'s geometry, and restating it up here
/// would be a second answer to a question the engine already answers.
pub(super) fn breakpoints_between(
    spec: &SourceSpec,
    timing: PreviewTiming,
    from: f64,
    to: f64,
    budget: usize,
) -> Breakpoints {
    let mut collector = Collector::new(from, to, budget);
    match spec {
        SourceSpec::Pulse { pulse_count, .. } => {
            if let Some((delay, rise, fall, width, period)) = pulse_timing(spec, timing) {
                collector.corner(delay);
                if period.is_finite() && period > 0.0 {
                    let ends = if *pulse_count > 0.0 {
                        delay + pulse_count * period
                    } else {
                        f64::INFINITY
                    };
                    let mut cycle = ((from - delay) / period).floor().max(0.0);
                    while !collector.full() {
                        let base = delay + cycle * period;
                        if base > to || base > ends {
                            break;
                        }
                        collector.corner(base);
                        collector.corner(base + rise);
                        collector.corner(base + rise + width);
                        collector.corner(base + rise + width + fall);
                        cycle += 1.0;
                    }
                    if *pulse_count > 0.0 {
                        collector.corner(ends);
                    }
                }
            }
        }
        SourceSpec::Exp {
            td1,
            tau1,
            td2,
            tau2,
            ..
        } => {
            let (td1, _, td2, _) = exp_timing(*td1, *tau1, *td2, *tau2, timing);
            collector.corner(td1);
            collector.corner(td2);
        }
        SourceSpec::Sin { delay, .. }
        | SourceSpec::Sffm { delay, .. }
        | SourceSpec::Am { delay, .. } => collector.step(*delay),
        SourceSpec::Pwl { points, delay, .. } => {
            // The source is exactly zero until TD, so a table whose first
            // level is not zero opens with a jump rather than a corner.
            if let Some((first, value)) = points.first()
                && *value != 0.0
            {
                collector.step(delay + first);
            }
            let ahead = points.partition_point(|(time, _)| delay + time < from);
            for (time, _) in &points[ahead..] {
                if collector.full() || delay + time > to {
                    break;
                }
                collector.corner(delay + time);
            }
        }
        SourceSpec::Pat {
            delay,
            rise,
            fall,
            sample,
            data,
            repeat_count,
            ..
        } => {
            let sample = resolved_sample(*sample, timing);
            let bits = pattern_bits(data).len() as f64;
            if sample.is_finite() && sample > 0.0 && bits > 0.0 {
                let ends = if *repeat_count < 0 {
                    f64::INFINITY
                } else {
                    delay + bits * sample * (f64::from(*repeat_count) + 1.0)
                };
                let mut index = ((from - delay) / sample).floor().max(0.0);
                while !collector.full() {
                    let edge = delay + index * sample;
                    if edge > to || edge > ends {
                        break;
                    }
                    collector.corner(edge);
                    collector.corner(edge + rise);
                    collector.corner(edge + fall);
                    index += 1.0;
                }
            }
        }
        _ => {}
    }
    collector.finish()
}

/// Collects the breakpoints inside one interval, and stops looking once it has
/// more of them than the caller can use.
struct Collector {
    times: Vec<f64>,
    from: f64,
    to: f64,
    budget: usize,
    saturated: bool,
}

impl Collector {
    const fn new(from: f64, to: f64, budget: usize) -> Self {
        Self {
            times: Vec::new(),
            from,
            to,
            budget,
            saturated: false,
        }
    }

    /// A corner of a continuous waveform: one time, where the slope changes.
    fn corner(&mut self, time: f64) {
        if !time.is_finite() || time < self.from || time > self.to {
            return;
        }
        if self.times.len() >= self.budget {
            self.saturated = true;
            return;
        }
        self.times.push(time);
    }

    /// A step: the instant before the jump as well as the jump, so that the
    /// edge is drawn where it happens rather than as a ramp from wherever the
    /// grid last landed.
    fn step(&mut self, time: f64) {
        self.corner(just_before(time));
        self.corner(time);
    }

    /// Whether the budget is spent and nothing more is worth computing.
    const fn full(&self) -> bool {
        self.saturated
    }

    fn finish(self) -> Breakpoints {
        Breakpoints {
            times: self.times,
            saturated: self.saturated,
        }
    }
}

/// The instant before a discontinuity, at the finest separation the time
/// itself can carry. A jump at zero has no instant before it inside a window
/// that starts there, and the clip drops the negative result.
fn just_before(time: f64) -> f64 {
    time - (time.abs() * 8.0 * f64::EPSILON).max(f64::MIN_POSITIVE)
}

/// The authored points a PWL table marks on the plot.
fn markers(spec: &SourceSpec) -> Vec<PwlMarker> {
    match spec {
        SourceSpec::Pwl { points, delay, .. } => points
            .iter()
            .enumerate()
            .map(|(index, (time, value))| PwlMarker {
                time: delay + time,
                value: *value,
                index,
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// The numbers the strip states, per family.
fn readouts_of(
    record: &StimulusDefinition,
    spec: &SourceSpec,
    timing: PreviewTiming,
    span: f64,
) -> Vec<(String, String)> {
    let unit = unit(record);
    let row = |label: &str, value: String| (label.to_owned(), value);
    let mut rows = Vec::new();
    match spec {
        SourceSpec::Sin {
            amplitude,
            frequency,
            ..
        } => {
            let frequency = sin_frequency(*frequency, timing);
            rows.push(row("f", seconds_free(frequency, "Hz")));
            rows.push(row("T", seconds_free(1.0 / frequency, "s")));
            rows.push(row(
                &format!("{unit}pp"),
                seconds_free(2.0 * amplitude, unit),
            ));
        }
        SourceSpec::Pulse {
            v1,
            v2,
            width_defaults_to_zero,
            pulse_count,
            ..
        } => {
            let Some((delay, rise, fall, width, period)) = pulse_timing(spec, timing) else {
                return rows;
            };
            rows.push(row("Δ", seconds_free(v2 - v1, unit)));
            rows.push(row("PW", seconds_free(width, "s")));
            rows.push(row("PER", seconds_free(period, "s")));
            rows.push(row(
                "duty",
                if *width_defaults_to_zero {
                    "one triangle".to_owned()
                } else if period > 0.0 {
                    format!("{:.1} %", (width + (rise + fall) * 0.5) / period * 100.0)
                } else {
                    "held".to_owned()
                },
            ));
            if *pulse_count > 0.0 {
                rows.push(row(
                    "NP",
                    format!(
                        "×{pulse_count} · ends {}",
                        seconds_free(delay + pulse_count * period, "s")
                    ),
                ));
            }
        }
        SourceSpec::Pwl {
            points,
            repeat_from,
            ..
        } => {
            rows.push(row("points", points.len().to_string()));
            rows.push(row(
                "span",
                seconds_free(points.last().map_or(0.0, |(time, _)| *time), "s"),
            ));
            rows.push(row(
                "max slew",
                seconds_free(max_slew(points), &format!("{unit}/s")),
            ));
            rows.push(row(
                "R",
                match repeat_from {
                    Some(knot) => format!("from {}", seconds_free(*knot, "s")),
                    None => "off".to_owned(),
                },
            ));
        }
        SourceSpec::PwlFile {
            path, repeat_from, ..
        } => {
            rows.push(row("file", file_name(path)));
            rows.push(row(
                "R",
                match repeat_from {
                    Some(knot) => format!("from {}", seconds_free(*knot, "s")),
                    None => "off".to_owned(),
                },
            ));
        }
        SourceSpec::Pat {
            sample,
            data,
            repeat_count,
            ..
        } => {
            let bits = pattern_bits(data);
            let sample = resolved_sample(*sample, timing);
            rows.push(row("bits", bits.len().to_string()));
            rows.push(row("rate", seconds_free(1.0 / sample, "b/s")));
            rows.push(row(
                "pattern",
                seconds_free(bits.len() as f64 * sample, "s"),
            ));
            rows.push(row(
                "R",
                if *repeat_count < 0 {
                    "forever".to_owned()
                } else {
                    format!("×{}", repeat_count + 1)
                },
            ));
        }
        SourceSpec::Exp {
            v1,
            v2,
            td1,
            tau1,
            td2,
            tau2,
        } => {
            let (_, tau1, _, tau2) = exp_timing(*td1, *tau1, *td2, *tau2, timing);
            rows.push(row("τ rise", seconds_free(tau1, "s")));
            rows.push(row("τ fall", seconds_free(tau2, "s")));
            rows.push(row("Δ", seconds_free(v2 - v1, unit)));
        }
        SourceSpec::Sffm {
            carrier_freq,
            modulation_index,
            signal_freq,
            ..
        } => {
            let (fc, fm, mdi) =
                sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, timing);
            rows.push(row("FC", seconds_free(fc, "Hz")));
            rows.push(row("FM", seconds_free(fm, "Hz")));
            rows.push(row("MDI", format!("{mdi:.2}")));
        }
        SourceSpec::Am {
            modulation_offset,
            modulation_amplitude,
            modulating_freq,
            carrier_freq,
            ..
        } => {
            let (fm, fc) = am_frequencies(*modulating_freq, *carrier_freq, timing);
            rows.push(row("FC", seconds_free(fc, "Hz")));
            rows.push(row("FM", seconds_free(fm, "Hz")));
            rows.push(row(
                "depth",
                if *modulation_offset == 0.0 {
                    "suppressed carrier".to_owned()
                } else {
                    format!(
                        "{:.0} %",
                        modulation_amplitude / modulation_offset.abs() * 100.0
                    )
                },
            ));
        }
        SourceSpec::TrNoise { na, nt, .. } => {
            rows.push(row("RMS", seconds_free(*na, unit)));
            rows.push(row("NT", seconds_free(*nt, "s")));
            if nt.is_finite() && *nt > 0.0 {
                rows.push(row(
                    "samples / TRAN",
                    format!("{:.0}", (timing.tstop / nt).floor()),
                ));
            }
        }
        SourceSpec::TrRandom {
            distribution,
            sample_interval,
            parameter1,
            ..
        } => {
            rows.push(row("TYPE", distribution_label(*distribution).to_owned()));
            rows.push(row("TS", seconds_free(*sample_interval, "s")));
            rows.push(row(
                distribution_parameter_label(*distribution),
                if *distribution == 4 {
                    format!("{parameter1}")
                } else {
                    seconds_free(*parameter1, unit)
                },
            ));
        }
        SourceSpec::Dc(level) => rows.push(row("level", seconds_free(*level, unit))),
        _ => {}
    }
    if record.family() == StimulusFamily::Dc || record.family() == StimulusFamily::Ac {
        rows.extend(unresolved_readouts(record));
    }
    if rows.is_empty() {
        rows.push(row("window", seconds_free(span, "s")));
    }
    rows
}

/// The `PULSE` timing this transient resolves, in card order.
fn pulse_timing(spec: &SourceSpec, timing: PreviewTiming) -> Option<(f64, f64, f64, f64, f64)> {
    let SourceSpec::Pulse {
        delay,
        rise,
        fall,
        width,
        period,
        width_defaults_to_zero,
        ..
    } = spec
    else {
        return None;
    };
    Some(VoltageSources::resolve_pulse_timing_with_defaults(
        *delay,
        *rise,
        *fall,
        *width,
        *period,
        *width_defaults_to_zero,
        timing.tstep,
        timing.tstop,
        PREVIEW_DIALECT,
    ))
}

/// The `EXP` timing this transient resolves, in card order: TD1, TAU1, TD2,
/// TAU2. Every one of the four has a substitution the card does not show, and
/// the fit span, the guides, the readouts and the breakpoints all have to be
/// looking at the same four numbers.
fn exp_timing(
    td1: f64,
    tau1: f64,
    td2: f64,
    tau2: f64,
    timing: PreviewTiming,
) -> (f64, f64, f64, f64) {
    VoltageSources::resolve_exp_timing_with_defaults(
        td1,
        tau1,
        td2,
        tau2,
        timing.tstep,
        PREVIEW_DIALECT,
    )
}

fn sin_frequency(frequency: f64, timing: PreviewTiming) -> f64 {
    VoltageSources::resolve_sin_frequency_with_dialect(
        frequency,
        timing.tstep,
        timing.tstop,
        PREVIEW_DIALECT,
    )
}

fn sffm_parameters(
    carrier: f64,
    modulation_index: f64,
    signal: f64,
    timing: PreviewTiming,
) -> (f64, f64, f64) {
    VoltageSources::resolve_sffm_parameters_with_dialect(
        carrier,
        modulation_index,
        signal,
        timing.tstep,
        timing.tstop,
        PREVIEW_DIALECT,
    )
}

fn am_frequencies(modulating: f64, carrier: f64, timing: PreviewTiming) -> (f64, f64) {
    VoltageSources::resolve_am_frequencies_with_dialect(
        modulating,
        carrier,
        timing.tstep,
        timing.tstop,
        PREVIEW_DIALECT,
    )
}

/// A `PAT` or `TRRANDOM` sample interval, falling back to the analysis step
/// exactly as the engine's own timing defaults do.
fn resolved_sample(sample: f64, timing: PreviewTiming) -> f64 {
    if sample.is_finite() && sample > 0.0 {
        sample
    } else {
        timing.tstep
    }
}

/// The bits of a `PAT` pattern, without the leading `B` marker the card
/// carries.
fn pattern_bits(data: &str) -> &str {
    let trimmed = data.trim();
    trimmed
        .strip_prefix('B')
        .or_else(|| trimmed.strip_prefix('b'))
        .unwrap_or(trimmed)
}

/// The steepest segment of an authored point list.
fn max_slew(points: &[(f64, f64)]) -> f64 {
    points
        .windows(2)
        .filter_map(|pair| {
            let step = pair[1].0 - pair[0].0;
            (step > 0.0).then(|| (pair[1].1 - pair[0].1).abs() / step)
        })
        .fold(0.0_f64, f64::max)
}

/// The file's own name, which is what a one-line readout has room for.
fn file_name(path: &str) -> String {
    path.rsplit(['/', '\\'])
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(path)
        .to_owned()
}

/// What `TRRANDOM`'s integer TYPE selects.
///
/// The words are the property sheet's; the integer is what the card carries.
const fn distribution_label(distribution: u8) -> &'static str {
    match distribution {
        1 => "uniform",
        2 => "Gaussian",
        3 => "exponential",
        4 => "Poisson",
        _ => "unknown",
    }
}

/// What `PARAM1` means under each distribution.
const fn distribution_parameter_label(distribution: u8) -> &'static str {
    match distribution {
        1 => "range",
        2 => "σ",
        3 => "mean",
        4 => "λ",
        _ => "PARAM1",
    }
}

/// An engineering-formatted quantity with its unit, or a dash when the number
/// is not one a reader can act on.
///
/// Spelled the way the axis under it spells a tick (`1 kHz`, `2 mV`): the
/// readouts sit a few points above those ticks, and one number written two
/// ways in one band reads as two numbers.
fn seconds_free(value: f64, unit: &str) -> String {
    if !value.is_finite() {
        return "—".to_owned();
    }
    tick_with_unit(&format_engineering_display(value), unit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;
    use crate::workbench::state::PreviewSpan;

    fn definition(kind: ComponentType, value: &str, params: &str) -> StimulusDefinition {
        let mut definition = StimulusDefinition::new("probe", kind).expect("placeable");
        definition.value = value.to_owned();
        definition.params = params.to_owned();
        definition
    }

    fn timing(tstop: f64) -> PreviewTiming {
        PreviewTiming {
            tstep: tstop / 1000.0,
            tstop,
            from_analysis: true,
        }
    }

    fn readout(realization: &StimulusRealization, label: &str) -> String {
        realization
            .derived
            .iter()
            .find(|(name, _)| name == label)
            .map(|(_, value)| value.clone())
            .unwrap_or_else(|| panic!("no {label} readout in {:?}", realization.derived))
    }

    fn guide(realization: &StimulusRealization, label: &str) -> Option<f64> {
        realization
            .guides
            .iter()
            .find(|guide| guide.label == label)
            .map(|guide| guide.time)
    }

    /// The ngspice-46/47 pin, restated as a readout. `PULSE(V1 V2 TD TR TF)`
    /// resolves to PW = 0 and PER = TSTOP, and the strip has to say so rather
    /// than computing a duty cycle from the fields the card shows.
    #[test]
    fn an_edges_only_pulse_reads_zero_width_and_one_triangle() {
        let spec = rspice_core::netlist::parse_source_spec_text(
            "PULSE(0 5 0 1n 1n)",
            0,
            &rspice_core::netlist::ParamContext::new(),
        )
        .expect("spec");
        let timing = timing(1e-3);
        let (_, rise, fall, width, period) =
            pulse_timing(&spec, timing).expect("a pulse resolves its timing");

        assert_eq!(width, 0.0);
        assert!((period - timing.tstop).abs() < 1e-18);
        assert!((fit_span(&spec, timing) - (rise + fall) * 4.0).abs() < 1e-18);
    }

    #[test]
    fn an_authored_pulse_states_its_period_and_duty() {
        let record = definition(
            ComponentType::VoltageSourcePulse,
            "0",
            "v2=5 tr=1u tf=1u pw=500u per=1m",
        );
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-2));

        assert_eq!(readout(&realization, "PER"), "1 ms");
        assert_eq!(readout(&realization, "duty"), "50.1 %");
        assert_eq!(readout(&realization, "Δ"), "5 V");
        assert_eq!(realization.fundamental, Some(1e-3));
        assert_eq!(guide(&realization, "PER"), Some(1e-3));
    }

    /// An omitted SIN frequency is `1 / TSTOP`, so the strip's `f` is a
    /// property of the reading transient and not of the card.
    #[test]
    fn an_omitted_sine_frequency_reads_one_over_the_stop_time() {
        let spec = rspice_core::netlist::parse_source_spec_text(
            "SIN(0 1)",
            0,
            &rspice_core::netlist::ParamContext::new(),
        )
        .expect("spec");
        assert!((sin_frequency_of(&spec, timing(1e-3)) - 1e3).abs() < 1e-6);
        assert!((sin_frequency_of(&spec, timing(1e-1)) - 10.0).abs() < 1e-9);
    }

    fn sin_frequency_of(spec: &SourceSpec, timing: PreviewTiming) -> f64 {
        let SourceSpec::Sin { frequency, .. } = spec else {
            panic!("a SIN card parses as a SIN spec");
        };
        sin_frequency(*frequency, timing)
    }

    /// An SFFM carrier authored as zero is read as omitted and becomes
    /// `5 / TSTOP`; the modulating frequency's own default is `500 / TSTOP`.
    #[test]
    fn an_sffm_carrier_of_zero_reads_the_engine_substitution() {
        let record = definition(
            ComponentType::VoltageSourceSffm,
            "0",
            "va=1 fc=0 mdi=0 fm=0",
        );
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert_eq!(readout(&realization, "FC"), "5 kHz");
        assert_eq!(readout(&realization, "FM"), "500 kHz");
    }

    #[test]
    fn an_exponential_reads_the_time_constants_the_engine_substitutes() {
        let record = definition(ComponentType::VoltageSourceExp, "0", "v2=1 tau1=0 tau2=0");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert_eq!(readout(&realization, "τ rise"), "1 µs");
        assert_eq!(readout(&realization, "τ fall"), "1 µs");
    }

    #[test]
    fn a_pwl_table_reads_its_points_span_and_steepest_segment() {
        let record = definition(ComponentType::VoltageSourcePwl, "0 0 1m 5 6m 5", "");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-2));

        assert_eq!(readout(&realization, "points"), "3");
        assert_eq!(readout(&realization, "span"), "6 ms");
        assert_eq!(readout(&realization, "max slew"), "5 kV/s");
        assert_eq!(readout(&realization, "R"), "off");
        assert_eq!(realization.markers.len(), 3);
        assert_eq!(realization.markers[1].index, 1);
    }

    #[test]
    fn a_pattern_reads_its_bit_count_rate_and_repeat() {
        let record = definition(
            ComponentType::VoltageSourcePat,
            "5",
            "vlo=0 data=B00111 tsample=200u",
        );
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-2));

        assert_eq!(readout(&realization, "bits"), "5");
        assert_eq!(readout(&realization, "rate"), "5 kb/s");
        assert_eq!(readout(&realization, "pattern"), "1 ms");
        assert_eq!(realization.fundamental, Some(1e-3));
    }

    #[test]
    fn a_noise_train_states_its_defect_and_keeps_the_readouts_that_hold() {
        let record = definition(ComponentType::VoltageSourceNoise, "20u", "nt=1u");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert!(
            realization
                .defect
                .as_deref()
                .is_some_and(|defect| defect.contains("TRNOISE"))
        );
        assert_eq!(realization.trace, WaveformTrace::Curve(Vec::new()));
        assert_eq!(readout(&realization, "RMS"), "20 µV");
        assert_eq!(readout(&realization, "NT"), "1 µs");
    }

    #[test]
    fn a_random_source_names_its_distribution_and_that_parameters_meaning() {
        let record = definition(
            ComponentType::VoltageSourceRandom,
            "500u",
            "type=gaussian param1=2m param2=0",
        );
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert_eq!(readout(&realization, "TYPE"), "Gaussian");
        assert_eq!(readout(&realization, "TS"), "500 µs");
        assert_eq!(readout(&realization, "σ"), "2 mV");
    }

    /// A DC level holding a design variable cannot be parsed against an empty
    /// scope, and the realization says so rather than drawing a flat zero.
    #[test]
    fn a_design_variable_level_names_the_variable_and_states_it_cannot_resolve() {
        let record = definition(ComponentType::VoltageSource, "{VSUP}", "");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert!(realization.defect.is_some());
        assert_eq!(readout(&realization, "variable"), "VSUP");
    }

    #[test]
    fn a_dc_level_reads_its_level_and_has_no_fundamental() {
        let record = definition(ComponentType::VoltageSource, "1.8", "");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert_eq!(readout(&realization, "level"), "1.800 V");
        assert_eq!(realization.fundamental, None);
        assert_eq!(realization.span, 1e-3);
    }

    /// A transient window holding hundreds of cycles is measured, and the rules
    /// that label a family's breakpoints collapse to the ones the window can
    /// still tell apart: TD, TR, PW, TF and PER all land inside the first
    /// column of a five-hundred-cycle band, and five labels on one column is
    /// five labels on top of each other.
    #[test]
    fn a_many_cycle_transient_window_bands_the_curve_and_keeps_one_guide() {
        let record = definition(
            ComponentType::VoltageSourcePulse,
            "1",
            "v2=5 td=100u tr=1n tf=1n pw=1u per=2u",
        );
        let realization = StimulusRealization::of(&record, SpanChoice::Transient, timing(1e-3));

        assert_eq!(
            realization.trace.caption().as_deref(),
            Some("500 cycles \u{b7} envelope")
        );
        assert_eq!(realization.guides.len(), 1);
        // `100u` is a hundred times a millionth, which is not the same float as
        // `1e-4`; the guide is the engine's own resolved delay, so it is read
        // to within a picosecond rather than bit for bit.
        assert!(
            guide(&realization, "TD").is_some_and(|time| (time - 1e-4).abs() < 1e-12),
            "{:?}",
            realization.guides
        );
        let readouts = realization.readouts.expect("a band has readouts");
        assert_eq!((readouts.minimum, readouts.maximum), (1.0, 5.0));
    }

    /// The same definition over one period is a curve, and says nothing about
    /// cycles because there is one.
    #[test]
    fn one_period_of_the_same_train_is_a_curve() {
        let record = definition(
            ComponentType::VoltageSourcePulse,
            "1",
            "v2=5 td=100u tr=1n tf=1n pw=1u per=2u",
        );
        let realization = StimulusRealization::of(&record, SpanChoice::Period, timing(1e-3));

        assert_eq!(realization.span, 2e-6);
        assert_eq!(realization.trace.caption(), None);
    }

    /// Period falls back to the fit span for a family with no fundamental, so
    /// the button never produces an empty window.
    #[test]
    fn the_period_span_falls_back_to_the_fit_span_where_there_is_no_period() {
        let record = definition(ComponentType::VoltageSourceExp, "0", "v2=1 tau1=1u tau2=1u");
        let fit = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));
        let period = StimulusRealization::of(&record, SpanChoice::Period, timing(1e-3));

        assert_eq!(fit.span, period.span);
        assert_eq!(PreviewSpan::Period.label(), "Period");
    }
}
