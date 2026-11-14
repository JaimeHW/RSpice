//! Owned-revision save and generated-revision comparison.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::MessageId;
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::CodeRequest;
use crate::workbench::design_system::{Problem, problem_line};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::framework::feature::Cx;

pub(super) fn comparison_dialog_window(ctx: &egui::Context, cx: &mut Cx<'_, CodeFeature>) {
    if !cx.state.netlist.comparison_dialog.open {
        return;
    }
    let owned = cx.state.netlist.active_document == ActiveNetlistDocument::OwnedSource
        && cx.state.netlist.active_dependency_identity.is_none();
    let history_len = if owned {
        cx.model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .map_or(0, |descriptor| descriptor.revision_history.len())
    } else {
        cx.state.netlist.generated_history.len()
    };
    let current_available = if owned {
        cx.state.netlist.owned_document.is_some()
    } else {
        cx.model.netlist_artifacts.generated_document.is_some()
    };
    if history_len == 0 || !current_available {
        cx.state.netlist.comparison_dialog.open = false;
        return;
    }
    let mut dialog = cx.state.netlist.comparison_dialog.clone();
    let messages = cx.wb.messages();
    dialog.selected_history_index = dialog.selected_history_index.min(history_len - 1);
    let selected_label = if owned {
        let Some(selected) = cx
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| {
                descriptor
                    .revision_history
                    .get(dialog.selected_history_index)
            })
        else {
            cx.state.netlist.comparison_dialog.open = false;
            return;
        };
        let revision = selected.document_revision.to_string();
        let digest = selected
            .content_digest
            .to_string()
            .chars()
            .take(12)
            .collect::<String>();
        messages.format(
            MessageId::NetlistOwnedRevisionChoice,
            &[
                ("revision", &revision),
                ("digest", &digest),
                ("message", &selected.message),
            ],
        )
    } else {
        let selected = &cx.state.netlist.generated_history[dialog.selected_history_index];
        let revision = selected.provenance().input().revision().get().to_string();
        let digest = selected
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
    let restore_enabled = owned
        && cx
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| {
                descriptor
                    .revision_history
                    .get(dialog.selected_history_index)
            })
            .zip(cx.state.netlist.owned_document.as_ref())
            .is_some_and(|(snapshot, current)| {
                snapshot.content_digest != current.content_digest()
                    || snapshot.dependencies != current.dependencies()
            });
    let mut transaction = Dialog::new(
        if owned {
            messages.text(MessageId::NetlistComparisonOwnedTitle)
        } else {
            messages.text(MessageId::NetlistComparisonGeneratedTitle)
        },
        messages.text(MessageId::NetlistCompareRevisions),
    )
    .description(if owned {
        messages.text(MessageId::NetlistComparisonOwnedDescription)
    } else {
        messages.text(MessageId::NetlistComparisonGeneratedDescription)
    })
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel));
    if owned {
        transaction = transaction
            .secondary(messages.text(MessageId::NetlistRestoreAsNewRevision))
            .secondary_enabled(restore_enabled);
    }
    let choice = transaction.show_with_initial_body_focus(ctx, |ui| {
        ui.label(if owned {
            messages.text(MessageId::NetlistComparisonOwnedPrompt)
        } else {
            messages.text(MessageId::NetlistComparisonGeneratedPrompt)
        });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistPriorRevision));
        let revision =
            crate::ui::widgets::Combo::from_id_salt("rspice.code.compare-revision-select")
                .selected_text(selected_label)
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    if owned {
                        if let Some(descriptor) =
                            cx.model.workspace.content.netlist_descriptor.as_ref()
                        {
                            for (index, snapshot) in
                                descriptor.revision_history.iter().enumerate().rev()
                            {
                                let revision = snapshot.document_revision.to_string();
                                let digest = snapshot
                                    .content_digest
                                    .to_string()
                                    .chars()
                                    .take(12)
                                    .collect::<String>();
                                let label = messages.format(
                                    MessageId::NetlistOwnedRevisionChoice,
                                    &[
                                        ("revision", &revision),
                                        ("digest", &digest),
                                        ("message", &snapshot.message),
                                    ],
                                );
                                ui.selectable_value(
                                    &mut dialog.selected_history_index,
                                    index,
                                    label,
                                );
                            }
                        }
                    } else {
                        for (index, artifact) in
                            cx.state.netlist.generated_history.iter().enumerate().rev()
                        {
                            let revision =
                                artifact.provenance().input().revision().get().to_string();
                            let digest = artifact
                                .content_digest()
                                .to_string()
                                .chars()
                                .take(12)
                                .collect::<String>();
                            let label = messages.format(
                                MessageId::NetlistGeneratedRevisionChoice,
                                &[("revision", &revision), ("digest", &digest)],
                            );
                            ui.selectable_value(&mut dialog.selected_history_index, index, label);
                        }
                    }
                });
        Some(revision.response.id)
    });
    match choice {
        DialogChoice::Primary => {
            let result = if owned {
                crate::workbench::documents::netlist_document::compare_owned_revision(
                    cx.model,
                    &mut cx.state.netlist,
                    dialog.selected_history_index,
                )
            } else {
                crate::workbench::documents::netlist_document::compare_generated_revision(
                    cx.model,
                    &mut cx.state.netlist,
                    dialog.selected_history_index,
                )
            };
            match result {
                Ok(()) => dialog.open = false,
                Err(error) => cx.wb.prompt(error),
            }
        }
        DialogChoice::Secondary => {
            cx.wb.outbox.push(CodeRequest::RestoreOwnedNetlistRevision);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None => {}
    }
    cx.state.netlist.comparison_dialog = dialog;
}

