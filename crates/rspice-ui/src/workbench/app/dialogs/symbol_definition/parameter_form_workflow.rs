use egui::{
    Align, ComboBox, Context, Frame, Grid, Layout, Margin, ScrollArea, Stroke, TextEdit, Ui, vec2,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, ParameterInheritance, PropertyType,
    SymbolParameterConstraints, SymbolParameterDefault, SymbolParameterField, SymbolParameterForm,
    SymbolParameterSection, SymbolParameterVisibility,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::RSpiceApp;

use super::import_workflow::{setting_row, table_header};
use super::state::ParameterFormPhase;
use crate::workbench::app::publish_symbol_definition_candidate;

const FORM_EYEBROW: &str = "SYMBOL LIBRARY \u{00b7} TYPED PARAMETER FORM";
const FORM_REVIEW_TITLE: &str = "Open component form designer";
const FORM_DESIGNER_TITLE: &str = "Component form designer";
const DISCARD_TITLE: &str = "Discard unsaved symbol changes?";
const DISCARD_DETAIL: &str =
    "The isolated draft will be removed. The project library has not been changed.";

impl RSpiceApp {
    pub(super) fn render_symbol_parameter_form_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.symbol_parameter_form.open {
            return;
        }
        let phase = self.state.dialogs.symbol_parameter_form.phase;
        let discard_confirm = self.state.dialogs.symbol_parameter_form.discard_confirm;
        let diagnostics = self
            .state
            .dialogs
            .symbol_parameter_form
            .draft_form
            .as_ref()
            .map(SymbolParameterForm::validate_diagnostics)
            .unwrap_or_else(|| {
                vec![crate::state::SymbolFormDiagnostic {
                    section_key: "form".to_owned(),
                    field_key: None,
                    message: "typed form draft is unavailable".to_owned(),
                }]
            });
        let error = self
            .state
            .dialogs
            .symbol_parameter_form
            .validation_error
            .clone();
        let (title, primary, size, height) = match phase {
            ParameterFormPhase::Review => (
                FORM_REVIEW_TITLE,
                "Open form designer",
                DialogSize::SimulationWorkflow,
                420.0,
            ),
            ParameterFormPhase::Designer => (
                FORM_DESIGNER_TITLE,
                "Save form revision",
                DialogSize::WideWorkflow,
                720.0,
            ),
        };
        let mut dialog = Dialog::new(FORM_EYEBROW, title, primary)
            .description("Edit typed engineering parameters, defaults, constraints, inheritance, visibility and device help as one versioned symbol contract.")
            .size(size)
            .initial_height(height)
            .initial_focus(DialogInitialFocus::BodyControl)
            .ghost(if discard_confirm { "Discard changes" } else { "Cancel" })
            .primary_enabled(phase == ParameterFormPhase::Review || diagnostics.is_empty());
        if self.state.dialogs.symbol_parameter_form.dirty && !discard_confirm {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if let Some(error) = error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Parameter form was not saved",
                error,
            );
        } else if let Some(finding) = diagnostics.first() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Parameter form validation",
                &finding.message,
            );
        }

        let mut changed = false;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let id = match phase {
                ParameterFormPhase::Review => form_review_body(
                    ui,
                    &mut self.state.dialogs.symbol_parameter_form,
                    &self.state.library_manager,
                ),
                ParameterFormPhase::Designer => form_designer_body(
                    ui,
                    &mut self.state.dialogs.symbol_parameter_form,
                    &diagnostics,
                    &mut changed,
                ),
            };
            Some(id)
        });
        if changed {
            self.state.dialogs.symbol_parameter_form.dirty = true;
            self.state.dialogs.symbol_parameter_form.discard_confirm = false;
            self.state.dialogs.symbol_parameter_form.validation_error = None;
        }
        match choice {
            DialogChoice::Primary if phase == ParameterFormPhase::Review => {
                self.state.dialogs.symbol_parameter_form.phase = ParameterFormPhase::Designer;
            }
            DialogChoice::Primary => self.commit_symbol_parameter_form(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if self.state.dialogs.symbol_parameter_form.dirty
                    && !self.state.dialogs.symbol_parameter_form.discard_confirm
                {
                    self.state.dialogs.symbol_parameter_form.discard_confirm = true;
                } else {
                    self.state.dialogs.symbol_parameter_form.close_and_discard();
                }
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn commit_symbol_parameter_form(&mut self) {
        self.state.dialogs.symbol_parameter_form.validation_error = None;
        let Some(target) = self.state.dialogs.symbol_parameter_form.target.clone() else {
            self.state.dialogs.symbol_parameter_form.validation_error =
                Some("The target symbol is unavailable.".to_owned());
            return;
        };
        let Some(original) = self
            .state
            .dialogs
            .symbol_parameter_form
            .original_definition
            .clone()
        else {
            self.state.dialogs.symbol_parameter_form.validation_error =
                Some("The original symbol definition is unavailable.".to_owned());
            return;
        };
        let Some(draft) = self.state.dialogs.symbol_parameter_form.draft_form.clone() else {
            self.state.dialogs.symbol_parameter_form.validation_error =
                Some("The parameter form draft is unavailable.".to_owned());
            return;
        };
        let result = (|| {
            let current_view = self
                .state
                .library_manager
                .get_library(&target.library)
                .and_then(|library| library.get_cell(&target.cell))
                .and_then(|cell| cell.get_view(&target.view))
                .ok_or_else(|| "The target symbol view no longer exists.".to_owned())?;
            let current = ModelBoundSymbolDefinition::load_from_view(current_view)
                .map_err(|error| error.to_string())?
                .ok_or_else(|| "The target symbol no longer has a typed definition.".to_owned())?;
            if current
                .validation_digest()
                .map_err(|error| error.to_string())?
                != original
                    .validation_digest()
                    .map_err(|error| error.to_string())?
            {
                return Err(
                    "The symbol definition changed after the form designer opened. Reopen it before saving."
                        .to_owned(),
                );
            }
            let replacement = original
                .replace_parameter_form(draft)
                .map_err(|error| error.to_string())?;
            let mut candidate = self.state.library_manager.clone();
            let library = candidate
                .get_library_mut(&target.library)
                .ok_or_else(|| format!("Library '{}' no longer exists.", target.library))?;
            let plan = replacement
                .build_plan(library)
                .map_err(|error| error.to_string())?;
            plan.commit(library).map_err(|error| error.to_string())?;
            publish_symbol_definition_candidate(
                &mut self.state,
                candidate,
                &target.library,
                &target.cell,
                format!("save parameter form for '{}'/{}", target.cell, target.view),
            )?;
            Ok::<u64, String>(replacement.parameter_form.revision)
        })();
        match result {
            Ok(revision) => {
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "Saved parameter form revision {revision} for {}/{}",
                    target.library, target.cell
                )));
                self.state.dialogs.symbol_parameter_form.close_and_discard();
            }
            Err(error) => self.state.dialogs.symbol_parameter_form.validation_error = Some(error),
        }
    }
}

