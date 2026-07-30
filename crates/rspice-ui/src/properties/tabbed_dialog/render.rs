//! The schematic instance editor on the modal primitive.
//!
//! Registry categories become sections in the independently scrolling
//! Parameters pane, while the evidence pane is rebuilt from live document
//! state on every frame. Each property is a stable-height field block in the
//! mockup's two-column grid, with validation replacing its reserved hint
//! track instead of reflowing neighboring controls. PWL sources gain a
//! structured point editor, every
//! model-bound family gains model browsing, and source families gain a live
//! preview. The shell follows the mockup's Cancel, Apply, and OK lifecycle.

use egui::{Align, Id, Layout, Margin, Sense, Stroke, Ui, pos2, vec2};

use crate::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertyValue,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};

use super::editors::render_value_editor;
use super::state::{ComponentEditorContext, TabbedDialogResult, TabbedPropertyDialogState};

const DIALOG_SIZE: DialogSize = DialogSize::ComponentEditor;
const EYEBROW: &str = "EDIT · TYPED PARAMETERS";
const DESCRIPTION: &str = "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata.";

/// Render the dedicated schematic instance editor.
///
/// Every component family is driven by its registered typed property sheet;
/// the shell and evidence pane follow the latest component-editor mockup.
pub fn render_tabbed_property_dialog(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    registry: &PropertyRegistry,
    model_library_manager: &crate::state::ModelLibraryManager,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
    commit_policy: crate::state::PropertyCommitPolicy,
) -> TabbedDialogResult {
    let mut result = TabbedDialogResult::None;
    if !state.open {
        return result;
    }
    let Some(component_type) = state.component_type else {
        state.close();
        return TabbedDialogResult::Cancelled;
    };
    let Some(sheet) = registry.get(component_type) else {
        state.close();
        return TabbedDialogResult::Cancelled;
    };

    state.sync_pwl_validation_error();
    let session_error = state.session_error.clone();
    let dirty = state.has_modifications();
    let footer_hint = session_error
        .clone()
        .or_else(|| dirty.then(|| "Unapplied changes".to_owned()));

    let mut dialog = Dialog::new(EYEBROW, "Edit instance properties", "OK")
        .description(DESCRIPTION)
        .size(DIALOG_SIZE)
        .fixed_height(680.0)
        .without_header()
        .flush_body()
        .manual_body_scroll()
        .ghost("Cancel")
        .secondary("Apply")
        .secondary_enabled(dirty && state.can_apply(commit_policy) && session_error.is_none())
        .primary_enabled(session_error.is_none() && (!dirty || state.can_apply(commit_policy)))
        .interaction_enabled(!state.model_browser.open)
        .initial_focus(DialogInitialFocus::BodyControl);
    if let Some(hint) = footer_hint.as_deref() {
        dialog = dialog.hint(hint);
    }

    let mut side_action = TabbedDialogResult::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        component_identity_header(ui, state, context);
        let body_height = ui.available_height().max(1.0);
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            Tokens::get(ui.ctx()).color.bg_panel,
        );
        let wide = ctx.content_rect().width() > 760.0;
        let mut first_parameter = None;
        if wide {
            let gap = 1.0;
            let left_width = ((ui.available_width() - gap) * (1.1 / 2.1)).max(300.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(left_width, body_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        first_parameter = parameters_pane(
                            ui,
                            state,
                            sheet,
                            component_type,
                            quantity_policy,
                            number_locale,
                        );
                    },
                );
                let (divider, _) = ui.allocate_exact_size(vec2(gap, body_height), Sense::hover());
                ui.painter()
                    .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), body_height),
                    Layout::top_down(Align::Min),
                    |ui| evidence_pane(ui, state, context, component_type, &mut side_action),
                );
            });
        } else {
            let gap = 1.0;
            let parameters_height = ((body_height - gap) * 0.56).max(1.0);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), parameters_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        first_parameter = parameters_pane(
                            ui,
                            state,
                            sheet,
                            component_type,
                            quantity_policy,
                            number_locale,
                        );
                    },
                );
                let (divider, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), gap), Sense::hover());
                ui.painter()
                    .rect_filled(divider, 0.0, Tokens::get(ui.ctx()).color.border);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), ui.available_height()),
                    Layout::top_down(Align::Min),
                    |ui| evidence_pane(ui, state, context, component_type, &mut side_action),
                );
            });
        }
        first_parameter
    });

    let dirty_after_render = state.has_modifications();
    if side_action != TabbedDialogResult::None {
        result = side_action;
    } else {
        match choice {
            DialogChoice::Primary => {
                if !dirty_after_render {
                    state.close();
                    result = TabbedDialogResult::Cancelled;
                } else if state.prepare_commit(sheet, commit_policy) {
                    result = if state.validation_errors.is_empty() {
                        TabbedDialogResult::AppliedAndClose
                    } else {
                        TabbedDialogResult::Applied
                    };
                }
            }
            DialogChoice::Secondary => {
                if dirty_after_render && state.prepare_commit(sheet, commit_policy) {
                    result = TabbedDialogResult::Applied;
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                state.close();
                result = TabbedDialogResult::Cancelled;
            }
            DialogChoice::None => {}
        }
    }

    render_model_browser(ctx, state, model_library_manager);
    result
}

