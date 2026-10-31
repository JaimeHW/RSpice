//! Whether a command can run right now: the one predicate every menu row,
//! toolbar button, palette entry and shortcut asks before it acts, and the
//! test [`Command::availability`] builds its reasons on.

use super::*;

impl Command {
    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        // The dispatcher's own guards, so what is drawn enabled is what runs.
        if self.held_by_project_operation(&state.platform) {
            return false;
        }
        if self.lacks_open_project(&state.platform) {
            return false;
        }
        if let Some((_, accepted)) = self.source_editor_redirect(&state.workbench) {
            return accepted;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                workspace_available(state.platform.project_lifecycle.is_open(), workspace)
            }
            Self::Save => {
                state.platform.project_lifecycle.is_open()
                    || state.model.schematic.session.current_file.is_some()
                    || state.platform.browser_schematic_save_name.is_some()
            }
            Self::SaveAs => state.platform.project_lifecycle.is_open(),
            Self::SaveAll => {
                state.platform.project_lifecycle.is_open()
                    && crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state)
            }
            Self::RevertActiveDocument => {
                state.platform.project_lifecycle.accepted().is_some()
                    && crate::workbench::lifecycle::project_lifecycle::active_document_is_dirty(state)
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::CloseActiveDocument => {
                crate::workbench::lifecycle::project_lifecycle::can_close_active_document(state)
            }
            Self::CloseProject => state.platform.project_lifecycle.is_open(),
            // Each is enabled exactly where its dialog could create something.
            Self::NewView => state.new_view_target().is_some(),
            Self::NewVerilogADevice => crate::workbench::app::new_veriloga_device_available(state),
            Self::OpenNetlist => {
                !state.model.simulation.execution.has_active_execution()
            }
            Self::ImportNetlist => {
                state.platform.project_lifecycle.is_open()
                    && !state.model.safe_mode.project_read_only()
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::Undo => {
                if state.model.safe_mode.project_read_only()
                    || state.schematic_edit_in_progress()
                {
                    false
                } else if (netlist_page_is_visible(state)
                    && crate::workbench::documents::netlist_document::can_undo_netlist_edit(state))
                    || state.model.can_undo_project_design()
                {
                    true
                } else if active_symbol_editor(app) {
                    state.can_undo_active_symbol_document()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && state.model.schematic.can_undo()
                }
            }
            Self::Redo => {
                if state.model.safe_mode.project_read_only()
                    || state.schematic_edit_in_progress()
                {
                    false
                } else if (netlist_page_is_visible(state)
                    && crate::workbench::documents::netlist_document::can_redo_netlist_edit(state))
                    || state.model.can_redo_project_design()
                {
                    true
                } else if active_symbol_editor(app) {
                    state.can_redo_active_symbol_document()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && state.model.can_redo_active_schematic()
                }
            }
            Self::Cut => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.design.symbol.editor.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && schematic_selection_has_live_object(&state.model.schematic)
                }
            }
            Self::Delete => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.design.symbol.editor.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && schematic_selection_has_deletable_object(&state.model.schematic)
                }
            }
            Self::Duplicate => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.design.symbol.editor.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && schematic_selection_has_duplicable_object(&state.model.schematic)
                }
            }
            Self::Copy => {
                if let Some(enabled) = app.routed_command_key_enabled(ids::COPY) {
                    enabled
                } else if active_symbol_editor(app) {
                    !state.design.symbol.editor.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && schematic_selection_has_live_object(&state.model.schematic)
                }
            }
            Self::Paste => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only() && !state.design.symbol.editor.clipboard.is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && !state.design.schematic_clipboard.is_empty()
                }
            }
            Self::SelectAll => active_symbol_editor(app) || active_schematic_editor(app),
            Self::RenameSelection => {
                active_schematic_editor(app)
                    && crate::workbench::app::rename_selection_available(state)
            }
            Self::RotateSelection
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
                if active_symbol_editor(app) =>
            {
                !state.active_view_read_only()
                    && !state.design.symbol.editor.effective_selection().is_empty()
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && (schematic_selection_has_live_object(&state.model.schematic)
                        || state.design.floating_paste.is_some())
            }
            Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical => {
                active_schematic_editor(app)
                    && selection_layout_command(self).is_some_and(|command| {
                        crate::schematic::view::selection_layout::selection_layout_availability(
                            state, command,
                        )
                        .is_ok()
                    })
            }
            Self::JoinCollinearWires => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::SnapSelectionToGrid => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && !state.model.schematic.session.editor.selection.is_empty()
            }
            Self::MoveSelection => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.model.schematic.has_live_movable_selection()
            }
            Self::StretchSelection => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.model.schematic.default_stretch_target().is_some()
            }
            Self::ArraySelection => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.model.schematic.validate_array_source_selection().is_ok()
            }
            Self::ReplaceInstance => {
                active_schematic_editor(app)
                    && crate::workbench::app::replace_instance_available(state)
            }
            // The repair is offered exactly where the deck refuses: the
            // schematic itself answers whether the selected instance still
            // presents its master's interface, so a live row and a netlist
            // defect can never disagree.
            Self::UpdateInstanceInterface => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state
                        .model.schematic
                        .selected_instance_interface_is_stale(
                            &state.model.library_manager,
                            &state.model.workspace.content.schematic_buffers,
                        )
            }
            Self::CreateHierarchy => {
                active_schematic_editor(app)
                    && state.model.create_hierarchy_available()
            }
            Self::ConnectivityManager => {
                active_schematic_editor(app) && state.platform.project_lifecycle.is_open()
            }
            Self::DesignManagement => {
                active_schematic_editor(app)
                    && state.platform.project_lifecycle.is_open()
                    && !state.schematic_edit_read_only()
            }
            Self::SelectionBulkEdit => {
                active_schematic_editor(app) && state.platform.project_lifecycle.is_open()
            }
            Self::FindReplace => active_schematic_editor(app) && !state.schematic_edit_read_only(),
            Self::RepeatLastCommand => {
                active_schematic_editor(app)
                    && vocabulary::last_repeatable(state.workbench.last_repeatable_command.as_deref())
                        .and_then(|spec| vocabulary::legacy_command(spec.key))
                        .is_some_and(|command| command.is_enabled(app))
            }
            Self::ConfigurationSets | Self::RuleSetup => state.platform.project_lifecycle.is_open(),
            Self::ReviewComments => {
                active_schematic_editor(app) && state.platform.project_lifecycle.is_open()
            }
            Self::RevisionHistory => state.platform.project_lifecycle.is_open(),
            Self::ObjectProperties => {
                if active_symbol_editor(app) {
                    let selection = state.design.symbol.editor.effective_selection();
                    selection.pins.len() + selection.shapes.len() == 1
                } else {
                    active_schematic_editor(app)
                        && (crate::workbench::app::selected_object_properties_available(state)
                            || crate::workbench::app::shared_properties_available(state))
                }
            }
            Self::ZoomFit => {
                active_symbol_editor(app)
                    || active_schematic_editor(app)
                    || app.routed_command_key_enabled(ids::ZOOM_FIT) == Some(true)
            }
            Self::FitSchematicContent => active_schematic_editor(app),
            // Magnifying a result viewport needs the sheet's own extents, so
            // it is offered on the unit-pane stack and nowhere it could not
            // be carried out.
            Self::ZoomIn | Self::ZoomOut => {
                active_symbol_editor(app)
                    || active_schematic_editor(app)
                    || app.routed_command_key_enabled(self.key()) == Some(true)
            }
            Self::ZoomOneToOne => active_symbol_editor(app) || active_schematic_editor(app),
            Self::ZoomToSelection => {
                active_schematic_editor(app) && schematic_selection_has_live_object(&state.model.schematic)
            }
            Self::ZoomBox => active_schematic_editor(app),
            Self::ResetActiveView => reset_active_view_available(state.workbench.workspace),
            Self::EngineeringTableView => active_schematic_editor(app),
            Self::NewApplicationWindow
            | Self::DetachDocument
            | Self::MoveDocumentToWindow
            | Self::ReattachDocument
            | Self::ConsolidateWindows
            | Self::MonitorRecovery
                if state.application_modal_open() =>
            {
                false
            }
            Self::NewApplicationWindow => true,
            Self::DetachDocument => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.active)
            }
            Self::MoveDocumentToWindow => {
                state.workbench.window_session.windows().count() > 1
                    && crate::workbench::chrome::document_bar::document_descriptors(state)
                        .iter()
                        .any(|document| document.open && document.active)
            }
            Self::ReattachDocument => {
                !state.workbench.window_session.current().is_primary()
                    && crate::workbench::chrome::document_bar::document_descriptors(state)
                        .iter()
                        .any(|document| document.open && document.active)
            }
            Self::ConsolidateWindows | Self::MonitorRecovery => {
                !state
                    .workbench
                    .window_session
                    .secondary_window_ids()
                    .is_empty()
            }
            Self::PreviousDocument | Self::NextDocument => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .filter(|document| document.open)
                    .count()
                    > 1
            }
            Self::CloseOtherDocuments => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable && !document.active)
            }
            Self::CloseAllDocuments => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable)
            }
            Self::CycleGrid => active_symbol_editor(app) || active_schematic_editor(app),
            Self::GridSnapRouting => active_schematic_editor(app),
            Self::VisibilityOptions => active_schematic_editor(app),
            Self::SelectTool => active_symbol_editor(app) || active_schematic_editor(app),
            Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceProbe
            | Self::PlacePin
            | Self::PlaceText
            | Self::PlaceShape => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::PlaceInstance
            | Self::PlaceLabel
            | Self::PlaceOffSheetConnector
            | Self::Place(_) => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolRectangleTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolPolygonTool
            | Self::SymbolTextTool => active_symbol_editor(app) && !state.active_view_read_only(),
            Self::SymbolRotatePin | Self::SymbolMirrorPin => {
                active_symbol_editor(app)
                    && !state.active_view_read_only()
                    && state.design.symbol.editor.effective_selection().pins.len() == 1
                    && state.design.symbol.editor.effective_selection().shapes.is_empty()
                    && state.design.symbol.editor.effective_selection().attributes.is_empty()
            }
            // Nothing to reconcile against without a declared interface, and
            // the command must say so by reporting unavailable rather than
            // opening on an empty contract.
            Self::SymbolUpdatePinsFromContract => {
                active_symbol_editor(app)
                    && !state.active_view_read_only()
                    && !state.active_symbol_ports().is_empty()
            }
            Self::SymbolSave => active_symbol_editor(app) && !state.active_view_read_only(),
            Self::EditSymbolInPlace => {
                active_schematic_editor(app) && state.selected_symbol_in_place_target().is_ok()
            }
            Self::ExitSymbolInPlace => {
                active_symbol_editor(app) && state.symbol_in_place().is_some()
            }
            Self::AscendHierarchy => {
                active_schematic_editor(app) && state.model.workspace.content.hierarchy_stack.len() > 1
            }
            Self::DescendHierarchy => {
                active_schematic_editor(app) && state.selected_hierarchy_master().is_some()
            }
            // The direct route opens what the instance is an instance of, as
            // its context-menu row does: a Verilog-A instance's module opens
            // as its cell view.
            Self::DescendHierarchyDirect => {
                active_schematic_editor(app)
                    && (state.selected_hierarchy_master().is_some()
                        || state.selected_veriloga_source().is_some())
            }
            Self::ShowInNetlist => {
                active_schematic_editor(app) && state.selected_instance_netlist_block().is_none()
            }
            // A symbol cellview checks its pin contract. The editor's own
            // keyboard route always did this; the command must agree, or the
            // toolbar shows a dead control beside a working shortcut.
            Self::CheckCellview => active_schematic_editor(app) || active_symbol_editor(app),
            // The design is the configured one, whichever view is open.
            Self::CheckDesign => true,
            Self::CheckAndSave => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::ClearChecks if active_symbol_editor(app) => {
                state.active_symbol_pin_finding_count().is_some()
            }
            Self::ClearChecks => {
                state.model.design_checks.drc_results.is_some()
                    || state.active_design_check_status().last_receipt().is_some()
            }
            Self::NextViolation | Self::PreviousViolation if active_symbol_editor(app) => {
                state
                    .active_symbol_pin_finding_count()
                    .is_some_and(|findings| findings > 0)
            }
            Self::NextViolation | Self::PreviousViolation => state
                .active_design_check_status()
                .current_receipt()
                .is_some_and(|receipt| !receipt.result.violations().is_empty()),
            Self::RunSimulation => {
                if state.workbench.workspace == Workspace::Netlist {
                    app.manual_deck_run_block_reason().is_none()
                } else {
                    !state.model.simulation.execution.has_active_execution()
                        && !state.model.simulation.execution.trigger_simulation
                }
            }
            Self::PreflightChecks => state.runs.preflight.checking.is_none(),
            // Both edit the plan catalog through the active plan's stable
            // identity, so a project whose plan never migrated to it has
            // nothing to select, nothing to add to, and no dialog worth opening.
            Self::ManageSimulationPlans | Self::AddAnalysis => {
                state.model.sim_setup.stable_analysis_plan().is_ok()
            }
            // Each of these opens a window that only the netlist page draws.
            // Availability therefore has to name the page, not just the
            // workspace: from Automation they would set a dialog open with
            // nothing on screen, and it would then appear unprompted the
            // moment the user came back.
            // Find exists on every code document: the netlist page has its
            // own deck find, and the Automation page and a Verilog-A cell view
            // in Design search their bundle. It was restricted to the netlist
            // page only while the bundle search had no surface.
            Self::FindCodeDocument => code_document_is_visible(state),
            Self::ValidateCodeDocument => {
                netlist_page_is_visible(state)
                    && state.code.netlist.active_document
                        != crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
                    && !state.model.simulation.source.netlist_content.trim().is_empty()
            }
            // Unlike its neighbours this one belongs to the source documents,
            // the Automation page and a Verilog-A cell view, not the netlist
            // page: it manages a project source bundle, which the deck is not.
            Self::ManageSourceDocument => {
                code_document_is_visible(state)
                    && crate::workbench::app::source_document_dialog_is_available(state)
            }
            Self::SourceLanguageTools => {
                code_document_is_visible(state)
                    && crate::workbench::app::language_tools_are_available(state)
            }
            // With a source editor on screen these were answered above.
            Self::ToggleLineComment
            | Self::IndentLines
            | Self::OutdentLines
            | Self::DuplicateLine
            | Self::DeleteLine
            | Self::MoveLineUp
            | Self::MoveLineDown
            | Self::GoToMatchingBracket
            | Self::GoToLine
            | Self::FormatDocument
            | Self::FoldSelection
            | Self::UnfoldAtCaret
            | Self::FoldAll
            | Self::UnfoldAll => false,
            Self::GoToDefinition | Self::FindReferences => state
                .workbench
                .active_source_editor
                .is_some_and(|editor| editor.navigable),
            Self::RenameSymbol => state
                .workbench
                .active_source_editor
                .is_some_and(|editor| editor.navigable && editor.editable),
            // This gate was written for the compile command and never
            // attached, so the button stayed enabled with no bundle, no
            // selected source, an empty root, a compile already running, or a
            // read-only project -- and then the surface refused the request it
            // had just been handed.
            Self::CompileVerilogA => code_context::resolve_veriloga_compile(app).is_some(),
            // Build settings opens on a read-only project too, to be read;
            // its Save is what that refuses.
            Self::VerilogABuildSettings => {
                crate::workbench::app::veriloga_build_settings_available(app)
            }
            Self::CompareGeneratedRevisions => {
                netlist_page_is_visible(state)
                    && !state.code.netlist.generated_history.is_empty()
                    && state.model.netlist_artifacts.generated_document.is_some()
            }
            Self::StopSimulation => {
                state.runs.preflight.checking.is_some()
                    || stop_simulation_enabled(&state.model.simulation)
            }
            // Every library verb needs the instrument open, because they all
            // act on the definition it is reading; New and Validate need only
            // that, because a library with nothing in it is exactly where New
            // belongs.
            Self::StimulusNewDefinition | Self::StimulusValidateLibrary => {
                state.workbench.workspace == Workspace::Stimulus
            }
            Self::StimulusDuplicateDefinition | Self::StimulusDeleteDefinition => {
                state.workbench.workspace == Workspace::Stimulus
                    && state.design.selected_stimulus_definition.is_some()
            }
            // Placing is an edit of the schematic, so a read-only view is the
            // one extra thing it needs beyond a definition to place.
            Self::StimulusPlaceDefinition => {
                state.workbench.workspace == Workspace::Stimulus
                    && state.design.selected_stimulus_definition.is_some()
                    && !state.schematic_edit_read_only()
            }
            Self::StimulusApplyDraft | Self::StimulusRevertDraft => {
                state.workbench.workspace == Workspace::Stimulus
                    && crate::workbench::app::actions::stimulus::selected_draft_is_dirty(state)
            }
            Self::StimulusShowAdopter => {
                state.workbench.workspace == Workspace::Stimulus
                    && crate::workbench::app::actions::stimulus::selected_definition_has_adopter(
                        state,
                    )
            }
            // A run's log is a record; only the session's log is cleared.
            Self::ClearConsole => match state.workbench.console_page {
                ConsolePage::Console => {
                    state.workbench.console_view.channel == ConsoleChannel::Session
                        && !state.model.log_buffer.is_empty()
                }
                ConsolePage::Problems | ConsolePage::Measurements | ConsolePage::TaskLog => false,
            },
            _ => true,
        }
    }

    /// Whether the state this command switches is on right now, for a
    /// command that switches one; `None` for a command that acts once.
    ///
    /// Each reading is the one the command flips, so choosing the row always
    /// turns its mark over. The console reads off in focus mode because
    /// focus mode hides it and the command leaves focus mode to show it.
    pub fn checked(self, app: &RSpiceApp) -> Option<bool> {
        let workbench = &app.state.workbench;
        match self {
            Self::ToggleNavigator => Some(workbench.navigator_visible),
            Self::ToggleInspector => Some(workbench.inspector_visible),
            Self::ToggleConsole => Some(workbench.console_visible && !workbench.focus_mode),
            Self::ToggleFocusMode => Some(workbench.focus_mode),
            Self::ToggleFullScreen => Some(workbench.full_screen_presentation),
            _ => None,
        }
    }
}
