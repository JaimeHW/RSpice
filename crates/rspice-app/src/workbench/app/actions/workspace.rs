//! Workspace-level edit actions.
//!
//! The clipboard, selection, undo/redo, and hierarchy operations that a menu
//! item or a shortcut resolves to. These act on the whole active document
//! rather than on one selected object, and they are the actions that a
//! read-only schematic refuses.

mod design_checks;
mod history;
mod libraries;
pub(in crate::workbench) mod symbol_document;
pub(in crate::workbench) mod symbol_in_place;
mod waivers;

use super::hierarchy_net_highlight::NetCarry;
use crate::diagnostics::LogAnchor;
use crate::schematic::view::schematic_symbol_context;
use crate::state::{
    CellViewRef, ComponentType, OpenCellView, Point, SchematicState, SymbolDocument, View, ViewType,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::state::WorkspaceDocumentId;

fn view_type_for_reference(state: &AppState, reference: &CellViewRef) -> ViewType {
    state
        .model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
        .unwrap_or(ViewType::Schematic)
}

fn reference_exists_in(libraries: &crate::state::LibraryManager, reference: &CellViewRef) -> bool {
    libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .is_some()
}

fn first_schematic_reference_in(libraries: &crate::state::LibraryManager) -> Option<CellViewRef> {
    for library in libraries.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell.views_sorted() {
                if is_schematic_like(view.view_type) {
                    return Some(CellViewRef::new(&library.name, &cell.name, &view.name));
                }
            }
        }
    }
    None
}

