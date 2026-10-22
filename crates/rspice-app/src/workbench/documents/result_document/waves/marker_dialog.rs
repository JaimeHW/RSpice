//! Host projection and transactional commit for marker-purpose editing.
use super::{MarkerSelector, cached_models};
use crate::ui::tokens::Tokens;
use crate::workbench::AppState;
use rspice_results_ui::waves::marker_dialog::{self, MarkerDialogResponse};

pub(in super::super) fn open(state: &mut AppState, selector: MarkerSelector) {
    marker_dialog::open(&mut state.results.results.session, selector);
}

pub(in super::super) fn show(ctx: &egui::Context, state: &mut AppState) {
    let Some(dialog) = marker_dialog::prepare(&mut state.results.results.session) else {
        return;
    };
    let presentation = state.result_presentation_policy();
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
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
                state.prompt(format!("Could not apply the marker edit: {error}"));
                state.results.results.session.marker_edit = Some(draft);
                return;
            }
            state.results.results.session.marker_edit = None;
        }
        MarkerDialogResponse::Cancel => state.results.results.session.marker_edit = None,
        MarkerDialogResponse::Editing(draft) => {
            state.results.results.session.marker_edit = Some(draft)
        }
    }
}
