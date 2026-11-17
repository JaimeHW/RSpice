//! What the code feature carries out for the requests its commands, dialogs,
//! navigators and pages raise, each in its own context, and what it hands
//! the shell because it reaches past that context: the file workflow, the
//! LTspice and QSPICE import, a Design document, or a transaction over the
//! whole project.
//!
//! The shell lends the context at the drain point that answered each
//! request before, so nothing runs earlier or later than it did.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app::dialogs::source_document;
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::{
    AutomationRequest, CodeCommand, CodeHandOff, CodeRequest, NetlistEditorRequest,
    SourceDocumentRequest,
};
use crate::workbench::documents::{code_workspace, netlist_document};
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::StandardCommand;
use crate::workbench::state::Workspace;

/// Carry out what the code feature raised, in its context `cx`: export the
/// deck from outside the netlist page, export the bill of materials, open
/// the generated deck, open the Code workspace's find, validation and
/// document tools, language navigation from the caret, the Verilog-A
/// compile and its build settings, and what the feature's dialogs,
/// navigators and pages committed. What reaches past the context comes back,
/// for the shell to carry out.
pub fn answer(cx: &mut Cx<'_, CodeFeature>, request: CodeRequest) -> Option<CodeHandOff> {
    match request {
        CodeRequest::OpenNetlist => return Some(CodeHandOff::OpenNetlist),
        CodeRequest::ImportNetlist => return Some(CodeHandOff::ImportNetlist),
        CodeRequest::ImportVerilogA => return Some(CodeHandOff::ImportVerilogA),
        CodeRequest::ImportLtspiceSchematic => return Some(CodeHandOff::ImportLtspiceSchematic),
        CodeRequest::ImportQspiceSchematic => return Some(CodeHandOff::ImportQspiceSchematic),
        CodeRequest::ExportNetlist(format) => {
            crate::workbench::menu_bar::netlist_actions::action_export_netlist_with_io(
                cx.model,
                cx.wb,
                format,
                cx.handles.export_io,
            );
        }
        CodeRequest::ExportBillOfMaterials => {
            crate::workbench::menu_bar::bom_export::action_export_bill_of_materials_with_io(
                cx.model,
                cx.wb,
                cx.handles.export_io,
            );
        }
        CodeRequest::GenerateNetlist => {
            crate::workbench::menu_bar::netlist_actions::action_view_netlist(cx.model, cx.wb);
            cx.wb.activate(Workspace::Netlist);
        }
        CodeRequest::ShowInNetlist => return Some(CodeHandOff::ShowInNetlist),
        CodeRequest::FindCodeDocument => {
            if let Err(error) =
                code_workspace::open_active_source_search(&mut CodeFeature::workspace_parts(cx))
            {
                cx.wb.prompt(error);
            }
        }
        CodeRequest::ValidateCodeDocument => {
            crate::workbench::code::netlist_validation::request_validation(cx, false);
        }
        CodeRequest::ManageSourceDocument => {
            if let Err(error) = source_document::open_source_document_dialog_in(
                cx.wb.workspace,
                cx.model,
                &mut cx.state.code_workspace,
                &cx.state.netlist,
                cx.platform,
            ) {
                cx.wb.prompt(error);
            }
        }
        CodeRequest::GoToDefinition => navigate_active_source(cx, CodeCommand::GoToDefinition),
        CodeRequest::FindReferences => navigate_active_source(cx, CodeCommand::FindReferences),
        CodeRequest::RenameSymbol => navigate_active_source(cx, CodeCommand::RenameSymbol),
        CodeRequest::CompileVerilogA => {
            // The command compiles the active Verilog-A cell view and
            // takes the reader to where its result is shown: the view is
            // edited in Design, so that is where a compile of it lands,
            // whichever workspace offered the command.
            cx.wb.activate(Workspace::Design);
            // The closure is taken now, at the click; the frame loop
            // starts the compile on its next pass. Nothing is asked.
            if let Err(error) =
                code_workspace::request_veriloga_compile(&mut CodeFeature::workspace_parts(cx))
            {
                cx.wb.prompt(error);
            }
        }
        CodeRequest::VerilogABuildSettings => {
            cx.wb.activate(Workspace::Design);
            if let Err(error) =
                crate::workbench::app::dialogs::veriloga_build_settings::open_veriloga_build_settings(
                    cx.model,
                    &mut cx.state.veriloga_build_settings,
                )
            {
                cx.wb.prompt(error);
            }
        }
        CodeRequest::VerilogAOpenSymbol => return Some(CodeHandOff::VerilogAOpenSymbol),
        CodeRequest::CreateVerilogADevice => return Some(CodeHandOff::CreateVerilogADevice),
        CodeRequest::ImportVerilogACells => return Some(CodeHandOff::ImportVerilogACells),
        CodeRequest::SaveVerilogABuildSettings { compile } => {
            match crate::workbench::app::dialogs::veriloga_build_settings::save_veriloga_build_settings(
                cx.model,
                &mut cx.state.code_workspace,
                &mut cx.state.veriloga_build_settings,
                cx.wb.messages(),
            ) {
                Ok(_) if compile => {
                    if let Err(error) = code_workspace::request_veriloga_compile(
                        &mut CodeFeature::workspace_parts(cx),
                    ) {
                        cx.wb.prompt(error);
                    }
                }
                Ok(_) => {}
                Err(error) => {
                    if let Some(draft) = cx.state.veriloga_build_settings.as_mut() {
                        draft.error = Some(error);
                    }
                }
            }
        }
        CodeRequest::ReplaceVerilogASource => {
            if let Err(error) = code_workspace::commit_veriloga_source_replacement(
                &mut CodeFeature::workspace_parts(cx),
            ) {
                cx.model.record(LogSeverity::Error, LogSource::Studio, error);
            }
        }
        CodeRequest::ReplaceSourceSearchMatches => {
            if let Err(error) =
                code_workspace::commit_source_search_replace(&mut CodeFeature::workspace_parts(cx))
            {
                if let Some(search) = cx.state.code_workspace.source_search.as_mut() {
                    search.error = Some(error);
                } else {
                    cx.wb.prompt(error);
                }
            }
        }
        CodeRequest::SourceDocument(request) => answer_source_document(cx, request),
        CodeRequest::CommitSymbolRename => {
            if let Err(error) =
                code_workspace::commit_language_rename(&mut CodeFeature::workspace_parts(cx))
                && let Some(open) = cx.state.code_workspace.language_tools.as_mut()
            {
                open.error = Some(error);
            }
        }
        CodeRequest::UseLibSection {
            dependency,
            original,
            replacement,
            selected,
        } => {
            use crate::workbench::documents::netlist_document::{
                OwnedNetlistReplacement, replace_owned_sources_atomically,
            };
            let edit = match dependency.as_deref() {
                Some(identity) => {
                    OwnedNetlistReplacement::dependency(identity, &original, replacement, 1)
                }
                None => OwnedNetlistReplacement::root(&original, replacement, 1),
            };
            match replace_owned_sources_atomically(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
                vec![edit],
            ) {
                Ok(_) => cx.wb.prompt(selected),
                Err(error) => cx.wb.prompt(error),
            }
        }
        CodeRequest::RelinkNetlistDependency(identity) => {
            crate::workbench::workflows::netlist_workflow::request_dependency_relink(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
                cx.runs,
                cx.wb,
                &identity,
            );
        }
        CodeRequest::SourceFileAction {
            language,
            bundle_id,
            action,
            path,
        } => {
            // The transaction's own form opens; Manage source document is
            // not behind it.
            let opened = code_workspace::open_source_file_dialog_in_bundle(
                &mut CodeFeature::workspace_parts(cx),
                language,
                bundle_id,
                action,
                &path,
                &path,
            );
            if let Err(error) = opened {
                cx.state.code_workspace.source_file_dialog = None;
                cx.wb.prompt(error);
            }
        }
        CodeRequest::OpenCellInstance(usage) => {
            return Some(CodeHandOff::OpenCellInstance(usage));
        }
        // Each of the netlist page's transactions commits through a
        // candidate copy, so the feature runs the body the dialog ran in
        // place over what the dialog stored.
        CodeRequest::CommitNetlistRename => {
            let messages = cx.wb.messages();
            match netlist_document::language::commit_rename(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
            ) {
                Ok(count) => cx.wb.prompt(messages.format(
                    crate::workbench::MessageId::NetlistRenameSucceeded,
                    &[("count", &count.to_string())],
                )),
                Err(error) => cx.state.netlist.rename_dialog.error = Some(error),
            }
        }
        CodeRequest::RestoreOwnedNetlistRevision => {
            let index = cx.state.netlist.comparison_dialog.selected_history_index;
            match netlist_document::restore_owned_revision(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
                index,
            ) {
                Ok(()) => cx.state.netlist.comparison_dialog.open = false,
                Err(error) => cx.wb.prompt(error),
            }
        }
        CodeRequest::ApplyExternalNetlistChange => {
            match crate::workbench::workflows::netlist_workflow::apply_staged_external_netlist_change(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
                cx.wb,
            ) {
                Ok(()) => {
                    cx.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Applied the resolved external change to the SPICE source.",
                    );
                }
                Err(error) => {
                    if let Some(current) = cx.state.netlist.external_change.as_mut() {
                        current.error = Some(error);
                    }
                }
            }
        }
        CodeRequest::CommitNetlistImport => return Some(CodeHandOff::CommitNetlistImport),
        CodeRequest::ApplyNetlistFind => {
            if let Some(step) = cx.state.netlist.find.pending.take() {
                crate::workbench::workflows::netlist_workflow::apply_netlist_find_step(
                    cx.model,
                    &mut cx.state.netlist,
                    &mut cx.state.netlist_open_documents,
                    cx.runs,
                    cx.wb,
                    step,
                );
            }
        }
        CodeRequest::CopyNetlistDependency => {
            let messages = cx.wb.messages();
            match netlist_document::copy_active_dependency_to_project(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
            ) {
                Ok(_) => {
                    cx.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        messages.text(crate::workbench::MessageId::NetlistCopySucceeded),
                    );
                }
                Err(error) => {
                    cx.model.record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
        }
        CodeRequest::ApplyNetlistQuickFix => {
            let answer = netlist_document::language::apply_preferred_quick_fix(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
            );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        CodeRequest::BeginNetlistProfileReview => {
            return Some(CodeHandOff::BeginNetlistProfileReview);
        }
        CodeRequest::FormatNetlistDocument => {
            crate::workbench::code::netlist_validation::format_owned_netlist(cx);
        }
        CodeRequest::ImportDroppedNetlist(dropped) => {
            return Some(CodeHandOff::ImportDroppedNetlist(dropped));
        }
        CodeRequest::Automation(request) => answer_automation(cx, request),
    }
    None
}

/// Carry out what the netlist page's editor raised in its frame, as the
/// editor did in place: commit the text it showed, or take the undo or redo
/// step past its own history, saying why when the deck refuses it.
pub fn answer_netlist_editor(cx: &mut Cx<'_, CodeFeature>, request: NetlistEditorRequest) {
    match request {
        NetlistEditorRequest::Commit {
            buffer,
            cursor_line,
            now,
        } => netlist_document::commit_editor_edit(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.wb,
            cx.runs,
            &buffer,
            cursor_line,
            now,
        ),
        NetlistEditorRequest::StepHistory(step) => {
            if let Err(error) = step_history(cx, step == StandardCommand::Undo) {
                cx.wb.prompt(error);
            }
        }
    }
}

/// Take the netlist page's own history step: Undo's where `undo`, otherwise
/// Redo's. `Ok(None)` when that history holds no step that way.
pub fn step_history(cx: &mut Cx<'_, CodeFeature>, undo: bool) -> Result<Option<String>, String> {
    if undo {
        netlist_document::undo_netlist_edit(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
        )
    } else {
        netlist_document::redo_netlist_edit(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
        )
    }
}

/// Drop what the last Verilog-A compile proved, and stop a compile still
/// running, as a history step that put back another source asks.
pub fn void_veriloga_evidence(cx: &mut Cx<'_, CodeFeature>) {
    cx.state.code_workspace.veriloga.void_evidence();
}

/// Carry out what the Automation page asked, as the page did where it drew.
fn answer_automation(cx: &mut Cx<'_, CodeFeature>, request: AutomationRequest) {
    match request {
        AutomationRequest::ImportDropped(dropped) => {
            let application_modal_open = cx.application_modal_open();
            code_workspace::import_dropped_automation_files(
                &mut CodeFeature::workspace_parts(cx),
                application_modal_open,
                dropped,
            );
        }
        AutomationRequest::CreateWorkspace => {
            if let Err(error) =
                code_workspace::open_source_workspace_dialog(&mut CodeFeature::workspace_parts(cx))
            {
                cx.wb.prompt(error);
            }
        }
        AutomationRequest::Validate => {
            code_workspace::validate_automation_workspace(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::DryRun => CodeFeature::dry_run_automation_workflow(cx),
        AutomationRequest::Run => CodeFeature::start_automation_workflow(cx),
        AutomationRequest::Stop => {
            code_workspace::cancel_automation_workflow(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::OpenGeneratedNetlist => {
            let _ = netlist_document::open_generated_primary_in(cx.model, &mut cx.state.netlist);
            cx.state.code_workspace.page = code_workspace::CodeWorkspacePage::Netlist;
        }
        AutomationRequest::ToggleBreakpoint(line) => {
            code_workspace::toggle_automation_breakpoint(
                &mut CodeFeature::workspace_parts(cx),
                line,
            );
        }
        AutomationRequest::FormatDocument { bundle_id, path } => {
            if let Err(error) = code_workspace::format_language_document(
                &mut CodeFeature::workspace_parts(cx),
                crate::state::ProjectSourceLanguage::RSpiceAutomation,
                bundle_id,
                &path,
            ) {
                cx.wb.prompt(error);
            }
        }
        AutomationRequest::FindInBundle { path } => {
            if let Err(error) = code_workspace::open_source_search(
                &mut CodeFeature::workspace_parts(cx),
                crate::state::ProjectSourceLanguage::RSpiceAutomation,
                &path,
            ) {
                cx.wb.prompt(error);
            }
        }
        AutomationRequest::ExportArtifact(kind) => {
            code_workspace::export_automation_artifact(&mut CodeFeature::workspace_parts(cx), kind);
        }
        AutomationRequest::StartDebugger => {
            code_workspace::start_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::ContinueDebugger => {
            code_workspace::continue_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::PauseDebugger => {
            code_workspace::pause_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::StepOver => {
            code_workspace::step_over_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::StepInto => {
            code_workspace::step_into_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::StepOut => {
            code_workspace::step_out_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::RestartDebugger => {
            code_workspace::restart_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::StopDebugger => {
            code_workspace::stop_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::RemoveBreakpoint { document_id, line } => {
            code_workspace::remove_automation_breakpoint(
                &mut CodeFeature::workspace_parts(cx),
                document_id,
                line,
            );
        }
        AutomationRequest::SyncBreakpoints => {
            code_workspace::sync_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
        AutomationRequest::SelectFrame(frame_id) => {
            code_workspace::select_automation_debug_frame(
                &mut CodeFeature::workspace_parts(cx),
                frame_id,
            );
        }
        AutomationRequest::RefreshWatches => {
            code_workspace::refresh_automation_debugger(&mut CodeFeature::workspace_parts(cx))
        }
    }
}

/// Carry out what a source document dialog handed over: create the
/// Automation workspace its draft describes or the top deck its creation
/// form drafted, or a lifecycle choice on the document it manages, then the
/// close the same frame asked for.
fn answer_source_document(cx: &mut Cx<'_, CodeFeature>, request: SourceDocumentRequest) {
    match request {
        SourceDocumentRequest::CreateWorkspace => {
            match code_workspace::commit_source_workspace_dialog(&mut CodeFeature::workspace_parts(
                cx,
            )) {
                Ok(message) => {
                    cx.model
                        .record(LogSeverity::Info, LogSource::Studio, message);
                }
                Err(error) => {
                    if let Some(state) = cx.state.code_workspace.source_workspace_dialog.as_mut() {
                        state.error = Some(error);
                    }
                }
            }
        }
        SourceDocumentRequest::Document {
            action,
            language,
            logical_path,
            close,
        } => {
            source_document::apply_lifecycle_action(
                &mut CodeFeature::workspace_parts(cx),
                action,
                language,
                &logical_path,
            );
            if close {
                source_document::close_document_lifecycle(&mut cx.state.code_workspace);
            }
        }
        SourceDocumentRequest::Netlist { action, close } => {
            crate::workbench::app::netlist_lifecycle::apply_netlist_lifecycle_action(
                &mut cx.state.code_workspace,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
                cx.model,
                cx.wb,
                action,
                close,
            );
        }
        SourceDocumentRequest::CreateTopDeck => {
            crate::workbench::app::netlist_lifecycle::commit_new_top_deck(
                &mut cx.state.code_workspace,
                &mut cx.state.netlist,
                &mut cx.state.netlist_open_documents,
                cx.model,
            );
        }
    }
}

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen, saying why not when it cannot. Verilog-A and
/// Automation answer through the project source index, over the code
/// workspace's parts; a netlist answers through its own language service.
fn navigate_active_source(cx: &mut Cx<'_, CodeFeature>, command: CodeCommand) {
    let language = code_workspace::active_source_language(
        cx.wb.workspace,
        &cx.model.workspace,
        cx.state.code_workspace.page,
    );
    let result = match language {
        Some(language) => {
            crate::workbench::app::dialogs::source_language_tools::navigate_source_bundle(
                &mut CodeFeature::workspace_parts(cx),
                language,
                command,
            )
        }
        None => navigate_netlist(cx, command),
    };
    if let Err(error) = result {
        cx.wb.prompt(error);
    }
}

/// Go to definition, Find references and Rename symbol from the caret of the
/// netlist editor, through the deck's own language service.
fn navigate_netlist(cx: &mut Cx<'_, CodeFeature>, command: CodeCommand) -> Result<(), String> {
    use crate::workbench::documents::netlist_document::language as netlist;

    let message = match command {
        CodeCommand::GoToDefinition => Some(netlist::go_to_definition_at_cursor_in(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.runs,
            cx.wb,
        )?),
        CodeCommand::FindReferences => Some(netlist::find_references_at_cursor_in(
            cx.model,
            &mut cx.state.netlist,
            cx.wb,
        )?),
        CodeCommand::RenameSymbol => {
            netlist::begin_rename_at_cursor_in(cx.model, &mut cx.state.netlist)?;
            None
        }
        _ => None,
    };
    if let Some(message) = message {
        cx.wb.prompt(message);
    }
    Ok(())
}
