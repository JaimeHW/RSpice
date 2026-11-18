//! The code toolbars: the Verilog-A cell view's build and source tools, and
//! the Netlist workspace's.
//!
//! They are the code feature's toolbars, drawn in its context. Each group
//! asks what the code workspace shows at the point it draws, after any tool
//! to its left has run.

use crate::state::command_catalog::CommandKey;
use crate::workbench::chrome::toolbar::{explicit_label_width, toolbar_icon_button_size};
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use crate::workbench::documents::netlist_document::NetlistTab;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{
    Cx, Reads, ToolbarAction, ToolbarControl, ToolbarEntry, ToolbarIcon, ToolbarId, ToolbarTool,
};
use crate::workbench::layout::LayoutSpec;

/// The toolbar of an open Verilog-A cell view.
pub const VERILOGA_TOOLBAR: ToolbarId = ToolbarId::new("code.veriloga");
/// The toolbar of the Netlist workspace.
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

/// Take the reader to the generated deck: the Netlist workspace, with the
/// generated primary in front.
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

/// The Netlist workspace's toolbar follows the tab in front, with only the
/// verbs that tab has: the generated netlist is saved as a deck, exported
/// and compared with an earlier build; a deck is saved, saved elsewhere
/// when it is a file, exported and compared; an include of a deck is saved.
/// The Added lines are saved with the project, by the Save at the toolbar's
/// head, and a run's input and a comparison are only read.
///
/// Find is deliberately absent: the page's own toolbar sits directly under
/// this one and owns it.
const NETLIST: &[ToolbarEntry<CodeFeature>] = &[
    ToolbarEntry::Only(
        generated_netlist_in_front,
        &[
            labelled(
                CodeCommand::SaveNetlistAsDeck.key(),
                WorkbenchIcon::Save,
                "Save as deck",
            ),
            labelled(
                CodeCommand::ExportNetlist.key(),
                WorkbenchIcon::Export,
                "Export netlist",
            ),
            labelled(
                CodeCommand::CompareGeneratedRevisions.key(),
                WorkbenchIcon::Compare,
                "Compare with earlier build",
            ),
        ],
    ),
    ToolbarEntry::Only(
        deck_in_front,
        &[
            labelled(ids::SAVE, WorkbenchIcon::Save, "Save deck"),
            ToolbarEntry::Only(
                deck_file_in_front,
                &[labelled(
                    CodeCommand::SaveDeckAs.key(),
                    WorkbenchIcon::Save,
                    "Save deck as",
                )],
            ),
            labelled(
                CodeCommand::ExportNetlist.key(),
                WorkbenchIcon::Export,
                "Export deck",
            ),
            ToolbarEntry::Control(ToolbarControl {
                show: compare_deck_menu,
                overflow: None,
            }),
        ],
    ),
    ToolbarEntry::Only(
        include_of_deck_in_front,
        &[labelled(ids::SAVE, WorkbenchIcon::Save, "Save include")],
    ),
];

fn generated_netlist_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    reads.state.netlist.front.shows_generated()
}

fn deck_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    matches!(reads.state.netlist.front, NetlistTab::Deck(_))
}

/// A deck that is a file is in front: Save deck as writes it elsewhere. A
/// deck the project keeps as text, as every deck in the browser is, has no
/// file to write elsewhere.
fn deck_file_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    cfg!(not(target_arch = "wasm32"))
        && reads
            .state
            .netlist
            .open_deck
            .as_ref()
            .is_some_and(|deck| !deck.is_stored())
}

fn include_of_deck_in_front(reads: &Reads<'_, CodeFeature>) -> bool {
    matches!(
        reads.state.netlist.front,
        NetlistTab::Include { deck: Some(_), .. }
    )
}

/// Compare the deck in front: a menu of the texts it can be compared with,
/// its file on disk, the last run's input and the generated netlist, each
/// disabled with the reason when there is nothing to compare it with. The
/// comparison opens on a tab of its own.
fn compare_deck_menu(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    use crate::workbench::MessageId;
    use crate::workbench::documents::netlist_document::{self, CompareSide};

    let Some(deck) = cx.state.netlist.open_deck.as_ref() else {
        return;
    };
    let id = deck.id;
    let messages = cx.wb.messages();
    // A deck the project keeps as text has no file; nor has any deck in
    // the browser, which keeps them all.
    let on_disk = !deck.is_stored() && cfg!(not(target_arch = "wasm32"));
    let last_run = netlist_document::run_deck_snapshot_run_id_in(cx.runs);
    let generated = cx.model.netlist_artifacts.generated_document.is_some();
    let label = messages.text(MessageId::NetlistCompareMenu);
    let response = if layout.toolbar_labels {
        // The label, then a chevron saying the button opens a menu.
        let response = labeled_icon_button_sized(
            ui,
            WorkbenchIcon::Compare,
            &label,
            false,
            explicit_label_width(ui, &label, false) + COMPARE_CHEVRON_WIDTH,
            layout.toolbar_control_height,
        );
        let t = crate::ui::tokens::Tokens::get(ui.ctx());
        WorkbenchIcon::ChevronDown.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    response.rect.right() - COMPARE_CHEVRON_WIDTH / 2.0 - 4.0,
                    response.rect.center().y,
                ),
                egui::Vec2::splat(12.0),
            ),
            t.color.text_dim,
        );
        response
    } else {
        icon_button(
            ui,
            WorkbenchIcon::Compare,
            &label,
            false,
            toolbar_icon_button_size(layout),
        )
    };
    let mut chosen = None;
    egui::Popup::menu(&response).show(|ui| {
        let mut row = |ui: &mut egui::Ui, text: String, why: MessageId, sides| {
            if ui
                .add_enabled(Option::is_some(&sides), egui::Button::new(text))
                .on_disabled_hover_text(messages.text(why))
                .clicked()
            {
                chosen = sides;
                ui.close();
            }
        };
        row(
            ui,
            messages.text(MessageId::NetlistCompareWithDisk),
            MessageId::NetlistCompareKeptInProject,
            on_disk.then_some((CompareSide::DiskFile(id), CompareSide::Deck(id))),
        );
        row(
            ui,
            last_run.map_or_else(
                || messages.text(MessageId::NetlistCompareWithLastRun),
                |run| {
                    messages.format(
                        MessageId::NetlistCompareWithRun,
                        &[("run", &run.to_string())],
                    )
                },
            ),
            MessageId::NetlistCompareNoRun,
            last_run.map(|run| (CompareSide::RunInput(run), CompareSide::Deck(id))),
        );
        row(
            ui,
            messages.text(MessageId::NetlistCompareWithGenerated),
            MessageId::NetlistCompareNoGenerated,
            generated.then_some((CompareSide::Deck(id), CompareSide::Generated)),
        );
    });
    if let Some((left, right)) = chosen
        && let Err(error) =
            netlist_document::compare(cx.model, &mut cx.state.netlist, cx.runs, left, right)
    {
        cx.wb.prompt(error);
    }
}

/// Room a menu's chevron takes after the Compare button's label.
const COMPARE_CHEVRON_WIDTH: f32 = 14.0;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::framework::feature::FeatureRegistry;

    /// The code feature registers each toolbar once, and both toolbars the
    /// shell's lane chooses (an open Verilog-A cell view's and the Netlist
    /// workspace's) are ones it registered.
    #[test]
    fn every_code_toolbar_the_shell_chooses_is_registered_once() {
        assert_eq!(
            FeatureRegistry::<CodeFeature>::build()
                .validate_toolbars([VERILOGA_TOOLBAR, NETLIST_TOOLBAR]),
            Ok(())
        );
    }
}
