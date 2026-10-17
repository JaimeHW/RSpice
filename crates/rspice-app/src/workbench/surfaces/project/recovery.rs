//! The recovery page: the project's checkpoints and where they are kept.
//!
//! A checkpoint is a complete, integrity-checked copy of the project. Comparing
//! or restoring one never mutates or consumes it — restoring saves a separate
//! project file — so inspecting a checkpoint cannot cost the reader the
//! ability to recover from it later.

use super::page::{self, BODY_TOP, CARD_GAP, HEADER_TOP, STACK_BREAKPOINT, paint_elided};
use super::*;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::{property_row_path, property_row_problem};
use crate::workbench::lifecycle::project_checkpoint::{
    MAX_RETAINED_CHECKPOINTS, ProjectCheckpointSummary,
};
use rspice_simulation_contract::run_set::format_bytes;

/// The one sentence the page opens with: what a checkpoint is, and the
/// promise restoring one keeps.
const RECOVERY_NOTE: &str = "A checkpoint is a complete, verified copy of the project. \
     Restoring one saves it as a new project file; your current work is never changed.";
pub(super) const CHECKPOINT_ROW_HEIGHT: f32 = 52.0;
/// Below this card width a row's actions move under its text.
const CHECKPOINT_ROW_STACK_WIDTH: f32 = 460.0;
const ROW_INSET: f32 = 12.0;
/// Side of a problem mark set in front of a line on this page, and the
/// advance from its left edge to the text after it: read from the kit.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

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

/// Everything the page draws, read before any of it is drawn, so an action
/// taken on this frame changes the next one rather than half of this one.
struct RecoverySnapshot {
    checkpoints: Vec<ProjectCheckpointSummary>,
    /// Label and reason of each artifact that failed verification.
    set_aside: Vec<(String, String)>,
    error: Option<String>,
    loading: bool,
    creating: bool,
    selected: Option<String>,
    location: String,
    stored_bytes: u64,
    now_unix_ms: Option<u64>,
}

impl RecoverySnapshot {
    fn capture(state: &AppState) -> Self {
        let recovery = &state.dialogs.project_checkpoint_recovery;
        #[cfg(target_arch = "wasm32")]
        let loading = recovery.loading && !recovery.initialized;
        #[cfg(not(target_arch = "wasm32"))]
        let loading = false;
        Self {
            checkpoints: recovery.checkpoints.clone(),
            set_aside: recovery
                .quarantined
                .iter()
                .map(|artifact| (artifact.label().to_owned(), artifact.reason().to_owned()))
                .collect(),
            error: recovery.error.clone(),
            loading,
            creating: manual_checkpoint_pending(),
            selected: state.workbench.project_checkpoint_selection.clone(),
            location: storage_location(state),
            stored_bytes: recovery
                .checkpoints
                .iter()
                .map(ProjectCheckpointSummary::snapshot_byte_len)
                .sum(),
            now_unix_ms: crate::time_compat::checked_unix_time_ms().ok(),
        }
    }

