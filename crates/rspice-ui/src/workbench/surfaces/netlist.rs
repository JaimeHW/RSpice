//! Mockup-owned Code & Automation netlist document surface.
//!
//! The center well is deliberately flat: one 33-point document toolbar over
//! an exact-entry editor. Generated and owned source are independent retained
//! documents and switching between them never deletes either one.
//!
//! This module owns the surface's frame — reconciliation entry, the review
//! banner, file drop, deterministic formatting, and the readiness predicates
//! every other part reads. Each submodule owns one transaction of its own.

mod generation;
mod language;
mod ownership;
mod revision;
mod search;
mod toolbar;
mod transfer;

use egui::Ui;

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::{WorkbenchIcon, empty_state, empty_state_with_actions};
use crate::workbench::documents::netlist_document::{
    ActiveNetlistDocument, ExecutedDeckVerification, source_content_digest,
};
use crate::workbench::{AppState, MessageId, RSpiceApp};

pub(super) fn prepare_workspace(app: &mut RSpiceApp) {
    generation::reconcile_documents(app);
    crate::workbench::documents::netlist_document::prepare(&mut app.state);
}

pub(super) fn show_prepared(ui: &mut Ui, app: &mut RSpiceApp) {
    handle_netlist_file_drop(ui.ctx(), app);
    let toolbar = toolbar::code_toolbar(ui, app);
    // The strip aligns to the toolbar's own content rect, so the two rows keep
    // one padding and one edge rather than each deriving a width. The strip
    // states the run and reports what was clicked; routing that click is this
    // surface's transaction, not the painter's.
    match run_strip(ui, &app.state, toolbar.content) {
        Some(RunStripAction::OpenDeckSnapshot) => {
            if !crate::workbench::documents::netlist_document::open_run_deck_snapshot(
                &mut app.state,
            ) {
                app.state.push_user_message(ConsoleMessage::warning(
                    "The deck this run used is no longer retained in this session.",
                ));
            }
        }
        Some(RunStripAction::Compare) => {
            if let Err(error) =
                crate::workbench::documents::netlist_document::compare_run_deck_snapshot(
                    &mut app.state,
                )
            {
                app.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
        Some(RunStripAction::OpenTaskDeck { run_id, point }) => {
            if !crate::workbench::documents::netlist_document::open_executed_deck(
                &mut app.state,
                run_id,
                point,
            ) {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "The decks Run {run_id} executed are no longer retained in this project."
                )));
            }
        }
        Some(RunStripAction::OpenInResults(run_id)) => {
            app.state.simulation.select_run_by_sequence(run_id);
            crate::workbench::commands::vocabulary::Command::OpenWorkspace(
                crate::workbench::state::Workspace::Results,
            )
            .execute(app);
        }
        None => {}
    }
    execution_profile_review_banner(ui, app);
    control_disposition_band(ui, &app.state);
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui,
        crate::workbench::documents::netlist_document::editor_id(&app.state),
    ) {
        format_owned_netlist(ui.ctx(), app);
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        if generated_primary_unavailable(&app.state) {
            let messages = app.state.ui.messages();
            if app.state.workspace.netlist_source.is_none()
                && app.state.ui.netlist.generated_document.is_none()
            {
                let mut action = None;
                empty_state_with_actions(
                    ui,
                    WorkbenchIcon::Netlist,
                    &messages.text(MessageId::NetlistEmptyWorkspaceTitle),
                    &messages.text(MessageId::NetlistEmptyWorkspaceDescription),
                    |ui| {
                        if ui
                            .button(messages.text(MessageId::NetlistNewTopDeck))
                            .clicked()
                        {
                            action = Some(EmptyNetlistAction::NewTopDeck);
                        }
                        if ui
                            .button(messages.text(MessageId::NetlistImportDeck))
                            .clicked()
                        {
                            action = Some(EmptyNetlistAction::ImportDeck);
                        }
                    },
                );
                match action {
                    Some(EmptyNetlistAction::NewTopDeck) => {
                        let result = crate::workbench::app::open_source_document_dialog(
                            &mut app.state,
                        )
                        .and_then(|()| {
                            crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action(
                                &mut app.state,
                                crate::workbench::documents::code_workspace::CodeSourceFileAction::New,
                            )
                        });
                        if let Err(error) = result {
                            app.state.push_user_message(ConsoleMessage::error(error));
                        }
                    }
                    Some(EmptyNetlistAction::ImportDeck) => {
                        crate::workbench::commands::vocabulary::Command::ImportNetlist.execute(app);
                    }
                    None => {}
                }
            } else {
                let title = messages.text(MessageId::NetlistGeneratedUnavailable);
                let default_description =
                    messages.text(MessageId::NetlistGeneratedUnavailableDescription);
                empty_state(
                    ui,
                    WorkbenchIcon::Netlist,
                    &title,
                    app.state
                        .ui
                        .netlist
                        .generation_error
                        .as_deref()
                        .unwrap_or(&default_description),
                );
            }
        } else {
            crate::workbench::documents::netlist_document::show_editor(ui, &mut app.state);
        }
    });
    search::find_replace_window(ui.ctx(), app);
    language::rename_dialog_window(ui.ctx(), app);
    ownership::ownership_dialog_window(ui.ctx(), app);
    revision::comparison_dialog_window(ui.ctx(), app);
    revision::save_source_dialog_window(ui.ctx(), app);
    transfer::external_change_dialog_window(ui.ctx(), app);
    transfer::export_generated_dialog_window(ui.ctx(), app);
    transfer::import_review_dialog_window(ui.ctx(), app);
}

#[derive(Clone, Copy)]
enum EmptyNetlistAction {
    NewTopDeck,
    ImportDeck,
}

/// Height of the post-run strip, matching the mockup's `.netlist-override-strip`.
pub(super) const RUN_STRIP_HEIGHT: f32 = 24.0;
const RUN_STRIP_PADDING_X: f32 = 8.0;
const RUN_STRIP_GAP: f32 = 8.0;
const RUN_STRIP_ACTION_HEIGHT: f32 = 20.0;
const RUN_STRIP_DOT_DIAMETER: f32 = 5.0;
const RUN_STRIP_DOT_GAP: f32 = 6.0;
/// Width a live-state dot takes out of the chip, gap included.
const RUN_STRIP_DOT_COLUMN: f32 = RUN_STRIP_DOT_DIAMETER + RUN_STRIP_DOT_GAP;

/// Which statement the strip is making about the run it names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RunStripPhase {
    /// The working deck is byte-for-byte the deck this run executed.
    Current,
    /// The working deck has moved on since this run executed.
    Edited,
    /// This run is executing the deck now.
    Running,
    /// The strip is the run snapshot document's own header.
    Snapshot,
}

