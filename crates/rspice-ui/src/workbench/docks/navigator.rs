//! Workspace-aware navigation tree.

mod design;
mod symbol;

mod netlist;
mod source_bundle;

use netlist::*;

use egui::{Align, Response, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::product::DatasetId;
use crate::simulation::netlist_gen::bus_notations;
use crate::state::ViewType;
use crate::state::{OutlineEntry, OutlineEntryKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultArtifactPresentationKey, ResultBrowserSelectionKey,
    ResultExpressionPresentationKey, SourceWaveformPresentationKey, analysis_default_unit,
    browser_signal_unit, exact_result_artifact_text, exact_result_signal_last_sample,
    exact_result_signal_tsv, result_artifact_stable_path, result_signal_stable_path,
    retained_evidence_is_valid,
};

use super::super::commands::result_navigation::reveal_producer_log;
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
const RESULT_BROWSER_VIRTUALIZATION_THRESHOLD: usize = 250;
const RESULT_BROWSER_RENDER_WINDOW_ROWS: usize = 120;
const RESULT_BROWSER_OVERSCAN_ROWS: usize = 20;
/// The content width the pair still reads at: the navigator dock's own minimum
/// of 220 px less its 8 px insets. Above it the typed facets stay side by side
/// as the mockup draws them, rather than stacking into a column of selects at
/// every ordinary dock width. Below it — the tablet and phone drawers — they
/// stack, so a right-to-left row can never enlarge beyond its host and resize
/// stays monotonic.
const RESULT_BROWSER_STACKED_FACET_MAX_WIDTH: f32 = 204.0;
/// Clipboard generation is synchronous on every supported target. Keep the
/// operation bounded and direct larger evidence sets to the streaming export
/// workflow instead of allocating an unbounded browser/WASM string.
const RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT: usize = 100_000;
const RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT: usize = 8_000_000;
const RESULT_MANIFEST_ROW_HEIGHT: f32 = 26.0;
const TOUCH_TARGET_HEIGHT: f32 = 44.0;
const PANEL_SEARCH_MARGIN_X: f32 = 8.0;
const SCHEMATIC_NAV_ROW_HEIGHT: f32 = 24.0;
const SCHEMATIC_NAV_LABEL_SIZE: f32 = tokens::FS_1;
const SCHEMATIC_NAV_META_SIZE: f32 = 10.0;
// Mirrors the mockup's `.section-body { padding-inline: 10px; }` contract so
// run-set values remain visually contained beside the analysis-stack divider.
const NAV_PROPERTY_PADDING_X: f32 = 10.0;
/// Gutter between a property row's label column and its value column, from the
/// mockup's `.form-grid { gap: 6px 11px; }` column gap.
const NAV_PROPERTY_COLUMN_GAP: f32 = 11.0;
/// The share of a property row its label column may claim before it elides.
///
/// The mockup lays this card out as a `.form-grid` — `minmax(110px, 0.8fr)`
/// for the label, `minmax(130px, 1.2fr)` for the value — so neither column can
/// grow into the other. At the 208 px of content the dock leaves below 1261 px
/// the 110 px floor is what binds, which is just over half the row. Laying the
/// two out as one row with the value floated right instead let a long value
/// grow leftwards until it reached the label: at that width "Supply voltage"
/// and "3 values · axis off" were painted with no gap between them at all.
const NAV_PROPERTY_LABEL_FRACTION: f32 = 0.5;
/// Axis values a Run set row shows before it counts the rest. Five, the same
/// as the mockup rail: enough to recognize a corner list, short enough that
/// the row stays one line.
const NAV_AXIS_VALUE_LIMIT: usize = 5;
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

fn result_browser_facets_stack(available_width: f32) -> bool {
    available_width < RESULT_BROWSER_STACKED_FACET_MAX_WIDTH
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
        use crate::workbench::documents::code_workspace::CodeWorkspacePage;

        // Canonical code navigator order: header, search, page tabs, outline.
        // The outline belongs to the page: a Verilog-A module, an automation
        // pipeline, and a SPICE deck are different objects and read
        // differently, so none of them borrows another's tree.
        workspace_search(ui, app, Workspace::Netlist);
        code_workspace_pages(ui, app);
        match app.state.ui.code_workspace.page {
            CodeWorkspacePage::Netlist => netlist(ui, app),
            CodeWorkspacePage::VerilogA => source_bundle::veriloga(ui, app),
            CodeWorkspacePage::Automation => source_bundle::automation(ui, app),
        }
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
    let messages = app.state.ui.messages();
    let mut selected = None;
    for (index, page) in CodeWorkspacePage::ALL.into_iter().enumerate() {
        // The tab's identity is its position, not its name. Keying the `Id` on
        // the label made the widget move whenever the label was translated.
        let name = messages.text(match page {
            CodeWorkspacePage::Netlist => crate::workbench::MessageId::CodePageNetlist,
            CodeWorkspacePage::VerilogA => crate::workbench::MessageId::CodePageVerilogA,
            CodeWorkspacePage::Automation => crate::workbench::MessageId::CodePageAutomation,
        });
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
            egui::Id::new(("workbench.code.page", index)),
            Sense::click(),
        );
        let active = app.state.ui.code_workspace.page == page;
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), active, &name)
        });
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &name,
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
        Workspace::Netlist => {
            use crate::workbench::documents::code_workspace::CodeWorkspacePage;
            match app.state.ui.code_workspace.page {
                CodeWorkspacePage::Netlist => "Netlist outline",
                CodeWorkspacePage::VerilogA => "Verilog-A project",
                CodeWorkspacePage::Automation => "Automation workspace",
            }
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
        app.state.workbench.navigator_trees.filter_mut(workspace),
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
        .navigator_filter()
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
        .navigator_filter()
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
        .navigator_filter()
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
        .navigator_filter()
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

/// The section that holds the standing capability notice.
const CAPABILITY_POLICY_SECTION: &str = "Capability policy";
/// What that notice says, named once so the filter and the paint agree.
const CAPABILITY_POLICY_NOTICE: &str = "Every analysis declares release-engine, preview, \
     compatibility, platform and sign-off contracts before it can be added.";

/// Whether one line of the simulation rail survives the navigator's filter.
///
/// `query` arrives trimmed and lower-cased from the panel's own field. An
/// empty one keeps everything and allocates nothing, which is the path the
/// rail takes on nearly every frame; only a reader who is actually filtering
/// pays for the comparison.
///
/// The rail used to spend the query on the nine route rows and nothing else,
/// so a search for a corner name hid every page and left the Run set axes, the
/// Variation row and the capability banner sitting under it, unfiltered and
/// unrelated to what had been typed.
fn nav_matches(query: &str, text: &str) -> bool {
    query.is_empty() || text.to_lowercase().contains(query)
}

fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::simulation::plan::AnalysisKind;

    let enabled = app.state.sim_setup.enabled_analysis_instance_count();
    let total = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.instances().len());
    let query = app.state.workbench.navigator_filter().trim().to_lowercase();
    // One validation for the whole rail. The Run set row states the point
    // count and the Run set card below states it again, and each asked the run
    // set for it — which validates the declaration and expands the space, so
    // the dock paid for the expansion twice a frame to print one number in two
    // places. The forecast's point count does not depend on how many analyses
    // are enabled; only its task count does, so one validation answers both.
    let run_set_validation =
        crate::simulation::run_set::validate(&app.state.sim_setup.run_set, enabled);
    let declared_points = run_set_validation
        .errors
        .is_empty()
        .then_some(run_set_validation.forecast.point_count);
    // The nine routes are resolved before the head is drawn, because the head
    // is that section's own and a header over nothing is a claim the rail
    // cannot support.
    let routes = SimulationPage::NAVIGATION
        .into_iter()
        .filter(|page| nav_matches(&query, page.label()))
        .collect::<Vec<_>>();
    if !routes.is_empty() {
        section_header(ui, "Lab characterization", Some(&format!("{enabled} on")));
    }
    ScrollArea::vertical()
        .id_salt("workbench.simulation.navigator")
        .show(ui, |ui| {
            let active = app.state.workbench.simulation_page;
            // Enabled over total. The previous wording called the total
            // "active", which is what `enabled` means, so a one-analysis plan
            // read "1 active · 1 enabled".
            let analyses_meta = format!("{enabled} / {total}");
            let mut requested = None;
            for page in routes {
                let label = page.label();
                let meta = simulate_nav_meta(app, page, &analyses_meta, declared_points);
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
            // One row per declared axis, because the card is a read of the run
            // set and the run set is its axes. The reference point it named
            // instead is one point of that space: with three axes enabled the
            // rail reported a single process corner and a single temperature
            // while the plan was going to run twenty-seven points over four
            // dimensions, and nothing on the rail said so.
            let run_set = &app.state.sim_setup.run_set;
            let points = run_set_validation.forecast.point_count.max(1);
            // An axis answers for its own name, which is what the row states.
            // Matching the values instead would leave a row on screen for a
            // query the reader cannot see the reason for.
            let axes = run_set
                .dimensions
                .iter()
                .filter(|dimension| nav_matches(&query, &dimension.name))
                .collect::<Vec<_>>();
            let no_axes = run_set.dimensions.is_empty() && nav_matches(&query, "Axes");
            let variation = nav_matches(&query, "Variation");
            if !axes.is_empty() || no_axes || variation {
                section_header(ui, "Run set", Some(&format!("{points} pts")));
            }
            if no_axes {
                nav_property(ui, "Axes", "none declared");
            }
            for dimension in axes {
                let values = dimension
                    .values
                    .iter()
                    .map(|value| value.lexical.as_str())
                    .collect::<Vec<_>>();
                // A disabled axis states its size rather than its values: it is
                // declared, it is not in the run, and a row that showed the
                // values would read as one that is.
                // The unit is the axis's, not the value's: the run set stores
                // temperature values as bare lexical numbers, so a rail that
                // printed them as authored said "-40 25 125" and left the
                // reader to guess °C from the row's name. The mockup card
                // carries the unit on every value; one unit for the axis says
                // the same thing and is what fits the dock's 228 px.
                let unit = dimension.unit();
                let value = if !dimension.enabled {
                    format!("{} values \u{b7} axis off", values.len())
                } else if values.len() > NAV_AXIS_VALUE_LIMIT {
                    format!(
                        "{}{} +{}",
                        values[..NAV_AXIS_VALUE_LIMIT].join(" "),
                        unit.map(|unit| format!(" {unit}")).unwrap_or_default(),
                        values.len() - NAV_AXIS_VALUE_LIMIT
                    )
                } else {
                    format!(
                        "{}{}",
                        values.join(" "),
                        unit.map(|unit| format!(" {unit}")).unwrap_or_default()
                    )
                };
                nav_property(ui, &dimension.name, &value);
            }
            if variation {
                nav_property(
                    ui,
                    "Variation",
                    if app
                        .state
                        .sim_setup
                        .has_enabled_analysis_kind(AnalysisKind::MonteCarlo)
                    {
                        "Monte Carlo enabled"
                    } else {
                        "no Monte Carlo instance"
                    },
                );
            }
            // The banner is prose rather than a row, so it answers for the
            // sentence it paints as well as for the section that holds it.
            if nav_matches(&query, CAPABILITY_POLICY_SECTION)
                || nav_matches(&query, CAPABILITY_POLICY_NOTICE)
            {
                ui.add_space(8.0);
                section_header(ui, CAPABILITY_POLICY_SECTION, None);
                capability_policy_banner(ui);
            }
        });
}

/// The panel's one creating action, at the foot of the tree it adds to.
///
/// Full-bleed and square rather than an inset pill: it reads as the last row
/// of the tree rather than a control floating inside the panel, and it carries
/// the filled accent so it is the obvious way into an empty plan.
///
/// The row is the command's, not a look-alike: the panel draws it on all nine
/// setup routes, and it was writing the catalogue's four palette fields itself
/// while only the Analyses route drew the catalogue. Routing through
/// `Command::AddAnalysis` puts the arming and the drawing under one owner.
fn simulation_plan_creator(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.add_space(SIMULATION_CREATOR_GAP);
    let width = ui.available_width();
    if Button::new(Command::AddAnalysis.spec().label)
        .accent()
        .min_width(width)
        .min_height(Tokens::get(ui.ctx()).metrics.row_h + SIMULATION_CREATOR_EXTRA_HEIGHT)
        .square()
        .show(ui)
        .clicked()
    {
        Command::AddAnalysis.execute(app);
    }
}

