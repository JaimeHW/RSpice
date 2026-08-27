//! The viewer stage, its exact-data dock, and the inspector beside it.
//!
//! The stage draws only what the active binding actually resolves to: when a
//! retained result does not satisfy a viewer's contract the stage says so and
//! draws nothing, rather than rendering a partial or substituted view. The
//! exact-data dock is the paired numeric view — it lists the retained values a
//! curve was drawn from, never resampled or interpolated ones.

use super::*;

pub(super) fn viewer_stage(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let selected = app
        .state
        .workbench
        .visualization_studio
        .selected_viewer_document
        .clone();
    let definition = viewer_document(&selected);
    let analysis_ids = available_analysis_ids(&app.state);
    let capabilities = ViewerCapabilities {
        analysis_ids: &analysis_ids,
        external_capabilities: &[],
    };
    let family_selection = active_family_sample_selection(app);
    let availability = active_binding_error(app).map_or_else(
        || {
            definition
                .ok_or_else(|| "Viewer identity is not registered".to_owned())
                .and_then(|definition| {
                    resolved_viewer_availability(&app.state, definition, capabilities)
                })
                .and_then(|viewer| {
                    family_selection
                        .as_ref()
                        .map_or_else(|error| Err(error.clone()), |_| Ok(viewer))
                })
        },
        Err,
    );
    let compatible = availability.is_ok();
    let header = Frame::NONE
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(9, 6))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                VIEWER_STAGE_HEADER_HEIGHT,
                VIEWER_STAGE_HEADER_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(definition.map_or("RESULT VIEWER", |meta| meta.domain))
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new(definition.map_or("Unknown viewer", |meta| meta.title))
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let source = app.state.simulation.active_run().map_or_else(
                        || "No active dataset".to_owned(),
                        |run| format!("{} · {}", run.label, short_dataset(run.dataset_id)),
                    );
                    status_label(
                        ui,
                        &source,
                        if availability.is_ok() {
                            t.color.ok
                        } else {
                            t.color.warn
                        },
                    );
                });
            });
        });
    paint_bottom_rule(ui, header.response.rect, t.color.border_strong);

    let dock_height = exact_rows_height();
    let plot_height = (ui.available_height() - dock_height - VIEWER_STAGE_STATUS_HEIGHT).max(80.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), plot_height),
        Layout::top_down(Align::Min),
        |ui| match availability {
            Ok(viewer) => {
                app.state.ui.results.viewer = viewer;
                let interaction = match app.state.workbench.visualization_studio.tool {
                    ViewerTool::Select => crate::ui::plot::InteractionMode::Select,
                    ViewerTool::Pan => crate::ui::plot::InteractionMode::Pan,
                    ViewerTool::Zoom => crate::ui::plot::InteractionMode::Zoom,
                };
                crate::ui::plot::set_interaction_mode(ui.ctx(), interaction);
                result_document::show_embedded_with_sample_selection(
                    ui,
                    app,
                    family_selection.as_ref().ok().cloned().flatten(),
                );
                capture_active_link_state(ui.ctx(), app);
                paint_visualization_markers(ui, app);
                crate::ui::plot::set_interaction_mode(
                    ui.ctx(),
                    crate::ui::plot::InteractionMode::All,
                );
            }
            Err(reason) => unavailable_viewer(ui, definition, &reason),
        },
    );
    viewer_stage_status(ui, app, compatible);
    exact_data_dock(ui, app);
}

/// Whether the sheet on screen draws the analysis's own retained sweep on its
/// horizontal axis.
///
/// A marker or annotation is anchored by an X coordinate taken from a
/// retained waveform — seconds on a transient, hertz on a noise sweep. The
/// overlay maps that coordinate onto whatever the well is currently showing,
/// so it must only draw where the well's horizontal axis is that same sweep.
/// The unit-pane stack is exactly that set; the derived sheets — the folded
/// eye, the binned histogram, the spectrum, the Smith and Nyquist charts —
/// compute their own abscissa, and a time marker placed on one of those is
/// drawn at a position that means nothing.
pub(super) fn marker_domain_matches_the_pane(viewer: ResultViewer) -> bool {
    result_document::viewer_uses_wave_stack(viewer)
}

