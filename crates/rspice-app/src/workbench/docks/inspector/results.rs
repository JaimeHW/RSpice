//! The Results workspace's inspector, which also reads a trace selected in the
//! split beside Design: the active pane and its axes, the selected trace or
//! artifact, and the run it came from.

use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{Problem, panel_inset, section_block_gap, section_header};

use egui::{Align2, Color32, Pos2, Rect, Sense, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::result_document::analysis_evidence_failure;
use crate::workbench::documents::result_document::manifest::active_manifest;
use crate::workbench::{AppState, RSpiceApp, ResultViewer};
use rspice_results_ui::session::PaneAxis;

use super::super::super::commands::vocabulary::Command;
use super::super::super::design_system::{
    WorkbenchIcon, property_row, property_row_input_action, property_row_problem,
};
use super::super::super::state::Workspace;
use super::{compact_readout, result_authority, run_details};

pub(in crate::workbench) fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected_trace = app
        .state
        .results
        .results
        .valid_selected_trace(&app.state.model.simulation)
        .cloned();
    let selected_artifact = app
        .state
        .results
        .results
        .session
        .selected_result_artifact
        .clone()
        .filter(|key| {
            key.resolve(&app.state.model.simulation.retained.runs)
                .is_some()
        });

    // What is being read comes before where it came from: a reader adjusting
    // a pane needs its axes and bindings first, and the dataset provenance
    // below is unchanged by any of that.
    active_result_pane(ui, app, selected_trace.as_ref());
    if let Some(selected) = selected_trace.as_ref() {
        selected_result_trace(ui, app, selected);
    } else if let Some(selected) = selected_artifact.as_ref() {
        selected_result_artifact(ui, app, selected);
    }

    // The workspace's memoized projection: taking it fresh re-digested the run.
    let manifest = active_manifest(&mut app.state);
    let details = match app
        .state
        .model
        .simulation
        .active_run()
        .zip(manifest.as_deref())
    {
        Some((run, manifest)) => result_authority::result_run(ui, run, &manifest.model),
        None => {
            muted_inspector_copy(ui, "No run selected.");
            None
        }
    };
    if let Some(run_id) = details {
        run_details::open(&mut app.state, run_id);
    }

    // The active viewer owns its engineering readout (measurements, margins,
    // harmonics, eye metrics, distribution statistics, and so on). It follows
    // the trace and pane controls, matching the upgraded mockup's hierarchy
    // without creating a second result-data owner here.
    if app.state.results.results.session.viewer != ResultViewer::Manifest {
        crate::workbench::documents::result_document::right_panel(ui, &mut app.state);
    }

    let Some(run) = app.state.model.simulation.active_run() else {
        return;
    };
    if let Some(index) = app.state.model.simulation.view.active_analysis_idx
        && let Some(analysis) = run.analyses.get(index)
    {
        result_authority::result_convergence(ui, analysis);
        result_authority::result_dc_sweep(ui, analysis);
    }
}

fn selected_result_artifact(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &rspice_results::result_presentation::ResultArtifactPresentationKey,
) {
    let Some((run_index, _, analysis)) =
        selected.resolve(&app.state.model.simulation.retained.runs)
    else {
        return;
    };
    let run = &app.state.model.simulation.retained.runs[run_index];
    // Memoized per dataset generation: the validator walks every retained
    // sample. Intact data is the normal state and says nothing.
    let corrupted = analysis_evidence_failure(&app.state, run.dataset_id, analysis);
    let analysis_label = analysis.label.clone();

    section_header(ui, "Selected result", None);
    property_row(ui, "Name", selected.canonical_name());
    property_row(ui, "Analysis", &analysis_label);
    if let Some(error) = corrupted {
        property_row_problem(ui, "Data", &error.to_string(), Some(Problem::Failure));
    }
}

