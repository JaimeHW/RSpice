//! Context toolbar.  Tool groups change with the canonical workspace while
//! global run state remains visible at the trailing edge.

use egui::containers::menu::MenuButton;
use egui::{Align, Context, Frame, Layout, Panel, Ui, Vec2};

use crate::product::{ContentDigest, ObjectRevision};
use crate::state::{ComponentType, Tool, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;

use super::super::RouteTransitionSource;
use super::super::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, Workspace};
use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::lifecycle::session::SymbolTool;

const TOOLBAR_CONTEXT_GAP: f32 = 3.0;
const DESIGN_DIRECT_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, &str); 10] = [
    (Command::SelectTool, WorkbenchIcon::Select, "Select (Esc)"),
    (
        Command::PlaceInstance,
        WorkbenchIcon::Instance,
        "Place instance",
    ),
    (Command::PlaceWire, WorkbenchIcon::Wire, "Draw wire"),
    (Command::PlaceBus, WorkbenchIcon::Bus, "Draw bus"),
    (Command::PlaceBusTap, WorkbenchIcon::BusTap, "Place bus tap"),
    (
        Command::PlaceJunction,
        WorkbenchIcon::Junction,
        "Place junction",
    ),
    (Command::PlaceLabel, WorkbenchIcon::Label, "Net label"),
    (Command::PlacePin, WorkbenchIcon::Pin, "Place pin or port"),
    (Command::PlaceProbe, WorkbenchIcon::Probe, "Probe signal"),
    (
        Command::PlaceText,
        WorkbenchIcon::Text,
        "Place text or note",
    ),
];

/// Authoring tools of the symbol document, in the order the editor's
/// keyboard shortcuts declare them. A symbol cellview is a design document,
/// so its tools live in the shared workspace toolbar rather than in a bar
/// of the editor's own.
const SYMBOL_DIRECT_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, SymbolTool); 8] = [
    (
        Command::SelectTool,
        WorkbenchIcon::Select,
        SymbolTool::Select,
    ),
    (
        Command::SymbolPinTool,
        WorkbenchIcon::Probe,
        SymbolTool::PlacePin,
    ),
    (
        Command::SymbolPolylineTool,
        WorkbenchIcon::Wire,
        SymbolTool::Line,
    ),
    (
        Command::SymbolRectangleTool,
        WorkbenchIcon::Grid,
        SymbolTool::Rectangle,
    ),
    (
        Command::SymbolCircleTool,
        WorkbenchIcon::Target,
        SymbolTool::Circle,
    ),
    (
        Command::SymbolArcTool,
        WorkbenchIcon::Refresh,
        SymbolTool::Arc,
    ),
    (
        Command::SymbolPolygonTool,
        WorkbenchIcon::Layers,
        SymbolTool::Polygon,
    ),
    (
        Command::SymbolTextTool,
        WorkbenchIcon::Label,
        SymbolTool::Text,
    ),
];

const MODELS_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, Option<&str>); 4] = [
    (Command::Save, WorkbenchIcon::Save, None),
    (
        Command::PdkSettings,
        WorkbenchIcon::Add,
        Some("Add library"),
    ),
    (Command::RescanModelLibraries, WorkbenchIcon::Refresh, None),
    (Command::CompileVerilogA, WorkbenchIcon::Netlist, None),
];

pub fn show(root: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let t = Tokens::get(ctx);
    let viewport_width = ctx.content_rect().width();
    let padding = toolbar_horizontal_padding(viewport_width);
    let outer_gap = toolbar_outer_gap(viewport_width);
    let show_simulation_context = app.state.project_lifecycle.project_open
        && matches!(
            app.state.workbench.workspace,
            Workspace::Design | Workspace::Simulate
        );
    Panel::top("workbench.context_toolbar")
        .exact_size(layout.toolbar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(root, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            let toolbar = ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = outer_gap;
                ui.add_space(padding);
                if layout.navigator_uses_drawer {
                    let navigator_toggle = icon_button(
                        ui,
                        WorkbenchIcon::Navigator,
                        "Open navigator",
                        app.state.workbench.drawer == Some(Drawer::Navigator),
                        panel_toggle_size(layout),
                    );
                    if navigator_toggle.clicked() {
                        let opening = app.state.workbench.drawer != Some(Drawer::Navigator);
                        remember_drawer_invoker(ui.ctx(), Drawer::Navigator, navigator_toggle.id);
                        if opening {
                            app.state.workbench.focus_navigator_search = true;
                            mark_drawer_focus_pending(ui.ctx(), Drawer::Navigator);
                        }
                        app.state.workbench.toggle_drawer(Drawer::Navigator);
                    }
                }
                if app.state.project_lifecycle.project_open {
                    global_document_tools(ui, app, layout);
                }

                // Context tools own the flexible middle lane. The mockup
                // keeps navigator/run/inspector controls reachable and lets
                // this lane scroll when a dense engineering toolbar exceeds
                // the tablet or phone budget.
                let run_width = reserved_run_control_width(ui, app, layout);
                let pvt_width = reserved_pvt_selector_width(ui, app);
                let context_width = context_tools_width(
                    ui.available_width(),
                    layout,
                    padding,
                    outer_gap,
                    show_simulation_context,
                    run_width,
                    pvt_width,
                );
                egui::ScrollArea::horizontal()
                    .id_salt("workbench.context_toolbar.tools")
                    .max_width(context_width)
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.x = TOOLBAR_CONTEXT_GAP;
                        ui.horizontal_centered(|ui| workspace_tools(ui, app, layout));
                    });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = TOOLBAR_CONTEXT_GAP;
                    ui.add_space(padding);
                    if layout.inspector_uses_drawer {
                        let inspector_toggle = icon_button(
                            ui,
                            WorkbenchIcon::Inspector,
                            "Open inspector",
                            app.state.workbench.drawer == Some(Drawer::Inspector),
                            panel_toggle_size(layout),
                        );
                        if inspector_toggle.clicked() {
                            let opening = app.state.workbench.drawer != Some(Drawer::Inspector);
                            remember_drawer_invoker(
                                ui.ctx(),
                                Drawer::Inspector,
                                inspector_toggle.id,
                            );
                            if opening {
                                mark_drawer_focus_pending(ui.ctx(), Drawer::Inspector);
                            }
                            app.state.workbench.toggle_drawer(Drawer::Inspector);
                        }
                    }
                    if show_simulation_context {
                        run_controls(ui, app, layout);
                        if layout.show_run_config_selector {
                            run_config_selector(ui, app, layout.run_control_height);
                        }
                        if layout.show_pvt_selector {
                            pvt_selector(ui, app, layout.toolbar_control_height);
                        }
                    }
                });
            });
            ui.ctx()
                .accesskit_node_builder(toolbar.response.id, |node| {
                    node.set_role(egui::accesskit::Role::Toolbar);
                    node.set_label("Context toolbar");
                });
        });
}

