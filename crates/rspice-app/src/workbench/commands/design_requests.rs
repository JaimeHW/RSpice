//! The shell's answers to what the design surface asks of another
//! feature's workspace: the results workspace, the code feature's form and
//! netlist, the models feature's selection and the project's copy-cell
//! dialog. The surface raises each request where it once wrote in place,
//! and the shell answers it once the surface has drawn, in the same frame.
//!
//! Here too, what the design feature's dialogs hand the shell to carry out
//! over the whole session.

use crate::schematic::view::{DesignSurfaceRequest, SelectProbedTrace};
use crate::workbench::app::dialogs::connectivity_manager::{
    apply_selected_connectivity_repairs, reveal_connectivity_target,
};
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app::dialogs::hierarchy::create::commit_create_hierarchy;
use crate::workbench::app::dialogs::hierarchy::descend::answer_descend_hierarchy;
use crate::workbench::app::dialogs::library::new_veriloga_device::open_new_veriloga_device_form_in;
use crate::workbench::app::open_create_model_bound_symbol_dialog;
use crate::workbench::documents::result_document::open_specification_editor;
use crate::workbench::{AppState, RSpiceApp, ResultViewer};

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
        DesignSurfaceRequest::NewVerilogADevice => {
            open_new_veriloga_device_form_in(&state.model, &mut state.code.new_veriloga_device);
        }
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

impl RSpiceApp {
    /// Carry out what a design dialog handed over, as the dialog once did in
    /// place.
    ///
    /// The shell answers these for now, because what they do still reaches
    /// the whole session: the source map the code feature's netlist, and
    /// applying repairs the simulation controller. Design carries each out
    /// itself once it sits on its own parts.
    pub(crate) fn answer_design_request(&mut self, request: DesignRequest) {
        match request {
            DesignRequest::Descend(choice) => answer_descend_hierarchy(&mut self.state, choice),
            DesignRequest::ReopenPropertyEditor(component_id) => {
                self.state.open_property_editor(component_id);
            }
            DesignRequest::RevealConnectivity(target) => {
                self.state.design.connectivity_manager.close();
                reveal_connectivity_target(self.state.navigation(), target);
            }
            DesignRequest::OpenSourceMap => open_connectivity_source_map(&mut self.state),
            DesignRequest::ApplyConnectivityRepairs => {
                let state = &mut self.state;
                let review_markers = state.design.schematic_visibility.review_markers;
                if let Err(error) = apply_selected_connectivity_repairs(
                    &mut state.design.connectivity_manager,
                    &mut state.model,
                    &mut state.workbench,
                    &mut state.runs,
                    &mut self.simulation_controller,
                    review_markers,
                ) {
                    state.design.connectivity_manager.error = Some(error);
                }
            }
            DesignRequest::CreateHierarchy => {
                let state = &mut self.state;
                if let Err(error) = commit_create_hierarchy(
                    &mut state.design.create_hierarchy,
                    &mut state.design.symbol,
                    &mut state.model,
                    &state.workbench,
                ) {
                    self.state.design.create_hierarchy.validation_error = Some(error);
                }
            }
        }
    }
}

/// Open the generated netlist on its source map, at the first line it maps,
/// with the manager closed.
fn open_connectivity_source_map(state: &mut AppState) {
    state.design.connectivity_manager.close();
    state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    state.workbench.navigator_visible = true;
    state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);
    state.code.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated;
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
