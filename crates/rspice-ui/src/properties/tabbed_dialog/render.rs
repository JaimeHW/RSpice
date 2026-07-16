//! The tabbed property editor on the modal primitive.
//!
//! Categories from the registry become underline tabs; each property is a
//! registry row — dimmed label, typed editor, unit, accent dot when
//! modified — with inline validation under the row. PWL sources gain a
//! waveform tab; semiconductors a model Browse action. Footer follows the
//! dialog grammar: Revert (ghost) · Apply (secondary) · OK (primary).

use egui::Ui;

use crate::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertyValue,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, dialog_tabs};

use super::editors::render_value_editor;
use super::state::{TabbedDialogResult, TabbedPropertyDialogState};

/// Label column width, matching the inspector form grid.
const LABEL_COL: f32 = 110.0;

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
        None => return result,
    };

    let sheet = match registry.get(component_type) {
        Some(s) => s,
        None => return result,
    };

    state.update_tab_modified_counts(sheet);

    let title = state
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

    let choice = Dialog::new("Schematic", &title, "OK")
        .description(
            "Edit and validate this component's typed properties and optional PWL data before applying or reverting changes.",
        )
        .size(DialogSize::Transaction)
        .secondary("Apply")
        .ghost("Revert")
        .hint(&hint)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;

            let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
            dialog_tabs(ui, &label_refs, &mut active_index);
            if let Some(name) = names.get(active_index) {
                state.active_tab = name.clone();
            }

            if state.active_tab == "PWL Data" && component_type.is_pwl_source() {
                if crate::properties::pwl_editor::render_pwl_editor(
                    ui,
                    &mut state.pwl_editor,
                    quantity_policy,
                    number_locale,
                )
                    == crate::properties::pwl_editor::PwlEditorResult::Modified
                {
                    state.pwl_editor.is_modified = true;
                    state.set_value(
                        "pwl_data",
                        PropertyValue::String(state.pwl_editor.to_string()),
                    );
                }
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
                    render_property_row(ui, def, state, quantity_policy, number_locale);
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
        });

    match choice {
        DialogChoice::Primary => {
            if state.prepare_commit(sheet, commit_policy) {
                result = TabbedDialogResult::Applied;
                // Partial application with failures remains open so the
                // rejected draft fields can be repaired or explicitly
                // cancelled. No invalid value crosses the dialog boundary.
                if state.validation_errors.is_empty() {
                    state.close_visual();
                }
            }
        }
        DialogChoice::Secondary => {
            if state.prepare_commit(sheet, commit_policy) {
                result = TabbedDialogResult::Applied;
            }
        }
        DialogChoice::Ghost => {
            state.revert();
            result = TabbedDialogResult::Reverted;
        }
        DialogChoice::Cancelled => {
            state.close();
            result = TabbedDialogResult::Cancelled;
        }
        DialogChoice::None => {}
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
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;

    let is_modified = state.is_modified(&def.name);
    let has_error = state.validation_errors.contains_key(&def.name);
    let current_value = state
        .get_value(&def.name)
        .cloned()
        .unwrap_or_else(|| def.default_value.clone());

    let browse = def.name == "model"
        && state
            .component_type
            .is_some_and(|kind| kind.is_semiconductor());

    let mut new_value: Option<PropertyValue> = None;
    let mut browse_clicked = false;

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
            new_value = render_value_editor(
                ui,
                def,
                &current_value,
                editor_width,
                quantity_policy,
                number_locale,
            );

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
}
