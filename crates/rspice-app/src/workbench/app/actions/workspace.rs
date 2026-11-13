//! Workspace-level edit actions.
//!
//! The clipboard, selection, undo/redo, and hierarchy operations that a menu
//! item or a shortcut resolves to. These act on the whole active document
//! rather than on one selected object, and they are the actions that a
//! read-only schematic refuses.

pub(in crate::workbench) mod design_checks;
mod history;
mod navigation;
pub(in crate::workbench) mod symbol_document;
pub(in crate::workbench) mod symbol_in_place;
mod waivers;

use crate::diagnostics::LogAnchor;
use crate::state::{CellViewRef, ComponentType, RetainedSimulationState, SchematicState, ViewType};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::interaction_state::DragState;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::workbench::slots::runs::RunsState;
use symbol_document::{SymbolUiState, project_active_symbol_session};

pub(crate) use history::{
    DesignHistoryParts, HistoryDirection, HistoryStepParts, commit_pending_inspector_edit,
    take_history_step,
};
pub(crate) use navigation::NavigationParts;

/// What restoring the active schematic from the workspace reads and writes
/// of the design feature's state: the canvas drag it lets go, the snap and
/// routing the new buffer takes up, the visibility whose wire routing follows
/// a new schematic, and the symbol editor projected onto the view in front.
pub(crate) struct SchematicRestoreParts<'a> {
    pub(crate) drag: &'a mut DragState,
    pub(crate) snap: &'a mut crate::state::SnapEngine,
    pub(crate) routing_mode: &'a mut crate::state::WireRoutingMode,
    pub(crate) visibility: &'a mut crate::state::SchematicVisibilityPolicy,
    pub(crate) symbol: &'a mut SymbolUiState,
}

impl SchematicRestoreParts<'_> {
    /// The same parts, lent again for a shorter while.
    pub(crate) fn reborrow(&mut self) -> SchematicRestoreParts<'_> {
        SchematicRestoreParts {
            drag: &mut *self.drag,
            snap: &mut *self.snap,
            routing_mode: &mut *self.routing_mode,
            visibility: &mut *self.visibility,
            symbol: &mut *self.symbol,
        }
    }
}

fn schematic_for_workspace(
    design: &mut SchematicRestoreParts<'_>,
    model: &Model,
    reference: &CellViewRef,
) -> SchematicState {
    let existing = model.workspace.clone_schematic_editor(&reference.key());
    let created = existing.is_none();
    let mut schematic = existing
        .unwrap_or_else(|| new_schematic_document(model, design.snap, *design.routing_mode));
    // Workspace buffers round-trip through serde, which skips the runtime
    // ID counter and name counters: without recalculation a freshly placed
    // component reuses an existing ID and selection matches both.
    schematic.recalculate_runtime_state();
    schematic.session.editor.snap_engine = design.snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    design.snap.grid_size = schematic.document().grid_size;
    schematic.session.editor.automatic_junctions = model.preferences.automatic_junction_dots();
    schematic.session.editor.selection_crossing = model.preferences.selection_crossing();
    if created {
        // A new schematic starts from the routing preference, and the
        // session's routing control follows it.
        let routing_mode = schematic.session.editor.wire_drawing.routing_mode;
        *design.routing_mode = routing_mode;
        design.visibility.wire_routing =
            crate::workbench::app::dialogs::grid_snap_routing::routing_style(routing_mode);
    } else {
        schematic.session.editor.wire_drawing.routing_mode = *design.routing_mode;
        schematic.session.editor.bus_drawing.routing_mode = *design.routing_mode;
    }
    // Views from read-only libraries open for inspection, never for edit —
    // the docbar banner explains and every edit path checks this flag.
    schematic.session.read_only = model.safe_mode.project_read_only()
        || model
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| library.read_only);
    schematic
}

/// Copy the active complete-object selection into `clipboard`, using the
/// exact rendered terminal geometry of authored and generated symbols.
///
/// `SchematicState::copy_selection` remains a state-only fallback for
/// primitive fixtures. Production UI commands must enter here so a wire
/// joining two authored cell pins is captured with the selected
/// instances instead of being silently omitted. The copy goes to the
/// app's clipboard, which outlives the cell view it came from.
pub(crate) fn copy_active_schematic_selection(
    clipboard: &mut rspice_design::schematic::clipboard::ClipboardData,
    model: &Model,
) -> bool {
    let Some(copy) = capture_active_schematic_selection(model) else {
        return false;
    };
    *clipboard = copy;
    true
}

