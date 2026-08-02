//! The correlation workspace sections: datasets through the release gate.
//!
//! Each section renders one part of the correlation projection and none of
//! them recompute it — the projection is built once and cached against its
//! inputs, so every section on screen is describing the same evaluation. The
//! comparison plot draws residuals at their measured coordinates rather than
//! resampling them onto an even grid.

use super::*;

pub(super) fn suite_selector(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
    let lineages = projection.correlation.suite_lineages();
    if lineages.len() <= 1 {
        return;
    }
    let current_source = model_correlation_controller::current_source_binding(app).ok();
    let selected = projection
        .suite
        .as_ref()
        .map_or("Select suite", |suite| suite.name.as_str());
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Suite").font(theme::sans(tokens::FS_0, FontWeight::Medium)));
        egui::ComboBox::from_id_salt("model-correlation-suite-selector")
            .selected_text(selected)
            .width(260.0)
            .show_ui(ui, |ui| {
                for suite in lineages {
                    let active = projection
                        .suite
                        .as_ref()
                        .is_some_and(|selected| selected.id.eq_ignore_ascii_case(&suite.id));
                    let source_state = if current_source
                        .as_ref()
                        .is_some_and(|source| suite.source == *source)
                    {
                        "current source"
                    } else {
                        "earlier source"
                    };
                    if ui
                        .selectable_label(
                            active,
                            format!(
                                "{} · r{} · {source_state}",
                                suite.name,
                                suite.revision.get()
                            ),
                        )
                        .clicked()
                    {
                        app.state.workbench.model_correlation.selected_suite_id =
                            Some(suite.id.clone());
                        app.state.workbench.model_correlation.selected_dataset_id = None;
                        app.state.workbench.model_correlation.selected_metric_id = None;
                    }
                }
            });
    });
    ui.add_space(8.0);
}

pub(super) fn datasets_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let Some(suite) = projection.suite.as_ref() else {
        empty_state(
            ui,
            projection,
            "No reference datasets retained",
            "Add a checksum-bound bench, silicon, vendor, oracle, or exact simulated dataset.",
        );
        return;
    };
    let Ok(datasets) = suite.latest_datasets() else {
        error_panel(ui, "Dataset revisions are internally inconsistent");
        return;
    };
    if datasets.is_empty() {
        empty_state(
            ui,
            projection,
            "No reference datasets retained",
            "Add a checksum-bound bench, silicon, vendor, oracle, or exact simulated dataset.",
        );
        return;
    }
    section_panel(ui, "Retained dataset revisions", |ui| {
        ScrollArea::horizontal()
            .id_salt("correlation-datasets-table")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("correlation-datasets-grid")
                    .striped(true)
                    .min_col_width(92.0)
                    .spacing(Vec2::new(18.0, 8.0))
                    .show(ui, |ui| {
                        table_header(ui, "Dataset");
                        table_header(ui, "Class");
                        table_header(ui, "Revision");
                        table_header(ui, "Rows");
                        table_header(ui, "Source");
                        table_header(ui, "SHA-256");
                        ui.end_row();
                        for dataset in &datasets {
                            let selected = app
                                .state
                                .workbench
                                .model_correlation
                                .selected_dataset_id
                                .as_deref()
                                .is_some_and(|id| id.eq_ignore_ascii_case(&dataset.id));
                            if ui.selectable_label(selected, &dataset.name).clicked() {
                                app.state.workbench.model_correlation.selected_dataset_id =
                                    Some(dataset.id.clone());
                            }
                            ui.label(dataset_class_label(dataset.class));
                            ui.monospace(dataset.revision.get().to_string());
                            ui.monospace(dataset.observations.len().to_string());
                            ui.label(&dataset.source_name);
                            ui.monospace(short_digest(dataset.raw_digest.to_string().as_str()));
                            ui.end_row();
                        }
                    });
            });
    });
    ui.add_space(10.0);
    section_panel(ui, "Selected dataset evidence", |ui| {
        let selected = app
            .state
            .workbench
            .model_correlation
            .selected_dataset_id
            .as_deref()
            .and_then(|id| {
                datasets
                    .iter()
                    .find(|dataset| dataset.id.eq_ignore_ascii_case(id))
                    .copied()
            })
            .or_else(|| datasets.first().copied());
        if let Some(dataset) = selected {
            key_value(ui, "Authority", &dataset.authority);
            key_value(ui, "Device or lot", &dataset.device_or_lot);
            key_value(ui, "Fixture", &dataset.fixture);
            key_value(ui, "Calibration", &dataset.calibration);
            key_value(ui, "Raw source", &dataset.source_name);
            if let Some(provenance) = &dataset.simulation_provenance {
                key_value(ui, "Simulation run", &provenance.run_id);
                key_value(ui, "Run dataset", &provenance.run_dataset_id);
                key_value(
                    ui,
                    "Analysis result",
                    &format!(
                        "{} · {}",
                        provenance.analysis_id,
                        short_digest(&provenance.analysis_result_digest.to_string())
                    ),
                );
                key_value(
                    ui,
                    "Plan snapshot",
                    &format!(
                        "{} · project r{} · {}",
                        provenance.plan_id,
                        provenance.project_revision.get(),
                        short_digest(&provenance.prepared_snapshot_digest.to_string())
                    ),
                );
                key_value(ui, "Execution target", &provenance.execution_target);
                key_value(
                    ui,
                    "Canonical export",
                    &short_digest(&provenance.export_digest.to_string()),
                );
            }
        }
    });
}

