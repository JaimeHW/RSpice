//! Host projection and transactional commit for marker-purpose editing.
use super::{MarkerSelector, cached_models};
use crate::ui::tokens::Tokens;
use crate::workbench::AppState;
use rspice_results_ui::waves::marker_dialog::{self, MarkerDialogResponse};

pub(in super::super) fn open(state: &mut AppState, selector: MarkerSelector) {
    marker_dialog::open(&mut state.ui.results.session, selector);
}

pub(in super::super) fn show(ctx: &egui::Context, state: &mut AppState) {
    let Some(dialog) = marker_dialog::prepare(&mut state.ui.results.session) else {
        return;
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::get(ctx),
    );
    match marker_dialog::show(
        ctx,
        dialog,
        &models,
        presentation.readout(),
        quantity_policy,
    ) {
        MarkerDialogResponse::Apply(draft) => {
            if let Err(error) =
                super::super::commit_marker_edit(state, draft.selector, &draft.note, draft.kind)
            {
                state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                    "Could not retain marker edit: {error}"
                )));
                state.ui.results.session.marker_edit = Some(draft);
                return;
            }
            state.ui.results.session.marker_edit = None;
        }
        MarkerDialogResponse::Cancel => state.ui.results.session.marker_edit = None,
        MarkerDialogResponse::Editing(draft) => state.ui.results.session.marker_edit = Some(draft),
    }
}
