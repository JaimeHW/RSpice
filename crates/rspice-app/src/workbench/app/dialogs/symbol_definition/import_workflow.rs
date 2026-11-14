//! Importing a symbol definition.

use egui::{Align, Context, Frame, Grid, Layout, Margin, Stroke, TextEdit, Ui, vec2};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, SymbolDefinitionImport, SymbolSourceContract,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use super::SymbolImportDialogState;
use super::source_picker;
use super::state::SymbolImportBindingChoice;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::publish_symbol_definition_candidate;

const IMPORT_TITLE: &str = "Import symbol definition";
const DISCARD_TITLE: &str = "Discard unsaved symbol changes?";
const DISCARD_DETAIL: &str =
    "The isolated draft will be removed. The project library has not been changed.";

pub(super) fn render_symbol_import_dialog(
    symbol_import: &mut SymbolImportDialogState,
    model: &mut Model,
    ctx: &Context,
) {
    if !symbol_import.open {
        return;
    }
    suggest_target_name(&mut *symbol_import);
    let binding_options = bound_symbol_references(model);
    let writable_libraries = model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| !library.read_only)
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    if symbol_import.binding_source.is_none() {
        symbol_import.binding_source = binding_options.first().cloned();
    }
    let preview = import_candidate(symbol_import, model).map_err(|error| error.to_string());
    let discard_confirm = symbol_import.discard_confirm;
    let validation_error = symbol_import
        .validation_error
        .as_deref()
        .or(symbol_import.source_error.as_deref())
        .map(str::to_owned)
        .or_else(|| {
            (!symbol_import.source_text.is_empty())
                .then(|| preview.as_ref().err().cloned())
                .flatten()
        });
    let primary_enabled = preview.is_ok();
    let mut dialog = Dialog::new(IMPORT_TITLE, "Import into project library")
        .description("Import validated RSpice JSON, SVG, EDIF, or LTspice symbol geometry with an explicit electrical contract or as a review-only graphic.")
        .size(DialogSize::SimulationWorkflow)
        .initial_height(468.0)
        .initial_focus(DialogInitialFocus::BodyControl)
        .ghost(if discard_confirm { "Discard changes" } else { "Cancel" })
        .primary_enabled(primary_enabled);
    if discard_confirm {
        dialog =
            dialog.transaction_state(DialogTransactionTone::Error, DISCARD_TITLE, DISCARD_DETAIL);
    } else if let Some(error) = validation_error.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Symbol definition is not ready",
            error,
        );
    }

    let mut browse = false;
    let mut changed = false;
    let mut response = dialog.show_transaction(ctx, |ui| {
        let first = import_body(
            ui,
            &mut *symbol_import,
            &writable_libraries,
            &binding_options,
            preview.as_ref().ok(),
            &mut browse,
            &mut changed,
        );
        Some(first)
    });
    if browse {
        source_picker::pick_symbol_source(&mut *symbol_import);
    }
    if changed {
        symbol_import.dirty = true;
        symbol_import.discard_confirm = false;
        symbol_import.validation_error = None;
    }
    match response.choice {
        DialogChoice::Primary => commit_symbol_import(symbol_import, model),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if symbol_import.dirty && !symbol_import.discard_confirm {
                symbol_import.discard_confirm = true;
                response.retain_cancel_focus(DialogInitialFocus::Ghost);
            } else {
                symbol_import.close_and_discard();
            }
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

fn commit_symbol_import(symbol_import: &mut SymbolImportDialogState, model: &mut Model) {
    symbol_import.validation_error = None;
    let imported = match import_candidate(symbol_import, model) {
        Ok(imported) => imported,
        Err(error) => {
            symbol_import.validation_error = Some(error.to_string());
            return;
        }
    };
    let library_name = symbol_import.target_library.clone();
    let cell_name = symbol_import.target_name.trim().to_owned();
    let mut candidate = model.library_manager.clone();
    let result = (|| {
        candidate
            .edit_library(&library_name)
            .ok_or_else(|| format!("Target library '{library_name}' no longer exists."))?
            .construct_symbol(&imported.definition)
            .map_err(|error| error.to_string())?;
        publish_symbol_definition_candidate(
            &mut *model,
            candidate,
            &library_name,
            &cell_name,
            format!("import symbol definition '{cell_name}'"),
        )?;
        Ok::<(), String>(())
    })();
    match result {
        Ok(()) => {
            model
                .library_manager
                .select_view(&library_name, &cell_name, "symbol");
            let format = imported.report.format_label();
            if imported
                .definition
                .source
                .is_explicitly_unbound_for_review()
            {
                model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    format!(
                        "Imported symbol {cell_name} from {format} as graphics only; it \
                         cannot be placed or netlisted until its pins are bound."
                    ),
                );
            } else {
                model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Imported symbol {cell_name} from {format}."),
                );
            }
            symbol_import.close_and_discard();
        }
        Err(error) => symbol_import.validation_error = Some(error),
    }
}