pub(super) fn conditions_section(
    ui: &mut Ui,
    _app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let Some(suite) = projection.suite.as_ref() else {
        empty_state(
            ui,
            projection,
            "No operating-envelope policy retained",
            "Condition alignment is derived only after compatible datasets are present.",
        );
        return;
    };
    let Ok(datasets) = suite.latest_datasets() else {
        error_panel(ui, "Dataset revisions are internally inconsistent");
        return;
    };
    let observation_count = datasets
        .iter()
        .map(|dataset| dataset.observations.len())
        .sum::<usize>();
    if observation_count == 0 {
        empty_state(
            ui,
            projection,
            "No operating-envelope observations retained",
            "Import condition:<name>[<unit>] CSV columns to retain exact voltage, temperature, load, frequency, lot, and fixture coordinates.",
        );
        return;
    }
    section_panel(ui, "Declared operating-envelope observations", |ui| {
        let page = pagination_controls(
            ui,
            egui::Id::new((
                "correlation-condition-observations-page",
                suite.id.as_str(),
                suite.revision.get(),
            )),
            observation_count,
            CONDITION_OBSERVATIONS_PER_PAGE,
            "observations",
        );
        ui.add_space(6.0);
        ScrollArea::horizontal()
            .id_salt("correlation-condition-grid-scroll")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("correlation-condition-grid")
                    .striped(true)
                    .min_col_width(112.0)
                    .spacing(Vec2::new(24.0, 8.0))
                    .show(ui, |ui| {
                        table_header(ui, "Dataset");
                        table_header(ui, "Observation");
                        table_header(ui, "Quantity");
                        table_header(ui, "Retained coordinates");
                        ui.end_row();
                        let mut dataset_start: usize = 0;
                        for dataset in &datasets {
                            let dataset_end =
                                dataset_start.saturating_add(dataset.observations.len());
                            if dataset_start >= page.end {
                                break;
                            }
                            if page.start < dataset_end && page.end > dataset_start {
                                let local_start = page.start.saturating_sub(dataset_start);
                                let local_end = page
                                    .end
                                    .saturating_sub(dataset_start)
                                    .min(dataset.observations.len());
                                for observation in &dataset.observations[local_start..local_end] {
                                    ui.label(&dataset.name);
                                    ui.monospace(&observation.id);
                                    ui.monospace(format!(
                                        "{} [{}]",
                                        observation.quantity, observation.unit
                                    ));
                                    let coordinates = observation_coordinates(observation);
                                    ui.monospace(if coordinates.is_empty() {
                                        "No retained coordinates"
                                    } else {
                                        coordinates.as_str()
                                    });
                                    ui.end_row();
                                }
                            }
                            dataset_start = dataset_end;
                        }
                    });
            });
    });
    ui.add_space(10.0);
    section_panel(ui, "Alignment contract", |ui| {
        if suite.metrics.is_empty() {
            ui.label("Add a metric to declare exact matching or monotone interpolation.");
        } else {
            let page = pagination_controls(
                ui,
                egui::Id::new((
                    "correlation-condition-metrics-page",
                    suite.id.as_str(),
                    suite.revision.get(),
                )),
                suite.metrics.len(),
                METRICS_PER_PAGE,
                "metric contracts",
            );
            ui.add_space(6.0);
            for metric in &suite.metrics[page.start..page.end] {
                key_value(
                    ui,
                    &metric.name,
                    &format!(
                        "{} · {}",
                        alignment_label(&metric.alignment),
                        metric.domain.as_ref().map_or_else(
                            || "full retained domain".to_owned(),
                            |domain| {
                                format!(
                                    "{} {}..{} {}",
                                    domain.axis,
                                    domain.minimum.get(),
                                    domain.maximum.get(),
                                    domain.unit
                                )
                            }
                        )
                    ),
                );
            }
        }
    });
}

