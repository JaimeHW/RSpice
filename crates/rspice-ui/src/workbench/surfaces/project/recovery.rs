//! The recovery page: checkpoints, their contents, and exporting a copy.
//!
//! A checkpoint row states what it holds and when it was taken, and exporting
//! a copy never mutates or consumes the checkpoint — recovery is always
//! additive, so inspecting or exporting one cannot cost you the ability to
//! recover from it later.

use super::*;
use crate::workbench::app_state::AppState;

#[cfg(target_arch = "wasm32")]
struct BrowserManualCheckpointCompletion {
    project_id: String,
    result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_MANUAL_CHECKPOINT_PENDING: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
    static BROWSER_MANUAL_CHECKPOINT_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserManualCheckpointCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

pub(super) fn recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_manual_checkpoint(ui.ctx(), &mut app.state);
    ensure_project_recovery_catalog(ui.ctx(), &mut app.state);
    recovery_context_strip(ui, &mut app.state);
    let width = visible_workspace_width(ui);
    if width >= 640.0 {
        let timeline_width = (width * 0.60).floor().max(360.0);
        let policy_width = (width - timeline_width - 1.0).max(240.0);
        let shown = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(
                vec2(timeline_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_width(timeline_width);
                    recovery_checkpoint_timeline(ui, &mut app.state);
                },
            );
            ui.allocate_ui_with_layout(
                vec2(policy_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_width(policy_width);
                    recovery_policy_panel(ui, app);
                },
            );
        });
        ui.painter().vline(
            shown.response.rect.left() + timeline_width + 0.5,
            shown.response.rect.y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
        );
    } else {
        recovery_checkpoint_timeline(ui, &mut app.state);
        recovery_policy_panel(ui, app);
    }
}