fn import_body(
    ui: &mut Ui,
    state: &mut super::SymbolImportDialogState,
    writable_libraries: &[String],
    binding_options: &[CellViewRef],
    preview: Option<&SymbolDefinitionImport>,
    browse: &mut bool,
    changed: &mut bool,
) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 8.0;
    let source_response = setting_row(
        ui,
        "Source",
        "RSpice JSON, SVG, EDIF 2 0 0, or LTspice .asy.",
        |ui| {
            ui.horizontal(|ui| {
                let width = (ui.available_width() - 92.0).max(120.0);
                let response = ui.add_sized(
                    vec2(width, t.metrics.ctl_h),
                    TextEdit::singleline(&mut state.source_name)
                        .interactive(false)
                        .font(theme::field_mono())
                        .hint_text("Choose a symbol definition"),
                );
                if Button::new("Browse...").show(ui).clicked() {
                    *browse = true;
                }
                response
            })
            .inner
        },
    );
    setting_row(
        ui,
        "Target library",
        "Writable project-library destination.",
        |ui| {
            crate::ui::widgets::Combo::from_id_salt("symbol-import-target-library")
                .selected_text(&state.target_library)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for library in writable_libraries {
                        if ui
                            .selectable_value(&mut state.target_library, library.clone(), library)
                            .changed()
                        {
                            *changed = true;
                        }
                    }
                });
        },
    );
    let target = setting_row(
        ui,
        "Target name",
        "Stable project-library identifier.",
        |ui| {
            ui.add_sized(
                vec2(ui.available_width(), t.metrics.ctl_h),
                TextEdit::singleline(&mut state.target_name)
                    .font(theme::field_mono())
                    .hint_text("opamp_5pin_variant"),
            )
        },
    );
    *changed |= target.changed();
    setting_row(
        ui,
        "Pin contract",
        "Geometry never infers electrical semantics.",
        |ui| {
            let selected = match state.binding_choice {
                SymbolImportBindingChoice::ExplicitModelContract => state
                    .binding_source
                    .as_ref()
                    .map(CellViewRef::display_path)
                    .unwrap_or_else(|| "Choose an explicit bound definition".to_owned()),
                SymbolImportBindingChoice::SymbolAttributes => preview
                    .and_then(|import| match &import.definition.source {
                        SymbolSourceContract::Model { model, .. } => Some(format!(
                            "{} / {} (named by the symbol)",
                            model.library, model.model
                        )),
                        _ => None,
                    })
                    .unwrap_or_else(|| "Model the symbol names".to_owned()),
                SymbolImportBindingChoice::UnboundForReview => {
                    "Create unbound symbol for review".to_owned()
                }
            };
            let names_its_model = super::state::is_ltspice_symbol(&state.source_name);
            crate::ui::widgets::Combo::from_id_salt("symbol-import-pin-contract")
                .selected_text(selected)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    if names_its_model
                        && ui
                            .selectable_value(
                                &mut state.binding_choice,
                                SymbolImportBindingChoice::SymbolAttributes,
                                "Model the symbol names",
                            )
                            .changed()
                    {
                        *changed = true;
                    }
                    for reference in binding_options {
                        let response = ui.selectable_label(
                            state.binding_choice
                                == SymbolImportBindingChoice::ExplicitModelContract
                                && state.binding_source.as_ref() == Some(reference),
                            reference.display_path(),
                        );
                        if response.clicked() {
                            state.binding_choice = SymbolImportBindingChoice::ExplicitModelContract;
                            state.binding_source = Some(reference.clone());
                            *changed = true;
                        }
                    }
                    if ui
                        .selectable_value(
                            &mut state.binding_choice,
                            SymbolImportBindingChoice::UnboundForReview,
                            "Create unbound symbol for review",
                        )
                        .changed()
                    {
                        *changed = true;
                    }
                });
        },
    );
    ui.add_space(2.0);
    check_table(ui, preview);
    source_response.id
}

