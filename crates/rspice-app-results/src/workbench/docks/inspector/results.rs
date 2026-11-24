//! The Results workspace's inspector, which also reads a trace selected in the
//! split beside Design: the active pane and its axes, the selected trace or
//! artifact, and the run it came from.

use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{Problem, panel_inset, section_block_gap, section_header};

use egui::{Align2, Color32, Pos2, Rect, Sense, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::Model;
use crate::workbench::docks::RunDetailsDialogState;
use crate::workbench::documents::result_document::analysis_evidence_failure;
use crate::workbench::documents::result_document::manifest::active_manifest;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Execute;
use crate::workbench::framework::requests::PromptAfterCommands;
use rspice_results_ui::session::PaneAxis;

use super::super::super::design_system::{
    WorkbenchIcon, property_row, property_row_input_action, property_row_problem,
};
use super::super::super::state::Workspace;
use super::{result_authority, run_details};
use rspice_results::number_format::fmt_si_significant;
use rspice_schematic_editor::view::cross_probe::probed_device;

pub(in crate::workbench) fn results(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_details: &mut Option<Box<RunDetailsDialogState>>,
) {
    let selected_trace = state
        .results
        .valid_selected_trace(&state.model.simulation)
        .cloned();
    let selected_artifact = state
        .results
        .session
        .selected_result_artifact
        .clone()
        .filter(|key| key.resolve(&state.model.simulation.retained.runs).is_some());

    // What is being read comes before where it came from: a reader adjusting
    // a pane needs its axes and bindings first, and the dataset provenance
    // below is unchanged by any of that.
    active_result_pane(ui, state, selected_trace.as_ref());
    if let Some(selected) = selected_trace.as_ref() {
        selected_result_trace(ui, state, selected);
    } else if let Some(selected) = selected_artifact.as_ref() {
        selected_result_artifact(ui, state.results_reads(), selected);
    }

    // The workspace's memoized projection: taking it fresh re-digested the run.
    let manifest = active_manifest(state);
    let details = match state.model.simulation.active_run().zip(manifest.as_deref()) {
        Some((run, manifest)) => result_authority::result_run(
            ui,
            run,
            &manifest.model,
            &state.model.workspace.content.configuration_sets,
        ),
        None => {
            muted_inspector_copy(ui, "No run selected.");
            None
        }
    };
    if let Some(run_id) = details {
        run_details::open(run_details, &state.model.simulation, run_id);
    }
    saved_outputs(ui, state);

    // The active viewer owns its engineering readout (measurements, margins,
    // harmonics, eye metrics, distribution statistics, and so on). It follows
    // the trace and pane controls, matching the upgraded mockup's hierarchy
    // without creating a second result-data owner here.
    crate::workbench::documents::result_document::right_panel(ui, state);

    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    if let Some(index) = state.model.simulation.view.active_analysis_idx
        && let Some(analysis) = run.analyses.get(index)
    {
        result_authority::result_convergence(ui, analysis);
        result_authority::result_dc_sweep(ui, analysis);
    }
}

/// The active analysis's saved outputs, under the run that holds them, and the
/// materialization the reader asked for from them. A deferred output's
/// samples are evaluated from the run's retained data on request; the log
/// says what came of it.
fn saved_outputs(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let Some(run_id) = state.model.simulation.active_run().map(|run| run.run_id) else {
        return;
    };
    let Some(analysis) = state.model.simulation.active_analysis() else {
        return;
    };
    let analysis_id = analysis.id;
    let Some(receipt_index) = result_authority::saved_outputs(ui, &analysis.saved_output_receipts)
    else {
        return;
    };
    match state.model.simulation.materialize_deferred_saved_output(
        run_id,
        analysis_id,
        receipt_index,
    ) {
        Ok(()) => {
            state
                .analysis
                .synchronize_specialized_viewer_cache_authority(state.model);
            state.record(
                crate::diagnostics::LogSeverity::Info,
                crate::diagnostics::LogSource::Studio,
                "Materialized the deferred saved output from the retained run data.",
            );
        }
        Err(error) => {
            state.record(
                crate::diagnostics::LogSeverity::Error,
                crate::diagnostics::LogSource::Studio,
                format!("Saved output could not be materialized: {error}"),
            );
        }
    }
}

fn selected_result_artifact(
    ui: &mut Ui,
    state: ResultsReads<'_>,
    selected: &rspice_results::result_presentation::ResultArtifactPresentationKey,
) {
    let Some((run_index, _, analysis)) = selected.resolve(&state.model.simulation.retained.runs)
    else {
        return;
    };
    let run = &state.model.simulation.retained.runs[run_index];
    // Memoized per dataset generation: the validator walks every retained
    // sample. Intact data is the normal state and says nothing.
    let corrupted = analysis_evidence_failure(state, run.dataset_id, analysis);
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
    state: &mut ResultsParts<'_>,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
) {
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    let Some((analysis_index, waveform_index, analysis, waveform)) = selected.resolve(run) else {
        return;
    };

    let waveform_name = waveform.name.clone();
    let waveform_color = waveform.color.clone();
    let waveform_visible = crate::workbench::documents::result_document::waveform_plotted(
        state.model,
        state.results,
        &rspice_results::result_presentation::SourceWaveformPresentationKey::new(
            selected.analysis_key(),
            waveform.name.clone(),
        ),
        &waveform.name,
        waveform.visible,
    );
    // Computed once per trace and data version; a stored trace is read off
    // the frame, and its rows wait for it.
    let statistics =
        crate::workbench::documents::result_document::trace_statistics::trace_statistics(
            state.results_reads(),
            selected.analysis_key(),
            waveform_index,
            waveform,
            ui.ctx(),
        );
    // The statistics are measurements of the trace, read in the trace's own
    // unit to the reader's significant digits: the measurement rows below
    // them read to those, and one panel holds one precision.
    let unit = crate::workbench::documents::result_document::browser_signal_unit(
        &waveform.name,
        waveform.unit.as_deref(),
        crate::workbench::documents::result_document::analysis_default_unit(analysis.analysis_type),
    )
    .to_owned();
    let digits = usize::from(
        state
            .results_reads()
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let measured = |value: f64| fmt_si_significant(value, &unit, digits);

    section_header(ui, "Selected trace", None);
    property_row(ui, "Name", &waveform_name);
    match statistics {
        Some(statistics) => {
            // Samples the solver could not give a finite value are a problem
            // with the trace; a complete trace says nothing about its sample
            // count.
            if statistics.finite < statistics.samples {
                property_row_problem(
                    ui,
                    "Finite samples",
                    &format!("{} of {}", statistics.finite, statistics.samples),
                    Some(Problem::Warning),
                );
            }
            if let Some((minimum, maximum)) = statistics.extremes {
                property_row(ui, "Minimum", &measured(minimum));
                property_row(ui, "Maximum", &measured(maximum));
                // The average over X, as `avg()` takes it: a mean of the
                // samples leans toward wherever the solver stepped finely.
                let average = match &statistics.average {
                    Ok(average) => measured(*average),
                    Err(reason) => reason.clone(),
                };
                property_row(ui, "Average", &average);
                property_row(ui, "Peak-to-peak", &measured(maximum - minimum));
            }
        }
        None => {
            for label in ["Minimum", "Maximum", "Average", "Peak-to-peak"] {
                property_row(ui, label, "Computing\u{2026}");
            }
        }
    }

    // The swatch is the curve's colour as the strip drew it, through the one
    // lookup the browser's rows read theirs by: the strip gives curves their
    // palette places in the order they were plotted, not by column.
    let t = Tokens::get(ui.ctx());
    let mut color =
        crate::workbench::documents::result_document::browser_plot::plotted_signal_color(
            crate::workbench::documents::result_document::browser_plot::last_strip_models(state),
            &t.color.traces,
            selected.analysis_key(),
            waveform_index,
            &waveform_color,
        );
    if trace_colour_row(ui, &mut color) {
        let chosen = crate::results::visualization_document::TraceColor::from_rgb(
            color.r(),
            color.g(),
            color.b(),
        );
        if let Err(error) = crate::workbench::documents::result_document::set_trace_color(
            state,
            selected.analysis_key(),
            selected.source_name(),
            Some(chosen),
        ) {
            state.prompt(error);
        }
    }
    trace_style_rows(ui, state, selected, &waveform_name);

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
                state,
                analysis_index,
                waveform_index,
            );
        }

        if is_schematic_cross_probe_candidate(selected.source_name()) {
            let signal = selected.source_name().to_owned();
            let device = probed_device(&signal).is_some();
            let unavailable = schematic_cross_probe_unavailability(state.model, selected, &signal);
            let mut response = ui.add_enabled(
                unavailable.is_none(),
                egui::Button::new(if device {
                    "Cross-probe part"
                } else {
                    "Cross-probe net"
                }),
            );
            if let Some(reason) = unavailable {
                response = response.on_hover_text(reason);
            }
            if response.clicked() {
                match cross_probe_trace_to_design(state, &signal) {
                    Ok(name) => state.workbench.outbox.push(PromptAfterCommands(if device {
                        format!("Selected {name} from {signal}.")
                    } else {
                        format!("Selected conductor {name} from {signal}.")
                    })),
                    Err(message) => state.prompt(message),
                }
            }
        }
    });
}