fn component_identity_header(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
    context: &ComponentEditorContext,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let frame = egui::Frame::NONE
        .fill(c.bg_panel_2)
        .inner_margin(Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.set_width(width - 32.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;
                let (glyph, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
                ui.painter().rect_filled(glyph, 5.0, c.bg_panel);
                ui.painter().rect_stroke(
                    glyph,
                    5.0,
                    Stroke::new(1.0, c.border),
                    egui::StrokeKind::Inside,
                );
                ui.painter().text(
                    glyph.center(),
                    egui::Align2::CENTER_CENTER,
                    &context.glyph,
                    theme::mono(tokens::FS_2, FontWeight::SemiBold),
                    c.accent,
                );

                let status_width = (ui.available_width() * 0.28).clamp(110.0, 190.0);
                ui.allocate_ui_with_layout(
                    vec2((ui.available_width() - status_width).max(180.0), 34.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 7.0;
                            ui.label(
                                egui::RichText::new("Instance")
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            );
                            let mut name = state
                                .get_value("name")
                                .map(PropertyValue::display_string)
                                .or_else(|| state.component_name.clone())
                                .unwrap_or_default();
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut name)
                                    .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                                    .desired_width(88.0),
                            );
                            if response.changed() {
                                state.set_value("name", PropertyValue::String(name));
                            }
                            ui.label(
                                egui::RichText::new(&context.library_cell)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            ui.label(
                                egui::RichText::new(&context.subtitle)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            );
                            ui.label(
                                egui::RichText::new("·")
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&context.instance_path)
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(c.text_dim),
                                )
                                .truncate(),
                            );
                        });
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(status_width, 34.0),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&context.family)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_dim),
                            )
                            .truncate(),
                        );
                    },
                );
            });
        });
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        Stroke::new(1.0, c.border),
    );
}

fn parameters_pane(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
    sheet: &crate::state::property_types::PropertySheet,
    component_type: crate::state::ComponentType,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<Id> {
    let mut first_control = None;
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("component-editor-parameters")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    first_control = parameters_contents(
                        ui,
                        state,
                        sheet,
                        component_type,
                        quantity_policy,
                        number_locale,
                    );
                });
        });
    first_control
}

fn parameters_contents(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
    sheet: &crate::state::property_types::PropertySheet,
    component_type: crate::state::ComponentType,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<Id> {
    section_band(ui, "Parameters", "typed · unit-checked");
    let properties = sheet
        .iter()
        .filter(|definition| definition.name != "name")
        .filter(|definition| !(component_type.is_pwl_source() && definition.name == "pwl_data"))
        .filter(|definition| match definition.display_mode {
            DisplayMode::Hidden => false,
            DisplayMode::Advanced if !state.show_advanced => false,
            _ => true,
        })
        .filter(|definition| property_is_visible(definition, state))
        .cloned()
        .collect::<Vec<_>>();

    let mut first_control = None;
    egui::Frame::NONE
        .inner_margin(Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            for pair in properties.chunks(2) {
                let gap = 14.0;
                let field_width = ((ui.available_width() - gap) * 0.5).max(120.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    for definition in pair {
                        let control = ui
                            .push_id(("component-editor-property", &definition.name), |ui| {
                                ui.allocate_ui_with_layout(
                                    vec2(field_width, 58.0),
                                    Layout::top_down(Align::Min),
                                    |ui| {
                                        render_property_field(
                                            ui,
                                            definition,
                                            state,
                                            quantity_policy,
                                            number_locale,
                                        )
                                    },
                                )
                                .inner
                            })
                            .inner;
                        first_control = first_control.or(control);
                    }
                    if pair.len() == 1 {
                        ui.allocate_space(vec2(field_width, 58.0));
                    }
                });
            }

            if component_type.is_pwl_source() {
                ui.add_space(4.0);
                property_group_heading(ui, "Piecewise-linear waveform");
                let pwl_result = crate::properties::pwl_editor::render_pwl_editor(
                    ui,
                    &mut state.pwl_editor,
                    quantity_policy,
                    number_locale,
                );
                if pwl_result == crate::properties::pwl_editor::PwlEditorResult::Modified {
                    state.pwl_editor.is_modified = true;
                    state.set_value(
                        "pwl_data",
                        PropertyValue::String(state.pwl_editor.to_string()),
                    );
                }
                state.sync_pwl_validation_error();
            }

            let has_advanced = sheet
                .iter()
                .any(|definition| definition.display_mode == DisplayMode::Advanced);
            if has_advanced {
                ui.add_space(4.0);
                crate::ui::widgets::check_row(
                    ui,
                    "Show advanced properties",
                    &mut state.show_advanced,
                );
            }

            let t = Tokens::get(ui.ctx());
            let message = state
                .commit_error
                .as_deref()
                .or(state.global_error.as_deref())
                .unwrap_or(" ");
            ui.add_sized(
                [ui.available_width(), 16.0],
                egui::Label::new(
                    egui::RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(
                            if state.commit_error.is_some() || state.global_error.is_some() {
                                t.color.err
                            } else {
                                egui::Color32::TRANSPARENT
                            },
                        ),
                ),
            );
        });
    first_control
}

