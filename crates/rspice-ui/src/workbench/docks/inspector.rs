//! Context inspector with authoritative object and provenance details.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{property_row, section_header, status_dot};
use super::super::state::Workspace;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    header(ui, app);
    ScrollArea::vertical()
        .id_salt("workbench.inspector.scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| match app.state.workbench.workspace {
            Workspace::Project => project(ui, app),
            Workspace::Design => design(ui, app),
            Workspace::Simulate => simulate(ui, app),
            Workspace::Results => results(ui, app),
            Workspace::Verify => verify(ui, app),
            Workspace::Models => models(ui, app),
            Workspace::Netlist => netlist(ui, app),
        });
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(
                app.state
                    .workbench
                    .workspace
                    .inspector_title()
                    .to_ascii_uppercase(),
            )
            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
            .color(t.color.text),
        );
    });
    ui.add_space(4.0);
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.workbench.project_name_draft.is_empty() {
        app.state.workbench.project_name_draft = app.state.workspace.project.name().to_owned();
    }
    section_header(ui, "Identity", None);
    field_label(ui, "Project name");
    let response = ui.add_sized(
        [ui.available_width() - 24.0, 28.0],
        egui::TextEdit::singleline(&mut app.state.workbench.project_name_draft),
    );
    let apply = response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    if (ui.button("Apply project name").clicked() || apply)
        && app.state.workbench.project_name_draft != app.state.workspace.project.name()
    {
        match app
            .state
            .workspace
            .project
            .rename(app.state.workbench.project_name_draft.clone())
        {
            Ok(_) => app.state.workbench.project_name_error = None,
            Err(error) => app.state.workbench.project_name_error = Some(error.to_string()),
        }
    }
    if let Some(error) = &app.state.workbench.project_name_error {
        validation(ui, error);
    }
    property_row(
        ui,
        "Project ID",
        &app.state.workspace.project.id().to_string(),
    );
    property_row(
        ui,
        "Revision",
        &app.state.workspace.project.revision().get().to_string(),
    );
    property_row(
        ui,
        "Path",
        &app.state
            .workspace
            .project
            .path
            .as_ref()
            .map_or_else(|| "Not saved".to_owned(), |path| path.display().to_string()),
    );
    section_header(ui, "Design binding", None);
    property_row(
        ui,
        "Root library",
        &app.state.workspace.project.root_library,
    );
    property_row(ui, "Top cell", &app.state.workspace.project.top_cell);
    property_row(
        ui,
        "Technology",
        app.state
            .workspace
            .project
            .technology
            .as_deref()
            .unwrap_or("Unbound"),
    );
    property_row(
        ui,
        "Open documents",
        &app.state.workspace.open_views.len().to_string(),
    );
}

fn design(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Active document", None);
    property_row(ui, "Library", &app.state.workspace.active_view.library);
    property_row(ui, "Cell", &app.state.workspace.active_view.cell);
    property_row(ui, "View", &app.state.workspace.active_view.view);
    property_row(
        ui,
        "Access",
        if app.state.active_view_read_only() {
            "Read only"
        } else {
            "Editable"
        },
    );

    section_header(
        ui,
        "Selection",
        Some(&app.state.schematic.selection.count().to_string()),
    );
    let selected = app
        .state
        .schematic
        .selection
        .single_component()
        .and_then(|id| {
            app.state
                .schematic
                .components
                .iter()
                .find(|component| component.id == id)
                .cloned()
        });
    if let Some(component) = selected {
        property_row(ui, "Reference", &component.name);
        property_row(ui, "Device", &format!("{:?}", component.kind));
        property_row(ui, "Value", &component.value);
        property_row(
            ui,
            "Position",
            &format!("{}, {}", component.pos.x, component.pos.y),
        );
        property_row(ui, "Rotation", &format!("{:?}", component.rotation));
        if !component.params.trim().is_empty() {
            property_row(ui, "Parameters", &component.params);
        }
        if ui.button("Edit all properties…").clicked() {
            crate::common::app::open_property_editor(&mut app.state, component.id);
        }
    } else {
        property_row(
            ui,
            "Components",
            &app.state.schematic.components.len().to_string(),
        );
        property_row(ui, "Wires", &app.state.schematic.wires.len().to_string());
        property_row(
            ui,
            "Net labels",
            &app.state.schematic.net_labels.len().to_string(),
        );
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
        validation(ui, &error);
    } else {
        status_dot(ui, Tokens::get(ui.ctx()).color.ok, "Configuration valid");
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
        if app.state.simulation.is_running {
            "Running"
        } else {
            "Ready"
        },
    );
}

fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Dataset provenance", None);
    let Some(run_index) = app.state.simulation.active_run_idx else {
        property_row(ui, "Selection", "No active dataset");
        property_row(
            ui,
            "Available runs",
            &app.state.simulation.runs.len().to_string(),
        );
        return;
    };
    let Some(run) = app.state.simulation.runs.get(run_index) else {
        property_row(ui, "Selection", "Dataset no longer available");
        return;
    };
    property_row(ui, "Run", &run.label);
    property_row(ui, "Run ID", &run.run_id.to_string());
    property_row(ui, "Dataset ID", &run.dataset_id.to_string());
    property_row(ui, "Elapsed", &format!("{:.6} s", run.elapsed_time));
    property_row(ui, "Analyses", &run.analyses.len().to_string());
    property_row(
        ui,
        "Status",
        if run.success { "Completed" } else { "Failed" },
    );
    if let Some(index) = app.state.simulation.active_analysis_idx
        && let Some(analysis) = run.analyses.get(index)
    {
        section_header(ui, "Analysis", None);
        property_row(ui, "Label", &analysis.label);
        property_row(ui, "Type", &format!("{:?}", analysis.analysis_type));
        property_row(ui, "Waveforms", &analysis.waveforms.len().to_string());
        property_row(ui, "Measurements", &analysis.measurements.len().to_string());
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
    ui.add_space(8.0);
    crate::workbench::result_document::right_panel(ui, &mut app.state);
}

fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Currentness", None);
    let current = app.state.dialogs.drc_checked_version == app.state.schematic.topology_version();
    property_row(
        ui,
        "Schematic revision",
        &app.state.schematic.topology_version().to_string(),
    );
    property_row(
        ui,
        "Checks",
        if current {
            "Current"
        } else {
            "Stale / not run"
        },
    );
    if let Some(result) = &app.state.dialogs.drc_results {
        let summary = result.summary();
        property_row(ui, "Critical", &summary.critical.to_string());
        property_row(ui, "Errors", &summary.errors.to_string());
        property_row(ui, "Advisories", &summary.warnings.to_string());
        property_row(ui, "Runtime", &format!("{} ms", result.duration_ms));
    }
    section_header(ui, "Specification evidence", None);
    property_row(
        ui,
        "Tracked specifications",
        &app.state.workspace.specs.len().to_string(),
    );
    property_row(
        ui,
        "Simulation runs",
        &app.state.simulation.runs.len().to_string(),
    );
    property_row(
        ui,
        "SOA violations",
        &app.state.simulation.soa_violations.len().to_string(),
    );
    property_row(
        ui,
        "Aging results",
        &app.state.simulation.reliability_results.len().to_string(),
    );
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

fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Source", None);
    let manual = app.state.workspace.netlist_source.is_some();
    property_row(
        ui,
        "Ownership",
        if manual {
            "Manual source"
        } else {
            "Generated artifact"
        },
    );
    property_row(
        ui,
        "Dirty",
        if app.state.workspace.netlist_source_dirty {
            "Modified"
        } else {
            "Clean"
        },
    );
    property_row(
        ui,
        "Origin",
        &app.state
            .workspace
            .netlist_source_path
            .as_ref()
            .map_or_else(
                || {
                    if manual {
                        "In project".to_owned()
                    } else {
                        "Schematic generator".to_owned()
                    }
                },
                |path| path.display().to_string(),
            ),
    );
    property_row(
        ui,
        "Bytes",
        &app.state.simulation.netlist_content.len().to_string(),
    );
    property_row(
        ui,
        "Lines",
        &app.state
            .simulation
            .netlist_content
            .lines()
            .count()
            .to_string(),
    );
    section_header(ui, "Execution", None);
    property_row(
        ui,
        "Status",
        if app.state.simulation.is_running {
            "Running"
        } else {
            "Ready"
        },
    );
    property_row(
        ui,
        "Run history",
        &app.state.simulation.runs.len().to_string(),
    );
    super::super::netlist_document::show_parameter_inspector(ui, &mut app.state);
}

fn field_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_dim),
    );
}

fn validation(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(message)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.err),
    );
}
