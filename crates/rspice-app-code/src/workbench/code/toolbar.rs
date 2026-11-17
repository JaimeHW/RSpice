//! The code toolbars: the Verilog-A cell view's build and source tools, and
//! the Code & Automation workspace's netlist and automation pages.
//!
//! They are the code feature's toolbars, drawn in its context. Each group
//! asks what the code workspace shows at the point it draws, after any tool
//! to its left has run.

use crate::state::command_catalog::CommandKey;
use crate::workbench::chrome::toolbar::{explicit_label_width, toolbar_icon_button_size};
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{
    Cx, Reads, ToolbarAction, ToolbarControl, ToolbarEntry, ToolbarIcon, ToolbarId, ToolbarTool,
};
use crate::workbench::layout::LayoutSpec;

/// The toolbar of an open Verilog-A cell view.
pub const VERILOGA_TOOLBAR: ToolbarId = ToolbarId::new("code.veriloga");
/// The toolbar of the Code & Automation workspace.
pub const NETLIST_TOOLBAR: ToolbarId = ToolbarId::new("code.netlist");

/// Every toolbar the code feature registers.
pub const TOOLBARS: &[(ToolbarId, &[ToolbarEntry<CodeFeature>])] =
    &[(VERILOGA_TOOLBAR, VERILOGA), (NETLIST_TOOLBAR, NETLIST)];

/// A command drawn as a labelled button.
const fn labelled(
    key: CommandKey,
    icon: WorkbenchIcon,
    label: &'static str,
) -> ToolbarEntry<CodeFeature> {
    ToolbarEntry::Tool(ToolbarTool { key, icon, label })
}

/// A command drawn as an icon, named by its own label and pressed as its
/// own check mark says.
const fn icon(key: CommandKey, icon: WorkbenchIcon) -> ToolbarEntry<CodeFeature> {
    ToolbarEntry::Icon(ToolbarIcon {
        key,
        icon,
        label: None,
        pressed: None,
    })
}

/// Toolbar of an open Verilog-A cell view.
///
/// Compile and the cross-links to the generated deck and the cell's symbol,
/// then the build settings Compile reads, then the source tools, Format first.
/// The toolbar is the only
/// place the first two actions live, so they come first: on a phone, Compile
/// is the one tool the row keeps, and the deck heads More tools. Build settings, Format
/// and every source tool are in the command palette as well, and Format in the
/// editor's own menu. A schematic's placement and transform tools have nothing
/// to act on here.
const VERILOGA: &[ToolbarEntry<CodeFeature>] = &[
    // The control is Cancel until the compile finishes, as Run is Stop.
    // Compile is asked first: Cancel ends the compile as it is pressed, and
    // asked first it would leave Compile to draw beside it for that frame.
    ToolbarEntry::Only(
        no_compile_pending,
        &[labelled(
            CodeCommand::CompileVerilogA.key(),
            WorkbenchIcon::Run,
            "Compile Verilog-A/AMS",
        )],
    ),
    ToolbarEntry::Only(
        compile_pending,
        &[labelled(
            CodeCommand::CancelVerilogACompile.key(),
            WorkbenchIcon::Stop,
            "Cancel Verilog-A/AMS compile",
        )],
    ),
    // The generated deck, in the Netlist workspace: what the design this
    // module is placed in netlists to. The overflow row and the button take
    // the reader to the same place.
    ToolbarEntry::Action(ToolbarAction {
        show: generated_netlist_tool,
        icon: WorkbenchIcon::Netlist,
        label: crate::workbench::MessageId::CodeGeneratedNetlist,
        run: show_generated_deck,
    }),
    labelled(
        CodeCommand::VerilogAOpenSymbol.key(),
        WorkbenchIcon::Instance,
        "Open symbol",
    ),
    icon(
        CodeCommand::VerilogABuildSettings.key(),
        WorkbenchIcon::Sliders,
    ),
    ToolbarEntry::Separator,
    ToolbarEntry::Control(ToolbarControl {
        show: veriloga_format_tool,
        overflow: None,
    }),
    icon(CodeCommand::FindCodeDocument.key(), WorkbenchIcon::Search),
    icon(
        CodeCommand::ManageSourceDocument.key(),
        WorkbenchIcon::Folder,
    ),
];

fn compile_pending(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.state.code_workspace.veriloga.pending.is_some()
}

fn no_compile_pending(reads: &Reads<'_, CodeFeature>) -> bool {
    !compile_pending(reads)
}

/// Take the reader to the generated deck: its page in the Netlist workspace,
/// with the generated primary in front.
fn show_generated_deck(cx: &mut Cx<'_, CodeFeature>) {
    crate::workbench::surfaces::netlist::generation::show_generated_netlist(cx);
}

