//! The code feature: the Netlist and Automation workspaces, the Verilog-A
//! views the Design workspace opens, and the source editors they share.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

pub(crate) mod commands;
mod docks;

use egui::Context;

use crate::io::NetlistFormat;
use crate::state::ViewType;
use crate::state::command_catalog::CommandKey;
use crate::workbench::app::actions::veriloga_symbol::can_open_veriloga_symbol;
use crate::workbench::app::dialogs::library::new_veriloga_device::{
    self, new_veriloga_device_available_in, open_new_veriloga_device_form_in,
};
use crate::workbench::app::dialogs::library::veriloga_import;
use crate::workbench::app::dialogs::{
    source_document, source_find, source_language_tools, veriloga_build_settings,
};
use crate::workbench::app::{
    source_document_dialog_is_available, veriloga_build_settings_available,
};
use crate::workbench::commands::code_context::{
    resolve_veriloga_compile, veriloga_compile_refusal,
};
use crate::workbench::commands::foreign_import_refusal;
use crate::workbench::documents::code_workspace::{
    CodeWorkspacePage, ProblemStep, cancel_veriloga_compile, consume_veriloga_compile_request,
    design_hosts_veriloga, netlist_page_shown, poll_automation_source_import,
    poll_veriloga_compile, poll_veriloga_import, refresh_symbol_standing,
};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureNavigator, FeatureRegistry,
    Reads,
};
use crate::workbench::slots::CodeState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use commands::{CodeCommand, CodeRequest};

/// The code feature.
pub(crate) struct CodeFeature;

impl Feature for CodeFeature {
    type State = CodeState;
    type Session = WorkbenchSession;
    type Handles = crate::workbench::documents::code_workspace::CodeHandles;
    /// The Netlist inspector offers the tuning sandbox while the dispatcher
    /// would open it, and the deck page's toolbar offers Run or Stop while
    /// the dispatcher would run or stop, stating Stop's refusal in its words.
    const FOREIGN_AVAILABILITY: &'static [CommandKey] =
        &[ids::TUNING, ids::RUN_SIMULATION, ids::STOP_SIMULATION];

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<CodeCommand>();
        registry.dialogs(DIALOGS);
        registry.navigator(FeatureNavigator {
            show: docks::show_navigator,
        });
        registry.inspector(FeatureInspector {
            show: docks::show_inspector,
            // Nothing the section draws holds an edit for the shell to commit.
            commit_pending_edit: |_, _| {},
        });
    }
}

/// The dialog steps the feature draws, under the keys the shell's dialog
/// order lists them by: the source document dialogs, Find in source,
/// Rename symbol, New Verilog-A device, Import Verilog-A/AMS, Replace
/// source from file and Build settings. Each is open while its form is.
const DIALOGS: &[FeatureDialog<CodeFeature>] = &[
    FeatureDialog {
        key: DialogKey("code.source_document"),
        is_open: |code| {
            code.code_workspace.source_document_dialog
                || code.code_workspace.source_workspace_dialog.is_some()
        },
        render: CodeFeature::render_source_document_dialog,
    },
    FeatureDialog {
        key: DialogKey("code.source_find"),
        is_open: |code| code.code_workspace.source_search.is_some(),
        render: CodeFeature::render_source_find_dialog,
    },
    FeatureDialog {
        key: DialogKey("code.rename_symbol"),
        is_open: |code| code.code_workspace.language_tools.is_some(),
        render: CodeFeature::render_rename_symbol_form,
    },
    FeatureDialog {
        key: DialogKey("code.new_veriloga_device"),
        is_open: |code| code.new_veriloga_device.open,
        render: CodeFeature::render_new_veriloga_device_form,
    },
    FeatureDialog {
        key: DialogKey("code.veriloga_import"),
        is_open: |code| code.code_workspace.veriloga.import_form.is_some(),
        render: CodeFeature::render_veriloga_import_dialog,
    },
    FeatureDialog {
        key: DialogKey("code.veriloga_replace"),
        is_open: |code| code.code_workspace.veriloga.replace_review.is_some(),
        render: CodeFeature::render_veriloga_replace_dialog,
    },
    FeatureDialog {
        key: DialogKey("code.veriloga_build_settings"),
        is_open: |code| code.veriloga_build_settings.is_some(),
        render: CodeFeature::render_veriloga_build_settings,
    },
];

