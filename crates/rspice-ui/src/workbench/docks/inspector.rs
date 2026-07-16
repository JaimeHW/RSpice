//! Context inspector with authoritative object and provenance details.

use egui::{ScrollArea, Ui};

use crate::common::{AppState, RSpiceApp};
use crate::state::{Component, ComponentType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{
    PANEL_HEADER_H, property_row, section_header as design_section_header, status_dot,
};
use super::super::state::{VerificationPage, Workspace};

const INSPECTOR_PROPERTY_LIST_PADDING_TOP: f32 = 7.0;
const INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM: f32 = 10.0;

fn inspector_section_state_id() -> egui::Id {
    egui::Id::new("workbench.inspector.property-list-open")
}

fn begin_inspector_sections(ui: &mut Ui) {
    ui.data_mut(|data| data.insert_temp(inspector_section_state_id(), false));
}

fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let has_previous = ui.data_mut(|data| {
        data.get_temp::<bool>(inspector_section_state_id())
            .unwrap_or(false)
    });
    if has_previous {
        ui.add_space(INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM);
    }
    design_section_header(ui, title, meta);
    ui.add_space(INSPECTOR_PROPERTY_LIST_PADDING_TOP);
    ui.data_mut(|data| data.insert_temp(inspector_section_state_id(), true));
}