/// Save is the only document command with identical meaning in every
/// workspace. Keep it outside the horizontally scrolling context-tool lane so
/// it remains reachable even when a dense engineering toolbar overflows.
fn global_document_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let dirty = crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&app.state);
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        Command::Save,
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let state_label = if dirty {
        "unsaved changes in this project"
    } else {
        "no unsaved changes"
    };
    let label = if shortcut.is_empty() {
        format!("Save \u{00b7} {state_label}")
    } else {
        format!("Save ({shortcut}) \u{00b7} {state_label}")
    };
    let availability = Command::Save.availability(app);
    let enabled = availability == CommandAvailability::Available;
    let response = ui
        .add_enabled_ui(enabled, |ui| {
            icon_button(
                ui,
                WorkbenchIcon::Save,
                &label,
                dirty,
                toolbar_icon_button_size(layout),
            )
        })
        .inner;
    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.clicked() {
        Command::Save.execute(app);
    }
    context_separator(ui, layout);
}

fn toolbar_horizontal_padding(viewport_width: f32) -> f32 {
    if viewport_width <= 560.0 {
        3.0
    } else if viewport_width <= 820.0 {
        5.0
    } else {
        8.0
    }
}

fn drawer_owner(drawer: Drawer) -> &'static str {
    match drawer {
        Drawer::Navigator => "navigator",
        Drawer::Inspector => "inspector",
        Drawer::Workspaces => "workspaces",
    }
}

fn remember_drawer_invoker(ctx: &Context, drawer: Drawer, invoker: egui::Id) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.drawer.invoker", drawer_owner(drawer))),
            invoker,
        );
    });
}

fn mark_drawer_focus_pending(ctx: &Context, drawer: Drawer) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.drawer.focus_pending", drawer_owner(drawer))),
            true,
        );
    });
}

fn toolbar_outer_gap(viewport_width: f32) -> f32 {
    if viewport_width <= 560.0 { 2.0 } else { 5.0 }
}

fn reserved_run_control_width(ui: &egui::Ui, app: &RSpiceApp, layout: LayoutSpec) -> f32 {
    let label = if app.state.simulation.is_running {
        "Stop".to_owned()
    } else if layout.compact_shell {
        format!(
            "Run · {}",
            app.state.sim_setup.reference_pvt.process.short_name()
        )
    } else {
        "Run plan".to_owned()
    };
    let t = Tokens::get(ui.ctx());
    let label_width = ui
        .painter()
        .layout_no_wrap(
            label,
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        )
        .size()
        .x;
    let primary_width = (42.0 + label_width).max(67.0);
    let menu_width = if layout.run_control_height >= 44.0 {
        44.0
    } else {
        25.0
    };
    primary_width + menu_width
}

fn reserved_pvt_selector_width(ui: &egui::Ui, app: &RSpiceApp) -> f32 {
    let reference = app.state.sim_setup.reference_pvt;
    let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
    let label = format!(
        "{} · {}",
        reference.process.short_name(),
        pvt_temperature_label(reference.temperature_celsius, quantity_policy),
    );
    let t = Tokens::get(ui.ctx());
    ui.painter()
        .layout_no_wrap(
            label,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        )
        .size()
        .x
        + 42.0
}

fn context_tools_width(
    available_width: f32,
    layout: LayoutSpec,
    right_padding: f32,
    outer_gap: f32,
    show_simulation_context: bool,
    run_width: f32,
    pvt_width: f32,
) -> f32 {
    (available_width
        - trailing_controls_width(
            layout,
            outer_gap,
            show_simulation_context,
            run_width,
            pvt_width,
        )
        - right_padding)
        .max(1.0)
}

fn trailing_controls_width(
    layout: LayoutSpec,
    outer_gap: f32,
    show_simulation_context: bool,
    run_width: f32,
    pvt_width: f32,
) -> f32 {
    let mut width = 0.0;
    let mut controls: usize = 0;
    if show_simulation_context {
        width += run_width;
        controls += 1;
        if layout.show_pvt_selector {
            width += pvt_width;
            controls += 1;
        }
        if layout.show_run_config_selector {
            width += 190.0;
            controls += 1;
        }
    }
    if layout.inspector_uses_drawer {
        width += panel_toggle_size(layout).x;
        controls += 1;
    }
    width + (controls.saturating_sub(1) as f32 * TOOLBAR_CONTEXT_GAP) + outer_gap
}

fn workspace_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let workspace = app.state.workbench.workspace;
    ui.data_mut(|data| {
        data.insert_temp(projected_tool_count_id(), 0_usize);
    });
    if !app.state.project_lifecycle.project_open {
        no_project_tools(ui, app, layout);
        return;
    }
    if app.state.workbench.current_route().surface_id()
        == super::super::SurfaceId::VisualizationStudio
    {
        visualization_tools(ui, app, layout);
        workspace_focus_tool(ui, app, layout);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::super::SurfaceId::ReportAuthoring
    {
        report_authoring_tools(ui, app, layout);
        workspace_focus_tool(ui, app, layout);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::super::SurfaceId::ModelEditor {
        model_editor_tools(ui, app, layout);
        workspace_focus_tool(ui, app, layout);
        return;
    }
    match workspace {
        Workspace::Project => project_tools(ui, app, layout),
        Workspace::Design if app.state.workspace.active_view_type() == ViewType::Symbol => {
            symbol_tools(ui, app, layout);
        }
        Workspace::Design => design_tools(ui, app, layout),
        Workspace::Simulate => simulation_tools(ui, app, layout),
        Workspace::Results => results_tools(ui, app, layout),
        Workspace::Verify => verification_tools(ui, app, layout),
        Workspace::Models => models_tools(ui, app, layout),
        Workspace::Netlist => netlist_tools(ui, app, layout),
    }
    workspace_focus_tool(ui, app, layout);
}

/// The upgraded shell keeps focus mode at the end of every project toolbar,
/// independent of the active workspace's domain tools.
fn workspace_focus_tool(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    context_separator(ui, layout);
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::ToggleFocusMode,
        WorkbenchIcon::Focus,
        app.state.workbench.focus_mode,
        if app.state.workbench.focus_mode {
            "Restore workspace layout"
        } else {
            "Focus workspace"
        },
        layout,
    );
}

fn no_project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::ProjectLauncher,
        WorkbenchIcon::Folder,
        "Open project launcher",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::NewProject,
        WorkbenchIcon::Add,
        "New project",
        layout,
    );
}

fn project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::NewCell,
        WorkbenchIcon::Add,
        "New cell",
        layout,
    );
}

