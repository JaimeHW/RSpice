//! The tabbed property editor on the modal primitive.
//!
//! Categories from the registry become underline tabs; each property is a
//! registry row — dimmed label, typed editor, unit, accent dot when
//! modified — with inline validation under the row. PWL sources gain a
//! waveform tab; semiconductors a model Browse action. The shell follows the
//! mockup's one explicit Apply transaction plus guarded Cancel lifecycle.

use egui::Ui;

use crate::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertyValue,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, SelectionImpact,
    SelectionPreview, dialog_tabs, selection_command_workflow, workflow_preview_status,
};

use super::editors::render_value_editor;
use super::state::{TabbedDialogResult, TabbedPropertyDialogState};

/// Label column width, matching the inspector form grid.
const LABEL_COL: f32 = 110.0;
const DIALOG_SIZE: DialogSize = DialogSize::SimulationWorkflow;
const EYEBROW: &str = "EDIT · TYPED PARAMETERS";
const TITLE: &str = "Object properties";
const PRIMARY: &str = "Apply object properties";
const DESCRIPTION: &str = "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata.";
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No project or result data has been changed.";

/// Render the tabbed property dialog.
///
/// Returns the dialog result indicating user action.
pub fn render_tabbed_property_dialog(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
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

    let component_type = match state.component_type {
        Some(t) => t,
        None => {
            state.close();
            return TabbedDialogResult::Cancelled;
        }
    };

    let sheet = match registry.get(component_type) {
        Some(s) => s,
        None => {
            state.close();
            return TabbedDialogResult::Cancelled;
        }
    };

    state.sync_pwl_validation_error();
    state.update_tab_modified_counts(sheet);

    let object_name = state
        .component_name
        .clone()
        .unwrap_or_else(|| "Component".to_owned());
    let error_count = state.validation_errors.len();
    let hint = if error_count > 0 {
        format!(
            "{error_count} invalid value{}",
            if error_count == 1 { "" } else { "s" }
        )
    } else if state.has_modifications() {
        format!("{} modified", state.modified.len())
    } else {
        component_type.display_name().to_owned()
    };

    // Tab labels: registry categories plus the PWL waveform tab; a dot
    // marks tabs holding modified values.
    let mut labels: Vec<String> = Vec::with_capacity(state.tabs.len() + 1);
    let mut names: Vec<String> = Vec::with_capacity(state.tabs.len() + 1);
    for tab in &state.tabs {
        labels.push(if tab.modified_count > 0 {
            format!("{} •", tab.display_name)
        } else {
            tab.display_name.clone()
        });
        names.push(tab.name.clone());
    }
    if component_type.is_pwl_source() {
        labels.push(if state.pwl_editor.is_modified {
            "PWL •".to_owned()
        } else {
            "PWL".to_owned()
        });
        names.push("PWL Data".to_owned());
    }
    let mut active_index = names
        .iter()
        .position(|name| *name == state.active_tab)
        .unwrap_or(0);

    let discard_confirm = state.discard_confirm;
    let session_error = state.session_error.clone();
    let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
        .description(DESCRIPTION)
        .size(DIALOG_SIZE)
        .ghost(if discard_confirm {
            "Discard changes"
        } else {
            "Cancel"
        })
        .primary_enabled(state.can_apply(commit_policy) && session_error.is_none())
        .interaction_enabled(!state.model_browser.open)
        .initial_focus(DialogInitialFocus::BodyControl)
        .hint(&hint);
    if discard_confirm {
        dialog =
            dialog.transaction_state(DialogTransactionTone::Error, DISCARD_TITLE, DISCARD_DETAIL);
    } else if let Some(error) = session_error.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Properties cannot be applied",
            error,
        );
    }
    let preview_label = format!("{} · {}", object_name, component_type.display_name());
    let preview = SelectionPreview::Component {
        label: preview_label.clone(),
    };
    let effect = if state.has_modifications() {
        format!(
            "{} typed property change{}",
            state.modified.len(),
            if state.modified.len() == 1 { "" } else { "s" }
        )
    } else {
        "typed property draft".to_owned()
    };
    let options_status = if session_error.is_some() || !state.validation_errors.is_empty() {
        "attention required"
    } else {
        "scope resolved"
    };
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        selection_command_workflow(
            ui,
            "PROP",
            &preview,
            SelectionImpact {
                scope: "one selected component",
                effect: &effect,
                recovery: "one semantic undo record",
            },
            options_status,
            session_error.is_none() && state.validation_errors.is_empty(),
            |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;

                let t = Tokens::get(ui.ctx());
                ui.label(
                    egui::RichText::new("Object")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                let object_frame = egui::Frame::new()
                    .fill(t.color.bg_app)
                    .stroke(egui::Stroke::new(1.0, t.color.border))
                    .corner_radius(3.0)
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.label(
                            egui::RichText::new(&preview_label)
                                .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                                .color(t.color.text),
                        );
                    });
                ui.ctx()
                    .accesskit_node_builder(object_frame.response.id, |node| {
                        node.set_label("Object");
                        node.set_value(preview_label.clone());
                    });
                ui.add_space(8.0);

                let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
                egui::ScrollArea::horizontal()
                    .id_salt("component-property-tabs")
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .auto_shrink([false, true])
                    .show(ui, |ui| dialog_tabs(ui, &label_refs, &mut active_index));
                if let Some(name) = names.get(active_index) {
                    state.active_tab = name.clone();
                }

                let mut first_control = None;
                if state.active_tab == "PWL Data" && component_type.is_pwl_source() {
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
                } else {
                    let props: Vec<PropertyDefinition> = sheet
                        .iter()
                        .filter(|def| def.category == state.active_tab)
                        .filter(|def| match def.display_mode {
                            DisplayMode::Hidden => false,
                            DisplayMode::Advanced if !state.show_advanced => false,
                            _ => true,
                        })
                        .cloned()
                        .collect();
                    for def in &props {
                        let control = ui
                            .push_id(("object-property", &def.name), |ui| {
                                render_property_row(ui, def, state, quantity_policy, number_locale)
                            })
                            .inner;
                        first_control = first_control.or(control);
                    }

                    let has_advanced = sheet
                        .iter()
                        .any(|def| def.display_mode == DisplayMode::Advanced);
                    if has_advanced {
                        ui.add_space(6.0);
                        crate::ui::widgets::check_row(
                            ui,
                            "Show advanced properties",
                            &mut state.show_advanced,
                        );
                    }
                }

                if let Some(error) = state.global_error.clone() {
                    let t = Tokens::get(ui.ctx());
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    );
                }
                ui.add_space(8.0);
                let workflow_error = session_error
                    .as_deref()
                    .or_else(|| state.validation_errors.values().next().map(String::as_str));
                workflow_preview_status(
                    ui,
                    workflow_error.is_none(),
                    if workflow_error.is_some() {
                        "Transaction blocked"
                    } else {
                        "One explicit editor transaction"
                    },
                    workflow_error.unwrap_or(
                        "Locked, hidden, protected, and out-of-hierarchy objects are excluded and reported.",
                    ),
                );
                first_control
            },
        )
    });

    match choice {
        DialogChoice::Primary => {
            if state.prepare_commit(sheet, commit_policy) {
                result = TabbedDialogResult::Applied;
                // Partial-policy commits retain rejected fields in the same
                // isolated editor. Atomic or fully valid commits close.
                if state.validation_errors.is_empty() {
                    state.close_visual();
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if state.attempt_close() {
                result = TabbedDialogResult::Cancelled;
            }
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }

    render_model_browser(ctx, state, model_library_manager);

    result
}

fn render_model_browser(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    model_library_manager: &crate::state::ModelLibraryManager,
) {
    if state.model_browser.open {
        use crate::properties::model_browser::{ModelBrowserResult, render_model_browser};

        match render_model_browser(ctx, &mut state.model_browser, model_library_manager) {
            ModelBrowserResult::Selected { model, .. } => {
                state.set_value("model", PropertyValue::String(model));
                state.model_browser.open = false;
            }
            ModelBrowserResult::Cancelled => {
                state.model_browser.open = false;
            }
            ModelBrowserResult::None => {}
        }
    }
}

/// One registry row: dimmed label, typed editor, unit, modified dot.
fn render_property_row(
    ui: &mut Ui,
    def: &PropertyDefinition,
    state: &mut TabbedPropertyDialogState,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;

    let is_modified = state.is_modified(&def.name);
    let has_error = state.validation_errors.contains_key(&def.name);
    let current_value = state
        .get_value(&def.name)
        .cloned()
        .unwrap_or_else(|| def.default_value.clone());
    let numeric_text_draft = state.numeric_text_draft(&def.name).map(str::to_owned);

    let browse = def.name == "model"
        && state
            .component_type
            .is_some_and(|kind| kind.is_semiconductor());

    let mut new_value: Option<PropertyValue> = None;
    let mut numeric_text = None;
    let mut numeric_parse_error = None;
    let mut browse_clicked = false;
    let mut editor_id = None;

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Label — err when invalid, full text when modified, dim otherwise.
            let (label_rect, _) =
                ui.allocate_exact_size(egui::vec2(LABEL_COL, row_h), egui::Sense::hover());
            let label_color = if has_error {
                c.err
            } else if is_modified {
                c.text
            } else {
                c.text_dim
            };
            let label_text = if def.required {
                format!("{} *", def.display_name)
            } else {
                def.display_name.clone()
            };
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label_text,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                label_color,
            );
            if !def.description.is_empty() {
                ui.interact(
                    label_rect,
                    ui.id().with(("prop-label", &def.name)),
                    egui::Sense::hover(),
                )
                .on_hover_text(&def.description);
            }

            // Right-side reserve: modified dot, unit, browse.
            let mut reserve = 0.0;
            if is_modified {
                reserve += 14.0;
            }
            if let Some(unit) = &def.unit {
                reserve += 8.0 + 7.0 * unit.len() as f32;
            }
            if browse {
                reserve += 58.0;
            }

            let editor_width = (ui.available_width() - reserve).max(60.0);
            let editor = render_value_editor(
                ui,
                def,
                &current_value,
                numeric_text_draft.as_deref(),
                editor_width,
                quantity_policy,
                number_locale,
            );
            new_value = editor.changed;
            numeric_text = editor.numeric_text;
            numeric_parse_error = editor.parse_error;
            if let Some(control_id) = editor.control_id {
                editor_id = Some(control_id);
                ui.ctx().accesskit_node_builder(control_id, |node| {
                    node.set_label(def.display_name.clone());
                    if !def.description.is_empty() {
                        node.set_description(def.description.clone());
                    }
                    if has_error {
                        node.set_invalid(egui::accesskit::Invalid::True);
                    } else {
                        node.clear_invalid();
                    }
                });
            }

            if let Some(unit) = &def.unit {
                ui.label(
                    egui::RichText::new(unit)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            }
            if browse
                && crate::ui::widgets::Button::new("Browse")
                    .ghost()
                    .show(ui)
                    .clicked()
            {
                browse_clicked = true;
            }
            if is_modified {
                let (dot_rect, _) =
                    ui.allocate_exact_size(egui::vec2(8.0, row_h), egui::Sense::hover());
                ui.painter().circle_filled(dot_rect.center(), 2.5, c.accent);
            }
        },
    );

    if let Some(text) = numeric_text {
        state.update_numeric_text_draft(&def.name, text, numeric_parse_error);
    }
    if let Some(value) = new_value {
        state.set_value(&def.name, value);
    }
    if browse_clicked {
        state.model_browser.open = true;
    }

    if let Some(error) = state.validation_errors.get(&def.name) {
        ui.label(
            egui::RichText::new(error.as_str())
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }
    editor_id
}
