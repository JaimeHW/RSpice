//! The shell's answers to what the design surface asks of another
//! feature's workspace: the results workspace, the code feature's form and
//! netlist, the models feature's selection and the project's copy-cell
//! dialog. The surface raises each request where it once wrote in place,
//! and the shell answers it once the surface has drawn, in the same frame,
//! writing each workspace through its owner.
//!
//! Here too, the source map a design dialog asks for, which opens the code
//! feature's netlist page.

use crate::workbench::AppState;
use rspice_app_code::workbench::app::dialogs::library::new_veriloga_device::open_new_veriloga_device_form_in;
use rspice_app_design::schematic::view::{DesignSurfaceRequest, SelectProbedTrace};
use rspice_app_state::state::VerificationPage;
use rspice_app_state::workbench::workspace::Workspace;

/// Carry out what the design surface asked of another feature's workspace,
/// as the surface once did in place.
pub(in crate::workbench) fn answer_surface_request(
    state: &mut AppState,
    request: DesignSurfaceRequest,
) {
    match request {
        DesignSurfaceRequest::ShowOperatingPoint { device } => {
            state.results.show_operating_point(device);
        }
        DesignSurfaceRequest::OpenSpecifications => {
            state.simulate.verification_page = VerificationPage::Specifications;
            state.workbench.activate(Workspace::Verify);
        }
        DesignSurfaceRequest::NewVerilogADevice => {
            open_new_veriloga_device_form_in(&state.model, &mut state.code.new_veriloga_device);
        }
        DesignSurfaceRequest::CreateSymbolFromPorts => {
            state.design.open_create_model_bound_symbol(
                &state.model,
                &mut state.workbench,
                state.models.selected_model.as_deref(),
            );
        }
        DesignSurfaceRequest::ShowInNetlist => state.show_selected_instance_in_netlist(),
        DesignSurfaceRequest::CopyToEditableLibrary { library, cell } => {
            if let Err(error) = state
                .project
                .open_copy_cell_dialog(&state.model, &library, &cell)
            {
                state.prompt(error);
            }
        }
    }
}

/// Select the trace a probe selected.
pub(in crate::workbench) fn select_probed_trace(state: &mut AppState, probed: SelectProbedTrace) {
    let SelectProbedTrace(trace) = probed;
    state.results.select_probed_trace(trace);
}

/// Answer what the design surface raised, as the shell does once the
/// surface has drawn.
#[cfg(test)]
pub(crate) fn answer_surface_requests(state: &mut AppState) {
    use rspice_app_workbench::workbench::framework::requests::drain;
    drain(
        state,
        |state| &mut state.workbench.outbox,
        select_probed_trace,
    );
    drain(
        state,
        |state| &mut state.workbench.outbox,
        answer_surface_request,
    );
}

/// Open the generated netlist on its source map, at the first line it maps,
/// with the manager closed.
pub(in crate::workbench) fn open_connectivity_source_map(state: &mut AppState) {
    state.design.connectivity_manager.close();
    state.workbench.workspace = rspice_app_state::workbench::workspace::Workspace::Netlist;
    state.workbench.navigator_visible = true;
    state.workbench.drawer =
        Some(rspice_app_workbench::workbench::state::workbench_session::Drawer::Navigator);
    state.code.netlist.active_document =
        rspice_app_code::workbench::documents::netlist_document::ActiveNetlistDocument::Generated;
    state.workbench.set_navigator_filter("source mapping");
    if let Some(line) = state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map().first())
        .map(|entry| entry.generated_line())
    {
        state.code.netlist.cursor_line = line.saturating_sub(1);
        state.code.netlist.requested_line = Some(line);
    }
}