fn property_is_visible(definition: &PropertyDefinition, state: &TabbedPropertyDialogState) -> bool {
    use crate::state::property_types::VisibilityCondition;
    match &definition.visibility_condition {
        VisibilityCondition::Always => true,
        VisibilityCondition::WhenNonDefault => state
            .get_value(&definition.name)
            .is_some_and(|value| value != &definition.default_value),
        VisibilityCondition::WhenPropertyEquals { property, value } => state
            .get_value(property)
            .is_some_and(|current| current.display_string().eq_ignore_ascii_case(value)),
        VisibilityCondition::WhenPropertySet(property) => state
            .get_value(property)
            .is_some_and(|current| !current.display_string().trim().is_empty()),
    }
}

fn evidence_pane(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    component_type: crate::state::ComponentType,
    action: &mut TabbedDialogResult,
) {
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("component-editor-evidence")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    evidence_contents(ui, state, context, component_type, action)
                });
        });
}

fn evidence_contents(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    component_type: crate::state::ComponentType,
    action: &mut TabbedDialogResult,
) {
    model_binding_card(ui, state, context, action);
    operating_point_card(ui, context, action);
    if supports_source_preview(component_type) {
        source_preview_card(ui, state, component_type);
    }
    terminals_card(ui, context);
}

fn supports_source_preview(kind: crate::state::ComponentType) -> bool {
    use crate::state::ComponentType;
    matches!(
        kind,
        ComponentType::VoltageSource
            | ComponentType::CurrentSource
            | ComponentType::VoltageSourceAc
            | ComponentType::CurrentSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::CurrentSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::CurrentSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::CurrentSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::CurrentSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::CurrentSourceNoise
    )
}

fn section_band(ui: &mut Ui, title: &str, status: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin {
            left: 16,
            right: 16,
            top: 8,
            bottom: 4,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(title.to_uppercase())
                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(status.to_uppercase())
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                });
            });
        });
    let y = ui.cursor().top();
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, t.color.border));
}

fn property_group_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    ui.add_space(3.0);
}

fn section_block(ui: &mut Ui, title: &str, status: &str, body: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    section_band(ui, title, status);
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin {
            left: 16,
            right: 16,
            top: 4,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            body(ui);
        });
    let y = ui.cursor().top();
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, t.color.border));
}

fn evidence_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.set_min_height(19.0);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                )
                .truncate(),
            );
        });
    });
}