/// Everything the strip states, read back off the run's own receipt.
///
/// The strip keeps no copy of the run's identity: if the run is no longer
/// retained, or was not a manual deck run, there is nothing to state and the
/// strip does not appear.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RunStripProjection {
    pub run_id: u64,
    pub deck_digest: String,
    pub revision: u64,
    pub phase: RunStripPhase,
    /// Which point of the run the document is showing, when it is showing an
    /// executed deck rather than this session's manual baseline. A run's deck
    /// digest describes the source it was authorized over, not the per-point
    /// source a corner actually solved, so naming the point is the only way
    /// the header can be a true statement about what is below it.
    pub point: Option<String>,
    /// Where that point sits among the run's retained decks, and how many
    /// there are. Zero tasks means this session holds none of the run's decks,
    /// which is what makes the hop to them absent rather than offered and then
    /// refused.
    pub point_index: usize,
    pub tasks: usize,
    /// What the deck on screen was checked to be, when there is one. Absent
    /// while the strip is describing the manual baseline, which is the deck
    /// somebody typed and was never sealed on a receipt.
    pub verification: Option<ExecutedDeckVerification>,
}

pub(super) fn run_strip_projection(state: &AppState) -> Option<RunStripProjection> {
    use crate::workbench::documents::netlist_document::{
        effective_active_document, working_deck_source,
    };

    let active = effective_active_document(state);
    let (run_id, phase) = if active == ActiveNetlistDocument::RunSnapshot {
        // Which run the snapshot belongs to is whichever binding put it there.
        // `last_run_id` is the *manual baseline* binding, and an executed deck
        // is opened from a run that never touched it — reading only that one
        // left the header silent about exactly the runs this viewer was added
        // for.
        let run_id = match state.ui.netlist.executed_deck_view {
            Some(selection) => selection.run_id,
            None => state.ui.netlist.last_run_id?,
        };
        (run_id, RunStripPhase::Snapshot)
    } else {
        if state.ui.netlist.active_dependency_identity.is_some()
            || !matches!(
                active,
                ActiveNetlistDocument::Generated | ActiveNetlistDocument::OwnedSource
            )
        {
            return None;
        }
        match state.ui.netlist.pending_manual_run_id {
            Some(running) => (running, RunStripPhase::Running),
            None => {
                let run_id = state.ui.netlist.last_run_id?;
                let baseline = state.ui.netlist.last_run_buffer.as_deref()?;
                let phase = if working_deck_source(state) == baseline {
                    RunStripPhase::Current
                } else {
                    RunStripPhase::Edited
                };
                (run_id, phase)
            }
        }
    };
    // The manual-deck filter belongs to the *baseline* half of this strip: a
    // schematic run has no editor buffer to be current with or edited against,
    // so there is nothing to state about it there. An executed deck is a
    // statement about the run itself, which every run can make.
    let point = state.ui.netlist.executed_deck_view.and_then(|selection| {
        state
            .simulation
            .executed_decks
            .get(selection.run_id)
            .and_then(|deck| deck.point(selection.point))
            .map(|point| point.label.clone())
    });
    let receipt = state
        .simulation
        .run_by_sequence(run_id)?
        .prepared_receipt()
        .filter(|receipt| {
            point.is_some()
                || receipt.source_domain() == crate::state::AnalysisResultSourceDomain::ManualDeck
        })?;
    // The run named by the strip, not the run whose deck is open: in the
    // baseline phases they are the same run, and this is what decides whether
    // the hop to its task decks can be offered at all.
    let tasks = state
        .simulation
        .executed_decks
        .get(run_id)
        .map_or(0, |deck| deck.points.len());
    // Tied to the point actually resolving: a selection whose run the archive
    // has since dropped shows no deck, and a verdict about bytes nobody can
    // see is worse than silence.
    let verification = point.as_ref().and_then(|_| {
        state
            .ui
            .netlist
            .executed_deck_view
            .map(|selection| selection.verification)
    });
    Some(RunStripProjection {
        run_id,
        deck_digest: receipt
            .source_content_digest()
            .to_string()
            .chars()
            .take(12)
            .collect(),
        revision: receipt.project_revision().get(),
        phase,
        point,
        point_index: state
            .ui
            .netlist
            .executed_deck_view
            .map_or(0, |selection| selection.point),
        tasks,
        verification,
    })
}

/// What the user asked the strip for, for the surface to carry out.
#[derive(Clone, Copy)]
enum RunStripAction {
    OpenDeckSnapshot,
    OpenInResults(u64),
    Compare,
    /// Open the exact source one dispatched task's engine read. The point
    /// index travels with the request because the strip is the only thing that
    /// knows which point the reader was looking at.
    OpenTaskDeck {
        run_id: u64,
        point: usize,
    },
}

