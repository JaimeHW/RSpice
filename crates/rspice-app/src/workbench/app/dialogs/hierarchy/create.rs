//! Create hierarchy from selection.
//!
//! The form retains an exact parent authority and inferred interface. Primary
//! builds and validates parent, child, library, generated-symbol, hierarchy,
//! and netlist candidates before a single non-fallible project publication.

use std::collections::HashSet;

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::schematic::view::schematic_symbol_context;
use crate::state::{
    Cell, CellViewRef, HierarchyExtractionPlan, NetLabel, PortDirection, PortSpec, View, ViewType,
};
use crate::ui::widgets::{CommandForm, DialogChoice, TableColumn};
use rspice_design::drc::{DrcConfig, DrcViolation, run_check_with_hierarchy};
use rspice_design::hierarchy::HierarchySource;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app::schematic::hierarchy_reference_impact::validate_hierarchy_reference_impact;
use crate::workbench::app_state::design_history::{
    HierarchyExtractionHistoryEntry, validate_hierarchy_target_unreferenced,
};
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateHierarchyPortDraft {
    pub(crate) name: String,
    pub(crate) direction: crate::state::PortDirection,
    pub(crate) discipline: crate::state::PortDiscipline,
    pub(crate) source_net: String,
}

/// Isolated draft and complete source authority for Create hierarchy from
/// selection. The live parent, library, and workspace remain untouched until
/// the primary action validates and swaps a complete project candidate.
#[derive(Debug, Clone, Default)]
pub(crate) struct CreateHierarchyDialogState {
    pub(crate) open: bool,
    pub(crate) cell_name: String,
    pub(crate) target_view: crate::state::ViewType,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) plan: Option<crate::state::HierarchyExtractionPlan>,
    pub(crate) ports: Vec<CreateHierarchyPortDraft>,
    pub(crate) library_revision: u64,
    pub(crate) validation_error: Option<String>,
}