/// Why a command the feature cannot run here says no more than that.
const UNAVAILABLE: &str = "command is unavailable in this context";

impl CommandSet for CodeCommand {
    type Feature = CodeFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// The source editor's own commands are offered only while a source
    /// editor is the document on screen, and language navigation only where
    /// the document's language service answers it. Elsewhere they are hidden
    /// rather than offered disabled.
    fn offered(self, reads: &Reads<'_, CodeFeature>) -> bool {
        !self.source_editor_only()
            || reads
                .wb
                .active_source_editor
                .is_some_and(|editor| !self.navigates_source_language() || editor.navigable)
    }

    /// A source editor takes its own commands, and refuses the ones that
    /// would change a read-only document's text. Each window the netlist
    /// page draws is offered only on that page, not merely in its workspace:
    /// from Automation it would be set open with nothing on screen and then
    /// appear unprompted the moment the reader came back. Find and Manage
    /// act on every code document: the netlist page, the Automation page and
    /// a Verilog-A cell view in Design.
    fn availability(self, reads: &Reads<'_, CodeFeature>) -> CommandAvailability {
        let model = reads.model;
        let code = &reads.state.code_workspace;
        let execution_active = model.simulation.execution.has_active_execution();
        let (enabled, reason) = match self {
            Self::NewVerilogADevice => (
                new_veriloga_device_available_in(model),
                "no library can be written",
            ),
            Self::OpenNetlist => (
                !execution_active,
                "an active simulation execution still owns the project",
            ),
            Self::ImportNetlist => (
                reads.platform.project_lifecycle.is_open()
                    && !model.safe_mode.project_read_only()
                    && !execution_active,
                if execution_active {
                    "an active simulation execution still owns the project"
                } else if model.safe_mode.project_read_only() {
                    "the project is open read-only"
                } else {
                    UNAVAILABLE
                },
            ),
            Self::ImportLtspiceSchematic | Self::ImportQspiceSchematic => {
                match foreign_import_refusal(model, reads.platform) {
                    Some(refusal) => (false, refusal),
                    None => (true, UNAVAILABLE),
                }
            }
            Self::ImportVerilogA
            | Self::ExportNetlist(_)
            | Self::ExportBillOfMaterials
            | Self::GenerateNetlist
            | Self::AutomationConsole => (true, UNAVAILABLE),
            // The locator owns every reason the jump cannot be made, so the
            // disabled explanation is the one the execution would have hit.
            Self::ShowInNetlist if !schematic_in_front(reads) => {
                (false, "open a schematic or testbench")
            }
            Self::ShowInNetlist => match model.selected_instance_generated_line() {
                Ok(_) => (true, UNAVAILABLE),
                Err(block) => (false, block),
            },
            Self::FindCodeDocument => (code_document_on_screen(reads), UNAVAILABLE),
            Self::ValidateCodeDocument => (
                netlist_page_on_screen(reads)
                    && reads.state.netlist.active_document != ActiveNetlistDocument::GeneratedDiff
                    && !model.simulation.source.netlist_content.trim().is_empty(),
                UNAVAILABLE,
            ),
            // Unlike its neighbours this one belongs to the source documents,
            // the Automation page and a Verilog-A cell view, not the netlist
            // page: it manages a project source bundle, which the deck is not.
            Self::ManageSourceDocument => (
                code_document_on_screen(reads)
                    && source_document_dialog_is_available(
                        reads.wb.workspace,
                        model,
                        code,
                        &reads.state.netlist,
                        reads.platform,
                    ),
                UNAVAILABLE,
            ),
            Self::GoToDefinition | Self::FindReferences => (
                reads
                    .wb
                    .active_source_editor
                    .is_some_and(|editor| editor.navigable),
                "the source document is read-only",
            ),
            Self::RenameSymbol => (
                reads
                    .wb
                    .active_source_editor
                    .is_some_and(|editor| editor.navigable && editor.editable),
                "the source document is read-only",
            ),
            Self::CompareGeneratedRevisions => (
                netlist_page_on_screen(reads)
                    && !reads.state.netlist.generated_history.is_empty()
                    && model.netlist_artifacts.generated_document.is_some(),
                UNAVAILABLE,
            ),
            // This gate was written for the compile command and never
            // attached, so the button stayed enabled with no bundle, no
            // selected source, an empty root, a compile already running, or a
            // read-only project -- and then the surface refused the request it
            // had just been handed.
            Self::CompileVerilogA => (
                resolve_veriloga_compile(model, code, reads.platform).is_some(),
                veriloga_compile_refusal(model, code, reads.platform),
            ),
            Self::CancelVerilogACompile => (
                code.veriloga.pending.is_some(),
                "no Verilog-A compile is running",
            ),
            // Build settings opens on a read-only project too, to be read;
            // its Save is what that refuses.
            Self::VerilogABuildSettings => (
                veriloga_build_settings_available(model, reads.platform),
                UNAVAILABLE,
            ),
            Self::VerilogAOpenSymbol => (can_open_veriloga_symbol(model), UNAVAILABLE),
            // Every source editor can be stepped through, read-only or not.
            Self::NextProblem | Self::PreviousProblem | Self::ShowProblemAtCaret => (
                reads.wb.active_source_editor.is_some(),
                "the source document is read-only",
            ),
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
            | Self::UnfoldAll => (
                editor_takes(reads.wb, self),
                "the source document is read-only",
            ),
        };
        if enabled {
            CommandAvailability::Available
        } else {
            CommandAvailability::Disabled(reason)
        }
    }

