//! Verification evidence, specifications, checks, reliability, and history.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::state::SpecEntry;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot, workspace_title_row};
use super::super::state::VerificationPage;

const VERIFY_RESPONSIVE_BREAKPOINT: f32 = 820.0;
const VERIFY_PHONE_BREAKPOINT: f32 = 560.0;
const VERIFY_STACKED_CHART_HEIGHT: f32 = 250.0;
const VERIFY_PHONE_CHART_HEIGHT: f32 = 230.0;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        let surface_width = ui.available_width();
        ScrollArea::vertical()
            .id_salt("workbench.verify.surface")
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(surface_width);
                workspace_title_row(ui, |ui| {
                    verification_heading(ui, app);
                });
                match app.state.workbench.verification_page {
                    VerificationPage::Cockpit => cockpit(ui, app),
                    VerificationPage::Specifications => specifications(ui, app),
                    VerificationPage::Checks => checks(ui, app),
                    VerificationPage::Reliability => reliability(ui, app),
                    VerificationPage::History => history(ui, app),
                }
            });
    });
}

fn verification_heading(ui: &mut Ui, app: &RSpiceApp) {
    if app.state.workbench.verification_page == VerificationPage::Cockpit {
        let run = verification_run(app);
        let eyebrow = run.map_or_else(
            || "PVT & MONTE CARLO · NO RETAINED DATASET".to_owned(),
            |run| {
                format!(
                    "PVT & MONTE CARLO · RUN {} · DATASET {}",
                    run.id, run.dataset_id
                )
            },
        );
        heading(
            ui,
            &eyebrow,
            "PVT & Monte Carlo verification",
            "Yield, retained-run specification margins, and traceable measurement evidence from the active project.",
        );
    } else {
        heading(
            ui,
            "Verification-evidence owner",
            app.state.workbench.verification_page.label(),
            "Traceable checks, specification verdicts, reliability evidence, and run history.",
        );
    }
}

fn cockpit(ui: &mut Ui, app: &mut RSpiceApp) {
    let run_index = verification_run_index(app);
    let evidence = specification_evidence(app, run_index);
    engineering_status_strip(ui, app, &evidence);

    let width = ui.available_width();
    let viewport_width = ui.ctx().content_rect().width();
    let t = Tokens::get(ui.ctx());
    let header_height = if t.metrics.ctl_h >= 44.0 { 44.0 } else { 37.0 };
    let visible_remaining = (ui.clip_rect().bottom() - ui.cursor().top()).max(0.0);
    let layout = VerifyLayout::resolve(
        viewport_width,
        width,
        visible_remaining,
        evidence.len(),
        header_height,
        verification_table_row_height(viewport_width),
    );
    if layout.split {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui(
                egui::vec2(layout.left_width, layout.first_row_height),
                |ui| yield_chart(ui, app, layout.first_row_height),
            );
            ui.allocate_ui(
                egui::vec2(layout.right_width, layout.first_row_height),
                |ui| run_margin_matrix(ui, app, run_index, &evidence),
            );
        });
        let divider_x = ui.min_rect().left() + layout.left_width;
        let row_bottom = ui.cursor().top();
        ui.painter().vline(
            divider_x,
            egui::Rangef::new(row_bottom - layout.first_row_height, row_bottom),
            egui::Stroke::new(1.0, t.color.border),
        );
    } else {
        let chart_height = verification_stacked_chart_height(viewport_width);
        ui.allocate_ui(egui::vec2(width, chart_height), |ui| {
            yield_chart(ui, app, chart_height)
        });
        let divider_y = ui.cursor().top();
        ui.painter().hline(
            egui::Rangef::new(ui.min_rect().left(), ui.max_rect().right()),
            divider_y,
            egui::Stroke::new(1.0, t.color.border),
        );
        ui.add_space(1.0);
        run_margin_matrix(ui, app, run_index, &evidence);
    }
    let divider_y = ui.cursor().top();
    ui.painter().hline(
        egui::Rangef::new(ui.min_rect().left(), ui.max_rect().right()),
        divider_y,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(1.0);
    specification_matrix(ui, app, &evidence);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VerifyLayout {
    split: bool,
    left_width: f32,
    right_width: f32,
    first_row_height: f32,
}

impl VerifyLayout {
    fn resolve(
        viewport_width: f32,
        content_width: f32,
        visible_height: f32,
        specification_rows: usize,
        section_header_height: f32,
        row_height: f32,
    ) -> Self {
        let content_width = content_width.max(0.0);
        let specification_height =
            section_header_height + 27.0 + row_height * specification_rows.max(1) as f32;
        let first_row_height = (visible_height - specification_height - 1.0).max(210.0);
        let split = viewport_width > 1_020.0 && content_width >= 641.0;
        let left_width = if split {
            let usable = content_width - 1.0;
            (usable * 0.45).clamp(280.0, usable - 360.0)
        } else {
            content_width
        };
        let right_width = if split {
            content_width - left_width - 1.0
        } else {
            content_width
        };
        Self {
            split,
            left_width,
            right_width,
            first_row_height,
        }
    }
}

#[derive(Clone)]
struct SpecificationEvidence {
    spec: SpecEntry,
    values: Vec<(String, f64)>,
}

#[derive(Clone)]
struct TableCell {
    text: String,
    mono: bool,
    color: Option<egui::Color32>,
}

impl TableCell {
    fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: false,
            color: None,
        }
    }

    fn mono(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: true,
            color: None,
        }
    }

    fn tone(text: impl Into<String>, color: egui::Color32) -> Self {
        Self {
            text: text.into(),
            mono: true,
            color: Some(color),
        }
    }
}

