//! Mockup-owned Place bus tap transaction.
//!
//! The form retains only an isolated text draft. Its primary action parses
//! and range-checks a complete typed contract before publishing the one
//! pending placement configuration consumed by the schematic authoring tool.

use egui::{Align, Context, Frame, Layout, Rect, Response, Sense, Stroke, TextEdit, Ui, Vec2};

use crate::state::{BusDeclaration, BusSlice, BusTapOrientation, PendingBusTap, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
};

use super::{BusTapDialogState, RSpiceApp};

const EYEBROW: &str = "SCHEMATIC \u{00b7} CONNECTIVITY";
const TITLE: &str = "Place bus tap";
const PRIMARY: &str = "Arm bus-tap tool";
const BODY: &str = "Create a typed scalar or slice connection from a declared bus with direction and naming validation.";
const BUS_EXAMPLE: &str = "DATA[15:0]";
const SLICE_EXAMPLE: &str = "DATA[7:0]";
const ORIENTATION_LABELS: [&str; 5] = ["Automatic", "Left", "Right", "Up", "Down"];
const DIALOG_SIZE: DialogSize = DialogSize::Transaction;
const LEFT_PANE_WEIGHT: f32 = 1.55;
const RIGHT_PANE_WEIGHT: f32 = 0.8;
const RIGHT_PANE_MIN_WIDTH: f32 = 270.0;
const PANE_PADDING: i8 = 14;
const PREVIEW_MIN_HEIGHT: f32 = 250.0;
const STATUS_CARD_COUNT: usize = 3;
const STATUS_GAP: f32 = 8.0;
const STATUS_TOP_MARGIN: f32 = 9.0;
const STATUS_CARD_CONTENT_GAP: f32 = 4.0;
const STATUS_CARD_TOTAL_MARGIN: f32 = 22.0;
const STATUS_DOT_SLOT_WIDTH: f32 = 14.0;
const WORKFLOW_PANE_MIN_HEIGHT: f32 = 384.0;
const PREVIEW_TITLE: &str = "TAP \u{00b7} live schematic preview";
const VALID_CHECKS: &str = "connectivity \u{00b7} discipline \u{00b7} hierarchy";
const VALID_COMMIT: &str = "stable IDs + one undo record";
const INVALID_OUTCOME: &str = "no electrical change";
const INVALID_CHECKS: &str = "blocked \u{00b7} resolve field validation";
const INVALID_COMMIT: &str = "disabled \u{00b7} document unchanged";
const POINTER_NOTE: &str = "Pointer, touch, stylus, and keyboard entry resolve to the same exact coordinates. Escape cancels without modifying the document.";
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No project or result data has been changed.";

#[derive(Debug)]
enum DraftValidation {
    Incomplete(&'static str),
    Invalid(String),
    Valid(PendingBusTap),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewContract {
    legal: bool,
    subject: String,
    location: String,
    outcome: String,
    checks: String,
    commit: String,
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Incomplete(message) => Some(message),
            Self::Invalid(message) => Some(message.as_str()),
            Self::Valid(_) => None,
        }
    }
}

impl RSpiceApp {
    pub(super) fn render_bus_tap_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.bus_tap.open {
            return;
        }

        let editable = !self.state.schematic.read_only;
        let validation = if editable {
            validate_draft(&self.state.dialogs.bus_tap)
        } else {
            DraftValidation::Invalid("The active schematic is read-only.".to_owned())
        };
        let can_commit = editable && validation.can_commit();
        let message = validation.message().map(str::to_owned);
        let preview = preview_contract(&validation, &self.state.dialogs.bus_tap);