fn form_review_body(
    ui: &mut Ui,
    state: &mut super::SymbolParameterFormDialogState,
    libraries: &crate::state::LibraryManager,
) -> egui::Id {
    let target_label = state
        .target
        .as_ref()
        .map(CellViewRef::display_path)
        .unwrap_or_else(|| "unavailable".to_owned());
    let definition = state.original_definition.as_ref();
    let model = definition
        .and_then(|definition| definition.netlist.model.as_ref())
        .map(|model| model.model.as_str())
        .unwrap_or("unbound");
    let response = setting_row(
        ui,
        "Symbol family",
        "Selected model-bound component definition.",
        |ui| ui.monospace(format!("{target_label} \u{00b7} {model}")),
    );
    setting_row(
        ui,
        "Form revision",
        "Parameter schema, units, defaults, validation, visibility, and help.",
        |ui| {
            let current_revision = state
                .original_definition
                .as_ref()
                .map_or(0, |definition| definition.parameter_form.revision);
            ui.monospace(format!(
                "revision {current_revision} \u{00b7} next governed revision"
            ))
        },
    );
    form_summary_table(ui, state.draft_form.as_ref());
    let unsaved = state.target.as_ref().is_some_and(|target| {
        libraries
            .get_library(&target.library)
            .and_then(|library| library.get_cell(&target.cell))
            .is_some_and(|cell| cell.views.values().any(|view| view.modified))
    });
    setting_row(
        ui,
        "Unsaved library changes",
        "The form designer never overwrites another draft.",
        |ui| {
            let t = Tokens::get(ui.ctx());
            ui.colored_label(
                if unsaved { t.color.warn } else { t.color.ok },
                if unsaved {
                    "present \u{00b7} save or review before publication"
                } else {
                    "none \u{00b7} safe to open"
                },
            )
        },
    );
    response.id
}

