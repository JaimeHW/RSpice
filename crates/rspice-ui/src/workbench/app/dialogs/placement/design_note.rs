//! Mockup-owned Place text or design note transaction.

use egui::{Align, Context, Frame, Layout, Sense, Stroke, TextEdit, Ui, Vec2};

use crate::state::{
    DesignNote, DesignNoteKind, DesignNoteLayer, DesignNoteRenderContext,
    PendingDesignNotePlacement, Point, Tool,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
};

use crate::workbench::app::{DesignNoteDialogState, RSpiceApp};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC · DOCUMENTATION";
const TITLE: &str = "Place text or design note";
const PRIMARY: &str = "Arm text tool";
const DESCRIPTION: &str =
    "Create plain documentation, property display, requirement link, or governed review note.";
const WORKFLOW_HEIGHT: f32 = 356.0;
const SPLIT_MIN_WIDTH: f32 = 660.0;
const SPLIT_PREVIEW_HEIGHT: f32 = 220.0;
const STACKED_PREVIEW_HEIGHT: f32 = 116.0;
const STATUS_CARDS_INLINE_MIN_WIDTH: f32 = 320.0;
const PANE_PADDING: i8 = 14;
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No schematic documentation has been changed.";

#[derive(Debug)]
enum DraftValidation {
    Invalid(String),
    Valid(PendingDesignNotePlacement),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid(_) => None,
        }
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_design_note_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.design_note.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let preview_text = design_note_preview_text(&self.state);
        let discard_confirm = self.state.dialogs.design_note.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            workflow_body(
                ui,
                validation_message.as_deref(),
                &preview_text,
                &mut self.state.dialogs.design_note,
            )
        });
        match choice {
            DialogChoice::Primary => {
                if let DraftValidation::Valid(pending) = validate_draft(&self.state) {
                    self.state.schematic.pending_design_note = Some(pending);
                    self.state.schematic.arm_tool(Tool::DesignNote);
                    self.state.dialogs.design_note.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.design_note.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.design_note;
    if state.schematic_edit_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    if draft.design_execution_epoch != state.design_execution_epoch {
        return DraftValidation::Invalid(
            "The design document changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if draft.active_schematic_epoch != state.active_schematic_epoch {
        return DraftValidation::Invalid(
            "The active schematic buffer changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if draft.topology_version != state.schematic.topology_version() {
        return DraftValidation::Invalid(
            "The schematic topology changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if draft.view_path != state.workspace.active_view.display_path() {
        return DraftValidation::Invalid(
            "The active cell/view changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    match PendingDesignNotePlacement::new(
        draft.kind,
        draft.text.clone(),
        draft.topology_version,
        &state.schematic.design_notes,
    ) {
        Ok(pending) => DraftValidation::Valid(pending.with_document_authority(
            draft.design_execution_epoch,
            draft.active_schematic_epoch,
            draft.view_path.clone(),
        )),
        Err(error) => DraftValidation::Invalid(error.to_string()),
    }
}

fn design_note_preview_text(state: &AppState) -> String {
    let draft = &state.dialogs.design_note;
    let source = if draft.text.trim().is_empty() {
        "Bias network"
    } else {
        draft.text.trim()
    };
    let Ok(note) = DesignNote::new(0, Point::origin(), draft.kind, source) else {
        return source.to_owned();
    };
    let view_path = state.workspace.active_view.display_path();
    note.rendered_text(&DesignNoteRenderContext::for_schematic(
        &view_path,
        &state.schematic,
    ))
}

fn workflow_body(
    ui: &mut Ui,
    validation_message: Option<&str>,
    preview_text: &str,
    draft: &mut DesignNoteDialogState,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let mut focus = None;
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            if uses_split_layout(ui.available_width()) {
                let divider = 1.0;
                let content_width = (ui.available_width() - divider).max(1.0);
                let right_width = (content_width * 0.38).max(270.0).min(content_width - 1.0);
                let left_width = content_width - right_width;
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(left_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| preview_pane(ui, draft, preview_text, SPLIT_PREVIEW_HEIGHT),
                    );
                    let divider_rect = ui
                        .allocate_exact_size(Vec2::new(divider, WORKFLOW_HEIGHT), Sense::hover())
                        .0;
                    ui.painter().rect_filled(divider_rect, 0.0, t.color.border);
                    ui.allocate_ui_with_layout(
                        Vec2::new(right_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| focus = fields_pane(ui, validation_message, draft),
                    );
                });
            } else {
                preview_pane(ui, draft, preview_text, STACKED_PREVIEW_HEIGHT);
                let (divider, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
                ui.painter().rect_filled(divider, 0.0, t.color.border);
                focus = fields_pane(ui, validation_message, draft);
            }
        });
    focus
}

fn uses_split_layout(available_width: f32) -> bool {
    available_width >= SPLIT_MIN_WIDTH
}

fn preview_pane(
    ui: &mut Ui,
    draft: &DesignNoteDialogState,
    preview_text: &str,
    preview_height: f32,
) {
    Frame::new()
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_head(ui, "TEXT · live schematic preview", "100 mil grid");
            paint_preview(ui, draft, preview_text, preview_height);
            ui.add_space(9.0);
            let values = [
                ("Object", "Calibration path"),
                ("Layer", "annotation only"),
                ("Electrical", "non-electrical note"),
            ];
            if ui.available_width() >= STATUS_CARDS_INLINE_MIN_WIDTH {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.columns(3, |columns| {
                    for (column, (label, value)) in columns.iter_mut().zip(values) {
                        status_card(column, label, value);
                    }
                });
            } else {
                for (label, value) in values {
                    status_card(ui, label, value);
                    ui.add_space(5.0);
                }
            }
        });
}

fn fields_pane(
    ui: &mut Ui,
    validation_message: Option<&str>,
    draft: &mut DesignNoteDialogState,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let mut focus = None;
    Frame::new()
        .fill(theme::mix(t.color.bg_inset, t.color.bg_panel, 0.94))
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 9.0;
            section_head(
                ui,
                "Documentation parameters",
                if validation_message.is_some() {
                    "blocked"
                } else {
                    "legal preview"
                },
            );
            let labels = DesignNoteKind::ALL.map(|kind| kind.label().to_owned());
            let kind_changed = field_label(ui, "Type", |ui| {
                select(
                    ui,
                    "design-note-kind",
                    "Type",
                    draft.kind.label(),
                    &labels,
                    ui.available_width(),
                )
            })
            .is_some_and(|index| {
                let replace_default = is_default_text(&draft.text);
                draft.kind = DesignNoteKind::ALL[index];
                if replace_default {
                    draft.text = default_text(draft.kind).to_owned();
                }
                true
            });
            let text_response = field_label(ui, "Text", |ui| {
                ui.add_sized(
                    [ui.available_width(), 72.0],
                    TextEdit::multiline(&mut draft.text)
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .hint_text("Bias network"),
                )
            });
            focus = Some(text_response.id);
            read_only_value(ui, "Layer", DesignNoteLayer::DrawingAnnotation.label());
            if kind_changed || text_response.changed() {
                draft.mark_edited();
            }
            if let Some(message) = validation_message {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    )
                    .wrap(),
                );
            } else {
                ui.label(
                    egui::RichText::new(
                        "The placed object is documentation-only. It is retained with the schematic and cannot create or rename electrical connectivity.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            }
        });
    focus
}

fn default_text(kind: DesignNoteKind) -> &'static str {
    match kind {
        DesignNoteKind::PlainText => "Bias network",
        DesignNoteKind::PropertyDisplay => "${component_count} components",
        DesignNoteKind::RequirementLink => "REQ-19",
        DesignNoteKind::ReviewNote => "Review bias network",
    }
}