/// One 24-point row between the deck toolbar and the editor: what ran, what it
/// ran over, and the two routes out of it. While a run is in flight the row
/// states that and offers nothing that could race it.
fn run_strip(ui: &mut Ui, state: &AppState, toolbar_content: egui::Rect) -> Option<RunStripAction> {
    let projection = run_strip_projection(state)?;
    let messages = state.ui.messages();
    let run_id = projection.run_id.to_string();
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(
            toolbar_content.width() + RUN_STRIP_PADDING_X * 2.0,
            RUN_STRIP_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    let content = egui::Rect::from_x_y_ranges(toolbar_content.x_range(), rect.y_range());

    let mut action = None;
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = RUN_STRIP_GAP;
    // An in-flight run offers nothing, and an empty right-to-left child does
    // not report a right-hand edge — the statement takes the whole row rather
    // than measuring itself against a cursor that never moved.
    let offers_actions = projection.phase != RunStripPhase::Running;
    match projection.phase {
        RunStripPhase::Running => {}
        RunStripPhase::Snapshot => {
            let open_run =
                messages.format(MessageId::NetlistRunSnapshotOpenRun, &[("id", &run_id)]);
            if strip_button(&mut actions, &open_run, None).clicked() {
                action = Some(RunStripAction::OpenInResults(projection.run_id));
            }
            if let Some(next) = task_deck_hop(&projection) {
                if let Some(hop) = task_deck_button(&mut actions, &messages, &projection, next) {
                    action = Some(hop);
                }
            }
            // An executed deck has no working copy: it is the source a point
            // was handed after expansion and corner materialization, and a
            // diff against the deck being edited would report every one of
            // those as an edit. The control is absent rather than offered and
            // then refused.
            if projection.point.is_none() {
                let compare = messages.text(MessageId::NetlistRunSnapshotCompare);
                let compare_hint = messages.text(MessageId::NetlistRunSnapshotCompareTooltip);
                if strip_button(&mut actions, &compare, Some(&compare_hint)).clicked() {
                    action = Some(RunStripAction::Compare);
                }
            }
        }
        RunStripPhase::Current | RunStripPhase::Edited => {
            let results = messages.text(MessageId::NetlistRunStripOpenResults);
            if strip_button(&mut actions, &results, None).clicked() {
                action = Some(RunStripAction::OpenInResults(projection.run_id));
            }
            let deck = messages.text(MessageId::NetlistRunStripOpenDeck);
            if strip_button(&mut actions, &deck, None).clicked() {
                action = Some(RunStripAction::OpenDeckSnapshot);
            }
            if let Some(next) = task_deck_hop(&projection) {
                if let Some(hop) = task_deck_button(&mut actions, &messages, &projection, next) {
                    action = Some(hop);
                }
            }
        }
    }
    let statement_right = if offers_actions {
        (actions.min_rect().left().min(content.right()) - RUN_STRIP_GAP).max(content.left())
    } else {
        content.right()
    };
    let statement = egui::Rect::from_min_max(
        content.left_top(),
        egui::pos2(statement_right, content.bottom()),
    );

    let (chip, tone, chip_hint) = match projection.phase {
        RunStripPhase::Current => (
            messages.format(MessageId::NetlistRunStripRun, &[("id", &run_id)]),
            t.color.ok,
            MessageId::NetlistRunStripCurrentTooltip,
        ),
        RunStripPhase::Edited => (
            messages.format(MessageId::NetlistRunStripRun, &[("id", &run_id)]),
            t.color.warn,
            MessageId::NetlistRunStripEditedTooltip,
        ),
        RunStripPhase::Running => (
            messages.format(MessageId::NetlistRunStripRunning, &[("id", &run_id)]),
            t.color.accent,
            MessageId::NetlistRunStripRunningTooltip,
        ),
        RunStripPhase::Snapshot => (
            messages.format(MessageId::NetlistRunSnapshotImmutable, &[("id", &run_id)]),
            t.color.ok,
            MessageId::NetlistRunStripCurrentTooltip,
        ),
    };
    let revision = projection.revision.to_string();
    let (full, short) = match projection.point.as_deref() {
        Some(point) => (
            messages.format(
                MessageId::NetlistRunStripExecutedPoint,
                &[("point", point), ("revision", &revision)],
            ),
            messages.format(
                MessageId::NetlistRunStripExecutedPointShort,
                &[("point", point), ("revision", &revision)],
            ),
        ),
        None => (
            messages.format(
                MessageId::NetlistRunStripIdentity,
                &[("digest", &projection.deck_digest), ("revision", &revision)],
            ),
            messages.format(
                MessageId::NetlistRunStripIdentityShort,
                &[("digest", &projection.deck_digest), ("revision", &revision)],
            ),
        ),
    };
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let text_width = |label: &str| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), font.clone(), t.color.text_dim)
            .size()
            .x
    };
    // A dot marks LIVE state only. A finished run's currentness is static, so
    // it is carried by the tone of the text itself and nothing else.
    let live = projection.phase == RunStripPhase::Running;
    // A verdict about the deck on screen ranks with the run's identity, not
    // with the sentence about it: the sentence yields first, and the badge
    // yields only with the row.
    let badge = projection
        .verification
        .map(|verification| verification_chip(verification, &t));
    // The identifiers never clip: the labelled sentence yields to the two
    // identifiers alone, and those yield the row entirely rather than print a
    // half digest.
    let badge_width = badge.map_or(0.0, |(label, _, _)| {
        text_width(&messages.text(label)) + RUN_STRIP_GAP
    });
    let chip_width =
        if live { RUN_STRIP_DOT_COLUMN } else { 0.0 } + text_width(&chip) + badge_width;
    let copy_budget = statement.width() - chip_width - RUN_STRIP_GAP;
    let copy = if text_width(&full) <= copy_budget {
        Some((full.clone(), false))
    } else if text_width(&short) <= copy_budget {
        Some((short, true))
    } else {
        None
    };

    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(statement)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    row.spacing_mut().item_spacing.x = RUN_STRIP_GAP;
    run_strip_chip(&mut row, &chip, tone, live)
        .on_hover_text(messages.format(chip_hint, &[("id", &run_id)]));
    if let Some((label, hint, color)) = badge {
        run_strip_chip(&mut row, &messages.text(label), color, false)
            .on_hover_text(messages.format(hint, &[("id", &run_id)]));
    }
    if let Some((label, abbreviated)) = copy {
        let copy_response = row.label(
            egui::RichText::new(&label)
                .font(font)
                .color(t.color.text_dim),
        );
        if abbreviated {
            copy_response.on_hover_text(&full);
        }
    }

    action
}

/// Which task deck the strip's hop would open, or `None` when there is none
/// to offer.
///
/// A run whose decks this session does not hold offers nothing: the route
/// would be a control that refuses. Showing the run's own baseline, the hop
/// lands on its first task; showing a task already, it advances and wraps, so
/// a sweep's points are reachable one row at a time without a picker the strip
/// has no width for.
const fn task_deck_hop(projection: &RunStripProjection) -> Option<usize> {
    if projection.tasks == 0 {
        return None;
    }
    match projection.verification {
        None => Some(0),
        Some(_) if projection.tasks > 1 => Some((projection.point_index + 1) % projection.tasks),
        Some(_) => None,
    }
}

/// The hop control, labelled by whether it opens a run's decks or steps
/// through them.
fn task_deck_button(
    ui: &mut Ui,
    messages: &crate::workbench::localization::MessageCatalog,
    projection: &RunStripProjection,
    next: usize,
) -> Option<RunStripAction> {
    let run_id = projection.run_id.to_string();
    let (label, hint) = if projection.verification.is_some() {
        let index = (projection.point_index + 1).to_string();
        let count = projection.tasks.to_string();
        let arguments = [("index", index.as_str()), ("count", count.as_str())];
        (
            messages.format(MessageId::NetlistRunStripNextTask, &arguments),
            messages.format(MessageId::NetlistRunStripNextTaskTooltip, &arguments),
        )
    } else {
        (
            messages.text(MessageId::NetlistRunStripOpenTaskDeck),
            messages.format(
                MessageId::NetlistRunStripOpenTaskDeckTooltip,
                &[("id", run_id.as_str())],
            ),
        )
    };
    strip_button(ui, &label, Some(&hint))
        .clicked()
        .then_some(RunStripAction::OpenTaskDeck {
            run_id: projection.run_id,
            point: next,
        })
}