fn form_summary_table(ui: &mut Ui, form: Option<&SymbolParameterForm>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border))
        .show(ui, |ui| {
            Grid::new("symbol-form-summary")
                .num_columns(3)
                .striped(true)
                .min_col_width(150.0)
                .show(ui, |ui| {
                    table_header(ui, "SECTION");
                    table_header(ui, "FIELDS");
                    table_header(ui, "VALIDATION");
                    ui.end_row();
                    if let Some(form) = form {
                        for section in &form.sections {
                            let valid = form
                                .validate_diagnostics()
                                .iter()
                                .all(|finding| finding.section_key != section.key);
                            ui.label(&section.label);
                            ui.monospace(section.fields.len().to_string());
                            ui.colored_label(
                                if valid { t.color.ok } else { t.color.warn },
                                if valid { "complete" } else { "review" },
                            );
                            ui.end_row();
                        }
                    }
                });
        });
}

fn form_designer_body(
    ui: &mut Ui,
    state: &mut super::SymbolParameterFormDialogState,
    diagnostics: &[crate::state::SymbolFormDiagnostic],
    changed: &mut bool,
) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    let first_id = ui.id().with("form-designer-first");
    let Some(mut form) = state.draft_form.take() else {
        ui.colored_label(t.color.err, "Typed parameter form draft is unavailable.");
        return first_id;
    };
    normalize_form_selection(&form, state);
    section_toolbar(ui, &mut form, state, changed);
    ui.add_space(6.0);
    let available = ui.available_size();
    let list_width = (available.x * 0.34).clamp(250.0, 330.0);
    ui.allocate_ui_with_layout(available, Layout::left_to_right(Align::Min), |ui| {
        ui.allocate_ui(vec2(list_width, available.y), |ui| {
            field_list(ui, &mut form, state, changed);
        });
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        ui.allocate_ui(vec2(ui.available_width(), available.y), |ui| {
            field_editor(ui, &mut form, state, diagnostics, changed);
        });
    });
    state.draft_form = Some(form);
    first_id
}

fn section_toolbar(
    ui: &mut Ui,
    form: &mut SymbolParameterForm,
    state: &mut super::SymbolParameterFormDialogState,
    changed: &mut bool,
) {
    ui.horizontal_wrapped(|ui| {
        for section in &form.sections {
            if ui
                .selectable_label(
                    state.selected_section.as_deref() == Some(&section.key),
                    &section.label,
                )
                .clicked()
            {
                state.selected_section = Some(section.key.clone());
                state.selected_field = section.fields.first().map(|field| field.key.clone());
            }
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if Button::new("Add section").show(ui).clicked() {
                let key = unique_section_key(form);
                let section = SymbolParameterSection {
                    key: key.clone(),
                    label: "New section".to_owned(),
                    help: "Describe the parameter group for component users.".to_owned(),
                    fields: Vec::new(),
                };
                if form.add_section(section).is_ok() {
                    state.selected_section = Some(key);
                    state.selected_field = None;
                    *changed = true;
                }
            }
            let selected = state.selected_section.clone();
            if Button::new("Remove section")
                .enabled(form.sections.len() > 1 && selected.is_some())
                .show(ui)
                .clicked()
                && let Some(key) = selected
            {
                form.remove_section(&key);
                state.selected_section = form.sections.first().map(|section| section.key.clone());
                state.selected_field = None;
                *changed = true;
            }
            let can_move = form.sections.len() > 1 && state.selected_section.is_some();
            if Button::new("Down").enabled(can_move).show(ui).clicked() {
                *changed |= move_selected_section(form, state, 1);
            }
            if Button::new("Up").enabled(can_move).show(ui).clicked() {
                *changed |= move_selected_section(form, state, -1);
            }
        });
    });
    if let Some(section) = selected_section_mut(form, state) {
        let original_section_key = section.key.clone();
        Grid::new("parameter-section-properties")
            .num_columns(2)
            .spacing(vec2(8.0, 4.0))
            .show(ui, |ui| {
                ui.label("Section key");
                *changed |= ui.text_edit_singleline(&mut section.key).changed();
                ui.end_row();
                ui.label("Section label");
                *changed |= ui.text_edit_singleline(&mut section.label).changed();
                ui.end_row();
                ui.label("Section help");
                *changed |= ui.text_edit_singleline(&mut section.help).changed();
                ui.end_row();
            });
        if section.key != original_section_key && !section.key.trim().is_empty() {
            state.selected_section = Some(section.key.clone());
        }
    }
}

