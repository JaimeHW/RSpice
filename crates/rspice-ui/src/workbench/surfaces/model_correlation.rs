//! Project-owned measurement-correlation workspace.
//!
//! This specialist surface follows the exact seven-section contract in the
//! reviewed mockup. It never fabricates datasets, residuals, gate outcomes, or
//! reviewer identities: every projection is derived from persisted
//! model-library correlation records.

use std::collections::BTreeMap;
use std::sync::Arc;

use egui::{
    Align, Align2, Color32, Layout, Pos2, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, WidgetInfo,
    WidgetType,
};

use crate::product::{ObjectRevision, ProjectId};
use crate::state::model_library::{
    CorrelationAggregation, CorrelationAlignmentEvidence, CorrelationAlignmentPolicy,
    CorrelationCalculation, CorrelationDatasetClass, CorrelationEvaluation, CorrelationEvidence,
    CorrelationMetricOutcome, CorrelationObservation, CorrelationOutlierDecision,
    CorrelationReleaseRole, CorrelationSuite, ModelCorrelationState, ModelSourceEvidenceBinding,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, DialogTransactionTone};
use crate::workbench::RSpiceApp;

use super::super::design_system;
use super::super::model_correlation::{
    CorrelationAggregationDraft, CorrelationAlignmentDraft, CorrelationCalculationDraft,
    CorrelationDatasetClassDraft, CorrelationDatasetDraft, CorrelationExtrapolationDraft,
    CorrelationMetricDraft, CorrelationOutlierDecisionDraft, CorrelationOutlierDraft,
    CorrelationReleaseRoleDraft, CorrelationReviewDecisionDraft, CorrelationReviewDraft,
    ModelCorrelationDialog, ModelCorrelationSection,
};
use super::super::model_editor::{
    ModelEditorSection, open_project_model, resolve_project_model_for_editor,
};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use super::model_correlation_controller;

const OUTER_HEADER_H: f32 = 58.0;
const SUMMARY_H: f32 = 92.0;
const KPI_H: f32 = 62.0;
const NAV_W: f32 = 196.0;
const NAV_ROW_H: f32 = 36.0;
const COMPACT_BREAKPOINT: f32 = 820.0;
const CONDITION_OBSERVATIONS_PER_PAGE: usize = 24;
const CONDITION_COORDINATES_PER_OBSERVATION: usize = 8;
const METRICS_PER_PAGE: usize = 32;
const OUTCOMES_PER_PAGE: usize = 32;
const RESIDUALS_PER_PAGE: usize = 32;
const DISPOSITIONS_PER_PAGE: usize = 32;
const PLOT_SAMPLE_LIMIT: usize = 192;
const SIMULATION_SOURCE_DISPLAY_LIMIT: usize = 4;

const SUMMARY: &str = "Align measured, vendor, oracle, and simulated evidence over the declared operating envelope; define metrics, inspect systematic error, govern outliers, and hand a versioned correlation conclusion to model qualification.";
const EVIDENCE_CONTRACT: &str = "Dataset checksums, fixtures, calibration, axis alignment, model revision, conditions, metric definitions, aggregation, uncertainty, exclusions, residuals, outlier dispositions, and reviewer conclusions are retained.";

fn correlation_dialog_error_id() -> egui::Id {
    egui::Id::new("model-correlation-submit-error")
}

fn bounded_page(total: usize, page_size: usize, requested_index: usize) -> BoundedPage {
    debug_assert!(page_size > 0);
    let count = total.div_ceil(page_size).max(1);
    let index = requested_index.min(count - 1);
    let start = index.saturating_mul(page_size).min(total);
    let end = start.saturating_add(page_size).min(total);
    BoundedPage {
        index,
        count,
        start,
        end,
        total,
    }
}

fn pagination_controls(
    ui: &mut Ui,
    id: egui::Id,
    total: usize,
    page_size: usize,
    noun: &str,
) -> BoundedPage {
    let mut requested = ui
        .ctx()
        .data(|data| data.get_temp::<usize>(id))
        .unwrap_or_default();
    let initial = bounded_page(total, page_size, requested);
    requested = initial.index;
    ui.horizontal(|ui| {
        let previous = Button::new("Previous")
            .enabled(requested > 0)
            .min_width(74.0)
            .show(ui);
        if previous.clicked() {
            requested = requested.saturating_sub(1);
        }
        let next = Button::new("Next")
            .enabled(requested + 1 < initial.count)
            .min_width(58.0)
            .show(ui);
        if next.clicked() {
            requested = requested.saturating_add(1);
        }
        let page = bounded_page(total, page_size, requested);
        if total == 0 {
            ui.label(format!("Showing 0 of 0 {noun}"));
        } else {
            ui.label(format!(
                "Showing {}–{} of {} {noun} · page {} of {}",
                page.start + 1,
                page.end,
                page.total,
                page.index + 1,
                page.count
            ));
        }
    });
    let page = bounded_page(total, page_size, requested);
    ui.ctx().data_mut(|data| data.insert_temp(id, page.index));
    page
}

fn deterministic_sample_indices(total: usize, limit: usize) -> Vec<usize> {
    match (total, limit) {
        (0, _) | (_, 0) => Vec::new(),
        (_, 1) => vec![0],
        (total, limit) if total <= limit => (0..total).collect(),
        (total, limit) => (0..limit)
            .map(|sample| ((sample as u128 * (total - 1) as u128) / (limit - 1) as u128) as usize)
            .collect(),
    }
}

fn bounded_string_list(values: &[String], limit: usize) -> String {
    let shown = values
        .iter()
        .take(limit)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let omitted = values.len().saturating_sub(limit);
    if omitted == 0 {
        shown
    } else if shown.is_empty() {
        format!("{omitted} sources")
    } else {
        format!("{shown}, … +{omitted} more")
    }
}

