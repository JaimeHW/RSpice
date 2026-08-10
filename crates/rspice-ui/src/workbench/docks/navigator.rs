//! Workspace-aware navigation tree.

mod design;
mod symbol;

mod netlist;

use netlist::*;

use egui::{Align, Layout, Response, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::product::DatasetId;
use crate::state::ViewType;
use crate::state::{NetlistOutline, OutlineEntry, OutlineEntryKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    PANEL_HEADER_H, StatusMark, WorkbenchIcon, paint_status_mark, property_row, section_header,
};
use super::super::state::{ModelsPage, ProjectPage, SimulationPage, VerificationPage, Workspace};

const EXPRESSION_HEADER_HEIGHT: f32 = 28.0;
const SIGNAL_ROW_HEIGHT: f32 = 30.0;
// The results browser's two-register quantity row and its group head, from
// the mockup's `.result-browser-quantity` / `.result-browser-analysis-head`.
const RESULT_QUANTITY_ROW_HEIGHT: f32 = 36.0;
const RESULT_ANALYSIS_HEAD_HEIGHT: f32 = 31.0;
const RESULT_MANIFEST_ROW_HEIGHT: f32 = 26.0;
const TOUCH_TARGET_HEIGHT: f32 = 44.0;
const PANEL_SEARCH_MARGIN_X: f32 = 8.0;
const SCHEMATIC_NAV_ROW_HEIGHT: f32 = 24.0;
const SCHEMATIC_NAV_LABEL_SIZE: f32 = tokens::FS_1;
const SCHEMATIC_NAV_META_SIZE: f32 = 10.0;
// Mirrors the mockup's `.section-body { padding-inline: 10px; }` contract so
// run-set values remain visually contained beside the analysis-stack divider.
const NAV_PROPERTY_PADDING_X: f32 = 10.0;
// The creator needs clearance from the last tree row above it while staying
// flush to the panel's sides and to the block's bottom.
const SIMULATION_CREATOR_GAP: f32 = 6.0;
const SIMULATION_CREATOR_EXTRA_HEIGHT: f32 = 8.0;
const EMPTY_HINT_PADDING_X: i8 = 12;
const EMPTY_HINT_PADDING_Y: i8 = 20;
// The mock's 49 px is a minimum. Its three grid rows resolve to 63 px at the
// reviewed 11 px type scale (title/detail row, 4 px gap, status row).
const FLOW_ROW_HEIGHT: f32 = 63.0;
const FLOW_TEXT_LEFT: f32 = 35.0;
const FLOW_LABEL_TOP: f32 = 7.0;
const FLOW_DETAIL_TOP: f32 = 23.0;
const FLOW_STATUS_TOP: f32 = 43.0;
const FLOW_DETAIL_LINE_HEIGHT: f32 = 15.0;
const FLOW_STATUS_DOT_SIZE: f32 = 5.0;
const FLOW_STATUS_GAP: f32 = 6.0;
const CAPABILITY_BANNER_MARGIN: i8 = 8;
const CAPABILITY_BANNER_ICON_SIZE: f32 = 15.0;
const CAPABILITY_BANNER_GAP: f32 = 7.0;
const NETLIST_OUTLINE_ROW_HEIGHT: f32 = 27.0;
const NETLIST_OUTLINE_TOUCH_ROW_HEIGHT: f32 = 44.0;
const NETLIST_OUTLINE_ICON_SIZE: f32 = 14.0;
const NETLIST_OUTLINE_PADDING_X: f32 = 9.0;
const NETLIST_OUTLINE_ICON_GAP: f32 = 7.0;

fn panel_search_field_width(available_width: f32) -> f32 {
    (available_width - PANEL_SEARCH_MARGIN_X * 2.0).max(1.0)
}

fn responsive_result_control_height(desktop_height: f32, control_height: f32) -> f32 {
    if control_height >= TOUCH_TARGET_HEIGHT {
        control_height.max(TOUCH_TARGET_HEIGHT)
    } else {
        desktop_height
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.spacing_mut().item_spacing.y = 0.0;
    header(ui, app);
    if app.state.workbench.workspace == Workspace::Netlist {
        // Canonical code navigator order: header, search, page tabs, outline.
        workspace_search(ui, app, Workspace::Netlist);
        code_workspace_pages(ui, app);
        netlist(ui, app);
        return;
    }
    match app.state.workbench.workspace {
        // A symbol cellview is a design document with its own structure:
        // an ordered pin contract instead of a hierarchy of instances.
        Workspace::Design if app.state.workspace.active_view_type() == ViewType::Symbol => {
            symbol::show(ui, app);
        }
        Workspace::Design => design::show(ui, app),
        // The data browser leads with its tab band and owns the search below
        // it, because the query and the kind/sort facets filter the tab that
        // is showing — a search above the tabs would read as filtering all
        // three at once.
        Workspace::Results => results(ui, app),
        workspace => {
            workspace_search(ui, app, workspace);
            match workspace {
                Workspace::Project => project(ui, app),
                Workspace::Simulate => simulate(ui, app),
                Workspace::Verify => {
                    let scroll_bar_visibility =
                        if verification_navigator_requires_scroll(ui.available_height()) {
                            egui::scroll_area::ScrollBarVisibility::AlwaysVisible
                        } else {
                            egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
                        };
                    ScrollArea::vertical()
                        .id_salt("workbench.verify.navigator")
                        .auto_shrink([false, false])
                        .scroll_bar_visibility(scroll_bar_visibility)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            verify(ui, app);
                        });
                }
                Workspace::Models => models(ui, app),
                Workspace::Netlist | Workspace::Design | Workspace::Results => {
                    unreachable!("handled above")
                }
            }
        }
    }
}

fn code_workspace_pages(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let t = Tokens::get(ui.ctx());
    let touch = app.state.workbench.coarse_pointer;
    let height = if touch { 44.0 } else { 31.0 };
    let width = ui.available_width().max(1.0);
    let (strip, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    ui.painter().rect_filled(strip, 0.0, t.color.bg_panel);
    ui.painter().hline(
        strip.x_range(),
        strip.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let content = strip.shrink2(Vec2::new(8.0, 0.0));
    let button_width = content.width() / CodeWorkspacePage::ALL.len() as f32;
    let mut selected = None;
    for (index, page) in CodeWorkspacePage::ALL.into_iter().enumerate() {
        let left = content.left() + button_width * index as f32;
        let right = if index + 1 == CodeWorkspacePage::ALL.len() {
            content.right()
        } else {
            left + button_width
        };
        let rect = egui::Rect::from_min_max(
            egui::pos2(left, content.top()),
            egui::pos2(right, content.bottom()),
        );
        let response = ui.interact(
            rect,
            egui::Id::new(("workbench.code.page", page.label())),
            Sense::click(),
        );
        let active = app.state.ui.code_workspace.page == page;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                active,
                page.label(),
            )
        });
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            page.label(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if active || response.hovered() {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        if active {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.left() + 6.0, rect.bottom() - 2.0),
                    egui::pos2(rect.right() - 6.0, rect.bottom()),
                ),
                0.0,
                t.color.accent,
            );
        }
        theme::paint_focus_ring(ui, &response, rect.shrink(2.0));
        if response.clicked() {
            selected = Some(page);
        }
    }
    if let Some(page) = selected {
        app.state.ui.code_workspace.page = page;
        app.state.ui.netlist.completion_open = false;
    }
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
    let title = match app.state.workbench.workspace {
        Workspace::Project => match app.state.workbench.project_page {
            ProjectPage::Overview | ProjectPage::Library => "Project",
            ProjectPage::Configuration => "Configurations",
            ProjectPage::Dependencies => "Dependencies",
            ProjectPage::Recovery => "Recovery",
        },
        Workspace::Design => "Design navigator",
        Workspace::Simulate => "Simulation Studio",
        Workspace::Results => "Data browser",
        Workspace::Verify => "Verification flows",
        Workspace::Models => "Library browser",
        Workspace::Netlist => "Netlist outline",
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

fn workspace_search(ui: &mut Ui, app: &mut RSpiceApp, workspace: Workspace) {
    let placeholder = match workspace {
        Workspace::Project => match app.state.workbench.project_page {
            ProjectPage::Overview | ProjectPage::Library => "Filter libraries, cells, views…",
            ProjectPage::Configuration => "Filter configurations…",
            ProjectPage::Dependencies => "Filter dependencies…",
            ProjectPage::Recovery => "Filter checkpoints…",
        },
        Workspace::Design => "Find instance, net or port…",
        Workspace::Simulate => "Filter setup…",
        Workspace::Results => "Find signal, expression or run…",
        Workspace::Verify => "Filter flows, specs or samples…",
        Workspace::Models => "Search model, device or library…",
        Workspace::Netlist => "Find symbol or line…",
    };
    panel_search(
        ui,
        &mut app.state.workbench.navigator_query,
        "workbench.navigator.filter",
        placeholder,
        &mut app.state.workbench.focus_navigator_search,
    );
}

pub(super) fn panel_search(
    ui: &mut Ui,
    query: &mut String,
    id: &'static str,
    placeholder: &'static str,
    focus_pending: &mut bool,
) {
    let t = Tokens::get(ui.ctx());
    let field_width = panel_search_field_width(ui.available_width());
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        let response = ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(query)
                .id_salt(id)
                .hint_text(placeholder)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .margin(egui::Margin {
                    left: 29,
                    right: 8,
                    top: 5,
                    bottom: 5,
                }),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(placeholder);
            node.set_description("Filter the current navigator contents");
        });
        WorkbenchIcon::Search.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                egui::vec2(16.0, 16.0),
            ),
            t.color.text_faint,
        );
        if std::mem::take(focus_pending) {
            response.request_focus();
        }
    });
    ui.add_space(8.0);
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical().show(ui, |ui| match app.state.workbench.project_page {
        ProjectPage::Overview | ProjectPage::Library => project_library_navigator(ui, app),
        ProjectPage::Configuration => project_configuration_navigator(ui, app),
        ProjectPage::Dependencies => project_dependency_navigator(ui, app),
        ProjectPage::Recovery => project_recovery_navigator(ui, app),
    });
}

fn project_library_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_query
        .trim()
        .to_ascii_lowercase();
    let libraries = app
        .state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let matches_query = query.is_empty()
                || library.name.to_ascii_lowercase().contains(&query)
                || library.cells_sorted().into_iter().any(|cell| {
                    cell.name.to_ascii_lowercase().contains(&query)
                        || cell
                            .views_sorted()
                            .into_iter()
                            .any(|view| view.name.to_ascii_lowercase().contains(&query))
                });
            (
                library.name.clone(),
                library.cell_count(),
                library.read_only,
                matches_query,
            )
        })
        .collect::<Vec<_>>();
    let visible = libraries
        .iter()
        .filter(|(_, _, _, visible)| *visible)
        .count();
    section_header(
        ui,
        "Libraries",
        Some(&format!("{visible} / {}", libraries.len())),
    );
    for (name, cells, read_only, visible) in libraries {
        if !visible {
            continue;
        }
        let meta = format!("{} {}", cells, if read_only { "ro" } else { "rw" });
        if nav_row(
            ui,
            if read_only {
                WorkbenchIcon::Folder
            } else {
                WorkbenchIcon::Project
            },
            &name,
            app.state.library_manager.selected_library.as_deref() == Some(name.as_str()),
            Some(&meta),
        ) {
            app.state.library_manager.select_library(&name);
        }
    }

    let active_configuration = app.state.workspace.configuration_sets.active();
    section_header(
        ui,
        "Active simulation context",
        Some(if active_configuration.is_some() {
            "current"
        } else {
            "review"
        }),
    );
    nav_property(
        ui,
        "Testbench",
        &active_configuration.map_or_else(
            || "not configured".to_owned(),
            |configuration| configuration.root().display_path(),
        ),
    );
    nav_property(
        ui,
        "Configuration",
        active_configuration.map_or("not configured", |configuration| configuration.name()),
    );
    nav_property(
        ui,
        "Run plan",
        app.state.sim_setup.active_plan_name().as_str(),
    );
    nav_property(
        ui,
        "Revision",
        &app.state.workspace.project.revision().get().to_string(),
    );

    section_header(ui, "Project source", None);
    nav_property(
        ui,
        "Path",
        &app.state
            .workspace
            .project
            .path
            .as_ref()
            .map_or_else(|| "not saved".to_owned(), |path| path.display().to_string()),
    );
    nav_property(
        ui,
        "Working tree",
        if crate::workbench::lifecycle::project_lifecycle::dirty_document_count(&app.state) > 0 {
            "modified"
        } else {
            "clean"
        },
    );
}

fn project_configuration_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_query
        .trim()
        .to_ascii_lowercase();
    let active_id = app
        .state
        .workspace
        .configuration_sets
        .active_configuration_id();
    let configurations = app
        .state
        .workspace
        .configuration_sets
        .configurations()
        .iter()
        .filter(|configuration| {
            query.is_empty()
                || configuration.name().to_ascii_lowercase().contains(&query)
                || configuration
                    .root()
                    .display_path()
                    .to_ascii_lowercase()
                    .contains(&query)
        })
        .map(|configuration| {
            (
                configuration.id(),
                configuration.name().to_owned(),
                configuration.revision(),
            )
        })
        .collect::<Vec<_>>();
    section_header(
        ui,
        "Configuration sets",
        Some(&format!(
            "{} / {}",
            configurations.len(),
            app.state
                .workspace
                .configuration_sets
                .configurations()
                .len()
        )),
    );
    for (id, name, revision) in configurations {
        let active = active_id == Some(id);
        if nav_row(
            ui,
            WorkbenchIcon::Sliders,
            &name,
            active,
            Some(if active { "current" } else { "available" }),
        ) {
            app.state.workbench.project_page = ProjectPage::Configuration;
            if !active {
                Command::ConfigurationSets.execute(app);
            }
        }
        if active {
            nav_property(ui, "Revision", &revision.to_string());
        }
    }
    section_header(ui, "Active binding", None);
    if let Some(configuration) = app.state.workspace.configuration_sets.active() {
        nav_property(ui, "Testbench", &configuration.root().display_path());
        nav_property(ui, "DUT", configuration.dut_path());
        nav_property(
            ui,
            "Overrides",
            &configuration.definition().overrides.len().to_string(),
        );
    } else {
        muted(ui, "No project configuration is active.");
    }
}