fn selected_result_trace(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
) {
    let Some(run) = app.state.model.simulation.active_run() else {
        return;
    };
    let Some((analysis_index, waveform_index, analysis, waveform)) = selected.resolve(run) else {
        return;
    };

    let waveform_name = waveform.name.clone();
    let waveform_color = waveform.color.clone();
    let waveform_visible = waveform.visible;
    let sample_count = waveform.x.len().min(waveform.y.len());
    let statistics = finite_trace_statistics(&waveform.y);
    let available_sample_count = waveform.y.iter().filter(|value| value.is_finite()).count();
    // The statistics are measurements of the trace, read in the trace's own
    // unit at the precision this panel's rows hold.
    let unit = crate::workbench::documents::result_document::browser_signal_unit(
        &waveform.name,
        waveform.unit.as_deref(),
        crate::workbench::documents::result_document::analysis_default_unit(analysis.analysis_type),
    )
    .to_owned();
    let measured = |value: f64| compact_readout(value, &unit);

    section_header(ui, "Selected trace", None);
    property_row(ui, "Name", &waveform_name);
    // Samples the solver could not give a finite value are a problem with
    // the trace; a complete trace says nothing about its sample count.
    if available_sample_count < sample_count {
        property_row_problem(
            ui,
            "Finite samples",
            &format!("{available_sample_count} of {sample_count}"),
            Some(Problem::Warning),
        );
    }
    if let Some(statistics) = statistics {
        property_row(ui, "Minimum", &measured(statistics.minimum));
        property_row(ui, "Maximum", &measured(statistics.maximum));
        property_row(ui, "Mean", &measured(statistics.mean));
        property_row(
            ui,
            "Peak-to-peak",
            &measured(statistics.maximum - statistics.minimum),
        );
    }

    let t = Tokens::get(ui.ctx());
    let mut color = rspice_results_ui::presentation::trace_color(
        &waveform_color,
        t.color.traces[waveform_index % t.color.traces.len()],
    );
    if trace_colour_row(ui, &mut color) {
        set_selected_trace_color(&mut app.state, selected, color);
    }

    section_block_gap(ui);
    ui.horizontal_wrapped(|ui| {
        if ui
            .button(if waveform_visible {
                "Hide trace"
            } else {
                "Show trace"
            })
            .clicked()
        {
            crate::workbench::documents::result_document::toggle_visibility(
                &mut app.state,
                analysis_index,
                waveform_index,
            );
        }

        if is_schematic_cross_probe_candidate(selected.source_name()) {
            let signal = selected.source_name().to_owned();
            let unavailable = schematic_cross_probe_unavailability(&app.state, selected, &signal);
            let mut response =
                ui.add_enabled(unavailable.is_none(), egui::Button::new("Cross-probe net"));
            if let Some(reason) = unavailable {
                response = response.on_hover_text(reason);
            }
            if response.clicked() {
                match cross_probe_trace_to_design(app, &signal) {
                    Ok(net) => app
                        .state
                        .prompt(format!("Selected conductor {net} from {signal}.")),
                    Err(message) => app.state.prompt(message),
                }
            }
        }
    });
}

fn active_result_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: Option<&rspice_results_ui::selection::SelectedResultTrace>,
) {
    let viewer = app.state.results.results.session.viewer;
    // A pure evidence table — OP, specs, samples, events, the manifest — has
    // no drawn pane at all. Reporting one would have the inspector state a
    // view, a fit and a limit mask for a sheet that has none of them.
    if !crate::workbench::documents::result_document::viewer_draws_a_pane(viewer) {
        return;
    }
    let Some(run) = app.state.model.simulation.active_run() else {
        return;
    };
    let analysis_index = selected
        .and_then(|trace| {
            trace
                .resolve(run)
                .map(|(analysis_index, _, _, _)| analysis_index)
        })
        .or(app.state.model.simulation.view.active_analysis_idx)
        .unwrap_or(0);
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let pane_label = analysis.label.clone();
    // The unit names the pane in a unit-scoped stack; the viewer name is
    // already on the sheet tab above.
    let tokens = Tokens::get(ui.ctx());
    let facts =
        crate::workbench::documents::result_document::active_pane_facts(&tokens, &mut app.state);
    section_header(
        ui,
        "Active pane",
        Some(facts.unit.as_deref().unwrap_or_else(|| viewer.label())),
    );
    // The pane names the analysis it draws, which on a sheet outside the
    // waveform stack is not the one the run's analysis selector points at.
    property_row(ui, "Pane", facts.analysis.as_deref().unwrap_or(&pane_label));
    if let Some(scale) = facts.scale {
        property_row(ui, "Scale", scale);
    }
    property_row(ui, "Limit mask", facts.limit_mask);
    // Explicit limits, each with its own fit. Comparing two runs on identical
    // axes, or holding a decade while a parameter sweeps, cannot be done by
    // dragging; the fields show the viewport, so no row restates it.
    axis_limit_row(ui, app, &facts, PaneAxis::X, "X range");
    axis_limit_row(ui, app, &facts, PaneAxis::Y, "Y range");
}

