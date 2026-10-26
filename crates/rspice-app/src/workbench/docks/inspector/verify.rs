//! The Verify workspace's inspector: the Monte Carlo run and its worst
//! samples, a failed corner run, and the tuning session's open values.

use egui::{Pos2, Rect, Ui, Vec2};
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{Problem, panel_inset, section_header};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;

use super::super::super::design_system::property_row;
use super::super::super::state::VerificationPage;
use super::compact_readout;

/// The Verify inspector states only what its page does not: the Monte Carlo
/// run's definition and worst samples, a corner run that failed, and the
/// tuning session's uncommitted or invalid values. The other pages already
/// show everything there is to say about them.
pub(in crate::workbench) fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.simulate.verification_page {
        VerificationPage::Yield => yield_details(ui, app),
        VerificationPage::Corners => {
            let failed = app.state.model.simulation.active_run().and_then(|run| {
                run.analyses.iter().find(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::Corner
                        && !analysis.success
                })
            });
            if let Some(analysis) = failed {
                let message = analysis.error_message.as_deref().map_or_else(
                    || "The corner run failed.".to_owned(),
                    |error| format!("The corner run failed: {error}"),
                );
                crate::ui::widgets::problem_line(ui, Problem::Failure, &message);
            }
        }
        VerificationPage::Tuning => {
            let session = &app.state.simulate.verification;
            let dirty_count = session
                .tuning_variables
                .iter()
                .filter(|draft| draft.is_dirty())
                .count();
            let invalid_count = session
                .tuning_variables
                .iter()
                .filter(|draft| draft.validation_error.is_some())
                .count();
            let uncommitted = format!("{dirty_count} uncommitted");
            section_header(
                ui,
                "Tuning",
                (dirty_count > 0).then_some(uncommitted.as_str()),
            );
            if invalid_count > 0 {
                super::super::super::design_system::property_row_problem(
                    ui,
                    "Validation",
                    &format!("{invalid_count} invalid"),
                    Some(Problem::Failure),
                );
            }
        }
        VerificationPage::Optimization
        | VerificationPage::DeviceSafety
        | VerificationPage::Regression => {}
    }
}

fn yield_details(ui: &mut Ui, app: &RSpiceApp) {
    let run = app.state.model.simulation.active_run();
    let evidence = app
        .state
        .model
        .simulation
        .yield_results_for_active_dataset();
    let provenance = evidence.and(app.state.model.simulation.retained.yield_provenance());
    let results = evidence.unwrap_or_default();

    section_header(ui, "Monte Carlo", None);
    if let Some(provenance) = provenance {
        property_row(
            ui,
            "Samples",
            &format!(
                "{} / {}",
                provenance.runs_completed, provenance.runs_requested
            ),
        );
        property_row(ui, "Seed", &format!("0x{:X}", provenance.seed));
        property_row(ui, "Sampling", provenance.sampling_mode.display_name());
        let (passing, total) = joint_yield(results).unwrap_or((0, 0));
        if let Some((low, high)) = wilson_interval_95(passing, total) {
            property_row(
                ui,
                "Confidence",
                &format!("95% CI {:.2}–{:.2}%", low * 100.0, high * 100.0),
            );
        }
        if let Some(run) = run {
            property_row(ui, "Duration", &compact_readout(run.elapsed_time, "s"));
        }
    } else {
        muted_inspector_copy(ui, "No Monte Carlo run");
    }

    let failures = worst_samples(results);
    if !failures.is_empty() {
        let failed_count = joint_yield(results).map_or(0, |(passing, total)| total - passing);
        let failed_label = format!("{failed_count} fail");
        section_header(ui, "Worst samples", Some(&failed_label));
        for failure in failures.iter().take(3) {
            failure_row(ui, failure);
        }
    }

    if !results.is_empty() {
        section_header(ui, "Specification yields", None);
        for result in results {
            property_row(
                ui,
                &result.spec.target,
                &format!(
                    "{:.2}% · {} / {}",
                    result.yield_percent, result.pass_count, result.total_runs
                ),
            );
        }
    }
}

#[derive(Debug)]
struct FailedSample {
    target: String,
    unit: String,
    sample_index: usize,
    value: f64,
    normalized_margin: f64,
}

/// The samples outside their limits, worst first.
///
/// Judged by value, never by position in the trail: the trail has one entry
/// per trial and the samples one per finite observation, so after a trial
/// that did not converge the two no longer line up, and pairing them named
/// passing values as failures.
fn worst_samples(results: &[rspice_results::yield_analysis::YieldResult]) -> Vec<FailedSample> {
    let mut failures = results
        .iter()
        .flat_map(|result| {
            result
                .samples
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, value)| value.is_finite() && !result.spec.evaluates(*value))
                .filter_map(|(index, value)| {
                    normalized_yield_margin(&result.spec, value).map(|normalized_margin| {
                        FailedSample {
                            target: result.spec.target.clone(),
                            unit: result.spec.unit.clone(),
                            sample_index: index,
                            value,
                            normalized_margin,
                        }
                    })
                })
        })
        .collect::<Vec<_>>();
    failures.sort_by(|a, b| {
        a.normalized_margin
            .partial_cmp(&b.normalized_margin)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.target.cmp(&b.target))
            .then_with(|| a.sample_index.cmp(&b.sample_index))
    });
    failures
}

