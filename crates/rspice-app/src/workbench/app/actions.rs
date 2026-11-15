//! Keyboard shortcut dispatch, and the edit actions it resolves to.
//!
//! [`RSpiceApp::handle_shortcuts`] is the one place a key press becomes a
//! command: it resolves the chord against the active profile, checks the
//! command is available in this workspace, and dispatches. The submodules
//! hold the actions themselves — file operations, workspace navigation,
//! property edits, and image export.

pub(in crate::workbench) mod export_image;
pub(in crate::workbench) mod file;
pub(in crate::workbench) mod reveal;

use egui::{Context, Popup};

use crate::workbench::commands::vocabulary::{CommandRegistry, ShellCommand};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_workbench::workbench::app_state::session::shortcuts::{
    ShortcutEnvironment, ShortcutInputSnapshot, ShortcutResolverState,
    engineering_canvas_has_focus, escape_owner_was_let_go, runtime_command_platform,
};

fn shortcut_dispatch_blocked(state: &AppState, ctx: &Context) -> bool {
    state.application_modal_open() || state.workbench.drawer.is_some() || Popup::is_any_open(ctx)
}

impl RSpiceApp {
    /// Handle keyboard shortcuts
    pub(crate) fn handle_shortcuts(&mut self, ctx: &Context) {
        if shortcut_dispatch_blocked(&self.state, ctx) {
            self.state.shortcut_resolver.reset();
            self.answer_help_over_dialog(ctx);
            return;
        }
        let active_view = self.state.model.workspace.content.active_view_type();
        let canvas_focus =
            engineering_canvas_has_focus(ctx, self.state.workbench.workspace, active_view);
        if rspice_app_design::schematic::view::handle_pre_render_placement_transform(
            ctx,
            &mut self.state.canvas_parts(),
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
        let profile = self.state.model.preferences.shortcuts().clone();
        let platform = runtime_command_platform(ctx);
        // The editor the page drew last frame, as the menus also see it.
        let source_editor_focus = self
            .state
            .workbench
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
            &CommandRegistry,
            platform,
            environment,
            injected_now,
            |command| self.command_key_available(command),
        );
        self.state.shortcut_resolver = resolver;

        if let Some(delay) = resolution.repaint_after {
            ctx.request_repaint_after(delay);
        }
        let palette_input = if resolution.command == Some(ShellCommand::CommandPalette.key()) {
            ctx.input(|input| {
                rspice_app_workbench::ui::input::InputTransition::after_keys(
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
        // A key has no behaviour of its own once it has been resolved: the
        // command it names runs through the dispatcher the menus, toolbar,
        // context menu and palette reach, so a key and a menu row cannot
        // drift apart. What is particular to keys — the chord, the focus
        // owner, the opening keystroke the palette inherits — is settled
        // above.
        if consumed && let Some(command) = resolution.command {
            self.execute_command_key(command);
            if self.state.workbench.command_palette.open
                && let Some(input) = palette_input
            {
                rspice_app_workbench::workbench::app::command_palette::route_opening_input(
                    ctx, input,
                );
                self.state.shortcut_resolver.reset();
            }
        }
    }

    /// Help's key while a dialog holds the keyboard: it still opens the
    /// documentation, for that dialog. No other command answers there, and
    /// only a one-chord key does, as nothing is held between frames while a
    /// dialog is open.
    fn answer_help_over_dialog(&mut self, ctx: &Context) {
        let pressed = ctx.input(|input| {
            input
                .events
                .iter()
                .any(|event| matches!(event, egui::Event::Key { pressed: true, .. }))
        });
        if !pressed
            || !self.state.application_modal_open()
            || self.state.workbench.drawer.is_some()
            || Popup::is_any_open(ctx)
        {
            return;
        }
        let help = ShellCommand::HelpCenter.key();
        let snapshot = ctx.input(|input| ShortcutInputSnapshot::from_input_state(input, true));
        let now = ctx.input(|input| std::time::Duration::from_secs_f64(input.time.max(0.0)));
        let environment = ShortcutEnvironment {
            workspace: self.state.workbench.workspace,
            active_view: self.state.model.workspace.content.active_view_type(),
            canvas_focus: false,
            source_editor_focus: false,
        };
        let profile = self.state.model.preferences.shortcuts().clone();
        let resolution = ShortcutResolverState::default().resolve(
            &snapshot,
            &profile,
            &CommandRegistry,
            runtime_command_platform(ctx),
            environment,
            now,
            |command| command == help,
        );
        if resolution.command == Some(help)
            && ctx.input_mut(|input| {
                ShortcutInputSnapshot::consume_keys(
                    input,
                    &resolution.consume,
                    resolution.deferred_start,
                )
            })
        {
            self.execute_command_key(help);
        }
    }
}

#[cfg(test)]
mod shortcut_ownership_tests {
    use super::*;
    use rspice_app_workbench::workbench::framework::command_registry::ids;

    fn assert_grid_pitch_contract(
        app: &RSpiceApp,
        pitch: rspice_app_state::state::SchematicGridPitch,
    ) {
        let expected = pitch.canvas_grid_size();
        assert_eq!(
            app.state
                .model
                .schematic
                .document()
                .document_policy
                .grid_pitch,
            pitch
        );
        assert_eq!(app.state.model.schematic.document().grid_size, expected);
        assert_eq!(
            app.state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .grid_size,
            expected
        );
        assert_eq!(app.state.design.schematic_snap.grid_size, expected);
    }

    #[test]
    fn document_undo_and_redo_reconcile_all_grid_pitch_authorities() {
        use rspice_app_state::state::SchematicGridPitch;

        let mut app = RSpiceApp::test_instance();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil50);

        assert!(
            app.state
                .model
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
        app.state.design.schematic_snap =
            app.state.model.schematic.session.editor.snap_engine.clone();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil25);

        app.execute_command_key(ids::UNDO);
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil50);

        app.execute_command_key(ids::REDO);
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil25);
    }

    /// One reviewed design-management transaction over the active document,
    /// as ascending out of a child publishes.
    fn publish_sheet_assignment(
        state: &mut crate::workbench::app_state::AppState,
    ) -> rspice_app_state::state::CellViewRef {
        use rspice_app_state::workbench::app_state::design_history::DesignManagementHistoryEntry;

        state.open_project_session();
        let owner = state.model.workspace.content.active_schematic_reference();
        let before = (*state.model.workspace.content.design_management).clone();
        let drawn = state
            .model
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
            .model
            .workspace
            .content
            .replace_design_management(candidate)
            .expect("publish reviewed catalog");
        let after = (*state.model.workspace.content.design_management).clone();
        state
            .model
            .record_design_management_transaction(DesignManagementHistoryEntry {
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
        use rspice_app_state::state::{ComponentType, Point};

        let mut app = RSpiceApp::test_instance();
        assert!(
            app.state
                .model
                .schematic
                .with_undo("place resistor", |schematic| {
                    schematic.add_component(ComponentType::Resistor, Point::origin());
                })
        );
        let drawn = app.state.model.schematic.document().components.len();
        let owner = publish_sheet_assignment(&mut app.state);

        app.execute_command_key(ids::UNDO);
        assert_eq!(
            app.state.model.schematic.document().components.len(),
            drawn,
            "the edit made before the ascent is not what Undo owes the operator"
        );
        assert!(
            app.state
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_none(),
            "the ascent's own transaction is the step that reverts"
        );

        app.execute_command_key(ids::REDO);
        assert!(
            app.state
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_some()
        );
        assert_eq!(app.state.model.schematic.document().components.len(), drawn);
    }

    #[test]
    fn interleaved_project_and_document_edits_undo_in_global_sequence_order() {
        use rspice_app_state::state::{ComponentType, Point};

        let mut app = RSpiceApp::test_instance();
        assert!(
            app.state
                .model
                .schematic
                .with_undo("place resistor", |schematic| {
                    schematic.add_component(ComponentType::Resistor, Point::origin());
                })
        );
        let owner = publish_sheet_assignment(&mut app.state);
        assert!(
            app.state
                .model
                .schematic
                .with_undo("place capacitor", |schematic| {
                    schematic.add_component(ComponentType::Capacitor, Point::new(40, 0));
                })
        );
        assert_eq!(app.state.model.schematic.document().components.len(), 2);

        app.execute_command_key(ids::UNDO);
        assert_eq!(
            app.state.model.schematic.document().components.len(),
            1,
            "the newest step is the document edit made after the transaction"
        );
        assert!(
            app.state
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_some()
        );

        app.execute_command_key(ids::UNDO);
        assert!(
            app.state
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog(&owner.key())
                .is_none(),
            "the transaction is next, ahead of the older document edit"
        );
        assert_eq!(app.state.model.schematic.document().components.len(), 1);

        app.execute_command_key(ids::UNDO);
        assert!(app.state.model.schematic.document().components.is_empty());
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
        state.workbench.drawer =
            Some(rspice_app_workbench::workbench::state::workbench_session::Drawer::Navigator);

        assert!(shortcut_dispatch_blocked(&state, &ctx));
    }

    #[test]
    fn linked_cursor_command_toggles_real_results_state() {
        let mut app = RSpiceApp::test_instance();
        assert!(!app.state.results.results.session.linked_cursors);
        app.execute_command_key(
            rspice_app_results::workbench::results::commands::ResultsCommand::ToggleLinkedCursors
                .key(),
        );
        assert!(app.state.results.results.session.linked_cursors);
        app.execute_command_key(
            rspice_app_results::workbench::results::commands::ResultsCommand::ToggleLinkedCursors
                .key(),
        );
        assert!(!app.state.results.results.session.linked_cursors);
    }

    #[test]
    fn copy_shortcut_in_results_copies_cursor_readout_not_schematic_selection() {
        let mut app = RSpiceApp::test_instance();
        let analysis = rspice_app_state::state::AnalysisResult::new(
            1,
            rspice_app_state::state::AnalysisType::Transient,
            "TRAN",
        )
        .with_waveforms(vec![rspice_app_state::state::RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 2.0],
            "#ffbd2e",
        )]);
        let mut run = rspice_app_state::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.model.simulation.retained.runs = vec![run].into();
        assert!(app.state.model.simulation.select_run(0));
        app.state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Results);
        app.state.results.results.session.cursors.a = Some(0.5);
        app.state.results.results.session.cursor_strip = Some(0);

        app.execute_command_key(ids::COPY);

        use rspice_app_workbench::workbench::framework::requests::ClipboardText;
        let ClipboardText(copied) = app
            .state
            .workbench
            .outbox
            .peek::<ClipboardText>()
            .expect("cursor readout copied");
        assert!(copied.starts_with('A'));
        assert!(copied.contains("V(out)"));
    }

