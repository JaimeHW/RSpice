//! Mockup-owned Place pin or port transaction.
//!
//! The dialog edits only an isolated draft. Its primary action freezes one
//! validated interface contract; the schematic is mutated later, on the
//! snapped canvas click, as one undoable placement transaction.

use egui::{Align, Context, Frame, Layout, Response, Sense, Stroke, TextEdit, Ui, Vec2};

use crate::state::{PendingPortPlacement, PortDirectionType, PortDiscipline, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
};

use crate::workbench::app::{AppState, PinPortDialogState, RSpiceApp};

const EYEBROW: &str = "SCHEMATIC \u{00b7} INTERFACE CONTRACT";
const TITLE: &str = "Place pin or port";
const PRIMARY: &str = "Arm pin tool";
const DESCRIPTION: &str = "Create a named typed terminal with direction, discipline, netlist order, symbol synchronization, and documentation.";
const PREVIEW_TITLE: &str = "PIN \u{00b7} live schematic preview";
const DIRECTION_LABEL: &str = "Direction / type";
const DISCIPLINE_LABEL: &str = "Discipline";
const WORKFLOW_HEIGHT: f32 = 414.0;
const SPLIT_VIEWPORT_BREAKPOINT: f32 = 760.0;
const COMPACT_COLUMNS_BREAKPOINT: f32 = 980.0;
const RIGHT_MIN_WIDTH: f32 = 270.0;
const COMPACT_RIGHT_MIN_WIDTH: f32 = 240.0;
const PANE_PADDING: i8 = 14;
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No schematic or interface data has been changed.";
type PreviewMarkerSegment = ((f32, f32), (f32, f32));

#[derive(Debug)]
enum DraftValidation {
    Invalid(String),
    Valid(PendingPortPlacement),
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
    pub(in crate::workbench::app) fn render_pin_port_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.pin_port.open {
            return;
        }

        let validation = validate_draft(&self.state);
        let can_commit = validation.can_commit();
        let validation_message = validation.message().map(str::to_owned);
        let discard_confirm = self.state.dialogs.pin_port.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
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
            workflow_body(
                ui,
                validation_message.as_deref(),
                &mut self.state.dialogs.pin_port,
            )
        });

        match choice {
            DialogChoice::Primary => {
                // Revalidate the post-edit frame. Enter must never publish a
                // contract made invalid by the same frame's text edit.
                if let DraftValidation::Valid(pending) = validate_draft(&self.state) {
                    self.state.schematic.pending_port = Some(pending);
                    crate::workbench::commands::arm_schematic_tool(
                        &mut self.state.schematic,
                        Tool::Place(crate::state::ComponentType::Port),
                    );
                    self.state.dialogs.pin_port.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.pin_port.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.pin_port;
    if state.schematic.read_only || state.active_view_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    if draft.design_execution_epoch != state.design_execution_epoch {
        return DraftValidation::Invalid(
            "The design document changed. Close and reopen Place pin or port.".to_owned(),
        );
    }
    if draft.active_schematic_epoch != state.active_schematic_epoch {
        return DraftValidation::Invalid(
            "The active schematic buffer changed. Close and reopen Place pin or port.".to_owned(),
        );
    }
    if draft.topology_version != state.schematic.topology_version() {
        return DraftValidation::Invalid(
            "The schematic topology changed. Close and reopen Place pin or port.".to_owned(),
        );
    }
    if draft.view_path != state.workspace.active_view.display_path() {
        return DraftValidation::Invalid(
            "The active cell/view changed. Close and reopen Place pin or port.".to_owned(),
        );
    }
    let name = draft.name.trim();
    if let Err(error) = state.schematic.validate_new_port_name(name) {
        return DraftValidation::Invalid(error.to_string());
    }
    DraftValidation::Valid(
        PendingPortPlacement::new(
            name,
            draft.direction_type,
            draft.discipline,
            draft.topology_version,
            state.schematic.next_interface_order(),
        )
        .with_document_authority(
            draft.design_execution_epoch,
            draft.active_schematic_epoch,
            draft.view_path.clone(),
        ),
    )
}

fn workflow_body(
    ui: &mut Ui,
    validation_message: Option<&str>,
    draft: &mut PinPortDialogState,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let mut focus = None;
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let viewport_width = crate::ui::viewport::root_viewport_width(ui.ctx());
            if workflow_uses_columns(viewport_width) {
                let divider = 1.0;
                let content_width = (ui.available_width() - divider).max(1.0);
                let (right_fraction, right_min_width) = workflow_right_track(viewport_width);
                let right_width = (content_width * right_fraction)
                    .max(right_min_width)
                    .min((content_width - 1.0).max(1.0));
                let left_width = (content_width - right_width).max(1.0);
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(left_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| preview_pane(ui, draft),
                    );
                    let divider_rect = ui
                        .allocate_exact_size(Vec2::new(divider, WORKFLOW_HEIGHT), Sense::hover())
                        .0;
                    ui.painter().rect_filled(divider_rect, 0.0, t.color.border);
                    ui.allocate_ui_with_layout(
                        Vec2::new(right_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| {
                            focus = fields_pane(ui, validation_message, draft);
                        },
                    );
                });
            } else {
                preview_pane(ui, draft);
                let (divider, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
                ui.painter().rect_filled(divider, 0.0, t.color.border);
                focus = fields_pane(ui, validation_message, draft);
            }
        });
    focus
}