fn observation_coordinates(observation: &CorrelationObservation) -> String {
    let shown = observation
        .coordinates
        .iter()
        .take(CONDITION_COORDINATES_PER_OBSERVATION)
        .map(|coordinate| {
            format!(
                "{}={} {}",
                coordinate.dimension,
                coordinate.value.get(),
                coordinate.unit
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let omitted = observation
        .coordinates
        .len()
        .saturating_sub(CONDITION_COORDINATES_PER_OBSERVATION);
    if omitted == 0 {
        shown
    } else if shown.is_empty() {
        format!("{omitted} coordinates")
    } else {
        format!("{shown}, … +{omitted} more")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tone {
    Neutral,
    Good,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundedPage {
    index: usize,
    count: usize,
    start: usize,
    end: usize,
    total: usize,
}

#[derive(Debug, Clone)]
struct Metric {
    label: &'static str,
    value: String,
    detail: String,
    tone: Tone,
}

#[derive(Debug, Clone)]
struct CorrelationProjection {
    model: Option<String>,
    source_error: Option<String>,
    suite_name: String,
    correlation: ModelCorrelationState,
    suite: Option<CorrelationSuite>,
    evaluation: Option<CorrelationEvaluation>,
    evaluation_error: Option<String>,
    current_evidence: Vec<CorrelationEvidence>,
    source_current: bool,
    suite_count: usize,
    dataset_count: usize,
    metric_count: usize,
    passed_metrics: usize,
    coverage_percent: Option<f64>,
    worst_normalized_error: Option<f64>,
    open_dispositions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CorrelationProjectionCacheKey {
    project_id: ProjectId,
    project_revision: ObjectRevision,
    library: Option<String>,
    model: Option<String>,
    selected_suite_id: Option<String>,
}

#[derive(Debug, Clone)]
struct CorrelationProjectionCache {
    key: CorrelationProjectionCacheKey,
    projection: Arc<CorrelationProjection>,
}

fn correlation_projection_cache_id() -> egui::Id {
    egui::Id::new("model-correlation-projection-cache")
}

impl CorrelationProjection {
    fn from_app(ctx: &egui::Context, app: &RSpiceApp) -> Arc<Self> {
        let key = CorrelationProjectionCacheKey {
            project_id: app.state.workspace.project.id(),
            project_revision: app.state.workspace.project.revision(),
            library: app.state.model_library_manager.selected_library.clone(),
            model: app.state.workbench.selected_model.clone(),
            selected_suite_id: app
                .state
                .workbench
                .model_correlation
                .selected_suite_id
                .clone(),
        };
        if let Some(cache) = ctx.data(|data| {
            data.get_temp::<CorrelationProjectionCache>(correlation_projection_cache_id())
        }) && cache.key == key
        {
            return cache.projection;
        }
        let projection = Arc::new(Self::build(app));
        ctx.data_mut(|data| {
            data.insert_temp(
                correlation_projection_cache_id(),
                CorrelationProjectionCache {
                    key,
                    projection: Arc::clone(&projection),
                },
            );
        });
        projection
    }

    fn build(app: &RSpiceApp) -> Self {
        let library = app.state.model_library_manager.selected_library.clone();
        let model = app.state.workbench.selected_model.clone();
        let resolved = match (library.as_deref(), model.as_deref()) {
            (Some(library), Some(model)) => {
                resolve_project_model_for_editor(&app.state.model_library_manager, library, model)
            }
            (None, _) => Err("Select a project-owned model library first".to_owned()),
            (_, None) => Err("Select a project-owned model family first".to_owned()),
        };
        let source_error = resolved.as_ref().err().cloned();
        let source = resolved.ok();
        let correlation = match (library.as_deref(), model.as_deref()) {
            (Some(library_name), Some(model_name)) => app
                .state
                .model_library_manager
                .get_library(library_name)
                .and_then(|library| library.model_correlation.get(model_name))
                .cloned()
                .unwrap_or_default(),
            _ => ModelCorrelationState::default(),
        };
        let expected_source = source.as_ref().and_then(|resolved| {
            ModelSourceEvidenceBinding::try_new_project_bound(
                &resolved.definition.base.name,
                resolved.source_id,
                resolved.model_digest,
                resolved.model_revision,
            )
            .ok()
        });
        let selected_id = app
            .state
            .workbench
            .model_correlation
            .selected_suite_id
            .as_deref();
        let suite = selected_id
            .and_then(|id| correlation.latest_suite(id))
            .or_else(|| {
                expected_source.as_ref().and_then(|expected| {
                    correlation
                        .suite_lineages()
                        .into_iter()
                        .find(|suite| suite.source == *expected)
                })
            })
            .or_else(|| correlation.suite_lineages().into_iter().next())
            .cloned();
        let source_current = suite
            .as_ref()
            .zip(expected_source.as_ref())
            .is_some_and(|(suite, expected)| suite.source == *expected);
        let (evaluation, evaluation_error) = suite.as_ref().map_or((None, None), |suite| {
            if suite.metrics.is_empty() {
                (None, None)
            } else {
                match CorrelationEvaluation::evaluate(suite) {
                    Ok(evaluation) => (Some(evaluation), None),
                    Err(error) => (None, Some(error.to_string())),
                }
            }
        });
        let current_evidence = suite.as_ref().map_or_else(Vec::new, |suite| {
            let mut evidence = correlation
                .evidence
                .iter()
                .filter(|evidence| {
                    evidence.suite_id.eq_ignore_ascii_case(&suite.id)
                        && evidence.suite_revision == suite.revision
                        && evidence.validate_current(suite).is_ok()
                })
                .cloned()
                .collect::<Vec<_>>();
            evidence.sort_by_key(|record| (record.reviewed_at_unix_ms, record.id.clone()));
            evidence
        });
        let dataset_count = suite
            .as_ref()
            .and_then(|suite| suite.latest_datasets().ok())
            .map_or(0, |datasets| datasets.len());
        let metric_count = suite.as_ref().map_or(0, |suite| {
            suite
                .metrics
                .iter()
                .filter(|metric| metric.release_role == CorrelationReleaseRole::Review)
                .count()
        });
        let passed_metrics = evaluation.as_ref().map_or(0, |evaluation| {
            evaluation
                .metric_outcomes
                .iter()
                .filter(|outcome| {
                    outcome.release_role == CorrelationReleaseRole::Review && outcome.passed
                })
                .count()
        });
        let coverage_percent = evaluation.as_ref().and_then(|evaluation| {
            (!evaluation.metric_outcomes.is_empty()).then(|| {
                evaluation
                    .metric_outcomes
                    .iter()
                    .map(|outcome| outcome.coverage.get())
                    .sum::<f64>()
                    / evaluation.metric_outcomes.len() as f64
                    * 100.0
            })
        });
        let worst_normalized_error = evaluation.as_ref().and_then(|evaluation| {
            evaluation
                .metric_outcomes
                .iter()
                .map(|outcome| outcome.aggregate_normalized_error.get())
                .max_by(f64::total_cmp)
        });
        let open_dispositions = suite.as_ref().map_or(0, |suite| {
            let mut current = BTreeMap::new();
            for disposition in &suite.dispositions {
                current.insert(
                    (
                        disposition.metric_id.to_ascii_lowercase(),
                        disposition.reference_observation_id.to_ascii_lowercase(),
                    ),
                    disposition,
                );
            }
            evaluation.as_ref().map_or(0, |evaluation| {
                evaluation
                    .metric_outcomes
                    .iter()
                    .flat_map(|outcome| {
                        outcome
                            .residuals
                            .iter()
                            .map(move |residual| (outcome, residual))
                    })
                    .filter(|(outcome, residual)| {
                        residual.normalized_error.get() > 1.0
                            && !current.contains_key(&(
                                outcome.metric_id.to_ascii_lowercase(),
                                residual.reference_observation_id.to_ascii_lowercase(),
                            ))
                    })
                    .count()
            })
        });
        let suite_count = correlation.suite_lineages().len();
        let selected_suite_name = suite.as_ref().map_or_else(
            || {
                model.as_ref().map_or_else(
                    || "No correlation suite selected".to_owned(),
                    |model| format!("{model} · no retained correlation suite"),
                )
            },
            |suite| format!("{} · revision {}", suite.name, suite.revision.get()),
        );
        Self {
            model: model.clone(),
            source_error,
            suite_name: selected_suite_name,
            correlation,
            suite,
            evaluation,
            evaluation_error,
            current_evidence,
            source_current,
            suite_count,
            dataset_count,
            metric_count,
            passed_metrics,
            coverage_percent,
            worst_normalized_error,
            open_dispositions,
        }
    }

    fn model_label(&self) -> &str {
        self.model.as_deref().unwrap_or("No model selected")
    }

    fn state_label(&self) -> (&str, Tone) {
        if self.source_error.is_some() {
            ("source binding unavailable", Tone::Error)
        } else if self.suite_count == 0 {
            ("correlation suite not configured", Tone::Warning)
        } else if !self.source_current {
            ("suite bound to an earlier model revision", Tone::Warning)
        } else if self.evaluation_error.is_some() {
            ("correlation configuration blocked", Tone::Error)
        } else if self.open_dispositions > 0 {
            ("metric dispositions open", Tone::Warning)
        } else if self.current_evidence.is_empty() {
            ("review evidence not retained", Tone::Warning)
        } else if !self
            .current_evidence
            .last()
            .is_some_and(CorrelationEvidence::approved)
        {
            ("review approval not retained", Tone::Error)
        } else {
            ("correlation evidence current", Tone::Good)
        }
    }

    fn metrics(&self) -> [Metric; 4] {
        [
            Metric {
                label: "Datasets",
                value: self.dataset_count.to_string(),
                detail: if self.dataset_count == 0 {
                    "no retained reference evidence".to_owned()
                } else {
                    "bench · silicon · vendor · oracle · simulation".to_owned()
                },
                tone: if self.dataset_count == 0 {
                    Tone::Warning
                } else {
                    Tone::Neutral
                },
            },
            Metric {
                label: "Metric gates",
                value: format!("{} / {}", self.passed_metrics, self.metric_count),
                detail: format!("{} open dispositions", self.open_dispositions),
                tone: if self.metric_count > 0
                    && self.passed_metrics == self.metric_count
                    && self.open_dispositions == 0
                {
                    Tone::Good
                } else {
                    Tone::Warning
                },
            },
            Metric {
                label: "Envelope coverage",
                value: self
                    .coverage_percent
                    .map(|value| format!("{value:.1}%"))
                    .unwrap_or_else(|| "No evidence".to_owned()),
                detail: "declared operating domain".to_owned(),
                tone: if self.coverage_percent.is_some() {
                    Tone::Neutral
                } else {
                    Tone::Warning
                },
            },
            Metric {
                label: "Worst normalized error",
                value: self
                    .worst_normalized_error
                    .map(|value| format!("{value:.3}×"))
                    .unwrap_or_else(|| "No evidence".to_owned()),
                detail: "retained evaluated comparisons".to_owned(),
                tone: match self.worst_normalized_error {
                    Some(value) if value <= 1.0 => Tone::Good,
                    Some(_) => Tone::Warning,
                    None => Tone::Warning,
                },
            },
        ]
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let size = ui.available_size().max(Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(viewport, 0.0, t.color.bg_app);
    let mut surface = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    surface.spacing_mut().item_spacing = Vec2::ZERO;

    let projection = CorrelationProjection::from_app(ui.ctx(), app);
    let projected_suite_id = projection.suite.as_ref().map(|suite| suite.id.clone());
    let creating_suite_for_current_source = app
        .state
        .workbench
        .model_correlation
        .selected_suite_id
        .is_none()
        && !projection.source_current
        && matches!(
            &app.state.workbench.model_correlation.dialog,
            Some(ModelCorrelationDialog::AddDataset(_))
        );
    if !creating_suite_for_current_source
        && app
            .state
            .workbench
            .model_correlation
            .selected_suite_id
            .as_deref()
            != projected_suite_id.as_deref()
    {
        let workspace = &mut app.state.workbench.model_correlation;
        workspace.selected_suite_id = projected_suite_id;
        workspace.selected_dataset_id = None;
        workspace.selected_metric_id = None;
    }
    outer_header(&mut surface, app, &projection);
    summary_header(&mut surface, &projection);
    metric_strip(&mut surface, &projection.metrics());
    correlation_notice(&mut surface, app);
    workspace_body(&mut surface, app, &projection);
    show_correlation_dialog(ui.ctx(), app);
}

fn outer_header(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let stacked = width < 700.0;
    let height = if stacked { 96.0 } else { OUTER_HEADER_H };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    bottom_rule(ui, rect, t.color.border);

    let actions_w = if stacked {
        (width - 24.0).max(1.0)
    } else {
        428.0_f32.min((width * 0.68).max(1.0))
    };
    let copy = Rect::from_min_max(
        Pos2::new(rect.left() + 14.0, rect.top()),
        Pos2::new(
            if stacked {
                rect.right() - 14.0
            } else {
                (rect.right() - actions_w - 24.0).max(rect.left() + 80.0)
            },
            if stacked {
                rect.top() + OUTER_HEADER_H
            } else {
                rect.bottom()
            },
        ),
    );
    paint_elided(
        ui,
        copy,
        "MODELS · BENCH DATA · REFERENCE EVIDENCE",
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        9.0,
    );
    paint_elided(
        ui,
        copy,
        "Measurement correlation",
        theme::sans(14.0, FontWeight::SemiBold),
        t.color.text,
        29.0,
    );

    let action_rect = if stacked {
        Rect::from_min_size(
            Pos2::new(
                rect.right() - actions_w - 12.0,
                rect.bottom() - t.metrics.ctl_h - 9.0,
            ),
            Vec2::new(actions_w, t.metrics.ctl_h),
        )
    } else {
        Rect::from_center_size(
            Pos2::new(rect.right() - actions_w * 0.5 - 12.0, rect.center().y),
            Vec2::new(actions_w, t.metrics.ctl_h),
        )
    };
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(action_rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = 6.0;

    let validate_blocker = correlation_action_blocker(app, projection, true, true);
    let validate = Button::new("Validate suite")
        .accent()
        .enabled(validate_blocker.is_none())
        .min_width(118.0)
        .max_width(118.0)
        .accessible_label("Validate the current correlation suite and record review evidence")
        .show(&mut actions);
    if let Some(blocker) = validate_blocker.as_deref() {
        validate.on_disabled_hover_text(blocker);
    } else if validate.clicked() {
        open_correlation_dialog(
            ui.ctx(),
            app,
            ModelCorrelationDialog::Review(CorrelationReviewDraft::default()),
        );
    }

    let viewer_enabled = projection.evaluation.is_some();
    let viewer = Button::new("Open viewer")
        .enabled(viewer_enabled)
        .min_width(96.0)
        .max_width(96.0)
        .show(&mut actions);
    if viewer.clicked() {
        select_section(app, ModelCorrelationSection::Comparison);
    } else if !viewer_enabled {
        viewer.on_disabled_hover_text(
            projection
                .evaluation_error
                .as_deref()
                .unwrap_or("Evaluate a complete metric suite before opening comparison evidence"),
        );
    }

    let metric_blocker = correlation_action_blocker(app, projection, true, false);
    let add_metric = Button::new("Add metric")
        .enabled(metric_blocker.is_none())
        .min_width(92.0)
        .max_width(92.0)
        .show(&mut actions);
    if let Some(blocker) = metric_blocker.as_deref() {
        add_metric.on_disabled_hover_text(blocker);
    } else if add_metric.clicked() {
        open_correlation_dialog(
            ui.ctx(),
            app,
            ModelCorrelationDialog::AddMetric(initial_metric_draft(projection)),
        );
    }

    let dataset_blocker = correlation_action_blocker(app, projection, false, false);
    let add_dataset = Button::new("Add dataset")
        .enabled(dataset_blocker.is_none())
        .min_width(98.0)
        .max_width(98.0)
        .show(&mut actions);
    if let Some(blocker) = dataset_blocker.as_deref() {
        add_dataset.on_disabled_hover_text(blocker);
    } else if add_dataset.clicked() {
        if projection.suite.is_some() && !projection.source_current {
            app.state.workbench.model_correlation.selected_suite_id = None;
        }
        open_correlation_dialog(
            ui.ctx(),
            app,
            ModelCorrelationDialog::AddDataset(CorrelationDatasetDraft::default()),
        );
    }
    describe_region(
        ui,
        &response,
        "Measurement correlation",
        "Models, bench data, and reference evidence workspace",
    );
}

fn summary_header(ui: &mut Ui, projection: &CorrelationProjection) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let compact = width <= COMPACT_BREAKPOINT;
    let height = if compact { 132.0 } else { SUMMARY_H };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    bottom_rule(ui, rect, t.color.border);

    let mark = Rect::from_min_size(
        Pos2::new(rect.left() + 14.0, rect.top() + 14.0),
        Vec2::splat(48.0),
    );
    ui.painter().rect(
        mark,
        t.radius,
        t.color.bg_panel_2,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        mark.center(),
        Align2::CENTER_CENTER,
        "C",
        theme::mono(tokens::FS_4, FontWeight::SemiBold),
        t.color.accent,
    );

    let left = mark.right() + 12.0;
    let status_w = 202.0_f32.min(width * 0.30);
    let status_left = if compact {
        left
    } else {
        rect.right() - status_w - 12.0
    };
    let text_right = if compact {
        rect.right() - 12.0
    } else {
        (status_left - 12.0).max(left + 80.0)
    };
    let copy = Rect::from_min_max(
        Pos2::new(left, rect.top()),
        Pos2::new(text_right, rect.bottom()),
    );
    paint_elided(
        ui,
        copy,
        &format!("{} · CORRELATION EVIDENCE", projection.model_label()).to_uppercase(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        9.0,
    );
    paint_elided(
        ui,
        copy,
        &projection.suite_name,
        theme::sans(14.0, FontWeight::SemiBold),
        t.color.text,
        28.0,
    );
    let summary_rect = Rect::from_min_max(
        Pos2::new(left, rect.top() + 49.0),
        Pos2::new(
            text_right,
            if compact {
                rect.bottom() - 28.0
            } else {
                rect.bottom() - 6.0
            },
        ),
    );
    let galley = ui.painter().layout(
        SUMMARY.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        summary_rect.width().max(1.0),
    );
    ui.painter()
        .with_clip_rect(summary_rect)
        .galley(summary_rect.min, galley, t.color.text_dim);

    let (state, tone) = projection.state_label();
    let color = tone_color(&t, tone);
    let y = if compact {
        rect.bottom() - 14.0
    } else {
        rect.top() + 17.0
    };
    ui.painter()
        .circle_filled(Pos2::new(status_left + 4.0, y), 2.5, color);
    paint_elided(
        ui,
        Rect::from_min_max(
            Pos2::new(status_left + 13.0, rect.top()),
            Pos2::new(rect.right() - 12.0, rect.bottom()),
        ),
        state,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        color,
        if compact { rect.height() - 22.0 } else { 10.0 },
    );
    describe_region(
        ui,
        &response,
        &format!("{} correlation evidence", projection.model_label()),
        &format!("{}. {}", projection.suite_name, state),
    );
}

fn metric_strip(ui: &mut Ui, metrics: &[Metric; 4]) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let columns = if width <= COMPACT_BREAKPOINT { 2 } else { 4 };
    let rows = 4_usize.div_ceil(columns);
    let total_h = KPI_H * rows as f32;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, total_h), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    bottom_rule(ui, rect, t.color.border);
    let cell_w = width / columns as f32;

    for (index, metric) in metrics.iter().enumerate() {
        let row = index / columns;
        let column = index % columns;
        let cell = Rect::from_min_size(
            Pos2::new(
                rect.left() + cell_w * column as f32,
                rect.top() + KPI_H * row as f32,
            ),
            Vec2::new(cell_w, KPI_H),
        );
        if column > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        if row > 0 {
            ui.painter()
                .hline(cell.x_range(), cell.top(), Stroke::new(1.0, t.color.border));
        }
        let copy = cell.shrink2(Vec2::new(12.0, 0.0));
        paint_elided(
            ui,
            copy,
            metric.label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
            8.0,
        );
        paint_elided(
            ui,
            copy,
            &metric.value,
            theme::mono(tokens::FS_3, FontWeight::SemiBold),
            tone_color(&t, metric.tone),
            26.0,
        );
        paint_elided(
            ui,
            copy,
            &metric.detail,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            46.0,
        );
    }
    let description = metrics
        .iter()
        .map(|metric| format!("{}: {}; {}", metric.label, metric.value, metric.detail))
        .collect::<Vec<_>>()
        .join(". ");
    describe_region(ui, &response, "Correlation summary metrics", &description);
}

fn workspace_body(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
    let size = ui.available_size().max(Vec2::splat(1.0));
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    if rect.width() <= COMPACT_BREAKPOINT {
        compact_workspace(ui, rect, app, projection);
    } else {
        desktop_workspace(ui, rect, app, projection);
    }
}

fn desktop_workspace(
    ui: &mut Ui,
    rect: Rect,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let nav_rect = Rect::from_min_max(
        rect.min,
        Pos2::new((rect.left() + NAV_W).min(rect.right()), rect.bottom()),
    );
    let content_rect = Rect::from_min_max(Pos2::new(nav_rect.right(), rect.top()), rect.max);
    section_navigation(ui, nav_rect, app, false);
    section_content(ui, content_rect, app, projection);
}

fn compact_workspace(
    ui: &mut Ui,
    rect: Rect,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let nav_h = NAV_ROW_H + 1.0;
    let nav_rect = Rect::from_min_max(
        rect.min,
        Pos2::new(rect.right(), (rect.top() + nav_h).min(rect.bottom())),
    );
    let content_rect = Rect::from_min_max(Pos2::new(rect.left(), nav_rect.bottom()), rect.max);
    section_navigation(ui, nav_rect, app, true);
    section_content(ui, content_rect, app, projection);
}

fn section_navigation(ui: &mut Ui, rect: Rect, app: &mut RSpiceApp, compact: bool) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    if compact {
        ui.painter().hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        let mut nav = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::left_to_right(Align::Min)),
        );
        nav.spacing_mut().item_spacing = Vec2::ZERO;
        ScrollArea::horizontal()
            .id_salt("model-correlation-sections-compact")
            .auto_shrink([false, false])
            .show(&mut nav, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    for section in ModelCorrelationSection::ALL {
                        let selected = app.state.workbench.model_correlation.section == section;
                        let response = navigation_row(
                            ui,
                            section,
                            selected,
                            Vec2::new(130.0, NAV_ROW_H),
                            true,
                        );
                        if response.clicked() {
                            select_section(app, section);
                        }
                    }
                });
            });
        return;
    }

    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        Stroke::new(1.0, t.color.border),
    );
    let mut nav = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
    );
    nav.spacing_mut().item_spacing = Vec2::ZERO;
    for section in ModelCorrelationSection::ALL {
        let selected = app.state.workbench.model_correlation.section == section;
        let response = navigation_row(
            &mut nav,
            section,
            selected,
            Vec2::new(rect.width(), NAV_ROW_H),
            false,
        );
        if response.clicked() {
            select_section(app, section);
        }
    }
    let note_top = (rect.top() + NAV_ROW_H * 7.0).min(rect.bottom());
    let note = Rect::from_min_max(Pos2::new(rect.left(), note_top), rect.max);
    if note.height() > 65.0 {
        ui.painter()
            .hline(note.x_range(), note.top(), Stroke::new(1.0, t.color.border));
        let copy = note.shrink2(Vec2::new(12.0, 10.0));
        paint_elided(
            ui,
            copy,
            "EVIDENCE CONTRACT",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_dim,
            0.0,
        );
        let body = Rect::from_min_max(Pos2::new(copy.left(), copy.top() + 19.0), copy.max);
        let galley = ui.painter().layout(
            EVIDENCE_CONTRACT.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
            body.width().max(1.0),
        );
        ui.painter()
            .with_clip_rect(body)
            .galley(body.min, galley, t.color.text_faint);
    }
}

