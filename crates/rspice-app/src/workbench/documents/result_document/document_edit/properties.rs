//! The digits values are shown with and whether phase is drawn unwrapped.
//!
//! A project result document keeps its own, applied whenever it is shown.
//! Over a quick view they are the reader's own: the digits preference and
//! the session's phase setting.

use rspice_results_ui::document_dialogs::properties::PropertiesDraft;

use super::*;
use crate::results::visualization_document::VisualizationPresentationPolicy;
use crate::workbench::ScalarPreference;

/// Open Plot properties on what the Results workspace is showing.
pub(crate) fn open_properties(state: &mut AppState) {
    let draft = match active_document_id(state)
        .and_then(|document_id| state.workspace.content.visualization_document(document_id))
    {
        Some(document) => {
            let presentation = document.presentation();
            PropertiesDraft {
                significant_digits: presentation.significant_digits,
                phase_continuous: presentation.phase_continuous,
            }
        }
        None => PropertiesDraft {
            significant_digits: state
                .ui
                .preferences
                .result_presentation_policy()
                .displayed_significant_digits()
                .get(),
            phase_continuous: state.ui.results.session.phase_continuous,
        },
    };
    state.workbench.result_document_dialog = Some(ResultDocumentDialog::Properties(draft));
}

/// What the properties apply to: the open document, or every quick view.
pub(super) fn scope(state: &AppState) -> String {
    active_document_id(state)
        .and_then(|document_id| state.workspace.content.visualization_document(document_id))
        .map_or_else(
            || "Quick view".to_owned(),
            |document| document.title().to_owned(),
        )
}

/// Keep the draft with the open document, or as the reader's own over a
/// quick view. The form closes once they land.
pub(super) fn commit(
    state: &mut AppState,
    draft: PropertiesDraft,
) -> Result<Option<ResultDocumentDialog>, String> {
    if let Some(document_id) = active_document_id(state) {
        transact(
            state,
            document_id,
            vec![DocumentEdit::SetPresentation(
                VisualizationPresentationPolicy {
                    significant_digits: draft.significant_digits,
                    phase_continuous: draft.phase_continuous,
                },
            )],
        )?;
    } else {
        state
            .ui
            .preferences
            .set_scalar(
                ScalarPreference::DisplayedSignificantDigits,
                u32::from(draft.significant_digits),
            )
            .map_err(str::to_owned)?;
        state.ui.results.session.phase_continuous = draft.phase_continuous;
    }
    Ok(None)
}