fn project_dependency_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_query
        .trim()
        .to_ascii_lowercase();
    let mut rows = app
        .state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            (
                library.name.clone(),
                "Design library",
                if library.read_only {
                    "read only"
                } else {
                    "writable"
                },
                WorkbenchIcon::Project,
            )
        })
        .chain(
            app.state
                .model_library_manager
                .libraries_sorted()
                .into_iter()
                .map(|library| {
                    (
                        library.name.clone(),
                        "Model library",
                        if library.source_authority.has_execution_source()
                            && !library.source_closure.is_empty()
                        {
                            "qualified"
                        } else {
                            "catalog"
                        },
                        WorkbenchIcon::Models,
                    )
                }),
        )
        .filter(|(name, kind, status, _)| {
            query.is_empty()
                || format!("{name} {kind} {status}")
                    .to_ascii_lowercase()
                    .contains(&query)
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    if app.state.workbench.project_dependency_selection.is_none() {
        app.state.workbench.project_dependency_selection =
            rows.first().map(|(name, _, _, _)| name.clone());
    }
    section_header(ui, "Resolved closure", Some(&rows.len().to_string()));
    for (name, _, status, icon) in &rows {
        if nav_row(
            ui,
            *icon,
            name,
            app.state.workbench.project_dependency_selection.as_deref() == Some(name.as_str()),
            Some(status),
        ) {
            app.state.workbench.project_dependency_selection = Some(name.clone());
        }
    }
    let unresolved = rows
        .iter()
        .filter(|(_, _, status, _)| *status == "catalog")
        .count();
    section_header(
        ui,
        "Closure contract",
        Some(if unresolved == 0 {
            "portable"
        } else {
            "review"
        }),
    );
    nav_property(ui, "Missing", "0");
    nav_property(ui, "Source advisories", &unresolved.to_string());
    nav_property(
        ui,
        "Technology",
        if app.state.workspace.project.technology_binding().is_some() {
            "attached"
        } else {
            "not attached"
        },
    );
    nav_property(ui, "Manifest", "project dependencies");
}

fn project_recovery_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_query
        .trim()
        .to_ascii_lowercase();
    let checkpoints = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .checkpoints
        .iter()
        .filter(|checkpoint| {
            query.is_empty()
                || format!(
                    "{} {} {}",
                    checkpoint.reason().label(),
                    checkpoint.project_revision(),
                    checkpoint.checkpoint_id()
                )
                .to_ascii_lowercase()
                .contains(&query)
        })
        .cloned()
        .collect::<Vec<_>>();
    if app.state.workbench.project_checkpoint_selection.is_none() {
        app.state.workbench.project_checkpoint_selection = checkpoints
            .first()
            .map(|checkpoint| checkpoint.checkpoint_id().to_string());
    }
    section_header(
        ui,
        "Project checkpoints",
        Some(&checkpoints.len().to_string()),
    );
    for checkpoint in &checkpoints {
        let id = checkpoint.checkpoint_id().to_string();
        if nav_row(
            ui,
            WorkbenchIcon::History,
            checkpoint.reason().label(),
            app.state.workbench.project_checkpoint_selection.as_deref() == Some(id.as_str()),
            Some("restorable"),
        ) {
            app.state.workbench.project_checkpoint_selection = Some(id);
        }
    }
    section_header(
        ui,
        "Recovery store",
        Some(
            if app
                .state
                .dialogs
                .project_checkpoint_recovery
                .quarantined
                .is_empty()
            {
                "verified"
            } else {
                "review"
            },
        ),
    );
    nav_property(ui, "Protected", &checkpoints.len().to_string());
    nav_property(
        ui,
        "Quarantined",
        &app.state
            .dialogs
            .project_checkpoint_recovery
            .quarantined
            .len()
            .to_string(),
    );
    nav_property(ui, "Integrity", "payloads verified on load");
}

fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::simulation::plan::AnalysisKind;

    let enabled = app.state.sim_setup.enabled_analysis_instance_count();
    let total = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.instances().len());
    let query = app.state.workbench.navigator_query.trim().to_lowercase();
    section_header(ui, "Lab characterization", Some(&format!("{enabled} on")));
    ScrollArea::vertical()
        .id_salt("workbench.simulation.navigator")
        .show(ui, |ui| {
            let active = app.state.workbench.simulation_page;
            let analyses_meta = format!("{total} active · {enabled} enabled");
            let mut requested = None;
            for page in SimulationPage::NAVIGATION {
                let label = page.label();
                if !query.is_empty() && !label.to_lowercase().contains(&query) {
                    continue;
                }
                let meta = simulate_nav_meta(app, page, &analyses_meta);
                if nav_row(
                    ui,
                    simulate_nav_icon(page),
                    label,
                    active == page,
                    meta.as_deref(),
                ) {
                    requested = Some(page);
                }
            }
            if let Some(page) = requested {
                app.state.workbench.simulation_page = page;
                app.state.workbench.close_drawer();
            }
            simulation_plan_creator(ui, app);
            section_header(ui, "Run set", Some("Reference point"));
            nav_property(
                ui,
                "Process",
                app.state.sim_setup.reference_pvt.process.short_name(),
            );
            nav_property(
                ui,
                "Temperature",
                &format!(
                    "{} °C",
                    app.state.sim_setup.reference_pvt.temperature_celsius
                ),
            );
            nav_property(
                ui,
                "Variation",
                if app
                    .state
                    .sim_setup
                    .has_enabled_analysis_kind(AnalysisKind::MonteCarlo)
                {
                    "enabled"
                } else {
                    "disabled"
                },
            );
            ui.add_space(8.0);
            section_header(ui, "Capability policy", None);
            capability_policy_banner(ui);
        });
}

/// The panel's one creating action, at the foot of the tree it adds to.
///
/// Full-bleed and square rather than an inset pill: it reads as the last row
/// of the tree rather than a control floating inside the panel, and it carries
/// the filled accent so it is the obvious way into an empty plan.
fn simulation_plan_creator(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.add_space(SIMULATION_CREATOR_GAP);
    let width = ui.available_width();
    if Button::new("Add analysis or workflow…")
        .accent()
        .min_width(width)
        .min_height(Tokens::get(ui.ctx()).metrics.row_h + SIMULATION_CREATOR_EXTRA_HEIGHT)
        .square()
        .show(ui)
        .clicked()
    {
        // On tablet and phone this action is hosted by the navigator drawer.
        // Close that transient layer before opening the catalog so the modal
        // is never obscured by its invoker.
        app.state.workbench.close_drawer();
        app.state.sim_setup.palette_open = true;
        app.state.sim_setup.palette_query.clear();
        app.state.sim_setup.palette_active = 0;
        app.state.sim_setup.palette_scroll_to_active = true;
    }
}

const fn simulate_nav_icon(page: SimulationPage) -> WorkbenchIcon {
    match page {
        SimulationPage::Analyses => WorkbenchIcon::Results,
        SimulationPage::Variables => WorkbenchIcon::Sliders,
        SimulationPage::Outputs => WorkbenchIcon::Probe,
        SimulationPage::Specifications => WorkbenchIcon::Target,
        SimulationPage::RunSet => WorkbenchIcon::Grid,
        SimulationPage::Models => WorkbenchIcon::Models,
        SimulationPage::Solver => WorkbenchIcon::Settings,
        SimulationPage::Save => WorkbenchIcon::Save,
    }
}

/// Row meta is a count of what the page owns, read from the same state the
/// page edits. A page whose owner is empty shows nothing rather than a zero,
/// so the column does not fill with noise before a plan has been authored.
fn simulate_nav_meta(app: &RSpiceApp, page: SimulationPage, analyses: &str) -> Option<String> {
    let payload = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.active_plan_data(plan_id));
    let count = |value: usize| (value > 0).then(|| value.to_string());
    match page {
        SimulationPage::Analyses => Some(analyses.to_owned()),
        SimulationPage::Variables => count(payload.map_or(0, |data| data.design_variables.len())),
        SimulationPage::Outputs | SimulationPage::Save => {
            count(payload.map_or(0, |data| data.saved_outputs.len()))
        }
        SimulationPage::Specifications => count(payload.map_or(0, |data| data.specs.len())),
        SimulationPage::RunSet => app
            .state
            .sim_setup
            .run_set_point_count()
            .map(|points| format!("{points} pt")),
        SimulationPage::Models => count(app.state.model_library_manager.libraries_sorted().len()),
        // The active numerical policy, not a count — the tree's job is to say
        // what each route currently holds, and for the solver that is which
        // preset the effective options match.
        SimulationPage::Solver => Some(app.state.sim_setup.options.preset_label()),
    }
}

fn capability_policy_banner(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::new()
        .fill(t.color.bg_inset)
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::same(CAPABILITY_BANNER_MARGIN))
        .outer_margin(egui::Margin::same(CAPABILITY_BANNER_MARGIN))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = CAPABILITY_BANNER_GAP;
                let (icon_rect, _) = ui.allocate_exact_size(
                    egui::Vec2::splat(CAPABILITY_BANNER_ICON_SIZE),
                    egui::Sense::hover(),
                );
                WorkbenchIcon::Info.paint(ui.painter(), icon_rect, t.color.text_dim);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "Every analysis declares release-engine, preview, compatibility, platform and sign-off contracts before it can be added.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
        })
        .response;
    let border_rect = response.rect.shrink(f32::from(CAPABILITY_BANNER_MARGIN));
    let outline = [
        border_rect.left_top(),
        border_rect.right_top(),
        border_rect.right_bottom(),
        border_rect.left_bottom(),
        border_rect.left_top(),
    ];
    ui.painter().add(egui::Shape::dashed_line(
        &outline,
        egui::Stroke::new(1.0, t.color.border_strong),
        3.0,
        3.0,
    ));
}

fn nav_property(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_space(NAV_PROPERTY_PADDING_X);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_space(NAV_PROPERTY_PADDING_X);
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text),
            );
        });
    });
}

fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app.state.workbench.navigator_query.trim().to_lowercase();
    // Kind/sort read the values the toolbar stored last frame — the row
    // renders below the tab band while filtering happens here, the same
    // one-frame contract the viewer-tab chevrons use.
    let kind = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserKind>(results_browser_kind_id()))
        .unwrap_or_default();
    let sort = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserSort>(results_browser_sort_id()))
        .unwrap_or_default();
    let scope = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserScope>(results_browser_scope_id()))
        .unwrap_or_default();
    // A snapshot keeps the favorites predicate borrow-free inside the run map;
    // recency reads its owning accessor, which states the rank once.
    let favorite_signals = app.state.ui.results.favorite_signals.clone();
    let active_run = app.state.simulation.active_run_idx;
    let active_analysis = app.state.simulation.active_analysis_idx;
    let selected_trace = app
        .state
        .ui
        .results
        .valid_selected_trace(&app.state.simulation)
        .cloned();
    let expressions = active_analysis
        .and_then(|analysis_index| app.state.ui.results.exprs.get(&analysis_index))
        .map(|expressions| {
            expressions
                .iter()
                .enumerate()
                .filter(|(_, expression)| {
                    query.is_empty() || expression.text.to_lowercase().contains(&query)
                })
                .map(|(expression_index, expression)| (expression_index, expression.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let expression_query_match = !query.is_empty() && !expressions.is_empty();
    let runs = app
        .state
        .simulation
        .runs
        .iter()
        .enumerate()
        .filter_map(|(run_index, run)| {
            let analyses = run
                .analyses
                .iter()
                .enumerate()
                .filter_map(|(analysis_index, analysis)| {
                    let mut signals = analysis
                        .waveforms
                        .iter()
                        .enumerate()
                        .filter(|(_, waveform)| {
                            query.is_empty() || waveform.name.to_lowercase().contains(&query)
                        })
                        .filter(|(_, waveform)| {
                            // The unit the signal actually reads in decides
                            // its kind. Treating "not a current" as a voltage
                            // filed noise densities and decibel magnitudes
                            // under Voltage.
                            kind.admits(
                                crate::workbench::documents::result_document::browser_signal_unit(
                                    &waveform.name,
                                    crate::workbench::documents::result_document::analysis_default_unit(
                                        analysis.analysis_type,
                                    ),
                                ),
                            )
                        })
                        .filter(|(_, waveform)| match scope {
                            ResultsBrowserScope::All => true,
                            ResultsBrowserScope::Favorites => {
                                favorite_signals.contains(&waveform.name)
                            }
                            ResultsBrowserScope::Recent => app
                                .state
                                .ui
                                .results
                                .recent_signal_rank(&waveform.name)
                                .is_some(),
                        })
                        .map(|(waveform_index, waveform)| {
                            let unit =
                                crate::workbench::documents::result_document::browser_signal_unit(
                                    &waveform.name,
                                    analysis.analysis_type.axis_info().3,
                                );
                            let samples = waveform.y.len();
                            ResultSignal {
                                waveform_index,
                                name: waveform.name.clone(),
                                color: waveform.color.clone(),
                                visible: waveform.visible,
                                meta: format!(
                                    "{} · vector · {} {}",
                                    result_quantity_kind_label(&waveform.name, unit),
                                    grouped_count(samples),
                                    if samples == 1 { "value" } else { "samples" },
                                ),
                                value: waveform
                                    .y
                                    .iter()
                                    .rev()
                                    .copied()
                                    .find(|value| value.is_finite())
                                    .map(|value| {
                                        // Degrees carry no SI prefix; a
                                        // near-zero phase as "f°" misreads.
                                        if unit == "°" {
                                            format!("{value:.3} °")
                                        } else {
                                            crate::ui::plot::fmt_si(value, unit, 3)
                                        }
                                    }),
                            }
                        })
                        .collect::<Vec<_>>();
                    if scope == ResultsBrowserScope::Recent {
                        // Recent means recency order; the sort facet yields.
                        signals.sort_by_key(|signal| {
                            app.state
                                .ui
                                .results
                                .recent_signal_rank(&signal.name)
                                .unwrap_or(usize::MAX)
                        });
                    } else if sort == ResultsBrowserSort::Name {
                        signals.sort_by(|left, right| left.name.cmp(&right.name));
                    }
                    let matches_analysis = query.is_empty()
                        || analysis.label.to_lowercase().contains(&query)
                        || analysis
                            .analysis_type
                            .display_name()
                            .to_lowercase()
                            .contains(&query)
                        || !signals.is_empty();
                    // A deck that labelled its analysis with the same code the
                    // kind glyph carries would print that code twice; the
                    // canonical name is the useful title in that case.
                    let label = if analysis
                        .label
                        .eq_ignore_ascii_case(analysis.analysis_type.short_label())
                    {
                        analysis.analysis_type.display_name().to_owned()
                    } else {
                        analysis.label.clone()
                    };
                    matches_analysis.then(|| ResultAnalysis {
                        domain: result_analysis_domain(
                            analysis.analysis_type,
                            analysis.waveforms.first(),
                        ),
                        total_signals: analysis.waveforms.len(),
                        analysis_index,
                        presentation_key:
                            crate::workbench::documents::result_document::AnalysisPresentationKey::new(
                                run.dataset_id,
                                analysis,
                            ),
                        label,
                        short_label: analysis.analysis_type.short_label(),
                        success: analysis.success,
                        signals,
                    })
                })
                .collect::<Vec<_>>();
            let matches_run = query.is_empty()
                || run.label.to_lowercase().contains(&query)
                || !analyses.is_empty()
                || (active_run == Some(run_index) && expression_query_match);
            matches_run.then(|| ResultRun {
                run_index,
                dataset_id: run.dataset_id,
                label: run.label.clone(),
                success: run.success,
                analysis_count: run.analyses.len(),
                analyses,
            })
        })
        .collect::<Vec<_>>();

    let signal_count = runs
        .iter()
        .filter(|run| active_run == Some(run.run_index))
        .flat_map(|run| run.analyses.iter())
        .map(|analysis| analysis.signals.len())
        .sum::<usize>();
    let tab = results_browser_tab_band(ui, [signal_count, runs.len(), expressions.len()]);
    // The mockup's browser toolbar: the query over this tab, then the kind
    // and sort facets. Both are absent on Datasets — a manifest binding has
    // no kind to filter and no hierarchy to order.
    if tab != ResultsBrowserTab::Datasets {
        results_browser_toolbar(ui, app, kind, sort);
    }
    // The mockup's status band: the signals tab owns the scope control on
    // the left — All | Favorites | Recent over the session's real star and
    // recency state — and every tab keeps the live inventory count on the
    // right, with the provenance sentence riding its tooltip.
    {
        let t = Tokens::get(ui.ctx());
        // A narrowing of any kind puts the count in the mockup's two-number
        // form, so the reader can see how much of the dataset is hidden
        // rather than only how much survived.
        let filtering =
            !query.is_empty() || kind != ResultsBrowserKind::All || scope != ResultsBrowserScope::All;
        let (shown, noun) = match tab {
            ResultsBrowserTab::Signals => (signal_count, "signals"),
            ResultsBrowserTab::Datasets => (runs.len(), "immutable datasets"),
            ResultsBrowserTab::Expressions => (expressions.len(), "expressions"),
        };
        let loaded = match tab {
            ResultsBrowserTab::Signals => app
                .state
                .simulation
                .active_run()
                .map(|run| {
                    run.analyses
                        .iter()
                        .map(|analysis| analysis.waveforms.len())
                        .sum::<usize>()
                })
                .unwrap_or(0),
            ResultsBrowserTab::Datasets => app.state.simulation.runs.len(),
            ResultsBrowserTab::Expressions => app
                .state
                .simulation
                .active_analysis_idx
                .and_then(|index| app.state.ui.results.exprs.get(&index))
                .map_or(0, Vec::len),
        };
        let count_copy = if filtering {
            format!("{shown} / {loaded}")
        } else if tab == ResultsBrowserTab::Signals {
            match scope {
                ResultsBrowserScope::All => format!("{shown} {noun}"),
                ResultsBrowserScope::Favorites => format!("{shown} starred"),
                ResultsBrowserScope::Recent => format!("{shown} recent"),
            }
        } else {
            format!("{shown} {noun}")
        };
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 28.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);
                if tab == ResultsBrowserTab::Signals {
                    // One scope is in force at a time, so the three read as
                    // one control rather than three independent marks.
                    const SCOPES: [ResultsBrowserScope; 3] = [
                        ResultsBrowserScope::All,
                        ResultsBrowserScope::Favorites,
                        ResultsBrowserScope::Recent,
                    ];
                    let mut index = SCOPES
                        .iter()
                        .position(|candidate| *candidate == scope)
                        .unwrap_or(0);
                    let changed = crate::ui::widgets::segmented(
                        ui,
                        "workbench.results.browser-scope",
                        &["All", "Favorites", "Recent"],
                        &mut index,
                        crate::ui::widgets::SegmentedWidth::Natural,
                    );
                    if changed {
                        let scope_now = SCOPES[index];
                        ui.ctx().data_mut(|data| {
                            data.insert_temp(results_browser_scope_id(), scope_now);
                        });
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(count_copy)
                            .font(theme::mono(tokens::FS_MICRO, FontWeight::Medium))
                            .color(t.color.text_faint),
                    )
                    .on_hover_text(
                        "Live inventory of the retained evidence behind this tab · exact metadata · stable IDs",
                    );
                });
            },
        );
    }
    // The mockup's `.result-browser-selection`: what a batch action would
    // act on, with only the actions RSpice actually performs on a set of
    // quantities — plot membership, and letting the set go.
    if tab == ResultsBrowserTab::Signals && !app.state.ui.results.checked_signals.is_empty() {
        let t = Tokens::get(ui.ctx());
        let checked = app.state.ui.results.checked_signals.clone();
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 29.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, t.color.accent_dim);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!("{} selected", checked.len()))
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(6.0);
                    if ui.small_button("Clear").clicked() {
                        app.state.ui.results.clear_checked_signals();
                    }
                    if ui
                        .small_button("Hide")
                        .on_hover_text("Remove every selected quantity from its pane")
                        .clicked()
                    {
                        set_checked_signal_visibility(app, &checked, false);
                    }
                    if ui
                        .small_button("Show")
                        .on_hover_text("Add every selected quantity to its pane")
                        .clicked()
                    {
                        set_checked_signal_visibility(app, &checked, true);
                    }
                });
            },
        );
    }
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .show(ui, |ui| {
            match tab {
                ResultsBrowserTab::Signals => {
                    let mut any = false;
                    for run in runs
                        .into_iter()
                        .filter(|run| active_run == Some(run.run_index))
                    {
                        for analysis in run.analyses {
                            let analysis_active = active_analysis == Some(analysis.analysis_index);
                            let signal_count = analysis.signals.len();
                            // A query is a request to see matches wherever
                            // they live, so it opens every group it hit.
                            let expanded = !query.is_empty()
                                || result_browser_group_expanded(
                                    ui.ctx(),
                                    analysis.analysis_index,
                                    analysis_active,
                                );
                            if result_browser_analysis_head(
                                ui,
                                analysis.short_label,
                                &analysis.label,
                                &analysis.domain,
                                signal_count,
                                analysis.total_signals,
                                analysis_active,
                                analysis.success,
                                expanded,
                            )
                            .clicked()
                            {
                                // The head owns disclosure; a quantity row
                                // owns selection. Selecting the analysis too
                                // would make every open cost a selection.
                                if query.is_empty() {
                                    set_result_browser_group_expanded(
                                        ui.ctx(),
                                        analysis.analysis_index,
                                        !expanded,
                                    );
                                }
                                select_result_analysis(app, run.run_index, analysis.analysis_index);
                            }
                            if !expanded {
                                continue;
                            }
                            if signal_count == 0 {
                                // The mockup's omission card: a group that
                                // lists nothing must say why, or it reads as
                                // a dataset that lost its quantities.
                                result_browser_omission(
                                    ui,
                                    if analysis.total_signals == 0 {
                                        "This analysis retained a scalar solution, not a sampled series. Its values are in the OP viewer."
                                    } else {
                                        "Every quantity of this analysis is filtered out by the current query, kind, or scope."
                                    },
                                );
                            }
                            for signal in analysis.signals {
                                any = true;
                                let t = Tokens::get(ui.ctx());
                                let color =
                                    crate::workbench::documents::result_document::trace_color(
                                        &signal.color,
                                        t.color.traces
                                            [signal.waveform_index % t.color.traces.len()],
                                    );
                                let signal_selected =
                                    selected_trace.as_ref().is_some_and(|selected| {
                                        selected.analysis_key() == analysis.presentation_key
                                            && selected.source_name() == signal.name
                                    });
                                let row_id = ui.id().with((
                                    "result-signal",
                                    analysis.analysis_index,
                                    signal.waveform_index,
                                ));
                                let responses = result_quantity_row(
                                    ui,
                                    row_id,
                                    &signal.name,
                                    &signal.meta,
                                    signal.value.as_deref(),
                                    color,
                                    signal_selected,
                                    signal.visible,
                                    app.state.ui.results.is_favorite_signal(&signal.name),
                                    app.state.ui.results.is_checked_signal(&signal.name),
                                );
                                if responses.check.clicked() {
                                    app.state
                                        .ui
                                        .results
                                        .toggle_checked_signal(&signal.name);
                                }
                                if responses.visibility.clicked() {
                                    crate::workbench::documents::result_document::toggle_visibility(
                                        &mut app.state,
                                        analysis.analysis_index,
                                        signal.waveform_index,
                                    );
                                }
                                if responses
                                    .favorite
                                    .as_ref()
                                    .is_some_and(|star| star.clicked())
                                {
                                    app.state
                                        .ui
                                        .results
                                        .toggle_favorite_signal(&signal.name);
                                }
                                if responses.selection.clicked() {
                                    select_result_signal(
                                        app,
                                        run.run_index,
                                        analysis.analysis_index,
                                        signal.waveform_index,
                                    );
                                }
                                locate_on_schematic_menu(&responses.selection, app, &signal.name);
                            }
                        }
                    }
                    if !any {
                        muted(
                            ui,
                            if app.state.simulation.runs.is_empty() {
                                "Run a simulation to create an immutable result dataset."
                            } else {
                                match scope {
                                    ResultsBrowserScope::Favorites => {
                                        "Star a signal to build the Favorites scope."
                                    }
                                    ResultsBrowserScope::Recent => {
                                        "Signals you select or show collect here."
                                    }
                                    ResultsBrowserScope::All => {
                                        "No signal of the active dataset matches this filter."
                                    }
                                }
                            },
                        );
                        results_browser_clear_filters(ui, app, kind, scope);
                    }
                }
                ResultsBrowserTab::Datasets => {
                    if runs.is_empty() {
                        muted(
                            ui,
                            if app.state.simulation.runs.is_empty() {
                                "Run a simulation to create an immutable result dataset."
                            } else {
                                "No dataset or analysis matches this filter."
                            },
                        );
                    }
                    for run in runs {
                        let run_active = active_run == Some(run.run_index);
                        let run_meta = format!("{} analyses", run.analysis_count);
                        let overlaid = app.state.simulation.is_dataset_overlaid(run.dataset_id);
                        let responses = result_dataset_row(
                            ui,
                            if run.success {
                                WorkbenchIcon::Success
                            } else {
                                WorkbenchIcon::Warning
                            },
                            &run.label,
                            run_active,
                            &run_meta,
                            overlaid,
                        );
                        if responses.selection.clicked() {
                            select_result_dataset(app, run.run_index);
                        }
                        if responses.overlay.is_some_and(|response| response.clicked()) {
                            let enabled =
                                app.state.simulation.toggle_dataset_overlay(run.dataset_id);
                            app.state.push_user_message(
                                crate::diagnostics::ConsoleMessage::info(if enabled {
                                    format!("Overlaying {} on the active result sheet.", run.label)
                                } else {
                                    format!("Removed {} from the active result sheet.", run.label)
                                }),
                            );
                        }
                        if !run_active && query.is_empty() {
                            continue;
                        }
                        for analysis in run.analyses {
                            let analysis_active =
                                run_active && active_analysis == Some(analysis.analysis_index);
                            if result_browser_manifest_row(
                                ui,
                                analysis.short_label,
                                &analysis.label,
                                &analysis.domain,
                                analysis_active,
                                analysis.success,
                            )
                            .clicked()
                            {
                                select_result_analysis(app, run.run_index, analysis.analysis_index);
                            }
                        }
                    }
                }
                ResultsBrowserTab::Expressions => {
                    let Some(analysis_index) = app.state.simulation.active_analysis_idx else {
                        muted(ui, "Select a retained result analysis to own expressions.");
                        return;
                    };
                    expression_header(ui, app);
                    let mut toggled_expression = None;
                    for (expression_index, expression) in &expressions {
                        let t = Tokens::get(ui.ctx());
                        let row_id = ui
                            .id()
                            .with(("result-expression", analysis_index, *expression_index));
                        let responses = signal_row(
                            ui,
                            row_id,
                            &expression.text,
                            Some("expression"),
                            t.color.traces[*expression_index % t.color.traces.len()],
                            false,
                            expression.visible,
                        );
                        if responses.visibility.clicked() {
                            toggled_expression = Some(*expression_index);
                        }
                        locate_on_schematic_menu(&responses.selection, app, &expression.text);
                    }
                    if let Some(expression_index) = toggled_expression
                        && let Some(expression) = app
                            .state
                            .ui
                            .results
                            .exprs
                            .get_mut(&analysis_index)
                            .and_then(|expressions| expressions.get_mut(expression_index))
                    {
                        expression.visible = !expression.visible;
                    }
                }
            }
            // The mockup's browser inspector: the selected quantity's exact
            // retained identity, present only while a selection exists.
            if let Some(selected) = &selected_trace
                && let Some(run) = app.state.simulation.active_run()
                && let Some((analysis, waveform)) = run.analyses.iter().find_map(|analysis| {
                    analysis
                        .waveforms
                        .iter()
                        .find(|waveform| waveform.name == selected.source_name())
                        .map(|waveform| (analysis, waveform))
                })
            {
                ui.add_space(6.0);
                section_header(ui, "Selection", None);
                let t = Tokens::get(ui.ctx());
                for (label, value) in [
                    ("Quantity", waveform.name.clone()),
                    ("Analysis", analysis.label.clone()),
                    ("Samples", waveform.y.len().to_string()),
                    ("Dataset", format!("Run {}", run.id)),
                ] {
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(label)
                                .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(value)
                                        .font(theme::mono(tokens::FS_MICRO, FontWeight::Medium))
                                        .color(t.color.text_dim),
                                );
                            },
                        );
                    });
                }
            }
            // The mockup's metadata footnote: preview values are formatted
            // display metadata; exact operations resolve stored values.
            ui.add_space(6.0);
            muted(
                ui,
                "Preview values are formatted display metadata. Copy, measurement, comparison, and export resolve stored values from the immutable dataset.",
            );
        });
}

/// Which content the results data browser shows. Stored in egui memory like
/// the pane log toggles: browser focus is a session gesture, not document
/// state.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserTab {
    #[default]
    Signals,
    Datasets,
    Expressions,
}

/// Quantity-kind facet of the browser toolbar.
///
/// The kinds are exactly the ones the results unit owner can name from a
/// retained signal — anything it cannot name belongs to no kind rather than
/// being swept into the commonest one. The mockup lists further kinds
/// (complex, spectrum, scalar, contribution) that a name and a unit alone do
/// not distinguish here; offering them would be a taxonomy this browser
/// cannot actually apply.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserKind {
    #[default]
    All,
    Voltage,
    Current,
    Power,
    NoiseDensity,
}

impl ResultsBrowserKind {
    const ALL: [Self; 5] = [
        Self::All,
        Self::Voltage,
        Self::Current,
        Self::Power,
        Self::NoiseDensity,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All kinds",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Power => "Power",
            Self::NoiseDensity => "Noise density",
        }
    }

    /// Whether a signal reading in `unit` belongs to this kind.
    fn admits(self, unit: &str) -> bool {
        match self {
            Self::All => true,
            Self::Voltage => unit == "V",
            Self::Current => unit == "A",
            Self::Power => unit == "W",
            Self::NoiseDensity => matches!(unit, "nV/√Hz" | "V^2/Hz"),
        }
    }
}