fn navigation_row(
    ui: &mut Ui,
    section: ModelCorrelationSection,
    selected: bool,
    size: Vec2,
    compact: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    ui.painter().rect_filled(
        rect,
        0.0,
        if selected {
            t.color.bg_active
        } else {
            t.color.bg_panel
        },
    );
    if response.hovered() && !selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if compact {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
        if selected {
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        }
    } else {
        ui.painter().hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        if selected {
            ui.painter().vline(
                rect.left() + 1.0,
                rect.y_range(),
                Stroke::new(2.0, t.color.accent),
            );
        }
    }
    let index = ModelCorrelationSection::ALL
        .iter()
        .position(|candidate| *candidate == section)
        .unwrap_or_default()
        + 1;
    ui.painter().text(
        Pos2::new(rect.left() + 11.0, rect.center().y),
        Align2::LEFT_CENTER,
        format!("{index:02}"),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if selected {
            t.color.accent
        } else {
            t.color.text_faint
        },
    );
    ui.painter().text(
        Pos2::new(rect.left() + 44.0, rect.center().y),
        Align2::LEFT_CENTER,
        section.label(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            section.label(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
    });
    response
}

fn section_content(
    ui: &mut Ui,
    rect: Rect,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let section = app.state.workbench.model_correlation.section;
    let mut content = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
    );
    content.spacing_mut().item_spacing = Vec2::ZERO;
    let initial_scroll_offset = app.state.workbench.model_correlation.scroll_offset;
    let output = ScrollArea::vertical()
        .id_salt(("model-correlation-content", section as u8))
        .vertical_scroll_offset(initial_scroll_offset)
        .auto_shrink([false, false])
        .show(&mut content, |ui| {
            section_heading(ui, section);
            section_body(ui, app, projection, section);
            ui.add_space(14.0);
        });
    app.state.workbench.model_correlation.scroll_offset = output.state.offset.y;
}

