//! Studio actions: fit, zoom, cursors, markers, and the integrity scan.
//!
//! Every action derives its result from the retained samples rather than from
//! what is drawn — a fit uses the exact extrema of the bound data, and a
//! cursor or marker is placed at a real sample index. When the data cannot
//! support an action the reason is reported instead of the action silently
//! doing nothing, which is why fit has an explicit block reason.

use super::*;

pub(super) fn source_integrity_scan_binding(state: &AppState) -> Option<(DatasetId, u64, usize)> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let total = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum::<usize>();
    (total > 0).then_some((run.dataset_id, analysis.id, total))
}

pub(super) fn start_source_integrity_scan(app: &mut RSpiceApp) {
    let Some((dataset_id, analysis_sequence, total)) = source_integrity_scan_binding(&app.state)
    else {
        app.state.push_user_message(ConsoleMessage::warning(
            "An exact retained analysis is required before an integrity scan can start.",
        ));
        return;
    };
    let studio = &mut app.state.workbench.visualization_studio;
    studio.operation_state = OperationState::Running;
    studio.operation_dataset_id = Some(dataset_id);
    studio.operation_analysis_sequence = Some(analysis_sequence);
    studio.operation_processed = 0;
    studio.operation_total = total;
    studio.operation_checksum = 0xcbf2_9ce4_8422_2325;
}

pub(super) fn advance_source_integrity_scan(app: &mut RSpiceApp) -> Result<(), String> {
    let studio = &app.state.workbench.visualization_studio;
    let dataset_id = studio
        .operation_dataset_id
        .ok_or_else(|| "The source-integrity scan has no bound dataset".to_owned())?;
    let analysis_sequence = studio
        .operation_analysis_sequence
        .ok_or_else(|| "The source-integrity scan has no bound analysis".to_owned())?;
    let processed = studio.operation_processed;
    let retained_total = studio.operation_total;
    let mut checksum = studio.operation_checksum;

    let run = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The integrity-scan dataset is no longer retained".to_owned())?;
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.id == analysis_sequence)
        .ok_or_else(|| "The integrity-scan analysis is no longer retained".to_owned())?;
    let actual_total = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum::<usize>();
    if actual_total != retained_total {
        return Err(
            "The immutable source sample count changed during the integrity scan".to_owned(),
        );
    }
    if processed >= retained_total {
        return Err("The source-integrity scan is already complete".to_owned());
    }

    let chunk = retained_total.div_ceil(3).max(1);
    let next = processed.saturating_add(chunk).min(retained_total);
    for (&x, &y) in analysis
        .waveforms
        .iter()
        .flat_map(|waveform| waveform.x.iter().zip(waveform.y.iter()))
        .skip(processed)
        .take(next - processed)
    {
        checksum ^= x.to_bits();
        checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
        checksum ^= y.to_bits();
        checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
    }

    let studio = &mut app.state.workbench.visualization_studio;
    studio.operation_processed = next;
    studio.operation_checksum = checksum;
    if next == retained_total {
        studio.operation_state = OperationState::Completed;
    }
    Ok(())
}

pub(super) fn recover_source_integrity_scan(app: &mut RSpiceApp) -> Result<(), String> {
    let studio = &app.state.workbench.visualization_studio;
    let dataset_id = studio
        .operation_dataset_id
        .ok_or_else(|| "The cancelled integrity scan has no bound dataset".to_owned())?;
    let analysis_sequence = studio
        .operation_analysis_sequence
        .ok_or_else(|| "The cancelled integrity scan has no bound analysis".to_owned())?;
    if studio.operation_processed >= studio.operation_total {
        return Err("A completed integrity scan cannot be recovered".to_owned());
    }
    let binding_exists = app.state.simulation.runs.iter().any(|run| {
        run.dataset_id == dataset_id
            && run
                .analyses
                .iter()
                .any(|analysis| analysis.id == analysis_sequence)
    });
    if !binding_exists {
        return Err("The cancelled integrity scan's immutable source is unavailable".to_owned());
    }
    app.state.workbench.visualization_studio.operation_state = OperationState::Running;
    Ok(())
}

pub(super) fn actions_sheet(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, app.state.workbench.visualization_studio.section);
    ScrollArea::vertical()
        .id_salt("visualization.actions-sheet")
        .show(ui, |ui| {
            let actions = [
                ("Add visualization pane", VisualizationDock::AddPane),
                ("Trace manager", VisualizationDock::TraceManager),
                ("Cursor manager", VisualizationDock::CursorManager),
                ("Document properties", VisualizationDock::DocumentProperties),
                ("Export document", VisualizationDock::Export),
            ];
            for (label, dock) in actions {
                if ui
                    .add_sized([ui.available_width(), 44.0], egui::Button::new(label))
                    .clicked()
                {
                    open_dock(app, dock);
                    app.state.workbench.visualization_studio.touch_pane =
                        VisualizationTouchPane::Stage;
                }
            }
        });
}

