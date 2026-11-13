//! The shell's answers to what the design surface asks of another
//! feature's workspace: the results workspace, the code feature's form and
//! netlist, the models feature's selection and the project's copy-cell
//! dialog. The surface raises each request where it once wrote in place,
//! and the shell answers it once the surface has drawn, in the same frame.

use crate::schematic::view::{DesignSurfaceRequest, SelectProbedTrace};
use crate::workbench::app::{open_create_model_bound_symbol_dialog, open_new_veriloga_device_form};
use crate::workbench::documents::result_document::open_specification_editor;
use crate::workbench::{AppState, ResultViewer};

/// Carry out what the design surface asked of another feature's workspace,
/// as the surface once did in place.
pub(in crate::workbench) fn answer_surface_request(
    state: &mut AppState,
    request: DesignSurfaceRequest,
) {
    match request {
        DesignSurfaceRequest::ShowOperatingPoint { device } => {
            let session = &mut state.results.results.session;
            session.op_filter = device;
            session.viewer = ResultViewer::Op;
        }
        DesignSurfaceRequest::OpenSpecificationEditor => {
            open_specification_editor(&mut state.results_parts());
        }
        DesignSurfaceRequest::NewVerilogADevice => open_new_veriloga_device_form(state),
        DesignSurfaceRequest::CreateSymbolFromPorts => {
            open_create_model_bound_symbol_dialog(
                &mut state.design.create_model_bound_symbol,
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
    state.results.results.session.selected_trace = Some(trace);
}

/// Answer what the design surface raised, as the shell does once the
/// surface has drawn.
#[cfg(test)]
pub(crate) fn answer_surface_requests(state: &mut AppState) {
    use crate::workbench::framework::requests::drain;
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
