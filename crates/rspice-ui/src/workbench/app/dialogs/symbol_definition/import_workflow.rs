use egui::{Align, ComboBox, Context, Frame, Grid, Layout, Margin, Stroke, TextEdit, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{CellViewRef, ModelBoundSymbolDefinition, SymbolDefinitionImport};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use super::source_picker;
use super::state::SymbolImportBindingChoice;
use crate::workbench::RSpiceApp;
use crate::workbench::app::publish_symbol_definition_candidate;

const IMPORT_EYEBROW: &str = "SYMBOL LIBRARY \u{00b7} VALIDATED INTERCHANGE";
const IMPORT_TITLE: &str = "Import symbol definition";
const DISCARD_TITLE: &str = "Discard unsaved symbol changes?";
const DISCARD_DETAIL: &str =
    "The isolated draft will be removed. The project library has not been changed.";

impl RSpiceApp {
    pub(super) fn render_symbol_import_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.symbol_import.open {
            return;
        }
        suggest_target_name(&mut self.state.dialogs.symbol_import);
        let binding_options = bound_symbol_references(&self.state);
        let writable_libraries = self
            .state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .filter(|library| !library.read_only)
            .map(|library| library.name.clone())
            .collect::<Vec<_>>();
        if self.state.dialogs.symbol_import.binding_source.is_none() {
            self.state.dialogs.symbol_import.binding_source = binding_options.first().cloned();
        }
        let preview = import_candidate(&self.state).map_err(|error| error.to_string());
        let discard_confirm = self.state.dialogs.symbol_import.discard_confirm;
        let validation_error = self
            .state
            .dialogs
            .symbol_import
            .validation_error
            .as_deref()
            .or(self.state.dialogs.symbol_import.source_error.as_deref())
            .map(str::to_owned)
            .or_else(|| {
                (!self.state.dialogs.symbol_import.source_text.is_empty())
                    .then(|| preview.as_ref().err().cloned())
                    .flatten()
            });
        let primary_enabled = preview.is_ok();
        let mut dialog = Dialog::new(IMPORT_EYEBROW, IMPORT_TITLE, "Import into project library")
            .description("Import validated RSpice JSON, SVG, EDIF, or LTspice symbol geometry with an explicit electrical contract or as a review-only graphic.")
            .size(DialogSize::SimulationWorkflow)
            .initial_height(468.0)
            .initial_focus(DialogInitialFocus::BodyControl)
            .ghost(if discard_confirm { "Discard changes" } else { "Cancel" })
            .primary_enabled(primary_enabled);
        if self.state.dialogs.symbol_import.dirty && !discard_confirm {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if let Some(error) = validation_error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Symbol definition is not ready",
                error,
            );
        }

        let mut browse = false;
        let mut changed = false;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let first = import_body(
                ui,
                &mut self.state.dialogs.symbol_import,
                &writable_libraries,
                &binding_options,
                preview.as_ref().ok(),
                &mut browse,
                &mut changed,
            );
            Some(first)
        });
        if browse {
            source_picker::pick_symbol_source(&mut self.state.dialogs.symbol_import);
        }
        if changed {
            self.state.dialogs.symbol_import.dirty = true;
            self.state.dialogs.symbol_import.discard_confirm = false;
            self.state.dialogs.symbol_import.validation_error = None;
        }
        match choice {
            DialogChoice::Primary => self.commit_symbol_import(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if self.state.dialogs.symbol_import.dirty
                    && !self.state.dialogs.symbol_import.discard_confirm
                {
                    self.state.dialogs.symbol_import.discard_confirm = true;
                } else {
                    self.state.dialogs.symbol_import.close_and_discard();
                }
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn commit_symbol_import(&mut self) {
        self.state.dialogs.symbol_import.validation_error = None;
        let imported = match import_candidate(&self.state) {
            Ok(imported) => imported,
            Err(error) => {
                self.state.dialogs.symbol_import.validation_error = Some(error.to_string());
                return;
            }
        };
        let library_name = self.state.dialogs.symbol_import.target_library.clone();
        let cell_name = self
            .state
            .dialogs
            .symbol_import
            .target_name
            .trim()
            .to_owned();
        let mut candidate = self.state.library_manager.clone();
        let result = (|| {
            let library = candidate
                .get_library_mut(&library_name)
                .ok_or_else(|| format!("Target library '{library_name}' no longer exists."))?;
            let plan = imported
                .definition
                .build_plan(library)
                .map_err(|error| error.to_string())?;
            plan.commit(library).map_err(|error| error.to_string())?;
            publish_symbol_definition_candidate(
                &mut self.state,
                candidate,
                &library_name,
                &cell_name,
                format!("import symbol definition '{cell_name}'"),
            )?;
            Ok::<(), String>(())
        })();
        match result {
            Ok(()) => {
                self.state
                    .library_manager
                    .select_view(&library_name, &cell_name, "symbol");
                let message = if imported
                    .definition
                    .source
                    .is_explicitly_unbound_for_review()
                {
                    format!(
                        "Imported {cell_name} from {} as a review-only vector graphic; placement and netlisting remain unavailable until it receives an explicit electrical contract",
                        imported.report.format_label()
                    )
                } else {
                    format!(
                        "Imported {cell_name} from {} with validated symbol, pin, parameter-form, and netlist contracts",
                        imported.report.format_label()
                    )
                };
                self.state.push_user_message(ConsoleMessage::info(message));
                self.state.dialogs.symbol_import.close_and_discard();
            }
            Err(error) => self.state.dialogs.symbol_import.validation_error = Some(error),
        }
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
                        .font(egui::TextStyle::Monospace)
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
            ComboBox::from_id_salt("symbol-import-target-library")
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
                    .font(egui::TextStyle::Monospace)
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
                SymbolImportBindingChoice::UnboundForReview => {
                    "Create unbound symbol for review".to_owned()
                }
            };
            ComboBox::from_id_salt("symbol-import-pin-contract")
                .selected_text(selected)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
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
            Grid::new("symbol-import-checks")
                .num_columns(3)
                .striped(true)
                .min_col_width(130.0)
                .show(ui, |ui| {
                    table_header(ui, "CHECK");
                    table_header(ui, "OBSERVED");
                    table_header(ui, "STATUS");
                    ui.end_row();
                    let rows = preview.map_or_else(
                        || {
                            vec![
                                (
                                    "Vector geometry",
                                    "not inspected".to_owned(),
                                    "choose source",
                                ),
                                ("Pin anchors", "not inspected".to_owned(), "choose contract"),
                                ("Pin order", "not inspected".to_owned(), "review"),
                            ]
                        },
                        |import| {
                            vec![
                                (
                                    "Vector geometry",
                                    format!("{} primitives", import.report.primitive_count),
                                    "valid",
                                ),
                                (
                                    "Pin anchors",
                                    format!("{} explicit", import.report.explicit_pin_anchor_count),
                                    if import.report.explicit_pin_anchor_count == 0 {
                                        "review-only"
                                    } else {
                                        "valid"
                                    },
                                ),
                                (
                                    "Pin order",
                                    if import.report.pin_order_valid {
                                        "matches explicit contract".to_owned()
                                    } else {
                                        "unbound".to_owned()
                                    },
                                    if import.report.pin_order_valid {
                                        "valid"
                                    } else {
                                        "review"
                                    },
                                ),
                            ]
                        },
                    );
                    for (check, observed, status) in rows {
                        ui.label(check);
                        ui.monospace(observed);
                        ui.colored_label(
                            if status == "valid" {
                                t.color.ok
                            } else {
                                t.color.warn
                            },
                            status,
                        );
                        ui.end_row();
                    }
                });
        });
}
fn import_candidate(
    state: &crate::workbench::app::AppState,
) -> Result<SymbolDefinitionImport, String> {
    let draft = &state.dialogs.symbol_import;
    if draft.source_text.is_empty() {
        return Err("Choose a symbol definition source.".to_owned());
    }
    let target_name = draft.target_name.trim();
    if target_name.is_empty() {
        return Err("Target name is required.".to_owned());
    }
    if state
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
                let view = state
                    .library_manager
                    .get_library(&reference.library)
                    .and_then(|library| library.get_cell(&reference.cell))
                    .and_then(|cell| cell.get_view(&reference.view))
                    .ok_or_else(|| "The selected pin/model contract is unavailable.".to_owned())?;
                let mut definition = ModelBoundSymbolDefinition::load_from_view(view)
                    .map_err(|error| error.to_string())?
                    .ok_or_else(|| {
                        "The selected symbol has no typed pin/model contract.".to_owned()
                    })?;
                retarget_definition(&mut definition, &draft.target_library, target_name);
                Some(definition)
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

fn bound_symbol_references(state: &crate::workbench::app::AppState) -> Vec<CellViewRef> {
    let mut references = Vec::new();
    for library in state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell.views_sorted() {
                if view.view_type == crate::state::ViewType::Symbol
                    && matches!(
                        ModelBoundSymbolDefinition::load_from_view(view),
                        Ok(Some(_))
                    )
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

    #[test]
    fn review_only_svg_import_keeps_real_geometry_without_an_executable_contract() {
        let mut app = crate::workbench::app::AppState::default();
        app.project_lifecycle.project_open = true;
        app.dialogs.symbol_import.source_name = "review.svg".to_owned();
        app.dialogs.symbol_import.source_text =
            r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="40" height="20"/></svg>"#
                .to_owned();
        app.dialogs.symbol_import.target_library = app.workspace.active_view.library.clone();
        app.dialogs.symbol_import.target_name = "review_graphic".to_owned();
        app.dialogs.symbol_import.binding_choice = SymbolImportBindingChoice::UnboundForReview;

        let imported = import_candidate(&app).expect("review-only SVG import");

        assert!(
            imported
                .definition
                .source
                .is_explicitly_unbound_for_review()
        );
        assert!(imported.definition.netlist.template.is_empty());
        assert_eq!(imported.report.primitive_count, 1);
        assert_eq!(
            imported
                .definition
                .imported_graphic
                .as_ref()
                .expect("retained imported geometry")
                .shapes
                .len(),
            1
        );
    }

    #[test]
    fn import_candidate_never_overwrites_an_existing_cell() {
        let mut app = crate::workbench::app::AppState::default();
        app.project_lifecycle.project_open = true;
        let library_name = app.workspace.active_view.library.clone();
        app.dialogs.symbol_import.source_name = "review.svg".to_owned();
        app.dialogs.symbol_import.source_text =
            r#"<svg><line x1="0" y1="0" x2="10" y2="10"/></svg>"#.to_owned();
        app.dialogs.symbol_import.target_library = library_name.clone();
        app.dialogs.symbol_import.target_name = "existing".to_owned();
        app.dialogs.symbol_import.binding_choice = SymbolImportBindingChoice::UnboundForReview;
        app.library_manager
            .get_library_mut(&library_name)
            .expect("project library")
            .add_cell(crate::state::Cell::new("existing"));

        assert!(
            import_candidate(&app)
                .expect_err("overwrite must fail closed")
                .contains("already exists")
        );
    }
}
