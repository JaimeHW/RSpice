//! Keyboard shortcut dispatch, and the edit actions it resolves to.
//!
//! [`RSpiceApp::handle_shortcuts`] is the one place a key press becomes a
//! command: it resolves the chord against the active profile, checks the
//! command is available in this workspace, and dispatches. The submodules
//! hold the actions themselves — file operations, workspace navigation,
//! property edits, and image export.

pub(in crate::workbench) mod export_image;
pub(in crate::workbench) mod file;
mod hierarchy_net_highlight;
pub(in crate::workbench) mod property_edit;
pub(in crate::workbench) mod reveal;
pub(in crate::workbench) mod sheets;
pub(in crate::workbench) mod stimulus;
pub(in crate::workbench) mod stimulus_placement;
mod symbol;
pub(in crate::workbench) mod veriloga_parameters;
mod veriloga_placement;
mod veriloga_symbol;
pub(in crate::workbench) mod workspace;

use egui::{Context, Popup};

use crate::workbench::commands::vocabulary::Command as ShortcutCommand;

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::shortcuts::{
    ShortcutEnvironment, ShortcutInputSnapshot, engineering_canvas_has_focus,
    escape_owner_was_let_go, runtime_command_platform,
};

fn shortcut_dispatch_blocked(state: &AppState, ctx: &Context) -> bool {
    state.application_modal_open() || state.workbench.drawer.is_some() || Popup::is_any_open(ctx)
}

impl RSpiceApp {
    /// Handle keyboard shortcuts
    pub(in crate::workbench) fn handle_shortcuts(&mut self, ctx: &Context) {
        if shortcut_dispatch_blocked(&self.state, ctx) {
            self.state.shortcut_resolver.reset();
            return;
        }
        let active_view = self.state.workspace.content.active_view_type();
        let canvas_focus =
            engineering_canvas_has_focus(ctx, self.state.workbench.workspace, active_view);
        if crate::schematic::view::handle_pre_render_placement_transform(
            ctx,
            &mut self.state,
            canvas_focus,
        ) {
            self.state.shortcut_resolver.reset();
            return;
        }
        // An Escape that let go of a widget which takes Escape itself is that
        // widget's key, as it would be had egui not dropped the focus first.
        let non_canvas_focus = (ctx.memory(|memory| memory.focused().is_some())
            || escape_owner_was_let_go(ctx))
            && !canvas_focus;
        let snapshot =
            ctx.input(|input| ShortcutInputSnapshot::from_input_state(input, non_canvas_focus));
        let injected_now =
            ctx.input(|input| std::time::Duration::from_secs_f64(input.time.max(0.0)));
        let profile = self.state.ui.preferences.shortcuts().clone();
        let platform = runtime_command_platform(ctx);
        // The editor the page drew last frame, as the menus also see it.
        let source_editor_focus = self
            .state
            .ui
            .active_source_editor
            .is_some_and(|editor| ctx.memory(|memory| memory.has_focus(editor.id)));
        let environment = ShortcutEnvironment {
            workspace: self.state.workbench.workspace,
            active_view,
            canvas_focus,
            source_editor_focus,
        };
        let mut resolver = std::mem::take(&mut self.state.shortcut_resolver);
        let resolution = resolver.resolve(
            &snapshot,
            &profile,
            platform,
            environment,
            injected_now,
            |command| command.availability(self).is_available(),
        );
        self.state.shortcut_resolver = resolver;

        if let Some(delay) = resolution.repaint_after {
            ctx.request_repaint_after(delay);
        }
        let palette_input = if resolution.command == Some(ShortcutCommand::CommandPalette) {
            ctx.input(|input| {
                crate::ui::input::InputTransition::after_keys(
                    input,
                    &resolution.consume[..resolution.deferred_start],
                )
            })
        } else {
            None
        };
        let consumed = ctx.input_mut(|input| {
            ShortcutInputSnapshot::consume_keys(
                input,
                &resolution.consume,
                resolution.deferred_start,
            )
        });
        if consumed && let Some(command) = resolution.command {
            self.execute_shortcut_command(command);
            if self.state.dialogs.command_palette.open
                && let Some(input) = palette_input
            {
                super::command_palette::route_opening_input(ctx, input);
                self.state.shortcut_resolver.reset();
            }
        }
    }