/// The active sheet's selection as a clipboard would hold it, or `None`
/// when nothing is selected. Nothing is stored anywhere.
pub(crate) fn capture_active_schematic_selection(
    model: &Model,
) -> Option<rspice_design::schematic::clipboard::ClipboardData> {
    if model.schematic.session.editor.selection.is_empty() {
        return None;
    }
    let symbols = model.schematic_symbol_context();
    let on_sheet =
        |id| crate::schematic::view::sheet_visibility::object_is_on_active_sheet(model, id);
    Some(model.schematic.capture_complete_selection_resolved(
        |component| {
            symbols
                .named_terminal_points(component)
                .into_iter()
                .map(|(_, point)| point)
                .collect()
        },
        &on_sheet,
    ))
}

impl AppState {
    /// [`new_schematic_document`] for the session.
    pub(crate) fn new_schematic_document(&self) -> SchematicState {
        new_schematic_document(
            &self.model,
            &self.design.schematic_snap,
            self.design.schematic_routing_mode,
        )
    }
}

impl AppState {
    /// [`Model::active_view_read_only`](crate::workbench::app_state::Model::active_view_read_only)
    /// for the session's model.
    pub(crate) fn active_view_read_only(&self) -> bool {
        self.model.active_view_read_only()
    }

    /// [`Model::schematic_edit_read_only`](crate::workbench::app_state::Model::schematic_edit_read_only)
    /// for the session's model.
    pub(crate) fn schematic_edit_read_only(&self) -> bool {
        self.model.schematic_edit_read_only()
    }

    /// [`read_only_master_message`] for the session's model.
    pub(crate) fn read_only_master_message(&self) -> String {
        read_only_master_message(&self.model)
    }

