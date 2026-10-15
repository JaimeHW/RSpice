//! The regression review: one table of every compared target, and the detail
//! of the one the reader selects.
//!
//! This is presentation over [`super::regression_contract`] — it renders the
//! verdicts and coverage gaps that module computes, and never re-derives one.
//! The toolbar names the candidate and the baseline and how many targets pass.
//! A row click selects a target; its tolerance form, and for a waveform its
//! comparison plot, open in the pane under the table. A target nobody selected
//! shows no detail.

use super::table::{Cell, Column, DataTable, TABLE_EDGE_INSET, TABLE_HEAD_H, TABLE_ROW_H, Verdict};
use super::toolbar::{PageToolbar, Summary, ToolbarAction, empty_line};
use super::*;

/// Least and greatest height of the detail pane under the table.
const DETAIL_MIN_H: f32 = 220.0;
const DETAIL_MAX_H: f32 = 320.0;
/// Width of the tolerance form beside a waveform plot.
const TOLERANCE_FORM_W: f32 = 360.0;

/// What the regression toolbar can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegressionAction {
    SelectBaseline,
    Run,
    Export,
    RemoveOrphans,
}

/// One table row: the target it compares, when it names one, and its cells.
struct RegressionRow {
    target: Option<crate::state::RegressionTargetSelector>,
    cells: Vec<Cell>,
}