/// An explicit interval for one axis of the active pane, or automatic fit.
///
/// The plot's own gestures can only ever say "a bit more than that". Holding
/// a decade while a parameter sweeps, or putting two runs on identical axes
/// so their curves can be compared at all, needs the numbers themselves.
fn axis_limit_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    facts: &crate::workbench::documents::result_document::ActivePaneFacts,
    axis: PaneAxis,
    label: &str,
) {
    use crate::workbench::documents::result_document::axis_range::{
        format_axis_range, parse_axis_range,
    };
    use crate::workbench::documents::result_document::{
        active_axis_is_pinned, active_axis_range, set_active_axis_range,
    };

    let viewer = app.state.results.results.session.viewer;
    let current = active_axis_range(&app.state, facts, axis);
    let pinned = active_axis_is_pinned(&app.state, axis);
    // The unit the field writes after its numbers, and strips before reading
    // them back. Only the waveform stack knows its axes' units; every other
    // sheet's field states bare engineering numbers, as it always has.
    let unit = match axis {
        PaneAxis::X => facts.x_unit.as_deref(),
        PaneAxis::Y => facts.unit.as_deref(),
    }
    .filter(|_| crate::workbench::documents::result_document::viewer_uses_wave_stack(viewer))
    .unwrap_or_default();
    let committed = current
        .map(|range| format_axis_range(range, unit))
        .unwrap_or_default();
    let mut text = match app.state.results.results.session.axis_limit_draft.as_ref() {
        Some((draft_viewer, draft_axis, text))
            if *draft_viewer == viewer && *draft_axis == axis =>
        {
            text.clone()
        }
        _ => committed.clone(),
    };

    let parsed = parse_axis_range(&text, unit);
    let invalid = !text.trim().is_empty() && parsed.is_none();
    let (edit, reset) = property_row_input_action(
        ui,
        label,
        &mut text,
        invalid,
        crate::workbench::design_system::PropertyRowAction {
            icon: WorkbenchIcon::ZoomFit,
            label: "Fit this axis to the retained data",
            enabled: pinned,
            disabled_reason: Some("This axis is already fitting its data."),
        },
    );

    if edit.changed() {
        app.state.results.results.session.axis_limit_draft = Some((viewer, axis, text.clone()));
    }
    // Commit on Enter or on leaving the field, never per keystroke: "1.2" is
    // a legal prefix of "1.2m" and three orders of magnitude away from it.
    if edit.lost_focus() {
        let tokens = Tokens::get(ui.ctx());
        if text == committed {
            // The field shows the interval to six significant digits. Read
            // back unedited, that would pin an autofitting axis, or nudge a
            // pinned one, by the rounding alone.
            app.state.results.results.session.axis_limit_draft = None;
        } else if text.trim().is_empty() {
            set_active_axis_range(&tokens, &mut app.state, axis, None);
            app.state.results.results.session.axis_limit_draft = None;
        } else if let Some(range) = parse_axis_range(&text, unit) {
            set_active_axis_range(&tokens, &mut app.state, axis, Some(range));
            app.state.results.results.session.axis_limit_draft = None;
        }
    }
    if reset.clicked() {
        let tokens = Tokens::get(ui.ctx());
        set_active_axis_range(&tokens, &mut app.state, axis, None);
        app.state.results.results.session.axis_limit_draft = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TraceStatistics {
    minimum: f64,
    maximum: f64,
    mean: f64,
}

fn finite_trace_statistics(values: &[f64]) -> Option<TraceStatistics> {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut count = 0_u64;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
        sum += value;
        count += 1;
    }
    (count != 0).then_some(TraceStatistics {
        minimum,
        maximum,
        mean: sum / count as f64,
    })
}

/// The trace's colour as one more property row: its name in the label column,
/// and in the value column the swatch that opens the picker beside the hex
/// value it holds. Returns whether the reader picked a different colour.
fn trace_colour_row(ui: &mut Ui, color: &mut Color32) -> bool {
    use super::super::super::design_system::{property_row_control_columns, property_row_height};
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let trailing = panel_inset(ui);
    let (label_left, value_left) = property_row_control_columns(ui, width);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, property_row_height(ui)), Sense::hover());
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    ui.painter().text(
        Pos2::new(rect.left() + label_left, rect.center().y),
        Align2::LEFT_CENTER,
        "Colour",
        label_font,
        t.color.text_dim,
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(rect.left() + value_left, rect.top()),
        Pos2::new(rect.right() - trailing, rect.bottom()),
    );
    // The swatch is the row's field: as tall as a field in any other row, a
    // pixel in from the row's top and bottom, not a free-standing control
    // that would stand out of the row.
    let field = value_rect.shrink2(Vec2::new(0.0, 1.0));
    let swatch = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(field)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().interact_size.y = field.height();
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    color,
                    egui::color_picker::Alpha::Opaque,
                )
            },
        )
        .inner;
    // Placing the swatch left the cursor under it; the row ends at its own
    // bottom.
    let _ = ui.advance_cursor_after_rect(rect);
    let hex = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
    ui.painter().with_clip_rect(value_rect).text(
        Pos2::new(swatch.rect.right() + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        &hex,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Colour");
        node.set_value(hex.as_str());
    });
    swatch.changed()
}

