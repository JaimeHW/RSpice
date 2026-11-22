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
    Cell, CellViewRef, HierarchyExtractionPlan, NetLabel, PortDirection, PortDiscipline, PortSpec,
    View, ViewType,
};
use crate::ui::widgets::{CommandForm, DialogChoice};
use rspice_design::drc::{DrcConfig, DrcViolation, run_check_with_hierarchy};
use rspice_design::hierarchy::HierarchySource;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::symbol_document::SymbolUiState;
use crate::workbench::app::actions::workspace::{
    bump_active_schematic_epoch, rebuild_active_connections_from_symbols,
};
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app::schematic::hierarchy_reference_impact::validate_hierarchy_reference_impact;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::design_history::{
    HierarchyExtractionHistoryEntry, validate_hierarchy_target_unreferenced,
};
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
pub struct CreateHierarchyDialogState {
    pub open: bool,
    pub cell_name: String,
    /// The library the new cell goes into: the parent's, or another the
    /// reader may write.
    pub library: String,
    /// The libraries the reader may write, the choices for `library`.
    pub(crate) libraries: Vec<String>,
    /// Whether the extraction then opens the new cell, which "Create and
    /// descend" asks and "Create" does not.
    pub descend: bool,
    pub(crate) target_view: crate::state::ViewType,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) plan: Option<crate::state::HierarchyExtractionPlan>,
    pub(crate) ports: Vec<CreateHierarchyPortDraft>,
    pub(crate) library_revision: u64,
    pub validation_error: Option<String>,
}