pub(super) fn paint_visualization_markers(ui: &Ui, app: &mut RSpiceApp) {
    if !marker_domain_matches_the_pane(app.state.ui.results.viewer) {
        return;
    }
    let Some(well) = app.state.ui.results.well_rect else {
        return;
    };
    let Some(analysis_index) = app.state.simulation.active_analysis_idx else {
        return;
    };
    let source = app.state.simulation.active_run().and_then(|run| {
        let analysis = run.analyses.get(analysis_index)?;
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.visible)?;
        let (source_min, source_max) = waveform.x_range();
        Some((run.dataset_id, analysis.id, source_min, source_max))
    });
    let Some((dataset_id, analysis_sequence, source_min, source_max)) = source else {
        return;
    };
    let (x_min, x_max) = result_document::active_renderer_axis_range(
        ui.ctx(),
        &mut app.state,
        result_document::PaneAxis::X,
    )
    .unwrap_or((source_min, source_max));
    if !x_min.is_finite() || !x_max.is_finite() || x_min >= x_max {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let plot = well.shrink2(vec2(28.0, 22.0));
    for marker in app
        .state
        .workbench
        .visualization_studio
        .markers
        .iter()
        .filter(|marker| {
            marker.dataset_id == dataset_id && marker.analysis_sequence == analysis_sequence
        })
    {
        let fraction = ((marker.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) as f32;
        let x = egui::lerp(plot.x_range(), fraction);
        ui.painter()
            .vline(x, plot.y_range(), Stroke::new(1.0, t.color.accent));
        ui.painter().text(
            egui::pos2(x + 4.0, plot.top() + 3.0),
            egui::Align2::LEFT_TOP,
            &marker.label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            t.color.accent,
        );
    }
    for annotation in app
        .state
        .workbench
        .visualization_studio
        .annotations
        .iter()
        .filter(|annotation| {
            annotation.dataset_id == dataset_id && annotation.analysis_sequence == analysis_sequence
        })
    {
        let fraction = ((annotation.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) as f32;
        let x = egui::lerp(plot.x_range(), fraction);
        let anchor = egui::pos2(x, plot.bottom() - 8.0);
        ui.painter().circle_filled(anchor, 3.0, t.color.info);
        ui.painter().text(
            anchor + vec2(5.0, -1.0),
            egui::Align2::LEFT_CENTER,
            format!("NOTE-{}", annotation.id),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.info,
        );
    }
}

pub(super) fn active_binding_error(app: &RSpiceApp) -> Option<String> {
    let studio = &app.state.workbench.visualization_studio;
    let active = studio.active_pane?;
    let pane = studio.panes.iter().find(|pane| pane.id == active)?;
    let Some(run) = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == pane.dataset_id)
    else {
        return Some(format!(
            "Bound dataset {} is no longer retained; the pane was not retargeted",
            short_dataset(pane.dataset_id)
        ));
    };
    (!run
        .analyses
        .iter()
        .any(|analysis| analysis.id == pane.analysis_sequence))
    .then(|| {
        format!(
            "Bound analysis {} is no longer retained in dataset {}; the pane was not retargeted",
            pane.analysis_sequence,
            short_dataset(pane.dataset_id)
        )
    })
}

pub(super) fn unavailable_viewer(
    ui: &mut Ui,
    definition: Option<&ViewerDocumentDefinition>,
    reason: &str,
) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect.shrink(24.0)), |ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                let (icon, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                WorkbenchIcon::Warning.paint(ui.painter(), icon, t.color.warn);
                ui.label(
                    RichText::new(format!(
                        "{} unavailable",
                        definition.map_or("Viewer", |meta| meta.title)
                    ))
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
                );
                ui.label(
                    RichText::new(reason)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.label(
                    RichText::new("No fallback viewer or fabricated data was substituted.")
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
            });
        });
    });
}

pub(super) fn viewer_stage_status(ui: &mut Ui, app: &RSpiceApp, compatible: bool) {
    let t = Tokens::get(ui.ctx());
    let status = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                VIEWER_STAGE_STATUS_HEIGHT,
                VIEWER_STAGE_STATUS_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Document  VIS-{:04} · revision {}",
                        1, app.state.workbench.visualization_studio.revision
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
                );
                ui.separator();
                ui.label(
                    RichText::new("Source  immutable result samples")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.separator();
                ui.label(
                    RichText::new("Interpolation  source exact on dock queries")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(if compatible {
                            "COMPATIBLE-RUNTIME"
                        } else {
                            "VIEWER-UNAVAILABLE"
                        })
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(if compatible {
                            t.color.ok
                        } else {
                            t.color.warn
                        }),
                    );
                });
            });
        });
    paint_top_rule(ui, status.response.rect, t.color.border_strong);
}

