use egui::{Align, Context, Frame, Layout, Margin, Rect, Response, Sense, Stroke, Ui, Vec2, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, IconButton,
};

use super::controller::{
    commit_create_model_bound_symbol, target_syntax_error, validate_create_symbol_draft,
};
use super::state::*;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::review_primitives::{configure_field_validation, field_label};

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_create_model_bound_symbol_dialog(
        &mut self,
        ctx: &Context,
    ) {
        if !self.state.dialogs.create_model_bound_symbol.open {
            return;
        }

        let validation = validate_create_symbol_draft(&self.state);
        let discard_confirm = self.state.dialogs.create_model_bound_symbol.discard_confirm;
        let retain_dirty_cancel =
            self.state.dialogs.create_model_bound_symbol.dirty && !discard_confirm;
        let pin_count = self.state.dialogs.create_model_bound_symbol.pins.len();
        let footer_hint = format!(
            "{} pin{} \u{00b7} atomic library revision",
            pin_count,
            if pin_count == 1 { "" } else { "s" }
        );
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .initial_height(INITIAL_HEIGHT)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .hint(&footer_hint)
            .primary_enabled(validation.is_ok())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if retain_dirty_cancel {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Discard symbol definition?",
                "The library is unchanged. Discarding closes this draft and its reviewed pin contract.",
            );
        } else if let Err(error) = &validation {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Symbol revision cannot be created",
                error,
            );
        } else {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Complete,
                "Symbol definition ready",
                "The source, ordered pin contract, generated views, and atomic target are valid.",
            );
        }

        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            Some(create_symbol_body(
                ui,
                &mut self.state.dialogs.create_model_bound_symbol,
            ))
        });
        match choice {
            DialogChoice::Primary => {
                if let Err(error) = commit_create_model_bound_symbol(&mut self.state) {
                    self.state
                        .dialogs
                        .create_model_bound_symbol
                        .validation_error = Some(error.clone());
                    self.state.push_user_message(ConsoleMessage::warning(error));
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.create_model_bound_symbol.attempt_close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn create_symbol_body(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let initial = split_body(ui, draft);
    generated_views_row(ui, draft);
    initial
}

fn split_body(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    let width = ui.available_width();
    let response = Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            if width <= SPLIT_BREAKPOINT {
                let initial = identity_section(ui, draft);
                ui.separator();
                pin_section(ui, draft);
                initial
            } else {
                let mut initial = None;
                ui.columns(2, |columns| {
                    columns[0].spacing_mut().item_spacing = Vec2::ZERO;
                    columns[1].spacing_mut().item_spacing = Vec2::ZERO;
                    initial = Some(identity_section(&mut columns[0], draft));
                    pin_section(&mut columns[1], draft);
                });
                let rect = ui.min_rect();
                ui.painter().vline(
                    rect.center().x,
                    rect.y_range(),
                    Stroke::new(1.0, t.color.border_strong),
                );
                initial.expect("identity section always renders")
            }
        });
    response.inner
}

fn identity_section(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) -> egui::Id {
    let response = section_title(ui, "Identity and source");
    Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = FIELD_GAP;
        field_label(ui, "Library / cell", |ui| {
            let t = Tokens::get(ui.ctx());
            let response = ui.add_sized(
                vec2(ui.available_width(), t.metrics.ctl_h),
                egui::TextEdit::singleline(&mut draft.target)
                    .font(egui::TextStyle::Monospace)
                    .margin(Margin::symmetric(8, 4)),
            );
            configure_field_validation(
                ui,
                &response,
                "Library / cell",
                target_syntax_error(&draft.target).as_deref(),
                "Writable destination identity in library / cell form",
            );
            if response.changed() {
                draft.mark_edited();
            }
            response
        });
        source_contract_combo(ui, draft);
        template_combo(ui, draft);
    });
    response.id
}