/// Row ordering facet: retained hierarchy order, or name.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserSort {
    #[default]
    Hierarchy,
    Name,
}

impl ResultsBrowserSort {
    const ALL: [Self; 2] = [Self::Hierarchy, Self::Name];

    const fn label(self) -> &'static str {
        match self {
            Self::Hierarchy => "Hierarchy order",
            // The mockup's third option, "Recently used", is the Recent
            // scope on the status band below: one owner for recency, not a
            // sort that silently competes with it.
            Self::Name => "Name",
        }
    }
}

/// Working-set scope of the signals tab: everything, the user's starred
/// signals, or the signals touched most recently. Backed by the real
/// favorite/recency state on `ResultsState`, never a static filter.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserScope {
    #[default]
    All,
    Favorites,
    Recent,
}

/// One way back when a narrowing has hidden everything.
///
/// A reader who filters to nothing has to be able to undo it without
/// remembering which of the three controls did it. Nothing is offered when
/// the emptiness is the dataset's own — there would be no filter to clear.
fn results_browser_clear_filters(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    let filtering = !app.state.workbench.navigator_query.trim().is_empty()
        || kind != ResultsBrowserKind::All
        || scope != ResultsBrowserScope::All;
    if !filtering {
        return;
    }
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        if ui
            .small_button("Clear filters")
            .on_hover_text("Reset the query, the kind facet, and the scope")
            .clicked()
        {
            app.state.workbench.navigator_query.clear();
            ui.ctx().data_mut(|data| {
                data.insert_temp(results_browser_kind_id(), ResultsBrowserKind::All);
                data.insert_temp(results_browser_scope_id(), ResultsBrowserScope::All);
            });
        }
    });
}

/// The data browser's own toolbar: the query, then the kind and sort facets.
///
/// The mockup lays this out as one bordered block under the tab band — an
/// 8 px inset, a 5 px gutter, and two facets that split the row evenly. They
/// are sized to the panel rather than to their own labels so the pair reads
/// as one control row at any dock width, and so neither facet's width
/// implies it is the more important of the two.
fn results_browser_toolbar(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    kind: ResultsBrowserKind,
    sort: ResultsBrowserSort,
) {
    const GUTTER: f32 = 5.0;

    let t = Tokens::get(ui.ctx());
    ui.add_space(7.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        let field_width = panel_search_field_width(ui.available_width() + PANEL_SEARCH_MARGIN_X);
        let placeholder = "Find canonical name, path, unit, or type…";
        let response = ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(&mut app.state.workbench.navigator_query)
                .id_salt("workbench.navigator.filter")
                .hint_text(placeholder)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .margin(egui::Margin {
                    left: 29,
                    right: 8,
                    top: 5,
                    bottom: 5,
                }),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(placeholder);
            node.set_description("Filter the quantities of the selected browser tab");
        });
        WorkbenchIcon::Search.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                egui::vec2(16.0, 16.0),
            ),
            t.color.text_faint,
        );
        if std::mem::take(&mut app.state.workbench.focus_navigator_search) {
            response.request_focus();
        }
    });
    ui.add_space(GUTTER);
    ui.horizontal(|ui| {
        // Spacing is added explicitly: an `add_space` inside a horizontal
        // with item spacing pays for both, and the facets drift off the
        // query field's inset.
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        // The design system's select allocates exactly the width it is
        // given, so the pair can be halved against the row. A raw combo box
        // pads itself past the width it is asked for, which pushed the sort
        // facet off the query's inset and clipped it at the panel edge.
        let row_width = ui.available_width() - PANEL_SEARCH_MARGIN_X;
        let facet_width = ((row_width - GUTTER) / 2.0).max(48.0);
        let kind_options = ResultsBrowserKind::ALL
            .map(|value| value.label().to_owned())
            .to_vec();
        if let Some(picked) = crate::ui::widgets::select(
            ui,
            "workbench.results.browser-kind",
            "Quantity kind",
            kind.label(),
            &kind_options,
            facet_width,
        ) {
            let kind_now = ResultsBrowserKind::ALL[picked];
            ui.ctx()
                .data_mut(|data| data.insert_temp(results_browser_kind_id(), kind_now));
        }
        ui.add_space(GUTTER);
        let sort_options = ResultsBrowserSort::ALL
            .map(|value| value.label().to_owned())
            .to_vec();
        if let Some(picked) = crate::ui::widgets::select(
            ui,
            "workbench.results.browser-sort",
            "Quantity sort",
            sort.label(),
            &sort_options,
            facet_width,
        ) {
            let sort_now = ResultsBrowserSort::ALL[picked];
            ui.ctx()
                .data_mut(|data| data.insert_temp(results_browser_sort_id(), sort_now));
        }
    });
    ui.add_space(7.0);
    // The toolbar is one block, so it closes with a rule like the tab band
    // above it rather than bleeding into the status band below.
    ui.painter().hline(
        egui::Rangef::new(ui.max_rect().left(), ui.max_rect().right()),
        ui.cursor().top() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn results_browser_kind_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-kind-facet")
}

fn results_browser_sort_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-sort-facet")
}

fn results_browser_scope_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-scope")
}

/// Disclosure state for one analysis group, defaulting to open for the
/// active analysis and closed for the rest: a run with a hundred retained
/// quantities should open as an index of its analyses, not one long list.
fn result_browser_group_expanded(ctx: &egui::Context, analysis_index: usize, active: bool) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new((
            "workbench.results.browser-group",
            analysis_index,
        )))
    })
    .unwrap_or(active)
}

fn set_result_browser_group_expanded(ctx: &egui::Context, analysis_index: usize, expanded: bool) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.results.browser-group", analysis_index)),
            expanded,
        );
    });
}

/// The mockup's data-browser tab band: three equal columns on one 30 px
/// row, label and mono count sharing the cell, a 2 px accent baseline on
/// the active tab. Returns the tab to render this frame.
fn results_browser_tab_band(ui: &mut Ui, counts: [usize; 3]) -> ResultsBrowserTab {
    let t = Tokens::get(ui.ctx());
    let band_id = egui::Id::new("workbench.results.browser-tab");
    let mut tab = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserTab>(band_id))
        .unwrap_or_default();
    let tabs = [
        (ResultsBrowserTab::Signals, "Signals", counts[0]),
        (ResultsBrowserTab::Datasets, "Datasets", counts[1]),
        (ResultsBrowserTab::Expressions, "Expressions", counts[2]),
    ];
    let (band, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 30.0), egui::Sense::hover());
    let painter = ui.painter_at(band);
    let column_width = band.width() / tabs.len() as f32;
    for (index, (candidate, label, count)) in tabs.into_iter().enumerate() {
        let cell = egui::Rect::from_min_max(
            egui::pos2(band.left() + column_width * index as f32, band.top()),
            egui::pos2(
                band.left() + column_width * (index + 1) as f32,
                band.bottom(),
            ),
        );
        let response = ui
            .interact(
                cell,
                ui.id().with(("results-browser-tab", index)),
                egui::Sense::click(),
            )
            .on_hover_text(format!("Show {label}"));
        let selected = tab == candidate;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                format!("{label}, {count}"),
            )
        });
        if response.clicked() {
            tab = candidate;
            ui.ctx().data_mut(|data| data.insert_temp(band_id, tab));
        }
        if response.hovered() && !selected {
            painter.rect_filled(cell, 0.0, t.color.bg_hover);
        }
        let label_color = if selected || response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        };
        let count_color = if selected {
            t.color.accent
        } else {
            t.color.text_faint
        };
        let label_galley = painter.layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_MICRO, FontWeight::SemiBold),
            label_color,
        );
        let count_galley = painter.layout_no_wrap(
            count.to_string(),
            theme::mono(tokens::FS_MICRO, FontWeight::Medium),
            count_color,
        );
        let total = label_galley.rect.width() + 5.0 + count_galley.rect.width();
        let start_x = cell.center().x - total / 2.0;
        let label_pos = egui::pos2(start_x, cell.center().y - label_galley.rect.height() / 2.0);
        let count_pos = egui::pos2(
            start_x + label_galley.rect.width() + 5.0,
            cell.center().y - count_galley.rect.height() / 2.0,
        );
        painter.galley(label_pos, label_galley, label_color);
        painter.galley(count_pos, count_galley, count_color);
        if selected {
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(cell.left(), cell.bottom() - 2.0), cell.max),
                0.0,
                t.color.accent,
            );
        }
        theme::paint_focus_ring(ui, &response, cell);
    }
    painter.hline(
        band.x_range(),
        band.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    tab
}

struct ResultRun {
    run_index: usize,
    dataset_id: DatasetId,
    label: String,
    success: bool,
    analysis_count: usize,
    analyses: Vec<ResultAnalysis>,
}

struct ResultDatasetRowResponses {
    selection: Response,
    overlay: Option<Response>,
}

fn result_dataset_row(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: &str,
    overlaid: bool,
) -> ResultDatasetRowResponses {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.row_h;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    let overlay_width = if selected { 0.0 } else { 30.0 };
    let selection_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2(rect.right() - overlay_width, rect.bottom()),
    );
    let selection = ui.interact(
        selection_rect,
        ui.id().with(("result-dataset-selection", label)),
        egui::Sense::click(),
    );
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    let overlay = (!selected).then(|| {
        let overlay_rect = egui::Rect::from_min_max(
            egui::pos2(selection_rect.right(), rect.top()),
            rect.right_bottom(),
        );
        let response = ui.interact(
            overlay_rect,
            ui.id().with(("result-dataset-overlay", label)),
            egui::Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                overlaid,
                format!("Overlay {label}"),
            )
        });
        let dot = egui::Rect::from_center_size(overlay_rect.center(), egui::vec2(10.0, 10.0));
        ui.painter().circle(
            dot.center(),
            4.5,
            if overlaid {
                t.color.accent
            } else {
                egui::Color32::TRANSPARENT
            },
            egui::Stroke::new(
                1.0,
                if response.hovered() || overlaid {
                    t.color.accent
                } else {
                    t.color.text_faint
                },
            ),
        );
        theme::paint_focus_ring(ui, &response, overlay_rect);
        response
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(if overlaid {
                format!("Remove {label} from the active result sheet")
            } else {
                format!("Overlay {label} on the active result sheet")
            })
    });
    if selected || selection.hovered() {
        ui.painter().rect_filled(
            selection_rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let caret_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        egui::vec2(9.0, 9.0),
    );
    if selected {
        WorkbenchIcon::ChevronDown.paint(ui.painter(), caret_rect, t.color.text_dim);
    } else {
        let stroke = egui::Stroke::new(1.0, t.color.text_faint);
        ui.painter().line_segment(
            [
                egui::pos2(caret_rect.left() + 2.5, caret_rect.top() + 1.5),
                caret_rect.center(),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                caret_rect.center(),
                egui::pos2(caret_rect.left() + 2.5, caret_rect.bottom() - 1.5),
            ],
            stroke,
        );
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 31.0, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_width = ui
        .painter()
        .layout_no_wrap(
            meta.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let label_left = rect.left() + 45.0;
    let label_right = selection_rect.right() - 14.0 - meta_width;
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    ui.painter().text(
        egui::pos2(selection_rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        meta,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &selection, selection_rect);
    ResultDatasetRowResponses { selection, overlay }
}

struct ResultAnalysis {
    analysis_index: usize,
    presentation_key: crate::workbench::documents::result_document::AnalysisPresentationKey,
    label: String,
    short_label: &'static str,
    /// Caption under the label: the domain this analysis swept. Repeating
    /// the analysis name here would state the head's own title twice.
    domain: String,
    /// Quantities the analysis retained, before the browser's filters. The
    /// head shows `shown / retained` whenever the two differ.
    total_signals: usize,
    success: bool,
    signals: Vec<ResultSignal>,
}

struct ResultSignal {
    waveform_index: usize,
    name: String,
    color: String,
    visible: bool,
    value: Option<String>,
    /// Typed metadata line: quantity kind and retained sample count.
    meta: String,
}

/// The caption under an analysis head: what it swept, over what interval.
///
/// The interval comes from the retained abscissa rather than the requested
/// setup, so a run that stopped early states the range it actually solved.
fn result_analysis_domain(
    analysis_type: crate::state::AnalysisType,
    first_waveform: Option<&crate::state::WaveformData>,
) -> String {
    let (sweep, unit, ..) = analysis_type.axis_info();
    if sweep.is_empty() {
        return "scalar solution".to_owned();
    }
    let sweep = sweep.to_lowercase();
    let span = first_waveform.and_then(|waveform| {
        let start = *waveform.x.first()?;
        let end = *waveform.x.last()?;
        (start.is_finite() && end.is_finite()).then(|| {
            format!(
                "{} … {}",
                sweep_endpoint(start, unit),
                sweep_endpoint(end, unit)
            )
        })
    });
    match span {
        Some(span) => format!("{sweep} · {span}"),
        None if unit.is_empty() => sweep,
        None => format!("{sweep} · {unit}"),
    }
}

/// One end of a swept interval. Zero stays bare: "0.00 s" spends three
/// characters restating that the sweep starts where sweeps start.
fn sweep_endpoint(value: f64, unit: &str) -> String {
    if value == 0.0 {
        "0".to_owned()
    } else {
        crate::ui::plot::fmt_si_significant(value, unit, 3)
    }
}

/// Thousands-grouped sample count. A six-figure retained length is read as a
/// magnitude, not parsed digit by digit.
fn grouped_count(value: usize) -> String {
    let digits = value.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// Human name for a retained quantity's kind, read from the same accessor
/// authority the browser's units come from. The metadata line states it once
/// per row so no row needs a repeated kind chip.
fn result_quantity_kind_label(name: &str, unit: &'static str) -> &'static str {
    if crate::workbench::documents::result_document::browser_signal_is_current(name) {
        return if unit == "°" {
            "Current phase"
        } else {
            "Current"
        };
    }
    match unit {
        "V" => "Voltage",
        "°" => "Phase",
        "W" => "Power",
        "dB" => "Magnitude",
        "A" => "Current",
        "V^2/Hz" => "Noise PSD",
        _ => "Quantity",
    }
}

/// The mockup's `.result-browser-manifest-row`: one retained analysis of a
/// dataset, as its kind glyph, code, and the domain it solved. It is an
/// inventory line rather than a tree row — the dataset above it is the
/// subject, and these state what that dataset contains.
fn result_browser_manifest_row(
    ui: &mut Ui,
    glyph: &str,
    code: &str,
    domain: &str,
    selected: bool,
    ok: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = responsive_result_control_height(RESULT_MANIFEST_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{code}, {domain}"),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    if selected {
        ui.painter().rect_filled(rect, 0.0, c.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
    }
    let glyph_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 28.0 + 8.0, rect.center().y),
        egui::vec2(16.0, 16.0),
    );
    let glyph_color = if ok { c.accent } else { c.warn };
    ui.painter().rect_filled(glyph_rect, 3.0, c.accent_dim);
    ui.painter().text(
        glyph_rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        glyph_color,
    );
    let code_end = ui
        .painter()
        .text(
            egui::pos2(glyph_rect.right() + 6.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            code,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            if selected { c.text } else { c.text_dim },
        )
        .right();
    // The domain caption keeps under 42% of the row, as the mockup bounds it.
    let domain_left = code_end + 6.0;
    let available = (rect.right() - 8.0 - domain_left).min(rect.width() * 0.42);
    if available > 24.0 {
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout(
                domain.to_owned(),
                theme::sans(tokens::FS_MICRO, FontWeight::Medium),
                c.text_faint,
                available,
            )
        });
        if galley.rows.len() == 1 {
            ui.painter().galley(
                egui::pos2(
                    rect.right() - 8.0 - galley.size().x,
                    rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                c.text_faint,
            );
        }
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// The mockup's `.result-browser-omission`: an inset card that accounts for
/// quantities a group does not list, so absence is always explained.
fn result_browser_omission(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = (ui.available_width() - 16.0).max(0.0);
    let galley = ui.fonts_mut(|fonts| {
        fonts.layout(
            text.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
            width - 30.0,
        )
    });
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), galley.size().y + 14.0 + 12.0),
        egui::Sense::hover(),
    );
    let card = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 8.0, rect.top() + 6.0),
        egui::pos2(rect.right() - 8.0, rect.bottom() - 6.0),
    );
    ui.painter().rect_filled(card, 5.0, c.bg_panel_2);
    ui.painter().rect_stroke(
        card,
        5.0,
        egui::Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Info.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(card.left() + 8.0 + 6.0, card.top() + 7.0 + 6.0),
            egui::vec2(12.0, 12.0),
        ),
        c.accent,
    );
    ui.painter().galley(
        egui::pos2(card.left() + 27.0, card.top() + 7.0),
        galley,
        c.text_dim,
    );
}