fn verification_run_index(app: &RSpiceApp) -> Option<usize> {
    app.state
        .simulation
        .active_run_idx
        .filter(|index| *index < app.state.simulation.runs.len())
        .or_else(|| (!app.state.simulation.runs.is_empty()).then_some(0))
}

fn verification_run(app: &RSpiceApp) -> Option<&crate::state::SimulationRun> {
    verification_run_index(app).and_then(|index| app.state.simulation.runs.get(index))
}

fn verification_yield_results(app: &RSpiceApp) -> &[crate::services::yield_manager::YieldResult] {
    verification_run(app)
        .and_then(|run| {
            app.state
                .simulation
                .yield_results_for_dataset(run.dataset_id)
        })
        .unwrap_or(&[])
}

fn specification_evidence(app: &RSpiceApp, run_index: Option<usize>) -> Vec<SpecificationEvidence> {
    let run = run_index.and_then(|index| app.state.simulation.runs.get(index));
    app.state
        .workspace
        .specs
        .iter()
        .cloned()
        .map(|spec| {
            let values = run
                .and_then(|run| {
                    measurement_in_run(run, &spec.measurement)
                        .map(|value| (format!("Run {}", run.id), value))
                })
                .into_iter()
                .collect();
            SpecificationEvidence { spec, values }
        })
        .collect()
}