fn section_heading(ui: &mut Ui, section: ModelCorrelationSection) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 70.0), Sense::hover());
    bottom_rule(ui, rect, t.color.border);
    let (title, description) = section_copy(section);
    let copy = rect.shrink2(Vec2::new(12.0, 0.0));
    paint_elided(
        ui,
        copy,
        &format!("COR · {}", section.label().to_uppercase()),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        10.0,
    );
    paint_elided(
        ui,
        copy,
        title,
        theme::sans(tokens::FS_2, FontWeight::SemiBold),
        t.color.text,
        28.0,
    );
    paint_elided(
        ui,
        copy,
        description,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        49.0,
    );
    describe_region(ui, &response, title, description);
}

fn section_copy(section: ModelCorrelationSection) -> (&'static str, &'static str) {
    match section {
        ModelCorrelationSection::Datasets => (
            "Reference datasets, fixtures, and calibration",
            "Distinguish independent evidence from derived or duplicated sources.",
        ),
        ModelCorrelationSection::Conditions => (
            "Operating-envelope alignment and interpolation",
            "Make voltage, temperature, load, frequency, lot, unit, and fixture dimensions explicit.",
        ),
        ModelCorrelationSection::Metrics => (
            "Metric definitions, uncertainty, aggregation, and gates",
            "A correlation result is meaningful only with a declared comparison contract.",
        ),
        ModelCorrelationSection::Comparison => (
            "Categorical aligned samples and residual structure",
            "Inspect each retained reference observation without implying an undeclared continuous axis.",
        ),
        ModelCorrelationSection::Outliers => (
            "Outlier and exclusion disposition ledger",
            "Exclusion changes evidence scope and therefore requires an accountable, reviewable decision.",
        ),
        ModelCorrelationSection::Evidence => (
            "Reproduction bundle and independent review",
            "Record exactly what another reviewer needs to reproduce the correlation result.",
        ),
        ModelCorrelationSection::Gate => (
            "Qualification handoff and release meaning",
            "Correlation concludes evidence fitness; Model qualification owns final vector and promotion gates.",
        ),
    }
}

fn section_body(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: &CorrelationProjection,
    section: ModelCorrelationSection,
) {
    ui.add_space(10.0);
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            suite_selector(ui, app, projection);
            match section {
                ModelCorrelationSection::Datasets => datasets_section(ui, app, projection),
                ModelCorrelationSection::Conditions => conditions_section(ui, app, projection),
                ModelCorrelationSection::Metrics => metrics_section(ui, app, projection),
                ModelCorrelationSection::Comparison => comparison_section(ui, app, projection),
                ModelCorrelationSection::Outliers => outliers_section(ui, app, projection),
                ModelCorrelationSection::Evidence => evidence_section(ui, app, projection),
                ModelCorrelationSection::Gate => gate_section(ui, app, projection),
            }
        });
}