/// What the strip says about the deck below it, and in what tone.
///
/// One of these four is a claim of verification and the other three are not,
/// and the tones say which is which before the words are read.
fn verification_chip(
    verification: ExecutedDeckVerification,
    t: &Tokens,
) -> (MessageId, MessageId, egui::Color32) {
    match verification {
        ExecutedDeckVerification::Verified => (
            MessageId::NetlistRunStripDeckVerified,
            MessageId::NetlistRunStripDeckVerifiedTooltip,
            t.color.ok,
        ),
        ExecutedDeckVerification::PointVariant => (
            MessageId::NetlistRunStripDeckPointVariant,
            MessageId::NetlistRunStripDeckPointVariantTooltip,
            t.color.text_dim,
        ),
        ExecutedDeckVerification::Unmatched => (
            MessageId::NetlistRunStripDeckUnmatched,
            MessageId::NetlistRunStripDeckUnmatchedTooltip,
            t.color.err,
        ),
        ExecutedDeckVerification::NotRecorded => (
            MessageId::NetlistRunStripDeckNotRecorded,
            MessageId::NetlistRunStripDeckNotRecordedTooltip,
            t.color.text_dim,
        ),
    }
}

/// The strip's status chip: toned text, and a dot only while the run is live.
fn run_strip_chip(ui: &mut Ui, label: &str, color: egui::Color32, live: bool) -> egui::Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = RUN_STRIP_DOT_GAP;
        if live {
            let (dot, _) = ui.allocate_exact_size(
                egui::vec2(RUN_STRIP_DOT_DIAMETER, 11.0),
                egui::Sense::hover(),
            );
            ui.painter()
                .circle_filled(dot.center(), RUN_STRIP_DOT_DIAMETER / 2.0, color);
        }
        ui.label(
            egui::RichText::new(label)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        )
    })
    .inner
}

fn strip_button(ui: &mut Ui, label: &str, hint: Option<&str>) -> egui::Response {
    let response = ui.add(
        egui::Button::new(
            egui::RichText::new(label).font(theme::sans(tokens::FS_0, FontWeight::Medium)),
        )
        .min_size(egui::vec2(0.0, RUN_STRIP_ACTION_HEIGHT)),
    );
    if let Some(hint) = hint {
        response.on_hover_text(hint)
    } else {
        response
    }
}

/// The banner recipe every advisory band above the editor shares: one tinted
/// row that states a condition of the document, tinted by its own tone.
fn advisory_band(ui: &mut Ui, tone: egui::Color32, contents: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(tone.gamma_multiply(0.10))
        .stroke(egui::Stroke::new(1.0, tone.gamma_multiply(0.65)))
        .inner_margin(8)
        .show(ui, |ui| {
            ui.horizontal_wrapped(contents);
        });
}

fn execution_profile_review_banner(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(descriptor) = app
        .state
        .workspace
        .netlist_descriptor
        .as_ref()
        .filter(|descriptor| descriptor.execution_profile_review_required())
    else {
        return;
    };
    let dialect = descriptor
        .imported_dialect
        .unwrap_or(crate::state::NetlistSourceDialect::RSpice);
    let messages = app.state.ui.messages();
    let description = messages.format(
        MessageId::NetlistProfileReviewRequiredDescription,
        &[("dialect", dialect.label())],
    );
    let t = Tokens::get(ui.ctx());
    let mut review = false;
    advisory_band(ui, t.color.warn, |ui| {
        ui.label(
            egui::RichText::new(messages.text(MessageId::NetlistProfileReviewRequired))
                .strong()
                .color(t.color.warn),
        );
        ui.label(description);
        review = ui
            .button(messages.text(MessageId::NetlistProfileReviewAction))
            .clicked();
    });
    if review {
        crate::workbench::workflows::netlist_workflow::begin_owned_netlist_profile_review(
            &mut app.state,
        );
    }
}

/// What became of the active document's `.control` region, stated once above
/// the deck.
///
/// The per-command verdicts already reach the gutter and Problems through the
/// parse; this band exists so the loss is visible without hunting for the
/// block, and it counts the same published diagnostics those rows read rather
/// than deriving a second tally from the buffer.
fn control_disposition_band(ui: &mut Ui, state: &AppState) {
    let Some(summary) = crate::workbench::documents::netlist_document::control_disposition_summary(
        &state.ui.netlist.diagnostics,
    ) else {
        return;
    };
    let messages = state.ui.messages();
    let promoted = summary.promoted.to_string();
    let dropped = summary.dropped.to_string();
    let arguments = [
        ("promoted", promoted.as_str()),
        ("dropped", dropped.as_str()),
    ];
    let t = Tokens::get(ui.ctx());
    // Nothing was lost when every command was promoted, so the band reports
    // rather than warns; amber is reserved for the deck that gave something up.
    let tone = if summary.dropped > 0 {
        t.color.warn
    } else {
        t.color.text_dim
    };
    let mut hover = messages.format(MessageId::NetlistControlRegionDetail, &arguments);
    for example in &summary.examples {
        hover.push('\n');
        hover.push_str(example);
    }
    advisory_band(ui, tone, |ui| {
        ui.label(
            egui::RichText::new(messages.text(MessageId::NetlistControlRegionDisposition))
                .strong()
                .color(tone),
        )
        .on_hover_text(&hover);
        ui.label(
            egui::RichText::new(messages.format(MessageId::NetlistControlRegionCounts, &arguments))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
        )
        .on_hover_text(&hover);
    });
}

fn handle_netlist_file_drop(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.application_modal_open() {
        return;
    }
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return;
    }
    if dropped.len() != 1 {
        app.state.push_user_message(ConsoleMessage::warning(
            "Drop one SPICE deck or RSpice netlist bundle at a time so the staged import review has one exact source identity.",
        ));
        return;
    }
    let file = &dropped[0];
    let source_path = file.path.clone();
    let display_name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            source_path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "dropped-netlist.spice".to_owned());
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            source_path
                .as_deref()
                .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("Browser drop did not provide immutable file bytes.".to_owned())
        }
    };
    match bytes {
        Ok(bytes) => {
            crate::workbench::workflows::netlist_workflow::stage_dropped_netlist_import(
                &mut app.state,
                bytes,
                source_path,
                display_name,
            );
        }
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Dropped SPICE source could not be read: {error}"
        ))),
    }
}