fn recovery_context_strip(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let checkpoints = state.dialogs.project_checkpoint_recovery.checkpoints.len();
    let verified = state.dialogs.project_checkpoint_recovery.error.is_none()
        && state
            .dialogs
            .project_checkpoint_recovery
            .quarantined
            .is_empty();
    let modified = crate::workbench::lifecycle::project_lifecycle::dirty_document_count(state);
    let shown = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let search = ui.add_sized(
                    [(ui.available_width() * 0.48).clamp(180.0, 430.0), 28.0],
                    egui::TextEdit::singleline(&mut state.workbench.project_recovery_filter)
                        .hint_text("Checkpoint, revision, state\u{2026}"),
                );
                ui.ctx().accesskit_node_builder(search.id, |node| {
                    node.set_label("Filter project recovery checkpoints");
                });
                ui.separator();
                recovery_context_fact(
                    ui,
                    "Working state",
                    if modified == 0 {
                        "current".to_owned()
                    } else {
                        format!("{modified} modified")
                    },
                    if modified == 0 {
                        t.color.ok
                    } else {
                        t.color.warn
                    },
                );
                ui.separator();
                recovery_context_fact(
                    ui,
                    "Integrity",
                    format!("{checkpoints} verified"),
                    if verified { t.color.ok } else { t.color.warn },
                );
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn recovery_context_fact(ui: &mut Ui, label: &str, value: String, color: Color32) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.label(
        egui::RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn recovery_checkpoint_timeline(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let filter = state
        .workbench
        .project_recovery_filter
        .trim()
        .to_ascii_lowercase();
    let checkpoints = state
        .dialogs
        .project_checkpoint_recovery
        .checkpoints
        .clone();
    let visible = checkpoints
        .iter()
        .filter(|checkpoint| {
            filter.is_empty()
                || format!(
                    "{} {} {}",
                    checkpoint.reason().label(),
                    checkpoint.project_revision(),
                    checkpoint.checkpoint_id()
                )
                .to_ascii_lowercase()
                .contains(&filter)
        })
        .collect::<Vec<_>>();
    workspace_table_panel_header(
        ui,
        "Project checkpoints",
        &format!("{} SHOWN", visible.len()),
        if visible.is_empty() {
            t.color.text_faint
        } else {
            t.color.ok
        },
    );
    if visible.is_empty() {
        workspace_empty_table_row(
            ui,
            ui.available_width().max(1.0),
            if checkpoints.is_empty() {
                "No integrity-verified project checkpoint is available."
            } else {
                "No checkpoint matches the current filter."
            },
        );
    }
    for checkpoint in visible {
        let id = checkpoint.checkpoint_id().to_string();
        let selected = state.workbench.project_checkpoint_selection.as_deref() == Some(id.as_str());
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width().max(1.0), 58.0), Sense::click());
        if selected || response.hovered() {
            ui.painter().rect_filled(
                rect,
                0.0,
                if selected {
                    t.color.bg_active
                } else {
                    t.color.bg_hover
                },
            );
        }
        if selected {
            ui.painter().rect_filled(
                Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
                0.0,
                t.color.accent,
            );
        }
        ui.painter().hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        let content = rect.shrink2(vec2(10.0, 5.0));
        ui.painter().text(
            content.left_top(),
            Align2::LEFT_TOP,
            checkpoint_age(checkpoint.created_unix_ms()),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
        ui.painter().text(
            pos2(content.left() + 70.0, content.top()),
            Align2::LEFT_TOP,
            checkpoint.reason().label(),
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
        );
        ui.painter().text(
            pos2(content.left() + 70.0, content.top() + 20.0),
            Align2::LEFT_TOP,
            format!(
                "revision {} \u{00b7} {} \u{00b7} verified",
                checkpoint.project_revision(),
                format_byte_count(checkpoint.snapshot_byte_len())
            ),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.ok,
        );
        let actions = Rect::from_min_max(
            pos2((rect.right() - 180.0).max(rect.left()), rect.top() + 12.0),
            pos2(rect.right() - 8.0, rect.bottom() - 8.0),
        );
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(actions)
                .layout(Layout::right_to_left(Align::Center)),
            |ui| {
                if Button::new("Restore\u{2026}").show(ui).clicked() {
                    export_project_checkpoint_copy(ui.ctx(), state, checkpoint.clone());
                }
                if Button::new("Compare\u{2026}").show(ui).clicked() {
                    compare_project_checkpoint(ui.ctx(), state, checkpoint);
                }
            },
        );
        if response.clicked() {
            state.workbench.project_checkpoint_selection = Some(id);
        }
    }
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(
                        "Compare and restore operate on immutable checkpoints. Restore publishes an independent project copy and never overwrites current work.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

fn recovery_policy_panel(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let recovery = &app.state.dialogs.project_checkpoint_recovery;
    let checkpoint_bytes = recovery
        .checkpoints
        .iter()
        .map(|checkpoint| checkpoint.snapshot_byte_len())
        .sum::<u64>();
    let verified = recovery.error.is_none() && recovery.quarantined.is_empty();
    workspace_table_panel_header(
        ui,
        "Policy & storage",
        if verified { "VERIFIED" } else { "REVIEW" },
        if verified { t.color.ok } else { t.color.warn },
    );
    property_row(ui, "Checkpoint mode", "manual and governed mutations");
    property_row(
        ui,
        "Retained checkpoints",
        &recovery.checkpoints.len().to_string(),
    );
    property_row(ui, "Restore destination", "independent project copy");
    property_row(ui, "Current work", "never overwritten");
    property_row(ui, "Storage", &format_byte_count(checkpoint_bytes));
    property_row(
        ui,
        "Quarantined payloads",
        &recovery.quarantined.len().to_string(),
    );
    property_row(
        ui,
        "Project path",
        &app.state
            .workspace
            .project
            .path
            .as_ref()
            .map_or_else(|| "not saved".to_owned(), |path| path.display().to_string()),
    );
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let pending = manual_checkpoint_pending();
                let create = Button::new(if pending {
                    "Creating checkpoint\u{2026}"
                } else {
                    "Checkpoint now\u{2026}"
                })
                .accent()
                .enabled(!pending)
                .show(ui);
                if create.clicked() {
                    create_manual_checkpoint(ui.ctx(), &mut app.state);
                }
                if Button::new("Save project").show(ui).clicked() {
                    Command::Save.execute(app);
                }
                if Button::new("Revision history\u{2026}").show(ui).clicked() {
                    Command::RevisionHistory.execute(app);
                }
            });
        });
}

