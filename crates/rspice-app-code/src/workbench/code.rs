//! The code feature: the Netlist workspace, the Verilog-A views the Design
//! workspace opens, the landing Design shows for a project its deck owns,
//! and the source editors they share.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

pub mod commands;
mod docks;
pub mod netlist_first;
pub mod netlist_validation;
pub mod requests;
mod source_menus;
pub mod toolbar;

use egui::Context;

use crate::state::ViewType;
use crate::state::command_catalog::CommandKey;
use crate::workbench::app::actions::veriloga_symbol::can_open_veriloga_symbol;
use crate::workbench::app::dialogs::library::new_veriloga_device::{
    self, new_veriloga_device_available_in, open_new_veriloga_device_form_in,
};
use crate::workbench::app::dialogs::library::veriloga_import;
use crate::workbench::app::dialogs::{
    source_document, source_language_tools, veriloga_build_settings,
};
use crate::workbench::app::{
    source_document_dialog_is_available, veriloga_build_settings_available,
};
use crate::workbench::chrome::title_bar::menu_entries::MenuEntry::Separator;
use crate::workbench::chrome::title_bar::menu_entries::{MenuContext, MenuEntry};
use crate::workbench::commands::code_context::{
    resolve_veriloga_compile, veriloga_compile_refusal,
};
use crate::workbench::documents::code_workspace::{
    CodeWorkspaceParts, ProblemStep, cancel_veriloga_compile, consume_veriloga_compile_request,
    design_hosts_veriloga, poll_veriloga_compile, poll_veriloga_import, refresh_symbol_standing,
};
use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, cross_probe};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureNavigator, FeatureRegistry,
    FeatureSurface, Reads, RoutedCommand,
};
use crate::workbench::slots::CodeState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use commands::{CodeCommand, CodeRequest};
use rspice_app_services::workbench::slots::platform::foreign_import_refusal_in;
use rspice_app_state::workbench::preferences::TogglePreference;

/// The code feature.
pub struct CodeFeature;

pub use source_menus::{
    SOURCE_EDIT_MENU, SOURCE_EDITOR_MENU, SOURCE_FOLDING_MENU, SOURCE_VIEW_ROWS,
};

impl Feature for CodeFeature {
    type State = CodeState;
    type Session = WorkbenchSession;
    type Handles = crate::workbench::documents::code_workspace::CodeHandles;
    /// The deck page's toolbar offers Run or Stop while the dispatcher would
    /// run or stop, stating Stop's refusal in its words.
    const FOREIGN_AVAILABILITY: &'static [CommandKey] =
        &[ids::RUN_SIMULATION, ids::STOP_SIMULATION];
    /// The source editor's right-click menu offers these, each greyed while
    /// the dispatcher would not run it.
    const MENU_AVAILABILITY: &'static [CommandKey] = &[
        CodeCommand::GoToDefinition.key(),
        CodeCommand::FindReferences.key(),
        CodeCommand::RenameSymbol.key(),
        CodeCommand::FormatDocument.key(),
        CodeCommand::ShowInSchematic.key(),
        CodeCommand::PlotNodeVoltage.key(),
        CodeCommand::PlotDeviceCurrent.key(),
        ids::UNDO,
        ids::REDO,
        ids::CUT,
        ids::COPY,
        ids::PASTE,
        ids::SELECT_ALL,
        CodeCommand::ToggleLineComment.key(),
    ];

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<CodeCommand>();
        registry.toolbars(toolbar::TOOLBARS);
        registry.dialogs(DIALOGS);
        registry.navigator(FeatureNavigator {
            show: docks::show_navigator,
        });
        registry.inspector(FeatureInspector {
            show: docks::show_inspector,
            // Nothing the section draws holds an edit for the shell to commit.
            commit_pending_edit: |_, _| {},
        });
        // The landing a deck-owned project shows in Design while it has no
        // schematic: the shell draws it in the document's place.
        registry.surface(FeatureSurface {
            show: netlist_first::show,
        });
        registry.routed(ROUTED);
    }
}

