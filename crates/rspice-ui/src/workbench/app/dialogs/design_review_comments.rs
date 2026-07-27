//! Durable, anchored design-review workflow for schematic documentation.
//!
//! Every row is projected from `DesignNoteKind::ReviewNote`; there are no
//! sample threads. Mutations use exact document snapshots, one schematic undo
//! entry, and active-view authority guards.

use egui::{Align, Context, Frame, Layout, RichText, ScrollArea, Stroke, TextEdit, Ui, Vec2};

use crate::state::{DesignNote, DesignNoteKind, DesignReviewMutation, DesignReviewState, Point};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
};
use crate::workbench::design_system::section_header;

use crate::workbench::app::{AppState, ConsoleMessage, RSpiceApp, SchematicEditAuthority};

const EYEBROW: &str = "COLLABORATION \u{00b7} ANCHORED REVIEW \u{00b7} RESOLUTION";
const TITLE: &str = "Design review comments";
const PRIMARY: &str = "Publish review update";
const CURRENT_ACTOR: &str = "Local project editor";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DesignReviewStatusFilter {
    #[default]
    Open,
    All,
    MyAssignments,
}

impl DesignReviewStatusFilter {
    const ALL: [Self; 3] = [Self::Open, Self::All, Self::MyAssignments];

    fn label(self, open_count: usize, all_count: usize) -> String {
        match self {
            Self::Open => format!("Open comments \u{00b7} {open_count}"),
            Self::All => format!("All comments \u{00b7} {all_count}"),
            Self::MyAssignments => "My assignments".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DesignReviewRevisionScope {
    #[default]
    CurrentAndCarried,
    CurrentOnly,
}

impl DesignReviewRevisionScope {
    const ALL: [Self; 2] = [Self::CurrentAndCarried, Self::CurrentOnly];

    const fn label(self) -> &'static str {
        match self {
            Self::CurrentAndCarried => "Current revision + carried forward",
            Self::CurrentOnly => "Current revision only",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DesignReviewCommentsDialogState {
    pub(crate) open: bool,
    pub(crate) status_filter: DesignReviewStatusFilter,
    pub(crate) revision_scope: DesignReviewRevisionScope,
    pub(crate) selected_note_id: Option<u64>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) expected_design_notes: Vec<DesignNote>,
    pub(crate) reply: String,
    pub(crate) assignment_editor_open: bool,
    pub(crate) assignment: String,
    pub(crate) evidence_picker_open: bool,
    pub(crate) selected_evidence: Option<usize>,
    pub(crate) error: Option<String>,
    pub(crate) receipt: Option<String>,
    pub(crate) body_scroll_offset: f32,
}

impl Default for DesignReviewCommentsDialogState {
    fn default() -> Self {
        Self {
            open: false,
            status_filter: DesignReviewStatusFilter::Open,
            revision_scope: DesignReviewRevisionScope::CurrentAndCarried,
            selected_note_id: None,
            authority: None,
            expected_design_notes: Vec::new(),
            reply: String::new(),
            assignment_editor_open: false,
            assignment: String::new(),
            evidence_picker_open: false,
            selected_evidence: None,
            error: None,
            receipt: None,
            body_scroll_offset: 0.0,
        }
    }
}

impl DesignReviewCommentsDialogState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    fn reset_thread_draft(&mut self) {
        self.reply.clear();
        self.assignment_editor_open = false;
        self.assignment.clear();
        self.evidence_picker_open = false;
        self.selected_evidence = None;
        self.error = None;
        self.receipt = None;
    }
}

#[derive(Debug, Clone)]
struct EvidenceCandidate {
    label: String,
    source_identity: String,
    digest: String,
}

#[derive(Debug, Clone)]
enum ReviewAction {
    None,
    Select(u64),
    NewComment,
    GoToAnchor(u64, Point),
    ApplyAssignment(Option<String>),
    AttachEvidence(usize),
    Resolve,
    Reopen,
}

impl Default for ReviewAction {
    fn default() -> Self {
        Self::None
    }
}

pub(crate) fn open_design_review_comments(state: &mut AppState) {
    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::warning(
            "Design review comments require an open project.",
        ));
        return;
    }
    if !matches!(
        state.workspace.active_view_type(),
        crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
    ) {
        state.push_user_message(ConsoleMessage::warning(
            "Design review comments require a schematic or testbench view.",
        ));
        return;
    }
    let selected_note_id = state
        .schematic
        .design_notes
        .iter()
        .find(|note| note.kind == DesignNoteKind::ReviewNote)
        .map(|note| note.id);
    state.dialogs.design_review_comments = DesignReviewCommentsDialogState {
        open: true,
        selected_note_id,
        authority: Some(SchematicEditAuthority::capture(state)),
        expected_design_notes: state.schematic.design_notes.clone(),
        ..DesignReviewCommentsDialogState::default()
    };
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_design_review_comments_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.design_review_comments.open {
            return;
        }

        let current_revision = current_revision_identity(&self.state);
        normalize_selected_thread(
            &self.state.schematic.design_notes,
            &mut self.state.dialogs.design_review_comments,
            current_revision.as_deref(),
        );
        let stale = review_authority_error(&self.state);
        let write_allowed = !self.state.schematic.read_only
            && !self.state.active_view_read_only()
            && !self.state.workbench.safe_mode.project_read_only();
        let selected = selected_review_note(&self.state).cloned();
        let evidence = evidence_candidates(&self.state);
        let primary_enabled = write_allowed
            && stale.is_none()
            && selected.is_some()
            && !self
                .state
                .dialogs
                .design_review_comments
                .reply
                .trim()
                .is_empty();
        let mut body_scroll_offset = self.state.dialogs.design_review_comments.body_scroll_offset;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(
                "Review durable comments anchored to the active schematic, its validated revisions, and retained result evidence.",
            )
            .size(DialogSize::SimulationWorkflow)
            .initial_height(610.0)
            .flush_body()
            .ghost("Close")
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        let transaction_error = self
            .state
            .dialogs
            .design_review_comments
            .error
            .clone()
            .or_else(|| stale.clone());
        if !write_allowed {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Inspection only",
                "The active schematic or project is read-only. Review history and anchor navigation remain available.",
            );
        } else if let Some(error) = transaction_error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Review update cannot continue",
                error,
            );
        }

        let notes = self.state.schematic.design_notes.clone();
        let mut action = ReviewAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = review_body(
                ui,
                &notes,
                selected.as_ref(),
                &evidence,
                current_revision.as_deref(),
                write_allowed && stale.is_none(),
                &mut self.state.dialogs.design_review_comments,
            );
            None
        });
        self.state.dialogs.design_review_comments.body_scroll_offset = body_scroll_offset;
        self.handle_design_review_action(action);

        match choice {
            DialogChoice::Primary => {
                let body = self.state.dialogs.design_review_comments.reply.clone();
                self.apply_review_mutation(DesignReviewMutation::Reply {
                    author: CURRENT_ACTOR.to_owned(),
                    body,
                    created_unix_ms: unix_time_ms(),
                });
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.design_review_comments.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_design_review_action(&mut self, action: ReviewAction) {
        match action {
            ReviewAction::None => {}
            ReviewAction::Select(id) => {
                let dialog = &mut self.state.dialogs.design_review_comments;
                dialog.selected_note_id = Some(id);
                dialog.reset_thread_draft();
            }
            ReviewAction::NewComment => {
                self.state.dialogs.design_review_comments.close();
                let view_path = self.state.workspace.active_view.display_path();
                self.state.dialogs.design_note.open(
                    self.state.design_execution_epoch,
                    self.state.active_schematic_epoch,
                    self.state.schematic.topology_version(),
                    view_path,
                );
                self.state.dialogs.design_note.kind = DesignNoteKind::ReviewNote;
                self.state.dialogs.design_note.text = "Review comment".to_owned();
                self.state.dialogs.design_note.dirty = false;
            }
            ReviewAction::GoToAnchor(id, point) => {
                self.state.dialogs.design_review_comments.close();
                self.state.workbench.workspace = crate::workbench::state::Workspace::Design;
                self.state.schematic.selection.select_only_design_note(id);
                self.state.schematic.center_request = Some(point);
            }
            ReviewAction::ApplyAssignment(assignee) => {
                self.apply_review_mutation(DesignReviewMutation::Assign { assignee });
            }
            ReviewAction::AttachEvidence(index) => {
                let candidates = evidence_candidates(&self.state);
                let Some(candidate) = candidates.get(index) else {
                    self.state.dialogs.design_review_comments.error =
                        Some("The selected result evidence no longer exists.".to_owned());
                    return;
                };
                self.apply_review_mutation(DesignReviewMutation::AttachEvidence {
                    label: candidate.label.clone(),
                    source_identity: candidate.source_identity.clone(),
                    content_digest: Some(candidate.digest.clone()),
                });
            }
            ReviewAction::Resolve => {
                let note = self.state.dialogs.design_review_comments.reply.clone();
                self.apply_review_mutation(DesignReviewMutation::Resolve {
                    author: CURRENT_ACTOR.to_owned(),
                    note,
                    created_unix_ms: unix_time_ms(),
                });
            }
            ReviewAction::Reopen => self.apply_review_mutation(DesignReviewMutation::Reopen),
        }
    }

    fn apply_review_mutation(&mut self, mutation: DesignReviewMutation) {
        let result = (|| {
            review_authority_error(&self.state).map_or(Ok(()), Err)?;
            if self.state.workbench.safe_mode.project_read_only() {
                return Err("The project is open read-only.".to_owned());
            }
            let id = self
                .state
                .dialogs
                .design_review_comments
                .selected_note_id
                .ok_or_else(|| "Select a review thread.".to_owned())?;
            let expected = self
                .state
                .dialogs
                .design_review_comments
                .expected_design_notes
                .clone();
            self.state
                .schematic
                .apply_design_review_mutation(id, &expected, mutation)
                .map_err(|error| error.to_string())?;
            self.state.sync_active_schematic_to_workspace();
            Ok::<(), String>(())
        })();

        match result {
            Ok(()) => {
                let authority = SchematicEditAuthority::capture(&self.state);
                let dialog = &mut self.state.dialogs.design_review_comments;
                dialog.authority = Some(authority);
                dialog.expected_design_notes = self.state.schematic.design_notes.clone();
                dialog.reply.clear();
                dialog.assignment_editor_open = false;
                dialog.assignment.clear();
                dialog.evidence_picker_open = false;
                dialog.selected_evidence = None;
                dialog.error = None;
                dialog.receipt = Some(
                    "Review update published to the active schematic. Undo restores the prior review record."
                        .to_owned(),
                );
            }
            Err(error) => self.state.dialogs.design_review_comments.error = Some(error),
        }
    }
}

fn review_authority_error(state: &AppState) -> Option<String> {
    state
        .dialogs
        .design_review_comments
        .authority
        .as_ref()
        .ok_or_else(|| "Review authority is unavailable. Close and reopen the workflow.".to_owned())
        .and_then(|authority| authority.validate(state, "Design review comments"))
        .err()
}

fn current_revision_identity(state: &AppState) -> Option<String> {
    state
        .schematic
        .validated_revisions
        .records()
        .last()
        .map(|record| record.revision_digest().to_string())
}

fn normalize_selected_thread(
    notes: &[DesignNote],
    dialog: &mut DesignReviewCommentsDialogState,
    current_revision: Option<&str>,
) {
    let current = dialog
        .selected_note_id
        .and_then(|id| notes.iter().find(|note| note.id == id))
        .filter(|note| note_visible(note, dialog, current_revision));
    if current.is_none() {
        dialog.selected_note_id = notes
            .iter()
            .find(|note| note_visible(note, dialog, current_revision))
            .map(|note| note.id);
    }
}

fn selected_review_note(state: &AppState) -> Option<&DesignNote> {
    let id = state.dialogs.design_review_comments.selected_note_id?;
    state
        .schematic
        .design_notes
        .iter()
        .find(|note| note.id == id && note.kind == DesignNoteKind::ReviewNote)
}

fn evidence_candidates(state: &AppState) -> Vec<EvidenceCandidate> {
    state
        .simulation
        .runs
        .iter()
        .filter(|run| run.lifecycle.is_terminal())
        .map(|run| EvidenceCandidate {
            label: format!(
                "{} \u{00b7} dataset {}",
                run.label,
                short_identity(&run.dataset_id.to_string())
            ),
            source_identity: format!("result://dataset/{}", run.dataset_id),
            digest: format!("sha256:{}", run.dataset_content_digest()),
        })
        .collect()
}

fn note_visible(
    note: &DesignNote,
    dialog: &DesignReviewCommentsDialogState,
    current_revision: Option<&str>,
) -> bool {
    let Some(review) = note.review.as_ref() else {
        return false;
    };
    let status_matches = match dialog.status_filter {
        DesignReviewStatusFilter::Open => review.state == DesignReviewState::Open,
        DesignReviewStatusFilter::All => true,
        DesignReviewStatusFilter::MyAssignments => {
            review.assignee.as_deref() == Some(CURRENT_ACTOR)
        }
    };
    let revision_matches = match dialog.revision_scope {
        DesignReviewRevisionScope::CurrentAndCarried => true,
        DesignReviewRevisionScope::CurrentOnly => {
            current_revision.is_some() && review.anchored_revision.as_deref() == current_revision
        }
    };
    status_matches && revision_matches
}

fn review_body(
    ui: &mut Ui,
    notes: &[DesignNote],
    selected: Option<&DesignNote>,
    evidence: &[EvidenceCandidate],
    current_revision: Option<&str>,
    write_allowed: bool,
    dialog: &mut DesignReviewCommentsDialogState,
) -> ReviewAction {
    let t = Tokens::get(ui.ctx());
    let mut action = ReviewAction::None;
    Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let open_count = notes
                    .iter()
                    .filter(|note| {
                        note.review
                            .as_ref()
                            .is_some_and(|review| review.state == DesignReviewState::Open)
                    })
                    .count();
                let all_count = notes
                    .iter()
                    .filter(|note| note.kind == DesignNoteKind::ReviewNote)
                    .count();
                let status_options = DesignReviewStatusFilter::ALL
                    .iter()
                    .map(|value| value.label(open_count, all_count))
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "design-review-status",
                    "Comment status filter",
                    &dialog.status_filter.label(open_count, all_count),
                    &status_options,
                    190.0,
                ) {
                    dialog.status_filter = DesignReviewStatusFilter::ALL[index];
                }
                let scope_options = DesignReviewRevisionScope::ALL
                    .iter()
                    .map(|value| value.label().to_owned())
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "design-review-revision-scope",
                    "Comment revision scope",
                    dialog.revision_scope.label(),
                    &scope_options,
                    220.0,
                ) {
                    dialog.revision_scope = DesignReviewRevisionScope::ALL[index];
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if Button::new("New comment")
                        .accent()
                        .enabled(write_allowed)
                        .show(ui)
                        .clicked()
                    {
                        action = ReviewAction::NewComment;
                    }
                });
            });
        });

    let available = ui.available_size();
    let left_width = (available.x * 0.38).clamp(250.0, 360.0);
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            Vec2::new(left_width, available.y.max(360.0)),
            Layout::top_down(Align::Min),
            |ui| {
                Frame::new()
                    .fill(t.color.bg_inset)
                    .stroke(Stroke::new(1.0, t.color.border))
                    .show(ui, |ui| {
                        ScrollArea::vertical()
                            .id_salt("design-review-thread-list")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let mut visible = 0;
                                for note in notes.iter().filter(|note| {
                                    note_visible(note, dialog, current_revision)
                                }) {
                                    visible += 1;
                                    if review_thread_row(
                                        ui,
                                        note,
                                        dialog.selected_note_id == Some(note.id),
                                    )
                                    .clicked()
                                    {
                                        action = ReviewAction::Select(note.id);
                                    }
                                }
                                if visible == 0 {
                                    ui.add_space(18.0);
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            RichText::new("No review threads match this filter")
                                                .font(theme::sans(
                                                    tokens::FS_0,
                                                    FontWeight::SemiBold,
                                                ))
                                                .color(t.color.text),
                                        );
                                        ui.label(
                                            RichText::new(
                                                "Change status or revision scope, or create an anchored review comment.",
                                            )
                                            .font(theme::sans(
                                                tokens::FS_0,
                                                FontWeight::Regular,
                                            ))
                                            .color(t.color.text_dim),
                                        );
                                    });
                                }
                            });
                    });
            },
        );
        ui.allocate_ui_with_layout(
            Vec2::new((available.x - left_width - 8.0).max(320.0), available.y.max(360.0)),
            Layout::top_down(Align::Min),
            |ui| {
                Frame::new()
                    .fill(t.color.bg_panel)
                    .stroke(Stroke::new(1.0, t.color.border))
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        if let Some(note) = selected {
                            action = review_thread_detail(
                                ui,
                                note,
                                evidence,
                                write_allowed,
                                dialog,
                            );
                        } else {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("Select a review thread")
                                        .font(theme::sans(
                                            tokens::FS_0,
                                            FontWeight::SemiBold,
                                        ))
                                        .color(t.color.text_dim),
                                );
                            });
                        }
                    });
            },
        );
    });
    action
}