        let discard_confirm = self.state.dialogs.bus_tap.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(BODY)
            .size(DIALOG_SIZE)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(can_commit)
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            schematic_workflow_body(
                ui,
                &preview,
                message.as_deref(),
                matches!(&validation, DraftValidation::Incomplete(_)),
                &mut self.state.dialogs.bus_tap,
            )
        });

        match choice {
            DialogChoice::Primary => {
                // The footer follows the body in the same immediate-mode
                // frame. Re-parse the post-edit draft so Enter can never
                // publish the prior frame's valid contract after a field was
                // changed to an invalid value.
                if !self.state.schematic.read_only
                    && let DraftValidation::Valid(pending) =
                        validate_draft(&self.state.dialogs.bus_tap)
                {
                    self.state.schematic.pending_bus_tap = Some(pending);
                    crate::workbench::commands::arm_schematic_tool(
                        &mut self.state.schematic,
                        Tool::BusTap,
                    );
                    self.state.dialogs.bus_tap.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.bus_tap.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn preview_contract(validation: &DraftValidation, draft: &BusTapDialogState) -> PreviewContract {
    match validation {
        DraftValidation::Valid(pending) => PreviewContract {
            legal: true,
            subject: pending.slice.to_string(),
            location: pending.bus_declaration.to_string(),
            outcome: if pending.slice.is_scalar() {
                "one typed scalar connection".to_owned()
            } else {
                format!("{}-member typed bus connection", pending.slice.width())
            },
            checks: VALID_CHECKS.to_owned(),
            commit: VALID_COMMIT.to_owned(),
        },
        DraftValidation::Incomplete(_) | DraftValidation::Invalid(_) => PreviewContract {
            legal: false,
            subject: nonempty_or(&draft.slice, "tap unresolved"),
            location: nonempty_or(&draft.bus, "bus unresolved"),
            outcome: INVALID_OUTCOME.to_owned(),
            checks: INVALID_CHECKS.to_owned(),
            commit: INVALID_COMMIT.to_owned(),
        },
    }
}

fn nonempty_or(value: &str, fallback: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        fallback.to_owned()
    } else {
        value.to_owned()
    }
}

fn schematic_workflow_body(
    ui: &mut Ui,
    preview: &PreviewContract,
    validation_message: Option<&str>,
    validation_incomplete: bool,
    draft: &mut BusTapDialogState,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let mut focus = None;
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(10.0)
        .show(ui, |ui| {
            let divider = 1.0;
            let content = (ui.available_width() - divider).max(1.0);
            let (left_width, right_width) = workflow_pane_widths(content);
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(left_width, WORKFLOW_PANE_MIN_HEIGHT),
                    Layout::top_down(Align::Min),
                    |ui| {
                        Frame::new()
                            .inner_margin(egui::Margin::same(PANE_PADDING))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.set_min_height(
                                    WORKFLOW_PANE_MIN_HEIGHT - f32::from(PANE_PADDING) * 2.0,
                                );
                                workflow_section_head(ui, PREVIEW_TITLE, "100 mil grid", None);
                                paint_schematic_preview(ui, preview);
                                ui.add_space(STATUS_TOP_MARGIN);
                                status_cards(ui, preview);
                            });
                    },
                );
                let divider_rect = ui
                    .allocate_exact_size(
                        Vec2::new(divider, WORKFLOW_PANE_MIN_HEIGHT),
                        Sense::hover(),
                    )
                    .0;
                ui.painter().rect_filled(divider_rect, 0.0, t.color.border);
                ui.allocate_ui_with_layout(
                    Vec2::new(right_width, WORKFLOW_PANE_MIN_HEIGHT),
                    Layout::top_down(Align::Min),
                    |ui| {
                        Frame::new()
                            .fill(theme::mix(t.color.bg_inset, t.color.bg_panel, 0.94))
                            .inner_margin(egui::Margin::same(PANE_PADDING))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.set_min_height(
                                    WORKFLOW_PANE_MIN_HEIGHT - f32::from(PANE_PADDING) * 2.0,
                                );
                                workflow_section_head(
                                    ui,
                                    "Placement / transform parameters",
                                    if preview.legal {
                                        "legal preview"
                                    } else {
                                        "blocked"
                                    },
                                    Some(preview.legal),
                                );
                                let (body_focus, edited) = typed_fields(ui, draft);
                                focus = body_focus;
                                if edited {
                                    draft.mark_edited();
                                }

                                if let Some(message) = validation_message {
                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(message)
                                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                            .color(if validation_incomplete {
                                                t.color.text_dim
                                            } else {
                                                t.color.err
                                            }),
                                    );
                                }
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new(POINTER_NOTE)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                );
                            });
                    },
                );
            });
        });
    focus
}