    /// The page's one-word state beside its title, and the mark it earns.
    ///
    /// Counting the checkpoints is the fact; calling them verified was the
    /// page reporting that nothing had gone wrong, in green.
    fn status(&self) -> (String, Option<Problem>) {
        if self.error.is_some() {
            ("unreadable".to_owned(), Some(Problem::Failure))
        } else if self.loading {
            ("loading".to_owned(), None)
        } else if !self.set_aside.is_empty() {
            (
                format!("{} set aside", self.set_aside.len()),
                Some(Problem::Warning),
            )
        } else if self.checkpoints.is_empty() {
            ("no checkpoints".to_owned(), None)
        } else {
            (format!("{} checkpoints", self.checkpoints.len()), None)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_location(state: &AppState) -> String {
    crate::workbench::lifecycle::project_checkpoint::storage_location(state)
        .map_or_else(|error| error, |path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
fn storage_location(_state: &AppState) -> String {
    "This browser's site storage".to_owned()
}

enum RecoveryIntent {
    Create,
    RevisionHistory,
    Reload,
    Select(String),
    Compare(ProjectCheckpointSummary),
    Restore(ProjectCheckpointSummary),
}

impl RecoveryIntent {
    fn execute(self, ctx: &Context, app: &mut RSpiceApp) {
        match self {
            Self::Create => create_manual_checkpoint(ctx, &mut app.state),
            Self::RevisionHistory => Command::RevisionHistory.execute(app),
            Self::Reload => app.state.dialogs.project_checkpoint_recovery.invalidate(),
            Self::Select(id) => app.state.workbench.project_checkpoint_selection = Some(id),
            Self::Compare(checkpoint) => {
                app.state.workbench.project_checkpoint_selection =
                    Some(checkpoint.checkpoint_id().to_string());
                compare_project_checkpoint(ctx, &mut app.state, &checkpoint);
            }
            Self::Restore(checkpoint) => {
                app.state.workbench.project_checkpoint_selection =
                    Some(checkpoint.checkpoint_id().to_string());
                export_project_checkpoint_copy(ctx, &mut app.state, checkpoint);
            }
        }
    }
}

fn keep_first(slot: &mut Option<RecoveryIntent>, candidate: Option<RecoveryIntent>) {
    if slot.is_none() {
        *slot = candidate;
    }
}

pub(super) fn recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_manual_checkpoint(&mut app.state);
    ensure_project_recovery_catalog(ui.ctx(), &mut app.state);
    let snapshot = RecoverySnapshot::capture(&app.state);
    let mut intent = None;
    page::show(
        ui,
        "workbench.project.recovery.page",
        |ui, inset, content_width| {
            keep_first(
                &mut intent,
                recovery_header(ui, &snapshot, inset, content_width),
            );
            ui.add_space(BODY_TOP);
            if content_width >= STACK_BREAKPOINT {
                let mut left_intent = None;
                page::columns(
                    ui,
                    inset,
                    content_width,
                    |left| left_intent = checkpoints_card(left, &snapshot),
                    |right| {
                        storage_card(right, &snapshot);
                        set_aside_card(right, &snapshot);
                    },
                );
                keep_first(&mut intent, left_intent);
            } else {
                page::inset_column(ui, inset, content_width, |ui| {
                    keep_first(&mut intent, checkpoints_card(ui, &snapshot));
                    ui.add_space(CARD_GAP);
                    storage_card(ui, &snapshot);
                    set_aside_card(ui, &snapshot);
                });
            }
        },
    );
    if let Some(intent) = intent {
        intent.execute(&ui.ctx().clone(), app);
    }
}

/// Title, state, the page's one note, and its two actions, above the rule
/// every carded project page opens with.
fn recovery_header(
    ui: &mut Ui,
    snapshot: &RecoverySnapshot,
    inset: f32,
    content_width: f32,
) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(HEADER_TOP);
    let intent = page::inset_column(ui, inset, content_width, |ui| {
        let (status, problem) = snapshot.status();
        let (title_rect, title_response) =
            ui.allocate_exact_size(vec2(content_width, 27.0), Sense::hover());
        let title = ui.painter().text(
            title_rect.left_center(),
            Align2::LEFT_CENTER,
            "Recovery",
            theme::sans(20.0, FontWeight::SemiBold),
            tokens.color.text,
        );
        let status_left = title.right() + 12.0;
        let text_left = if let Some(problem) = problem {
            paint_problem_mark(
                ui.painter(),
                Rect::from_center_size(
                    pos2(status_left + MARK_SIDE * 0.5, title_rect.center().y + 2.0),
                    vec2(MARK_SIDE, MARK_SIDE),
                ),
                problem,
            );
            status_left + MARK_ADVANCE
        } else {
            status_left
        };
        ui.painter().text(
            pos2(text_left, title_rect.center().y + 2.0),
            Align2::LEFT_CENTER,
            &status,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            tokens.color.text_dim,
        );
        let announcement = problem.map_or_else(
            || format!("Recovery · {status}"),
            |problem| problem.announce(&format!("Recovery · {status}")),
        );
        title_response
            .widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &announcement));

        ui.add_space(3.0);
        text_block(ui, RECOVERY_NOTE, body_font(), tokens.color.text_dim);
        ui.add_space(12.0);
        let mut intent = None;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
            let create = Button::new(if snapshot.creating {
                "Creating checkpoint\u{2026}"
            } else {
                "Create checkpoint"
            })
            .icon(crate::ui::icons::Icon::Add)
            .accent()
            .enabled(!snapshot.creating)
            .show(ui);
            if create.clicked() {
                intent = Some(RecoveryIntent::Create);
            }
            if Button::new("Revision history\u{2026}").show(ui).clicked() {
                intent = Some(RecoveryIntent::RevisionHistory);
            }
        });
        intent
    });
    page::header_rule(ui);
    intent
}