fn field_list(
    ui: &mut Ui,
    form: &mut SymbolParameterForm,
    state: &mut super::SymbolParameterFormDialogState,
    changed: &mut bool,
) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new("FIELDS")
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    ui.horizontal(|ui| {
        if Button::new("Add field").show(ui).clicked()
            && let Some(section_key) = state.selected_section.clone()
        {
            let key = unique_field_key(form);
            let field = default_parameter_field(&key);
            if form.add_field(&section_key, field).is_ok() {
                state.selected_field = Some(key);
                *changed = true;
            }
        }
        let can_edit = state.selected_field.is_some();
        if Button::new("Up").enabled(can_edit).show(ui).clicked() {
            *changed |= move_selected_field(form, state, -1);
        }
        if Button::new("Down").enabled(can_edit).show(ui).clicked() {
            *changed |= move_selected_field(form, state, 1);
        }
        if Button::new("Remove").enabled(can_edit).show(ui).clicked()
            && let Some(key) = state.selected_field.take()
        {
            form.remove_field(&key);
            state.selected_field = selected_section(form, state)
                .and_then(|section| section.fields.first())
                .map(|field| field.key.clone());
            *changed = true;
        }
    });
    Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(4))
        .show(ui, |ui| {
            ScrollArea::vertical()
                .id_salt("parameter-form-field-list")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if let Some(section) = selected_section(form, state) {
                        for field in &section.fields {
                            let label = format!("{}  \u{00b7}  {}", field.label, field.key);
                            if ui
                                .selectable_label(
                                    state.selected_field.as_deref() == Some(&field.key),
                                    label,
                                )
                                .clicked()
                            {
                                state.selected_field = Some(field.key.clone());
                            }
                        }
                    }
                });
        });
}