/// The mockup's `.result-browser-analysis-head`: a 31 px group head whose
/// kind glyph, name, domain caption, and quantity count identify the retained
/// analysis a quantity belongs to, so no row has to repeat any of it.
fn result_browser_analysis_head(
    ui: &mut Ui,
    glyph: &str,
    label: &str,
    domain: &str,
    count: usize,
    retained: usize,
    selected: bool,
    ok: bool,
    expanded: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = responsive_result_control_height(RESULT_ANALYSIS_HEAD_HEIGHT, t.metrics.ctl_h);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!(
                "{label}, {domain}, {count} quantities, {}",
                if expanded { "expanded" } else { "collapsed" }
            ),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    ui.painter().rect_filled(
        rect,
        0.0,
        if response.hovered() {
            c.bg_hover
        } else {
            c.bg_panel
        },
    );
    // `box-shadow: 0 1px var(--border)` — the head keeps its own hairline so
    // it reads as pinned chrome above the quantities it owns.
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    // The disclosure caret: a group with many quantities is closed by
    // default, so the panel opens as an index of the run's analyses.
    ui.painter().text(
        egui::pos2(rect.left() + 5.0 + 7.0, rect.center().y),
        egui::Align2::CENTER_CENTER,
        if expanded { "⌄" } else { "›" },
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );

    // 18x18 kind glyph in accent ink over an accent wash.
    let glyph_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 5.0 + 14.0 + 9.0, rect.center().y),
        egui::vec2(18.0, 18.0),
    );
    let glyph_color = if ok { c.accent } else { c.warn };
    ui.painter().rect_filled(glyph_rect, 4.0, c.accent_dim);
    ui.painter().rect_stroke(
        glyph_rect,
        4.0,
        egui::Stroke::new(1.0, glyph_color.gamma_multiply(0.55)),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        glyph_rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        glyph_color,
    );

    // `shown / retained` while a filter hides part of the group, so the head
    // never implies the analysis solved fewer quantities than it did.
    let count_text = if count == retained {
        count.to_string()
    } else {
        format!("{count} / {retained}")
    };
    let count_width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                count_text.clone(),
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                c.text_faint,
            )
            .size()
            .x
    });
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        &count_text,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        c.text_faint,
    );

    // Label and domain share one baseline; the domain elides first because the
    // label is the identity the head exists to state.
    let text_left = glyph_rect.right() + 5.0;
    let text_right = rect.right() - 8.0 - count_width - 8.0;
    let label_end = ui
        .painter()
        .text(
            egui::pos2(text_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            if selected { c.accent } else { c.text },
        )
        .right();
    let domain_left = label_end + 6.0;
    if domain_left < text_right {
        let available = text_right - domain_left;
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout(
                domain.to_owned(),
                theme::sans(tokens::FS_MICRO, FontWeight::Medium),
                c.text_faint,
                available,
            )
        });
        if galley.rows.len() == 1 {
            ui.painter().galley(
                egui::pos2(domain_left, rect.center().y - galley.size().y / 2.0),
                galley,
                c.text_faint,
            );
        }
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn expression_header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let header_height = responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), header_height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "EXPRESSIONS",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    let add_size = responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, t.metrics.ctl_h);
    let add_center_x = if add_size >= TOUCH_TARGET_HEIGHT {
        rect.right() - 10.0 - add_size * 0.5
    } else {
        rect.right() - 16.0
    };
    let add_rect = egui::Rect::from_center_size(
        egui::pos2(add_center_x, rect.center().y),
        egui::vec2(add_size, add_size),
    );
    let response = ui.interact(
        add_rect,
        ui.id().with("add-result-expression"),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Open waveform calculator",
        )
    });
    if response.hovered() {
        ui.painter()
            .rect_filled(add_rect, t.radius, t.color.bg_hover);
    }
    WorkbenchIcon::Add.paint(
        ui.painter(),
        egui::Rect::from_center_size(add_rect.center(), egui::vec2(14.0, 14.0)),
        t.color.text_dim,
    );
    theme::paint_focus_ring(ui, &response, add_rect);
    if response.on_hover_text("Open calculator").clicked() {
        super::super::commands::vocabulary::Command::WaveformCalculator.execute(app);
    }
}

struct SignalRowResponses {
    selection: egui::Response,
    /// Plot membership. On a quantity row this is the hover-revealed ±
    /// action; expression rows keep it on their swatch.
    visibility: egui::Response,
    /// The batch-selection checkbox, on quantity rows only.
    check: egui::Response,
    /// Hover-revealed star toggle; present only while the row shows it.
    favorite: Option<egui::Response>,
}

/// One retained quantity, in the mockup's two-register anatomy: a selection
/// checkbox column, the name with its persistent favorite flag and a
/// right-aligned preview value, and the typed metadata line beneath.
///
/// Each mark states exactly one fact. The checkbox is batch membership; the
/// name's ink is plot membership; the favorite flag is a saved preference.
/// The flag and the favorite *action* are deliberately different marks — the
/// flag states membership on an idle row, while the action cluster appears on
/// hover and takes the value column, so a resting list stays pure data
/// instead of carrying a column of controls down the panel.
fn result_quantity_row(
    ui: &mut Ui,
    id: egui::Id,
    name: &str,
    meta: &str,
    value: Option<&str>,
    color: egui::Color32,
    selected: bool,
    visible: bool,
    favorite: bool,
    checked: bool,
) -> SignalRowResponses {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_height = responsive_result_control_height(RESULT_QUANTITY_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let hovered = ui.rect_contains_pointer(rect);
    let visibility_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2((rect.left() + 26.0).min(rect.right()), rect.bottom()),
    );
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(visibility_rect.right(), rect.top()),
        rect.right_bottom(),
    );
    // The checkbox marks the row for a batch action; plotting is the row's
    // own affordance in the action cluster, as the mockup separates them.
    let check = ui.interact(visibility_rect, id.with("check"), egui::Sense::click());
    check.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            checked,
            format!("Select {name}"),
        )
    });
    let selection = ui.interact(selection_rect, id.with("selection"), egui::Sense::click());
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{name}, {meta}"),
        )
    });
    if !ui.is_rect_visible(rect) {
        return SignalRowResponses {
            selection,
            visibility: check.clone(),
            check,
            favorite: None,
        };
    }

    if selected {
        ui.painter().rect_filled(rect, 0.0, c.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
    } else if hovered {
        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
    }

    // The checkbox states one thing: whether this row is in the batch. It
    // used to double as the plot-membership light, tinted in the trace's
    // colour, which left an unselected-but-plotted row looking checked.
    // Plot membership moved to the name below.
    let box_rect = egui::Rect::from_center_size(visibility_rect.center(), egui::vec2(13.0, 13.0));
    if checked {
        ui.painter().rect_stroke(
            box_rect,
            3.0,
            egui::Stroke::new(1.0, c.accent),
            egui::StrokeKind::Inside,
        );
        ui.painter()
            .rect_filled(box_rect.shrink(3.0), 1.0, c.accent);
    } else {
        ui.painter().rect_filled(box_rect, 3.0, c.bg_panel);
        ui.painter().rect_stroke(
            box_rect,
            3.0,
            egui::Stroke::new(1.0, c.border_strong),
            egui::StrokeKind::Inside,
        );
    }

    // Two baselines inside the row: identity above, typed metadata below.
    //
    // The name carries the trace's own colour while the signal is plotted,
    // so the list answers "which curve is this" in the place the eye already
    // is — no swatch column, and no second meaning loaded onto the checkbox.
    // A signal that is not on a pane stays neutral.
    let name_y = rect.top() + 11.0;
    let meta_y = rect.top() + 25.0;
    let text_left = selection_rect.left();
    let name_end = ui
        .painter()
        .text(
            egui::pos2(text_left, name_y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            if visible { color } else { c.text_dim },
        )
        .right();
    if favorite {
        ui.painter().text(
            egui::pos2(name_end + 5.0, name_y),
            egui::Align2::LEFT_CENTER,
            "★",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.warn.gamma_multiply(0.72),
        );
    }

    // The action cluster and the preview value share the right column: on an
    // idle row the value owns it, on intent the actions take it.
    // The action cluster: plot membership, then the favorite mark.
    let plot_response = hovered.then(|| {
        let plot_rect = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 3.0 - 22.0 - 11.0, rect.center().y),
            egui::vec2(22.0, 24.0),
        );
        let response = ui.interact(plot_rect, id.with("plot"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                visible,
                format!("{name} plotted"),
            )
        });
        if response.hovered() {
            ui.painter().rect_filled(plot_rect, 3.0, c.bg_active);
        }
        ui.painter().text(
            plot_rect.center(),
            egui::Align2::CENTER_CENTER,
            if visible { "−" } else { "+" },
            theme::mono(tokens::FS_1, FontWeight::Medium),
            if visible { color } else { c.text_faint },
        );
        theme::paint_focus_ring(ui, &response, plot_rect);
        response.on_hover_text(if visible {
            format!("Remove {name} from its pane")
        } else {
            format!("Add {name} to its pane")
        })
    });
    let favorite_response = if hovered {
        let star_rect = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 3.0 - 11.0, rect.center().y),
            egui::vec2(22.0, 24.0),
        );
        let response = ui.interact(star_rect, id.with("favorite"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                favorite,
                format!("{name} favorite"),
            )
        });
        if response.hovered() {
            ui.painter().rect_filled(star_rect, 3.0, c.bg_active);
        }
        ui.painter().text(
            star_rect.center(),
            egui::Align2::CENTER_CENTER,
            if favorite { "★" } else { "☆" },
            theme::mono(tokens::FS_1, FontWeight::Regular),
            if favorite { c.warn } else { c.text_faint },
        );
        theme::paint_focus_ring(ui, &response, star_rect);
        Some(response.on_hover_text(if favorite {
            format!("Remove {name} from favorites")
        } else {
            format!("Add {name} to favorites")
        }))
    } else {
        if let Some(value) = value {
            ui.painter().text(
                egui::pos2(rect.right() - 8.0, name_y),
                egui::Align2::RIGHT_CENTER,
                value,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                if visible { c.text_dim } else { c.text_faint },
            );
        }
        None
    };

    ui.painter().text(
        egui::pos2(text_left, meta_y),
        egui::Align2::LEFT_CENTER,
        meta,
        theme::sans(tokens::FS_MICRO, FontWeight::Medium),
        c.text_faint,
    );

    theme::paint_focus_ring(ui, &check, visibility_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    let check = check.on_hover_text(if checked {
        format!("Deselect {name}")
    } else {
        format!("Select {name} for a batch action")
    });
    let selection = selection.on_hover_text(format!("Inspect {name}"));
    SignalRowResponses {
        selection,
        visibility: plot_response.unwrap_or_else(|| check.clone()),
        check,
        favorite: favorite_response,
    }
}