fn workflow_pane_widths(content_width: f32) -> (f32, f32) {
    let content_width = content_width.max(1.0);
    let weighted_right = content_width * RIGHT_PANE_WEIGHT / (LEFT_PANE_WEIGHT + RIGHT_PANE_WEIGHT);
    let right = weighted_right.max(RIGHT_PANE_MIN_WIDTH).min(content_width);
    ((content_width - right).max(1.0), right)
}

fn workflow_section_head(ui: &mut Ui, title: &str, trailing: &str, legal: Option<bool>) {
    let t = Tokens::get(ui.ctx());
    let title_font = theme::sans(tokens::FS_1, FontWeight::SemiBold);
    let trailing_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let title_width = ui
        .painter()
        .layout_no_wrap(title.to_owned(), title_font.clone(), t.color.text)
        .size()
        .x;
    let trailing_width = ui
        .painter()
        .layout_no_wrap(trailing.to_owned(), trailing_font.clone(), t.color.text_dim)
        .size()
        .x;
    let item_gap = ui.spacing().item_spacing.x;
    let status_dot_width = legal.map_or(0.0, |_| STATUS_DOT_SLOT_WIDTH);
    let stacks = workflow_head_stacks(
        ui.available_width(),
        title_width,
        trailing_width,
        status_dot_width,
        item_gap,
    );

    let paint_title = |ui: &mut Ui| {
        ui.label(
            egui::RichText::new(title)
                .font(title_font.clone())
                .color(t.color.text),
        );
    };
    if stacks {
        paint_title(ui);
        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                workflow_section_status(ui, trailing, legal, trailing_font.clone(), &t);
            });
        });
    } else {
        ui.horizontal(|ui| {
            paint_title(ui);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                workflow_section_status(ui, trailing, legal, trailing_font.clone(), &t);
            });
        });
    }
    ui.add_space(7.0);
}

fn workflow_head_stacks(
    available_width: f32,
    title_width: f32,
    trailing_width: f32,
    status_dot_width: f32,
    item_gap: f32,
) -> bool {
    title_width + trailing_width + status_dot_width + item_gap > available_width
}

fn workflow_section_status(
    ui: &mut Ui,
    trailing: &str,
    legal: Option<bool>,
    font: egui::FontId,
    t: &Tokens,
) {
    let color = legal.map_or(t.color.text_dim, |legal| {
        if legal { t.color.ok } else { t.color.err }
    });
    ui.label(egui::RichText::new(trailing).font(font).color(color));
    if let Some(legal) = legal {
        let (dot, _) =
            ui.allocate_exact_size(Vec2::new(STATUS_DOT_SLOT_WIDTH, 8.0), Sense::hover());
        ui.painter().circle_filled(
            egui::pos2(dot.left() + 3.0, dot.center().y),
            2.5,
            if legal { t.color.ok } else { t.color.err },
        );
    }
}