/// The generated deck's button, in the slot the toolbar took for it.
fn generated_netlist_tool(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    let label = cx
        .wb
        .messages()
        .text(crate::workbench::MessageId::CodeGeneratedNetlist);
    let response = if layout.toolbar_labels {
        labeled_icon_button_sized(
            ui,
            WorkbenchIcon::Netlist,
            &label,
            false,
            explicit_label_width(ui, &label, false),
            layout.toolbar_control_height,
        )
    } else {
        icon_button(
            ui,
            WorkbenchIcon::Netlist,
            &label,
            false,
            toolbar_icon_button_size(layout),
        )
    };
    if response.clicked() {
        show_generated_deck(cx);
    }
}

/// Format the source the editor is showing.
///
/// Formatting is the editor's own transaction — it is parse-gated and lands
/// as a reviewable document action — so the tool hands the request to the
/// visible editor rather than rewriting text itself. Offered only when that
/// file may be edited.
fn veriloga_format_tool(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    use crate::workbench::documents::code_workspace;

    let language = crate::state::ProjectSourceLanguage::VerilogA;
    let code = &cx.state.code_workspace;
    let editor = code_workspace::selected_veriloga_editor_snapshot_in(cx.model, code)
        .ok()
        .filter(|snapshot| {
            code_workspace::source_bundle_document_is_editable_in(
                cx.model,
                code,
                cx.platform,
                language,
                snapshot.bundle_id(),
                snapshot.active_path(),
            )
        })
        .map(|snapshot| {
            code_workspace::source_editor_id(language, snapshot.bundle_id(), snapshot.active_path())
        });
    let response = ui
        .add_enabled_ui(editor.is_some(), |ui| {
            icon_button(
                ui,
                WorkbenchIcon::Format,
                "Format document",
                false,
                toolbar_icon_button_size(layout),
            )
        })
        .inner;
    if response.clicked()
        && let Some(editor) = editor
    {
        crate::workbench::documents::text_editor_commands::queue_format_document(ui.ctx(), editor);
    }
}

/// The Code & Automation workspace hosts two different documents, so its
/// toolbar is chosen by the visible page.
///
/// Find is deliberately absent: the editor's own toolbar sits directly under
/// this one and owns it. Offering it twice was the mockup's own earlier
/// mistake, and here it was worse than redundant — the find window is drawn
/// only by the netlist page, so the button silently did nothing on the other
/// two and then ambushed the user on return.
///
/// The netlist page's set follows the document it is showing: a generated
/// primary is exported and compared, an owned deck is saved and run. The
/// Automation page's source tools manage the document and search the
/// closure.
const NETLIST: &[ToolbarEntry<CodeFeature>] = &[
    ToolbarEntry::Only(
        generated_deck_shown,
        &[
            labelled(
                CodeCommand::ExportNetlist.key(),
                WorkbenchIcon::Export,
                "Export generated deck",
            ),
            labelled(
                CodeCommand::ValidateCodeDocument.key(),
                WorkbenchIcon::Check,
                "Validate generated netlist",
            ),
            ToolbarEntry::Separator,
            labelled(
                CodeCommand::CompareGeneratedRevisions.key(),
                WorkbenchIcon::Compare,
                "Compare generated revisions",
            ),
        ],
    ),
    ToolbarEntry::Only(
        owned_deck_shown,
        &[
            labelled(ids::SAVE, WorkbenchIcon::Save, "Save deck"),
            labelled(
                CodeCommand::ValidateCodeDocument.key(),
                WorkbenchIcon::Check,
                "Validate source",
            ),
            ToolbarEntry::Separator,
            labelled(
                CodeCommand::ExportNetlist.key(),
                WorkbenchIcon::Export,
                "Export deck copy",
            ),
        ],
    ),
    ToolbarEntry::Only(
        automation_page_shown,
        &[
            labelled(
                CodeCommand::AutomationConsole.key(),
                WorkbenchIcon::Terminal,
                "Automation console",
            ),
            ToolbarEntry::Separator,
            icon(
                CodeCommand::ManageSourceDocument.key(),
                WorkbenchIcon::Folder,
            ),
            icon(CodeCommand::FindCodeDocument.key(), WorkbenchIcon::Search),
        ],
    ),
    icon(ids::RUN_SIMULATION, WorkbenchIcon::Run),
];

fn generated_deck_shown(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.state.code_workspace.page == CodeWorkspacePage::Netlist
        && reads.state.netlist.active_document == ActiveNetlistDocument::Generated
}

fn owned_deck_shown(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.state.code_workspace.page == CodeWorkspacePage::Netlist
        && reads.state.netlist.active_document != ActiveNetlistDocument::Generated
}

fn automation_page_shown(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.state.code_workspace.page == CodeWorkspacePage::Automation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::framework::feature::FeatureRegistry;

    /// The code feature registers each toolbar once, and both toolbars the
    /// shell's lane chooses (an open Verilog-A cell view's and the Code &
    /// Automation workspace's) are ones it registered.
    #[test]
    fn every_code_toolbar_the_shell_chooses_is_registered_once() {
        assert_eq!(
            FeatureRegistry::<CodeFeature>::build()
                .validate_toolbars([VERILOGA_TOOLBAR, NETLIST_TOOLBAR]),
            Ok(())
        );
    }
}
