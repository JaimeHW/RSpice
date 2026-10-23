//! Context inspector with authoritative object and provenance details.

mod code_workspace;
mod design;
mod executed_deck;
mod models;
mod project;
mod result_authority;
mod results;
/// Visible to the dock layer: the data browser opens it from a run's row, and
/// the frame draws it whichever dock is showing.
pub(super) mod run_details;
mod simulation;
/// Visible to the dock layer so the library browser's own gate can render this
/// pane beside it: the two docks state the same counts about the same design,
/// and a review that only ever opened one of them would not see them disagree.
pub(super) mod stimulus;
mod symbol;
mod verify;
mod veriloga;

use code_workspace::code_workspace_inspector;
use project::project;
use results::results;
use verify::verify;

/// The SOA verdict the Verify flow states is the one an instance's Checks row
/// states, read the same way.
pub(super) use design::analysis_soa_display;

use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{PANEL_INSET, Problem, panel_body, section_block_gap, section_header};

use egui::{ScrollArea, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId, RSpiceApp};

use super::super::design_system::PANEL_HEADER_H;
use super::super::state::{ProjectPage, VerificationPage, Workspace};

const INSPECTOR_SCROLL_HISTORY_LIMIT: usize = 64;

#[derive(Clone, Debug, Default)]
struct InspectorScrollMemory {
    active: Option<String>,
    /// Recent `(subject identity, vertical offset)` entries, oldest first.
    offsets: Vec<(String, f32)>,
}

impl InspectorScrollMemory {
    fn begin_subject(&mut self, identity: &str) -> Option<f32> {
        if self.active.as_deref() == Some(identity) {
            return None;
        }
        self.active = Some(identity.to_owned());
        Some(
            self.offsets
                .iter()
                .find(|(subject, _)| subject == identity)
                .map_or(0.0, |(_, offset)| *offset),
        )
    }

    fn record(&mut self, identity: String, offset: f32) {
        if let Some(index) = self
            .offsets
            .iter()
            .position(|(subject, _)| subject == &identity)
        {
            self.offsets.remove(index);
        }
        self.offsets.push((identity, offset.max(0.0)));
        if self.offsets.len() > INSPECTOR_SCROLL_HISTORY_LIMIT {
            self.offsets.remove(0);
        }
    }
}

fn inspector_scroll_memory_id() -> egui::Id {
    egui::Id::new("workbench.inspector.scroll-memory")
}

/// Validate the exact catalog binding without changing the schematic.
///
/// The Models workspace uses this to enable its action and the transaction
/// repeats it immediately before mutation, so presentation and execution
/// cannot disagree about model family, polarity, or provider authority.
pub(crate) fn validate_component_model_catalog_binding(
    state: &AppState,
    component_id: u64,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    let component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    let library = state
        .model
        .model_library_manager
        .get_library(library_name)
        .ok_or_else(|| format!("Model library '{library_name}' is no longer loaded."))?;
    // The card of that name, or every bin of the family it names: an instance
    // naming a binned family can run any of them, so each must suit it.
    let candidates = library.executable_cards(model_name);
    if candidates.is_empty() {
        return Err(format!(
            "Model '{model_name}' is no longer present in library '{library_name}'."
        ));
    }

    if let Some(binding) = component.library_cell.as_ref() {
        if !binding.library.eq_ignore_ascii_case(library_name) {
            return Err(format!(
                "The selected cell instance is bound to library '{}'; cross-library rebinding requires the Library/Cellview Manager.",
                binding.library
            ));
        }
        let current_name = binding
            .module_name
            .as_deref()
            .unwrap_or(binding.cell.as_str());
        let current = library
            .executable_cards(current_name)
            .into_iter()
            .next()
            .ok_or_else(|| {
                format!(
                    "The selected cell's current model '{current_name}' no longer resolves in library '{library_name}'."
                )
            })?;
        if !candidates.iter().all(|candidate| {
            crate::state::model_library::models_have_compatible_device_family(current, candidate)
        }) {
            return Err(format!(
                "Model '{model_name}' is incompatible with the selected cell's current model family."
            ));
        }
        return Ok(());
    }

    for candidate in &candidates {
        crate::state::model_library::validate_component_model_compatibility(
            component.kind,
            candidate,
        )?;
    }
    let effective = state
        .model
        .model_library_manager
        .effective_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            model_name,
        )?;
    let effective = effective
        .ok_or_else(|| format!("Model '{model_name}' has no executable catalog provider."))?;
    if !effective.library.eq_ignore_ascii_case(library_name) {
        return Err(format!(
            "Model '{model_name}' executes from project-global provider '{}', not selected library '{library_name}'. Resolve the project-global provider before binding.",
            effective.library
        ));
    }
    Ok(())
}

