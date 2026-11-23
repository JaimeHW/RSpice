//! What the source feature carries out for the requests its commands, dialogs,
//! navigators and pages raise, each in its own context, and what it hands
//! the shell because it reaches past that context: the file workflow, the
//! LTspice and QSPICE import, a Design document, or a transaction over the
//! whole project.
//!
//! The shell lends the context at the drain point that answered each
//! request before, so nothing runs earlier or later than it did.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app::dialogs::source_document;
use crate::workbench::code::SourceFeature;
use crate::workbench::code::commands::{
    CodeHandOff, NetlistEditorRequest, SourceCommand, SourceDocumentRequest, SourceRequest,
};
use crate::workbench::documents::{code_workspace, netlist_document};
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::StandardCommand;
use crate::workbench::state::Workspace;

/// Carry out what the source feature raised, in its context `cx`: export the
/// deck from outside the netlist page, export the bill of materials, open
/// the generated deck, open the Code workspace's find, validation and
/// document tools, language navigation from the caret, the Verilog-A
/// compile and its build settings, and what the feature's dialogs,
/// navigators and pages committed. What reaches past the context comes back,
/// for the shell to carry out.
pub fn answer(cx: &mut Cx<'_, SourceFeature>, request: SourceRequest) -> Option<CodeHandOff> {
    match request {
        SourceRequest::OpenNetlist => return Some(CodeHandOff::OpenNetlist),
        SourceRequest::NewDeck => return Some(CodeHandOff::NewDeck),
        SourceRequest::SaveDeckAs => return Some(CodeHandOff::SaveDeckAs),
        SourceRequest::SaveNetlistAsDeck => return Some(CodeHandOff::SaveNetlistAsDeck),
        SourceRequest::ExportDeck => return Some(CodeHandOff::ExportDeck),
        SourceRequest::ExportDeckFiles => return Some(CodeHandOff::ExportDeckFiles),
        SourceRequest::ImportVerilogA => return Some(CodeHandOff::ImportVerilogA),
        SourceRequest::ImportLtspiceSchematic => return Some(CodeHandOff::ImportLtspiceSchematic),
        SourceRequest::ImportQspiceSchematic => return Some(CodeHandOff::ImportQspiceSchematic),
        SourceRequest::ExportNetlist => {
            crate::workbench::menu_bar::netlist_actions::action_export_netlist_with_io(
                cx.model,
                cx.wb,
                cx.handles.export_io,
            );
        }
        SourceRequest::ExportBillOfMaterials => {
            crate::workbench::menu_bar::bom_export::action_export_bill_of_materials_with_io(
                cx.model,
                cx.wb,
                cx.handles.export_io,
            );
        }
        SourceRequest::GenerateNetlist => {
            crate::workbench::surfaces::netlist::generation::show_generated_netlist(cx);
        }
        SourceRequest::ShowInNetlist => {
            // The instance is located in the netlist of the design as it
            // stands, so it is generated first.
            crate::workbench::surfaces::netlist::generation::refresh_generated_artifact(cx);
            return Some(CodeHandOff::ShowInNetlist);
        }
        SourceRequest::ManageSourceDocument => {
            if let Err(error) = source_document::open_source_document_dialog_in(
                cx.wb.workspace,
                cx.model,
                &mut cx.state.code_workspace,
                &cx.state.netlist,
                cx.platform, cx.wb.messages(),
            ) {
                cx.wb.prompt(error);
            }
        }
        SourceRequest::GoToDefinition => navigate_active_source(cx, SourceCommand::GoToDefinition),
        SourceRequest::FindReferences => navigate_active_source(cx, SourceCommand::FindReferences),
        SourceRequest::RenameSymbol => navigate_active_source(cx, SourceCommand::RenameSymbol),
        SourceRequest::CompileVerilogA => {
            // The command compiles the active Verilog-A cell view and
            // takes the reader to where its result is shown: the view is
            // edited in Design, so that is where a compile of it lands,
            // whichever workspace offered the command.
            cx.wb.activate(Workspace::Design);
            // The closure is taken now, at the click; the frame loop
            // starts the compile on its next pass. Nothing is asked.
            if let Err(error) =
                code_workspace::request_veriloga_compile(&mut SourceFeature::workspace_parts(cx))
            {
                cx.wb.prompt(error);
            }
        }
        SourceRequest::VerilogABuildSettings => {
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
        SourceRequest::VerilogAOpenSymbol => return Some(CodeHandOff::VerilogAOpenSymbol),
        SourceRequest::CreateVerilogADevice => return Some(CodeHandOff::CreateVerilogADevice),
        SourceRequest::ImportVerilogACells => return Some(CodeHandOff::ImportVerilogACells),
        SourceRequest::SaveVerilogABuildSettings { compile } => {
            match crate::workbench::app::dialogs::veriloga_build_settings::save_veriloga_build_settings(
                cx.model,
                &mut cx.state.code_workspace,
                &mut cx.state.veriloga_build_settings,
                cx.wb.messages(),
            ) {
                Ok(_) if compile => {
                    if let Err(error) = code_workspace::request_veriloga_compile(
                        &mut SourceFeature::workspace_parts(cx),
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
        SourceRequest::ReplaceVerilogASource => {
            if let Err(error) = code_workspace::commit_veriloga_source_replacement(
                &mut SourceFeature::workspace_parts(cx),
            ) {
                cx.model.record(LogSeverity::Error, LogSource::Studio, error);
            }
        }
        SourceRequest::SourceDocument(request) => answer_source_document(cx, request),
        SourceRequest::CommitSymbolRename => {
            if let Err(error) =
                code_workspace::commit_language_rename(&mut SourceFeature::workspace_parts(cx))
                && let Some(open) = cx.state.code_workspace.language_tools.as_mut()
            {
                open.error = Some(error);
            }
        }
        SourceRequest::UseLibSection {
            dependency,
            original,
            replacement,
            selected,
        } => {
            use crate::workbench::documents::netlist_document::{
                OwnedNetlistReplacement, replace_deck_files,
            };
            let edit = match dependency.as_deref() {
                Some(identity) => {
                    OwnedNetlistReplacement::dependency(identity, &original, replacement, 1)
                }
                None => OwnedNetlistReplacement::root(&original, replacement, 1),
            };
            match replace_deck_files(cx.model, &mut cx.state.netlist, vec![edit], cx.wb.messages()) {
                Ok(_) => cx.wb.prompt(selected),
                Err(error) => cx.wb.prompt(error),
            }
        }
        SourceRequest::SourceFileAction {
            bundle_id,
            action,
            path,
        } => {
            // The transaction's own form opens.
            let opened = code_workspace::open_source_file_dialog_in_bundle(
                &mut SourceFeature::workspace_parts(cx),
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
        SourceRequest::OpenCellInstance(usage) => {
            return Some(CodeHandOff::OpenCellInstance(usage));
        }
        SourceRequest::ShowNetInSchematic { schematic, net } => {
            return Some(CodeHandOff::ShowNetInSchematic { schematic, net });
        }
        SourceRequest::ApplyNetlistQuickFix => {
            // Add file… asks for the file, which the shell's pickers do.
            if let Some(request) =
                netlist_document::language::preferred_add_file_request(&cx.state.netlist)
            {
                return Some(CodeHandOff::AddDeckFile(request));
            }
            let answer = netlist_document::language::apply_preferred_quick_fix(
                cx.model,
                &mut cx.state.netlist, cx.wb.messages(),
            );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        SourceRequest::ApplyNetlistFindingFix(finding) => {
            if let Some(request) =
                netlist_document::language::add_file_request_for_finding(&cx.state.netlist, finding)
            {
                return Some(CodeHandOff::AddDeckFile(request));
            }
            let answer = netlist_document::language::apply_quick_fix_for_finding(
                cx.model,
                &mut cx.state.netlist,
                finding,
                cx.wb.messages(),
            );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        SourceRequest::FormatNetlistDocument => {
            crate::workbench::code::netlist_format::format_owned_netlist(cx);
        }
        SourceRequest::SaveDeckFile => {
            #[cfg(not(target_arch = "wasm32"))]
            if let Err(error) = crate::workbench::documents::deck_files::save_shown_file(
                &mut cx.state.netlist,
                cx.wb.messages(),
            ) {
                cx.wb.prompt(error);
            }
        }
        SourceRequest::OpenDroppedDeck(dropped) => {
            return Some(CodeHandOff::OpenDroppedDeck(dropped));
        }
    }
    None
}

/// Carry out what the netlist page's editor raised in its frame, as the
/// editor did in place: commit the text it showed, or take the undo or redo
/// step past its own history, saying why when the deck refuses it.
pub fn answer_netlist_editor(cx: &mut Cx<'_, SourceFeature>, request: NetlistEditorRequest) {
    match request {
        NetlistEditorRequest::Commit {
            front,
            cursor_line,
            now,
        } => netlist_document::commit_editor_edit(
            cx.model,
            &mut cx.state.netlist,
            cx.runs,
            &front,
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
pub fn step_history(cx: &mut Cx<'_, SourceFeature>, undo: bool) -> Result<Option<String>, String> {
    if undo {
        netlist_document::undo_netlist_edit(cx.model, &mut cx.state.netlist, cx.wb.messages())
    } else {
        netlist_document::redo_netlist_edit(cx.model, &mut cx.state.netlist, cx.wb.messages())
    }
}

/// Drop what the last Verilog-A compile proved, and stop a compile still
/// running, as a history step that put back another source asks.
pub fn void_veriloga_evidence(cx: &mut Cx<'_, SourceFeature>) {
    cx.state.code_workspace.veriloga.void_evidence();
}

/// Carry out what a file form, the revision history or the Files rail
/// handed over: a choice on a Verilog-A document.
fn answer_source_document(cx: &mut Cx<'_, SourceFeature>, request: SourceDocumentRequest) {
    match request {
        SourceDocumentRequest::Document {
            action,
            logical_path,
        } => source_document::apply_lifecycle_action(
            &mut SourceFeature::workspace_parts(cx),
            action,
            &logical_path,
        ),
    }
}

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen, saying why not when it cannot. Verilog-A
/// answers through the project source index, over the code workspace's
/// parts; a netlist answers through its own language service.
fn navigate_active_source(cx: &mut Cx<'_, SourceFeature>, command: SourceCommand) {
    let result = if code_workspace::design_hosts_veriloga(cx.wb.workspace, &cx.model.workspace) {
        crate::workbench::app::dialogs::source_language_tools::navigate_source_bundle(
            &mut SourceFeature::workspace_parts(cx),
            command,
        )
    } else {
        navigate_netlist(cx, command)
    };
    if let Err(error) = result {
        cx.wb.prompt(error);
    }
}

/// Go to definition, Find references and Rename symbol from the caret of
/// the netlist editor, through the engine's index of the deck and its
/// includes.
fn navigate_netlist(cx: &mut Cx<'_, SourceFeature>, command: SourceCommand) -> Result<(), String> {
    use crate::workbench::documents::netlist_document::language as netlist;
    use crate::workbench::documents::netlist_document::references;

    let message = match command {
        SourceCommand::GoToDefinition => Some(netlist::go_to_definition_at_cursor_in(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.runs,
            cx.wb,
        )?),
        SourceCommand::FindReferences => Some(references::find_references(
            cx.model,
            &mut cx.state.netlist,
            cx.wb,
        )?),
        SourceCommand::RenameSymbol => {
            cx.state.deck_rename = Some(references::DeckRename::at_caret(
                cx.model,
                &mut cx.state.netlist,
                cx.wb.messages(),
            )?);
            None
        }
        _ => None,
    };
    if let Some(message) = message {
        cx.wb.prompt(message);
    }
    Ok(())
}