fn engineering_status_strip(ui: &mut Ui, app: &RSpiceApp, evidence: &[SpecificationEvidence]) {
    let t = Tokens::get(ui.ctx());
    let yield_results = verification_yield_results(app);
    let joint_yield = joint_sample_summary(yield_results);
    let covered = evidence.iter().filter(|row| !row.values.is_empty()).count();
    let worst = evidence
        .iter()
        .filter_map(|row| {
            row.values
                .iter()
                .filter_map(|(run, value)| {
                    normalized_margin(&row.spec, *value).map(|margin| (margin, row, run, *value))
                })
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        })
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let items = [
        (
            "Estimated yield".to_owned(),
            joint_yield.map_or_else(
                || "No evidence".to_owned(),
                |summary| format!("{:.2}%", summary.yield_percent()),
            ),
            joint_yield.map_or_else(
                || "No aligned all-spec sample trail".to_owned(),
                |summary| {
                    format!(
                        "Joint AND · {} specs · {} samples",
                        summary.specification_count, summary.total
                    )
                },
            ),
            joint_yield.map_or(t.color.warn, |summary| {
                if summary.passing == summary.total {
                    t.color.ok
                } else {
                    t.color.warn
                }
            }),
        ),
        (
            "Passing samples".to_owned(),
            joint_yield.map_or_else(
                || "0 / 0".to_owned(),
                |summary| format!("{} / {}", summary.passing, summary.total),
            ),
            joint_yield.map_or_else(
                || "No retained sample trail".to_owned(),
                |summary| {
                    format!(
                        "{} violate at least one executable specification",
                        summary.total - summary.passing
                    )
                },
            ),
            joint_yield.map_or(t.color.text, |summary| {
                if summary.passing == summary.total {
                    t.color.ok
                } else {
                    t.color.text
                }
            }),
        ),
        (
            "Worst margin".to_owned(),
            worst.map_or_else(
                || "No evidence".to_owned(),
                |(margin, _, _, _)| format!("{:+.2}%", margin * 100.0),
            ),
            worst.map_or_else(
                || "No bounded measurement available".to_owned(),
                |(_, row, run, _)| format!("{} · {run}", row.spec.measurement),
            ),
            worst.map_or(t.color.warn, |(margin, _, _, _)| {
                if margin < 0.0 {
                    t.color.err
                } else {
                    t.color.ok
                }
            }),
        ),
        (
            "Evidence coverage".to_owned(),
            format!("{} / {}", covered, evidence.len()),
            if evidence.is_empty() {
                "No project specifications".to_owned()
            } else {
                format!(
                    "{} specifications lack active-run evidence",
                    evidence.len() - covered
                )
            },
            if covered == evidence.len() && !evidence.is_empty() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    let columns = verification_status_columns(ui.ctx().content_rect().width());
    let width = ui.available_width();
    let cell_width = width / columns as f32;
    for row in items.chunks(columns) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            for (index, (label, value, detail, tone)) in row.iter().enumerate() {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(cell_width, 68.0), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        format!("{label}: {value}. {detail}"),
                    )
                });
                if ui.is_rect_visible(rect) {
                    let content_rect = rect.shrink2(egui::vec2(12.0, 0.0));
                    let painter = ui.painter().with_clip_rect(content_rect);
                    painter.text(
                        rect.left_top() + egui::vec2(12.0, 9.0),
                        egui::Align2::LEFT_TOP,
                        label,
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_dim,
                    );
                    painter.text(
                        rect.left_top() + egui::vec2(12.0, 28.0),
                        egui::Align2::LEFT_TOP,
                        value,
                        theme::mono(17.0, FontWeight::Medium),
                        *tone,
                    );
                    painter.text(
                        rect.left_top() + egui::vec2(12.0, 49.0),
                        egui::Align2::LEFT_TOP,
                        detail,
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_faint,
                    );
                    if index + 1 < row.len() {
                        ui.painter().vline(
                            rect.right(),
                            rect.y_range(),
                            egui::Stroke::new(1.0, t.color.border),
                        );
                    }
                    if columns == 2 && row.as_ptr() == items.as_ptr() {
                        ui.painter().hline(
                            rect.x_range(),
                            rect.bottom(),
                            egui::Stroke::new(1.0, t.color.border),
                        );
                    }
                }
            }
        });
    }
    let bottom = ui.cursor().top();
    ui.painter().hline(
        egui::Rangef::new(ui.min_rect().left(), ui.max_rect().right()),
        bottom,
        egui::Stroke::new(1.0, t.color.border_strong),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JointSampleSummary {
    passing: usize,
    total: usize,
    specification_count: usize,
}

impl JointSampleSummary {
    fn yield_percent(self) -> f64 {
        self.passing as f64 / self.total as f64 * 100.0
    }
}

fn joint_sample_summary(
    results: &[crate::services::yield_manager::YieldResult],
) -> Option<JointSampleSummary> {
    let first = results.first()?;
    let total = first.total_runs;
    if total == 0
        || results
            .iter()
            .any(|result| result.total_runs != total || result.trail.len() != total)
    {
        return None;
    }
    let passing = (0..total)
        .filter(|index| results.iter().all(|result| result.trail[*index]))
        .count();
    Some(JointSampleSummary {
        passing,
        total,
        specification_count: results.len(),
    })
}

fn worst_individual_yield_result(
    results: &[crate::services::yield_manager::YieldResult],
) -> Option<&crate::services::yield_manager::YieldResult> {
    results.iter().min_by(|a, b| {
        a.yield_percent
            .partial_cmp(&b.yield_percent)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn yield_chart(ui: &mut Ui, app: &RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, height.max(210.0)), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    let head = egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + 36.0));
    ui.painter().rect_filled(head, 0.0, t.color.bg_panel);
    ui.painter().hline(
        head.x_range(),
        head.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let Some(result) = worst_individual_yield_result(verification_yield_results(app)) else {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                "Monte Carlo distribution: no evidence for the active dataset",
            )
        });
        ui.painter().text(
            egui::pos2(rect.center().x, rect.center().y + 12.0),
            egui::Align2::CENTER_CENTER,
            "No Monte Carlo distribution evidence for the active dataset",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    };
    // A joint all-spec verdict is boolean and has no honest physical x-axis.
    // Keep the histogram on one real measurement distribution and label that
    // scope explicitly; the status strip above owns the joint AND yield.
    let title = format!(
        "Worst individual spec · {} distribution",
        result.spec.target
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let title = elide_table_text(ui, &title, &title_font, (head.width() * 0.52).max(80.0));
    ui.painter().text(
        head.left_center() + egui::vec2(10.0, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text,
    );
    ui.painter().text(
        head.right_center() - egui::vec2(10.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        format!(
            "mean {} · σ {}",
            format_value(result.stats.mean, &result.spec.unit),
            format_value(result.stats.std_dev, &result.spec.unit)
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let samples: Vec<f64> = result
        .samples
        .iter()
        .copied()
        .filter(|sample| sample.is_finite())
        .collect();
    if samples.is_empty() {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                format!(
                    "Worst individual specification {} distribution: summary retained, exact sample payload unavailable",
                    result.spec.target
                ),
            )
        });
        ui.painter().text(
            egui::pos2(rect.center().x, rect.center().y + 12.0),
            egui::Align2::CENTER_CENTER,
            "Summary retained; exact sample payload unavailable",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    }
    let min = samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bins = histogram_bins(&samples, 22, min, max);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Monte Carlo worst individual specification {} distribution. {} exact samples. Mean {}. Standard deviation {}. Range {} to {}. Limit {}. Histogram bin counts: {}",
                result.spec.target,
                samples.len(),
                format_value(result.stats.mean, &result.spec.unit),
                format_value(result.stats.std_dev, &result.spec.unit),
                format_value(min, &result.spec.unit),
                format_value(max, &result.spec.unit),
                yield_limit_text(&result.spec),
                bins.iter().map(usize::to_string).collect::<Vec<_>>().join(", ")
            ),
        )
    });
    let peak = bins.iter().copied().max().unwrap_or(1).max(1) as f32;
    let plot = egui::Rect::from_min_max(
        head.left_bottom() + egui::vec2(10.0, 12.0),
        rect.right_bottom() - egui::vec2(10.0, 24.0),
    );
    for step in 0..=4 {
        let y = egui::lerp(plot.bottom()..=plot.top(), step as f32 / 4.0);
        ui.painter().hline(
            plot.x_range(),
            y,
            egui::Stroke::new(1.0, t.color.canvas_grid.gamma_multiply(0.55)),
        );
    }
    let bar_width = plot.width() / bins.len() as f32;
    let span = (max - min).max(f64::EPSILON);
    for (index, count) in bins.iter().copied().enumerate() {
        let height = plot.height() * count as f32 / peak;
        let center = min + span * (index as f64 + 0.5) / bins.len() as f64;
        let failing = !result.spec.evaluates(center);
        let bar = egui::Rect::from_min_max(
            egui::pos2(
                plot.left() + index as f32 * bar_width + 1.0,
                plot.bottom() - height,
            ),
            egui::pos2(
                plot.left() + (index + 1) as f32 * bar_width - 1.0,
                plot.bottom(),
            ),
        );
        ui.painter().rect_filled(
            bar,
            1.0,
            if failing {
                t.color.err.gamma_multiply(0.82)
            } else {
                t.color.accent.gamma_multiply(0.72)
            },
        );
    }
    for limit in [result.spec.min, result.spec.max].into_iter().flatten() {
        if limit >= min && limit <= max && max > min {
            let x = egui::remap(limit, min..=max, plot.left() as f64..=plot.right() as f64) as f32;
            ui.painter()
                .vline(x, plot.y_range(), egui::Stroke::new(1.0, t.color.err));
        }
    }
    for step in 0..=5 {
        let fraction = step as f64 / 5.0;
        let value = min + (max - min) * fraction;
        let x = egui::lerp(plot.left()..=plot.right(), fraction as f32);
        let align = if step == 0 {
            egui::Align2::LEFT_TOP
        } else if step == 5 {
            egui::Align2::RIGHT_TOP
        } else {
            egui::Align2::CENTER_TOP
        };
        ui.painter().text(
            egui::pos2(x, plot.bottom() + 4.0),
            align,
            format_scalar(value),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

fn yield_limit_text(spec: &crate::services::yield_manager::YieldSpec) -> String {
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => format!(
            "{}…{} {}",
            format_scalar(min),
            format_scalar(max),
            spec.unit
        ),
        (Some(min), None) => format!("≥ {} {}", format_scalar(min), spec.unit),
        (None, Some(max)) => format!("≤ {} {}", format_scalar(max), spec.unit),
        (None, None) => "unbounded".to_owned(),
    }
}

fn histogram_bins(samples: &[f64], count: usize, min: f64, max: f64) -> Vec<usize> {
    let mut bins = vec![0; count.max(1)];
    let span = max - min;
    for sample in samples {
        let index = if span <= f64::EPSILON {
            bins.len() / 2
        } else {
            (((sample - min) / span) * bins.len() as f64)
                .floor()
                .clamp(0.0, (bins.len() - 1) as f64) as usize
        };
        bins[index] += 1;
    }
    bins
}

fn run_margin_matrix(
    ui: &mut Ui,
    app: &RSpiceApp,
    run_index: Option<usize>,
    evidence: &[SpecificationEvidence],
) {
    let run = run_index.and_then(|index| app.state.simulation.runs.get(index));
    table_section_header(
        ui,
        "Active dataset · specification margin",
        run.map(|run| format!("Run {}", run.id)).as_deref(),
        None,
    );
    let headers = vec![
        ("Specification".to_owned(), 0.55),
        ("Margin".to_owned(), 0.45),
    ];
    let rows = evidence
        .iter()
        .map(|row| {
            let margin = run
                .and_then(|run| measurement_in_run(run, &row.spec.measurement))
                .and_then(|value| normalized_margin(&row.spec, value));
            vec![
                TableCell::text(format!(
                    "{} · {}",
                    row.spec.measurement,
                    limit_text(&row.spec)
                )),
                margin.map_or_else(
                    || TableCell::tone("NO EVIDENCE", Tokens::get(ui.ctx()).color.warn),
                    |margin| {
                        TableCell::tone(
                            format!("{:+.2}%", margin * 100.0),
                            if margin < 0.0 {
                                Tokens::get(ui.ctx()).color.err
                            } else {
                                Tokens::get(ui.ctx()).color.ok
                            },
                        )
                    },
                ),
            ]
        })
        .collect::<Vec<_>>();
    render_data_table(ui, "verification-run-margin", &headers, &rows, 360.0);
}

fn specification_matrix(ui: &mut Ui, app: &mut RSpiceApp, evidence: &[SpecificationEvidence]) {
    let action = table_section_header(
        ui,
        "Measurement & specification matrix",
        Some(&format!(
            "{} coverage gaps",
            evidence.iter().filter(|row| row.values.is_empty()).count()
        )),
        Some("Edit specifications"),
    );
    if action {
        Command::EditSpecifications.execute(app);
    }
    let t = Tokens::get(ui.ctx());
    let headers = vec![
        ("Measurement".to_owned(), 0.25),
        ("Limit".to_owned(), 0.15),
        ("Active value".to_owned(), 0.16),
        ("Margin".to_owned(), 0.16),
        ("Run".to_owned(), 0.14),
        ("Status".to_owned(), 0.14),
    ];
    let rows = evidence
        .iter()
        .map(|row| {
            let active = row.values.first();
            let margin = active.and_then(|(_, value)| normalized_margin(&row.spec, *value));
            let failed = margin.is_some_and(|value| value < 0.0);
            vec![
                TableCell::text(&row.spec.measurement),
                TableCell::mono(limit_text(&row.spec)),
                TableCell::mono(active.map_or_else(
                    || "—".to_owned(),
                    |(_, value)| format_value(*value, &row.spec.unit),
                )),
                TableCell::mono(margin.map_or_else(
                    || "—".to_owned(),
                    |margin| format!("{:+.2}%", margin * 100.0),
                )),
                TableCell::text(active.map_or("—", |(run, _)| run)),
                TableCell::tone(
                    if row.values.is_empty() {
                        "NO EVIDENCE"
                    } else if failed {
                        "FAIL"
                    } else {
                        "PASS"
                    },
                    if row.values.is_empty() {
                        t.color.warn
                    } else if failed {
                        t.color.err
                    } else {
                        t.color.ok
                    },
                ),
            ]
        })
        .collect::<Vec<_>>();
    render_data_table(
        ui,
        "verification-specification-matrix",
        &headers,
        &rows,
        if t.metrics.ctl_h >= 44.0 { 540.0 } else { 0.0 },
    );
}

fn table_section_header(
    ui: &mut Ui,
    title: &str,
    status: Option<&str>,
    action: Option<&str>,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let height = if t.metrics.ctl_h >= 44.0 { 44.0 } else { 37.0 };
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(egui::vec2(11.0, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 8.0;
    child.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    let mut clicked = false;
    child.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if let Some(action) = action {
            clicked = Button::new(action).ghost().show(ui).clicked();
        }
        if let Some(status) = status {
            ui.label(
                egui::RichText::new(status)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        }
    });
    clicked
}

fn render_data_table(
    ui: &mut Ui,
    id: &str,
    headers: &[(String, f32)],
    rows: &[Vec<TableCell>],
    min_width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let row_height = verification_table_row_height(ui.ctx().content_rect().width());
    egui::ScrollArea::horizontal()
        .id_salt(id)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
        .show(ui, |ui| {
            let width = ui.available_width().max(min_width);
            ui.set_min_width(width);
            let (head, head_response) =
                ui.allocate_exact_size(egui::vec2(width, 27.0), egui::Sense::hover());
            head_response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Label,
                    ui.is_enabled(),
                    format!(
                        "Table columns: {}",
                        headers
                            .iter()
                            .map(|(label, _)| label.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                )
            });
            ui.painter().rect_filled(head, 0.0, t.color.bg_panel_2);
            paint_table_cells(ui, head, headers, true, |label| {
                TableCell::text(label.to_uppercase())
            });
            ui.painter().hline(
                head.x_range(),
                head.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            if rows.is_empty() {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        "No specification evidence",
                    )
                });
                ui.painter().text(
                    rect.left_center() + egui::vec2(8.0, 0.0),
                    egui::Align2::LEFT_CENTER,
                    "No specification evidence",
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                );
                return;
            }
            for cells in rows {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        headers
                            .iter()
                            .zip(cells)
                            .map(|((header, _), cell)| format!("{header}: {}", cell.text))
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                });
                if response.hovered() {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                }
                paint_table_cells(ui, rect, headers, false, |label| {
                    let index = headers
                        .iter()
                        .position(|(header, _)| header == label)
                        .unwrap_or(0);
                    cells
                        .get(index)
                        .cloned()
                        .unwrap_or_else(|| TableCell::text("—"))
                });
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom(),
                    egui::Stroke::new(1.0, t.color.border),
                );
            }
        });
}