pub(super) fn recovery_status_strip(ui: &mut Ui, app: &RSpiceApp, layout: ProjectContractLayout) {
    let t = Tokens::get(ui.ctx());
    let recovery = &app.state.dialogs.project_checkpoint_recovery;
    let modified_count =
        crate::workbench::lifecycle::project_lifecycle::dirty_document_count(&app.state);
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

pub(super) fn ensure_project_recovery_catalog(ctx: &Context, state: &mut AppState) {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ctx;
    let project_id = state.workspace.project.id().to_string();
    if state
        .dialogs
        .project_checkpoint_recovery
        .project_id
        .as_deref()
        != Some(project_id.as_str())
    {
        state.dialogs.project_checkpoint_recovery = Default::default();
        state.dialogs.project_checkpoint_recovery.project_id = Some(project_id.clone());
    }
    if state.dialogs.project_checkpoint_recovery.initialized {
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let result = crate::workbench::lifecycle::project_checkpoint::list(state);
        let recovery = &mut state.dialogs.project_checkpoint_recovery;
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
        if state.dialogs.project_checkpoint_recovery.loading {
            return;
        }
        state.dialogs.project_checkpoint_recovery.loading = true;
        let queued_project_id = project_id;
        let repaint = ctx.clone();
        crate::workbench::lifecycle::project_checkpoint::start_list(state, move |result| {
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

pub(super) fn recovery_checkpoint_table(ui: &mut Ui, state: &mut AppState) {
    let loading = {
        #[cfg(target_arch = "wasm32")]
        {
            state.dialogs.project_checkpoint_recovery.loading
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    };
    let checkpoints = state
        .dialogs
        .project_checkpoint_recovery
        .checkpoints
        .clone();
    let error = state.dialogs.project_checkpoint_recovery.error.clone();
    let quarantined = state
        .dialogs
        .project_checkpoint_recovery
        .quarantined
        .clone();
    let filter = state
        .workbench
        .project_recovery_filter
        .trim()
        .to_ascii_lowercase();
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
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            let search = ui.add_sized(
                [ui.available_width().min(420.0), t.metrics.ctl_h.min(28.0)],
                egui::TextEdit::singleline(&mut state.workbench.project_recovery_filter)
                    .hint_text("Filter checkpoints by identity, cause, or revision\u{2026}"),
            );
            ui.ctx().accesskit_node_builder(search.id, |node| {
                node.set_label("Filter project recovery checkpoints");
            });
        });
    let visible_width = ui.available_width().max(1.0);
    ScrollArea::horizontal()
        .id_salt("workbench.project.recovery.checkpoints")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = visible_width.max(RECOVERY_TABLE_MIN_WIDTH);
            ui.set_min_width(width);
            let fractions = [0.12, 0.15, 0.19, 0.10, 0.11, 0.10, 0.23];
            workspace_table_row(
                ui,
                width,
                [
                    "CREATED",
                    "CHECKPOINT",
                    "CAUSE",
                    "REVISION",
                    "SNAPSHOT",
                    "INTEGRITY",
                    "ACTIONS",
                ],
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
                let created = checkpoint_age(checkpoint.created_unix_ms());
                let digest = short_identity(&checkpoint.snapshot_digest().to_string());
                let searchable = format!(
                    "{identity} {digest} {} {revision} {created}",
                    checkpoint.reason().label()
                )
                .to_ascii_lowercase();
                if !filter.is_empty() && !searchable.contains(&filter) {
                    continue;
                }
                let (response, cells) = workspace_table_row(
                    ui,
                    width,
                    [
                        created.as_str(),
                        identity.as_str(),
                        checkpoint.reason().label(),
                        revision.as_str(),
                        size.as_str(),
                        "verified",
                        "",
                    ],
                    fractions,
                    false,
                    &[0, 1, 3, 4],
                    &[(5, t.color.ok)],
                );
                response.on_hover_text(format!(
                    "Checkpoint {}\nSnapshot digest {}\n{}",
                    checkpoint.checkpoint_id(),
                    checkpoint.snapshot_digest(),
                    checkpoint.reason().label()
                ));
                let action_rect = cells[6].shrink2(vec2(6.0, 4.0));
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(action_rect)
                        .layout(Layout::left_to_right(Align::Center)),
                    |ui| {
                        if Button::new("Compare").show(ui).clicked() {
                            compare_project_checkpoint(ui.ctx(), state, &checkpoint);
                        }
                        if Button::new("Restore copy\u{2026}").show(ui).clicked() {
                            export_project_checkpoint_copy(ui.ctx(), state, checkpoint.clone());
                        }
                    },
                );
            }
            for record in quarantined {
                let searchable =
                    format!("{} {}", record.label(), record.reason()).to_ascii_lowercase();
                if !filter.is_empty() && !searchable.contains(&filter) {
                    continue;
                }
                let artifacts = format!(
                    "{} artifact{}",
                    record.artifact_count(),
                    plural(record.artifact_count())
                );
                let (response, _) = workspace_table_row(
                    ui,
                    width,
                    [
                        "—",
                        record.label(),
                        record.reason(),
                        "—",
                        artifacts.as_str(),
                        "quarantined",
                        "not recoverable",
                    ],
                    fractions,
                    false,
                    &[0, 1, 3, 4],
                    &[(5, t.color.warn)],
                );
                response.on_hover_text(record.reason());
            }
        });
}

fn checkpoint_age(created_unix_ms: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);
    let seconds = now.saturating_sub(created_unix_ms) / 1_000;
    match seconds {
        0..=59 => format!("{seconds} s ago"),
        60..=3_599 => format!("{} min ago", seconds / 60),
        3_600..=86_399 => format!("{} h ago", seconds / 3_600),
        _ => format!("{} d ago", seconds / 86_400),
    }
}