fn active_result_pane(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    selected: Option<&rspice_results_ui::selection::SelectedResultTrace>,
) {
    let viewer = state.results.session.viewer;
    // A pure evidence table — OP, specs, samples, events, the manifest — has
    // no drawn pane at all. Reporting one would have the inspector state a
    // view, a fit and a limit mask for a sheet that has none of them.
    if !crate::workbench::documents::result_document::viewer_draws_a_pane(viewer) {
        return;
    }
    // The SOA sheet's one pane with a viewport is the stress trace. With it
    // closed the sheet draws the device's locus, whose axes the reader does
    // not set, so there is no range to state.
    if viewer == rspice_results::result_presentation::ResultViewer::Soa
        && !state.results.session.soa_stress_trace_open
    {
        return;
    }
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    let analysis_index = selected
        .and_then(|trace| {
            trace
                .resolve(run)
                .map(|(analysis_index, _, _, _)| analysis_index)
        })
        .or(state.model.simulation.view.active_analysis_idx)
        .unwrap_or(0);
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let pane_label = analysis.label.clone();
    // The unit names the pane in a unit-scoped stack; the viewer name is
    // already on the sheet tab above.
    let tokens = Tokens::get(ui.ctx());
    let facts = crate::workbench::documents::result_document::active_pane_facts(&tokens, state);
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
    // A distribution's abscissa is its measurements, a slot each: there is
    // no interval to set on it, so only its ordinate takes one.
    if viewer != rspice_results::result_presentation::ResultViewer::BoxViolin {
        axis_limit_row(ui, state, &facts, PaneAxis::X, "X range");
    }
    axis_limit_row(ui, state, &facts, PaneAxis::Y, "Y range");
}