    /// [`NavigationParts::navigate_to_log_anchor`] for the session.
    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        self.navigation().navigate_to_log_anchor(anchor);
    }

    /// Why a row carrying `anchor` cannot jump, or `None` if it can.
    ///
    /// Asked while painting, so a row whose target no longer exists renders
    /// inert with the reason rather than offering a jump that refuses on
    /// click. The two answers are deliberately different surfaces: a refusal
    /// the reader can read before committing belongs in a tooltip, and only
    /// a refusal they could not have foreseen belongs in the console.
    ///
    /// [`LogAnchor::Schematic`] and [`LogAnchor::Symbol`] carry geometry
    /// captured at the finding, so they always have somewhere to go — the
    /// staleness question is only asked of anchors that resolve by name.
    pub(crate) fn log_anchor_refusal(&self, anchor: &LogAnchor) -> Option<String> {
        match anchor {
            LogAnchor::Schematic { .. } | LogAnchor::Symbol { .. } => None,
            LogAnchor::Simulation { nets, devices } => {
                let named = nets
                    .iter()
                    .chain(devices.iter())
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(", ");
                match crate::schematic::view::drawn_failure_site_count(self, nets, devices) {
                    Err(error) => Some(error.message(&named)),
                    Ok(0) => Some(format!(
                        "This sheet draws none of the objects that run named ({named})."
                    )),
                    Ok(_) => None,
                }
            }
            LogAnchor::ResultRun { run_sequence } => self
                .model
                .simulation
                .retained
                .runs
                .iter()
                .all(|run| run.id != *run_sequence)
                .then(|| RetainedSimulationState::retired_run_message(*run_sequence)),
        }
    }

    /// Mark the objects the analysis on display was attributed to.
    ///
    /// The command form of [`highlight_failed_run_sites`], taking its
    /// subject from the selected result so any surface can offer it without
    /// re-deriving which run is being looked at.
    pub(crate) fn highlight_active_failure_sites(&mut self) -> bool {
        let Some(attribution) = self
            .model
            .simulation
            .active_analysis()
            .and_then(|analysis| analysis.failure_attribution.clone())
        else {
            return false;
        };
        let nets: Vec<String> = attribution.nets().map(str::to_owned).collect();
        let devices: Vec<String> = attribution.devices().map(str::to_owned).collect();
        highlight_failed_run_sites(&mut self.model, &mut self.workbench, &nets, &devices)
    }

    pub(crate) fn generate_active_symbol_document(&mut self) -> Result<(), String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let before = self.model.load_active_symbol_document()?;
        let ports = self.active_symbol_ports();
        let document = self
            .model
            .generated_symbol_document(&self.model.workspace.content.active_view, &ports);
        if document != before {
            self.record_symbol_metadata_edit(&before);
        }
        self.store_active_symbol_document(&document)?;
        Ok(())
    }

    /// Migrate persisted sessions away from the legacy seeded "primitives"
    /// library: any workspace tab or schematic buffer pointing into it is
    /// moved to the user library (preserving drawn content), then the
    /// placeholder library is dropped. Without the workspace scrub,
    /// `ensure_library_model` would faithfully re-create the legacy cell
    /// the active tab points at on every launch.
    pub(crate) fn migrate_legacy_primitives(&mut self) {
        const LEGACY: &str = crate::state::LibraryManager::PRIMITIVES_LIBRARY;
        let user = crate::state::LibraryManager::USER_LIBRARY;
        let legacy_prefix = format!("{LEGACY}/");

        // Move schematic buffers (the actual drawn content) to user keys.
        let legacy_keys: Vec<String> = self
            .model
            .workspace
            .content
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&legacy_prefix))
            .cloned()
            .collect();
        for key in legacy_keys {
            let Some(buffer) = self.model.workspace.remove_schematic_editor(&key) else {
                continue;
            };
            let tail = &key[legacy_prefix.len()..];
            let new_key = format!("{user}/{tail}");
            if !self
                .model
                .workspace
                .content
                .schematic_buffers
                .contains_key(&new_key)
            {
                self.model
                    .workspace
                    .insert_schematic_editor(new_key, buffer);
            }

            // Make the migrated cell/view exist in the user library so the
            // browser lists it and tabs resolve.
            if let Some((cell_name, view_name)) = tail.split_once('/')
                && let Some(mut library) = self.model.library_manager.edit_library(user)
            {
                library.ensure_cell_view(
                    cell_name,
                    view_name,
                    crate::state::ViewType::Schematic,
                    "",
                );
            }
        }

        // Repoint tabs, breadcrumbs, and the active view.
        let remap = |reference: &mut CellViewRef| {
            if reference.library == LEGACY {
                reference.library = user.to_string();
            }
        };
        remap(&mut self.model.workspace.content.active_view);
        for open in &mut self.model.workspace.content.open_views {
            remap(&mut open.reference);
        }
        self.model
            .workspace
            .content
            .remap_occurrence_masters(&remap);

        self.model.library_manager.purge_legacy_primitives();
    }

    /// [`Model::sync_active_schematic_to_workspace`], stating what it has to
    /// say on the status bar.
    ///
    /// [`Model::sync_active_schematic_to_workspace`]: crate::workbench::app_state::Model::sync_active_schematic_to_workspace
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        sync_active_schematic(&mut self.model, &mut self.workbench);
    }

    /// [`restore_active_schematic_from_workspace`] for the session.
    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        restore_active_schematic_from_workspace(
            self.design.schematic_restore_parts(),
            &mut self.model,
            &self.workbench,
            &self.runs,
        );
    }

    /// [`NavigationParts::open_workspace_view`] for the session.
    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.navigation().open_workspace_view(reference);
    }

    /// [`NavigationParts::descend_into_instance`] for the session.
    pub(crate) fn descend_into_instance(
        &mut self,
        instance: Option<String>,
        reference: CellViewRef,
    ) {
        self.navigation().descend_into_instance(instance, reference);
    }

    /// What moving the canvas to another cell view or occurrence reads and
    /// writes of the session.
    pub(crate) fn navigation(&mut self) -> NavigationParts<'_> {
        self.design
            .navigation_parts(&mut self.model, &mut self.workbench, &self.runs)
    }

    /// [`deny_read_only_edit`] for the session's model, prompting on its
    /// status bar.
    pub(crate) fn deny_read_only_edit(&mut self) -> bool {
        deny_read_only_edit(&self.model, &mut self.workbench)
    }

    /// [`NavigationParts::ascend_workspace_level`] for the session.
    #[cfg(test)]
    pub(crate) fn ascend_workspace_level(&mut self) {
        self.navigation().ascend_workspace_level();
    }

    /// [`NavigationParts::focus_workspace_breadcrumb`] for the session.
    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        self.navigation().focus_workspace_breadcrumb(index);
    }

    /// [`bump_active_schematic_epoch`] for the session.
    pub(crate) fn bump_active_schematic_epoch(&mut self) {
        bump_active_schematic_epoch(&mut self.design.symbol, &mut self.model, &self.workbench);
    }

    /// [`hierarchy_master_for_component`] for the session's model.
    pub(crate) fn hierarchy_master_for_component(
        &self,
        component_id: u64,
    ) -> Option<(String, CellViewRef)> {
        hierarchy_master_for_component(&self.model, component_id)
    }

    /// [`NavigationParts::open_veriloga_source_for_component`] for the
    /// session.
    #[cfg(test)]
    pub(crate) fn open_veriloga_source_for_component(&mut self, component_id: u64) -> bool {
        self.navigation()
            .open_veriloga_source_for_component(component_id)
    }

    /// [`NavigationParts::open_selected_instance_master`] for the session.
    pub(crate) fn open_selected_instance_master(&mut self) {
        self.navigation().open_selected_instance_master();
    }

    /// Why the selected instance cannot be shown in the generated deck, or
    /// `None` when it can. Every surface that offers the jump asks this one
    /// question, so a disabled row, a disabled palette entry and the refusal
    /// the action itself would report cannot word the same block differently.
    #[cfg(test)]
    pub(crate) fn selected_instance_netlist_block(&self) -> Option<&'static str> {
        self.model.selected_instance_generated_line().err()
    }

    /// Open the generated deck at the selected instance's card, the way
    /// [`Self::open_selected_instance_master`] opens its schematic master.
    ///
    /// The Netlist workspace, the generated primary document and the caret
    /// move as one transaction, and the status bar states where the instance
    /// was found. The schematic selection is deliberately left alone: the netlist
    /// navigator's reveal reads it to bring the round trip back.
    pub(crate) fn show_selected_instance_in_netlist(&mut self) {
        let Ok(location) = self.model.selected_instance_generated_line() else {
            return;
        };
        crate::workbench::documents::netlist_document::open_generated_primary(self);
        self.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        self.workbench
            .activate(crate::workbench::state::Workspace::Netlist);
        self.code.netlist.cursor_line = location.line.saturating_sub(1);
        self.code.netlist.requested_line = Some(location.line);
        let announced = self.workbench.messages().format(
            if location.stale {
                crate::workbench::MessageId::NetlistShowInstanceStale
            } else {
                crate::workbench::MessageId::NetlistShowInstanceLocated
            },
            &[
                ("instance", location.instance.as_str()),
                ("line", &location.line.to_string()),
            ],
        );
        self.prompt(announced);
    }
}