    /// Carry out the command a key resolved to.
    ///
    /// A key has no behaviour of its own once it has been resolved: the
    /// command runs through [`ShortcutCommand::execute`], the boundary the
    /// menus, toolbar, context menu and palette call, so a key and a menu row
    /// cannot drift apart. What is particular to keys — the chord, the
    /// focus owner, the opening keystroke the palette inherits — is settled in
    /// [`Self::handle_shortcuts`] before this runs.
    pub(in crate::workbench) fn execute_shortcut_command(&mut self, command: ShortcutCommand) {
        command.execute(self);
    }

    fn try_project_design_undo(&mut self) -> bool {
        match self.state.undo_project_design() {
            Ok(Some(_)) => self.invalidate_simulation_preflight(),
            Ok(None) if self.state.project_undo_sequence().is_some() => {
                self.state.prompt(
                    "Undo is unavailable because the latest project edit's document or references changed.",
                );
            }
            Ok(None) => return false,
            Err(error) => self.state.prompt(error),
        }
        true
    }

    fn try_project_design_redo(&mut self) -> bool {
        match self.state.redo_project_design() {
            Ok(Some(_)) => self.invalidate_simulation_preflight(),
            Ok(None) if self.state.project_redo_sequence().is_some() => {
                self.state.prompt(
                    "Redo is unavailable because the latest project edit's document or references changed.",
                );
            }
            Ok(None) => return false,
            Err(error) => self.state.prompt(error),
        }
        true
    }

