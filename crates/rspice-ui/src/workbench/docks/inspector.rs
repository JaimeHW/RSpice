//! Context inspector with authoritative object and provenance details.

mod design;
mod symbol;

use egui::{Align2, Color32, Pos2, Rect, Response, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{CellViewRef, Component, ComponentType, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::documents::result_document::PaneAxis;
use crate::workbench::lifecycle::project_lifecycle::dirty_document_count;
use crate::workbench::{AppState, MessageId, RSpiceApp, ResultViewer};

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    PANEL_HEADER_H, PANEL_SECTION_H, StatusMark, WorkbenchIcon, property_row,
    property_row_input_action, property_row_status,
    schematic_section_header as design_schematic_section_header,
    section_header as design_section_header,
};
use super::super::state::{ModelsPage, ProjectPage, VerificationPage, Workspace};

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
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    let library = state
        .model_library_manager
        .get_library(library_name)
        .ok_or_else(|| format!("Model library '{library_name}' is no longer loaded."))?;
    let candidate = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(model_name))
        .ok_or_else(|| {
            format!("Model '{model_name}' is no longer present in library '{library_name}'.")
        })?;

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
            .models
            .values()
            .find(|model| model.name.eq_ignore_ascii_case(current_name))
            .ok_or_else(|| {
                format!(
                    "The selected cell's current model '{current_name}' no longer resolves in library '{library_name}'."
                )
            })?;
        if !crate::state::model_library::models_have_compatible_device_family(current, candidate) {
            return Err(format!(
                "Model '{model_name}' is incompatible with the selected cell's current model family."
            ));
        }
        return Ok(());
    }

    crate::state::model_library::validate_component_model_compatibility(component.kind, candidate)?;
    let effective = state.model_library_manager.effective_definition_provider(
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
    validate_component_model_catalog_binding(&app.state, component_id, library_name, model_name)?;
    let component = app
        .state
        .schematic
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

    let mut params = crate::state::parse_params_string(&component.params);
    let model_unchanged = params
        .get("model")
        .is_some_and(|current| current.eq_ignore_ascii_case(model_name));
    let provider_unchanged = params
        .get("model_library")
        .is_some_and(|current| current.eq_ignore_ascii_case(&effective.library));
    if model_unchanged && provider_unchanged {
        return Ok(());
    }
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    let target = app
        .state
        .schematic
        .components
        .iter_mut()
        .find(|candidate| candidate.id == component_id)
        .expect("the selected component was resolved above");
    params.insert("model".to_owned(), model_name.to_owned());
    params.insert("model_library".to_owned(), effective.library);
    target.params = crate::state::format_params_string(&params);
    app.state.schematic.is_dirty = true;
    app.state.schematic.bump_topology_version();
    app.state
        .schematic
        .commit_undo_from(before, "bind instance model");
    app.invalidate_simulation_preflight();
    Ok(())
}

/// Space a section leaves above the first block of its body, below the last,
/// and between two blocks of the same body.
///
/// One measure governs every section — property list, tree, annotation card
/// or action row — so the panel reads as a single rhythm instead of a stack
/// of separately tuned boxes, and no section is framed unevenly.
const INSPECTOR_SECTION_PADDING: f32 = 8.0;

fn inspector_section_state_id() -> egui::Id {
    egui::Id::new("workbench.inspector.property-list-open")
}

/// Top of the open section's body, so a second block in the same body can
/// tell itself apart from the first one.
fn inspector_section_body_id() -> egui::Id {
    egui::Id::new("workbench.inspector.section-body-top")
}

fn begin_inspector_sections(ui: &mut Ui) {
    ui.data_mut(|data| {
        data.insert_temp(inspector_section_state_id(), -1.0_f32);
        data.remove_temp::<f32>(inspector_section_body_id());
    });
}

/// Close the body of the section above, if there is one.
fn close_open_section(ui: &mut Ui) {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
}

/// Open a body under the header just painted, and record the step the body
/// owes its last block.
fn open_section_body(ui: &mut Ui) {
    ui.add_space(INSPECTOR_SECTION_PADDING);
    let body_top = ui.cursor().top();
    ui.data_mut(|data| {
        data.insert_temp(inspector_section_state_id(), INSPECTOR_SECTION_PADDING);
        data.insert_temp(inspector_section_body_id(), body_top);
    });
}

/// One section step between two blocks of the same body. The body's first
/// block already sits below the header's padding and takes no further gap.
fn section_block_gap(ui: &mut Ui) {
    let body_top = ui.data_mut(|data| data.get_temp::<f32>(inspector_section_body_id()));
    if body_top.is_none_or(|top| ui.cursor().top() > top + 0.5) {
        ui.add_space(INSPECTOR_SECTION_PADDING);
    }
}

fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    close_open_section(ui);
    design_section_header(ui, title, meta);
    open_section_body(ui);
}

/// Schematic-dock section heading: the same rhythm as [`section_header`] with
/// the schematic's larger, tracked EDA typography.
fn schematic_section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    close_open_section(ui);
    design_schematic_section_header(ui, title, meta);
    open_section_body(ui);
}

fn schematic_section_header_action(
    ui: &mut Ui,
    title: &str,
    action: &str,
    enabled: bool,
) -> Response {
    close_open_section(ui);
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PANEL_SECTION_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));

    let title_job = egui::text::LayoutJob::single_section(
        title.to_uppercase(),
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_2, FontWeight::SemiBold),
            color: t.color.text_dim,
            extra_letter_spacing: 0.055 * tokens::FS_2,
            ..Default::default()
        },
    );
    let title_galley = ui.fonts_mut(|fonts| fonts.layout_job(title_job));
    ui.painter().galley(
        Pos2::new(
            rect.left() + 10.0,
            rect.center().y - title_galley.size().y * 0.5,
        ),
        title_galley,
        t.color.text_dim,
    );

    let action_galley = ui.painter().layout_no_wrap(
        action.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let action_rect = Rect::from_min_max(
        Pos2::new(
            rect.right() - 10.0 - action_galley.size().x - 10.0,
            rect.top() + 2.0,
        ),
        Pos2::new(rect.right() - 8.0, rect.bottom() - 2.0),
    );
    let mut response = ui.interact(
        action_rect,
        ui.id().with(("schematic-section-action", title, action)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if !enabled {
        crate::ui::widgets::mark_response_disabled(&mut response);
    }
    if response.hovered() && enabled {
        ui.painter()
            .rect_filled(action_rect, t.radius, t.color.bg_hover);
    }
    ui.painter().galley(
        Pos2::new(
            action_rect.center().x - action_galley.size().x * 0.5,
            action_rect.center().y - action_galley.size().y * 0.5,
        ),
        action_galley,
        t.color.text_dim,
    );
    theme::paint_focus_ring_outset(ui, &response, action_rect);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, action.to_owned())
    });
    open_section_body(ui);
    response
}

fn finish_inspector_sections(ui: &mut Ui) {
    let bottom = ui
        .data_mut(|data| {
            data.remove_temp::<f32>(inspector_section_body_id());
            data.remove_temp::<f32>(inspector_section_state_id())
        })
        .unwrap_or(-1.0);
    if bottom >= 0.0 {
        ui.add_space(bottom);
    }
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
        begin_inspector_sections(ui);
        if split_selected_trace_is_inspected(app) {
            results(ui, app);
        } else {
            match app.state.workbench.workspace {
                Workspace::Project => project(ui, app),
                // A symbol cellview is edited against its pin contract, not
                // against a schematic selection.
                Workspace::Design
                    if app.state.workspace.active_view_type() == crate::state::ViewType::Symbol =>
                {
                    symbol::show(ui, app);
                }
                Workspace::Design => design::show(ui, app),
                Workspace::Simulate => simulate(ui, app),
                Workspace::Results => results(ui, app),
                Workspace::Verify => verify(ui, app),
                Workspace::Models => models(ui, app),
                Workspace::Netlist => code_workspace_inspector(ui, app),
            }
        }
        finish_inspector_sections(ui);
    });
    scroll_memory.record(scroll_identity, output.state.offset.y);
    ui.ctx().data_mut(|data| {
        data.insert_temp(inspector_scroll_memory_id(), scroll_memory);
    });
}

fn inspector_scroll_identity(app: &RSpiceApp) -> String {
    let route = app.state.workbench.current_route();
    if app.state.workbench.workspace != Workspace::Design {
        return route.to_string();
    }

    format!(
        "{}|{}|{:?}",
        route,
        app.state.workspace.active_display_path(),
        app.state.schematic.selection
    )
}