fn finish_inspector_sections(ui: &mut Ui) {
    if ui
        .data_mut(|data| data.remove_temp::<bool>(inspector_section_state_id()))
        .unwrap_or(false)
    {
        ui.add_space(INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM);
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    header(ui, app);
    ScrollArea::vertical()
        .id_salt("workbench.inspector.scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            begin_inspector_sections(ui);
            match app.state.workbench.workspace {
                Workspace::Project => project(ui, app),
                Workspace::Design => design(ui, app),
                Workspace::Simulate => simulate(ui, app),
                Workspace::Results => results(ui, app),
                Workspace::Verify => verify(ui, app),
                Workspace::Models => models(ui, app),
                Workspace::Netlist => netlist(ui, app),
            }
            finish_inspector_sections(ui);
        });
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_HEADER_H),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let title = match app.state.workbench.workspace {
        Workspace::Verify if app.state.workbench.verification_page == VerificationPage::Cockpit => {
            "Yield details"
        }
        Workspace::Verify => app.state.workbench.verification_page.label(),
        _ => app.state.workbench.workspace.inspector_title(),
    };
    ui.painter().text(
        egui::pos2(rect.left() + 11.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title.to_ascii_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Identity", None);
    property_row(ui, "Project", app.state.workspace.project.name());
    property_row(
        ui,
        "Project ID",
        &app.state.workspace.project.id().to_string(),
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
    section_header(ui, "Dependencies", None);
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
        "Model libraries",
        &app.state.model_library_manager.library_count().to_string(),
    );
    section_header(ui, "Working revision", None);
    property_row(
        ui,
        "Revision",
        &app.state.workspace.project.revision().get().to_string(),
    );
    property_row(
        ui,
        "Open documents",
        &app.state.workspace.open_views.len().to_string(),
    );
    property_row(
        ui,
        "Modified",
        &app.state
            .workspace
            .open_views
            .iter()
            .filter(|view| view.dirty)
            .count()
            .to_string(),
    );
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
                return catalog_model_evidence(library, model);
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
                section: "Not declared".to_owned(),
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
            section: "Not applicable".to_owned(),
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

    if matches!(
        component.kind,
        ComponentType::NVdmos | ComponentType::PVdmos
    ) {
        return ComponentModelEvidence {
            status: "no executable model binding · review".to_owned(),
            model: "Unresolved".to_owned(),
            source: "No supported netlist binding".to_owned(),
            section: "Unavailable".to_owned(),
            tone: ModelEvidenceTone::Error,
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
    let params = crate::properties::parse_params_string(&component.params);
    let param_model = params
        .get("model")
        .map(|model| model.trim())
        .filter(|model| !model.is_empty());
    let value_model = component.value.trim();
    match component.kind {
        ComponentType::NpnBjt | ComponentType::PnpBjt | ComponentType::VSwitch => param_model
            .or((!value_model.is_empty()).then_some(value_model))
            .map(str::to_owned),
        ComponentType::Diode => (!value_model.is_empty()).then(|| value_model.to_owned()),
        _ => None,
    }
}

fn generated_inline_model(component: &Component) -> Option<String> {
    let prefix = match component.kind {
        ComponentType::Nmos => "nmos",
        ComponentType::Pmos => "pmos",
        ComponentType::NpnBjt => "npn",
        ComponentType::PnpBjt => "pnp",
        ComponentType::Njfet => "njf",
        ComponentType::Pjfet => "pjf",
        ComponentType::VSwitch => "sw",
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

fn design(ui: &mut Ui, app: &mut RSpiceApp) {
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
    let model_evidence = selected
        .as_ref()
        .map(|component| component_model_evidence(&app.state, component));
    inspector_hero(ui, app, selected.as_ref(), model_evidence.as_ref());
    if let Some(component) = selected.as_ref() {
        section_header(ui, "Identity", Some("editable"));
        property_row(ui, "Instance", &component.name);
        property_row(ui, "Value", &component.value);
        let library_cell = component.library_cell.as_ref().map_or_else(
            || format!("primitives/{}", component.kind.display_name()),
            |binding| format!("{}/{}", binding.library, binding.cell),
        );
        property_row(ui, "Library cell", &library_cell);
        property_row(
            ui,
            "View",
            component
                .library_cell
                .as_ref()
                .map_or("symbol · spice", |binding| binding.view.as_str()),
        );

        section_header(ui, "Simulation parameters", None);
        let evidence = model_evidence
            .as_ref()
            .expect("selected components always have model evidence");
        property_row(ui, "Model", &evidence.model);
        property_row(ui, "Source", &evidence.source);
        property_row(ui, "Section", &evidence.section);
        property_row(
            ui,
            "Temperature",
            &format!(
                "inherit · {} °C",
                app.state.sim_setup.reference_pvt.temperature_celsius
            ),
        );
        property_row(
            ui,
            "Parameters",
            if component.params.trim().is_empty() {
                "instance value"
            } else {
                component.params.as_str()
            },
        );

        operating_point(ui, app, component);
        component_checks(ui, app, component);
    } else {
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

fn operating_point(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let retained = app.state.simulation.runs.iter().find_map(|run| {
        run.analyses.iter().find_map(|analysis| {
            analysis.device_op.as_ref().and_then(|report| {
                report
                    .entries
                    .iter()
                    .find(|entry| entry.name.eq_ignore_ascii_case(&component.name))
                    .map(|entry| {
                        (
                            run.id,
                            analysis.label.clone(),
                            entry.region,
                            entry.params.clone(),
                        )
                    })
            })
        })
    });
    if let Some((run_id, analysis, region, params)) = retained {
        section_header(
            ui,
            &format!("Operating point · Run {run_id}"),
            Some("retained"),
        );
        if let Some(region) = region {
            property_row(ui, "Region", region);
        }
        for (name, value) in params.into_iter().take(4) {
            property_row(ui, name, &format!("{value:.6e}"));
        }
        property_row(ui, "Analysis", &analysis);
    } else {
        section_header(ui, "Operating point", Some("no evidence"));
        property_row(ui, "Selection", "No retained device operating point");
        property_row(ui, "Required analysis", "DC operating point");
    }
}

fn component_checks(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let topology = app.state.schematic.topology_version();
    let current = app.state.dialogs.drc_checked_version == topology;
    let summary = current
        .then(|| {
            app.state
                .dialogs
                .drc_results
                .as_ref()
                .map(|result| result.summary())
        })
        .flatten();
    let finding_count = summary.map_or(0, |summary| summary.critical + summary.errors);
    let status = if !current || summary.is_none() {
        "stale".to_owned()
    } else {
        format!("{finding_count} errors")
    };
    section_header(ui, "Checks", Some(&status));
    property_row(
        ui,
        "Connectivity",
        if current && summary.is_some() {
            if finding_count == 0 {
                "✓ checked"
            } else {
                "△ findings present"
            }
        } else {
            "△ pending recheck"
        },
    );
    let soa_findings = app
        .state
        .simulation
        .soa_violations
        .iter()
        .filter(|violation| violation.device_id.eq_ignore_ascii_case(&component.name))
        .count();
    property_row(
        ui,
        "Safe operating area",
        &if soa_findings == 0 {
            "No retained violations".to_owned()
        } else {
            format!("{soa_findings} retained violation(s)")
        },
    );
    property_row(
        ui,
        "Last checked",
        &if current {
            format!("topology revision {topology}")
        } else {
            format!("rerun for topology revision {topology}")
        },
    );
}

fn inspector_hero(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: Option<&Component>,
    model_evidence: Option<&ComponentModelEvidence>,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 82.0),
        if component.is_some() {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let preview = egui::Rect::from_min_max(
        rect.min,
        egui::pos2((rect.left() + 82.0).min(rect.right()), rect.bottom()),
    );
    ui.painter().rect_filled(preview, 0.0, t.color.canvas_bg);
    ui.painter().vline(
        preview.right(),
        preview.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let text_left = preview.right() + 10.0;
    let text_right = rect.right() - 10.0;
    let text_clip = egui::Rect::from_x_y_ranges(text_left..=text_right, rect.y_range());
    let painter = ui.painter().with_clip_rect(text_clip);
    if let Some(component) = component {
        crate::schematic::view::draw_symbol_preview(
            ui.painter(),
            preview.shrink(12.0),
            component.kind,
            t.color.symbol,
            app.symbol_library.as_ref(),
        );
        let path = format!(
            "/{}/{}",
            app.state.workspace.active_view.cell, component.name
        );
        painter.text(
            egui::pos2(text_left, rect.top() + 12.0),
            egui::Align2::LEFT_CENTER,
            format!("{} · {path}", component.name),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        painter.text(
            egui::pos2(text_left, rect.top() + 31.0),
            egui::Align2::LEFT_CENTER,
            if component.value.trim().is_empty() {
                component.kind.display_name()
            } else {
                component.value.as_str()
            },
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
        painter.text(
            egui::pos2(text_left, rect.top() + 49.0),
            egui::Align2::LEFT_CENTER,
            component.kind.display_name(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        let evidence = model_evidence.expect("selected components always have model evidence");
        painter.text(
            egui::pos2(text_left, rect.top() + 68.0),
            egui::Align2::LEFT_CENTER,
            &evidence.status,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            evidence.tone.color(&t),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                "Open selected component properties",
            )
        });
        if response.double_clicked() && !app.state.active_view_read_only() {
            crate::common::app::open_property_editor(&mut app.state, component.id);
        }
        response.on_hover_text("Double-click to edit component properties");
    } else {
        super::super::design_system::WorkbenchIcon::Select.paint(
            ui.painter(),
            preview.shrink(28.0),
            t.color.text_faint,
        );
        painter.text(
            egui::pos2(text_left, rect.top() + 29.0),
            egui::Align2::LEFT_CENTER,
            "No schematic selection",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
        painter.text(
            egui::pos2(text_left, rect.top() + 50.0),
            egui::Align2::LEFT_CENTER,
            "Select an instance, wire, or net",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
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
    if app.state.workbench.verification_page == VerificationPage::Cockpit {
        yield_details(ui, app);
        return;
    }
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

fn yield_details(ui: &mut Ui, app: &RSpiceApp) {
    let run = app
        .state
        .simulation
        .active_run()
        .or_else(|| app.state.simulation.runs.first());
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

fn failure_row(ui: &mut Ui, failure: &FailedSample) {
    let t = Tokens::get(ui.ctx());
    let height = if t.metrics.ctl_h >= 44.0 { 58.0 } else { 47.0 };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
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
    let painter = ui
        .painter()
        .with_clip_rect(rect.shrink2(egui::vec2(9.0, 0.0)));
    painter.text(
        rect.left_top() + egui::vec2(9.0, 8.0),
        egui::Align2::LEFT_TOP,
        format!("#{:04} · {}", failure.sample_index + 1, failure.target),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    painter.text(
        rect.right_top() + egui::vec2(-9.0, 8.0),
        egui::Align2::RIGHT_TOP,
        format!("{:+.2}%", failure.normalized_margin * 100.0),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.err,
    );
    painter.text(
        rect.left_bottom() + egui::vec2(9.0, -8.0),
        egui::Align2::LEFT_BOTTOM,
        format!("observed {:.6} {}", failure.value, failure.unit),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
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

fn validation(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(message)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.err),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspector_property_lists_keep_mockup_vertical_padding() {
        assert_eq!(INSPECTOR_PROPERTY_LIST_PADDING_TOP, 7.0);
        assert_eq!(INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM, 10.0);
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
    fn wilson_interval_is_bounded_and_contains_the_observed_rate() {
        let (low, high) = wilson_interval_95(986, 1_000).expect("valid population");

        assert!((0.0..=1.0).contains(&low));
        assert!((0.0..=1.0).contains(&high));
        assert!(low < 0.986 && high > 0.986);
        assert!(wilson_interval_95(0, 0).is_none());
        assert!(wilson_interval_95(2, 1).is_none());
    }
}