    fn execute(self, cx: &mut Cx<'_, CodeFeature>) {
        match self {
            Self::NewVerilogADevice => {
                open_new_veriloga_device_form_in(cx.model, &mut cx.state.new_veriloga_device);
            }
            Self::ExportNetlist(format) => open_export_dialog(cx, format),
            Self::CompareGeneratedRevisions => {
                let netlist = &mut cx.state.netlist;
                netlist.comparison_dialog.open = true;
                netlist.comparison_dialog.selected_history_index =
                    netlist.generated_history.len().saturating_sub(1);
            }
            Self::AutomationConsole => {
                cx.wb.activate(Workspace::Netlist);
                cx.state.code_workspace.page = CodeWorkspacePage::Automation;
            }
            Self::CancelVerilogACompile => {
                cancel_veriloga_compile(&mut cx.state.code_workspace, cx.wb);
            }
            // The focused editor's page takes the step as it draws.
            Self::NextProblem => cx.wb.outbox.push(ProblemStep::Next),
            Self::PreviousProblem => cx.wb.outbox.push(ProblemStep::Previous),
            Self::ShowProblemAtCaret => cx.wb.outbox.push(ProblemStep::AtCaret),
            // Asked of the source editor on screen, which refuses an edit of
            // a read-only document; with none on screen there is nothing for
            // them to act on.
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
            | Self::UnfoldAll => {
                if editor_takes(cx.wb, self)
                    && let Some(request) = self.source_editor_request()
                {
                    cx.wb.outbox.push(request);
                }
            }
            // Each of these still reaches past the feature's own parts, so
            // the shell carries it out over the whole application as its
            // table did, straight after this call returns.
            Self::OpenNetlist => cx.wb.outbox.push(CodeRequest::OpenNetlist),
            Self::ImportNetlist => cx.wb.outbox.push(CodeRequest::ImportNetlist),
            Self::ImportVerilogA => cx.wb.outbox.push(CodeRequest::ImportVerilogA),
            Self::ImportLtspiceSchematic => {
                cx.wb.outbox.push(CodeRequest::ImportLtspiceSchematic);
            }
            Self::ImportQspiceSchematic => {
                cx.wb.outbox.push(CodeRequest::ImportQspiceSchematic);
            }
            Self::ExportBillOfMaterials => {
                cx.wb.outbox.push(CodeRequest::ExportBillOfMaterials);
            }
            Self::GenerateNetlist => cx.wb.outbox.push(CodeRequest::GenerateNetlist),
            Self::ShowInNetlist => cx.wb.outbox.push(CodeRequest::ShowInNetlist),
            Self::FindCodeDocument => cx.wb.outbox.push(CodeRequest::FindCodeDocument),
            Self::ValidateCodeDocument => cx.wb.outbox.push(CodeRequest::ValidateCodeDocument),
            Self::ManageSourceDocument => cx.wb.outbox.push(CodeRequest::ManageSourceDocument),
            Self::GoToDefinition => cx.wb.outbox.push(CodeRequest::GoToDefinition),
            Self::FindReferences => cx.wb.outbox.push(CodeRequest::FindReferences),
            Self::RenameSymbol => cx.wb.outbox.push(CodeRequest::RenameSymbol),
            Self::CompileVerilogA => cx.wb.outbox.push(CodeRequest::CompileVerilogA),
            Self::VerilogABuildSettings => {
                cx.wb.outbox.push(CodeRequest::VerilogABuildSettings);
            }
            Self::VerilogAOpenSymbol => cx.wb.outbox.push(CodeRequest::VerilogAOpenSymbol),
        }
    }
}