fn checkpoints_card(ui: &mut Ui, snapshot: &RecoverySnapshot) -> Option<RecoveryIntent> {
    let meta = if snapshot.checkpoints.is_empty() {
        String::new()
    } else {
        format!(
            "{} of {MAX_RETAINED_CHECKPOINTS} kept",
            snapshot.checkpoints.len()
        )
    };
    page::card(ui, "Checkpoints", &meta, None, |ui| {
        if let Some(error) = &snapshot.error {
            return catalog_error(ui, error);
        }
        if snapshot.loading {
            card_message(ui, "Loading checkpoints\u{2026}", None);
            return None;
        }
        if snapshot.checkpoints.is_empty() {
            card_message(
                ui,
                "No checkpoints yet",
                Some(
                    "RSpice creates one before every technology change. Create one yourself \
                     before an edit you may want to undo.",
                ),
            );
            return None;
        }
        let mut intent = None;
        let count = snapshot.checkpoints.len();
        for (index, checkpoint) in snapshot.checkpoints.iter().enumerate() {
            let selected = snapshot.selected.as_deref()
                == Some(checkpoint.checkpoint_id().to_string().as_str());
            keep_first(
                &mut intent,
                checkpoint_row(
                    ui,
                    checkpoint,
                    selected,
                    snapshot.now_unix_ms,
                    index + 1 < count,
                ),
            );
        }
        intent
    })
}

/// A card's whole body when it has no rows: a title and, optionally, a
/// sentence under it.
fn card_message(ui: &mut Ui, title: &str, detail: Option<&str>) {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(14.0);
    let width = ui.available_width() - 2.0 * (ROW_INSET + 2.0);
    page::inset_column(ui, ROW_INSET + 2.0, width, |ui| {
        text_block(ui, title, title_font(), tokens.color.text);
        if let Some(detail) = detail {
            ui.add_space(4.0);
            text_block(ui, detail, body_font(), tokens.color.text_dim);
        }
    });
    ui.add_space(5.0);
}

fn title_font() -> egui::FontId {
    theme::sans(tokens::FS_2, FontWeight::Medium)
}

fn body_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::Regular)
}

/// A card's title with the kit's mark in front of it, for the one card whose
/// whole body is the problem. The mark carries the severity; the words stay
/// in the page's ordinary title tone.
fn marked_title(ui: &mut Ui, problem: Problem, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    let font = title_font();
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font, tokens.color.text);
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            (galley.size().x + MARK_ADVANCE).min(ui.available_width().max(1.0)),
            galley.size().y.max(MARK_SIDE),
        ),
        Sense::hover(),
    );
    let announcement = problem.announce(text);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &announcement));
    paint_problem_mark(
        ui.painter(),
        Rect::from_center_size(
            pos2(rect.left() + MARK_SIDE * 0.5, rect.center().y),
            vec2(MARK_SIDE, MARK_SIDE),
        ),
        problem,
    );
    ui.painter().galley(
        pos2(rect.left() + MARK_ADVANCE, rect.top()),
        galley,
        tokens.color.text,
    );
}