pub(super) const fn exact_rows_height() -> f32 {
    EXACT_DATA_DOCK_HEIGHT
}

pub(super) fn exact_data_dock(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let rows = exact_source_rows(&app.state);
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(8.0)
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            panel_heading(
                ui,
                "Exact-data dock",
                &format!("{} source rows · no display interpolation", rows.len()),
            );
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), EXACT_DATA_TABLE_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ScrollArea::both()
                        .id_salt("visualization.exact-data")
                        .show(ui, |ui| {
                            Grid::new("visualization.exact-data.grid")
                                .num_columns(5)
                                .striped(true)
                                .spacing(vec2(14.0, 5.0))
                                .show(ui, |ui| {
                                    table_header(ui, "Binding");
                                    table_header(ui, "Stable row");
                                    table_header(ui, "Typed coordinate");
                                    table_header(ui, "Exact f64 value");
                                    table_header(ui, "Origin");
                                    ui.end_row();
                                    if rows.is_empty() {
                                        ui.label(
                                            RichText::new("No exact source row is available")
                                                .color(t.color.warn),
                                        );
                                        for _ in 0..4 {
                                            ui.label("—");
                                        }
                                        ui.end_row();
                                    }
                                    for row in rows {
                                        ui.monospace(row.binding);
                                        ui.monospace(row.stable_row);
                                        ui.monospace(row.coordinate);
                                        ui.monospace(row.value);
                                        ui.label(row.origin);
                                        ui.end_row();
                                    }
                                });
                        });
                },
            );
        });
}

pub(super) struct ExactSourceRow {
    pub(super) binding: String,
    pub(super) stable_row: String,
    pub(super) coordinate: String,
    pub(super) value: String,
    pub(super) origin: String,
}

pub(super) fn exact_source_rows(state: &AppState) -> Vec<ExactSourceRow> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let Some(analysis_index) = state.simulation.active_analysis_idx else {
        return Vec::new();
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return Vec::new();
    };
    if let Some((poles, zeros, gain)) = retained_pole_zero_payload(analysis) {
        return exact_pole_zero_rows(run, analysis, poles, zeros, gain);
    }
    if let Some((output, result_mode, rows)) = retained_sensitivity_payload(analysis) {
        return exact_sensitivity_rows(run, analysis, output, result_mode, rows);
    }
    let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
        return Vec::new();
    };
    let count = waveform.x.len().min(waveform.y.len());
    if count == 0 {
        return Vec::new();
    }
    let mut indices = vec![0, count / 2, count - 1];
    for cursor in [state.ui.results.cursors.a, state.ui.results.cursors.b]
        .into_iter()
        .flatten()
    {
        if let Some(index) = waveform.x[..count]
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_finite())
            .min_by(|(_, left), (_, right)| {
                (*left - cursor).abs().total_cmp(&(*right - cursor).abs())
            })
            .map(|(index, _)| index)
        {
            indices.push(index);
        }
    }
    indices.sort_unstable();
    indices.dedup();
    indices
        .into_iter()
        .take(5)
        .map(|index| ExactSourceRow {
            binding: short_dataset(run.dataset_id),
            stable_row: format!("{}:{index}", analysis.id),
            coordinate: format!("x={:.17e}", waveform.x[index]),
            value: format!("{:.17e}", waveform.y[index]),
            origin: waveform.name.clone(),
        })
        .collect()
}