fn suite_selector(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn datasets_section(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn conditions_section(ui: &mut Ui, _app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn metrics_section(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn comparison_section(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn comparison_plot(ui: &mut Ui, outcome: &CorrelationMetricOutcome) {
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

fn outliers_section(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn evidence_section(ui: &mut Ui, _app: &mut RSpiceApp, projection: &CorrelationProjection) {
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

fn gate_section(ui: &mut Ui, app: &mut RSpiceApp, projection: &CorrelationProjection) {
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
            app.state.workbench.models_page = super::super::state::ModelsPage::Qualification;
            if let Err(error) = app.state.workbench.navigate(
                SurfaceRoute::surface(SurfaceId::Models),
                RouteTransitionSource::User,
            ) {
                app.state.workbench.model_correlation.notice = Some(error.to_string());
            }
        }
    });
}

fn open_affected_vectors(app: &mut RSpiceApp) -> Result<(), String> {
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

fn empty_state(ui: &mut Ui, projection: &CorrelationProjection, title: &str, detail: &str) {
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

fn correlation_notice(ui: &mut Ui, app: &mut RSpiceApp) {
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

fn select_section(app: &mut RSpiceApp, section: ModelCorrelationSection) {
    app.state.workbench.model_correlation.section = section;
    app.state.workbench.model_correlation.scroll_offset = 0.0;
}

fn correlation_action_blocker(
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

fn initial_metric_draft(projection: &CorrelationProjection) -> CorrelationMetricDraft {
    let mut draft = CorrelationMetricDraft::default();
    draft.calculation = CorrelationCalculationDraft::AbsoluteLinear;
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

fn metric_draft_from_definition(
    metric: &crate::state::model_library::CorrelationMetricDefinition,
) -> CorrelationMetricDraft {
    let (domain_enabled, domain_axis, domain_unit, domain_minimum, domain_maximum) =
        metric.domain.as_ref().map_or_else(
            || {
                (
                    false,
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                )
            },
            |domain| {
                (
                    true,
                    domain.axis.clone(),
                    domain.unit.clone(),
                    domain.minimum.get().to_string(),
                    domain.maximum.get().to_string(),
                )
            },
        );
    let (alignment, alignment_axis, extrapolation, extrapolation_fraction) = match &metric.alignment
    {
        CorrelationAlignmentPolicy::ExactOnly => (
            CorrelationAlignmentDraft::Exact,
            String::new(),
            CorrelationExtrapolationDraft::Forbid,
            "0.05".to_owned(),
        ),
        CorrelationAlignmentPolicy::MonotoneInterpolation {
            axis,
            extrapolation,
        } => {
            let (draft_policy, fraction) = match extrapolation {
                crate::state::model_library::CorrelationExtrapolationPolicy::Forbid => {
                    (CorrelationExtrapolationDraft::Forbid, "0.05".to_owned())
                }
                crate::state::model_library::CorrelationExtrapolationPolicy::Limited {
                    max_axis_span_fraction,
                } => (
                    CorrelationExtrapolationDraft::Limited,
                    max_axis_span_fraction.get().to_string(),
                ),
            };
            (
                CorrelationAlignmentDraft::MonotoneInterpolation,
                axis.clone(),
                draft_policy,
                fraction,
            )
        }
    };
    CorrelationMetricDraft {
        id: metric.id.clone(),
        name: metric.name.clone(),
        quantity: metric.quantity.clone(),
        reference_dataset_id: metric.reference_dataset_id.clone(),
        simulation_dataset_id: metric.simulation_dataset_id.clone(),
        calculation: match metric.calculation {
            CorrelationCalculation::AbsoluteLinear => CorrelationCalculationDraft::AbsoluteLinear,
            CorrelationCalculation::AbsoluteDecibels => {
                CorrelationCalculationDraft::AbsoluteDecibels
            }
            CorrelationCalculation::Relative => CorrelationCalculationDraft::Relative,
            CorrelationCalculation::WeightedRelative => {
                CorrelationCalculationDraft::WeightedRelative
            }
            CorrelationCalculation::PhaseWrappedDegrees => {
                CorrelationCalculationDraft::PhaseWrapped
            }
        },
        domain_enabled,
        domain_axis,
        domain_unit,
        domain_minimum,
        domain_maximum,
        limit: metric.limit.get().to_string(),
        uncertainty_multiplier: metric.uncertainty_multiplier.get().to_string(),
        minimum_coverage: metric.minimum_coverage.get().to_string(),
        aggregation: match metric.aggregation {
            CorrelationAggregation::EveryPoint => CorrelationAggregationDraft::EveryPoint,
            CorrelationAggregation::WorstCondition => CorrelationAggregationDraft::WorstCase,
            CorrelationAggregation::Percentile95 => CorrelationAggregationDraft::Percentile95,
            CorrelationAggregation::RootMeanSquare => CorrelationAggregationDraft::RootMeanSquare,
        },
        alignment,
        alignment_axis,
        extrapolation,
        extrapolation_fraction,
        release_role: match metric.release_role {
            CorrelationReleaseRole::Review => CorrelationReleaseRoleDraft::Review,
            CorrelationReleaseRole::Advisory => CorrelationReleaseRoleDraft::Advisory,
        },
        validation_error: None,
    }
}

fn section_panel(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(0))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_min_width(ui.available_width().max(1.0));
            egui::Frame::new()
                .fill(t.color.bg_panel_2)
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width().max(1.0));
                    ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                });
            let (rule, _) = ui.allocate_exact_size(
                Vec2::new(ui.available_width().max(1.0), 1.0),
                Sense::hover(),
            );
            ui.painter().hline(
                rule.x_range(),
                rule.center().y,
                Stroke::new(1.0, t.color.border),
            );
            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width().max(1.0));
                    add_contents(ui);
                });
        });
}

fn table_header(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
}

fn key_value(ui: &mut Ui, key: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Grid::new(ui.next_auto_id())
        .num_columns(2)
        .min_col_width(148.0)
        .spacing(Vec2::new(18.0, 7.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(key)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text),
            );
            ui.end_row();
        });
}

fn error_panel(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.err))
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.label(
                egui::RichText::new("Correlation configuration blocked")
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.err),
            );
            ui.label(error);
        });
}

fn status_text(ui: &mut Ui, passed: bool, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(
            egui::RichText::new("●")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(if passed { t.color.ok } else { t.color.err }),
        );
        ui.label(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(if passed { t.color.ok } else { t.color.err }),
        );
    });
}

fn short_digest(value: &str) -> String {
    const PREFIX: usize = 12;
    if value.chars().count() <= PREFIX {
        value.to_owned()
    } else {
        format!("{}…", value.chars().take(PREFIX).collect::<String>())
    }
}

const fn dataset_class_label(value: CorrelationDatasetClass) -> &'static str {
    match value {
        CorrelationDatasetClass::BenchMeasurement => "Bench measurement",
        CorrelationDatasetClass::SiliconCharacterization => "Silicon characterization",
        CorrelationDatasetClass::VendorReference => "Vendor reference",
        CorrelationDatasetClass::IndependentOracle => "Independent oracle",
        CorrelationDatasetClass::ModelSimulation => "Model simulation",
    }
}

const fn calculation_label(value: CorrelationCalculation) -> &'static str {
    match value {
        CorrelationCalculation::AbsoluteLinear => "Absolute linear error",
        CorrelationCalculation::AbsoluteDecibels => "Absolute dB error",
        CorrelationCalculation::Relative => "Relative error",
        CorrelationCalculation::WeightedRelative => "Weighted relative error",
        CorrelationCalculation::PhaseWrappedDegrees => "Phase-wrapped degrees",
    }
}

const fn aggregation_label(value: CorrelationAggregation) -> &'static str {
    match value {
        CorrelationAggregation::EveryPoint => "Every point",
        CorrelationAggregation::WorstCondition => "Worst condition",
        CorrelationAggregation::Percentile95 => "95th percentile",
        CorrelationAggregation::RootMeanSquare => "Root mean square",
    }
}

fn alignment_label(value: &CorrelationAlignmentPolicy) -> String {
    match value {
        CorrelationAlignmentPolicy::ExactOnly => "Exact coordinates".to_owned(),
        CorrelationAlignmentPolicy::MonotoneInterpolation {
            axis,
            extrapolation,
        } => match extrapolation {
            crate::state::model_library::CorrelationExtrapolationPolicy::Forbid => {
                format!("Monotone interpolation on {axis}; extrapolation forbidden")
            }
            crate::state::model_library::CorrelationExtrapolationPolicy::Limited {
                max_axis_span_fraction,
            } => format!(
                "Monotone interpolation on {axis}; extrapolation ≤ {:.3} span",
                max_axis_span_fraction.get()
            ),
        },
    }
}

const fn release_role_label(value: CorrelationReleaseRole) -> &'static str {
    match value {
        CorrelationReleaseRole::Review => "Review",
        CorrelationReleaseRole::Advisory => "Advisory",
    }
}

const fn alignment_evidence_label(value: CorrelationAlignmentEvidence) -> &'static str {
    match value {
        CorrelationAlignmentEvidence::Exact => "Exact",
        CorrelationAlignmentEvidence::Interpolated => "Interpolated",
        CorrelationAlignmentEvidence::Extrapolated => "Extrapolated",
    }
}

const fn outlier_decision_label(value: CorrelationOutlierDecision) -> &'static str {
    match value {
        CorrelationOutlierDecision::Retain => "Retain in gate",
        CorrelationOutlierDecision::ExcludeFixtureFault => "Exclude: fixture fault",
        CorrelationOutlierDecision::LimitOnlyEvidence => "Limit-only evidence",
    }
}