fn design_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    for (index, (command, icon, label)) in DESIGN_DIRECT_TOOLBAR_COMMANDS.into_iter().enumerate() {
        if index == 1 {
            context_separator(ui, layout);
        }
        let selected = design_toolbar_command_selected(app.state.schematic.tool, command);
        toolbar_icon_command_selected_as(ui, app, command, icon, selected, label, layout);
    }
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::RotateSelection,
        WorkbenchIcon::Rotate,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::MirrorSelectionHorizontal,
        WorkbenchIcon::Mirror,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::MirrorSelectionVertical,
        WorkbenchIcon::MirrorVertical,
        layout,
    );
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::Duplicate,
        WorkbenchIcon::Copy,
        false,
        "Duplicate and place",
        layout,
    );
    toolbar_icon_command(ui, app, Command::Delete, WorkbenchIcon::Trash, layout);
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_icon_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::DescendHierarchyDirect,
        WorkbenchIcon::Layers,
        layout,
    );
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::AscendHierarchy,
        WorkbenchIcon::ArrowUp,
        false,
        "Ascend to parent sheet · double-click the sheet background also ascends",
        layout,
    );
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::ZoomOut, WorkbenchIcon::ZoomOut, layout);
    toolbar_icon_command(ui, app, Command::ZoomIn, WorkbenchIcon::ZoomIn, layout);
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::ZoomFit,
        WorkbenchIcon::ZoomFit,
        false,
        "Fit drawing sheet",
        layout,
    );
    design_grid_and_snap_menu(ui, app, layout);
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::VisibilityOptions,
        WorkbenchIcon::Visibility,
        false,
        "Hierarchy and annotation visibility",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::RunChecks,
        WorkbenchIcon::Check,
        "Run schematic checks",
        layout,
    );
}

fn design_toolbar_command_selected(tool: Tool, command: Command) -> bool {
    match command {
        Command::SelectTool => tool == Tool::Select,
        Command::PlaceInstance => {
            matches!(tool, Tool::Place(component) if component != ComponentType::Port)
        }
        Command::PlaceWire => tool == Tool::Wire,
        Command::PlaceBus => tool == Tool::Bus,
        Command::PlaceBusTap => tool == Tool::BusTap,
        Command::PlaceJunction => tool == Tool::Junction,
        Command::PlaceLabel => tool == Tool::Label,
        Command::PlacePin => tool == Tool::Place(ComponentType::Port),
        Command::PlaceProbe => tool == Tool::Probe,
        Command::PlaceText => tool == Tool::DesignNote,
        _ => false,
    }
}

fn design_grid_and_snap_menu(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let grid_visible = app.state.ui.grid != crate::state::GridStyle::Off;
    let height = layout.toolbar_control_height;
    let response = ui.add(egui::Button::selectable(grid_visible, "").min_size(Vec2::splat(height)));
    if response.clicked() {
        Command::CycleGrid.execute(app);
    }
    let grid_visible = app.state.ui.grid != crate::state::GridStyle::Off;
    let t = Tokens::get(ui.ctx());
    WorkbenchIcon::Grid.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), Vec2::splat(16.0)),
        if grid_visible {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Cycle grid display");
        node.set_description("Cycle the schematic grid between dots, lines, and off.");
    });
    response.on_hover_text("Cycle grid display (G)");
}

/// Toolbar of an open symbol cellview.
///
/// Symbol geometry has its own undo stack, so undo and redo route through
/// the symbol document rather than the schematic's.
fn symbol_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let active = app.state.ui.symbol.tool;
    for (command, icon, tool) in SYMBOL_DIRECT_TOOLBAR_COMMANDS {
        toolbar_icon_command_selected(ui, app, command, icon, active == tool, layout);
    }
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::SymbolRotatePin,
        WorkbenchIcon::Rotate,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::SymbolMirrorPin,
        WorkbenchIcon::Mirror,
        layout,
    );
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_icon_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::ZoomOut, WorkbenchIcon::ZoomOut, layout);
    toolbar_icon_command(ui, app, Command::ZoomIn, WorkbenchIcon::ZoomIn, layout);
    toolbar_icon_command(ui, app, Command::ZoomFit, WorkbenchIcon::ZoomFit, layout);
    symbol_toggle(
        ui,
        &mut app.state.ui.symbol.show_grid,
        WorkbenchIcon::Grid,
        "Show grid",
        layout,
    );
    symbol_toggle(
        ui,
        &mut app.state.ui.symbol.snap_to_grid,
        WorkbenchIcon::Check,
        "Snap to grid",
        layout,
    );
    symbol_grid_spacing(ui, app, layout);
    context_separator(ui, layout);
    symbol_toggle(
        ui,
        &mut app.state.ui.symbol.preview_as_placed,
        WorkbenchIcon::Focus,
        "Preview as placed on a sheet",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::SymbolSave,
        WorkbenchIcon::Save,
        "Save symbol",
        layout,
    );
}

fn symbol_toggle(
    ui: &mut egui::Ui,
    value: &mut bool,
    icon: WorkbenchIcon,
    label: &'static str,
    layout: LayoutSpec,
) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let response = icon_button(ui, icon, label, *value, toolbar_icon_button_size(layout));
    if response.clicked() {
        *value = !*value;
    }
}

fn symbol_grid_spacing(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    egui::ComboBox::from_id_salt("symbol-editor.grid-spacing")
        .selected_text(app.state.ui.symbol.grid_spacing.label())
        .width(72.0)
        .show_ui(ui, |ui| {
            for spacing in crate::workbench::SymbolGridSpacing::ALL {
                ui.selectable_value(
                    &mut app.state.ui.symbol.grid_spacing,
                    spacing,
                    spacing.label(),
                );
            }
        });
}

fn simulation_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        Command::SimulationOptions,
        WorkbenchIcon::Sliders,
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::PreflightChecks,
        WorkbenchIcon::Check,
        "Run preflight",
        layout,
    );
}

fn results_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let historical = results_document_is_historical(app);
    if results_tracking_button(ui, app, layout) {
        context_separator(ui, layout);
    }
    toolbar_icon_command(
        ui,
        app,
        Command::VisualizationTraceManager,
        WorkbenchIcon::Results,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::VisualizationCursorManager,
        WorkbenchIcon::Target,
        layout,
    );
    context_separator(ui, layout);
    if !historical {
        toolbar_icon_command(
            ui,
            app,
            Command::WaveformCalculator,
            WorkbenchIcon::Sliders,
            layout,
        );
        context_separator(ui, layout);
    }
    toolbar_icon_command(
        ui,
        app,
        Command::CompareResultDatasets,
        WorkbenchIcon::Compare,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ImportResultDataset,
        WorkbenchIcon::Folder,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ExportWaveformsCsv,
        WorkbenchIcon::Export,
        layout,
    );
    toolbar_icon_command(ui, app, Command::PrintHardcopy, WorkbenchIcon::File, layout);
}

