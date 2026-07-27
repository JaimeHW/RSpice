//! Mockup-owned Create hierarchy from selection workflow.
//!
//! The modal retains an exact parent authority and inferred interface. Primary
//! builds and validates parent, child, library, generated-symbol, hierarchy,
//! and netlist candidates before a single non-fallible project publication.

use std::collections::HashSet;

use egui::{Align, Context, Frame, Label, Layout, Margin, Sense, Stroke, Ui, UiBuilder, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::schematic::view::SchematicSymbolContext;
use crate::services::drc::{DrcConfig, DrcViolation, run_drc_check_with_hierarchy_and_config};
use crate::simulation::netlist_gen::{HierarchySource, generate_netlist_hierarchical};
use crate::state::{
    Cell, CellViewRef, HierarchyExtractionPlan, HierarchyExtractionTerminal,
    HierarchyNetConnectivity, NetLabel, PortDirection, PortDiscipline, PortSpec,
    SheetMoveConnectivityPlan, SymbolDocument, View, ViewType, hierarchy_terminal_direction,
    hierarchy_terminal_discipline,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    select_mono_with_response,
};

use crate::workbench::app::design_history::{
    HierarchyExtractionHistoryEntry, validate_hierarchy_target_unreferenced,
};
use crate::workbench::app::dialogs::schematic_command::{DISCARD_DETAIL, DISCARD_TITLE};
use crate::workbench::app::dialogs::state::{CreateHierarchyDialogState, CreateHierarchyPortDraft};
use crate::workbench::app::schematic::hierarchy_reference_impact::validate_hierarchy_reference_impact;
use crate::workbench::app::{AppState, RSpiceApp, SchematicEditAuthority};

const EYEBROW: &str = "SCHEMATIC \u{00b7} CELL / VIEW CONTRACT";
const TITLE: &str = "Create hierarchical cell";
const PRIMARY: &str = "Create and descend";
const DESCRIPTION: &str = "Create a stable cell/view from the selected schematic instances and infer an explicit electrical interface.";
const CELL_ROW_TITLE: &str = "Cell and view";
const CELL_ROW_DETAIL: &str = "Stable library identity used by hierarchy and netlisting.";
const SOURCE_ROW_TITLE: &str = "Source selection";
const SOURCE_ROW_DETAIL: &str =
    "Selected instances move transactionally; nets become explicit ports.";
const VIEW_OPTIONS: [&str; 2] = ["schematic", "symbol"];
const TABLE_HEADERS: [&str; 4] = ["Port", "Direction", "Discipline", "Net"];
#[cfg(test)]
const DEFAULT_CELL: &str = "sensor_frontend";
const SETTING_ROW_HORIZONTAL_PADDING: i8 = 12;
const SETTING_ROW_VERTICAL_PADDING: i8 = 10;
const SETTING_ROW_MIN_HEIGHT: f32 = 54.0;
const STATUS_DOT_DIAMETER: f32 = 5.0;
const SURFACE_WITHOUT_TABLE_ROWS: f32 = 267.0;
const TABLE_ROW_HEIGHT: f32 = 28.0;

pub(crate) fn create_hierarchy_available(state: &AppState) -> bool {
    !state.schematic.read_only
        && !state.active_view_read_only()
        && !state.schematic.selection.components.is_empty()
        && state.schematic.selection.count() == state.schematic.selection.components.len()
        && state.schematic.selection.components.iter().all(|id| {
            state.schematic.components.iter().any(|component| {
                component.id == *id && component.kind != crate::state::ComponentType::Port
            })
        })
}

pub(crate) fn open_create_hierarchy_dialog(state: &mut AppState) {
    if !create_hierarchy_available(state) {
        state.push_user_message(ConsoleMessage::warning(
            "Create hierarchy requires one or more complete, writable schematic instances and no other selected objects."
                .to_owned(),
        ));
        return;
    }
    let plan = match source_plan(state) {
        Ok(plan) => plan,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Create hierarchy cannot open: {error}"
            )));
            return;
        }
    };
    state.dialogs.create_hierarchy.open(
        SchematicEditAuthority::capture(state),
        plan,
        state.library_manager.revision(),
    );
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_create_hierarchy_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.create_hierarchy.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let discard_confirm = self.state.dialogs.create_hierarchy.discard_confirm;
        let retain_dirty_cancel = self.state.dialogs.create_hierarchy.dirty && !discard_confirm;
        let table_rows = self.state.dialogs.create_hierarchy.ports.len().max(1) as f32;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .size(DialogSize::Transaction)
            .flush_body()
            .initial_height(
                SURFACE_WITHOUT_TABLE_ROWS
                    + table_rows * TABLE_ROW_HEIGHT
                    + if discard_confirm || validation.is_err() {
                        37.0
                    } else {
                        0.0
                    },
            )
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.is_ok())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .description(DESCRIPTION);
        if retain_dirty_cancel {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if let Err(error) = &validation {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Create hierarchy cannot continue",
                error,
            );
        }

        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            Some(hierarchy_body(ui, &mut self.state.dialogs.create_hierarchy))
        });
        match choice {
            DialogChoice::Primary => {
                if let Err(error) = commit_create_hierarchy(&mut self.state) {
                    self.state.dialogs.create_hierarchy.validation_error = Some(error.clone());
                    self.state.push_user_message(ConsoleMessage::warning(error));
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.create_hierarchy.attempt_close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn hierarchy_body(ui: &mut Ui, draft: &mut CreateHierarchyDialogState) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let initial = setting_row(ui, CELL_ROW_TITLE, CELL_ROW_DETAIL, |ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let width = (ui.available_width() - 116.0).max(1.0);
        let response = ui.add_sized(
            vec2(width, t.metrics.ctl_h),
            egui::TextEdit::singleline(&mut draft.cell_name)
                .font(egui::TextStyle::Monospace)
                .margin(Margin::symmetric(8, 4)),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label("New cell name");
        });
        if response.changed() {
            draft.mark_edited();
        }
        let labels = VIEW_OPTIONS.map(str::to_owned);
        let selected = draft.target_view.display_name();
        let view = select_mono_with_response(
            ui,
            "create-hierarchy-view",
            "New cell view",
            selected,
            &labels,
            110.0,
        );
        if let Some(index) = view.picked {
            let next = if index == 0 {
                ViewType::Schematic
            } else {
                ViewType::Symbol
            };
            if next != draft.target_view {
                draft.target_view = next;
                draft.mark_edited();
            }
        }
        response.id
    });
    setting_row(ui, SOURCE_ROW_TITLE, SOURCE_ROW_DETAIL, |ui| {
        let summary = draft.plan.as_ref().map_or_else(
            || "Selection unavailable".to_owned(),
            |plan| {
                format!(
                    "{} \u{00b7} {} nets",
                    plan.source_instance_names.join(" \u{00b7} "),
                    plan.source_net_count
                )
            },
        );
        status_text(ui, &summary);
    });
    hierarchy_port_table(ui, &draft.ports);
    initial
}

