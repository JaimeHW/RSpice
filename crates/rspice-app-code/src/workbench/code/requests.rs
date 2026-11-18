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
    CodeCommand, CodeHandOff, CodeRequest, NetlistEditorRequest, SourceDocumentRequest,
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
        CodeRequest::NewDeck => return Some(CodeHandOff::NewDeck),
        CodeRequest::SaveDeckAs => return Some(CodeHandOff::SaveDeckAs),
        CodeRequest::SaveNetlistAsDeck => return Some(CodeHandOff::SaveNetlistAsDeck),
        CodeRequest::ExportDeck => return Some(CodeHandOff::ExportDeck),
        CodeRequest::ExportDeckFiles => return Some(CodeHandOff::ExportDeckFiles),
        CodeRequest::ImportVerilogA => return Some(CodeHandOff::ImportVerilogA),
        CodeRequest::ImportLtspiceSchematic => return Some(CodeHandOff::ImportLtspiceSchematic),
        CodeRequest::ImportQspiceSchematic => return Some(CodeHandOff::ImportQspiceSchematic),
        CodeRequest::ExportNetlist => {
            crate::workbench::menu_bar::netlist_actions::action_export_netlist_with_io(
                cx.model,
                cx.wb,
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
            crate::workbench::surfaces::netlist::generation::show_generated_netlist(cx);
        }
        CodeRequest::ShowInNetlist => {
            // The instance is located in the netlist of the design as it
            // stands, so it is generated first.
            crate::workbench::surfaces::netlist::generation::refresh_generated_artifact(cx);
            return Some(CodeHandOff::ShowInNetlist);
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
                OwnedNetlistReplacement, replace_deck_files,
            };
            let edit = match dependency.as_deref() {
                Some(identity) => {
                    OwnedNetlistReplacement::dependency(identity, &original, replacement, 1)
                }
                None => OwnedNetlistReplacement::root(&original, replacement, 1),
            };
            match replace_deck_files(cx.model, &mut cx.state.netlist, vec![edit]) {
                Ok(_) => cx.wb.prompt(selected),
                Err(error) => cx.wb.prompt(error),
            }
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
        CodeRequest::ShowNetInSchematic { schematic, net } => {
            return Some(CodeHandOff::ShowNetInSchematic { schematic, net });
        }
        CodeRequest::ApplyNetlistQuickFix => {
            let answer = netlist_document::language::apply_preferred_quick_fix(
                cx.model,
                &mut cx.state.netlist,
            );
            cx.wb.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        CodeRequest::FormatNetlistDocument => {
            crate::workbench::code::netlist_format::format_owned_netlist(cx);
        }
        CodeRequest::SaveDeckFile => {
            #[cfg(not(target_arch = "wasm32"))]
            if let Err(error) =
                crate::workbench::documents::deck_files::save_shown_file(&mut cx.state.netlist)
            {
                cx.wb.prompt(error);
            }
        }
        CodeRequest::OpenDroppedDeck(dropped) => {
            return Some(CodeHandOff::OpenDroppedDeck(dropped));
        }
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
        netlist_document::undo_netlist_edit(cx.model, &mut cx.state.netlist)
    } else {
        netlist_document::redo_netlist_edit(cx.model, &mut cx.state.netlist)
    }
}

/// Drop what the last Verilog-A compile proved, and stop a compile still
/// running, as a history step that put back another source asks.
pub fn void_veriloga_evidence(cx: &mut Cx<'_, CodeFeature>) {
    cx.state.code_workspace.veriloga.void_evidence();
}

/// Carry out what a source document dialog handed over: a lifecycle choice
/// on the Verilog-A document it manages, then the close the same frame
/// asked for.
fn answer_source_document(cx: &mut Cx<'_, CodeFeature>, request: SourceDocumentRequest) {
    match request {
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
    }
}

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen, saying why not when it cannot. Verilog-A
/// answers through the project source index, over the code workspace's
/// parts; a netlist answers through its own language service.
fn navigate_active_source(cx: &mut Cx<'_, CodeFeature>, command: CodeCommand) {
    let language = code_workspace::active_source_language(cx.wb.workspace, &cx.model.workspace);
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

/// Go to definition, Find references and Rename symbol from the caret of
/// the netlist editor, through the engine's index of the deck and its
/// includes.
fn navigate_netlist(cx: &mut Cx<'_, CodeFeature>, command: CodeCommand) -> Result<(), String> {
    use crate::workbench::documents::netlist_document::language as netlist;
    use crate::workbench::documents::netlist_document::references;

    let message = match command {
        CodeCommand::GoToDefinition => Some(netlist::go_to_definition_at_cursor_in(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.runs,
            cx.wb,
        )?),
        CodeCommand::FindReferences => Some(references::find_references(
            cx.model,
            &mut cx.state.netlist,
            cx.wb,
        )?),
        CodeCommand::RenameSymbol => {
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