fn split_selected_trace_is_inspected(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && app.state.workbench.results_split_visible(
            app.state.project_lifecycle.project_open,
            app.state.simulation.has_retained_result_dataset(),
        )
        && app
            .state
            .ui
            .results
            .valid_selected_trace(&app.state.simulation)
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
    let messages = app.state.ui.messages();
    let title = if split_selected_trace_is_inspected(app) {
        "Result details".to_owned()
    } else {
        match app.state.workbench.workspace {
            Workspace::Project => match app.state.workbench.project_page {
                ProjectPage::Overview => "Project overview".to_owned(),
                ProjectPage::Library => "Cell view".to_owned(),
                ProjectPage::Configuration => "Configuration".to_owned(),
                ProjectPage::Dependencies => "Dependency".to_owned(),
                ProjectPage::Recovery => "Checkpoint".to_owned(),
            },
            Workspace::Verify
                if app.state.workbench.verification_page == VerificationPage::Yield =>
            {
                "Yield details".to_owned()
            }
            Workspace::Verify => app.state.workbench.verification_page.label().to_owned(),
            Workspace::Netlist => match app.state.ui.code_workspace.page {
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
                    messages.text(MessageId::CodeInspectorNetlistTitle)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA => {
                    messages.text(MessageId::CodeInspectorVerilogATitle)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
                    messages.text(MessageId::CodeInspectorAutomationTitle)
                }
            },
            // A result sheet's context panel is named by the sheet it is
            // reading, the way every other workspace names its subject.
            Workspace::Results => {
                format!("{} details", app.state.ui.results.viewer.tab_label())
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
    if app.state.workbench.workspace == Workspace::Design {
        let job = egui::text::LayoutJob::single_section(
            title.to_ascii_uppercase(),
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_2, FontWeight::SemiBold),
                color: t.color.text,
                extra_letter_spacing: 0.065 * tokens::FS_2,
                ..Default::default()
            },
        );
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        ui.painter().galley(
            egui::pos2(rect.left() + 11.0, rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text,
        );
    } else {
        ui.painter().text(
            egui::pos2(rect.left() + 11.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            title.to_ascii_uppercase(),
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
        );
    }
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.workbench.project_page {
        ProjectPage::Overview => project_overview(ui, app),
        ProjectPage::Library => project_library(ui, app),
        ProjectPage::Configuration => project_configuration(ui, app),
        ProjectPage::Dependencies => project_dependency(ui, app),
        ProjectPage::Recovery => project_recovery(ui, app),
    }
}

fn project_overview(ui: &mut Ui, app: &mut RSpiceApp) {
    let active_configuration = app.state.workspace.configuration_sets.active();
    let testbench = active_configuration.map_or_else(
        || "not configured".to_owned(),
        |configuration| configuration.root().display_path(),
    );
    let configuration = active_configuration.map_or_else(
        || "not configured".to_owned(),
        |item| item.name().to_owned(),
    );
    let modified = dirty_document_count(&app.state);
    let t = Tokens::get(ui.ctx());

    section_header(
        ui,
        "Active simulation context",
        Some(if active_configuration.is_some() {
            "CURRENT"
        } else {
            "REVIEW"
        }),
    );
    property_row(
        ui,
        "Top",
        &format!(
            "{}/{}",
            app.state.workspace.project.root_library, app.state.workspace.project.top_cell
        ),
    );
    property_row(ui, "Testbench", &testbench);
    property_row(ui, "Configuration", &configuration);
    property_row(
        ui,
        "Run plan",
        app.state.sim_setup.active_plan_name().as_str(),
    );
    property_row(
        ui,
        "Execution",
        crate::state::ExecutionTarget::current().label(),
    );

    section_header(
        ui,
        "Working revision",
        Some(if modified == 0 { "CLEAN" } else { "MODIFIED" }),
    );
    property_row(
        ui,
        "Revision",
        &app.state.workspace.project.revision().get().to_string(),
    );
    property_row(ui, "Modified documents", &modified.to_string());
    property_row_status(
        ui,
        "Working tree",
        if modified == 0 { "clean" } else { "modified" },
        if modified == 0 {
            t.color.ok
        } else {
            t.color.warn
        },
        if modified == 0 {
            StatusMark::Success
        } else {
            StatusMark::Warning
        },
    );
    property_row(
        ui,
        "Location",
        &app.state.workspace.project.path.as_ref().map_or_else(
            || "no accepted native path".to_owned(),
            |path| path.display().to_string(),
        ),
    );

    let check_status = app.state.project_root_design_check_status();
    let check_meta = match &check_status {
        DesignCheckStatus::Current(_) => "CURRENT",
        DesignCheckStatus::NotRun => "NOT RUN",
        DesignCheckStatus::Stale(_) => "STALE",
        DesignCheckStatus::Unavailable { .. } => "UNAVAILABLE",
    };
    section_header(ui, "Problems", Some(check_meta));
    match check_status {
        DesignCheckStatus::Current(receipt) => {
            let summary = receipt.result.summary();
            property_row(
                ui,
                "Blocking errors",
                &(summary.critical + summary.errors).to_string(),
            );
            property_row(
                ui,
                "Advisories",
                &(summary.warnings + summary.info).to_string(),
            );
            for violation in receipt.result.violations().iter().take(2) {
                property_row(ui, &format!("CHK-{:03}", violation.id), &violation.message);
            }
            property_row(
                ui,
                "Checked revision",
                &receipt.checked_project_revision.get().to_string(),
            );
        }
        DesignCheckStatus::NotRun => {
            property_row(ui, "Checks", "Not run for the project root");
        }
        DesignCheckStatus::Stale(_) => {
            property_row(ui, "Checks", "Project-root inputs changed after checking");
        }
        DesignCheckStatus::Unavailable { reason, .. } => {
            property_row(ui, "Checks", &reason);
        }
    }
    if let Some(advisory) = app.state.pdk_config.scan_errors.first() {
        property_row(ui, "Model advisory", advisory);
    }

    section_header(ui, "Project actions", None);
    if inspector_action(ui, "Revision history\u{2026}") {
        Command::RevisionHistory.execute(app);
    }
    if inspector_action(ui, "Configuration sets\u{2026}") {
        Command::ConfigurationSets.execute(app);
    }
    if inspector_action(ui, "Switch project\u{2026}") {
        Command::ProjectLauncher.execute(app);
    }
}

fn project_library(ui: &mut Ui, app: &mut RSpiceApp) {
    let library = app.state.library_manager.current_library();
    let library_name = library.map_or("No library selected", |item| item.name.as_str());
    let library_path = library
        .and_then(|item| item.path.as_ref())
        .map_or_else(|| "not bound".to_owned(), |path| path.display().to_string());
    let access = library.map_or("unavailable", |item| {
        if item.read_only {
            "read only"
        } else {
            "writable"
        }
    });
    let technology = library.map_or("unbound", |item| {
        if item.technology.trim().is_empty() {
            "unbound"
        } else {
            item.technology.as_str()
        }
    });
    let cell_name = app
        .state
        .library_manager
        .current_cell()
        .map_or("No cell selected", |item| item.name.as_str());
    let view = app.state.library_manager.current_view();
    let view_name = view.map_or("No view selected", |item| item.name.as_str());
    let view_type = view.map_or("—", |item| item.view_type.display_name());
    let open_target = match (
        app.state.library_manager.current_library(),
        app.state.library_manager.current_cell(),
        view,
    ) {
        (Some(library), Some(cell), Some(view)) => Some((
            CellViewRef::new(&library.name, &cell.name, &view.name),
            view.view_type,
        )),
        _ => None,
    };
    let usage = open_target
        .as_ref()
        .map_or_else(Vec::new, |(reference, _)| library_usage(app, reference));
    let project_revision = app.state.workspace.project.revision().get().to_string();
    let active_configuration = app
        .state
        .workspace
        .configuration_sets
        .active()
        .map_or("not configured", |item| item.name());
    let (checks_meta, checks) = match open_target.as_ref() {
        Some((reference, ViewType::Schematic | ViewType::Testbench)) => {
            match app.state.design_check_status(reference) {
                DesignCheckStatus::Current(receipt) => {
                    let summary = receipt.result.summary();
                    (
                        "CURRENT",
                        format!(
                            "{} errors \u{00b7} {} advisories",
                            summary.critical + summary.errors,
                            summary.warnings + summary.info
                        ),
                    )
                }
                DesignCheckStatus::NotRun => ("NOT RUN", "not run".to_owned()),
                DesignCheckStatus::Stale(_) => ("STALE", "inputs changed".to_owned()),
                DesignCheckStatus::Unavailable { reason, .. } => ("UNAVAILABLE", reason),
            }
        }
        Some(_) => ("N/A", "not applicable to this view type".to_owned()),
        None => ("NO VIEW", "select a cell view".to_owned()),
    };

    section_header(ui, "Selection", Some(&access.to_ascii_uppercase()));
    property_row(ui, "Library", library_name);
    property_row(ui, "Cell", cell_name);
    property_row(ui, "View", view_name);
    property_row(ui, "View type", view_type);

    section_header(ui, "Ownership & binding", None);
    property_row(ui, "Revision", &project_revision);
    property_row(ui, "Access", access);
    property_row(ui, "Technology", technology);
    property_row(ui, "Source", &library_path);
    property_row(
        ui,
        "State",
        if view.is_some_and(|item| item.modified) {
            "modified"
        } else {
            "current"
        },
    );

    section_header(ui, "Validation", Some(checks_meta));
    property_row(ui, "Checks", &checks);
    property_row(ui, "Configuration", active_configuration);
    property_row(ui, "Consumers", &usage.len().to_string());
    property_row(
        ui,
        "Where used",
        usage.first().map_or("project root", String::as_str),
    );

    section_header(ui, "Actions", None);
    if inspector_action_enabled(ui, "Open selected view", open_target.is_some())
        && let Some((reference, view_type)) = open_target.as_ref()
    {
        app.state.open_workspace_view(reference.clone());
        let workspace = match view_type {
            ViewType::Verilog | ViewType::VerilogA | ViewType::Spice => Workspace::Netlist,
            ViewType::Config => {
                app.state.workbench.project_page = ProjectPage::Configuration;
                Workspace::Project
            }
            _ => Workspace::Design,
        };
        Command::OpenWorkspace(workspace).execute(app);
    }
    if inspector_action_enabled(ui, "Where used & impact\u{2026}", open_target.is_some())
        && let Some((reference, _)) = open_target.as_ref()
    {
        app.state.push_user_message(ConsoleMessage::info(format!(
            "{} has {} loaded schematic consumer{}: {}",
            reference.display_path(),
            usage.len(),
            if usage.len() == 1 { "" } else { "s" },
            if usage.is_empty() {
                "none".to_owned()
            } else {
                usage.join(", ")
            }
        )));
        Command::OpenConsole.execute(app);
    }
    if inspector_action(ui, "Revision history\u{2026}") {
        Command::RevisionHistory.execute(app);
    }
}

fn project_configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    let active = app.state.workspace.configuration_sets.active();
    let name = active.map_or_else(
        || "No active configuration".to_owned(),
        |item| item.name().to_owned(),
    );
    let testbench = active.map_or_else(
        || "not configured".to_owned(),
        |item| item.root().display_path(),
    );
    let dut = active.map_or_else(
        || "not configured".to_owned(),
        |item| item.dut_path().to_owned(),
    );
    let revision = active.map_or_else(|| "—".to_owned(), |item| item.revision().to_string());
    let overrides = active.map_or(0, |item| item.definition().overrides.len());

    section_header(ui, "Active binding", None);
    property_row(ui, "Configuration", &name);
    property_row(ui, "Testbench", &testbench);
    property_row(ui, "DUT", &dut);
    property_row(ui, "Revision", &revision);

    section_header(ui, "Netlisting contract", None);
    property_row(ui, "Overrides", &overrides.to_string());
    property_row(ui, "Global precedence", "project → configuration → run");
    property_row(
        ui,
        "Binding",
        if active.is_some() {
            "resolved"
        } else {
            "missing"
        },
    );

    section_header(ui, "Actions", None);
    if inspector_action(ui, "Validate configuration") {
        Command::PreflightChecks.execute(app);
    }
    if inspector_action(ui, "Manage configuration sets\u{2026}") {
        Command::ConfigurationSets.execute(app);
    }
}

fn project_dependency(ui: &mut Ui, app: &mut RSpiceApp) {
    let selection = app
        .state
        .workbench
        .project_dependency_selection
        .clone()
        .unwrap_or_else(|| "No dependency selected".to_owned());
    let design_library = app.state.library_manager.get_library(&selection);
    let model_library = app.state.model_library_manager.get_library(&selection);

    section_header(ui, "Resolved dependency", None);
    property_row(ui, "Name", &selection);
    if let Some(library) = design_library {
        property_row(ui, "Type", "Design library");
        property_row(
            ui,
            "Access",
            if library.read_only {
                "read only"
            } else {
                "writable"
            },
        );
        property_row(ui, "Cells", &library.cell_count().to_string());
        property_row(
            ui,
            "Source",
            &library.path.as_ref().map_or_else(
                || "in project".to_owned(),
                |path| path.display().to_string(),
            ),
        );
    } else if let Some(library) = model_library {
        property_row(ui, "Type", "Model library");
        property_row(ui, "Version", &library.version);
        property_row(ui, "Models", &library.models.len().to_string());
        property_row(ui, "Corners", &library.corners.len().to_string());
        property_row(
            ui,
            "Execution source",
            if library.source_authority.has_execution_source() && !library.source_closure.is_empty()
            {
                "authenticated"
            } else {
                "catalog only"
            },
        );
    } else {
        property_row(ui, "Status", "Select a dependency in the navigator");
    }

    section_header(ui, "Project contract", None);
    property_row(
        ui,
        "Technology",
        if app.state.workspace.project.technology_binding().is_some() {
            "attached"
        } else {
            "not attached"
        },
    );
    property_row(ui, "Missing dependencies", "0");

    section_header(ui, "Actions", None);
    if inspector_action(ui, "Model & library catalog") {
        Command::ModelsPage(ModelsPage::Models).execute(app);
    }
    if inspector_action(ui, "PDK and model paths\u{2026}") {
        Command::PdkSettings.execute(app);
    }
}

fn project_recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app.state.workbench.project_checkpoint_selection.as_deref();
    let checkpoint = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .checkpoints
        .iter()
        .find(|item| selected.is_some_and(|id| id == item.checkpoint_id().to_string()));
    let integrity = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .error
        .is_none()
        && app
            .state
            .dialogs
            .project_checkpoint_recovery
            .quarantined
            .is_empty();

    section_header(ui, "Selected checkpoint", None);
    if let Some(checkpoint) = checkpoint {
        property_row(ui, "Reason", checkpoint.reason().label());
        property_row(ui, "Revision", &checkpoint.project_revision().to_string());
        property_row(
            ui,
            "Payload",
            &format!("{} bytes", checkpoint.snapshot_byte_len()),
        );
        property_row(ui, "Checkpoint ID", &checkpoint.checkpoint_id().to_string());
    } else {
        property_row(ui, "Selection", "No checkpoint selected");
    }

    section_header(ui, "Recovery contract", None);
    property_row(
        ui,
        "Integrity",
        if integrity {
            "verified"
        } else {
            "review required"
        },
    );
    property_row(ui, "Restore mode", "independent project copy");
    property_row(ui, "Current work", "never overwritten");
    property_row(
        ui,
        "Quarantined",
        &app.state
            .dialogs
            .project_checkpoint_recovery
            .quarantined
            .len()
            .to_string(),
    );

    section_header(ui, "Actions", None);
    if inspector_action(ui, "Save project") {
        Command::Save.execute(app);
    }
    if inspector_action(ui, "Revision history\u{2026}") {
        Command::RevisionHistory.execute(app);
    }
}

fn inspector_action(ui: &mut Ui, label: &str) -> bool {
    inspector_action_enabled(ui, label, true)
}

fn inspector_action_enabled(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    let width = ui.available_width().max(1.0);
    let response = ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized([width, 28.0], egui::Button::new(label))
        })
        .inner;
    ui.add_space(4.0);
    response.clicked()
}