/// Mark the design objects a failed run named, or say why it cannot.
///
/// The refusal is the same sentence the operating-point inspector uses
/// for the same condition, and for the same reason: the map that would
/// find these conductors describes a drawing that has since changed, so
/// pointing anywhere would be pointing at where a wire used to be. The
/// answer is a prompt, because it answers the gesture that asked.
pub(crate) fn highlight_failed_run_sites(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    nets: &[String],
    devices: &[String],
) -> bool {
    match crate::schematic::view::select_failure_sites(model, nets, devices) {
        Ok(selection) if selection.is_empty() => {
            wb.prompt(format!(
                "This sheet draws none of the objects that run named ({}).",
                selection.unlocated.join(", ")
            ));
            false
        }
        Ok(selection) => {
            wb.activate(crate::workbench::state::Workspace::Design);
            if !selection.unlocated.is_empty() {
                wb.prompt(format!(
                    "Marked {} of {} objects; this sheet does not draw {}.",
                    selection.nets.len() + selection.devices.len(),
                    nets.len() + devices.len(),
                    selection.unlocated.join(", ")
                ));
            }
            true
        }
        Err(error) => {
            let named = nets
                .iter()
                .chain(devices.iter())
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ");
            wb.prompt(error.message(&named));
            false
        }
    }
}

/// The openable Verilog-A view of the one selected instance.
pub(crate) fn selected_instance_veriloga_source(model: &Model) -> Option<CellViewRef> {
    model
        .schematic
        .session
        .editor
        .selection
        .single_component()
        .and_then(|component_id| veriloga_source_for_component(model, component_id))
}

/// Whether descending into an instance edits it in place under its parent
/// rather than opening its master on its own.
pub(crate) fn hierarchical_edit_in_place_enabled(model: &Model) -> bool {
    model
        .preferences
        .toggle(crate::workbench::TogglePreference::HierarchicalEditInPlace)
}