fn field_editor(
    ui: &mut Ui,
    form: &mut SymbolParameterForm,
    state: &mut super::SymbolParameterFormDialogState,
    diagnostics: &[crate::state::SymbolFormDiagnostic],
    changed: &mut bool,
) {
    let Some(field_key) = state.selected_field.clone() else {
        ui.label("Add or select a field to edit its typed contract.");
        return;
    };
    let Some(current_section_key) = form
        .sections
        .iter()
        .find(|section| section.fields.iter().any(|field| field.key == field_key))
        .map(|section| section.key.clone())
    else {
        ui.label("The selected field is no longer available.");
        return;
    };
    let section_options = form
        .sections
        .iter()
        .map(|section| (section.key.clone(), section.label.clone()))
        .collect::<Vec<_>>();
    let mut destination_section = current_section_key.clone();
    ui.horizontal(|ui| {
        ui.label("Section");
        ComboBox::from_id_salt("parameter-field-section")
            .selected_text(
                section_options
                    .iter()
                    .find(|(key, _)| *key == destination_section)
                    .map_or(destination_section.as_str(), |(_, label)| label.as_str()),
            )
            .show_ui(ui, |ui| {
                for (key, label) in &section_options {
                    ui.selectable_value(&mut destination_section, key.clone(), label);
                }
            });
    });
    if destination_section != current_section_key {
        let destination = form
            .sections
            .iter()
            .find(|section| section.key == destination_section)
            .map_or(0, |section| section.fields.len());
        if form
            .move_field(&field_key, &destination_section, destination)
            .is_ok()
        {
            state.selected_section = Some(destination_section);
            *changed = true;
        }
    }
    let Some(field) = form
        .sections
        .iter_mut()
        .flat_map(|section| &mut section.fields)
        .find(|field| field.key == field_key)
    else {
        ui.label("The selected field is no longer available.");
        return;
    };
    let original_field_key = field.key.clone();
    let t = Tokens::get(ui.ctx());
    ScrollArea::vertical()
        .id_salt("parameter-form-field-editor")
        .show(ui, |ui| {
            Grid::new("parameter-field-properties")
                .num_columns(2)
                .spacing(vec2(10.0, 6.0))
                .show(ui, |ui| {
                    ui.label("Stable key");
                    *changed |= ui.text_edit_singleline(&mut field.key).changed();
                    ui.end_row();
                    ui.label("Label");
                    *changed |= ui.text_edit_singleline(&mut field.label).changed();
                    ui.end_row();
                    ui.label("Type");
                    let prior = field.property_type;
                    ComboBox::from_id_salt("parameter-field-type")
                        .selected_text(property_type_label(field.property_type))
                        .show_ui(ui, |ui| {
                            for value in [
                                PropertyType::Number,
                                PropertyType::String,
                                PropertyType::Expression,
                                PropertyType::Enum,
                                PropertyType::Boolean,
                            ] {
                                ui.selectable_value(
                                    &mut field.property_type,
                                    value,
                                    property_type_label(value),
                                );
                            }
                        });
                    if prior != field.property_type {
                        field.default = default_for_type(field.property_type);
                        field.constraints = SymbolParameterConstraints::default();
                        *changed = true;
                    }
                    ui.end_row();
                    ui.label("Default");
                    *changed |= default_editor(ui, field);
                    ui.end_row();
                    if field.property_type == PropertyType::Number {
                        ui.label("Unit");
                        let mut unit = field.unit.clone().unwrap_or_default();
                        if ui.text_edit_singleline(&mut unit).changed() {
                            field.unit = (!unit.trim().is_empty()).then(|| unit.trim().to_owned());
                            *changed = true;
                        }
                        ui.end_row();
                        ui.label("Minimum");
                        *changed |= optional_text(ui, &mut field.constraints.minimum);
                        ui.end_row();
                        ui.label("Maximum");
                        *changed |= optional_text(ui, &mut field.constraints.maximum);
                        ui.end_row();
                    }
                    if field.property_type == PropertyType::Enum {
                        ui.label("Choices");
                        let mut options = field.constraints.enum_values.join(", ");
                        if ui.text_edit_singleline(&mut options).changed() {
                            field.constraints.enum_values = options
                                .split(',')
                                .map(str::trim)
                                .filter(|value| !value.is_empty())
                                .map(str::to_owned)
                                .collect();
                            *changed = true;
                        }
                        ui.end_row();
                    }
                    if matches!(
                        field.property_type,
                        PropertyType::String | PropertyType::Expression
                    ) {
                        ui.label("Maximum length");
                        ui.horizontal(|ui| {
                            let mut limited = field.constraints.max_length.is_some();
                            if ui.checkbox(&mut limited, "Limit").changed() {
                                field.constraints.max_length = limited.then_some(256);
                                *changed = true;
                            }
                            if let Some(max_length) = &mut field.constraints.max_length {
                                *changed |= ui
                                    .add(egui::DragValue::new(max_length).range(1..=1_048_576))
                                    .changed();
                                ui.label("characters");
                            }
                        });
                        ui.end_row();
                    }
                    ui.label("Aliases");
                    let mut aliases = field.aliases.join(", ");
                    if ui.text_edit_singleline(&mut aliases).changed() {
                        field.aliases = aliases
                            .split(',')
                            .map(str::trim)
                            .filter(|value| !value.is_empty())
                            .map(str::to_owned)
                            .collect();
                        *changed = true;
                    }
                    ui.end_row();
                    ui.label("Inheritance");
                    ui.vertical(|ui| {
                        ComboBox::from_id_salt("parameter-field-inheritance")
                            .selected_text(inheritance_label(field.inheritance))
                            .show_ui(ui, |ui| {
                                for value in ParameterInheritance::ALL {
                                    if ui
                                        .selectable_value(
                                            &mut field.inheritance,
                                            value,
                                            inheritance_label(value),
                                        )
                                        .changed()
                                    {
                                        *changed = true;
                                    }
                                }
                            });
                        ui.weak(inheritance_detail(field.inheritance));
                    });
                    ui.end_row();
                    ui.label("Visibility");
                    ComboBox::from_id_salt("parameter-field-visibility")
                        .selected_text(visibility_label(field.visibility))
                        .show_ui(ui, |ui| {
                            for value in [
                                SymbolParameterVisibility::Visible,
                                SymbolParameterVisibility::Advanced,
                                SymbolParameterVisibility::Hidden,
                            ] {
                                if ui
                                    .selectable_value(
                                        &mut field.visibility,
                                        value,
                                        visibility_label(value),
                                    )
                                    .changed()
                                {
                                    *changed = true;
                                }
                            }
                        });
                    ui.end_row();
                    ui.label("Required");
                    *changed |= ui.checkbox(&mut field.required, "Required value").changed();
                    ui.end_row();
                    ui.label("Device help");
                    *changed |= ui
                        .add_sized(
                            vec2(ui.available_width(), 62.0),
                            TextEdit::multiline(&mut field.help).desired_rows(3),
                        )
                        .changed();
                    ui.end_row();
                });
            for finding in diagnostics
                .iter()
                .filter(|finding| finding.field_key.as_deref() == Some(&field_key))
            {
                ui.colored_label(t.color.err, &finding.message);
            }
        });
    if field.key != original_field_key && !field.key.trim().is_empty() {
        state.selected_field = Some(field.key.clone());
    }
}
fn normalize_form_selection(
    form: &SymbolParameterForm,
    state: &mut super::SymbolParameterFormDialogState,
) {
    if state
        .selected_section
        .as_deref()
        .is_none_or(|key| !form.sections.iter().any(|section| section.key == key))
    {
        state.selected_section = form.sections.first().map(|section| section.key.clone());
    }
    if state
        .selected_field
        .as_deref()
        .is_some_and(|key| form.field(key).is_none())
    {
        state.selected_field = None;
    }
    if state.selected_field.is_none() {
        state.selected_field = selected_section(form, state)
            .and_then(|section| section.fields.first())
            .map(|field| field.key.clone());
    }
}