fn format_owned_netlist(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !crate::workbench::documents::netlist_document::active_netlist_source_is_editable(&app.state)
    {
        return;
    }
    let source = app.state.simulation.netlist_content.clone();
    let dependency_document = app.state.ui.netlist.active_dependency_identity.is_some();
    if dependency_document {
        let has_errors = app.state.ui.netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        });
        if has_errors {
            app.state.push_user_message(ConsoleMessage::error(
                "Include formatting is blocked until the current document has no syntax errors.",
            ));
            return;
        }
        let formatted = normalize_owned_netlist_whitespace(&source);
        if formatted == source {
            app.state.push_user_message(ConsoleMessage::info(
                "The owned include already matches the deterministic source format.",
            ));
        } else if crate::workbench::documents::netlist_document::replace_owned_dependency_source(
            &mut app.state,
            formatted,
        ) {
            app.state.push_user_message(ConsoleMessage::info(
                "Formatted the exact project-owned include revision; root-deck validation was invalidated.",
            ));
        }
        return;
    }
    let digest = source_content_digest(&source);
    let validation_current = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
        });
    if !validation_current {
        crate::workbench::workflows::netlist_workflow::validate_visible_netlist_source(app);
    }
    let validation_current = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
        });
    if !validation_current {
        let message = app
            .state
            .ui
            .netlist
            .validation_error
            .clone()
            .unwrap_or_else(|| {
                "Formatting is blocked until the exact owned source passes executable validation."
                    .to_owned()
            });
        app.state
            .push_user_message(ConsoleMessage::error(message.clone()));
        app.state
            .ui
            .toasts
            .error_with_title(ctx, "Netlist format blocked", message);
        return;
    }

    let formatted = normalize_owned_netlist_whitespace(&source);
    if formatted == source {
        app.state.push_user_message(ConsoleMessage::info(
            "The owned netlist already matches the deterministic source format.",
        ));
        return;
    }
    if crate::workbench::documents::netlist_document::replace_owned_source(
        &mut app.state,
        formatted,
    ) {
        app.state.push_user_message(ConsoleMessage::info(
            "Formatted the exact owned netlist revision; prior validation was invalidated.",
        ));
    } else {
        app.state.push_user_message(ConsoleMessage::error(
            "The owned netlist changed before formatting could commit. Review the current revision and retry.",
        ));
    }
}

fn normalize_owned_netlist_whitespace(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let preferred_eol = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut formatted = String::with_capacity(source.len().saturating_add(preferred_eol.len()));
    for (line, segment) in source.split_inclusive('\n').enumerate() {
        let (body, eol) = segment.strip_suffix("\r\n").map_or_else(
            || {
                segment
                    .strip_suffix('\n')
                    .map_or((segment, ""), |body| (body, "\n"))
            },
            |body| (body, "\r\n"),
        );
        // The first physical card is the circuit title and is user data, not
        // executable whitespace. Preserve it byte-for-byte.
        if line == 0 {
            formatted.push_str(body);
        } else {
            formatted.push_str(body.trim_end_matches([' ', '\t']));
        }
        formatted.push_str(eol);
    }
    if !source.ends_with('\n') {
        formatted.push_str(preferred_eol);
    }
    formatted
}

fn generated_primary_unavailable(state: &AppState) -> bool {
    state.ui.netlist.active_dependency_identity.is_none()
        && state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && !generated_primary_ready(state)
}

fn generated_primary_ready(state: &AppState) -> bool {
    state.ui.netlist.generated_document.is_some() && !state.ui.netlist.generated_source.is_empty()
}

fn active_document_available(state: &AppState) -> bool {
    if state.ui.netlist.active_dependency_identity.is_some() {
        return crate::workbench::documents::netlist_document::active_dependency(state).is_some();
    }
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => generated_primary_ready(state),
        ActiveNetlistDocument::OwnedSource => state.workspace.netlist_source.is_some(),
        ActiveNetlistDocument::GeneratedDiff => !state.ui.netlist.generated_diff_source.is_empty(),
        ActiveNetlistDocument::RunSnapshot => {
            crate::workbench::documents::netlist_document::run_deck_snapshot_run_id(state).is_some()
        }
    }
}

fn generation_block_reason(state: &AppState) -> String {
    state
        .ui
        .netlist
        .generation_error
        .clone()
        .unwrap_or_else(|| {
            state
                .ui
                .messages()
                .text(MessageId::NetlistGenerateBeforeAction)
        })
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentStatusTone {
    Valid,
    Warning,
    Error,
}

fn document_syntax_status(state: &AppState) -> (String, DocumentStatusTone) {
    let messages = state.ui.messages();
    if state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return (
            messages.text(MessageId::NetlistComparisonReady),
            DocumentStatusTone::Valid,
        );
    }
    if state.ui.netlist.active_document == ActiveNetlistDocument::RunSnapshot {
        return (
            messages.text(MessageId::NetlistRunSnapshotStatus),
            DocumentStatusTone::Valid,
        );
    }
    if state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && (state.ui.netlist.generation_error.is_some()
            || !generated_primary_ready(state)
            || state.ui.netlist.generated_input_digest
                != state.ui.netlist.current_generation_input_digest)
    {
        let retained_artifact = generated_primary_ready(state);
        return (
            if retained_artifact {
                messages.text(MessageId::NetlistGenerationStaleBlocked)
            } else {
                messages.text(MessageId::NetlistGenerationBlocked)
            },
            DocumentStatusTone::Warning,
        );
    }
    let errors = state
        .ui
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        })
        .count();
    if errors > 0 {
        let count = errors.to_string();
        return (
            messages.format(
                if errors == 1 {
                    MessageId::NetlistSyntaxErrorSingular
                } else {
                    MessageId::NetlistSyntaxErrors
                },
                &[("count", &count)],
            ),
            DocumentStatusTone::Error,
        );
    }
    (
        messages.text(MessageId::NetlistSyntaxValid),
        DocumentStatusTone::Valid,
    )
}
#[cfg(test)]
mod tests {
    use egui::vec2;

    use super::*;

    #[test]
    fn deterministic_netlist_format_preserves_line_ending_policy() {
        assert_eq!(
            normalize_owned_netlist_whitespace("deck  \r\nR1 1 0 1k\t\r\n.end"),
            "deck  \r\nR1 1 0 1k\r\n.end\r\n"
        );
        assert_eq!(
            normalize_owned_netlist_whitespace("deck\n.end\n"),
            "deck\n.end\n"
        );
    }