/// The workbench commands the code feature takes a part in: Find on the
/// Netlist workspace opens the deck's own find bar, and View > Zoom over a
/// source editor sizes the editors' text, as the editor's own Ctrl+= and
/// Ctrl+- do. The shell asks it before the design feature, which would take
/// both there too.
const ROUTED: &[RoutedCommand<CodeFeature>] = &[
    RoutedCommand {
        key: ids::FIND_IN_DESIGN,
        takes: |reads| reads.wb.workspace == Workspace::Netlist,
        enabled: |_| true,
        execute: |cx| cx.state.netlist.find.open = true,
    },
    RoutedCommand {
        key: ids::ZOOM_IN,
        takes: source_editor_in_front,
        enabled: |reads| reads.model.preferences.editor_font_size_can_step(1),
        execute: |cx| cx.model.preferences.step_editor_font_size(1),
    },
    RoutedCommand {
        key: ids::ZOOM_OUT,
        takes: source_editor_in_front,
        enabled: |reads| reads.model.preferences.editor_font_size_can_step(-1),
        execute: |cx| cx.model.preferences.step_editor_font_size(-1),
    },
    RoutedCommand {
        key: ids::ZOOM_ONE_TO_ONE,
        takes: source_editor_in_front,
        enabled: |reads| !reads.model.preferences.editor_font_size_is_actual(),
        execute: |cx| cx.model.preferences.reset_editor_font_size(),
    },
];

/// Whether a source editor is in front, so a view command is its text's.
fn source_editor_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.wb.active_source_editor.is_some()
}