fn model_binding_card(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    context: &ComponentEditorContext,
    action: &mut TabbedDialogResult,
) {
    let draft_model = state
        .get_value("model")
        .map(PropertyValue::display_string)
        .filter(|model| !model.trim().is_empty());
    let draft_library = state
        .get_value("model_library")
        .map(PropertyValue::display_string)
        .filter(|library| !library.trim().is_empty());
    let pending_model = context.model.as_ref().and_then(|model| {
        let identity_changed = draft_model
            .as_deref()
            .is_some_and(|draft| !draft.eq_ignore_ascii_case(&model.name))
            || draft_library.as_deref().is_some_and(|library| {
                model
                    .library
                    .as_deref()
                    .is_none_or(|resolved| !library.eq_ignore_ascii_case(resolved))
            });
        identity_changed.then(|| draft_model.as_deref().unwrap_or(&model.name))
    });
    let status = if pending_model.is_some() {
        "pending"
    } else {
        context
            .model
            .as_ref()
            .map(|model| {
                if model.status.contains("resolved") {
                    "qualified"
                } else if model.status.contains("inline") || model.status.contains("exact") {
                    "exact"
                } else {
                    "unverified"
                }
            })
            .unwrap_or("not bound")
    };

    section_block(ui, "Model binding", status, |ui| {
        if let Some(model) = &context.model {
            let t = Tokens::get(ui.ctx());
            if let Some(pending_model) = pending_model {
                evidence_row(ui, "Model", pending_model);
                evidence_row(
                    ui,
                    "Source",
                    draft_library.as_deref().unwrap_or("Pending validation"),
                );
                evidence_row(ui, "Section", &model.section);
                ui.label(
                    egui::RichText::new("Apply to resolve the new model binding.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.warn),
                );
            } else {
                evidence_row(ui, "Model", &model.name);
                evidence_row(ui, "Source", &model.source);
                evidence_row(ui, "Section", &model.section);
                ui.label(
                    egui::RichText::new(&model.status)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            }
            if model.can_open || model.can_qualify {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if crate::ui::widgets::Button::new("Open model detail…")
                        .enabled(model.can_open && pending_model.is_none())
                        .show(ui)
                        .clicked()
                    {
                        *action = TabbedDialogResult::OpenModel;
                    }
                    if crate::ui::widgets::Button::new("Qualification…")
                        .enabled(model.can_qualify && pending_model.is_none())
                        .show(ui)
                        .clicked()
                    {
                        *action = TabbedDialogResult::OpenQualification;
                    }
                });
            }
        } else if let Some(model) = draft_model.as_deref() {
            evidence_row(ui, "Model", model);
            evidence_row(ui, "Source", "No catalog source resolved");
            evidence_row(ui, "Section", "default");
        } else {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("This component has no model binding.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        }
    });
}

fn operating_point_card(
    ui: &mut Ui,
    context: &ComponentEditorContext,
    action: &mut TabbedDialogResult,
) {
    let status = context
        .operating_point
        .as_ref()
        .map(|operating_point| {
            format!(
                "Run {} · {}{}",
                operating_point.run_id,
                operating_point.analysis,
                if operating_point.current {
                    ""
                } else {
                    " · stale"
                }
            )
        })
        .unwrap_or_else(|| "no retained run".to_owned());
    section_block(ui, "Evaluated at operating point", &status, |ui| {
        if let Some(operating_point) = &context.operating_point {
            for (label, value) in &operating_point.rows {
                evidence_row(ui, label, value);
            }
            ui.add_space(8.0);
            if crate::ui::widgets::Button::new("Cross-probe in results…")
                .show(ui)
                .clicked()
            {
                *action = TabbedDialogResult::CrossProbe;
            }
        } else {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("No retained device operating point")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new("Run a DC operating-point analysis to populate this card.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        }
    });
}

fn terminals_card(ui: &mut Ui, context: &ComponentEditorContext) {
    let open_count = context
        .terminals
        .iter()
        .filter(|terminal| terminal.net.is_none())
        .count();
    let status = if context.terminals.is_empty() {
        "none declared".to_owned()
    } else if open_count == 0 {
        "all bound".to_owned()
    } else {
        format!("{open_count} open")
    };
    section_band(ui, "Terminals", &status);
    if context.terminals.is_empty() {
        let t = Tokens::get(ui.ctx());
        egui::Frame::NONE
            .fill(t.color.bg_panel)
            .inner_margin(Margin::symmetric(16, 10))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("This component has no declared terminals.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        return;
    }

    terminal_table_row(ui, "PIN", "DIRECTION", "NET", true, false);
    for terminal in &context.terminals {
        terminal_table_row(
            ui,
            &terminal.pin,
            &terminal.direction,
            terminal.net.as_deref().unwrap_or("open"),
            false,
            terminal.net.is_none(),
        );
    }
}

fn terminal_table_row(
    ui: &mut Ui,
    pin: &str,
    direction: &str,
    net: &str,
    heading: bool,
    open: bool,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let frame = egui::Frame::NONE
        .fill(if heading { c.bg_panel_2 } else { c.bg_panel })
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                let font = if heading {
                    theme::mono(tokens::FS_0, FontWeight::Medium)
                } else {
                    theme::mono(tokens::FS_0, FontWeight::Regular)
                };
                let color = if heading { c.text_faint } else { c.text };
                ui.add_sized(
                    [62.0, 16.0],
                    egui::Label::new(egui::RichText::new(pin).font(font.clone()).color(color)),
                );
                ui.add_sized(
                    [88.0, 16.0],
                    egui::Label::new(
                        egui::RichText::new(direction)
                            .font(if heading {
                                font.clone()
                            } else {
                                theme::sans(tokens::FS_0, FontWeight::Regular)
                            })
                            .color(if heading { c.text_faint } else { c.text_dim }),
                    ),
                );
                ui.add(
                    egui::Label::new(egui::RichText::new(net).font(font).color(if open {
                        c.warn
                    } else {
                        color
                    }))
                    .truncate(),
                );
            });
        });
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        Stroke::new(1.0, c.border),
    );
}

