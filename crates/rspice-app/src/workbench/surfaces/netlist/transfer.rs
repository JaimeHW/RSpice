//! Source crossing the project boundary: external-change review, generated
//! export, and staged import review.
//!
//! Every one of these is a reviewed transaction. Nothing here writes project
//! bytes before the exact source identity it was opened against is confirmed
//! to still be current.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{CommandForm, Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::design_system::{Problem, problem_line};
use crate::workbench::{MessageCatalog, MessageId, RSpiceApp};

fn external_resolution_label(
    messages: MessageCatalog,
    resolution: crate::workbench::documents::netlist_document::NetlistExternalChangeResolution,
) -> String {
    use crate::workbench::documents::netlist_document::NetlistExternalChangeResolution;

    messages.text(match resolution {
        NetlistExternalChangeResolution::Merge => MessageId::NetlistExternalMerge,
        NetlistExternalChangeResolution::KeepLocal => MessageId::NetlistExternalKeepLocal,
        NetlistExternalChangeResolution::ReloadExternal => MessageId::NetlistExternalReload,
    })
}

pub(super) fn external_change_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::NetlistExternalChangeResolution;

    let Some(mut review) = app.state.code.netlist.external_change.clone() else {
        return;
    };
    let messages = app.state.workbench.messages();
    let digest = |bytes: &[u8; 32]| {
        bytes
            .iter()
            .take(6)
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    let hint = match review.resolution {
        NetlistExternalChangeResolution::Merge if review.merge_conflict_count == 0 => {
            messages.text(MessageId::NetlistExternalConflictFree)
        }
        NetlistExternalChangeResolution::Merge => {
            messages.text(MessageId::NetlistExternalConflictResolutionRequired)
        }
        NetlistExternalChangeResolution::KeepLocal => {
            messages.text(MessageId::NetlistExternalKeepLocalHint)
        }
        NetlistExternalChangeResolution::ReloadExternal => {
            messages.text(MessageId::NetlistExternalReloadHint)
        }
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistExternalTitle),
        messages.text(MessageId::NetlistExternalApply),
    )
    .description(messages.text(MessageId::NetlistExternalDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        ui.monospace(review.path.display().to_string());
        let expected = digest(&review.expected_sha256);
        let observed = digest(&review.observed_sha256);
        ui.weak(messages.format(
            MessageId::NetlistExternalEvidenceSummary,
            &[
                ("expected", &expected),
                ("observed", &observed),
                ("encoding", review.external_encoding.label()),
            ],
        ));
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistExternalResolution));
        let resolution = egui::ComboBox::from_id_salt("rspice.netlist.external-resolution")
            .selected_text(external_resolution_label(messages, review.resolution))
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for resolution in NetlistExternalChangeResolution::ALL {
                    ui.selectable_value(
                        &mut review.resolution,
                        resolution,
                        external_resolution_label(messages, resolution),
                    );
                }
            });
        ui.add_space(8.0);
        ui.label(egui::RichText::new(messages.text(MessageId::NetlistExternalEvidence)).strong());
        ui.label(if review.base_source.is_some() {
            let count = review.merge_conflict_count.to_string();
            messages.format(
                if review.merge_conflict_count == 1 {
                    MessageId::NetlistExternalMergeConflictsSingular
                } else {
                    MessageId::NetlistExternalMergeConflicts
                },
                &[("count", &count)],
            )
        } else {
            messages.text(MessageId::NetlistExternalNoBase)
        });
        let candidate = match review.resolution {
            NetlistExternalChangeResolution::Merge => &review.merged_source,
            NetlistExternalChangeResolution::KeepLocal => &review.local_source,
            NetlistExternalChangeResolution::ReloadExternal => &review.external_source,
        };
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(8)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("rspice.netlist.external-candidate")
                    .max_height(150.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (line, text) in candidate.lines().take(200).enumerate() {
                            ui.monospace(format!("{:>5}  {text}", line + 1));
                        }
                        if candidate.lines().nth(200).is_some() {
                            ui.weak(messages.text(MessageId::NetlistExternalPreviewLimited));
                        }
                    });
            });
        ui.add_space(8.0);
        ui.label(egui::RichText::new(messages.text(MessageId::NetlistExternalComparison)).strong());
        egui::ScrollArea::vertical()
            .id_salt("rspice.netlist.external-diff")
            .max_height(120.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for line in review.comparison.lines().take(200) {
                    ui.monospace(line);
                }
            });
        if let Some(error) = &review.error {
            ui.add_space(6.0);
            problem_line(ui, Problem::Failure, error);
        }
        Some(resolution.response.id)
    });

    app.state.code.netlist.external_change = Some(review);
    match choice {
        DialogChoice::Primary => {
            match crate::workbench::workflows::netlist_workflow::apply_staged_external_netlist_change(
                &mut app.state,
            ) {
                Ok(()) => {
                    app.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Applied the resolved external change to the SPICE source.",
                    );
                }
                Err(error) => {
                    if let Some(current) = app.state.code.netlist.external_change.as_mut() {
                        current.error = Some(error);
                    }
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.code.netlist.external_change = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// The dialects a generated deck exports in, as the Dialect select lists them.
const EXPORT_DIALECTS: [(crate::io::NetlistFormat, &str); 2] = [
    (crate::io::NetlistFormat::Spice, "SPICE"),
    (crate::io::NetlistFormat::Spectre, "Spectre"),
];

/// "1 included file", "3 included files".
fn files_phrase(count: usize) -> String {
    if count == 1 {
        "1 included file".to_owned()
    } else {
        format!("{count} included files")
    }
}

pub(super) fn export_generated_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.code.netlist.export_dialog.open {
        return;
    }
    let mut dialog = app.state.code.netlist.export_dialog.clone();
    let current = app.state.model.netlist_artifacts.generation_error.is_none()
        && app
            .state
            .model
            .netlist_artifacts
            .generated_document
            .is_some()
        && app.state.model.netlist_artifacts.generated_input_digest
            == app
                .state
                .model
                .netlist_artifacts
                .current_generation_input_digest;
    let (dependency_count, dependencies_sealed) = app
        .state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .map(|document| {
            (
                document.dependencies().len(),
                document.dependency_graph_is_sealed(),
            )
        })
        .unwrap_or_default();
    // A deck that includes other files exports them with it, as a .zip.
    let requires_bundle = dependency_count > 0;
    if requires_bundle {
        dialog.bundle_dependencies = true;
    }
    // Source-map positions are the SPICE deck's, before translation.
    let source_map_supported = dialog.format == crate::io::NetlistFormat::Spice;
    if !source_map_supported {
        dialog.include_source_map = false;
    }
    let messages = app.state.workbench.messages();
    let netlist = &app.state.model.netlist_artifacts;
    let refusal = if netlist.generation_error.is_some() {
        Some("The netlist could not be generated. Fix its problems, then export.".to_owned())
    } else if netlist.generated_document.is_none() {
        Some("No netlist has been generated yet.".to_owned())
    } else if !current {
        Some("The netlist changed since it was generated. Regenerate it, then export.".to_owned())
    } else if dialog.bundle_dependencies && !dependencies_sealed {
        Some(format!(
            "Resolve the deck's {} before exporting it.",
            files_phrase(dependency_count)
        ))
    } else {
        None
    };
    let error = dialog.error.clone();
    let status = match (error.as_deref(), refusal.as_deref()) {
        (Some(error), _) => Err(error),
        (None, Some(refusal)) => Err(refusal),
        (None, None) => Ok(None),
    };
    let dialects = EXPORT_DIALECTS.map(|(_, label)| label.to_owned());
    let bundle_label = messages.text(MessageId::NetlistExportDependencyBundle);
    let source_map_label = messages.text(MessageId::NetlistExportSourceMap);
    let bundled_includes = requires_bundle.then(|| {
        format!(
            "The deck includes {}, so it exports as a .zip bundle.",
            files_phrase(dependency_count)
        )
    });
    let before = (dialog.bundle_dependencies, dialog.include_source_map);
    let choice = CommandForm::new(
        &messages.text(MessageId::NetlistExportTitle),
        &messages.text(MessageId::NetlistExportDeck),
    )
    .option_columns()
    .describe(&messages.text(MessageId::NetlistExportDescription))
    .primary_enabled(refusal.is_none())
    .status(status)
    .show(ctx, |rows| {
        let current_dialect = EXPORT_DIALECTS
            .iter()
            .position(|(format, _)| *format == dialog.format)
            .unwrap_or_default();
        let dialect = rows.select_with_response(
            &messages.text(MessageId::NetlistDialect),
            "rspice.code.export-dialect",
            EXPORT_DIALECTS[current_dialect].1,
            &dialects,
        );
        if let Some((format, _)) = dialect
            .picked
            .and_then(|picked| EXPORT_DIALECTS.get(picked))
        {
            dialog.format = *format;
        }
        let mut items: Vec<(&str, &mut bool)> = Vec::new();
        if !requires_bundle {
            items.push((bundle_label.as_str(), &mut dialog.bundle_dependencies));
        }
        if source_map_supported {
            items.push((source_map_label.as_str(), &mut dialog.include_source_map));
        }
        if !items.is_empty() {
            rows.check_columns("Include", &mut items);
        }
        if let Some(bundled) = &bundled_includes {
            rows.derived(bundled);
        }
        Some(dialect.response.id)
    });
    // A source map travels in the bundle: asking for one asks for the
    // bundle, and leaving the bundle leaves the map.
    if dialog.include_source_map && !before.1 {
        dialog.bundle_dependencies = true;
    } else if !dialog.bundle_dependencies && before.0 {
        dialog.include_source_map = false;
    }
    if (dialog.bundle_dependencies, dialog.include_source_map) != before {
        dialog.error = None;
    }
    match choice {
        DialogChoice::Primary => {
            if crate::workbench::menu_bar::action_export_generated_netlist_with_options(
                &mut app.state,
                dialog.format,
                dialog.bundle_dependencies,
                dialog.include_source_map,
                app.export_workflow_io.as_ref(),
            ) {
                dialog.open = false;
                dialog.error = None;
            } else {
                dialog.error =
                    Some("Export did not complete; review the application log.".to_owned());
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.code.netlist.export_dialog = dialog;
}

fn import_operation_text(
    messages: MessageCatalog,
    operation: crate::workbench::documents::netlist_document::NetlistImportOperation,
) -> (String, String) {
    use crate::workbench::documents::netlist_document::NetlistImportOperation;

    match operation {
        NetlistImportOperation::OpenProject => (
            messages.text(MessageId::NetlistImportOpenTitle),
            messages.text(MessageId::NetlistImportOpen),
        ),
        NetlistImportOperation::ImportIntoProject => (
            messages.text(MessageId::NetlistImportDeckTitle),
            messages.text(MessageId::NetlistImportDeck),
        ),
        NetlistImportOperation::RequalifyOwnedSource => (
            messages.text(MessageId::NetlistImportReviewProfileTitle),
            messages.text(MessageId::NetlistImportRecordProfile),
        ),
    }
}

pub(super) fn import_review_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssueSeverity, NetlistImportOperation,
    };

    let Some(mut review) = app.state.code.netlist.import_review.clone() else {
        return;
    };
    let messages = app.state.workbench.messages();
    let blocking = review.blocking_issue_count();
    let dialect_qualification_error = review.dialect_qualification().err();
    let qualified_execution_profile = dialect_qualification_error
        .is_none()
        .then(|| review.selected_dialect.execution_profile())
        .flatten();
    let compatibility_ready =
        !review.selected_dialect.requires_compatibility_review() || review.compatibility_accepted;
    let primary_enabled =
        blocking == 0 && dialect_qualification_error.is_none() && compatibility_ready;
    let digest = review
        .original_sha256
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let hint = if blocking > 0 {
        let count = blocking.to_string();
        messages.format(
            if blocking == 1 {
                MessageId::NetlistImportBlockingSingular
            } else {
                MessageId::NetlistImportBlocking
            },
            &[("count", &count)],
        )
    } else if let Some(error) = dialect_qualification_error.as_deref() {
        error.to_owned()
    } else if !compatibility_ready {
        messages.text(MessageId::NetlistImportCompatibilityRequired)
    } else {
        messages.text(MessageId::NetlistImportReady)
    };
    let (title, primary) = import_operation_text(messages, review.operation);
    let choice = Dialog::new(title, primary)
        .description(messages.text(MessageId::NetlistImportDescription))
        .size(DialogSize::Transaction)
        .initial_focus(DialogInitialFocus::BodyControl)
        .primary_enabled(primary_enabled)
        .ghost(messages.text(MessageId::CommonCancel))
        .hint(&hint)
        .show_with_initial_body_focus(ctx, |ui| {
            let t = Tokens::get(ctx);
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(10)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&review.display_name)
                            .font(theme::mono(tokens::FS_1, FontWeight::Medium)),
                    );
                    let source_kind =
                        if review.operation == NetlistImportOperation::RequalifyOwnedSource {
                            messages.text(MessageId::NetlistImportProjectSourceSnapshot)
                        } else if review.archive_import {
                            let count = review.dependencies.len().to_string();
                            messages.format(
                                if review.dependencies.len() == 1 {
                                    MessageId::NetlistImportBundleSourceSingular
                                } else {
                                    MessageId::NetlistImportBundleSource
                                },
                                &[("count", &count)],
                            )
                        } else {
                            messages.text(MessageId::NetlistImportLosslessSource)
                        };
                    ui.label(source_kind);
                    let byte_count = review.original_byte_count.to_string();
                    ui.monospace(messages.format(
                        MessageId::NetlistImportSourceSummary,
                        &[
                            ("count", &byte_count),
                            ("digest", &digest),
                            ("encoding", review.encoding.label()),
                            ("line_ending", review.line_ending.label()),
                        ],
                    ));
                    let fallback_path = messages.text(MessageId::NetlistImportBrowserSnapshot);
                    let invalid_path = messages.text(MessageId::NetlistImportInvalidUnicodePath);
                    ui.weak(
                        review
                            .selected_file_path
                            .as_deref()
                            .map_or(fallback_path.as_str(), |path| {
                                path.to_str().unwrap_or(invalid_path.as_str())
                            }),
                    );
                });

            ui.add_space(8.0);
            ui.label(messages.text(MessageId::NetlistSourceDialect));
            let dialect = egui::ComboBox::from_id_salt("rspice.netlist.import-dialect")
                .selected_text(review.selected_dialect.label())
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    for dialect in crate::state::NetlistSourceDialect::ALL {
                        ui.selectable_value(&mut review.selected_dialect, dialect, dialect.label());
                    }
                });
            ui.weak(messages.format(
                if review.detection_evidence.is_empty() {
                    MessageId::NetlistImportDetectedNoMarker
                } else {
                    MessageId::NetlistImportDetectedEvidence
                },
                &[("dialect", review.detected_dialect.label())],
            ));
            for evidence in &review.detection_evidence {
                ui.monospace(evidence);
            }
            if let Some(error) = dialect_qualification_error.as_deref() {
                ui.add_space(6.0);
                problem_line(ui, Problem::Failure, error);
            }

            ui.add_space(6.0);
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(8)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(
                            messages.text(MessageId::NetlistImportExecutionProfile),
                        )
                        .strong(),
                    );
                    if let Some(profile) = qualified_execution_profile {
                        ui.monospace(messages.format(
                            MessageId::NetlistImportExecutionProfileReceipt,
                            &[("profile", profile.id())],
                        ));
                    } else {
                        problem_line(
                            ui,
                            Problem::Failure,
                            &messages.text(MessageId::NetlistImportNoExecutionProfile),
                        );
                    }
                });

            if review.selected_dialect.requires_compatibility_review()
                && let Some(profile) = qualified_execution_profile
            {
                ui.add_space(6.0);
                ui.checkbox(
                    &mut review.compatibility_accepted,
                    messages.format(
                        MessageId::NetlistImportAcceptProfile,
                        &[
                            ("dialect", review.selected_dialect.label()),
                            ("profile", profile.id()),
                        ],
                    ),
                )
                .on_hover_text(messages.text(MessageId::NetlistImportAcceptanceNotice));
            } else {
                review.compatibility_accepted = false;
            }

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(messages.text(MessageId::NetlistImportTransformations))
                    .strong(),
            );
            for transformation in &review.transformations {
                ui.label(format!("- {transformation}"));
            }

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(messages.text(MessageId::NetlistImportValidation)).strong(),
            );
            // Nothing is listed when the parse and the include resolution both
            // went through: the heading above is the whole statement.
            for issue in &review.issues {
                // The mark replaces the prefix that used to spell the
                // severity out in front of every line.
                let problem = match issue.severity {
                    NetlistImportIssueSeverity::Advisory => Problem::Warning,
                    NetlistImportIssueSeverity::Blocking => Problem::Failure,
                };
                problem_line(ui, problem, &issue.message);
            }

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(messages.text(MessageId::NetlistImportSourcePreview)).strong(),
            );
            egui::Frame::new()
                .fill(t.color.bg_inset)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(8)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("rspice.netlist.import-preview")
                        .max_height(160.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (line, text) in review.source.lines().take(200).enumerate() {
                                ui.monospace(format!("{:>5}  {text}", line + 1));
                            }
                            if review.source.lines().nth(200).is_some() {
                                ui.weak(messages.text(MessageId::NetlistImportPreviewLimited));
                            }
                        });
                });
            if let Some(error) = &review.error {
                ui.add_space(6.0);
                problem_line(ui, Problem::Failure, error);
            }
            Some(dialect.response.id)
        });

    // Persist review choices before a commit attempt revalidates the exact
    // lifecycle transaction and candidate snapshot.
    app.state.code.netlist.import_review = Some(review);
    match choice {
        DialogChoice::Primary => {
            crate::workbench::workflows::netlist_workflow::commit_staged_netlist_import(
                &mut app.state,
            );
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            crate::workbench::workflows::netlist_workflow::cancel_staged_netlist_import(
                &mut app.state,
            );
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Export is a command form: the dialect, what goes with the deck, and
    /// Export…; a refusal is its status. It used to open on a boxed paragraph
    /// about snapshots, close on a box saying "0 external dependencies ·
    /// sealed" and "accompanied by an integrity manifest", and repeat the
    /// count in the footer.
    #[test]
    fn export_is_a_command_form_without_the_sealing_boxes() {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.code.netlist.export_dialog.open = true;
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ui| export_generated_dialog_window(ui.ctx(), &mut app),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let names = nodes
            .values()
            .flat_map(|node| [node.label(), node.value()])
            .flatten()
            .collect::<Vec<_>>();
        let says = |text: &str| names.iter().any(|name| name.contains(text));
        assert!(says("Dialect"), "{names:?}");
        assert!(says("Export\u{2026}"), "the primary is not Export…");
        assert!(says("Cancel"));
        for jargon in ["sealed", "integrity", "snapshot", "dependency members"] {
            assert!(!says(jargon), "the dialog says {jargon:?}");
        }
    }
}