fn setting_row<R>(ui: &mut Ui, title: &str, detail: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let response = Frame::NONE
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(
            SETTING_ROW_HORIZONTAL_PADDING,
            SETTING_ROW_VERTICAL_PADDING,
        ))
        .show(ui, |ui| {
            let inner_width = (width - f32::from(SETTING_ROW_HORIZONTAL_PADDING) * 2.0).max(1.0);
            let inner_height =
                (SETTING_ROW_MIN_HEIGHT - f32::from(SETTING_ROW_VERTICAL_PADDING) * 2.0).max(1.0);
            ui.set_min_size(vec2(inner_width, inner_height));
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;
                let content_width = (inner_width - 12.0).max(1.0);
                let label_width = (content_width * (0.38 / 1.38)).max(150.0);
                ui.allocate_ui_with_layout(
                    vec2(label_width.min(content_width), inner_height),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 3.0;
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
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| content(ui))
                    .inner
            })
            .inner
        });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    response.inner
}

fn hierarchy_port_table(ui: &mut Ui, ports: &[CreateHierarchyPortDraft]) {
    table_row(ui, true, TABLE_HEADERS);
    if ports.is_empty() {
        let t = Tokens::get(ui.ctx());
        Frame::NONE
            .fill(t.color.bg_app)
            .inner_margin(Margin::symmetric(12, 0))
            .show(ui, |ui| {
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), TABLE_ROW_HEIGHT),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new("No boundary nets \u{00b7} zero-port child cell")
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                    },
                );
            });
        return;
    }
    for port in ports {
        table_row(
            ui,
            false,
            [
                port.name.as_str(),
                direction_label(port.direction),
                port.discipline.keyword(),
                port.source_net.as_str(),
            ],
        );
    }
}