fn source_contract_combo(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) {
    field_label(ui, "Source contract", |ui| {
        let selected = draft.source_label(draft.source_mode);
        let response = egui::ComboBox::from_id_salt("create-model-bound-symbol-source")
            .selected_text(selected)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                let mut picked = None;
                for mode in CreateSymbolSourceMode::ALL {
                    let available = draft.source_available(mode);
                    let label = draft.source_label(mode);
                    if ui
                        .add_enabled(
                            available,
                            egui::Button::selectable(draft.source_mode == mode, label),
                        )
                        .clicked()
                    {
                        picked = Some(mode);
                    }
                }
                if let Some(mode) = picked {
                    draft.select_source(mode);
                }
            })
            .response;
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label("Source contract");
            node.set_description(
                "Explicit model, schematic-interface, or blank terminal contract; source changes replace the pin draft",
            );
        });
    });
}

fn template_combo(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) {
    field_label(ui, "Template", |ui| {
        let before = draft.template;
        let response = egui::ComboBox::from_id_salt("create-model-bound-symbol-template")
            .selected_text(draft.template.label())
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for template in CreateSymbolTemplate::ALL {
                    ui.selectable_value(&mut draft.template, template, template.label());
                }
            })
            .response;
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label("Graphical template");
            node.set_description("Initial geometry for the new versioned symbol view");
        });
        if before != draft.template {
            draft.mark_edited();
        }
    });
}

fn pin_section(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) {
    section_title(ui, "Pin order and electrical types");
    pin_header(ui);
    if draft.pins.is_empty() {
        empty_pin_row(ui);
    } else {
        let mut action = None;
        for index in 0..draft.pins.len() {
            if let Some(next) = pin_row(ui, draft, index) {
                action = Some(next);
            }
        }
        if let Some(action) = action {
            apply_pin_action(draft, action);
        }
    }
    pin_toolbar(ui, draft);
}

fn section_title(ui: &mut Ui, title: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), SECTION_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    response
}

fn pin_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), PIN_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    paint_pin_columns(
        ui,
        rect,
        ["ORDER", "NAME", "TYPE", "SIDE"],
        t.color.text_dim,
        true,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn empty_pin_row(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(vec2(ui.available_width(), PIN_ROW_HEIGHT), Sense::hover());
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "Add an explicit ordered pin contract",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

#[derive(Debug, Clone, Copy)]
enum PinAction {
    Select(usize),
    MoveUp(usize),
    MoveDown(usize),
}

fn pin_row(
    ui: &mut Ui,
    draft: &mut CreateModelBoundSymbolDialogState,
    index: usize,
) -> Option<PinAction> {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, PIN_ROW_HEIGHT), Sense::click());
    if draft.selected_pin == Some(index) {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        ui.painter().vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let [order_rect, name_rect, type_rect, side_rect] = pin_column_rects(rect);
    let mut action = response.clicked().then_some(PinAction::Select(index));
    let mut order_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(order_rect.shrink2(vec2(3.0, 2.0)))
            .layout(Layout::left_to_right(Align::Center)),
    );
    order_ui.spacing_mut().item_spacing.x = 1.0;
    order_ui.label(
        egui::RichText::new((index + 1).to_string())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text),
    );
    if IconButton::new(Icon::ChevronUp)
        .enabled(index > 0)
        .side(24.0)
        .tooltip("Move pin earlier in netlist order")
        .show(&mut order_ui)
        .clicked()
    {
        action = Some(PinAction::MoveUp(index));
    }
    if IconButton::new(Icon::ChevronDown)
        .enabled(index + 1 < draft.pins.len())
        .side(24.0)
        .tooltip("Move pin later in netlist order")
        .show(&mut order_ui)
        .clicked()
    {
        action = Some(PinAction::MoveDown(index));
    }

    let name_response = ui.put(
        name_rect.shrink2(vec2(3.0, 3.0)),
        egui::TextEdit::singleline(&mut draft.pins[index].name)
            .font(egui::TextStyle::Monospace)
            .margin(Margin::symmetric(5, 2)),
    );
    if name_response.changed() {
        draft.selected_pin = Some(index);
        draft.pin_contract_reviewed = true;
        draft.mark_edited();
    }

    let before_type = draft.pins[index].electrical_type;
    let type_cell = type_rect.shrink2(vec2(3.0, 2.0));
    let type_response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(type_cell)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                egui::ComboBox::from_id_salt(("create-symbol-pin-type", index))
                    .selected_text(before_type.label())
                    .width(type_cell.width())
                    .show_ui(ui, |ui| {
                        for kind in CreateSymbolPinType::ALL {
                            ui.selectable_value(
                                &mut draft.pins[index].electrical_type,
                                kind,
                                kind.label(),
                            );
                        }
                    })
                    .response
            },
        )
        .inner;
    if type_response.changed() || before_type != draft.pins[index].electrical_type {
        draft.pin_contract_reviewed = true;
        draft.mark_edited();
    }

    let before_side = draft.pins[index].side;
    let side_cell = side_rect.shrink2(vec2(3.0, 2.0));
    let side_response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(side_cell)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                egui::ComboBox::from_id_salt(("create-symbol-pin-side", index))
                    .selected_text(before_side.label())
                    .width(side_cell.width())
                    .show_ui(ui, |ui| {
                        for side in CreateSymbolPinSide::ALL {
                            ui.selectable_value(&mut draft.pins[index].side, side, side.label());
                        }
                    })
                    .response
            },
        )
        .inner;
    if side_response.changed() || before_side != draft.pins[index].side {
        draft.pin_contract_reviewed = true;
        draft.mark_edited();
    }
    action
}