fn review_thread_row(ui: &mut Ui, note: &DesignNote, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let review = note.review.as_ref().expect("filtered review note");
    let status = if review.state == DesignReviewState::Resolved {
        "resolved"
    } else if review.assignee.is_some() {
        "assigned"
    } else {
        "open"
    };
    let author = review
        .messages
        .first()
        .map(|message| message.author.as_str())
        .or(review.assignee.as_deref())
        .unwrap_or("Review");
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 58.0), egui::Sense::click());
    let fill = if selected {
        t.color.accent_dim
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_panel
    };
    ui.painter().rect(
        rect,
        0.0,
        fill,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let avatar = initials(author);
    ui.painter().circle_filled(
        egui::pos2(rect.left() + 22.0, rect.center().y),
        14.0,
        t.color.bg_elevated,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 22.0, rect.center().y),
        egui::Align2::CENTER_CENTER,
        avatar,
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let title_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 44.0, rect.top() + 8.0),
        egui::pos2(rect.right() - 64.0, rect.top() + 28.0),
    );
    ui.painter().text(
        title_rect.left_top(),
        egui::Align2::LEFT_TOP,
        ellipsize(&note.text, 46),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 44.0, rect.top() + 34.0),
        egui::Align2::LEFT_TOP,
        format!("{} \u{00b7} annotation", format_anchor(note.pos)),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        status,
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        if review.state == DesignReviewState::Resolved {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    response
}

