//! SPICE PWL publication: one trace as a piecewise-linear source.
//!
//! A waveform recorded in one simulation drives another: the file is a
//! subcircuit holding one independent source whose PWL breakpoints are the
//! trace's retained samples. A voltage trace becomes a voltage source and a
//! current trace a current source, between the subcircuit's two ports:
//!
//! ```text
//! .include "OUT.inc"
//! X1 in 0 PWL_OUT
//! ```
//!
//! An inline `PWL(...)` card is the form every SPICE reads, where a
//! two-column `PWL FILE=` is not. Every time and value is spelled in the
//! shortest exponent notation that reads back as the same double, which is
//! what every SPICE number parser, this product's included, reads as a plain
//! number: a source run from this file passes through the retained samples
//! exactly.
//!
//! A PWL source has one value at each time, so a trace that holds two values
//! at one instant cannot be written and is refused at that instant; a sample
//! that repeats its predecessor exactly carries nothing and is dropped.
//!
//! # Written as it is read
//!
//! The file opens with a comment stating how many breakpoints it holds and
//! the times they span, which only the last sample settles. So it is written
//! in two passes over the trace on a Results job, a block at a time: the
//! first checks every sample and counts the breakpoints, the second writes
//! them. Neither holds the trace, so a run kept in a results store exports
//! whatever its length.

use std::io::Write;

use super::NO_ACTIVE_ANALYSIS_MESSAGE;
use super::job::ResultExportJob;
use super::members::{ExportFile, publish_export};
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;

const LABEL: &str = "SPICE PWL";
const EXTENSION: &str = "inc";
const MIME_TYPE: &str = "text/plain;charset=utf-8";

/// The analyses whose traces are waveforms in time, which a PWL source is.
pub(super) const fn carries_time_waveforms(analysis: AnalysisType) -> bool {
    matches!(
        analysis,
        AnalysisType::Transient | AnalysisType::TransientNoise | AnalysisType::Pss
    )
}

/// The traces of `analysis` a PWL source can be written from: real samples
/// on its time axis.
pub(super) fn pwl_traces(analysis: &AnalysisResult) -> impl Iterator<Item = &RetainedTrace> {
    carries_time_waveforms(analysis.analysis_type)
        .then_some(analysis.waveforms.iter())
        .into_iter()
        .flatten()
        .filter(|waveform| waveform.complex.is_none())
}

/// What a written source file holds.
pub(super) struct PwlFacts {
    pub subcircuit: String,
    pub source: char,
    pub points: usize,
}

/// The subcircuit name: the trace's name in the characters every SPICE
/// accepts in one. A name that had to lose characters to fit carries a digest
/// of the name it was made from, so two traces whose names read alike, `v(a)`
/// and `v_a`, never give two files one subcircuit name.
pub(super) fn subcircuit_name(trace: &str) -> String {
    let mut name = format!("PWL_{}", readable_name(trace));
    if !trace
        .chars()
        .all(|character| character.is_ascii_alphanumeric())
    {
        // FNV-1a over the exact name: stable across builds and platforms.
        let digest = trace.bytes().fold(0x811c_9dc5_u32, |hash, byte| {
            (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193)
        });
        name.push_str(&format!("_{digest:08x}"));
    }
    name
}

/// The trace's name in the characters every SPICE and every file system accept: what the
/// subcircuit is named after and the file is offered as.
fn readable_name(trace: &str) -> String {
    let mut name = String::new();
    let mut gap = false;
    for character in trace.chars() {
        if character.is_ascii_alphanumeric() {
            if gap && !name.is_empty() && !name.ends_with('_') {
                name.push('_');
            }
            name.push(character);
            gap = false;
        } else {
            gap = true;
        }
    }
    if name.is_empty() {
        name.push_str("TRACE");
    }
    name
}