const fn simulate_nav_icon(page: SimulationPage) -> WorkbenchIcon {
    match page {
        SimulationPage::Analyses => WorkbenchIcon::Results,
        SimulationPage::Excitations => WorkbenchIcon::ArrowRight,
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
fn simulate_nav_meta(
    app: &RSpiceApp,
    page: SimulationPage,
    analyses: &str,
    points: Option<usize>,
) -> Option<String> {
    let payload = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.plan_data(plan_id));
    let count = |value: usize| (value > 0).then(|| value.to_string());
    match page {
        SimulationPage::Analyses => Some(analyses.to_owned()),
        // Counted from the drawing rather than from the plan payload: a source
        // is a placed instance, and the plan holds no list of them.
        SimulationPage::Excitations => count(
            crate::simulation::placed_sources::placed_source_count(&app.state.schematic),
        ),
        SimulationPage::Variables => count(payload.map_or(0, |data| data.design_variables.len())),
        SimulationPage::Outputs | SimulationPage::Save => {
            count(payload.map_or(0, |data| data.saved_outputs.len()))
        }
        SimulationPage::Specifications => count(payload.map_or(0, |data| data.specs.len())),
        SimulationPage::RunSet => points.map(|points| format!("{points} pt")),
        SimulationPage::Models => count(app.state.sim_setup.model_bindings.len()),
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
                        egui::RichText::new(CAPABILITY_POLICY_NOTICE)
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

/// One `label`/`value` row of a navigator card, laid out as two columns.
///
/// The label column is measured — it takes what the label needs, up to
/// [`NAV_PROPERTY_LABEL_FRACTION`] of the row, and elides past that. The value
/// column is what is left, and the value wraps inside it rather than growing
/// out of it. Neither column can reach the other, which is the whole point:
/// the previous single-row layout floated the value right and let it run
/// leftwards until it hit the label, so at the dock's 228 px "Supply voltage"
/// and "3 values · axis off" were painted over each other with no space
/// between them and nothing in the row said which glyphs belonged to which.
fn nav_property(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let width = ui.available_width();
    let inner = (width - NAV_PROPERTY_PADDING_X * 2.0).max(0.0);
    let label_natural = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(label.to_owned(), label_font.clone(), t.color.text_dim)
            .size()
            .x
    });
    let label_column = label_natural.min(inner * NAV_PROPERTY_LABEL_FRACTION);
    let value_column = (inner - label_column - NAV_PROPERTY_COLUMN_GAP).max(0.0);
    // Both galleys are laid out against their own column before either is
    // placed. Nesting two child regions inside one row does not hold here:
    // egui grows a region to include whatever overflowed it, so the first
    // value that did not fit widened the card and every row below it started
    // further right than the one above.
    let label_galley = ui.fonts_mut(|fonts| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            label.to_owned(),
            label_font,
            t.color.text_dim,
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width(label_column);
        fonts.layout_job(job)
    });
    let value_galley = ui
        .fonts_mut(|fonts| fonts.layout(value.to_owned(), value_font, t.color.text, value_column));
    let height = label_galley.size().y.max(value_galley.size().y);
    let (_, rect) = ui.allocate_space(Vec2::new(width, height));
    let painter = ui.painter();
    painter.galley(
        egui::pos2(rect.left() + NAV_PROPERTY_PADDING_X, rect.top()),
        label_galley,
        t.color.text_dim,
    );
    // The value block is flush to the card's trailing inset. A value that had
    // to wrap keeps its lines left-aligned inside that block, so a two-line
    // value still reads as one value rather than two ragged fragments.
    painter.galley(
        egui::pos2(
            rect.right() - NAV_PROPERTY_PADDING_X - value_galley.size().x,
            rect.top(),
        ),
        value_galley,
        t.color.text,
    );
}

fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    if !app.state.ui.results.checked_result_quantities.is_empty()
        && !egui::Popup::is_any_open(ui.ctx())
        && ui.input(|input| input.key_pressed(egui::Key::Escape))
    {
        app.state.ui.results.clear_checked_signals();
    }
    let query = app.state.workbench.navigator_filter().trim().to_lowercase();
    let active_browser_tab = results_browser_active_tab(ui.ctx());
    // Kind/sort read the values the toolbar stored last frame — the row
    // renders below the tab band while filtering happens here, the same
    // one-frame contract the viewer-tab chevrons use.
    let stored_kind = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserKind>(results_browser_kind_id()))
        .unwrap_or_default();
    let sort = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserSort>(results_browser_sort_id()))
        .unwrap_or_default();
    let stored_scope = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserScope>(results_browser_scope_id()))
        .unwrap_or_default();
    let stored_producer_facet = ui.ctx().data(|data| {
        data.get_temp::<Option<AnalysisPresentationKey>>(results_browser_producer_id())
            .flatten()
    });
    // Every facet belongs to the quantity list, so every one of them yields on
    // the tabs that do not list quantities.
    let kind = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_kind
    } else {
        ResultsBrowserKind::All
    };
    let scope = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_scope
    } else {
        ResultsBrowserScope::All
    };
    let producer_facet = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_producer_facet
    } else {
        None
    };
    // A snapshot keeps the favorites predicate borrow-free inside the run map;
    // recency reads its owning accessor, which states the rank once.
    let favorite_signals = app.state.ui.results.favorite_signals.clone();
    let active_run = app
        .state
        .simulation
        .active_run_idx
        .or_else(|| app.state.simulation.runs.len().checked_sub(1));
    let active_analysis = app.state.simulation.active_analysis_idx.or_else(|| {
        active_run.and_then(|run| {
            app.state
                .simulation
                .runs
                .get(run)
                .is_some_and(|run| !run.analyses.is_empty())
                .then_some(0)
        })
    });
    let active_analysis_key = active_run
        .and_then(|run| app.state.simulation.runs.get(run))
        .zip(active_analysis)
        .and_then(|(run, analysis_index)| {
            run.analyses
                .get(analysis_index)
                .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
        });
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
    let expression_source = active_analysis_key.map_or_else(Vec::new, |analysis| {
        app.state
            .ui
            .results
            .expression_entries_for_analysis(&app.state.simulation, analysis)
    });
    let expressions = expression_source
        .iter()
        .enumerate()
        .filter(|(_, (_, expression))| {
            query.is_empty() || expression.text.to_lowercase().contains(&query)
        })
        .map(|(expression_index, (identity, expression))| {
            (identity.clone(), expression_index, expression.clone())
        })
        .collect::<Vec<_>>();
    let expression_query_match = !query.is_empty() && !expressions.is_empty();
    let analysis_keys = app
        .state
        .simulation
        .runs
        .iter()
        .flat_map(|run| {
            run.analyses
                .iter()
                .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
        })
        .collect::<Vec<_>>();
    let analysis_integrity = analysis_keys
        .into_iter()
        .map(|key| {
            let valid = retained_evidence_is_valid(&mut app.state, key);
            (key, valid)
        })
        .collect::<std::collections::HashMap<_, _>>();
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
                    let presentation_key =
                        AnalysisPresentationKey::new(run.dataset_id, analysis);
                    let evidence_valid = analysis_integrity
                        .get(&presentation_key)
                        .copied()
                        .unwrap_or(false);
                    let source_currentness = crate::workbench::documents::result_document::operational_state::analysis_currentness(
                        &app.state,
                        run,
                        analysis,
                        evidence_valid,
                    );
                    let currentness_label = source_currentness.id();
                    let completeness_class = result_analysis_completeness(run, analysis);
                    if producer_facet.is_some_and(|producer| producer != presentation_key) {
                        return None;
                    }
                    let (integrity, completeness) = result_analysis_browser_status(
                        run,
                        analysis,
                        evidence_valid,
                    );
                    let analysis_query_matches = query.is_empty()
                        || run.label.to_lowercase().contains(&query)
                        || analysis.label.to_lowercase().contains(&query)
                        || analysis
                            .analysis_type
                            .display_name()
                            .to_lowercase()
                            .contains(&query)
                        || analysis
                            .analysis_type
                            .short_label()
                            .to_lowercase()
                            .contains(&query)
                        || analysis.id.to_string().contains(&query)
                        || analysis.provenance().is_some_and(|provenance| {
                            provenance
                                .source_instance_id()
                                .to_string()
                                .to_lowercase()
                                .contains(&query)
                        })
                        || integrity.contains(&query)
                        || completeness.contains(&query)
                        || currentness_label.contains(&query);
                    let mut signals = analysis
                        .waveforms
                        .iter()
                        .enumerate()
                        .filter(|(_, waveform)| {
                            // The unit the signal actually reads in decides
                            // its kind. Treating "not a current" as a voltage
                            // filed noise densities and decibel magnitudes
                            // under Voltage.
                            let unit = browser_signal_unit(
                                &waveform.name,
                                waveform.unit.as_deref(),
                                analysis_default_unit(analysis.analysis_type),
                            );
                            kind.admits(unit)
                        })
                        .filter_map(|(waveform_index, waveform)| {
                            let unit = browser_signal_unit(
                                &waveform.name,
                                waveform.unit.as_deref(),
                                analysis.analysis_type.axis_info().3,
                            );
                            let quantity_kind =
                                result_quantity_kind_label(&waveform.name, unit);
                            if !analysis_query_matches
                                && !waveform.name.to_lowercase().contains(&query)
                                && !unit.to_lowercase().contains(&query)
                                && !quantity_kind.to_lowercase().contains(&query)
                                && !"vector".contains(&query)
                            {
                                return None;
                            }
                            let identity = SourceWaveformPresentationKey::new(
                                presentation_key,
                                waveform.name.clone(),
                            );
                            let in_scope = match scope {
                                ResultsBrowserScope::All => true,
                                ResultsBrowserScope::Favorites => {
                                    favorite_signals.contains(&identity)
                                }
                                ResultsBrowserScope::Recent => app
                                    .state
                                    .ui
                                    .results
                                    .recent_signal_rank(&identity)
                                    .is_some(),
                            };
                            if !in_scope {
                                return None;
                            }
                            let samples = waveform.y.len();
                            Some(ResultSignal {
                                waveform_index,
                                visible: app
                                    .state
                                    .ui
                                    .results
                                    .waveform_visibility(&identity, waveform.visible),
                                identity,
                                name: waveform.name.clone(),
                                color: waveform.color.clone(),
                                meta: format!(
                                    "{} · vector · {} {}",
                                    quantity_kind,
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
                            })
                        })
                        .collect::<Vec<_>>();
                    // The row's meta line stays typed metadata: kind, shape,
                    // sample count. Data state belongs to the producing
                    // analysis, so appending it here would repeat one word down
                    // every row the analysis owns; the head above them and the
                    // inspector beside them each state it once.
                    let all_artifacts = retained_result_artifacts(analysis, presentation_key);
                    let total_artifacts = all_artifacts.len();
                    let mut artifacts = all_artifacts
                        .into_iter()
                        .filter(|artifact| kind.admits_artifact(artifact.kind, &artifact.unit))
                        .filter(|artifact| {
                            analysis_query_matches
                                || artifact.name.to_lowercase().contains(&query)
                                || artifact
                                    .identity
                                    .canonical_name()
                                    .to_lowercase()
                                    .contains(&query)
                                || artifact.kind.label().to_lowercase().contains(&query)
                                || artifact.meta.to_lowercase().contains(&query)
                        })
                        .filter(|artifact| match scope {
                            ResultsBrowserScope::All => true,
                            ResultsBrowserScope::Favorites => app
                                .state
                                .ui
                                .results
                                .is_favorite_result_artifact(&artifact.identity),
                            ResultsBrowserScope::Recent => app
                                .state
                                .ui
                                .results
                                .recent_result_artifact_rank(&artifact.identity)
                                .is_some(),
                        })
                        .collect::<Vec<_>>();
                    if scope == ResultsBrowserScope::Recent {
                        // Recent means recency order; the sort facet yields.
                        signals.sort_by_key(|signal| {
                            app.state
                                .ui
                                .results
                                .recent_signal_rank(&signal.identity)
                                .unwrap_or(usize::MAX)
                        });
                        artifacts.sort_by_key(|artifact| {
                            app.state
                                .ui
                                .results
                                .recent_result_artifact_rank(&artifact.identity)
                                .unwrap_or(usize::MAX)
                        });
                    } else if sort == ResultsBrowserSort::Name {
                        signals.sort_by(|left, right| {
                            left.name
                                .to_ascii_lowercase()
                                .cmp(&right.name.to_ascii_lowercase())
                                .then_with(|| left.name.cmp(&right.name))
                        });
                        artifacts.sort_by(|left, right| {
                            left.name
                                .to_ascii_lowercase()
                                .cmp(&right.name.to_ascii_lowercase())
                                .then_with(|| left.name.cmp(&right.name))
                        });
                    }
                    let matches_analysis =
                        analysis_query_matches || !signals.is_empty() || !artifacts.is_empty();
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
                        total_signals: analysis.waveforms.len() + total_artifacts,
                        data_state: result_analysis_data_state(
                            source_currentness,
                            completeness_class,
                        ),
                        analysis_index,
                        presentation_key,
                        label,
                        short_label: analysis.analysis_type.short_label(),
                        success: analysis.success,
                        signals,
                        artifacts,
                    })
                })
                .collect::<Vec<_>>();
            let run_currentness = crate::workbench::documents::result_document::operational_state::run_currentness(
                &app.state,
                run,
                |analysis| {
                    analysis_integrity
                        .get(&AnalysisPresentationKey::new(run.dataset_id, analysis))
                        .copied()
                        .unwrap_or(false)
                },
            );
            let run_currentness_label = run_currentness.id();
            let matches_run = query.is_empty()
                || run.label.to_lowercase().contains(&query)
                || !analyses.is_empty()
                || (active_run == Some(run_index) && expression_query_match);
            let run_integrity = if run.analyses.iter().all(|analysis| {
                analysis_integrity
                    .get(&AnalysisPresentationKey::new(run.dataset_id, analysis))
                    .copied()
                    .unwrap_or(false)
            }) {
                "integrity verified"
            } else {
                "corrupted"
            };
            let run_state = result_run_operational_label(run);
            // Every state word stays searchable even though the rows no longer
            // print them: a reader who types "corrupted" is asking a question
            // about the dataset, not about a label.
            let matches_run = matches_run
                || run_integrity.contains(&query)
                || run_state.contains(&query)
                || run_currentness_label.contains(&query)
                || run.dataset_id.to_string().to_lowercase().contains(&query);
            matches_run.then(|| ResultRun {
                run_index,
                dataset_id: run.dataset_id,
                label: run.label.clone(),
                success: run.success,
                data_state: result_analysis_data_state(
                    run_currentness,
                    result_run_completeness(run),
                ),
                analysis_count: run.analyses.len(),
                analyses,
            })
        })
        .collect::<Vec<_>>();

    let signal_count = runs
        .iter()
        .flat_map(|run| run.analyses.iter())
        .map(|analysis| analysis.signals.len() + analysis.artifacts.len())
        .sum::<usize>();
    let visible_result_keys =
        runs.iter()
            .filter(|run| {
                !query.is_empty()
                    || result_browser_dataset_expanded(
                        ui.ctx(),
                        run.dataset_id,
                        active_run == Some(run.run_index) || runs.len() == 1,
                    )
            })
            .flat_map(|run| {
                run.analyses.iter().filter(|analysis| {
                    !query.is_empty()
                        || result_browser_group_expanded(
                            ui.ctx(),
                            analysis.presentation_key,
                            active_run == Some(run.run_index)
                                && active_analysis == Some(analysis.analysis_index),
                        )
                })
            })
            .flat_map(|analysis| {
                analysis
                    .signals
                    .iter()
                    .map(|signal| ResultBrowserSelectionKey::Waveform(signal.identity.clone()))
                    .chain(analysis.artifacts.iter().map(|artifact| {
                        ResultBrowserSelectionKey::Artifact(artifact.identity.clone())
                    }))
            })
            .collect::<Vec<_>>();
    let tab = results_browser_tab_band(ui, [signal_count, runs.len(), expressions.len()]);
    // The mockup's browser toolbar: the query over this tab, then the analysis,
    // kind, and sort facets. The facets are absent on Datasets and Expressions —
    // a manifest binding has no quantity kind to filter and no hierarchy to
    // order, and a saved expression is found by name.
    results_browser_toolbar(
        ui,
        app,
        stored_kind,
        sort,
        stored_producer_facet,
        tab == ResultsBrowserTab::Signals,
    );
    // The mockup's status band: the signals tab owns the scope control on
    // the left — All | Favorites | Recent over the session's real star and
    // recency state — and every tab keeps the live inventory count on the
    // right, with the provenance sentence riding its tooltip.
    {
        let t = Tokens::get(ui.ctx());
        let shown = match tab {
            ResultsBrowserTab::Signals => signal_count,
            ResultsBrowserTab::Datasets => runs.len(),
            ResultsBrowserTab::Expressions => expressions.len(),
        };
        let loaded = match tab {
            ResultsBrowserTab::Signals => app
                .state
                .simulation
                .runs
                .iter()
                .map(|run| {
                    run.analyses
                        .iter()
                        .map(|analysis| {
                            analysis.waveforms.len()
                                + retained_result_artifacts(
                                    analysis,
                                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                                )
                                .len()
                        })
                        .sum::<usize>()
                })
                .sum(),
            ResultsBrowserTab::Datasets => app.state.simulation.runs.len(),
            ResultsBrowserTab::Expressions => expression_source.len(),
        };
        // Two numbers, no noun: the tab band above already names what is being
        // counted and the scope control beside it already says which working
        // set is in force, so spelling either one again costs the row the width
        // the count needs at the dock's minimum.
        let count_copy = format!("{shown} / {loaded}");
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
                // The count is the band's last register, and it carries the
                // whole inventory disclosure on its tooltip: matching,
                // rendered, and loaded stay distinguished without a second
                // band restating the number printed here.
                let inventory = ui
                    .ctx()
                    .data(|data| data.get_temp::<(usize, usize, usize)>(results_browser_inventory_id()));
                let count_tooltip = match inventory {
                    Some((matching, rendered, loaded)) if tab == ResultsBrowserTab::Signals => {
                        format!(
                            "{matching} matching · {rendered} rendered · {loaded} loaded · exact metadata · stable IDs\nLists virtualize at {RESULT_BROWSER_VIRTUALIZATION_THRESHOLD} rows; the render window is bounded to {RESULT_BROWSER_RENDER_WINDOW_ROWS} rows with a {RESULT_BROWSER_OVERSCAN_ROWS}-row contract."
                        )
                    }
                    _ => "Live inventory of the retained evidence behind this tab · exact metadata · stable IDs".to_owned(),
                };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(count_copy)
                            .font(theme::mono(tokens::FS_MICRO, FontWeight::Medium))
                            .color(t.color.text_faint),
                    )
                    .on_hover_text(count_tooltip);
                });
            },
        );
    }
    // The mockup's `.result-browser-selection`: what a batch action would
    // act on, with only the actions RSpice actually performs on a set of
    // quantities — plot membership, and letting the set go.
    if tab == ResultsBrowserTab::Signals
        && (!app.state.ui.results.checked_result_quantities.is_empty()
            || !visible_result_keys.is_empty())
    {
        let t = Tokens::get(ui.ctx());
        let checked = app.state.ui.results.checked_result_quantities.clone();
        let checked_ordered = ordered_checked_result_keys(&checked, &app.state.simulation.runs);
        let exact_validation_error = checked_ordered.iter().find_map(|key| {
            crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
                key,
                &app.state.simulation.runs,
            )
            .err()
        });
        let exact_evidence_available = exact_validation_error.is_none();
        let plot_compatible = !checked.is_empty()
            && exact_evidence_available
            && checked.iter().all(|quantity| quantity.waveform().is_some());
        let clipboard_sample_count = checked_ordered
            .iter()
            .filter_map(ResultBrowserSelectionKey::waveform)
            .filter_map(|key| {
                key.resolve(&app.state.simulation.runs)
                    .map(|(.., waveform)| waveform.x.len())
            })
            .sum::<usize>();
        let clipboard_exact_available = exact_evidence_available
            && !checked_ordered.is_empty()
            && clipboard_sample_count <= RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT;
        let compare_selection_compatible = exact_evidence_available
            && checked_ordered.len() >= 2
            && checked_ordered
                .iter()
                .map(ResultBrowserSelectionKey::dataset_id)
                .collect::<std::collections::HashSet<_>>()
                .len()
                >= 2
            && checked_ordered
                .iter()
                .filter_map(|key| {
                    crate::workbench::documents::result_document::result_browser_selection_canonical_name(
                        key,
                        &app.state.simulation.runs,
                    )
                    .ok()
                })
                .collect::<std::collections::HashSet<_>>()
                .len()
                == 1;
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 29.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, t.color.accent_dim);
                if checked.is_empty() {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            if ui
                                .add_enabled(
                                    !visible_result_keys.is_empty(),
                                    egui::Button::new(format!(
                                        "Select visible ({})",
                                        visible_result_keys.len()
                                    )),
                                )
                                .on_hover_text(
                                    "Select every quantity in the current filtered scope",
                                )
                                .clicked()
                            {
                                app.state
                                    .ui
                                    .results
                                    .select_visible_signals(&visible_result_keys);
                            }
                        },
                    );
                    return;
                }
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!("{} selected", checked.len()))
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(6.0);
                    ui.menu_button("Actions...", |ui| {
                        if ui
                            .add_enabled(
                                !visible_result_keys.is_empty(),
                                egui::Button::new("Select visible quantities"),
                            )
                            .clicked()
                        {
                            app.state
                                .ui
                                .results
                                .select_visible_signals(&visible_result_keys);
                            ui.close();
                        }
                        if ui.button("Clear selection").clicked() {
                            app.state.ui.results.clear_checked_signals();
                            ui.close();
                        }
                        let show = ui.add_enabled(
                            plot_compatible,
                            egui::Button::new("Show selected waveforms"),
                        );
                        if !plot_compatible {
                            show.clone().on_disabled_hover_text(
                                exact_validation_error.as_deref().unwrap_or(
                                    "Plot membership requires waveform-only selection; choose a compatible typed viewer for other evidence.",
                                ),
                            );
                        }
                        if show.clicked() {
                            set_checked_signal_visibility(app, &checked, true);
                            ui.close();
                        }
                        let hide = ui.add_enabled(
                            plot_compatible,
                            egui::Button::new("Hide selected waveforms"),
                        );
                        if !plot_compatible {
                            hide.clone().on_disabled_hover_text(
                                exact_validation_error.as_deref().unwrap_or(
                                    "Plot membership requires waveform-only selection; typed scalars, arrays, events, and contributions remain selected for metadata actions.",
                                ),
                            );
                        }
                        if hide.clicked() {
                            set_checked_signal_visibility(app, &checked, false);
                            ui.close();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(
                                !checked_ordered.is_empty(),
                                egui::Button::new("Copy canonical names"),
                            )
                            .clicked()
                        {
                            let result = checked_ordered
                                .iter()
                                .map(|key| {
                                    crate::workbench::documents::result_document::result_browser_selection_canonical_name(
                                        key,
                                        &app.state.simulation.runs,
                                    )
                                })
                                .collect::<Result<Vec<_>, _>>()
                                .map(|names| names.join("\n"));
                            match result {
                                Ok(names) => ui.ctx().copy_text(names),
                                Err(error) => result_browser_action_error(ui.ctx(), app, error),
                            }
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                !checked_ordered.is_empty(),
                                egui::Button::new("Copy stable dataset paths"),
                            )
                            .clicked()
                        {
                            let result = checked_ordered
                                .iter()
                                .map(|key| {
                                    crate::workbench::documents::result_document::result_browser_selection_stable_path(
                                        key,
                                        &app.state.simulation.runs,
                                    )
                                })
                                .collect::<Result<Vec<_>, _>>()
                                .map(|paths| paths.join("\n"));
                            match result {
                                Ok(paths) => ui.ctx().copy_text(paths),
                                Err(error) => result_browser_action_error(ui.ctx(), app, error),
                            }
                            ui.close();
                        }
                        let copy_exact = ui.add_enabled(
                            clipboard_exact_available,
                            egui::Button::new("Copy exact selected evidence"),
                        );
                        if !checked_ordered.is_empty() && !clipboard_exact_available {
                            copy_exact.clone().on_disabled_hover_text(
                                exact_validation_error.clone().unwrap_or_else(|| format!(
                                    "The waveform portion contains {clipboard_sample_count} samples. Clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT}; use exact export instead."
                                )),
                            );
                        }
                        if copy_exact.clicked() {
                            match crate::workbench::documents::result_document::exact_result_browser_selection_bundle(
                                &checked_ordered,
                                &app.state.simulation.runs,
                            ) {
                                Ok(exact) if exact.len() <= RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT => {
                                    ui.ctx().copy_text(exact);
                                }
                                Ok(exact) => result_browser_action_error(
                                    ui.ctx(),
                                    app,
                                    format!(
                                        "The exact selection requires {} bytes. Clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT}; use exact export instead.",
                                        exact.len()
                                    ),
                                ),
                                Err(error) => result_browser_action_error(ui.ctx(), app, error),
                            }
                            ui.close();
                        }
                        let export_exact = ui.add_enabled(
                            exact_evidence_available && !checked_ordered.is_empty(),
                            egui::Button::new("Export exact selection..."),
                        );
                        if let Some(error) = exact_validation_error.as_deref() {
                            export_exact.clone().on_disabled_hover_text(error);
                        }
                        if export_exact.clicked() {
                            app.state.ui.export_result_quantities_requested =
                                Some(checked_ordered.clone());
                            ui.close();
                        }
                        ui.separator();
                        let compare = ui.add_enabled(
                            compare_selection_compatible
                                && Command::CompareResultDatasets.is_enabled(app),
                            egui::Button::new("Compare selected quantity across datasets..."),
                        );
                        if !checked_ordered.is_empty() && !compare_selection_compatible {
                            compare.clone().on_disabled_hover_text(
                                exact_validation_error.as_deref().unwrap_or(
                                    "Select the same canonical quantity in at least two immutable datasets.",
                                ),
                            );
                        }
                        if compare.clicked() {
                            if let Some(first) = checked_ordered.first()
                                && select_result_browser_key(app, first)
                            {
                                Command::CompareResultDatasets.execute(app);
                            }
                            ui.close();
                        }
                        let inspect = ui.add_enabled(
                            checked_ordered.len() == 1,
                            egui::Button::new("Inspect selected entity"),
                        );
                        if checked_ordered.len() > 1 {
                            inspect.clone().on_disabled_hover_text(
                                "The inspector owns one exact entity at a time; reduce the selection to one row.",
                            );
                        }
                        if inspect.clicked() {
                            if let Some(first) = checked_ordered.first() {
                                select_result_browser_key(app, first);
                                app.state.workbench.inspector_visible = true;
                            }
                            ui.close();
                        }
                    });
                });
            },
        );
    }
    if tab == ResultsBrowserTab::Signals {
        show_virtualized_result_signals(
            ui,
            app,
            &runs,
            active_run,
            active_analysis,
            selected_trace.as_ref(),
            selected_artifact.as_ref(),
            &visible_result_keys,
            &query,
            kind,
            scope,
        );
        result_browser_selection_summary(
            ui,
            app,
            selected_trace.as_ref(),
            selected_artifact.as_ref(),
        );
        result_browser_precision_note(ui);
        return;
    }
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .show(ui, |ui| {
            match tab {
                ResultsBrowserTab::Signals => {
                    unreachable!("signal rows return through the virtualized result browser")
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
                        // Count, then the binding's one merged reading. Three
                        // state words here read as three separate verdicts on
                        // the same retained-evidence check.
                        let run_meta =
                            format!("{} analyses · {}", run.analysis_count, run.data_state.word);
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
                            app.state
                                .push_user_message(crate::diagnostics::ConsoleMessage::info(
                                    if enabled {
                                        format!(
                                            "Overlaying {} on the active result sheet.",
                                            run.label
                                        )
                                    } else {
                                        format!(
                                            "Removed {} from the active result sheet.",
                                            run.label
                                        )
                                    },
                                ));
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
                                select_result_analysis_by_key(app, analysis.presentation_key);
                            }
                        }
                    }
                }
                ResultsBrowserTab::Expressions => {
                    let Some(_analysis_index) = app.state.simulation.active_analysis_idx else {
                        muted(ui, "Select a retained result analysis to own expressions.");
                        return;
                    };
                    let Some(_analysis_key) = active_analysis_key else {
                        muted(ui, "The active retained analysis identity is unavailable.");
                        return;
                    };
                    expression_header(ui, app);
                    let mut toggled_expression: Option<ResultExpressionPresentationKey> = None;
                    for (identity, expression_index, expression) in &expressions {
                        let t = Tokens::get(ui.ctx());
                        let row_id = ui.id().with((
                            "result-expression",
                            identity.analysis(),
                            identity.text(),
                        ));
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
                            toggled_expression = Some(identity.clone());
                        }
                        locate_on_schematic_menu(&responses.selection, app, &expression.text);
                    }
                    if let Some(identity) = toggled_expression {
                        match app
                            .state
                            .ui
                            .results
                            .toggle_expression_visibility_by_key(&app.state.simulation, &identity)
                        {
                            Ok(()) => {
                                app.state.workspace.visualization_documents_dirty = true;
                            }
                            Err(error) => app.state.push_user_message(
                                crate::diagnostics::ConsoleMessage::warning(error),
                            ),
                        }
                    }
                }
            }
            result_browser_selection_summary(
                ui,
                app,
                selected_trace.as_ref(),
                selected_artifact.as_ref(),
            );
            result_browser_precision_note(ui);
        });
}