fn workflow_uses_columns(viewport_width: f32) -> bool {
    viewport_width > SPLIT_VIEWPORT_BREAKPOINT
}

fn workflow_right_track(viewport_width: f32) -> (f32, f32) {
    if viewport_width <= COMPACT_COLUMNS_BREAKPOINT {
        (0.72 / 1.72, COMPACT_RIGHT_MIN_WIDTH)
    } else {
        (0.8 / 2.35, RIGHT_MIN_WIDTH)
    }
}

fn preview_pane(ui: &mut Ui, draft: &PinPortDialogState) {
    Frame::new()
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_head(ui, PREVIEW_TITLE, "100 mil grid");
            let preview_height = if ui.available_width() >= 360.0 {
                250.0
            } else {
                184.0
            };
            paint_preview(ui, draft, preview_height);
            ui.add_space(9.0);
            let values = [
                ("Electrical outcome", "sheet/hierarchy interface".to_owned()),
                (
                    "Checks",
                    "connectivity \u{00b7} discipline \u{00b7} hierarchy".to_owned(),
                ),
                ("Commit", "stable IDs + one undo record".to_owned()),
            ];
            if ui.available_width() >= 390.0 {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.columns(3, |columns| {
                    for (column, (label, value)) in columns.iter_mut().zip(values.iter()) {
                        status_card(column, label, value);
                    }
                });
            } else {
                for (label, value) in &values {
                    status_card(ui, label, value);
                    ui.add_space(5.0);
                }
            }
        });
}

fn fields_pane(
    ui: &mut Ui,
    validation_message: Option<&str>,
    draft: &mut PinPortDialogState,
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
                "Placement / transform parameters",
                if validation_message.is_some() {
                    "blocked"
                } else {
                    "legal preview"
                },
            );
            let name_response = input_field(ui, "Name", &mut draft.name, "BIAS_EN");
            focus = Some(name_response.id);
            let mut edited = name_response.changed();
            edited |= direction_type_field(ui, &mut draft.direction_type);
            edited |= discipline_field(ui, &mut draft.discipline);
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
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "Pointer, touch, stylus, and keyboard entry resolve to the same exact coordinates. Escape cancels without modifying the document.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            }
            if edited {
                draft.mark_edited();
            }
        });
    focus
}

fn section_head(ui: &mut Ui, title: &str, status: &str) {
    if section_head_wraps(ui.available_width(), title, status) {
        let width = ui.available_width();
        section_title(ui, title);
        ui.add_space(2.0);
        ui.allocate_ui_with_layout(
            Vec2::new(width, 14.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                section_status(ui, status);
            },
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

fn input_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> Response {
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
                .hint_text(hint)
                .margin(egui::Margin::symmetric(8, 4)),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(label);
            node.set_description("Named schematic net and cell-interface terminal");
        });
        response
    })
    .inner
}

fn direction_type_field(ui: &mut Ui, value: &mut PortDirectionType) -> bool {
    enum_field(
        ui,
        "pin-port-direction-type",
        DIRECTION_LABEL,
        value.label(),
        &PortDirectionType::ALL.map(|entry| entry.label().to_owned()),
    )
    .is_some_and(|index| {
        *value = PortDirectionType::ALL[index];
        true
    })
}

fn discipline_field(ui: &mut Ui, value: &mut PortDiscipline) -> bool {
    enum_field(
        ui,
        "pin-port-discipline",
        DISCIPLINE_LABEL,
        value.keyword(),
        &PortDiscipline::ALL.map(|entry| entry.keyword().to_owned()),
    )
    .is_some_and(|index| {
        *value = PortDiscipline::ALL[index];
        true
    })
}

fn enum_field(
    ui: &mut Ui,
    salt: &str,
    label: &str,
    selected: &str,
    options: &[String],
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        select(ui, salt, label, selected, options, ui.available_width())
    })
    .inner
}