fn verification_status_columns(viewport_width: f32) -> usize {
    if viewport_width <= VERIFY_RESPONSIVE_BREAKPOINT {
        2
    } else {
        4
    }
}

fn verification_stacked_chart_height(viewport_width: f32) -> f32 {
    if viewport_width <= VERIFY_PHONE_BREAKPOINT {
        VERIFY_PHONE_CHART_HEIGHT
    } else {
        VERIFY_STACKED_CHART_HEIGHT
    }
}

fn verification_table_row_height(viewport_width: f32) -> f32 {
    if viewport_width <= VERIFY_PHONE_BREAKPOINT {
        36.0
    } else if viewport_width <= VERIFY_RESPONSIVE_BREAKPOINT {
        31.0
    } else {
        28.0
    }
}

fn paint_table_cells(
    ui: &mut Ui,
    rect: egui::Rect,
    columns: &[(String, f32)],
    header: bool,
    mut cell: impl FnMut(&str) -> TableCell,
) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left();
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        let cell_rect = egui::Rect::from_min_max(
            egui::pos2(x, rect.top()),
            egui::pos2((x + width).min(rect.right()), rect.bottom()),
        );
        let value = cell(label);
        let font = if value.mono {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(
                tokens::FS_0,
                if header {
                    FontWeight::Medium
                } else {
                    FontWeight::Regular
                },
            )
        };
        let text = elide_table_text(ui, &value.text, &font, (cell_rect.width() - 16.0).max(0.0));
        let painter = ui
            .painter()
            .with_clip_rect(cell_rect.shrink2(egui::vec2(8.0, 0.0)));
        painter.text(
            cell_rect.left_center() + egui::vec2(8.0, 0.0),
            egui::Align2::LEFT_CENTER,
            text,
            font,
            value.color.unwrap_or(if header {
                t.color.text_faint
            } else {
                t.color.text_dim
            }),
        );
        x += width;
    }
}