/// Wrapped copy, painted as text rather than laid out as a selectable label.
fn text_block(ui: &mut Ui, text: &str, font: egui::FontId, color: Color32) {
    let galley = ui
        .painter()
        .layout(text.to_owned(), font, color, ui.available_width().max(1.0));
    let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, color);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), text));
}

fn catalog_error(ui: &mut Ui, error: &str) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(14.0);
    let width = ui.available_width() - 2.0 * (ROW_INSET + 2.0);
    let reload = page::inset_column(ui, ROW_INSET + 2.0, width, |ui| {
        marked_title(ui, Problem::Failure, "Checkpoints could not be read");
        ui.add_space(4.0);
        text_block(
            ui,
            &sentence_case(error),
            body_font(),
            tokens.color.text_dim,
        );
        ui.add_space(10.0);
        Button::new("Try again")
            .icon(crate::ui::icons::Icon::Refresh)
            .show(ui)
            .clicked()
    });
    ui.add_space(5.0);
    reload.then_some(RecoveryIntent::Reload)
}

fn checkpoint_row(
    ui: &mut Ui,
    checkpoint: &ProjectCheckpointSummary,
    selected: bool,
    now_unix_ms: Option<u64>,
    separated: bool,
) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let stacked = width < CHECKPOINT_ROW_STACK_WIDTH;
    let control_height = tokens.metrics.ctl_h.max(if tokens.metrics.is_touch() {
        tokens::TOUCH_TARGET
    } else {
        0.0
    });
    let text_height = 34.0;
    let height = if stacked {
        12.0 + text_height + 6.0 + control_height + 12.0
    } else {
        CHECKPOINT_ROW_HEIGHT.max(control_height + 16.0)
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    if selected {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_active);
        ui.painter().rect_filled(
            Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
            0.0,
            tokens.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    if separated {
        ui.painter().hline(
            (rect.left() + ROW_INSET)..=(rect.right() - ROW_INSET),
            rect.bottom() - 0.5,
            Stroke::new(1.0, tokens.color.border),
        );
    }

    let compare = Button::new("Compare");
    let restore = Button::new("Restore\u{2026}");
    let actions_width = compare.measured_width(ui) + 6.0 + restore.measured_width(ui);
    let actions = if stacked {
        Rect::from_min_size(
            pos2(
                rect.left() + ROW_INSET,
                rect.bottom() - 12.0 - control_height,
            ),
            vec2(actions_width, control_height),
        )
    } else {
        Rect::from_min_max(
            pos2(
                rect.right() - ROW_INSET - actions_width,
                rect.center().y - control_height * 0.5,
            ),
            pos2(
                rect.right() - ROW_INSET,
                rect.center().y + control_height * 0.5,
            ),
        )
    };
    let text_right = if stacked {
        rect.right() - ROW_INSET
    } else {
        actions.left() - 12.0
    };
    let text_top = if stacked {
        rect.top() + 12.0
    } else {
        rect.center().y - text_height * 0.5
    };
    let text_width = (text_right - rect.left() - ROW_INSET).max(1.0);
    let age = checkpoint_age(checkpoint.created_unix_ms(), now_unix_ms);
    paint_elided(
        ui,
        pos2(rect.left() + ROW_INSET, text_top),
        checkpoint.reason().label(),
        theme::sans(tokens::FS_1, FontWeight::Medium),
        tokens.color.text,
        text_width,
    );
    let detail = format!(
        "{age} \u{b7} revision {} \u{b7} {}",
        checkpoint.project_revision(),
        format_bytes(checkpoint.snapshot_byte_len())
    );
    paint_elided(
        ui,
        pos2(rect.left() + ROW_INSET, text_top + 19.0),
        &detail,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        tokens.color.text_faint,
        text_width,
    );

    let mut intent = None;
    // A child, not a scope: a scope moves the list's cursor to the bottom of
    // the buttons, which sit above the bottom of the row, so the next row
    // would start inside this one.
    let mut actions_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(actions)
            .layout(Layout::left_to_right(Align::Center)),
    );
    actions_ui.spacing_mut().item_spacing.x = 6.0;
    if compare
        .accessible_label(&format!(
            "Compare {} with the current project",
            checkpoint.reason().label()
        ))
        .show(&mut actions_ui)
        .on_hover_text("Check whether the current project still matches this checkpoint")
        .clicked()
    {
        intent = Some(RecoveryIntent::Compare(checkpoint.clone()));
    }
    if restore
        .accessible_label(&format!(
            "Restore {} as a new project",
            checkpoint.reason().label()
        ))
        .show(&mut actions_ui)
        .on_hover_text("Save this checkpoint as a new project file")
        .clicked()
    {
        intent = Some(RecoveryIntent::Restore(checkpoint.clone()));
    }

    let row_label = format!("{}, {detail}", checkpoint.reason().label());
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_text(format!(
        "{}\nCreated {}\nCheckpoint {}",
        checkpoint.reason().label(),
        utc_stamp(checkpoint.created_unix_ms()),
        checkpoint.checkpoint_id()
    ));
    if response.clicked() && intent.is_none() {
        intent = Some(RecoveryIntent::Select(
            checkpoint.checkpoint_id().to_string(),
        ));
    }
    intent
}