fn ordered_checked_result_keys(
    checked: &std::collections::HashSet<ResultBrowserSelectionKey>,
    runs: &[crate::state::SimulationRun],
) -> Vec<ResultBrowserSelectionKey> {
    let mut keys = checked.iter().cloned().collect::<Vec<_>>();
    keys.sort_by(|left, right| {
        let left =
            crate::workbench::documents::result_document::result_browser_selection_stable_path(
                left, runs,
            )
            .unwrap_or_default();
        let right =
            crate::workbench::documents::result_document::result_browser_selection_stable_path(
                right, runs,
            )
            .unwrap_or_default();
        left.cmp(&right)
    });
    keys
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
/// Waveform kinds come from their exact engineering unit. Typed producer
/// artifacts carry their own scalar/array/event/contribution classification,
/// so those facets never guess from a display label.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserKind {
    #[default]
    All,
    Voltage,
    Current,
    Power,
    NoiseDensity,
    Scalar,
    Array,
    EventStream,
    Contribution,
    OtherUnits,
}

impl ResultsBrowserKind {
    const ALL: [Self; 10] = [
        Self::All,
        Self::Voltage,
        Self::Current,
        Self::Power,
        Self::NoiseDensity,
        Self::Scalar,
        Self::Array,
        Self::EventStream,
        Self::Contribution,
        Self::OtherUnits,
    ];

    /// The base electrical dimensions. The facet closes with one negation over
    /// this set, so "anything that is not a volt, an amp, a watt, or a noise
    /// density" stays askable without a second unit control beside this one.
    const BASE_UNITS: [&'static str; 5] = ["V", "A", "W", "nV/√Hz", "V^2/Hz"];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All kinds",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Power => "Power",
            Self::NoiseDensity => "Noise density",
            Self::Scalar => "Scalar",
            Self::Array => "Array",
            Self::EventStream => "Event stream",
            Self::Contribution => "Contribution",
            Self::OtherUnits => "Other units",
        }
    }

    /// Whether a signal reading in `unit` belongs to this kind.
    fn admits(self, unit: &str) -> bool {
        match self {
            Self::All => true,
            Self::Voltage => unit == "V",
            Self::Current => unit == "A",
            Self::Power => unit == "W",
            Self::Scalar | Self::Array | Self::EventStream | Self::Contribution => false,
            Self::NoiseDensity => matches!(unit, "nV/√Hz" | "V^2/Hz"),
            Self::OtherUnits => !Self::BASE_UNITS.contains(&unit),
        }
    }

    /// A typed artifact answers to its own shape class, and to the closing
    /// negation through the unit it carries.
    fn admits_artifact(self, kind: ResultArtifactKind, unit: &str) -> bool {
        match self {
            Self::All => true,
            Self::Scalar => kind == ResultArtifactKind::Scalar,
            Self::Array => kind == ResultArtifactKind::Array,
            Self::EventStream => kind == ResultArtifactKind::EventStream,
            Self::Contribution => kind == ResultArtifactKind::Contribution,
            Self::Voltage | Self::Current | Self::Power | Self::NoiseDensity => false,
            Self::OtherUnits => !Self::BASE_UNITS.contains(&unit),
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
            // Labels stay inside the facet's own share of the row: the pair is
            // sized to the panel, not to its longest string.
            Self::Hierarchy => "Hierarchy",
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

/// One reading per producing analysis, merged from the one retained-evidence
/// read that currentness, integrity, and completeness all resolve from.
///
/// It is stated, never filtered on. Within a dataset binding every producer is
/// visible at once, so a facet over these words would narrow a list the reader
/// can already read; and because integrity failure *is* corrupted currentness,
/// separate controls could be aimed at each other to name an empty set. The
/// word has three homes and no fourth: the dataset row states its binding's
/// condition, the analysis head's retained fraction carries the reason on its
/// tooltip, and the inspector states it for one selected quantity.
/// The word and its consequence travel together, so no caller can look one up
/// from the other's display text.
#[derive(Clone, Copy)]
struct ResultAnalysisDataState {
    word: &'static str,
    note: &'static str,
}

const fn result_analysis_data_state(
    currentness: crate::workbench::documents::result_document::operational_state::ResultCurrentness,
    completeness: ResultCompletenessClass,
) -> ResultAnalysisDataState {
    use crate::workbench::documents::result_document::operational_state::ResultCurrentness;

    const fn state(word: &'static str, note: &'static str) -> ResultAnalysisDataState {
        ResultAnalysisDataState { word, note }
    }

    // Severity first: an unreadable payload outranks a short retained scope,
    // which outranks any claim about which source revision produced it.
    match (currentness, completeness) {
        (ResultCurrentness::Corrupted, _) => state(
            "corrupted",
            "Payload validation failed on load. Metadata stays readable; numeric actions stay blocked.",
        ),
        (_, ResultCompletenessClass::Failed) => state(
            "failed",
            "The producing analysis stopped before it finished. Retained scope is exact and never extrapolated.",
        ),
        (_, ResultCompletenessClass::Cancelled) => state(
            "cancelled",
            "The run was cancelled. Complete sample chunks were retained; nothing partial was synthesized.",
        ),
        (_, ResultCompletenessClass::Loading) => state(
            "loading",
            "Sample pages are still arriving. Counts grow; stable row identities are preserved.",
        ),
        (_, ResultCompletenessClass::Partial) => state(
            "partial",
            "Retained scope is narrower than the declared inventory. Absent quantities are named, never invented.",
        ),
        (ResultCurrentness::Stale, _) => state(
            "stale",
            "Source inputs changed after this immutable dataset was created. Review stays open; current-evidence actions stay blocked.",
        ),
        (ResultCurrentness::Superseded, _) => state(
            "superseded",
            "A later run covers this same analysis instance. The dataset itself is unchanged.",
        ),
        (ResultCurrentness::Unresolved, _) => state(
            "unresolved",
            "The relationship to the current source revision is unknown, so neither current nor stale may be claimed.",
        ),
        (ResultCurrentness::Recovered, _) => state(
            "recovered",
            "Reopened from a verified replica. Row identities were restored.",
        ),
        (ResultCurrentness::Current | ResultCurrentness::Partial, _) => state(
            "current",
            "Matches the current source revision, verified on load, complete against its declared inventory.",
        ),
    }
}

/// One way back when a narrowing has hidden everything.
///
/// A reader who filters to nothing has to be able to undo it without
/// remembering which control did it, so this resets every narrowing the
/// browser owns and names them all on its tooltip. Nothing is offered when the
/// emptiness is the dataset's own — there would be no filter to clear.
fn results_browser_clear_filters(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    let producer_filtering = ui.ctx().data(|data| {
        data.get_temp::<Option<AnalysisPresentationKey>>(results_browser_producer_id())
            .flatten()
            .is_some()
    });
    let filtering = !app.state.workbench.navigator_filter().trim().is_empty()
        || kind != ResultsBrowserKind::All
        || scope != ResultsBrowserScope::All
        || producer_filtering;
    if !filtering {
        return;
    }
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        if ui
            .small_button("Clear filters")
            .on_hover_text("Reset the query, the analysis, the kind, and the scope")
            .clicked()
        {
            app.state.workbench.clear_navigator_filter();
            ui.ctx().data_mut(|data| {
                data.insert_temp(results_browser_kind_id(), ResultsBrowserKind::All);
                data.insert_temp(results_browser_scope_id(), ResultsBrowserScope::All);
                data.insert_temp::<Option<AnalysisPresentationKey>>(
                    results_browser_producer_id(),
                    None,
                );
            });
        }
    });
}

/// The data browser's own toolbar: the query, the producing analysis, then the
/// kind and sort pair.
///
/// The mockup lays this out as one bordered block under the tab band with an
/// 8 px inset and a 5 px gutter, in three registers widest-first. The analysis
/// facet takes a whole row because its option strings are the longest in the
/// panel and because it is the one control that changes what the tree contains
/// rather than how it reads; the typed pair below splits unevenly, since kind
/// carries the longer option names and sort carries three short ones. Facets
/// are sized to the panel rather than to their own labels, so the block reads
/// as one control stack at any dock width.
fn results_browser_toolbar(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    kind: ResultsBrowserKind,
    sort: ResultsBrowserSort,
    producer: Option<AnalysisPresentationKey>,
    show_quantity_facets: bool,
) {
    const GUTTER: f32 = 5.0;

    let t = Tokens::get(ui.ctx());
    let workspace = app.state.workbench.workspace;
    ui.add_space(7.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        let field_width = panel_search_field_width(ui.available_width() + PANEL_SEARCH_MARGIN_X);
        let placeholder = "Find canonical name, path, unit, or type…";
        let response = ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(app.state.workbench.navigator_trees.filter_mut(workspace))
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
    if show_quantity_facets {
        // The producing analysis, on its own row. Every analysis the session
        // retained is offered under one leading "all" entry that carries the
        // count, so a reader can see how many producers the tree spans without
        // opening the list.
        let producers = app
            .state
            .simulation
            .runs
            .iter()
            .flat_map(|run| {
                run.analyses.iter().map(move |analysis| {
                    (
                        AnalysisPresentationKey::new(run.dataset_id, analysis),
                        format!("{} · {}", run.label, analysis.label),
                    )
                })
            })
            .collect::<Vec<_>>();
        let selected_producer = producer.and_then(|selected| {
            producers
                .iter()
                .find(|(key, _)| *key == selected)
                .map(|(_, label)| label.clone())
        });
        let mut producer_options = vec![format!("All analyses · {}", producers.len())];
        producer_options.extend(producers.iter().map(|(_, label)| label.clone()));
        ui.add_space(GUTTER);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(PANEL_SEARCH_MARGIN_X);
            let row_width = ui.available_width() - PANEL_SEARCH_MARGIN_X;
            if let Some(picked) = crate::ui::widgets::select(
                ui,
                "workbench.results.browser-producer",
                "Producing analysis",
                selected_producer.as_deref().unwrap_or(&producer_options[0]),
                &producer_options,
                row_width,
            ) {
                let key = picked
                    .checked_sub(1)
                    .and_then(|index| producers.get(index).map(|(key, _)| *key));
                ui.ctx()
                    .data_mut(|data| data.insert_temp(results_browser_producer_id(), key));
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
            // given, so the pair can be split against the row. A raw combo box
            // pads itself past the width it is asked for, which pushed the sort
            // facet off the query's inset and clipped it at the panel edge.
            let row_width = ui.available_width() - PANEL_SEARCH_MARGIN_X;
            let kind_options = ResultsBrowserKind::ALL
                .map(|value| value.label().to_owned())
                .to_vec();
            let sort_options = ResultsBrowserSort::ALL
                .map(|value| value.label().to_owned())
                .to_vec();
            if result_browser_facets_stack(row_width) {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = GUTTER;
                    if let Some(picked) = crate::ui::widgets::select(
                        ui,
                        "workbench.results.browser-kind",
                        "Quantity kind",
                        kind.label(),
                        &kind_options,
                        row_width,
                    ) {
                        let kind_now = ResultsBrowserKind::ALL[picked];
                        ui.ctx()
                            .data_mut(|data| data.insert_temp(results_browser_kind_id(), kind_now));
                    }
                    if let Some(picked) = crate::ui::widgets::select(
                        ui,
                        "workbench.results.browser-sort",
                        "Quantity sort",
                        sort.label(),
                        &sort_options,
                        row_width,
                    ) {
                        let sort_now = ResultsBrowserSort::ALL[picked];
                        ui.ctx()
                            .data_mut(|data| data.insert_temp(results_browser_sort_id(), sort_now));
                    }
                });
            } else {
                // Kind takes the wider share of the pair: its option list holds
                // the long names, sort holds three short ones.
                let pair_width = row_width - GUTTER;
                let kind_width = (pair_width * 0.554).max(48.0);
                let sort_width = (pair_width - kind_width).max(48.0);
                if let Some(picked) = crate::ui::widgets::select(
                    ui,
                    "workbench.results.browser-kind",
                    "Quantity kind",
                    kind.label(),
                    &kind_options,
                    kind_width,
                ) {
                    let kind_now = ResultsBrowserKind::ALL[picked];
                    ui.ctx()
                        .data_mut(|data| data.insert_temp(results_browser_kind_id(), kind_now));
                }
                ui.add_space(GUTTER);
                if let Some(picked) = crate::ui::widgets::select(
                    ui,
                    "workbench.results.browser-sort",
                    "Quantity sort",
                    sort.label(),
                    &sort_options,
                    sort_width,
                ) {
                    let sort_now = ResultsBrowserSort::ALL[picked];
                    ui.ctx()
                        .data_mut(|data| data.insert_temp(results_browser_sort_id(), sort_now));
                }
            }
        });
    }
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