    #[test]
    fn object_properties_shortcut_dispatches_a_bus_and_escape_leaves_it_alone() {
        use rspice_app_state::state::{Bus, BusDeclaration, Point};

        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .buses
            .push(
                Bus::segment(
                    87,
                    Point::new(0, 0),
                    Point::new(20, 0),
                    Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
                )
                .unwrap(),
            );
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(87);

        app.execute_command_key(
            rspice_app_design::workbench::design::commands::DesignCommand::ObjectProperties.key(),
        );
        assert!(matches!(
            app.state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::Bus(_))
        ));
        let Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::Bus(draft)) =
            app.state.design.object_properties.draft.as_mut()
        else {
            unreachable!()
        };
        draft.declaration = "ADDR[7:0]".to_owned();
        app.state.design.object_properties.mark_edited();

        // The draft owns nothing until Apply, so one Escape leaves and the
        // bus is as it was.
        app.execute_command_key(ids::CANCEL);
        assert!(!app.state.design.object_properties.open);
        assert_eq!(
            app.state.model.schematic.document().buses[0].declaration,
            Some(BusDeclaration::parse("DATA[7:0]").unwrap())
        );
    }

    #[test]
    fn next_marker_resolves_again_after_first_jump_activates_design() {
        use rspice_app_state::state::command_catalog::CommandPlatform;
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
            .activate(rspice_app_state::workbench::workspace::Workspace::Verify);

        // The tour's position is the shared current finding, named by the
        // canonical identity its published record carries, so the Problems
        // grid's current row is the finding the canvas just jumped to.
        let subject = app
            .state
            .model
            .workspace
            .content
            .active_schematic_reference();
        let identities = app.state.model.design_rule_finding_identities(&subject);
        assert_eq!(identities.len(), 2, "both findings were published");

        let next =
            rspice_app_design::workbench::design::commands::DesignCommand::NextViolation.key();
        app.execute_command_key(next);
        assert_eq!(
            app.state.workbench.workspace,
            rspice_app_state::workbench::workspace::Workspace::Design
        );
        assert_eq!(
            app.state.model.diagnostics.current_finding(),
            Some(identities[0])
        );

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
        let profile = app.state.model.preferences.shortcuts().clone();
        let environment = ShortcutEnvironment {
            workspace: app.state.workbench.workspace,
            active_view: app.state.model.workspace.content.active_view_type(),
            canvas_focus: false,
            source_editor_focus: false,
        };
        let mut resolver = std::mem::take(&mut app.state.shortcut_resolver);
        let resolution = resolver.resolve(
            &snapshot,
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            environment,
            std::time::Duration::from_millis(1),
            |command| app.command_key_available(command),
        );
        app.state.shortcut_resolver = resolver;
        assert_eq!(resolution.command, Some(next));

        app.execute_command_key(resolution.command.unwrap());
        assert_eq!(
            app.state.model.diagnostics.current_finding(),
            Some(identities[1])
        );
        assert_eq!(
            app.state.workbench.workspace,
            rspice_app_state::workbench::workspace::Workspace::Design
        );
    }
}