    fn configured_veriloga_state() -> (AppState, String) {
        let mut state = AppState::default();
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "sealed_gain".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            crate::state::ProjectSourceLanguage::VerilogA,
            "behavioral/gain.va",
            "`ifdef NEVER\n`include \"behavioral/inactive.va\"\n`endif\n`include \"behavioral/gain_constants.va\"\nmodule sealed_gain(p, n); inout p, n; electrical p, n; analog I(p,n) <+ `RSPICE_GAIN * V(p,n); endmodule\n",
            [
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/gain_constants.va",
                    "`define RSPICE_GAIN 1.0\n",
                )
                .expect("valid included source"),
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/inactive.va",
                    "module must_not_enter_provenance; endmodule\n",
                )
                .expect("valid inactive source"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/gain_constants.va",
                )
                .expect("valid dependency edge"),
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/inactive.va",
                )
                .expect("valid inactive dependency edge"),
            ],
        )
        .expect("valid source closure");
        state
            .workspace
            .project_sources
            .insert_bundle(bundle)
            .expect("attach cell-view source");

        let mut placed = crate::state::LibraryCellInstance::new("behavioral", "gain", "schematic");
        placed.terminal_order = vec!["p".to_owned(), "n".to_owned()];
        state
            .schematic
            .add_library_cell_component(crate::state::Point::new(20, 20), placed);
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: crate::state::CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Mixed-signal design".to_owned(),
            })
            .expect("create mixed-signal configuration");
        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            )
            .expect("resolve configured behavioral view");
        let source_key = projection
            .plan()
            .binding(&crate::state::InstancePath::parse("/X1").expect("fixture instance path"))
            .and_then(|binding| binding.project_veriloga())
            .expect("project Verilog-A binding")
            .source_key()
            .to_owned();
        (state, source_key)
    }

    pub(super) fn retain_generated(state: &mut AppState, source: &str) {
        let input_digest = crate::product::ContentDigest::from_bytes([0x41; 32]);
        let source = source.to_owned();
        let (document, owned) =
            generation::publish_generated_document(state, input_digest, source.clone())
                .expect("canonical generated fixture");
        state.ui.netlist.generated_source = source;
        state.ui.netlist.generated_document = Some(document);
        state.ui.netlist.owned_document = owned;
        state.ui.netlist.generated_input_digest = Some(input_digest);
        state.ui.netlist.current_generation_input_digest = Some(input_digest);
    }

    #[test]
    fn editable_source_and_generated_primary_coexist_without_overwrite() {
        let mut state = AppState::default();
        retain_generated(&mut state, "generated\n.end\n");
        let retained_generated = state.ui.netlist.generated_source.clone();
        state.simulation.netlist_content = state.ui.netlist.generated_source.clone();

        ownership::create_owned_source(
            &mut state,
            "top_override.sp",
            crate::state::OwnedNetlistEditStrategy::OwnedSource,
        )
        .expect("create owned source");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_generated.as_str())
        );
        assert!(
            crate::workbench::documents::netlist_document::replace_owned_source(
                &mut state,
                "owned edit\n.end\n".to_owned()
            )
        );

        assert!(crate::workbench::documents::netlist_document::open_generated_primary(&mut state));
        assert_eq!(state.simulation.netlist_content, retained_generated);
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("owned edit\n.end\n")
        );
        assert!(!crate::workbench::documents::netlist_document::open_generated_primary(&mut state));
    }

    #[test]
    fn opening_existing_owned_source_never_overwrites_its_bytes() {
        let mut state = AppState::default();
        retain_generated(&mut state, "new generated\n.end\n");
        state.workspace.netlist_source = Some("retained owned\n.end\n".to_owned());

        assert!(ownership::open_owned_source(&mut state));
        assert_eq!(state.simulation.netlist_content, "retained owned\n.end\n");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("retained owned\n.end\n")
        );
    }

    #[test]
    fn toolbar_geometry_matches_the_mockup_contract() {
        assert_eq!(toolbar::CODE_TOOLBAR_HEIGHT, 33.0);
        assert_eq!(toolbar::CODE_TOOLBAR_PADDING_X, 8.0);
        assert_eq!(toolbar::CODE_TOOLBAR_GAP, 5.0);
        assert_eq!(toolbar::CODE_TOOLBAR_ACTION_GUTTER, 12.0);
        assert_eq!(toolbar::CODE_TOOLBAR_COMPACT_BREAKPOINT, 720.0);
        assert_eq!(toolbar::CODE_TOOLBAR_TABLET_VIEWPORT_BREAKPOINT, 1024.0);
        assert_eq!(toolbar::CODE_TOOLBAR_FULL_STATUS_MIN_WIDTH, 320.0);
        assert_eq!(toolbar::PHONE_BREAKPOINT, 560.0);
        assert_eq!(toolbar::PHONE_PRIMARY_WIDTH, 154.0);
        assert_eq!(toolbar::CODE_TOOLBAR_ICON_WIDTH, 28.0);
        assert_eq!(toolbar::code_toolbar_visible_width(1024.0, 745.0), 745.0);
        assert_eq!(toolbar::code_toolbar_visible_width(700.0, 745.0), 700.0);
        assert!(toolbar::code_toolbar_compact(607.0));
        assert!(!toolbar::code_toolbar_compact(721.0));
        assert!(
            toolbar::code_toolbar_prefers_compact(1024.0, 744.0, 405.0),
            "tablet viewport must retain the compact projection"
        );
        assert!(
            toolbar::code_toolbar_prefers_compact(1280.0, 744.0, 405.0),
            "dock collapse must not re-enable a crowded full action set"
        );
        assert!(!toolbar::code_toolbar_prefers_compact(1280.0, 800.0, 405.0));
        assert!(!toolbar::toolbar_status_visible(
            true,
            DocumentStatusTone::Warning
        ));
        assert!(toolbar::toolbar_status_visible(
            true,
            DocumentStatusTone::Error
        ));
        assert!(toolbar::toolbar_status_visible(
            false,
            DocumentStatusTone::Warning
        ));
        assert!(toolbar::toolbar_advisory_fits(500.0, 260.0, 110.0, 70.0));
        assert!(
            !toolbar::toolbar_advisory_fits(430.0, 260.0, 110.0, 70.0),
            "advisory must yield before language, blocking status, or actions clip"
        );
        let content = egui::Rect::from_min_size(egui::pos2(8.0, 0.0), vec2(526.0, 33.0));
        let (status_and_language, actions) = toolbar::code_toolbar_regions(content, 342.0);
        assert_eq!(actions.right(), content.right());
        assert_eq!(
            actions.left() - status_and_language.right(),
            toolbar::CODE_TOOLBAR_ACTION_GUTTER
        );
        assert!(status_and_language.right() < actions.left());
    }

    #[test]
    fn empty_generated_primary_reports_blocked_without_claiming_staleness() {
        let mut state = AppState::default();
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generation_error =
            Some("Add a circuit before generating the primary netlist.".to_owned());

        assert!(generated_primary_unavailable(&state));
        assert!(!generated_primary_ready(&state));
        assert_eq!(
            document_syntax_status(&state),
            ("generation blocked".to_owned(), DocumentStatusTone::Warning)
        );
    }

    #[test]
    fn retained_generated_primary_reports_stale_when_regeneration_is_blocked() {
        let mut state = AppState::default();
        retain_generated(&mut state, "retained\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generation_error = Some("Regeneration failed.".to_owned());

        assert!(!generated_primary_unavailable(&state));
        assert!(generated_primary_ready(&state));
        assert_eq!(
            document_syntax_status(&state),
            (
                "stale · generation blocked".to_owned(),
                DocumentStatusTone::Warning
            )
        );
    }

    #[test]
    fn split_generated_state_fails_closed_as_unavailable() {
        let mut state = AppState::default();
        retain_generated(&mut state, "retained\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generated_source.clear();
        state.ui.netlist.generation_error = None;

        assert!(state.ui.netlist.generated_document.is_some());
        assert!(generated_primary_unavailable(&state));
        assert!(!generated_primary_ready(&state));
        assert!(!active_document_available(&state));
        assert_eq!(
            document_syntax_status(&state),
            ("generation blocked".to_owned(), DocumentStatusTone::Warning)
        );
    }

    /// Seal one manual-deck run into history the way a dispatch does, so the
    /// strip has a real receipt to read its digest and revision back off.
    fn seal_manual_run(state: &mut AppState, run_number: u64, deck_digest: u8, revision: u64) {
        let project_revision =
            crate::product::ObjectRevision::new(revision).expect("non-zero revision");
        let task = crate::state::PreparedRunTaskReceipt::new(
            crate::product::AnalysisInstanceId::new(),
            project_revision,
            Vec::new(),
            0,
            crate::product::ContentDigest::from_bytes([0x11; 32]),
        )
        .expect("valid task receipt");
        let receipt = crate::state::PreparedRunReceipt::new(
            crate::state::AnalysisResultSourceDomain::ManualDeck,
            None,
            project_revision,
            crate::product::ContentDigest::from_bytes([0x22; 32]),
            crate::product::ContentDigest::from_bytes([deck_digest; 32]),
            crate::state::PreparedSourceCheckReceipt::ManualSourceCheck(
                crate::product::ContentDigest::from_bytes([0x33; 32]),
            ),
            vec![task],
        )
        .expect("valid manual deck receipt");
        let mut run = crate::state::SimulationRun::new(run_number);
        run.restore_provenance(crate::state::SimulationRunProvenance::Prepared(Box::new(
            receipt,
        )))
        .expect("fresh run accepts its receipt");
        state.simulation.runs.push(run);
    }

    fn ran_owned_deck(deck: &str) -> AppState {
        let mut state = AppState::default();
        state.workspace.netlist_source = Some(deck.to_owned());
        state.simulation.netlist_content = deck.to_owned();
        state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        state.ui.netlist.active_document_initialized = true;
        state.ui.netlist.last_run_buffer = Some(deck.to_owned());
        state.ui.netlist.last_run_id = Some(7);
        seal_manual_run(&mut state, 7, 0xAB, 4);
        state
    }

    #[test]
    fn run_strip_states_the_run_identity_from_the_run_receipt_alone() {
        let state = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");

        let projection = run_strip_projection(&state).expect("a completed run owns the strip");
        assert_eq!(projection.phase, RunStripPhase::Current);
        assert_eq!(projection.run_id, 7);
        assert_eq!(projection.revision, 4);
        assert_eq!(
            projection.deck_digest,
            crate::product::ContentDigest::from_bytes([0xAB; 32])
                .to_string()
                .chars()
                .take(12)
                .collect::<String>()
        );
    }

    #[test]
    fn run_strip_warns_once_the_working_deck_moves_past_the_run() {
        let mut state = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        state.workspace.netlist_source = Some("deck\nR1 out 0 2k\n.op\n.end\n".to_owned());
        state.simulation.netlist_content = "deck\nR1 out 0 2k\n.op\n.end\n".to_owned();

        assert_eq!(
            run_strip_projection(&state).map(|projection| projection.phase),
            Some(RunStripPhase::Edited)
        );
    }

    #[test]
    fn run_strip_states_the_run_in_flight_and_never_the_stale_baseline() {
        let mut state = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        state.workspace.netlist_source = Some("deck\nR1 out 0 2k\n.op\n.end\n".to_owned());
        seal_manual_run(&mut state, 8, 0xCD, 5);
        state.ui.netlist.pending_manual_run_id = Some(8);

        let projection = run_strip_projection(&state).expect("an active run owns the strip");
        assert_eq!(projection.phase, RunStripPhase::Running);
        assert_eq!(projection.run_id, 8);
        assert_eq!(projection.revision, 5);
    }

    #[test]
    fn run_strip_is_silent_without_a_retained_manual_run() {
        let mut state = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        state.simulation.runs.clear();

        assert!(run_strip_projection(&state).is_none());
    }

    #[test]
    fn run_strip_does_not_speak_for_an_include_or_a_comparison() {
        let mut state = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
        assert!(run_strip_projection(&state).is_none());

        state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        state.ui.netlist.active_dependency_identity = Some("models/r.inc".to_owned());
        assert!(run_strip_projection(&state).is_none());
    }

    #[test]
    fn run_snapshot_document_owns_the_strip_as_its_immutable_header() {
        let mut state = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        assert!(crate::workbench::documents::netlist_document::open_run_deck_snapshot(&mut state));

        let projection = run_strip_projection(&state).expect("the snapshot states its run");
        assert_eq!(projection.phase, RunStripPhase::Snapshot);
        assert_eq!(projection.run_id, 7);
        assert!(
            !crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
                &state
            )
        );
    }

    /// Render the deck stage and read back the strip's own 24-point band.
    ///
    /// The band is where the strip is or is not: a phase that painted nothing
    /// there, or two phases that painted the same thing, would be a strip that
    /// states its status in prose only.
    fn run_strip_band(width: f32, phase: RunStripPhase) -> (crate::ui::raster::Canvas, Vec<u8>) {
        const DECK: &str = "run strip fixture\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
        const EDITED: &str = "run strip fixture\nV1 out 0 1\nR1 out 0 2k\n.op\n.end\n";

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        app.state.workspace.netlist_source = Some(DECK.to_owned());
        app.state.simulation.netlist_content = DECK.to_owned();
        app.state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        app.state.ui.netlist.active_document_initialized = true;
        app.state.ui.netlist.last_run_buffer = Some(DECK.to_owned());
        app.state.ui.netlist.last_run_id = Some(7);
        seal_manual_run(&mut app.state, 7, 0xAB, 4);
        match phase {
            RunStripPhase::Current => {}
            RunStripPhase::Snapshot => {
                assert!(
                    crate::workbench::documents::netlist_document::open_run_deck_snapshot(
                        &mut app.state
                    )
                );
            }
            RunStripPhase::Edited => {
                app.state.workspace.netlist_source = Some(EDITED.to_owned());
                app.state.simulation.netlist_content = EDITED.to_owned();
            }
            RunStripPhase::Running => {
                seal_manual_run(&mut app.state, 8, 0xCD, 5);
                app.state.ui.netlist.pending_manual_run_id = Some(8);
            }
        }
        assert_eq!(
            run_strip_projection(&app.state).map(|projection| projection.phase),
            Some(phase),
            "the raster fixture must actually be in the phase it renders"
        );

        let canvas = crate::ui::raster::render(vec2(width, 520.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| show_prepared(ui, &mut app));
        });
        let band = egui::Rect::from_min_max(
            egui::pos2(0.0, toolbar::CODE_TOOLBAR_HEIGHT + 1.0),
            egui::pos2(width, toolbar::CODE_TOOLBAR_HEIGHT + RUN_STRIP_HEIGHT - 1.0),
        );
        let pixels = canvas
            .pixels_in(band)
            .flat_map(|pixel| pixel.to_array())
            .collect::<Vec<_>>();
        assert!(!pixels.is_empty(), "the strip band is off canvas");
        assert!(
            pixels
                .chunks_exact(4)
                .any(|pixel| pixel != canvas.background().to_array()),
            "the strip band painted nothing"
        );
        (canvas, pixels)
    }

    /// Render the deck stage showing one point's executed deck.
    ///
    /// The strip above it has to say which point, because a run's deck digest
    /// describes the source it was authorized over and a corner point solves
    /// something else — the same digest over four different decks would be a
    /// header that is true of the run and false of what is under it.
    fn executed_deck_stage(width: f32) -> crate::ui::raster::Canvas {
        const EXECUTED: &str = "run strip fixture\n* RSpice sealed model source: pack rspice-opamps 2.1.0\n.model OPA2340 D\n.OPTIONS TEMP=125\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        app.state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        app.state.ui.netlist.active_document_initialized = true;
        seal_manual_run(&mut app.state, 7, 0xAB, 4);
        let deck: std::sync::Arc<str> = std::sync::Arc::from(EXECUTED);
        app.state
            .simulation
            .executed_decks
            .retain(crate::state::ExecutedDeck {
                run_id: 7,
                points: vec![crate::state::ExecutedDeckPoint {
                    label: "SS 1.62V 125C".to_owned(),
                    model_sources: crate::state::sealed_model_sources(&deck),
                    deck,
                }],
            });
        assert!(
            crate::workbench::documents::netlist_document::reveal_executed_deck(
                &mut app.state,
                7,
                0
            )
        );
        assert_eq!(
            run_strip_projection(&app.state).and_then(|projection| projection.point),
            Some("SS 1.62V 125C".to_owned()),
            "the fixture must be showing the point it renders"
        );
        crate::ui::raster::render(vec2(width, 520.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| show_prepared(ui, &mut app));
        })
    }

    #[test]
    fn run_strip_paints_a_distinct_row_for_every_phase_at_both_stage_widths() {
        for width in [1600.0, 1000.0] {
            let (_, current) = run_strip_band(width, RunStripPhase::Current);
            let (_, edited) = run_strip_band(width, RunStripPhase::Edited);
            let (_, running) = run_strip_band(width, RunStripPhase::Running);
            assert_ne!(current, edited, "ok and warn must not render alike");
            assert_ne!(current, running, "ok and running must not render alike");
            assert_ne!(edited, running, "warn and running must not render alike");
        }
    }

    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_run_strip_phases() {
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for width in [2560.0_f32, 1600.0, 1000.0] {
            for phase in [
                RunStripPhase::Current,
                RunStripPhase::Edited,
                RunStripPhase::Running,
                RunStripPhase::Snapshot,
            ] {
                // The widest stage only has to show the steady state.
                if width > 1600.0 && phase != RunStripPhase::Current {
                    continue;
                }
                let (canvas, _) = run_strip_band(width, phase);
                let height = canvas.content_height().clamp(1, 200);
                let path = directory.join(format!(
                    "netlist-run-strip-{phase:?}-{}.png",
                    width.round() as u32
                ));
                std::fs::write(&path, canvas.png(height)).expect("write strip render");
            }
        }
        for width in [1600.0_f32, 1000.0] {
            let canvas = executed_deck_stage(width);
            let height = canvas.content_height().clamp(1, 420);
            let path = directory.join(format!(
                "netlist-executed-deck-{}.png",
                width.round() as u32
            ));
            std::fs::write(&path, canvas.png(height)).expect("write executed deck render");
        }
    }

    #[test]
    fn generated_deck_does_not_inject_unreferenced_code_workspace_veriloga() {
        let state = AppState::default();
        let source = "R1 1 0 1k\n.end\n".to_owned();

        let dependencies = generation::generated_project_source_dependencies(&state, &source)
            .expect("unreferenced source does not create a dependency");
        assert!(dependencies.is_empty());
    }

    #[test]
    fn generated_deck_retains_the_exact_transitive_cell_view_source_closure() {
        let (state, source_key) = configured_veriloga_state();
        let source = format!("configured deck\n.veriloga \"{source_key}\" sealed_gain\n.end\n");

        let dependencies = generation::generated_project_source_dependencies(&state, &source)
            .expect("retain exact project source closure");

        assert_eq!(dependencies.len(), 2);
        assert!(dependencies.iter().all(|dependency| {
            !dependency
                .resolution()
                .source()
                .is_some_and(|source| source.contains("must_not_enter_provenance"))
        }));
        let root = dependencies
            .iter()
            .find(|dependency| dependency.direct_include_index().is_some())
            .expect("direct root dependency");
        assert_eq!(root.requested_locator(), source_key);
        assert!(root.parent().is_none());
        assert!(
            root.resolution()
                .source()
                .unwrap()
                .contains("module sealed_gain")
        );
        let included = dependencies
            .iter()
            .find(|dependency| dependency.parent().is_some())
            .expect("transitive included dependency");
        assert_eq!(included.requested_locator(), "behavioral/gain_constants.va");
        assert_eq!(
            included.resolution().source(),
            Some("`define RSPICE_GAIN 1.0\n")
        );
    }
}