fn paint_schematic_preview(ui: &mut Ui, preview: &PreviewContract) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PREVIEW_MIN_HEIGHT),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            true,
            format!("Preview {} for {}", preview.outcome, preview.subject),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Image);
        node.set_label(format!(
            "Preview {} for {} on {}",
            preview.outcome, preview.subject, preview.location
        ));
    });

    let painter = ui.painter();
    painter.rect(
        rect,
        8.0,
        t.color.canvas_bg,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    let mut x = rect.left();
    while x < rect.right() {
        let mut y = rect.top();
        while y < rect.bottom() {
            painter.circle_filled(egui::pos2(x, y), 0.75, t.color.canvas_grid);
            y += 12.0;
        }
        x += 12.0;
    }

    let horizontal_y = rect.top() + rect.height() * 0.47;
    let crossing_x = rect.left() + rect.width() * 0.51;
    let conductor = if preview.legal {
        t.color.wire
    } else {
        t.color.text_faint
    };
    painter.hline(
        (rect.left() + rect.width() * 0.08)..=(rect.right() - rect.width() * 0.08),
        horizontal_y,
        Stroke::new(2.0, conductor),
    );
    painter.vline(
        crossing_x,
        (rect.top() + rect.height() * 0.20)..=(rect.bottom() - rect.height() * 0.18),
        Stroke::new(2.0, conductor),
    );

    let source_center = egui::pos2(
        rect.left() + rect.width() * 0.22 + 27.0,
        rect.top() + rect.height() * 0.34 + 31.0,
    );
    painter.add(egui::Shape::closed_line(
        vec![
            source_center + egui::vec2(-25.0, -31.0),
            source_center + egui::vec2(-25.0, 31.0),
            source_center + egui::vec2(29.0, 0.0),
        ],
        Stroke::new(2.0, t.color.symbol),
    ));
    let target = Rect::from_min_size(
        egui::pos2(
            rect.right() - rect.width() * 0.22 - 56.0,
            rect.top() + rect.height() * 0.42,
        ),
        Vec2::new(56.0, 20.0),
    );
    painter.rect_stroke(
        target,
        0.0,
        Stroke::new(2.0, t.color.symbol),
        egui::StrokeKind::Inside,
    );

    let cursor_color = if preview.legal {
        t.color.accent
    } else {
        t.color.err
    };
    painter.circle_stroke(
        egui::pos2(crossing_x, horizontal_y),
        7.0,
        Stroke::new(2.0, cursor_color),
    );
    painter.circle_stroke(
        egui::pos2(crossing_x, horizontal_y),
        12.0,
        Stroke::new(1.0, cursor_color.gamma_multiply(0.45)),
    );

    let tooltip_width = (rect.width() * 0.44).clamp(132.0, 210.0);
    let tooltip = Rect::from_min_size(
        egui::pos2(crossing_x + 12.0, horizontal_y + 12.0),
        Vec2::new(tooltip_width, 47.0),
    )
    .intersect(rect.shrink(8.0));
    painter.rect(
        tooltip,
        6.0,
        t.color.bg_inset,
        Stroke::new(1.0, cursor_color.gamma_multiply(0.7)),
        egui::StrokeKind::Inside,
    );
    painter.text(
        tooltip.left_top() + egui::vec2(9.0, 10.0),
        egui::Align2::LEFT_TOP,
        &preview.subject,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
    painter.text(
        tooltip.left_bottom() + egui::vec2(9.0, -9.0),
        egui::Align2::LEFT_BOTTOM,
        &preview.location,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn status_cards(ui: &mut Ui, preview: &PreviewContract) {
    let cards = [
        ("Electrical outcome", preview.outcome.as_str()),
        ("Checks", preview.checks.as_str()),
        ("Commit", preview.commit.as_str()),
    ];
    debug_assert_eq!(cards.len(), STATUS_CARD_COUNT);
    let width = (ui.available_width() - STATUS_GAP * (STATUS_CARD_COUNT.saturating_sub(1) as f32))
        / STATUS_CARD_COUNT as f32;
    let height = status_card_row_height(ui, width, &cards);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = STATUS_GAP;
        for (label, value) in cards {
            ui.allocate_ui_with_layout(
                Vec2::new(width, height),
                Layout::top_down(Align::Min),
                |ui| status_card(ui, width, height, label, value, preview.legal),
            );
        }
    });
}

fn status_card_row_height(ui: &mut Ui, width: f32, cards: &[(&str, &str)]) -> f32 {
    let t = Tokens::get(ui.ctx());
    let content_width = (width - STATUS_CARD_TOTAL_MARGIN).max(1.0);
    let label_height = ui
        .painter()
        .layout_no_wrap(
            "Ag".to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        )
        .size()
        .y;
    let value_height = cards
        .iter()
        .map(|(_, value)| {
            let mut job = egui::text::LayoutJob::single_section(
                (*value).to_owned(),
                egui::TextFormat {
                    font_id: theme::sans(tokens::FS_1, FontWeight::SemiBold),
                    color: t.color.text,
                    ..Default::default()
                },
            );
            job.wrap.max_width = content_width;
            ui.fonts_mut(|fonts| fonts.layout_job(job)).size().y
        })
        .fold(0.0_f32, f32::max);

    STATUS_CARD_TOTAL_MARGIN + label_height + STATUS_CARD_CONTENT_GAP + value_height
}

fn status_card(ui: &mut Ui, width: f32, height: f32, label: &str, value: &str, legal: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(7.0)
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width((width - STATUS_CARD_TOTAL_MARGIN).max(1.0));
            ui.set_min_height((height - STATUS_CARD_TOTAL_MARGIN).max(1.0));
            ui.spacing_mut().item_spacing.y = STATUS_CARD_CONTENT_GAP;
            status_card_label(ui, label);
            ui.label(
                egui::RichText::new(value)
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(if legal { t.color.text } else { t.color.err }),
            );
        });
}

