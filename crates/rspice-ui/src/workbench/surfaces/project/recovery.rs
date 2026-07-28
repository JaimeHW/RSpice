//! The recovery page: checkpoints, their contents, and exporting a copy.
//!
//! A checkpoint row states what it holds and when it was taken, and exporting
//! a copy never mutates or consumes the checkpoint — recovery is always
//! additive, so inspecting or exporting one cannot cost you the ability to
//! recover from it later.

use super::*;

pub(super) fn recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    ensure_project_recovery_catalog(ui.ctx(), app);
    let layout = ProjectContractLayout::resolve(ui.available_width());
    header_with_actions(
        ui,
        "RECOVERY · CHECKPOINTS · STORE INTEGRITY",
        "Project recovery center",
        "Review recoverable working state and integrity-verified full-project checkpoints. Saved projects and live working state are never overwritten by a recovery-copy operation.",
        |ui, app| {
            if Button::new("Project").show(ui).clicked() {
                Command::ProjectPage(ProjectPage::Dashboard).execute(app);
            }
            if Button::new("Save current project")
                .accent()
                .show(ui)
                .clicked()
            {
                Command::Save.execute(app);
            }
        },
        app,
    );
    let dirty_documents: Vec<_> = app
        .state
        .workspace
        .open_views
        .iter()
        .filter(|view| view.dirty)
        .map(|view| view.reference.display_path())
        .collect();
    recovery_status_strip(ui, app, &dirty_documents, layout);
    recovery_checkpoint_table(ui, app);
    recovery_operation_grid(ui, app, layout);
}

pub(super) fn recovery_status_strip(
    ui: &mut Ui,
    app: &RSpiceApp,
    dirty_documents: &[String],
    layout: ProjectContractLayout,
) {
    let t = Tokens::get(ui.ctx());
    let recovery = &app.state.dialogs.project_checkpoint_recovery;
    let modified_count = dirty_documents.len()
        + usize::from(app.state.schematic.is_dirty)
        + usize::from(app.state.workspace.netlist_source_dirty);
    let checkpoint_count = recovery.checkpoints.len();
    let integrity_healthy = recovery.error.is_none() && recovery.quarantined.is_empty();
    let project_path = app
        .state
        .workspace
        .project
        .path
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "No saved project path".to_owned());
    let specs = [
        WorkspaceBandSpec {
            label: "Working state",
            value: if modified_count == 0 {
                "current".to_owned()
            } else {
                format!("{modified_count} modified")
            },
            detail: format!(
                "{} open document{}",
                app.state.workspace.open_views.len(),
                plural(app.state.workspace.open_views.len())
            ),
            value_color: if modified_count == 0 {
                t.color.ok
            } else {
                t.color.warn
            },
        },
        WorkspaceBandSpec {
            label: "Recovery points",
            value: checkpoint_count.to_string(),
            detail: if checkpoint_count == 0 {
                "No integrity-verified checkpoint available".to_owned()
            } else {
                "whole-project snapshots protected".to_owned()
            },
            value_color: if checkpoint_count > 0 {
                t.color.ok
            } else {
                t.color.text
            },
        },
        WorkspaceBandSpec {
            label: "Store integrity",
            value: if recovery.error.is_some() {
                "error".to_owned()
            } else if recovery.quarantined.is_empty() {
                "verified".to_owned()
            } else {
                format!("{} quarantined", recovery.quarantined.len())
            },
            detail: recovery
                .error
                .clone()
                .unwrap_or_else(|| "checkpoint manifests and payloads inspected".to_owned()),
            value_color: if integrity_healthy {
                t.color.ok
            } else {
                t.color.warn
            },
        },
        WorkspaceBandSpec {
            label: "Project storage",
            value: if app.state.workspace.project.path.is_some() {
                "saved path".to_owned()
            } else {
                "unsaved".to_owned()
            },
            detail: project_path,
            value_color: if app.state.workspace.project.path.is_some() {
                t.color.text
            } else {
                t.color.warn
            },
        },
    ];
    workspace_status_band(
        ui,
        "workbench.project.recovery.status",
        &specs,
        layout,
        RECOVERY_STATUS_HEIGHT,
        false,
    );
}