fn results_tracking_button(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) -> bool {
    use crate::results::visualization_document::{
        DocumentEdit, ResultDocumentTracking, ResultDocumentTrackingMode,
    };
    use crate::workbench::state::WorkspaceDocumentId;

    let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) = app
        .state
        .workbench
        .documents
        .active(Workspace::Results)
        .cloned()
    else {
        return false;
    };
    if results_document_is_historical(app) {
        if take_projected_tool_slot(ui, layout) {
            labeled_icon_button_sized(
                ui,
                WorkbenchIcon::History,
                "Historical",
                false,
                explicit_label_width("Historical"),
                layout.toolbar_control_height,
            )
            .on_hover_text(
                "This document remains readable from its immutable binding, but no completed run matches the current project revision and generated source.",
            );
        }
        return true;
    }
    let Some((tracking, revision)) = app
        .state
        .workspace
        .visualization_document(document_id)
        .map(|document| (document.tracking(), document.revision()))
    else {
        return false;
    };
    let (Some(plan_id), Some(analysis_id)) =
        (tracking.simulation_plan_id, tracking.authored_analysis_id)
    else {
        // A migrated or externally imported historical document cannot
        // truthfully claim Latest until exact plan and authored-analysis
        // authority exists.
        return false;
    };
    if !take_projected_tool_slot(ui, layout) {
        return true;
    }

    let pinned = tracking.mode == ResultDocumentTrackingMode::Pinned;
    let label = if pinned { "Pinned" } else { "Latest" };
    let tooltip = if pinned {
        "Pinned to the current immutable dataset; reruns will not rebind this document."
    } else {
        "Tracking latest: this document follows the newest run of its exact plan and authored analysis."
    };
    let response = labeled_icon_button_sized(
        ui,
        WorkbenchIcon::Pin,
        label,
        pinned,
        explicit_label_width(label),
        layout.toolbar_control_height,
    );
    let response = response.on_hover_text(tooltip);
    if response.clicked() {
        let next_mode = if pinned {
            ResultDocumentTrackingMode::Latest
        } else {
            ResultDocumentTrackingMode::Pinned
        };
        let next = ResultDocumentTracking::for_plan(next_mode, plan_id, analysis_id);
        let result = app
            .state
            .workspace
            .visualization_documents
            .iter_mut()
            .find(|document| document.id() == document_id)
            .ok_or_else(|| "The active result document is no longer retained.".to_owned())
            .and_then(|document| {
                document
                    .transact(revision, vec![DocumentEdit::SetTracking(next)])
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            });
        match result {
            Ok(()) => {
                app.state.workspace.visualization_documents_dirty = true;
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(if pinned {
                        "Result document now tracks the latest exact plan run."
                    } else {
                        "Result document is pinned to its current immutable dataset."
                    }));
            }
            Err(error) => app
                .state
                .push_user_message(crate::diagnostics::ConsoleMessage::error(error)),
        }
    }
    true
}

fn current_result_source_digest(app: &RSpiceApp) -> Option<ContentDigest> {
    let netlist = &app.state.ui.netlist;
    (netlist.generation_error.is_none()
        && netlist.generated_input_digest.is_some()
        && netlist.generated_input_digest == netlist.current_generation_input_digest
        && !app.state.simulation.netlist_content.trim().is_empty())
    .then(|| {
        crate::workbench::documents::netlist_document::source_content_digest(
            &app.state.simulation.netlist_content,
        )
    })
}

fn run_has_current_success_authority(
    run: &crate::state::SimulationRun,
    project_revision: ObjectRevision,
    source_digest: Option<ContentDigest>,
) -> bool {
    run.lifecycle == crate::state::SimulationRunLifecycle::Completed
        && run.success
        && run.prepared_receipt().is_some_and(|receipt| {
            receipt.project_revision() == project_revision
                && source_digest.is_some_and(|digest| receipt.source_content_digest() == digest)
        })
}

fn results_document_is_historical(app: &RSpiceApp) -> bool {
    use crate::workbench::state::WorkspaceDocumentId;

    let project_revision = app.state.workspace.project.revision();
    let source_digest = current_result_source_digest(app);
    match app.state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset_id)) => app
            .state
            .simulation
            .runs
            .iter()
            .filter(|run| run_has_current_success_authority(run, project_revision, source_digest))
            .max_by_key(|run| run.id)
            .is_none_or(|latest| latest.dataset_id != *dataset_id),
        Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => {
            let Some(document) = app.state.workspace.visualization_document(*document_id) else {
                return true;
            };
            let Some(binding) = document
                .panes()
                .iter()
                .filter_map(|pane| pane.binding)
                .next()
            else {
                return true;
            };
            let tracking = document.tracking();
            let (Some(plan_id), Some(analysis_id)) =
                (tracking.simulation_plan_id, tracking.authored_analysis_id)
            else {
                return true;
            };
            app.state
                .simulation
                .runs
                .iter()
                .filter(|run| {
                    run_has_current_success_authority(run, project_revision, source_digest)
                        && run
                            .prepared_receipt()
                            .and_then(|receipt| receipt.simulation_plan_id())
                            == Some(plan_id)
                        && run.analyses.iter().any(|analysis| {
                            analysis.success
                                && crate::workbench::documents::result_document::analysis_matches_authored_source(
                                    analysis,
                                    analysis_id,
                                )
                        })
                })
                .max_by_key(|run| run.id)
                .is_none_or(|latest| latest.dataset_id != binding.dataset.dataset_id)
        }
        _ => false,
    }
}

fn visualization_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::AddVisualizationPane,
        WorkbenchIcon::Add,
        "Add visualization pane",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::VisualizationTraceManager,
        WorkbenchIcon::Sliders,
        "Trace manager",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::VisualizationCursorManager,
        WorkbenchIcon::Target,
        "Cursor manager",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::VisualizationDocumentProperties,
        WorkbenchIcon::Settings,
        "Document properties",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ExportVisualizationDocument,
        WorkbenchIcon::Export,
        "Export document",
        layout,
    );
}

fn report_authoring_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::SaveReportDocument,
        WorkbenchIcon::Save,
        "Save report document",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::AddReportPage,
        WorkbenchIcon::Add,
        "Add report page",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ReportPageProperties,
        WorkbenchIcon::Sliders,
        "Page properties",
        layout,
    );
    // The mockup's Build review draft action remains absent until exact plot
    // artwork publication and the release-handoff executor complete the full
    // contract. Report authoring never advertises a non-executable artifact.
}

fn model_editor_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    if let Some(origin) = app.state.workbench.previous_route() {
        let label = format!("Source: {}", origin.surface_id().label());
        if take_projected_tool_slot(ui, layout)
            && labeled_icon_button_sized(
                ui,
                WorkbenchIcon::ArrowLeft,
                &label,
                false,
                explicit_label_width(&label),
                layout.toolbar_control_height,
            )
            .clicked()
        {
            app.state
                .workbench
                .navigate_back(RouteTransitionSource::User);
            return;
        }
        context_separator(ui, layout);
    }
    toolbar_text_command(
        ui,
        app,
        Command::ModelSaveRevision,
        WorkbenchIcon::Save,
        "Save model revision",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ModelValidate,
        WorkbenchIcon::Check,
        "Validate model",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ModelRunQualificationTests,
        WorkbenchIcon::Run,
        "Run qualification tests",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ModelCompareRelease,
        WorkbenchIcon::Compare,
        "Compare with release",
        layout,
    );
}

fn verification_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        Command::EditSpecifications,
        WorkbenchIcon::Target,
        layout,
    );
}

fn models_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    for (index, (command, icon, label)) in MODELS_TOOLBAR_COMMANDS.into_iter().enumerate() {
        if matches!(index, 1 | 3) {
            context_separator(ui, layout);
        }
        if let Some(label) = label {
            toolbar_text_command(ui, app, command, icon, label, layout);
        } else {
            toolbar_icon_command(ui, app, command, icon, layout);
        }
    }
}

