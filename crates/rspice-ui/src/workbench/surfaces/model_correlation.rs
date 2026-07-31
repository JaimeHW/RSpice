//! Project-owned measurement-correlation workspace.
//!
//! This specialist surface follows the exact seven-section contract in the
//! reviewed mockup. It never fabricates datasets, residuals, gate outcomes, or
//! reviewer identities: every projection is derived from persisted
//! model-library correlation records.

mod dialogs;
mod sections;

use dialogs::*;
use sections::*;

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
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use super::model_correlation_controller;
use crate::workbench::documents::model_correlation::{
    CorrelationAggregationDraft, CorrelationAlignmentDraft, CorrelationCalculationDraft,
    CorrelationDatasetClassDraft, CorrelationDatasetDraft, CorrelationExtrapolationDraft,
    CorrelationMetricDraft, CorrelationOutlierDecisionDraft, CorrelationOutlierDraft,
    CorrelationReleaseRoleDraft, CorrelationReviewDecisionDraft, CorrelationReviewDraft,
    ModelCorrelationDialog, ModelCorrelationSection,
};
use crate::workbench::documents::model_editor::{
    ModelEditorSection, open_project_model, resolve_project_model_for_editor,
};

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