impl CreateHierarchyDialogState {
    pub(crate) fn open(
        &mut self,
        authority: SchematicEditAuthority,
        plan: crate::state::HierarchyExtractionPlan,
        library_revision: u64,
        cell_name: String,
    ) {
        let ports = plan
            .ports
            .iter()
            .map(|port| CreateHierarchyPortDraft {
                name: port.name.clone(),
                direction: port.direction,
                discipline: port.discipline,
                source_net: port.source_net.clone(),
            })
            .collect::<Vec<_>>();
        *self = Self {
            open: true,
            cell_name,
            target_view: crate::state::ViewType::Schematic,
            authority: Some(authority),
            plan: Some(plan),
            ports,
            library_revision,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Create hierarchical cell";
const PRIMARY: &str = "Create and descend";
/// The sentence assistive technology reads for the whole form.
const DESCRIPTION: &str = "Move the selected instances into a new cell, with a port for every net that crosses the selection's edge.";
const VIEW_OPTIONS: [&str; 2] = ["Schematic", "Symbol"];
/// The ports table: each port, and the parent net it carries.
const PORT_COLUMNS: [TableColumn<'static>; 4] = [
    TableColumn {
        header: "Port",
        mono: true,
    },
    TableColumn {
        header: "Direction",
        mono: true,
    },
    TableColumn {
        header: "Discipline",
        mono: true,
    },
    TableColumn {
        header: "Net",
        mono: true,
    },
];

pub(crate) fn open_create_hierarchy_dialog(
    dialog: &mut CreateHierarchyDialogState,
    model: &Model,
    workbench: &mut WorkbenchSession,
) {
    // The command is greyed whenever this does not hold.
    if !model.create_hierarchy_available() {
        return;
    }
    let plan = match model.hierarchy_source_plan() {
        Ok(plan) => plan,
        Err(error) => {
            workbench.prompt(format!("Create hierarchy cannot open: {error}"));
            return;
        }
    };
    let cell_name = default_cell_name(model);
    dialog.open(
        SchematicEditAuthority::capture(model),
        plan,
        model.library_manager.revision(),
        cell_name,
    );
}

/// The new cell's first name: the parent cell's with "_sub", numbered from 2
/// when that is taken in the parent's library or among the open documents.
/// A fixed sample name was offered whatever the design was.
fn default_cell_name(model: &Model) -> String {
    let parent = &model.workspace.content.active_view;
    let taken = |name: &str| {
        model
            .library_manager
            .get_library(&parent.library)
            .is_some_and(|library| {
                library
                    .cells_sorted()
                    .iter()
                    .any(|cell| cell.name.eq_ignore_ascii_case(name))
            })
            || ["schematic", "symbol"].into_iter().any(|view| {
                model
                    .workspace
                    .content
                    .schematic_buffers
                    .contains_key(&CellViewRef::new(&parent.library, name, view).key())
            })
    };
    let base = format!("{}_sub", parent.cell);
    std::iter::once(base.clone())
        .chain((2..).map(|index| format!("{base}{index}")))
        .find(|name| !taken(name))
        .expect("the numbered names are unbounded")
}

/// What Create hierarchy draws over: its draft, the model it checks the
/// draft against and the session it hands the extraction to.
pub(in crate::workbench) struct CreateHierarchy<'a> {
    pub(in crate::workbench) dialog: &'a mut CreateHierarchyDialogState,
    pub(in crate::workbench) model: &'a Model,
    pub(in crate::workbench) workbench: &'a mut WorkbenchSession,
}

impl CreateHierarchy<'_> {
    /// Draw Create hierarchy. A checked extraction goes to the shell, which
    /// carries it out over the whole session.
    pub(in crate::workbench) fn render_create_hierarchy_dialog(&mut self, ctx: &Context) {
        if !self.dialog.open {
            return;
        }
        let validation = validate_draft(self.dialog, self.model);
        let draft = &mut *self.dialog;
        let instances = draft
            .plan
            .as_ref()
            .map_or_else(String::new, |plan| plan.source_instance_names.join(", "));
        let ports = draft
            .ports
            .iter()
            .map(|port| {
                vec![
                    port.name.clone(),
                    direction_label(port.direction).to_owned(),
                    port.discipline.keyword().to_owned(),
                    port.source_net.clone(),
                ]
            })
            .collect::<Vec<_>>();
        let mut view = usize::from(draft.target_view == ViewType::Symbol);
        let mut edited = false;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .wide()
            .describe(DESCRIPTION)
            .primary_enabled(validation.is_ok())
            .status(validation.as_ref().map(|_| None).map_err(String::as_str))
            .show(ctx, |rows| {
                let name = egui::Id::new("create-hierarchy-cell-name");
                edited |= rows
                    .text("Cell name", name, &mut draft.cell_name, "")
                    .changed();
                // View spans the column Cell name spans.
                edited |= rows.segmented_columns(
                    "View",
                    "create-hierarchy-view",
                    &VIEW_OPTIONS,
                    &mut view,
                );
                rows.value_mono("Instances", &instances);
                if ports.is_empty() {
                    rows.value("Ports", "None: no net crosses the selection's edge");
                } else {
                    rows.table(&PORT_COLUMNS, &ports);
                }
                Some(name)
            });
        if edited {
            draft.target_view = if view == 1 {
                ViewType::Symbol
            } else {
                ViewType::Schematic
            };
            draft.validation_error = None;
        }
        match choice {
            DialogChoice::Primary => {
                // The shell extracts it straight after this step draws; the
                // footer states a refusal.
                self.workbench.outbox.push(DesignRequest::CreateHierarchy);
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.dialog.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

const fn direction_label(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "input",
        PortDirection::Out => "output",
        PortDirection::InOut | PortDirection::Supply => "inout",
    }
}

fn validate_draft(
    dialog: &CreateHierarchyDialogState,
    model: &Model,
) -> Result<HierarchyExtractionPlan, String> {
    if let Some(error) = dialog.validation_error.as_deref() {
        return Err(error.to_owned());
    }
    let authority = dialog.authority.as_ref().ok_or_else(|| {
        "The retained source authority is unavailable. Close and reopen Create hierarchy."
            .to_owned()
    })?;
    authority.validate(model, "Create hierarchy")?;
    if dialog.library_revision != model.library_manager.revision() {
        return Err(
            "The library or symbol contract changed. Close and reopen Create hierarchy.".to_owned(),
        );
    }
    let mut plan = dialog.plan.clone().ok_or_else(|| {
        "The retained hierarchy plan is unavailable. Close and reopen Create hierarchy.".to_owned()
    })?;
    if plan.ports.len() != dialog.ports.len() {
        return Err("The inferred interface draft is inconsistent.".to_owned());
    }
    let mut names = HashSet::new();
    for (port, draft) in plan.ports.iter_mut().zip(&dialog.ports) {
        if port.source_net != draft.source_net {
            return Err("A boundary net identity changed inside the isolated draft.".to_owned());
        }
        NetLabel::validate_name(
            draft.name.trim(),
            model.schematic.document().document_policy.net_naming,
        )
        .map_err(|error| format!("Port '{}': {error}", draft.name.trim()))?;
        if !names.insert(draft.name.trim().to_ascii_lowercase()) {
            return Err(format!("Port '{}' is duplicated.", draft.name.trim()));
        }
        port.name = draft.name.trim().to_owned();
        port.direction = draft.direction;
        port.discipline = draft.discipline;
    }
    validate_hierarchy_reference_impact(model, &plan)?;

    let cell_name = dialog.cell_name.trim();
    let target = CellViewRef::new(
        &model.workspace.content.active_view.library,
        cell_name,
        dialog.target_view.display_name(),
    );
    target
        .validate_name_segments()
        .map_err(|error| format!("Cell and view: {error}"))?;
    if !matches!(dialog.target_view, ViewType::Schematic | ViewType::Symbol) {
        return Err("Create hierarchy supports the schematic or symbol child view.".to_owned());
    }
    let library = model
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
        if model
            .workspace
            .content
            .schematic_buffers
            .contains_key(&reference.key())
        {
            return Err(format!(
                "Workspace buffer '{}' already exists.",
                reference.display_path()
            ));
        }
    }
    validate_hierarchy_target_unreferenced(model, &target)?;
    Ok(plan)
}

pub(in crate::workbench) fn commit_create_hierarchy(state: &mut AppState) -> Result<(), String> {
    let plan = validate_draft(&state.design.create_hierarchy, &state.model)?;
    let authority = state
        .design
        .create_hierarchy
        .authority
        .as_ref()
        .cloned()
        .ok_or_else(|| "The retained source authority is unavailable.".to_owned())?;
    authority.validate(state, "Create hierarchy")?;
    let current_plan = state.model.hierarchy_source_plan()?;
    if current_plan
        != state
            .design
            .create_hierarchy
            .plan
            .clone()
            .ok_or_else(|| "The retained hierarchy plan is unavailable.".to_owned())?
    {
        return Err("The resolved connectivity or symbol geometry changed. Close and reopen Create hierarchy.".to_owned());
    }

    let parent_ref = state.model.workspace.content.active_view.clone();
    let library_name = parent_ref.library.clone();
    let cell_name = state.design.create_hierarchy.cell_name.trim().to_owned();
    let target_view_type = state.design.create_hierarchy.target_view;
    let target_schematic_ref = CellViewRef::new(&library_name, &cell_name, "schematic");
    let target_open_ref =
        CellViewRef::new(&library_name, &cell_name, target_view_type.display_name());
    let candidate = state
        .model
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
    let mut symbol_view = View::new(
        rspice_design::symbol::generated::GENERATED_SYMBOL_VIEW,
        ViewType::Symbol,
    );
    rspice_design::symbol::generated::store_generated_symbol(&mut symbol_view, &ports, |_| None)
        .map_err(|error| format!("Generated symbol could not be encoded: {error}"))?;
    symbol_view.modified = true;
    target_cell.add_view(symbol_view);

    let open_views_before = state.model.workspace.content.open_views.clone();
    let hierarchy_stack_before = state.model.workspace.content.hierarchy_stack.clone();
    let hierarchy_instances_before = state.model.workspace.content.hierarchy_instances.clone();
    let before_parent = source_schematic_with_canonical_connections(state);
    let baseline_resolution = state.model.workspace.resolve_hierarchy_with_active(
        &state.model.library_manager,
        &parent_ref,
        &state.model.schematic,
    );
    let baseline_failures = unresolved_hierarchy(&baseline_resolution);
    let baseline_generation = Model::hierarchy_netlist(
        &state.model.schematic,
        &state.model.workspace.content.schematic_buffers,
        &state.model.library_manager,
    );

    let mut next_libraries = state.model.library_manager.clone();
    next_libraries
        .edit_library(&library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?
        .add_cell(target_cell.clone());
    let mut next_workspace = state.model.workspace.clone();
    next_workspace.insert_schematic_editor(parent_ref.key(), candidate.parent.clone());
    next_workspace.insert_schematic_editor(target_schematic_ref.key(), candidate.child.clone());
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
    let parent_generation = Model::hierarchy_netlist(
        &candidate.parent,
        &next_workspace.content.schematic_buffers,
        &next_libraries,
    );
    let child_generation = Model::hierarchy_netlist(
        &candidate.child,
        &next_workspace.content.schematic_buffers,
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
        &state.model.schematic,
        &candidate.parent,
        &state.model.workspace.content.schematic_buffers,
        &next_workspace.content.schematic_buffers,
        &state.model.library_manager,
        &next_libraries,
        // Both sides under the configuration the parent is checked under
        // everywhere else. A ground the extraction moves into the child is
        // still reached through the instance that places it.
        state
            .model
            .design_check_config(&state.model.workspace.content.active_view),
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
    let open_views_after = next_workspace.content.open_views.clone();
    let hierarchy_stack_after = next_workspace.content.hierarchy_stack.clone();
    let hierarchy_instances_after = next_workspace.content.hierarchy_instances.clone();

    state.model.library_manager = next_libraries;
    state.model.workspace = next_workspace;
    state.model.schematic = candidate.child.clone();
    state.rebuild_active_connections_from_symbols();
    state.bump_active_schematic_epoch();
    state.model.library_manager.select_view(
        &library_name,
        &cell_name,
        target_view_type.display_name(),
    );
    state
        .model
        .record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
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
    state.design.create_hierarchy.close();
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Created hierarchical cell {library_name}/{cell_name} and descended into its {}.",
            target_view_type.display_name()
        ),
    );
    Ok(())
}

fn source_schematic_with_canonical_connections(state: &AppState) -> crate::state::SchematicState {
    let symbols = schematic_symbol_context(state);
    let terminals = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .flat_map(|component| {
            symbols
                .named_terminal_points(component)
                .into_iter()
                .map(move |(name, point)| (component.id, name, point))
        })
        .collect::<Vec<_>>();
    let mut schematic = state.model.schematic.clone();
    schematic.rebuild_connections_from_terminals(&terminals);
    schematic
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
    baseline_buffers: &std::collections::HashMap<
        String,
        rspice_design::schematic::owned::Schematic,
    >,
    candidate_buffers: &std::collections::HashMap<
        String,
        rspice_design::schematic::owned::Schematic,
    >,
    baseline_libraries: &crate::state::LibraryManager,
    candidate_libraries: &crate::state::LibraryManager,
    config: DrcConfig,
) -> Result<(), String> {
    let baseline_hierarchy =
        HierarchySource::from_workspace(baseline_libraries.catalog(), baseline_buffers);
    let candidate_hierarchy =
        HierarchySource::from_workspace(candidate_libraries.catalog(), candidate_buffers);
    let baseline_result = run_check_with_hierarchy(baseline, &baseline_hierarchy, config.clone());
    // The parent's check walks into the extracted cell through the instance it
    // now places, so the child's findings are already in this one report.
    // Checking the child a second time and chaining the two would count every
    // finding below the instance twice and reject an extraction that added
    // nothing. If the instance failed to bind at all, the parent's own report
    // says so as a finding the baseline does not have, which is the comparison
    // this guard makes.
    let candidate_result = run_check_with_hierarchy(parent, &candidate_hierarchy, config);
    let baseline = baseline_result
        .violations()
        .iter()
        .map(drc_diagnostic_key)
        .collect::<Vec<_>>();
    let candidate = candidate_result
        .violations()
        .iter()
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
        SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SymbolDocument,
    };
    use crate::workbench::app::RSpiceApp;

    fn two_selected_resistors() -> AppState {
        let mut state = AppState::default();
        for x in [0, 40] {
            let id = state
                .model
                .schematic
                .add_component(ComponentType::Resistor, Point::new(x, 0));
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .select_component(id);
        }
        state
    }

    /// The new cell is named after its parent, not "sensor_frontend", and
    /// the next free number follows when that name is taken.
    #[test]
    fn the_new_cell_is_named_after_its_parent() {
        let mut state = two_selected_resistors();
        let parent = state.model.workspace.content.active_view.clone();
        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        assert_eq!(
            state.design.create_hierarchy.cell_name,
            format!("{}_sub", parent.cell)
        );

        state.design.create_hierarchy.close();
        state
            .model
            .library_manager
            .get_library_mut(&parent.library)
            .expect("the parent's library")
            .add_cell(Cell::new(format!("{}_sub", parent.cell)));
        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        assert_eq!(
            state.design.create_hierarchy.cell_name,
            format!("{}_sub2", parent.cell)
        );
    }

    /// Create hierarchy is a command form: its ports table on the labels'
    /// edge, and none of the explainer lines under its labels.
    #[test]
    fn create_hierarchy_is_a_command_form_without_explainers() {
        let mut app = RSpiceApp::test_instance();
        rspice_app_state::workbench::examples::load_example(
            "RC Lowpass Filter",
            &mut app.state.model.schematic,
        );
        let r1 = app
            .state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == "R1")
            .expect("the example places R1")
            .id;
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(r1);
        open_create_hierarchy_dialog(
            &mut app.state.design.create_hierarchy,
            &app.state.model,
            &mut app.state.workbench,
        );
        assert!(app.state.design.create_hierarchy.open);
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut nodes = Vec::new();
        for _ in 0..2 {
            nodes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1440.0, 900.0),
                        )),
                        ..Default::default()
                    },
                    |ui| {
                        CreateHierarchy {
                            dialog: &mut app.state.design.create_hierarchy,
                            model: &app.state.model,
                            workbench: &mut app.state.workbench,
                        }
                        .render_create_hierarchy_dialog(ui.ctx());
                    },
                )
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes;
        }
        let texts = nodes
            .iter()
            .flat_map(|(_, node)| [node.label(), node.value()])
            .flatten()
            .collect::<Vec<_>>();
        assert!(
            texts.iter().any(|text| text.starts_with("Port: ")),
            "the ports table is published: {texts:?}"
        );
        let explainers = texts
            .iter()
            .filter(|text| {
                text.contains("Stable library identity") || text.contains("transactionally")
            })
            .collect::<Vec<_>>();
        assert!(
            explainers.is_empty(),
            "the explainers are back: {explainers:?}"
        );
        // View's two choices span the column Cell name spans.
        let control = |name: &str| {
            nodes
                .iter()
                .map(|(_, node)| node)
                .find(|node| {
                    node.label() == Some(name) && node.role() != egui::accesskit::Role::Label
                })
                .and_then(egui::accesskit::Node::bounds)
                .unwrap_or_else(|| panic!("{name} is published"))
        };
        let (cell, schematic, symbol) = (
            control("Cell name"),
            control(VIEW_OPTIONS[0]),
            control(VIEW_OPTIONS[1]),
        );
        assert!(
            (schematic.x0 - cell.x0).abs() < 1.0 && (symbol.x1 - cell.x1).abs() < 1.0,
            "{schematic:?}..{symbol:?} against {cell:?}"
        );
    }

    #[test]
    fn open_is_unavailable_for_mixed_selection() {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let wire = state
            .model
            .schematic
            .add_wire(vec![Point::origin(), Point::new(20, 0)])
            .expect("wire");
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(id);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_wire(wire);
        assert!(!state.model.create_hierarchy_available());
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
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let r2 = state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(80, 0));
        state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(160, 0));
        state
            .model
            .schematic
            .add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        state
            .model
            .schematic
            .add_wire(vec![Point::new(100, 0), Point::new(140, 0)]);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(r1);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(r2);
        let parent_ref = state.model.workspace.content.active_view.clone();
        let library_name = parent_ref.library.clone();

        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        assert!(state.design.create_hierarchy.open);
        let child = state.design.create_hierarchy.cell_name.clone();
        commit_create_hierarchy(&mut state).expect("transaction commits");

        let target_ref = CellViewRef::new(&library_name, &child, "schematic");
        assert_eq!(state.model.workspace.content.active_view, target_ref);
        let target_cell = state
            .model
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell(&child))
            .expect("target cell");
        let symbol_view = target_cell.get_view("symbol").expect("symbol view");
        assert_eq!(
            symbol_view.metadata.get("generated").map(String::as_str),
            Some("ports")
        );
        let symbol = SymbolDocument::load_from_view(symbol_view).expect("generated symbol");
        assert_eq!(
            symbol.pins.len(),
            state.model.schematic.interface_ports().len()
        );
        assert!(
            state
                .model
                .workspace
                .content
                .schematic_buffers
                .get(&parent_ref.key())
                .expect("parent buffer")
                .document()
                .components
                .iter()
                .any(|component| component.kind == ComponentType::CellInstance)
        );

        state
            .model
            .library_manager
            .get_library_mut(&library_name)
            .and_then(|library| library.get_cell_mut(&child))
            .expect("target cell")
            .views
            .values_mut()
            .for_each(|view| view.modified = false);
        state.model.schematic.session.is_dirty = false;
        assert!(
            state.model.can_undo_project_design(),
            "saving must not disable undo"
        );
        assert_eq!(
            state.undo_project_design().expect("undo"),
            Some("create hierarchical cell".to_owned())
        );
        assert_eq!(state.model.workspace.content.active_view, parent_ref);
        assert!(
            state
                .model
                .library_manager
                .get_library(&library_name)
                .and_then(|library| library.get_cell(&child))
                .is_none()
        );
        assert!(state.model.can_redo_project_design());
        assert_eq!(
            state.redo_project_design().expect("redo"),
            Some("create hierarchical cell".to_owned())
        );
        assert_eq!(state.model.workspace.content.active_view, target_ref);
    }

    #[test]
    fn referenced_moved_instance_blocks_without_mutating_the_project() {
        let mut state = AppState::default();
        let selected = state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let selected_name = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == selected)
            .expect("selected")
            .name
            .clone();
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(selected);
        let plan_id = state
            .model
            .sim_setup
            .analysis_plan
            .as_ref()
            .expect("active plan")
            .id();
        state
            .model
            .workspace
            .content
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
        let before = crate::state::SchematicSnapshot::capture(state.model.schematic.document());

        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        assert!(state.design.create_hierarchy.open);
        let child = state.design.create_hierarchy.cell_name.clone();
        let error = commit_create_hierarchy(&mut state).expect_err("reference must block");
        assert!(error.contains("saved output 'branch_current'"), "{error}");
        assert!(before.is_equal_document(state.model.schematic.document()));
        assert!(
            state
                .model
                .library_manager
                .get_library(&state.model.workspace.content.active_view.library)
                .and_then(|library| library.get_cell(&child))
                .is_none()
        );
    }
}