fn elide_table_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let ellipsis = '…';
    let characters = text.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once(ellipsis))
            .collect::<String>();
        let width = ui
            .painter()
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once(ellipsis))
        .collect()
}

fn measurement_in_run(run: &crate::state::SimulationRun, name: &str) -> Option<f64> {
    run.analyses.iter().find_map(|analysis| {
        analysis
            .measurements
            .iter()
            .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
            .and_then(|measurement| measurement.value)
    })
}

#[cfg(test)]
fn signed_margin(spec: &SpecEntry, value: f64) -> f64 {
    if let Some(min) = spec.min
        && value < min
    {
        return value - min;
    }
    if let Some(max) = spec.max
        && value > max
    {
        return max - value;
    }
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => (value - min).min(max - value),
        (Some(min), None) => value - min,
        (None, Some(max)) => max - value,
        (None, None) => 0.0,
    }
}

/// Dimensionless specification margin used for cross-specification ranking.
/// One-sided limits are normalized by the limit magnitude; ranges are
/// normalized by their span. Unbounded tracked values have no comparable
/// margin and are deliberately excluded.
fn normalized_margin(spec: &SpecEntry, value: f64) -> Option<f64> {
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

fn limit_text(spec: &SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => format!(
            "{}…{} {}",
            format_scalar(min),
            format_scalar(max),
            spec.unit
        ),
        (Some(min), None) => format!("≥ {} {}", format_scalar(min), spec.unit),
        (None, Some(max)) => format!("≤ {} {}", format_scalar(max), spec.unit),
        (None, None) => "tracked value".to_owned(),
    }
}