    /// Where the active document's next undo step sits in the global order.
    ///
    /// Symbol documents keep their history as plain snapshot stacks that no
    /// commit boundary stamps, so they carry no position in that order and
    /// answer `None` rather than a position that would sort as the oldest
    /// record in the session.
    fn active_document_undo_sequence(&self) -> Option<crate::state::UndoSequence> {
        if self.state.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
            return None;
        }
        self.state.schematic.history().undo_sequence()
    }

    /// Where the active document's next redo step sits in the global order.
    fn active_document_redo_sequence(&self) -> Option<crate::state::UndoSequence> {
        if self.state.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
            return None;
        }
        self.state.schematic.history().redo_sequence()
    }

    fn try_active_document_undo(&mut self) -> bool {
        if self.state.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
            return match self.state.undo_active_symbol_document() {
                Ok(stepped) => stepped,
                Err(error) => {
                    self.state.prompt(error);
                    true
                }
            };
        }
        if !self.state.schematic.can_undo() {
            return false;
        }
        if self.state.schematic.undo() {
            self.state.ui.schematic_snap = self.state.schematic.session.editor.snap_engine.clone();
            self.state.sync_active_schematic_to_workspace();
            return true;
        }
        false
    }

    fn try_active_document_redo(&mut self) -> bool {
        if self.state.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
            return match self.state.redo_active_symbol_document() {
                Ok(stepped) => stepped,
                Err(error) => {
                    self.state.prompt(error);
                    true
                }
            };
        }
        if !self.state.can_redo_active_schematic() {
            return false;
        }
        if self.state.schematic.redo() {
            self.state.ui.schematic_snap = self.state.schematic.session.editor.snap_engine.clone();
            self.state.sync_active_schematic_to_workspace();
            return true;
        }
        false
    }

    /// Whether the schematic is in the middle of an edit (a drag still held,
    /// a move still floating). A history step taken then would be folded
    /// into that edit's undo record when it lands, so it is refused, with a
    /// line saying why.
    fn history_waits_for_the_edit_in_progress(&mut self, step: &str) -> bool {
        if !self.state.schematic_edit_in_progress() {
            return false;
        }
        self.state.prompt(format!(
            "Finish or cancel the edit in progress before {step}."
        ));
        true
    }

    pub(crate) fn action_edit_undo(&mut self) {
        if self.history_waits_for_the_edit_in_progress("undoing") {
            return;
        }
        let waivers = self.state.stored_waivers();
        self.step_undo();
        self.after_history_step(&waivers);
    }

    pub(crate) fn action_edit_redo(&mut self) {
        if self.history_waits_for_the_edit_in_progress("redoing") {
            return;
        }
        let waivers = self.state.stored_waivers();
        self.step_redo();
        self.after_history_step(&waivers);
    }

    /// What an undo or redo that recorded or took away a waiver changes
    /// beyond the document: the published findings are stated again, and a
    /// run prepared or checked under the old waivers is dropped, as a waiver
    /// recorded from the Problems page drops it.
    fn after_history_step(
        &mut self,
        waivers: &crate::workbench::app_state::waivers::StoredWaivers,
    ) {
        if self.state.restate_waived_findings(waivers) {
            self.invalidate_simulation_preflight();
        }
    }

    fn step_undo(&mut self) {
        // A stimulus draft is the workspace's own edit history, like the
        // symbol document's. It is stepped first while that workspace is open
        // so Ctrl+Z means "take back what I just typed" rather than reaching
        // past the instrument into the design.
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Stimulus
            && stimulus::undo_draft(&mut self.state)
        {
            return;
        }
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist
            && self.state.ui.code_workspace.page
                == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
        {
            match crate::workbench::documents::netlist_document::undo_netlist_edit(&mut self.state)
            {
                Ok(Some(_)) => return,
                Err(error) => {
                    self.state.prompt(error);
                    return;
                }
                Ok(None) => {}
            }
        }
        // The history holding the newer step takes it. With nothing to undo
        // the command is silent: the Edit menu already shows it disabled.
        let project_first = project_step_is_newer(
            self.state.project_undo_sequence(),
            self.active_document_undo_sequence(),
        );
        if project_first && self.try_project_design_undo() {
            return;
        }
        if !self.try_active_document_undo() && !project_first {
            self.try_project_design_undo();
        }
    }

    fn step_redo(&mut self) {
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Stimulus
            && stimulus::redo_draft(&mut self.state)
        {
            return;
        }
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist
            && self.state.ui.code_workspace.page
                == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
        {
            match crate::workbench::documents::netlist_document::redo_netlist_edit(&mut self.state)
            {
                Ok(Some(_)) => return,
                Err(error) => {
                    self.state.prompt(error);
                    return;
                }
                Ok(None) => {}
            }
        }
        // A step committed in either history since an undo replaces what
        // the other can redo, so only the steps no newer edit replaced are
        // left to arbitrate.
        self.state.retire_replaced_redo_steps();
        let project_first = project_step_is_newer(
            self.state.project_redo_sequence(),
            self.active_document_redo_sequence(),
        );
        if project_first && self.try_project_design_redo() {
            return;
        }
        if !self.try_active_document_redo() && !project_first {
            self.try_project_design_redo();
        }
    }

    pub(in crate::workbench) fn action_edit_paste(&mut self) {
        let anchor = self.state.schematic_paste_anchor();
        let message = match self.state.paste_schematic_clipboard_at(anchor) {
            Ok(true) => return,
            Ok(false) => "Nothing to paste.".to_owned(),
            Err(reason) => reason,
        };
        self.state.prompt(message);
    }
}

/// Which stack Undo and Redo act on.
///
/// The project history and the active document's history are one timeline as
/// far as the operator is concerned, so the step that moved most recently
/// wins — that is what the global sequence records. A document history with
/// no position in that order cannot be compared, so a stamped project step
/// takes precedence over it and nothing takes precedence over a document
/// history when the project has no step at all.
fn project_step_is_newer(
    project: Option<crate::state::UndoSequence>,
    document: Option<crate::state::UndoSequence>,
) -> bool {
    match (project, document) {
        (Some(project), Some(document)) => project > document,
        (Some(_), None) => true,
        (None, _) => false,
    }
}