pub(super) fn metrics_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let Some(suite) = projection.suite.as_ref() else {
        empty_state(
            ui,
            projection,
            "No correlation metrics retained",
            "Add reference and simulation datasets, then define a unit-aware metric contract.",
        );
        return;
    };
    if suite.metrics.is_empty() {
        empty_state(
            ui,
            projection,
            "No correlation metrics retained",
            "Define a unit-aware calculation, limit, uncertainty treatment, aggregation, alignment, and review role.",
        );
        return;
    }
    section_panel(ui, "Versioned metric definitions", |ui| {
        let page = pagination_controls(
            ui,
            egui::Id::new((
                "correlation-metric-definitions-page",
                suite.id.as_str(),
                suite.revision.get(),
            )),
            suite.metrics.len(),
            METRICS_PER_PAGE,
            "metric definitions",
        );
        ui.add_space(6.0);
        ScrollArea::horizontal()
            .id_salt("correlation-metrics-table")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("correlation-metrics-grid")
                    .striped(true)
                    .min_col_width(92.0)
                    .spacing(Vec2::new(18.0, 8.0))
                    .show(ui, |ui| {
                        for header in [
                            "Metric",
                            "Quantity",
                            "Reference",
                            "Simulation",
                            "Calculation",
                            "Limit",
                            "Coverage",
                            "Aggregation",
                            "Role",
                        ] {
                            table_header(ui, header);
                        }
                        ui.end_row();
                        for metric in &suite.metrics[page.start..page.end] {
                            let selected = app
                                .state
                                .workbench
                                .model_correlation
                                .selected_metric_id
                                .as_deref()
                                .is_some_and(|id| id.eq_ignore_ascii_case(&metric.id));
                            if ui.selectable_label(selected, &metric.name).clicked() {
                                app.state.workbench.model_correlation.selected_metric_id =
                                    Some(metric.id.clone());
                            }
                            ui.monospace(&metric.quantity);
                            ui.monospace(&metric.reference_dataset_id);
                            ui.monospace(&metric.simulation_dataset_id);
                            ui.label(calculation_label(metric.calculation));
                            ui.monospace(metric.limit.get().to_string());
                            ui.monospace(format!("{:.1}%", metric.minimum_coverage.get() * 100.0));
                            ui.label(aggregation_label(metric.aggregation));
                            ui.label(release_role_label(metric.release_role));
                            ui.end_row();
                        }
                    });
            });
        ui.add_space(8.0);
        let selected_metric = app
            .state
            .workbench
            .model_correlation
            .selected_metric_id
            .as_deref()
            .and_then(|id| {
                suite
                    .metrics
                    .iter()
                    .find(|metric| metric.id.eq_ignore_ascii_case(id))
            });
        let blocker = correlation_action_blocker(app, projection, true, false);
        let revise = Button::new("Revise selected metric")
            .enabled(selected_metric.is_some() && blocker.is_none())
            .show(ui);
        if let Some(reason) = blocker.as_deref() {
            revise.on_disabled_hover_text(reason);
        } else if selected_metric.is_none() {
            revise.on_disabled_hover_text("Select a retained metric definition first");
        } else if revise.clicked()
            && let Some(metric) = selected_metric
        {
            open_correlation_dialog(
                ui.ctx(),
                app,
                ModelCorrelationDialog::AddMetric(metric_draft_from_definition(metric)),
            );
        }
    });
}