/// Bind one selected schematic instance from the Models & PDKs workbench
/// through the same guarded schematic transaction used by the inspector.
/// Library-cell instances retain their exact library identity; primitive
/// instances retain provider provenance as editor metadata, while the binding
/// is accepted only when it agrees with the project-global executable provider.
pub(crate) fn bind_component_model_from_catalog(
    app: &mut RSpiceApp,
    component_id: u64,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    app.state.commit_inline_component_edit()?;
    validate_component_model_catalog_binding(&app.state, component_id, library_name, model_name)?;
    let component = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    if let Some(binding) = component.library_cell.as_ref() {
        if !binding.library.eq_ignore_ascii_case(library_name) {
            return Err(format!(
                "The selected cell instance is bound to library '{}'; cross-library rebinding requires the Library/Cellview Manager.",
                binding.library
            ));
        }
        design::apply_bound_model_choice(app, component_id, model_name)?;
        return Ok(());
    }

    let effective = app
        .state
        .model
        .model_library_manager
        .effective_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            model_name,
        )?
        .ok_or_else(|| format!("Model '{model_name}' has no executable catalog provider."))?;
    if !effective.library.eq_ignore_ascii_case(library_name) {
        return Err(format!(
            "Model '{model_name}' executes from project-global provider '{}', not selected library '{library_name}'. Resolve the project-global provider before binding.",
            effective.library
        ));
    }

    let params =
        crate::state::params_string::set_parameter_value(&component.params, "model", model_name)
            .and_then(|text| {
                crate::state::params_string::set_parameter_value(
                    &text,
                    "model_library",
                    &effective.library,
                )
            })?;
    let mut candidate = component.clone();
    candidate.params = params;
    app.state
        .model
        .edit_component_transaction(&component, candidate, "bind instance model")?;
    app.invalidate_simulation_preflight();
    Ok(())
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.spacing_mut().item_spacing.y = 0.0;
    header(ui, app);
    let scroll_identity = inspector_scroll_identity(app);
    let mut scroll_memory = ui
        .ctx()
        .data_mut(|data| data.get_temp::<InspectorScrollMemory>(inspector_scroll_memory_id()))
        .unwrap_or_default();
    let requested_offset = scroll_memory.begin_subject(&scroll_identity);
    // The ScrollArea widget ID must remain stable across egui's sizing and
    // paint passes. Subject-specific IDs can change after a pointer selection
    // inside the same frame, assigning two IDs to one scrollbar rectangle.
    // Preserve per-subject positions in the bounded cache above instead.
    let scroll_area = ScrollArea::vertical()
        .id_salt("workbench.inspector.scroll")
        .auto_shrink([false, false]);
    let scroll_area = if let Some(offset) = requested_offset {
        scroll_area.vertical_scroll_offset(offset)
    } else {
        scroll_area
    };
    let output = scroll_area.show(ui, |ui| {
        panel_body(ui, |ui| inspector_body(ui, app));
    });
    scroll_memory.record(scroll_identity, output.state.offset.y);
    ui.ctx().data_mut(|data| {
        data.insert_temp(inspector_scroll_memory_id(), scroll_memory);
    });
}

