//! The style and the name a reader gives a trace, kept by the view that
//! draws its strip (the saved document's pane, or the run's working view)
//! and drawn from the frame they are given. The run keeps neither.

use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results_ui::session::WaveformLook;

use super::{AnalysisPresentationKey, ResultsParts, view_edits};
use crate::product::{ObjectRevision, ResultDocumentId};
use crate::results::visualization_document::{DocumentEdit, EntityRef, Trace, TraceId, TraceStyle};

/// What `trace` gives its signal beyond its colour, by the signal's name: its
/// style, and the name the reader gave it when they gave one.
pub(super) fn look_of(trace: &Trace) -> (String, WaveformLook) {
    (
        trace.source_signal().unwrap_or(&trace.label).to_owned(),
        trace_look(trace),
    )
}

/// How `trace` is drawn: its style, and the name it is given when that is
/// not its signal's.
pub(super) fn trace_look(trace: &Trace) -> WaveformLook {
    let name = trace
        .source_signal()
        .filter(|signal| *signal != trace.label)
        .map(|_| trace.label.clone());
    WaveformLook {
        style: trace.style,
        name,
    }
}

/// The view that keeps `analysis`' strip, and its traces that read `signal`
/// with what each holds now.
fn held_traces(
    state: &ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    signal: &str,
) -> Result<(ResultDocumentId, ObjectRevision, Vec<Trace>), String> {
    let context = state
        .results
        .session
        .pane_for(analysis)
        .ok_or_else(|| format!("No view keeps this strip, so '{signal}' keeps its style."))?;
    let document = state
        .model
        .workspace
        .content
        .visualization_document(context.document_id)
        .ok_or_else(|| "The view is no longer in the project.".to_owned())?;
    let held: Vec<Trace> = document
        .traces()
        .iter()
        .filter(|trace| {
            trace.pane_id == context.pane_id
                && trace.source_signal().unwrap_or(&trace.label) == signal
        })
        .cloned()
        .collect();
    if held.is_empty() {
        return Err(format!(
            "'{signal}' is not a trace this view keeps, so its style was not changed."
        ));
    }
    Ok((context.document_id, document.revision(), held))
}

/// Keep `edits` in `document`, one step of its undo history.
fn commit(
    state: &mut ResultsParts<'_>,
    document: ResultDocumentId,
    revision: ObjectRevision,
    edits: Vec<DocumentEdit>,
) -> Result<(), String> {
    if edits.is_empty() {
        return Ok(());
    }
    view_edits::apply(state, document, revision, edits)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

/// Draw `analysis`' traces of `signal` in `style` in the view that keeps the
/// strip, from this frame on.
pub fn set_trace_style(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    signal: &str,
    style: TraceStyle,
) -> Result<(), String> {
    let (document, revision, held) = held_traces(state, analysis, signal)?;
    let edits = held
        .iter()
        .filter(|trace| trace.style != style)
        .map(|trace| DocumentEdit::SetTraceStyle {
            trace_id: trace.id,
            style,
        })
        .collect();
    commit(state, document, revision, edits)?;
    let key = SourceWaveformPresentationKey::new(analysis, signal);
    let mut look = state
        .results
        .session
        .waveform_looks
        .get(&key)
        .cloned()
        .unwrap_or_default();
    look.style = style;
    state.results.draw_waveform_look(&key, look);
    Ok(())
}

/// Give `analysis`' traces of `signal` the name `name` in the view that keeps
/// the strip; an empty name gives them their signal's again.
pub fn rename_trace(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    signal: &str,
    name: &str,
) -> Result<(), String> {
    let name = name.trim();
    let value = if name.is_empty() { signal } else { name };
    let (document, revision, held) = held_traces(state, analysis, signal)?;
    // A trace the view lists by its signal's name alone keeps that name: it
    // is how the view finds the signal in the next run.
    let renamable: Vec<TraceId> = held
        .iter()
        .filter(|trace| trace.source_signal().is_some())
        .map(|trace| trace.id)
        .collect();
    if renamable.is_empty() {
        return Err(format!(
            "'{signal}' keeps its name: the view finds the signal by it."
        ));
    }
    let edits = held
        .iter()
        .filter(|trace| renamable.contains(&trace.id) && trace.label != value)
        .map(|trace| DocumentEdit::Rename {
            entity: EntityRef::Trace(trace.id),
            value: value.to_owned(),
        })
        .collect();
    commit(state, document, revision, edits)?;
    let key = SourceWaveformPresentationKey::new(analysis, signal);
    let mut look = state
        .results
        .session
        .waveform_looks
        .get(&key)
        .cloned()
        .unwrap_or_default();
    look.name = (value != signal).then(|| value.to_owned());
    state.results.draw_waveform_look(&key, look);
    Ok(())
}
