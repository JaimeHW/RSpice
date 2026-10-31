//! Workspace-level edit actions.
//!
//! The clipboard, selection, undo/redo, and hierarchy operations that a menu
//! item or a shortcut resolves to. These act on the whole active document
//! rather than on one selected object, and they are the actions that a
//! read-only schematic refuses.

mod design_checks;
mod history;
pub(in crate::workbench) mod symbol_document;
pub(in crate::workbench) mod symbol_in_place;
mod waivers;

use super::hierarchy_net_highlight::NetCarry;
use crate::diagnostics::LogAnchor;
use crate::schematic::view::schematic_symbol_context;
use crate::state::{
    CellViewRef, ComponentType, Point, RetainedSimulationState, SchematicState, SymbolDocument,
    ViewType,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::state::WorkspaceDocumentId;

fn schematic_for_workspace(state: &mut AppState, reference: &CellViewRef) -> SchematicState {
    let existing = state
        .model
        .workspace
        .clone_schematic_editor(&reference.key());
    let created = existing.is_none();
    let mut schematic = existing.unwrap_or_else(|| state.new_schematic_document());
    // Workspace buffers round-trip through serde, which skips the runtime
    // ID counter and name counters: without recalculation a freshly placed
    // component reuses an existing ID and selection matches both.
    schematic.recalculate_runtime_state();
    schematic.session.editor.snap_engine = state.design.schematic_snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    state.design.schematic_snap.grid_size = schematic.document().grid_size;
    schematic.session.editor.automatic_junctions =
        state.model.preferences.automatic_junction_dots();
    schematic.session.editor.selection_crossing = state.model.preferences.selection_crossing();
    if created {
        // A new schematic starts from the routing preference, and the
        // session's routing control follows it.
        let routing_mode = schematic.session.editor.wire_drawing.routing_mode;
        state.design.schematic_routing_mode = routing_mode;
        state.design.schematic_visibility.wire_routing =
            crate::workbench::app::dialogs::grid_snap_routing::routing_style(routing_mode);
    } else {
        schematic.session.editor.wire_drawing.routing_mode = state.design.schematic_routing_mode;
        schematic.session.editor.bus_drawing.routing_mode = state.design.schematic_routing_mode;
    }
    // Views from read-only libraries open for inspection, never for edit —
    // the docbar banner explains and every edit path checks this flag.
    schematic.session.read_only = state.model.safe_mode.project_read_only()
        || state
            .model
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| library.read_only);
    schematic
}

impl AppState {
    /// Copy the active complete-object selection using the exact rendered
    /// terminal geometry of authored and generated symbols.
    ///
    /// `SchematicState::copy_selection` remains a state-only fallback for
    /// primitive fixtures. Production UI commands must enter here so a wire
    /// joining two authored cell pins is captured with the selected
    /// instances instead of being silently omitted. The copy goes to the
    /// app's clipboard, which outlives the cell view it came from.
    pub(crate) fn copy_active_schematic_selection(&mut self) -> bool {
        let Some(copy) = self.capture_active_schematic_selection() else {
            return false;
        };
        self.design.schematic_clipboard = copy;
        true
    }