fn status_card_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &label.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_dim,
            extra_letter_spacing: 0.045 * tokens::FS_0,
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), galley.size().y),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    ui.painter().galley(rect.min, galley, t.color.text_dim);
}

fn validate_draft(draft: &BusTapDialogState) -> DraftValidation {
    let bus = draft.bus.trim();
    let slice = draft.slice.trim();
    if bus.is_empty() && slice.is_empty() {
        return DraftValidation::Incomplete("Enter a declared bus and the scalar or range to tap.");
    }
    if bus.is_empty() {
        return DraftValidation::Incomplete("Enter the declared bus, for example DATA[15:0].");
    }
    if slice.is_empty() {
        return DraftValidation::Incomplete("Enter the tap slice, for example DATA[7:0].");
    }

    let declaration = match BusDeclaration::parse(bus) {
        Ok(declaration) => declaration,
        Err(error) => return DraftValidation::Invalid(format!("Bus: {error}")),
    };
    let slice = match BusSlice::parse(slice) {
        Ok(slice) => slice,
        Err(error) => return DraftValidation::Invalid(format!("Slice: {error}")),
    };
    if let Err(error) = declaration.validate_slice(&slice) {
        return DraftValidation::Invalid(error.to_string());
    }
    match PendingBusTap::new(declaration, slice, draft.orientation) {
        Ok(pending) => DraftValidation::Valid(pending),
        Err(error) => DraftValidation::Invalid(error.to_string()),
    }
}

fn typed_fields(ui: &mut Ui, draft: &mut BusTapDialogState) -> (Option<egui::Id>, bool) {
    let mut focus = None;
    let mut edited = false;
    Frame::new()
        .inner_margin(egui::Margin::symmetric(10, 9))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 9.0;
            let bus = input_with_example(ui, "Bus", &mut draft.bus, BUS_EXAMPLE);
            focus = Some(bus.id);
            edited |= bus.changed();
            edited |= input_with_example(ui, "Slice", &mut draft.slice, SLICE_EXAMPLE).changed();
            edited |= orientation_field(ui, &mut draft.orientation);
        });
    (focus, edited)
}

fn input_with_example(ui: &mut Ui, label: &str, value: &mut String, example: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        let response = ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            TextEdit::singleline(value)
                .font(egui::TextStyle::Monospace)
                .hint_text(example)
                .margin(egui::Margin::symmetric(8, 4)),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(label);
            node.set_description(format!("Example: {example}"));
        });
        response
    })
    .inner
}

fn orientation_field(ui: &mut Ui, orientation: &mut BusTapOrientation) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(
            egui::RichText::new("Orientation")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        let options = ORIENTATION_LABELS.map(str::to_owned);
        if let Some(index) = select(
            ui,
            "bus-tap-orientation",
            "Bus tap orientation",
            orientation_label(*orientation),
            &options,
            ui.available_width(),
        ) {
            *orientation = orientation_at(index);
            true
        } else {
            false
        }
    })
    .inner
}