pub(super) fn ensure_project_recovery_catalog(ctx: &Context, app: &mut RSpiceApp) {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ctx;
    let project_id = app.state.workspace.project.id().to_string();
    if app
        .state
        .dialogs
        .project_checkpoint_recovery
        .project_id
        .as_deref()
        != Some(project_id.as_str())
    {
        app.state.dialogs.project_checkpoint_recovery = Default::default();
        app.state.dialogs.project_checkpoint_recovery.project_id = Some(project_id.clone());
    }
    if app.state.dialogs.project_checkpoint_recovery.initialized {
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let result = crate::workbench::lifecycle::project_checkpoint::list(&app.state);
        let recovery = &mut app.state.dialogs.project_checkpoint_recovery;
        recovery.initialized = true;
        match result {
            Ok(catalog) => {
                recovery.checkpoints = catalog.checkpoints;
                recovery.quarantined = catalog.quarantined;
                recovery.error = None;
            }
            Err(error) => recovery.error = Some(error),
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        if app.state.dialogs.project_checkpoint_recovery.loading {
            return;
        }
        app.state.dialogs.project_checkpoint_recovery.loading = true;
        let queued_project_id = project_id;
        let repaint = ctx.clone();
        crate::workbench::lifecycle::project_checkpoint::start_list(&app.state, move |result| {
            BROWSER_RECOVERY_CATALOG_COMPLETIONS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserRecoveryCatalogCompletion {
                        project_id: queued_project_id,
                        result,
                    });
            });
            repaint.request_repaint();
        });
    }
}

pub(super) fn recovery_checkpoint_table(ui: &mut Ui, app: &mut RSpiceApp) {
    let loading = {
        #[cfg(target_arch = "wasm32")]
        {
            app.state.dialogs.project_checkpoint_recovery.loading
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    };
    let checkpoints = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .checkpoints
        .clone();
    let error = app.state.dialogs.project_checkpoint_recovery.error.clone();
    let quarantined = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .quarantined
        .clone();
    let t = Tokens::get(ui.ctx());
    let (meta, meta_color) = if loading {
        ("LOADING", t.color.text_dim)
    } else if error.is_some() {
        ("CATALOG ERROR", t.color.err)
    } else if !quarantined.is_empty() {
        ("QUARANTINED ARTIFACTS", t.color.warn)
    } else if checkpoints.is_empty() {
        ("NO VERIFIED POINTS", t.color.text_faint)
    } else {
        ("PROTECTED", t.color.ok)
    };
    workspace_table_panel_header(ui, "Recovery points", meta, meta_color);
    let visible_width = ui.available_width().max(1.0);
    ScrollArea::horizontal()
        .id_salt("workbench.project.recovery.checkpoints")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = visible_width.max(RECOVERY_TABLE_MIN_WIDTH);
            ui.set_min_width(width);
            let fractions = [0.14, 0.24, 0.12, 0.13, 0.13, 0.24];
            workspace_table_row(
                ui,
                width,
                ["CHECKPOINT", "CAUSE", "BASE REVISION", "SNAPSHOT", "INTEGRITY", "ACTIONS"],
                fractions,
                true,
                &[],
                &[],
            );
            if loading {
                workspace_empty_table_row(ui, width, "Loading and verifying recovery artifacts…");
            } else if checkpoints.is_empty() && quarantined.is_empty() {
                workspace_empty_table_row(
                    ui,
                    width,
                    error.as_deref().unwrap_or(
                        "No integrity-verified full-project checkpoints are available for this project.",
                    ),
                );
            }
            for checkpoint in checkpoints {
                let identity = short_identity(&checkpoint.checkpoint_id().to_string());
                let revision = checkpoint.project_revision().to_string();
                let size = format_byte_count(checkpoint.snapshot_byte_len());
                let (_, cells) = workspace_table_row(
                    ui,
                    width,
                    [
                        identity.as_str(),
                        checkpoint.reason().label(),
                        revision.as_str(),
                        size.as_str(),
                        "verified",
                        "",
                    ],
                    fractions,
                    false,
                    &[0, 2, 3],
                    &[(4, t.color.ok)],
                );
                let action_rect = cells[5].shrink2(vec2(6.0, 4.0));
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(action_rect)
                        .layout(Layout::left_to_right(Align::Center)),
                    |ui| {
                        if Button::new("Save recovered copy…").show(ui).clicked() {
                            export_project_checkpoint_copy(ui.ctx(), app, checkpoint.clone());
                        }
                    },
                );
            }
            for record in quarantined {
                let artifacts = format!(
                    "{} artifact{}",
                    record.artifact_count(),
                    plural(record.artifact_count())
                );
                let (response, _) = workspace_table_row(
                    ui,
                    width,
                    [
                        record.label(),
                        record.reason(),
                        "—",
                        artifacts.as_str(),
                        "quarantined",
                        "not recoverable",
                    ],
                    fractions,
                    false,
                    &[0, 2, 3],
                    &[(4, t.color.warn)],
                );
                response.on_hover_text(record.reason());
            }
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RecoveryOperationAction {
    SaveCurrent,
    RefreshCatalog,
    SaveProjectCopy,
}

pub(super) fn recovery_operation_grid(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    layout: ProjectContractLayout,
) {
    let t = Tokens::get(ui.ctx());
    let operations = [
        (
            "Current working state",
            "Persist current editable project state transactionally without altering any retained recovery point.",
            "Save current project",
            RecoveryOperationAction::SaveCurrent,
        ),
        (
            "Verified checkpoint recovery",
            "Refresh the owned checkpoint catalog. Each row can publish an independent recovered project identity without overwriting the live project.",
            "Refresh recovery points",
            RecoveryOperationAction::RefreshCatalog,
        ),
        (
            "Archives and portability",
            "Create a separate project copy through the platform file workflow while the current project and retained checkpoints remain available.",
            "Save project copy…",
            RecoveryOperationAction::SaveProjectCopy,
        ),
    ];
    let mut requested = None;
    let mut panel_rects = Vec::with_capacity(operations.len());
    let shown = egui::Grid::new("workbench.project.recovery.operations")
        .num_columns(layout.operation_columns)
        .spacing(vec2(0.0, 0.0))
        .show(ui, |ui| {
            for (index, (title, copy, action_label, action)) in operations.iter().enumerate() {
                let panel = ui.allocate_ui_with_layout(
                    vec2(layout.operation_cell_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(layout.operation_cell_width);
                        if recovery_operation_panel(ui, title, copy, action_label) {
                            requested = Some(*action);
                        }
                    },
                );
                panel_rects.push(panel.response.rect);
                if (index + 1) % layout.operation_columns == 0 {
                    ui.end_row();
                }
            }
        });
    if layout.operation_columns == 1 {
        for rect in panel_rects.iter().take(panel_rects.len().saturating_sub(1)) {
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                Stroke::new(1.0, t.color.border_strong),
            );
        }
    } else {
        for column in 1..layout.operation_columns {
            ui.painter().vline(
                shown.response.rect.left() + layout.operation_cell_width * column as f32,
                shown.response.rect.y_range(),
                Stroke::new(1.0, t.color.border_strong),
            );
        }
    }
    if let Some(action) = requested {
        match action {
            RecoveryOperationAction::SaveCurrent => Command::Save.execute(app),
            RecoveryOperationAction::RefreshCatalog => {
                app.state.dialogs.project_checkpoint_recovery.invalidate();
                let ctx = ui.ctx().clone();
                ensure_project_recovery_catalog(&ctx, app);
            }
            RecoveryOperationAction::SaveProjectCopy => Command::SaveAs.execute(app),
        }
    }
}