fn selected_section<'a>(
    form: &'a SymbolParameterForm,
    state: &super::SymbolParameterFormDialogState,
) -> Option<&'a SymbolParameterSection> {
    let key = state.selected_section.as_deref()?;
    form.sections.iter().find(|section| section.key == key)
}

fn selected_section_mut<'a>(
    form: &'a mut SymbolParameterForm,
    state: &super::SymbolParameterFormDialogState,
) -> Option<&'a mut SymbolParameterSection> {
    let key = state.selected_section.as_deref()?;
    form.sections.iter_mut().find(|section| section.key == key)
}

fn move_selected_field(
    form: &mut SymbolParameterForm,
    state: &super::SymbolParameterFormDialogState,
    delta: isize,
) -> bool {
    let Some(section_key) = state.selected_section.as_deref() else {
        return false;
    };
    let Some(field_key) = state.selected_field.as_deref() else {
        return false;
    };
    let Some(section) = form
        .sections
        .iter()
        .find(|section| section.key == section_key)
    else {
        return false;
    };
    let Some(index) = section
        .fields
        .iter()
        .position(|field| field.key == field_key)
    else {
        return false;
    };
    let destination = (index as isize + delta).clamp(0, section.fields.len() as isize - 1) as usize;
    destination != index && form.move_field(field_key, section_key, destination).is_ok()
}

fn move_selected_section(
    form: &mut SymbolParameterForm,
    state: &super::SymbolParameterFormDialogState,
    delta: isize,
) -> bool {
    let Some(section_key) = state.selected_section.as_deref() else {
        return false;
    };
    let Some(index) = form
        .sections
        .iter()
        .position(|section| section.key == section_key)
    else {
        return false;
    };
    let destination = (index as isize + delta).clamp(0, form.sections.len() as isize - 1) as usize;
    destination != index && form.move_section(section_key, destination).is_ok()
}

fn unique_section_key(form: &SymbolParameterForm) -> String {
    (1..)
        .map(|index| format!("section_{index}"))
        .find(|key| !form.sections.iter().any(|section| section.key == *key))
        .expect("finite form always has another section key")
}

fn unique_field_key(form: &SymbolParameterForm) -> String {
    (1..)
        .map(|index| format!("parameter_{index}"))
        .find(|key| form.field(key).is_none())
        .expect("finite form always has another field key")
}