fn review_thread_detail(
    ui: &mut Ui,
    note: &DesignNote,
    evidence: &[EvidenceCandidate],
    write_allowed: bool,
    dialog: &mut DesignReviewCommentsDialogState,
) -> ReviewAction {
    let t = Tokens::get(ui.ctx());
    let review = note.review.as_ref().expect("selected review note");
    let mut action = ReviewAction::None;
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(&note.text)
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if Button::new("Go to anchor").show(ui).clicked() {
                action = ReviewAction::GoToAnchor(note.id, note.pos);
            }
        });
    });
    ui.separator();
    ScrollArea::vertical()
        .id_salt("design-review-thread-detail")
        .max_height(240.0)
        .show(ui, |ui| {
            if review.messages.is_empty() {
                review_message(
                    ui,
                    "Design note",
                    &note.text,
                    review
                        .anchored_revision
                        .as_deref()
                        .map_or_else(
                            || "unversioned anchor".to_owned(),
                            |revision| format!("anchored to revision {}", short_identity(revision)),
                        )
                        .as_str(),
                    false,
                );
            } else {
                for (index, message) in review.messages.iter().enumerate() {
                    review_message(
                        ui,
                        &message.author,
                        &message.body,
                        &relative_time(message.created_unix_ms),
                        index > 0,
                    );
                }
            }
            if !review.evidence.is_empty() {
                section_header(
                    ui,
                    "Attached evidence",
                    Some(&review.evidence.len().to_string()),
                );
                for item in &review.evidence {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("\u{25cf}")
                                .color(t.color.accent)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            RichText::new(&item.label)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text),
                        )
                        .on_hover_text(&item.source_identity);
                    });
                }
            }
        });

    ui.label(
        RichText::new("Reply")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_enabled(
        write_allowed,
        TextEdit::multiline(&mut dialog.reply)
            .desired_rows(3)
            .desired_width(f32::INFINITY)
            .hint_text("Add review evidence, mention a teammate, or record a decision\u{2026}"),
    );

    if dialog.assignment_editor_open {
        Frame::new()
            .fill(t.color.bg_inset)
            .stroke(Stroke::new(1.0, t.color.border))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("Assign review")
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add_enabled(
                    write_allowed,
                    TextEdit::singleline(&mut dialog.assignment)
                        .desired_width(f32::INFINITY)
                        .hint_text("Exact project member identity"),
                );
                ui.horizontal(|ui| {
                    let valid = !dialog.assignment.trim().is_empty();
                    if Button::new("Apply assignment")
                        .accent()
                        .enabled(write_allowed && valid)
                        .show(ui)
                        .clicked()
                    {
                        action = ReviewAction::ApplyAssignment(Some(dialog.assignment.clone()));
                    }
                    if Button::new("Clear assignment")
                        .enabled(write_allowed && review.assignee.is_some())
                        .show(ui)
                        .clicked()
                    {
                        action = ReviewAction::ApplyAssignment(None);
                    }
                });
            });
    }

    if dialog.evidence_picker_open {
        Frame::new()
            .fill(t.color.bg_inset)
            .stroke(Stroke::new(1.0, t.color.border))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                section_header(
                    ui,
                    "Retained result evidence",
                    Some(&evidence.len().to_string()),
                );
                if evidence.is_empty() {
                    ui.label(
                        RichText::new(
                            "No completed retained result dataset is available to attach.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    );
                } else {
                    let options = evidence
                        .iter()
                        .map(|candidate| candidate.label.clone())
                        .collect::<Vec<_>>();
                    let selected = dialog
                        .selected_evidence
                        .filter(|index| *index < options.len())
                        .unwrap_or(0);
                    if let Some(index) = select(
                        ui,
                        "design-review-evidence",
                        "Retained result evidence",
                        &options[selected],
                        &options,
                        ui.available_width().min(430.0),
                    ) {
                        dialog.selected_evidence = Some(index);
                    }
                    if Button::new("Attach selected result")
                        .accent()
                        .enabled(write_allowed)
                        .show(ui)
                        .clicked()
                    {
                        action = ReviewAction::AttachEvidence(
                            dialog.selected_evidence.unwrap_or(selected),
                        );
                    }
                }
            });
    }

    ui.horizontal_wrapped(|ui| {
        if Button::new("Attach result\u{2026}")
            .enabled(write_allowed)
            .show(ui)
            .clicked()
        {
            dialog.evidence_picker_open = !dialog.evidence_picker_open;
            dialog.assignment_editor_open = false;
        }
        if Button::new("Assign\u{2026}")
            .enabled(write_allowed)
            .show(ui)
            .clicked()
        {
            dialog.assignment_editor_open = !dialog.assignment_editor_open;
            dialog.evidence_picker_open = false;
            dialog.assignment = review.assignee.clone().unwrap_or_default();
        }
        if review.state == DesignReviewState::Resolved {
            if Button::new("Reopen")
                .accent()
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = ReviewAction::Reopen;
            }
        } else if Button::new("Resolve with note")
            .accent()
            .enabled(write_allowed && !dialog.reply.trim().is_empty())
            .show(ui)
            .clicked()
        {
            action = ReviewAction::Resolve;
        }
    });

    if let Some(receipt) = dialog.receipt.as_deref() {
        ui.label(
            RichText::new(receipt)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.ok),
        );
    }
    action
}