/// Whether the source editor on screen takes `command`: one is there, and it
/// accepts the request, which a read-only editor refuses when the request
/// would change its text.
fn editor_takes(wb: &WorkbenchSession, command: CodeCommand) -> bool {
    match (wb.active_source_editor, command.source_editor_request()) {
        (Some(editor), Some(request)) => editor.editable || !request.mutates_source(),
        _ => false,
    }
}

/// Open the export dialog on `format` where the netlist page draws it;
/// anywhere else the deck is exported in that format straight away.
fn open_export_dialog(cx: &mut Cx<'_, CodeFeature>, format: NetlistFormat) {
    if netlist_page_shown(cx.wb.workspace, &cx.state.code_workspace) {
        let dialog = &mut cx.state.netlist.export_dialog;
        dialog.open = true;
        dialog.format = format;
        dialog.error = None;
    } else {
        cx.wb.outbox.push(CodeRequest::ExportNetlist(format));
    }
}

/// Whether the netlist document, not merely the workspace that hosts it, is
/// the thing on screen.
fn netlist_page_on_screen(reads: &Reads<'_, CodeFeature>) -> bool {
    netlist_page_shown(reads.wb.workspace, &reads.state.code_workspace)
}

/// Whether a code document is the thing on screen: one of the Netlist
/// workspace's pages, or a Verilog-A cell view the Design workspace shows.
fn code_document_on_screen(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.wb.workspace == Workspace::Netlist
        || design_hosts_veriloga(reads.wb.workspace, &reads.model.workspace)
}

/// Whether the Design workspace is showing a schematic or a testbench.
fn schematic_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.wb.workspace == Workspace::Design
        && matches!(
            reads.model.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
}

/// What the feature settles each frame before any surface draws, and how
/// its automation runtime stops.
impl CodeFeature {
    /// Settle, in this order: a project the last frame replaced takes its
    /// automation authority with it, the runtime is discovered, the
    /// Verilog-A import and compile that finished are taken, the compile a
    /// request asked for starts, where the open module's symbol stands is
    /// measured, and the Automation import that finished is taken.
    pub(crate) fn poll(cx: &mut Cx<'_, Self>, ctx: &Context) {
        Self::terminate_after_project_change(cx);
        #[cfg(not(target_arch = "wasm32"))]
        cx.own.automation_runtime.poll_discovery();
        // Both source imports record intent and complete from the frame loop.
        // Neither was driven from anywhere, so a requested import stayed
        // pending forever and blocked every later one.
        poll_veriloga_import(cx.model, &mut cx.state.code_workspace, cx.wb);
        // A Verilog-A compile is published here for the same reason: its
        // result belongs to the bundle, not to whichever editor host happens
        // to be on screen when the worker answers.
        poll_veriloga_compile(cx.model, &mut cx.state.code_workspace, cx.wb);
        let shown = cx.state.netlist_shown();
        consume_veriloga_compile_request(cx.model, &mut cx.state.code_workspace, shown, ctx);
        // Where the open module's symbol stands is measured here, once per
        // library revision, so neither the editor nor the inspector parses it.
        refresh_symbol_standing(
            &mut cx.state.code_workspace.veriloga,
            &cx.model.library_manager,
            &cx.model.workspace.content.active_view,
            cx.model.workspace.content.active_view_type(),
        );
        poll_automation_source_import(
            cx.model,
            &mut cx.state.code_workspace,
            cx.wb,
            cx.platform,
            cx.own,
        );
    }