fn format_value(value: f64, unit: &str) -> String {
    if unit.is_empty() {
        format_scalar(value)
    } else {
        format!("{} {unit}", format_scalar(value))
    }
}

fn format_scalar(value: f64) -> String {
    let magnitude = value.abs();
    if magnitude != 0.0 && !(1.0e-3..1.0e6).contains(&magnitude) {
        format!("{value:.4e}")
    } else {
        format!("{value:.6}")
    }
}

fn specifications(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Specification matrix", |ui| {
        ui.label("Bounds are project design intent. Verdicts are evaluated only against .MEAS evidence in the active immutable dataset.");
        ui.add_space(8.0);
        let verdicts: Vec<_> = app
            .state
            .workspace
            .specs
            .iter()
            .map(|spec| active_dataset_measurement(&app.state.simulation, &spec.measurement))
            .collect();
        let mut remove = None;
        for (index, spec) in app.state.workspace.specs.iter_mut().enumerate() {
            let t = Tokens::get(ui.ctx());
            let selected = app.state.workbench.selected_spec == Some(index);
            let response = egui::Frame::new()
                .fill(if selected {
                    t.color.accent_dim
                } else {
                    t.color.bg_inset
                })
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Spec {}", index + 1));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if Button::new("Remove").ghost().show(ui).clicked() {
                                remove = Some(index);
                            }
                            verdict_badge(ui, spec, verdicts[index]);
                        });
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Measurement");
                        ui.add(
                            egui::TextEdit::singleline(&mut spec.measurement).desired_width(150.0),
                        );
                        optional_bound(ui, "Min", &mut spec.min);
                        optional_bound(ui, "Max", &mut spec.max);
                        ui.label("Unit");
                        ui.add(egui::TextEdit::singleline(&mut spec.unit).desired_width(70.0));
                    });
                })
                .response
                .interact(egui::Sense::click());
            response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    format!("Select specification {}: {}", index + 1, spec.measurement),
                )
            });
            theme::paint_focus_ring(ui, &response, response.rect);
            if response.clicked() {
                app.state.workbench.selected_spec = Some(index);
            }
            ui.add_space(6.0);
        }
        if let Some(index) = remove {
            app.state.workspace.specs.remove(index);
            app.state.workbench.selected_spec = None;
        }
        if Button::new("+ Add specification").show(ui).clicked() {
            app.state.workspace.specs.push(SpecEntry {
                measurement: "measurement_name".to_owned(),
                min: None,
                max: None,
                unit: String::new(),
            });
            app.state.workbench.selected_spec = Some(app.state.workspace.specs.len() - 1);
        }
    });
}

fn checks(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Schematic checks", |ui| {
        ui.horizontal(|ui| {
            if Button::new("Run checks now").accent().show(ui).clicked() {
                Command::RunChecks.execute(app);
            }
            if Button::new("Clear evidence")
                .enabled(Command::ClearChecks.is_enabled(app))
                .show(ui)
                .clicked()
            {
                Command::ClearChecks.execute(app);
            }
        });
        ui.add_space(8.0);
        let current =
            app.state.dialogs.drc_checked_version == app.state.schematic.topology_version();
        property_row(
            ui,
            "Evidence state",
            if current {
                "Current"
            } else {
                "Stale / not run"
            },
        );
        if let Some(result) = &app.state.dialogs.drc_results {
            property_row(ui, "Runtime", &format!("{} ms", result.duration_ms));
            for violation in result.violations() {
                ui.separator();
                ui.horizontal(|ui| {
                    let color = match violation.severity {
                        crate::services::drc::DrcSeverity::Critical
                        | crate::services::drc::DrcSeverity::Error => {
                            Tokens::get(ui.ctx()).color.err
                        }
                        crate::services::drc::DrcSeverity::Warning => {
                            Tokens::get(ui.ctx()).color.warn
                        }
                        crate::services::drc::DrcSeverity::Info => {
                            Tokens::get(ui.ctx()).color.text_dim
                        }
                    };
                    ui.label(
                        egui::RichText::new(violation.severity.display_name())
                            .color(color)
                            .strong(),
                    );
                    ui.vertical(|ui| {
                        ui.label(&violation.message);
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · {}",
                                violation.location.display(),
                                violation.violation_type.suggested_fix()
                            ))
                            .small()
                            .color(Tokens::get(ui.ctx()).color.text_dim),
                        );
                    });
                });
            }
        } else {
            ui.label("Run checks to create current verification evidence.");
        }
    });
}

