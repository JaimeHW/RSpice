//! The notes a view keeps on its strips: drawn from the view's annotations
//! each frame, and written back through the view's one writer when the
//! reader adds, moves, pins, rewrites or deletes one.

use rspice_results::result_presentation::WaveformPresentationKey;
use rspice_results_ui::session::{DocumentNote, NoteAnchor};
use rspice_results_ui::waves::notes::NoteEdit;

use super::{AnalysisPresentationKey, ResultsParts, view_edits};
use crate::product::ResultDocumentId;
use crate::results::visualization_document::{
    Annotation, AnnotationAnchor, DocumentEdit, EntityRef, PaneId, Trace, TypedValue,
};

/// The note `annotation` of the pane `pane_id` of the view `document_id`,
/// drawn on the strip of `analysis`, as the strip draws it. `resolve` finds
/// the drawn curve of one of the pane's traces; a note on a curve the strip
/// does not draw has nothing to stand on, and is `None`.
pub(crate) fn document_note(
    (document_id, pane_id, analysis): (ResultDocumentId, PaneId, AnalysisPresentationKey),
    annotation: &Annotation,
    traces: &[&Trace],
    mut resolve: impl FnMut(&Trace) -> Option<(AnalysisPresentationKey, WaveformPresentationKey)>,
) -> Option<DocumentNote> {
    let (analysis, anchor) = match &annotation.anchor {
        AnnotationAnchor::Pane {
            x_fraction,
            y_fraction,
            unit,
        } => (
            analysis,
            NoteAnchor::Frame {
                unit: unit.clone(),
                x: *x_fraction,
                y: *y_fraction,
            },
        ),
        AnnotationAnchor::Trace {
            trace_id,
            coordinate,
        } => {
            let trace = traces.iter().find(|trace| trace.id == *trace_id)?;
            let x = match coordinate {
                TypedValue::Real(x) => *x,
                TypedValue::Integer(x) => *x as f64,
                _ => return None,
            };
            let (analysis, anchor) = resolve(trace)?;
            (
                analysis,
                NoteAnchor::Trace {
                    anchor,
                    trace_name: trace.label.clone(),
                    x,
                },
            )
        }
        AnnotationAnchor::Data { x, y, unit } => (
            analysis,
            NoteAnchor::Data {
                unit: unit.clone(),
                x: *x,
                y: *y,
            },
        ),
    };
    Some(DocumentNote {
        document_id,
        pane_id,
        retained_id: annotation.id,
        analysis,
        anchor,
        text: annotation.text.clone(),
    })
}

/// Keep what the reader did to a note in the view that keeps its strip: the
/// saved document's pane, or the working view's pane for the strip's
/// analysis. One change is one step the view can undo.
pub fn edit_note(state: &mut ResultsParts<'_>, edit: NoteEdit) {
    let (document_id, pane_id) = match &edit {
        NoteEdit::Add { analysis, .. } => match state.results.session.pane_for(*analysis) {
            Some(context) => (context.document_id, context.pane_id),
            None => {
                state.prompt("No view keeps this strip, so the note was not added.".to_owned());
                return;
            }
        },
        NoteEdit::Set { note, .. } | NoteEdit::Delete { note } => (note.document_id, note.pane_id),
    };
    let Some(document) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
    else {
        return;
    };
    let revision = document.revision();
    let anchored = |anchor: NoteAnchor| -> Result<AnnotationAnchor, String> {
        Ok(match anchor {
            NoteAnchor::Frame { unit, x, y } => AnnotationAnchor::Pane {
                x_fraction: x,
                y_fraction: y,
                unit,
            },
            NoteAnchor::Data { unit, x, y } => AnnotationAnchor::Data { x, y, unit },
            NoteAnchor::Trace {
                anchor,
                trace_name,
                x,
            } => {
                let trace = document
                    .traces()
                    .iter()
                    .find(|trace| {
                        trace.pane_id == pane_id
                            && (trace.label == trace_name
                                || trace.source_signal() == Some(anchor.trace.source_name.as_str()))
                    })
                    .ok_or_else(|| format!("'{trace_name}' is not a trace this view keeps"))?;
                AnnotationAnchor::Trace {
                    trace_id: trace.id,
                    coordinate: TypedValue::Real(x),
                }
            }
        })
    };
    let edits = match edit {
        NoteEdit::Add { anchor, text, .. } => anchored(anchor).map(|anchor| {
            vec![DocumentEdit::AddAnnotation {
                pane_id,
                anchor,
                text,
            }]
        }),
        NoteEdit::Set { note, anchor, text } => anchored(anchor).map(|anchor| {
            vec![DocumentEdit::SetAnnotation {
                annotation_id: note.note_id,
                anchor,
                text,
            }]
        }),
        NoteEdit::Delete { note } => Ok(vec![DocumentEdit::Remove(EntityRef::Annotation(
            note.note_id,
        ))]),
    };
    let result = edits.and_then(|edits| {
        view_edits::apply(state, document_id, revision, edits)
            .map(|_| ())
            .map_err(|error| error.to_string())
    });
    if let Err(error) = result {
        state.prompt(format!("Could not keep the note: {error}"));
    }
}