fn table_row(ui: &mut Ui, heading: bool, values: [&str; 4]) {
    let t = Tokens::get(ui.ctx());
    let height = if heading { 27.0 } else { TABLE_ROW_HEIGHT };
    let fill = if heading {
        t.color.bg_elevated
    } else {
        t.color.bg_app
    };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, fill);
    let column_width = rect.width() / values.len() as f32;
    let mut x = rect.left();
    for (index, value) in values.iter().enumerate() {
        let right = if index == values.len() - 1 {
            rect.right()
        } else {
            x + column_width
        };
        let cell = egui::Rect::from_min_max(
            egui::pos2(x + 8.0, rect.top()),
            egui::pos2((right - 8.0).max(x + 8.0), rect.bottom()),
        );
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(cell)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                ui.set_clip_rect(cell);
                ui.add_sized(
                    cell.size(),
                    Label::new(
                        egui::RichText::new(if heading {
                            value.to_uppercase()
                        } else {
                            (*value).to_owned()
                        })
                        .font(if heading {
                            theme::sans(tokens::FS_0, FontWeight::Medium)
                        } else {
                            theme::sans(tokens::FS_0, FontWeight::Regular)
                        })
                        .color(if heading {
                            t.color.text_faint
                        } else {
                            t.color.text_dim
                        }),
                    )
                    .truncate(),
                );
            },
        );
        x = right;
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(values.join(", "));
    });
}

const fn direction_label(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "input",
        PortDirection::Out => "output",
        PortDirection::InOut | PortDirection::Supply => "inout",
    }
}

fn status_text(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        let (dot, _) = ui.allocate_exact_size(
            vec2(STATUS_DOT_DIAMETER, STATUS_DOT_DIAMETER),
            Sense::hover(),
        );
        ui.painter()
            .circle_filled(dot.center(), STATUS_DOT_DIAMETER * 0.5, t.color.ok);
        ui.add_sized(
            vec2(ui.available_width().max(1.0), t.metrics.ctl_h),
            Label::new(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text_dim),
            )
            .truncate(),
        );
    });
}

fn validate_draft(state: &AppState) -> Result<HierarchyExtractionPlan, String> {
    if let Some(error) = state.dialogs.create_hierarchy.validation_error.as_deref() {
        return Err(error.to_owned());
    }
    let authority = state
        .dialogs
        .create_hierarchy
        .authority
        .as_ref()
        .ok_or_else(|| {
            "The retained source authority is unavailable. Close and reopen Create hierarchy."
                .to_owned()
        })?;
    authority.validate(state, "Create hierarchy")?;
    if state.dialogs.create_hierarchy.library_revision != state.library_manager.revision() {
        return Err(
            "The library or symbol contract changed. Close and reopen Create hierarchy.".to_owned(),
        );
    }
    let mut plan = state.dialogs.create_hierarchy.plan.clone().ok_or_else(|| {
        "The retained hierarchy plan is unavailable. Close and reopen Create hierarchy.".to_owned()
    })?;
    if plan.ports.len() != state.dialogs.create_hierarchy.ports.len() {
        return Err("The inferred interface draft is inconsistent.".to_owned());
    }
    let mut names = HashSet::new();
    for (port, draft) in plan
        .ports
        .iter_mut()
        .zip(&state.dialogs.create_hierarchy.ports)
    {
        if port.source_net != draft.source_net {
            return Err("A boundary net identity changed inside the isolated draft.".to_owned());
        }
        NetLabel::validate_name(
            draft.name.trim(),
            state.schematic.document_policy.net_naming,
        )
        .map_err(|error| format!("Port '{}': {error}", draft.name.trim()))?;
        if !names.insert(draft.name.trim().to_ascii_lowercase()) {
            return Err(format!("Port '{}' is duplicated.", draft.name.trim()));
        }
        port.name = draft.name.trim().to_owned();
        port.direction = draft.direction;
        port.discipline = draft.discipline;
    }
    validate_hierarchy_reference_impact(state, &plan)?;

    let cell_name = state.dialogs.create_hierarchy.cell_name.trim();
    let target = CellViewRef::new(
        &state.workspace.active_view.library,
        cell_name,
        state.dialogs.create_hierarchy.target_view.display_name(),
    );
    target
        .validate_name_segments()
        .map_err(|error| format!("Cell and view: {error}"))?;
    if !matches!(
        state.dialogs.create_hierarchy.target_view,
        ViewType::Schematic | ViewType::Symbol
    ) {
        return Err("Create hierarchy supports the schematic or symbol child view.".to_owned());
    }
    let library = state
        .library_manager
        .get_library(&target.library)
        .ok_or_else(|| format!("Library '{}' no longer exists.", target.library))?;
    if library.read_only {
        return Err(format!("Library '{}' is read-only.", target.library));
    }
    if library
        .cells_sorted()
        .iter()
        .any(|cell| cell.name.eq_ignore_ascii_case(cell_name))
    {
        return Err(format!(
            "Cell '{cell_name}' already exists in library '{}'.",
            target.library
        ));
    }
    for view in ["schematic", "symbol"] {
        let reference = CellViewRef::new(&target.library, cell_name, view);
        if state
            .workspace
            .schematic_buffers
            .contains_key(&reference.key())
        {
            return Err(format!(
                "Workspace buffer '{}' already exists.",
                reference.display_path()
            ));
        }
    }
    validate_hierarchy_target_unreferenced(state, &target)?;
    Ok(plan)
}