fn paint_pin_columns(ui: &Ui, rect: Rect, values: [&str; 4], color: egui::Color32, header: bool) {
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    for (cell, value) in pin_column_rects(rect).into_iter().zip(values) {
        ui.painter().text(
            egui::pos2(cell.left() + 6.0, cell.center().y),
            egui::Align2::LEFT_CENTER,
            value,
            font.clone(),
            color,
        );
    }
}

fn pin_column_rects(rect: Rect) -> [Rect; 4] {
    // The order cell owns an index plus two 24 px icon buttons. Ratios made
    // that cell narrower than its contents in the mockup-sized 760 px dialog,
    // so the contract is expressed in stable minimum tracks instead.
    let order_width = 72.0_f32.min(rect.width() * 0.24);
    let name_width = (rect.width() * 0.23).clamp(72.0, 96.0);
    let side_width = 68.0_f32.min(rect.width() * 0.21);
    let widths = [
        order_width,
        name_width,
        (rect.width() - order_width - name_width - side_width).max(84.0),
        side_width,
    ];
    let mut left = rect.left();
    std::array::from_fn(|index| {
        let right = if index == 3 {
            rect.right()
        } else {
            (left + widths[index]).min(rect.right())
        };
        let cell = Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        );
        left = right;
        cell
    })
}

fn apply_pin_action(draft: &mut CreateModelBoundSymbolDialogState, action: PinAction) {
    match action {
        PinAction::Select(index) => draft.selected_pin = Some(index),
        PinAction::MoveUp(index) if index > 0 => {
            draft.pins.swap(index, index - 1);
            draft.selected_pin = Some(index - 1);
            draft.pin_contract_reviewed = true;
            draft.mark_edited();
        }
        PinAction::MoveDown(index) if index + 1 < draft.pins.len() => {
            draft.pins.swap(index, index + 1);
            draft.selected_pin = Some(index + 1);
            draft.pin_contract_reviewed = true;
            draft.mark_edited();
        }
        PinAction::MoveUp(_) | PinAction::MoveDown(_) => {}
    }
}

