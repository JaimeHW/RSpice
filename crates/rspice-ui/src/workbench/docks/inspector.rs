//! Context inspector with authoritative object and provenance details.

mod design;
mod symbol;

use egui::{Align2, Color32, Pos2, Rect, Response, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::state::{Component, ComponentType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, RSpiceApp};

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    PANEL_HEADER_H, PANEL_SECTION_H, StatusMark, WorkbenchIcon, property_row, property_row_status,
    schematic_section_header as design_schematic_section_header,
    section_header as design_section_header,
};
use super::super::state::{VerificationPage, Workspace};

const INSPECTOR_PROPERTY_LIST_PADDING_TOP: f32 = 7.0;
const INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM: f32 = 10.0;
const INSPECTOR_TREE_PADDING_TOP: f32 = 4.0;
const INSPECTOR_TREE_PADDING_BOTTOM: f32 = 7.0;

fn inspector_section_state_id() -> egui::Id {
    egui::Id::new("workbench.inspector.property-list-open")
}

fn begin_inspector_sections(ui: &mut Ui) {
    ui.data_mut(|data| data.insert_temp(inspector_section_state_id(), -1.0_f32));
}

fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
    design_section_header(ui, title, meta);
    ui.add_space(INSPECTOR_PROPERTY_LIST_PADDING_TOP);
    ui.data_mut(|data| {
        data.insert_temp(
            inspector_section_state_id(),
            INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM,
        )
    });
}

fn schematic_section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
    design_schematic_section_header(ui, title, meta);
    ui.add_space(INSPECTOR_PROPERTY_LIST_PADDING_TOP);
    ui.data_mut(|data| {
        data.insert_temp(
            inspector_section_state_id(),
            INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM,
        )
    });
}

/// Schematic section followed by a compact mockup `.tree` rather than a
/// property list. Keeping this local prevents the tree rhythm from changing
/// form sections or inspectors in other workspaces.
fn schematic_tree_section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
    design_schematic_section_header(ui, title, meta);
    ui.add_space(INSPECTOR_TREE_PADDING_TOP);
    ui.data_mut(|data| {
        data.insert_temp(inspector_section_state_id(), INSPECTOR_TREE_PADDING_BOTTOM)
    });
}

fn schematic_annotation_section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
    design_schematic_section_header(ui, title, meta);
    ui.add_space(8.0);
    ui.data_mut(|data| data.insert_temp(inspector_section_state_id(), 8.0_f32));
}