fn source_plan(state: &AppState) -> Result<HierarchyExtractionPlan, String> {
    let symbols = SchematicSymbolContext::from_state(state);
    let component_bounds = state
        .schematic
        .components
        .iter()
        .map(|component| (component.id, symbols.component_bounds_tuple(component)))
        .collect::<std::collections::HashMap<_, _>>();
    let terminals = state
        .schematic
        .components
        .iter()
        .flat_map(|component| {
            symbols
                .named_terminal_points(component)
                .into_iter()
                .map(move |(name, point)| {
                    let (direction, discipline) =
                        resolved_terminal_contract(state, component, &name);
                    HierarchyExtractionTerminal {
                        component_id: component.id,
                        direction,
                        discipline,
                        terminal_name: name,
                        point,
                    }
                })
        })
        .collect::<Vec<_>>();
    let result = generated_for(
        state,
        &state.schematic,
        &state.workspace.schematic_buffers,
        &state.library_manager,
    );
    state
        .schematic
        .plan_hierarchy_extraction(
            &terminals,
            &HierarchyNetConnectivity {
                point_to_net: result.point_to_net,
                net_segments: result.net_segments,
            },
            &component_bounds,
        )
        .map_err(|error| error.to_string())
}

/// Resolve the live selected-instance partition through the same canonical
/// connectivity authority used by transactional hierarchy extraction.  Sheet
/// management consumes this result to preserve internal conductors and create
/// exact typed contracts for every source/destination boundary terminal.
pub(in crate::workbench::app) fn selected_component_sheet_move_plan(
    state: &AppState,
) -> Result<SheetMoveConnectivityPlan, String> {
    if !create_hierarchy_available(state) {
        return Err(
            "A governed sheet move requires one or more complete, writable schematic instances and no partial or non-instance selections."
                .to_owned(),
        );
    }
    source_plan(state)?
        .sheet_move_connectivity(&state.schematic)
        .map_err(|error| error.to_string())
}

fn resolved_terminal_contract(
    state: &AppState,
    component: &crate::state::Component,
    terminal_name: &str,
) -> (Option<PortDirection>, PortDiscipline) {
    let fallback = (
        hierarchy_terminal_direction(component, terminal_name),
        hierarchy_terminal_discipline(component, terminal_name),
    );
    let Some(binding) = component.library_cell.as_ref() else {
        return fallback;
    };
    let reference = CellViewRef::new(&binding.library, &binding.cell, "schematic");
    let Some(master) = state.workspace.schematic_buffers.get(&reference.key()) else {
        return fallback;
    };
    master
        .components
        .iter()
        .filter_map(|port| port.port_contract().map(|contract| (port, contract)))
        .find(|(port, _)| port.value.trim().eq_ignore_ascii_case(terminal_name))
        .map_or(fallback, |(_, contract)| {
            (Some(contract.direction), contract.discipline)
        })
}

