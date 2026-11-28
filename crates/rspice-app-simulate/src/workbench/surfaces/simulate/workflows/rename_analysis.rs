//! Naming an analysis, or returning it to its kind label.

use super::*;

/// Name one analysis instance.
///
/// The plan is asked what it would refuse *while the field is being typed in*,
/// so the refusal that would stop the commit is already on screen and the
/// primary control is already disabled. Committing then re-asks the plan: the
/// preview is a courtesy, and the plan remains the only authority.
pub(super) fn rename_analysis_dialog(
    ctx: &egui::Context,
    cx: &mut Cx<'_, SimulateFeature>,
    mut draft: RenameAnalysisDraft,
) {
    let (validation_error, clear_refusal, default_label, shown_as) =
        match cx.model.sim_setup.stable_analysis_plan() {
            Ok(plan) => (
                plan.instance_name_refusal(draft.instance_id, &draft.name)
                    .map(|refusal| refusal.to_string()),
                plan.instance_name_clear_refusal(draft.instance_id)
                    .map(|refusal| refusal.to_string()),
                plan.instance(draft.instance_id)
                    .map(|instance| instance.kind().label().to_owned()),
                plan.instance(draft.instance_id)
                    .map(|instance| instance.display_name().to_owned()),
            ),
            Err(error) => (Some(error.clone()), Some(error), None, None),
        };
    draft.validation_error = validation_error;
    // The dialog opens on the name already shown, so its untouched state is a
    // rename to the name the analysis already has. Committing that advances the
    // plan revision and stales every pinned preflight artifact — which is what
    // the Effect row promises — for no change at all, so the control that would
    // do it is disabled until the field says something different.
    let unchanged = shown_as.is_some_and(|shown| shown == draft.name.trim());
    let enabled = draft.validation_error.is_none() && !unchanged;
    // Giving the name back is offered only when it would be taken. A control
    // that is present and refused teaches the reader nothing; a disabled one
    // beside the reason teaches them why.
    let can_clear = clear_refusal.is_none();
    let default_note = match (clear_refusal, default_label) {
        (None, Some(label)) => {
            format!("Give up the name. This analysis would be shown as {label} again.")
        }
        (Some(reason), _) => reason,
        (None, None) => "This analysis is no longer in the plan.".to_owned(),
    };
    let choice = Dialog::new("Name analysis", "Apply name")
    .description(
        "Name this analysis instance so every receipt, job and result that reports it says which analysis it was, rather than repeating the kind of analysis it is.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .secondary("Use kind label")
    .secondary_enabled(can_clear)
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_setting_row(
            ui,
            "Name",
            "Unique within this plan, ignoring case.",
            |ui| {
                mono_input(ui, "Name", &mut draft.name, ui.available_width().min(330.0));
            },
        );
        workflow_setting_row(ui, "Use kind label", &default_note, |ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new(if can_clear { "Available" } else { "Unavailable" })
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(if can_clear {
                        t.color.text
                    } else {
                        t.color.text_faint
                    }),
            );
        });
        workflow_setting_row(ui, "Analysis", "Kind and stable identity.", |ui| {
            ui.label(
                egui::RichText::new(&draft.subject)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            );
        });
        workflow_setting_row(
            ui,
            "Effect",
            "A rename is a plan edit and advances the plan revision.",
            |ui| {
                // Typed like every other value in this dialog. A bare `ui.label`
                // fell back to egui's own body style, so one row in a column of
                // four was set in a different face.
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "Pinned preflight artifacts go stale; retained datasets are unchanged.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(Tokens::get(ui.ctx()).color.text),
                    )
                    .wrap(),
                );
            },
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    // Clearing is a different intent from renaming, so it takes a different
    // control and a different commit rather than being inferred from an empty
    // field. `finish_workflow_choice` treats a secondary press as "stay open",
    // which is why this is answered before delegating to it.
    if matches!(choice, DialogChoice::Secondary) {
        match commit_clear_analysis_name(cx, &draft) {
            Ok(message) => {
                cx.state.simulation_workflow = None;
                cx.wb.prompt(message);
            }
            Err(error) => {
                set_workflow_error(&mut draft, error);
                cx.state.simulation_workflow = Some(draft.into());
            }
        }
        return;
    }
    finish_workflow_choice(cx, choice, draft, commit_rename_analysis);
}

/// Return the analysis to its kind label, through the same funnel a rename
/// uses, so both earn a receipt and both invalidate preflight identically.
fn commit_clear_analysis_name(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &RenameAnalysisDraft,
) -> Result<String, String> {
    let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
    let receipt = cx
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .and_then(|plan| {
            plan.clear_instance_name(draft.instance_id)
                .map_err(|error| error.to_string())
        })?;
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    super::lifecycle::refresh_analysis_projections(cx);
    cx.state.active_analysis_instance = Some(draft.instance_id);
    let message = receipt.detail().to_owned();
    cx.model
        .record_plan_receipt(super::lifecycle::receipt_line(&receipt));
    Ok(message)
}

fn commit_rename_analysis(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &RenameAnalysisDraft,
) -> Result<String, String> {
    let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
    let receipt = cx
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .and_then(|plan| {
            plan.set_instance_name(draft.instance_id, &draft.name)
                .map_err(|error| error.to_string())
        })?;
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    super::lifecycle::refresh_analysis_projections(cx);
    // Selection is held by identity, and a rename does not mint one, so the
    // renamed analysis stays selected. Saying so here is what keeps a later
    // change from quietly moving selection to a position instead.
    cx.state.active_analysis_instance = Some(draft.instance_id);
    let message = receipt.detail().to_owned();
    cx.model
        .record_plan_receipt(super::lifecycle::receipt_line(&receipt));
    Ok(message)
}