fn library_usage(app: &RSpiceApp, reference: &CellViewRef) -> Vec<String> {
    fn collect(
        schematic: &crate::state::SchematicState,
        owner: &str,
        reference: &CellViewRef,
        consumers: &mut Vec<String>,
    ) {
        if schematic.components.iter().any(|component| {
            component.library_cell.as_ref().is_some_and(|binding| {
                binding.library.eq_ignore_ascii_case(&reference.library)
                    && binding.cell.eq_ignore_ascii_case(&reference.cell)
                    && binding.view.eq_ignore_ascii_case(&reference.view)
            })
        }) {
            consumers.push(owner.to_owned());
        }
    }

    let mut consumers = Vec::new();
    let active_key = app.state.workspace.active_view.key();
    collect(
        &app.state.schematic,
        &app.state.workspace.active_view.display_path(),
        reference,
        &mut consumers,
    );
    for (key, schematic) in &app.state.workspace.schematic_buffers {
        if key != &active_key {
            collect(schematic, key, reference, &mut consumers);
        }
    }
    consumers.sort_by_key(|item| item.to_ascii_lowercase());
    consumers.dedup();
    consumers
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelEvidenceTone {
    Neutral,
    Info,
    Warn,
    Error,
    Ok,
}

impl ModelEvidenceTone {
    fn color(self, tokens: &Tokens) -> egui::Color32 {
        match self {
            Self::Neutral => tokens.color.text_dim,
            Self::Info => tokens.color.info,
            Self::Warn => tokens.color.warn,
            Self::Error => tokens.color.err,
            Self::Ok => tokens.color.ok,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComponentModelEvidence {
    status: String,
    model: String,
    source: String,
    section: String,
    tone: ModelEvidenceTone,
}

fn component_model_evidence(state: &AppState, component: &Component) -> ComponentModelEvidence {
    if let Some(binding) = component.library_cell.as_ref() {
        if binding.is_builtin_xspice() {
            return match crate::state::validate_builtin_xspice_binding(binding) {
                Ok(descriptor) => ComponentModelEvidence {
                    status: "compiled XSPICE contract verified".to_owned(),
                    model: descriptor.model_type.to_owned(),
                    source: descriptor.stable_id.to_owned(),
                    section: "Not applicable".to_owned(),
                    tone: ModelEvidenceTone::Info,
                },
                Err(error) => ComponentModelEvidence {
                    status: "compiled XSPICE contract invalid".to_owned(),
                    model: binding.cell.clone(),
                    source: error,
                    section: "Unavailable".to_owned(),
                    tone: ModelEvidenceTone::Error,
                },
            };
        }
        if binding.is_generated_veriloga() {
            return match crate::state::validate_generated_veriloga_binding(binding) {
                Ok(descriptor) => ComponentModelEvidence {
                    status: "compiled Verilog-A contract verified".to_owned(),
                    model: descriptor.model_name.to_owned(),
                    source: format!("source {}", descriptor.source_digest),
                    section: "Not applicable".to_owned(),
                    tone: ModelEvidenceTone::Info,
                },
                Err(error) => ComponentModelEvidence {
                    status: "compiled Verilog-A contract invalid".to_owned(),
                    model: binding.cell.clone(),
                    source: error,
                    section: "Unavailable".to_owned(),
                    tone: ModelEvidenceTone::Error,
                },
            };
        }
        if let Some(library) = state.model_library_manager.get_library(&binding.library) {
            let candidates = [
                binding.module_name.as_deref(),
                Some(binding.cell.as_str()),
                (!component.value.trim().is_empty()).then_some(component.value.trim()),
            ];
            if let Some(model) = candidates.into_iter().flatten().find_map(|candidate| {
                library
                    .models
                    .values()
                    .find(|model| model.name.eq_ignore_ascii_case(candidate))
            }) {
                let mut evidence = catalog_model_evidence(library, model);
                evidence.section = binding
                    .model_section
                    .clone()
                    .unwrap_or_else(|| "default".to_owned());
                return evidence;
            }
        }

        if let Some(path) = binding.source_path.as_deref() {
            return ComponentModelEvidence {
                status: "source binding declared · catalog unverified".to_owned(),
                model: binding
                    .module_name
                    .clone()
                    .unwrap_or_else(|| binding.cell.clone()),
                source: path.display().to_string(),
                section: binding
                    .model_section
                    .clone()
                    .unwrap_or_else(|| "default".to_owned()),
                tone: ModelEvidenceTone::Warn,
            };
        }

        let project_view_exists = state
            .library_manager
            .get_library(&binding.library)
            .and_then(|library| library.get_cell(&binding.cell))
            .and_then(|cell| cell.get_view(&binding.view))
            .is_some();
        return ComponentModelEvidence {
            status: if project_view_exists {
                "project cell binding present"
            } else {
                "library binding unresolved · review"
            }
            .to_owned(),
            model: binding
                .module_name
                .clone()
                .unwrap_or_else(|| binding.cell.clone()),
            source: format!("{}/{}/{}", binding.library, binding.cell, binding.view),
            section: binding
                .model_section
                .clone()
                .unwrap_or_else(|| "default".to_owned()),
            tone: if project_view_exists {
                ModelEvidenceTone::Info
            } else {
                ModelEvidenceTone::Error
            },
        };
    }

    if let Some(model_name) = explicit_component_model(component) {
        let mut matches = Vec::new();
        for library in state.model_library_manager.libraries_sorted() {
            for model in library
                .models
                .values()
                .filter(|model| model.name.eq_ignore_ascii_case(&model_name))
            {
                matches.push((library, model));
            }
        }
        return match matches.as_slice() {
            [(library, model)] => catalog_model_evidence(library, model),
            [] => ComponentModelEvidence {
                status: "model reference not loaded · review".to_owned(),
                model: model_name,
                source: "No matching loaded catalog entry".to_owned(),
                section: "Unavailable".to_owned(),
                tone: ModelEvidenceTone::Error,
            },
            _ => ComponentModelEvidence {
                status: "model reference ambiguous · review".to_owned(),
                model: model_name,
                source: format!("{} matching catalog entries", matches.len()),
                section: "Unavailable".to_owned(),
                tone: ModelEvidenceTone::Error,
            },
        };
    }

    if let Some(model_name) = generated_inline_model(component) {
        return ComponentModelEvidence {
            status: "inline model · generated for netlist".to_owned(),
            model: model_name,
            source: "Netlist generator".to_owned(),
            section: "Not applicable".to_owned(),
            tone: ModelEvidenceTone::Info,
        };
    }

    ComponentModelEvidence {
        status: "native device · no catalog model".to_owned(),
        model: "Not applicable".to_owned(),
        source: "Built-in device equation".to_owned(),
        section: "Not applicable".to_owned(),
        tone: ModelEvidenceTone::Neutral,
    }
}

fn explicit_component_model(component: &Component) -> Option<String> {
    let params = crate::state::parse_params_string(&component.params);
    let param_model = params
        .get("model")
        .map(|model| model.trim())
        .filter(|model| !model.is_empty());
    let value_model = component.value.trim();
    match component.kind {
        ComponentType::NpnBjt
        | ComponentType::PnpBjt
        | ComponentType::NpnBjt4
        | ComponentType::PnpBjt4
        | ComponentType::NpnBjt5
        | ComponentType::PnpBjt5
        | ComponentType::VSwitch
        | ComponentType::ISwitch
        | ComponentType::Diode
        | ComponentType::Nmos
        | ComponentType::Pmos
        | ComponentType::NVdmos
        | ComponentType::PVdmos
        | ComponentType::NmosSoi
        | ComponentType::PmosSoi
        | ComponentType::Njfet
        | ComponentType::Pjfet
        | ComponentType::Nmesfet
        | ComponentType::Pmesfet
        | ComponentType::Memristor
        | ComponentType::LossyTransmissionLine
        | ComponentType::CoupledTransmissionLine => param_model
            .or((!value_model.is_empty()).then_some(value_model))
            .map(str::to_owned),
        // The saturable inductor's value field is the inductance, so only
        // an explicit model= parameter names a library core model.
        ComponentType::SaturableInductor | ComponentType::GenericSwitch => {
            param_model.map(str::to_owned)
        }
        _ => None,
    }
}

fn generated_inline_model(component: &Component) -> Option<String> {
    let prefix = match component.kind {
        ComponentType::Nmos => "nmos",
        ComponentType::Pmos => "pmos",
        ComponentType::NVdmos => "nvdmos",
        ComponentType::PVdmos => "pvdmos",
        ComponentType::NmosSoi => "nmossoi",
        ComponentType::PmosSoi => "pmossoi",
        ComponentType::NpnBjt | ComponentType::NpnBjt4 | ComponentType::NpnBjt5 => "npn",
        ComponentType::PnpBjt | ComponentType::PnpBjt4 | ComponentType::PnpBjt5 => "pnp",
        ComponentType::Njfet => "njf",
        ComponentType::Pjfet => "pjf",
        ComponentType::Nmesfet => "nmf",
        ComponentType::Pmesfet => "pmf",
        ComponentType::VSwitch => "sw",
        ComponentType::ISwitch => "isw",
        ComponentType::GenericSwitch => "sw",
        ComponentType::Diode => "d",
        ComponentType::SaturableInductor => "core",
        ComponentType::Memristor => "mem",
        ComponentType::CoupledTransmissionLine => "cpl",
        ComponentType::LossyTransmissionLine => {
            let params = crate::state::parse_params_string(&component.params);
            let kind = if params.get("kind").is_some_and(|kind| kind == "txl") {
                "txl"
            } else {
                "ltra"
            };
            return Some(format!("{kind}_{}", component.name));
        }
        kind if kind.spice_prefix() == "A" => return Some(format!("{}_model", component.name)),
        _ => return None,
    };
    Some(format!("{prefix}_{}", component.name))
}

fn catalog_model_evidence(
    library: &crate::state::model_library::ModelLibrary,
    model: &crate::state::model_library::DeviceModel,
) -> ComponentModelEvidence {
    let section = library
        .selected_corner
        .as_deref()
        .map_or_else(|| "Not selected".to_owned(), |section| section.to_owned());
    let external = library.root_path.is_some() || model.file_path.is_some();
    if !external {
        return ComponentModelEvidence {
            status: "catalog metadata · in memory".to_owned(),
            model: model.name.clone(),
            source: format!("{} · in-memory catalog", library.name),
            section,
            tone: ModelEvidenceTone::Info,
        };
    }

    let source = model.file_path.as_deref().or(library.root_path.as_deref());
    let pin = library.root_path.as_ref().and_then(|_| {
        source.and_then(|source| library.source_closure.iter().find(|pin| pin.path == source))
    });
    match (source, pin) {
        (Some(source), Some(pin)) => ComponentModelEvidence {
            status: "catalog model source pinned".to_owned(),
            model: model.name.clone(),
            source: format!(
                "{} · {}",
                source.display(),
                short_content_digest(&pin.digest.to_string())
            ),
            section,
            tone: ModelEvidenceTone::Ok,
        },
        (Some(source), None) => ComponentModelEvidence {
            status: "catalog model source unpinned · review".to_owned(),
            model: model.name.clone(),
            source: source.display().to_string(),
            section,
            tone: ModelEvidenceTone::Error,
        },
        (None, _) => ComponentModelEvidence {
            status: "catalog source metadata incomplete · review".to_owned(),
            model: model.name.clone(),
            source: "External library has no model source path".to_owned(),
            section,
            tone: ModelEvidenceTone::Error,
        },
    }
}

fn short_content_digest(digest: &str) -> String {
    if digest.len() <= 12 {
        digest.to_owned()
    } else {
        format!("{}…{}", &digest[..8], &digest[digest.len() - 4..])
    }
}

fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            let legacy_kind = crate::simulation::plan::AnalysisKind::from_legacy_index(
                app.state.workbench.active_analysis,
            );
            plan.instances()
                .iter()
                .find(|instance| {
                    Some(instance.id()) == app.state.workbench.active_analysis_instance
                })
                .or_else(|| {
                    legacy_kind.and_then(|kind| {
                        plan.instances()
                            .iter()
                            .find(|instance| instance.kind() == kind)
                    })
                })
                .or_else(|| plan.instances().first())
                .map(|instance| {
                    (
                        instance.id(),
                        instance.kind(),
                        instance.draft().clone(),
                        instance.enabled(),
                        instance.dependencies().len(),
                        instance.modified_revision(),
                    )
                })
        });

    section_header(ui, "Selected analysis", None);
    let Some((id, kind, draft, enabled, dependency_count, revision)) = selected else {
        property_row(ui, "Selection", "No analysis instances in this plan");
        return;
    };
    app.state.workbench.active_analysis_instance = Some(id);
    app.state.workbench.active_analysis = kind.legacy_index();
    property_row(ui, "Type", kind.label());
    property_row(ui, "Instance", &id.to_string());
    property_row(
        ui,
        "Configuration",
        &app.state.sim_setup.analysis_draft_summary(&draft),
    );
    property_row(ui, "Run set", if enabled { "Enabled" } else { "Excluded" });
    property_row(ui, "Revision", &revision.get().to_string());
    property_row(ui, "Prerequisites", &dependency_count.to_string());
    if let Some(error) = app.state.sim_setup.analysis_draft_validation_error(&draft) {
        property_row_status(
            ui,
            "Validation",
            &error,
            Tokens::get(ui.ctx()).color.err,
            StatusMark::Failure,
        );
    } else {
        property_row_status(
            ui,
            "Validation",
            "Configuration valid",
            Tokens::get(ui.ctx()).color.ok,
            StatusMark::Success,
        );
    }
    section_header(ui, "Execution context", None);
    property_row(
        ui,
        "Corner",
        app.state.sim_setup.reference_pvt.process.short_name(),
    );
    property_row(
        ui,
        "Temperature",
        &format!(
            "{} °C",
            app.state.sim_setup.reference_pvt.temperature_celsius
        ),
    );
    property_row(
        ui,
        "Enabled analyses",
        &app.state
            .sim_setup
            .enabled_analysis_instance_count()
            .to_string(),
    );
    property_row(
        ui,
        "Engine status",
        if app.state.simulation.cancellation_is_pending() {
            "Stopping"
        } else if matches!(
            app.state.simulation.active_execution_lifecycle(),
            Some(crate::state::SimulationRunLifecycle::Preparing)
        ) {
            "Preparing"
        } else if app.state.simulation.has_active_execution() {
            "Running"
        } else {
            "Ready"
        },
    );
}

fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected_trace = app
        .state
        .ui
        .results
        .valid_selected_trace(&app.state.simulation)
        .cloned();
    let selected_artifact = app
        .state
        .ui
        .results
        .selected_result_artifact
        .clone()
        .filter(|key| key.resolve(&app.state.simulation.runs).is_some());

    // What is being read comes before where it came from: a reader adjusting
    // a pane needs its axes and bindings first, and the dataset provenance
    // below is unchanged by any of that.
    active_result_pane(ui, app, selected_trace.as_ref());
    if let Some(selected) = selected_trace.as_ref() {
        selected_result_trace(ui, app, selected);
    } else if let Some(selected) = selected_artifact.as_ref() {
        selected_result_artifact(ui, app, selected);
    }
    ui.add_space(8.0);

    let active_run_index = app.state.simulation.active_run_idx;
    if let Some(run) = active_run_index.and_then(|index| app.state.simulation.runs.get(index)) {
        let manifest =
            crate::workbench::documents::result_document::manifest::ManifestViewModel::from_run(
                run,
            );
        result_dataset_authority(ui, run, &manifest);
    } else {
        section_header(ui, "Dataset identity", None);
        property_row(ui, "Selection", "No active dataset");
        property_row(
            ui,
            "Available runs",
            &app.state.simulation.runs.len().to_string(),
        );
        result_qualification_gaps(ui);
    }

    // The active viewer owns its engineering readout (measurements, margins,
    // harmonics, eye metrics, distribution statistics, and so on). It follows
    // the trace and pane controls, matching the upgraded mockup's hierarchy
    // without creating a second result-data owner here.
    ui.add_space(8.0);
    if app.state.ui.results.viewer != ResultViewer::Manifest {
        crate::workbench::documents::result_document::right_panel(ui, &mut app.state);
    }

    let Some(run_index) = active_run_index else {
        return;
    };
    let Some(run) = app.state.simulation.runs.get(run_index) else {
        return;
    };

    // The analysis's own provenance belongs to the same folded record as the
    // dataset's, even though a different owner draws it.
    if inspector_disclosure_open(ui.ctx(), "result-provenance")
        && let Some(index) = app.state.simulation.active_analysis_idx
        && let Some(analysis) = run.analyses.get(index)
    {
        section_header(ui, "Active analysis provenance", None);
        property_row(ui, "Analysis", &analysis.label);
        property_row(
            ui,
            "Result digest",
            &analysis.result_data_digest().to_string(),
        );
        if let Some(provenance) = &analysis.provenance {
            property_row(
                ui,
                "Source instance",
                &provenance.source_instance_id().to_string(),
            );
            property_row(
                ui,
                "Source revision",
                &provenance.source_revision().get().to_string(),
            );
            property_row(
                ui,
                "Prepared snapshot",
                &provenance.prepared_snapshot_digest().to_string(),
            );
            property_row(
                ui,
                "Dependencies",
                &provenance.dependency_ids().len().to_string(),
            );
        } else {
            property_row(ui, "Source identity", "Legacy result · unavailable");
        }
    }
}

const RESULT_QUALIFICATION_GAPS: [(&str, &str); 5] = [
    ("Qualification receipt", "not retained"),
    ("Requirements mapping", "not retained"),
    ("Release gates", "not assessed"),
    ("Sign-off eligibility", "not assessed"),
    ("Approval authority", "not retained"),
];