#[cfg(test)]
mod symbol_action_tests {
    use super::*;
    use rspice_app_design::workbench::design::commands::DesignCommand;
    use rspice_app_state::state::{
        Cell, CellViewRef, Library, Point, PortDirection, SymbolDocument, SymbolPin, View, ViewType,
    };
    use rspice_app_workbench::workbench::framework::command_registry::ids;

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
        app.state.model.library_manager.add_library(library);
        app.state.open_workspace_view(reference);
        app.state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Design);

        let select =
            rspice_app_design::workbench::design::commands::DesignCommand::SelectTool.key();
        assert!(app.command_availability(select).is_available());
        for command in [
            DesignCommand::SymbolPinTool.key(),
            DesignCommand::SymbolPolylineTool.key(),
            DesignCommand::SymbolRectangleTool.key(),
            DesignCommand::SymbolCircleTool.key(),
            DesignCommand::SymbolArcTool.key(),
            DesignCommand::SymbolPolygonTool.key(),
            DesignCommand::SymbolTextTool.key(),
            ids::ZOOM_FIT,
        ] {
            assert!(
                app.command_availability(command).is_available(),
                "{command:?}"
            );
        }

        app.execute_command_key(DesignCommand::SymbolPinTool.key());
        assert_eq!(
            app.state.design.symbol.editor.tool,
            rspice_schematic_editor::symbol_editor::session::SymbolTool::PlacePin
        );
        assert_eq!(
            app.state.design.symbol.editor.selected_pin.as_deref(),
            Some("NEXT")
        );

        app.state.design.symbol.editor.pending_polyline = vec![Point::new(0, 0)];
        app.execute_command_key(DesignCommand::SymbolPolylineTool.key());
        assert_eq!(
            app.state.design.symbol.editor.tool,
            rspice_schematic_editor::symbol_editor::session::SymbolTool::Line
        );
        assert!(app.state.design.symbol.editor.pending_polyline.is_empty());

        for (command, expected) in [
            (
                DesignCommand::SymbolRectangleTool,
                rspice_schematic_editor::symbol_editor::session::SymbolTool::Rectangle,
            ),
            (
                DesignCommand::SymbolCircleTool,
                rspice_schematic_editor::symbol_editor::session::SymbolTool::Circle,
            ),
            (
                DesignCommand::SymbolArcTool,
                rspice_schematic_editor::symbol_editor::session::SymbolTool::Arc,
            ),
            (
                DesignCommand::SymbolPolygonTool,
                rspice_schematic_editor::symbol_editor::session::SymbolTool::Polygon,
            ),
            (
                DesignCommand::SymbolTextTool,
                rspice_schematic_editor::symbol_editor::session::SymbolTool::Text,
            ),
        ] {
            app.state.design.symbol.editor.shape_start = Some(Point::new(10, 10));
            app.execute_command_key(command.key());
            assert_eq!(app.state.design.symbol.editor.tool, expected);
            if matches!(
                command,
                DesignCommand::SymbolRectangleTool
                    | DesignCommand::SymbolCircleTool
                    | DesignCommand::SymbolArcTool
            ) {
                assert!(app.state.design.symbol.editor.shape_start.is_none());
            }
        }
        app.state.design.symbol.editor.shape_start = Some(Point::new(10, 10));
        app.execute_command_key(select);
        assert_eq!(
            app.state.design.symbol.editor.tool,
            rspice_schematic_editor::symbol_editor::session::SymbolTool::Select
        );

        app.state.design.symbol.editor.needs_fit = false;
        app.execute_command_key(ids::ZOOM_FIT);
        assert!(app.state.design.symbol.editor.needs_fit);
    }
}