pub(super) fn comparison_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    if let Some(error) = projection.evaluation_error.as_deref() {
        error_panel(ui, error);
        return;
    }
    let Some(evaluation) = projection.evaluation.as_ref() else {
        empty_state(
            ui,
            projection,
            "No aligned comparison evidence",
            "Add compatible datasets and at least one valid metric to evaluate the retained suite.",
        );
        return;
    };
    let selected = app
        .state
        .workbench
        .model_correlation
        .selected_metric_id
        .as_deref()
        .and_then(|id| {
            evaluation
                .metric_outcomes
                .iter()
                .find(|outcome| outcome.metric_id.eq_ignore_ascii_case(id))
        })
        .or_else(|| evaluation.metric_outcomes.first());
    if let Some(outcome) = selected {
        section_panel(ui, "Categorical aligned samples and residuals", |ui| {
            comparison_plot(ui, outcome);
        });
        ui.add_space(10.0);
    }
    section_panel(ui, "Metric outcomes", |ui| {
        let page = pagination_controls(
            ui,
            egui::Id::new((
                "correlation-metric-outcomes-page",
                evaluation.suite_id.as_str(),
                evaluation.suite_revision.get(),
            )),
            evaluation.metric_outcomes.len(),
            OUTCOMES_PER_PAGE,
            "metric outcomes",
        );
        ui.add_space(6.0);
        ScrollArea::horizontal()
            .id_salt("correlation-outcomes-grid-scroll")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("correlation-outcomes-grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .spacing(Vec2::new(20.0, 8.0))
                    .show(ui, |ui| {
                        for header in [
                            "Metric",
                            "Role",
                            "Evaluated",
                            "Excluded",
                            "Coverage",
                            "Aggregate",
                            "Normalized",
                            "Verdict",
                        ] {
                            table_header(ui, header);
                        }
                        ui.end_row();
                        for outcome in &evaluation.metric_outcomes[page.start..page.end] {
                            let selected = app
                                .state
                                .workbench
                                .model_correlation
                                .selected_metric_id
                                .as_deref()
                                .is_some_and(|id| id.eq_ignore_ascii_case(&outcome.metric_id));
                            if ui.selectable_label(selected, &outcome.metric_id).clicked() {
                                app.state.workbench.model_correlation.selected_metric_id =
                                    Some(outcome.metric_id.clone());
                            }
                            ui.label(release_role_label(outcome.release_role));
                            ui.monospace(outcome.evaluated_points.to_string());
                            ui.monospace(outcome.excluded_points.to_string());
                            ui.monospace(format!("{:.1}%", outcome.coverage.get() * 100.0));
                            ui.monospace(format!("{:.6}", outcome.aggregate_error.get()));
                            ui.monospace(format!(
                                "{:.4}×",
                                outcome.aggregate_normalized_error.get()
                            ));
                            status_text(
                                ui,
                                outcome.passed,
                                if outcome.passed { "pass" } else { "fail" },
                            );
                            ui.end_row();
                        }
                    });
            });
    });
    ui.add_space(10.0);
    if let Some(outcome) = selected {
        section_panel(ui, "Source-aligned residuals", |ui| {
            let page = pagination_controls(
                ui,
                egui::Id::new((
                    "correlation-residuals-page",
                    evaluation.suite_id.as_str(),
                    evaluation.suite_revision.get(),
                    outcome.metric_id.as_str(),
                )),
                outcome.residuals.len(),
                RESIDUALS_PER_PAGE,
                "residuals",
            );
            ui.add_space(6.0);
            ScrollArea::horizontal()
                .id_salt("correlation-residual-table")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    egui::Grid::new("correlation-residual-grid")
                        .striped(true)
                        .min_col_width(94.0)
                        .spacing(Vec2::new(18.0, 8.0))
                        .show(ui, |ui| {
                            for header in [
                                "Reference point",
                                "Simulation source",
                                "Alignment",
                                "Reference",
                                "Simulated",
                                "Error",
                                "Effective limit",
                                "Normalized",
                                "Scope",
                            ] {
                                table_header(ui, header);
                            }
                            ui.end_row();
                            for residual in &outcome.residuals[page.start..page.end] {
                                ui.monospace(&residual.reference_observation_id);
                                ui.monospace(bounded_string_list(
                                    &residual.simulation_observation_ids,
                                    SIMULATION_SOURCE_DISPLAY_LIMIT,
                                ));
                                ui.label(alignment_evidence_label(residual.alignment_evidence));
                                ui.monospace(residual.reference_value.get().to_string());
                                ui.monospace(residual.simulated_value.get().to_string());
                                ui.monospace(format!("{:.6}", residual.metric_error.get()));
                                ui.monospace(format!("{:.6}", residual.effective_limit.get()));
                                ui.monospace(format!("{:.4}×", residual.normalized_error.get()));
                                ui.label(if residual.excluded {
                                    "excluded · retained"
                                } else {
                                    "gate"
                                });
                                ui.end_row();
                            }
                        });
                });
        });
    }
}