fn schematic_section_header_action(
    ui: &mut Ui,
    title: &str,
    action: &str,
    enabled: bool,
) -> Response {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
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
    let response = ui.interact(
        action_rect,
        ui.id().with(("schematic-section-action", title, action)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
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
    ui.add_space(INSPECTOR_PROPERTY_LIST_PADDING_TOP);
    ui.data_mut(|data| {
        data.insert_temp(
            inspector_section_state_id(),
            INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM,
        )
    });
    response
}

fn finish_inspector_sections(ui: &mut Ui) {
    let bottom = ui
        .data_mut(|data| data.remove_temp::<f32>(inspector_section_state_id()))
        .unwrap_or(-1.0);
    if bottom >= 0.0 {
        ui.add_space(bottom);
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.spacing_mut().item_spacing.y = 0.0;
    header(ui, app);
    let scroll_identity = inspector_scroll_identity(app);
    ScrollArea::vertical()
        // Each inspected object owns its scroll state. Sharing one scroll
        // offset across a tall component, the sheet, and another component
        // made the next inspector appear halfway down after selection
        // changes, which was both jarring and unlike the upgraded mockup.
        .id_salt(("workbench.inspector.scroll", scroll_identity))
        .auto_shrink([false, false])
        .show(ui, |ui| {
            begin_inspector_sections(ui);
            if split_selected_trace_is_inspected(app) {
                results(ui, app);
            } else {
                match app.state.workbench.workspace {
                    Workspace::Project => project(ui, app),
                    // A symbol cellview is edited against its pin contract, not
                    // against a schematic selection.
                    Workspace::Design
                        if app.state.workspace.active_view_type()
                            == crate::state::ViewType::Symbol =>
                    {
                        symbol::show(ui, app);
                    }
                    Workspace::Design => design::show(ui, app),
                    Workspace::Simulate => simulate(ui, app),
                    Workspace::Results => results(ui, app),
                    Workspace::Verify => verify(ui, app),
                    Workspace::Models => models(ui, app),
                    Workspace::Netlist => netlist(ui, app),
                }
            }
            finish_inspector_sections(ui);
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
    let title = if split_selected_trace_is_inspected(app) {
        "Result details"
    } else {
        match app.state.workbench.workspace {
            Workspace::Verify
                if app.state.workbench.verification_page == VerificationPage::Yield =>
            {
                "Yield details"
            }
            Workspace::Verify => app.state.workbench.verification_page.label(),
            Workspace::Netlist => "Diagnostics & tuner",
            _ => app.state.workbench.workspace.inspector_title(),
        }
    };
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title);
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
        ComponentType::SaturableInductor => param_model.map(str::to_owned),
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
        if app.state.simulation.is_running {
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
    if let Some(selected) = selected_trace {
        section_header(ui, "Selected trace", None);
        property_row(ui, "Name", &selected.source_name);
        property_row(ui, "Analysis", &(selected.analysis_index + 1).to_string());
        property_row(ui, "Dataset", &selected.dataset_id.to_string());
        if is_schematic_cross_probe_candidate(&selected.source_name) {
            let signal = selected.source_name;
            let unavailable = schematic_cross_probe_unavailability(&app.state, &signal);
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
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
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
                            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                                message,
                            ));
                    }
                }
            }
        }
    }
    ui.add_space(8.0);
    crate::workbench::documents::result_document::right_panel(ui, &mut app.state);
}

fn is_schematic_cross_probe_candidate(signal: &str) -> bool {
    (signal.starts_with("V(") || signal.starts_with("I(")) && signal.ends_with(')')
}

fn schematic_cross_probe_unavailability(state: &AppState, signal: &str) -> Option<String> {
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

fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, DiagnosticSeverity};

    let errors = app
        .state
        .ui
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        .collect::<Vec<_>>();
    let advisories = app
        .state
        .ui
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity != DiagnosticSeverity::Error)
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        diagnostic_section_header(ui, "Errors", errors.len(), DiagnosticSeverity::Error);
        for diagnostic in errors {
            diagnostic_row(ui, diagnostic);
        }
    }

    diagnostic_section_header(
        ui,
        "Advisories",
        advisories.len(),
        DiagnosticSeverity::Warning,
    );
    if advisories.is_empty() {
        empty_diagnostic_row(ui, "No advisories for the current document.");
    } else {
        for diagnostic in advisories {
            diagnostic_row(ui, diagnostic);
        }
    }

    design_section_header(ui, "Parameter exploration", Some("dedicated workspace"));
    muted_inspector_copy(
        ui,
        "Use the non-destructive tuning sandbox for live plots, measurement deltas, limit checks and explicit commit or revert.",
    );

    match app.state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => generated_provenance(ui, &app.state),
        ActiveNetlistDocument::OwnedSource => owned_source_provenance(ui, &app.state),
        ActiveNetlistDocument::GeneratedDiff => generated_provenance(ui, &app.state),
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

fn diagnostic_row(ui: &mut Ui, diagnostic: &crate::workbench::documents::netlist_document::Diagnostic) {
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
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => WorkbenchIcon::Info,
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => WorkbenchIcon::Warning,
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
        .and_then(|document| document.generated_artifact().source_map().first())
        .map(|entry| entry.view_identity().to_owned())
        .unwrap_or_else(|| "Unavailable".to_owned());
    property_row(ui, "Source cell/view", &source);
    property_row(ui, "Input revision", &generated_input_revision(state));
    property_row(
        ui,
        "Input digest",
        &canonical
            .map(|document| {
                short_digest(document.generated_artifact().provenance().input().digest())
            })
            .unwrap_or_else(|| "Unavailable".to_owned()),
    );
    property_row(ui, "Generator state", generated_state(state));
    property_row(
        ui,
        "Netlist digest",
        &canonical
            .map(|document| short_digest(document.generated_artifact().content_digest()))
            .unwrap_or_else(|| "Unavailable".to_owned()),
    );
}