/// The Code & Automation workspace hosts three different documents, so its
/// toolbar is chosen by the visible page.
///
/// Find is deliberately absent: the editor's own toolbar sits directly under
/// this one and owns it. Offering it twice was the mockup's own earlier
/// mistake, and here it was worse than redundant — the find window is drawn
/// only by the netlist page, so the button silently did nothing on the other
/// two and then ambushed the user on return.
fn netlist_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    match app.state.ui.code_workspace.page {
        CodeWorkspacePage::Netlist => netlist_page_tools(ui, app, layout),
        CodeWorkspacePage::VerilogA => {
            toolbar_text_command(
                ui,
                app,
                Command::CompileVerilogA,
                WorkbenchIcon::Run,
                "Compile Verilog-A project",
                layout,
            );
            context_separator(ui, layout);
            toolbar_text_command(
                ui,
                app,
                Command::ModelRunQualificationTests,
                WorkbenchIcon::Models,
                "Open model qualification",
                layout,
            );
            source_page_tools(ui, app, layout);
        }
        CodeWorkspacePage::Automation => {
            toolbar_text_command(
                ui,
                app,
                Command::AutomationConsole,
                WorkbenchIcon::Terminal,
                "Automation console",
                layout,
            );
            source_page_tools(ui, app, layout);
        }
    }
    toolbar_icon_command(ui, app, Command::RunSimulation, WorkbenchIcon::Run, layout);
}

/// The three tools every bundle-backed source page shares: navigate the index,
/// manage the document, search the closure. Both language pages carry the same
/// group in the same order, so it is written once.
fn source_page_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::SourceLanguageTools,
        WorkbenchIcon::Target,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ManageSourceDocument,
        WorkbenchIcon::Folder,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::FindCodeDocument,
        WorkbenchIcon::Search,
        layout,
    );
}

/// The netlist page's set follows the document it is showing: a generated
/// primary is exported and compared, an owned deck is saved and run.
fn netlist_page_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let generated = app.state.ui.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated;
    if generated {
        toolbar_text_command(
            ui,
            app,
            Command::ExportNetlist(crate::io::NetlistFormat::Spice),
            WorkbenchIcon::Export,
            "Export generated deck",
            layout,
        );
        toolbar_text_command(
            ui,
            app,
            Command::ValidateCodeDocument,
            WorkbenchIcon::Check,
            "Validate generated netlist",
            layout,
        );
        context_separator(ui, layout);
        toolbar_text_command(
            ui,
            app,
            Command::CompareGeneratedRevisions,
            WorkbenchIcon::Compare,
            "Compare generated revisions",
            layout,
        );
        return;
    }
    toolbar_text_command(
        ui,
        app,
        Command::Save,
        WorkbenchIcon::Save,
        "Save deck",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ValidateCodeDocument,
        WorkbenchIcon::Check,
        "Validate source",
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
        WorkbenchIcon::Export,
        "Export deck copy",
        layout,
    );
}

/// Render a canonical compact precision tool. The design mockup uses these
/// for all direct canvas transforms and placement modes.
fn toolbar_icon_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    layout: LayoutSpec,
) {
    toolbar_icon_command_selected(ui, app, command, icon, false, layout);
}

fn toolbar_icon_command_selected(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    selected: bool,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Icon {
            icon,
            selected,
            label: None,
        },
    );
}

fn toolbar_icon_command_selected_as(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    selected: bool,
    label: &'static str,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Icon {
            icon,
            selected,
            label: Some(label),
        },
    );
}

/// Render one explicitly labeled toolbar action. The design mockup reserves
/// this treatment for checks so its state-changing validation scope remains
/// visible without opening a menu.
fn toolbar_text_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    label: &'static str,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Text { icon, label },
    );
}

#[derive(Clone, Copy)]
enum ToolbarCommandPresentation {
    Icon {
        icon: WorkbenchIcon,
        selected: bool,
        label: Option<&'static str>,
    },
    Text {
        icon: WorkbenchIcon,
        label: &'static str,
    },
}

fn toolbar_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    layout: LayoutSpec,
    presentation: ToolbarCommandPresentation,
) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let availability = command.availability(app);
    if availability == CommandAvailability::Hidden {
        return;
    }
    let spec = command.spec();
    let (icon, base_label, show_label, selected) = match presentation {
        ToolbarCommandPresentation::Icon {
            icon,
            selected,
            label,
        } => (icon, label.unwrap_or(spec.label), false, selected),
        ToolbarCommandPresentation::Text { icon, label } => {
            (icon, label, layout.toolbar_labels, false)
        }
    };
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        command,
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let label = if shortcut.is_empty() {
        base_label.to_owned()
    } else {
        format!("{base_label} ({shortcut})")
    };
    let enabled = availability == CommandAvailability::Available;
    let response = ui.add_enabled_ui(enabled, |ui| {
        if show_label {
            labeled_icon_button_sized(
                ui,
                icon,
                &label,
                selected,
                explicit_label_width(&label),
                layout.toolbar_control_height,
            )
        } else {
            icon_button(ui, icon, &label, selected, toolbar_icon_button_size(layout))
        }
    });
    let response = match availability {
        CommandAvailability::Disabled(reason) => response.inner.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response.inner,
    };
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.clicked() {
        command.execute_with_feedback(app, ui.ctx());
    }
}

fn projected_tool_count_id() -> egui::Id {
    egui::Id::new("workbench.context_toolbar.projected_tool_count")
}

fn take_projected_tool_slot(ui: &mut egui::Ui, layout: LayoutSpec) -> bool {
    let Some(limit) = layout.toolbar_tool_limit else {
        return true;
    };
    let id = projected_tool_count_id();
    let count = ui.data(|data| data.get_temp::<usize>(id).unwrap_or_default());
    if count >= limit {
        return false;
    }
    ui.data_mut(|data| data.insert_temp(id, count + 1));
    true
}

fn context_separator(ui: &mut egui::Ui, layout: LayoutSpec) {
    if layout.toolbar_tool_limit.is_none() {
        separator(ui);
    }
}