pub(super) fn owned_source_save_ready(
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
) -> bool {
    if netlist.active_document != ActiveNetlistDocument::OwnedSource
        || netlist.active_dependency_identity.is_some()
        || model.workspace.content.netlist_source.is_none()
    {
        return false;
    }
    let digest = crate::state::content_digest(&model.simulation.source.netlist_content);
    netlist.externally_saved_content_digest != Some(digest)
}

pub(super) fn save_source_dialog_window(ctx: &egui::Context, cx: &mut Cx<'_, CodeFeature>) {
    if !cx.state.netlist.save_dialog.open {
        return;
    }
    if cx.state.netlist.active_document != ActiveNetlistDocument::OwnedSource
        || cx.state.netlist.active_dependency_identity.is_some()
    {
        cx.state.netlist.save_dialog.open = false;
        return;
    }

    let mut dialog = cx.state.netlist.save_dialog.clone();
    let messages = cx.wb.messages();
    let current_digest = crate::state::content_digest(&cx.model.simulation.source.netlist_content);
    let browser_copy = cfg!(target_arch = "wasm32");
    let message_valid = browser_copy || {
        let message = dialog.message.trim();
        message.chars().count() <= 240 && !message.chars().any(char::is_control)
    };
    let needs_save =
        dialog.save_as || cx.state.netlist.externally_saved_content_digest != Some(current_digest);
    let primary_enabled = message_valid && needs_save;
    let footer_hint = if needs_save {
        messages.text(MessageId::NetlistSaveStaleNotice)
    } else {
        messages.text(MessageId::NetlistAlreadyPublished)
    };
    let choice = Dialog::new(
        if browser_copy {
            messages.text(MessageId::NetlistDownloadSourceCopy)
        } else if dialog.save_as {
            "Save owned source as".to_owned()
        } else {
            messages.text(MessageId::NetlistSaveTitle)
        },
        if browser_copy {
            messages.text(MessageId::NetlistDownloadSourceCopy)
        } else if dialog.save_as {
            "Save source as".to_owned()
        } else {
            messages.text(MessageId::NetlistSave)
        },
    )
    .description(messages.text(if browser_copy {
        MessageId::NetlistDownloadSourceDescription
    } else {
        MessageId::NetlistSaveDescription
    }))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(primary_enabled)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(footer_hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        let descriptor = cx.model.workspace.content.netlist_descriptor.as_ref();
        let artifact_name = descriptor.map_or_else(
            || messages.text(MessageId::NetlistOwnedSpiceSource),
            |value| value.artifact_name.clone(),
        );
        ui.label(
            egui::RichText::new(artifact_name).font(theme::mono(tokens::FS_1, FontWeight::Medium)),
        );
        ui.horizontal(|ui| {
            ui.label("Encoding");
            if dialog.save_as {
                crate::ui::widgets::Combo::from_id_salt("rspice.netlist.save-as-encoding")
                    .selected_text(dialog.encoding.label())
                    .show_ui(ui, |ui| {
                        for encoding in crate::state::NetlistTextEncoding::ALL {
                            ui.selectable_value(&mut dialog.encoding, encoding, encoding.label());
                        }
                    });
            } else {
                ui.monospace(dialog.encoding.label());
            }
        });
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(messages.text(MessageId::NetlistSaveValidatedNotice));
                if let Some(document) = cx.state.netlist.owned_document.as_ref() {
                    let digest = document
                        .generated_artifact()
                        .map_or_else(
                            || document.content_digest(),
                            |artifact| artifact.content_digest(),
                        )
                        .to_string()
                        .chars()
                        .take(12)
                        .collect::<String>();
                    let revision = document.revision().get().to_string();
                    ui.monospace(messages.format(
                        MessageId::NetlistGeneratedBaseRevision,
                        &[("digest", &digest), ("revision", &revision)],
                    ));
                }
                ui.label(messages.text(MessageId::NetlistSaveImmutableNotice));
            });
        let focus = if browser_copy {
            None
        } else {
            ui.add_space(8.0);
            ui.label(messages.text(MessageId::NetlistRevisionMessage));
            let revision_message = ui.add(
                egui::TextEdit::singleline(&mut dialog.message)
                    .desired_width(f32::INFINITY)
                    .char_limit(240),
            );
            if !message_valid {
                ui.colored_label(
                    t.color.err,
                    messages.text(MessageId::NetlistRevisionMessageInvalid),
                );
            }
            Some(revision_message.id)
        };
        if !needs_save {
            ui.weak(messages.text(MessageId::NetlistAlreadyPublished));
        }
        if let Some(error) = &dialog.error {
            // Not attached to a field: the publication itself was refused.
            problem_line(ui, Problem::Failure, error);
        }
        focus
    });
    match choice {
        DialogChoice::Primary => {
            if crate::workbench::workflows::netlist_workflow::save_owned_netlist_source(
                cx.model,
                &mut cx.state.netlist,
                cx.wb,
                cx.handles.export_io,
                dialog.save_as,
                dialog.encoding,
                &dialog.message,
            ) {
                dialog.open = false;
                dialog.save_as = false;
                dialog.error = None;
                dialog.message = "Update owned SPICE source".to_owned();
            } else {
                dialog.error = Some(
                    "Source publication did not complete; review the application log.".to_owned(),
                );
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    cx.state.netlist.save_dialog = dialog;
}