fn commit_create_hierarchy(state: &mut AppState) -> Result<(), String> {
    let plan = validate_draft(state)?;
    let authority = state
        .dialogs
        .create_hierarchy
        .authority
        .as_ref()
        .cloned()
        .ok_or_else(|| "The retained source authority is unavailable.".to_owned())?;
    authority.validate(state, "Create hierarchy")?;
    let current_plan = source_plan(state)?;
    if current_plan
        != state
            .dialogs
            .create_hierarchy
            .plan
            .clone()
            .ok_or_else(|| "The retained hierarchy plan is unavailable.".to_owned())?
    {
        return Err("The resolved connectivity or symbol geometry changed. Close and reopen Create hierarchy.".to_owned());
    }

    let parent_ref = state.workspace.active_view.clone();
    let library_name = parent_ref.library.clone();
    let cell_name = state.dialogs.create_hierarchy.cell_name.trim().to_owned();
    let target_view_type = state.dialogs.create_hierarchy.target_view;
    let target_schematic_ref = CellViewRef::new(&library_name, &cell_name, "schematic");
    let target_open_ref =
        CellViewRef::new(&library_name, &cell_name, target_view_type.display_name());
    let candidate = state
        .schematic
        .materialize_hierarchy_extraction(
            &plan,
            &library_name,
            &cell_name,
            target_view_type.display_name(),
        )
        .map_err(|error| error.to_string())?;

    let ports = plan
        .ports
        .iter()
        .map(|port| PortSpec {
            name: port.name.clone(),
            direction: port.direction,
        })
        .collect::<Vec<_>>();
    let mut target_cell = Cell::new(&cell_name);
    target_cell.description = format!(
        "Hierarchical cell extracted transactionally from {}",
        parent_ref.display_path()
    );
    let mut schematic_view = View::new("schematic", ViewType::Schematic);
    schematic_view.modified = true;
    target_cell.add_view(schematic_view);
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    SymbolDocument::generated_from_ports(&ports)
        .store_in_view(&mut symbol_view)
        .map_err(|error| format!("Generated symbol could not be encoded: {error}"))?;
    symbol_view
        .metadata
        .insert("generated".to_owned(), "ports".to_owned());
    symbol_view.metadata.insert(
        "ports".to_owned(),
        ports
            .iter()
            .map(|port| format!("{}:{}", port.name, port.direction.keyword()))
            .collect::<Vec<_>>()
            .join(" "),
    );
    symbol_view.modified = true;
    target_cell.add_view(symbol_view);

    let open_views_before = state.workspace.open_views.clone();
    let hierarchy_stack_before = state.workspace.hierarchy_stack.clone();
    let hierarchy_instances_before = state.workspace.hierarchy_instances.clone();
    let before_parent = source_schematic_with_canonical_connections(state);
    let baseline_resolution = state.workspace.resolve_hierarchy_with_active(
        &state.library_manager,
        &parent_ref,
        &state.schematic,
    );
    let baseline_failures = unresolved_hierarchy(&baseline_resolution);
    let baseline_generation = generated_for(
        state,
        &state.schematic,
        &state.workspace.schematic_buffers,
        &state.library_manager,
    );

    let mut next_libraries = state.library_manager.clone();
    next_libraries
        .get_library_mut(&library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?
        .add_cell(target_cell.clone());
    let mut next_workspace = state.workspace.clone();
    next_workspace
        .schematic_buffers
        .insert(parent_ref.key(), candidate.parent.clone());
    next_workspace
        .schematic_buffers
        .insert(target_schematic_ref.key(), candidate.child.clone());
    let candidate_resolution = next_workspace.resolve_hierarchy(&next_libraries);
    let new_failures = unresolved_hierarchy(&candidate_resolution)
        .difference(&baseline_failures)
        .cloned()
        .collect::<Vec<_>>();
    if !new_failures.is_empty() {
        return Err(format!(
            "Hierarchy extraction introduces unresolved bindings: {}",
            new_failures.join("; ")
        ));
    }
    let parent_generation = generated_for(
        state,
        &candidate.parent,
        &next_workspace.schematic_buffers,
        &next_libraries,
    );
    let child_generation = generated_for(
        state,
        &candidate.child,
        &next_workspace.schematic_buffers,
        &next_libraries,
    );
    reject_new_netlist_errors(&baseline_generation.errors, &parent_generation.errors)?;
    reject_child_netlist_errors(&child_generation.errors)?;
    reject_new_diagnostics(
        "netlist warnings",
        &baseline_generation.warnings,
        &parent_generation.warnings,
    )?;
    validate_candidate_drc(
        &state.schematic,
        &candidate.parent,
        &candidate.child,
        &state.workspace.schematic_buffers,
        &next_workspace.schematic_buffers,
        &state.library_manager,
        &next_libraries,
    )?;
    candidate
        .validate_connectivity(
            &plan,
            &parent_generation.point_to_net,
            &child_generation.point_to_net,
        )
        .map_err(|error| error.to_string())?;

    next_workspace.descend_into(
        candidate.instance_name.clone(),
        target_open_ref.clone(),
        target_view_type,
    );
    let open_views_after = next_workspace.open_views.clone();
    let hierarchy_stack_after = next_workspace.hierarchy_stack.clone();
    let hierarchy_instances_after = next_workspace.hierarchy_instances.clone();

    state.library_manager = next_libraries;
    state.workspace = next_workspace;
    state.schematic = candidate.child.clone();
    state.rebuild_active_connections_from_symbols();
    state.bump_active_schematic_epoch();
    state
        .library_manager
        .select_view(&library_name, &cell_name, target_view_type.display_name());
    state.record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
        parent_ref,
        target_schematic_ref,
        target_open_ref,
        before_parent,
        after_parent: candidate.parent,
        child: candidate.child,
        target_cell,
        open_views_before,
        hierarchy_stack_before,
        hierarchy_instances_before,
        open_views_after,
        hierarchy_stack_after,
        hierarchy_instances_after,
    });
    state.dialogs.create_hierarchy.close();
    state.push_user_message(ConsoleMessage::info(format!(
        "Created hierarchical cell {library_name}/{cell_name} and descended into {}",
        target_view_type.display_name()
    )));
    Ok(())
}