/// Matching, rendered, and loaded row counts from the list pass, for the status
/// band's count tooltip.
fn results_browser_inventory_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-inventory")
}

fn results_browser_producer_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-producer-facet")
}

fn results_browser_active_tab(ctx: &egui::Context) -> ResultsBrowserTab {
    ctx.data(|data| {
        data.get_temp::<ResultsBrowserTab>(egui::Id::new("workbench.results.browser-tab"))
    })
    .unwrap_or_default()
}

fn result_browser_dataset_expanded(
    ctx: &egui::Context,
    dataset: DatasetId,
    default_open: bool,
) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new((
            "workbench.results.browser-dataset",
            dataset,
        )))
    })
    .unwrap_or(default_open)
}

fn set_result_browser_dataset_expanded(ctx: &egui::Context, dataset: DatasetId, expanded: bool) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.results.browser-dataset", dataset)),
            expanded,
        );
    });
}

/// Disclosure state for one analysis group, defaulting to open for the
/// active analysis and closed for the rest: a run with a hundred retained
/// quantities should open as an index of its analyses, not one long list.
fn result_browser_group_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    active: bool,
) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(("workbench.results.browser-group", analysis)))
    })
    .unwrap_or(active)
}

fn set_result_browser_group_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    expanded: bool,
) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.results.browser-group", analysis)),
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
    /// The binding's own merged reading. A dataset row is where a condition
    /// that holds for every analysis under it is stated, once.
    data_state: ResultAnalysisDataState,
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
    /// The one merged reading of this producer's retained evidence. The head
    /// paints no word for it; it explains the retained fraction on the tooltip.
    data_state: ResultAnalysisDataState,
    success: bool,
    signals: Vec<ResultSignal>,
    artifacts: Vec<ResultArtifact>,
}

struct ResultSignal {
    waveform_index: usize,
    identity: SourceWaveformPresentationKey,
    name: String,
    color: String,
    visible: bool,
    value: Option<String>,
    /// Typed metadata line: quantity kind and retained sample count.
    meta: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResultArtifactKind {
    Scalar,
    Array,
    EventStream,
    Contribution,
}

impl ResultArtifactKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Scalar => "Scalar",
            Self::Array => "Array",
            Self::EventStream => "Event stream",
            Self::Contribution => "Contribution",
        }
    }
}

struct ResultArtifact {
    identity: ResultArtifactPresentationKey,
    name: String,
    kind: ResultArtifactKind,
    unit: String,
    meta: String,
    value: Option<String>,
    viewer: crate::workbench::ResultViewer,
}

#[derive(Clone, Copy)]
enum ResultSignalVirtualRow {
    Dataset {
        run: usize,
        expanded: bool,
    },
    DatasetOmission {
        run: usize,
    },
    Analysis {
        run: usize,
        analysis: usize,
        expanded: bool,
    },
    Omission {
        run: usize,
        analysis: usize,
    },
    Signal {
        run: usize,
        analysis: usize,
        signal: usize,
    },
    Artifact {
        run: usize,
        analysis: usize,
        artifact: usize,
    },
}

fn retained_result_artifacts(
    analysis: &crate::state::AnalysisResult,
    presentation_key: AnalysisPresentationKey,
) -> Vec<ResultArtifact> {
    use crate::state::{AnalysisResultFamilyMetadata, AnalysisResultPayload};
    use crate::workbench::ResultViewer;

    let mut artifacts = Vec::new();
    let mut push = |canonical: &str,
                    name: String,
                    kind: ResultArtifactKind,
                    count: usize,
                    unit: &str,
                    value: Option<String>,
                    viewer: ResultViewer| {
        let shape = match kind {
            ResultArtifactKind::Scalar => "scalar".to_owned(),
            ResultArtifactKind::Array => format!("array[{count}]"),
            ResultArtifactKind::EventStream => format!("events[{count}]"),
            ResultArtifactKind::Contribution => format!("contributions[{count}]"),
        };
        let unit_dimension = unit.to_owned();
        let unit_suffix = if unit.is_empty() {
            String::new()
        } else {
            format!(" / {unit}")
        };
        artifacts.push(ResultArtifact {
            identity: ResultArtifactPresentationKey::new(presentation_key, canonical),
            name,
            kind,
            unit: unit_dimension,
            meta: format!("{} / {shape}{unit_suffix}", kind.label()),
            value,
            viewer,
        });
    };

    if let Some(dc) = &analysis.dc_op {
        for (canonical, name, values, unit) in [
            (
                "dc-op/node-voltages",
                "Node voltages",
                dc.node_voltages.as_slice(),
                "V",
            ),
            (
                "dc-op/branch-currents",
                "Branch currents",
                dc.branch_currents.as_slice(),
                "A",
            ),
            (
                "dc-op/power-dissipation",
                "Device power dissipation",
                dc.power_dissipation.as_slice(),
                "W",
            ),
        ] {
            if !values.is_empty() {
                push(
                    canonical,
                    name.to_owned(),
                    ResultArtifactKind::Array,
                    values.len(),
                    unit,
                    None,
                    ResultViewer::Op,
                );
            }
        }
    }
    if let Some(report) = &analysis.device_op
        && !report.entries.is_empty()
    {
        push(
            "dc-op/device-report",
            "Device operating points".to_owned(),
            ResultArtifactKind::Array,
            report.entries.len(),
            "",
            None,
            ResultViewer::Op,
        );
    }
    if let Some(noise) = &analysis.noise_summary {
        if !noise.rows.is_empty() {
            push(
                "noise/contributions",
                "Integrated noise contributors".to_owned(),
                ResultArtifactKind::Contribution,
                noise.rows.len(),
                "V^2",
                None,
                ResultViewer::NoiseContrib,
            );
        }
        for (canonical, name, value) in [
            ("noise/output-rms", "Output noise RMS", noise.total_rms),
            (
                "noise/input-rms",
                "Input-referred noise RMS",
                noise.input_rms,
            ),
        ] {
            if let Some(value) = value {
                push(
                    canonical,
                    name.to_owned(),
                    ResultArtifactKind::Scalar,
                    1,
                    "V",
                    Some(crate::ui::plot::fmt_si(value, "V", 3)),
                    ResultViewer::NoiseContrib,
                );
            }
        }
    }
    if let Some(payload) = &analysis.result_payload {
        let (canonical, name, kind, count, value, viewer) = match payload {
            AnalysisResultPayload::OperatingPoint { mna_solution, .. } => (
                "payload/operating-point",
                "Operating-point execution evidence",
                ResultArtifactKind::Array,
                mna_solution.len(),
                None,
                ResultViewer::Op,
            ),
            AnalysisResultPayload::PoleZero { poles, zeros, gain } => (
                "payload/pole-zero",
                "Poles, zeros, and gain",
                ResultArtifactKind::Array,
                poles.len() + zeros.len() + 1,
                Some(format!("gain {:.6e}", gain)),
                ResultViewer::PoleZero,
            ),
            AnalysisResultPayload::Sensitivity { rows, .. } => (
                "payload/sensitivity",
                "Sensitivity coefficients",
                ResultArtifactKind::Array,
                rows.len(),
                None,
                ResultViewer::Contribution,
            ),
            AnalysisResultPayload::ScalarMeasurements { values } => (
                "payload/scalar-measurements",
                "Scalar result values",
                ResultArtifactKind::Scalar,
                values.len(),
                None,
                ResultViewer::Table,
            ),
            AnalysisResultPayload::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                ..
            } => (
                "payload/transfer-function",
                "Transfer-function scalars",
                ResultArtifactKind::Scalar,
                [gain, input_resistance, output_resistance]
                    .into_iter()
                    .filter(|value| value.is_some())
                    .count(),
                None,
                ResultViewer::TransferFunction,
            ),
            AnalysisResultPayload::Reliability { devices } => (
                "payload/reliability",
                "Reliability device evidence",
                ResultArtifactKind::Array,
                devices.len(),
                None,
                ResultViewer::Reliability,
            ),
            AnalysisResultPayload::Soa {
                evaluations,
                violations,
            } => (
                "payload/safe-operating-area",
                "Safe-operating-area evidence",
                ResultArtifactKind::Contribution,
                evaluations.len() + violations.len(),
                None,
                ResultViewer::Soa,
            ),
            AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
            } => (
                "payload/transient-events",
                "Committed mixed-signal event streams",
                ResultArtifactKind::EventStream,
                digital_traces
                    .iter()
                    .map(|trace| trace.points.len())
                    .sum::<usize>()
                    + real_traces
                        .iter()
                        .map(|trace| trace.points.len())
                        .sum::<usize>(),
                None,
                ResultViewer::Events,
            ),
        };
        push(canonical, name.to_owned(), kind, count, "", value, viewer);
    }
    if let Some(metadata) = &analysis.family_metadata {
        let (name, count, viewer) = match metadata {
            AnalysisResultFamilyMetadata::Parametric { sweep_values, .. } => (
                "Parametric family metadata",
                sweep_values.len(),
                ResultViewer::Waves,
            ),
            AnalysisResultFamilyMetadata::Corner { x_values, .. } => (
                "Corner family metadata",
                x_values.len(),
                ResultViewer::Waves,
            ),
            AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } => (
                "Monte Carlo source samples",
                variables
                    .iter()
                    .map(|variable| variable.samples.len())
                    .sum(),
                ResultViewer::Hist,
            ),
            AnalysisResultFamilyMetadata::Reliability { years } => (
                "Reliability checkpoints",
                years.len(),
                ResultViewer::Reliability,
            ),
            AnalysisResultFamilyMetadata::Optimization { iterations, .. } => (
                "Optimization iterations",
                iterations.len(),
                ResultViewer::Optimization,
            ),
            AnalysisResultFamilyMetadata::Soa { time } => {
                ("SOA time coordinates", time.len(), ResultViewer::Soa)
            }
            AnalysisResultFamilyMetadata::PeriodicNoise { .. } => {
                ("Periodic-noise authority", 1, ResultViewer::PhaseNoise)
            }
            AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm,
            } => (
                "Port reference impedances",
                reference_impedances_ohm.len(),
                ResultViewer::Smith,
            ),
        };
        push(
            "family/metadata",
            name.to_owned(),
            ResultArtifactKind::Array,
            count,
            "",
            None,
            viewer,
        );
    }
    for measurement in &analysis.measurements {
        push(
            &format!("measurement/{}", measurement.name),
            format!(".MEAS {}", measurement.name),
            ResultArtifactKind::Scalar,
            1,
            "",
            measurement.value.map(|value| format!("{value:.6e}")),
            ResultViewer::Specs,
        );
    }
    let mut counts = std::collections::HashMap::with_capacity(artifacts.len());
    for artifact in &artifacts {
        *counts.entry(artifact.identity.clone()).or_insert(0_usize) += 1;
    }
    artifacts.retain(|artifact| counts.get(&artifact.identity) == Some(&1));
    artifacts
}

