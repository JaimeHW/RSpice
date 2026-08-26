//! The recovery page: checkpoints, their contents, and exporting a copy.
//!
//! A checkpoint row states what it holds and when it was taken, and exporting
//! a copy never mutates or consumes the checkpoint — recovery is always
//! additive, so inspecting or exporting one cannot cost you the ability to
//! recover from it later.

use super::*;
use crate::simulation::run_set::format_bytes;
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
                format_bytes(checkpoint.snapshot_byte_len())
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
        let row_label = format!(
            "{} checkpoint, revision {}",
            checkpoint.reason().label(),
            checkpoint.project_revision()
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                row_label.clone(),
            )
        });
        theme::paint_focus_ring(ui, &response, rect);
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
    property_row(ui, "Storage", &format_bytes(checkpoint_bytes));
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