fn paint_preview(ui: &mut Ui, draft: &PinPortDialogState, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Port preview: {}, {}, {}",
                draft.name,
                draft.direction_type.label(),
                draft.discipline.keyword()
            ),
        )
    });
    ui.painter().rect(
        rect,
        8.0,
        t.color.canvas_bg,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    let center = rect.center();
    let grid_color = t.color.canvas_grid.gamma_multiply(0.5);
    let mut y = rect.top() + 6.0;
    while y < rect.bottom() {
        let mut x = rect.left() + 6.0;
        while x < rect.right() {
            ui.painter()
                .circle_filled(egui::pos2(x, y), 0.75, grid_color);
            x += 12.0;
        }
        y += 12.0;
    }
    let stroke = Stroke::new(1.6, t.color.accent);
    let tip = egui::pos2(center.x - 58.0, center.y);
    ui.painter().line_segment(
        [egui::pos2(rect.left() + 18.0, tip.y), tip],
        Stroke::new(1.6, t.color.wire),
    );
    ui.painter().line_segment(
        [
            egui::pos2(tip.x, rect.top() + 20.0),
            egui::pos2(tip.x, rect.bottom() - 20.0),
        ],
        Stroke::new(1.0, t.color.wire.gamma_multiply(0.55)),
    );
    let outline = [
        tip,
        egui::pos2(center.x - 46.0, center.y - 16.0),
        egui::pos2(center.x + 44.0, center.y - 16.0),
        egui::pos2(center.x + 44.0, center.y + 16.0),
        egui::pos2(center.x - 46.0, center.y + 16.0),
    ];
    for index in 0..outline.len() {
        ui.painter().line_segment(
            [outline[index], outline[(index + 1) % outline.len()]],
            stroke,
        );
    }
    ui.painter().circle_filled(tip, 2.4, t.color.accent);
    ui.painter()
        .circle_stroke(tip, 8.0, Stroke::new(1.5, t.color.accent));
    let direction_segments: &[PreviewMarkerSegment] = match draft.direction_type {
        PortDirectionType::InputLogic | PortDirectionType::InputAnalog => &[
            ((-8.0, 0.0), (9.0, 0.0)),
            ((9.0, 0.0), (3.0, -5.0)),
            ((9.0, 0.0), (3.0, 5.0)),
        ],
        PortDirectionType::OutputAnalog => &[
            ((9.0, 0.0), (-8.0, 0.0)),
            ((-8.0, 0.0), (-2.0, -5.0)),
            ((-8.0, 0.0), (-2.0, 5.0)),
        ],
        PortDirectionType::InOutPower => &[
            ((-8.0, 0.0), (9.0, 0.0)),
            ((9.0, 0.0), (3.0, -5.0)),
            ((9.0, 0.0), (3.0, 5.0)),
            ((-8.0, 0.0), (-2.0, -5.0)),
            ((-8.0, 0.0), (-2.0, 5.0)),
        ],
    };
    for &((x1, y1), (x2, y2)) in direction_segments {
        ui.painter().line_segment(
            [center + Vec2::new(x1, y1), center + Vec2::new(x2, y2)],
            stroke,
        );
    }
    let name = if draft.name.trim().is_empty() {
        "port name"
    } else {
        draft.name.trim()
    };
    ui.painter().text(
        center + Vec2::new(0.0, -29.0),
        egui::Align2::CENTER_BOTTOM,
        name,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        t.color.text,
    );
    ui.painter().text(
        center + Vec2::new(0.0, 28.0),
        egui::Align2::CENTER_TOP,
        format!(
            "{} \u{00b7} {}",
            draft.direction_type.label(),
            draft.discipline.keyword()
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn status_card(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(7.0)
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(
                egui::RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                )
                .wrap(),
            );
        });
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

    fn open_dialog(app: &mut RSpiceApp) {
        let name = app.state.schematic.suggested_port_name("BIAS_EN");
        app.state.dialogs.pin_port.open(
            name,
            app.state.design_execution_epoch,
            app.state.active_schematic_epoch,
            app.state.schematic.topology_version(),
            app.state.workspace.active_view.display_path(),
        );
    }

    #[test]
    fn mockup_contract_is_exact_and_complete() {
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} INTERFACE CONTRACT");
        assert_eq!(TITLE, "Place pin or port");
        assert_eq!(PRIMARY, "Arm pin tool");
        assert_eq!(PortDirectionType::ALL[0].label(), "input \u{00b7} logic");
        assert_eq!(PortDirectionType::ALL[3].label(), "inout \u{00b7} power");
        assert_eq!(PortDiscipline::ALL[2].keyword(), "wreal");
        assert_eq!(WORKFLOW_HEIGHT, 414.0);
        assert_eq!(PANE_PADDING, 14);
        assert!(!workflow_uses_columns(760.0));
        assert!(workflow_uses_columns(761.0));
        assert_eq!(
            workflow_right_track(980.0),
            (0.72 / 1.72, COMPACT_RIGHT_MIN_WIDTH)
        );
        assert_eq!(workflow_right_track(981.0), (0.8 / 2.35, RIGHT_MIN_WIDTH));
        assert!(section_head_wraps(
            242.0,
            "Placement / transform parameters",
            "legal preview"
        ));
        assert!(!section_head_wraps(
            430.0,
            "PIN \u{00b7} live schematic preview",
            "100 mil grid"
        ));
    }

    #[test]
    fn valid_draft_freezes_complete_pending_contract_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let name = app.state.schematic.suggested_port_name("BIAS_EN");
        app.state.dialogs.pin_port.open(
            name,
            app.state.design_execution_epoch,
            app.state.active_schematic_epoch,
            app.state.schematic.topology_version(),
            app.state.workspace.active_view.display_path(),
        );
        let before = app.state.schematic.components.clone();
        let DraftValidation::Valid(pending) = validate_draft(&app.state) else {
            panic!("default mockup draft must be valid");
        };
        assert_eq!(pending.name, "BIAS_EN");
        assert_eq!(
            pending.contract.signal_type,
            crate::state::PortSignalType::Logic
        );
        assert_eq!(pending.contract.discipline, PortDiscipline::Electrical);
        assert_eq!(app.state.schematic.components, before);
    }

    #[test]
    fn duplicate_and_stale_drafts_fail_closed() {
        let mut app = RSpiceApp::test_instance();
        let id = app.state.schematic.add_component(
            crate::state::ComponentType::Port,
            crate::state::Point::origin(),
        );
        app.state
            .schematic
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .value = "BIAS_EN".to_owned();
        app.state.dialogs.pin_port.open(
            "bias_en".to_owned(),
            app.state.design_execution_epoch,
            app.state.active_schematic_epoch,
            app.state.schematic.topology_version(),
            app.state.workspace.active_view.display_path(),
        );
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        app.state.dialogs.pin_port.name = "UNIQUE".to_owned();
        app.state.schematic.bump_topology_version();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
    }

    #[test]
    fn rendered_primary_arms_exactly_one_pending_contract_without_mutation() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_dialog(&mut app);
        let topology = app.state.schematic.topology_version();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_pin_port_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_pin_port_dialog(ctx)
        });

        assert!(!app.state.dialogs.pin_port.open);
        assert_eq!(
            app.state.schematic.tool,
            Tool::Place(crate::state::ComponentType::Port)
        );
        let pending = app
            .state
            .schematic
            .pending_port
            .as_ref()
            .expect("validated pending interface port");
        assert_eq!(pending.name, "BIAS_EN");
        assert_eq!(pending.expected_topology_version, topology);
        assert_eq!(pending.contract.direction, crate::state::PortDirection::In);
        assert_eq!(
            pending.contract.signal_type,
            crate::state::PortSignalType::Logic
        );
        assert_eq!(pending.contract.netlist_order, Some(1));
        assert!(app.state.schematic.components.is_empty());
        assert!(!app.state.schematic.is_dirty);
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn rendered_read_only_and_stale_dialogs_never_arm_or_publish() {
        for stale in [false, true] {
            let ctx = Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            open_dialog(&mut app);
            if stale {
                app.state.schematic.bump_topology_version();
            } else {
                app.state.schematic.read_only = true;
            }

            let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
                app.render_pin_port_dialog(ctx)
            });
            let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
                app.render_pin_port_dialog(ctx)
            });

            assert!(app.state.dialogs.pin_port.open);
            assert_eq!(app.state.schematic.tool, Tool::Select);
            assert!(app.state.schematic.pending_port.is_none());
            assert!(app.state.schematic.components.is_empty());
        }
    }

    #[test]
    fn rendered_escape_requires_explicit_second_discard_after_edit() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_dialog(&mut app);
        app.state.dialogs.pin_port.mark_edited();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_pin_port_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_pin_port_dialog(ctx)
        });
        assert!(app.state.dialogs.pin_port.open);
        assert!(app.state.dialogs.pin_port.discard_confirm);

        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_pin_port_dialog(ctx)
        });
        assert!(!app.state.dialogs.pin_port.open);
        assert!(app.state.schematic.pending_port.is_none());
    }

    #[test]
    fn rendered_contract_strings_contain_no_mojibake_markers() {
        for value in [
            EYEBROW,
            TITLE,
            PRIMARY,
            DESCRIPTION,
            PREVIEW_TITLE,
            DIRECTION_LABEL,
            DISCIPLINE_LABEL,
            DISCARD_TITLE,
            DISCARD_DETAIL,
        ] {
            for forbidden in ['\u{00c2}', '\u{00e2}', '\u{fffd}'] {
                assert!(
                    !value.contains(forbidden),
                    "mojibake in rendered contract string: {value:?}"
                );
            }
        }
    }
}