#[cfg(test)]
mod shortcut_ownership_tests {
    use super::*;

    fn assert_grid_pitch_contract(app: &RSpiceApp, pitch: crate::state::SchematicGridPitch) {
        let expected = pitch.canvas_grid_size();
        assert_eq!(
            app.state.schematic.document().document_policy.grid_pitch,
            pitch
        );
        assert_eq!(app.state.schematic.document().grid_size, expected);
        assert_eq!(
            app.state.schematic.session.editor.snap_engine.grid_size,
            expected
        );
        assert_eq!(app.state.ui.schematic_snap.grid_size, expected);
    }

    #[test]
    fn document_undo_and_redo_reconcile_all_grid_pitch_authorities() {
        use crate::state::SchematicGridPitch;

        let mut app = RSpiceApp::test_instance();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil50);

        assert!(
            app.state
                .schematic
                .with_undo("change schematic grid pitch", |schematic| {
                    schematic.document_mut_for_test().document_policy.grid_pitch =
                        SchematicGridPitch::Mil25;
                    schematic.document_mut_for_test().grid_size =
                        SchematicGridPitch::Mil25.canvas_grid_size();
                    schematic.session.editor.snap_engine.grid_size =
                        SchematicGridPitch::Mil25.canvas_grid_size();
                })
        );
        app.state.ui.schematic_snap = app.state.schematic.session.editor.snap_engine.clone();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil25);

        app.action_edit_undo();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil50);

        app.action_edit_redo();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil25);
    }

    /// One reviewed design-management transaction over the active document,
    /// as ascending out of a child publishes.
    fn publish_sheet_assignment(
        state: &mut crate::workbench::app_state::AppState,
    ) -> crate::state::CellViewRef {
        use crate::workbench::app_state::DesignManagementHistoryEntry;

        state.project_lifecycle.authority.open_session();
        let owner = state.workspace.content.active_schematic_reference();
        let before = (*state.workspace.content.design_management).clone();
        let drawn = state
            .schematic
            .document()
            .components
            .iter()
            .map(|component| component.id)
            .collect::<Vec<_>>();
        let mut candidate = before.clone();
        candidate
            .bootstrap_for_cell_view(&owner.key(), "Sheet 1", drawn)
            .expect("bootstrap reviewed sheet catalog");
        let committed_revision = state
            .workspace
            .content
            .replace_design_management(candidate)
            .expect("publish reviewed catalog");
        let after = (*state.workspace.content.design_management).clone();
        state.record_design_management_transaction(DesignManagementHistoryEntry {
            description: "assign objects to the active sheet".to_owned(),
            owner: owner.clone(),
            before,
            after,
            before_schematics: std::collections::BTreeMap::new(),
            after_schematics: std::collections::BTreeMap::new(),
            references: Default::default(),
            committed_revision,
        });
        owner
    }

    /// Closes the case where Undo after leaving a child reverted the edit made
    /// inside it. The ascent's own transaction is the newer step, so Undo owes
    /// the operator that one; the edit underneath it survives and Redo puts
    /// the ascent back.
    #[test]
    fn undo_after_ascend_reverts_the_extraction_not_the_prior_edit() {
        use crate::state::{ComponentType, Point};

        let mut app = RSpiceApp::test_instance();
        assert!(
            app.state
                .schematic
                .with_undo("place resistor", |schematic| {
                    schematic.add_component(ComponentType::Resistor, Point::origin());
                })
        );
        let drawn = app.state.schematic.document().components.len();
        let owner = publish_sheet_assignment(&mut app.state);

        app.action_edit_undo();
        assert_eq!(
            app.state.schematic.document().components.len(),
            drawn,
            "the edit made before the ascent is not what Undo owes the operator"
        );
        assert!(
            app.state
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_none(),
            "the ascent's own transaction is the step that reverts"
        );

        app.action_edit_redo();
        assert!(
            app.state
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_some()
        );
        assert_eq!(app.state.schematic.document().components.len(), drawn);
    }

    #[test]
    fn interleaved_project_and_document_edits_undo_in_global_sequence_order() {
        use crate::state::{ComponentType, Point};

        let mut app = RSpiceApp::test_instance();
        assert!(
            app.state
                .schematic
                .with_undo("place resistor", |schematic| {
                    schematic.add_component(ComponentType::Resistor, Point::origin());
                })
        );
        let owner = publish_sheet_assignment(&mut app.state);
        assert!(
            app.state
                .schematic
                .with_undo("place capacitor", |schematic| {
                    schematic.add_component(ComponentType::Capacitor, Point::new(40, 0));
                })
        );
        assert_eq!(app.state.schematic.document().components.len(), 2);

        app.action_edit_undo();
        assert_eq!(
            app.state.schematic.document().components.len(),
            1,
            "the newest step is the document edit made after the transaction"
        );
        assert!(
            app.state
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_some()
        );

        app.action_edit_undo();
        assert!(
            app.state
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_none(),
            "the transaction is next, ahead of the older document edit"
        );
        assert_eq!(app.state.schematic.document().components.len(), 1);

        app.action_edit_undo();
        assert!(app.state.schematic.document().components.is_empty());
    }

    fn app_on_a_symbol_view() -> RSpiceApp {
        use crate::state::{Cell, CellViewRef, Library, SymbolDocument, View, ViewType};

        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("dispatch");
        let mut cell = Cell::new("amp");
        let mut view = View::new("symbol", ViewType::Symbol);
        SymbolDocument::default().store_in_view(&mut view).unwrap();
        cell.add_view(view);
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state
            .open_workspace_view(CellViewRef::new("dispatch", "amp", "symbol"));
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        app
    }

    /// A key runs the command the menus, toolbar, context menu and palette
    /// run. The key route used to carry its own copy of some forty commands,
    /// and the copies drifted: Find, Escape and Zoom 1:1 each did something
    /// different depending on how they were asked for.
    #[test]
    fn every_shortcut_command_reaches_the_one_dispatcher() {
        use crate::workbench::commands::dispatch_probe;
        use crate::workbench::commands::vocabulary::COMMAND_REGISTRY;

        let mut schematic = RSpiceApp::test_instance();
        schematic
            .state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        let mut symbol = app_on_a_symbol_view();
        for app in [&mut schematic, &mut symbol] {
            let view = app.state.workspace.content.active_view_type();
            let mut bypassed = Vec::new();
            for &command in COMMAND_REGISTRY {
                let reached = dispatch_probe::reached(|| app.execute_shortcut_command(command));
                if reached != [command] {
                    bypassed.push(command);
                }
            }
            assert!(
                bypassed.is_empty(),
                "on a {view:?} view these keys bypass Command::execute: {bypassed:?}"
            );
        }
    }

    #[test]
    fn open_popup_blocks_application_shortcut_dispatch() {
        let ctx = Context::default();
        let state = AppState::default();
        assert!(!shortcut_dispatch_blocked(&state, &ctx));

        Popup::open_id(&ctx, egui::Id::new("shortcut-ownership-test"));

        assert!(shortcut_dispatch_blocked(&state, &ctx));
    }

    #[test]
    fn open_responsive_drawer_blocks_background_shortcut_dispatch() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);

        assert!(shortcut_dispatch_blocked(&state, &ctx));
    }

    #[test]
    fn linked_cursor_command_toggles_real_results_state() {
        let mut app = RSpiceApp::test_instance();
        assert!(!app.state.ui.results.session.linked_cursors);
        app.execute_shortcut_command(ShortcutCommand::ToggleLinkedCursors);
        assert!(app.state.ui.results.session.linked_cursors);
        app.execute_shortcut_command(ShortcutCommand::ToggleLinkedCursors);
        assert!(!app.state.ui.results.session.linked_cursors);
    }

    #[test]
    fn copy_shortcut_in_results_copies_cursor_readout_not_schematic_selection() {
        let mut app = RSpiceApp::test_instance();
        let analysis =
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![0.0, 2.0],
                    "#ffbd2e",
                )]);
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.retained.runs = vec![run].into();
        assert!(app.state.simulation.select_run(0));
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        app.state.ui.results.session.cursors.a = Some(0.5);
        app.state.ui.results.session.cursor_strip = Some(0);

        app.execute_shortcut_command(ShortcutCommand::Copy);

        let copied = app
            .state
            .ui
            .clipboard_text_request
            .as_deref()
            .expect("cursor readout copied");
        assert!(copied.starts_with('A'));
        assert!(copied.contains("V(out)"));
    }

    #[test]
    fn object_properties_shortcut_dispatches_a_bus_and_escape_leaves_it_alone() {
        use crate::state::{Bus, BusDeclaration, Point};

        let mut app = RSpiceApp::test_instance();
        app.state.schematic.document_mut_for_test().buses.push(
            Bus::segment(
                87,
                Point::new(0, 0),
                Point::new(20, 0),
                Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
            )
            .unwrap(),
        );
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(87);

        app.execute_shortcut_command(ShortcutCommand::ObjectProperties);
        assert!(matches!(
            app.state.dialogs.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::Bus(_))
        ));
        let Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::Bus(draft)) =
            app.state.dialogs.object_properties.draft.as_mut()
        else {
            unreachable!()
        };
        draft.declaration = "ADDR[7:0]".to_owned();
        app.state.dialogs.object_properties.mark_edited();

        // The draft owns nothing until Apply, so one Escape leaves and the
        // bus is as it was.
        app.execute_shortcut_command(ShortcutCommand::Cancel);
        assert!(!app.state.dialogs.object_properties.open);
        assert_eq!(
            app.state.schematic.document().buses[0].declaration,
            Some(BusDeclaration::parse("DATA[7:0]").unwrap())
        );
    }

    #[test]
    fn next_marker_resolves_again_after_first_jump_activates_design() {
        use crate::workbench::commands::vocabulary::CommandPlatform;
        use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

        let mut app = RSpiceApp::test_instance();
        let mut result = DrcResult::new();
        for (id, x) in [(1, 0.0), (2, 10.0)] {
            result.add_violation(DrcViolation::new(
                id,
                DrcViolationType::DanglingWire,
                format!("Finding {id}"),
                DrcLocation::Point { x, y: 0.0 },
            ));
        }
        result.completed = true;
        // Availability resolves against the design-check receipt, so publish
        // through the canonical owner and let it refresh the legacy canvas
        // projection the violation cursor still reads.
        app.state
            .publish_active_design_check_result(result)
            .expect("publish the active design-check receipt");
        app.state.refresh_active_design_check_projection();
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Verify);

        // The tour's position is the shared current finding, named by the
        // canonical identity its published record carries, so the Problems
        // grid's current row is the finding the canvas just jumped to.
        let subject = app.state.workspace.content.active_schematic_reference();
        let identities = app.state.design_rule_finding_identities(&subject);
        assert_eq!(identities.len(), 2, "both findings were published");

        app.execute_shortcut_command(ShortcutCommand::NextViolation);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
        assert_eq!(app.state.diagnostics.current_finding(), Some(identities[0]));

        let snapshot = ShortcutInputSnapshot::from_events_for_test(
            &[egui::Event::Key {
                key: egui::Key::F8,
                physical_key: Some(egui::Key::F8),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            false,
        );
        let profile = app.state.ui.preferences.shortcuts().clone();
        let environment = ShortcutEnvironment {
            workspace: app.state.workbench.workspace,
            active_view: app.state.workspace.content.active_view_type(),
            canvas_focus: false,
            source_editor_focus: false,
        };
        let mut resolver = std::mem::take(&mut app.state.shortcut_resolver);
        let resolution = resolver.resolve(
            &snapshot,
            &profile,
            CommandPlatform::Desktop,
            environment,
            std::time::Duration::from_millis(1),
            |command| command.availability(&app).is_available(),
        );
        app.state.shortcut_resolver = resolver;
        assert_eq!(resolution.command, Some(ShortcutCommand::NextViolation));

        app.execute_shortcut_command(resolution.command.unwrap());
        assert_eq!(app.state.diagnostics.current_finding(), Some(identities[1]));
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
    }
}