    /// A project replaced since the last check takes its automation
    /// authority with it: the runtime serving it stops, the Code &
    /// Automation page starts over, and the run prepared for it is dropped.
    pub(crate) fn terminate_after_project_change(cx: &mut Cx<'_, Self>) {
        let project_id = cx.model.workspace.content.project.id();
        if project_id == cx.own.automation_runtime_project_id {
            return;
        }
        let _ = cx.own.automation_runtime.terminate();
        cx.own.automation_runtime_project_id = project_id;
        cx.state.code_workspace = Default::default();
        cx.handles.run_controller.clear_prepared_run();
    }

    /// Stop the automation runtime, as the application shuts down.
    pub(crate) fn terminate_runtime(cx: &mut Cx<'_, Self>) {
        let _ = cx.own.automation_runtime.terminate();
    }
}

/// The feature's dialog steps, each over the parts of the code slot it
/// owns. What a dialog's choice still needs of the whole application it
/// hands the shell, which the step answers as it returns.
impl CodeFeature {
    /// New Verilog-A device….
    fn render_new_veriloga_device_form(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let messages = cx.wb.messages();
        if new_veriloga_device::render_new_veriloga_device_form(
            &mut cx.state.new_veriloga_device,
            cx.model,
            &messages,
            ctx,
        ) {
            cx.wb.outbox.push(CodeRequest::CreateVerilogADevice);
        }
    }

    /// Build settings.
    fn render_veriloga_build_settings(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let messages = cx.wb.messages();
        if let Some(compile) = veriloga_build_settings::render_veriloga_build_settings(
            &mut cx.state.veriloga_build_settings,
            cx.model,
            &messages,
            ctx,
        ) {
            cx.wb
                .outbox
                .push(CodeRequest::SaveVerilogABuildSettings { compile });
        }
    }

    /// Import Verilog-A/AMS….
    fn render_veriloga_import_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let messages = cx.wb.messages();
        if veriloga_import::render_veriloga_import_dialog(
            &mut cx.state.code_workspace.veriloga,
            cx.model,
            &messages,
            ctx,
        ) {
            cx.wb.outbox.push(CodeRequest::ImportVerilogACells);
        }
    }

    /// Replace source from file….
    fn render_veriloga_replace_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let messages = cx.wb.messages();
        if veriloga_import::render_veriloga_replace_dialog(
            &mut cx.state.code_workspace.veriloga.replace_review,
            &messages,
            ctx,
        ) {
            cx.wb.outbox.push(CodeRequest::ReplaceVerilogASource);
        }
    }

    /// The source document dialogs: the Automation workspace's draft, else
    /// the lifecycle of the document on the page in front.
    fn render_source_document_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let code = &mut *cx.state;
        if let Some(request) = source_document::render_source_document_dialog(
            &mut code.code_workspace,
            &mut code.netlist,
            cx.model,
            cx.wb,
            cx.platform,
            ctx,
        ) {
            cx.wb.outbox.push(CodeRequest::SourceDocument(request));
        }
    }

    /// Find in source.
    fn render_source_find_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let messages = cx.wb.messages();
        let code = &mut *cx.state;
        if source_find::render_source_find_dialog(
            &mut code.code_workspace,
            &code.netlist,
            cx.model,
            cx.platform,
            messages,
            ctx,
        ) {
            cx.wb.outbox.push(CodeRequest::ReplaceSourceSearchMatches);
        }
    }

    /// Rename symbol….
    fn render_rename_symbol_form(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if source_language_tools::render_rename_symbol_form(
            &mut cx.state.code_workspace,
            cx.model,
            cx.platform,
            ctx,
        ) {
            cx.wb.outbox.push(CodeRequest::CommitSymbolRename);
        }
    }
}