fn open_correlation_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    dialog: ModelCorrelationDialog,
) {
    match model_correlation_controller::current_source_binding(app) {
        Ok(source) => {
            app.state.workbench.model_correlation.dialog_source = Some(source);
            ctx.data_mut(|data| {
                data.remove::<String>(correlation_dialog_error_id());
            });
            app.state.workbench.model_correlation.dialog = Some(dialog);
        }
        Err(error) => {
            app.state.workbench.model_correlation.notice = Some(error);
        }
    }
}

fn correlation_dialog_source_blocker(app: &RSpiceApp) -> Option<String> {
    let Some(opening) = app.state.workbench.model_correlation.dialog_source.as_ref() else {
        return Some(
            "The dialog has no opening source identity; close it and reopen from the current model"
                .to_owned(),
        );
    };
    match model_correlation_controller::current_source_binding(app) {
        Ok(current) if current == *opening => None,
        Ok(_) => Some(
            "The selected model source changed after this dialog opened; cancel and reopen it"
                .to_owned(),
        ),
        Err(error) => Some(format!(
            "The dialog's opening model source is no longer available: {error}"
        )),
    }
}

fn clear_correlation_dialog_runtime(ctx: &egui::Context) {
    ctx.data_mut(|data| {
        data.remove::<String>(correlation_dialog_error_id());
    });
}

fn dialog_field_width(ui: &Ui) -> f32 {
    (ui.ctx().screen_rect().width() - 220.0).clamp(140.0, 430.0)
}

fn show_correlation_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(mut dialog) = app.state.workbench.model_correlation.dialog.take() else {
        return;
    };
    let (title, primary, description, contains_multiline) = match &dialog {
        ModelCorrelationDialog::AddDataset(_) => (
            "Add correlation dataset",
            "Retain dataset",
            "Import immutable, source-identified correlation evidence.",
            true,
        ),
        ModelCorrelationDialog::AddMetric(_) => (
            "Add correlation metric",
            "Retain metric",
            "Define a unit-aware comparison contract for the selected suite.",
            false,
        ),
        ModelCorrelationDialog::Review(_) => (
            "Validate correlation suite",
            "Record review",
            "Retain an independent review of the exact selected suite revision.",
            false,
        ),
        ModelCorrelationDialog::Disposition(_) => (
            "Review outlier disposition",
            "Record disposition",
            "Retain an append-only, independently reviewed residual disposition.",
            false,
        ),
    };
    let projection = CorrelationProjection::from_app(ctx, app);
    let action_blocker = match &dialog {
        ModelCorrelationDialog::AddDataset(_) => {
            correlation_action_blocker(app, &projection, false, false)
        }
        ModelCorrelationDialog::AddMetric(_) | ModelCorrelationDialog::Disposition(_) => {
            correlation_action_blocker(app, &projection, true, false)
        }
        ModelCorrelationDialog::Review(_) => {
            correlation_action_blocker(app, &projection, true, true)
        }
    };
    let source_blocker = correlation_dialog_source_blocker(app);
    let submit_error = ctx.data(|data| data.get_temp::<String>(correlation_dialog_error_id()));
    let draft_error = match &dialog {
        ModelCorrelationDialog::AddDataset(draft) => draft.validation_error.as_deref(),
        ModelCorrelationDialog::AddMetric(draft) => draft.validation_error.as_deref(),
        ModelCorrelationDialog::Review(draft) => draft.validation_error.as_deref(),
        ModelCorrelationDialog::Disposition(draft) => draft.validation_error.as_deref(),
    };
    let visible_error = submit_error
        .or_else(|| draft_error.map(str::to_owned))
        .or_else(|| source_blocker.clone())
        .or_else(|| action_blocker.clone());

    let mut shell = Dialog::new("MODELS · CORRELATION", title, primary)
        .description(description)
        .size(DialogSize::Transaction)
        .ghost("Cancel")
        .primary_enabled(source_blocker.is_none() && action_blocker.is_none())
        .primary_on_enter(!contains_multiline);
    if let Some(error) = visible_error.as_deref() {
        shell = shell.transaction_state(
            DialogTransactionTone::Error,
            "Correlation record was not retained",
            error,
        );
    }
    let choice = shell.show(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 8.0;
        match &mut dialog {
            ModelCorrelationDialog::AddDataset(draft) => dataset_dialog_body(ui, app, draft),
            ModelCorrelationDialog::AddMetric(draft) => metric_dialog_body(ui, draft),
            ModelCorrelationDialog::Review(draft) => review_dialog_body(ui, app, draft),
            ModelCorrelationDialog::Disposition(draft) => disposition_dialog_body(ui, draft),
        }
    });

    let keep_open = match choice {
        DialogChoice::None | DialogChoice::Secondary => true,
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            clear_correlation_dialog_runtime(ctx);
            app.state.workbench.model_correlation.dialog_source = None;
            false
        }
        DialogChoice::Primary => {
            let result = correlation_dialog_source_blocker(app).map_or_else(
                || match &dialog {
                    ModelCorrelationDialog::AddDataset(draft) => {
                        model_correlation_controller::append_dataset(app, draft)
                    }
                    ModelCorrelationDialog::AddMetric(draft) => {
                        model_correlation_controller::append_metric(app, draft)
                    }
                    ModelCorrelationDialog::Review(draft) => {
                        model_correlation_controller::append_review_evidence(app, draft)
                    }
                    ModelCorrelationDialog::Disposition(draft) => {
                        model_correlation_controller::append_disposition(app, draft)
                    }
                },
                Err,
            );
            match result {
                Ok(()) => {
                    clear_correlation_dialog_runtime(ctx);
                    app.state.workbench.model_correlation.dialog_source = None;
                    false
                }
                Err(error) => {
                    ctx.data_mut(|data| {
                        data.insert_temp(correlation_dialog_error_id(), error);
                    });
                    true
                }
            }
        }
    };
    if keep_open {
        app.state.workbench.model_correlation.dialog = Some(dialog);
    }
}

fn form_text_row(ui: &mut Ui, label: &str, value: &mut String) {
    let label_response = ui.label(label);
    let input_response = ui.add_sized(
        [dialog_field_width(ui), Tokens::get(ui.ctx()).metrics.ctl_h],
        egui::TextEdit::singleline(value),
    );
    ui.ctx().accesskit_node_builder(input_response.id, |node| {
        node.push_labelled_by(label_response.id.accesskit_id());
    });
    ui.end_row();
}

#[derive(Debug, Clone)]
struct RetainedTraceOption {
    name: String,
    sample_count: usize,
}

#[derive(Debug, Clone)]
struct RetainedAnalysisOption {
    id: u64,
    label: String,
    kind: String,
    traces: Vec<RetainedTraceOption>,
}

#[derive(Debug, Clone)]
struct RetainedRunOption {
    id: String,
    label: String,
    plan_id: String,
    execution_target: String,
    analyses: Vec<RetainedAnalysisOption>,
}

fn retained_run_options(app: &RSpiceApp) -> Vec<RetainedRunOption> {
    let Some(source) = app.state.workbench.model_correlation.dialog_source.as_ref() else {
        return Vec::new();
    };
    let Some(source_id) = source.source_id else {
        return Vec::new();
    };
    app.state
        .simulation
        .runs
        .iter()
        .filter(|run| {
            run.lifecycle == crate::state::SimulationRunLifecycle::Completed
                && run.success
                && run.validate_provenance().is_ok()
        })
        .filter_map(|run| {
            let receipt = run.prepared_receipt()?;
            let plan_id = receipt.simulation_plan_id()?;
            let execution_target = run.execution_target?;
            if !receipt.project_model_sources().iter().any(|candidate| {
                candidate.source_id() == source_id
                    && candidate
                        .model_name()
                        .eq_ignore_ascii_case(&source.model_id)
                    && candidate.revision() == source.source_revision
                    && candidate.content_digest() == source.source_digest
            }) {
                return None;
            }
            let analyses = run
                .analyses
                .iter()
                .filter(|analysis| analysis.success && analysis.provenance().is_some())
                .filter_map(|analysis| {
                    let traces = analysis
                        .waveforms
                        .iter()
                        .filter(|trace| {
                            !trace.x.is_empty()
                                && trace.x.len() == trace.y.len()
                                && trace
                                    .x
                                    .iter()
                                    .chain(trace.y.iter())
                                    .all(|value| value.is_finite())
                        })
                        .map(|trace| RetainedTraceOption {
                            name: trace.name.clone(),
                            sample_count: trace.x.len(),
                        })
                        .collect::<Vec<_>>();
                    (!traces.is_empty()).then(|| RetainedAnalysisOption {
                        id: analysis.id,
                        label: analysis.label.clone(),
                        kind: analysis.analysis_type.display_name().to_owned(),
                        traces,
                    })
                })
                .collect::<Vec<_>>();
            (!analyses.is_empty()).then(|| RetainedRunOption {
                id: run.run_id.to_string(),
                label: run.label.clone(),
                plan_id: plan_id.to_string(),
                execution_target: execution_target.label().to_owned(),
                analyses,
            })
        })
        .collect()
}