fn source_schematic_with_canonical_connections(state: &AppState) -> crate::state::SchematicState {
    let symbols = SchematicSymbolContext::from_state(state);
    let terminals = state
        .schematic
        .components
        .iter()
        .flat_map(|component| {
            symbols
                .named_terminal_points(component)
                .into_iter()
                .map(move |(name, point)| (component.id, name, point))
        })
        .collect::<Vec<_>>();
    let mut schematic = state.schematic.clone();
    schematic.rebuild_connections_from_terminals(&terminals);
    schematic
}

fn generated_for(
    _state: &AppState,
    schematic: &crate::state::SchematicState,
    buffers: &std::collections::HashMap<String, crate::state::SchematicState>,
    libraries: &crate::state::LibraryManager,
) -> crate::simulation::netlist_gen::NetlistResult {
    let hierarchy = HierarchySource::from_workspace(libraries, buffers);
    generate_netlist_hierarchical(schematic, &[], &hierarchy)
}

fn unresolved_hierarchy(
    resolution: &crate::state::workspace::HierarchyResolution,
) -> HashSet<String> {
    resolution
        .bindings
        .iter()
        .filter(|binding| !binding.status.is_resolved())
        .map(|binding| {
            format!(
                "{} ({})",
                binding.reference.display_path(),
                binding.status.label()
            )
        })
        .collect()
}

fn reject_new_netlist_errors(baseline: &[String], candidate: &[String]) -> Result<(), String> {
    reject_new_diagnostics("netlist errors", baseline, candidate)
}

fn reject_new_diagnostics(
    kind: &str,
    baseline: &[String],
    candidate: &[String],
) -> Result<(), String> {
    let mut baseline_counts = std::collections::HashMap::<String, usize>::new();
    for error in baseline {
        *baseline_counts
            .entry(canonical_hierarchy_diagnostic(error))
            .or_default() += 1;
    }
    let mut new = Vec::new();
    for error in candidate {
        let retained = baseline_counts
            .entry(canonical_hierarchy_diagnostic(error))
            .or_default();
        if *retained == 0 {
            new.push(error.clone());
        } else {
            *retained -= 1;
        }
    }
    if new.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Hierarchy extraction introduces new {kind}: {}",
            new.join(" ")
        ))
    }
}