fn run_controls(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let running = app.state.simulation.is_running;
    let command = if running {
        Command::StopSimulation
    } else {
        Command::RunSimulation
    };
    let icon = if running {
        WorkbenchIcon::Stop
    } else {
        WorkbenchIcon::Run
    };
    let t = Tokens::get(ui.ctx());
    let radius = t.radius.round().clamp(0.0, u8::MAX as f32) as u8;
    let label = if running {
        "Stop".to_owned()
    } else if layout.compact_shell {
        format!(
            "Run · {}",
            app.state.sim_setup.reference_pvt.process.short_name()
        )
    } else {
        "Run plan".to_owned()
    };
    let label_width = ui
        .painter()
        .layout_no_wrap(
            label.clone(),
            theme::sans(tokens::FS_1, FontWeight::SemiBold),
            t.color.text,
        )
        .size()
        .x;
    let more_width = if layout.run_control_height >= 44.0 {
        44.0
    } else {
        25.0
    };
    let primary_width = (22.0 + 14.0 + 6.0 + label_width).max(67.0);
    let width = primary_width + more_width;
    ui.allocate_ui_with_layout(
        Vec2::new(width, layout.run_control_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let accessibility_label = if running {
                "Stop active simulation".to_owned()
            } else {
                let reference = app.state.sim_setup.reference_pvt;
                let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
                format!(
                    "Run active simulation plan at {} · {}",
                    reference.process.short_name(),
                    pvt_temperature_label(reference.temperature_celsius, quantity_policy)
                )
            };
            let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
                command,
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            let accessibility_label = if shortcut.is_empty() {
                accessibility_label
            } else {
                format!("{accessibility_label} ({shortcut})")
            };
            let enabled = command.is_enabled(app);
            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(primary_width, layout.run_control_height),
                if enabled {
                    egui::Sense::click()
                } else {
                    egui::Sense::hover()
                },
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    enabled && ui.is_enabled(),
                    &accessibility_label,
                )
            });
            if enabled && !shortcut.is_empty() {
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_keyboard_shortcut(shortcut.as_str());
                });
            }
            let base_fill = if enabled {
                if running { t.color.err } else { t.color.accent }
            } else {
                t.color.bg_inset
            };
            let fill = if enabled && response.hovered() {
                brighten_srgb(base_fill, 1.06)
            } else {
                base_fill
            };
            let ink = if !enabled {
                t.color.text_faint
            } else if running {
                t.color.text
            } else {
                t.color.accent_ink
            };
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius {
                    nw: radius,
                    sw: radius,
                    ne: 0,
                    se: 0,
                },
                fill,
            );
            icon.paint(
                ui.painter(),
                egui::Rect::from_center_size(
                    egui::Pos2::new(rect.left() + 16.0, rect.center().y),
                    Vec2::splat(14.0),
                ),
                ink,
            );
            ui.painter().text(
                egui::Pos2::new(rect.left() + 30.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &label,
                // The mockup run control inherits the 12 px application body type.
                theme::sans(tokens::FS_1, FontWeight::SemiBold),
                ink,
            );
            theme::paint_focus_ring_outset(ui, &response, rect);
            if response.clicked() && enabled {
                command.execute(app);
            }
            if enabled {
                response.clone().on_hover_text(&accessibility_label);
            }
            if !enabled {
                let reason = if app.state.workbench.workspace == Workspace::Netlist {
                    app.manual_deck_run_block_reason()
                } else {
                    app.state.simulation_run_block_reason()
                };
                response.on_hover_text(reason.unwrap_or_else(|| "Command unavailable".to_owned()));
            }

            let (more_response, _) = ui
                .scope(|ui| {
                    ui.spacing_mut().button_padding = Vec2::ZERO;
                    MenuButton::from_button(
                        egui::Button::new("")
                            .frame(false)
                            .min_size(Vec2::new(more_width, layout.run_control_height)),
                    )
                    .ui(ui, |ui| {
                        ui.set_min_width(210.0);
                        ui.spacing_mut().item_spacing.y = 0.0;
                        let primary = if running {
                            Command::StopSimulation
                        } else {
                            Command::RunSimulation
                        };
                        run_menu_item(ui, app, primary);
                        run_menu_item(ui, app, Command::PreflightChecks);
                        ui.separator();
                        run_menu_item(ui, app, Command::SimulationOptions);
                    })
                })
                .inner;
            more_response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "More run commands",
                )
            });
            let more_fill = if more_response.hovered() {
                brighten_srgb(base_fill, 1.06)
            } else {
                base_fill
            };
            ui.painter().rect_filled(
                more_response.rect,
                egui::CornerRadius {
                    nw: 0,
                    sw: 0,
                    ne: radius,
                    se: radius,
                },
                more_fill,
            );
            ui.painter().vline(
                more_response.rect.left(),
                more_response.rect.y_range().shrink(5.0),
                egui::Stroke::new(1.0, ink.gamma_multiply(0.28)),
            );
            WorkbenchIcon::ChevronDown.paint(
                ui.painter(),
                egui::Rect::from_center_size(more_response.rect.center(), Vec2::splat(12.0)),
                ink,
            );
            theme::paint_focus_ring_outset(ui, &more_response, more_response.rect);
        },
    );
}

fn run_menu_item(ui: &mut egui::Ui, app: &mut RSpiceApp, command: Command) {
    let enabled = command.is_enabled(app);
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        command,
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let label = if shortcut.is_empty() {
        command.spec().label.to_owned()
    } else {
        format!("{} ({shortcut})", command.spec().label)
    };
    let response = ui.add_enabled(enabled, egui::Button::new(label).frame(false));
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.clicked() {
        command.execute(app);
        ui.close();
    }
}

fn pvt_selector(ui: &mut egui::Ui, app: &mut RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.sim_setup.reference_pvt;
    let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
    let label = format!(
        "{} · {}",
        reference.process.short_name(),
        pvt_temperature_label(reference.temperature_celsius, quantity_policy),
    );
    let width = ui
        .painter()
        .layout_no_wrap(
            label.clone(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        )
        .size()
        .x
        + 42.0;
    let (response, _) = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(
                egui::Button::new("")
                    .frame(false)
                    .min_size(Vec2::new(width, height)),
            )
            .ui(ui, |ui| {
                ui.set_min_width(190.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                use crate::simulation::dialog::corner::ProcessCorner;
                let option_height = if height >= 44.0 { 44.0 } else { 29.0 };
                egui::ScrollArea::vertical()
                    .id_salt("workbench.reference_pvt.options")
                    .max_height(pvt_menu_height_for_viewport(
                        ui.ctx().content_rect().height(),
                        option_height,
                    ))
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        for process in [
                            ProcessCorner::FF,
                            ProcessCorner::FS,
                            ProcessCorner::TT,
                            ProcessCorner::SF,
                            ProcessCorner::SS,
                        ] {
                            for temperature in [-40.0, 27.0, 125.0] {
                                let selected = reference.process == process
                                    && (reference.temperature_celsius - temperature).abs()
                                        < f64::EPSILON;
                                let label = format!(
                                    "{} · {}",
                                    process.short_name(),
                                    pvt_temperature_label(temperature, quantity_policy)
                                );
                                if ui
                                    .add_sized(
                                        [ui.available_width(), option_height],
                                        egui::Button::selectable(selected, &label),
                                    )
                                    .clicked()
                                {
                                    match commit_reference_pvt(app, process, temperature) {
                                        Ok(true) => app.state.push_user_message(
                                            crate::diagnostics::ConsoleMessage::info(format!(
                                                "Reference PVT changed to {label}"
                                            )),
                                        ),
                                        Ok(false) => {}
                                        Err(error) => app.state.push_user_message(
                                            crate::diagnostics::ConsoleMessage::warning(format!(
                                                "Reference PVT was not changed: {error}"
                                            )),
                                        ),
                                    }
                                    ui.close();
                                }
                            }
                        }
                    });
            })
        })
        .inner;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &label)
    });
    ui.painter().rect(
        response.rect,
        t.radius,
        t.color.bg_inset,
        egui::Stroke::new(
            1.0,
            if response.hovered() {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.painter().circle_filled(
        egui::pos2(response.rect.left() + 11.0, response.rect.center().y),
        3.0,
        t.color.accent,
    );
    let text_clip = egui::Rect::from_min_max(
        egui::pos2(response.rect.left() + 21.0, response.rect.top()),
        egui::pos2(response.rect.right() - 21.0, response.rect.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        egui::pos2(response.rect.left() + 21.0, response.rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(response.rect.right() - 10.0, response.rect.center().y),
            Vec2::splat(11.0),
        ),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, response.rect);
}

fn pvt_menu_height_for_viewport(viewport_height: f32, row_height: f32) -> f32 {
    (viewport_height - 72.0).clamp(row_height * 3.0, 484.0)
}

pub(in crate::workbench) fn commit_reference_pvt(
    app: &mut RSpiceApp,
    process: crate::simulation::dialog::corner::ProcessCorner,
    temperature_celsius: f64,
) -> Result<bool, String> {
    let current = app.state.sim_setup.reference_pvt;
    if current.process == process
        && (current.temperature_celsius - temperature_celsius).abs() < f64::EPSILON
    {
        return Ok(false);
    }
    let mut setup = app.state.sim_setup.clone();
    setup.set_reference_pvt(process, temperature_celsius)?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Reference PVT changed to {} at {temperature_celsius} degrees Celsius.",
            process.short_name()
        ))
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.invalidate_simulation_preflight();
    app.state.workbench.analysis_lifecycle_status = receipt.status_line();
    Ok(true)
}

fn brighten_srgb(color: egui::Color32, factor: f32) -> egui::Color32 {
    let channel = |value: u8| (f32::from(value) * factor).round().clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r()),
        channel(color.g()),
        channel(color.b()),
        color.a(),
    )
}

