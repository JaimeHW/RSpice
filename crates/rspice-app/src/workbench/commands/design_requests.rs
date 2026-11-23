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
use rspice_app_design::schematic::view::{DesignSurfaceRequest, PlotProbedTrace, ProbePlot};
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
        // The command's own route, which generates the netlist before it
        // locates; the code feature answers it straight after this.
        DesignSurfaceRequest::ShowInNetlist => state
            .workbench
            .outbox
            .push(rspice_app_code::workbench::code::commands::SourceRequest::ShowInNetlist),
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

/// Plot the trace a probe named through the tick the results workspace's
/// Data Browser uses, so the row, the legend and the strip all show it,
/// select it, and say what changed.
pub(in crate::workbench) fn plot_probed_trace(state: &mut AppState, probed: PlotProbedTrace) {
    use rspice_app_results::workbench::docks::navigator::results_actions::set_result_quantity_ticked;
    use rspice_app_results::workbench::documents::result_document::waveform_plotted;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    use rspice_results_ui::selection::ResultBrowserSelectionKey;

    let PlotProbedTrace {
        trace,
        plot,
        display,
    } = probed;
    let key = SourceWaveformPresentationKey::new(trace.analysis_key(), trace.source_name());
    let Some((.., waveform)) = key.resolve(&state.model.simulation.retained.runs) else {
        return;
    };
    let plotted = waveform_plotted(
        &state.model,
        &state.results.results,
        &key,
        trace.source_name(),
        waveform.visible,
    );
    let tick = match plot {
        ProbePlot::Show => true,
        ProbePlot::Toggle => !plotted,
    };
    set_result_quantity_ticked(
        &mut state.results_parts(),
        &ResultBrowserSelectionKey::Waveform(key),
        tick,
    );
    state.results.select_probed_trace(trace);
    // A probe asks to see the signal: the plot pane opens beside the
    // drawing, whether or not the reader closed it before.
    if tick && state.workbench.supports_results_split() {
        state.workbench.show_results_split();
    }
    state.prompt(match (plotted, tick) {
        (false, _) => format!("{display} added to the plot."),
        (true, true) => format!("{display} is already in the plot."),
        (true, false) => format!("{display} removed from the plot."),
    });
}

/// Answer what the design surface raised, as the shell does once the
/// surface has drawn.
#[cfg(test)]
pub(crate) fn answer_surface_requests(state: &mut AppState) {
    use rspice_app_workbench::workbench::framework::requests::drain;
    drain(
        state,
        |state| &mut state.workbench.outbox,
        plot_probed_trace,
    );
    drain(
        state,
        |state| &mut state.workbench.outbox,
        answer_surface_request,
    );
}

/// Show the generated netlist at the first card that connects `net`, with
/// the manager closed; a net no card names lands on the first mapped line.
/// Nothing is typed into a filter: the jump is the answer, and a filter left
/// in the Netlist workspace's navigator is cleared, so the navigator lists
/// the schematic source of the line it lands on. The caller has put the
/// generated netlist in front the way every jump to it does.
pub(in crate::workbench) fn open_connectivity_source_map(state: &mut AppState, net: &str) {
    state.design.connectivity_manager.close();
    state
        .workbench
        .navigator_trees
        .filter_mut(Workspace::Netlist)
        .clear();
    let source_map = state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .map(|artifact| artifact.source_map());
    if let Some(line) = source_map
        .and_then(|entries| {
            entries
                .iter()
                .find(|entry| {
                    entry
                        .nets()
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(net))
                })
                .or_else(|| entries.first())
        })
        .map(|entry| entry.generated_line())
    {
        state.code.netlist.cursor_line = line.saturating_sub(1);
        state.code.netlist.requested_line = Some(line);
    }
}