#[cfg(test)]
mod symbol_action_tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, Library, Point, PortDirection, SymbolDocument, SymbolPin, View, ViewType,
    };

    #[test]
    fn registered_symbol_tools_are_available_and_execute_complete_state_transitions() {
        let mut app = RSpiceApp::test_instance();
        let reference = CellViewRef::new("shortcut_tools", "amp", "symbol");
        let mut library = Library::new("shortcut_tools");
        let mut cell = Cell::new("amp");
        let mut view = View::new("symbol", ViewType::Symbol);
        SymbolDocument {
            pins: vec![
                SymbolPin::new("PLACED", PortDirection::In, Some(Point::new(-10, 0))),
                SymbolPin::new("NEXT", PortDirection::Out, None),
            ],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut view)
        .unwrap();
        cell.add_view(view);
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state.open_workspace_view(reference);
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);

        for command in [
            ShortcutCommand::SelectTool,
            ShortcutCommand::SymbolPinTool,
            ShortcutCommand::SymbolPolylineTool,
            ShortcutCommand::SymbolRectangleTool,
            ShortcutCommand::SymbolCircleTool,
            ShortcutCommand::SymbolArcTool,
            ShortcutCommand::SymbolPolygonTool,
            ShortcutCommand::SymbolTextTool,
            ShortcutCommand::ZoomFit,
        ] {
            assert!(command.availability(&app).is_available(), "{command:?}");
        }

        app.execute_shortcut_command(ShortcutCommand::SymbolPinTool);
        assert_eq!(
            app.state.ui.symbol.editor.tool,
            crate::workbench::SymbolTool::PlacePin
        );
        assert_eq!(
            app.state.ui.symbol.editor.selected_pin.as_deref(),
            Some("NEXT")
        );

        app.state.ui.symbol.editor.pending_polyline = vec![Point::new(0, 0)];
        app.execute_shortcut_command(ShortcutCommand::SymbolPolylineTool);
        assert_eq!(
            app.state.ui.symbol.editor.tool,
            crate::workbench::SymbolTool::Line
        );
        assert!(app.state.ui.symbol.editor.pending_polyline.is_empty());

        for (command, expected) in [
            (
                ShortcutCommand::SymbolRectangleTool,
                crate::workbench::SymbolTool::Rectangle,
            ),
            (
                ShortcutCommand::SymbolCircleTool,
                crate::workbench::SymbolTool::Circle,
            ),
            (
                ShortcutCommand::SymbolArcTool,
                crate::workbench::SymbolTool::Arc,
            ),
            (
                ShortcutCommand::SymbolPolygonTool,
                crate::workbench::SymbolTool::Polygon,
            ),
            (
                ShortcutCommand::SymbolTextTool,
                crate::workbench::SymbolTool::Text,
            ),
            (
                ShortcutCommand::SelectTool,
                crate::workbench::SymbolTool::Select,
            ),
        ] {
            app.state.ui.symbol.editor.shape_start = Some(Point::new(10, 10));
            app.execute_shortcut_command(command);
            assert_eq!(app.state.ui.symbol.editor.tool, expected);
            if matches!(
                command,
                ShortcutCommand::SymbolRectangleTool
                    | ShortcutCommand::SymbolCircleTool
                    | ShortcutCommand::SymbolArcTool
            ) {
                assert!(app.state.ui.symbol.editor.shape_start.is_none());
            }
        }

        app.state.ui.symbol.editor.needs_fit = false;
        app.execute_shortcut_command(ShortcutCommand::ZoomFit);
        assert!(app.state.ui.symbol.editor.needs_fit);
    }
}
