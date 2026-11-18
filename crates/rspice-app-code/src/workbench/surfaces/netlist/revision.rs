//! Generated-revision comparison.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::MessageId;
use crate::workbench::code::CodeFeature;
use crate::workbench::framework::feature::Cx;

pub(super) fn comparison_dialog_window(ctx: &egui::Context, cx: &mut Cx<'_, CodeFeature>) {
    if !cx.state.netlist.comparison_dialog.open {
        return;
    }
    let history_len = cx.state.netlist.generated_history.len();
    if history_len == 0 || cx.model.netlist_artifacts.generated_document.is_none() {
        cx.state.netlist.comparison_dialog.open = false;
        return;
    }
    let mut dialog = cx.state.netlist.comparison_dialog.clone();
    let messages = cx.wb.messages();
    dialog.selected_history_index = dialog.selected_history_index.min(history_len - 1);
    let revision_label = |artifact: &crate::state::GeneratedArtifact| {
        let revision = artifact.provenance().input().revision().get().to_string();
        let digest = artifact
            .content_digest()
            .to_string()
            .chars()
            .take(12)
            .collect::<String>();
        messages.format(
            MessageId::NetlistGeneratedRevisionChoice,
            &[("revision", &revision), ("digest", &digest)],
        )
    };
    let selected_label =
        revision_label(&cx.state.netlist.generated_history[dialog.selected_history_index]);
    let transaction = Dialog::new(
        messages.text(MessageId::NetlistComparisonGeneratedTitle),
        messages.text(MessageId::NetlistCompareRevisions),
    )
    .description(messages.text(MessageId::NetlistComparisonGeneratedDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel));
    let choice = transaction.show_with_initial_body_focus(ctx, |ui| {
        ui.label(messages.text(MessageId::NetlistComparisonGeneratedPrompt));
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistPriorRevision));
        let revision =
            crate::ui::widgets::Combo::from_id_salt("rspice.code.compare-revision-select")
                .selected_text(selected_label)
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    for (index, artifact) in
                        cx.state.netlist.generated_history.iter().enumerate().rev()
                    {
                        ui.selectable_value(
                            &mut dialog.selected_history_index,
                            index,
                            revision_label(artifact),
                        );
                    }
                });
        Some(revision.response.id)
    });
    match choice {
        DialogChoice::Primary => {
            match crate::workbench::documents::netlist_document::compare_generated_revision(
                cx.model,
                &mut cx.state.netlist,
                cx.runs,
                dialog.selected_history_index,
            ) {
                Ok(()) => dialog.open = false,
                Err(error) => cx.wb.prompt(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    cx.state.netlist.comparison_dialog = dialog;
}
