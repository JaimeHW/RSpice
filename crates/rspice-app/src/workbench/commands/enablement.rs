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
            Self::RevertActiveDocument => {
                state.platform.project_lifecycle.accepted().is_some()
                    && crate::workbench::lifecycle::project_lifecycle::active_document_is_dirty(
                        state,
                    )
                    && !state.model.simulation.execution.has_active_execution()
            }
            Self::CloseActiveDocument => {
                crate::workbench::lifecycle::project_lifecycle::can_close_active_document(state)
            }
            // The netlist page's own history is the shell's to step; the
            // design feature's part answers for every other.
            Self::Undo | Self::Redo => {
                let direction = if self == Self::Undo {
                    HistoryDirection::Undo
                } else {
                    HistoryDirection::Redo
                };
                (netlist_history_steps(state, direction)
                    && !state.model.safe_mode.project_read_only()
                    && !state.schematic_edit_in_progress())
                    || app.routed_command_key_enabled(self.key()) == Some(true)
            }
            // Whether the part that takes an edit command here can carry it out.
            Self::Cut
            | Self::Copy
            | Self::Paste
            | Self::Duplicate
            | Self::Delete
            | Self::SelectAll => app.routed_command_key_enabled(self.key()) == Some(true),
            Self::DeselectAll => {
                if active_symbol_editor(app) {
                    !state.design.symbol.editor.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.model.schematic.session.editor.selection.is_empty()
                }
            }
            Self::SelectOfKind(kind) => {
                active_schematic_editor(app)
                    && !crate::schematic::view::sheet_visibility::objects_of_kind_on_active_sheet(
                        state, kind,
                    )
                    .is_empty()
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
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
            Self::SnapToStandardValue => {
                active_schematic_editor(app)
                    && crate::schematic::view::standard_value_snap::snap_block_reason(state)
                        .is_none()
            }
            Self::SetStandardValueSeries(_) => true,
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
                    && state
                        .model
                        .schematic
                        .validate_array_source_selection()
                        .is_ok()
            }
            Self::SwapToRSpiceSymbols => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.model.schematic.can_swap_to_rspice_symbols()
            }
            // The repair is offered exactly where the deck refuses: the
            // schematic itself answers whether the selected instance still
            // presents its master's interface, so a live row and a netlist
            // defect can never disagree.
            Self::UpdateInstanceInterface => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.model.schematic.selected_instance_interface_is_stale(
                        &state.model.library_manager,
                        &state.model.workspace.content.schematic_buffers,
                    )
            }
            Self::HighlightNet => {
                active_schematic_editor(app) && crate::schematic::view::selected_net(state).is_ok()
            }
            Self::RepeatLastCommand => {
                active_schematic_editor(app)
                    && vocabulary::last_repeatable(
                        state.workbench.last_repeatable_command.as_deref(),
                    )
                    .is_some_and(|spec| app.command_key_enabled(spec.key))
            }
            Self::ConfigurationSets => state.platform.project_lifecycle.is_open(),
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
            // A view command is the answer of the part that takes it where the
            // reader is working: Results where it is in front, the design
            // feature in a schematic or symbol editor.
            Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomOneToOne
            | Self::ZoomToSelection
            | Self::ZoomBox
            | Self::ZoomPrevious => app.routed_command_key_enabled(self.key()) == Some(true),
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
            Self::ConsolidateWindows | Self::MonitorRecovery => !state
                .workbench
                .window_session
                .secondary_window_ids()
                .is_empty(),
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
            Self::AnnotateOperatingPoint | Self::AnnotateTransientVoltages => {
                active_schematic_editor(app)
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
                    && state
                        .design
                        .symbol
                        .editor
                        .effective_selection()
                        .shapes
                        .is_empty()
                    && state
                        .design
                        .symbol
                        .editor
                        .effective_selection()
                        .attributes
                        .is_empty()
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
                active_schematic_editor(app)
                    && state.model.workspace.content.hierarchy_stack.len() > 1
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
            // A symbol cellview checks its pin contract. The editor's own
            // keyboard route always did this; the command must agree, or the
            // toolbar shows a dead control beside a working shortcut.
            Self::CheckCellview => active_schematic_editor(app) || active_symbol_editor(app),
            // The design is the configured one, whichever view is open.
            Self::CheckDesign => true,
            Self::ClearChecks if active_symbol_editor(app) => {
                state.active_symbol_pin_finding_count().is_some()
            }
            Self::ClearChecks => {
                state.model.design_checks.drc_results.is_some()
                    || state.active_design_check_status().last_receipt().is_some()
            }
            Self::NextViolation | Self::PreviousViolation if active_symbol_editor(app) => state
                .active_symbol_pin_finding_count()
                .is_some_and(|findings| findings > 0),
            Self::NextViolation | Self::PreviousViolation => state
                .active_design_check_status()
                .current_receipt()
                .is_some_and(|receipt| !receipt.result.violations().is_empty()),
            // A run's log is a record; only the session's log is cleared.
            Self::ClearConsole => match state.workbench.console_page {
                ConsolePage::Console => {
                    state.workbench.console_view.channel == ConsoleChannel::Session
                        && !state.model.log_buffer.is_empty()
                }
                ConsolePage::Problems
                | ConsolePage::Measurements
                | ConsolePage::TaskLog
                | ConsolePage::References => false,
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
            Self::SetStandardValueSeries(series) => {
                Some(app.state.model.workspace.content.standard_value_series == series)
            }
            Self::AnnotateOperatingPoint => Some(
                app.state.design.schematic_visibility.annotations
                    == crate::state::SchematicAnnotationVisibility::OperatingPoint,
            ),
            Self::AnnotateTransientVoltages => Some(
                app.state.design.schematic_visibility.annotations
                    == crate::state::SchematicAnnotationVisibility::TransientVoltages,
            ),
            _ => None,
        }
    }
}