pub(super) fn add_cursor_at_midpoint(app: &mut RSpiceApp) {
    let midpoint = app
        .state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.waveforms.iter().find(|waveform| waveform.visible))
        .and_then(|waveform| {
            let count = waveform.x.len().min(waveform.y.len());
            (count > 0).then(|| waveform.x[count / 2])
        });
    if let Some(x) = midpoint {
        app.state.ui.results.cursors.place(x);
        app.state.ui.results.cursor_strip = app.state.simulation.active_analysis_idx;
    } else {
        app.state.push_user_message(ConsoleMessage::warning(
            "An exact source waveform is required before a cursor can be placed.",
        ));
    }
}

pub(super) fn fit_active_view(app: &mut RSpiceApp) {
    if let Some(reason) = fit_block_reason(&app.state) {
        app.state.push_user_message(ConsoleMessage::warning(reason));
        return;
    }

    let viewer = app.state.ui.results.viewer;
    let strip = app.state.simulation.active_analysis_idx.unwrap_or_default();
    match app.state.workbench.visualization_studio.autoscale {
        VisualizationAutoscale::RobustVisible => {
            app.state.ui.results.reset_plot_view(viewer, strip);
        }
        VisualizationAutoscale::ExactExtrema => {
            let (x, y) = exact_extrema_fit(&app.state)
                .expect("fit availability guarantees finite exact waveform extrema");
            let view = app.state.ui.results.plot_view_mut(viewer, strip);
            view.x = Some(x);
            view.y = Some(y);
        }
        VisualizationAutoscale::SpecificationBounds => {
            let (x, y) = specification_bound_fit(&app.state)
                .expect("fit availability guarantees exact specification bounds");
            let view = app.state.ui.results.plot_view_mut(viewer, strip);
            view.x = Some(x);
            view.y = Some(y);
        }
    }
    app.state.workbench.visualization_studio.zoom = 1.0;
}

pub(super) fn normalize_fit_policy_for_renderer(
    autoscale: &mut VisualizationAutoscale,
    viewer: ResultViewer,
) -> bool {
    if viewer != ResultViewer::Waves && *autoscale == VisualizationAutoscale::ExactExtrema {
        *autoscale = VisualizationAutoscale::RobustVisible;
        true
    } else {
        false
    }
}

pub(super) fn fit_block_reason(state: &AppState) -> Option<&'static str> {
    match state.workbench.visualization_studio.autoscale {
        VisualizationAutoscale::RobustVisible => None,
        VisualizationAutoscale::ExactExtrema if state.ui.results.viewer != ResultViewer::Waves => {
            Some("Exact-extrema fitting is available only for the waveform renderer.")
        }
        VisualizationAutoscale::ExactExtrema if exact_extrema_fit(state).is_none() => Some(
            "Exact-extrema fitting requires at least one visible waveform with finite samples.",
        ),
        VisualizationAutoscale::ExactExtrema => None,
        VisualizationAutoscale::SpecificationBounds
            if state.ui.results.viewer != ResultViewer::Waves =>
        {
            Some("Specification-bound fitting is available only for the waveform renderer.")
        }
        VisualizationAutoscale::SpecificationBounds if specification_bound_fit(state).is_none() => {
            Some(
                "Specification-bound fitting requires a visible waveform whose exact quantity name matches a configured project specification.",
            )
        }
        VisualizationAutoscale::SpecificationBounds => None,
    }
}