fn reliability(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Safe operating area", |ui| {
        if app.state.simulation.soa_violations.is_empty() {
            status_dot(ui, Tokens::get(ui.ctx()).color.ok, "No recorded violations");
        }
        for violation in &app.state.simulation.soa_violations {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(format!("{:?}", violation.severity))
                        .color(Tokens::get(ui.ctx()).color.err),
                );
                ui.label(&violation.device_id);
                ui.label(format!(
                    "{:?}: {:.6} (limit {:.6})",
                    violation.parameter, violation.actual_value, violation.limit_value
                ));
                ui.label(format!("t = {:.6} s", violation.time));
            });
        }
    });
    ui.add_space(1.0);
    card(ui, "Aging and reliability", |ui| {
        if app.state.simulation.reliability_results.is_empty() {
            ui.label("No reliability analysis evidence is available in the active result set.");
        }
        for result in &app.state.simulation.reliability_results {
            ui.collapsing(&result.device_id, |ui| {
                property_row(ui, "Stress", &format!("{:?}", result.stress));
                for (lifetime, shift) in &result.shifts {
                    property_row(
                        ui,
                        lifetime,
                        &format!(
                            "ΔVth {:.3e} · Δμ {:.3e} · ΔRds {:.3e}",
                            shift.vth_shift, shift.mobility_shift, shift.rds_shift
                        ),
                    );
                }
            });
        }
        if Button::new("Configure reliability analysis…")
            .show(ui)
            .clicked()
        {
            app.state
                .workbench
                .activate(super::super::state::Workspace::Simulate);
            let kind = crate::simulation::plan::AnalysisKind::Reliability;
            let existing = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.kind() == kind)
                        .map(|instance| instance.id())
                });
            let selected = if let Some(id) = existing {
                Some(id)
            } else {
                match app
                    .state
                    .sim_setup
                    .stable_analysis_plan_mut()
                    .and_then(|plan| plan.insert(kind).map_err(|error| error.to_string()))
                {
                    Ok((id, receipt)) => {
                        app.state.workbench.analysis_lifecycle_status = receipt.detail().to_owned();
                        app.state.sim_setup.refresh_legacy_analysis_projections();
                        Some(id)
                    }
                    Err(error) => {
                        app.state.workbench.analysis_lifecycle_status = error;
                        None
                    }
                }
            };
            app.state.workbench.active_analysis = kind.legacy_index();
            app.state.workbench.active_analysis_instance = selected;
        }
    });
}

fn history(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Verification run history", |ui| {
        if app.state.simulation.runs.is_empty() {
            ui.label("No simulation runs have produced verification evidence.");
        }
        let mut selected_run = None;
        for (index, run) in app.state.simulation.runs.iter().enumerate() {
            ui.horizontal(|ui| {
                status_dot(
                    ui,
                    if run.success {
                        Tokens::get(ui.ctx()).color.ok
                    } else {
                        Tokens::get(ui.ctx()).color.err
                    },
                    &run.label,
                );
                ui.label(format!(
                    "{} analyses · {:.3} s",
                    run.analyses.len(),
                    run.elapsed_time
                ));
                if Button::new("Open dataset").show(ui).clicked() {
                    selected_run = Some(index);
                }
            });
        }
        if selected_run.is_some_and(|index| app.state.simulation.select_run(index)) {
            app.state
                .workbench
                .activate(super::super::state::Workspace::Results);
        }
    });
}

fn optional_bound(ui: &mut Ui, label: &str, value: &mut Option<f64>) {
    let mut enabled = value.is_some();
    ui.checkbox(&mut enabled, label);
    if enabled {
        let mut number = value.unwrap_or(0.0);
        if ui
            .add(
                egui::DragValue::new(&mut number)
                    .speed(0.01)
                    .max_decimals(12),
            )
            .changed()
            || value.is_none()
        {
            *value = Some(number);
        }
    } else {
        *value = None;
    }
}

fn active_dataset_measurement(
    simulation: &crate::state::SimulationState,
    name: &str,
) -> Option<f64> {
    simulation
        .active_run()
        .and_then(|run| measurement_in_run(run, name))
}