fn signal_row(
    ui: &mut Ui,
    id: egui::Id,
    name: &str,
    value: Option<&str>,
    color: egui::Color32,
    selected: bool,
    visible: bool,
) -> SignalRowResponses {
    let t = Tokens::get(ui.ctx());
    let row_height = responsive_result_control_height(SIGNAL_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let visibility_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 19.0, rect.center().y),
        egui::vec2(26.0, row_height),
    )
    .intersect(rect);
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(visibility_rect.right(), rect.top()),
        rect.right_bottom(),
    );
    let visibility = ui.interact(visibility_rect, id.with("visibility"), egui::Sense::click());
    visibility.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            visible,
            format!("{name} trace visibility"),
        )
    });
    let selection = ui.interact(selection_rect, id.with("selection"), egui::Sense::click());
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            name,
        )
    });
    if selected || selection.hovered() || visibility.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let swatch = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 19.0, rect.center().y),
        egui::vec2(12.0, 3.0),
    );
    ui.painter().rect_filled(
        swatch,
        1.0,
        if visible { color } else { t.color.text_faint },
    );
    ui.painter().text(
        egui::pos2(rect.left() + 31.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if visible {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if let Some(value) = value {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &visibility, visibility_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    let visibility = visibility.on_hover_text(if visible {
        format!("Hide {name}")
    } else {
        format!("Show {name}")
    });
    let selection = selection.on_hover_text(format!("Select {name}"));
    SignalRowResponses {
        // Expression rows carry no batch selection: the swatch is their only
        // control, so the check response mirrors it rather than inventing one.
        check: visibility.clone(),
        selection,
        visibility,
        favorite: None,
    }
}

/// Put every checked quantity of the active dataset into the requested plot
/// state. Toggling one at a time is the same act repeated, so the batch runs
/// through the same visibility path rather than writing the flag itself.
fn set_checked_signal_visibility(
    app: &mut RSpiceApp,
    checked: &std::collections::BTreeSet<String>,
    visible: bool,
) {
    let Some(run_index) = app.state.simulation.active_run_idx else {
        return;
    };
    let Some(run) = app.state.simulation.runs.get(run_index) else {
        return;
    };
    let targets: Vec<(usize, usize)> = run
        .analyses
        .iter()
        .enumerate()
        .flat_map(|(analysis_index, analysis)| {
            analysis
                .waveforms
                .iter()
                .enumerate()
                .filter(|(_, waveform)| {
                    checked.contains(&waveform.name) && waveform.visible != visible
                })
                .map(move |(waveform_index, _)| (analysis_index, waveform_index))
                .collect::<Vec<_>>()
        })
        .collect();
    for (analysis_index, waveform_index) in targets {
        crate::workbench::documents::result_document::toggle_visibility(
            &mut app.state,
            analysis_index,
            waveform_index,
        );
    }
}

fn select_result_dataset(app: &mut RSpiceApp, run_index: usize) -> bool {
    let Some(dataset_id) = app
        .state
        .simulation
        .runs
        .get(run_index)
        .map(|run| run.dataset_id)
    else {
        return false;
    };
    let document = super::super::state::WorkspaceDocumentId::ResultDataset(dataset_id);
    if !super::super::chrome::document_bar::activate_document_by_id(&mut app.state, &document) {
        return false;
    }
    app.state.ui.results.selected_trace = None;
    true
}

fn select_result_analysis(app: &mut RSpiceApp, run_index: usize, analysis_index: usize) -> bool {
    if app
        .state
        .simulation
        .runs
        .get(run_index)
        .and_then(|run| run.analyses.get(analysis_index))
        .is_none()
    {
        return false;
    }
    if !select_result_dataset(app, run_index) {
        return false;
    }
    if !app.state.simulation.select_analysis(analysis_index) {
        return false;
    }
    app.state.ui.results.selected_trace = None;
    true
}

fn select_result_signal(
    app: &mut RSpiceApp,
    run_index: usize,
    analysis_index: usize,
    waveform_index: usize,
) -> bool {
    let Some(run) = app.state.simulation.runs.get(run_index) else {
        return false;
    };
    let Some(selected) =
        crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
            run,
            analysis_index,
            waveform_index,
        )
    else {
        return false;
    };
    if !select_result_analysis(app, run_index, analysis_index) {
        return false;
    }
    // Document/run activation advances the canonical simulation data version.
    // Reconcile the viewer before installing the exact trace selection so the
    // next Results frame cannot clear the selection as if it belonged to the
    // previously active dataset.
    crate::workbench::documents::result_document::prepare_viewer_state(app);
    let name = selected.source_name().to_owned();
    app.state.ui.results.selected_trace = Some(selected);
    // Selecting a trace is a deliberate act; feed the browser's Recent scope.
    app.state.ui.results.note_recent_signal(&name);
    true
}

/// The other direction of the probe loop: from a trace back to the conductor
/// that produced it.
///
/// A derived signal explains itself rather than pretending to be a net, and
/// a result whose drawing has since changed says so instead of selecting
/// geometry that no longer means the same thing.
fn locate_on_schematic_menu(response: &egui::Response, app: &mut RSpiceApp, signal: &str) {
    let signal = signal.to_owned();
    response.context_menu(|ui| {
        if ui.button("Show on schematic").clicked() {
            match crate::schematic::view::select_signal_conductor(&mut app.state, &signal) {
                Ok(net) => {
                    Command::OpenWorkspace(Workspace::Design).execute(app);
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "Selected conductor {net} from {signal}."
                        )));
                }
                Err(error) => {
                    let message = error.message(&signal);
                    app.state.ui.toasts.warn_with_title(
                        ui.ctx(),
                        "Cannot cross-probe",
                        message.clone(),
                    );
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
                }
            }
            ui.close();
        }
    });
}

fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_query
        .trim()
        .to_ascii_lowercase();
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
    for page in VerificationPage::NAVIGATION {
        let flow = verification_flow_presentation(app, page);
        let label = &flow.label;
        let detail = &flow.detail;
        let status = &flow.status;
        if !query.is_empty()
            && !format!("{label} {detail} {status}")
                .to_ascii_lowercase()
                .contains(&query)
        {
            continue;
        }
        if flow_row(
            ui,
            &flow,
            app.state.workbench.verification_page == page,
            page.is_operational(),
        ) {
            app.state.workbench.verification_page = page;
        }
    }
    section_header(ui, "Active evidence coverage", None);
    let coverage = verification_coverage(app);
    let status_width = ui.available_width().max(1.0);
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_width((status_width - 20.0).max(1.0));
            let healthy = coverage.total > 0 && coverage.gaps == 0;
            let tone = if healthy {
                Tokens::get(ui.ctx()).color.ok
            } else {
                Tokens::get(ui.ctx()).color.warn
            };
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let (dot, _) = ui.allocate_exact_size(egui::vec2(5.0, 13.0), egui::Sense::hover());
                ui.painter().circle_filled(dot.center(), 2.5, tone);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&coverage.status)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(tone),
                    )
                    .wrap(),
                );
            });
        });
    property_row(
        ui,
        "Mapped specifications",
        &format!("{} / {}", coverage.mapped, coverage.total),
    );
    property_row(
        ui,
        "PVT points",
        &format!("{} retained", coverage.pvt_points),
    );
    property_row(
        ui,
        "Executable checks",
        &format!(
            "{} executed · {} passed",
            coverage.executed, coverage.passed
        ),
    );
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

#[derive(Clone)]
struct VerificationFlowPresentation {
    label: String,
    detail: String,
    status: String,
    mark: VerificationFlowMark,
    icon_tone: FlowTone,
    status_tone: FlowTone,
}

#[derive(Clone, Copy)]
enum VerificationFlowMark {
    Status(StatusMark),
    Text(&'static str),
}

const fn verification_flow_label(page: VerificationPage) -> &'static str {
    match page {
        VerificationPage::Yield => "PVT & Monte Carlo",
        VerificationPage::Corners => "Process corners",
        VerificationPage::Tuning => "Parameter tuning sandbox",
        VerificationPage::Optimization => "Optimization",
        VerificationPage::Reliability => "Electrical reliability & SOA",
        VerificationPage::Regression => "Regression · main",
        VerificationPage::Drc => "Physical DRC",
    }
}

fn verification_flow_presentation(
    app: &RSpiceApp,
    page: VerificationPage,
) -> VerificationFlowPresentation {
    let active_run = app.state.simulation.active_run();
    match page {
        VerificationPage::Yield => {
            let evidence = active_run.and_then(|run| {
                app.state
                    .simulation
                    .yield_provenance
                    .filter(|provenance| {
                        provenance.source_run_id == run.run_id
                            && provenance.source_dataset_id == run.dataset_id
                    })
                    .map(|provenance| (run, provenance))
            });
            let results = evidence
                .and_then(|(run, _)| {
                    app.state
                        .simulation
                        .yield_results_for_dataset(run.dataset_id)
                })
                .unwrap_or(&[]);
            let worst = results.iter().min_by(|left, right| {
                left.yield_percent
                    .partial_cmp(&right.yield_percent)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            VerificationFlowPresentation {
                label: evidence.map_or_else(
                    || verification_flow_label(page).to_owned(),
                    |(run, _)| format!("PVT & Monte Carlo · Run {}", run.id),
                ),
                detail: evidence.map_or_else(
                    || "No retained Monte Carlo evidence for the active dataset".to_owned(),
                    |(_, provenance)| {
                        format!(
                            "{} / {} samples · seed {:#x}",
                            provenance.runs_completed, provenance.runs_requested, provenance.seed
                        )
                    },
                ),
                status: worst.map_or_else(
                    || "not run".to_owned(),
                    |result| format!("{:.2}% retained yield", result.yield_percent),
                ),
                mark: VerificationFlowMark::Status(
                    if worst.is_some_and(|result| result.fail_count == 0) {
                        StatusMark::Success
                    } else if worst.is_some() {
                        StatusMark::Warning
                    } else {
                        StatusMark::Neutral
                    },
                ),
                icon_tone: if worst.is_some_and(|result| result.fail_count == 0) {
                    FlowTone::Ok
                } else if worst.is_some() {
                    FlowTone::Warn
                } else {
                    FlowTone::Neutral
                },
                status_tone: if worst.is_some_and(|result| result.fail_count == 0) {
                    FlowTone::Ok
                } else if worst.is_some() {
                    FlowTone::Warn
                } else {
                    FlowTone::Neutral
                },
            }
        }
        VerificationPage::Corners => {
            let result = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Corner));
            let point_count = result
                .and_then(|analysis| analysis.waveforms.first())
                .map_or(0, |waveform| waveform.x.len());
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: if point_count == 0 {
                    "No retained process-corner evidence".to_owned()
                } else {
                    format!("{point_count} retained corner points")
                },
                status: if point_count == 0 {
                    "not run".to_owned()
                } else if result.is_some_and(|analysis| analysis.success) {
                    format!("{point_count} complete")
                } else {
                    "failed / incomplete".to_owned()
                },
                mark: VerificationFlowMark::Status(
                    if result.is_some_and(|analysis| analysis.success) {
                        StatusMark::Success
                    } else if result.is_some() {
                        StatusMark::Warning
                    } else {
                        StatusMark::Neutral
                    },
                ),
                icon_tone: if result.is_some_and(|analysis| analysis.success) {
                    FlowTone::Ok
                } else if result.is_some() {
                    FlowTone::Warn
                } else {
                    FlowTone::Neutral
                },
                status_tone: if result.is_some_and(|analysis| analysis.success) {
                    FlowTone::Ok
                } else if result.is_some() {
                    FlowTone::Warn
                } else {
                    FlowTone::Neutral
                },
            }
        }
        VerificationPage::Tuning => {
            let variable_count = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| app.state.workspace.active_plan_data(plan.id()))
                .map_or(0, |payload| payload.design_variables.len());
            let dirty_count = app
                .state
                .workbench
                .verification
                .tuning_variables
                .iter()
                .filter(|draft| draft.is_dirty())
                .count();
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: if variable_count == 0 {
                    "No active-plan design variables".to_owned()
                } else {
                    format!("{variable_count} typed active-plan variable(s)")
                },
                status: if dirty_count == 0 {
                    "baseline".to_owned()
                } else {
                    format!("{dirty_count} provisional")
                },
                mark: VerificationFlowMark::Text("T"),
                icon_tone: if dirty_count == 0 {
                    FlowTone::Accent
                } else {
                    FlowTone::Warn
                },
                status_tone: if dirty_count == 0 {
                    FlowTone::Accent
                } else {
                    FlowTone::Warn
                },
            }
        }
        VerificationPage::Optimization => {
            let result = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Optimization));
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: if result.is_some() {
                    "Source-attributed optimization result retained".to_owned()
                } else {
                    "Bounded production optimization analysis".to_owned()
                },
                status: if result.is_some() {
                    "retained result".to_owned()
                } else {
                    "not run".to_owned()
                },
                mark: VerificationFlowMark::Text("O"),
                icon_tone: if result.is_some() {
                    FlowTone::Accent
                } else {
                    FlowTone::Neutral
                },
                status_tone: if result.is_some() {
                    FlowTone::Accent
                } else {
                    FlowTone::Neutral
                },
            }
        }
        VerificationPage::Reliability => {
            let soa_evidence =
                active_run.and_then(|run| verified_analysis(run, crate::state::AnalysisType::Soa));
            let aging_evidence = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Reliability));
            let has_evidence = soa_evidence.is_some() || aging_evidence.is_some();
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: if has_evidence {
                    "Execution receipt retained · dataset-owned payload unavailable".to_owned()
                } else {
                    "No source-attributed reliability or SOA evidence".to_owned()
                },
                status: if has_evidence {
                    "verdict unavailable".to_owned()
                } else {
                    "not run".to_owned()
                },
                mark: VerificationFlowMark::Status(if has_evidence {
                    StatusMark::Warning
                } else {
                    StatusMark::Neutral
                }),
                icon_tone: if has_evidence {
                    FlowTone::Warn
                } else {
                    FlowTone::Neutral
                },
                status_tone: if has_evidence {
                    FlowTone::Warn
                } else {
                    FlowTone::Neutral
                },
            }
        }
        VerificationPage::Regression => {
            let retained_runs = app
                .state
                .simulation
                .runs
                .iter()
                .filter(|run| {
                    run.analyses
                        .iter()
                        .any(|analysis| analysis.success && analysis.provenance.is_some())
                })
                .count();
            let ready = retained_runs >= 2;
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: "Measurements and waveforms vs governed baseline".to_owned(),
                status: if ready {
                    format!("{retained_runs} source-attributed runs")
                } else {
                    "baseline unavailable".to_owned()
                },
                mark: VerificationFlowMark::Status(if ready {
                    StatusMark::Success
                } else {
                    StatusMark::Neutral
                }),
                icon_tone: if ready {
                    FlowTone::Accent
                } else {
                    FlowTone::Neutral
                },
                status_tone: if ready {
                    FlowTone::Accent
                } else {
                    FlowTone::Neutral
                },
            }
        }
        VerificationPage::Drc => VerificationFlowPresentation {
            label: verification_flow_label(page).to_owned(),
            detail: "Unavailable until layout, rule-deck, and marker evidence are retained"
                .to_owned(),
            status: "not selectable".to_owned(),
            mark: VerificationFlowMark::Status(StatusMark::Neutral),
            icon_tone: FlowTone::Neutral,
            status_tone: FlowTone::Error,
        },
    }
}

fn verified_analysis(
    run: &crate::state::SimulationRun,
    analysis_type: crate::state::AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    run.analyses.iter().rev().find(|analysis| {
        analysis.analysis_type == analysis_type && analysis.success && analysis.provenance.is_some()
    })
}

struct VerificationCoverage {
    total: usize,
    mapped: usize,
    executed: usize,
    passed: usize,
    pvt_points: usize,
    gaps: usize,
    status: String,
}

fn verification_coverage(app: &RSpiceApp) -> VerificationCoverage {
    let run = app.state.simulation.active_run();
    let total = app.state.workspace.specs.len();
    let values = app
        .state
        .workspace
        .specs
        .iter()
        .map(|spec| {
            run.and_then(|run| {
                run.analyses.iter().find_map(|analysis| {
                    if !analysis.success || analysis.provenance.is_none() {
                        return None;
                    }
                    analysis.measurements.iter().find_map(|measurement| {
                        if measurement.name.eq_ignore_ascii_case(&spec.measurement) {
                            measurement.value.filter(|value| value.is_finite())
                        } else {
                            None
                        }
                    })
                })
            })
        })
        .collect::<Vec<_>>();
    let mapped = app
        .state
        .workspace
        .specs
        .iter()
        .filter(|spec| !spec.measurement.trim().is_empty())
        .count();
    let executed = values.iter().filter(|value| value.is_some()).count();
    let passed = app
        .state
        .workspace
        .specs
        .iter()
        .zip(&values)
        .filter(|(spec, value)| value.is_some_and(|value| spec.passes(value)))
        .count();
    let pvt_points = run
        .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Corner))
        .and_then(|analysis| analysis.waveforms.first())
        .map_or(0, |waveform| waveform.x.len());
    let gaps = total.saturating_sub(executed);
    VerificationCoverage {
        total,
        mapped,
        executed,
        passed,
        pvt_points,
        gaps,
        status: if total == 0 {
            "No project specifications configured".to_owned()
        } else if gaps == 0 {
            "Coverage current for active dataset".to_owned()
        } else {
            format!("{gaps} evidence gaps · review required")
        },
    }
}