/// The Verilog-A view one instance is bound to, whether or not that view
/// resolves to a module yet. This says what kind of master the instance
/// has; [`veriloga_source_for_component`] says whether it can be
/// opened.
pub(crate) fn veriloga_master_of_component(
    model: &Model,
    component_id: u64,
) -> Option<CellViewRef> {
    let binding = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .filter(|component| component.kind == ComponentType::CellInstance)?
        .library_cell
        .as_ref()?;
    let reference = CellViewRef::new(
        binding.library.clone(),
        binding.cell.clone(),
        binding.view.clone(),
    );
    (model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
        == Some(ViewType::VerilogA))
    .then_some(reference)
}

/// Return the exact project-owned Verilog-A source view bound to one
/// schematic instance. Merely spelling a view `veriloga` is insufficient:
/// the library view, cell-view-owned source bundle, and selected module
/// contract must all resolve before navigation is offered.
pub(crate) fn veriloga_source_for_component(
    model: &Model,
    component_id: u64,
) -> Option<CellViewRef> {
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)?;
    if component.kind != ComponentType::CellInstance {
        return None;
    }
    let binding = component.library_cell.as_ref()?;
    let reference = CellViewRef::new(
        binding.library.clone(),
        binding.cell.clone(),
        binding.view.clone(),
    );
    crate::state::workspace::project_veriloga_binding_for_view(
        &model.workspace,
        &model.library_manager,
        &reference,
    )
    .ok()?;
    Some(reference)
}

/// The selected instance's name and the schematic master it descends into.
pub(crate) fn selected_hierarchy_master(model: &Model) -> Option<(String, CellViewRef)> {
    model
        .schematic
        .session
        .editor
        .selection
        .single_component()
        .and_then(|component_id| hierarchy_master_for_component(model, component_id))
}

/// Return the exact schematic master of one hierarchy instance.
///
/// A generic `CellInstance` can also bind a SPICE, symbol, or Verilog-A
/// view. Those bindings are valid design objects, but they are not
/// descendable hierarchy and must not enable the descend command.
pub(crate) fn hierarchy_master_for_component(
    model: &Model,
    component_id: u64,
) -> Option<(String, CellViewRef)> {
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)?;
    if component.kind != ComponentType::CellInstance {
        return None;
    }
    let binding = component.library_cell.as_ref()?;
    let reference = CellViewRef::new(
        binding.library.clone(),
        binding.cell.clone(),
        binding.view.clone(),
    );
    let view_type = model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)?;
    (view_type == ViewType::Schematic).then(|| (component.name.clone(), reference))
}

/// Names the owner of the refusal, in the order
/// [`Model::active_view_read_only`] consults them.
pub(crate) fn read_only_master_message(model: &Model) -> String {
    if model.safe_mode.project_read_only() {
        return "Safe mode opened this project read-only - restart without safe mode to edit"
            .to_owned();
    }
    if model.workspace.content.active_read_only_reference() {
        return "Read-only hierarchy reference - reopen the view in an editable context to modify it"
            .to_owned();
    }
    let library = &model.workspace.content.active_view.library;
    format!("Read-only - '{library}' masters cannot be edited")
}

/// Refuse an edit on a read-only view, with a prompt that names the
/// library. Returns true when the edit must be blocked.
pub(crate) fn deny_read_only_edit(model: &Model, wb: &mut WorkbenchSession) -> bool {
    if !model.schematic_edit_read_only() {
        return false;
    }
    let live_lock_holder = model
        .live_write_locks
        .schematic_views
        .get(&model.workspace.content.active_key())
        .cloned();
    let message = if model.safe_mode.project_read_only() {
        "Safe mode is read-only; no design data was changed.".to_owned()
    } else if model.active_view_read_only() {
        read_only_master_message(model)
    } else if let Some(holder) = live_lock_holder {
        format!("{holder} holds the write lease on this schematic; no design data was changed.")
    } else if model.live_write_locks.mirror {
        "This is the host's working copy; request the write lease from the live session \
         to edit."
            .to_owned()
    } else {
        "The active schematic is read-only; no design data was changed.".to_owned()
    };
    wb.prompt(message);
    true
}

/// Sync the active schematic into the workspace through
/// [`Model::sync_active_schematic_to_workspace`], stating what it has to say
/// on the status bar, over the parts it reaches.
///
/// [`Model::sync_active_schematic_to_workspace`]: crate::workbench::app_state::Model::sync_active_schematic_to_workspace
pub(crate) fn sync_active_schematic(model: &mut Model, workbench: &mut WorkbenchSession) {
    if let Some(prompt) = model.sync_active_schematic_to_workspace() {
        workbench.prompt(prompt);
    }
}