pub(super) fn retained_pole_zero_payload(
    analysis: &AnalysisResult,
) -> Option<(
    &[crate::state::ComplexResultValue],
    &[crate::state::ComplexResultValue],
    f64,
)> {
    if !analysis.success || analysis.analysis_type != AnalysisType::PoleZero {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    match payload {
        AnalysisResultPayload::PoleZero { poles, zeros, gain } => {
            Some((poles.as_slice(), zeros.as_slice(), *gain))
        }
        AnalysisResultPayload::OperatingPoint { .. }
        | AnalysisResultPayload::Sensitivity { .. }
        | AnalysisResultPayload::TransferFunction { .. }
        | AnalysisResultPayload::ScalarMeasurements { .. }
        | AnalysisResultPayload::Reliability { .. }
        | AnalysisResultPayload::Soa { .. }
        | AnalysisResultPayload::TransientEvents { .. } => None,
    }
}

pub(super) fn retained_sensitivity_payload(
    analysis: &AnalysisResult,
) -> Option<(&str, SensitivityResultMode, &[SensitivityResultRow])> {
    if !analysis.success || analysis.analysis_type != AnalysisType::Sensitivity {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    match payload {
        AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        } => Some((output.as_str(), *result_mode, rows.as_slice())),
        AnalysisResultPayload::OperatingPoint { .. }
        | AnalysisResultPayload::PoleZero { .. }
        | AnalysisResultPayload::TransferFunction { .. }
        | AnalysisResultPayload::ScalarMeasurements { .. }
        | AnalysisResultPayload::Reliability { .. }
        | AnalysisResultPayload::Soa { .. }
        | AnalysisResultPayload::TransientEvents { .. } => None,
    }
}

pub(super) fn exact_sensitivity_rows(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    output: &str,
    result_mode: SensitivityResultMode,
    rows: &[SensitivityResultRow],
) -> Vec<ExactSourceRow> {
    let basis = match result_mode {
        SensitivityResultMode::Dc => "dc".to_owned(),
        SensitivityResultMode::Ac { frequency_hz } => {
            format!("ac@{frequency_hz:.17e}Hz")
        }
    };
    let mut exact = Vec::with_capacity(rows.len() * 2);
    for (index, row) in rows.iter().enumerate() {
        for (quantity, value) in [("raw", row.raw), ("normalized", row.normalized)] {
            exact.push(ExactSourceRow {
                binding: short_dataset(run.dataset_id),
                stable_row: format!("{}:sensitivity[{index}].{quantity}", analysis.id),
                coordinate: format!("parameter={};basis={basis}", row.parameter),
                value: format!("{value:.17e}"),
                origin: output.to_owned(),
            });
        }
    }
    exact
}

pub(super) fn exact_pole_zero_rows(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    poles: &[crate::state::ComplexResultValue],
    zeros: &[crate::state::ComplexResultValue],
    gain: f64,
) -> Vec<ExactSourceRow> {
    let mut rows = Vec::with_capacity(1 + (poles.len() + zeros.len()) * 2);
    rows.push(ExactSourceRow {
        binding: short_dataset(run.dataset_id),
        stable_row: format!("{}:gain", analysis.id),
        coordinate: "scalar".to_owned(),
        value: format!("{gain:.17e}"),
        origin: "DC transfer gain".to_owned(),
    });
    for (kind, roots) in [("pole", poles), ("zero", zeros)] {
        for (index, root) in roots.iter().enumerate() {
            for (component, value) in [("real", root.real), ("imaginary", root.imaginary)] {
                rows.push(ExactSourceRow {
                    binding: short_dataset(run.dataset_id),
                    stable_row: format!("{}:{kind}[{index}].{component}", analysis.id),
                    coordinate: format!("{kind}[{index}].{component}"),
                    value: format!("{value:.17e}"),
                    origin: format!("ordered {kind} root"),
                });
            }
        }
    }
    rows
}

/// A dataset identity as a stage label spells it, elided by
/// [`crate::product::short_identity`] so it reads as the same prefix the
/// navigator and the specification band show for the same dataset.
pub(super) fn short_dataset(id: DatasetId) -> String {
    crate::product::short_identity(id)
}

pub(super) struct ResultEntityRow {
    pub(super) identity: String,
    pub(super) kind: &'static str,
    pub(super) binding: String,
    pub(super) state: String,
}

pub(super) fn result_entity_rows(state: &AppState) -> Vec<ResultEntityRow> {
    let mut rows = Vec::new();
    if let (Some(run), Some(analysis)) = (
        state.simulation.active_run(),
        state.simulation.active_analysis(),
    ) {
        let dataset = short_dataset(run.dataset_id);
        if let Some(waveform) = analysis.waveforms.first() {
            rows.push(ResultEntityRow {
                identity: format!("axis:{}:x", analysis.id),
                kind: "axis",
                binding: format!("{} · source X coordinate", dataset),
                state: format!("{} exact rows", waveform.x.len()),
            });
        }
        for (index, waveform) in analysis.waveforms.iter().enumerate() {
            rows.push(ResultEntityRow {
                identity: format!("trace:{}:{index}", analysis.id),
                kind: "trace",
                binding: format!("{} · {}", dataset, waveform.name),
                state: if waveform.visible {
                    "visible · source exact".to_owned()
                } else {
                    "hidden · retained".to_owned()
                },
            });
        }
    }
    for (label, coordinate) in [
        ("A", state.ui.results.cursors.a),
        ("B", state.ui.results.cursors.b),
    ] {
        if let Some(coordinate) = coordinate {
            rows.push(ResultEntityRow {
                identity: format!("cursor:{label}"),
                kind: "cursor",
                binding: format!("x={coordinate:.17e}"),
                state: "nearest source row".to_owned(),
            });
        }
    }
    for marker in &state.workbench.visualization_studio.markers {
        rows.push(ResultEntityRow {
            identity: format!("marker:{}", marker.id),
            kind: "marker",
            binding: format!(
                "{} · {}[{}]",
                short_dataset(marker.dataset_id),
                marker.waveform_name,
                marker.sample_index
            ),
            state: marker.label.clone(),
        });
    }
    for measurement in &state.workbench.visualization_studio.measurements {
        rows.push(ResultEntityRow {
            identity: format!("measurement:{}", measurement.id),
            kind: "measurement",
            binding: measurement.expression.clone(),
            state: format!("{:.17e}", measurement.value),
        });
    }
    for annotation in &state.workbench.visualization_studio.annotations {
        rows.push(ResultEntityRow {
            identity: format!("annotation:{}", annotation.id),
            kind: "annotation",
            binding: format!(
                "{} · analysis {} · x={:.9e}",
                short_dataset(annotation.dataset_id),
                annotation.analysis_sequence,
                annotation.x
            ),
            state: annotation.text.clone(),
        });
    }
    rows
}

pub(super) fn fixed_table_row<const N: usize>(
    ui: &mut Ui,
    fractions: [f32; N],
    cells: [&str; N],
    header: bool,
    minimum_width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 28.0 };
    let width = ui.available_width().max(minimum_width);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let color = if header {
        t.color.text_faint
    } else {
        t.color.text_dim
    };
    let mut left = rect.left();
    for index in 0..N {
        let right = if index + 1 == N {
            rect.right()
        } else {
            left + rect.width() * fractions[index]
        };
        let cell = Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        );
        if index + 1 < N {
            ui.painter().vline(
                right - 0.5,
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.painter().with_clip_rect(cell).text(
            egui::pos2(cell.left() + 8.0, cell.center().y),
            Align2::LEFT_CENTER,
            cells[index],
            font.clone(),
            color,
        );
        left = right;
    }
}

pub(super) fn result_entity_table(ui: &mut Ui, rows: &[ResultEntityRow]) {
    const FRACTIONS: [f32; 4] = [0.23, 0.16, 0.37, 0.24];
    ScrollArea::both()
        .id_salt("visualization.result-entities")
        .max_height(196.0)
        .show(ui, |ui| {
            fixed_table_row(
                ui,
                FRACTIONS,
                ["IDENTITY", "TYPE", "BINDING / DEFINITION", "STATE"],
                true,
                520.0,
            );
            if rows.is_empty() {
                fixed_table_row(
                    ui,
                    FRACTIONS,
                    ["—", "none", "No versioned result entities", "empty"],
                    false,
                    520.0,
                );
            }
            for row in rows {
                fixed_table_row(
                    ui,
                    FRACTIONS,
                    [&row.identity, row.kind, &row.binding, &row.state],
                    false,
                    520.0,
                );
            }
        });
}

pub(super) fn viewer_inspector(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let entities = result_entity_rows(&app.state);
    panel_heading(ui, "Versioned result entities", &entities.len().to_string());
    ScrollArea::vertical()
        .id_salt("visualization.entity-inspector")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            result_entity_table(ui, &entities);

            separator(ui, t.color.border);
            let latest_comparison = app
                .state
                .workbench
                .visualization_studio
                .comparison_receipts
                .last()
                .cloned();
            panel_heading(
                ui,
                "Comparison receipt",
                if latest_comparison.is_some() {
                    "current"
                } else {
                    "none"
                },
            );
            ui.label(
                RichText::new(
                    "Select explicit dataset alignment, units, interpolation, resampling, extrapolation, and precision before comparing immutable sources.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
            );
            if let Some(receipt) = latest_comparison {
                property_row(
                    ui,
                    "Datasets",
                    &format!(
                        "{} → {}",
                        short_dataset(receipt.baseline.dataset_id),
                        short_dataset(receipt.candidate.dataset_id)
                    ),
                );
                property_row(ui, "Rows", &receipt.rows_compared.to_string());
                property_row(ui, "Alignment", "Exact coordinate rows");
                property_row(ui, "Units", "Identical units required");
                property_row(ui, "Interpolation", "None · exact only");
                property_row(ui, "Resampling", "None · source grid retained");
                property_row(ui, "Extrapolation", "Forbidden");
                property_row(ui, "Precision", "Source f64 · no rounding");
                property_row(
                    ui,
                    "Disposition",
                    match receipt.disposition {
                        crate::results::visualization_document::ComparisonDisposition::Passed => {
                            "passed"
                        }
                        crate::results::visualization_document::ComparisonDisposition::Failed => {
                            "failed"
                        }
                    },
                );
            } else {
                empty_note(
                    ui,
                    "No comparison receipt has been created for the selected immutable datasets.",
                );
            }
            if Button::new("Plan explicit comparison").show(ui).clicked() {
                open_dock(app, VisualizationDock::Comparison);
            }

            separator(ui, t.color.border);
            let operation = app.state.workbench.visualization_studio.operation_state;
            panel_heading(ui, "Progressive operation", operation_label(operation));
            ui.label(
                RichText::new("Exact source-sample integrity scan")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            let studio = &app.state.workbench.visualization_studio;
            let progress = if studio.operation_total == 0 {
                0.0
            } else {
                studio.operation_processed as f32 / studio.operation_total as f32
            };
            ui.add(egui::ProgressBar::new(progress).show_percentage());
            if operation == OperationState::Completed {
                ui.monospace(format!(
                    "{} samples · checksum {:016x}",
                    studio.operation_total, studio.operation_checksum
                ));
            }
            let source_available = source_integrity_scan_binding(&app.state).is_some();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(
                        source_available
                            && matches!(
                                operation,
                                OperationState::NotStarted | OperationState::Completed
                            ),
                        egui::Button::new("Start"),
                    )
                    .on_disabled_hover_text(
                        "Start requires a selected retained analysis with exact source samples",
                    )
                    .clicked()
                {
                    start_source_integrity_scan(app);
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Running,
                        egui::Button::new("Advance"),
                    )
                    .on_disabled_hover_text("Advance requires a running selected operation")
                    .clicked()
                    && let Err(error) = advance_source_integrity_scan(app)
                {
                    app.state.push_user_message(ConsoleMessage::error(error));
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Running,
                        egui::Button::new("Cancel"),
                    )
                    .on_disabled_hover_text("Cancel requires a running selected operation")
                    .clicked()
                {
                    app.state.workbench.visualization_studio.operation_state =
                        OperationState::Cancelled;
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Cancelled,
                        egui::Button::new("Recover"),
                    )
                    .on_disabled_hover_text("Recover requires a cancelled operation")
                    .clicked()
                    && let Err(error) = recover_source_integrity_scan(app)
                {
                    app.state.push_user_message(ConsoleMessage::error(error));
                }
            });
            if compact {
                ui.add_space(16.0);
            }
        });
}