fn is_default_text(text: &str) -> bool {
    DesignNoteKind::ALL
        .into_iter()
        .any(|kind| text == default_text(kind))
}

fn field_label<R>(ui: &mut Ui, label: &str, body: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        body(ui)
    })
    .inner
}

fn section_head(ui: &mut Ui, title: &str, status: &str) {
    if section_head_wraps(ui.available_width(), title, status) {
        let width = ui.available_width();
        section_title(ui, title);
        ui.add_space(2.0);
        ui.allocate_ui_with_layout(
            Vec2::new(width, 14.0),
            Layout::right_to_left(Align::Center),
            |ui| section_status(ui, status),
        );
        return;
    }
    ui.horizontal(|ui| {
        section_title(ui, title);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            section_status(ui, status);
        });
    });
}

fn section_head_wraps(width: f32, title: &str, status: &str) -> bool {
    width < 292.0 && title.chars().count() + status.chars().count() > 34
}

fn section_title(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .wrap(),
    );
}

fn section_status(ui: &mut Ui, status: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(status)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}

fn read_only_value(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    field_label(ui, label, |ui| {
        Frame::new()
            .fill(t.color.bg_app)
            .stroke(Stroke::new(1.0, t.color.border))
            .corner_radius(3.0)
            .inner_margin(egui::Margin::symmetric(8, 6))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
            });
    });
}