fn dataset_dialog_body(ui: &mut Ui, app: &RSpiceApp, draft: &mut CorrelationDatasetDraft) {
    let projection = CorrelationProjection::from_app(ui.ctx(), app);
    let current_source = model_correlation_controller::current_source_binding(app).ok();
    let current_suite = current_source.as_ref().and_then(|source| {
        projection
            .correlation
            .suite_lineages()
            .into_iter()
            .find(|suite| suite.source == *source)
    });
    ui.label("Import immutable UTF-8 CSV evidence and retain its source identity and checksum.");
    egui::Grid::new("correlation-dataset-form")
        .num_columns(2)
        .min_col_width(136.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            form_text_row(ui, "Dataset ID", &mut draft.id);
            form_text_row(ui, "Name", &mut draft.name);
            if let Some(suite) = current_suite {
                ui.label("Suite owner identity");
                ui.monospace(&suite.owner_id);
                ui.end_row();
            } else {
                form_text_row(ui, "Suite owner identity", &mut draft.suite_owner_id);
            }
            ui.label("Evidence class");
            egui::ComboBox::from_id_salt("correlation-dataset-class")
                .selected_text(draft.class.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationDatasetClassDraft::ALL {
                        ui.selectable_value(&mut draft.class, value, value.label());
                    }
                });
            ui.end_row();
            if draft.class != CorrelationDatasetClassDraft::Simulated {
                form_text_row(ui, "Authority", &mut draft.authority);
                form_text_row(ui, "Device or lot", &mut draft.device_or_lot);
                form_text_row(ui, "Fixture", &mut draft.fixture);
                form_text_row(ui, "Calibration", &mut draft.calibration);
                form_text_row(ui, "Source name", &mut draft.source_name);
            }
        });
    ui.add_space(8.0);
    if draft.class == CorrelationDatasetClassDraft::Simulated {
        retained_simulation_picker(ui, app, draft);
    } else {
        ui.label("CSV source");
        ui.label(
            "Required columns: id, quantity, value, unit. Optional: uncertainty, weight, condition:<name>[<unit>].",
        );
        ui.add_sized(
            [ui.available_width().max(1.0), 170.0],
            egui::TextEdit::multiline(&mut draft.csv)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .code_editor()
                .desired_rows(8),
        );
    }
}

fn retained_simulation_picker(ui: &mut Ui, app: &RSpiceApp, draft: &mut CorrelationDatasetDraft) {
    let runs = retained_run_options(app);
    section_panel(ui, "Authenticated retained simulation", |ui| {
        ui.label(
            "Select a completed run prepared against this exact project model revision. RSpice generates and hashes the canonical CSV from the retained waveform.",
        );
        ui.add_space(6.0);
        egui::Grid::new("correlation-retained-simulation-form")
            .num_columns(2)
            .min_col_width(136.0)
            .spacing(Vec2::new(14.0, 8.0))
            .show(ui, |ui| {
                ui.label("Retained run");
                let selected_run_label = runs
                    .iter()
                    .find(|run| run.id == draft.retained_run_id)
                    .map_or("Select a completed run", |run| run.label.as_str());
                let previous_run = draft.retained_run_id.clone();
                egui::ComboBox::from_id_salt("correlation-retained-run")
                    .selected_text(selected_run_label)
                    .width(dialog_field_width(ui))
                    .show_ui(ui, |ui| {
                        for run in &runs {
                            ui.selectable_value(
                                &mut draft.retained_run_id,
                                run.id.clone(),
                                &run.label,
                            );
                        }
                    });
                if draft.retained_run_id != previous_run {
                    draft.retained_analysis_id.clear();
                    draft.retained_trace_name.clear();
                }
                ui.end_row();

                let selected_run = runs.iter().find(|run| run.id == draft.retained_run_id);
                ui.label("Analysis");
                let selected_analysis_label = selected_run
                    .and_then(|run| {
                        run.analyses
                            .iter()
                            .find(|analysis| analysis.id.to_string() == draft.retained_analysis_id)
                    })
                    .map_or("Select an analysis".to_owned(), |analysis| {
                        format!("{} · {}", analysis.kind, analysis.label)
                    });
                let previous_analysis = draft.retained_analysis_id.clone();
                ui.add_enabled_ui(selected_run.is_some(), |ui| {
                    egui::ComboBox::from_id_salt("correlation-retained-analysis")
                        .selected_text(selected_analysis_label)
                        .width(dialog_field_width(ui))
                        .show_ui(ui, |ui| {
                            if let Some(run) = selected_run {
                                for analysis in &run.analyses {
                                    ui.selectable_value(
                                        &mut draft.retained_analysis_id,
                                        analysis.id.to_string(),
                                        format!("{} · {}", analysis.kind, analysis.label),
                                    );
                                }
                            }
                        });
                });
                if draft.retained_analysis_id != previous_analysis {
                    draft.retained_trace_name.clear();
                }
                ui.end_row();

                let selected_analysis = selected_run.and_then(|run| {
                    run.analyses
                        .iter()
                        .find(|analysis| analysis.id.to_string() == draft.retained_analysis_id)
                });
                ui.label("Waveform");
                let selected_trace_label = selected_analysis
                    .and_then(|analysis| {
                        analysis
                            .traces
                            .iter()
                            .find(|trace| trace.name == draft.retained_trace_name)
                    })
                    .map_or("Select a retained waveform".to_owned(), |trace| {
                        format!("{} · {} samples", trace.name, trace.sample_count)
                    });
                ui.add_enabled_ui(selected_analysis.is_some(), |ui| {
                    egui::ComboBox::from_id_salt("correlation-retained-trace")
                        .selected_text(selected_trace_label)
                        .width(dialog_field_width(ui))
                        .show_ui(ui, |ui| {
                            if let Some(analysis) = selected_analysis {
                                for trace in &analysis.traces {
                                    ui.selectable_value(
                                        &mut draft.retained_trace_name,
                                        trace.name.clone(),
                                        format!("{} · {} samples", trace.name, trace.sample_count),
                                    );
                                }
                            }
                        });
                });
                ui.end_row();
            });
        if let Some(run) = runs.iter().find(|run| run.id == draft.retained_run_id) {
            ui.add_space(6.0);
            key_value(ui, "Simulation plan", &run.plan_id);
            key_value(ui, "Execution target", &run.execution_target);
        } else if runs.is_empty() {
            ui.add_space(6.0);
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "No successfully completed retained run authenticates this exact model revision. Run the current simulation plan, then reopen this dialog.",
            );
        }
    });
}