/// Whether a trace of no stated unit is named as a current: `I(R1)`, or a
/// device's pin current as LTspice names one, `Ix(U1:a)`, `Ib(Q1)`.
fn named_as_current(name: &str) -> bool {
    let Some(rest) = name.strip_prefix(['I', 'i']) else {
        return false;
    };
    rest.trim_start_matches(|character: char| character.is_ascii_alphabetic())
        .starts_with('(')
}

/// Each sample of `trace` in order, a block at a time, handed to `visit` as (time, value).
fn each_sample(
    trace: &RetainedTrace,
    mut visit: impl FnMut(f64, f64) -> Result<(), String>,
) -> Result<(), String> {
    let rows = trace.x.len();
    let read = |values, window| read_window(trace, values, window);
    let mut first = 0;
    while first < rows {
        let end = first
            .saturating_add(rspice_results::waveform::BLOCK_SAMPLES)
            .min(rows);
        let times = read(&trace.x, first..end)?;
        let values = read(&trace.y, first..end)?;
        for (&time, &value) in times.iter().zip(values.iter()) {
            visit(time, value)?;
        }
        first = end;
    }
    Ok(())
}

/// Samples `window` of `values`, a column of `trace`: borrowed from memory, or one block read
/// from its store.
fn read_window<'a>(
    trace: &RetainedTrace,
    values: &'a rspice_results::waveform::WaveformValues,
    window: std::ops::Range<usize>,
) -> Result<std::borrow::Cow<'a, [f64]>, String> {
    values
        .blocks(window)
        .next()
        .transpose()
        .map_err(|failure| format!("'{}' could not be read: {failure}", trace.name))
        .map(Option::unwrap_or_default)
}

/// Whether `trace` has a PWL form, checked without reading a sample: one value at each time.
fn check_pwl(trace: &RetainedTrace) -> Result<(), String> {
    if trace.x.len() != trace.y.len() {
        return Err(format!(
            "'{}' holds {} times and {} values, so it has no PWL form.",
            trace.name,
            trace.x.len(),
            trace.y.len()
        ));
    }
    if trace.x.is_empty() {
        return Err(format!("'{}' holds no samples to export.", trace.name));
    }
    Ok(())
}

/// Write `trace` of the analysis `label` as a PWL source subcircuit to `out`: every sample
/// checked and the breakpoints counted in a first pass, written in a second.
pub(super) fn write_pwl(
    label: &str,
    trace: &RetainedTrace,
    out: &mut dyn Write,
) -> Result<PwlFacts, String> {
    check_pwl(trace)?;
    let current = trace.unit.as_deref() == Some("A")
        || (trace.unit.is_none() && named_as_current(&trace.name));
    let source = if current { 'I' } else { 'V' };
    let unit = if current { "A" } else { "V" };
    // The first pass: what the header states, and every refusal before a byte is written.
    let mut previous: Option<(f64, f64)> = None;
    let mut first = None;
    let mut points = 0_usize;
    each_sample(trace, |time, value| {
        if !time.is_finite() || !value.is_finite() {
            return Err(format!(
                "'{}' holds a non-finite sample at t = {time:e} s, which a PWL source cannot take.",
                trace.name
            ));
        }
        if let Some((previous_time, previous_value)) = previous {
            if time < previous_time {
                return Err(format!(
                    "'{}' steps back in time at t = {time:e} s; a PWL source needs increasing times.",
                    trace.name
                ));
            }
            if time == previous_time {
                if value.to_bits() == previous_value.to_bits() {
                    return Ok(());
                }
                return Err(format!(
                    "'{}' holds two values at t = {time:e} s ({previous_value:e} and {value:e}); a \
                     PWL source has one value at each time.",
                    trace.name
                ));
            }
        }
        first.get_or_insert(time);
        previous = Some((time, value));
        points += 1;
        Ok(())
    })?;
    let (Some(first), Some((last, _))) = (first, previous) else {
        return Err(format!("'{}' holds no samples to export.", trace.name));
    };
    let subcircuit = subcircuit_name(&trace.name);
    let mut out = std::io::BufWriter::with_capacity(1 << 16, out);
    let written = |error: std::io::Error| format!("the file could not be written: {error}");
    // Comment lines stay on one line each: a label is the reader's text.
    let one_line = |text: &str| text.replace(['\r', '\n'], " ");
    writeln!(
        out,
        "* SPICE PWL source: {} ({unit}) of '{}', {points} retained samples from {first:e} s to {last:e} s.",
        one_line(&trace.name),
        one_line(label),
    )
    .map_err(written)?;
    writeln!(
        out,
        "* Use: .include this file, then X<name> <p> <n> {subcircuit}"
    )
    .map_err(written)?;
    writeln!(out, ".subckt {subcircuit} p n").map_err(written)?;
    writeln!(out, "{source}1 p n PWL(").map_err(written)?;
    // The second pass: the breakpoints, a sample that repeats its predecessor dropped again.
    let mut previous: Option<(f64, f64)> = None;
    each_sample(trace, |time, value| {
        if previous.is_some_and(|(previous_time, previous_value)| {
            time == previous_time && value.to_bits() == previous_value.to_bits()
        }) {
            return Ok(());
        }
        previous = Some((time, value));
        writeln!(out, "+ {time:e} {value:e}").map_err(written)
    })?;
    writeln!(out, "+ )").map_err(written)?;
    writeln!(out, ".ends {subcircuit}").map_err(written)?;
    out.flush().map_err(written)?;
    Ok(PwlFacts {
        subcircuit,
        source,
        points,
    })
}