pub(super) fn comparison_plot(ui: &mut Ui, outcome: &CorrelationMetricOutcome) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 250.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    if outcome.residuals.is_empty() {
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            "No aligned residual points",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        describe_region(
            ui,
            &response,
            "Categorical aligned comparison",
            "No aligned residual points",
        );
        return;
    }
    let sample_indices = deterministic_sample_indices(outcome.residuals.len(), PLOT_SAMPLE_LIMIT);
    let samples = sample_indices
        .iter()
        .map(|index| &outcome.residuals[*index])
        .collect::<Vec<_>>();

    let source_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 48.0, rect.top() + 30.0),
        Pos2::new(rect.right() - 14.0, rect.top() + 169.0),
    );
    let residual_rect = Rect::from_min_max(
        Pos2::new(source_rect.left(), rect.top() + 192.0),
        Pos2::new(source_rect.right(), rect.bottom() - 14.0),
    );
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    for residual in &samples {
        minimum = minimum
            .min(residual.reference_value.get())
            .min(residual.simulated_value.get());
        maximum = maximum
            .max(residual.reference_value.get())
            .max(residual.simulated_value.get());
    }
    if (maximum - minimum).abs() <= f64::EPSILON {
        let padding = maximum.abs().max(1.0) * 0.05;
        minimum -= padding;
        maximum += padding;
    }
    for index in 0..=4 {
        let fraction = index as f32 / 4.0;
        let y = egui::lerp(source_rect.bottom()..=source_rect.top(), fraction);
        ui.painter().hline(
            source_rect.x_range(),
            y,
            Stroke::new(1.0, t.color.border.gamma_multiply(0.55)),
        );
        let value = minimum + (maximum - minimum) * f64::from(fraction);
        ui.painter().text(
            Pos2::new(source_rect.left() - 7.0, y),
            Align2::RIGHT_CENTER,
            format!("{value:.3}"),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    let point_x = |index: usize| {
        if samples.len() == 1 {
            source_rect.center().x
        } else {
            egui::lerp(
                source_rect.x_range(),
                index as f32 / (samples.len() - 1) as f32,
            )
        }
    };
    let point_y = |value: f64| {
        let fraction = ((value - minimum) / (maximum - minimum)) as f32;
        egui::lerp(
            source_rect.bottom()..=source_rect.top(),
            fraction.clamp(0.0, 1.0),
        )
    };
    for (index, residual) in samples.iter().enumerate() {
        let x = point_x(index);
        let reference = Pos2::new(x, point_y(residual.reference_value.get()));
        let simulation = Pos2::new(x, point_y(residual.simulated_value.get()));
        ui.painter().line_segment(
            [reference, simulation],
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().circle_filled(reference, 2.5, t.color.accent);
        ui.painter()
            .circle_stroke(simulation, 3.0, Stroke::new(1.5, t.color.ok));
    }
    ui.painter().text(
        Pos2::new(source_rect.left(), rect.top() + 11.0),
        Align2::LEFT_CENTER,
        "● reference sample",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.accent,
    );
    ui.painter().text(
        Pos2::new(source_rect.left() + 138.0, rect.top() + 11.0),
        Align2::LEFT_CENTER,
        "○ simulation sample",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.ok,
    );
    ui.painter().text(
        Pos2::new(source_rect.right(), rect.top() + 11.0),
        Align2::RIGHT_CENTER,
        format!(
            "SHOWING {} OF {} · DETERMINISTIC CATEGORICAL SAMPLE",
            samples.len(),
            outcome.residuals.len()
        ),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
    ui.painter().text(
        Pos2::new(source_rect.left(), residual_rect.top() - 13.0),
        Align2::LEFT_CENTER,
        "NORMALIZED RESIDUAL · 1.0× GATE",
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
    let gate_y = residual_rect.center().y;
    ui.painter().hline(
        residual_rect.x_range(),
        gate_y,
        Stroke::new(1.0, t.color.warn),
    );
    let bar_width = (residual_rect.width() / samples.len() as f32 * 0.56).clamp(2.0, 16.0);
    for (index, residual) in samples.iter().enumerate() {
        let value = residual.normalized_error.get().min(2.0) as f32;
        let height = residual_rect.height() * value / 2.0;
        let x = point_x(index);
        let bar = Rect::from_min_max(
            Pos2::new(x - bar_width * 0.5, residual_rect.bottom() - height),
            Pos2::new(x + bar_width * 0.5, residual_rect.bottom()),
        );
        ui.painter().rect_filled(
            bar,
            0.0,
            if residual.excluded {
                t.color.text_faint
            } else if residual.normalized_error.get() > 1.0 {
                t.color.err
            } else {
                t.color.accent
            },
        );
    }
    if let Some(pointer) = response.hover_pos()
        && source_rect
            .union(residual_rect)
            .expand2(Vec2::new(0.0, 8.0))
            .contains(pointer)
    {
        let relative = ((pointer.x - source_rect.left()) / source_rect.width()).clamp(0.0, 1.0);
        let index = if samples.len() == 1 {
            0
        } else {
            (relative * (samples.len() - 1) as f32).round() as usize
        };
        let residual = samples[index];
        let x = point_x(index);
        ui.painter().vline(
            x,
            source_rect.top()..=residual_rect.bottom(),
            Stroke::new(1.0, t.color.text_dim),
        );
        response.clone().on_hover_text(format!(
            "{}\nreference {}\nsimulation {}\nnormalized {:.4}×\n{}",
            residual.reference_observation_id,
            residual.reference_value.get(),
            residual.simulated_value.get(),
            residual.normalized_error.get(),
            alignment_evidence_label(residual.alignment_evidence)
        ));
    }
    let accessible_summary = format!(
        "Showing {} of {} categorical aligned points using deterministic index sampling; {} evaluated, {} excluded; aggregate normalized error {:.4}",
        samples.len(),
        outcome.residuals.len(),
        outcome.evaluated_points,
        outcome.excluded_points,
        outcome.aggregate_normalized_error.get()
    );
    describe_region(
        ui,
        &response,
        "Categorical aligned comparison",
        &accessible_summary,
    );
}

pub(super) fn outliers_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let Some(suite) = projection.suite.as_ref() else {
        empty_state(
            ui,
            projection,
            "No outlier decisions retained",
            "Evaluate a suite before recording accountable residual dispositions.",
        );
        return;
    };
    if suite.dispositions.is_empty() {
        empty_state(
            ui,
            projection,
            "No outlier decisions retained",
            "All numerical residuals remain in gate scope. Review any point below to retain, exclude a verified fixture fault, or preserve it as limit-only evidence.",
        );
    } else {
        section_panel(ui, "Append-only disposition ledger", |ui| {
            let page = pagination_controls(
                ui,
                egui::Id::new((
                    "correlation-dispositions-page",
                    suite.id.as_str(),
                    suite.revision.get(),
                )),
                suite.dispositions.len(),
                DISPOSITIONS_PER_PAGE,
                "dispositions",
            );
            ui.add_space(6.0);
            ScrollArea::horizontal()
                .id_salt("correlation-dispositions-table")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    egui::Grid::new("correlation-dispositions-grid")
                        .striped(true)
                        .min_col_width(100.0)
                        .spacing(Vec2::new(18.0, 8.0))
                        .show(ui, |ui| {
                            for header in [
                                "Metric",
                                "Point",
                                "Decision",
                                "Owner",
                                "Reviewer",
                                "Reason",
                                "Supersedes",
                            ] {
                                table_header(ui, header);
                            }
                            ui.end_row();
                            for disposition in &suite.dispositions[page.start..page.end] {
                                ui.monospace(&disposition.metric_id);
                                ui.monospace(&disposition.reference_observation_id);
                                ui.label(outlier_decision_label(disposition.decision));
                                ui.monospace(&disposition.owner_id);
                                ui.monospace(&disposition.reviewer_id);
                                ui.label(&disposition.reason);
                                ui.monospace(
                                    disposition.supersedes.as_deref().unwrap_or("initial"),
                                );
                                ui.end_row();
                            }
                        });
                });
        });
        ui.add_space(10.0);
    }
    let Some(evaluation) = projection.evaluation.as_ref() else {
        return;
    };
    let disposition_blocker = correlation_action_blocker(app, projection, true, false);
    section_panel(ui, "Review residual scope", |ui| {
        let residual_count = evaluation
            .metric_outcomes
            .iter()
            .map(|outcome| outcome.residuals.len())
            .sum::<usize>();
        let page = pagination_controls(
            ui,
            egui::Id::new((
                "correlation-outlier-residuals-page",
                evaluation.suite_id.as_str(),
                evaluation.suite_revision.get(),
            )),
            residual_count,
            RESIDUALS_PER_PAGE,
            "residuals",
        );
        ui.add_space(6.0);
        let mut outcome_start: usize = 0;
        for outcome in &evaluation.metric_outcomes {
            let outcome_end = outcome_start.saturating_add(outcome.residuals.len());
            if outcome_start >= page.end {
                break;
            }
            if page.start < outcome_end && page.end > outcome_start {
                let local_start = page.start.saturating_sub(outcome_start);
                let local_end = page
                    .end
                    .saturating_sub(outcome_start)
                    .min(outcome.residuals.len());
                for residual in &outcome.residuals[local_start..local_end] {
                    ui.horizontal(|ui| {
                        ui.monospace(format!(
                            "{} / {}",
                            outcome.metric_id, residual.reference_observation_id
                        ));
                        ui.label(format!("{:.4}×", residual.normalized_error.get()));
                        ui.label(if residual.excluded {
                            "excluded · retained in evidence"
                        } else {
                            "included in gate"
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let review = Button::new("Review disposition")
                                .enabled(disposition_blocker.is_none())
                                .show(ui);
                            if let Some(blocker) = disposition_blocker.as_deref() {
                                review.on_disabled_hover_text(blocker);
                            } else if review.clicked() {
                                open_correlation_dialog(
                                    ui.ctx(),
                                    app,
                                    ModelCorrelationDialog::Disposition(CorrelationOutlierDraft {
                                        metric_id: outcome.metric_id.clone(),
                                        reference_observation_id: residual
                                            .reference_observation_id
                                            .clone(),
                                        ..CorrelationOutlierDraft::default()
                                    }),
                                );
                            }
                        });
                    });
                    ui.separator();
                }
            }
            outcome_start = outcome_end;
        }
    });
}

pub(super) fn evidence_section(
    ui: &mut Ui,
    _app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let Some(suite) = projection.suite.as_ref() else {
        empty_state(
            ui,
            projection,
            "No immutable correlation evidence",
            "Validate a complete suite, then record an independent reviewer identity and conclusion.",
        );
        return;
    };
    if projection.current_evidence.is_empty() {
        empty_state(
            ui,
            projection,
            "No current immutable review evidence",
            "The suite may be numerically evaluable, but a reviewer identity and explicit conclusion have not been retained for this exact revision.",
        );
    } else {
        section_panel(ui, "Immutable reviewer conclusions", |ui| {
            for evidence in projection.current_evidence.iter().rev() {
                ui.horizontal_wrapped(|ui| {
                    status_text(
                        ui,
                        evidence.approved(),
                        if evidence.approved() {
                            "reviewer approved"
                        } else if evidence.passed {
                            "numerical gate passed · reviewer rejected"
                        } else {
                            "numerical gate failed"
                        },
                    );
                    ui.monospace(format!("{} · {}", evidence.reviewer_id, evidence.id));
                });
                ui.label(&evidence.conclusion);
                key_value(
                    ui,
                    "Reviewed",
                    &format!("{} ms since Unix epoch", evidence.reviewed_at_unix_ms),
                );
                key_value(
                    ui,
                    "Evidence digest",
                    &evidence
                        .content_digest()
                        .map(|digest| short_digest(&digest.to_string()))
                        .unwrap_or_else(|error| format!("invalid: {error}")),
                );
                ui.separator();
            }
        });
        ui.add_space(10.0);
    }
    section_panel(ui, "Reproduction bundle", |ui| {
        key_value(
            ui,
            "Suite",
            &format!("{} · r{}", suite.id, suite.revision.get()),
        );
        key_value(
            ui,
            "Suite digest",
            &suite
                .content_digest()
                .map(|digest| short_digest(&digest.to_string()))
                .unwrap_or_else(|error| format!("invalid: {error}")),
        );
        key_value(
            ui,
            "Model source",
            &format!(
                "{} · r{} · {}",
                suite.source.model_id,
                suite.source.source_revision.get(),
                short_digest(&suite.source.source_digest.to_string())
            ),
        );
        if let Ok(datasets) = suite.latest_datasets() {
            for dataset in datasets {
                key_value(
                    ui,
                    &dataset.name,
                    &format!(
                        "{} rows · {}",
                        dataset.observations.len(),
                        short_digest(&dataset.raw_digest.to_string())
                    ),
                );
            }
        }
    });
}

pub(super) fn gate_section(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
    let current = projection.current_evidence.last();
    section_panel(ui, "Qualification handoff", |ui| {
        match (projection.suite.as_ref(), current) {
            (Some(suite), Some(evidence)) => {
                status_text(
                    ui,
                    evidence.approved() && projection.source_current,
                    if evidence.approved() && projection.source_current {
                        "current reviewed correlation evidence"
                    } else {
                        "correlation evidence does not satisfy handoff"
                    },
                );
                key_value(ui, "Evidence", &evidence.id);
                key_value(ui, "Reviewer", &evidence.reviewer_id);
                key_value(
                    ui,
                    "Exact source",
                    &format!(
                        "{} · r{} · {}",
                        suite.source.model_id,
                        suite.source.source_revision.get(),
                        short_digest(&suite.source.source_digest.to_string())
                    ),
                );
                ui.label(&evidence.conclusion);
            }
            (Some(_), None) => {
                ui.label("Validate and review this exact suite revision before handoff.");
            }
            (None, _) => {
                ui.label("Create a correlation suite before qualification handoff.");
            }
        }
    });
    ui.add_space(10.0);
    ui.horizontal_wrapped(|ui| {
        let vector_blocker = projection.source_error.as_deref().or_else(|| {
            (projection.model.is_none()).then_some("Select a project-owned model first")
        });
        let review_vectors = Button::new("Review affected vectors")
            .enabled(vector_blocker.is_none())
            .min_width(164.0)
            .show(ui);
        if let Some(reason) = vector_blocker {
            review_vectors.on_disabled_hover_text(reason);
        } else if review_vectors.clicked()
            && let Err(error) = open_affected_vectors(app)
        {
            app.state.workbench.model_correlation.notice = Some(error);
        }
        if Button::new("Return to qualification")
            .accent()
            .min_width(158.0)
            .show(ui)
            .clicked()
        {
            app.state.workbench.models_page = super::super::super::state::ModelsPage::Qualification;
            if let Err(error) = app.state.workbench.navigate(
                SurfaceRoute::surface(SurfaceId::Models),
                RouteTransitionSource::User,
            ) {
                app.state.workbench.model_correlation.notice = Some(error.to_string());
            }
        }
    });
}

pub(super) fn open_affected_vectors(app: &mut RSpiceApp) -> Result<(), String> {
    let library = app
        .state
        .model_library_manager
        .selected_library
        .clone()
        .ok_or_else(|| "Select a project-owned model library first".to_owned())?;
    let model = app
        .state
        .workbench
        .selected_model
        .clone()
        .ok_or_else(|| "Select a project-owned model family first".to_owned())?;
    open_project_model(app, &library, &model)
        .map_err(|error| format!("Qualification vectors cannot be opened: {error}"))?;
    app.state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::ModelEditor),
            RouteTransitionSource::User,
        )
        .map_err(|error| format!("Qualification vector browser cannot be shown: {error}"))?;
    app.state.workbench.model_editor.active_section = ModelEditorSection::Tests;
    app.state.workbench.model_editor.qualification_plan_open = true;
    Ok(())
}