fn result_dataset_authority(
    ui: &mut Ui,
    run: &crate::state::SimulationRun,
    manifest: &crate::workbench::documents::result_document::manifest::ManifestViewModel,
) {
    let successful_results = run
        .analyses
        .iter()
        .filter(|analysis| analysis.success)
        .count();

    // Identity leads with what a reader uses to tell one dataset from
    // another; the identifiers that only prove it are provenance, and fold
    // away with the rest of the authority record below.
    section_header(ui, "Dataset identity", Some("current"));
    property_row(ui, "Dataset", &manifest.run_label);
    property_row(ui, "Run sequence", &manifest.run_sequence);
    property_row(ui, "Lifecycle", &manifest.lifecycle);
    property_row(ui, "Duration", &manifest.elapsed_time);
    property_row(ui, "Execution target", &manifest.execution_target);

    if !inspector_disclosure(ui, "result-provenance", "Run provenance", "immutable") {
        return;
    }

    property_row(ui, "Dataset ID", &manifest.dataset_id);
    property_row(ui, "Dataset digest", &manifest.dataset_digest);
    property_row(ui, "Run ID", &manifest.run_id);
    property_row(
        ui,
        "Job ID",
        &run.job_id
            .map_or_else(|| "not retained".to_owned(), |id| id.to_string()),
    );

    section_header(ui, "Retained inventory", None);
    property_row(
        ui,
        "Task receipts",
        &manifest.authority.as_ref().map_or_else(
            || "not retained".to_owned(),
            |_| manifest.task_count.to_string(),
        ),
    );
    property_row(
        ui,
        "Retained results",
        &manifest.retained_result_count.to_string(),
    );
    property_row(
        ui,
        "Successful results",
        &format!("{successful_results} / {}", manifest.retained_result_count),
    );
    property_row(ui, "Receipt integrity", &manifest.integrity);

    section_header(ui, "Prepared source authority", None);
    if let Some(authority) = &manifest.authority {
        property_row(ui, "Source domain", &authority.source_domain);
        property_row(
            ui,
            "Simulation plan",
            authority
                .simulation_plan_id
                .as_deref()
                .unwrap_or("manual deck · no simulation plan"),
        );
        property_row(ui, "Project revision", &authority.project_revision);
        property_row(
            ui,
            "Prepared input digest",
            &authority.prepared_snapshot_digest,
        );
        property_row(
            ui,
            "Source content digest",
            &authority.source_content_digest,
        );
        property_row(ui, "Source check", &authority.source_check);
        property_row(ui, "Source-check digest", &authority.source_check_digest);

        section_header(ui, "Model source digests", None);
        if authority.model_sources.is_empty() {
            property_row(ui, "Model identities", "not retained");
        } else {
            for (identity, digest) in &authority.model_sources {
                property_row(ui, identity, digest);
            }
        }
    } else {
        for label in [
            "Source domain",
            "Simulation plan",
            "Project revision",
            "Prepared input digest",
            "Source content digest",
            "Source check",
            "Source-check digest",
        ] {
            property_row(ui, label, "not retained");
        }
        section_header(ui, "Model source digests", None);
        property_row(ui, "Model identities", "not retained");
    }

    result_qualification_gaps(ui);
}

fn result_qualification_gaps(ui: &mut Ui) {
    section_header(ui, "Qualification and release", None);
    for (label, value) in RESULT_QUALIFICATION_GAPS {
        property_row(ui, label, value);
    }
}

/// Whether a folded group is showing, for rows that belong to it but are
/// drawn by a different owner.
fn inspector_disclosure_open(ctx: &egui::Context, key: &str) -> bool {
    ctx.data(|data| data.get_temp::<bool>(egui::Id::new(("workbench.inspector.disclosure", key))))
        .unwrap_or(false)
}

/// A folded group of rows, named on the left and standing on the right.
///
/// A result's provenance runs to dozens of digests and is rarely the thing a
/// reader came for. Folding it behind one row keeps every fact reachable
/// without pushing the pane and trace readouts off the panel.
fn inspector_disclosure(ui: &mut Ui, key: &str, title: &str, status: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;
    let memory = egui::Id::new(("workbench.inspector.disclosure", key));
    let open = inspector_disclosure_open(ui.ctx(), key);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PANEL_SECTION_H),
        Sense::click(),
    );
    let label = format!(
        "{title}, {status}, {}",
        if open { "expanded" } else { "collapsed" }
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label.clone())
    });
    if response.clicked() {
        ui.ctx().data_mut(|data| data.insert_temp(memory, !open));
    }
    let painter = ui.painter();
    if response.hovered() {
        painter.rect_filled(rect, 0.0, c.bg_hover);
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let text = if response.hovered() {
        c.text
    } else {
        c.text_dim
    };
    painter.text(
        Pos2::new(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        if open { "\u{2304}" } else { "\u{203a}" },
        theme::mono(tokens::FS_1, FontWeight::Regular),
        text,
    );
    painter.text(
        Pos2::new(rect.left() + 22.0, rect.center().y),
        Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        text,
    );
    painter.text(
        Pos2::new(rect.right() - 10.0, rect.center().y),
        Align2::RIGHT_CENTER,
        status,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.ok,
    );
    theme::paint_focus_ring(ui, &response, rect);
    open
}

fn selected_result_artifact(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &crate::workbench::documents::result_document::ResultArtifactPresentationKey,
) {
    use crate::state::SimulationRunLifecycle;

    let Some((run_index, _, analysis)) = selected.resolve(&app.state.simulation.runs) else {
        return;
    };
    let run = &app.state.simulation.runs[run_index];
    let analysis_identity = analysis.provenance().map_or_else(
        || format!("legacy-{}", analysis.id),
        |provenance| provenance.source_instance_id().to_string(),
    );
    let source_domain =
        analysis.provenance().map_or("Legacy result", |provenance| {
            match provenance.source_domain() {
                crate::state::AnalysisResultSourceDomain::SimulationPlan => "Simulation plan",
                crate::state::AnalysisResultSourceDomain::ManualDeck => "Manual source deck",
                crate::state::AnalysisResultSourceDomain::LegacyUnclassified => {
                    "Legacy unclassified producer"
                }
            }
        });
    let lifecycle = match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "Legacy / unknown",
        SimulationRunLifecycle::Preparing => "Preparing",
        SimulationRunLifecycle::Running => "Loading",
        SimulationRunLifecycle::Cancelling => "Cancelling",
        SimulationRunLifecycle::Completed => "Complete",
        SimulationRunLifecycle::Failed => "Failed",
        SimulationRunLifecycle::Aborted => "Cancelled",
        SimulationRunLifecycle::Interrupted => "Interrupted",
    };
    let integrity = match analysis.validate_retained_evidence() {
        Ok(()) => "Verified".to_owned(),
        Err(error) => format!("Corrupted: {error}"),
    };
    let canonical = selected.canonical_name().to_owned();
    let stable_path = format!(
        "dataset/{}/analysis/{analysis_identity}/artifact/{canonical}",
        run.dataset_id
    );
    let analysis_label = analysis.label.clone();
    let analysis_kind = analysis.analysis_type.display_name().to_owned();
    let dataset = run.dataset_id.to_string();

    section_header(ui, "Typed result selection", None);
    property_row(ui, "Canonical name", &canonical);
    property_row(ui, "Analysis", &analysis_label);
    property_row(ui, "Quantity family", &analysis_kind);
    property_row(ui, "Producer", source_domain);
    property_row(ui, "Lifecycle", lifecycle);
    property_row(ui, "Integrity", &integrity);
    property_row(ui, "Dataset", &dataset);
    ui.add_space(6.0);
    if ui.button("Copy stable source path").clicked() {
        ui.ctx().copy_text(stable_path);
    }
    ui.add_space(8.0);
}

fn selected_result_trace(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &crate::workbench::documents::result_document::SelectedResultTrace,
) {
    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    let Some((analysis_index, waveform_index, analysis, waveform)) = selected.resolve(run) else {
        return;
    };

    let analysis_label = analysis.label.clone();
    let waveform_name = waveform.name.clone();
    let waveform_color = waveform.color.clone();
    let waveform_visible = waveform.visible;
    let sample_count = waveform.x.len().min(waveform.y.len());
    let statistics = finite_trace_statistics(&waveform.y);

    section_header(ui, "Selected trace", None);
    property_row(ui, "Name", &waveform_name);
    property_row(ui, "Analysis", &analysis_label);
    property_row(ui, "Dataset", &selected.dataset_id().to_string());
    property_row(ui, "Samples", &sample_count.to_string());
    property_row(
        ui,
        "Visibility",
        if waveform_visible {
            "visible"
        } else {
            "hidden"
        },
    );
    if let Some(statistics) = statistics {
        property_row(ui, "Minimum", &format_result_scalar(statistics.minimum));
        property_row(ui, "Maximum", &format_result_scalar(statistics.maximum));
        property_row(ui, "Mean", &format_result_scalar(statistics.mean));
        property_row(
            ui,
            "Peak-to-peak",
            &format_result_scalar(statistics.maximum - statistics.minimum),
        );
    }

    let t = Tokens::get(ui.ctx());
    let mut color = crate::workbench::documents::result_document::trace_color(
        &waveform_color,
        t.color.traces[waveform_index % t.color.traces.len()],
    );
    ui.horizontal(|ui| {
        ui.label("Colour");
        if egui::color_picker::color_edit_button_srgba(
            ui,
            &mut color,
            egui::color_picker::Alpha::Opaque,
        )
        .changed()
        {
            set_selected_trace_color(&mut app.state, selected, color);
        }
        ui.label(
            egui::RichText::new(format!(
                "#{:02X}{:02X}{:02X}",
                color.r(),
                color.g(),
                color.b()
            ))
            .monospace(),
        );
    });

    ui.horizontal_wrapped(|ui| {
        if ui
            .button(if waveform_visible {
                "Hide trace"
            } else {
                "Show trace"
            })
            .clicked()
        {
            crate::workbench::documents::result_document::toggle_visibility(
                &mut app.state,
                analysis_index,
                waveform_index,
            );
        }

        if is_schematic_cross_probe_candidate(selected.source_name()) {
            let signal = selected.source_name().to_owned();
            let unavailable = schematic_cross_probe_unavailability(&app.state, selected, &signal);
            let mut response =
                ui.add_enabled(unavailable.is_none(), egui::Button::new("Cross-probe net"));
            if let Some(reason) = unavailable {
                response = response.on_hover_text(reason);
            }
            if response.clicked() {
                match cross_probe_trace_to_design(app, &signal) {
                    Ok(net) => {
                        app.state.ui.toasts.success(
                            ui.ctx(),
                            "Schematic net located",
                            format!("{signal} cross-probed to {net}."),
                        );
                        app.state.push_user_message(ConsoleMessage::info(format!(
                            "Selected conductor {net} from {signal}."
                        )));
                    }
                    Err(message) => {
                        app.state.ui.toasts.warn_with_title(
                            ui.ctx(),
                            "Cannot cross-probe",
                            message.clone(),
                        );
                        app.state
                            .push_user_message(ConsoleMessage::warning(message));
                    }
                }
            }
        }
    });
}

fn active_result_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: Option<&crate::workbench::documents::result_document::SelectedResultTrace>,
) {
    let viewer = app.state.ui.results.viewer;
    // A pure evidence table — OP, specs, samples, events, the manifest — has
    // no drawn pane at all. Reporting one would have the inspector state a
    // view, a fit and a limit mask for a sheet that has none of them.
    if !crate::workbench::documents::result_document::viewer_draws_a_pane(viewer) {
        return;
    }
    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    let analysis_index = selected
        .and_then(|trace| {
            trace
                .resolve(run)
                .map(|(analysis_index, _, _, _)| analysis_index)
        })
        .or(app.state.simulation.active_analysis_idx)
        .unwrap_or(0);
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let pane_label = analysis.label.clone();
    let bound = analysis.waveforms.len();
    let visible = analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .count();
    // The unit names the pane in a unit-scoped stack; the viewer name is
    // already on the sheet tab above.
    let tokens = Tokens::get(ui.ctx());
    let facts =
        crate::workbench::documents::result_document::active_pane_facts(&tokens, &mut app.state);
    // The waveform stack keys its viewports by analysis; only the
    // single-canvas viewers keep one under a plot ordinal. Reading the wrong
    // store here reported "automatic fit" over a pinned pane and left the fit
    // button permanently disabled.
    let wave_stack = crate::workbench::documents::result_document::viewer_uses_wave_stack(viewer);
    let view = app.state.ui.results.plot_view(viewer, 0);
    let pinned = if wave_stack {
        facts.pinned.unwrap_or(false)
    } else {
        view.is_zoomed()
    };
    let view_label = if pinned {
        "manual range"
    } else {
        "automatic fit"
    };
    section_header(
        ui,
        "Active pane",
        Some(facts.unit.as_deref().unwrap_or_else(|| viewer.label())),
    );
    // The pane names the analysis it draws, which on a sheet outside the
    // waveform stack is not the one the run's analysis selector points at,
    // and counts only the traces its own axis carries.
    property_row(ui, "Pane", facts.analysis.as_deref().unwrap_or(&pane_label));
    let (visible, bound) = facts.traces.unwrap_or((visible, bound));
    property_row(ui, "Traces", &format!("{visible} visible · {bound} bound"));
    property_row(ui, "View", view_label);
    if let Some(x_viewport) = facts.x_viewport.as_deref() {
        property_row(ui, "X viewport", x_viewport);
    }
    if let Some(y_viewport) = facts.y_viewport.as_deref() {
        property_row(ui, "Y viewport", y_viewport);
    }
    if let Some(scale) = facts.scale {
        property_row(ui, "Scale", scale);
    }
    property_row(ui, "Limit mask", facts.limit_mask);
    // Where the mockup states a corner family, this workspace states the run
    // composition: overlaying runs is how a pane here comes to draw the same
    // signal more than once.
    if let Some(runs) = facts.runs {
        let composition = if runs > 1 {
            format!("{runs} runs overlaid")
        } else {
            "active run only".to_owned()
        };
        property_row(ui, "Composition", &composition);
    }
    // Explicit limits, not just a readout of the gesture that got here.
    // Comparing two runs on identical axes, or holding a decade while a
    // parameter sweeps, cannot be done by dragging.
    axis_limit_row(ui, app, &facts, PaneAxis::X, "X range");
    axis_limit_row(ui, app, &facts, PaneAxis::Y, "Y range");
    let fit_label = if wave_stack {
        "Fit active strip"
    } else {
        "Fit active pane"
    };
    if ui
        .add_enabled(pinned, egui::Button::new(fit_label))
        .clicked()
    {
        crate::workbench::documents::result_document::request_view_gesture(
            &mut app.state,
            crate::workbench::documents::result_document::ViewGesture::Fit,
        );
    }
}