fn pin_toolbar(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), t.metrics.ctl_h + 12.0),
        Sense::hover(),
    );
    let mut toolbar = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(8.0, 6.0)))
            .layout(Layout::right_to_left(Align::Center)),
    );
    toolbar.spacing_mut().item_spacing.x = 4.0;
    let can_remove = draft
        .selected_pin
        .is_some_and(|index| index < draft.pins.len());
    if toolbar
        .add_enabled_ui(can_remove, |ui| Button::new("Remove pin").show(ui))
        .inner
        .clicked()
        && let Some(index) = draft.selected_pin
    {
        draft.pins.remove(index);
        draft.selected_pin = index
            .checked_sub(1)
            .or_else(|| (!draft.pins.is_empty()).then_some(0));
        draft.pin_contract_reviewed = true;
        draft.mark_edited();
    }
    if Button::new("Add pin").show(&mut toolbar).clicked() {
        let next = draft.pins.len() + 1;
        draft.pins.push(CreateSymbolPinDraft::new(
            format!("PIN{next}"),
            CreateSymbolPinType::AnalogBidirectional,
            CreateSymbolPinSide::Left,
        ));
        draft.selected_pin = Some(next - 1);
        draft.pin_contract_reviewed = true;
        draft.mark_edited();
    }
    let reviewed_before = draft.pin_contract_reviewed;
    toolbar.checkbox(&mut draft.pin_contract_reviewed, "Pin contract reviewed");
    if reviewed_before != draft.pin_contract_reviewed {
        draft.mark_edited();
    }
}

fn generated_views_row(ui: &mut Ui, draft: &mut CreateModelBoundSymbolDialogState) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    // The mockup's workflow setting row is a 54 px border-box. Its two-line
    // explanatory copy owns the same compact vertical rhythm without growing
    // the body or pushing the footer off a 477 px viewport.
    let row_h = 54.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, row_h), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    // Reserve the action track first. The mockup copy wraps inside the label
    // track instead of painting through the three checkboxes.
    let action_width = 396.0_f32.min((rect.width() - 176.0).max(0.0));
    let label_width = (rect.width() - action_width - 12.0).max(164.0);
    let label_rect = Rect::from_min_max(
        egui::pos2(rect.left(), rect.top()),
        egui::pos2(rect.left() + label_width, rect.bottom()),
    );
    let mut labels = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(label_rect.shrink2(vec2(0.0, 7.0)))
            .layout(Layout::top_down(Align::Min)),
    );
    labels.spacing_mut().item_spacing.y = 3.0;
    labels.label(
        egui::RichText::new("Generated views")
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    labels.add_sized(
        vec2(label_rect.width(), 27.0),
        egui::Label::new(
            egui::RichText::new(
                "The symbol and typed parameter form are independent versioned views.",
            )
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
        )
        .wrap(),
    );
    let action_rect = Rect::from_min_max(
        egui::pos2(label_rect.right() + 12.0, rect.top()),
        rect.right_bottom(),
    );
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(action_rect.shrink2(vec2(0.0, 7.0)))
            .layout(Layout::left_to_right(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = 10.0;
    let before = (
        draft.symbol,
        draft.parameter_form,
        draft.simulation_test_fixture,
    );
    let fixture_available = draft.source_mode != CreateSymbolSourceMode::BlankExplicitContract;
    actions.checkbox(&mut draft.symbol, "symbol");
    actions.checkbox(&mut draft.parameter_form, "parameter form");
    actions
        .add_enabled_ui(fixture_available, |ui| {
            ui.checkbox(
                &mut draft.simulation_test_fixture,
                "simulation test fixture",
            )
        })
        .inner
        .on_disabled_hover_text(
            "A blank explicit contract has no executable DUT implementation to fixture.",
        );
    if before
        != (
            draft.symbol,
            draft.parameter_form,
            draft.simulation_test_fixture,
        )
    {
        draft.mark_edited();
    }
}