fn set_selected_trace_color(
    state: &mut AppState,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
    color: Color32,
) {
    let color = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
    let Some(run_index) = state.model.simulation.view.active_run_idx else {
        return;
    };
    let Some((analysis_index, waveform_index)) = state
        .model
        .simulation
        .retained
        .runs
        .get(run_index)
        .and_then(|run| {
            selected
                .resolve(run)
                .map(|(analysis_index, waveform_index, _, _)| (analysis_index, waveform_index))
        })
    else {
        return;
    };
    if let Some(waveform) = state
        .model
        .simulation
        .retained
        .runs
        .get_mut(run_index)
        .and_then(|run| run.analyses.get_mut(analysis_index))
        .and_then(|analysis| analysis.waveforms.get_mut(waveform_index))
        .filter(|waveform| waveform.name == selected.source_name())
    {
        waveform.color.clone_from(&color);
    } else {
        return;
    }
    if state.model.simulation.view.active_analysis_idx == Some(analysis_index)
        && let Some(waveform) = state
            .model
            .simulation
            .view
            .waveforms
            .iter_mut()
            .find(|waveform| waveform.name == selected.source_name())
    {
        waveform.color = color;
    }
}

fn is_schematic_cross_probe_candidate(signal: &str) -> bool {
    (signal.starts_with("V(") || signal.starts_with("I(")) && signal.ends_with(')')
}

fn schematic_cross_probe_unavailability(
    state: &AppState,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
    signal: &str,
) -> Option<String> {
    let Some(net) = rspice_schematic_editor::view::cross_probe::wrapped_signal_name(signal, 'V')
    else {
        return Some(format!(
            "{signal} is a device current or derived quantity; no single schematic net carries it."
        ));
    };
    let map = &state.model.simulation.source.cross_probe;
    let topology = state.model.schematic.topology_version();
    if !map.is_current_for(&state.model.workspace.content.active_view, topology) {
        return Some(
            "The schematic changed since this result was produced; run again to cross-probe it."
                .to_owned(),
        );
    }
    let Some(run) = state.model.simulation.active_run() else {
        return Some("The selected result dataset is no longer retained.".to_owned());
    };
    if run.dataset_id != selected.dataset_id() || selected.resolve(run).is_none() {
        return Some("The selected trace does not belong to the open run.".to_owned());
    }
    let Some(receipt) = run.prepared_receipt() else {
        return Some(
            "This result does not record which project revision produced it, so it cannot be cross-probed."
                .to_owned(),
        );
    };
    if receipt.project_revision() != state.model.workspace.content.project.revision() {
        return Some(
            "The selected result was produced from a different project revision; run again before cross-probing it."
                .to_owned(),
        );
    }
    let here = state.model.workspace.content.occurrence_path(); // only that occurrence answers for a trace
    // A trace is scoped by written instance names, `xamp` for a cell drawn
    // `amp`, so its scope is compared with the drawn occurrence by those names
    // and named to the reader as the occurrence the run drew.
    let leaf = match crate::state::ProbeTarget::parse_legacy(net) {
        Ok(probe) if !probe.scope.same_engine_scope(&here) => {
            let deeper = probe.scope.engine_scope_starts_with(&here);
            let verb = if deeper { "Descend" } else { "Ascend" };
            let drawn = receipt
                .hierarchy_map()
                .iter()
                .filter_map(|row| crate::state::InstancePath::parse(row.occurrence()).ok())
                .collect::<Vec<_>>();
            let scope = probe.scope.resolve_drawn(&drawn).unwrap_or(&probe.scope);
            return Some(format!("{verb} to {scope} to cross-probe it."));
        }
        Ok(probe) => probe.leaf,
        Err(_) => net.to_owned(),
    };
    let mut nets = map
        .on_sheet(state.model.active_sheet_id())
        .net_to_points
        .iter();
    let known = nets.any(|(name, points)| name.eq_ignore_ascii_case(&leaf) && !points.is_empty());
    (!known).then(|| format!("The open sheet has no conductor named {leaf}."))
}

/// Resolve against the current schematic map before navigation. A stale or
/// derived signal therefore leaves the engineer in Results with an exact
/// explanation. On success, the Design surface opens; a remembered split
/// remains active and projects Design + the same canonical Results document.
fn cross_probe_trace_to_design(app: &mut RSpiceApp, signal: &str) -> Result<String, String> {
    let selected = app
        .state
        .results
        .results
        .valid_selected_trace(&app.state.model.simulation)
        .ok_or_else(|| "No result trace is selected.".to_owned())?;
    if let Some(reason) = schematic_cross_probe_unavailability(&app.state, selected, signal) {
        return Err(reason);
    }
    let net = app
        .state
        .model
        .select_signal_conductor(signal)
        .map_err(|error| error.message(signal))?;
    if app.state.workbench.workspace != Workspace::Design {
        Command::OpenWorkspace(Workspace::Design).execute(app);
    }
    Ok(net)
}

#[cfg(test)]
mod tests;