fn normalized_yield_margin(
    spec: &rspice_results::yield_analysis::YieldSpec,
    value: f64,
) -> Option<f64> {
    const SCALE_FLOOR: f64 = 1.0e-30;
    match (spec.min, spec.max) {
        (Some(min), Some(max)) if max > min => {
            Some((value - min).min(max - value) / (max - min).max(SCALE_FLOOR))
        }
        (Some(min), None) => Some((value - min) / min.abs().max(SCALE_FLOOR)),
        (None, Some(max)) => Some((max - value) / max.abs().max(SCALE_FLOOR)),
        (Some(_), Some(_)) | (None, None) => None,
    }
}

const FAILURE_ROW_PADDING_Y: f32 = 8.0;
const FAILURE_ROW_COLUMN_GAP: f32 = 8.0;
const FAILURE_ROW_DETAIL_GAP: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct FailureRowColumns {
    target_width: f32,
    margin_width: f32,
}

/// The target and margin columns of a failure row `row_width` wide whose text
/// is inset `padding_x` from both edges: the panel's own inset, so the row's
/// text lines up with the property rows around it.
fn failure_row_columns(
    row_width: f32,
    measured_margin_width: f32,
    padding_x: f32,
) -> FailureRowColumns {
    let content_width = (row_width - padding_x * 2.0).max(1.0);
    let margin_width = measured_margin_width.max(1.0).min(content_width);
    let target_width = (content_width - margin_width - FAILURE_ROW_COLUMN_GAP).max(1.0);
    FailureRowColumns {
        target_width,
        margin_width,
    }
}

fn failure_row(ui: &mut Ui, failure: &FailedSample) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let title = format!("#{:04} · {}", failure.sample_index + 1, failure.target);
    let margin = format!("{:+.2}%", failure.normalized_margin * 100.0);
    let detail = format!(
        "observed {}",
        crate::ui::plot::fmt_si_significant(failure.value, &failure.unit, 4)
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let margin_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measured_margin = ui
        .painter()
        .layout_no_wrap(margin, margin_font, t.color.err);
    let padding_x = panel_inset(ui);
    let columns = failure_row_columns(width, measured_margin.size().x, padding_x);
    let title_galley = ui
        .painter()
        .layout(title, title_font, t.color.text, columns.target_width);
    let content_width = (width - padding_x * 2.0).max(1.0);
    let detail_galley = ui
        .painter()
        .layout(detail, detail_font, t.color.text_faint, content_width);
    let title_height = title_galley.size().y.max(measured_margin.size().y);
    let natural_height = FAILURE_ROW_PADDING_Y * 2.0
        + title_height
        + FAILURE_ROW_DETAIL_GAP
        + detail_galley.size().y;
    let minimum_height = if t.metrics.ctl_h >= 44.0 { 58.0 } else { 47.0 };
    let height = natural_height.max(minimum_height);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "Sample {}. {}. Value {} {}. Margin {:+.2}%",
                failure.sample_index + 1,
                failure.target,
                failure.value,
                failure.unit,
                failure.normalized_margin * 100.0
            ),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let title_top = rect.top() + FAILURE_ROW_PADDING_Y;
    let target_rect = Rect::from_min_size(
        Pos2::new(rect.left() + padding_x, title_top),
        Vec2::new(columns.target_width, title_height),
    );
    ui.painter().with_clip_rect(target_rect).galley(
        target_rect.left_top(),
        title_galley,
        t.color.text,
    );
    let margin_rect = Rect::from_min_size(
        Pos2::new(rect.right() - padding_x - columns.margin_width, title_top),
        Vec2::new(columns.margin_width, title_height),
    );
    ui.painter().with_clip_rect(margin_rect).galley(
        Pos2::new(
            margin_rect.right() - measured_margin.size().x,
            margin_rect.top(),
        ),
        measured_margin,
        t.color.err,
    );
    let detail_top = title_top + title_height + FAILURE_ROW_DETAIL_GAP;
    let detail_rect = Rect::from_min_max(
        Pos2::new(rect.left() + padding_x, detail_top),
        Pos2::new(rect.right() - padding_x, rect.bottom()),
    );
    ui.painter().with_clip_rect(detail_rect).galley(
        detail_rect.left_top(),
        detail_galley,
        t.color.text_faint,
    );
    response
}

fn joint_yield(results: &[rspice_results::yield_analysis::YieldResult]) -> Option<(usize, usize)> {
    let total = results.first()?.total_runs;
    if total == 0
        || results
            .iter()
            .any(|result| result.total_runs != total || result.trail.len() != total)
    {
        return None;
    }
    Some((
        (0..total)
            .filter(|index| results.iter().all(|result| result.trail[*index]))
            .count(),
        total,
    ))
}

fn wilson_interval_95(successes: usize, total: usize) -> Option<(f64, f64)> {
    if total == 0 || successes > total {
        return None;
    }
    let n = total as f64;
    let p = successes as f64 / n;
    let z = 1.959_963_984_540_054_f64;
    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;
    let half = z * ((p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt()) / denominator;
    Some(((center - half).max(0.0), (center + half).min(1.0)))
}

#[cfg(test)]
mod tests;