fn canonical_hierarchy_diagnostic(diagnostic: &str) -> String {
    let mut body = diagnostic.trim();
    while let Some(prefixed) = body.strip_prefix("in cell ") {
        let Some((_, nested)) = prefixed.split_once(": ") else {
            break;
        };
        body = nested.trim();
    }
    body.to_owned()
}

fn validate_candidate_drc(
    baseline: &crate::state::SchematicState,
    parent: &crate::state::SchematicState,
    child: &crate::state::SchematicState,
    baseline_buffers: &std::collections::HashMap<String, crate::state::SchematicState>,
    candidate_buffers: &std::collections::HashMap<String, crate::state::SchematicState>,
    baseline_libraries: &crate::state::LibraryManager,
    candidate_libraries: &crate::state::LibraryManager,
) -> Result<(), String> {
    let config = DrcConfig {
        // A reusable cell need not own a local node-0 symbol. Canonical
        // netlisting and connectivity validation separately prove node 0.
        check_missing_ground: false,
        ..DrcConfig::default()
    };
    let baseline_hierarchy = HierarchySource::from_workspace(baseline_libraries, baseline_buffers);
    let candidate_hierarchy =
        HierarchySource::from_workspace(candidate_libraries, candidate_buffers);
    let baseline_result =
        run_drc_check_with_hierarchy_and_config(baseline, &baseline_hierarchy, config.clone());
    let parent_result =
        run_drc_check_with_hierarchy_and_config(parent, &candidate_hierarchy, config.clone());
    let child_result = run_drc_check_with_hierarchy_and_config(child, &candidate_hierarchy, config);
    let baseline = baseline_result
        .violations()
        .iter()
        .map(drc_diagnostic_key)
        .collect::<Vec<_>>();
    let candidate = parent_result
        .violations()
        .iter()
        .chain(child_result.violations())
        .map(drc_diagnostic_key)
        .collect::<Vec<_>>();
    reject_new_diagnostics("design-rule findings", &baseline, &candidate)
}

fn drc_diagnostic_key(violation: &DrcViolation) -> String {
    // Auto-generated node names and coordinates legitimately change when the
    // same topology is translated into the child. Connectivity validation
    // proves the exact net partition; DRC comparison therefore retains the
    // stable severity/type multiplicity and rejects any newly added class.
    format!("{:?}|{:?}", violation.severity, violation.violation_type)
}