fn check_table(ui: &mut Ui, preview: Option<&SymbolDefinitionImport>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            // A Status column stood beside Observed and was derived from it:
            // "12 primitives" was "valid", "0 explicit" was "review-only",
            // "unbound" was "review". What was observed is the status.
            Grid::new("symbol-import-checks")
                .num_columns(2)
                .striped(true)
                .min_col_width(130.0)
                .show(ui, |ui| {
                    table_header(ui, "Check");
                    table_header(ui, "Observed");
                    ui.end_row();
                    let rows = preview.map_or_else(
                        || {
                            vec![
                                ("Vector geometry", "not inspected".to_owned()),
                                ("Pin anchors", "not inspected".to_owned()),
                                ("Pin order", "not inspected".to_owned()),
                            ]
                        },
                        |import| {
                            vec![
                                (
                                    "Vector geometry",
                                    format!("{} primitives", import.report.primitive_count),
                                ),
                                (
                                    "Pin anchors",
                                    format!("{} explicit", import.report.explicit_pin_anchor_count),
                                ),
                                (
                                    "Pin order",
                                    if import.report.pin_order_valid {
                                        "matches explicit contract".to_owned()
                                    } else {
                                        "unbound".to_owned()
                                    },
                                ),
                            ]
                        },
                    );
                    for (check, observed) in rows {
                        ui.label(check);
                        ui.monospace(observed);
                        ui.end_row();
                    }
                });
        });
}
pub(crate) fn import_candidate(
    draft: &SymbolImportDialogState,
    model: &Model,
) -> Result<SymbolDefinitionImport, String> {
    if draft.source_text.is_empty() {
        return Err("Choose a symbol definition source.".to_owned());
    }
    let target_name = draft.target_name.trim();
    if target_name.is_empty() {
        return Err("Target name is required.".to_owned());
    }
    if model
        .library_manager
        .get_library(&draft.target_library)
        .and_then(|library| library.get_cell(target_name))
        .is_some()
    {
        return Err(format!(
            "Target cell '{}/{}' already exists; choose a new stable name.",
            draft.target_library, target_name
        ));
    }
    let canonical_json = source_looks_like_json(&draft.source_name, &draft.source_text);
    let explicit = if canonical_json {
        None
    } else {
        match draft.binding_choice {
            SymbolImportBindingChoice::ExplicitModelContract => {
                let reference = draft.binding_source.as_ref().ok_or_else(|| {
                    "Bound SVG, EDIF and LTspice geometry require an existing explicit pin/model/netlist contract."
                        .to_owned()
                })?;
                let view = model
                    .library_manager
                    .get_library(&reference.library)
                    .and_then(|library| library.get_cell(&reference.cell))
                    .and_then(|cell| cell.get_view(&reference.view))
                    .ok_or_else(|| "The selected pin/model contract is unavailable.".to_owned())?;
                let mut definition = crate::state::load_model_bound_symbol(view)
                    .map_err(|error| error.to_string())?
                    .ok_or_else(|| {
                        "The selected symbol has no typed pin/model contract.".to_owned()
                    })?;
                retarget_definition(&mut definition, &draft.target_library, target_name);
                Some(definition)
            }
            SymbolImportBindingChoice::SymbolAttributes => {
                let graphic = rspice_model_library::symbol::ImportedGraphicSource::from_bytes(
                    rspice_model_library::symbol::ImportedGraphicFormat::LtspiceAsy,
                    &draft.source_name,
                    draft.source_text.as_bytes(),
                )
                .map_err(|error| error.to_string())?;
                Some(
                    super::super::create_model_bound_symbol::symbol_attribute_definition(
                        model,
                        &graphic,
                        &draft.target_library,
                        target_name,
                    )?,
                )
            }
            SymbolImportBindingChoice::UnboundForReview => Some(
                ModelBoundSymbolDefinition::review_only(&draft.target_library, target_name),
            ),
        }
    };
    let mut imported = SymbolDefinitionImport::from_bytes(
        draft.source_text.as_bytes(),
        &draft.source_name,
        explicit,
    )
    .map_err(|error| error.to_string())?;
    retarget_definition(&mut imported.definition, &draft.target_library, target_name);
    imported
        .definition
        .validate()
        .map_err(|error| error.to_string())?;
    Ok(imported)
}