pub(super) fn empty_state(
    ui: &mut Ui,
    projection: &CorrelationProjection,
    title: &str,
    detail: &str,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 116.0), Sense::hover());
    ui.painter().rect(
        rect,
        0.0,
        t.color.bg_panel,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let (title, detail) = projection
        .source_error
        .as_deref()
        .map_or((title, detail), |error| {
            ("Exact project model is unavailable", error)
        });
    ui.painter().text(
        Pos2::new(rect.left() + 14.0, rect.top() + 18.0),
        Align2::LEFT_TOP,
        title,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        if projection.source_error.is_some() {
            t.color.err
        } else {
            t.color.text
        },
    );
    let detail_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 14.0, rect.top() + 43.0),
        Pos2::new(rect.right() - 14.0, rect.bottom() - 12.0),
    );
    let accessible_detail = detail.to_owned();
    let detail_galley = ui.painter().layout(
        accessible_detail.clone(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        detail_rect.width().max(1.0),
    );
    ui.painter().with_clip_rect(detail_rect).galley(
        detail_rect.min,
        detail_galley,
        t.color.text_dim,
    );
    describe_region(ui, &response, title, &accessible_detail);
}

pub(super) fn correlation_notice(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(notice) = app.state.workbench.model_correlation.notice.clone() else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 38.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height())),
        0.0,
        t.color.accent,
    );
    bottom_rule(ui, rect, t.color.border);
    let dismiss_width = 66.0;
    paint_elided(
        ui,
        Rect::from_min_max(
            Pos2::new(rect.left() + 13.0, rect.top()),
            Pos2::new(rect.right() - dismiss_width - 12.0, rect.bottom()),
        ),
        &notice,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text,
        12.0,
    );
    let mut dismiss = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_max(
                Pos2::new(rect.right() - dismiss_width - 8.0, rect.top()),
                Pos2::new(rect.right() - 8.0, rect.bottom()),
            ))
            .layout(Layout::right_to_left(Align::Center)),
    );
    if Button::new("Dismiss").ghost().show(&mut dismiss).clicked() {
        app.state.workbench.model_correlation.notice = None;
    }
    describe_region(ui, &response, "Correlation notification", &notice);
}