fn status_card(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(6.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(40.0);
            ui.label(
                egui::RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

fn paint_preview(ui: &mut Ui, draft: &DesignNoteDialogState, preview_text: &str, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!("{} preview: {}", draft.kind.label(), draft.text),
        )
    });
    ui.painter().rect(
        rect,
        8.0,
        t.color.canvas_bg,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    let mut y = rect.top() + 7.0;
    while y < rect.bottom() {
        let mut x = rect.left() + 7.0;
        while x < rect.right() {
            ui.painter()
                .circle_filled(egui::pos2(x, y), 0.7, t.color.canvas_grid);
            x += 12.0;
        }
        y += 12.0;
    }
    let painter = ui.painter().with_clip_rect(rect.shrink(2.0));
    let marker = egui::pos2(rect.left() + 28.0, rect.center().y);
    let color = match draft.kind {
        DesignNoteKind::ReviewNote => t.color.warn,
        DesignNoteKind::RequirementLink | DesignNoteKind::PropertyDisplay => t.color.accent,
        DesignNoteKind::PlainText => t.color.text,
    };
    match draft.kind {
        DesignNoteKind::PlainText => {
            painter.circle_filled(marker, 2.5, color);
        }
        DesignNoteKind::PropertyDisplay => {
            painter.rect_stroke(
                egui::Rect::from_center_size(marker, Vec2::splat(5.0)),
                0.0,
                Stroke::new(1.0, color),
                egui::StrokeKind::Inside,
            );
        }
        DesignNoteKind::RequirementLink => {
            painter.circle_stroke(marker, 2.5, Stroke::new(1.0, color));
        }
        DesignNoteKind::ReviewNote => {
            painter.line_segment(
                [
                    marker + egui::vec2(-2.5, -2.5),
                    marker + egui::vec2(2.5, 2.5),
                ],
                Stroke::new(1.0, color),
            );
            painter.line_segment(
                [
                    marker + egui::vec2(2.5, -2.5),
                    marker + egui::vec2(-2.5, 2.5),
                ],
                Stroke::new(1.0, color),
            );
        }
    }
    let origin = marker + egui::vec2(9.0, 0.0);
    for (index, line) in preview_text.split('\n').enumerate() {
        let position = origin + egui::vec2(0.0, index as f32 * 17.0);
        let galley = painter.layout_no_wrap(
            if line.is_empty() { " " } else { line }.to_owned(),
            theme::mono(tokens::FS_2, FontWeight::Regular),
            color,
        );
        painter.galley(position, galley.clone(), color);
        if draft.kind == DesignNoteKind::RequirementLink {
            painter.line_segment(
                [
                    position + egui::vec2(0.0, galley.size().y),
                    position + galley.size(),
                ],
                Stroke::new(1.0, color),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_contract_is_exact() {
        assert_eq!(TITLE, "Place text or design note");
        assert_eq!(EYEBROW, "SCHEMATIC · DOCUMENTATION");
        assert_eq!(PRIMARY, "Arm text tool");
        assert_eq!(
            DesignNoteLayer::DrawingAnnotation.label(),
            "drawing / annotation"
        );
        assert_eq!(
            DesignNoteKind::ALL.map(DesignNoteKind::label),
            [
                "Plain text",
                "Property display",
                "Requirement link",
                "Review note"
            ]
        );
    }

    #[test]
    fn valid_draft_freezes_authority_without_mutating_document() {
        let mut state = AppState::default();
        state.dialogs.design_note.open(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        let DraftValidation::Valid(pending) = validate_draft(&state) else {
            panic!("valid draft");
        };
        assert_eq!(pending.text, "Bias network");
        assert!(pending.document_authority.is_some());
        assert!(state.schematic.design_notes.is_empty());
    }

    #[test]
    fn stale_and_read_only_drafts_fail_closed() {
        let mut state = AppState::default();
        state.dialogs.design_note.open(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        state.schematic.read_only = true;
        assert!(matches!(
            validate_draft(&state),
            DraftValidation::Invalid(_)
        ));
        state.schematic.read_only = false;
        state.design_execution_epoch += 1;
        assert!(matches!(
            validate_draft(&state),
            DraftValidation::Invalid(_)
        ));
    }

    #[test]
    fn changing_type_preserves_authored_text_but_recognizes_known_defaults() {
        assert!(is_default_text("Bias network"));
        assert!(is_default_text("REQ-19"));
        assert!(!is_default_text("User-authored note"));
    }

    #[test]
    fn narrow_section_heads_wrap_before_labels_overlap() {
        assert!(section_head_wraps(
            270.0,
            "Documentation parameters",
            "legal preview"
        ));
        assert!(!section_head_wraps(
            410.0,
            "Documentation parameters",
            "legal preview"
        ));
    }

    #[test]
    fn responsive_workflow_keeps_compact_fields_above_the_footer() {
        assert!(uses_split_layout(SPLIT_MIN_WIDTH));
        assert!(!uses_split_layout(SPLIT_MIN_WIDTH - 1.0));
        assert!(STACKED_PREVIEW_HEIGHT < SPLIT_PREVIEW_HEIGHT);
        assert!(STATUS_CARDS_INLINE_MIN_WIDTH < SPLIT_MIN_WIDTH);
    }
}