/// An explicit interval for one axis of the active pane, or automatic fit.
///
/// The plot's own gestures can only ever say "a bit more than that". Holding
/// a decade while a parameter sweeps, or putting two runs on identical axes
/// so their curves can be compared at all, needs the numbers themselves.
fn axis_limit_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    facts: &crate::workbench::documents::result_document::ActivePaneFacts,
    axis: PaneAxis,
    label: &str,
) {
    use crate::workbench::documents::result_document::{
        active_axis_is_pinned, active_axis_range, set_active_axis_range,
    };

    let viewer = app.state.ui.results.viewer;
    let current = active_axis_range(&app.state, facts, axis);
    let pinned = active_axis_is_pinned(&app.state, axis);
    let committed = current.map(format_axis_range).unwrap_or_default();
    let mut text = match app.state.ui.results.axis_limit_draft.as_ref() {
        Some((draft_viewer, draft_axis, text))
            if *draft_viewer == viewer && *draft_axis == axis =>
        {
            text.clone()
        }
        _ => committed.clone(),
    };

    let parsed = parse_axis_range(&text);
    let invalid = !text.trim().is_empty() && parsed.is_none();
    let (edit, reset) = property_row_input_action(
        ui,
        label,
        &mut text,
        invalid,
        WorkbenchIcon::ZoomFit,
        "Fit this axis to the retained data",
        pinned,
        Some("This axis is already fitting its data."),
    );

    if edit.changed() {
        app.state.ui.results.axis_limit_draft = Some((viewer, axis, text.clone()));
    }
    // Commit on Enter or on leaving the field, never per keystroke: "1.2" is
    // a legal prefix of "1.2m" and three orders of magnitude away from it.
    if edit.lost_focus() {
        let tokens = Tokens::get(ui.ctx());
        if text.trim().is_empty() {
            set_active_axis_range(&tokens, &mut app.state, axis, None);
            app.state.ui.results.axis_limit_draft = None;
        } else if let Some(range) = parse_axis_range(&text) {
            set_active_axis_range(&tokens, &mut app.state, axis, Some(range));
            app.state.ui.results.axis_limit_draft = None;
        }
    }
    if reset.clicked() {
        let tokens = Tokens::get(ui.ctx());
        set_active_axis_range(&tokens, &mut app.state, axis, None);
        app.state.ui.results.axis_limit_draft = None;
    }
}

/// Render an interval the field can read back without loss.
fn format_axis_range((minimum, maximum): (f64, f64)) -> String {
    format!(
        "{} … {}",
        format_result_scalar(minimum),
        format_result_scalar(maximum)
    )
}

/// Read an interval typed as `min … max`, in engineering notation.
///
/// Accepts the ellipsis this writes back, plus `..`, a comma or plain space,
/// so a value pasted from a datasheet or a netlist parses without reformatting.
/// A reversed or degenerate interval is refused rather than silently sorted:
/// on a log axis the two are not the same request.
pub(crate) fn parse_axis_range(text: &str) -> Option<(f64, f64)> {
    let cleaned = text.replace('…', " ").replace("..", " ").replace(',', " ");
    let mut parts = cleaned.split_whitespace();
    let minimum = crate::quantity::engineering::parse_engineering_value(parts.next()?).ok()?;
    let maximum = crate::quantity::engineering::parse_engineering_value(parts.next()?).ok()?;
    if parts.next().is_some() {
        return None;
    }
    (minimum.is_finite() && maximum.is_finite() && maximum > minimum).then_some((minimum, maximum))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TraceStatistics {
    minimum: f64,
    maximum: f64,
    mean: f64,
}

fn finite_trace_statistics(values: &[f64]) -> Option<TraceStatistics> {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut count = 0_u64;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
        sum += value;
        count += 1;
    }
    (count != 0).then_some(TraceStatistics {
        minimum,
        maximum,
        mean: sum / count as f64,
    })
}

fn format_result_scalar(value: f64) -> String {
    if value == 0.0 {
        "0".to_owned()
    } else if (1.0e-3..1.0e6).contains(&value.abs()) {
        format!("{value:.7}")
    } else {
        format!("{value:.7e}")
    }
}

fn set_selected_trace_color(
    state: &mut AppState,
    selected: &crate::workbench::documents::result_document::SelectedResultTrace,
    color: Color32,
) {
    let color = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
    let Some(run_index) = state.simulation.active_run_idx else {
        return;
    };
    let Some((analysis_index, waveform_index)) =
        state.simulation.runs.get(run_index).and_then(|run| {
            selected
                .resolve(run)
                .map(|(analysis_index, waveform_index, _, _)| (analysis_index, waveform_index))
        })
    else {
        return;
    };
    if let Some(waveform) = state
        .simulation
        .runs
        .get_mut(run_index)
        .and_then(|run| run.analyses.get_mut(analysis_index))
        .and_then(|analysis| analysis.waveforms.get_mut(waveform_index))
        .filter(|waveform| waveform.name == selected.source_name())
    {
        waveform.color.clone_from(&color);
    } else {
        return;
    }
    if state.simulation.active_analysis_idx == Some(analysis_index)
        && let Some(waveform) = state
            .simulation
            .waveforms
            .iter_mut()
            .find(|waveform| waveform.name == selected.source_name())
    {
        waveform.color = color;
    }
}

fn is_schematic_cross_probe_candidate(signal: &str) -> bool {
    (signal.starts_with("V(") || signal.starts_with("I(")) && signal.ends_with(')')
}

fn schematic_cross_probe_unavailability(
    state: &AppState,
    selected: &crate::workbench::documents::result_document::SelectedResultTrace,
    signal: &str,
) -> Option<String> {
    let Some(net) = crate::schematic::view::wrapped_signal_name(signal, 'V') else {
        return Some(format!(
            "{signal} is a device current or derived quantity; no single schematic net carries it."
        ));
    };
    if !state.simulation.cross_probe.is_current_for(
        &state.workspace.active_view,
        state.schematic.topology_version(),
    ) {
        return Some(
            "The schematic changed since this result was produced; run again to cross-probe it."
                .to_owned(),
        );
    }
    let Some(run) = state.simulation.active_run() else {
        return Some("The selected result dataset is no longer retained.".to_owned());
    };
    if run.dataset_id != selected.dataset_id() || selected.resolve(run).is_none() {
        return Some(
            "The selected trace does not belong to the active immutable result dataset.".to_owned(),
        );
    }
    let Some(receipt) = run.prepared_receipt() else {
        return Some(
            "This legacy result has no authenticated project revision and cannot be cross-probed."
                .to_owned(),
        );
    };
    if receipt.project_revision() != state.workspace.project.revision() {
        return Some(
            "The selected result was produced from a different project revision; run again before cross-probing it."
                .to_owned(),
        );
    }
    if !state
        .simulation
        .cross_probe
        .net_to_points
        .iter()
        .any(|(name, points)| name.eq_ignore_ascii_case(net) && !points.is_empty())
    {
        return Some(format!("The open sheet has no conductor named {net}."));
    }
    None
}

/// Resolve against the current schematic map before navigation. A stale or
/// derived signal therefore leaves the engineer in Results with an exact
/// explanation. On success, the Design surface opens; a remembered split
/// remains active and projects Design + the same canonical Results document.
fn cross_probe_trace_to_design(app: &mut RSpiceApp, signal: &str) -> Result<String, String> {
    let selected = app
        .state
        .ui
        .results
        .valid_selected_trace(&app.state.simulation)
        .ok_or_else(|| "No exact retained result trace is selected.".to_owned())?;
    if let Some(reason) = schematic_cross_probe_unavailability(&app.state, selected, signal) {
        return Err(reason);
    }
    let net = crate::schematic::view::select_signal_conductor(&mut app.state, signal)
        .map_err(|error| error.message(signal))?;
    if app.state.workbench.workspace != Workspace::Design {
        Command::OpenWorkspace(Workspace::Design).execute(app);
    }
    Ok(net)
}

fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.workbench.verification_page {
        VerificationPage::Yield => yield_details(ui, app),
        VerificationPage::Corners => {
            let result = app
                .state
                .simulation
                .active_run()
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Corner));
            let points = result
                .and_then(|analysis| analysis.waveforms.first())
                .map_or(0, |waveform| waveform.x.len());
            section_header(
                ui,
                "Corner details",
                Some(if result.is_some() {
                    "retained"
                } else {
                    "not run"
                }),
            );
            property_row(ui, "Points", &points.to_string());
            property_row(
                ui,
                "Signals",
                &result
                    .map_or(0, |analysis| analysis.waveforms.len())
                    .to_string(),
            );
            property_row(
                ui,
                "Execution",
                if result.is_some_and(|analysis| analysis.success) {
                    "complete"
                } else if result.is_some() {
                    "failed / incomplete"
                } else {
                    "no evidence"
                },
            );
        }
        VerificationPage::Tuning => {
            let session = &app.state.workbench.verification;
            let variable_count = session.tuning_variables.len();
            let dirty_count = session
                .tuning_variables
                .iter()
                .filter(|draft| draft.is_dirty())
                .count();
            let invalid_count = session
                .tuning_variables
                .iter()
                .filter(|draft| draft.validation_error.is_some())
                .count();
            section_header(
                ui,
                "Parameter tuning",
                Some(if dirty_count == 0 {
                    "committed baseline"
                } else {
                    "provisional"
                }),
            );
            property_row(ui, "Design variables", &variable_count.to_string());
            property_row(ui, "Provisional changes", &dirty_count.to_string());
            property_row(
                ui,
                "Validation",
                if invalid_count == 0 {
                    "ready"
                } else {
                    "blocked"
                },
            );
            property_row(
                ui,
                "Commit contract",
                "one plan revision + retained production run",
            );
        }
        VerificationPage::Optimization => {
            section_header(ui, "Optimization details", None);
            let retained =
                app.state.simulation.active_run().and_then(|run| {
                    verified_analysis(run, crate::state::AnalysisType::Optimization)
                });
            if retained.is_some() {
                property_row(ui, "State", "retained optimization result");
            } else {
                property_row(ui, "State", "no retained optimization evidence");
            }
        }
        VerificationPage::Reliability => {
            let active_run = app.state.simulation.active_run();
            let has_soa = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Soa))
                .is_some();
            let has_aging = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Reliability))
                .is_some();
            section_header(
                ui,
                "Reliability details",
                Some(if has_soa || has_aging {
                    "execution receipt only"
                } else {
                    "not run"
                }),
            );
            property_row(
                ui,
                "SOA evidence",
                if has_soa {
                    "payload unavailable"
                } else {
                    "unavailable"
                },
            );
            property_row(ui, "SOA verdict", "blocked");
            property_row(
                ui,
                "Aging evidence",
                if has_aging {
                    "payload unavailable"
                } else {
                    "unavailable"
                },
            );
            property_row(ui, "Reliability verdict", "blocked");
            property_row(ui, "Geometry", "owned by Physical DRC");
        }
        VerificationPage::Regression => {
            section_header(ui, "Regression details", Some("main"));
            property_row(
                ui,
                "Retained runs",
                &app.state.simulation.runs.len().to_string(),
            );
            property_row(
                ui,
                "Baseline",
                if app.state.simulation.runs.len() >= 2 {
                    "selectable immutable run"
                } else {
                    "unavailable"
                },
            );
            property_row(ui, "Verdict", "requires configured tolerance contract");
        }
        VerificationPage::Drc => {
            section_header(ui, "Physical evidence", Some("unavailable"));
            property_row(ui, "Layout source", "not attached");
            property_row(ui, "Rule deck", "not attached");
            property_row(ui, "Marker database", "unavailable");
            property_row(ui, "Sign-off", "blocked");
            section_header(ui, "Release consumption", None);
            ui.label("Physical verification owns marker disposition. Release closure consumes this exact database and cannot override it.");
        }
    }
}

fn verified_analysis(
    run: &crate::state::SimulationRun,
    analysis_type: crate::state::AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    run.analyses.iter().find(|analysis| {
        analysis.analysis_type == analysis_type && analysis.success && analysis.provenance.is_some()
    })
}

fn yield_details(ui: &mut Ui, app: &RSpiceApp) {
    let run = app.state.simulation.active_run();
    let provenance = run.and_then(|run| {
        app.state.simulation.yield_provenance.filter(|provenance| {
            provenance.source_run_id == run.run_id && provenance.source_dataset_id == run.dataset_id
        })
    });
    let results = if provenance.is_some() {
        app.state.simulation.yield_results.as_slice()
    } else {
        &[]
    };

    let status = if provenance.is_some() {
        "complete"
    } else {
        "no active evidence"
    };
    section_header(ui, "Run definition", Some(status));
    if let Some(provenance) = provenance {
        property_row(
            ui,
            "Samples",
            &format!(
                "{} / {}",
                provenance.runs_completed, provenance.runs_requested
            ),
        );
        property_row(ui, "Seed", &format!("0x{:X}", provenance.seed));
        property_row(ui, "Sampling", provenance.sampling_mode.display_name());
        let (passing, total) = joint_yield(results).unwrap_or((0, 0));
        let interval = wilson_interval_95(passing, total);
        property_row(
            ui,
            "Confidence",
            &interval.map_or_else(
                || "Unavailable".to_owned(),
                |(low, high)| format!("95% Wilson · {:.2}–{:.2}%", low * 100.0, high * 100.0),
            ),
        );
        if let Some(run) = run {
            property_row(ui, "Duration", &format!("{:.3} s", run.elapsed_time));
            property_row(ui, "Dataset", &run.dataset_id.to_string());
        }
    } else {
        property_row(ui, "Samples", "No retained Monte Carlo dataset");
        property_row(ui, "Seed", "Unavailable");
        property_row(ui, "Sampling", "Unavailable");
        property_row(ui, "Confidence", "Unavailable");
    }

    let failures = worst_samples(results);
    let failed_count = joint_yield(results).map_or(0, |(passing, total)| total - passing);
    let failed_label = format!("{failed_count} fail");
    section_header(ui, "Worst samples", Some(&failed_label));
    if failures.is_empty() {
        property_row(
            ui,
            "Evidence",
            if results.is_empty() {
                "No active sample trail"
            } else {
                "No failing samples"
            },
        );
    } else {
        for failure in failures.iter().take(3) {
            failure_row(ui, failure);
        }
    }

    section_header(ui, "Specification yields", None);
    if results.is_empty() {
        property_row(ui, "Evidence", "No active-dataset yield results");
    } else {
        for result in results {
            property_row(
                ui,
                &result.spec.target,
                &format!(
                    "{:.2}% · {} / {}",
                    result.yield_percent, result.pass_count, result.total_runs
                ),
            );
        }
    }
}

#[derive(Debug)]
struct FailedSample {
    target: String,
    unit: String,
    sample_index: usize,
    value: f64,
    normalized_margin: f64,
}