/// The dialog steps the feature draws, under the keys the shell's dialog
/// order lists them by: the source document dialogs, Rename symbol, New Verilog-A device, Import Verilog/Verilog-A/AMS, Replace
/// source from file and Build settings. Each is open while its form is.
const DIALOGS: &[FeatureDialog<CodeFeature>] = &[
    FeatureDialog {
        key: DialogKey("code.source_document"),
        is_open: |code| {
            code.code_workspace.source_document_dialog
                || code.code_workspace.source_file_dialog.is_some()
        },
        render: CodeFeature::render_source_document_dialog,
    },
    FeatureDialog {
        key: DialogKey("code.rename_symbol"),
        is_open: |code| code.code_workspace.language_tools.is_some() || code.deck_rename.is_some(),
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
    /// rather than offered disabled. A deck answers all three through the
    /// engine's index of it and its includes.
    fn offered(self, reads: &Reads<'_, CodeFeature>) -> bool {
        if !self.source_editor_only() {
            return true;
        }
        // Only a line of the generated netlist was written from a schematic.
        if self == Self::ShowInSchematic {
            return reads.wb.active_source_editor.is_some()
                && netlist_page_on_screen(reads)
                && cross_probe::generated_netlist_in_front(&reads.state.netlist);
        }
        // A plot is of the name under the caret, so each is offered only on
        // a name of its kind, in a document a run can be of.
        if let Some(quantity) = plot_quantity(self) {
            let netlist = &reads.state.netlist;
            return reads.wb.active_source_editor.is_some()
                && netlist_page_on_screen(reads)
                && cross_probe::plottable_document_in_front(netlist)
                && cross_probe::caret_name(reads.model, netlist)
                    .and_then(|name| quantity.of(name))
                    .is_some();
        }
        reads
            .wb
            .active_source_editor
            .is_some_and(|editor| !self.navigates_source_language() || editor.navigable)
    }

    fn checked(self, reads: &Reads<'_, CodeFeature>) -> Option<bool> {
        (self == Self::ToggleWordWrap).then(|| {
            reads
                .model
                .preferences
                .toggle(TogglePreference::EditorWordWrap)
        })
    }

    /// A source editor takes its own commands, and refuses the ones that
    /// would change a read-only document's text. Each window the deck draws
    /// is offered only on the Netlist workspace. Find and Manage act on every
    /// code document: the deck and a Verilog-A cell view in Design.
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
            Self::NewDeck => (
                !execution_active,
                "an active simulation execution still owns the project",
            ),
            Self::SaveDeckAs => (
                cfg!(not(target_arch = "wasm32")) && reads.state.netlist.open_deck.is_some(),
                if cfg!(target_arch = "wasm32") {
                    "decks are saved as files on the desktop"
                } else {
                    "no deck is open"
                },
            ),
            Self::SaveNetlistAsDeck => (
                !execution_active && !model.netlist_artifacts.generated_source.trim().is_empty(),
                if execution_active {
                    "an active simulation execution still owns the project"
                } else {
                    "the netlist has not been generated"
                },
            ),
            Self::ExportDeck | Self::ExportDeckFiles => (
                reads
                    .state
                    .netlist
                    .open_deck
                    .as_ref()
                    .is_some_and(|deck| deck.is_stored()),
                "no deck kept in the project is open",
            ),
            Self::ImportLtspiceSchematic | Self::ImportQspiceSchematic => {
                match foreign_import_refusal_in(model, reads.platform) {
                    Some(refusal) => (false, refusal),
                    None => (true, UNAVAILABLE),
                }
            }
            Self::ImportVerilogA
            | Self::ExportNetlist
            | Self::ExportBillOfMaterials
            | Self::GenerateNetlist => (true, UNAVAILABLE),
            // The locator owns every reason the jump cannot be made, so the
            // disabled explanation is the one the execution would have hit.
            Self::ShowInNetlist if !schematic_in_front(reads) => {
                (false, "open a schematic or testbench")
            }
            Self::ShowInNetlist => match model.show_in_netlist_block() {
                None => (true, UNAVAILABLE),
                Some(block) => (false, block),
            },
            Self::ShowInSchematic => {
                let netlist = &reads.state.netlist;
                let name = cross_probe::caret_name(model, netlist);
                match cross_probe::schematic_target(model, netlist, name) {
                    Ok(_) => (true, UNAVAILABLE),
                    Err(reason) => (false, reason),
                }
            }
            Self::PlotNodeVoltage | Self::PlotDeviceCurrent => {
                let netlist = &reads.state.netlist;
                let target = plot_quantity(self)
                    .zip(cross_probe::caret_name(model, netlist))
                    .ok_or(cross_probe::NOT_A_NAME)
                    .and_then(|(quantity, name)| {
                        cross_probe::plot_target(model, netlist, reads.runs, quantity, name)
                    });
                match target {
                    Ok(_) => (true, UNAVAILABLE),
                    Err(reason) => (false, reason),
                }
            }
            Self::ValidateCodeDocument => (
                netlist_page_on_screen(reads)
                    && reads.state.netlist.active_document != ActiveNetlistDocument::GeneratedDiff
                    && !model.simulation.source.netlist_content.trim().is_empty(),
                UNAVAILABLE,
            ),
            // Unlike its neighbours this one belongs to the source documents,
            // a Verilog-A cell view, not the deck: it manages a project source
            // bundle, which the deck is not.
            Self::ManageSourceDocument => (
                code_document_on_screen(reads)
                    && source_document_dialog_is_available(
                        reads.wb.workspace,
                        model,
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
                resolve_veriloga_compile(model, code, reads.platform),
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
            // Wrap is every source editor's, read-only or not.
            Self::ToggleWordWrap => (true, UNAVAILABLE),
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
            | Self::FindCodeDocument
            | Self::ReplaceInCodeDocument
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
            Self::ExportNetlist => open_export_dialog(cx),
            Self::CompareGeneratedRevisions => {
                let netlist = &mut cx.state.netlist;
                netlist.comparison_dialog.open = true;
                netlist.comparison_dialog.selected_history_index =
                    netlist.generated_history.len().saturating_sub(1);
            }
            Self::CancelVerilogACompile => {
                cancel_veriloga_compile(&mut cx.state.code_workspace, cx.wb);
            }
            // The focused editor's page takes the step as it draws.
            Self::NextProblem => cx.wb.outbox.push(ProblemStep::Next),
            Self::ToggleWordWrap => {
                let wraps = cx
                    .model
                    .preferences
                    .toggle(TogglePreference::EditorWordWrap);
                cx.model
                    .preferences
                    .set_toggle(TogglePreference::EditorWordWrap, !wraps);
            }
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
            | Self::FindCodeDocument
            | Self::ReplaceInCodeDocument
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
            Self::NewDeck => cx.wb.outbox.push(CodeRequest::NewDeck),
            Self::SaveDeckAs => cx.wb.outbox.push(CodeRequest::SaveDeckAs),
            Self::SaveNetlistAsDeck => cx.wb.outbox.push(CodeRequest::SaveNetlistAsDeck),
            Self::ExportDeck => cx.wb.outbox.push(CodeRequest::ExportDeck),
            Self::ExportDeckFiles => cx.wb.outbox.push(CodeRequest::ExportDeckFiles),
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
            Self::ShowInSchematic => show_in_schematic(cx),
            Self::PlotNodeVoltage | Self::PlotDeviceCurrent => {
                if let Some(quantity) = plot_quantity(self) {
                    plot_from_source(cx, quantity);
                }
            }
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

/// Open the schematic the generated line under the caret was written from,
/// at the net under the caret when the line connects it, else at the
/// instance the line is the card of. The name is read from the text as it
/// stands, not from the frame before.
pub(crate) fn show_in_schematic(cx: &mut Cx<'_, CodeFeature>) {
    if !cross_probe::generated_netlist_in_front(&cx.state.netlist) {
        return;
    }
    let name = cross_probe::caret_name_now(cx.model, &mut cx.state.netlist);
    match cross_probe::schematic_target(cx.model, &cx.state.netlist, name.as_ref()) {
        Ok(cross_probe::SchematicTarget::Instance(usage)) => {
            cx.wb.outbox.push(CodeRequest::OpenCellInstance(usage));
        }
        Ok(cross_probe::SchematicTarget::Net { schematic, net }) => {
            cx.wb
                .outbox
                .push(CodeRequest::ShowNetInSchematic { schematic, net });
        }
        Err(reason) => cx
            .wb
            .prompt(format!("Cannot show the line in the schematic: {reason}.")),
    }
}

/// The quantity a plot command asks the run for.
const fn plot_quantity(command: CodeCommand) -> Option<cross_probe::PlotQuantity> {
    match command {
        CodeCommand::PlotNodeVoltage => Some(cross_probe::PlotQuantity::Voltage),
        CodeCommand::PlotDeviceCurrent => Some(cross_probe::PlotQuantity::Current),
        _ => None,
    }
}

/// What the source editor menu's row of `key` reads where it names what the
/// caret is on: `Plot V(out)` for the node `out`, `Plot I(R1)` for `R1`.
/// The page's menu and the host's read it alike.
pub fn source_menu_label(
    key: CommandKey,
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
) -> Option<String> {
    let quantity = plot_quantity(CodeCommand::from_key(key)?)?;
    let (name, _) = quantity.of(cross_probe::caret_name(model, netlist)?)?;
    Some(format!("Plot {}", quantity.vector(name)))
}

/// Put the quantity of the name under the caret on the waveform viewer's
/// plot, from the run of the document in front. The name is read from the
/// text as it stands.
fn plot_from_source(cx: &mut Cx<'_, CodeFeature>, quantity: cross_probe::PlotQuantity) {
    use crate::workbench::framework::requests::ShowWaveforms;
    use rspice_results::result_presentation::{
        AnalysisPresentationKey, SourceWaveformPresentationKey,
    };
    let target = cross_probe::caret_name_now(cx.model, &mut cx.state.netlist)
        .ok_or(cross_probe::NOT_A_NAME)
        .and_then(|name| {
            cross_probe::plot_target(cx.model, &cx.state.netlist, cx.runs, quantity, &name)
        });
    let target = match target {
        Ok(target) => target,
        Err(reason) => {
            cx.wb.prompt(format!("Cannot plot: {reason}."));
            return;
        }
    };
    if !cx.model.simulation.select_run_by_sequence(target.run)
        || !cx.model.simulation.select_analysis(target.analysis)
    {
        return;
    }
    let Some(run) = cx.model.simulation.active_run() else {
        return;
    };
    let Some((analysis, waveform)) = run.analyses.get(target.analysis).and_then(|analysis| {
        analysis
            .waveforms
            .get(target.waveform)
            .map(|waveform| (analysis, waveform))
    }) else {
        return;
    };
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, analysis),
        waveform.name.clone(),
    );
    let shown = waveform.visible;
    // Raised before the viewer's command and taken after it, as every
    // hand-off into the waveform viewer is.
    cx.wb.outbox.push(ShowWaveforms {
        visibility: vec![(key, shown)],
        selected: None,
    });
    cx.execute(ids::RESULT_WAVEFORMS);
}

/// Open the export dialog where the deck draws it; anywhere else the deck is
/// exported straight away.
fn open_export_dialog(cx: &mut Cx<'_, CodeFeature>) {
    if cx.wb.workspace == Workspace::Netlist {
        let dialog = &mut cx.state.netlist.export_dialog;
        dialog.open = true;
        dialog.error = None;
    } else {
        cx.wb.outbox.push(CodeRequest::ExportNetlist);
    }
}

/// Whether the deck is the thing on screen.
fn netlist_page_on_screen(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.wb.workspace == Workspace::Netlist
}

/// Whether a code document is the thing on screen: the Netlist workspace's
/// deck, or a Verilog-A cell view the Design workspace shows.
fn code_document_on_screen(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.wb.workspace == Workspace::Netlist
        || design_hosts_veriloga(reads.wb.workspace, &reads.model.workspace)
}

/// Whether the Design workspace is showing a schematic or a testbench.
fn schematic_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.wb.workspace == Workspace::Design
        && matches!(
            reads.model.workspace.content.active_view_type(),
            Some(ViewType::Schematic | ViewType::Testbench)
        )
}

/// What the feature settles each frame before any surface draws.
impl CodeFeature {
    /// Settle, in this order: a project the last frame replaced starts the
    /// code workspace over, the Verilog-A import and compile that finished
    /// are taken, the compile a request asked for starts, and where the open
    /// module's symbol stands is measured.
    pub fn poll(cx: &mut Cx<'_, Self>, ctx: &Context) {
        Self::reset_after_project_change(cx);
        // A source import records intent and completes from the frame loop.
        // It was driven from nowhere once, so a requested import stayed
        // pending forever and blocked every later one.
        poll_veriloga_import(cx.model, &mut cx.state.code_workspace, cx.wb);
        // A Verilog-A compile is published here for the same reason: its
        // result belongs to the bundle, not to whichever editor host happens
        // to be on screen when the worker answers.
        poll_veriloga_compile(cx.model, &mut cx.state.code_workspace, cx.wb);
        let deck = crate::workbench::slots::code::manual_deck_in(&cx.state.netlist);
        consume_veriloga_compile_request(cx.model, &mut cx.state.code_workspace, deck, ctx);
        // Where the open module's symbol stands is measured here, once per
        // library revision, so neither the editor nor the inspector parses it.
        refresh_symbol_standing(
            &mut cx.state.code_workspace.veriloga,
            &cx.model.library_manager,
            &cx.model.workspace.content.active_view,
            cx.model.workspace.content.active_view_type(),
        );
    }

    /// A project replaced since the last check starts the code workspace
    /// over, and the run prepared for the old project is dropped.
    pub fn reset_after_project_change(cx: &mut Cx<'_, Self>) {
        let project_id = cx.model.workspace.content.project.id();
        if project_id == cx.own.project_id {
            return;
        }
        cx.own.project_id = project_id;
        cx.state.code_workspace = Default::default();
        cx.handles.run_controller.clear_prepared_run();
    }
}

/// What the code workspace's operations take, lent from the feature's
/// context.
impl CodeFeature {
    /// The code workspace's parts, lent from `cx` for one call.
    pub(in crate::workbench) fn workspace_parts<'c>(
        cx: &'c mut Cx<'_, Self>,
    ) -> CodeWorkspaceParts<'c> {
        CodeWorkspaceParts {
            code: &mut cx.state.code_workspace,
            netlist: &mut cx.state.netlist,
            model: &mut *cx.model,
            wb: &mut *cx.wb,
            platform: &*cx.platform,
            runs: &mut *cx.runs,
            run_controller: &mut *cx.handles.run_controller,
            export_io: cx.handles.export_io,
        }
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

    /// Import Verilog/Verilog-A/AMS….
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

    /// The source document dialogs: the open file transaction's form, else
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

    /// Rename symbol…: the deck's, while one is open, else a source
    /// bundle's.
    fn render_rename_symbol_form(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if cx.state.deck_rename.is_some() {
            source_language_tools::render_deck_rename_form(cx, ctx);
            return;
        }
        if source_language_tools::render_rename_symbol_form(
            &mut cx.state.code_workspace,
            cx.model,
            cx.platform,
            ctx,
            cx.wb.messages(),
        ) {
            cx.wb.outbox.push(CodeRequest::CommitSymbolRename);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests;