/// An explicit interval for one axis of the active pane, or automatic fit.
///
/// The plot's own gestures can only ever say "a bit more than that". Holding
/// a decade while a parameter sweeps, or putting two runs on identical axes
/// so their curves can be compared at all, needs the numbers themselves.
fn axis_limit_row(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    facts: &crate::workbench::documents::result_document::ActivePaneFacts,
    axis: PaneAxis,
    label: &str,
) {
    use crate::workbench::documents::result_document::axis_range::{
        format_axis_range, parse_axis_range, shorter_axis_ranges,
    };
    use crate::workbench::documents::result_document::{
        active_axis_is_pinned, active_axis_range, set_active_axis_range,
    };

    let viewer = state.results.session.viewer;
    let current = active_axis_range(state.results_reads(), facts, axis);
    let pinned = active_axis_is_pinned(state.results_reads(), axis);
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
    let mut text = match state.results.session.axis_limit_draft.as_ref() {
        Some((draft_viewer, draft_axis, text))
            if *draft_viewer == viewer && *draft_axis == axis =>
        {
            text.clone()
        }
        _ => committed.clone(),
    };

    let parsed = parse_axis_range(&text, unit);
    let invalid = !text.trim().is_empty() && parsed.is_none();
    // A field too narrow for the interval shows it to fewer digits at rest,
    // never cut through its own ellipsis; a draft shows as typed.
    let shorter = current
        .filter(|_| text == committed)
        .map(|range| shorter_axis_ranges(range, unit))
        .unwrap_or_default();
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
        &shorter,
    );

    if edit.changed() {
        state.results.session.axis_limit_draft = Some((viewer, axis, text.clone()));
    }
    // Commit on Enter or on leaving the field, never per keystroke: "1.2" is
    // a legal prefix of "1.2m" and three orders of magnitude away from it.
    if edit.lost_focus() {
        let tokens = Tokens::get(ui.ctx());
        if text == committed {
            // The field shows the interval to six significant digits. Read
            // back unedited, that would pin an autofitting axis, or nudge a
            // pinned one, by the rounding alone.
            state.results.session.axis_limit_draft = None;
        } else if text.trim().is_empty() {
            set_active_axis_range(&tokens, state, axis, None);
            state.results.session.axis_limit_draft = None;
        } else if let Some(range) = parse_axis_range(&text, unit) {
            set_active_axis_range(&tokens, state, axis, Some(range));
            state.results.session.axis_limit_draft = None;
        }
    }
    if reset.clicked() {
        let tokens = Tokens::get(ui.ctx());
        set_active_axis_range(&tokens, state, axis, None);
        state.results.session.axis_limit_draft = None;
    }
}

