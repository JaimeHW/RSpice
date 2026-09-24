//! Source crossing the project boundary: external-change review, generated
//! export, and staged import review.
//!
//! Every one of these is a reviewed transaction. Nothing here writes project
//! bytes before the exact source identity it was opened against is confirmed
//! to still be current.

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
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

    let Some(mut review) = app.state.ui.netlist.external_change.clone() else {
        return;
    };
    let messages = app.state.ui.messages();
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
        messages.text(MessageId::NetlistExternalEyebrow),
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
            ui.colored_label(t.color.err, error);
        }
        Some(resolution.response.id)
    });

    app.state.ui.netlist.external_change = Some(review);
    match choice {
        DialogChoice::Primary => {
            match crate::workbench::workflows::netlist_workflow::apply_staged_external_netlist_change(
                &mut app.state,
            ) {
                Ok(()) => {
                    app.state.push_user_message(ConsoleMessage::info(
                        "External source resolution applied as a journaled project revision.",
                    ));
                }
                Err(error) => {
                    if let Some(current) = app.state.ui.netlist.external_change.as_mut() {
                        current.error = Some(error);
                    }
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.ui.netlist.external_change = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn dependency_status_label(
    messages: MessageCatalog,
    count: usize,
    sealed: bool,
    external: bool,
) -> String {
    let id = match (external, sealed, count == 1) {
        (false, true, true) => MessageId::NetlistDependencySealedSingular,
        (false, true, false) => MessageId::NetlistDependencySealed,
        (false, false, true) => MessageId::NetlistDependencyResolutionSingular,
        (false, false, false) => MessageId::NetlistDependencyResolution,
        (true, true, true) => MessageId::NetlistExportExternalSealedSingular,
        (true, true, false) => MessageId::NetlistExportExternalSealed,
        (true, false, true) => MessageId::NetlistExportExternalResolutionSingular,
        (true, false, false) => MessageId::NetlistExportExternalResolution,
    };
    let count = count.to_string();
    messages.format(id, &[("count", &count)])
}

pub(super) fn export_generated_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.export_dialog.open {
        return;
    }
    let mut dialog = app.state.ui.netlist.export_dialog.clone();
    let current = app.state.ui.netlist.generation_error.is_none()
        && app.state.ui.netlist.generated_document.is_some()
        && app.state.ui.netlist.generated_input_digest
            == app.state.ui.netlist.current_generation_input_digest;
    let (dependency_count, dependencies_sealed) = app
        .state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .map(|document| {
            (
                document.dependencies().len(),
                document.dependency_graph_is_sealed(),
            )
        })
        .unwrap_or_default();
    let requires_bundle = dependency_count > 0;
    if requires_bundle {
        dialog.bundle_dependencies = true;
    }
    let messages = app.state.ui.messages();
    let bundle_ready = !dialog.bundle_dependencies || dependencies_sealed;
    let primary = if dialog.bundle_dependencies {
        messages.text(MessageId::NetlistExportBundle)
    } else {
        messages.text(MessageId::NetlistExportDeck)
    };
    let footer_hint =
        dependency_status_label(messages, dependency_count, dependencies_sealed, false);
    let choice = Dialog::new(
        messages.text(MessageId::NetlistExportEyebrow),
        messages.text(MessageId::NetlistExportTitle),
        primary,
    )
    .description(messages.text(MessageId::NetlistExportDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(current && bundle_ready)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(&footer_hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(messages.text(MessageId::NetlistExportSnapshotNotice));
            });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistDialect));
        let dialect = egui::ComboBox::from_id_salt("rspice.code.export-dialect")
            .selected_text(match dialog.format {
                crate::io::NetlistFormat::Spice => "SPICE",
                crate::io::NetlistFormat::Spectre => "Spectre",
                crate::io::NetlistFormat::Hspice => "HSPICE",
                crate::io::NetlistFormat::Xyce => "Xyce",
            })
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for (format, label) in [
                    (crate::io::NetlistFormat::Spice, "SPICE"),
                    (crate::io::NetlistFormat::Spectre, "Spectre"),
                    (crate::io::NetlistFormat::Hspice, "HSPICE"),
                    (crate::io::NetlistFormat::Xyce, "Xyce"),
                ] {
                    ui.selectable_value(&mut dialog.format, format, label);
                }
            });
        ui.add_space(6.0);
        let bundle_response = ui.add_enabled(
            !requires_bundle,
            egui::Checkbox::new(
                &mut dialog.bundle_dependencies,
                messages.text(MessageId::NetlistExportDependencyBundle),
            ),
        );
        if requires_bundle {
            bundle_response
                .clone()
                .on_disabled_hover_text(messages.text(MessageId::NetlistExportBundleRequired));
        }
        if bundle_response.changed() && !dialog.bundle_dependencies {
            dialog.include_source_map = false;
        }
        let source_map_supported = dialog.format == crate::io::NetlistFormat::Spice;
        let source_map_response = ui.add_enabled(
            source_map_supported,
            egui::Checkbox::new(
                &mut dialog.include_source_map,
                messages.text(MessageId::NetlistExportSourceMap),
            ),
        );
        if source_map_response.changed() && dialog.include_source_map {
            dialog.bundle_dependencies = true;
        }
        if !source_map_supported {
            dialog.include_source_map = false;
            source_map_response
                .on_disabled_hover_text(messages.text(MessageId::NetlistExportSourceMapSpiceOnly));
        }
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(dependency_status_label(
                    messages,
                    dependency_count,
                    dependencies_sealed,
                    true,
                ));
                ui.label(if dialog.bundle_dependencies {
                    messages.text(MessageId::NetlistExportBundleBehavior)
                } else {
                    messages.text(MessageId::NetlistExportDeckBehavior)
                });
            });
        if let Some(error) = &dialog.error {
            ui.colored_label(t.color.err, error);
        }
        Some(dialect.response.id)
    });
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
    app.state.ui.netlist.export_dialog = dialog;
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

    let Some(mut review) = app.state.ui.netlist.import_review.clone() else {
        return;
    };
    let messages = app.state.ui.messages();
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
    let choice = Dialog::new(
        messages.text(MessageId::NetlistImportEyebrow),
        title,
        primary,
    )
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
            ui.colored_label(t.color.err, error);
        }

        ui.add_space(6.0);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(8)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(messages.text(MessageId::NetlistImportExecutionProfile))
                        .strong(),
                );
                if let Some(profile) = qualified_execution_profile {
                    ui.monospace(messages.format(
                        MessageId::NetlistImportExecutionProfileReceipt,
                        &[("profile", profile.id())],
                    ));
                } else {
                    ui.colored_label(
                        t.color.err,
                        messages.text(MessageId::NetlistImportNoExecutionProfile),
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
            egui::RichText::new(messages.text(MessageId::NetlistImportTransformations)).strong(),
        );
        for transformation in &review.transformations {
            ui.label(format!("- {transformation}"));
        }

        ui.add_space(8.0);
        ui.label(egui::RichText::new(messages.text(MessageId::NetlistImportValidation)).strong());
        if review.issues.is_empty() {
            ui.colored_label(
                t.color.ok,
                messages.text(MessageId::NetlistImportValidationPassed),
            );
        } else {
            for issue in &review.issues {
                let (prefix, color) = match issue.severity {
                    NetlistImportIssueSeverity::Advisory => (
                        messages.text(MessageId::NetlistImportAdvisory),
                        t.color.warn,
                    ),
                    NetlistImportIssueSeverity::Blocking => {
                        (messages.text(MessageId::NetlistImportBlocked), t.color.err)
                    }
                };
                ui.colored_label(color, format!("{prefix}: {}", issue.message));
            }
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
            ui.colored_label(t.color.err, error);
        }
        Some(dialect.response.id)
    });

    // Persist review choices before a commit attempt revalidates the exact
    // lifecycle transaction and candidate snapshot.
    app.state.ui.netlist.import_review = Some(review);
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