fn show_virtualized_result_signals(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    runs: &[ResultRun],
    active_run: Option<usize>,
    active_analysis: Option<usize>,
    selected_trace: Option<&crate::workbench::documents::result_document::SelectedResultTrace>,
    selected_artifact: Option<&ResultArtifactPresentationKey>,
    ordered_visible: &[ResultBrowserSelectionKey],
    query: &str,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    // A row labels the conductor the design drew; its identity stays the deck
    // name the engine answered under.
    let notations = bus_notations(&app.state.workspace, &app.state.schematic);
    let mut rows = Vec::new();
    let mut signal_rows = 0_usize;
    for (run_position, run) in runs.iter().enumerate() {
        let run_expanded = !query.is_empty()
            || result_browser_dataset_expanded(
                ui.ctx(),
                run.dataset_id,
                active_run == Some(run.run_index) || runs.len() == 1,
            );
        rows.push(ResultSignalVirtualRow::Dataset {
            run: run_position,
            expanded: run_expanded,
        });
        if !run_expanded {
            continue;
        }
        if run.analyses.is_empty() {
            rows.push(ResultSignalVirtualRow::DatasetOmission { run: run_position });
            continue;
        }
        for (analysis_position, analysis) in run.analyses.iter().enumerate() {
            let analysis_active = active_run == Some(run.run_index)
                && active_analysis == Some(analysis.analysis_index);
            let expanded = !query.is_empty()
                || result_browser_group_expanded(
                    ui.ctx(),
                    analysis.presentation_key,
                    analysis_active,
                );
            rows.push(ResultSignalVirtualRow::Analysis {
                run: run_position,
                analysis: analysis_position,
                expanded,
            });
            if !expanded {
                continue;
            }
            if analysis.signals.is_empty() && analysis.artifacts.is_empty() {
                rows.push(ResultSignalVirtualRow::Omission {
                    run: run_position,
                    analysis: analysis_position,
                });
            } else {
                signal_rows += analysis.signals.len() + analysis.artifacts.len();
                rows.extend((0..analysis.signals.len()).map(|signal| {
                    ResultSignalVirtualRow::Signal {
                        run: run_position,
                        analysis: analysis_position,
                        signal,
                    }
                }));
                rows.extend((0..analysis.artifacts.len()).map(|artifact| {
                    ResultSignalVirtualRow::Artifact {
                        run: run_position,
                        analysis: analysis_position,
                        artifact,
                    }
                }));
            }
        }
    }

    let row_height = responsive_result_control_height(
        RESULT_QUANTITY_ROW_HEIGHT.max(RESULT_ANALYSIS_HEAD_HEIGHT),
        Tokens::get(ui.ctx()).metrics.ctl_h,
    );
    let pinned_footer_height = if selected_trace.is_some() || selected_artifact.is_some() {
        122.0
    } else {
        34.0
    };
    let list_height = (ui.available_height() - pinned_footer_height)
        .max(row_height)
        .min(row_height * RESULT_BROWSER_RENDER_WINDOW_ROWS as f32);
    let mut rendered_signal_rows = 0_usize;
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .max_height(list_height)
        .show_viewport(ui, |ui, viewport| {
            let spacing = ui.spacing().item_spacing.y;
            let row_stride = row_height + spacing;
            ui.set_height((row_stride * rows.len() as f32 - spacing).max(0.0));
            let mut visible_start = (viewport.min.y / row_stride).floor().max(0.0) as usize;
            let visible_end =
                ((viewport.max.y / row_stride).ceil() as usize + 1).min(rows.len());
            if visible_end == rows.len() {
                let visible_count = visible_end.saturating_sub(visible_start);
                visible_start = rows.len().saturating_sub(visible_count);
            }
            let (render_start, render_end) = if rows.len()
                >= RESULT_BROWSER_VIRTUALIZATION_THRESHOLD
            {
                (
                    visible_start.saturating_sub(RESULT_BROWSER_OVERSCAN_ROWS),
                    visible_end
                        .saturating_add(RESULT_BROWSER_OVERSCAN_ROWS)
                        .min(rows.len()),
                )
            } else {
                (0, rows.len())
            };
            rendered_signal_rows += (render_start..render_end)
                .filter(|index| {
                    matches!(
                        rows[*index],
                        ResultSignalVirtualRow::Signal { .. }
                            | ResultSignalVirtualRow::Artifact { .. }
                    )
                })
                .count();
            let y_min = ui.max_rect().top() + render_start as f32 * row_stride;
            let y_max = ui.max_rect().top() + render_end as f32 * row_stride;
            let render_rect =
                egui::Rect::from_x_y_ranges(ui.max_rect().x_range(), y_min..=y_max);
            ui.scope_builder(egui::UiBuilder::new().max_rect(render_rect), |ui| {
                ui.skip_ahead_auto_ids(render_start);
                for row in rows
                    .iter()
                    .take(render_end)
                    .skip(render_start)
                    .copied()
                {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), row_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| match row {
                        ResultSignalVirtualRow::Dataset { run, expanded } => {
                            let run = &runs[run];
                            let count = run
                                .analyses
                                .iter()
                                .map(|analysis| {
                                    analysis.signals.len() + analysis.artifacts.len()
                                })
                                .sum::<usize>();
                            let dataset_id = run.dataset_id.to_string();
                            let domain = format!(
                                "dataset {} · {}",
                                dataset_id.get(..8).unwrap_or(dataset_id.as_str()),
                                run.data_state.word,
                            );
                            let head = result_browser_analysis_head(
                                ui,
                                "DS",
                                &run.label,
                                &domain,
                                count,
                                count,
                                active_run == Some(run.run_index),
                                run.success,
                                expanded,
                            );
                            if head.clicked() {
                                set_result_browser_dataset_expanded(
                                    ui.ctx(),
                                    run.dataset_id,
                                    !expanded,
                                );
                            }
                            if head.has_focus() {
                                let collapse = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowLeft,
                                    )
                                });
                                let expand = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowRight,
                                    )
                                });
                                if (collapse && expanded) || (expand && !expanded) {
                                    set_result_browser_dataset_expanded(
                                        ui.ctx(),
                                        run.dataset_id,
                                        expand,
                                    );
                                }
                            }
                        }
                        ResultSignalVirtualRow::DatasetOmission { run } => {
                            let run = &runs[run];
                            result_browser_virtual_omission(
                                ui,
                                &format!(
                                    "No analysis in {} matches the analysis, kind, scope, and text filters.",
                                    run.label
                                ),
                                row_height,
                            );
                        }
                        ResultSignalVirtualRow::Analysis {
                            run,
                            analysis,
                            expanded,
                        } => {
                            let run = &runs[run];
                            let analysis = &run.analyses[analysis];
                            let analysis_active = active_run == Some(run.run_index)
                                && active_analysis == Some(analysis.analysis_index);
                            let shown = analysis.signals.len() + analysis.artifacts.len();
                            let head = result_browser_analysis_head(
                                ui,
                                analysis.short_label,
                                &analysis.label,
                                &analysis.domain,
                                shown,
                                analysis.total_signals,
                                analysis_active,
                                analysis.success,
                                expanded,
                            );
                            // The retained fraction is the head's whole
                            // statement about scope; its reason rides the
                            // tooltip rather than a word beside the number.
                            let head = if shown == analysis.total_signals {
                                head
                            } else {
                                head.on_hover_text(format!(
                                    "{shown} listed of {} retained · {}",
                                    analysis.total_signals, analysis.data_state.note
                                ))
                            };
                            if head.clicked() {
                                if query.is_empty() {
                                    set_result_browser_group_expanded(
                                        ui.ctx(),
                                        analysis.presentation_key,
                                        !expanded,
                                    );
                                }
                                select_result_analysis_by_key(app, analysis.presentation_key);
                            }
                            if head.has_focus() {
                                let collapse = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowLeft,
                                    )
                                });
                                let expand = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowRight,
                                    )
                                });
                                if (collapse && expanded) || (expand && !expanded) {
                                    set_result_browser_group_expanded(
                                        ui.ctx(),
                                        analysis.presentation_key,
                                        expand,
                                    );
                                }
                            }
                        }
                        ResultSignalVirtualRow::Omission { run, analysis } => {
                            let analysis = &runs[run].analyses[analysis];
                            let reason = if analysis.total_signals == 0
                                && analysis.domain == "scalar solution"
                            {
                                "This analysis retained a scalar solution, not a sampled series. Its values are in the OP viewer."
                            } else if analysis.total_signals == 0 {
                                "No waveform quantities were retained for this swept analysis. Place a probe or change Output selection, then run again."
                            } else {
                                "Every quantity of this analysis is filtered out by the current query, kind, or scope."
                            };
                            result_browser_virtual_omission(ui, reason, row_height);
                        }
                        ResultSignalVirtualRow::Signal {
                            run,
                            analysis,
                            signal,
                        } => {
                            let analysis = &runs[run].analyses[analysis];
                            let signal = &analysis.signals[signal];
                            let t = Tokens::get(ui.ctx());
                            let color = crate::workbench::documents::result_document::trace_color(
                                &signal.color,
                                t.color.traces[signal.waveform_index % t.color.traces.len()],
                            );
                            let signal_selected = selected_trace.is_some_and(|selected| {
                                selected.analysis_key() == analysis.presentation_key
                                    && selected.source_name() == signal.name
                            });
                            let responses = result_quantity_row(
                                ui,
                                result_signal_row_id(&ResultBrowserSelectionKey::Waveform(
                                    signal.identity.clone(),
                                )),
                                &notations.display(&signal.name),
                                &signal.meta,
                                signal.value.as_deref(),
                                color,
                                signal_selected,
                                signal.visible,
                                app.state.ui.results.is_favorite_signal(&signal.identity),
                                app.state.ui.results.is_checked_signal(&signal.identity),
                            );
                            if responses.check.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                update_result_browser_multi_selection(
                                    app,
                                    &ResultBrowserSelectionKey::Waveform(signal.identity.clone()),
                                    ordered_visible,
                                    modifiers,
                                );
                            }
                            if responses.visibility.clicked() {
                                toggle_result_signal_visibility(app, &signal.identity);
                            }
                            if responses
                                .favorite
                                .as_ref()
                                .is_some_and(|star| star.clicked())
                            {
                                app.state
                                    .ui
                                    .results
                                    .toggle_favorite_signal(signal.identity.clone());
                            }
                            if responses.selection.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                if modifiers.shift || modifiers.command || modifiers.ctrl {
                                    update_result_browser_multi_selection(
                                        app,
                                        &ResultBrowserSelectionKey::Waveform(
                                            signal.identity.clone(),
                                        ),
                                        ordered_visible,
                                        modifiers,
                                    );
                                } else {
                                    app.state
                                        .ui
                                        .results
                                        .set_browser_range_anchor(
                                            ResultBrowserSelectionKey::Waveform(
                                                signal.identity.clone(),
                                            ),
                                        );
                                }
                                select_result_signal_by_key(app, &signal.identity);
                            }
                            handle_result_signal_keyboard(
                                &responses.selection,
                                app,
                                &signal.identity,
                                ordered_visible,
                            );
                            let favorite = app
                                .state
                                .ui
                                .results
                                .is_favorite_signal(&signal.identity);
                            result_signal_context_menu(
                                &responses.selection,
                                app,
                                &signal.identity,
                                signal.visible,
                                favorite,
                            );
                        }
                        ResultSignalVirtualRow::Artifact {
                            run,
                            analysis,
                            artifact,
                        } => {
                            let analysis = &runs[run].analyses[analysis];
                            let artifact = &analysis.artifacts[artifact];
                            let selection_key =
                                ResultBrowserSelectionKey::Artifact(artifact.identity.clone());
                            let selected = selected_artifact == Some(&artifact.identity);
                            let favorite = app
                                .state
                                .ui
                                .results
                                .is_favorite_result_artifact(&artifact.identity);
                            let responses = result_artifact_row(
                                ui,
                                result_signal_row_id(&selection_key),
                                &artifact.name,
                                &artifact.meta,
                                artifact.value.as_deref(),
                                artifact.kind,
                                selected,
                                favorite,
                                app.state
                                    .ui
                                    .results
                                    .is_checked_result_artifact(&artifact.identity),
                            );
                            if responses.check.clicked() {
                                update_result_browser_multi_selection(
                                    app,
                                    &selection_key,
                                    ordered_visible,
                                    ui.input(|input| input.modifiers),
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
                                    .toggle_favorite_result_artifact(artifact.identity.clone());
                            }
                            if responses.selection.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                if modifiers.shift || modifiers.command || modifiers.ctrl {
                                    update_result_browser_multi_selection(
                                        app,
                                        &selection_key,
                                        ordered_visible,
                                        modifiers,
                                    );
                                } else {
                                    app.state
                                        .ui
                                        .results
                                        .set_browser_range_anchor(selection_key.clone());
                                }
                                select_result_artifact(app, &artifact.identity);
                            }
                            handle_result_artifact_keyboard(
                                &responses.selection,
                                app,
                                artifact,
                                ordered_visible,
                            );
                            result_artifact_context_menu(
                                &responses.selection,
                                app,
                                artifact,
                                favorite,
                            );
                        }
                        },
                    );
                }
            });
        });

    let loaded_signal_rows = runs
        .iter()
        .flat_map(|run| &run.analyses)
        .map(|analysis| analysis.total_signals)
        .sum::<usize>();
    // Matching, rendered and loaded stay distinguished, but they ride the
    // status band's count tooltip instead of a second band that restates the
    // number printed above it. The band renders before this pass, so it reads
    // these values on the next frame — the same one-frame contract the kind and
    // sort facets use.
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            results_browser_inventory_id(),
            (signal_rows, rendered_signal_rows, loaded_signal_rows),
        );
    });

    if signal_rows == 0 {
        muted(
            ui,
            if app.state.simulation.runs.is_empty() {
                "Run a simulation to create an immutable result dataset."
            } else {
                match scope {
                    ResultsBrowserScope::Favorites => "Star a signal to build the Favorites scope.",
                    ResultsBrowserScope::Recent => "Signals you select or show collect here.",
                    ResultsBrowserScope::All => {
                        "No signal of the active dataset matches this filter."
                    }
                }
            },
        );
        results_browser_clear_filters(ui, app, kind, scope);
    }
}

fn result_browser_virtual_omission(ui: &mut Ui, text: &str, row_height: f32) {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        [ui.available_width(), row_height],
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .truncate(),
    );
    response.on_hover_text(text);
}

fn result_browser_selection_summary(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&crate::workbench::documents::result_document::SelectedResultTrace>,
    selected_artifact: Option<&ResultArtifactPresentationKey>,
) {
    if let Some(selected) = selected {
        let Some(run) = app.state.simulation.active_run() else {
            return;
        };
        let Some((_, analysis)) = selected.analysis_key().resolve(run) else {
            return;
        };
        let Some(waveform) = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == selected.source_name())
        else {
            return;
        };
        result_browser_selection_rows(
            ui,
            [
                ("Quantity", waveform.name.clone()),
                ("Analysis", analysis.label.clone()),
                ("Samples", waveform.y.len().to_string()),
                ("Dataset", format!("Run {}", run.id)),
            ],
        );
        return;
    }

    let Some(selected_artifact) = selected_artifact else {
        return;
    };
    let Some((run_index, _, analysis)) = selected_artifact.resolve(&app.state.simulation.runs)
    else {
        return;
    };
    let Some(artifact) = retained_result_artifacts(analysis, selected_artifact.analysis())
        .into_iter()
        .find(|artifact| artifact.identity == *selected_artifact)
    else {
        return;
    };
    let run = &app.state.simulation.runs[run_index];
    result_browser_selection_rows(
        ui,
        [
            ("Quantity", artifact.name),
            ("Analysis", analysis.label.clone()),
            ("Type", artifact.kind.label().to_owned()),
            ("Dataset", format!("Run {}", run.id)),
        ],
    );
}