/// The trace's colour as one more property row: its name in the label column,
/// and in the value column the swatch that opens the picker beside the hex
/// value it holds. Returns whether the reader picked a different colour.
/// The selected trace's name and style rows. The style is the view's, and
/// one edit styles every trace Ctrl-clicked beside the selected one with
/// it; the name is the selected trace's own.
fn trace_style_rows(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
    signal: &str,
) {
    use super::super::super::design_system::{property_row_combo, property_row_input_with_hint};
    use crate::results::visualization_document::{
        TraceDash, TraceDraw, TraceStyle, TraceSymbol, TraceWidth,
    };
    let analysis = selected.analysis_key();
    let together: Vec<String> = std::iter::once(signal.to_owned())
        .chain(
            state
                .results
                .session
                .also_selected_traces
                .iter()
                .filter(|held| held.analysis_key() == analysis && held.source_name() != signal)
                .map(|held| held.source_name().to_owned()),
        )
        .collect();
    let key =
        rspice_results::result_presentation::SourceWaveformPresentationKey::new(analysis, signal);
    let look = state
        .results
        .session
        .waveform_looks
        .get(&key)
        .cloned()
        .unwrap_or_default();

    // The name is typed into a draft the field keeps until Enter or a click
    // away, so the view is renamed once, not on every key.
    let draft_id = egui::Id::new(("rspice.inspector.trace-name", analysis, signal));
    let mut draft = ui
        .data(|data| data.get_temp::<String>(draft_id))
        .unwrap_or_else(|| look.name.clone().unwrap_or_default());
    let field = property_row_input_with_hint(ui, "Shown as", &mut draft, signal, false);
    if field.lost_focus() {
        ui.data_mut(|data| data.remove::<String>(draft_id));
        if draft.trim() != look.name.as_deref().unwrap_or_default()
            && let Err(error) = crate::workbench::documents::result_document::rename_trace(
                state, analysis, signal, &draft,
            )
        {
            state.prompt(error);
        }
    } else if field.has_focus() {
        ui.data_mut(|data| data.insert_temp(draft_id, draft));
    }

    if together.len() > 1 {
        property_row(ui, "Styled together", &format!("{} traces", together.len()));
    }
    let automatic = ("automatic".to_owned(), "Automatic".to_owned());
    let options = |labels: &[&'static str]| -> Vec<(String, String)> {
        std::iter::once(automatic.clone())
            .chain(
                labels
                    .iter()
                    .map(|label| ((*label).to_owned(), (*label).to_owned())),
            )
            .collect()
    };
    let pick = |selected: &str, labels: &[&'static str]| {
        labels.iter().position(|label| *label == selected)
    };
    let mut style = look.style;
    let mut changed = false;

    let widths = TraceWidth::ALL.map(TraceWidth::label);
    let mut width = style
        .width
        .map_or("automatic", TraceWidth::label)
        .to_owned();
    if property_row_combo(
        ui,
        "Width",
        ("trace-width", signal),
        &mut width,
        &options(&widths),
        true,
    ) {
        style.width = pick(&width, &widths).map(|at| TraceWidth::ALL[at]);
        changed = true;
    }
    let dashes = TraceDash::ALL.map(TraceDash::label);
    let mut dash = style.dash.map_or("automatic", TraceDash::label).to_owned();
    if property_row_combo(
        ui,
        "Dash",
        ("trace-dash", signal),
        &mut dash,
        &options(&dashes),
        true,
    ) {
        style.dash = pick(&dash, &dashes).map(|at| TraceDash::ALL[at]);
        changed = true;
    }
    let symbols = TraceSymbol::ALL.map(TraceSymbol::label);
    let mut symbol = style
        .symbol
        .map_or("automatic", TraceSymbol::label)
        .to_owned();
    if property_row_combo(
        ui,
        "Symbols",
        ("trace-symbol", signal),
        &mut symbol,
        &options(&symbols),
        true,
    ) {
        style.symbol = pick(&symbol, &symbols).map(|at| TraceSymbol::ALL[at]);
        changed = true;
    }
    let draws = TraceDraw::ALL.map(TraceDraw::label);
    let mut draw = style.draw.label().to_owned();
    let draw_options: Vec<(String, String)> = draws
        .iter()
        .map(|label| ((*label).to_owned(), (*label).to_owned()))
        .collect();
    if property_row_combo(
        ui,
        "Draw as",
        ("trace-draw", signal),
        &mut draw,
        &draw_options,
        true,
    ) {
        style.draw = pick(&draw, &draws).map_or(TraceDraw::Line, |at| TraceDraw::ALL[at]);
        changed = true;
    }
    if changed && style != look.style {
        // Each trace styled together takes the part changed, and keeps the
        // parts it has of its own.
        for signal in &together {
            let own: TraceStyle = state
                .results
                .session
                .waveform_looks
                .get(
                    &rspice_results::result_presentation::SourceWaveformPresentationKey::new(
                        analysis,
                        signal.clone(),
                    ),
                )
                .map(|look| look.style)
                .unwrap_or_default();
            if let Err(error) = crate::workbench::documents::result_document::set_trace_style(
                state,
                analysis,
                signal,
                own.with_changes(look.style, style),
            ) {
                state.prompt(error);
            }
        }
    }
}

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
        "Color",
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
        node.set_label("Color");
        node.set_value(hex.as_str());
    });
    swatch.changed()
}