fn run_config_selector(ui: &mut egui::Ui, app: &mut RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let analysis_count = app.state.sim_setup.enabled_analysis_instance_count();
    let pvt_count = configured_pvt_count(app);
    let summary = format!("{pvt_count} PVT · {analysis_count} analyses");
    let title_width = ui
        .painter()
        .layout_no_wrap(
            "Lab characterization".to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text,
        )
        .size()
        .x;
    let summary_width = ui
        .painter()
        .layout_no_wrap(
            summary.clone(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let width = (31.0 + title_width.max(summary_width) + 22.0).min(190.0);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, height),
        if ui.is_enabled() {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Open Lab characterization simulation plan",
        )
    });

    ui.painter().rect(
        rect,
        t.radius,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
        egui::Stroke::new(
            1.0,
            if response.hovered() {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Sliders.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        t.color.text_dim,
    );
    let copy_clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 31.0, rect.top()),
        egui::pos2(rect.right() - 22.0, rect.bottom()),
    );
    let copy_painter = ui.painter().with_clip_rect(copy_clip);
    copy_painter.text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        "Lab characterization",
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    copy_painter.text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        summary,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.right() - 10.0, rect.center().y),
            Vec2::splat(11.0),
        ),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    if response.clicked() {
        Command::OpenWorkspace(Workspace::Simulate).execute(app);
    }
}

fn configured_pvt_count(app: &RSpiceApp) -> usize {
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

    let mut count = usize::from(
        app.state
            .sim_setup
            .enabled_analysis_instances()
            .any(|instance| {
                !matches!(
                    instance.kind(),
                    AnalysisKind::Corner | AnalysisKind::Temperature
                )
            }),
    );
    for instance in app.state.sim_setup.enabled_analysis_instances() {
        count = count.saturating_add(match instance.draft() {
            AnalysisDraft::Corner(draft) => draft.run_set.point_count(),
            AnalysisDraft::Temperature(draft) => {
                draft.to_config().map_or(0, |config| config.num_temps())
            }
            _ => 0,
        });
    }
    count.max(1)
}

fn pvt_temperature_label(
    celsius: f64,
    policy: crate::quantity::QuantityPresentationPolicy,
) -> String {
    policy
        .format_temperature(celsius + 273.15, 1)
        .replace(".0 ", " ")
        .replace('-', "−")
}