fn verdict_badge(ui: &mut Ui, spec: &SpecEntry, value: Option<f64>) {
    let t = Tokens::get(ui.ctx());
    let (text, color) = match value {
        Some(value) if spec.passes(value) => {
            (format!("PASS · {value:.6} {}", spec.unit), t.color.ok)
        }
        Some(value) => (format!("FAIL · {value:.6} {}", spec.unit), t.color.err),
        None => ("NO EVIDENCE".to_owned(), t.color.warn),
    };
    ui.label(egui::RichText::new(text).color(color).strong());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationState};

    #[test]
    fn verify_layout_matches_desktop_tablet_and_phone_compositions() {
        let desktop = VerifyLayout::resolve(1_440.0, 820.0, 620.0, 5, 37.0, 28.0);
        assert!(desktop.split);
        assert!((desktop.left_width + 1.0 + desktop.right_width - 820.0).abs() < f32::EPSILON);
        assert!(desktop.first_row_height > 400.0);

        let compact_desktop = VerifyLayout::resolve(1_024.0, 744.0, 600.0, 5, 37.0, 28.0);
        assert!(compact_desktop.split);

        let tablet = VerifyLayout::resolve(820.0, 768.0, 600.0, 5, 44.0, 31.0);
        assert!(!tablet.split);
        assert_eq!(tablet.left_width, 768.0);

        let phone = VerifyLayout::resolve(390.0, 390.0, 540.0, 5, 44.0, 36.0);
        assert!(!phone.split);
        assert_eq!(phone.right_width, 390.0);
    }

    #[test]
    fn responsive_verify_geometry_matches_mockup_breakpoints() {
        assert_eq!(verification_status_columns(821.0), 4);
        assert_eq!(verification_status_columns(820.0), 2);
        assert_eq!(verification_status_columns(390.0), 2);

        assert_eq!(verification_stacked_chart_height(561.0), 250.0);
        assert_eq!(verification_stacked_chart_height(560.0), 230.0);

        assert_eq!(verification_table_row_height(1_440.0), 28.0);
        assert_eq!(verification_table_row_height(820.0), 31.0);
        assert_eq!(verification_table_row_height(560.0), 36.0);
    }

    #[test]
    fn histogram_bins_preserve_every_exact_sample() {
        let samples = [0.0, 0.2, 0.4, 0.8, 1.0];
        let bins = histogram_bins(&samples, 4, 0.0, 1.0);

        assert_eq!(bins.iter().sum::<usize>(), samples.len());
        assert_eq!(bins, vec![2, 1, 0, 2]);
    }

    #[test]
    fn signed_margin_is_positive_inside_and_negative_outside_bounds() {
        let spec = SpecEntry {
            measurement: "gain".to_owned(),
            min: Some(10.0),
            max: Some(20.0),
            unit: "dB".to_owned(),
        };

        assert_eq!(signed_margin(&spec, 15.0), 5.0);
        assert_eq!(signed_margin(&spec, 8.0), -2.0);
        assert_eq!(signed_margin(&spec, 23.0), -3.0);
        assert_eq!(normalized_margin(&spec, 15.0), Some(0.5));
        assert_eq!(normalized_margin(&spec, 8.0), Some(-0.2));
    }

    #[test]
    fn joint_sample_summary_requires_aligned_trails_and_ands_every_spec() {
        let make_result = |target: &str, trail: Vec<bool>| {
            let pass_count = trail.iter().filter(|passes| **passes).count();
            crate::services::yield_manager::YieldResult {
                spec: crate::services::YieldSpec::lower(target, 0.0, ""),
                total_runs: trail.len(),
                pass_count,
                fail_count: trail.len() - pass_count,
                yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
                stats: crate::services::DistributionStats::default(),
                samples: vec![1.0; trail.len()],
                trail,
            }
        };
        let aligned = [
            make_result("gain", vec![true, false, false]),
            make_result("bandwidth", vec![true, false, true]),
        ];
        let joint = joint_sample_summary(&aligned).expect("aligned joint trail");
        assert_eq!(joint.passing, 1);
        assert_eq!(joint.total, 3);
        assert_eq!(joint.specification_count, 2);
        assert!((joint.yield_percent() - 100.0 / 3.0).abs() < 1.0e-12);
        assert_eq!(
            worst_individual_yield_result(&aligned)
                .expect("worst individual detail")
                .spec
                .target,
            "gain"
        );

        let misaligned = [
            make_result("gain", vec![true, true]),
            make_result("bandwidth", vec![true]),
        ];
        assert_eq!(joint_sample_summary(&misaligned), None);
    }

    #[test]
    fn joint_headline_can_be_lower_than_every_individual_spec_yield() {
        let make_result = |target: &str, trail: Vec<bool>| {
            let pass_count = trail.iter().filter(|passes| **passes).count();
            crate::services::yield_manager::YieldResult {
                spec: crate::services::YieldSpec::lower(target, 0.0, ""),
                total_runs: trail.len(),
                pass_count,
                fail_count: trail.len() - pass_count,
                yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
                stats: crate::services::DistributionStats::default(),
                samples: vec![1.0; trail.len()],
                trail,
            }
        };
        let results = [
            make_result("gain", vec![true, true, false, true]),
            make_result("bandwidth", vec![true, false, true, true]),
        ];

        let joint = joint_sample_summary(&results).expect("aligned joint trail");
        assert_eq!((joint.passing, joint.total), (2, 4));
        assert_eq!(joint.yield_percent(), 50.0);
        assert!(
            results
                .iter()
                .all(|result| result.yield_percent > joint.yield_percent())
        );
    }

    #[test]
    fn specification_editor_never_falls_through_to_an_inactive_dataset() {
        let mut inactive = SimulationRun::new(1);
        inactive.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "inactive")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 99.0)]),
        );
        let mut active = SimulationRun::new(2);
        active.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "active")
                .with_measurements(vec![rspice_core::MeasureResult::success("bandwidth", 42.0)]),
        );
        let active_dataset = active.dataset_id;
        let mut simulation = SimulationState {
            runs: vec![inactive, active],
            active_run_idx: Some(1),
            ..SimulationState::default()
        };

        assert_eq!(
            simulation.active_run().map(|run| run.dataset_id),
            Some(active_dataset)
        );
        assert_eq!(
            active_dataset_measurement(&simulation, "bandwidth"),
            Some(42.0)
        );
        assert_eq!(active_dataset_measurement(&simulation, "gain"), None);

        simulation.active_run_idx = None;
        assert_eq!(active_dataset_measurement(&simulation, "bandwidth"), None);
        assert_eq!(active_dataset_measurement(&simulation, "gain"), None);
    }
}
