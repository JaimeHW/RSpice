//! Whether a command can run right now: the one predicate every menu row,
//! toolbar button, palette entry and shortcut asks before it acts, and the
//! test [`Command::availability`] builds its reasons on.

use super::*;

impl Command {
    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state)
            && self.blocked_by_project_operation()
        {
            return false;
        }
        if self.requires_open_project() && !state.project_lifecycle.is_open() {
            return false;
        }
        if let Some(request) = self.source_editor_request()
            && let Some(editor) = state.workbench.active_source_editor
        {
            return editor.editable || !request.mutates_source();
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                workspace_available(state.project_lifecycle.is_open(), workspace)
            }
            Self::Save => {
                state.project_lifecycle.is_open()
                    || state.model.schematic.session.current_file.is_some()
                    || state.browser_schematic_save_name.is_some()
            }
            Self::SaveAs => state.project_lifecycle.is_open(),
            Self::SaveAll => {
                state.project_lifecycle.is_open()
                    && crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state)
            }
            Self::RevertActiveDocument => {
                state.project_lifecycle.accepted().is_some()
                    && crate::workbench::lifecycle::project_lifecycle::active_document_is_dirty(state)
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::CloseActiveDocument => {
                crate::workbench::lifecycle::project_lifecycle::can_close_active_document(state)
            }
            Self::CloseProject => state.project_lifecycle.is_open(),
            // Each is enabled exactly where its dialog could create something.
            Self::NewView => state.new_view_target().is_some(),
            Self::NewVerilogADevice => crate::workbench::app::new_veriloga_device_available(state),
            Self::OpenNetlist => {
                !state.model.simulation.execution.has_active_execution()
            }
            Self::ImportNetlist => {
                state.project_lifecycle.is_open()
                    && !state.model.safe_mode.project_read_only()
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::PublishToWeb => {
                matches!(
                    app.cloud_account.availability(),
                    crate::services::cloud_account::CloudAccountAvailability::Native
                ) && app
                    .cloud_account
                    .snapshot()
                    .cloud_feature_enabled(crate::services::cloud_account::CLOUD_PUBLISHING_FEATURE)
            }
            // Hosting needs an open project, but the same dialog joins by
            // code from any state — so the command never requires one.
            Self::LiveSession => {
                matches!(
                    app.cloud_account.availability(),
                    crate::services::cloud_account::CloudAccountAvailability::Native
                        | crate::services::cloud_account::CloudAccountAvailability::Browser
                ) && app.cloud_account.snapshot().cloud_feature_enabled(
                    crate::services::cloud_account::LIVE_COLLABORATION_FEATURE,
                )
            }
            Self::SheetProperties => {
                crate::workbench::app::drawing_sheet_edit::drawing_sheet_editable(state)
            }
            Self::SheetFormatManager => {
                crate::workbench::app::drawing_sheet_edit::drawing_sheet_editable(state)
                    && !state.schematic_edit_read_only()
                    && !state.sheets.drawing_sheet_support.manager.open
                    && state
                        .model.workspace
                        .content.design_management
                        .sheet_catalog(&state.model.workspace.content.active_key())
                        .is_some_and(|catalog| catalog.active().is_some())
            }
            Self::CustomSheetSizes => {
                crate::workbench::app::drawing_sheet_edit::drawing_sheet_editable(state)
                    && !state.sheets.drawing_sheet_presets.any_open()
            }
            Self::SheetDefaults => {
                crate::workbench::app::drawing_sheet_edit::drawing_sheet_editable(state)
                    && !state.sheets.drawing_sheet_defaults.open
            }
            Self::ExportActiveView => {
                crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(state)
            }
            Self::PrintHardcopy => {
                cfg!(any(target_os = "windows", target_arch = "wasm32"))
                    && crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(
                        state,
                    )
            }
            Self::Undo => {
                if state.model.safe_mode.project_read_only()
                    || state.schematic_edit_in_progress()
                {
                    false
                } else if (netlist_page_is_visible(state)
                    && crate::workbench::documents::netlist_document::can_undo_netlist_edit(state))
                    || state.can_undo_project_design()
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
                    || state.can_redo_project_design()
                {
                    true
                } else if active_symbol_editor(app) {
                    state.can_redo_active_symbol_document()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && state.can_redo_active_schematic()
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
                if state.workbench.workspace == Workspace::Results {
                    state.results.results.session.cursors.a.is_some()
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
                    && crate::workbench::app::create_hierarchy_available(state)
            }
            Self::ConnectivityManager => {
                active_schematic_editor(app) && state.project_lifecycle.is_open()
            }
            Self::DesignManagement => {
                active_schematic_editor(app)
                    && state.project_lifecycle.is_open()
                    && !state.schematic_edit_read_only()
            }
            Self::SelectionBulkEdit => {
                active_schematic_editor(app) && state.project_lifecycle.is_open()
            }
            Self::FindReplace => active_schematic_editor(app) && !state.schematic_edit_read_only(),
            Self::RepeatLastCommand => {
                active_schematic_editor(app)
                    && Self::last_repeatable(app).is_some_and(|command| command.is_enabled(app))
            }
            Self::ConfigurationSets | Self::RuleSetup => state.project_lifecycle.is_open(),
            Self::ReviewComments => {
                active_schematic_editor(app) && state.project_lifecycle.is_open()
            }
            Self::RevisionHistory => state.project_lifecycle.is_open(),
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
                    || (state.workbench.workspace == Workspace::Results
                        && crate::workbench::documents::result_document::fit_gesture_available(state))
            }
            Self::FitSchematicContent | Self::DrawingSheetLayers => active_schematic_editor(app),
            // Magnifying a result viewport needs the sheet's own extents, so
            // it is offered on the unit-pane stack and nowhere it could not
            // be carried out.
            Self::ZoomIn | Self::ZoomOut => {
                active_symbol_editor(app)
                    || active_schematic_editor(app)
                    || (state.workbench.workspace == Workspace::Results
                        && crate::workbench::documents::result_document::zoom_gesture_available(state))
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
            Self::ToggleResultsSplit => {
                state.project_lifecycle.is_open()
                    && state.workbench.supports_results_split()
                    && state.model.simulation.retained.has_retained_result_dataset()
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
            // Sheet navigation is offered only where there is a second sheet
            // to reach. A drawing with no sheet catalog is one sheet, and a
            // new sheet there creates the catalog.
            Self::NextSheet | Self::PreviousSheet => {
                active_schematic_editor(app) && sheet_actions::sheet_count(state) > 1
            }
            Self::NewSheet => active_schematic_editor(app) && !state.schematic_edit_read_only(),
            Self::DeleteSheet => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && sheet_actions::sheet_count(state) > 1
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
            Self::PreflightChecks => state.workbench.preflight.checking.is_none(),
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
                state.workbench.preflight.checking.is_some()
                    || stop_simulation_enabled(&state.model.simulation)
            }
            Self::ClearResults => {
                state.model.simulation.retained.has_results()
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::ImportResultDataset => {
                !state.model.safe_mode.project_read_only()
                    && !state.model.simulation.execution.has_active_execution()
            }
            // Waves is the workspace's default sheet, and its empty state is
            // the workspace's landing, so this route stays actionable before a
            // dataset exists. Every other viewer is only actionable when the
            // active retained dataset satisfies the same compatibility
            // contract used by the in-workspace viewer tabs.
            Self::ResultViewer(crate::workbench::ResultViewer::Waves) => true,
            Self::ResultViewer(viewer) => {
                crate::workbench::documents::result_document::viewer_is_available(state, viewer)
            }
            Self::ToggleLinkedCursors => {
                state.workbench.workspace == Workspace::Results && state.model.simulation.retained.has_results()
            }
            Self::DatasetManifestBrowser => {
                state.project_lifecycle.is_open() && !state.model.simulation.retained.runs.is_empty()
            }
            Self::CreateResultDocument => {
                state.project_lifecycle.is_open() && state.model.simulation.retained.has_results()
            }
            Self::CompareResultDatasets => {
                crate::workbench::documents::result_document::compare_available(state)
            }
            Self::ModelCreateProjectCopy => selected_model_for_project_copy(app).is_ok(),
            Self::ModelEditor | Self::ModelCorrelation => {
                selected_project_model_for_editor(app).is_ok()
            }
            Self::ModelSaveRevision => {
                state
                    .workbench
                    .model_editor
                    .draft
                    .as_ref()
                    .is_some_and(|draft| {
                        draft.is_dirty()
                            && !state.model.safe_mode.project_read_only()
                            && state.project_lifecycle.is_open()
                            && state
                                .workbench
                                .model_editor
                                .qualification_execution
                                .is_none()
                    })
            }
            Self::ModelValidate => state.workbench.model_editor.draft.is_some(),
            // Every library verb needs the instrument open, because they all
            // act on the definition it is reading; New and Validate need only
            // that, because a library with nothing in it is exactly where New
            // belongs.
            Self::StimulusNewDefinition | Self::StimulusValidateLibrary => {
                state.workbench.workspace == Workspace::Stimulus
            }
            Self::StimulusDuplicateDefinition | Self::StimulusDeleteDefinition => {
                state.workbench.workspace == Workspace::Stimulus
                    && state.workbench.selected_stimulus_definition.is_some()
            }
            // Placing is an edit of the schematic, so a read-only view is the
            // one extra thing it needs beyond a definition to place.
            Self::StimulusPlaceDefinition => {
                state.workbench.workspace == Workspace::Stimulus
                    && state.workbench.selected_stimulus_definition.is_some()
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
            Self::ModelRunQualificationTests => state
                .workbench
                .model_editor
                .draft
                .as_ref()
                .is_some_and(|draft| {
                    state.project_lifecycle.is_open()
                        && !state.model.safe_mode.project_read_only()
                        && !draft.definition_is_dirty()
                        && draft
                            .qualification
                            .validate_for_model(&draft.model_name)
                            .is_ok()
                        && crate::state::model_library::ModelSourceEvidenceBinding::try_new_project_bound(
                            &draft.model_name,
                            draft.source_id,
                            draft.base_source_digest,
                            draft.base_source_revision,
                        )
                        .is_ok_and(|source| {
                            draft
                                .qualification
                                .exact_suites_for_source(&source)
                                .is_ok_and(|suites| !suites.is_empty())
                        })
                        && state.workbench.model_editor.validation.is_some()
                        && state
                            .workbench
                            .model_editor
                            .qualification_execution
                            .is_none()
                }),
            Self::ModelCompareRelease => {
                state
                    .workbench
                    .model_editor
                    .draft
                    .as_ref()
                    .is_some_and(|draft| {
                        !draft.definition_is_dirty()
                            && !draft.qualification.candidates.is_empty()
                            && !draft.qualification.releases.is_empty()
                            && draft
                                .qualification
                                .validate_for_model(&draft.model_name)
                                .is_ok()
                            && state
                                .workbench
                                .model_editor
                                .qualification_execution
                                .is_none()
                    })
            }
            Self::ReportAuthoring => crate::workbench::surfaces::report_authoring::can_open(state),
            Self::SaveReportDocument => {
                crate::workbench::surfaces::report_authoring::can_save_document(state)
            }
            Self::AddReportPage => crate::workbench::surfaces::report_authoring::can_add_page(state),
            Self::ReportPageProperties => {
                crate::workbench::surfaces::report_authoring::can_edit_page_properties(state)
            }
            Self::AddVisualizationPane => {
                state.workbench.current_route().surface_id() == crate::workbench::SurfaceId::Results
                    && crate::workbench::documents::result_document::add_pane_available(state)
            }
            // Add measurement draws over the result viewer, and over a quick
            // view it evaluates without keeping, so it needs only a result.
            Self::MeasurementLibrary => {
                state.workbench.current_route().surface_id() == crate::workbench::SurfaceId::Results
                    && state.model.simulation.retained.has_results()
            }
            Self::FamilySlicing => {
                state.workbench.current_route().surface_id() == crate::workbench::SurfaceId::Results
                    && crate::workbench::documents::result_document::family_available(state)
            }
            Self::VisualizationDocumentProperties => {
                state.workbench.current_route().surface_id() == crate::workbench::SurfaceId::Results
                    && state.model.simulation.retained.has_results()
            }
            // Both return edges are judged on the dataset, never on the
            // surface the reader is standing on: they exist precisely to be
            // taken from somewhere else.
            Self::OpenRunInResults => {
                state.project_lifecycle.is_open()
                    && state
                        .model.simulation
                        .active_run()
                        .is_some_and(|run| !run.analyses.is_empty())
            }
            Self::OpenProducingPlan => result_navigation::producing_plan_hop(app).is_ok(),
            Self::OpenTaskDeck => result_navigation::task_deck_hop(app).is_ok(),
            Self::RevealProducerLog => result_navigation::producer_log_hop(app).is_ok(),
            Self::ExpressionDiagnostics => state.model.simulation.retained.has_results(),
            Self::ExportWaveformsCsv => result_navigation::dataset_export_is_resolvable(state),
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
            Self::ToggleResultsSplit => Some(workbench.split_with_results),
            Self::ToggleFocusMode => Some(workbench.focus_mode),
            Self::ToggleFullScreen => Some(workbench.full_screen_presentation),
            Self::ToggleLinkedCursors => {
                Some(crate::workbench::documents::result_document::linked_cursors(&app.state))
            }
            _ => None,
        }
    }
}
