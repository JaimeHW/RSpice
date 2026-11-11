//! The code feature's inspector, over what the feature lends the docks that
//! draw it.

use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::docks::inspector::code_workspace::{CodeInspector, code_workspace_inspector};
use crate::workbench::docks::inspector::veriloga;
use crate::workbench::documents::code_workspace::{CodeWorkspacePage, VerilogAInspectorAction};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::state::Workspace;

use super::CodeFeature;

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