fn owned_source_provenance(ui: &mut Ui, state: &AppState) {
    design_section_header(ui, "Owned source provenance", None);
    let source = &state.simulation.netlist_content;
    let source_digest = crate::workbench::documents::netlist_document::source_content_digest(source);
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
            &short_digest(document.generated_artifact().content_digest()),
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

fn generated_state(state: &AppState) -> &'static str {
    let netlist = &state.ui.netlist;
    if netlist.generated_source.is_empty() {
        "not generated"
    } else if netlist.generation_error.is_some() {
        "blocked · prior artifact retained"
    } else if netlist.generated_input_digest != netlist.current_generation_input_digest {
        "stale · refresh pending"
    } else {
        let digest =
            crate::workbench::documents::netlist_document::source_content_digest(&netlist.generated_source);
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
        .map(|document| {
            document
                .generated_artifact()
                .provenance()
                .input()
                .revision()
                .get()
                .to_string()
        })
        .unwrap_or_else(|| "Unavailable".to_owned())
}

fn owned_source_state(state: &AppState, digest: crate::product::ContentDigest) -> &'static str {
    let validated = state.ui.netlist.validation.as_ref().is_some_and(|receipt| {
        receipt.visible_content_digest == digest
            && receipt.project_revision == state.workspace.project.revision().get()
    });
    let saved = state.ui.netlist.externally_saved_content_digest == Some(digest);
    if state.workspace.netlist_source_dirty {
        "modified · validation pending"
    } else if validated && saved {
        "saved · validated"
    } else if validated {
        "validated · save required"
    } else if saved {
        "saved · validation required"
    } else {
        "owned · validation required"
    }
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    let digest = digest.to_string();
    format!("{}…{}", &digest[..8], &digest[digest.len() - 4..])
}

fn diagnostic_location(diagnostic: &crate::workbench::documents::netlist_document::Diagnostic) -> String {
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
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => tokens.color.info,
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => tokens.color.warn,
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => tokens.color.err,
    }
}

fn diagnostic_severity_name(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> &'static str {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => "Information",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => "Warning",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => "Error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        app.state.simulation.start_run().add_analysis(
            crate::state::AnalysisResult::new(
                1,
                crate::state::AnalysisType::Transient,
                "retained TRAN",
            )
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#ffbd2e",
            )]),
        );
        let dataset_id = app
            .state
            .simulation
            .active_run()
            .expect("retained run")
            .dataset_id
            .clone();
        app.state.ui.results.selected_trace =
            Some(crate::workbench::documents::result_document::SelectedResultTrace {
                dataset_id,
                analysis_index: 0,
                waveform_index: 0,
                source_name: "V(out)".to_owned(),
            });
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
    fn inspector_property_lists_keep_mockup_vertical_padding() {
        assert_eq!(INSPECTOR_PROPERTY_LIST_PADDING_TOP, 7.0);
        assert_eq!(INSPECTOR_PROPERTY_LIST_PADDING_BOTTOM, 10.0);
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
        let diagnostic = crate::workbench::documents::netlist_document::Diagnostic {
            severity: crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
            source_path: None,
            source_line: Some(127),
            span: None,
            line: Some(127),
            column: Some(8),
            message: "Maximum transient step is implicit".to_owned(),
            fix: None,
        };

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

        assert_eq!(owned_source_state(&state, digest), "saved · validated");
        state.simulation.netlist_content.push_str("* edit\n");
        let edited = crate::workbench::documents::netlist_document::source_content_digest(
            &state.simulation.netlist_content,
        );
        assert_eq!(
            owned_source_state(&state, edited),
            "owned · validation required"
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