fn metric_dialog_body(ui: &mut Ui, draft: &mut CorrelationMetricDraft) {
    ui.label("Define a unit-aware, source-bound comparison contract.");
    egui::Grid::new("correlation-metric-form")
        .num_columns(2)
        .min_col_width(150.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            form_text_row(ui, "Metric ID", &mut draft.id);
            form_text_row(ui, "Name", &mut draft.name);
            form_text_row(ui, "Quantity", &mut draft.quantity);
            form_text_row(ui, "Reference dataset ID", &mut draft.reference_dataset_id);
            form_text_row(
                ui,
                "Simulation dataset ID",
                &mut draft.simulation_dataset_id,
            );
            ui.label("Calculation");
            egui::ComboBox::from_id_salt("correlation-metric-calculation")
                .selected_text(draft.calculation.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationCalculationDraft::ALL {
                        ui.selectable_value(&mut draft.calculation, value, value.label());
                    }
                });
            ui.end_row();
            form_text_row(ui, "Limit", &mut draft.limit);
            form_text_row(
                ui,
                "Uncertainty multiplier",
                &mut draft.uncertainty_multiplier,
            );
            form_text_row(ui, "Minimum coverage (0..1)", &mut draft.minimum_coverage);
            ui.label("Aggregation");
            egui::ComboBox::from_id_salt("correlation-metric-aggregation")
                .selected_text(draft.aggregation.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationAggregationDraft::ALL {
                        ui.selectable_value(&mut draft.aggregation, value, value.label());
                    }
                });
            ui.end_row();
            ui.label("Alignment");
            egui::ComboBox::from_id_salt("correlation-metric-alignment")
                .selected_text(draft.alignment.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationAlignmentDraft::ALL {
                        ui.selectable_value(&mut draft.alignment, value, value.label());
                    }
                });
            ui.end_row();
            ui.label("Release role");
            egui::ComboBox::from_id_salt("correlation-metric-release-role")
                .selected_text(draft.release_role.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationReleaseRoleDraft::ALL {
                        ui.selectable_value(&mut draft.release_role, value, value.label());
                    }
                });
            ui.end_row();
        });
    ui.add_space(8.0);
    section_panel(ui, "Operating domain", |ui| {
        ui.checkbox(
            &mut draft.domain_enabled,
            "Restrict metric to a declared axis range",
        );
        ui.add_enabled_ui(draft.domain_enabled, |ui| {
            egui::Grid::new("correlation-metric-domain-form")
                .num_columns(2)
                .min_col_width(150.0)
                .spacing(Vec2::new(14.0, 8.0))
                .show(ui, |ui| {
                    form_text_row(ui, "Axis", &mut draft.domain_axis);
                    form_text_row(ui, "Unit", &mut draft.domain_unit);
                    form_text_row(ui, "Minimum", &mut draft.domain_minimum);
                    form_text_row(ui, "Maximum", &mut draft.domain_maximum);
                });
        });
    });
    ui.add_space(8.0);
    section_panel(ui, "Alignment and extrapolation", |ui| {
        ui.label(
            if draft.alignment == CorrelationAlignmentDraft::MonotoneInterpolation {
                "Declare the interpolation axis and the exact extrapolation boundary."
            } else {
                "Exact alignment never interpolates or extrapolates."
            },
        );
        ui.add_space(6.0);
        ui.add_enabled_ui(
            draft.alignment == CorrelationAlignmentDraft::MonotoneInterpolation,
            |ui| {
                egui::Grid::new("correlation-metric-alignment-form")
                    .num_columns(2)
                    .min_col_width(150.0)
                    .spacing(Vec2::new(14.0, 8.0))
                    .show(ui, |ui| {
                        form_text_row(ui, "Alignment axis", &mut draft.alignment_axis);
                        ui.label("Extrapolation");
                        egui::ComboBox::from_id_salt("correlation-metric-extrapolation")
                            .selected_text(draft.extrapolation.label())
                            .width(dialog_field_width(ui))
                            .show_ui(ui, |ui| {
                                for value in CorrelationExtrapolationDraft::ALL {
                                    ui.selectable_value(
                                        &mut draft.extrapolation,
                                        value,
                                        value.label(),
                                    );
                                }
                            });
                        ui.end_row();
                        form_text_row(
                            ui,
                            "Maximum span fraction",
                            &mut draft.extrapolation_fraction,
                        );
                    });
            },
        );
    });
}

fn review_dialog_body(ui: &mut Ui, app: &RSpiceApp, draft: &mut CorrelationReviewDraft) {
    let projection = CorrelationProjection::from_app(ui.ctx(), app);
    section_panel(ui, "Evaluation to be retained", |ui| {
        if let Some(evaluation) = projection.evaluation.as_ref() {
            status_text(
                ui,
                evaluation.passed,
                if evaluation.passed {
                    "numerical gate passed"
                } else {
                    "numerical gate failed"
                },
            );
            key_value(ui, "Suite", &evaluation.suite_id);
            key_value(
                ui,
                "Suite revision",
                &evaluation.suite_revision.get().to_string(),
            );
            key_value(ui, "Metrics", &evaluation.metric_outcomes.len().to_string());
            key_value(
                ui,
                "Suite SHA-256",
                &short_digest(&evaluation.suite_digest.to_string()),
            );
        } else {
            ui.label("The selected suite does not currently produce a valid evaluation.");
        }
    });
    ui.add_space(8.0);
    egui::Grid::new("correlation-review-form")
        .num_columns(2)
        .min_col_width(150.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            ui.label("Suite owner");
            ui.monospace(
                projection
                    .suite
                    .as_ref()
                    .map_or("Unavailable", |suite| suite.owner_id.as_str()),
            );
            ui.end_row();
            form_text_row(ui, "Independent reviewer", &mut draft.reviewer_id);
            ui.label("Decision");
            egui::ComboBox::from_id_salt("correlation-review-decision")
                .selected_text(draft.decision.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationReviewDecisionDraft::ALL {
                        let enabled = value != CorrelationReviewDecisionDraft::Approve
                            || projection
                                .evaluation
                                .as_ref()
                                .is_some_and(|evaluation| evaluation.passed);
                        ui.add_enabled_ui(enabled, |ui| {
                            ui.selectable_value(&mut draft.decision, value, value.label());
                        });
                    }
                });
            ui.end_row();
            form_text_row(ui, "Conclusion", &mut draft.conclusion);
        });
}

fn disposition_dialog_body(ui: &mut Ui, draft: &mut CorrelationOutlierDraft) {
    ui.label("Retain an append-only, independently reviewed disposition for this residual.");
    egui::Grid::new("correlation-disposition-form")
        .num_columns(2)
        .min_col_width(160.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            ui.label("Metric ID");
            ui.monospace(&draft.metric_id);
            ui.end_row();
            ui.label("Reference point");
            ui.monospace(&draft.reference_observation_id);
            ui.end_row();
            ui.label("Decision");
            egui::ComboBox::from_id_salt("correlation-outlier-decision")
                .selected_text(draft.decision.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationOutlierDecisionDraft::ALL {
                        ui.selectable_value(&mut draft.decision, value, value.label());
                    }
                });
            ui.end_row();
            form_text_row(ui, "Reason", &mut draft.reason);
            form_text_row(ui, "Owner", &mut draft.owner_id);
            form_text_row(ui, "Independent reviewer", &mut draft.reviewer_id);
        });
}

fn bottom_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, color));
}

fn paint_elided(
    ui: &Ui,
    rect: Rect,
    text: &str,
    font: egui::FontId,
    color: Color32,
    y_offset: f32,
) {
    let value = design_system::elide_text(ui, text, &font, rect.width().max(1.0));
    ui.painter().with_clip_rect(rect).text(
        Pos2::new(rect.left(), rect.top() + y_offset),
        Align2::LEFT_TOP,
        value,
        font,
        color,
    );
}

fn describe_region(ui: &Ui, response: &egui::Response, label: &str, description: &str) {
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, response.enabled(), label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Group);
        node.set_label(label);
        node.set_description(description);
    });
}

fn tone_color(t: &Tokens, tone: Tone) -> Color32 {
    match tone {
        Tone::Neutral => t.color.text,
        Tone::Good => t.color.ok,
        Tone::Warning => t.color.warn,
        Tone::Error => t.color.err,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_copy_is_complete_for_the_mockup_contract() {
        for section in ModelCorrelationSection::ALL {
            let (title, description) = section_copy(section);
            assert!(!title.is_empty());
            assert!(!description.is_empty());
        }
    }

    #[test]
    fn empty_projection_never_claims_correlation_evidence() {
        let projection = CorrelationProjection {
            model: Some("dut".to_owned()),
            source_error: None,
            suite_name: "dut · no retained correlation suite".to_owned(),
            correlation: ModelCorrelationState::default(),
            suite: None,
            evaluation: None,
            evaluation_error: None,
            current_evidence: Vec::new(),
            source_current: false,
            suite_count: 0,
            dataset_count: 0,
            metric_count: 0,
            passed_metrics: 0,
            coverage_percent: None,
            worst_normalized_error: None,
            open_dispositions: 0,
        };
        assert_eq!(
            projection.state_label(),
            ("correlation suite not configured", Tone::Warning)
        );
        assert!(projection.metrics().iter().all(|metric| {
            metric.value == "0" || metric.value == "0 / 0" || metric.value == "No evidence"
        }));
    }

    #[test]
    fn bounded_pages_clamp_without_growing_the_render_window() {
        assert_eq!(
            bounded_page(0, 24, usize::MAX),
            BoundedPage {
                index: 0,
                count: 1,
                start: 0,
                end: 0,
                total: 0,
            }
        );
        assert_eq!(
            bounded_page(49, 24, usize::MAX),
            BoundedPage {
                index: 2,
                count: 3,
                start: 48,
                end: 49,
                total: 49,
            }
        );
        let middle = bounded_page(49, 24, 1);
        assert_eq!(middle.end - middle.start, 24);
    }

    #[test]
    fn plot_sampling_is_deterministic_bounded_and_keeps_endpoints() {
        let first = deterministic_sample_indices(10_000, 192);
        let second = deterministic_sample_indices(10_000, 192);
        assert_eq!(first, second);
        assert_eq!(first.len(), 192);
        assert_eq!(first.first(), Some(&0));
        assert_eq!(first.last(), Some(&9_999));
        assert!(first.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(deterministic_sample_indices(3, 192), vec![0, 1, 2]);
    }

    #[test]
    fn long_source_lists_disclose_omitted_members() {
        let sources = (0..8)
            .map(|index| format!("run-{index}"))
            .collect::<Vec<_>>();
        assert_eq!(
            bounded_string_list(&sources, 4),
            "run-0, run-1, run-2, run-3, … +4 more"
        );
    }
}
