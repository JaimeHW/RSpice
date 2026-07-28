//! The regression review pane: verdicts, the baseline contract, and the diff.
//!
//! This is presentation over [`super::regression_contract`] — it renders the
//! verdicts and coverage gaps that module computes, and never re-derives one.
//! The contract pane deliberately shows the digest a verdict was anchored to,
//! so a reviewer can see which baseline a pass actually refers to.

use super::*;

pub(super) fn regression(ui: &mut Ui, app: &mut RSpiceApp, viewport_height: f32) {
    let surface_top = ui.cursor().top();
    let targets = regression_run_pair(app)
        .map(|(baseline, current)| {
            let checks = derive_regression_checks(baseline, current);
            let waveforms = regression_waveform_pairs(baseline, current);
            regression_target_descriptors(&checks, &waveforms)
        })
        .unwrap_or_default();
    synchronize_regression_drafts(app, &targets);
    let mut regression_session = app.state.workbench.verification.clone();
    let pair = regression_run_pair(app);
    let checks = pair
        .map(|(baseline, current)| derive_regression_checks(baseline, current))
        .unwrap_or_default();
    let waveforms = pair
        .map(|(baseline, current)| regression_waveform_pairs(baseline, current))
        .unwrap_or_default();
    let rules = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.active_plan_data(plan.id()))
        .map(|payload| payload.regression_tolerances.clone())
        .unwrap_or_default();
    let coverage_issues = pair
        .map(|(baseline, current)| regression_coverage_issues(baseline, current, &rules))
        .unwrap_or_default();
    let active_contract = app.state.sim_setup.analysis_plan.as_ref().map(|plan| {
        (
            plan.id(),
            plan.revision(),
            regression_tolerance_digest(&rules),
        )
    });
    let baseline_contract = pair.map(|(baseline, current)| RegressionBaselineContractSnapshot {
        baseline_id: baseline.id,
        candidate_id: current.id,
        baseline_dataset: baseline.dataset_id.to_string(),
        baseline_revision: baseline
            .prepared_receipt()
            .expect("run pair requires prepared baseline")
            .project_revision(),
        baseline_recorded_at: baseline.timestamp,
        baseline_provenance: if baseline.prepared_receipt().is_some() {
            "prepared snapshot retained"
        } else {
            "legacy / unclassified"
        },
        receipt: regression_session
            .regression_comparison
            .as_ref()
            .filter(|receipt| {
                active_contract.is_some_and(|(plan_id, revision, digest)| {
                    regression_receipt_matches_contract(
                        receipt, plan_id, revision, digest, baseline, current,
                    )
                })
            })
            .cloned(),
    });
    let check_verdicts = checks
        .iter()
        .map(|check| evaluate_regression_check(check, regression_rule(&rules, &check.target)))
        .collect::<Vec<_>>();
    let waveform_verdicts = waveforms
        .iter()
        .map(|pair| evaluate_regression_waveform(pair, regression_rule(&rules, &pair.target)))
        .collect::<Vec<_>>();
    let t = Tokens::get(ui.ctx());
    let passed_checks = check_verdicts
        .iter()
        .filter(|verdict| verdict.passed())
        .count();
    let passed_waveforms = waveform_verdicts
        .iter()
        .filter(|verdict| verdict.passed())
        .count();
    let failures = check_verdicts
        .iter()
        .chain(&waveform_verdicts)
        .filter(|verdict| verdict.failed())
        .count();
    let unresolved = check_verdicts
        .iter()
        .chain(&waveform_verdicts)
        .filter(|verdict| {
            matches!(
                verdict,
                RegressionVerdict::NotConfigured | RegressionVerdict::NotEvaluated(_)
            )
        })
        .count()
        + coverage_issues.len();
    let worst_normalized = check_verdicts
        .iter()
        .chain(&waveform_verdicts)
        .filter_map(|verdict| match verdict {
            RegressionVerdict::Pass {
                worst_delta,
                allowed_delta,
            }
            | RegressionVerdict::Fail {
                worst_delta,
                allowed_delta,
                ..
            } => Some(if *allowed_delta > 0.0 {
                *worst_delta / *allowed_delta
            } else if *worst_delta == 0.0 {
                0.0
            } else {
                f64::INFINITY
            }),
            RegressionVerdict::NotConfigured | RegressionVerdict::NotEvaluated(_) => None,
        })
        .max_by(f64::total_cmp);
    let configured = targets
        .iter()
        .filter(|target| regression_rule(&rules, &target.target).is_some())
        .count();
    let comparison_available = pair.is_some();
    let overall_pass = failures == 0 && unresolved == 0 && !targets.is_empty();
    let items = [
        (
            "Checks passing".to_owned(),
            format!("{passed_checks} / {}", checks.len()),
            if comparison_available {
                format!("{failures} blocking failures across all targets")
            } else {
                "retain two immutable runs before evaluation".to_owned()
            },
            if checks.is_empty() {
                t.color.text_dim
            } else if failures == 0 {
                t.color.ok
            } else {
                t.color.err
            },
        ),
        (
            "Waveform matches".to_owned(),
            format!("{passed_waveforms} / {}", waveforms.len()),
            if comparison_available {
                "configured envelope, skew, and window".to_owned()
            } else {
                "no source-aligned waveform pair".to_owned()
            },
            if waveforms.is_empty() {
                t.color.text_dim
            } else if passed_waveforms == waveforms.len() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        (
            "Worst normalized delta".to_owned(),
            worst_normalized.map_or_else(
                || "No verdict".to_owned(),
                |value| {
                    if value.is_finite() {
                        format!("{value:.3} × limit")
                    } else {
                        "∞ × limit".to_owned()
                    }
                },
            ),
            if !comparison_available {
                "no retained comparison pair is available".to_owned()
            } else if overall_pass {
                "all evaluated targets pass".to_owned()
            } else {
                format!("{unresolved} targets unresolved")
            },
            if !comparison_available {
                t.color.text_dim
            } else if overall_pass {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        (
            "Tolerance contract".to_owned(),
            format!("{configured} / {}", targets.len()),
            if targets.is_empty() {
                "no comparison targets available".to_owned()
            } else if regression_session
                .regression_tolerance_drafts
                .iter()
                .any(|draft| draft.dirty)
            {
                "pending edits apply on Run regression".to_owned()
            } else {
                "persisted by active simulation plan".to_owned()
            },
            if targets.is_empty() {
                t.color.text_dim
            } else if configured == targets.len() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    verification_kpi_strip(ui, &items);

    let width = ui.available_width();
    let viewport_width = ui.ctx().content_rect().width();
    let header_height = if t.metrics.ctl_h >= 44.0 { 44.0 } else { 37.0 };
    let visible_remaining =
        remaining_viewport_height(viewport_height, ui.cursor().top() - surface_top);
    let comparison_rows = checks.len() + waveforms.len() + coverage_issues.len();
    let layout = VerifyLayout::resolve(
        viewport_width,
        width,
        visible_remaining,
        comparison_rows,
        header_height,
        verification_table_row_height(viewport_width),
    );
    if layout.split {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(
                egui::vec2(layout.left_width, layout.first_row_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    regression_waveform_chart(
                        ui,
                        &waveforms,
                        &rules,
                        regression_session.regression_selected_target.as_ref(),
                    )
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(layout.right_width, layout.first_row_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    regression_baseline_contract_pane(
                        ui,
                        &mut regression_session,
                        baseline_contract.as_ref(),
                        &targets,
                    )
                },
            );
        });
    } else {
        let chart_height = verification_stacked_chart_height(viewport_width);
        ui.allocate_ui(egui::vec2(width, chart_height), |ui| {
            regression_waveform_chart(
                ui,
                &waveforms,
                &rules,
                regression_session.regression_selected_target.as_ref(),
            )
        });
        ui.add_space(1.0);
        regression_baseline_contract(
            ui,
            &mut regression_session,
            baseline_contract.as_ref(),
            &targets,
        );
    }
    ui.add_space(1.0);
    let headers = vec![
        ("Check".to_owned(), 0.17),
        ("Comparison".to_owned(), 0.14),
        ("Baseline".to_owned(), 0.14),
        ("Current".to_owned(), 0.14),
        ("Delta".to_owned(), 0.14),
        ("Tolerance".to_owned(), 0.13),
        ("Status".to_owned(), 0.14),
    ];
    let mut rows = checks
        .iter()
        .zip(&check_verdicts)
        .map(|(check, verdict)| {
            let rule = regression_rule(&rules, &check.target);
            let (status, color) = match verdict {
                RegressionVerdict::Pass { .. } => ("PASS".to_owned(), t.color.ok),
                RegressionVerdict::Fail { detail, .. } => (format!("FAIL · {detail}"), t.color.err),
                RegressionVerdict::NotConfigured => ("NOT CONFIGURED".to_owned(), t.color.warn),
                RegressionVerdict::NotEvaluated(detail) => {
                    (format!("BLOCKED · {detail}"), t.color.err)
                }
            };
            vec![
                TableCell::text(&check.name),
                TableCell::text(format!(
                    "{} · {}",
                    rule.map_or("unconfigured", |rule| rule.method.label()),
                    check.source_identity
                )),
                TableCell::mono(format_scalar(check.baseline)),
                TableCell::mono(format_scalar(check.current)),
                TableCell::mono(format!("{:+.6e}", check.delta())),
                TableCell::mono(
                    rule.map_or_else(|| "not configured".to_owned(), format_tolerance_rule),
                ),
                TableCell::tone(status, color),
            ]
        })
        .collect::<Vec<_>>();
    rows.extend(
        waveforms
            .iter()
            .zip(&waveform_verdicts)
            .map(|(pair, verdict)| {
                let rule = regression_rule(&rules, &pair.target);
                let (delta, allowed, status, color) = match verdict {
                    RegressionVerdict::Pass {
                        worst_delta,
                        allowed_delta,
                    } => (*worst_delta, *allowed_delta, "PASS".to_owned(), t.color.ok),
                    RegressionVerdict::Fail {
                        worst_delta,
                        allowed_delta,
                        detail,
                    } => (
                        *worst_delta,
                        *allowed_delta,
                        format!("FAIL · {detail}"),
                        t.color.err,
                    ),
                    RegressionVerdict::NotConfigured => (
                        f64::NAN,
                        f64::NAN,
                        "NOT CONFIGURED".to_owned(),
                        t.color.warn,
                    ),
                    RegressionVerdict::NotEvaluated(detail) => (
                        f64::NAN,
                        f64::NAN,
                        format!("BLOCKED · {detail}"),
                        t.color.err,
                    ),
                };
                vec![
                    TableCell::text(format!("Waveform · {}", pair.current.name)),
                    TableCell::text(rule.map_or("unconfigured", |rule| rule.method.label())),
                    TableCell::mono(format!("{} samples", pair.baseline.y.len())),
                    TableCell::mono(format!("{} samples", pair.current.y.len())),
                    TableCell::mono(if delta.is_finite() {
                        format!("max {delta:.6e}")
                    } else {
                        "—".to_owned()
                    }),
                    TableCell::mono(rule.map_or_else(
                        || {
                            if allowed.is_finite() {
                                format!("{allowed:.6e}")
                            } else {
                                "not configured".to_owned()
                            }
                        },
                        format_tolerance_rule,
                    )),
                    TableCell::tone(status, color),
                ]
            }),
    );
    rows.extend(coverage_issues.iter().map(|issue| {
        vec![
            TableCell::text(&issue.label),
            TableCell::text("evidence coverage"),
            TableCell::mono("—"),
            TableCell::mono("—"),
            TableCell::mono("—"),
            TableCell::mono("fail closed"),
            TableCell::tone(format!("BLOCKED · {}", issue.detail), t.color.err),
        ]
    }));
    let orphan_targets = orphaned_regression_targets(&coverage_issues);
    let mut remove_orphans_requested = false;
    if !orphan_targets.is_empty() {
        let names = coverage_issues
            .iter()
            .filter(|issue| {
                issue.detail == "persisted tolerance target is absent from both datasets"
            })
            .map(|issue| issue.label.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        card(ui, "Orphaned tolerance recovery", |ui| {
            ui.label(format!(
                "{} persisted tolerance target(s) are absent from both retained datasets: {names}",
                orphan_targets.len()
            ));
            ui.label(
                "Remove these obsolete rules, then select a current target to configure its replacement contract.",
            );
            remove_orphans_requested = Button::new("Remove orphaned tolerances").show(ui).clicked();
        });
    }
    let export_documents = baseline_contract
        .as_ref()
        .and_then(|contract| contract.receipt.as_ref())
        .map(|receipt| {
            let cases = regression_export_cases(
                &checks,
                &check_verdicts,
                &waveforms,
                &waveform_verdicts,
                &coverage_issues,
            );
            regression_ci_documents(receipt, &cases)
        });
    let export_requested = table_section_header(
        ui,
        "Regression checks",
        Some("persisted per-check tolerance and deterministic verdict"),
        export_documents.as_ref().map(|_| "Export JUnit / TAP"),
    );
    render_data_table(ui, "verify-regression-checks", &headers, &rows, None);
    drop(waveforms);
    if remove_orphans_requested {
        app.state.workbench.verification.action_receipt = match remove_orphaned_regression_rules(
            app,
            &orphan_targets,
        ) {
            Ok(removed) => format!(
                "Removed {removed} orphaned regression tolerance rule(s). Configure replacement targets before the next governed comparison."
            ),
            Err(error) => format!("Orphaned tolerance removal blocked: {error}."),
        };
    }
    if export_requested && let Some(documents) = export_documents {
        match documents {
            Ok((junit, tap)) => export_regression_ci(app, &junit, &tap),
            Err(error) => {
                app.state.workbench.verification.action_receipt =
                    format!("Golden Regression CI export blocked: {error}.");
            }
        }
    }
    if !remove_orphans_requested {
        app.state.workbench.verification.regression_selected_target =
            regression_session.regression_selected_target;
        app.state.workbench.verification.regression_tolerance_drafts =
            regression_session.regression_tolerance_drafts;
    }
    action_receipt(ui, app);
}

pub(super) fn regression_baseline_contract_pane(
    ui: &mut Ui,
    session: &mut super::super::super::state::VerificationSessionState,
    contract: Option<&RegressionBaselineContractSnapshot>,
    targets: &[RegressionTargetDescriptor],
) {
    let t = Tokens::get(ui.ctx());
    let size = ui.available_size().max(egui::Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect_filled(viewport, 0.0, t.color.bg_app);
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    child.spacing_mut().item_spacing.y = 0.0;
    ScrollArea::vertical()
        .id_salt("verify.regression.baseline-contract")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(viewport.width());
            regression_baseline_contract(ui, session, contract, targets);
        });
}

#[derive(Debug, Clone)]
pub(super) struct RegressionBaselineContractSnapshot {
    pub(super) baseline_id: u64,
    pub(super) candidate_id: u64,
    pub(super) baseline_dataset: String,
    pub(super) baseline_revision: crate::product::ObjectRevision,
    pub(super) baseline_recorded_at: f64,
    pub(super) baseline_provenance: &'static str,
    pub(super) receipt: Option<super::super::super::state::RegressionComparisonReceipt>,
}

pub(super) fn regression_baseline_contract(
    ui: &mut Ui,
    session: &mut super::super::super::state::VerificationSessionState,
    contract: Option<&RegressionBaselineContractSnapshot>,
    targets: &[RegressionTargetDescriptor],
) {
    table_section_header(ui, "Baseline contract", Some("immutable reference"), None);
    if let Some(contract) = contract {
        property_row(ui, "Baseline run", &format!("Run {}", contract.baseline_id));
        property_row(
            ui,
            "Candidate run",
            &format!("Run {}", contract.candidate_id),
        );
        property_row(ui, "Baseline dataset", &contract.baseline_dataset);
        property_row(
            ui,
            "Revision",
            &contract.baseline_revision.get().to_string(),
        );
        property_row(ui, "Producer engine", "not retained by this dataset schema");
        property_row(
            ui,
            "Comparison engine",
            concat!("RSpice ", env!("CARGO_PKG_VERSION")),
        );
        property_row(
            ui,
            "Recorded authority",
            &format!(
                "Local prepared-run authority · {:.3} Unix s",
                contract.baseline_recorded_at
            ),
        );
        property_row(
            ui,
            "Replacement policy",
            "Explicit selection · simulation-plan revision required",
        );
        property_row(ui, "Baseline provenance", contract.baseline_provenance);
        property_row(
            ui,
            "Comparison receipt",
            &contract.receipt.as_ref().map_or_else(
                || "not run / stale".to_owned(),
                |receipt| {
                    format!(
                        "{} checks · {} waveforms · {} pass · {} fail · {} blocked",
                        receipt.aligned_checks,
                        receipt.aligned_waveforms,
                        receipt.passed_checks + receipt.passed_waveforms,
                        receipt.failed_checks + receipt.failed_waveforms,
                        receipt.unconfigured_targets + receipt.unevaluated_targets,
                    )
                },
            ),
        );
        ui.add_space(5.0);
        table_section_header(
            ui,
            "Tolerance & decision",
            Some("per selected immutable result target"),
            None,
        );
        let mut selected = session.regression_selected_target.clone();
        let selected_label = selected
            .as_ref()
            .and_then(|selected| targets.iter().find(|target| target.target == *selected))
            .map_or("No aligned target", |target| target.label.as_str());
        egui::ComboBox::from_id_salt("verify.regression.tolerance.target")
            .selected_text(selected_label)
            .width(ui.available_width().max(120.0))
            .show_ui(ui, |ui| {
                for target in targets {
                    ui.selectable_value(&mut selected, Some(target.target.clone()), &target.label);
                }
            });
        session.regression_selected_target = selected.clone();
        if let Some(index) = selected.as_ref().and_then(|selected| {
            session
                .regression_tolerance_drafts
                .iter()
                .position(|draft| draft.target == *selected)
        }) {
            let draft = &mut session.regression_tolerance_drafts[index];
            let previous_method = draft.method;
            egui::ComboBox::from_label("Comparison method")
                .selected_text(draft.method.label())
                .width(ui.available_width().max(120.0))
                .show_ui(ui, |ui| {
                    for method in crate::state::RegressionComparisonMethod::ALL {
                        ui.selectable_value(&mut draft.method, method, method.label());
                    }
                });
            draft.dirty |= draft.method != previous_method;
            egui::Grid::new("verify.regression.tolerance.fields")
                .num_columns(2)
                .spacing(egui::vec2(8.0, 4.0))
                .show(ui, |ui| {
                    ui.label("Absolute tolerance");
                    draft.dirty |= ui
                        .add(
                            egui::TextEdit::singleline(&mut draft.absolute_tolerance)
                                .hint_text("10m"),
                        )
                        .changed();
                    ui.end_row();
                    ui.label("Relative tolerance (%)");
                    draft.dirty |= ui
                        .add(
                            egui::TextEdit::singleline(&mut draft.relative_tolerance_percent)
                                .hint_text("0.5"),
                        )
                        .changed();
                    ui.end_row();
                    if draft.target.kind == crate::state::RegressionTargetKind::Waveform {
                        ui.label("Time-skew allowance");
                        draft.dirty |= ui
                            .add(
                                egui::TextEdit::singleline(&mut draft.time_skew_allowance)
                                    .hint_text("20u"),
                            )
                            .changed();
                        ui.end_row();
                        ui.label("Comparison window");
                        draft.dirty |= ui
                            .add(
                                egui::TextEdit::singleline(&mut draft.comparison_window)
                                    .hint_text("0 … 20m"),
                            )
                            .changed();
                        ui.end_row();
                    }
                });
            let t = Tokens::get(ui.ctx());
            if let Some(error) = &draft.validation_error {
                ui.label(egui::RichText::new(error).color(t.color.err));
            } else if draft.dirty {
                ui.label(
                    egui::RichText::new(
                        "Pending · Run regression validates and commits this contract",
                    )
                    .color(t.color.warn),
                );
            } else {
                ui.label(egui::RichText::new("Persisted project contract").color(t.color.ok));
            }
        }
    } else {
        let t = Tokens::get(ui.ctx());
        let available_width = ui.available_width();
        egui::Frame::new()
            .fill(t.color.bg_panel)
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                ui.set_min_width((available_width - 20.0).max(1.0));
                status_dot(ui, t.color.warn, "No governed baseline selected");
                ui.label(
                    "Retain two immutable runs, then select the baseline used for exact comparison.",
                );
            });
    }
}

pub(super) fn regression_waveform_chart(
    ui: &mut Ui,
    waveforms: &[RegressionWaveformPair<'_>],
    rules: &[crate::state::RegressionToleranceRule],
    selected: Option<&crate::state::RegressionTargetSelector>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let height = ui.available_height().max(210.0);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    let header = egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + 36.0));
    ui.painter().rect_filled(header, 0.0, t.color.bg_panel);
    ui.painter().hline(
        header.x_range(),
        header.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let header_left = egui::Rect::from_min_max(
        header.min,
        egui::pos2(header.left() + header.width() * 0.45, header.bottom()),
    );
    let header_right =
        egui::Rect::from_min_max(egui::pos2(header_left.right(), header.top()), header.max);
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let title = elide_table_text(
        ui,
        "Waveform comparison",
        &title_font,
        (header_left.width() - 20.0).max(0.0),
    );
    ui.painter().with_clip_rect(header_left).text(
        header_left.left_center() + egui::vec2(10.0, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text,
    );
    let pair = selected
        .and_then(|selected| waveforms.iter().find(|pair| pair.target == *selected))
        .or_else(|| waveforms.first());
    let Some(pair) = pair else {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                "Waveform comparison: no source-aligned retained waveform pair",
            )
        });
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 12.0),
            egui::Align2::CENTER_CENTER,
            "No source-aligned waveform pair",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    };
    let baseline = pair.baseline;
    let current = pair.current;
    let rule = regression_rule(rules, &pair.target);
    let verdict = evaluate_regression_waveform(pair, rule);
    let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let detail = elide_table_text(
        ui,
        &format!(
            "{} · {}",
            current.name,
            match &verdict {
                RegressionVerdict::Pass { .. } => "pass",
                RegressionVerdict::Fail { .. } => "fail",
                RegressionVerdict::NotConfigured => "tolerance not configured",
                RegressionVerdict::NotEvaluated(_) => "evaluation blocked",
            }
        ),
        &detail_font,
        (header_right.width() - 20.0).max(0.0),
    );
    ui.painter().with_clip_rect(header_right).text(
        header_right.right_center() - egui::vec2(10.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        detail,
        detail_font,
        t.color.text_faint,
    );
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
    let Some(y_min) = finite_values.iter().copied().min_by(f64::total_cmp) else {
        return;
    };
    let Some(y_max) = finite_values.iter().copied().max_by(f64::total_cmp) else {
        return;
    };
    let finite_x = baseline
        .x
        .iter()
        .chain(current.x.iter())
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    let Some(x_min) = finite_x.iter().copied().min_by(f64::total_cmp) else {
        return;
    };
    let Some(x_max) = finite_x.iter().copied().max_by(f64::total_cmp) else {
        return;
    };
    let plot = egui::Rect::from_min_max(
        header.left_bottom() + egui::vec2(10.0, 10.0),
        rect.right_bottom() - egui::vec2(10.0, 18.0),
    );
    for step in 0..=4 {
        let y = egui::lerp(plot.bottom()..=plot.top(), step as f32 / 4.0);
        ui.painter().hline(
            plot.x_range(),
            y,
            egui::Stroke::new(1.0, t.color.canvas_grid.gamma_multiply(0.55)),
        );
    }
    let points = |x_values: &[f64], y_values: &[f64]| {
        x_values
            .iter()
            .zip(y_values)
            .filter_map(|(x_value, y_value)| {
                if !x_value.is_finite() || !y_value.is_finite() {
                    return None;
                }
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
                Some(egui::pos2(
                    egui::lerp(plot.left()..=plot.right(), x_fraction),
                    egui::lerp(plot.bottom()..=plot.top(), y_fraction),
                ))
            })
            .collect::<Vec<_>>()
    };
    ui.painter().add(egui::Shape::line(
        points(baseline.x.as_slice(), baseline.y.as_slice()),
        egui::Stroke::new(1.5, t.color.text_faint),
    ));
    if let Some(rule) = rule {
        let lower = baseline
            .y
            .iter()
            .map(|value| *value - permitted_delta(*value, rule))
            .collect::<Vec<_>>();
        let upper = baseline
            .y
            .iter()
            .map(|value| *value + permitted_delta(*value, rule))
            .collect::<Vec<_>>();
        let envelope_stroke = egui::Stroke::new(1.0, t.color.ok.gamma_multiply(0.65));
        ui.painter().add(egui::Shape::line(
            points(baseline.x.as_slice(), &lower),
            envelope_stroke,
        ));
        ui.painter().add(egui::Shape::line(
            points(baseline.x.as_slice(), &upper),
            envelope_stroke,
        ));
    }
    ui.painter().add(egui::Shape::line(
        points(current.x.as_slice(), current.y.as_slice()),
        egui::Stroke::new(1.8, t.color.accent),
    ));
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "Regression waveform {} with {} candidate samples; baseline, candidate, and configured tolerance envelope shown with verdict {:?}",
                current.name,
                current.y.len(),
                verdict
            ),
        )
    });
}