fn review_message(ui: &mut Ui, author: &str, body: &str, meta: &str, reply: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(if reply {
            t.color.bg_inset
        } else {
            t.color.bg_panel
        })
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(30.0), egui::Sense::hover());
                ui.painter()
                    .circle_filled(rect.center(), 14.0, t.color.bg_elevated);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    initials(author),
                    theme::mono(tokens::FS_0, FontWeight::SemiBold),
                    t.color.text,
                );
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(author)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        RichText::new(body)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text),
                    );
                    ui.label(
                        RichText::new(meta)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
            });
        });
}

fn initials(author: &str) -> String {
    let initials = author
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    if initials.is_empty() {
        "RV".to_owned()
    } else {
        initials
    }
}

fn format_anchor(point: Point) -> String {
    format!("({}, {})", point.x, point.y)
}

fn short_identity(identity: &str) -> &str {
    identity.get(..7).unwrap_or(identity)
}

fn ellipsize(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_owned();
    }
    let mut result = value
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    result.push('\u{2026}');
    result
}

fn relative_time(timestamp_ms: u64) -> String {
    let elapsed = unix_time_ms().saturating_sub(timestamp_ms);
    match elapsed {
        0..=59_999 => "just now".to_owned(),
        60_000..=3_599_999 => format!("{} min ago", elapsed / 60_000),
        3_600_000..=86_399_999 => format!("{} h ago", elapsed / 3_600_000),
        _ => format!("{} d ago", elapsed / 86_400_000),
    }
}