/// Resolve the user defaults into one explicit, project-portable document
/// policy. This is the only constructor production UI paths use for a new
/// schematic/testbench document. Existing buffers are never rewritten.
pub(crate) fn new_schematic_document(
    model: &Model,
    snap: &crate::state::SnapEngine,
    routing_mode: crate::state::WireRoutingMode,
) -> SchematicState {
    use crate::state::{SchematicGridPitch, WireJunctionPolicy};
    use crate::workbench::ChoicePreference;

    let mut document = rspice_design::schematic::document::SchematicDocument::default();
    // Snap targets and radius are device-local session preferences. A new
    // document inherits those exact choices, while its pitch is resolved
    // below into project-portable policy and then projected back into the
    // runtime engine.
    let preferences = &model.preferences;
    document.document_policy.grid_pitch = SchematicGridPitch::ALL
        .get(preferences.choice(ChoicePreference::SchematicGrid))
        .copied()
        .expect("schematic grid preference is normalized before use");
    // Whether the schematic may route at an angle is the document's; the
    // junction dots and drag selection the editor applies come from
    // Preferences live, so the document's copies of those are left alone.
    document.document_policy.wire_junctions = if preferences.any_angle_wire_routing() {
        WireJunctionPolicy::AnyAngle
    } else if preferences.automatic_junction_dots() {
        WireJunctionPolicy::OrthogonalAutomatic
    } else {
        WireJunctionPolicy::OrthogonalManual
    };
    document.document_policy.net_naming = preferences.new_schematic_net_naming();

    let mut schematic = SchematicState::from_document(document);
    schematic.session.editor.snap_engine = snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    schematic.session.editor.automatic_junctions = preferences.automatic_junction_dots();
    schematic.session.editor.selection_crossing = preferences.selection_crossing();
    // Routing comes from the preference, not from whatever the session
    // last drew with.
    let routing_mode = preferences.new_schematic_routing_mode(routing_mode);
    schematic
        .session
        .editor
        .wire_drawing
        .set_routing_mode(routing_mode);
    schematic.session.editor.bus_drawing.routing_mode = routing_mode;
    schematic
}

/// Rebuild the active schematic's rubber-band connections against the
/// symbol geometry that is rendered and netlisted.
pub(crate) fn rebuild_active_connections_from_symbols(model: &mut Model) {
    let terminals = {
        let symbols = model.schematic_symbol_context();
        model
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
            .collect::<Vec<_>>()
    };
    model
        .schematic
        .rebuild_connections_from_terminals(&terminals);
}

/// Take up the active view's schematic from the workspace's working copy,
/// as a history step or a reload left it.
pub(crate) fn restore_active_schematic_from_workspace(
    mut design: SchematicRestoreParts<'_>,
    model: &mut Model,
    wb: &WorkbenchSession,
    runs: &RunsState,
) {
    design.drag.cancel_schematic_drag(model);
    model
        .workspace
        .ensure_library_model(&mut model.library_manager);
    let reference = model.workspace.content.active_view.clone();
    let schematic_reference = model.workspace.content.active_schematic_reference();
    model.schematic = schematic_for_workspace(&mut design, model, &schematic_reference);
    bump_active_schematic_epoch(design.symbol, model, wb);
    // Project persistence stores topology, not the derived rubber-band
    // connection cache. Rebuild against the same authored/generated
    // symbol geometry that is rendered and netlisted.
    rebuild_active_connections_from_symbols(model);
    model.refresh_active_design_check_projection(runs.slot_facts(design.visibility.review_markers));
    model
        .library_manager
        .select_view(&reference.library, &reference.cell, &reference.view);
}

/// Mark the active schematic as a new document for the edits that read it,
/// and project the symbol editor's session onto the view now in front.
pub(crate) fn bump_active_schematic_epoch(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &WorkbenchSession,
) {
    model.active_schematic_epoch = model.active_schematic_epoch.wrapping_add(1);
    project_active_symbol_session(symbol, model, wb);
}

impl RSpiceApp {
    pub(crate) fn restore_workspace_after_project_load(&mut self) {
        self.state.restore_active_schematic_from_workspace();
        self.state
            .results
            .analysis
            .clear_transient_specialized_viewer_data();
    }
}

#[cfg(test)]
mod tests;