pub(super) fn exact_extrema_fit(state: &AppState) -> Option<((f64, f64), (f64, f64))> {
    if state.ui.results.viewer != ResultViewer::Waves {
        return None;
    }
    let analysis = state.simulation.active_analysis()?;
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    for waveform in analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
    {
        for &x in waveform.x.iter() {
            if x.is_finite() {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }
        for &y in waveform.y.iter() {
            if y.is_finite() {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
    }
    (x_min.is_finite() && x_max.is_finite() && y_min.is_finite() && y_max.is_finite()).then_some((
        nondegenerate_range(x_min, x_max),
        nondegenerate_range(y_min, y_max),
    ))
}

pub(super) fn nondegenerate_range(minimum: f64, maximum: f64) -> (f64, f64) {
    if minimum < maximum {
        return (minimum, maximum);
    }
    let padding = (minimum.abs() * 1.0e-9).max(1.0e-12);
    (minimum - padding, maximum + padding)
}

/// Resolve an exact waveform-to-specification binding and return the finite
/// coordinate envelope needed to make both the selected data and every
/// applicable limit visible. Matching is deliberately limited to the stable,
/// case-insensitive measurement/quantity name contract; fuzzy labels or unit
/// coercion could silently apply an unrelated engineering limit.
pub(super) fn specification_bound_fit(state: &AppState) -> Option<((f64, f64), (f64, f64))> {
    if state.ui.results.viewer != ResultViewer::Waves {
        return None;
    }
    let analysis = state.simulation.active_analysis()?;
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    let mut matched_bound = false;

    for waveform in analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
    {
        let matching_specs = state.workspace.specs.iter().filter(|spec| {
            spec.measurement.eq_ignore_ascii_case(&waveform.name)
                && (spec.min.is_some() || spec.max.is_some())
        });
        for spec in matching_specs {
            if let Some(minimum) = spec.min.filter(|value| value.is_finite()) {
                matched_bound = true;
                y_min = y_min.min(minimum);
                y_max = y_max.max(minimum);
            }
            if let Some(maximum) = spec.max.filter(|value| value.is_finite()) {
                matched_bound = true;
                y_min = y_min.min(maximum);
                y_max = y_max.max(maximum);
            }
        }
        for &x in waveform.x.iter() {
            if x.is_finite() {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }
        for &y in waveform.y.iter() {
            if y.is_finite() {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
    }

    (matched_bound
        && x_min.is_finite()
        && x_max.is_finite()
        && y_min.is_finite()
        && y_max.is_finite())
    .then_some((
        nondegenerate_range(x_min, x_max),
        nondegenerate_range(y_min, y_max),
    ))
}

pub(super) fn zoom_active(app: &mut RSpiceApp, factor: f32) {
    let next_zoom = (app.state.workbench.visualization_studio.zoom * factor).clamp(0.25, 8.0);
    let Some(analysis) = app.state.simulation.active_analysis() else {
        return;
    };
    let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
        return;
    };
    let (x_min, x_max) = waveform.x_range();
    let (y_min, y_max) = waveform.y_range();
    if !x_min.is_finite() || !x_max.is_finite() || !y_min.is_finite() || !y_max.is_finite() {
        return;
    }
    let scale = f64::from(1.0 / next_zoom);
    let x_mid = (x_min + x_max) * 0.5;
    let y_mid = (y_min + y_max) * 0.5;
    let viewer = app.state.ui.results.viewer;
    let strip = app.state.simulation.active_analysis_idx.unwrap_or_default();
    let view = app.state.ui.results.plot_view_mut(viewer, strip);
    view.x = Some((
        x_mid - (x_max - x_min) * 0.5 * scale,
        x_mid + (x_max - x_min) * 0.5 * scale,
    ));
    view.y = Some((
        y_mid - (y_max - y_min) * 0.5 * scale,
        y_mid + (y_max - y_min) * 0.5 * scale,
    ));
    app.state.workbench.visualization_studio.zoom = next_zoom;
}

pub(super) fn add_marker_at_midpoint(app: &mut RSpiceApp) {
    let Some((dataset_id, analysis_sequence, waveform_name, sample_index, x, y)) =
        source_midpoint(&app.state)
    else {
        app.state.push_user_message(ConsoleMessage::warning(
            "An exact source waveform is required before a marker can be created.",
        ));
        return;
    };
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Visualization marker identity space is exhausted".to_owned())?;
        studio.markers.push(VisualizationMarker {
            id,
            dataset_id,
            analysis_sequence,
            waveform_name,
            sample_index,
            x,
            y,
            label: format!("M{id}"),
        });
        Ok(())
    });
    report_visualization_commit(app, result);
}

pub(super) fn source_midpoint(
    state: &AppState,
) -> Option<(DatasetId, u64, String, usize, f64, f64)> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.visible)?;
    let count = waveform.x.len().min(waveform.y.len());
    if count == 0 {
        return None;
    }
    let index = count / 2;
    Some((
        run.dataset_id,
        analysis.id,
        waveform.name.clone(),
        index,
        waveform.x[index],
        waveform.y[index],
    ))
}

pub(super) fn apply_lod_policy(app: &mut RSpiceApp) {
    let policy = app.state.workbench.visualization_studio.display_lod;
    let index = match policy {
        DisplayLodPolicy::EnvelopePreserving => 0,
        DisplayLodPolicy::UniformSampling => 1,
        DisplayLodPolicy::ExactVisibleSamples => 2,
    };
    if let Err(error) = app
        .state
        .ui
        .preferences
        .set_choice(ChoicePreference::LargeDatasetDisplay, index)
    {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
}

pub(super) fn show_dock_if_open(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    let Some(dock) = app.state.workbench.visualization_studio.dock else {
        return;
    };
    let mut window_open = true;
    let mut close_requested = false;
    let viewport_width = ui.ctx().content_rect().width();
    let (dock_width, compact_body_max_width) = if compact {
        compact_dock_geometry(viewport_width)
    } else {
        (460.0, 0.0)
    };
    egui::Window::new(dock.title())
        .id(Id::new(("visualization.dock", dock as u8)))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(!compact)
        .default_width(dock_width)
        .min_width(if compact {
            dock_width.min(240.0)
        } else {
            300.0
        })
        .max_width(if compact { dock_width } else { 620.0 })
        .show(ui.ctx(), |ui| {
            if compact {
                // The window frame consumes part of `dock_width`. Advertising
                // the desktop 520 px body width here allowed long labels to
                // widen the window beyond a phone viewport and clipped both
                // content and the title-bar close control.
                ui.set_max_width(compact_body_max_width);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            } else {
                ui.set_max_width(620.0);
            }
            close_requested = dock_body(ui, app, dock);
        });
    if !window_open || close_requested {
        app.state.workbench.visualization_studio.dock = None;
    }
}