fn unix_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().try_into().unwrap_or(u64::MAX))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::DesignNote;

    fn review_note(id: u64) -> DesignNote {
        DesignNote::new(
            id,
            Point::new(10, 20),
            DesignNoteKind::ReviewNote,
            "Confirm model",
        )
        .unwrap()
    }

    #[test]
    fn filters_use_durable_state_and_never_fixture_counts() {
        let mut open = review_note(1);
        open.assign_review(Some(CURRENT_ACTOR)).unwrap();
        let mut resolved = review_note(2);
        resolved
            .resolve_review(CURRENT_ACTOR, "Accepted.", unix_time_ms())
            .unwrap();
        let mut dialog = DesignReviewCommentsDialogState::default();
        assert!(note_visible(&open, &dialog, None));
        assert!(!note_visible(&resolved, &dialog, None));

        dialog.status_filter = DesignReviewStatusFilter::MyAssignments;
        assert!(note_visible(&open, &dialog, None));
        assert!(!note_visible(&resolved, &dialog, None));

        dialog.status_filter = DesignReviewStatusFilter::All;
        assert!(note_visible(&resolved, &dialog, None));
    }

    #[test]
    fn current_only_fails_closed_without_exact_revision_identity() {
        let mut note = review_note(1);
        note.anchor_review_to_revision(Some("abcdef012345"))
            .unwrap();
        let dialog = DesignReviewCommentsDialogState {
            revision_scope: DesignReviewRevisionScope::CurrentOnly,
            ..DesignReviewCommentsDialogState::default()
        };
        assert!(!note_visible(&note, &dialog, None));
        assert!(!note_visible(&note, &dialog, Some("different")));
        assert!(note_visible(&note, &dialog, Some("abcdef012345")));
    }
}