impl CreateHierarchyDialogState {
    pub(crate) fn open(
        &mut self,
        authority: SchematicEditAuthority,
        plan: crate::state::HierarchyExtractionPlan,
        library_revision: u64,
        cell_name: String,
        (library, libraries): (String, Vec<String>),
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
            library,
            libraries,
            descend: false,
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
/// The primary leaves the reader on the parent with the new instance in
/// place, as Virtuoso's Make Cell does; the secondary then opens the cell.
const PRIMARY: &str = "Create";
const SECONDARY: &str = "Create and descend";
/// The sentence assistive technology reads for the whole form.
const DESCRIPTION: &str = "Move the selected instances into a new cell, with a port for every net that crosses the selection's edge.";
pub const VIEW_OPTIONS: [&str; 2] = ["Schematic", "Symbol"];
/// A port's directions, in the order its select lists them.
const DIRECTIONS: [PortDirection; 4] = [
    PortDirection::In,
    PortDirection::Out,
    PortDirection::InOut,
    PortDirection::Supply,
];

pub fn open_create_hierarchy_dialog(
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
    let libraries = writable_libraries(model);
    let parent_library = &model.workspace.content.active_view.library;
    let library = libraries
        .iter()
        .find(|library| *library == parent_library)
        .or_else(|| libraries.first())
        .cloned()
        .unwrap_or_else(|| parent_library.clone());
    let cell_name = default_cell_name(model, &library);
    dialog.open(
        SchematicEditAuthority::capture(model),
        plan,
        model.library_manager.revision(),
        cell_name,
        (library, libraries),
    );
}

/// The libraries a new cell can go into: every one the reader may write, by
/// name.
fn writable_libraries(model: &Model) -> Vec<String> {
    model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| !library.read_only)
        .map(|library| library.name.clone())
        .collect()
}

/// The new cell's first name: the parent cell's with "_sub", numbered from 2
/// when that is taken in `library` or among the open documents. A fixed
/// sample name was offered whatever the design was.
fn default_cell_name(model: &Model, library: &str) -> String {
    let parent = &model.workspace.content.active_view;
    let taken = |name: &str| {
        model
            .library_manager
            .get_library(library)
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
                    .contains_key(&CellViewRef::new(library, name, view).key())
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
pub struct CreateHierarchy<'a> {
    pub dialog: &'a mut CreateHierarchyDialogState,
    pub model: &'a Model,
    pub workbench: &'a mut WorkbenchSession,
}

impl CreateHierarchy<'_> {
    /// Draw Create hierarchy. A checked extraction goes to the shell, which
    /// carries it out over the whole session.
    pub fn render_create_hierarchy_dialog(&mut self, ctx: &Context) {
        if !self.dialog.open {
            return;
        }
        let validation = validate_draft(self.dialog, self.model);
        let draft = &mut *self.dialog;
        let instances = draft
            .plan
            .as_ref()
            .map_or_else(String::new, |plan| plan.source_instance_names.join(", "));
        let directions = DIRECTIONS.map(|direction| direction_label(direction).to_owned());
        let mut view = usize::from(draft.target_view == ViewType::Symbol);
        let mut edited = false;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .wide()
            .describe(DESCRIPTION)
            .secondary(SECONDARY)
            .primary_enabled(validation.is_ok())
            .secondary_enabled(validation.is_ok())
            .status(validation.as_ref().map(|_| None).map_err(String::as_str))
            .show(ctx, |rows| {
                let name = egui::Id::new("create-hierarchy-cell-name");
                edited |= rows
                    .text("Cell name", name, &mut draft.cell_name, "")
                    .changed();
                let library = rows.select_with_response(
                    "Library",
                    "create-hierarchy-library",
                    &draft.library,
                    &draft.libraries,
                );
                if let Some(picked) = library.picked.and_then(|index| draft.libraries.get(index)) {
                    draft.library = picked.clone();
                    edited = true;
                }
                // Both views are made; this picks the one Create and descend
                // opens. It spans the column Cell name spans.
                edited |= rows.segmented_columns(
                    "Descend into",
                    "create-hierarchy-view",
                    &VIEW_OPTIONS,
                    &mut view,
                );
                rows.value_mono("Instances", &instances);
                if draft.ports.is_empty() {
                    rows.value("Ports", "None: no net crosses the selection's edge");
                } else {
                    // Each port by the parent net it carries: its name, which
                    // starts as the net's or a pin's, and its direction.
                    rows.section("Ports");
                    for (index, port) in draft.ports.iter_mut().enumerate() {
                        let label = if port.discipline == PortDiscipline::default() {
                            port.source_net.clone()
                        } else {
                            format!("{} ({})", port.source_net, port.discipline.keyword())
                        };
                        let (field, picked) = rows.text_with_select(
                            &label,
                            egui::Id::new(("create-hierarchy-port", index)),
                            &mut port.name,
                            "direction",
                            direction_label(port.direction),
                            &directions,
                        );
                        edited |= field.changed();
                        if let Some(direction) = picked.and_then(|index| DIRECTIONS.get(index)) {
                            port.direction = *direction;
                            edited = true;
                        }
                    }
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
            // The shell extracts it straight after this step draws; the
            // footer states a refusal.
            DialogChoice::Primary | DialogChoice::Secondary => {
                self.dialog.descend = choice == DialogChoice::Secondary;
                self.workbench.outbox.push(DesignRequest::CreateHierarchy);
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.dialog.close();
            }
            DialogChoice::None => {}
        }
    }
}

const fn direction_label(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "input",
        PortDirection::Out => "output",
        PortDirection::InOut => "inout",
        PortDirection::Supply => "supply",
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
        &dialog.library,
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

pub fn commit_create_hierarchy(
    dialog: &mut CreateHierarchyDialogState,
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &WorkbenchSession,
) -> Result<String, String> {
    let plan = validate_draft(dialog, model)?;
    let authority = dialog
        .authority
        .as_ref()
        .cloned()
        .ok_or_else(|| "The retained source authority is unavailable.".to_owned())?;
    authority.validate(&*model, "Create hierarchy")?;
    let current_plan = model.hierarchy_source_plan()?;
    if current_plan
        != dialog
            .plan
            .clone()
            .ok_or_else(|| "The retained hierarchy plan is unavailable.".to_owned())?
    {
        return Err("The resolved connectivity or symbol geometry changed. Close and reopen Create hierarchy.".to_owned());
    }

    let parent_ref = model.workspace.content.active_view.clone();
    let library_name = dialog.library.clone();
    let cell_name = dialog.cell_name.trim().to_owned();
    let target_view_type = dialog.target_view;
    let descend = dialog.descend;
    let target_schematic_ref = CellViewRef::new(&library_name, &cell_name, "schematic");
    // What the step leaves in front: the view descended into, or the parent.
    let target_open_ref = if descend {
        CellViewRef::new(&library_name, &cell_name, target_view_type.display_name())
    } else {
        parent_ref.clone()
    };
    // The instance always binds the schematic, which it is netlisted and
    // descended through; the View choice decides only which view opens.
    let candidate = model
        .schematic
        .materialize_hierarchy_extraction(
            &plan,
            &library_name,
            &cell_name,
            ViewType::Schematic.display_name(),
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
    target_cell.description = format!("Made from a selection in {}", parent_ref.display_path());
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

    let open_views_before = model.workspace.content.open_views.clone();
    let hierarchy_stack_before = model.workspace.content.hierarchy_stack.clone();
    let hierarchy_instances_before = model.workspace.content.hierarchy_instances.clone();
    let before_parent = source_schematic_with_canonical_connections(model);
    let baseline_resolution = model.workspace.resolve_hierarchy_with_active(
        &model.library_manager,
        &parent_ref,
        &model.schematic,
    );
    let baseline_failures = unresolved_hierarchy(&baseline_resolution);
    let baseline_generation = Model::hierarchy_netlist(
        &model.schematic,
        &model.workspace.content.schematic_buffers,
        &model.library_manager,
    );

    let mut next_libraries = model.library_manager.clone();
    next_libraries
        .edit_library(&library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?
        .add_cell(target_cell.clone());
    let mut next_workspace = model.workspace.clone();
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
        &model.schematic,
        &candidate.parent,
        &model.workspace.content.schematic_buffers,
        &next_workspace.content.schematic_buffers,
        &model.library_manager,
        &next_libraries,
        // Both sides under the configuration the parent is checked under
        // everywhere else. A ground the extraction moves into the child is
        // still reached through the instance that places it.
        model.design_check_config(&model.workspace.content.active_view),
    )?;
    candidate
        .validate_connectivity(
            &plan,
            &parent_generation.point_to_net,
            &child_generation.point_to_net,
        )
        .map_err(|error| error.to_string())?;

    if descend {
        next_workspace.descend_into(
            candidate.instance_name.clone(),
            target_open_ref.clone(),
            target_view_type,
        );
    }
    let open_views_after = next_workspace.content.open_views.clone();
    let hierarchy_stack_after = next_workspace.content.hierarchy_stack.clone();
    let hierarchy_instances_after = next_workspace.content.hierarchy_instances.clone();

    model.library_manager = next_libraries;
    model.workspace = next_workspace;
    model.schematic = if descend {
        candidate.child.clone()
    } else {
        candidate.parent.clone()
    };
    rebuild_active_connections_from_symbols(model);
    bump_active_schematic_epoch(symbol, model, wb);
    if descend {
        model.library_manager.select_view(
            &library_name,
            &cell_name,
            target_view_type.display_name(),
        );
    }
    let instance_name = candidate.instance_name.clone();
    model.record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
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
    dialog.close();
    let summary = if descend {
        format!(
            "Created hierarchical cell {library_name}/{cell_name} and descended into its {}.",
            target_view_type.display_name()
        )
    } else {
        format!("Created hierarchical cell {library_name}/{cell_name}, placed as {instance_name}.")
    };
    model.record(LogSeverity::Info, LogSource::Studio, summary.clone());
    Ok(summary)
}

fn source_schematic_with_canonical_connections(model: &Model) -> crate::state::SchematicState {
    let symbols = schematic_symbol_context(model);
    let terminals = model
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
    let mut schematic = model.schematic.clone();
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
        SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };
    use crate::workbench::design::test_host::DesignTestHost;

    fn two_selected_resistors() -> DesignTestHost {
        let mut state = DesignTestHost::default();
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

    /// Two resistors joined by a wire, with a third beside them: what a
    /// selection box drawn round the first two would take, wire and all.
    fn two_resistors_boxed_with_their_wire() -> (DesignTestHost, u64) {
        let mut state = DesignTestHost::default();
        let schematic = &mut state.model.schematic;
        let r1 = schematic.add_component(ComponentType::Resistor, Point::origin());
        let r2 = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        schematic.add_component(ComponentType::Resistor, Point::new(160, 0));
        let inner = schematic
            .add_wire(vec![Point::new(20, 0), Point::new(60, 0)])
            .expect("inner wire");
        schematic
            .add_wire(vec![Point::new(100, 0), Point::new(140, 0)])
            .expect("outer wire");
        let selection = &mut schematic.session.editor.selection;
        selection.select_component(r1);
        selection.select_component(r2);
        selection.select_wire(inner);
        (state, inner)
    }

    /// A box drawn round a block takes its wires too, and Create hierarchy
    /// was greyed for it. The whole wires, junctions and labels the box takes
    /// are left to the plan; a piece of a wire or a bus is still refused, and
    /// a sheet move still carries instances only.
    #[test]
    fn create_hierarchy_takes_the_wires_a_box_takes_with_the_instances() {
        let (mut state, inner) = two_resistors_boxed_with_their_wire();
        assert!(state.model.create_hierarchy_available());
        assert!(!state.model.sheet_move_available());
        let label = state
            .model
            .schematic
            .add_net_label(Point::new(40, 0), "mid".to_owned());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_net_label(label);
        assert!(state.model.create_hierarchy_available());

        let mut with_piece = state.model.schematic.session.editor.selection.clone();
        with_piece.select_wire_segment(inner, 0);
        let mut with_bus = state.model.schematic.session.editor.selection.clone();
        let bus = state
            .model
            .schematic
            .add_bus(vec![Point::new(0, 80), Point::new(80, 80)], None)
            .expect("bus");
        with_bus.select_bus(bus);
        for refused in [with_piece, with_bus] {
            state.model.schematic.session.editor.selection = refused;
            assert!(!state.model.create_hierarchy_available());
        }

        let (mut state, _) = two_resistors_boxed_with_their_wire();
        let parent = state.model.workspace.content.active_view.clone();
        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        assert!(state.design.create_hierarchy.open);
        state.design.create_hierarchy.descend = true;
        commit_create_hierarchy(
            &mut state.design.create_hierarchy,
            &mut state.design.symbol,
            &mut state.model,
            &state.workbench,
        )
        .expect("the boxed block becomes a cell");
        let resistors = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::Resistor)
            .count();
        assert_eq!(resistors, 2, "both boxed resistors are in the new cell");
        let parent_parts = &state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&parent.key())
            .expect("parent buffer")
            .document()
            .components;
        let count = |kind| {
            parent_parts
                .iter()
                .filter(|component| component.kind == kind)
                .count()
        };
        assert_eq!(
            (
                parent_parts.len(),
                count(ComponentType::Resistor),
                count(ComponentType::CellInstance)
            ),
            (2, 1, 1),
            "the parent keeps the third resistor beside the new instance"
        );
    }

    /// Create leaves the reader on the parent with the new instance in place,
    /// the cell in the library chosen, its ports named and directed as typed.
    #[test]
    fn create_stays_on_the_parent_with_the_ports_as_typed_in_the_chosen_library() {
        let (mut state, _) = two_resistors_boxed_with_their_wire();
        let parent = state.model.workspace.content.active_view.clone();
        state
            .model
            .library_manager
            .add_library(crate::state::Library::new("blocks"));
        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        let dialog = &mut state.design.create_hierarchy;
        assert_eq!(dialog.library, parent.library, "the parent's library first");
        assert!(dialog.libraries.contains(&"blocks".to_owned()));
        assert!(!dialog.descend, "Create is the primary");
        dialog.library = "blocks".to_owned();
        let port = dialog.ports.first_mut().expect("a net crosses the box");
        port.name = "OUT".to_owned();
        port.direction = PortDirection::Out;
        let cell = dialog.cell_name.clone();

        let summary = commit_create_hierarchy(
            &mut state.design.create_hierarchy,
            &mut state.design.symbol,
            &mut state.model,
            &state.workbench,
        )
        .expect("the block becomes a cell");

        assert!(summary.contains(&format!("blocks/{cell}")), "{summary}");
        assert_eq!(state.model.workspace.content.active_view, parent);
        let binding = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find_map(|component| component.library_cell.clone())
            .expect("the new instance is on the parent in front");
        assert_eq!(
            (binding.library.as_str(), binding.cell.as_str()),
            ("blocks", cell.as_str())
        );
        let ports = state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&CellViewRef::new("blocks", &cell, "schematic").key())
            .expect("the child schematic")
            .document()
            .interface_ports();
        assert!(
            ports
                .iter()
                .any(|port| port.name == "OUT" && port.direction == PortDirection::Out),
            "{ports:?}"
        );
    }

    /// Choosing the Symbol view bound the new instance to the cell's symbol,
    /// and an instance bound to a symbol can never be descended into. The
    /// instance binds the schematic; the choice decides which view opens.
    #[test]
    fn create_hierarchy_binds_the_schematic_whichever_view_opens() {
        let (mut state, _) = two_resistors_boxed_with_their_wire();
        let parent = state.model.workspace.content.active_view.clone();
        open_create_hierarchy_dialog(
            &mut state.design.create_hierarchy,
            &state.model,
            &mut state.workbench,
        );
        state.design.create_hierarchy.target_view = ViewType::Symbol;
        state.design.create_hierarchy.descend = true;
        let child = state.design.create_hierarchy.cell_name.clone();
        commit_create_hierarchy(
            &mut state.design.create_hierarchy,
            &mut state.design.symbol,
            &mut state.model,
            &state.workbench,
        )
        .expect("the block becomes a cell");

        assert_eq!(
            state.model.workspace.content.active_view,
            CellViewRef::new(&parent.library, &child, "symbol"),
            "the symbol opens, as chosen"
        );
        let binding = state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&parent.key())
            .expect("parent buffer")
            .document()
            .components
            .iter()
            .find_map(|component| component.library_cell.clone())
            .expect("the new instance");
        assert_eq!(
            (binding.cell.as_str(), binding.view.as_str()),
            (child.as_str(), "schematic")
        );
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
    fn referenced_moved_instance_blocks_without_mutating_the_project() {
        let mut state = DesignTestHost::default();
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
        let error = commit_create_hierarchy(
            &mut state.design.create_hierarchy,
            &mut state.design.symbol,
            &mut state.model,
            &state.workbench,
        )
        .expect_err("reference must block");
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
