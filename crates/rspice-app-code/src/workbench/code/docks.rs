//! The code feature's navigator and inspector, over what the feature lends
//! the docks that draw them.

use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::docks::inspector::code_workspace::{CodeInspector, code_workspace_inspector};
use crate::workbench::docks::inspector::veriloga;
use crate::workbench::docks::navigator::netlist::{
    NetlistNavigator, code_workspace_pages, netlist as netlist_navigator,
};
use crate::workbench::docks::navigator::source_bundle::{RailAction, SourceRail, automation};
use crate::workbench::docks::navigator::veriloga as veriloga_navigator;
use crate::workbench::documents::code_workspace::{CodeWorkspacePage, VerilogAInspectorAction};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::OpenInDesign;
use crate::workbench::state::Workspace;

use super::CodeFeature;
use super::commands::CodeRequest;

/// The navigator of the code document in front: the Netlist workspace's
/// page tabs, then the page's outline or the automation bundle's rail; or
/// the rail of the Verilog-A view the Design workspace shows. What the
/// navigator changes of the feature's own state it changes in place; what
/// it asks of the whole application is raised for the shell, which answers
/// it once the navigator has drawn.
pub(super) fn show_navigator(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui) {
    if cx.wb.workspace != Workspace::Netlist {
        let asked = veriloga_navigator::show(ui, &mut source_rail(cx));
        answer_rail(cx, asked);
        return;
    }
    let asked = code_workspace_pages(
        ui,
        cx.state.code_workspace.page,
        cx.wb.coarse_pointer,
        cx.wb.messages(),
    );
    if let Some(page) = asked {
        cx.state.code_workspace.page = page;
        cx.state.netlist.completion_open = false;
    }
    // The outline belongs to the page: an automation pipeline and a SPICE
    // deck are different objects and read differently, so neither borrows
    // the other's tree.
    if cx.state.code_workspace.page == CodeWorkspacePage::Automation {
        let asked = automation(ui, &mut source_rail(cx));
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
        cx.wb.outbox.push(CodeRequest::UseLibSection {
            dependency: rewrite.dependency,
            original: rewrite.original,
            replacement: rewrite.replacement,
            selected: rewrite.selected,
        });
    }
    if let Some(reference) = asked.open_in_design {
        cx.wb.outbox.push(OpenInDesign(reference));
    }
    if let Some(identity) = asked.relink {
        cx.wb
            .outbox
            .push(CodeRequest::RelinkNetlistDependency(identity));
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
            language,
            bundle_id,
            action,
            path,
        }) => cx.wb.outbox.push(CodeRequest::SourceFileAction {
            language,
            bundle_id,
            action,
            path,
        }),
        Some(RailAction::OpenInstance(usage)) => {
            cx.wb.outbox.push(CodeRequest::OpenCellInstance(usage));
        }
    }
}

/// The inspector of the code document in front: the Netlist workspace's deck
/// or automation document, or the Verilog-A view the Design workspace shows.
/// Tuning the deck's parameters is raised as the command, which the shell
/// answers once the inspector has drawn; revealing a Verilog-A finding routes
/// the diagnostics in place.
pub(super) fn show_inspector(cx: &mut Cx<'_, CodeFeature>, ui: &mut egui::Ui) {
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
    // The answer is held only while the deck's page is in front, the one
    // page that offers the sandbox.
    let tuning_available = cx.state.code_workspace.page == CodeWorkspacePage::Netlist
        && cx.foreign_available(ids::TUNING);
    let tune = code_workspace_inspector(
        ui,
        &CodeInspector {
            model: cx.model,
            netlist: &cx.state.netlist,
            code_workspace: &cx.state.code_workspace,
            runs: cx.runs,
            workspace: cx.wb.workspace,
            project_open: cx.platform.project_lifecycle.is_open(),
            messages: cx.wb.messages(),
            tuning_available,
        },
    );
    if tune {
        cx.execute(ids::TUNING);
    }
}