const fn orientation_label(orientation: BusTapOrientation) -> &'static str {
    match orientation {
        BusTapOrientation::Automatic => "Automatic",
        BusTapOrientation::Left => "Left",
        BusTapOrientation::Right => "Right",
        BusTapOrientation::Up => "Up",
        BusTapOrientation::Down => "Down",
    }
}

const fn orientation_at(index: usize) -> BusTapOrientation {
    match index {
        1 => BusTapOrientation::Left,
        2 => BusTapOrientation::Right,
        3 => BusTapOrientation::Up,
        4 => BusTapOrientation::Down,
        _ => BusTapOrientation::Automatic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    #[test]
    fn mockup_contract_is_exact_and_complete() {
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} CONNECTIVITY");
        assert_eq!(TITLE, "Place bus tap");
        assert_eq!(PRIMARY, "Arm bus-tap tool");
        assert_eq!(
            BODY,
            "Create a typed scalar or slice connection from a declared bus with direction and naming validation."
        );
        assert_eq!(BUS_EXAMPLE, "DATA[15:0]");
        assert_eq!(SLICE_EXAMPLE, "DATA[7:0]");
        assert_eq!(DIALOG_SIZE, DialogSize::Transaction);
        assert_eq!(
            ORIENTATION_LABELS,
            ["Automatic", "Left", "Right", "Up", "Down"]
        );
    }

    #[test]
    fn dialog_opens_with_the_mockup_values_and_a_valid_clean_draft() {
        let mut draft = BusTapDialogState::default();
        draft.open();

        assert!(draft.open);
        assert_eq!(draft.bus, BUS_EXAMPLE);
        assert_eq!(draft.slice, SLICE_EXAMPLE);
        assert_eq!(draft.orientation, BusTapOrientation::Automatic);
        assert!(!draft.dirty);
        assert!(!draft.discard_confirm);
        assert!(validate_draft(&draft).can_commit());
    }

    #[test]
    fn incomplete_drafts_never_publish_a_pending_configuration() {
        let mut draft = BusTapDialogState::default();
        assert!(!validate_draft(&draft).can_commit());
        draft.bus = BUS_EXAMPLE.to_owned();
        assert!(!validate_draft(&draft).can_commit());
    }

    #[test]
    fn valid_scalar_and_slice_targets_are_accepted() {
        for slice in ["DATA[7]", "DATA[7:0]"] {
            let draft = BusTapDialogState {
                open: true,
                bus: BUS_EXAMPLE.to_owned(),
                slice: slice.to_owned(),
                orientation: BusTapOrientation::Automatic,
                ..BusTapDialogState::default()
            };
            assert!(validate_draft(&draft).can_commit(), "{slice}");
        }
    }

    #[test]
    fn base_and_range_mismatches_are_rejected_live() {
        for slice in ["ADDR[7:0]", "DATA[16]", "DATA[0:8]"] {
            let draft = BusTapDialogState {
                open: true,
                bus: BUS_EXAMPLE.to_owned(),
                slice: slice.to_owned(),
                orientation: BusTapOrientation::Automatic,
                ..BusTapDialogState::default()
            };
            assert!(!validate_draft(&draft).can_commit(), "{slice}");
        }
    }

    #[test]
    fn every_mockup_orientation_round_trips_through_the_select_projection() {
        for (index, label) in ORIENTATION_LABELS.iter().enumerate() {
            assert_eq!(orientation_label(orientation_at(index)), *label);
        }
    }

    #[test]
    fn edited_drafts_require_an_explicit_second_discard_action() {
        let mut draft = BusTapDialogState::default();
        draft.open();
        assert!(draft.attempt_close());
        assert!(!draft.open);

        draft.open();
        draft.mark_edited();
        assert!(!draft.attempt_close());
        assert!(draft.open);
        assert!(draft.discard_confirm);

        draft.mark_edited();
        assert!(!draft.discard_confirm);
        assert!(!draft.attempt_close());
        assert!(draft.attempt_close());
        assert!(!draft.open);
    }

    #[test]
    fn rendered_primary_revalidates_publishes_and_arms_the_bus_tap_tool() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.dialogs.bus_tap.open();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });

        assert!(!app.state.dialogs.bus_tap.open);
        assert_eq!(app.state.schematic.tool, Tool::BusTap);
        let pending = app
            .state
            .schematic
            .pending_bus_tap
            .as_ref()
            .expect("validated pending bus tap");
        assert_eq!(pending.bus_declaration.to_string(), BUS_EXAMPLE);
        assert_eq!(pending.slice.to_string(), SLICE_EXAMPLE);
    }

    #[test]
    fn rendered_read_only_dialog_cannot_publish_or_arm() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.schematic.read_only = true;
        app.state.dialogs.bus_tap.open();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });

        assert!(app.state.dialogs.bus_tap.open);
        assert_eq!(app.state.schematic.tool, Tool::Select);
        assert!(app.state.schematic.pending_bus_tap.is_none());
    }

    #[test]
    fn rendered_escape_requires_two_actions_for_an_edited_draft() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.dialogs.bus_tap.open();
        app.state.dialogs.bus_tap.mark_edited();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        assert!(app.state.dialogs.bus_tap.open);
        assert!(app.state.dialogs.bus_tap.discard_confirm);

        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        assert!(!app.state.dialogs.bus_tap.open);
    }

    #[test]
    fn default_transaction_width_preserves_the_mockup_minimum_right_pane() {
        for content_width in [510.0, 600.0, 734.0] {
            let (left, right) = workflow_pane_widths(content_width);
            assert!(left > 0.0);
            assert!(right >= RIGHT_PANE_MIN_WIDTH);
            assert!((left + right - content_width).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn workflow_header_stacks_before_title_and_status_can_overlap() {
        assert!(workflow_head_stacks(244.0, 188.0, 72.0, 14.0, 8.0));
        assert!(!workflow_head_stacks(290.0, 188.0, 72.0, 14.0, 8.0));
    }

    #[test]
    fn preview_contract_is_typed_when_valid_and_fails_closed_when_invalid() {
        let mut draft = BusTapDialogState::default();
        draft.open();
        let valid = validate_draft(&draft);
        let preview = preview_contract(&valid, &draft);
        assert!(preview.legal);
        assert_eq!(preview.subject, SLICE_EXAMPLE);
        assert_eq!(preview.location, BUS_EXAMPLE);
        assert_eq!(preview.outcome, "8-member typed bus connection");

        draft.slice = "ADDR[7:0]".to_owned();
        let invalid = preview_contract(&validate_draft(&draft), &draft);
        assert!(!invalid.legal);
        assert_eq!(invalid.outcome, INVALID_OUTCOME);
        assert_eq!(invalid.checks, INVALID_CHECKS);
        assert_eq!(invalid.commit, INVALID_COMMIT);
    }

    #[test]
    fn rendered_contract_strings_contain_no_mojibake_markers() {
        let mut draft = BusTapDialogState::default();
        draft.open();
        let valid = preview_contract(&validate_draft(&draft), &draft);
        draft.slice = "ADDR[7:0]".to_owned();
        let invalid = preview_contract(&validate_draft(&draft), &draft);
        let strings = [
            EYEBROW,
            TITLE,
            PRIMARY,
            BODY,
            PREVIEW_TITLE,
            POINTER_NOTE,
            DISCARD_TITLE,
            DISCARD_DETAIL,
            valid.checks.as_str(),
            valid.commit.as_str(),
            invalid.checks.as_str(),
            invalid.commit.as_str(),
        ];
        for value in strings {
            for forbidden in ['\u{00c2}', '\u{00e2}', '\u{fffd}'] {
                assert!(
                    !value.contains(forbidden),
                    "mojibake in rendered contract string: {value:?}"
                );
            }
        }
    }
}
