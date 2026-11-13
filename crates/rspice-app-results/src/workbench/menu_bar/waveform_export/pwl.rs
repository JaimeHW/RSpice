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

use super::{NO_ACTIVE_ANALYSIS_MESSAGE, note_result_export_failure, note_result_export_success};
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

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

/// One encoded source file.
pub(super) struct PwlSource {
    pub text: String,
    pub subcircuit: String,
    pub source: char,
    pub points: usize,
}

/// The subcircuit name: the trace's name in the characters every SPICE
/// accepts in one.
fn subcircuit_name(trace: &str) -> String {
    let mut name = String::from("PWL_");
    let mut gap = false;
    for character in trace.chars() {
        if character.is_ascii_alphanumeric() {
            if gap && !name.ends_with('_') {
                name.push('_');
            }
            name.push(character);
            gap = false;
        } else {
            gap = true;
        }
    }
    if name.len() == 4 {
        name.push_str("TRACE");
    }
    name
}

/// Write `trace` of `analysis` as a PWL source subcircuit.
pub(super) fn encode_pwl(
    analysis: &AnalysisResult,
    trace: &RetainedTrace,
) -> Result<PwlSource, String> {
    use std::fmt::Write as _;

    let current = trace.unit.as_deref() == Some("A")
        || (trace.unit.is_none()
            && trace.name.starts_with(['I', 'i'])
            && trace.name[1..].starts_with('('));
    let source = if current { 'I' } else { 'V' };
    let unit = if current { "A" } else { "V" };
    if trace.x.len() != trace.y.len() {
        return Err(format!(
            "'{}' holds {} times and {} values, so it has no PWL form.",
            trace.name,
            trace.x.len(),
            trace.y.len()
        ));
    }
    let mut points: Vec<(f64, f64)> = Vec::with_capacity(trace.x.len());
    for (&time, &value) in trace.x.iter().zip(trace.y.iter()) {
        if !time.is_finite() || !value.is_finite() {
            return Err(format!(
                "'{}' holds a non-finite sample at t = {time:e} s, which a PWL source cannot take.",
                trace.name
            ));
        }
        if let Some(&(previous_time, previous_value)) = points.last() {
            if time < previous_time {
                return Err(format!(
                    "'{}' steps back in time at t = {time:e} s; a PWL source needs increasing times.",
                    trace.name
                ));
            }
            if time == previous_time {
                if value.to_bits() == previous_value.to_bits() {
                    continue;
                }
                return Err(format!(
                    "'{}' holds two values at t = {time:e} s ({previous_value:e} and {value:e}); a \
                     PWL source has one value at each time.",
                    trace.name
                ));
            }
        }
        points.push((time, value));
    }
    let (Some(&(first, _)), Some(&(last, _))) = (points.first(), points.last()) else {
        return Err(format!("'{}' holds no samples to export.", trace.name));
    };
    let subcircuit = subcircuit_name(&trace.name);
    let mut text = String::new();
    // Comment lines stay on one line each: a label is the reader's text.
    let one_line = |text: &str| text.replace(['\r', '\n'], " ");
    writeln!(
        text,
        "* SPICE PWL source: {} ({unit}) of '{}', {} retained samples from {first:e} s to {last:e} s.",
        one_line(&trace.name),
        one_line(&analysis.label),
        points.len()
    )
    .expect("writing to a String cannot fail");
    writeln!(
        text,
        "* Use: .include this file, then X<name> <p> <n> {subcircuit}"
    )
    .expect("writing to a String cannot fail");
    writeln!(text, ".subckt {subcircuit} p n").expect("writing to a String cannot fail");
    writeln!(text, "{source}1 p n PWL(").expect("writing to a String cannot fail");
    for (time, value) in &points {
        writeln!(text, "+ {time:e} {value:e}").expect("writing to a String cannot fail");
    }
    writeln!(text, "+ )").expect("writing to a String cannot fail");
    writeln!(text, ".ends {subcircuit}").expect("writing to a String cannot fail");
    Ok(PwlSource {
        text,
        subcircuit,
        source,
        points: points.len(),
    })
}

pub(super) fn export_pwl(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    trace: Option<&str>,
) {
    let reads = state.results_reads();
    let prepared = match displayed.primary_analysis(reads) {
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
                Some(waveform) => encode_pwl(analysis, waveform),
                None => Err(match trace {
                    Some(name) => format!("'{name}' is not a trace of '{}'.", analysis.label),
                    None => format!("No trace of '{}' is plotted to export.", analysis.label),
                }),
            }
        }
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(message) => {
            state.prompt(message);
            return;
        }
    };

    let default_name = format!(
        "{}.{EXTENSION}",
        prepared.subcircuit.trim_start_matches("PWL_")
    );
    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: "Export SPICE PWL Source",
        default_name: &default_name,
        filter_name: "SPICE PWL Source",
        filter_extensions: &[EXTENSION],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, EXTENSION);
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, prepared.text.as_bytes(), MIME_TYPE)
            });
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from(default_name), Err(error)),
    };
    match export {
        Ok(()) => note_result_export_success(
            state,
            LABEL,
            crate::workbench::workflows::export_workflow::export_completion_message(
                LABEL,
                &published_path,
                Some(format!(
                    "subcircuit {} with a {} source through {} breakpoints; .include it and \
                     instance X<name> <p> <n> {}",
                    prepared.subcircuit,
                    if prepared.source == 'I' {
                        "current"
                    } else {
                        "voltage"
                    },
                    prepared.points,
                    prepared.subcircuit
                )),
                io,
            ),
        ),
        Err(error) => {
            note_result_export_failure(state, format!("{LABEL} export failed: {error}"));
            state.report_failure(format!("{LABEL} export failed"), error.to_string());
        }
    }
}