fn is_schematic_like(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

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
                .then(|| Self::retired_run_message(*run_sequence)),
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
                    && super::sheets::active_sheet_id(self) != Some(sheet)
                    && super::sheets::activate_sheet(self, sheet).is_err()
                {
                    return;
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
                    self.prompt(Self::retired_run_message(run_sequence));
                }
            }
        }
    }

    /// Why a run this session no longer holds cannot be opened.
    ///
    /// One sentence for one condition, wherever it is asked — the notice
    /// centre's offer and a console row's anchor name the same run the same
    /// way, so they must refuse it the same way too.
    pub(crate) fn retired_run_message(run_sequence: u64) -> String {
        format!(
            "Run {run_sequence} is no longer retained in this session, so it could not be opened \
             in Results."
        )
    }

    /// Whether the analysis on display named objects worth marking.
    ///
    /// A surface offering "highlight non-converged nodes" asks this to decide
    /// whether the control belongs on screen at all. It answers about the
    /// evidence, not about the schematic: whether the marking will land is
    /// [`Self::highlight_active_failure_sites`]'s answer, and it is a
    /// different question with a different sentence.
    ///
    /// The Results operational-status banner is the surface that asks it.
    pub(crate) fn active_failure_names_objects(&self) -> bool {
        self.model
            .simulation
            .active_analysis()
            .and_then(|analysis| analysis.failure_attribution.as_ref())
            .is_some_and(|attribution| !attribution.sites.is_empty())
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

    /// Take back a marking [`Self::highlight_active_failure_sites`] made.
    ///
    /// The exact inverse of what the marking did, and no more: the highlight
    /// and the selection go, the view stays where the reader left it. Scrolling
    /// them back to wherever they were before would be a second surprise
    /// rather than an undo of the first.
    pub(crate) fn clear_failure_site_marking(&mut self) {
        self.model.schematic.session.editor.net_highlight.clear();
        self.model.schematic.session.editor.selection.clear();
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
        let view_type = view_type_for_reference(self, &reference);
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
        let view_type = view_type_for_reference(self, &reference);
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

    /// Take a deleted cell out of every workspace collection that named it.
    ///
    /// Every view of the cell goes, not just the schematic ones: a layout
    /// document the cell no longer owns is a document the next save refuses to
    /// write. What survives is the placements — deleting a master is not a
    /// licence to edit the drawings that used it — so they are revalidated
    /// here instead, and read as unresolved from this point on.
    pub(crate) fn prune_workspace_after_cell_deleted(&mut self, library: &str, cell: &str) {
        self.sync_active_schematic_to_workspace();
        let removal = self.model.drop_workspace_references_to_cell(library, cell);
        self.restore_valid_workspace_focus_after_prune(
            removal.active_removed,
            removal.hierarchy_pruned,
            removal.project_root_removed,
            true,
        );
    }

    pub(crate) fn prune_workspace_after_view_deleted(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) {
        self.sync_active_schematic_to_workspace();
        let deleted = CellViewRef::new(library, cell, view);
        let active_removed = self.model.workspace.content.active_view == deleted;
        let project_root_removed = self.model.workspace.content.project.root_library == library
            && self.model.workspace.content.project.top_cell == cell
            && view == crate::state::workspace::DEFAULT_SCHEMATIC_VIEW;
        self.model.workspace.remove_schematic_editor(&deleted.key());
        self.model
            .workspace
            .content
            .remove_physical_layout_document(&deleted);
        self.model
            .workspace
            .content
            .open_views
            .retain(|open| open.reference != deleted);
        let hierarchy_pruned = self
            .model
            .workspace
            .content
            .retain_valid_occurrences(|reference| reference != &deleted);

        let preferred = CellViewRef::new(
            library,
            cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let preferred =
            reference_exists_in(&self.model.library_manager, &preferred).then_some(preferred);
        self.restore_valid_workspace_focus_with_preferred(
            active_removed,
            hierarchy_pruned,
            preferred,
            project_root_removed,
            true,
        );
    }

    fn restore_valid_workspace_focus_after_prune(
        &mut self,
        active_removed: bool,
        hierarchy_pruned: bool,
        repoint_project_root: bool,
        clear_execution_context: bool,
    ) {
        self.restore_valid_workspace_focus_with_preferred(
            active_removed,
            hierarchy_pruned,
            None,
            repoint_project_root,
            clear_execution_context,
        );
    }

    fn restore_valid_workspace_focus_with_preferred(
        &mut self,
        active_removed: bool,
        hierarchy_pruned: bool,
        preferred: Option<CellViewRef>,
        repoint_project_root: bool,
        clear_execution_context: bool,
    ) {
        let libraries = &self.model.library_manager;
        self.model
            .workspace
            .content
            .open_views
            .retain(|open| reference_exists_in(libraries, &open.reference));
        self.model
            .workspace
            .content
            .retain_valid_occurrences(|reference| reference_exists_in(libraries, reference));

        let active_valid = !active_removed
            && reference_exists_in(
                &self.model.library_manager,
                &self.model.workspace.content.active_view,
            );
        let fallback = if active_valid {
            self.model.workspace.content.active_view.clone()
        } else {
            preferred
                .filter(|reference| reference_exists_in(&self.model.library_manager, reference))
                .or_else(|| {
                    self.model
                        .workspace
                        .content
                        .open_views
                        .iter()
                        .find(|open| {
                            reference_exists_in(&self.model.library_manager, &open.reference)
                        })
                        .map(|open| open.reference.clone())
                })
                .unwrap_or_else(|| self.create_fallback_schematic_cell())
        };
        let fallback_type = view_type_for_reference(self, &fallback);

        if !self
            .model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == fallback)
        {
            self.model
                .workspace
                .content
                .open_views
                .push(OpenCellView::new(fallback.clone(), fallback_type));
        }
        if !active_valid {
            self.model.workspace.content.active_view = fallback.clone();
        }
        // Whatever the restored document was reached through did not survive
        // the prune, so it is a design root again rather than an occurrence
        // whose ancestors are gone.
        if hierarchy_pruned || !active_valid {
            self.model.workspace.content.reroot_active_occurrence();
        }
        if is_schematic_like(fallback_type) {
            self.model.workspace.ensure_active_buffer();
        }
        if repoint_project_root {
            self.repoint_project_root_to_surviving_schematic(&fallback, fallback_type);
        }
        self.restore_active_schematic_from_workspace();
        if clear_execution_context {
            self.clear_design_execution_context();
        }
    }

    fn repoint_project_root_to_surviving_schematic(
        &mut self,
        fallback: &CellViewRef,
        fallback_type: ViewType,
    ) {
        let root = if is_schematic_like(fallback_type)
            && reference_exists_in(&self.model.library_manager, fallback)
        {
            fallback.clone()
        } else {
            self.model
                .workspace
                .content
                .open_views
                .iter()
                .find(|open| {
                    is_schematic_like(open.view_type)
                        && reference_exists_in(&self.model.library_manager, &open.reference)
                })
                .map(|open| open.reference.clone())
                .or_else(|| first_schematic_reference_in(&self.model.library_manager))
                .unwrap_or_else(|| self.create_fallback_schematic_cell())
        };
        self.model.workspace.content.project.root_library = root.library;
        self.model.workspace.content.project.top_cell = root.cell;
    }

    fn create_fallback_schematic_cell(&mut self) -> CellViewRef {
        let library_name = crate::state::workspace::DEFAULT_PROJECT_LIBRARY.to_string();
        let view_name = crate::state::workspace::DEFAULT_SCHEMATIC_VIEW.to_string();
        if self
            .model
            .library_manager
            .get_library(&library_name)
            .is_none()
        {
            self.model
                .library_manager
                .add_library(crate::state::Library::new(&library_name));
        }

        let mut index = 1usize;
        let cell_name = loop {
            let candidate = format!("untitled_{index}");
            let exists = self
                .model
                .library_manager
                .get_library(&library_name)
                .and_then(|library| library.get_cell(&candidate))
                .is_some();
            if !exists {
                break candidate;
            }
            index += 1;
        };

        if let Some(mut library) = self.model.library_manager.edit_library(&library_name) {
            let mut cell = crate::state::Cell::new(&cell_name);
            cell.add_view(View::new(&view_name, ViewType::Schematic));
            library.add_cell(cell);
        }

        let reference = CellViewRef::new(library_name, cell_name, view_name);
        self.model.workspace.content.project.root_library = reference.library.clone();
        self.model.workspace.content.project.top_cell = reference.cell.clone();
        if !self
            .model
            .workspace
            .content
            .schematic_buffers
            .contains_key(&reference.key())
        {
            self.model
                .workspace
                .insert_schematic_editor(reference.key(), SchematicState::default());
        }
        reference
    }

    /// Copy a whole cell — every view and its drawn content — into a
    /// writable library under a new name. Returns the number of views
    /// copied, or the user-facing error.
    pub(crate) fn copy_cell(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let source = self
            .model
            .library_manager
            .get_library(src_library)
            .ok_or_else(|| format!("Library '{src_library}' not found"))?;
        let mut copy = source
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{cell}' not found in library '{src_library}'"))?
            .clone();
        let destination = self
            .model
            .library_manager
            .get_library(dst_library)
            .ok_or_else(|| format!("Library '{dst_library}' not found"))?;
        if destination.read_only {
            return Err(format!("Library '{dst_library}' is read-only"));
        }
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(dst_library, new_name, "");
        if destination.cells.values().any(|candidate| {
            crate::state::canonical_cell_view_owner_key(dst_library, &candidate.name, "")
                == requested_identity
        }) {
            return Err(format!(
                "Cell '{new_name}' conflicts with an existing cell in library '{dst_library}' whose name differs only in case or normalization"
            ));
        }

        self.sync_active_schematic_to_workspace();

        let mut candidate_sources = self.model.workspace.content.project_sources.clone();
        let copied_source_ids = candidate_sources
            .clone_cell_view_bundles(src_library, cell, dst_library, new_name)
            .map_err(|error| {
                format!("Could not copy the cell's project source bundles: {error}")
            })?;
        let mut candidate_design_management =
            self.model.workspace.content.design_management.clone();
        let copied_sheet_catalogs = candidate_design_management
            .copy_cell_sheet_catalogs(src_library, cell, dst_library, new_name)
            .map_err(|error| format!("Could not copy the cell's sheet-catalog ownership: {error}"))?
            .copied_sheet_catalogs;
        let candidate_layouts = self
            .model
            .workspace
            .content
            .prepare_copy_physical_layout_cell_documents(src_library, cell, dst_library, new_name)
            .map_err(|error| format!("Could not copy the cell's physical layouts: {error}"))?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CopyCell {
                source_library: src_library.to_owned(),
                source_cell: cell.to_owned(),
                target_library: dst_library.to_owned(),
                target_cell: new_name.to_owned(),
            },
        )?;

        copy.name = new_name.to_owned();
        let view_names: Vec<String> = copy.views.keys().cloned().collect();
        let view_count = view_names.len();
        self.model
            .library_manager
            .edit_library(dst_library)
            .ok_or_else(|| format!("Library '{dst_library}' disappeared during the copy"))?
            .add_cell(copy);

        // The drawn content lives in the workspace buffers, keyed
        // "library/cell/view" — a copy without it would be a lie. The
        // validation journal is the one thing that does not travel: it records
        // that *this* document was reviewed and signed off at a revision, and
        // a copy has been reviewed by nobody. It starts its own history.
        for view in view_names {
            let old_key = CellViewRef::new(src_library, cell, view.as_str()).key();
            let new_key = CellViewRef::new(dst_library, new_name, view.as_str()).key();
            if let Some(buffer) = self
                .model
                .workspace
                .schematic_editor(&old_key)
                .map(|source| source.copy_without_validated_revisions())
            {
                self.model
                    .workspace
                    .insert_schematic_editor(new_key, buffer);
            }
        }
        self.model
            .workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        if !copied_source_ids.is_empty() {
            self.model.workspace.content.project_sources = candidate_sources;
            self.model.workspace.content.project_sources_dirty = true;
        }
        if copied_sheet_catalogs > 0 {
            self.model.workspace.content.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        self.model
            .library_manager
            .select_cell(dst_library, new_name);
        Ok(view_count)
    }

    /// Rename a cell and remap everything that pointed at it: view buffers,
    /// open workspace references, and the Library/Cell binding of every
    /// instance in this project's designs. Returns the number of instance
    /// bindings remapped, or the user-facing error.
    pub(crate) fn rename_cell(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let lib = self
            .model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if lib.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        if lib.get_cell(cell).is_none() {
            return Err(format!("Cell '{cell}' not found in library '{library}'"));
        }
        if cell == new_name {
            return Err(format!("Cell '{cell}' already has that name"));
        }
        let requested_identity = crate::state::canonical_cell_view_owner_key(library, new_name, "");
        if lib.cells.iter().any(|(candidate_key, candidate)| {
            candidate_key != cell
                && crate::state::canonical_cell_view_owner_key(library, &candidate.name, "")
                    == requested_identity
        }) {
            return Err(format!(
                "Cell '{new_name}' conflicts with an existing cell in library '{library}' whose name differs only in case or normalization"
            ));
        }

        let mut candidate_sources = self.model.workspace.content.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_cell_view_bundles(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's project source ownership: {error}")
            })?;
        let mut candidate_configurations = self.model.workspace.content.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_cell_roots(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's configuration-set roots: {error}")
            })?;
        let mut candidate_design_management =
            self.model.workspace.content.design_management.clone();
        let design_management_receipt = candidate_design_management
            .rename_cell_sheet_catalogs(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's Design Management ownership: {error}")
            })?;
        let design_management_changed = design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_variant_objects > 0
            || design_management_receipt.remapped_annotation_objects > 0;
        let candidate_layouts = self
            .model
            .workspace
            .content
            .prepare_rename_physical_layout_cell_documents(library, cell, new_name)
            .map_err(|error| format!("Could not rename the cell's physical layouts: {error}"))?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::RenameCell {
                library: library.to_owned(),
                from_cell: cell.to_owned(),
                to_cell: new_name.to_owned(),
            },
        )?;

        let mut library_edit = self
            .model
            .library_manager
            .edit_library(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the rename"))?;
        if !library_edit.rename_cell(cell, new_name) {
            return Err(format!("Cell '{cell}' disappeared during the rename"));
        }

        // Buffers move with the cell.
        let old_prefix = format!("{library}/{cell}/");
        let moved_keys: Vec<String> = self
            .model
            .workspace
            .content
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&old_prefix))
            .cloned()
            .collect();
        for key in moved_keys {
            if let Some(buffer) = self.model.workspace.remove_schematic_editor(&key) {
                let tail = &key[old_prefix.len()..];
                self.model
                    .workspace
                    .insert_schematic_editor(format!("{library}/{new_name}/{tail}"), buffer);
            }
        }
        self.model
            .workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        // Open references follow.
        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library && reference.cell == cell {
                reference.cell = new_name.to_owned();
            }
        };
        remap_ref(&mut self.model.workspace.content.active_view);
        for open in &mut self.model.workspace.content.open_views {
            remap_ref(&mut open.reference);
        }
        self.model
            .workspace
            .content
            .remap_occurrence_masters(&remap_ref);

        // Instance bindings follow — in every buffer and the live sheet.
        let mut remapped = 0usize;
        let mut remap_schematic = |schematic: &mut crate::state::SchematicState| {
            remapped += schematic.rename_cell_bindings(library, cell, new_name);
        };
        self.model
            .workspace
            .for_each_schematic_editor_mut(|_, buffer| remap_schematic(buffer));
        remap_schematic(&mut self.model.schematic);

        if !renamed_source_ids.is_empty() {
            self.model.workspace.content.project_sources = candidate_sources;
            self.model.workspace.content.project_sources_dirty = true;
            let transient_uses_renamed = self
                .code
                .code_workspace
                .veriloga
                .receipt
                .as_ref()
                .is_some_and(|receipt| {
                    renamed_source_ids.contains(&receipt.compilation.token().bundle_id)
                })
                || self
                    .code
                    .code_workspace
                    .veriloga
                    .pending
                    .as_ref()
                    .is_some_and(|pending| renamed_source_ids.contains(&pending.token.bundle_id));
            if transient_uses_renamed {
                self.code.code_workspace.veriloga = Default::default();
            }
        }
        if renamed_configuration_roots > 0 {
            self.model.workspace.content.configuration_sets = candidate_configurations;
            self.model.workspace.content.project_metadata_dirty = true;
        }
        if design_management_changed {
            self.model.workspace.content.design_management = candidate_design_management;
        }
        // The project root names its top cell by name. Leaving the old name
        // behind makes `ProjectFile::validate` reject every later save.
        if self.model.workspace.content.project.root_library == library
            && self.model.workspace.content.project.top_cell == cell
        {
            self.model.workspace.content.project.top_cell = new_name.to_owned();
        }
        self.publish_project_library_mutation(project_mutation);

        self.model.library_manager.select_cell(library, new_name);
        Ok(remapped)
    }

    /// Rename one view of one cell and remap the buffer, open references, and
    /// every instance binding that selected exactly that view. Returns the
    /// number of instance bindings remapped.
    pub(crate) fn rename_view(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let owner = self
            .model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if owner.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        let owning_cell = owner
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{cell}' not found in library '{library}'"))?;
        if owning_cell.get_view(view).is_none() {
            return Err(format!("View '{library}/{cell}/{view}' not found"));
        }
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(library, cell, new_name);
        if crate::state::canonical_cell_view_owner_key(library, cell, view) == requested_identity {
            return Err(format!(
                "View '{view}' already has that identity; a name differing only in case or normalization is the same view"
            ));
        }
        if owning_cell.views.iter().any(|(candidate_key, candidate)| {
            candidate_key != view
                && crate::state::canonical_cell_view_owner_key(library, cell, &candidate.name)
                    == requested_identity
        }) {
            return Err(format!(
                "View '{new_name}' conflicts with an existing view in cell '{library}/{cell}' whose name differs only in case or normalization"
            ));
        }
        let candidate_layouts = self
            .model
            .workspace
            .content
            .prepare_rename_physical_layout_view_documents(library, cell, view, new_name)
            .map_err(|error| format!("Could not rename the view's physical layouts: {error}"))?;

        let mut candidate_configurations = self.model.workspace.content.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_view_roots(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's configuration-set roots: {error}")
            })?;
        let mut candidate_sources = self.model.workspace.content.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_view_bundles(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's project source ownership: {error}")
            })?;
        let mut candidate_design_management =
            self.model.workspace.content.design_management.clone();
        let design_management_receipt = candidate_design_management
            .rename_view_sheet_catalogs(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's Design Management ownership: {error}")
            })?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::RenameView {
                library: library.to_owned(),
                cell: cell.to_owned(),
                from_view: view.to_owned(),
                to_view: new_name.to_owned(),
            },
        )?;

        let renamed = self
            .model
            .library_manager
            .edit_library(library)
            .and_then(|mut library| library.rename_view(cell, view, new_name))
            .ok_or_else(|| format!("Cell '{library}/{cell}' disappeared during the rename"))?;
        if !renamed {
            return Err(format!("View '{view}' disappeared during the rename"));
        }

        let old_reference = CellViewRef::new(library, cell, view);
        let new_reference = CellViewRef::new(library, cell, new_name);
        if let Some(buffer) = self
            .model
            .workspace
            .remove_schematic_editor(&old_reference.key())
        {
            self.model
                .workspace
                .insert_schematic_editor(new_reference.key(), buffer);
        }
        self.model
            .workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        let remap_ref = |reference: &mut CellViewRef| {
            if *reference == old_reference {
                *reference = new_reference.clone();
            }
        };
        remap_ref(&mut self.model.workspace.content.active_view);
        for open in &mut self.model.workspace.content.open_views {
            remap_ref(&mut open.reference);
        }
        self.model
            .workspace
            .content
            .remap_occurrence_masters(&remap_ref);

        let remapped = self.remap_instance_bindings(|schematic| {
            schematic.rename_view_bindings(library, cell, view, new_name)
        });

        self.commit_renamed_project_sources(candidate_sources, &renamed_source_ids);
        if renamed_configuration_roots > 0 {
            self.model.workspace.content.configuration_sets = candidate_configurations;
            self.model.workspace.content.project_metadata_dirty = true;
        }
        if design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_annotation_objects > 0
        {
            self.model.workspace.content.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        self.model
            .library_manager
            .select_view(library, cell, new_name);
        Ok(remapped)
    }

    /// Rewrite the Library/Cell/View binding of every instance in every loaded
    /// buffer and in the live sheet. `rebind` applies a specific identity rename
    /// and reports the number of affected placements.
    ///
    /// The live sheet and the buffer under the active key are two copies of one
    /// document, so both move but only one is counted.
    fn remap_instance_bindings(&mut self, rebind: impl Fn(&mut SchematicState) -> usize) -> usize {
        let active_key = self.model.workspace.content.active_key();
        let mut remapped = 0usize;
        self.model
            .workspace
            .for_each_schematic_editor_mut(|key, buffer| {
                let count = rebind(buffer);
                if !key.eq_ignore_ascii_case(&active_key) {
                    remapped += count;
                }
            });
        remapped += rebind(&mut self.model.schematic);
        remapped
    }

    /// Loaded instances of one library's masters placed from outside it,
    /// counted once per instance.
    ///
    /// A library's own cells instancing each other is ordinary hierarchy and
    /// goes with the library; only a placement that would survive it is a
    /// reason to refuse the deletion, and it is the number the review states.
    pub(crate) fn external_instance_references_to_library(&self, library: &str) -> usize {
        let active_key = self.model.workspace.content.active_key();
        let owned_prefix = format!("{library}/");
        let count = |document: &rspice_design::schematic::document::SchematicDocument| {
            document
                .components
                .iter()
                .filter(|component| {
                    component
                        .library_cell
                        .as_ref()
                        .is_some_and(|binding| binding.library == library)
                })
                .count()
        };
        let live = if self.model.workspace.content.active_view.library == library {
            0
        } else {
            count(self.model.schematic.document())
        };
        live + self
            .model
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| {
                !key.eq_ignore_ascii_case(&active_key) && !key.starts_with(&owned_prefix)
            })
            .map(|(_, schematic)| count(schematic.document()))
            .sum::<usize>()
    }

    /// Adopt a prepared source registry and drop transient Verilog-A evidence
    /// that named a bundle whose owner just moved.
    fn commit_renamed_project_sources(
        &mut self,
        candidate: crate::state::ProjectSourceRegistry,
        renamed: &[crate::state::ProjectSourceId],
    ) {
        if renamed.is_empty() {
            return;
        }
        self.model.workspace.content.project_sources = candidate;
        self.model.workspace.content.project_sources_dirty = true;
        let transient_uses_renamed = self
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .is_some_and(|receipt| renamed.contains(&receipt.compilation.token().bundle_id))
            || self
                .code
                .code_workspace
                .veriloga
                .pending
                .as_ref()
                .is_some_and(|pending| renamed.contains(&pending.token.bundle_id));
        if transient_uses_renamed {
            self.code.code_workspace.veriloga = Default::default();
        }
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
        self.state.clear_transient_specialized_viewer_data();
    }
}

#[cfg(test)]
mod tests;