fn worst_samples(results: &[crate::services::yield_manager::YieldResult]) -> Vec<FailedSample> {
    let mut failures = results
        .iter()
        .flat_map(|result| {
            result
                .samples
                .iter()
                .copied()
                .enumerate()
                .filter(|(index, value)| {
                    result.trail.get(*index).is_some_and(|passes| !passes) && value.is_finite()
                })
                .filter_map(|(index, value)| {
                    normalized_yield_margin(&result.spec, value).map(|normalized_margin| {
                        FailedSample {
                            target: result.spec.target.clone(),
                            unit: result.spec.unit.clone(),
                            sample_index: index,
                            value,
                            normalized_margin,
                        }
                    })
                })
        })
        .collect::<Vec<_>>();
    failures.sort_by(|a, b| {
        a.normalized_margin
            .partial_cmp(&b.normalized_margin)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.target.cmp(&b.target))
            .then_with(|| a.sample_index.cmp(&b.sample_index))
    });
    failures
}

fn normalized_yield_margin(
    spec: &crate::services::yield_manager::YieldSpec,
    value: f64,
) -> Option<f64> {
    const SCALE_FLOOR: f64 = 1.0e-30;
    match (spec.min, spec.max) {
        (Some(min), Some(max)) if max > min => {
            Some((value - min).min(max - value) / (max - min).max(SCALE_FLOOR))
        }
        (Some(min), None) => Some((value - min) / min.abs().max(SCALE_FLOOR)),
        (None, Some(max)) => Some((max - value) / max.abs().max(SCALE_FLOOR)),
        (Some(_), Some(_)) | (None, None) => None,
    }
}

const FAILURE_ROW_PADDING_X: f32 = 9.0;
const FAILURE_ROW_PADDING_Y: f32 = 8.0;
const FAILURE_ROW_COLUMN_GAP: f32 = 8.0;
const FAILURE_ROW_DETAIL_GAP: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct FailureRowColumns {
    target_width: f32,
    margin_width: f32,
}

fn failure_row_columns(row_width: f32, measured_margin_width: f32) -> FailureRowColumns {
    let content_width = (row_width - FAILURE_ROW_PADDING_X * 2.0).max(1.0);
    let margin_width = measured_margin_width.max(1.0).min(content_width);
    let target_width = (content_width - margin_width - FAILURE_ROW_COLUMN_GAP).max(1.0);
    FailureRowColumns {
        target_width,
        margin_width,
    }
}

fn failure_row(ui: &mut Ui, failure: &FailedSample) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let title = format!("#{:04} · {}", failure.sample_index + 1, failure.target);
    let margin = format!("{:+.2}%", failure.normalized_margin * 100.0);
    let detail = format!("observed {:.6} {}", failure.value, failure.unit);
    let title_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let margin_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measured_margin = ui
        .painter()
        .layout_no_wrap(margin, margin_font, t.color.err);
    let columns = failure_row_columns(width, measured_margin.size().x);
    let title_galley = ui
        .painter()
        .layout(title, title_font, t.color.text, columns.target_width);
    let content_width = (width - FAILURE_ROW_PADDING_X * 2.0).max(1.0);
    let detail_galley = ui
        .painter()
        .layout(detail, detail_font, t.color.text_faint, content_width);
    let title_height = title_galley.size().y.max(measured_margin.size().y);
    let natural_height = FAILURE_ROW_PADDING_Y * 2.0
        + title_height
        + FAILURE_ROW_DETAIL_GAP
        + detail_galley.size().y;
    let minimum_height = if t.metrics.ctl_h >= 44.0 { 58.0 } else { 47.0 };
    let height = natural_height.max(minimum_height);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "Sample {}. {}. Value {} {}. Margin {:+.2}%",
                failure.sample_index + 1,
                failure.target,
                failure.value,
                failure.unit,
                failure.normalized_margin * 100.0
            ),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let title_top = rect.top() + FAILURE_ROW_PADDING_Y;
    let target_rect = Rect::from_min_size(
        Pos2::new(rect.left() + FAILURE_ROW_PADDING_X, title_top),
        Vec2::new(columns.target_width, title_height),
    );
    ui.painter().with_clip_rect(target_rect).galley(
        target_rect.left_top(),
        title_galley,
        t.color.text,
    );
    let margin_rect = Rect::from_min_size(
        Pos2::new(
            rect.right() - FAILURE_ROW_PADDING_X - columns.margin_width,
            title_top,
        ),
        Vec2::new(columns.margin_width, title_height),
    );
    ui.painter().with_clip_rect(margin_rect).galley(
        Pos2::new(
            margin_rect.right() - measured_margin.size().x,
            margin_rect.top(),
        ),
        measured_margin,
        t.color.err,
    );
    let detail_top = title_top + title_height + FAILURE_ROW_DETAIL_GAP;
    let detail_rect = Rect::from_min_max(
        Pos2::new(rect.left() + FAILURE_ROW_PADDING_X, detail_top),
        Pos2::new(rect.right() - FAILURE_ROW_PADDING_X, rect.bottom()),
    );
    ui.painter().with_clip_rect(detail_rect).galley(
        detail_rect.left_top(),
        detail_galley,
        t.color.text_faint,
    );
    response
}

fn joint_yield(results: &[crate::services::yield_manager::YieldResult]) -> Option<(usize, usize)> {
    let total = results.first()?.total_runs;
    if total == 0
        || results
            .iter()
            .any(|result| result.total_runs != total || result.trail.len() != total)
    {
        return None;
    }
    Some((
        (0..total)
            .filter(|index| results.iter().all(|result| result.trail[*index]))
            .count(),
        total,
    ))
}

fn wilson_interval_95(successes: usize, total: usize) -> Option<(f64, f64)> {
    if total == 0 || successes > total {
        return None;
    }
    let n = total as f64;
    let p = successes as f64 / n;
    let z = 1.959_963_984_540_054_f64;
    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;
    let half = z * ((p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt()) / denominator;
    Some(((center - half).max(0.0), (center + half).min(1.0)))
}

fn models(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Model binding", None);
    property_row(
        ui,
        "Library",
        app.state
            .model_library_manager
            .selected_library
            .as_deref()
            .unwrap_or("None"),
    );
    let selected_library = app.state.model_library_manager.current_library().cloned();
    if let Some(library) = selected_library {
        property_row(
            ui,
            "PDK",
            if library.pdk_name.is_empty() {
                "Unspecified"
            } else {
                &library.pdk_name
            },
        );
        property_row(
            ui,
            "Technology",
            if library.technology_node.is_empty() {
                "Unspecified"
            } else {
                &library.technology_node
            },
        );
        property_row(
            ui,
            "Version",
            if library.version.is_empty() {
                "Unspecified"
            } else {
                &library.version
            },
        );
        property_row(ui, "Models", &library.model_count().to_string());
        property_row(ui, "Corners", &library.corner_count().to_string());
        property_row(
            ui,
            "Selected corner",
            library.selected_corner.as_deref().unwrap_or("None"),
        );
        if let Some(model_name) = &app.state.workbench.selected_model
            && let Some(model) = library.models.get(model_name)
        {
            section_header(ui, "Selected model", None);
            property_row(ui, "Name", &model.name);
            property_row(ui, "Type", &format!("{:?}", model.model_type));
            property_row(ui, "Level", &format!("{:?}", model.level));
            property_row(ui, "Parameters", &model.parameters.len().to_string());
            if let Some(vdd) = model.vdd {
                property_row(ui, "Nominal VDD", &format!("{vdd:.6} V"));
            }
        }
    }
}

fn code_workspace_inspector(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.ui.code_workspace.page
        == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    {
        netlist(ui, app);
        return;
    }

    let messages = app.state.ui.messages();
    design_section_header(
        ui,
        &messages.text(MessageId::CodeInspectorExactContext),
        None,
    );
    let Some(context) = crate::workbench::commands::code_context::resolve(app) else {
        muted_inspector_copy(
            ui,
            &messages.text(MessageId::CodeInspectorContextUnavailable),
        );
        return;
    };
    let page = messages.text(match context.page {
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
            MessageId::CodePageNetlist
        }
        crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA => {
            MessageId::CodePageVerilogA
        }
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
            MessageId::CodePageAutomation
        }
    });
    let ownership = messages.text(match context.ownership {
        crate::workbench::commands::code_context::CodeDocumentOwnership::GeneratedReadOnly => {
            MessageId::CodeInspectorGeneratedReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ProjectOwned => {
            MessageId::CodeInspectorProjectOwned
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ExternalReadOnly => {
            MessageId::CodeInspectorExternalReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ComparisonReadOnly => {
            MessageId::CodeInspectorComparisonReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::GovernedReadOnly => {
            MessageId::CodeInspectorGovernedReadOnly
        }
    });
    property_row(ui, &messages.text(MessageId::CodeInspectorPage), &page);
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorLogicalPath),
        &context.logical_path,
    );
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorDocumentIdentity),
        &context.document_identity,
    );
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorRevision),
        &context.revision.to_string(),
    );
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorContentDigest),
        &context.content_digest.to_string(),
    );
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorOwnership),
        &ownership,
    );

    design_section_header(
        ui,
        &messages.text(MessageId::CodeInspectorCommandAvailability),
        None,
    );
    for (label, available) in [
        (
            messages.text(MessageId::CodeInspectorFind),
            context.capabilities.find,
        ),
        (
            messages.text(MessageId::CodeInspectorValidate),
            context.capabilities.validate,
        ),
        (
            messages.text(MessageId::CodeInspectorExecute),
            context.capabilities.execute,
        ),
        (
            messages.text(MessageId::CodeInspectorSave),
            context.capabilities.save,
        ),
        (
            messages.text(MessageId::CodeInspectorCompare),
            context.capabilities.compare_revisions,
        ),
    ] {
        property_row(
            ui,
            &label,
            &messages.text(if available {
                MessageId::CodeInspectorAvailable
            } else {
                MessageId::CodeInspectorUnavailable
            }),
        );
    }
}

fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::{
        ActiveNetlistDocument, DiagnosticSeverity,
    };

    let messages = app.state.ui.messages();
    const DIAGNOSTIC_PREVIEW_LIMIT: usize = 20;
    let diagnostics = std::sync::Arc::clone(&app.state.ui.netlist.diagnostics);
    let summary = diagnostics.summary();

    if summary.current_errors > 0 {
        diagnostic_section_header(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistErrors),
            summary.current_errors,
            DiagnosticSeverity::Error,
        );
        for diagnostic in diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.is_current() && diagnostic.severity == DiagnosticSeverity::Error
            })
            .take(DIAGNOSTIC_PREVIEW_LIMIT)
        {
            diagnostic_row(ui, diagnostic);
        }
        if summary.current_errors > DIAGNOSTIC_PREVIEW_LIMIT {
            empty_diagnostic_row(
                ui,
                &format!(
                    "{} more errors · open Problems for the complete virtualized list",
                    summary.current_errors - DIAGNOSTIC_PREVIEW_LIMIT
                ),
            );
        }
    }

    diagnostic_section_header(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistAdvisories),
        summary.current_advisories(),
        DiagnosticSeverity::Warning,
    );
    if summary.current_advisories() == 0 {
        empty_diagnostic_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistNoAdvisories),
        );
    } else {
        for diagnostic in diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.is_current() && diagnostic.severity != DiagnosticSeverity::Error
            })
            .take(DIAGNOSTIC_PREVIEW_LIMIT)
        {
            diagnostic_row(ui, diagnostic);
        }
        if summary.current_advisories() > DIAGNOSTIC_PREVIEW_LIMIT {
            empty_diagnostic_row(
                ui,
                &format!(
                    "{} more advisories · open Problems for the complete virtualized list",
                    summary.current_advisories() - DIAGNOSTIC_PREVIEW_LIMIT
                ),
            );
        }
    }

    design_section_header(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistParameterExploration),
        Some(&messages.text(crate::workbench::MessageId::NetlistDedicatedWorkspace)),
    );
    muted_inspector_copy(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistParameterExplorationHint),
    );

    if crate::workbench::documents::netlist_document::active_dependency(&app.state).is_some() {
        dependency_provenance(ui, &app.state);
    } else {
        match app.state.ui.netlist.active_document {
            ActiveNetlistDocument::Generated => generated_provenance(ui, &app.state),
            ActiveNetlistDocument::OwnedSource => owned_source_provenance(ui, &app.state),
            ActiveNetlistDocument::GeneratedDiff => generated_provenance(ui, &app.state),
        }
    }
}

fn diagnostic_section_header(
    ui: &mut Ui,
    title: &str,
    count: usize,
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PANEL_SECTION_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    ui.painter().text(
        Pos2::new(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        title.to_ascii_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );

    let tone = if count == 0 {
        t.color.ok
    } else {
        diagnostic_tone(&t, severity)
    };
    let count_text = count.to_string();
    let count_galley = ui.painter().layout_no_wrap(
        count_text,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        tone,
    );
    let count_width = count_galley.size().x;
    let count_pos = Pos2::new(rect.right() - 10.0, rect.center().y);
    ui.painter().galley(
        Pos2::new(
            count_pos.x - count_width,
            count_pos.y - count_galley.size().y * 0.5,
        ),
        count_galley,
        tone,
    );
    ui.painter().circle_filled(
        Pos2::new(count_pos.x - count_width - 8.5, rect.center().y),
        2.5,
        tone,
    );
}

fn diagnostic_row(
    ui: &mut Ui,
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) {
    const ICON_COLUMN_W: f32 = 14.0;
    const COLUMN_GAP: f32 = 7.0;
    const PADDING_X: f32 = 9.0;
    const PADDING_Y: f32 = 8.0;
    const DETAIL_GAP: f32 = 3.0;

    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let text_width = (width - PADDING_X * 2.0 - ICON_COLUMN_W - COLUMN_GAP).max(1.0);
    let message_galley = ui.painter().layout(
        diagnostic.message.clone(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        text_width,
    );
    let location = diagnostic_location(diagnostic);
    let location_galley = ui.painter().layout(
        location.clone(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let message_height = message_galley.size().y;
    let height = PADDING_Y * 2.0 + message_height + DETAIL_GAP + location_galley.size().y;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "{}: {}. {location}",
                diagnostic_severity_name(diagnostic.severity),
                diagnostic.message
            ),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let icon = match diagnostic.severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            WorkbenchIcon::Info
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            WorkbenchIcon::Warning
        }
    };
    icon.paint(
        ui.painter(),
        Rect::from_min_size(
            Pos2::new(rect.left() + PADDING_X, rect.top() + PADDING_Y),
            Vec2::splat(ICON_COLUMN_W),
        ),
        diagnostic_tone(&t, diagnostic.severity),
    );

    let text_x = rect.left() + PADDING_X + ICON_COLUMN_W + COLUMN_GAP;
    ui.painter().galley(
        Pos2::new(text_x, rect.top() + PADDING_Y),
        message_galley,
        t.color.text,
    );
    ui.painter().galley(
        Pos2::new(text_x, rect.top() + PADDING_Y + message_height + DETAIL_GAP),
        location_galley,
        t.color.text_faint,
    );
}

fn empty_diagnostic_row(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let galley = ui.painter().layout(
        message.to_owned(),
        font,
        t.color.text_faint,
        (ui.available_width() - 20.0).max(1.0),
    );
    let height = (galley.size().y + 16.0).max(31.0);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), message)
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().galley(
        Pos2::new(rect.left() + 10.0, rect.top() + 8.0),
        galley,
        t.color.text_faint,
    );
}