fn verification_navigator_requires_scroll(available_height: f32) -> bool {
    const HISTORY_COVERAGE_HEIGHT: f32 = 126.0;
    let flow_height = VerificationPage::NAVIGATION.len() as f32 * FLOW_ROW_HEIGHT;
    available_height < flow_height + HISTORY_COVERAGE_HEIGHT
}

#[cfg(test)]
fn active_mc_sample_trail(simulation: &crate::state::SimulationState) -> usize {
    simulation
        .yield_results_for_active_dataset()
        .unwrap_or(&[])
        .iter()
        .map(|result| result.trail.len())
        .max()
        .unwrap_or(0)
}

#[derive(Clone, Copy)]
enum FlowTone {
    Neutral,
    Accent,
    Ok,
    Warn,
    Error,
}

fn flow_row_geometry(detail_lines: usize) -> (f32, f32) {
    let extra_detail_height = detail_lines.saturating_sub(1) as f32 * FLOW_DETAIL_LINE_HEIGHT;
    (
        FLOW_ROW_HEIGHT + extra_detail_height,
        FLOW_STATUS_TOP + extra_detail_height,
    )
}

fn flow_row(
    ui: &mut Ui,
    flow: &VerificationFlowPresentation,
    selected: bool,
    enabled: bool,
) -> bool {
    let VerificationFlowPresentation {
        label,
        detail,
        status,
        mark,
        icon_tone,
        status_tone,
    } = flow;
    let t = Tokens::get(ui.ctx());
    let text_width = (ui.available_width() - FLOW_TEXT_LEFT - 9.0).max(1.0);
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let (row_height, status_top) = flow_row_geometry(detail_galley.rows.len().max(1));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            enabled,
            selected,
            format!("{label}. {detail}. {status}"),
        )
    });
    if selected || (enabled && response.hovered()) {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(rect.min, egui::pos2(rect.left() + 2.0, rect.bottom())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let icon_ink = match *icon_tone {
        FlowTone::Neutral => t.color.text_faint,
        FlowTone::Accent => t.color.accent,
        FlowTone::Ok => t.color.ok,
        FlowTone::Warn => t.color.warn,
        FlowTone::Error => t.color.err,
    };
    let (status_dot, status_ink) = match *status_tone {
        FlowTone::Neutral => (t.color.text_faint, t.color.text_dim),
        FlowTone::Accent => (t.color.accent, t.color.text_dim),
        FlowTone::Ok => (t.color.ok, t.color.text_dim),
        FlowTone::Warn => (t.color.warn, t.color.warn),
        FlowTone::Error => (t.color.err, t.color.err),
    };
    let status_circle = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 17.5, rect.top() + 16.5),
        egui::vec2(17.0, 17.0),
    );
    let circle_fill = match *icon_tone {
        FlowTone::Ok => t.color.ok.gamma_multiply(0.14),
        FlowTone::Warn => t.color.warn.gamma_multiply(0.14),
        FlowTone::Accent => t.color.accent.gamma_multiply(0.14),
        FlowTone::Neutral => t.color.bg_panel_2,
        FlowTone::Error => t.color.err.gamma_multiply(0.14),
    };
    let circle_border = match *icon_tone {
        FlowTone::Ok | FlowTone::Warn | FlowTone::Accent | FlowTone::Error => {
            icon_ink.gamma_multiply(0.7)
        }
        FlowTone::Neutral => icon_ink,
    };
    ui.painter()
        .circle_filled(status_circle.center(), 8.5, circle_fill);
    ui.painter().circle_stroke(
        status_circle.center(),
        8.0,
        egui::Stroke::new(1.0, circle_border),
    );
    match mark {
        VerificationFlowMark::Status(mark) => paint_status_mark(
            ui.painter(),
            egui::Rect::from_center_size(status_circle.center(), egui::Vec2::splat(8.0)),
            *mark,
            icon_ink,
        ),
        VerificationFlowMark::Text(text) => {
            ui.painter().text(
                status_circle.center(),
                egui::Align2::CENTER_CENTER,
                text,
                theme::sans(tokens::FS_0, FontWeight::Medium),
                icon_ink,
            );
        }
    }
    let text_left = rect.left() + FLOW_TEXT_LEFT;
    let text_right = rect.right() - 9.0;
    let clip = egui::Rect::from_x_y_ranges(text_left..=text_right, rect.y_range());
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_left, rect.top() + FLOW_LABEL_TOP),
        egui::Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    painter.galley(
        egui::pos2(text_left, rect.top() + FLOW_DETAIL_TOP),
        detail_galley,
        t.color.text_faint,
    );
    let status_dot_rect = egui::Rect::from_min_size(
        egui::pos2(text_left, rect.top() + status_top + 4.0),
        egui::Vec2::splat(FLOW_STATUS_DOT_SIZE),
    );
    painter.circle_filled(
        status_dot_rect.center(),
        FLOW_STATUS_DOT_SIZE * 0.5,
        status_dot,
    );
    painter.text(
        egui::pos2(
            text_left + FLOW_STATUS_DOT_SIZE + FLOW_STATUS_GAP,
            rect.top() + status_top,
        ),
        egui::Align2::LEFT_TOP,
        status,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        status_ink,
    );
    theme::paint_focus_ring(ui, &response, rect);
    let clicked = enabled && response.clicked();
    if !enabled {
        response.on_hover_text(
            "Unavailable: this flow has no qualified execution and retained-evidence pipeline.",
        );
    }
    clicked
}

fn models(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Model ownership", None);
    for page in ModelsPage::ALL {
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            page.label(),
            app.state.workbench.models_page == page,
            None,
        ) {
            app.state.workbench.models_page = page;
        }
    }
    section_header(
        ui,
        "Loaded model libraries",
        Some(&app.state.model_library_manager.library_count().to_string()),
    );
    let libraries: Vec<_> = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| (library.name.clone(), library.model_count()))
        .collect();
    for (name, count) in libraries {
        let selected = app.state.model_library_manager.selected_library.as_deref() == Some(&name);
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            &name,
            selected,
            Some(&count.to_string()),
        ) {
            app.state.model_library_manager.select_library(&name);
            app.state.workbench.models_page = ModelsPage::Models;
            app.state.workbench.selected_model = None;
        }
    }
}

fn nav_row_indented_styled(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        t.metrics.row_h,
        tokens::FS_0,
        tokens::FS_0,
        false,
        false,
        egui::Sense::click(),
    )
}

