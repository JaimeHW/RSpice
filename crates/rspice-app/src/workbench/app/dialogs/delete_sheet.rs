//! The confirmation a sheet delete asks for when it would move objects.
//!
//! A sheet that holds objects is deleted, under the catalog's
//! move-to-replacement policy, by moving them onto the sheet next to it. The
//! sheet strip states that outcome in its menu row before it is chosen; a
//! menu command has no such row, so the delete it runs stops here and names
//! the count and the destination instead of moving them silently.

use egui::{Context, Id};

use crate::state::SheetId;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::sheets::{self as sheet_actions, SheetDeletePlan};
use crate::workbench::app_state::AppState;

const PRIMARY: &str = "Delete sheet";

/// A delete that is waiting for its confirmation, as it was planned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PendingSheetDelete {
    pub(crate) sheet: SheetId,
    pub(crate) name: String,
    pub(crate) objects: usize,
    pub(crate) destination: SheetId,
    pub(crate) destination_name: String,
}

impl PendingSheetDelete {
    fn title(&self) -> String {
        format!("Delete sheet \u{201c}{}\u{201d}", self.name)
    }

    fn still_planned(&self, plan: &SheetDeletePlan) -> bool {
        matches!(
            plan,
            SheetDeletePlan::MovesObjects { objects, destination, .. }
                if *objects == self.objects && *destination == self.destination
        )
    }
}

pub(in crate::workbench) fn render_delete_sheet_confirmation(ctx: &Context, state: &mut AppState) {
    let Some(pending) = state.sheets.delete_sheet.clone() else {
        return;
    };
    let title = pending.title();
    let objects = pending.objects.to_string();
    let description = format!(
        "Deleting this sheet moves its {} object{} to \u{201c}{}\u{201d}.",
        pending.objects,
        if pending.objects == 1 { "" } else { "s" },
        pending.destination_name
    );
    let choice = CommandForm::new(&title, PRIMARY)
        .describe(&description)
        .status_fixed_at_open()
        .show(ctx, |rows| {
            rows.value("Objects on it", &objects);
            rows.value("Moved to", &pending.destination_name);
            None::<Id>
        });
    match choice {
        DialogChoice::Primary => {
            state.sheets.delete_sheet = None;
            confirm_sheet_delete(state, &pending);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => state.sheets.delete_sheet = None,
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// Run the confirmed delete, provided it still does what was confirmed.
///
/// Anything that changes the sheet's objects or its neighbour while the
/// confirmation is open changes the outcome, and a delete is not run on an
/// outcome nobody saw.
pub(in crate::workbench) fn confirm_sheet_delete(
    state: &mut AppState,
    pending: &PendingSheetDelete,
) {
    let plan = sheet_actions::plan_delete(&state.model, pending.sheet);
    if pending.still_planned(&plan) {
        let _ = sheet_actions::commit_delete_sheet(
            &mut state.model,
            &mut state.workbench,
            pending.sheet,
        );
    } else {
        state.prompt(format!(
            "Sheet \u{201c}{}\u{201d} changed while the delete waited for confirmation, so \
             nothing was deleted.",
            pending.name
        ));
    }
}