fn muted_inspector_copy(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (ui.available_width() - 20.0).max(1.0),
    );
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), galley.size().y + 16.0),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().galley(
        Pos2::new(rect.left() + 10.0, rect.top() + 8.0),
        galley,
        t.color.text_dim,
    );
}

fn generated_provenance(ui: &mut Ui, state: &AppState) {
    design_section_header(ui, "Generated provenance", None);
    let netlist = &state.ui.netlist;
    let canonical = netlist.generated_document.as_ref();
    let source = canonical
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map().first())
        .map(|entry| entry.view_identity().to_owned())
        .unwrap_or_else(|| "Unavailable".to_owned());
    property_row(ui, "Source cell/view", &source);
    property_row(ui, "Input revision", &generated_input_revision(state));
    property_row(
        ui,
        "Input digest",
        &canonical
            .and_then(|document| document.generated_artifact())
            .map(|artifact| short_digest(artifact.provenance().input().digest()))
            .unwrap_or_else(|| "Unavailable".to_owned()),
    );
    property_row(ui, "Generator state", generated_state(state));
    property_row(
        ui,
        "Netlist digest",
        &canonical
            .and_then(|document| document.generated_artifact())
            .map(|artifact| short_digest(artifact.content_digest()))
            .unwrap_or_else(|| "Unavailable".to_owned()),
    );
}

fn owned_source_provenance(ui: &mut Ui, state: &AppState) {
    design_section_header(ui, "Owned source provenance", None);
    let source = &state.simulation.netlist_content;
    let source_digest =
        crate::workbench::documents::netlist_document::source_content_digest(source);
    property_row(
        ui,
        "Source origin",
        &state.workspace.netlist_source_path.as_ref().map_or_else(
            || "Project-owned source".to_owned(),
            |path| path.display().to_string(),
        ),
    );
    property_row(
        ui,
        "Project revision",
        &state.workspace.project.revision().get().to_string(),
    );
    if let Some(document) = state.ui.netlist.owned_document.as_ref() {
        property_row(
            ui,
            "Document revision",
            &document.revision().get().to_string(),
        );
        property_row(
            ui,
            "Generated base",
            &document
                .generated_artifact()
                .map(|artifact| short_digest(artifact.content_digest()))
                .unwrap_or_else(|| "None · authored netlist-first".to_owned()),
        );
    }
    property_row(
        ui,
        "Document state",
        owned_source_state(state, source_digest),
    );
    property_row(ui, "Netlist digest", &short_digest(source_digest));
    property_row(
        ui,
        "Saved digest",
        &state
            .ui
            .netlist
            .externally_saved_content_digest
            .map(short_digest)
            .unwrap_or_else(|| "Not published".to_owned()),
    );
}

fn dependency_provenance(ui: &mut Ui, state: &AppState) {
    let Some(dependency) = crate::workbench::documents::netlist_document::active_dependency(state)
    else {
        return;
    };
    let messages = state.ui.messages();
    design_section_header(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistIncludeProvenance),
        None,
    );
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistDisplayName),
        dependency.locator().display_name(),
    );
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistLogicalIdentity),
        dependency.locator().logical_identity(),
    );
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistRequestedAs),
        dependency.requested_locator(),
    );
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistOrigin),
        dependency
            .locator()
            .native_origin()
            .map(str::to_owned)
            .unwrap_or_else(|| {
                messages.text(crate::workbench::MessageId::NetlistRetainedAuthenticatedSource)
            })
            .as_str(),
    );
    let relationship = messages.text(if dependency.direct_include_index().is_some() {
        crate::workbench::MessageId::NetlistDirectInclude
    } else {
        crate::workbench::MessageId::NetlistTransitiveInclude
    });
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistRelationship),
        &relationship,
    );
    if let Some(parent) = dependency.parent() {
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistParent),
            parent.display_name(),
        );
    }
    let owned = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.owned_include(dependency.locator().logical_identity()));
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistOwnership),
        &owned.map_or_else(
            || {
                messages.text(match dependency.authority() {
                    crate::state::DependencySourceAuthority::External => {
                        crate::workbench::MessageId::NetlistExternalReferenceReadOnly
                    }
                    crate::state::DependencySourceAuthority::Vendor => {
                        crate::workbench::MessageId::NetlistVendorSourceReadOnly
                    }
                    crate::state::DependencySourceAuthority::TechnologyPackage => {
                        crate::workbench::MessageId::NetlistTechnologyPackageReadOnly
                    }
                    crate::state::DependencySourceAuthority::StandardLibrary => {
                        crate::workbench::MessageId::NetlistStandardLibraryReadOnly
                    }
                })
            },
            |_| messages.text(crate::workbench::MessageId::NetlistProjectOwnedEditable),
        ),
    );
    if let Some(owned) = owned {
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistDocumentId),
            &owned.document_id.to_string(),
        );
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistDocumentRevision),
            &owned.revision.to_string(),
        );
    }
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistContentDigest),
        &dependency
            .resolution()
            .content_digest()
            .map(short_digest)
            .unwrap_or_else(|| messages.text(crate::workbench::MessageId::NetlistUnresolved)),
    );
}

fn generated_state(state: &AppState) -> &'static str {
    let netlist = &state.ui.netlist;
    if netlist.generated_source.is_empty() {
        "not generated"
    } else if netlist.generation_error.is_some() {
        "blocked · prior artifact retained"
    } else if netlist.generated_input_digest != netlist.current_generation_input_digest {
        "stale · refresh pending"
    } else {
        let digest = crate::workbench::documents::netlist_document::source_content_digest(
            &netlist.generated_source,
        );
        if netlist.validation.as_ref().is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == state.workspace.project.revision().get()
        }) {
            "generated · validated"
        } else {
            "generated · current"
        }
    }
}

fn generated_input_revision(state: &AppState) -> String {
    state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .map(|artifact| artifact.provenance().input().revision().get().to_string())
        .unwrap_or_else(|| "Unavailable".to_owned())
}

fn owned_source_state(state: &AppState, digest: crate::product::ContentDigest) -> &'static str {
    crate::workbench::documents::netlist_document::owned_netlist_publication_state(state, digest)
        .label()
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    let digest = digest.to_string();
    format!("{}…{}", &digest[..8], &digest[digest.len() - 4..])
}

fn diagnostic_location(
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) -> String {
    let location = match (
        diagnostic.source_line.or(diagnostic.line),
        diagnostic.column,
    ) {
        (Some(line), Some(column)) => format!("line {} · column {}", line + 1, column + 1),
        (Some(line), None) => format!("line {}", line + 1),
        (None, _) => "document scope".to_owned(),
    };
    match diagnostic.source_path.as_deref() {
        Some(path) if diagnostic.source_line.is_some() => {
            format!("{} · {location}", path.display())
        }
        Some(path) => path.display().to_string(),
        None => location,
    }
}