fn nav_row_indented_styled_with_metrics(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    height: f32,
    label_size: f32,
    meta_size: f32,
    expanded: bool,
    child_guide: bool,
    sense: egui::Sense,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    if selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let schematic_metrics = (height - SCHEMATIC_NAV_ROW_HEIGHT).abs() <= f32::EPSILON
        && (label_size - SCHEMATIC_NAV_LABEL_SIZE).abs() <= f32::EPSILON;
    let child_offset = if child_guide { 19.0 } else { 0.0 };
    if child_guide {
        ui.painter().vline(
            rect.left() + 19.0,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    // `.nav-children` owns its 19 px hierarchy offset; its rows reset their
    // own padding to the root-row contract instead of accumulating levels.
    let indent = if child_guide {
        0.0
    } else {
        14.0 * level as f32
    };
    if expanded {
        WorkbenchIcon::ChevronDown.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.left()
                        + child_offset
                        + if schematic_metrics { 14.0 } else { 12.0 }
                        + indent,
                    rect.center().y,
                ),
                egui::vec2(9.0, 9.0),
            ),
            t.color.text_faint,
        );
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(
                rect.left() + child_offset + if schematic_metrics { 33.5 } else { 31.0 } + indent,
                rect.center().y,
            ),
            egui::vec2(15.0, 15.0),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_width = meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.to_owned(),
                theme::mono(meta_size, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
    });
    let label_left =
        rect.left() + child_offset + if schematic_metrics { 47.0 } else { 45.0 } + indent;
    let label_right = if meta.is_some() {
        rect.right() - 14.0 - meta_width
    } else {
        rect.right() - 8.0
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            if mono {
                theme::mono(label_size, FontWeight::Regular)
            } else {
                theme::sans(label_size, FontWeight::Regular)
            },
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    if let Some(meta) = meta {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            theme::mono(meta_size, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: EMPTY_HINT_PADDING_X,
            right: EMPTY_HINT_PADDING_X,
            top: EMPTY_HINT_PADDING_Y,
            bottom: EMPTY_HINT_PADDING_Y,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap()
                .halign(Align::Center),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::{
        CAPABILITY_BANNER_GAP, CAPABILITY_BANNER_ICON_SIZE, CAPABILITY_BANNER_MARGIN,
        EMPTY_HINT_PADDING_X, EMPTY_HINT_PADDING_Y, EXPRESSION_HEADER_HEIGHT, FLOW_DETAIL_TOP,
        FLOW_LABEL_TOP, FLOW_ROW_HEIGHT, FLOW_STATUS_TOP, FLOW_TEXT_LEFT, NAV_PROPERTY_PADDING_X,
        NETLIST_OUTLINE_ICON_GAP, NETLIST_OUTLINE_PADDING_X, NETLIST_OUTLINE_ROW_HEIGHT,
        NETLIST_OUTLINE_TOUCH_ROW_HEIGHT, NetlistNavigatorProjection, NetlistNavigatorRowKind,
        PANEL_SEARCH_MARGIN_X, SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT, active_mc_sample_trail,
        flow_row_geometry, header, panel_search, panel_search_field_width,
        responsive_result_control_height, select_result_analysis, select_result_dataset,
        select_result_signal, verification_coverage, verification_flow_label,
        verification_navigator_requires_scroll,
    };
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::services::yield_manager::{
        DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
    };
    use crate::state::{
        AnalysisResult, AnalysisType, SimulationRun, SimulationState, WaveformData,
    };
    use crate::workbench::RSpiceApp;
    use crate::workbench::state::{VerificationPage, Workspace};

    fn result(trail: Vec<bool>) -> YieldResult {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        YieldResult {
            spec: YieldSpec::lower("gain", 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: DistributionStats::default(),
            samples: vec![1.0; trail.len()],
            trail,
        }
    }

    #[test]
    fn navigator_search_and_empty_hint_match_mockup_spacing() {
        assert_eq!(PANEL_SEARCH_MARGIN_X, 8.0);
        assert_eq!(panel_search_field_width(260.0), 244.0);
        assert_eq!(NAV_PROPERTY_PADDING_X, 10.0);
        assert_eq!(EMPTY_HINT_PADDING_X, 12);
        assert_eq!(EMPTY_HINT_PADDING_Y, 20);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn navigator_heading_and_search_expose_explicit_accesskit_names() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        let mut query = String::new();
        let mut focus_pending = false;

        let nodes = ctx
            .run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(260.0);
                    header(ui, &mut app);
                    panel_search(
                        ui,
                        &mut query,
                        "navigator-accessibility-test",
                        "Find instance, net or port",
                        &mut focus_pending,
                    );
                });
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Heading
                && node.label() == Some("Design navigator")
                && node.level() == Some(2)
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::TextInput
                && node.label() == Some("Find instance, net or port")
        }));
    }

    /// The mockup's data browser reads top to bottom as: tab band, then the
    /// query and facets that filter the tab it selected, then the scope and
    /// count band. A search above the tabs would read as filtering all three
    /// tabs at once, which is what this pins against.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_data_browser_puts_its_query_under_the_tab_it_filters() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Results);

        let mut search_rect: Option<egui::Rect> = None;
        let mut facet_rects: Vec<egui::Rect> = Vec::new();
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(255.0);
                super::show(ui, &mut app);
            });
        });
        for (_, node) in output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("AccessKit tree update")
            .nodes
            .iter()
        {
            if node.role() == egui::accesskit::Role::TextInput
                && node.label() == Some("Find canonical name, path, unit, or type…")
                && let Some(bounds) = node.bounds()
            {
                search_rect = Some(egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                ));
            }
            if node.role() == egui::accesskit::Role::ComboBox
                && matches!(node.label(), Some("Quantity kind" | "Quantity sort"))
                && let Some(bounds) = node.bounds()
            {
                facet_rects.push(egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                ));
            }
        }

        let search = search_rect.expect("the browser owns a query field with its own placeholder");
        // The tab band is a painted strip 30 px tall directly under the panel
        // header, so anything below it starts lower than that.
        assert!(
            search.top() > 30.0,
            "the query must sit below the tab band, not above it ({})",
            search.top()
        );
        assert_eq!(facet_rects.len(), 2, "kind and sort facets");
        facet_rects.sort_by(|a, b| a.left().total_cmp(&b.left()));
        let [kind, sort] = [facet_rects[0], facet_rects[1]];
        assert!(
            (kind.width() - sort.width()).abs() <= 1.0,
            "the facets split the row evenly ({} vs {})",
            kind.width(),
            sort.width()
        );
        assert!(
            (kind.top() - sort.top()).abs() <= 1.0,
            "the facets share one row"
        );
        assert!(
            kind.top() > search.bottom(),
            "the facets sit under the query, as one toolbar block"
        );
        // The whole toolbar shares one inset, so the pair reads as a control
        // row belonging to the field above it at any dock width.
        assert!(
            (kind.left() - search.left()).abs() <= 1.0,
            "the facet row starts on the query's inset ({} vs {})",
            kind.left(),
            search.left()
        );
        assert!(
            (sort.right() - search.right()).abs() <= 1.5,
            "the facet row ends on the query's inset ({} vs {})",
            sort.right(),
            search.right()
        );
    }

    #[test]
    fn verification_flow_rows_match_the_mock_grid_without_text_collisions() {
        assert_eq!(FLOW_ROW_HEIGHT, 63.0);
        assert_eq!(FLOW_TEXT_LEFT, 35.0);
        assert_eq!(FLOW_LABEL_TOP, 7.0);
        assert_eq!(FLOW_DETAIL_TOP, 23.0);
        assert_eq!(FLOW_STATUS_TOP, 43.0);
        assert!(FLOW_DETAIL_TOP - (FLOW_LABEL_TOP + 11.0) >= 4.0);
        assert!(FLOW_STATUS_TOP - (FLOW_DETAIL_TOP + 11.0) >= 4.0);
        assert!(FLOW_ROW_HEIGHT - (FLOW_STATUS_TOP + 11.0) >= 7.0);
        assert_eq!(flow_row_geometry(1), (63.0, 43.0));
        assert_eq!(flow_row_geometry(2), (78.0, 58.0));
        assert_eq!(flow_row_geometry(3), (93.0, 73.0));
    }

    #[test]
    fn verification_navigation_exposes_the_operational_tuning_route() {
        assert_eq!(VerificationPage::NAVIGATION.len(), 6);
        let labels = VerificationPage::NAVIGATION.map(verification_flow_label);
        assert_eq!(
            labels,
            [
                "PVT & Monte Carlo",
                "Process corners",
                "Parameter tuning sandbox",
                "Optimization",
                "Electrical reliability & SOA",
                "Regression · main",
            ]
        );
        assert!(!VerificationPage::NAVIGATION.contains(&VerificationPage::Drc));
        assert!(!VerificationPage::Drc.is_operational());
    }

    #[test]
    fn specification_mapping_does_not_claim_execution_without_an_active_dataset() {
        let mut app = RSpiceApp::test_instance();
        app.state.workspace.specs.push(crate::state::SpecEntry {
            measurement: "gain".to_owned(),
            expression: String::new(),
            min: Some(1.0),
            max: None,
            unit: "V/V".to_owned(),
        });
        app.state.simulation.active_run_idx = None;

        let coverage = verification_coverage(&app);

        assert_eq!(coverage.mapped, 1);
        assert_eq!(coverage.executed, 0);
        assert_eq!(coverage.passed, 0);
        assert_eq!(coverage.gaps, 1);
        assert_ne!(coverage.status, "Coverage current for active dataset");
    }

    #[test]
    fn empty_specification_set_is_neutral_not_healthy() {
        let app = RSpiceApp::test_instance();
        let coverage = verification_coverage(&app);

        assert_eq!(coverage.total, 0);
        assert_eq!(coverage.gaps, 0);
        assert_eq!(coverage.status, "No project specifications configured");
        assert_ne!(coverage.status, "Coverage current for active dataset");
    }

    #[test]
    fn finite_goal_miss_counts_as_executed_but_not_passed() {
        let mut app = RSpiceApp::test_instance();
        app.state.workspace.specs.push(crate::state::SpecEntry {
            measurement: "gain".to_owned(),
            expression: String::new(),
            min: Some(41.0),
            max: None,
            unit: "V/V".to_owned(),
        });
        let mut measurement = rspice_core::MeasureResult::success("gain", 40.0);
        measurement.passed = false;
        measurement.error = Some("value misses GOAL".to_owned());
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(vec![measurement])
            .with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x7b; 32]),
                    Vec::new(),
                )
                .expect("test provenance is valid"),
            );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.runs = vec![run];
        app.state.simulation.active_run_idx = Some(0);

        let coverage = verification_coverage(&app);
        assert_eq!(coverage.executed, 1);
        assert_eq!(coverage.passed, 0);
        assert_eq!(coverage.gaps, 0);
    }

    #[test]
    fn verification_navigator_scrolls_when_flows_exceed_compact_height() {
        assert!(verification_navigator_requires_scroll(503.0));
        assert!(!verification_navigator_requires_scroll(504.0));
        assert!(verification_navigator_requires_scroll(390.0));
        assert!(!verification_navigator_requires_scroll(560.0));
        assert!(!verification_navigator_requires_scroll(700.0));
    }

    #[test]
    fn capability_policy_banner_matches_mock_spacing() {
        assert_eq!(CAPABILITY_BANNER_MARGIN, 8);
        assert_eq!(CAPABILITY_BANNER_ICON_SIZE, 15.0);
        assert_eq!(CAPABILITY_BANNER_GAP, 7.0);
    }

    #[test]
    fn netlist_navigator_projects_exact_live_counts_and_include_lines() {
        let source = "Precision amplifier\n.include models/base.lib\n.lib corners/process.lib TT\n.param gain=10 offset=1m\nR1 in out 1k\nXAMP in out opamp\n.model nch nmos\n.ac dec 10 1 1g\n.meas ac peak max v(out)\n.end\n";
        let projection = NetlistNavigatorProjection::from_source(source, "", "top.sp", true);

        assert_eq!(projection.line_count, 10);
        let count = |kind| {
            projection
                .structure_rows
                .iter()
                .find(|row| row.kind == kind)
                .and_then(|row| row.meta.as_deref())
        };
        assert_eq!(count(NetlistNavigatorRowKind::Root), Some("root"));
        assert_eq!(count(NetlistNavigatorRowKind::Parameters), Some("1"));
        assert_eq!(count(NetlistNavigatorRowKind::Instances), Some("2"));
        let instances = projection
            .structure_rows
            .iter()
            .find(|row| row.kind == NetlistNavigatorRowKind::Instances)
            .expect("instances row exists");
        assert!(instances.contains_line(5));
        assert!(instances.contains_line(6));
        assert_eq!(count(NetlistNavigatorRowKind::Models), Some("1"));
        assert_eq!(count(NetlistNavigatorRowKind::Analyses), Some("1"));
        assert_eq!(count(NetlistNavigatorRowKind::Measurements), Some("1"));
        assert_eq!(projection.include_rows.len(), 2);
        assert_eq!(projection.include_rows[0].label, "models/base.lib");
        assert_eq!(projection.include_rows[0].target_line, Some(2));
        assert_eq!(projection.include_rows[1].label, "corners/process.lib");
        assert_eq!(projection.include_rows[1].target_line, Some(3));
        assert!(projection.show_source_mapping);
    }

    #[test]
    fn netlist_navigator_filter_matches_symbols_and_exact_source_lines() {
        let source = "deck\n.param gain=10\nR1 in out 1k\nR2 out 0 2k\n.end\n";

        let symbol = NetlistNavigatorProjection::from_source(source, "r2", "top.sp", true);
        assert_eq!(symbol.structure_rows.len(), 1);
        assert_eq!(
            symbol.structure_rows[0].kind,
            NetlistNavigatorRowKind::Instances
        );
        assert_eq!(symbol.structure_rows[0].meta.as_deref(), Some("2"));
        assert_eq!(symbol.structure_rows[0].target_line, Some(4));
        assert!(!symbol.structure_rows[0].contains_line(3));
        assert!(symbol.structure_rows[0].contains_line(4));
        assert!(!symbol.show_source_mapping);

        let line = NetlistNavigatorProjection::from_source(source, "line 2", "top.sp", true);
        assert_eq!(line.structure_rows.len(), 1);
        assert_eq!(
            line.structure_rows[0].kind,
            NetlistNavigatorRowKind::Parameters
        );
        assert_eq!(line.structure_rows[0].target_line, Some(2));
    }

    #[test]
    fn netlist_navigator_geometry_matches_mockup_and_touch_contract() {
        assert_eq!(NETLIST_OUTLINE_ROW_HEIGHT, 27.0);
        assert_eq!(NETLIST_OUTLINE_TOUCH_ROW_HEIGHT, 44.0);
        assert_eq!(NETLIST_OUTLINE_PADDING_X, 9.0);
        assert_eq!(NETLIST_OUTLINE_ICON_GAP, 7.0);
    }

    #[test]
    fn mc_sample_trail_is_visible_only_for_its_active_provenance_dataset() {
        let source = SimulationRun::new(1);
        let source_run_id = source.run_id;
        let source_dataset_id = source.dataset_id;
        let other = SimulationRun::new(2);
        let mut simulation = SimulationState {
            runs: vec![source, other],
            active_run_idx: Some(0),
            ..SimulationState::default()
        };
        simulation.replace_yield_evidence(
            vec![result(vec![true, false, true])],
            Some(YieldAnalysisProvenance {
                source_run_id,
                source_dataset_id,
                seed: 7,
                runs_requested: 3,
                runs_completed: 3,
                sampling_mode: MonteCarloSamplingMode::PseudoRandom,
            }),
        );

        assert_eq!(active_mc_sample_trail(&simulation), 3);
        simulation.active_run_idx = Some(1);
        assert_eq!(active_mc_sample_trail(&simulation), 0);
        simulation.active_run_idx = None;
        assert_eq!(active_mc_sample_trail(&simulation), 0);
    }

    #[test]
    fn result_navigator_preserves_desktop_density_and_expands_touch_controls() {
        assert_eq!(
            responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, 32.0),
            EXPRESSION_HEADER_HEIGHT
        );
        assert_eq!(
            responsive_result_control_height(SIGNAL_ROW_HEIGHT, 32.0),
            SIGNAL_ROW_HEIGHT
        );
        assert_eq!(
            responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, TOUCH_TARGET_HEIGHT),
            TOUCH_TARGET_HEIGHT
        );
        assert_eq!(
            responsive_result_control_height(SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT),
            TOUCH_TARGET_HEIGHT
        );
    }

    fn result_navigator_app() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Results;
        let transient =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
            ]);
        let ac =
            AnalysisResult::new(2, AnalysisType::Ac, "AC").with_waveforms(vec![WaveformData::new(
                "V(in)",
                vec![1.0, 10.0],
                vec![1.0, 0.5],
                "#55aaff",
            )]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(transient);
        run.add_analysis(ac);
        app.state.simulation.runs = vec![run];
        app.state.simulation.active_run_idx = None;
        app.state.simulation.active_analysis_idx = None;
        app
    }

    #[test]
    fn result_navigator_selection_preserves_dataset_and_visibility_invariants() {
        let mut app = result_navigator_app();
        let dataset_id = app.state.simulation.runs[0].dataset_id;

        assert!(select_result_dataset(&mut app, 0));
        assert_eq!(app.state.simulation.active_run_idx, Some(0));
        assert_eq!(app.state.simulation.active_analysis_idx, Some(0));
        assert_eq!(
            app.state.workbench.documents.active(Workspace::Results),
            Some(&crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id))
        );

        let was_visible = app.state.simulation.runs[0].analyses[0].waveforms[0].visible;
        assert!(select_result_signal(&mut app, 0, 0, 0));
        let selected = app
            .state
            .ui
            .results
            .valid_selected_trace(&app.state.simulation)
            .expect("signal selection resolves against the active dataset");
        assert_eq!(selected.source_name(), "V(out)");
        crate::workbench::documents::result_document::prepare_viewer_state(&mut app);
        assert_eq!(
            app.state
                .ui
                .results
                .valid_selected_trace(&app.state.simulation)
                .map(|selected| selected.source_name()),
            Some("V(out)"),
            "the first Results render after activation must retain the navigator selection"
        );
        assert_eq!(
            app.state.simulation.runs[0].analyses[0].waveforms[0].visible,
            was_visible
        );

        assert!(select_result_analysis(&mut app, 0, 1));
        assert_eq!(app.state.simulation.active_run_idx, Some(0));
        assert_eq!(app.state.simulation.active_analysis_idx, Some(1));
        assert!(
            app.state
                .ui
                .results
                .valid_selected_trace(&app.state.simulation)
                .is_none(),
            "changing analysis clears the now-invalid signal selection"
        );
    }

    #[test]
    fn schematic_navigator_uses_the_upgraded_compact_tree_metrics() {
        assert_eq!(super::SCHEMATIC_NAV_ROW_HEIGHT, 24.0);
        assert_eq!(super::SCHEMATIC_NAV_LABEL_SIZE, 12.0);
        assert_eq!(super::SCHEMATIC_NAV_META_SIZE, 10.0);
    }

    /// The metadata register names the kind through the same accessor
    /// authority the unit column uses, so a derived projection never reports
    /// the kind of the wrapper instead of the quantity underneath it.
    #[test]
    fn quantity_metadata_names_the_kind_under_derived_projections() {
        for (name, unit, kind) in [
            ("V(out)", "V", "Voltage"),
            ("I(C1)", "A", "Current"),
            ("|I(VIN)|", "A", "Current"),
            ("phase(V(OUT))", "°", "Phase"),
            ("phase(I(VIN))", "°", "Current phase"),
            ("re(I(R1))", "A", "Current"),
            ("onoise", "V^2/Hz", "Noise PSD"),
        ] {
            assert_eq!(
                super::result_quantity_kind_label(name, unit),
                kind,
                "{name}"
            );
        }
    }

    /// A run's quantities open as an index of its analyses: the active
    /// analysis is disclosed, the rest stay closed until asked for, and an
    /// explicit toggle survives as the session's answer either way.
    #[test]
    fn analysis_groups_default_to_disclosing_only_the_active_analysis() {
        let ctx = egui::Context::default();
        assert!(super::result_browser_group_expanded(&ctx, 0, true));
        assert!(!super::result_browser_group_expanded(&ctx, 1, false));

        super::set_result_browser_group_expanded(&ctx, 1, true);
        super::set_result_browser_group_expanded(&ctx, 0, false);
        assert!(super::result_browser_group_expanded(&ctx, 1, false));
        assert!(
            !super::result_browser_group_expanded(&ctx, 0, true),
            "closing the active analysis must outrank the default"
        );
    }

    /// A batch plot action is the single-row act repeated: it must reach the
    /// same visibility path, skip rows already in the requested state, and
    /// leave the checked set intact so a second action can follow.
    #[test]
    fn batch_plot_action_moves_only_the_checked_quantities() {
        let mut app = result_navigator_app();
        assert!(select_result_dataset(&mut app, 0));
        let checked: std::collections::BTreeSet<String> =
            ["V(out)".to_owned()].into_iter().collect();
        let untouched = app.state.simulation.runs[0].analyses[1].waveforms[0].visible;

        super::set_checked_signal_visibility(&mut app, &checked, false);
        assert!(
            !app.state.simulation.runs[0].analyses[0].waveforms[0].visible,
            "the checked quantity leaves its pane"
        );
        assert_eq!(
            app.state.simulation.runs[0].analyses[1].waveforms[0].visible, untouched,
            "an unchecked quantity in another analysis is not disturbed"
        );

        super::set_checked_signal_visibility(&mut app, &checked, true);
        assert!(
            app.state.simulation.runs[0].analyses[0].waveforms[0].visible,
            "the same act returns it"
        );
        // Idempotent: repeating the request cannot toggle it back off.
        super::set_checked_signal_visibility(&mut app, &checked, true);
        assert!(app.state.simulation.runs[0].analyses[0].waveforms[0].visible);
    }

    #[test]
    fn sample_counts_group_thousands_for_reading_as_magnitudes() {
        assert_eq!(super::grouped_count(0), "0");
        assert_eq!(super::grouped_count(151), "151");
        assert_eq!(super::grouped_count(15_001), "15,001");
        assert_eq!(super::grouped_count(100_008), "100,008");
        assert_eq!(super::grouped_count(1_000_000), "1,000,000");
    }

    #[test]
    fn browser_rows_keep_the_mockup_two_register_metrics() {
        assert_eq!(super::RESULT_QUANTITY_ROW_HEIGHT, 36.0);
        assert_eq!(super::RESULT_ANALYSIS_HEAD_HEIGHT, 31.0);
        // Touch stages still promote both to the full target height.
        assert_eq!(
            super::responsive_result_control_height(
                super::RESULT_QUANTITY_ROW_HEIGHT,
                super::TOUCH_TARGET_HEIGHT
            ),
            super::TOUCH_TARGET_HEIGHT
        );
    }
}