pub(super) fn regression(ui: &mut Ui, app: &mut RSpiceApp) {
    let regression_policy = active_regression_specification_policy(app);
    let targets = regression_run_pair(app)
        .map(|(baseline, current)| {
            let checks = derive_regression_checks(baseline, current);
            let waveforms = if regression_requires_waveforms(regression_policy) {
                regression_waveform_pairs(baseline, current)
            } else {
                Vec::new()
            };
            regression_target_descriptors(&checks, &waveforms)
        })
        .unwrap_or_default();
    synchronize_regression_drafts(app, &targets);
    let mut session = app.state.workbench.verification.clone();
    let pair = regression_run_pair(app);
    let checks = pair
        .map(|(baseline, current)| derive_regression_checks(baseline, current))
        .unwrap_or_default();
    let waveforms = if regression_requires_waveforms(regression_policy) {
        pair.map(|(baseline, current)| regression_waveform_pairs(baseline, current))
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let rules = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.content.plan_data(plan.id()))
        .map(|payload| payload.regression_tolerances.clone())
        .unwrap_or_default();
    let coverage_issues = pair
        .map(|(baseline, current)| {
            regression_coverage_issues_for_policy(baseline, current, &rules, regression_policy)
        })
        .unwrap_or_default();
    let active_contract = app.state.sim_setup.analysis_plan.as_ref().map(|plan| {
        (
            plan.id(),
            plan.revision(),
            regression_tolerance_digest(&rules),
        )
    });
    let receipt = pair.and_then(|(baseline, current)| {
        session
            .regression_comparison
            .as_ref()
            .filter(|receipt| {
                active_contract.is_some_and(|(plan_id, revision, digest)| {
                    regression_receipt_matches_contract(
                        receipt, plan_id, revision, digest, baseline, current,
                    )
                })
            })
            .cloned()
    });
    let check_verdicts = checks
        .iter()
        .map(|check| evaluate_regression_check(check, regression_rule(&rules, &check.target)))
        .collect::<Vec<_>>();
    let waveform_verdicts = waveforms
        .iter()
        .map(|pair| evaluate_regression_waveform(pair, regression_rule(&rules, &pair.target)))
        .collect::<Vec<_>>();
    let verdicts = check_verdicts.iter().chain(&waveform_verdicts);
    let passed = verdicts.clone().filter(|verdict| verdict.passed()).count();
    let failures = verdicts.clone().filter(|verdict| verdict.failed()).count();
    let unresolved = verdicts
        .filter(|verdict| {
            matches!(
                verdict,
                RegressionVerdict::NotConfigured | RegressionVerdict::NotEvaluated(_)
            )
        })
        .count()
        + coverage_issues.len();
    let total = checks.len() + waveforms.len() + coverage_issues.len();
    let orphan_targets = orphaned_regression_targets(&coverage_issues);
    let export_documents = receipt.as_ref().map(|receipt| {
        let cases = regression_export_cases(
            &checks,
            &check_verdicts,
            &waveforms,
            &waveform_verdicts,
            &coverage_issues,
        );
        regression_ci_documents(receipt, &cases)
    });

    let runs = run_choice(app, |run| validate_regression_run(run).is_ok());
    let candidate_name = verification_run(app).map(|run| format!("Run {}", run.id));
    let summary = pair.map(|(baseline, current)| {
        let text = format!(
            "Run {} vs baseline Run {} \u{00b7} {passed} / {total} pass",
            current.id, baseline.id
        );
        if failures > 0 {
            Summary::failing(text)
        } else if unresolved > 0 {
            Summary::warning(format!("{text} \u{00b7} {unresolved} unresolved"))
        } else {
            Summary::plain(text)
        }
    });
    let mut actions = vec![RegressionAction::SelectBaseline, RegressionAction::Run];
    let mut toolbar_actions = vec![
        ToolbarAction::new("Select baseline"),
        ToolbarAction::new("Run regression")
            .enabled(pair.is_some())
            .disabled_reason("Select a baseline run first."),
        ToolbarAction::new("Export JUnit / TAP")
            .enabled(export_documents.is_some())
            .disabled_reason("Run regression first."),
    ];
    actions.push(RegressionAction::Export);
    if !orphan_targets.is_empty() {
        actions.push(RegressionAction::RemoveOrphans);
        toolbar_actions.push(ToolbarAction::new("Remove orphaned rules"));
    }
    let output = PageToolbar {
        title: "Regression",
        runs: &runs,
        summary,
        actions: &toolbar_actions,
    }
    .show(ui);
    action_problem_line(ui, app);
    if !orphan_targets.is_empty() {
        let names = coverage_issues
            .iter()
            .filter(|issue| {
                issue
                    .target
                    .as_ref()
                    .is_some_and(|target| orphan_targets.contains(target))
            })
            .map(|issue| issue.label.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let _ = crate::ui::widgets::problem_line(
            ui,
            Problem::Warning,
            &format!(
                "{} tolerance rule{} name{} targets neither run holds: {names}",
                orphan_targets.len(),
                if orphan_targets.len() == 1 { "" } else { "s" },
                if orphan_targets.len() == 1 { "s" } else { "" },
            ),
        );
    }

    let mut clicked = None;
    if pair.is_none() {
        let text = candidate_name.map_or_else(
            || "No run selected. Regression compares a run of this plan against a baseline run.".to_owned(),
            |run| format!("No baseline selected. Select the run {run} is compared against."),
        );
        let _ = empty_line(ui, &text, None);
    } else {
        let rows = regression_rows(
            &checks,
            &check_verdicts,
            &waveforms,
            &waveform_verdicts,
            &coverage_issues,
            &rules,
        );
        let selected = session.regression_selected_target.as_ref().and_then(|selected| {
            rows.iter()
                .position(|row| row.target.as_ref() == Some(selected))
        });
        let body = ui.available_height();
        let detail = selected.map_or(0.0, |_| (body * 0.45).clamp(DETAIL_MIN_H, DETAIL_MAX_H));
        let columns = [
            Column::name("Check", 160.0),
            Column::words("Method", 120.0),
            Column::number("Baseline", 100.0),
            Column::number("Current", 100.0),
            Column::number("Delta", 100.0),
            Column::words("Tolerance", 150.0),
            Column::words("Result", 180.0),
        ];
        clicked = DataTable::new("verify.regression.checks", &columns, rows.len())
            .clickable()
            .selected(selected)
            .max_height((body - detail - 1.0).max(TABLE_HEAD_H + TABLE_ROW_H))
            .show(ui, |index| rows[index].cells.clone())
            .clicked
            .map(|index| rows[index].target.clone());
        if let Some(target) = selected.and_then(|index| rows[index].target.clone()) {
            regression_detail(ui, &mut session, &target, &waveforms, &rules, &checks);
        }
    }
    drop(waveforms);

    if let Some(target) = clicked {
        session.regression_selected_target = if target == session.regression_selected_target {
            None
        } else {
            target
        };
    }
    let mut removed_orphans = false;
    match output.action.and_then(|index| actions.get(index)) {
        Some(RegressionAction::SelectBaseline) => {
            app.state
                .workbench
                .verification
                .regression_baseline_picker_selection = active_regression_baseline(app);
            app.state
                .workbench
                .verification
                .regression_baseline_picker_open = true;
        }
        Some(RegressionAction::Run) => run_regression_comparison(app),
        Some(RegressionAction::Export) => match export_documents {
            Some(Ok((junit, tap))) => export_regression_ci(app, &junit, &tap),
            Some(Err(error)) => {
                app.state.workbench.verification.action_problem =
                    Some(format!("JUnit / TAP export failed: {error}."));
            }
            None => {}
        },
        Some(RegressionAction::RemoveOrphans) => {
            removed_orphans = true;
            let result = remove_orphaned_regression_rules(app, &orphan_targets).map(|_| ());
            record_action(&mut app.state.workbench.verification, result);
        }
        None => {}
    }
    if let Some(index) = output.run {
        select_verification_run(&mut app.state.simulation, index);
    }
    if !removed_orphans {
        app.state.workbench.verification.regression_selected_target =
            session.regression_selected_target;
        app.state.workbench.verification.regression_tolerance_drafts =
            session.regression_tolerance_drafts;
    }
}

/// Every compared target as a table row: the measurements, then the
/// waveforms, then the targets that could not be compared.
fn regression_rows(
    checks: &[RegressionCheck],
    check_verdicts: &[RegressionVerdict],
    waveforms: &[RegressionWaveformPair<'_>],
    waveform_verdicts: &[RegressionVerdict],
    coverage_issues: &[RegressionCoverageIssue],
    rules: &[crate::state::RegressionToleranceRule],
) -> Vec<RegressionRow> {
    let method = |rule: Option<&crate::state::RegressionToleranceRule>| {
        rule.map_or_else(Cell::absent, |rule| Cell::text(rule.method.label()))
    };
    let tolerance = |rule: Option<&crate::state::RegressionToleranceRule>| {
        rule.map_or_else(Cell::absent, |rule| Cell::number(format_tolerance_rule(rule)))
    };
    let mut rows = checks
        .iter()
        .zip(check_verdicts)
        .map(|(check, verdict)| {
            let rule = regression_rule(rules, &check.target);
            RegressionRow {
                target: Some(check.target.clone()),
                cells: vec![
                    Cell::text(&check.name),
                    method(rule),
                    Cell::number(format_scalar(check.baseline)),
                    Cell::number(format_scalar(check.current)),
                    Cell::number(signed_scalar(check.delta())),
                    tolerance(rule),
                    regression_status_cell(verdict),
                ],
            }
        })
        .collect::<Vec<_>>();
    rows.extend(waveforms.iter().zip(waveform_verdicts).map(|(pair, verdict)| {
        let rule = regression_rule(rules, &pair.target);
        let worst = match verdict {
            RegressionVerdict::Pass { worst_delta, .. }
            | RegressionVerdict::Fail { worst_delta, .. } => Some(*worst_delta),
            RegressionVerdict::NotConfigured | RegressionVerdict::NotEvaluated(_) => None,
        };
        RegressionRow {
            target: Some(pair.target.clone()),
            cells: vec![
                Cell::text(format!("Waveform {}", pair.current.name)),
                method(rule),
                Cell::number(format!("{} pts", pair.baseline.y.len())),
                Cell::number(format!("{} pts", pair.current.y.len())),
                worst.map_or_else(Cell::absent, |delta| {
                    Cell::number(format!("max {}", format_scalar(delta)))
                }),
                tolerance(rule),
                regression_status_cell(verdict),
            ],
        }
    }));
    rows.extend(coverage_issues.iter().map(|issue| RegressionRow {
        target: issue.target.clone(),
        cells: vec![
            Cell::text(&issue.label),
            Cell::absent(),
            Cell::absent(),
            Cell::absent(),
            Cell::absent(),
            Cell::absent(),
            Cell::problem(format!("Blocked \u{00b7} {}", issue.detail), Problem::Warning),
        ],
    }));
    rows
}

/// A regression target's verdict cell: a failure in the error colour, a pass
/// neutral, an unconfigured tolerance plain, and a target that could not be
/// evaluated behind a warning mark — blocked is not a failure.
fn regression_status_cell(verdict: &RegressionVerdict) -> Cell {
    match verdict {
        RegressionVerdict::Pass { .. } => Cell::verdict(Verdict::Pass),
        RegressionVerdict::Fail { detail, .. } => {
            Cell::text(format!("Fail \u{00b7} {detail}")).judged(Verdict::Fail)
        }
        RegressionVerdict::NotConfigured => Cell::text("No tolerance"),
        RegressionVerdict::NotEvaluated(detail) => {
            Cell::problem(format!("Blocked \u{00b7} {detail}"), Problem::Warning)
        }
    }
}

/// The selected target's detail: its waveform comparison, when it is a
/// waveform, beside the form that sets its tolerance.
fn regression_detail(
    ui: &mut Ui,
    session: &mut VerificationSessionState,
    target: &crate::state::RegressionTargetSelector,
    waveforms: &[RegressionWaveformPair<'_>],
    rules: &[crate::state::RegressionToleranceRule],
    checks: &[RegressionCheck],
) {
    let t = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    let left = ui.max_rect().left();
    let width = ui.available_width();
    ui.painter().hline(
        left..=left + width,
        top + 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(1.0);
    let height = ui.available_height().clamp(DETAIL_MIN_H, DETAIL_MAX_H);
    let pane = egui::Rect::from_min_size(egui::pos2(left, top + 1.0), egui::vec2(width, height));
    let _ = ui.allocate_exact_size(pane.size(), egui::Sense::hover());
    #[cfg(test)]
    super::raster::probe(ui, "split", pane);
    ui.painter().rect_filled(pane, 0.0, t.color.bg_panel);
    let waveform = waveforms.iter().find(|pair| pair.target == *target);
    let form_width = if waveform.is_some() {
        TOLERANCE_FORM_W.min(width * 0.5)
    } else {
        width
    };
    let form_rect = egui::Rect::from_min_max(
        egui::pos2(pane.right() - form_width, pane.top()),
        pane.max,
    );
    if let Some(pair) = waveform {
        let plot_rect =
            egui::Rect::from_min_max(pane.min, egui::pos2(form_rect.left() - 1.0, pane.bottom()));
        ui.painter().vline(
            form_rect.left() - 0.5,
            pane.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
        let mut plot_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(plot_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        plot_ui.set_clip_rect(plot_rect.intersect(ui.clip_rect()));
        regression_waveform_chart(&mut plot_ui, pair, regression_rule(rules, &pair.target));
    }
    let title = checks
        .iter()
        .find(|check| check.target == *target)
        .map(|check| check.name.clone())
        .or_else(|| waveform.map(|pair| pair.current.name.clone()))
        .unwrap_or_else(|| target.name.clone());
    let mut form_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(form_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    form_ui.set_clip_rect(form_rect.intersect(ui.clip_rect()));
    tolerance_form(&mut form_ui, session, target, &title);
}

/// The tolerance a target is judged by, edited in place; saved with the next
/// regression run.
fn tolerance_form(
    ui: &mut Ui,
    session: &mut VerificationSessionState,
    target: &crate::state::RegressionTargetSelector,
    title: &str,
) {
    crate::workbench::design_system::section_header(ui, &format!("{title} tolerance"), None);
    let Some(draft) = session
        .regression_tolerance_drafts
        .iter_mut()
        .find(|draft| draft.target == *target)
    else {
        return;
    };
    crate::workbench::design_system::panel_body(ui, |ui| {
        let previous_method = draft.method;
        egui::ComboBox::from_id_salt("verify.regression.tolerance.method")
            .selected_text(draft.method.label())
            .width(ui.available_width().max(120.0))
            .show_ui(ui, |ui| {
                for method in crate::state::RegressionComparisonMethod::ALL {
                    ui.selectable_value(&mut draft.method, method, method.label());
                }
            });
        draft.dirty |= draft.method != previous_method;
        ui.add_space(4.0);
        egui::Grid::new("verify.regression.tolerance.fields")
            .num_columns(2)
            .spacing(egui::vec2(8.0, 4.0))
            .show(ui, |ui| {
                let field = |ui: &mut Ui, label: &str, value: &mut String, hint: &str| {
                    ui.label(label);
                    let changed = ui
                        .add(egui::TextEdit::singleline(value).hint_text(hint))
                        .changed();
                    ui.end_row();
                    changed
                };
                draft.dirty |= field(ui, "Absolute", &mut draft.absolute_tolerance, "10m");
                draft.dirty |= field(
                    ui,
                    "Relative (%)",
                    &mut draft.relative_tolerance_percent,
                    "0.5",
                );
                if draft.target.kind == crate::state::RegressionTargetKind::Waveform {
                    draft.dirty |= field(ui, "Time skew", &mut draft.time_skew_allowance, "20u");
                    draft.dirty |= field(ui, "Window", &mut draft.comparison_window, "0 … 20m");
                }
            });
        let t = Tokens::get(ui.ctx());
        // The error sits under the form it rejects. Saved is the normal
        // state, and says nothing.
        if let Some(error) = &draft.validation_error {
            let _ = crate::ui::widgets::problem_line(ui, Problem::Failure, error);
        } else if draft.dirty {
            ui.label(
                egui::RichText::new("Unsaved: applied on the next regression run.")
                    .color(t.color.text_dim),
            );
        }
    });
}

/// Baseline, candidate and the tolerance envelope of one waveform target.
fn regression_waveform_chart(
    ui: &mut Ui,
    pair: &RegressionWaveformPair<'_>,
    rule: Option<&crate::state::RegressionToleranceRule>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let height = ui.available_height().max(1.0);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let baseline = pair.baseline;
    let current = pair.current;
    let candidate_color = t.color.traces[0];
    let header = egui::Rect::from_min_size(rect.min, egui::vec2(width, PANEL_HEADER_H_CHART));
    ui.painter().text(
        egui::pos2(header.left() + TABLE_EDGE_INSET, header.center().y),
        egui::Align2::LEFT_CENTER,
        &current.name,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let legend_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let mut x = header.right() - TABLE_EDGE_INSET;
    for (label, stroke, dashed) in [
        ("tolerance", egui::Stroke::new(1.0, t.color.text_faint), true),
        ("candidate", egui::Stroke::new(1.5, candidate_color), false),
        ("baseline", egui::Stroke::new(1.5, t.color.text_dim), false),
    ] {
        if rule.is_none() && dashed {
            continue;
        }
        let galley =
            ui.painter()
                .layout_no_wrap(label.to_owned(), legend_font.clone(), t.color.text_dim);
        x -= galley.size().x;
        ui.painter().galley(
            egui::pos2(x, header.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_dim,
        );
        let line = [
            egui::pos2(x - 20.0, header.center().y),
            egui::pos2(x - 6.0, header.center().y),
        ];
        if dashed {
            ui.painter()
                .extend(egui::Shape::dashed_line(&line, stroke, 3.0, 2.0));
        } else {
            ui.painter().line_segment(line, stroke);
        }
        x -= 34.0;
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Waveform {}: baseline {} samples, candidate {} samples",
                current.name,
                baseline.y.len(),
                current.y.len()
            ),
        )
    });

    let mut finite_values = baseline
        .y
        .iter()
        .chain(current.y.iter())
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    if let Some(rule) = rule {
        finite_values.extend(
            baseline
                .y
                .iter()
                .copied()
                .filter(|value| value.is_finite())
                .flat_map(|value| {
                    let allowed = permitted_delta(value, rule);
                    [value - allowed, value + allowed]
                }),
        );
    }
    let finite_x = baseline
        .x
        .iter()
        .chain(current.x.iter())
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    let (Some(y_min), Some(y_max), Some(x_min), Some(x_max)) = (
        finite_values.iter().copied().min_by(f64::total_cmp),
        finite_values.iter().copied().max_by(f64::total_cmp),
        finite_x.iter().copied().min_by(f64::total_cmp),
        finite_x.iter().copied().max_by(f64::total_cmp),
    ) else {
        return;
    };
    let plot = egui::Rect::from_min_max(
        header.left_bottom() + egui::vec2(TABLE_EDGE_INSET, 6.0),
        rect.right_bottom() - egui::vec2(TABLE_EDGE_INSET, TABLE_EDGE_INSET),
    );
    if plot.width() <= 4.0 || plot.height() <= 4.0 {
        return;
    }
    ui.painter().rect_filled(plot, 0.0, t.color.canvas_bg);
    for step in 1..4 {
        let y = egui::lerp(plot.bottom()..=plot.top(), step as f32 / 4.0);
        ui.painter().hline(
            plot.x_range(),
            y,
            egui::Stroke::new(1.0, t.color.canvas_grid),
        );
    }
    let points = |x_values: &[f64], y_values: &[f64]| {
        x_values
            .iter()
            .zip(y_values)
            .filter(|(x_value, y_value)| x_value.is_finite() && y_value.is_finite())
            .map(|(x_value, y_value)| {
                let x_fraction = if (x_max - x_min).abs() <= f64::EPSILON {
                    0.5
                } else {
                    ((*x_value - x_min) / (x_max - x_min)) as f32
                };
                let y_fraction = if (y_max - y_min).abs() <= f64::EPSILON {
                    0.5
                } else {
                    ((*y_value - y_min) / (y_max - y_min)) as f32
                };
                egui::pos2(
                    egui::lerp(plot.left()..=plot.right(), x_fraction),
                    egui::lerp(plot.bottom()..=plot.top(), y_fraction),
                )
            })
            .collect::<Vec<_>>()
    };
    ui.painter().add(egui::Shape::line(
        points(baseline.x.as_slice(), baseline.y.as_slice()),
        egui::Stroke::new(1.5, t.color.text_dim),
    ));
    if let Some(rule) = rule {
        for side in [-1.0, 1.0] {
            let envelope = baseline
                .y
                .iter()
                .map(|value| *value + side * permitted_delta(*value, rule))
                .collect::<Vec<_>>();
            ui.painter().extend(egui::Shape::dashed_line(
                &points(baseline.x.as_slice(), &envelope),
                egui::Stroke::new(1.0, t.color.text_faint),
                4.0,
                3.0,
            ));
        }
    }
    ui.painter().add(egui::Shape::line(
        points(current.x.as_slice(), current.y.as_slice()),
        egui::Stroke::new(1.5, candidate_color),
    ));
}

/// Height of the waveform plot's header row.
const PANEL_HEADER_H_CHART: f32 = 24.0;