pub(super) const fn operation_label(state: OperationState) -> &'static str {
    match state {
        OperationState::NotStarted => "not started",
        OperationState::Running => "running",
        OperationState::Cancelled => "cancelled",
        OperationState::Completed => "completed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Markers overlay only panes whose horizontal axis is the sweep the
    /// marker's coordinate was taken in.
    ///
    /// The overlay maps a retained X coordinate onto whatever the well is
    /// showing. On the folded eye, the binned histogram, the spectrum or a
    /// Smith chart the abscissa is computed by the sheet, so a marker placed
    /// in seconds was drawn at a position that means nothing.
    #[test]
    fn markers_overlay_only_panes_that_draw_the_retained_sweep() {
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::DcSweep,
            ResultViewer::Bode,
            ResultViewer::NoiseContrib,
        ] {
            assert!(
                marker_domain_matches_the_pane(viewer),
                "{viewer:?} draws the retained sweep"
            );
        }
        for viewer in [
            ResultViewer::Fft,
            ResultViewer::Eye,
            ResultViewer::Hist,
            ResultViewer::Smith,
            ResultViewer::Nyquist,
            ResultViewer::PhaseNoise,
            ResultViewer::HarmonicBalance,
        ] {
            assert!(
                !marker_domain_matches_the_pane(viewer),
                "{viewer:?} computes its own abscissa"
            );
        }
    }
}
