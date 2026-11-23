//! The code feature's navigator and inspector, over what the feature lends
//! the docks that draw them.

use crate::workbench::MessageId;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::design_system::inspector::muted_inspector_copy;
use crate::workbench::docks::inspector::code_workspace::{
    CodeInspector, FindingPress, code_workspace_inspector,
};
use crate::workbench::docks::inspector::veriloga;
use crate::workbench::docks::navigator::netlist::{NetlistNavigator, netlist as netlist_navigator};
use crate::workbench::docks::navigator::source_bundle::{RailAction, SourceRail};
use crate::workbench::docks::navigator::veriloga as veriloga_navigator;
use crate::workbench::documents::code_workspace::VerilogAInspectorAction;
use crate::workbench::documents::netlist_document::{
    NetlistRoot, active_dependency_in, root_deck_source_index,
};
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::OpenInDesign;
use crate::workbench::state::Workspace;

use super::CodeFeature;
use super::commands::{SourceDocumentRequest, SourceRequest};
use super::netlist_first::{self, DeckFacts, deck_fact_rows, deck_facts};

/// The navigator of the code document in front: the Netlist workspace's
/// outline, or the rail of the Verilog-A view the Design workspace shows.
/// What the navigator changes of the feature's own state it changes in
/// place; what it asks of the whole application is raised for the shell,
/// which answers it once the navigator has drawn.
pub(super) fn show_navigator(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui) {
    // A deck project's Design page has no schematic, so nothing of one to
    // list: every group would be empty, and its one occurrence the empty top
    // cell. Its Simulate page has no plan, whose setup is what Simulate's
    // navigator lists.
    if deck_page(cx) {
        let says = if cx.wb.workspace == Workspace::Simulate {
            MessageId::NetlistFirstNoPlan
        } else {
            MessageId::NetlistFirstNoSchematic
        };
        muted_inspector_copy(ui, &cx.wb.messages().text(says));
        return;
    }
    if cx.wb.workspace != Workspace::Netlist {
        let asked = veriloga_navigator::show(ui, &mut source_rail(cx));
        answer_rail(cx, asked);
        return;
    }
    let messages = cx.wb.messages();
    let asked = netlist_navigator(
        ui,
        &mut NetlistNavigator {
            model: cx.model,
            netlist: &mut cx.state.netlist,
            code_workspace: &mut cx.state.code_workspace,
            outline_collapsed: &mut cx.state.netlist_outline_collapsed,
            open_documents: &mut cx.state.netlist_open_documents,
            runs: cx.runs,
            filter: cx.wb.navigator_filter(),
            coarse_pointer: cx.wb.coarse_pointer,
            messages,
        },
    );
    if let Some(text) = asked.prompt {
        cx.wb.prompt(text);
    }
    if let Some(rewrite) = asked.use_section {
        cx.wb.outbox.push(SourceRequest::UseLibSection {
            dependency: rewrite.dependency,
            original: rewrite.original,
            replacement: rewrite.replacement,
            selected: rewrite.selected,
        });
    }
    if let Some(reference) = asked.open_in_design {
        cx.wb.outbox.push(OpenInDesign(reference));
    }
}

/// What a source bundle rail reads, lent from the feature's context.
fn source_rail<'a>(cx: &'a mut Cx<'_, CodeFeature>) -> SourceRail<'a> {
    SourceRail {
        model: cx.model,
        code_workspace: &mut cx.state.code_workspace,
        platform: cx.platform,
        messages: cx.wb.messages(),
    }
}

/// Carry out what a rail asked: say why its choice failed, or raise the
/// file action or the placed instance for the shell to open.
fn answer_rail(cx: &mut Cx<'_, CodeFeature>, asked: Option<RailAction>) {
    match asked {
        None => {}
        Some(RailAction::Prompt(text)) => cx.wb.prompt(text),
        Some(RailAction::FileAction {
            bundle_id,
            action,
            path,
        }) => cx.wb.outbox.push(SourceRequest::SourceFileAction {
            bundle_id,
            action,
            path,
        }),
        Some(RailAction::Document { action, path }) => {
            cx.wb.outbox.push(SourceRequest::SourceDocument(
                SourceDocumentRequest::Document {
                    action,
                    logical_path: path,
                },
            ));
        }
        Some(RailAction::OpenInstance(usage)) => {
            cx.wb.outbox.push(SourceRequest::OpenCellInstance(usage));
        }
    }
}

/// The inspector of the code document in front: the Netlist workspace's
/// deck, or the Verilog-A view the Design workspace shows.
/// Revealing a Verilog-A finding routes the diagnostics in place.
pub(super) fn show_inspector(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui) {
    // A deck project's Design and Simulate pages inspect the deck: the facts
    // the Netlist workspace lists beside it.
    if deck_page(cx) {
        netlist_first::open_deck(cx);
        let facts = root_deck_facts(cx, ui.ctx());
        deck_fact_rows(ui, &facts, &[], cx.wb.messages());
        return;
    }
    if cx.wb.workspace != Workspace::Netlist {
        let asked = veriloga::show(ui, cx.model, &cx.state.code_workspace, cx.wb.messages());
        // The same road a finding in Problems takes, so the two agree on
        // where the caret lands.
        if let Some(VerilogAInspectorAction::RevealFinding {
            bundle,
            logical_path,
            line,
            column,
        }) = asked
        {
            cx.model
                .diagnostics
                .request_route(DiagnosticRoute::CodeDocument {
                    bundle,
                    logical_path,
                    line: Some(line),
                    column,
                });
        }
        return;
    }
    // The deck's own facts while the deck is in front; an include, the
    // generated netlist or a run's input states its own.
    let netlist = &cx.state.netlist;
    let deck_in_front = netlist.open_deck.is_some()
        && netlist.front.shows_deck()
        && netlist.front.root_kind() == NetlistRoot::Deck
        && active_dependency_in(cx.model, netlist).is_none();
    let facts = deck_in_front.then(|| root_deck_facts(cx, ui.ctx()));
    let pressed = code_workspace_inspector(
        ui,
        &CodeInspector {
            model: cx.model,
            netlist: &cx.state.netlist,
            deck_facts: facts.as_ref(),
            messages: cx.wb.messages(),
        },
    );
    match pressed {
        // A pressed finding takes the road a Problems row takes to its line.
        Some(FindingPress::Row(finding)) => {
            cx.model
                .diagnostics
                .request_route(DiagnosticRoute::NetlistDiagnostic(finding));
        }
        Some(FindingPress::Fix(finding)) => {
            cx.wb
                .outbox
                .push(SourceRequest::ApplyNetlistFindingFix(finding));
        }
        None => {}
    }
}

/// Whether the docks are the Design or Simulate page's of a project whose
/// simulation is its deck and which has no schematic.
fn deck_page(cx: &Cx<'_, CodeFeature>) -> bool {
    matches!(cx.wb.workspace, Workspace::Design | Workspace::Simulate)
        && cx.model.is_netlist_first_without_schematic()
}

/// The project's deck's facts, counted from its root file.
fn root_deck_facts(cx: &mut Cx<'_, CodeFeature>, ctx: &egui::Context) -> DeckFacts {
    let index = root_deck_source_index(ctx, cx.model, &mut cx.state.netlist);
    deck_facts(cx.model, &cx.state.netlist, &index, cx.wb.messages())
}