fn separator(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(11.0, 23.0), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn toolbar_icon_button_size(layout: LayoutSpec) -> Vec2 {
    if layout.toolbar_control_height >= 44.0 {
        Vec2::splat(44.0)
    } else {
        Vec2::new(30.0, 29.0)
    }
}

fn panel_toggle_size(layout: LayoutSpec) -> Vec2 {
    if layout.toolbar_control_height >= 44.0 {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    }
}

fn explicit_label_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 6.7 + 36.0).clamp(76.0, 224.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, SimulationPlanId};
    use crate::state::{
        AnalysisResultSourceDomain, PreparedRunReceipt, PreparedRunTaskReceipt,
        PreparedSourceCheckReceipt, SimulationRun, SimulationRunLifecycle, SimulationRunProvenance,
    };
    use crate::workbench::state::WorkbenchState;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn completed_prepared_run(
        project_revision: ObjectRevision,
        source_digest: ContentDigest,
    ) -> SimulationRun {
        let task = PreparedRunTaskReceipt::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            5,
            digest(0x64),
        )
        .expect("task receipt");
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            project_revision,
            digest(0x61),
            source_digest,
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x63)),
            vec![task],
        )
        .expect("run receipt");
        let mut run = SimulationRun::new(1);
        run.restore_provenance(SimulationRunProvenance::Prepared(receipt))
            .expect("run provenance");
        run.mark_running().expect("running lifecycle");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("completed lifecycle");
        run
    }

    #[test]
    fn toolbar_currentness_rejects_stale_failed_and_nonterminal_runs() {
        let revision = ObjectRevision::INITIAL;
        let source = digest(0x72);
        let mut run = completed_prepared_run(revision, source);

        assert!(run_has_current_success_authority(
            &run,
            revision,
            Some(source)
        ));
        assert!(!run_has_current_success_authority(
            &run,
            ObjectRevision::new(revision.get() + 1).expect("next revision"),
            Some(source)
        ));
        assert!(!run_has_current_success_authority(
            &run,
            revision,
            Some(digest(0x73))
        ));
        assert!(!run_has_current_success_authority(&run, revision, None));

        run.success = false;
        assert!(!run_has_current_success_authority(
            &run,
            revision,
            Some(source)
        ));
        run.success = true;
        run.lifecycle = SimulationRunLifecycle::Running;
        assert!(!run_has_current_success_authority(
            &run,
            revision,
            Some(source)
        ));
    }

    #[test]
    fn running_dataset_is_never_presented_as_current() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Results;
        let source = "R1 1 0 1k\n";
        let source_digest =
            crate::workbench::documents::netlist_document::source_content_digest(source);
        let generation_input = digest(0x81);
        app.state.simulation.netlist_content = source.to_owned();
        app.state.ui.netlist.generated_input_digest = Some(generation_input);
        app.state.ui.netlist.current_generation_input_digest = Some(generation_input);
        let revision = app.state.workspace.project.revision();
        let run = completed_prepared_run(revision, source_digest);
        let dataset_id = run.dataset_id;
        app.state.simulation.runs = vec![run];
        app.state.workbench.documents.activate(
            crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
        );

        assert!(!results_document_is_historical(&app));
        app.state.simulation.runs[0].lifecycle = SimulationRunLifecycle::Running;
        assert!(results_document_is_historical(&app));
    }

    #[test]
    fn missing_current_generated_source_never_authenticates_a_retained_dataset() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Results;
        let revision = app.state.workspace.project.revision();
        let run = completed_prepared_run(revision, digest(0x83));
        let dataset_id = run.dataset_id;
        app.state.simulation.runs = vec![run];
        app.state.workbench.documents.activate(
            crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
        );

        assert!(results_document_is_historical(&app));
    }

    #[test]
    fn long_toolbar_labels_remain_bounded() {
        assert_eq!(explicit_label_width("Run"), 76.0);
        assert!(explicit_label_width("Run schematic checks (Ctrl+E)") > 162.0);
        assert!(explicit_label_width("A very long engineering command label") <= 224.0);
    }

    #[test]
    fn desktop_design_toolbar_matches_the_upgraded_mockup() {
        assert_eq!(
            DESIGN_DIRECT_TOOLBAR_COMMANDS.map(|(command, _, _)| command),
            [
                Command::SelectTool,
                Command::PlaceInstance,
                Command::PlaceWire,
                Command::PlaceBus,
                Command::PlaceBusTap,
                Command::PlaceJunction,
                Command::PlaceLabel,
                Command::PlacePin,
                Command::PlaceProbe,
                Command::PlaceText,
            ]
        );
    }

    #[test]
    fn schematic_tooltips_use_the_upgraded_mockup_copy() {
        assert_eq!(
            DESIGN_DIRECT_TOOLBAR_COMMANDS.map(|(_, _, label)| label),
            [
                "Select (Esc)",
                "Place instance",
                "Draw wire",
                "Draw bus",
                "Place bus tap",
                "Place junction",
                "Net label",
                "Place pin or port",
                "Probe signal",
                "Place text or note",
            ]
        );
    }

    #[test]
    fn direct_schematic_toolbar_selection_tracks_every_armed_tool() {
        assert!(design_toolbar_command_selected(
            Tool::Place(ComponentType::Resistor),
            Command::PlaceInstance,
        ));
        assert!(!design_toolbar_command_selected(
            Tool::Place(ComponentType::Port),
            Command::PlaceInstance,
        ));
        assert!(design_toolbar_command_selected(
            Tool::Place(ComponentType::Port),
            Command::PlacePin,
        ));
        assert!(design_toolbar_command_selected(
            Tool::BusTap,
            Command::PlaceBusTap,
        ));
        assert!(design_toolbar_command_selected(
            Tool::Junction,
            Command::PlaceJunction,
        ));
        assert!(design_toolbar_command_selected(
            Tool::DesignNote,
            Command::PlaceText,
        ));
    }

    #[test]
    fn models_toolbar_projects_the_mockup_command_order_and_icons() {
        assert_eq!(
            MODELS_TOOLBAR_COMMANDS,
            [
                (Command::Save, WorkbenchIcon::Save, None),
                (
                    Command::PdkSettings,
                    WorkbenchIcon::Add,
                    Some("Add library"),
                ),
                (Command::RescanModelLibraries, WorkbenchIcon::Refresh, None,),
                (Command::CompileVerilogA, WorkbenchIcon::Netlist, None),
            ]
        );
    }

    #[test]
    fn responsive_toolbar_reserves_fixed_trailing_controls() {
        let tablet = LayoutSpec::resolve(834.0, 1112.0, &WorkbenchState::default());
        let available_after_navigator = 780.0;
        assert_eq!(
            trailing_controls_width(tablet, 5.0, true, 117.0, 96.0),
            252.0
        );
        assert_eq!(
            context_tools_width(
                available_after_navigator,
                tablet,
                8.0,
                5.0,
                true,
                117.0,
                96.0,
            ),
            520.0
        );

        let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(
            trailing_controls_width(phone, 2.0, true, 136.0, 106.0),
            185.0
        );
        assert_eq!(
            trailing_controls_width(phone, 2.0, false, 136.0, 106.0),
            46.0
        );
        assert!(context_tools_width(336.0, phone, 3.0, 2.0, true, 136.0, 106.0) > 0.0);
    }

    #[test]
    fn toolbar_padding_and_group_gaps_match_every_mockup_breakpoint() {
        assert_eq!(toolbar_horizontal_padding(1_280.0), 8.0);
        assert_eq!(toolbar_horizontal_padding(820.0), 5.0);
        assert_eq!(toolbar_horizontal_padding(560.0), 3.0);
        assert_eq!(toolbar_outer_gap(1_280.0), 5.0);
        assert_eq!(toolbar_outer_gap(820.0), 5.0);
        assert_eq!(toolbar_outer_gap(560.0), 2.0);
        assert_eq!(TOOLBAR_CONTEXT_GAP, 3.0);
    }

    #[test]
    fn toolbar_icon_and_panel_toggle_boxes_match_the_mockup_classes() {
        let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
        assert_eq!(toolbar_icon_button_size(desktop), Vec2::new(30.0, 29.0));
        assert_eq!(panel_toggle_size(desktop), Vec2::new(28.0, 27.0));

        let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(toolbar_icon_button_size(phone), Vec2::splat(44.0));
        assert_eq!(panel_toggle_size(phone), Vec2::splat(44.0));
    }

    #[test]
    fn compact_toolbar_projects_the_mockup_button_limit_for_every_workspace() {
        let compact = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(compact.toolbar_tool_limit, Some(2));
    }

    #[test]
    fn pvt_menu_height_is_bounded_for_short_and_touch_viewports() {
        assert_eq!(pvt_menu_height_for_viewport(900.0, 29.0), 484.0);
        assert_eq!(pvt_menu_height_for_viewport(600.0, 44.0), 484.0);
        assert_eq!(pvt_menu_height_for_viewport(140.0, 44.0), 132.0);
    }

    #[test]
    fn reference_pvt_commit_advances_the_plan_and_invalidates_preflight() {
        let mut app = RSpiceApp::test_instance();
        let (plan_id, source_revision) = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| (plan.id(), plan.revision()))
            .expect("default plan");
        let (topology_root, topology_revision, topology_closure) =
            crate::workbench::preflight::configured_topology_revision(&app.state);
        app.state.workbench.preflight.report = Some(crate::workbench::state::PreflightReport {
            project_revision: app.state.workspace.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: Some(plan_id),
            simulation_plan_revision: Some(source_revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        });

        assert!(
            commit_reference_pvt(
                &mut app,
                crate::simulation::dialog::corner::ProcessCorner::FF,
                -40.0,
            )
            .expect("PVT selection commits")
        );

        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            source_revision.next().expect("revision advances")
        );
        assert!(app.state.workbench.preflight.report.is_none());
    }
}