fn diagnostic_tone(
    tokens: &Tokens,
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> Color32 {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint => {
            tokens.color.text_dim
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            tokens.color.info
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => {
            tokens.color.warn
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            tokens.color.err
        }
    }
}

fn diagnostic_severity_name(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> &'static str {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint => "Hint",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => "Information",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => "Warning",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => "Error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_binding_uses_the_resolved_project_global_provider() {
        use crate::state::model_library::{
            DeviceModel, ModelConsumerScope, ModelLibrary, ModelType,
        };

        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        for library_name in ["alpha", "beta"] {
            let mut library = ModelLibrary::new(library_name);
            library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
            app.state.model_library_manager.add_library(library);
        }
        app.state
            .model_library_manager
            .resolve_definition_provider(
                ModelConsumerScope::PrimitiveModel,
                "shared_diode",
                "beta",
                "Test selects the executable provider.",
            )
            .expect("provider decision");
        let component_id = 9_001;
        app.state.schematic.components.push(
            Component::new(
                component_id,
                ComponentType::Diode,
                crate::state::Point::origin(),
            )
            .with_name_value("D9001", ""),
        );

        bind_component_model_from_catalog(&mut app, component_id, "beta", "shared_diode")
            .expect("resolved provider binds");
        let params = crate::state::parse_params_string(
            &app.state
                .schematic
                .components
                .iter()
                .find(|component| component.id == component_id)
                .expect("component")
                .params,
        );
        assert_eq!(
            params.get("model").map(String::as_str),
            Some("shared_diode")
        );
        assert_eq!(
            params.get("model_library").map(String::as_str),
            Some("beta")
        );

        let error =
            bind_component_model_from_catalog(&mut app, component_id, "alpha", "shared_diode")
                .expect_err("losing provider cannot bind");
        assert!(error.contains("project-global provider 'beta'"));
    }

    #[test]
    fn catalog_binding_rejects_an_incompatible_primitive_without_mutation() {
        use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        let mut library = ModelLibrary::new("models");
        library.add_model(DeviceModel::new("junction", ModelType::Diode));
        app.state.model_library_manager.add_library(library);
        let component_id = 9_002;
        app.state.schematic.components.push(
            Component::new(
                component_id,
                ComponentType::Resistor,
                crate::state::Point::origin(),
            )
            .with_name_value("R9002", "1k"),
        );
        let topology_before = app.state.schematic.topology_version();
        let params_before = app
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)
            .expect("test resistor")
            .params
            .clone();

        let error = bind_component_model_from_catalog(&mut app, component_id, "models", "junction")
            .expect_err("a diode model cannot bind to a resistor");

        assert!(error.contains("incompatible with the selected Resistor instance"));
        assert_eq!(app.state.schematic.topology_version(), topology_before);
        assert_eq!(
            app.state
                .schematic
                .components
                .iter()
                .find(|component| component.id == component_id)
                .expect("test resistor")
                .params,
            params_before
        );
    }

    #[test]
    fn a_typed_interval_round_trips_through_the_field_that_shows_it() {
        for range in [
            (0.0, 5.0),
            (-1.5e-3, 2.25e-3),
            (1.0e6, 1.0e9),
            (-1.0e-15, 1.0e-14),
        ] {
            let text = format_axis_range(range);
            let parsed = parse_axis_range(&text)
                .unwrap_or_else(|| panic!("{text} did not read back as an interval"));
            assert!(
                (parsed.0 - range.0).abs() <= range.0.abs() * 1.0e-6
                    && (parsed.1 - range.1).abs() <= range.1.abs() * 1.0e-6,
                "{range:?} rendered as {text} and read back as {parsed:?}"
            );
        }
    }

    #[test]
    fn an_interval_parses_from_the_separators_an_engineer_would_paste() {
        for text in ["1m … 5m", "1m..5m", "1m,5m", "1m 5m", "  1m   5m  "] {
            assert_eq!(
                parse_axis_range(text),
                Some((1.0e-3, 5.0e-3)),
                "failed on {text:?}"
            );
        }
        assert_eq!(parse_axis_range("-2.5 -1.5"), Some((-2.5, -1.5)));
    }

    #[test]
    fn a_reversed_or_degenerate_interval_is_refused_rather_than_sorted() {
        // On a log axis "5m … 1m" is not the same request as "1m … 5m", and
        // quietly swapping them would hide the typo behind a correct-looking
        // plot.
        assert_eq!(parse_axis_range("5m … 1m"), None);
        assert_eq!(parse_axis_range("1m … 1m"), None);
        assert_eq!(parse_axis_range("1m"), None);
        assert_eq!(parse_axis_range("1m … 2m … 3m"), None);
        assert_eq!(parse_axis_range(""), None);
        assert_eq!(parse_axis_range("wide open"), None);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn inspector_header_exposes_its_workspace_heading() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);

        let nodes = ctx
            .run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(312.0);
                    header(ui, &mut app);
                });
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Heading
                && node.label() == Some("Inspector")
                && node.level() == Some(2)
        }));
    }

    fn result_app_with_current_out_map(split: bool) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.split_with_results = split;
        app.state.workbench.activate(Workspace::Results);
        let project_revision = app.state.workspace.project.revision();
        let analysis_id = crate::product::AnalysisInstanceId::new();
        let receipt = crate::state::PreparedRunReceipt::new(
            crate::state::AnalysisResultSourceDomain::SimulationPlan,
            Some(crate::product::SimulationPlanId::new()),
            project_revision,
            crate::product::ContentDigest::from_bytes([0x11; 32]),
            crate::product::ContentDigest::from_bytes([0x22; 32]),
            crate::state::PreparedSourceCheckReceipt::SchematicDrc(
                crate::product::ContentDigest::from_bytes([0x33; 32]),
            ),
            vec![
                crate::state::PreparedRunTaskReceipt::new(
                    analysis_id,
                    project_revision,
                    Vec::new(),
                    1,
                    crate::product::ContentDigest::from_bytes([0x44; 32]),
                )
                .expect("valid prepared task"),
            ],
        )
        .expect("valid prepared run receipt");
        app.state
            .simulation
            .start_prepared_run(receipt)
            .add_analysis(
                crate::state::AnalysisResult::new(
                    1,
                    crate::state::AnalysisType::Transient,
                    "retained TRAN",
                )
                .with_provenance(
                    crate::state::AnalysisResultProvenance::new_with_source_domain(
                        crate::state::AnalysisResultSourceDomain::SimulationPlan,
                        analysis_id,
                        project_revision,
                        crate::product::ContentDigest::from_bytes([0x44; 32]),
                        Vec::new(),
                    )
                    .expect("valid prepared analysis provenance"),
                )
                .with_waveforms(vec![crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![0.0, 1.0],
                    "#ffbd2e",
                )]),
            );
        app.state.ui.results.selected_trace = Some(
            crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
                app.state.simulation.active_run().expect("retained run"),
                0,
                0,
            )
            .expect("selected retained trace"),
        );
        let a = crate::state::Point::new(0, 0);
        let b = crate::state::Point::new(40, 0);
        app.state
            .schematic
            .wires
            .push(crate::state::Wire::new(91, vec![a, b]));
        app.state.simulation.cross_probe.update(
            app.state.workspace.active_view.clone(),
            std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
            std::collections::HashMap::new(),
            app.state.schematic.topology_version(),
        );
        app
    }

    #[test]
    fn one_step_frames_every_inspector_section_body() {
        assert_eq!(INSPECTOR_SECTION_PADDING, 8.0);
    }

    #[test]
    fn result_trace_statistics_ignore_non_finite_samples() {
        let statistics = finite_trace_statistics(&[f64::NAN, -2.0, 1.0, f64::INFINITY, 4.0])
            .expect("three finite samples");

        assert_eq!(statistics.minimum, -2.0);
        assert_eq!(statistics.maximum, 4.0);
        assert_eq!(statistics.mean, 1.0);
        assert!(finite_trace_statistics(&[f64::NAN, f64::INFINITY]).is_none());
    }

    #[test]
    fn result_qualification_rows_never_infer_unretained_release_authority() {
        assert_eq!(
            RESULT_QUALIFICATION_GAPS,
            [
                ("Qualification receipt", "not retained"),
                ("Requirements mapping", "not retained"),
                ("Release gates", "not assessed"),
                ("Sign-off eligibility", "not assessed"),
                ("Approval authority", "not retained"),
            ]
        );
    }

    #[test]
    fn schematic_subjects_do_not_share_inspector_scroll_state() {
        let mut app = RSpiceApp::test_instance();
        let sheet = inspector_scroll_identity(&app);
        let component_id = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, crate::state::Point::new(10, 10));
        app.state
            .schematic
            .selection
            .select_only_component(component_id);
        let component = inspector_scroll_identity(&app);

        assert_ne!(sheet, component);
        app.state.schematic.selection.clear();
        assert_eq!(sheet, inspector_scroll_identity(&app));
    }

    #[test]
    fn inspector_scroll_memory_restores_subject_offsets_with_one_stable_widget() {
        let mut memory = InspectorScrollMemory::default();

        assert_eq!(memory.begin_subject("sheet"), Some(0.0));
        memory.record("sheet".to_owned(), 320.0);
        assert_eq!(memory.begin_subject("component:R1"), Some(0.0));
        memory.record("component:R1".to_owned(), 48.0);
        assert_eq!(memory.begin_subject("sheet"), Some(320.0));
        assert_eq!(memory.begin_subject("sheet"), None);
    }

    #[test]
    fn inspector_scroll_memory_is_bounded_and_clamps_invalid_offsets() {
        let mut memory = InspectorScrollMemory::default();
        for index in 0..=INSPECTOR_SCROLL_HISTORY_LIMIT {
            memory.record(format!("subject:{index}"), index as f32);
        }
        memory.record("negative".to_owned(), -50.0);

        assert_eq!(memory.offsets.len(), INSPECTOR_SCROLL_HISTORY_LIMIT);
        assert_eq!(memory.begin_subject("negative"), Some(0.0));
        assert!(
            memory
                .offsets
                .iter()
                .all(|(_, offset)| offset.is_finite() && *offset >= 0.0)
        );
    }

    #[test]
    fn full_results_cross_probe_navigates_to_design_after_exact_resolution() {
        let mut app = result_app_with_current_out_map(false);

        assert_eq!(
            cross_probe_trace_to_design(&mut app, "V(out)"),
            Ok("OUT".to_owned())
        );

        assert_eq!(app.state.workbench.workspace, Workspace::Design);
        assert!(!app.state.workbench.split_with_results);
        assert!(app.state.schematic.selection.wires.contains(&91));
        assert!(app.state.schematic.net_highlight.is_wire_highlighted(91));
    }

    #[test]
    fn split_cross_probe_keeps_the_canonical_result_document_beside_design() {
        let mut app = result_app_with_current_out_map(true);

        assert_eq!(
            cross_probe_trace_to_design(&mut app, "V(out)"),
            Ok("OUT".to_owned())
        );

        assert_eq!(app.state.workbench.workspace, Workspace::Design);
        assert!(app.state.workbench.results_split_visible(
            app.state.project_lifecycle.project_open,
            app.state.simulation.has_retained_result_dataset(),
        ));
    }

    #[test]
    fn stale_cross_probe_map_fails_without_leaving_results() {
        let mut app = result_app_with_current_out_map(true);
        app.state.schematic.add_wire(vec![
            crate::state::Point::new(80, 0),
            crate::state::Point::new(120, 0),
        ]);

        let error = cross_probe_trace_to_design(&mut app, "V(out)")
            .expect_err("topology mismatch must fail closed");

        assert!(error.contains("changed since this result"));
        assert_eq!(app.state.workbench.workspace, Workspace::Results);
        assert!(app.state.schematic.selection.wires.is_empty());
    }

    #[test]
    fn historical_result_revision_cannot_cross_probe_current_geometry() {
        let mut app = result_app_with_current_out_map(false);
        let stale_revision = app
            .state
            .workspace
            .project
            .revision()
            .next()
            .expect("next project revision");
        let run = app
            .state
            .simulation
            .active_run()
            .expect("prepared retained run");
        let analysis_id = run.analyses[0]
            .provenance()
            .expect("prepared analysis provenance")
            .source_instance_id();
        let receipt = crate::state::PreparedRunReceipt::new(
            crate::state::AnalysisResultSourceDomain::SimulationPlan,
            Some(crate::product::SimulationPlanId::new()),
            stale_revision,
            crate::product::ContentDigest::from_bytes([0x51; 32]),
            crate::product::ContentDigest::from_bytes([0x52; 32]),
            crate::state::PreparedSourceCheckReceipt::SchematicDrc(
                crate::product::ContentDigest::from_bytes([0x53; 32]),
            ),
            vec![
                crate::state::PreparedRunTaskReceipt::new(
                    analysis_id,
                    stale_revision,
                    Vec::new(),
                    1,
                    crate::product::ContentDigest::from_bytes([0x54; 32]),
                )
                .expect("valid prepared task"),
            ],
        )
        .expect("valid stale receipt");
        let mut stale_run = crate::state::SimulationRun::new_prepared(1, receipt);
        stale_run.add_analysis(
            crate::state::AnalysisResult::new(
                1,
                crate::state::AnalysisType::Transient,
                "historical TRAN",
            )
            .with_provenance(
                crate::state::AnalysisResultProvenance::new_with_source_domain(
                    crate::state::AnalysisResultSourceDomain::SimulationPlan,
                    analysis_id,
                    stale_revision,
                    crate::product::ContentDigest::from_bytes([0x54; 32]),
                    Vec::new(),
                )
                .expect("valid historical analysis provenance"),
            )
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#ffbd2e",
            )]),
        );
        app.state.simulation.runs[0] = stale_run;
        app.state.ui.results.selected_trace = Some(
            crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
                &app.state.simulation.runs[0],
                0,
                0,
            )
            .expect("selected historical trace"),
        );

        let error = cross_probe_trace_to_design(&mut app, "V(out)")
            .expect_err("historical result must fail closed");

        assert!(error.contains("different project revision"));
        assert_eq!(app.state.workbench.workspace, Workspace::Results);
        assert!(app.state.schematic.selection.wires.is_empty());
    }

    #[test]
    fn failure_row_reserves_a_non_overlapping_margin_column_at_drawer_width() {
        let row_width = 228.0;
        let columns = failure_row_columns(row_width, 48.0);
        let target_right = FAILURE_ROW_PADDING_X + columns.target_width;
        let margin_left = row_width - FAILURE_ROW_PADDING_X - columns.margin_width;

        assert!(target_right + FAILURE_ROW_COLUMN_GAP <= margin_left + f32::EPSILON);
        assert_eq!(columns.margin_width, 48.0);
    }

    #[test]
    fn failure_row_height_grows_from_wrapped_target_copy_at_drawer_width() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(228.0, 400.0))),
            ..egui::RawInput::default()
        };
        let failure = FailedSample {
            target: "an intentionally long production specification target that must wrap"
                .to_owned(),
            unit: "V".to_owned(),
            sample_index: 16,
            value: 0.912_345,
            normalized_margin: -0.1834,
        };
        let mut row_height = 0.0;

        let _ = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::new())
                .show(ctx, |ui| {
                    row_height = failure_row(ui, &failure).rect.height();
                });
        });

        assert!(row_height > 58.0, "wrapped row height was {row_height}");
    }

    #[test]
    fn netlist_diagnostic_locations_are_one_based_and_exact() {
        let mut diagnostic = crate::workbench::documents::netlist_document::Diagnostic::current(
            "rspice.test",
            "TEST-ADVISORY",
            crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
            "Maximum transient step is implicit",
        );
        diagnostic.source_line = Some(127);
        diagnostic.line = Some(127);
        diagnostic.column = Some(8);

        assert_eq!(diagnostic_location(&diagnostic), "line 128 · column 9");
    }

    #[test]
    fn generated_provenance_never_claims_source_mapping_without_evidence() {
        let mut state = AppState::default();
        state.ui.netlist.generated_source = "generated\n.end\n".to_owned();
        let input = crate::product::ContentDigest::from_bytes([0x31; 32]);
        state.ui.netlist.generated_input_digest = Some(input);
        state.ui.netlist.current_generation_input_digest = Some(input);

        assert_eq!(generated_state(&state), "generated · current");
        assert!(!generated_state(&state).contains("source mapped"));
    }

    #[test]
    fn owned_provenance_requires_exact_saved_and_validated_bytes() {
        let mut state = AppState::default();
        state.simulation.netlist_content = "owned\n.end\n".to_owned();
        let project_revision = state.workspace.project.revision().get();
        let digest = crate::workbench::documents::netlist_document::source_content_digest(
            &state.simulation.netlist_content,
        );
        state.ui.netlist.externally_saved_content_digest = Some(digest);
        state.ui.netlist.validation = Some(
            crate::workbench::documents::netlist_document::NetlistValidationReceipt {
                visible_content_digest: digest,
                executable_source_digest: digest,
                prepared_snapshot_digest: digest,
                project_revision,
                task_count: 1,
                advisory_count: 0,
            },
        );

        assert_eq!(
            owned_source_state(&state, digest),
            "externally synchronized · validated"
        );
        state.workspace.netlist_source_dirty = true;
        assert_eq!(
            owned_source_state(&state, digest),
            "externally synchronized · validated · project modified"
        );
        state.simulation.netlist_content.push_str("* edit\n");
        let edited = crate::workbench::documents::netlist_document::source_content_digest(
            &state.simulation.netlist_content,
        );
        assert_eq!(
            owned_source_state(&state, edited),
            "modified · validation required"
        );
    }

    #[test]
    fn native_component_never_claims_a_resolved_catalog_model() {
        let state = AppState::default();
        let component = Component::new(1, ComponentType::Resistor, crate::state::Point::origin());

        let evidence = component_model_evidence(&state, &component);

        assert_eq!(evidence.tone, ModelEvidenceTone::Neutral);
        assert_eq!(evidence.model, "Not applicable");
        assert!(!evidence.status.contains("resolved"));
    }

    #[test]
    fn missing_explicit_model_is_reported_as_unresolved_review_evidence() {
        let state = AppState::default();
        let component = Component::new(2, ComponentType::NpnBjt, crate::state::Point::origin())
            .with_name_value("Q1", "vendor_npn");

        let evidence = component_model_evidence(&state, &component);

        assert_eq!(evidence.tone, ModelEvidenceTone::Error);
        assert_eq!(evidence.model, "vendor_npn");
        assert!(evidence.status.contains("not loaded"));
    }

    #[test]
    fn green_model_evidence_requires_an_exact_pinned_catalog_source() {
        let mut state = AppState::default();
        let path = std::path::PathBuf::from("models/vendor.lib");
        let mut library = crate::state::model_library::ModelLibrary::new("vendor");
        library.root_path = Some(path.clone());
        library
            .source_closure
            .push(crate::state::model_library::ModelSourcePin {
                path: path.clone(),
                digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
            });
        let mut model = crate::state::model_library::DeviceModel::new(
            "vendor_npn",
            crate::state::model_library::ModelType::Npn,
        );
        model.file_path = Some(path);
        library.add_model(model);
        state.model_library_manager.add_library(library);
        let component = Component::new(3, ComponentType::NpnBjt, crate::state::Point::origin())
            .with_name_value("Q1", "vendor_npn");

        let evidence = component_model_evidence(&state, &component);

        assert_eq!(evidence.tone, ModelEvidenceTone::Ok);
        assert_eq!(evidence.model, "vendor_npn");
        assert!(evidence.status.contains("source pinned"));
        assert!(evidence.source.contains("vendor.lib"));
    }

    #[test]
    fn bound_model_evidence_uses_the_instance_model_and_section_overrides() {
        let mut state = AppState::default();
        let mut library = crate::state::model_library::ModelLibrary::new("vendor_analog");
        library.add_model(crate::state::model_library::DeviceModel::new(
            "OPA189_A",
            crate::state::model_library::ModelType::Other,
        ));
        library.add_model(crate::state::model_library::DeviceModel::new(
            "OPA189_B",
            crate::state::model_library::ModelType::Other,
        ));
        state.model_library_manager.add_library(library);

        let mut binding =
            crate::state::LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
        binding.module_name = Some("OPA189_B".to_owned());
        binding.model_section = Some("high_accuracy".to_owned());
        let component = Component::new(
            9,
            ComponentType::CellInstance,
            crate::state::Point::origin(),
        )
        .with_library_cell(binding)
        .with_name_value("XU1", "OPA189");

        let evidence = component_model_evidence(&state, &component);

        assert_eq!(evidence.model, "OPA189_B");
        assert_eq!(evidence.section, "high_accuracy");
    }

    #[test]
    fn wilson_interval_is_bounded_and_contains_the_observed_rate() {
        let (low, high) = wilson_interval_95(986, 1_000).expect("valid population");

        assert!((0.0..=1.0).contains(&low));
        assert!((0.0..=1.0).contains(&high));
        assert!(low < 0.986 && high > 0.986);
        assert!(wilson_interval_95(0, 0).is_none());
        assert!(wilson_interval_95(2, 1).is_none());
    }
}