fn reject_child_netlist_errors(errors: &[String]) -> Result<(), String> {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "The extracted cell is not netlist-compatible: {}",
            errors.join(" ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ComponentType, Point, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
        SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };

    #[test]
    fn restored_mockup_contract_is_exact() {
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} CELL / VIEW CONTRACT");
        assert_eq!(TITLE, "Create hierarchical cell");
        assert_eq!(PRIMARY, "Create and descend");
        assert_eq!(DEFAULT_CELL, "sensor_frontend");
        assert_eq!(SETTING_ROW_HORIZONTAL_PADDING, 12);
        assert_eq!(SETTING_ROW_VERTICAL_PADDING, 10);
        assert_eq!(SETTING_ROW_MIN_HEIGHT, 54.0);
        assert_eq!(STATUS_DOT_DIAMETER, 5.0);
        assert_eq!(SURFACE_WITHOUT_TABLE_ROWS, 267.0);
    }

    #[test]
    fn open_is_unavailable_for_mixed_selection() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let wire = state
            .schematic
            .add_wire(vec![Point::origin(), Point::new(20, 0)])
            .expect("wire");
        state.schematic.selection.select_component(id);
        state.schematic.selection.select_wire(wire);
        assert!(!create_hierarchy_available(&state));
    }

    #[test]
    fn moved_warning_keeps_its_identity_under_a_cell_prefix() {
        let baseline = vec!["Net label 'sense' is not connected".to_owned()];
        let candidate =
            vec!["in cell work/sensor_frontend: Net label 'sense' is not connected".to_owned()];
        assert!(reject_new_diagnostics("netlist warnings", &baseline, &candidate).is_ok());
        assert_eq!(
            canonical_hierarchy_diagnostic(
                "in cell work/outer: in cell work/inner: retained warning"
            ),
            "retained warning"
        );
    }

    #[test]
    fn commit_publishes_synchronized_cell_and_exact_project_history() {
        let mut state = AppState::default();
        let r1 = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let r2 = state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(80, 0));
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(160, 0));
        state
            .schematic
            .add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        state
            .schematic
            .add_wire(vec![Point::new(100, 0), Point::new(140, 0)]);
        state.schematic.selection.select_component(r1);
        state.schematic.selection.select_component(r2);
        let parent_ref = state.workspace.active_view.clone();
        let library_name = parent_ref.library.clone();

        open_create_hierarchy_dialog(&mut state);
        assert!(state.dialogs.create_hierarchy.open);
        commit_create_hierarchy(&mut state).expect("transaction commits");

        let target_ref = CellViewRef::new(&library_name, DEFAULT_CELL, "schematic");
        assert_eq!(state.workspace.active_view, target_ref);
        let target_cell = state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell(DEFAULT_CELL))
            .expect("target cell");
        let symbol_view = target_cell.get_view("symbol").expect("symbol view");
        assert_eq!(
            symbol_view.metadata.get("generated").map(String::as_str),
            Some("ports")
        );
        let symbol = SymbolDocument::load_from_view(symbol_view).expect("generated symbol");
        assert_eq!(symbol.pins.len(), state.schematic.interface_ports().len());
        assert!(
            state
                .workspace
                .schematic_buffers
                .get(&parent_ref.key())
                .expect("parent buffer")
                .components
                .iter()
                .any(|component| component.kind == ComponentType::CellInstance)
        );

        state
            .library_manager
            .get_library_mut(&library_name)
            .and_then(|library| library.get_cell_mut(DEFAULT_CELL))
            .expect("target cell")
            .views
            .values_mut()
            .for_each(|view| view.modified = false);
        state.schematic.is_dirty = false;
        assert!(
            state.can_undo_project_design(),
            "saving must not disable undo"
        );
        assert_eq!(
            state.undo_project_design().expect("undo"),
            Some("create hierarchical cell".to_owned())
        );
        assert_eq!(state.workspace.active_view, parent_ref);
        assert!(
            state
                .library_manager
                .get_library(&library_name)
                .and_then(|library| library.get_cell(DEFAULT_CELL))
                .is_none()
        );
        assert!(state.can_redo_project_design());
        assert_eq!(
            state.redo_project_design().expect("redo"),
            Some("create hierarchical cell".to_owned())
        );
        assert_eq!(state.workspace.active_view, target_ref);
    }

    #[test]
    fn referenced_moved_instance_blocks_without_mutating_the_project() {
        let mut state = AppState::default();
        let selected = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let selected_name = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == selected)
            .expect("selected")
            .name
            .clone();
        state.schematic.selection.select_component(selected);
        let plan_id = state
            .sim_setup
            .analysis_plan
            .as_ref()
            .expect("active plan")
            .id();
        state
            .workspace
            .ensure_active_plan_data(plan_id)
            .saved_outputs
            .push(
                SavedOutput::new(
                    SavedOutputKind::RawVoltageOrCurrent,
                    "branch_current",
                    format!("I({selected_name})"),
                    SavedOutputCompatibility::OpTranAc,
                    SavedOutputPolicy::EveryAcceptedPoint,
                    SavedOutputPrecision::FullSourcePrecision,
                    SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
                )
                .expect("saved output"),
            );
        let before = crate::state::SchematicSnapshot::capture(&state.schematic);

        open_create_hierarchy_dialog(&mut state);
        assert!(state.dialogs.create_hierarchy.open);
        let error = commit_create_hierarchy(&mut state).expect_err("reference must block");
        assert!(error.contains("saved output 'branch_current'"), "{error}");
        assert!(before.is_equal_state(&state.schematic));
        assert!(
            state
                .library_manager
                .get_library(&state.workspace.active_view.library)
                .and_then(|library| library.get_cell(DEFAULT_CELL))
                .is_none()
        );
    }
}