pub(super) fn select_section(app: &mut RSpiceApp, section: ModelCorrelationSection) {
    app.state.workbench.model_correlation.section = section;
    app.state.workbench.model_correlation.scroll_offset = 0.0;
}

pub(super) fn correlation_action_blocker(
    app: &RSpiceApp,
    projection: &CorrelationProjection,
    require_datasets: bool,
    require_evaluation: bool,
) -> Option<String> {
    if app.state.workbench.safe_mode.project_read_only() {
        return Some("Correlation records cannot change while the project is read-only".to_owned());
    }
    if let Some(error) = projection.source_error.as_deref() {
        return Some(error.to_owned());
    }
    if model_correlation_controller::current_source_binding(app).is_err() {
        return Some("Select an editable project-owned model source first".to_owned());
    }
    if require_datasets {
        if projection.suite.is_some() && !projection.source_current {
            return Some(
                "Select a suite bound to the current model source before defining a metric"
                    .to_owned(),
            );
        }
        let datasets = projection
            .suite
            .as_ref()
            .and_then(|suite| suite.latest_datasets().ok())
            .unwrap_or_default();
        if !datasets
            .iter()
            .any(|dataset| !dataset.class.is_simulation())
        {
            return Some(
                "Add at least one independent reference dataset before defining a metric"
                    .to_owned(),
            );
        }
        if !datasets.iter().any(|dataset| dataset.class.is_simulation()) {
            return Some(
                "Add an exact source-bound model-simulation dataset before defining a metric"
                    .to_owned(),
            );
        }
    }
    if require_evaluation {
        if let Some(error) = projection.evaluation_error.as_deref() {
            return Some(error.to_owned());
        }
        if projection.evaluation.is_none() {
            return Some("A complete, evaluable metric suite is required".to_owned());
        }
        if !projection.source_current {
            return Some("The suite must be bound to the current model source revision".to_owned());
        }
    }
    None
}

pub(super) fn initial_metric_draft(projection: &CorrelationProjection) -> CorrelationMetricDraft {
    let mut draft = CorrelationMetricDraft {
        calculation: CorrelationCalculationDraft::AbsoluteLinear,
        ..Default::default()
    };
    let Some(suite) = projection.suite.as_ref() else {
        return draft;
    };
    let Ok(datasets) = suite.latest_datasets() else {
        return draft;
    };
    if let Some(reference) = datasets
        .iter()
        .find(|dataset| !dataset.class.is_simulation())
    {
        draft.reference_dataset_id.clone_from(&reference.id);
        if let Some(observation) = reference.observations.first() {
            draft.quantity.clone_from(&observation.quantity);
        }
    }
    if let Some(simulation) = datasets
        .iter()
        .find(|dataset| dataset.class.is_simulation())
    {
        draft.simulation_dataset_id.clone_from(&simulation.id);
        if draft.quantity.is_empty()
            && let Some(observation) = simulation.observations.first()
        {
            draft.quantity.clone_from(&observation.quantity);
        }
    }
    draft
}