fn result_browser_selection_rows<const N: usize>(ui: &mut Ui, rows: [(&str, String); N]) {
    ui.add_space(6.0);
    section_header(ui, "Selection", None);
    let t = Tokens::get(ui.ctx());
    for (label, value) in rows {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_MICRO, FontWeight::Medium))
                        .color(t.color.text_dim),
                );
            });
        });
    }
}

fn result_browser_precision_note(ui: &mut Ui) {
    ui.add_space(6.0);
    muted(
        ui,
        "Preview values are formatted display metadata. Copy, measurement, comparison, and export resolve stored values from the immutable dataset.",
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResultCompletenessClass {
    Complete,
    Partial,
    Loading,
    Failed,
    Cancelled,
}

fn result_run_operational_label(run: &crate::state::SimulationRun) -> &'static str {
    use crate::state::SimulationRunLifecycle;

    match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy / unknown",
        SimulationRunLifecycle::Preparing => "loading / preparing",
        SimulationRunLifecycle::Running => "loading",
        SimulationRunLifecycle::Cancelling => "loading / cancelling",
        SimulationRunLifecycle::Completed if run.success => "ready",
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "cancelled",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

impl ResultCompletenessClass {
    const fn label(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Loading => "loading",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    /// Worst first, so a rollup over several analyses can take the minimum.
    const fn severity_rank(self) -> u8 {
        match self {
            Self::Failed => 0,
            Self::Cancelled => 1,
            Self::Loading => 2,
            Self::Partial => 3,
            Self::Complete => 4,
        }
    }
}

fn result_analysis_completeness(
    run: &crate::state::SimulationRun,
    analysis: &crate::state::AnalysisResult,
) -> ResultCompletenessClass {
    use crate::state::SimulationRunLifecycle;

    if analysis.is_live_partial() {
        return ResultCompletenessClass::Partial;
    }
    match run.lifecycle {
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => ResultCompletenessClass::Loading,
        SimulationRunLifecycle::Completed if analysis.success => ResultCompletenessClass::Complete,
        SimulationRunLifecycle::Aborted => ResultCompletenessClass::Cancelled,
        SimulationRunLifecycle::LegacyUnknown if analysis.success => {
            ResultCompletenessClass::Partial
        }
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Interrupted
        | SimulationRunLifecycle::LegacyUnknown => ResultCompletenessClass::Failed,
    }
}

/// The binding's completeness: the worst reading among the analyses it retains,
/// so a dataset row never reads complete over a failed or still-loading one.
fn result_run_completeness(run: &crate::state::SimulationRun) -> ResultCompletenessClass {
    run.analyses
        .iter()
        .map(|analysis| result_analysis_completeness(run, analysis))
        .min_by_key(|class| class.severity_rank())
        .unwrap_or(ResultCompletenessClass::Complete)
}

fn result_analysis_browser_status(
    run: &crate::state::SimulationRun,
    analysis: &crate::state::AnalysisResult,
    evidence_valid: bool,
) -> (&'static str, &'static str) {
    let integrity = if evidence_valid {
        "integrity verified"
    } else {
        "corrupted"
    };
    (
        integrity,
        result_analysis_completeness(run, analysis).label(),
    )
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
fn result_quantity_kind_label(name: &str, unit: &str) -> &'static str {
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
    // default, so the panel opens as an index of the run's analyses. It is a
    // stroked chevron rather than a text glyph — the bundled faces carry no
    // U+2304, so an expanded head used to paint a missing-glyph box.
    if expanded {
        WorkbenchIcon::ChevronDown
    } else {
        WorkbenchIcon::ChevronRight
    }
    .paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 5.0 + 7.0, rect.center().y),
            egui::vec2(12.0, 12.0),
        ),
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
    // On a coarse pointer there is no hover to reveal anything with, so the
    // row's actions are simply present. Hiding them behind a state a finger
    // cannot produce would put plot membership and favouriting out of reach
    // on the tablet composition entirely.
    let hovered = ui.rect_contains_pointer(rect) || t.metrics.is_touch();
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
        WorkbenchIcon::StarFilled.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(name_end + 5.0 + 5.0, name_y),
                egui::vec2(10.0, 10.0),
            ),
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
        if favorite {
            WorkbenchIcon::StarFilled
        } else {
            WorkbenchIcon::Star
        }
        .paint(
            ui.painter(),
            egui::Rect::from_center_size(star_rect.center(), egui::vec2(13.0, 13.0)),
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

struct ArtifactRowResponses {
    selection: egui::Response,
    check: egui::Response,
    favorite: Option<egui::Response>,
}

fn result_artifact_row(
    ui: &mut Ui,
    id: egui::Id,
    name: &str,
    meta: &str,
    value: Option<&str>,
    kind: ResultArtifactKind,
    selected: bool,
    favorite: bool,
    checked: bool,
) -> ArtifactRowResponses {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_height = responsive_result_control_height(RESULT_QUANTITY_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let hovered = ui.rect_contains_pointer(rect) || t.metrics.is_touch();
    let check_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2((rect.left() + 26.0).min(rect.right()), rect.bottom()),
    );
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(check_rect.right(), rect.top()),
        rect.right_bottom(),
    );
    let check = ui.interact(check_rect, id.with("check"), egui::Sense::click());
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
        return ArtifactRowResponses {
            selection,
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
    let box_rect = egui::Rect::from_center_size(check_rect.center(), egui::vec2(13.0, 13.0));
    ui.painter().rect_filled(box_rect, 3.0, c.bg_panel);
    ui.painter().rect_stroke(
        box_rect,
        3.0,
        egui::Stroke::new(1.0, if checked { c.accent } else { c.border_strong }),
        egui::StrokeKind::Inside,
    );
    if checked {
        ui.painter()
            .rect_filled(box_rect.shrink(3.0), 1.0, c.accent);
    }
    let name_y = rect.top() + 11.0;
    let meta_y = rect.top() + 25.0;
    let glyph = match kind {
        ResultArtifactKind::Scalar => "#",
        ResultArtifactKind::Array => "[]",
        ResultArtifactKind::EventStream => ">",
        ResultArtifactKind::Contribution => "S",
    };
    ui.painter().text(
        egui::pos2(selection_rect.left(), name_y),
        egui::Align2::LEFT_CENTER,
        glyph,
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        c.accent,
    );
    let name_left = selection_rect.left() + 18.0;
    let name_end = ui
        .painter()
        .text(
            egui::pos2(name_left, name_y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text,
        )
        .right();
    if favorite {
        ui.painter().text(
            egui::pos2(name_end + 5.0, name_y),
            egui::Align2::LEFT_CENTER,
            "*",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.warn.gamma_multiply(0.72),
        );
    }
    let favorite_response = hovered.then(|| {
        let star_rect = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 14.0, rect.center().y),
            egui::vec2(26.0, 26.0),
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
        ui.painter().text(
            star_rect.center(),
            egui::Align2::CENTER_CENTER,
            if favorite { "*" } else { "+" },
            theme::mono(tokens::FS_1, FontWeight::Regular),
            if favorite { c.warn } else { c.text_faint },
        );
        theme::paint_focus_ring(ui, &response, star_rect);
        response
    });
    if !hovered && let Some(value) = value {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, name_y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        );
    }
    ui.painter().text(
        egui::pos2(name_left, meta_y),
        egui::Align2::LEFT_CENTER,
        meta,
        theme::sans(tokens::FS_MICRO, FontWeight::Medium),
        c.text_faint,
    );
    theme::paint_focus_ring(ui, &check, check_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    ArtifactRowResponses {
        selection: selection.on_hover_text(format!("Inspect {name}")),
        check: check.on_hover_text(if checked {
            format!("Deselect {name}")
        } else {
            format!("Select {name} for a batch action")
        }),
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
fn update_result_browser_multi_selection(
    app: &mut RSpiceApp,
    target: &ResultBrowserSelectionKey,
    ordered_visible: &[ResultBrowserSelectionKey],
    modifiers: egui::Modifiers,
) {
    if modifiers.shift {
        app.state
            .ui
            .results
            .select_checked_result_range(target, ordered_visible);
    } else {
        app.state
            .ui
            .results
            .toggle_checked_result_quantity(target.clone());
        app.state
            .ui
            .results
            .set_browser_range_anchor(target.clone());
    }
}

fn result_signal_row_id(key: &ResultBrowserSelectionKey) -> egui::Id {
    egui::Id::new(("workbench.result-browser.signal", key))
}

fn handle_result_signal_keyboard(
    response: &egui::Response,
    app: &mut RSpiceApp,
    current: &SourceWaveformPresentationKey,
    ordered_visible: &[ResultBrowserSelectionKey],
) {
    if !response.has_focus() {
        return;
    }
    let modifiers = response.ctx.input(|input| input.modifiers);
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Space))
    {
        update_result_browser_multi_selection(
            app,
            &ResultBrowserSelectionKey::Waveform(current.clone()),
            ordered_visible,
            modifiers,
        );
        return;
    }
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
    {
        open_signal_table(app, current, false);
        return;
    }
    let direction = if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowUp)
    }) {
        -1_isize
    } else if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowDown)
    }) {
        1_isize
    } else {
        return;
    };
    let current_key = ResultBrowserSelectionKey::Waveform(current.clone());
    let Some(index) = ordered_visible.iter().position(|key| key == &current_key) else {
        return;
    };
    let next_index = index
        .saturating_add_signed(direction)
        .min(ordered_visible.len().saturating_sub(1));
    let Some(next) = ordered_visible.get(next_index) else {
        return;
    };
    if modifiers.shift {
        app.state
            .ui
            .results
            .select_checked_result_range(next, ordered_visible);
    } else {
        app.state.ui.results.set_browser_range_anchor(next.clone());
    }
    select_result_browser_key(app, next);
    response
        .ctx
        .memory_mut(|memory| memory.request_focus(result_signal_row_id(next)));
}

fn handle_result_artifact_keyboard(
    response: &egui::Response,
    app: &mut RSpiceApp,
    artifact: &ResultArtifact,
    ordered_visible: &[ResultBrowserSelectionKey],
) {
    if !response.has_focus() {
        return;
    }
    let current = ResultBrowserSelectionKey::Artifact(artifact.identity.clone());
    let modifiers = response.ctx.input(|input| input.modifiers);
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Space))
    {
        update_result_browser_multi_selection(app, &current, ordered_visible, modifiers);
        return;
    }
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
    {
        open_result_artifact(app, artifact, false);
        return;
    }
    let direction = if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowUp)
    }) {
        -1_isize
    } else if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowDown)
    }) {
        1_isize
    } else {
        return;
    };
    let Some(index) = ordered_visible.iter().position(|key| key == &current) else {
        return;
    };
    let next_index = index
        .saturating_add_signed(direction)
        .min(ordered_visible.len().saturating_sub(1));
    let Some(next) = ordered_visible.get(next_index) else {
        return;
    };
    if modifiers.shift {
        app.state
            .ui
            .results
            .select_checked_result_range(next, ordered_visible);
    } else {
        app.state.ui.results.set_browser_range_anchor(next.clone());
    }
    select_result_browser_key(app, next);
    response
        .ctx
        .memory_mut(|memory| memory.request_focus(result_signal_row_id(next)));
}

fn set_checked_signal_visibility(
    app: &mut RSpiceApp,
    checked: &std::collections::HashSet<ResultBrowserSelectionKey>,
    visible: bool,
) {
    let Some(run_index) = app.state.simulation.active_run_idx else {
        return;
    };
    let targets = checked
        .iter()
        .filter_map(|key| {
            let key = key.waveform()?;
            let (target_run, analysis_index, waveform_index, waveform) =
                key.resolve(&app.state.simulation.runs)?;
            let currently_visible = app
                .state
                .ui
                .results
                .waveform_visibility(key, waveform.visible);
            (target_run == run_index && currently_visible != visible)
                .then_some((analysis_index, waveform_index))
        })
        .collect::<Vec<_>>();
    for (analysis_index, waveform_index) in targets {
        crate::workbench::documents::result_document::toggle_visibility(
            &mut app.state,
            analysis_index,
            waveform_index,
        );
    }
}

fn toggle_result_signal_visibility(
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
) -> bool {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&app.state.simulation.runs)
    else {
        return false;
    };
    if app.state.simulation.active_run_idx != Some(run_index) {
        return false;
    }
    crate::workbench::documents::result_document::toggle_visibility(
        &mut app.state,
        analysis_index,
        waveform_index,
    );
    true
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
    app.state.ui.results.selected_result_artifact = None;
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
    app.state.ui.results.selected_result_artifact = None;
    true
}

fn select_result_analysis_by_key(app: &mut RSpiceApp, key: AnalysisPresentationKey) -> bool {
    let Some((run_index, analysis_index)) =
        app.state
            .simulation
            .runs
            .iter()
            .enumerate()
            .find_map(|(run_index, run)| {
                key.resolve(run)
                    .map(|(analysis_index, _)| (run_index, analysis_index))
            })
    else {
        return false;
    };
    select_result_analysis(app, run_index, analysis_index)
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
    let recent = SourceWaveformPresentationKey::new(selected.analysis_key(), name);
    app.state.ui.results.selected_trace = Some(selected);
    app.state.ui.results.selected_result_artifact = None;
    // Selecting a trace is a deliberate act; feed the browser's Recent scope.
    app.state.ui.results.note_recent_signal(recent);
    true
}

fn select_result_signal_by_key(app: &mut RSpiceApp, key: &SourceWaveformPresentationKey) -> bool {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&app.state.simulation.runs)
    else {
        return false;
    };
    select_result_signal(app, run_index, analysis_index, waveform_index)
}

