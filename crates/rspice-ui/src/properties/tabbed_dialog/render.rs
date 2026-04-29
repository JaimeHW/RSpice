use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertyValue,
};
use egui::{Color32, RichText, Ui, Window};

use super::editors::render_value_editor;
use super::state::{TabInfo, TabbedDialogResult, TabbedPropertyDialogState};

/// Render the tabbed property dialog.
///
/// Returns the dialog result indicating user action.
pub fn render_tabbed_property_dialog(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    registry: &PropertyRegistry,
    model_library_manager: &crate::state::ModelLibraryManager,
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

    let active_tab = state.active_tab.clone();
    state.update_tab_modified_counts(sheet);

    let title = format!(
        "Edit Properties - {}",
        match (&state.component_name, &state.component_type) {
            (Some(name), Some(kind)) => format!("{} ({:?})", name, kind),
            (Some(name), None) => name.clone(),
            (None, Some(kind)) => format!("{:?}", kind),
            (None, None) => "Component".to_string(),
        }
    );

    let mut should_close = false;
    let mut open = true;

    Window::new(&title)
        .id(egui::Id::new("rspice_property_dialog"))
        .open(&mut open)
        .resizable(true)
        .collapsible(false)
        .scroll(false)
        .default_size([450.0, 400.0])
        .min_width(350.0)
        .min_height(200.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            render_tab_bar(ui, state, component_type, &active_tab);

            ui.separator();

            if active_tab == "PWL Data" && component_type.is_pwl_source() {
                use crate::properties::pwl_editor::render_pwl_editor;
                egui::ScrollArea::vertical()
                    .id_salt("tabbed_dialog_pwl_content")
                    .max_height(280.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        render_pwl_editor(ui, &mut state.pwl_editor);
                    });
            } else {
                let props: Vec<PropertyDefinition> = sheet
                    .iter()
                    .filter(|def| def.category == active_tab)
                    .filter(|def| match def.display_mode {
                        DisplayMode::Hidden => false,
                        DisplayMode::Advanced if !state.show_advanced => false,
                        _ => true,
                    })
                    .cloned()
                    .collect();

                egui::ScrollArea::vertical()
                    .id_salt("tabbed_dialog_props_content")
                    .max_height(280.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for def in &props {
                            render_property_row(ui, def, state);
                        }
                    });
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.checkbox(&mut state.show_advanced, "Show Advanced");

                if state.has_modifications() {
                    ui.label(
                        RichText::new(format!("{} modified", state.modified.len()))
                            .color(Color32::YELLOW),
                    );
                }
            });

            if let Some(error) = &state.global_error.clone() {
                ui.colored_label(Color32::RED, error);
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Apply").clicked() && state.validate_all(sheet) {
                    result = TabbedDialogResult::Applied;
                    should_close = true;
                }

                if ui.button("Cancel").clicked() {
                    result = TabbedDialogResult::Cancelled;
                    should_close = true;
                }

                let has_mods = state.has_modifications();
                if ui
                    .add_enabled(has_mods, egui::Button::new("Revert"))
                    .clicked()
                {
                    state.revert();
                    result = TabbedDialogResult::Reverted;
                }
            });
        });

    if !open {
        should_close = true;
        result = TabbedDialogResult::Cancelled;
    }

    if should_close {
        match result {
            TabbedDialogResult::Applied => state.close_visual(),
            _ => state.close(),
        }
    }

    render_model_browser(ctx, state, model_library_manager);

    result
}

fn render_tab_bar(
    ui: &mut Ui,
    state: &mut TabbedPropertyDialogState,
    component_type: crate::state::ComponentType,
    active_tab: &str,
) {
    ui.horizontal(|ui| {
        let tabs: Vec<TabInfo> = state.tabs.clone();
        for tab in tabs {
            let is_active = active_tab == tab.name;
            let label = if tab.modified_count > 0 {
                format!("{}*", tab.display_name)
            } else {
                tab.display_name.clone()
            };

            if ui.selectable_label(is_active, &label).clicked() {
                state.active_tab = tab.name.clone();
            }
        }

        if component_type.is_pwl_source() {
            let is_pwl_active = active_tab == "PWL Data";
            let pwl_label = if state.pwl_editor.is_modified {
                "PWL Data*"
            } else {
                "PWL Data"
            };
            if ui.selectable_label(is_pwl_active, pwl_label).clicked() {
                state.active_tab = "PWL Data".to_string();
            }
        }
    });
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

/// Render a single property row
fn render_property_row(
    ui: &mut Ui,
    def: &PropertyDefinition,
    state: &mut TabbedPropertyDialogState,
) {
    let is_modified = state.is_modified(&def.name);
    let is_non_default = state.is_non_default(def);
    let has_error = state.validation_errors.contains_key(&def.name);

    ui.horizontal(|ui| {
        let name_text = if def.required {
            format!("{}*", def.display_name)
        } else {
            def.display_name.clone()
        };

        let name_color = if has_error {
            Color32::RED
        } else if is_modified {
            Color32::YELLOW
        } else {
            ui.visuals().text_color()
        };

        let name_richtext = if is_non_default {
            RichText::new(&name_text).color(name_color).strong()
        } else {
            RichText::new(&name_text).color(name_color)
        };

        ui.label(name_richtext);

        let current_value = state
            .get_value(&def.name)
            .cloned()
            .unwrap_or_else(|| def.default_value.clone());
        if let Some(new_value) = render_value_editor(ui, def, &current_value) {
            state.set_value(&def.name, new_value);
        }

        if let Some(unit) = &def.unit {
            ui.label(RichText::new(unit).weak());
        }

        if def.name == "model"
            && let Some(comp_type) = state.component_type
            && comp_type.is_semiconductor()
            && ui.small_button("📖 Browse...").clicked()
        {
            state.model_browser.open = true;
        }

        if current_value.is_expression() {
            ui.label(RichText::new("{E}").color(Color32::LIGHT_BLUE).small());
        }
    });

    if let Some(error) = state.validation_errors.get(&def.name) {
        ui.colored_label(Color32::RED, format!("  ↳ {}", error));
    }

    if !def.description.is_empty() {
        ui.label(RichText::new(&def.description).weak().small());
    }
}