fn source_preview_card(
    ui: &mut Ui,
    state: &TabbedPropertyDialogState,
    kind: crate::state::ComponentType,
) {
    let status = if kind == crate::state::ComponentType::CurrentSourceNoise {
        "representative realization"
    } else {
        "exact expression"
    };
    section_band(ui, "Transient stimulus preview", status);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 132.0), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            true,
            "Preview of the configured stimulus waveform",
        )
    });
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let plot = egui::Rect::from_min_max(rect.min + vec2(46.0, 8.0), rect.max - vec2(12.0, 20.0));
    ui.painter().line_segment(
        [plot.left_bottom(), plot.left_top()],
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().line_segment(
        [plot.left_bottom(), plot.right_bottom()],
        Stroke::new(1.0, t.color.border),
    );
    for fraction in [0.0_f32, 0.5, 1.0] {
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter()
            .hline(plot.x_range(), y, Stroke::new(0.5, t.color.border));
    }
    let samples = source_preview_samples(state, kind, 96);
    if samples.len() < 2 {
        ui.painter().text(
            plot.center(),
            egui::Align2::CENTER_CENTER,
            "Preview unavailable until values are valid",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    }
    let (minimum, maximum) = samples.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), value| (minimum.min(*value), maximum.max(*value)),
    );
    let raw_span = (maximum - minimum).abs();
    let span = raw_span.max(1e-12);
    for (fraction, label) in [
        (1.0_f32, format!("{maximum:.2e}")),
        (0.5, format!("{:.2e}", (maximum + minimum) * 0.5)),
        (0.0, format!("{minimum:.2e}")),
    ] {
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter().text(
            pos2(plot.left() - 5.0, y),
            egui::Align2::RIGHT_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    for (fraction, label) in [(0.0_f32, "0"), (0.5, "50%"), (1.0, "100%")] {
        let x = egui::lerp(plot.left()..=plot.right(), fraction);
        ui.painter().text(
            pos2(x, plot.bottom() + 5.0),
            egui::Align2::CENTER_TOP,
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    let points = samples
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let x = egui::lerp(
                plot.left()..=plot.right(),
                index as f32 / (samples.len() - 1) as f32,
            );
            let normalized = if raw_span <= 1e-12 {
                0.5
            } else {
                ((*value - minimum) / span) as f32
            };
            let y = egui::lerp(plot.bottom()..=plot.top(), normalized);
            pos2(x, y)
        })
        .collect::<Vec<_>>();
    ui.painter()
        .add(egui::Shape::line(points, Stroke::new(1.5, t.color.accent)));
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn source_preview_samples(
    state: &TabbedPropertyDialogState,
    kind: crate::state::ComponentType,
    count: usize,
) -> Vec<f64> {
    use crate::state::ComponentType;
    if count < 2 {
        return Vec::new();
    }
    let value = |key: &str, fallback: f64| {
        state
            .get_value(key)
            .and_then(PropertyValue::as_number)
            .or_else(|| {
                state.get_value(key).and_then(|value| {
                    crate::quantity::parse_engineering_value(&value.display_string()).ok()
                })
            })
            .unwrap_or(fallback)
    };
    let enabled = |key: &str, fallback: bool| {
        state
            .get_value(key)
            .map(|value| match value {
                PropertyValue::Boolean(value) => *value,
                _ => matches!(
                    value.display_string().trim().to_ascii_lowercase().as_str(),
                    "true" | "yes" | "on" | "1"
                ),
            })
            .unwrap_or(fallback)
    };
    let sample_times = |duration: f64| {
        let duration = duration.max(f64::EPSILON);
        (0..count)
            .map(|index| duration * index as f64 / (count - 1) as f64)
            .collect::<Vec<_>>()
    };
    match kind {
        ComponentType::VoltageSource | ComponentType::CurrentSource => {
            vec![value("dc", 0.0); count]
        }
        ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => {
            let amplitude = value("ac", 1.0);
            let phase_offset = value("acphase", 0.0).to_radians();
            (0..count)
                .map(|index| {
                    let phase = index as f64 / (count - 1) as f64 * std::f64::consts::TAU;
                    amplitude * (phase + phase_offset).sin()
                })
                .collect()
        }
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
            let low = value(
                if kind == ComponentType::VoltageSourcePulse {
                    "v1"
                } else {
                    "i1"
                },
                0.0,
            );
            let high = value(
                if kind == ComponentType::VoltageSourcePulse {
                    "v2"
                } else {
                    "i2"
                },
                1.0,
            );
            let delay = value("td", 0.0).max(0.0);
            let rise = value("tr", 1e-9).max(0.0);
            let fall = value("tf", 1e-9).max(0.0);
            let width = value("pw", 1e-6).max(0.0);
            let period = value("per", 2e-6).max(f64::EPSILON);
            sample_times(delay + 2.0 * period)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        return low;
                    }
                    let phase = (time - delay) % period;
                    if rise > 0.0 && phase < rise {
                        low + (high - low) * phase / rise
                    } else if phase < rise + width {
                        high
                    } else if fall > 0.0 && phase < rise + width + fall {
                        high - (high - low) * (phase - rise - width) / fall
                    } else {
                        low
                    }
                })
                .collect()
        }
        ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
            let offset = value(
                if kind == ComponentType::VoltageSourceSin {
                    "vo"
                } else {
                    "io"
                },
                0.0,
            );
            let amplitude = value(
                if kind == ComponentType::VoltageSourceSin {
                    "va"
                } else {
                    "ia"
                },
                1.0,
            );
            let frequency = value("freq", 1e6).abs().max(f64::EPSILON);
            let delay = value("td", 0.0).max(0.0);
            let damping = value("theta", 0.0).max(0.0);
            let phase = value("phase", 0.0).to_radians();
            sample_times(delay + 2.0 / frequency)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        offset + amplitude * phase.sin()
                    } else {
                        let elapsed = time - delay;
                        offset
                            + amplitude
                                * (-damping * elapsed).exp()
                                * (std::f64::consts::TAU * frequency * elapsed + phase).sin()
                    }
                })
                .collect()
        }
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
            let points = state.pwl_editor.data.points();
            if points.len() < 2 {
                return Vec::new();
            }
            let delay = value("td", 0.0).max(0.0);
            let end = points.last().map_or(1.0, |point| point.time).max(1e-18);
            let repeat = enabled("repeat", false);
            let duration = delay + if repeat { 2.0 * end } else { end };
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    if time < delay {
                        return points[0].value;
                    }
                    let mut time = time - delay;
                    if repeat {
                        time %= end;
                    } else {
                        time = time.min(end);
                    }
                    let right = points
                        .iter()
                        .position(|point| point.time >= time)
                        .unwrap_or(points.len() - 1);
                    if right == 0 {
                        return points[0].value;
                    }
                    let left = right - 1;
                    let span = (points[right].time - points[left].time).max(1e-18);
                    let ratio = (time - points[left].time) / span;
                    points[left].value + ratio * (points[right].value - points[left].value)
                })
                .collect()
        }
        ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
            let low = value(
                if kind == ComponentType::VoltageSourceExp {
                    "v1"
                } else {
                    "i1"
                },
                0.0,
            );
            let high = value(
                if kind == ComponentType::VoltageSourceExp {
                    "v2"
                } else {
                    "i2"
                },
                1.0,
            );
            let first_delay = value("td1", 0.0).max(0.0);
            let first_tau = value("tau1", 1e-6).max(f64::EPSILON);
            let second_delay = value("td2", 5e-6).max(first_delay);
            let second_tau = value("tau2", 1e-6).max(f64::EPSILON);
            let duration = second_delay + 5.0 * second_tau;
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    let rise = if time < first_delay {
                        0.0
                    } else {
                        1.0 - (-(time - first_delay) / first_tau).exp()
                    };
                    let fall = if time < second_delay {
                        0.0
                    } else {
                        1.0 - (-(time - second_delay) / second_tau).exp()
                    };
                    low + (high - low) * (rise - fall)
                })
                .collect()
        }
        ComponentType::VoltageSourceSffm => {
            let offset = value("vo", 0.0);
            let amplitude = value("va", 1.0);
            let carrier_frequency = value("fc", 1e6).abs().max(f64::EPSILON);
            let modulation = value("mdi", 1.0);
            let signal_frequency = value("fs", 1e3).abs();
            sample_times(3.0 / carrier_frequency)
                .into_iter()
                .map(|time| {
                    offset
                        + amplitude
                            * (std::f64::consts::TAU * carrier_frequency * time
                                + modulation
                                    * (std::f64::consts::TAU * signal_frequency * time).sin())
                            .sin()
                })
                .collect()
        }
        ComponentType::CurrentSourceNoise => {
            let dc = value("dc", 0.0);
            if !enabled("isnoisy", true) {
                return vec![dc; count];
            }
            let white_amplitude = value("na", 1e-9).abs();
            let interval = value("nt", 1e-6).max(f64::EPSILON);
            let flicker_exponent = value("nalpha", 0.0).clamp(0.0, 2.0);
            let flicker_amplitude = value("namp", 0.0).abs();
            let duration = interval * 16.0;
            sample_times(duration)
                .into_iter()
                .map(|time| {
                    // Deterministic, sample-and-hold white component plus a
                    // bounded multi-octave 1/f surrogate. This is explicitly
                    // labeled a representative realization in the UI.
                    let sample = (time / interval).floor();
                    let x = (sample + 1.0) * 12.9898;
                    let white = white_amplitude * (x.sin() * 43_758.545_3).sin();
                    let flicker = if flicker_amplitude == 0.0 {
                        0.0
                    } else {
                        let weighted = (1..=5)
                            .map(|octave| {
                                let frequency = 2_f64.powi(octave - 1) / duration;
                                let weight =
                                    frequency.powf(-0.5 * flicker_exponent.max(f64::EPSILON));
                                weight
                                    * (std::f64::consts::TAU * frequency * time
                                        + octave as f64 * 0.731)
                                        .sin()
                            })
                            .sum::<f64>();
                        let normalization = (1..=5)
                            .map(|octave| {
                                let frequency = 2_f64.powi(octave - 1) / duration;
                                frequency.powf(-0.5 * flicker_exponent.max(f64::EPSILON))
                            })
                            .sum::<f64>()
                            .max(f64::EPSILON);
                        flicker_amplitude * weighted / normalization
                    };
                    dc + white + flicker
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn render_model_browser(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    model_library_manager: &crate::state::ModelLibraryManager,
) {
    if state.model_browser.open {
        use crate::properties::model_browser::{ModelBrowserResult, render_model_browser};

        match render_model_browser(ctx, &mut state.model_browser, model_library_manager) {
            ModelBrowserResult::Selected {
                library,
                model,
                corner: _,
            } => {
                state.set_value("model", PropertyValue::String(model));
                state.set_value("model_library", PropertyValue::String(library));
                // Process corner is owned by the simulation plan. Clear any
                // legacy per-instance hint rather than presenting it as an
                // executable component property.
                state.set_value("model_corner", PropertyValue::String(String::new()));
                state.model_browser.open = false;
            }
            ModelBrowserResult::Cancelled => {
                state.model_browser.open = false;
            }
            ModelBrowserResult::None => {}
        }
    }
}

/// One mockup field block: caption, fixed control track, and a stable
/// micro-copy track that validation can replace without moving its neighbors.
fn render_property_field(
    ui: &mut Ui,
    def: &PropertyDefinition,
    state: &mut TabbedPropertyDialogState,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.spacing_mut().item_spacing.y = 2.0;
    let is_modified = state.is_modified(&def.name);
    let error = state.validation_errors.get(&def.name).cloned();
    let current_value = state
        .get_value(&def.name)
        .cloned()
        .unwrap_or_else(|| def.default_value.clone());
    let numeric_text_draft = state.numeric_text_draft(&def.name).map(str::to_owned);
    let browse = def.name == "model";

    ui.set_width(ui.available_width());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        ui.label(
            egui::RichText::new(if def.required {
                format!("{} *", def.display_name)
            } else {
                def.display_name.clone()
            })
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(if error.is_some() { c.err } else { c.text_dim }),
        );
        if is_modified {
            let (dot, _) = ui.allocate_exact_size(vec2(7.0, 12.0), Sense::hover());
            ui.painter().circle_filled(dot.center(), 2.5, c.accent);
        }
    });

    let mut changed_value = None;
    let mut numeric_text = None;
    let mut numeric_parse_error = None;
    let mut browse_clicked = false;
    let mut editor_id = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let editor_width = if browse {
            (ui.available_width() - 58.0).max(60.0)
        } else {
            ui.available_width().max(60.0)
        };
        let editor = render_value_editor(
            ui,
            def,
            &current_value,
            numeric_text_draft.as_deref(),
            editor_width,
            quantity_policy,
            number_locale,
        );
        changed_value = editor.changed;
        numeric_text = editor.numeric_text;
        numeric_parse_error = editor.parse_error;
        editor_id = editor.control_id;
        if browse && crate::ui::widgets::Button::new("Browse").show(ui).clicked() {
            browse_clicked = true;
        }
    });

    if let Some(control_id) = editor_id {
        ui.ctx().accesskit_node_builder(control_id, |node| {
            node.set_label(def.display_name.clone());
            if !def.description.is_empty() {
                node.set_description(def.description.clone());
            }
            if error.is_some() {
                node.set_invalid(egui::accesskit::Invalid::True);
            } else {
                node.clear_invalid();
            }
        });
        if error.is_some()
            && let Some(response) = ui.ctx().read_response(control_id)
        {
            ui.painter().rect_stroke(
                response.rect,
                3.0,
                Stroke::new(1.0, c.err),
                egui::StrokeKind::Inside,
            );
        }
    }
    if let Some(text) = numeric_text {
        state.update_numeric_text_draft(&def.name, text, numeric_parse_error);
    }
    if let Some(value) = changed_value {
        if def.name == "model" {
            state.set_value(&def.name, value);
            // A manually typed name has no proven catalog identity. Clear the
            // prior exact binding so a duplicate name cannot silently retain
            // the wrong library or process corner.
            state.set_value("model_library", PropertyValue::String(String::new()));
            state.set_value("model_corner", PropertyValue::String(String::new()));
        } else {
            state.set_value(&def.name, value);
        }
    }
    if browse_clicked {
        state.model_browser.type_filter = state.component_type.and_then(model_type_for_component);
        state.model_browser.allow_corner_selection = false;
        state.model_browser.selected_library = state
            .get_value("model_library")
            .map(PropertyValue::display_string)
            .filter(|value| !value.trim().is_empty());
        state.model_browser.selected_model = state
            .get_value("model")
            .map(PropertyValue::display_string)
            .filter(|value| !value.trim().is_empty());
        state.model_browser.selected_corner = None;
        state.model_browser.open = true;
    }

    let hint = error.clone().unwrap_or_else(|| {
        let mut hint = def.description.clone();
        if let Some(unit) = def.unit.as_deref() {
            if hint.is_empty() {
                hint = unit.to_owned();
            } else {
                hint.push_str(" · ");
                hint.push_str(unit);
            }
        }
        hint
    });
    let response = ui.add_sized(
        [ui.available_width(), 13.0],
        egui::Label::new(
            egui::RichText::new(if hint.is_empty() { " " } else { &hint })
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(if error.is_some() { c.err } else { c.text_faint }),
        )
        .truncate(),
    );
    if !hint.is_empty() {
        response.on_hover_text(hint);
    }
    editor_id
}