fn retarget_definition(definition: &mut ModelBoundSymbolDefinition, library: &str, cell: &str) {
    definition.identity.library = library.to_owned();
    definition.identity.cell = cell.to_owned();
    definition.identity.revision = 1;
    definition.identity.binding_id = format!("symbol:{library}:{cell}:1");
}

fn source_looks_like_json(name: &str, source: &str) -> bool {
    name.rsplit_once('.').is_some_and(|(_, extension)| {
        extension.eq_ignore_ascii_case("json") || extension.eq_ignore_ascii_case("rspicesym")
    }) || source.trim_start().starts_with('{')
}

fn bound_symbol_references(model: &Model) -> Vec<CellViewRef> {
    let mut references = Vec::new();
    for library in model.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell.views_sorted() {
                if view.view_type == crate::state::ViewType::Symbol
                    && matches!(crate::state::load_model_bound_symbol(view), Ok(Some(_)))
                {
                    references.push(CellViewRef::new(&library.name, &cell.name, &view.name));
                }
            }
        }
    }
    references
}

fn suggest_target_name(state: &mut super::SymbolImportDialogState) {
    if !state.target_name.is_empty() || state.source_name.is_empty() {
        return;
    }
    let filename = state
        .source_name
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(&state.source_name);
    let stem = filename.rsplit_once('.').map_or(filename, |(stem, _)| stem);
    state.target_name = stem
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect();
}
pub(super) fn setting_row<R>(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    content: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .inner_margin(Margin::symmetric(0, 5))
        .show(ui, |ui| {
            let width = ui.available_width();
            let label_width = (width * 0.38).clamp(190.0, 280.0);
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    vec2(label_width, t.metrics.ctl_h + 13.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.label(
                            egui::RichText::new(title)
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text),
                        );
                        ui.label(
                            egui::RichText::new(detail)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    },
                );
                ui.add_space(8.0);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), t.metrics.ctl_h + 13.0),
                    Layout::top_down_justified(Align::Min),
                    content,
                )
                .inner
            })
            .inner
        })
        .inner
}

pub(super) fn table_header(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggested_target_name_is_portable_and_stable() {
        let mut state = super::super::SymbolImportDialogState {
            source_name: "OPA 189+variant.svg".to_owned(),
            ..Default::default()
        };
        suggest_target_name(&mut state);
        assert_eq!(state.target_name, "OPA_189_variant");
    }
}