pub(super) fn recovery_operation_panel(
    ui: &mut Ui,
    title: &str,
    copy: &str,
    action_label: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.add_space(4.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(copy)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
            ui.add_space(9.0);
            Button::new(action_label).show(ui).clicked()
        });
    shown.inner
}

pub(super) fn format_byte_count(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MiB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}

pub(super) fn recovery_copy_filename(
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) -> String {
    let base = checkpoint
        .project_name()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    format!(
        "{}-recovered-r{}.rspiceproj",
        base.trim_matches('-'),
        checkpoint.project_revision()
    )
}

pub(super) fn export_project_checkpoint_copy(
    ctx: &Context,
    app: &mut RSpiceApp,
    checkpoint: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    let filename = recovery_copy_filename(&checkpoint);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(destination) = rfd::FileDialog::new()
            .add_filter("RSpice Project", &["rspiceproj"])
            .set_file_name(&filename)
            .save_file()
        else {
            return;
        };
        match crate::workbench::lifecycle::project_checkpoint::publish_recovery_copy(
            &checkpoint,
            &destination,
        ) {
            Ok(()) => {
                let receipt = format!("Saved independent recovery copy: {}", destination.display());
                app.state
                    .push_user_message(ConsoleMessage::info(receipt.clone()));
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Recovery copy saved", receipt);
            }
            Err(error) => {
                app.state
                    .push_user_message(ConsoleMessage::error(error.clone()));
                app.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Recovery copy failed", error);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let project_id = app.state.workspace.project.id().to_string();
        let queued_filename = filename.clone();
        let repaint = ctx.clone();
        crate::workbench::lifecycle::project_checkpoint::start_recovery_copy_bytes(
            checkpoint,
            std::path::PathBuf::from(&filename),
            move |result| {
                BROWSER_RECOVERY_COPY_COMPLETIONS.with(|queue| {
                    queue.borrow_mut().push_back(BrowserRecoveryCopyCompletion {
                        project_id,
                        filename: queued_filename,
                        result,
                    });
                });
                repaint.request_repaint();
            },
        );
    }
}