fn is_schematic_cross_probe_candidate(signal: &str) -> bool {
    (signal.starts_with("V(") && signal.ends_with(')')) || probed_device(signal).is_some()
}

fn schematic_cross_probe_unavailability(
    model: &Model,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
    signal: &str,
) -> Option<String> {
    let device = probed_device(signal).is_some();
    let net = rspice_schematic_editor::view::cross_probe::wrapped_signal_name(signal, 'V');
    if net.is_none() && !device {
        return Some(
            rspice_schematic_editor::view::cross_probe::LocateSignalError::NotANet.message(signal),
        );
    }
    if !model.result_mapping_is_current() {
        return Some(
            "The schematic changed since this result was produced; run again to cross-probe it."
                .to_owned(),
        );
    }
    let Some(run) = model.simulation.active_run() else {
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
    if receipt.project_revision() != model.workspace.content.project.revision() {
        return Some(
            "The selected result was produced from a different project revision; run again before cross-probing it."
                .to_owned(),
        );
    }
    let Some(net) = net else {
        // A current names its part; the part's own rules say whether this
        // drawing, at this occurrence, has it.
        return model
            .locate_signal_device(signal)
            .err()
            .map(|error| error.message(signal));
    };
    let here = model.workspace.content.occurrence_path(); // only that occurrence answers for a trace
    // A trace is scoped by written instance names, `xamp` for a cell drawn
    // `amp`, so its scope is compared with the drawn occurrence by those names
    // and named to the reader as the occurrence the run drew.
    let leaf = match crate::state::ProbeTarget::parse(net) {
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
    (!model.drawing_draws_net(&leaf))
        .then(|| format!("No sheet of this drawing has a conductor named {leaf}."))
}

/// Resolve against the current schematic map before navigation. A stale or
/// derived signal therefore leaves the engineer in Results with an exact
/// explanation. On success, the Design surface opens once the inspector
/// returns; a remembered split remains active and projects Design + the same
/// canonical Results document.
pub fn cross_probe_trace_to_design(
    state: &mut ResultsParts<'_>,
    signal: &str,
) -> Result<String, String> {
    let selected = state
        .results
        .valid_selected_trace(&state.model.simulation)
        .ok_or_else(|| "No result trace is selected.".to_owned())?;
    if let Some(reason) = schematic_cross_probe_unavailability(state.model, selected, signal) {
        return Err(reason);
    }
    if probed_device(signal).is_some() {
        let (component, position, name) = state
            .model
            .locate_signal_device(signal)
            .map_err(|error| error.message(signal))?;
        state.show_component_on_schematic(component, position);
        return Ok(name);
    }
    let net = state
        .model
        .select_signal_conductor(signal)
        .map_err(|error| error.message(signal))?;
    if state.workbench.workspace != Workspace::Design {
        state.workbench.outbox.push(Execute(ids::DESIGN_WORKSPACE));
    }
    Ok(net)
}

#[cfg(test)]
pub(crate) mod tests;