    /// The active sheet's selection as a clipboard would hold it, or `None`
    /// when nothing is selected. Nothing is stored anywhere.
    pub(crate) fn capture_active_schematic_selection(
        &self,
    ) -> Option<rspice_design::schematic::clipboard::ClipboardData> {
        if self.model.schematic.session.editor.selection.is_empty() {
            return None;
        }
        let symbols = schematic_symbol_context(self);
        let on_sheet =
            |id| crate::schematic::view::sheet_visibility::object_is_on_active_sheet(self, id);
        Some(self.model.schematic.capture_complete_selection_resolved(
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

    pub(crate) fn rebuild_active_connections_from_symbols(&mut self) {
        let terminals = {
            let symbols = schematic_symbol_context(self);
            self.model
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
        self.model
            .schematic
            .rebuild_connections_from_terminals(&terminals);
    }

    /// Resolve the user defaults into one explicit, project-portable document
    /// policy. This is the only constructor production UI paths use for a new
    /// schematic/testbench document. Existing buffers are never rewritten.
    pub(crate) fn new_schematic_document(&self) -> SchematicState {
        use crate::state::{SchematicGridPitch, WireJunctionPolicy};
        use crate::workbench::ChoicePreference;

        let mut document = rspice_design::schematic::document::SchematicDocument::default();
        // Snap targets and radius are device-local session preferences. A new
        // document inherits those exact choices, while its pitch is resolved
        // below into project-portable policy and then projected back into the
        // runtime engine.
        let preferences = &self.model.preferences;
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
        schematic.session.editor.snap_engine = self.design.schematic_snap.clone();
        schematic.reconcile_grid_pitch_runtime();
        schematic.session.editor.automatic_junctions = preferences.automatic_junction_dots();
        schematic.session.editor.selection_crossing = preferences.selection_crossing();
        // Routing comes from the preference, not from whatever the session
        // last drew with.
        let routing_mode =
            preferences.new_schematic_routing_mode(self.design.schematic_routing_mode);
        schematic
            .session
            .editor
            .wire_drawing
            .set_routing_mode(routing_mode);
        schematic.session.editor.bus_drawing.routing_mode = routing_mode;
        schematic
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

    /// Names the owner of the refusal, in the order
    /// [`Self::active_view_read_only`] consults them.
    pub(crate) fn read_only_master_message(&self) -> String {
        if self.model.safe_mode.project_read_only() {
            return "Safe mode opened this project read-only - restart without safe mode to edit"
                .to_owned();
        }
        if self.model.workspace.content.active_read_only_reference() {
            return "Read-only hierarchy reference - reopen the view in an editable context to modify it"
                .to_owned();
        }
        let library = &self.model.workspace.content.active_view.library;
        format!("Read-only - '{library}' masters cannot be edited")
    }

    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        self.navigate_to_log_anchor(anchor);
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

    fn navigate_to_log_anchor(&mut self, anchor: LogAnchor) {
        match anchor {
            LogAnchor::Schematic {
                cell_view,
                x,
                y,
                component,
                wire,
                sheet,
            } => {
                // The point and the object are the drawing's own, so the
                // drawing opens first. A view that will not open — an edit
                // in the inspector that cannot commit — leaves the reader
                // where they were rather than selecting in the wrong view.
                if !cell_view.key().eq_ignore_ascii_case(
                    &self
                        .model
                        .workspace
                        .content
                        .active_schematic_reference()
                        .key(),
                ) {
                    self.open_workspace_view(cell_view.clone());
                    if !cell_view.key().eq_ignore_ascii_case(
                        &self
                            .model
                            .workspace
                            .content
                            .active_schematic_reference()
                            .key(),
                    ) {
                        self.prompt(format!(
                            "{} could not be opened, so the finding was not followed.",
                            cell_view.display_path()
                        ));
                        return;
                    }
                }
                // An object on another sheet is drawn nowhere until its sheet
                // is in view, and selecting it there would select something
                // the reader cannot see. A sheet that will not come forward
                // has said why, and the reader stays where they were.
                if let Some(sheet) = sheet
                    && self.model.active_sheet_id() != Some(sheet)
                {
                    let outcome = self
                        .model
                        .activate_sheet(sheet, self.design.interaction.drag.let_go());
                    if self.workbench.report(outcome).is_err() {
                        return;
                    }
                }
                self.workbench
                    .activate(crate::workbench::state::Workspace::Design);
                self.model.schematic.session.editor.center_request = Some(Point::new(x, y));
                self.model.schematic.session.editor.net_highlight.clear();
                self.model.schematic.session.editor.selection.clear();
                if let Some(id) = component {
                    self.model
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_component(id);
                }
                if let Some(id) = wire {
                    self.model
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_wire(id);
                }
            }
            LogAnchor::Symbol {
                reference,
                pin_name,
                point,
            } => {
                self.open_workspace_view(reference);
                self.workbench
                    .activate(crate::workbench::state::Workspace::Design);
                self.design.symbol.editor.select_pin(pin_name);
                if let Some(point) = point {
                    let zoom = self.design.symbol.editor.zoom.max(1.0);
                    self.design.symbol.editor.pan =
                        (-(point.x as f32) * zoom, -(point.y as f32) * zoom);
                }
                self.design.symbol.editor.needs_fit = false;
            }
            LogAnchor::Simulation { nets, devices } => {
                self.highlight_failed_run_sites(&nets, &devices);
            }
            LogAnchor::ResultRun { run_sequence } => {
                // Select first, activate second: landing in Results on
                // whichever run happened to be selected would be a different
                // dataset than the row named.
                if self.model.simulation.select_run_by_sequence(run_sequence) {
                    self.workbench
                        .activate(crate::workbench::state::Workspace::Results);
                } else {
                    self.prompt(RetainedSimulationState::retired_run_message(run_sequence));
                }
            }
        }
    }

    /// Mark the objects the analysis on display was attributed to.
    ///
    /// The command form of [`Self::highlight_failed_run_sites`], taking its
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
        self.highlight_failed_run_sites(&nets, &devices)
    }

    /// Mark the design objects a failed run named, or say why it cannot.
    ///
    /// The refusal is the same sentence the operating-point inspector uses
    /// for the same condition, and for the same reason: the map that would
    /// find these conductors describes a drawing that has since changed, so
    /// pointing anywhere would be pointing at where a wire used to be. The
    /// answer is a prompt, because it answers the gesture that asked.
    pub(crate) fn highlight_failed_run_sites(
        &mut self,
        nets: &[String],
        devices: &[String],
    ) -> bool {
        match crate::schematic::view::select_failure_sites(self, nets, devices) {
            Ok(selection) if selection.is_empty() => {
                self.prompt(format!(
                    "This sheet draws none of the objects that run named ({}).",
                    selection.unlocated.join(", ")
                ));
                false
            }
            Ok(selection) => {
                self.workbench
                    .activate(crate::workbench::state::Workspace::Design);
                if !selection.unlocated.is_empty() {
                    self.prompt(format!(
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
                self.prompt(error.message(&named));
                false
            }
        }
    }

    pub(crate) fn generate_active_symbol_document(&mut self) -> Result<(), String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let before = self.load_active_symbol_document()?;
        let ports = self.active_symbol_ports();
        let document = SymbolDocument::generated_from_ports(&ports);
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
        if let Some(prompt) = self.model.sync_active_schematic_to_workspace() {
            self.prompt(prompt);
        }
    }

    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        self.cancel_schematic_drag();
        self.model
            .workspace
            .ensure_library_model(&mut self.model.library_manager);
        let reference = self.model.workspace.content.active_view.clone();
        let schematic_reference = self.model.workspace.content.active_schematic_reference();
        self.model.schematic = schematic_for_workspace(self, &schematic_reference);
        self.bump_active_schematic_epoch();
        // Project persistence stores topology, not the derived rubber-band
        // connection cache. Rebuild against the same authored/generated
        // symbol geometry that is rendered and netlisted.
        self.rebuild_active_connections_from_symbols();
        self.refresh_active_design_check_projection();
        self.model.library_manager.select_view(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
    }

    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        if self.model.workspace.content.active_view != reference {
            if !self.commit_pending_inspector_edit() {
                return;
            }
            self.cancel_schematic_drag();
        }
        self.sync_active_schematic_to_workspace();
        if self.model.workspace.content.active_view == reference {
            self.workbench
                .documents
                .activate(WorkspaceDocumentId::CellView(reference));
            return;
        }
        self.design.canvas_hover = None;
        self.design.canvas_view_center = None;
        let view_type = self.model.cell_view_type(&reference);
        self.model
            .workspace
            .activate_view(reference.clone(), view_type);
        self.workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference.clone()));
        self.model.library_manager.select_view(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        let schematic_reference = self.model.workspace.content.active_schematic_reference();
        self.model.schematic = schematic_for_workspace(self, &schematic_reference);
        self.bump_active_schematic_epoch();
        self.refresh_active_design_check_projection();
        self.prompt(format!("Opened {}.", reference.display_path()));
    }

    /// Descend into a hierarchical instance: open its master and record the
    /// instance name on the active document's occurrence. Without an instance
    /// there is no occurrence step to record, so the master opens as its own
    /// design root rather than inheriting a name it was not reached by.
    pub(crate) fn descend_into_instance(
        &mut self,
        instance: Option<String>,
        reference: CellViewRef,
    ) {
        if !self.commit_pending_inspector_edit() {
            return;
        }
        self.cancel_schematic_drag();
        self.sync_active_schematic_to_workspace();
        let carried = self.leave_occurrence_net_highlight(match &instance {
            Some(name) => NetCarry::Descend(name.clone()),
            None => NetCarry::Nothing,
        });
        let view_type = self.model.cell_view_type(&reference);
        match instance {
            Some(name) => self
                .model
                .workspace
                .descend_into(name, reference.clone(), view_type),
            None => self
                .model
                .workspace
                .open_as_root(reference.clone(), view_type),
        }
        self.model
            .workspace
            .content
            .set_active_read_only_reference(false);
        self.model.library_manager.select_view(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        let schematic_reference = self.model.workspace.content.active_schematic_reference();
        self.model.schematic = schematic_for_workspace(self, &schematic_reference);
        self.bump_active_schematic_epoch();
        self.refresh_active_design_check_projection();
        self.enter_occurrence_net_highlight(carried);
        self.prompt(format!("Entered {}.", reference.display_path()));
    }

    /// Refuse an edit on a read-only view, with a prompt that names the
    /// library. Returns true when the edit must be blocked.
    pub(crate) fn deny_read_only_edit(&mut self) -> bool {
        if !self.schematic_edit_read_only() {
            return false;
        }
        let live_lock_holder = self
            .model
            .live_write_locks
            .schematic_views
            .get(&self.model.workspace.content.active_key())
            .cloned();
        let message = if self.model.safe_mode.project_read_only() {
            "Safe mode is read-only; no design data was changed.".to_owned()
        } else if self.active_view_read_only() {
            self.read_only_master_message()
        } else if let Some(holder) = live_lock_holder {
            format!("{holder} holds the write lease on this schematic; no design data was changed.")
        } else if self.model.live_write_locks.mirror {
            "This is the host's working copy; request the write lease from the live session \
             to edit."
                .to_owned()
        } else {
            "The active schematic is read-only; no design data was changed.".to_owned()
        };
        self.prompt(message);
        true
    }

    /// Ascend one hierarchy level (the U gesture / pathbar action).
    pub(crate) fn ascend_workspace_level(&mut self) {
        let depth = self.model.workspace.content.occurrence_depth();
        if depth >= 2 {
            self.focus_workspace_breadcrumb(depth - 2);
        }
    }

    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        if !self.commit_pending_inspector_edit() {
            return;
        }
        self.cancel_schematic_drag();
        self.sync_active_schematic_to_workspace();
        // One level up carries the highlight out through the port; a longer
        // jump restores what the level held.
        let carried = self.leave_occurrence_net_highlight(
            if index + 2 == self.model.workspace.content.occurrence_depth() {
                NetCarry::Ascend
            } else {
                NetCarry::Nothing
            },
        );
        if let Some(reference) = self.model.workspace.focus_breadcrumb(index) {
            self.model.library_manager.select_view(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
            self.model.schematic = schematic_for_workspace(self, &reference);
            self.bump_active_schematic_epoch();
            self.refresh_active_design_check_projection();
            self.enter_occurrence_net_highlight(carried);
        }
    }

    pub(crate) fn bump_active_schematic_epoch(&mut self) {
        self.model.active_schematic_epoch = self.model.active_schematic_epoch.wrapping_add(1);
        self.project_active_symbol_session();
    }

    /// Return the exact schematic master of one hierarchy instance.
    ///
    /// A generic `CellInstance` can also bind a SPICE, symbol, or Verilog-A
    /// view. Those bindings are valid design objects, but they are not
    /// descendable hierarchy and must not enable the descend command.
    pub(crate) fn hierarchy_master_for_component(
        &self,
        component_id: u64,
    ) -> Option<(String, CellViewRef)> {
        let component = self
            .model
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
        let view_type = self
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(|view| view.view_type)?;
        (view_type == ViewType::Schematic).then(|| (component.name.clone(), reference))
    }

    /// Return the exact project-owned Verilog-A source view bound to one
    /// schematic instance. Merely spelling a view `veriloga` is insufficient:
    /// the library view, cell-view-owned source bundle, and selected module
    /// contract must all resolve before navigation is offered.
    pub(crate) fn veriloga_source_for_component(&self, component_id: u64) -> Option<CellViewRef> {
        let component = self
            .model
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
            &self.model.workspace,
            &self.model.library_manager,
            &reference,
        )
        .ok()?;
        Some(reference)
    }

    /// Open the exact Verilog-A source owned by a bound schematic instance.
    /// Resolution happens before any workspace mutation, so stale or partial
    /// bindings fail closed and leave the active document untouched.
    pub(crate) fn open_veriloga_source_for_component(&mut self, component_id: u64) -> bool {
        let Some(reference) = self.veriloga_source_for_component(component_id) else {
            return false;
        };
        // The module is a view of the cell, so it opens as a Design document
        // beside the schematic it was reached from.
        self.open_workspace_view(reference);
        self.workbench
            .activate(crate::workbench::state::Workspace::Design);
        true
    }

    pub(crate) fn selected_hierarchy_master(&self) -> Option<(String, CellViewRef)> {
        self.model
            .schematic
            .session
            .editor
            .selection
            .single_component()
            .and_then(|component_id| self.hierarchy_master_for_component(component_id))
    }

    /// The Verilog-A view one instance is bound to, whether or not that view
    /// resolves to a module yet. This says what kind of master the instance
    /// has; [`Self::veriloga_source_for_component`] says whether it can be
    /// opened.
    pub(crate) fn veriloga_master_of_component(&self, component_id: u64) -> Option<CellViewRef> {
        let binding = self
            .model
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
        (self
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(|view| view.view_type)
            == Some(ViewType::VerilogA))
        .then_some(reference)
    }

    /// The openable Verilog-A view of the one selected instance.
    pub(crate) fn selected_veriloga_source(&self) -> Option<CellViewRef> {
        self.model
            .schematic
            .session
            .editor
            .selection
            .single_component()
            .and_then(|component_id| self.veriloga_source_for_component(component_id))
    }

    pub(crate) fn hierarchical_edit_in_place_enabled(&self) -> bool {
        self.model
            .preferences
            .toggle(crate::workbench::TogglePreference::HierarchicalEditInPlace)
    }

    /// Open what the one selected instance is an instance of: descend into
    /// its schematic master, or, for a Verilog-A instance, open its module's
    /// cell view. A module has no occurrences below it, so that is not a
    /// descent and records no occurrence step.
    pub(crate) fn open_selected_instance_master(&mut self) {
        if let Some(component_id) = self
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component()
            && self.open_veriloga_source_for_component(component_id)
        {
            return;
        }
        let Some((instance_name, reference)) = self.selected_hierarchy_master() else {
            self.prompt("Select one instance with a resolved schematic master first.");
            return;
        };
        if self.hierarchical_edit_in_place_enabled() {
            self.descend_into_instance(Some(instance_name), reference);
        } else {
            self.open_workspace_view(reference);
        }
    }

    /// Why the selected instance cannot be shown in the generated deck, or
    /// `None` when it can. Every surface that offers the jump asks this one
    /// question, so a disabled row, a disabled palette entry and the refusal
    /// the action itself would report cannot word the same block differently.
    pub(crate) fn selected_instance_netlist_block(&self) -> Option<&'static str> {
        selected_instance_generated_line(self).err()
    }

    /// Open the generated deck at the selected instance's card, the way
    /// [`Self::open_selected_instance_master`] opens its schematic master.
    ///
    /// The Netlist workspace, the generated primary document and the caret
    /// move as one transaction, and the status bar states where the instance
    /// was found. The schematic selection is deliberately left alone: the netlist
    /// navigator's reveal reads it to bring the round trip back.
    pub(crate) fn show_selected_instance_in_netlist(&mut self) {
        let Ok(location) = selected_instance_generated_line(self) else {
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

/// Where one selected schematic instance appears in the retained generated
/// deck, and whether that deck still matches the project it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedInstanceLocation {
    /// One-based generated line carrying the instance's own card.
    line: usize,
    /// Emitted SPICE identity written on that line.
    instance: String,
    /// Whether the retained artifact predates the current project input.
    stale: bool,
}

/// Locate the selected schematic instance in the retained generated deck.
///
/// The error is the exact reason the jump is unavailable, worded for the
/// disabled control that reports it, so availability and execution can never
/// disagree about why. A stale artifact still resolves — it is what the
/// project last generated — and the caller says so rather than refusing.
fn selected_instance_generated_line(
    state: &AppState,
) -> Result<GeneratedInstanceLocation, &'static str> {
    const NO_DECK: &str = "no generated netlist yet — open the Netlist workspace to generate one";

    let component_id = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_component()
        .ok_or("select one instance")?;
    if state
        .model
        .netlist_artifacts
        .generated_source
        .trim()
        .is_empty()
    {
        return Err(NO_DECK);
    }
    let artifact = state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(crate::state::NetlistDocument::generated_artifact)
        .ok_or(NO_DECK)?;
    let identity = crate::state::GeneratedSourceMapEntry::component_identity_for(
        &state.model.workspace.content.active_view.key(),
        component_id,
    );
    let line = artifact
        .generated_lines_for_component(&identity)
        .first()
        .copied()
        .ok_or("no netlist line for this instance")?;
    Ok(GeneratedInstanceLocation {
        line,
        instance: artifact
            .source_map_entry(line)
            .and_then(crate::state::GeneratedSourceMapEntry::instance_identity)
            .unwrap_or_default()
            .to_owned(),
        stale: state.model.netlist_artifacts.generated_input_digest
            != state
                .model
                .netlist_artifacts
                .current_generation_input_digest,
    })
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