fn select_result_artifact(app: &mut RSpiceApp, key: &ResultArtifactPresentationKey) -> bool {
    let Some((run_index, analysis_index, _)) = key.resolve(&app.state.simulation.runs) else {
        return false;
    };
    if !select_result_analysis(app, run_index, analysis_index) {
        return false;
    }
    app.state.ui.results.selected_trace = None;
    app.state.ui.results.selected_result_artifact = Some(key.clone());
    app.state
        .ui
        .results
        .note_recent_result_artifact(key.clone());
    true
}

fn select_result_browser_key(app: &mut RSpiceApp, key: &ResultBrowserSelectionKey) -> bool {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => select_result_signal_by_key(app, key),
        ResultBrowserSelectionKey::Artifact(key) => select_result_artifact(app, key),
    }
}

fn result_artifact_context_menu(
    response: &egui::Response,
    app: &mut RSpiceApp,
    artifact: &ResultArtifact,
    favorite: bool,
) {
    let exact_error =
        crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
            &ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
            &app.state.simulation.runs,
        )
        .err();
    let exact_available = exact_error.is_none();
    let keyboard_open = response.has_focus()
        && response
            .ctx
            .input_mut(|input| input.consume_key(egui::Modifiers::SHIFT, egui::Key::F10));
    let popup = egui::Popup::context_menu(response);
    let popup = if keyboard_open {
        popup
            .open_memory(egui::SetOpenCommand::Bool(true))
            .anchor(response)
    } else {
        popup
    };
    popup.show(|ui| {
        let open = ui.add_enabled(
            exact_available,
            egui::Button::new("Open in active result pane"),
        );
        if let Some(error) = exact_error.as_deref() {
            open.clone().on_disabled_hover_text(error);
        }
        if open.clicked() {
            open_result_artifact(app, artifact, false);
            ui.close();
        }
        let add = ui.add_enabled(exact_available, egui::Button::new("Add to new pane..."));
        if let Some(error) = exact_error.as_deref() {
            add.clone().on_disabled_hover_text(error);
        }
        if add.clicked() {
            if select_result_artifact(app, &artifact.identity) {
                app.state.ui.results.viewer = artifact.viewer;
                crate::workbench::documents::visualization_studio::open(app);
                crate::workbench::documents::visualization_studio::open_add_pane(app);
            }
            ui.close();
        }
        let table = ui.add_enabled(
            exact_available,
            egui::Button::new("Open exact evidence table"),
        );
        if let Some(error) = exact_error.as_deref() {
            table.clone().on_disabled_hover_text(error);
        }
        if table.clicked() {
            if select_result_artifact(app, &artifact.identity) {
                Command::ResultViewer(crate::workbench::ResultViewer::Table).execute(app);
            }
            ui.close();
        }
        let compare_enabled = exact_available && Command::CompareResultDatasets.is_enabled(app);
        let compare = ui.add_enabled(
            compare_enabled,
            egui::Button::new("Compare selected with dataset..."),
        );
        if !compare_enabled {
            compare.clone().on_disabled_hover_text(
                exact_error.as_deref().unwrap_or(
                    "Retain a compatible second dataset and open a result document before comparing.",
                ),
            );
        }
        if compare.clicked() {
            select_result_artifact(app, &artifact.identity);
            Command::CompareResultDatasets.execute(app);
            ui.close();
        }
        ui.separator();
        if ui.button("Copy canonical name").clicked() {
            ui.ctx()
                .copy_text(artifact.identity.canonical_name().to_owned());
            ui.close();
        }
        if ui.button("Copy stable dataset path").clicked() {
            match result_artifact_stable_path(&artifact.identity, &app.state.simulation.runs) {
                Ok(path) => ui.ctx().copy_text(path),
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
        let copy_exact = ui.add_enabled(
            exact_available,
            egui::Button::new("Copy exact typed value"),
        );
        if let Some(error) = exact_error.as_deref() {
            copy_exact.clone().on_disabled_hover_text(error);
        }
        if copy_exact.clicked() {
            match exact_result_artifact_text(&artifact.identity, &app.state.simulation.runs) {
                Ok(value) => ui.ctx().copy_text(value),
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
        if ui.button("Inspect metadata and provenance").clicked() {
            select_result_artifact(app, &artifact.identity);
            app.state.workbench.inspector_visible = true;
            ui.close();
        }
        if ui
            .button(if favorite {
                "Remove from favorites"
            } else {
                "Add to favorites"
            })
            .clicked()
        {
            app.state
                .ui
                .results
                .toggle_favorite_result_artifact(artifact.identity.clone());
            ui.close();
        }
        let export = ui.add_enabled(
            exact_available,
            egui::Button::new("Export selected exact evidence..."),
        );
        if let Some(error) = exact_error.as_deref() {
            export.clone().on_disabled_hover_text(error);
        }
        if export.clicked() {
            open_result_artifact(app, artifact, true);
            ui.close();
        }
        ui.add_enabled(false, egui::Button::new("Show on schematic"))
            .on_disabled_hover_text(
                "This typed producer artifact is not a schematic conductor. Open one of its source quantities to cross-probe.",
            );
        if ui.button("Reveal producer log").clicked() {
            match result_artifact_stable_path(&artifact.identity, &app.state.simulation.runs) {
                Ok(path) => reveal_producer_log(app, path, artifact.identity.canonical_name()),
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
    });
}

fn open_result_artifact(app: &mut RSpiceApp, artifact: &ResultArtifact, request_export: bool) {
    if select_result_artifact(app, &artifact.identity) {
        if request_export {
            app.state.ui.export_result_quantities_requested =
                Some(vec![ResultBrowserSelectionKey::Artifact(
                    artifact.identity.clone(),
                )]);
        } else {
            Command::ResultViewer(artifact.viewer).execute(app);
        }
    }
}

fn result_signal_context_menu(
    response: &egui::Response,
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
    visible: bool,
    favorite: bool,
) {
    let key = key.clone();
    let exact_error =
        crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
            &ResultBrowserSelectionKey::Waveform(key.clone()),
            &app.state.simulation.runs,
        )
        .err();
    let exact_available = exact_error.is_none();
    let keyboard_open = response.has_focus()
        && response
            .ctx
            .input_mut(|input| input.consume_key(egui::Modifiers::SHIFT, egui::Key::F10));
    let popup = egui::Popup::context_menu(response);
    let popup = if keyboard_open {
        popup
            .open_memory(egui::SetOpenCommand::Bool(true))
            .anchor(response)
    } else {
        popup
    };
    popup.show(|ui| {
        let membership = ui.add_enabled(
            exact_available,
            egui::Button::new(if visible {
                "Remove from active pane"
            } else {
                "Add to active pane"
            }),
        );
        if let Some(error) = exact_error.as_deref() {
            membership.clone().on_disabled_hover_text(error);
        }
        if membership.clicked() {
            toggle_result_signal_visibility(app, &key);
            ui.close();
        }
        let add = ui.add_enabled(exact_available, egui::Button::new("Add to new pane..."));
        if let Some(error) = exact_error.as_deref() {
            add.clone().on_disabled_hover_text(error);
        }
        if add.clicked() {
            if select_result_signal_by_key(app, &key) {
                crate::workbench::documents::visualization_studio::open(app);
                crate::workbench::documents::visualization_studio::open_add_pane(app);
            }
            ui.close();
        }
        let create = ui.add_enabled(
            exact_available,
            egui::Button::new("Create result document..."),
        );
        if let Some(error) = exact_error.as_deref() {
            create.clone().on_disabled_hover_text(error);
        }
        if create.clicked() {
            select_result_signal_by_key(app, &key);
            crate::workbench::documents::result_document::open_create_document(app);
            ui.close();
        }
        ui.separator();
        let table = ui.add_enabled(
            exact_available,
            egui::Button::new("Open exact sample table"),
        );
        if let Some(error) = exact_error.as_deref() {
            table.clone().on_disabled_hover_text(error);
        }
        if table.clicked() {
            open_signal_table(app, &key, false);
            ui.close();
        }
        let export = ui.add_enabled(
            exact_available,
            egui::Button::new("Export source analysis (CSV)..."),
        );
        if let Some(error) = exact_error.as_deref() {
            export.clone().on_disabled_hover_text(error);
        }
        if export.clicked() {
            open_signal_table(app, &key, true);
            ui.close();
        }
        let compare_enabled = exact_available && Command::CompareResultDatasets.is_enabled(app);
        let compare = ui.add_enabled(
            compare_enabled,
            egui::Button::new("Compare with dataset..."),
        );
        if !compare_enabled {
            compare.clone().on_disabled_hover_text(
                exact_error.as_deref().unwrap_or(
                    "Retain a compatible second dataset and open a result document before comparing.",
                ),
            );
        }
        if compare.clicked() {
            select_result_signal_by_key(app, &key);
            Command::CompareResultDatasets.execute(app);
            ui.close();
        }
        ui.separator();
        if ui.button("Copy canonical name").clicked() {
            if let Some((.., waveform)) = key.resolve(&app.state.simulation.runs) {
                ui.ctx().copy_text(waveform.name.clone());
            }
            ui.close();
        }
        if ui.button("Copy stable dataset path").clicked() {
            match result_signal_stable_path(&key, &app.state.simulation.runs) {
                Ok(path) => ui.ctx().copy_text(path),
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
        let copy_last = ui.add_enabled(
            exact_available,
            egui::Button::new("Copy exact last sample"),
        );
        if let Some(error) = exact_error.as_deref() {
            copy_last.clone().on_disabled_hover_text(error);
        }
        if copy_last.clicked() {
            match exact_result_signal_last_sample(&key, &app.state.simulation.runs) {
                Ok(value) => ui.ctx().copy_text(value),
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
        let retained_samples = key
            .resolve(&app.state.simulation.runs)
            .map_or(0, |(.., waveform)| waveform.x.len().max(waveform.y.len()));
        let may_copy_samples =
            exact_available && retained_samples <= RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT;
        let copy_samples = ui.add_enabled(
            may_copy_samples,
            egui::Button::new("Copy exact samples (TSV)"),
        );
        if !may_copy_samples {
            copy_samples.clone().on_disabled_hover_text(
                exact_error.clone().unwrap_or_else(|| format!(
                    "This quantity retains {retained_samples} samples. Clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT}; use CSV export for larger evidence."
                )),
            );
        }
        if copy_samples.clicked() {
            match exact_result_signal_tsv(&key, &app.state.simulation.runs) {
                Ok(tsv) => ui.ctx().copy_text(tsv),
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
        ui.separator();
        if ui.button("Inspect metadata and provenance").clicked() {
            select_result_signal_by_key(app, &key);
            app.state.workbench.inspector_visible = true;
            ui.close();
        }
        if ui
            .button(if favorite {
                "Remove from favorites"
            } else {
                "Add to favorites"
            })
            .clicked()
        {
            app.state.ui.results.toggle_favorite_signal(key.clone());
            ui.close();
        }
        let signal_name = key
            .resolve(&app.state.simulation.runs)
            .map(|(.., waveform)| waveform.name.clone());
        let cross_probe = ui.add_enabled(
            signal_name.is_some(),
            egui::Button::new("Show on schematic"),
        );
        if signal_name.is_none() {
            cross_probe
                .clone()
                .on_disabled_hover_text("The immutable source quantity is no longer available.");
        }
        if cross_probe.clicked() {
            if let Some(signal) = signal_name {
                show_signal_on_schematic(ui.ctx(), app, &signal);
            }
            ui.close();
        }
        if ui.button("Reveal producer log").clicked() {
            match result_signal_stable_path(&key, &app.state.simulation.runs) {
                Ok(path) => {
                    let quantity = key
                        .resolve(&app.state.simulation.runs)
                        .map(|(.., waveform)| waveform.name.clone())
                        .unwrap_or_default();
                    reveal_producer_log(app, path, &quantity);
                }
                Err(error) => result_browser_action_error(ui.ctx(), app, error),
            }
            ui.close();
        }
    });
}

fn open_signal_table(
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
    request_export: bool,
) {
    if select_result_signal_by_key(app, key) {
        if request_export {
            app.state.ui.export_result_quantities_requested =
                Some(vec![ResultBrowserSelectionKey::Waveform(key.clone())]);
        } else {
            if let Some(selected) = &app.state.ui.results.selected_trace {
                let (analysis, column) = selected.table_binding();
                app.state.ui.results.table.analysis = Some(analysis);
                app.state.ui.results.table.columns = vec![column];
            }
            Command::ResultViewer(crate::workbench::ResultViewer::Table).execute(app);
        }
    }
}

fn result_browser_action_error(ctx: &egui::Context, app: &mut RSpiceApp, error: String) {
    app.state
        .ui
        .toasts
        .warn_with_title(ctx, "Result action unavailable", error.clone());
    app.state
        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
}

fn show_signal_on_schematic(ctx: &egui::Context, app: &mut RSpiceApp, signal: &str) {
    match crate::schematic::view::select_signal_conductor(&mut app.state, signal) {
        Ok(net) => {
            Command::OpenWorkspace(Workspace::Design).execute(app);
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Selected conductor {net} from {signal}."
                )));
        }
        Err(error) => {
            let message = error.message(signal);
            app.state
                .ui
                .toasts
                .warn_with_title(ctx, "Cannot cross-probe", message.clone());
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
        }
    }
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
            show_signal_on_schematic(ui.ctx(), app, &signal);
            ui.close();
        }
    });
}

fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_filter()
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
                .and_then(|plan| app.state.workspace.plan_data(plan.id()))
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
            app.state.select_model_library(&name);
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
mod tests;