fn compare_project_checkpoint(
    ctx: &Context,
    state: &mut AppState,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    match crate::workbench::lifecycle::project_checkpoint::matches_current_state(checkpoint, state)
    {
        Ok(true) => state.ui.toasts.success(
            ctx,
            "Checkpoint matches",
            "The current project is byte-for-byte equivalent to this validated checkpoint.",
        ),
        Ok(false) => state.ui.toasts.info_with_title(
            ctx,
            "Checkpoint differs",
            format!(
                "The current project differs from checkpoint {} (revision {}).",
                short_identity(&checkpoint.checkpoint_id().to_string()),
                checkpoint.project_revision()
            ),
        ),
        Err(error) => state
            .ui
            .toasts
            .error_with_title(ctx, "Checkpoint comparison failed", error),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn manual_checkpoint_pending() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn manual_checkpoint_pending() -> bool {
    BROWSER_MANUAL_CHECKPOINT_PENDING.with(std::cell::Cell::get)
}

fn create_manual_checkpoint(ctx: &Context, state: &mut AppState) {
    #[cfg(not(target_arch = "wasm32"))]
    match crate::workbench::lifecycle::project_checkpoint::create(
        state,
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::Manual,
    ) {
        Ok(checkpoint) => {
            state.dialogs.project_checkpoint_recovery.initialized = false;
            let receipt = format!(
                "Created full-project checkpoint {} for revision {}",
                short_identity(&checkpoint.checkpoint_id().to_string()),
                checkpoint.project_revision()
            );
            state.push_user_message(ConsoleMessage::info(receipt.clone()));
            state.ui.toasts.success(ctx, "Checkpoint created", receipt);
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(error.clone()));
            state
                .ui
                .toasts
                .error_with_title(ctx, "Checkpoint failed", error);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let project_id = state.workspace.project.id().to_string();
        let repaint = ctx.clone();
        BROWSER_MANUAL_CHECKPOINT_PENDING.with(|pending| pending.set(true));
        if let Err(error) = crate::workbench::lifecycle::project_checkpoint::start_create(
            state,
            crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::Manual,
            move |result| {
                BROWSER_MANUAL_CHECKPOINT_COMPLETIONS.with(|queue| {
                    queue
                        .borrow_mut()
                        .push_back(BrowserManualCheckpointCompletion { project_id, result });
                });
                repaint.request_repaint();
            },
        ) {
            BROWSER_MANUAL_CHECKPOINT_PENDING.with(|pending| pending.set(false));
            state
                .ui
                .toasts
                .error_with_title(ctx, "Checkpoint failed", error);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_manual_checkpoint(ctx: &Context, state: &mut AppState) {
    let current_project_id = state.workspace.project.id().to_string();
    let completions = BROWSER_MANUAL_CHECKPOINT_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    if completions.is_empty() {
        return;
    }
    BROWSER_MANUAL_CHECKPOINT_PENDING.with(|pending| pending.set(false));
    for completion in completions {
        if completion.project_id != current_project_id {
            continue;
        }
        match completion.result {
            Ok(checkpoint) => {
                state.dialogs.project_checkpoint_recovery.initialized = false;
                let receipt = format!(
                    "Created full-project checkpoint {} for revision {}",
                    short_identity(&checkpoint.checkpoint_id().to_string()),
                    checkpoint.project_revision()
                );
                state.push_user_message(ConsoleMessage::info(receipt.clone()));
                state.ui.toasts.success(ctx, "Checkpoint created", receipt);
            }
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(error.clone()));
                state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Checkpoint failed", error);
            }
        }
    }
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
                ensure_project_recovery_catalog(&ctx, &mut app.state);
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
    state: &mut AppState,
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
                state.push_user_message(ConsoleMessage::info(receipt.clone()));
                state.ui.toasts.success(ctx, "Recovery copy saved", receipt);
            }
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(error.clone()));
                state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Recovery copy failed", error);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let project_id = state.workspace.project.id().to_string();
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