pub(super) fn export_pwl(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    trace: Option<&str>,
) -> Option<ResultExportJob> {
    let reads = state.results_reads();
    let chosen = match displayed.primary_analysis(reads) {
        Some(analysis) if !carries_time_waveforms(analysis.analysis_type) => Err(format!(
            "A PWL source is a waveform in time, and '{}' is a {} result. Choose a transient \
             analysis.",
            analysis.label,
            analysis.analysis_type.short_label()
        )),
        Some(analysis) => {
            // The form names the trace; without it, the first one on the plot.
            let chosen = match trace {
                Some(name) => pwl_traces(analysis).find(|waveform| waveform.name == name),
                None => super::exported_waveforms(
                    reads,
                    displayed.dataset_id,
                    analysis,
                    super::ExportedSignals::Plotted,
                )
                .into_iter()
                .find(|waveform| waveform.complex.is_none()),
            };
            match chosen {
                Some(waveform) => check_pwl(waveform).map(|()| (analysis, waveform)),
                None => Err(match trace {
                    Some(name) => format!("'{name}' is not a trace of '{}'.", analysis.label),
                    None => format!("No trace of '{}' is plotted to export.", analysis.label),
                }),
            }
        }
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let (label, waveform) = match chosen {
        Ok((analysis, waveform)) => (analysis.label.clone(), waveform.clone()),
        Err(message) => {
            state.prompt(message);
            return None;
        }
    };
    // Its time and value, read twice: once to check them and count the breakpoints, once to
    // write them.
    let samples = (waveform.x.len() as u64).saturating_mul(4);
    publish_export(
        state,
        io,
        ExportFile {
            label: LABEL,
            title: "Export SPICE PWL Source",
            default_name: format!("{}.{EXTENSION}", readable_name(&waveform.name)),
            filter_name: "SPICE PWL Source",
            extension: EXTENSION,
            mime_type: MIME_TYPE,
            text: false,
        },
        samples,
        move |out| {
            let facts = write_pwl(&label, &waveform, out)?;
            Ok(format!(
                "subcircuit {} with a {} source through {} breakpoints; .include it and \
                 instance X<name> <p> <n> {}",
                facts.subcircuit,
                if facts.source == 'I' {
                    "current"
                } else {
                    "voltage"
                },
                facts.points,
                facts.subcircuit
            ))
        },
    )
}