/// Where checkpoints live and the rules they are kept under.
fn storage_card(ui: &mut Ui, snapshot: &RecoverySnapshot) {
    page::card(ui, "Storage", "", None, |ui| {
        ui.add_space(3.0);
        property_row_path(ui, "Location", &snapshot.location);
        property_row(
            ui,
            "Keeps",
            &format!("The newest {MAX_RETAINED_CHECKPOINTS} checkpoints"),
        );
        property_row(ui, "Space used", &format_bytes(snapshot.stored_bytes));
        // `ProjectCheckpointReason::TechnologyAttachment` is the one
        // checkpoint RSpice takes on its own.
        property_row(ui, "Taken automatically", "Before technology changes");
        // Integrity is a row only when it is not in order. "All checkpoints
        // verified" and "Nothing to check" were the card reporting that
        // nothing had gone wrong, which is what the absence of the row says.
        if snapshot.error.is_some() {
            property_row_problem(ui, "Integrity", "Not checked", Some(Problem::Failure));
        } else if !snapshot.set_aside.is_empty() {
            property_row_problem(
                ui,
                "Integrity",
                &format!("{} damaged, set aside", snapshot.set_aside.len()),
                Some(Problem::Warning),
            );
        }
    });
}

/// Artifacts that failed verification. They are never offered for restore,
/// and the card is drawn only when there is something in it.
fn set_aside_card(ui: &mut Ui, snapshot: &RecoverySnapshot) {
    if snapshot.set_aside.is_empty() {
        return;
    }
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(CARD_GAP);
    page::card(
        ui,
        "Set aside",
        &snapshot.set_aside.len().to_string(),
        Some(Problem::Warning),
        |ui| {
            ui.add_space(10.0);
            let width = ui.available_width() - 2.0 * ROW_INSET;
            page::inset_column(ui, ROW_INSET, width, |ui| {
                text_block(
                    ui,
                    "These files failed verification, so they are never offered for restore.",
                    body_font(),
                    tokens.color.text_dim,
                );
                for (label, reason) in &snapshot.set_aside {
                    ui.add_space(10.0);
                    let (rect, response) =
                        ui.allocate_exact_size(vec2(width, 16.0), Sense::hover());
                    paint_elided(
                        ui,
                        rect.left_top(),
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Medium),
                        tokens.color.text,
                        width,
                    );
                    response.on_hover_text(label.as_str());
                    text_block(
                        ui,
                        &sentence_case(reason),
                        body_font(),
                        tokens.color.text_dim,
                    );
                }
            });
        },
    );
}