/// The inspected subject's sections, laid out on the panel body.
fn inspector_body(ui: &mut Ui, app: &mut RSpiceApp) {
    if split_selected_trace_is_inspected(app) {
        results(ui, app);
        return;
    }
    match app.state.workbench.workspace {
        Workspace::Project => project(ui, app),
        // A symbol cellview is edited against its pin contract, not against a
        // schematic selection.
        Workspace::Design
            if app.state.model.workspace.content.active_view_type()
                == crate::state::ViewType::Symbol =>
        {
            symbol::show(ui, &mut app.state);
        }
        // A Verilog-A module is inspected for what it compiles to, not for a
        // schematic selection it does not have.
        Workspace::Design
            if app.state.model.workspace.content.active_view_type()
                == crate::state::ViewType::VerilogA =>
        {
            veriloga::show(ui, app);
        }
        Workspace::Design => design::show(ui, app),
        Workspace::Simulate => simulation::simulate(ui, app),
        Workspace::Stimulus => stimulus::show(ui, &mut app.state),
        Workspace::Results => results(ui, app),
        Workspace::Verify => verify(ui, app),
        Workspace::Models => models::show(ui, app),
        Workspace::Netlist => code_workspace_inspector(ui, app),
    }
}

fn inspector_scroll_identity(app: &RSpiceApp) -> String {
    let route = app.state.workbench.current_route();
    if app.state.workbench.workspace != Workspace::Design {
        return route.to_string();
    }

    format!(
        "{}|{}|{:?}",
        route,
        app.state.model.workspace.content.active_display_path(),
        app.state.model.schematic.session.editor.selection
    )
}

fn split_selected_trace_is_inspected(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && app.state.workbench.results_split_visible(
            app.state.platform.project_lifecycle.is_open(),
            app.state
                .model
                .simulation
                .retained
                .has_retained_result_dataset(),
        )
        && app
            .state
            .results
            .results
            .valid_selected_trace(&app.state.model.simulation)
            .is_some()
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_HEADER_H),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let messages = app.state.workbench.messages();
    let title = if split_selected_trace_is_inspected(app) {
        "Result details".to_owned()
    } else {
        match app.state.workbench.workspace {
            Workspace::Project => match app.state.project.project_page {
                ProjectPage::Overview => "Project overview".to_owned(),
                ProjectPage::Library => "Cell view".to_owned(),
                ProjectPage::Configuration => "Configuration".to_owned(),
                ProjectPage::Dependencies => "Dependency".to_owned(),
                ProjectPage::Recovery => "Checkpoint".to_owned(),
            },
            Workspace::Verify
                if app.state.simulate.verification_page == VerificationPage::Yield =>
            {
                "Yield details".to_owned()
            }
            Workspace::Verify => app.state.simulate.verification_page.label().to_owned(),
            Workspace::Netlist => match app.state.code.code_workspace.page {
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
                    messages.text(MessageId::CodeInspectorNetlistTitle)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
                    messages.text(MessageId::CodeInspectorAutomationTitle)
                }
            },
            // A result sheet's context panel is named by the sheet it is
            // reading, the way every other workspace names its subject.
            Workspace::Results => {
                format!(
                    "{} details",
                    app.state.results.results.session.viewer.tab_label()
                )
            }
            _ => app.state.workbench.workspace.inspector_title().to_owned(),
        }
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &title)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title.as_str());
        node.set_level(2);
    });
    let size = if app.state.workbench.workspace == Workspace::Design {
        tokens::FS_2
    } else {
        tokens::FS_0
    };
    ui.painter().text(
        egui::pos2(rect.left() + PANEL_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &title,
        theme::sans(size, FontWeight::SemiBold),
        t.color.text,
    );
}

/// The significant digits a compact panel readout holds: an operating-point
/// row, a node voltage, a trace's statistics. Four, as an annotated operating
/// point reads in Spectre — the reader's own precision belongs to the result
/// tables, which have the width to spend it.
const COMPACT_SIGNIFICANT_DIGITS: usize = 4;

/// `value` in engineering notation with its unit, at the precision a compact
/// panel row reads at: `96.86 µA`, not `96.86112 µA`.
fn compact_readout(value: f64, unit: &str) -> String {
    engineering_readout(value, unit, COMPACT_SIGNIFICANT_DIGITS)
}

/// `value` in engineering notation with its unit, to the reader's chosen
/// number of significant digits, without the zeros that only pad it out:
/// `900 mV` and `4.1 V`, not `900.0000 mV` and `4.100000 V`.
fn engineering_readout(value: f64, unit: &str, significant_digits: usize) -> String {
    rspice_results_ui::waves::fmt_in_unit_readout(value, unit, significant_digits)
}

#[cfg(test)]
mod tests;
