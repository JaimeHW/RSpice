//! Where the proof surface looks, what it marks, and what it states.
//!
//! Everything here is a choice of *x-range* or a restatement of a number the
//! engine resolved. Nothing evaluates a waveform: the curve comes from
//! [`evaluate_waveform`], which is the transient's own evaluator, and every
//! substituted field — a pulse width the card omitted, a sine frequency that
//! defaults to `1 / TSTOP`, an exponential time constant that defaults to
//! TSTEP — is read back out of `rspice_core`'s resolvers rather than worked
//! out again up here.
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
    DETACHED_NETS, PREVIEW_DIALECT, PreviewTiming, PreviewWindow, WaveformReadouts,
    evaluate_waveform, preview_defect, source_spec, transient_part,
};
use crate::state::format_engineering;
use crate::state::stimulus_library::definition::{
    StimulusDefinition, StimulusFamily, StimulusKind,
};

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
    /// The curve, as the engine steps it.
    pub samples: Vec<(f64, f64)>,
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
                    samples: Vec::new(),
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
        let samples = if defect.is_some() {
            Vec::new()
        } else {
            evaluate_waveform(
                &spec,
                PreviewWindow {
                    start: 0.0,
                    stop: span,
                    samples: PREVIEW_SAMPLES,
                },
                timing.tstep,
                timing.tstop,
                PREVIEW_DIALECT,
            )
        };
        let readouts = WaveformReadouts::of(&samples);
        let mut guides = guides(transient, timing);
        guides.retain(|guide| guide.time > 0.0 && guide.time <= span && guide.time.is_finite());
        let mut markers = markers(transient);
        markers.retain(|marker| marker.time >= 0.0 && marker.time <= span);
        Self {
            span,
            fundamental,
            samples,
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

/// The variable a `{NAME}` level names, if the text is one.
fn design_variable(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let inner = trimmed.strip_prefix('{')?.strip_suffix('}')?.trim();
    (!inner.is_empty()).then(|| inner.to_owned())
}

/// The period this waveform repeats at, where it has one.
fn fundamental_period(spec: &SourceSpec, timing: PreviewTiming) -> Option<f64> {
    let positive = |period: f64| (period.is_finite() && period > 0.0).then_some(period);
    match spec {
        SourceSpec::Pulse { .. } => positive(pulse_timing(spec, timing)?.4),
        SourceSpec::Sin { frequency, .. } => positive(1.0 / sin_frequency(*frequency, timing)),
        SourceSpec::Sffm {
            carrier_freq,
            modulation_index,
            signal_freq,
            ..
        } => {
            let (_, fm, _) =
                sffm_parameters(*carrier_freq, *modulation_index, *signal_freq, timing);
            positive(1.0 / fm)
        }
        SourceSpec::Am {
            modulating_freq,
            carrier_freq,
            ..
        } => {
            let (fm, _) = am_frequencies(*modulating_freq, *carrier_freq, timing);
            positive(1.0 / fm)
        }
        SourceSpec::Pat { sample, data, .. } => {
            positive(pattern_bits(data).len() as f64 * resolved_sample(*sample, timing))
        }
        _ => None,
    }
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
            let (td1, tau1, td2, tau2) = VoltageSources::resolve_exp_timing_with_defaults(
                *td1,
                *tau1,
                *td2,
                *tau2,
                timing.tstep,
                PREVIEW_DIALECT,
            );
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
            let (td1, _, td2, _) = VoltageSources::resolve_exp_timing_with_defaults(
                *td1,
                *tau1,
                *td2,
                *tau2,
                timing.tstep,
                PREVIEW_DIALECT,
            );
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
            rows.push(row("max slew", seconds_free(max_slew(points), unit)));
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
            let (_, tau1, _, tau2) = VoltageSources::resolve_exp_timing_with_defaults(
                *td1,
                *tau1,
                *td2,
                *tau2,
                timing.tstep,
                PREVIEW_DIALECT,
            );
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
fn seconds_free(value: f64, unit: &str) -> String {
    if !value.is_finite() {
        return "—".to_owned();
    }
    format!("{}{unit}", format_engineering(value))
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

        assert_eq!(readout(&realization, "PER"), "1ms");
        assert_eq!(readout(&realization, "duty"), "50.1 %");
        assert_eq!(readout(&realization, "Δ"), "5V");
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

        assert_eq!(readout(&realization, "FC"), "5kHz");
        assert_eq!(readout(&realization, "FM"), "500kHz");
    }

    #[test]
    fn an_exponential_reads_the_time_constants_the_engine_substitutes() {
        let record = definition(ComponentType::VoltageSourceExp, "0", "v2=1 tau1=0 tau2=0");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-3));

        assert_eq!(readout(&realization, "τ rise"), "1us");
        assert_eq!(readout(&realization, "τ fall"), "1us");
    }

    #[test]
    fn a_pwl_table_reads_its_points_span_and_steepest_segment() {
        let record = definition(ComponentType::VoltageSourcePwl, "0 0 1m 5 6m 5", "");
        let realization = StimulusRealization::of(&record, SpanChoice::Fit, timing(1e-2));

        assert_eq!(readout(&realization, "points"), "3");
        assert_eq!(readout(&realization, "span"), "6ms");
        assert_eq!(readout(&realization, "max slew"), "5kV");
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
        assert_eq!(readout(&realization, "rate"), "5kb/s");
        assert_eq!(readout(&realization, "pattern"), "1ms");
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
        assert!(realization.samples.is_empty());
        assert_eq!(readout(&realization, "RMS"), "20uV");
        assert_eq!(readout(&realization, "NT"), "1us");
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
        assert_eq!(readout(&realization, "TS"), "500us");
        assert_eq!(readout(&realization, "σ"), "2mV");
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

        assert_eq!(readout(&realization, "level"), "1.800V");
        assert_eq!(realization.fundamental, None);
        assert_eq!(realization.span, 1e-3);
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