fn model_type_for_component(
    kind: crate::state::ComponentType,
) -> Option<crate::state::model_library::ModelType> {
    use crate::state::ComponentType;
    use crate::state::model_library::ModelType;
    Some(match kind {
        ComponentType::Nmos | ComponentType::NmosSoi | ComponentType::NVdmos => ModelType::Nmos,
        ComponentType::Pmos | ComponentType::PmosSoi | ComponentType::PVdmos => ModelType::Pmos,
        ComponentType::NpnBjt | ComponentType::NpnBjt4 | ComponentType::NpnBjt5 => ModelType::Npn,
        ComponentType::PnpBjt | ComponentType::PnpBjt4 | ComponentType::PnpBjt5 => ModelType::Pnp,
        ComponentType::Diode => ModelType::Diode,
        ComponentType::SaturableInductor => ModelType::Inductor,
        ComponentType::LossyTransmissionLine
        | ComponentType::CoupledTransmissionLine
        | ComponentType::RfPort => ModelType::Rf,
        ComponentType::Memristor
        | ComponentType::VSwitch
        | ComponentType::ISwitch
        | ComponentType::Njfet
        | ComponentType::Pjfet
        | ComponentType::Nmesfet
        | ComponentType::Pmesfet
        | ComponentType::CellInstance => ModelType::Other,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;

    #[test]
    fn sine_preview_preserves_phase_during_the_configured_delay() {
        let mut state = TabbedPropertyDialogState::default();
        state.set_value("vo", PropertyValue::number(1.0));
        state.set_value("va", PropertyValue::number(2.0));
        state.set_value("freq", PropertyValue::number(1.0));
        state.set_value("td", PropertyValue::number(10.0));
        state.set_value("phase", PropertyValue::number(90.0));

        let samples = source_preview_samples(&state, ComponentType::VoltageSourceSin, 8);

        assert!((samples[0] - 3.0).abs() < 1e-12);
        assert!((samples[1] - 3.0).abs() < 1e-12);
    }

    #[test]
    fn disabled_noise_preview_is_exactly_the_dc_bias() {
        let mut state = TabbedPropertyDialogState::default();
        state.set_value("dc", PropertyValue::number(2e-3));
        state.set_value("na", PropertyValue::number(1e-3));
        state.set_value("namp", PropertyValue::number(1e-3));
        state.set_value("isnoisy", PropertyValue::Boolean(false));

        let samples = source_preview_samples(&state, ComponentType::CurrentSourceNoise, 16);

        assert_eq!(samples, vec![2e-3; 16]);
    }
}