fn default_parameter_field(key: &str) -> SymbolParameterField {
    SymbolParameterField {
        key: key.to_owned(),
        label: "New parameter".to_owned(),
        help: "Describe the parameter's electrical meaning and expected values.".to_owned(),
        property_type: PropertyType::Number,
        default: SymbolParameterDefault::Number {
            engineering: "0".to_owned(),
            unit: None,
        },
        unit: None,
        constraints: SymbolParameterConstraints::default(),
        inheritance: ParameterInheritance::InstanceOverride,
        visibility: SymbolParameterVisibility::Visible,
        required: false,
        aliases: Vec::new(),
    }
}

fn default_for_type(property_type: PropertyType) -> SymbolParameterDefault {
    match property_type {
        PropertyType::Number => SymbolParameterDefault::Number {
            engineering: "0".to_owned(),
            unit: None,
        },
        PropertyType::String => SymbolParameterDefault::String {
            value: String::new(),
        },
        PropertyType::Expression => SymbolParameterDefault::Expression {
            value: "0".to_owned(),
        },
        PropertyType::Enum => SymbolParameterDefault::Enum {
            selected: String::new(),
        },
        PropertyType::Boolean => SymbolParameterDefault::Boolean { value: false },
    }
}

fn default_editor(ui: &mut Ui, field: &mut SymbolParameterField) -> bool {
    match &mut field.default {
        SymbolParameterDefault::Number { engineering, .. }
        | SymbolParameterDefault::String { value: engineering }
        | SymbolParameterDefault::Expression { value: engineering }
        | SymbolParameterDefault::Enum {
            selected: engineering,
        } => ui.text_edit_singleline(engineering).changed(),
        SymbolParameterDefault::Boolean { value } => ui.checkbox(value, "Enabled").changed(),
    }
}

fn optional_text(ui: &mut Ui, value: &mut Option<String>) -> bool {
    let mut text = value.clone().unwrap_or_default();
    let changed = ui.text_edit_singleline(&mut text).changed();
    if changed {
        *value = (!text.trim().is_empty()).then(|| text.trim().to_owned());
    }
    changed
}

fn property_type_label(value: PropertyType) -> &'static str {
    match value {
        PropertyType::Number => "Engineering number",
        PropertyType::String => "Text",
        PropertyType::Expression => "Expression",
        PropertyType::Enum => "Enumeration",
        PropertyType::Boolean => "Boolean",
    }
}

fn inheritance_label(value: ParameterInheritance) -> &'static str {
    match value {
        ParameterInheritance::InstanceOverride => "Instance override",
        ParameterInheritance::CellDefault => "Cell default",
        ParameterInheritance::ModelDefault => "Model default",
    }
}

fn inheritance_detail(value: ParameterInheritance) -> &'static str {
    match value {
        ParameterInheritance::InstanceOverride => "instance-editable \u{00b7} emitted in netlist",
        ParameterInheritance::CellDefault => {
            "cell-owned \u{00b7} read-only at instance \u{00b7} emitted"
        }
        ParameterInheritance::ModelDefault => {
            "model-owned \u{00b7} read-only \u{00b7} omitted from instance"
        }
    }
}

fn visibility_label(value: SymbolParameterVisibility) -> &'static str {
    match value {
        SymbolParameterVisibility::Visible => "Visible",
        SymbolParameterVisibility::Advanced => "Advanced",
        SymbolParameterVisibility::Hidden => "Hidden by default",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_and_field_reordering_use_the_typed_form_operations() {
        let mut form = SymbolParameterForm {
            revision: 1,
            sections: vec![
                SymbolParameterSection {
                    key: "first".to_owned(),
                    label: "First".to_owned(),
                    help: String::new(),
                    fields: vec![default_parameter_field("a"), default_parameter_field("b")],
                },
                SymbolParameterSection {
                    key: "second".to_owned(),
                    label: "Second".to_owned(),
                    help: String::new(),
                    fields: Vec::new(),
                },
            ],
        };
        let mut state = super::super::SymbolParameterFormDialogState {
            selected_section: Some("first".to_owned()),
            selected_field: Some("a".to_owned()),
            ..Default::default()
        };

        assert!(move_selected_field(&mut form, &state, 1));
        assert_eq!(form.sections[0].fields[1].key, "a");
        assert!(move_selected_section(&mut form, &state, 1));
        assert_eq!(form.sections[1].key, "first");

        normalize_form_selection(&form, &mut state);
        assert_eq!(state.selected_section.as_deref(), Some("first"));
        assert_eq!(state.selected_field.as_deref(), Some("a"));
    }
}
