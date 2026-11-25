//! The shell's answers to what the design surface asks of another
//! feature's workspace: the results workspace, the source feature's form and
//! netlist, the models feature's selection and the project's copy-cell
//! dialog. The surface raises each request where it once wrote in place,
//! and the shell answers it once the surface has drawn, in the same frame,
//! writing each workspace through its owner.
//!
//! Here too, the source map a design dialog asks for, which opens the code
//! feature's netlist page, and the other way round, a result signal the
//! results workspace asks Design to show in another occurrence.

use crate::workbench::AppState;
use rspice_app_code::workbench::app::dialogs::library::new_veriloga_device::open_new_veriloga_device_form_in;
use rspice_app_design::schematic::view::{
    DesignSurfaceRequest, DifferencePlot, PlotProbedDifference, PlotProbedTrace, ProbePlot,
};
use rspice_app_results::workbench::commands::schematic_cross_probe::ShowSignalOnSchematic;
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
        // locates; the source feature answers it straight after this.
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

/// Keep the voltage between two probed nets as an expression, as the
/// calculator's Plot keeps one: on the view in front, so the strip draws it
/// beside the run's own signals, or, with no run shown, for the view of the
/// plan's next run. Say what changed.
pub(in crate::workbench) fn plot_probed_difference(
    state: &mut AppState,
    probed: PlotProbedDifference,
) {
    use rspice_app_results::workbench::documents::result_document::{
        keep_for_next_run, keep_plotted_expression,
    };

    let PlotProbedDifference {
        on,
        expression,
        saved,
    } = probed;
    let waiting = (!saved.is_empty()).then(|| {
        format!(
            "Saved {} for the next run; {expression} plots once it runs.",
            saved.join(" and ")
        )
    });
    let analysis = match on {
        DifferencePlot::Analysis(analysis) => analysis,
        DifferencePlot::NextRunOf(plan) => {
            keep_for_next_run(&mut state.results_parts(), plan, expression.clone());
            state.prompt(
                waiting.unwrap_or_else(|| format!("{expression} plots after the next run.")),
            );
            return;
        }
    };
    let kept = keep_plotted_expression(&mut state.results_parts(), analysis, expression.clone());
    match (kept, waiting) {
        // The run shown lacks a net: the view keeps the difference, and the
        // run that saves the net draws it.
        (Ok(_), Some(waiting)) => state.prompt(waiting),
        (Ok(added), None) => {
            // A probe asks to see the signal, as a single net's probe does.
            if state.workbench.supports_results_split() {
                state.workbench.show_results_split();
            }
            state.prompt(if added {
                format!("{expression} added to the plot.")
            } else {
                format!("{expression} is already in the plot.")
            });
        }
        (Err(error), _) => state.prompt(format!("{expression} was not plotted: {error}")),
    }
}

/// Show a result signal read in another occurrence than the tab in front:
/// land the canvas on the occurrence the design draws for its scope, through
/// the descent the hierarchy's occurrence rows take, and show the signal
/// there as every Results surface shows one.
pub(in crate::workbench) fn show_signal_in_its_occurrence(
    state: &mut AppState,
    request: ShowSignalOnSchematic,
) {
    use rspice_app_design::workbench::design::navigator::hierarchy_tree::open_occurrence;
    use rspice_app_state::state::InstancePath;

    let ShowSignalOnSchematic {
        signal,
        scope,
        from,
    } = request;
    let drawn = if scope.is_root() {
        // The root is the first step of the breadcrumb the tab descended by.
        state.navigation().focus_workspace_breadcrumb(0);
        InstancePath::root()
    } else {
        let drawn = state
            .model
            .workspace
            .design_projection(
                &state.model.library_manager,
                &state.model.workspace.content.active_view,
                &state.model.schematic,
            )
            .ok()
            .and_then(|projection| {
                projection
                    .plan()
                    .engine_occurrence(&scope)
                    .map(|occurrence| occurrence.instance_path().clone())
            });
        let Some(drawn) = drawn else {
            state.prompt(format!(
                "{from} is read inside {scope}, which this design does not draw."
            ));
            return;
        };
        open_occurrence(
            &mut state.design,
            &mut state.model,
            &mut state.workbench,
            &state.runs,
            &drawn,
        );
        drawn
    };
    if !state
        .model
        .workspace
        .content
        .occurrence_path()
        .same_engine_scope(&scope)
    {
        state.prompt(format!(
            "{drawn} could not be opened, so {from} cannot be shown there."
        ));
        return;
    }
    state.workbench.activate(Workspace::Design);
    match state.model.select_signal_on_drawing(&signal) {
        Ok(place) => state.prompt(place.message(&from)),
        Err(error) => state.prompt(error.message(&from)),
    }
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
        plot_probed_difference,
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