fn sentence_case(text: &str) -> String {
    let mut characters = text.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
}

/// A checkpoint's creation time in UTC, for the row's tooltip, in the spelling
/// every stored timestamp in this product is written out in.
fn utc_stamp(created_unix_ms: u64) -> String {
    crate::time_compat::utc_stamp(created_unix_ms)
}

pub(super) fn ensure_project_recovery_catalog(ctx: &Context, state: &mut AppState) {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ctx;
    let project_id = state.workspace.content.project.id().to_string();
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

fn checkpoint_age(created_unix_ms: u64, now_unix_ms: Option<u64>) -> String {
    // A checkpoint stamped after the clock's present says the clock moved,
    // not how old the checkpoint is.
    let Some(elapsed) = now_unix_ms
        .filter(|_| created_unix_ms != 0)
        .and_then(|now| now.checked_sub(created_unix_ms))
    else {
        return "time unavailable".to_owned();
    };
    crate::time_compat::relative_age(std::time::Duration::from_millis(elapsed))
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
            "No changes since this checkpoint",
            "The current project is identical to this checkpoint.",
        ),
        Ok(false) => state.ui.toasts.info_with_title(
            ctx,
            "Changed since this checkpoint",
            format!(
                "The current project differs from checkpoint {} (revision {}).",
                short_identity(&checkpoint.checkpoint_id().to_string()),
                checkpoint.project_revision()
            ),
        ),
        Err(error) => state
            .ui
            .toasts
            .error_with_title(ctx, "Could not compare", error),
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

/// Create a checkpoint the reader asked for. The Console line is its record;
/// a failure also reaches the notice centre through the activity mirror.
#[cfg(not(target_arch = "wasm32"))]
fn create_manual_checkpoint(_: &Context, state: &mut AppState) {
    match crate::workbench::lifecycle::project_checkpoint::create(
        state,
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::Manual,
    ) {
        Ok(checkpoint) => {
            state.dialogs.project_checkpoint_recovery.initialized = false;
            record_manual_checkpoint(state, &checkpoint);
        }
        Err(error) => {
            state.record(LogSeverity::Error, LogSource::Studio, error);
        }
    }
}

fn record_manual_checkpoint(
    state: &mut AppState,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Created project checkpoint {} at revision {}.",
            short_identity(&checkpoint.checkpoint_id().to_string()),
            checkpoint.project_revision()
        ),
    );
}

#[cfg(target_arch = "wasm32")]
fn create_manual_checkpoint(ctx: &Context, state: &mut AppState) {
    let project_id = state.workspace.content.project.id().to_string();
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
        state.record(LogSeverity::Error, LogSource::Studio, error);
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_manual_checkpoint(state: &mut AppState) {
    let current_project_id = state.workspace.content.project.id().to_string();
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
                record_manual_checkpoint(state, &checkpoint);
            }
            Err(error) => {
                state.record(LogSeverity::Error, LogSource::Studio, error);
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

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn export_project_checkpoint_copy(
    _: &Context,
    state: &mut AppState,
    checkpoint: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    let filename = recovery_copy_filename(&checkpoint);
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
            state.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!("Saved a recovery copy to {}.", destination.display()),
            );
        }
        Err(error) => {
            state.record(LogSeverity::Error, LogSource::Studio, error);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn export_project_checkpoint_copy(
    ctx: &Context,
    state: &mut AppState,
    checkpoint: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    let filename = recovery_copy_filename(&checkpoint);
    let project_id = state.workspace.content.project.id().to_string();
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
