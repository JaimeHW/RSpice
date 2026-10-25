//! The Verify workspace's navigator: one row per verification flow, and the
//! specification coverage under them.

use egui::Ui;

use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::RSpiceApp;

use super::super::super::design_system::{property_row, section_header};
use super::super::super::state::VerificationPage;
use super::flow_row;

/// Whether a navigator `available_height` tall keeps its scroll bar showing:
/// the verification flows and the coverage history under them need more.
pub(in crate::workbench) fn verification_navigator_requires_scroll(available_height: f32) -> bool {
    const HISTORY_COVERAGE_HEIGHT: f32 = 126.0;
    let flow_height = VerificationPage::NAVIGATION.len() as f32 * super::FLOW_ROW_HEIGHT;
    available_height < flow_height + HISTORY_COVERAGE_HEIGHT
}

pub(in crate::workbench) fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_filter()
        .trim()
        .to_ascii_lowercase();
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
    for page in VerificationPage::NAVIGATION {
        let flow = verification_flow_presentation(app, page);
        let label = &flow.label;
        let detail = &flow.detail;
        let status = &flow.status;
        if !query.is_empty()
            && !format!("{label} {detail} {status}")
                .to_ascii_lowercase()
                .contains(&query)
        {
            continue;
        }
        if flow_row(ui, &flow, app.state.simulate.verification_page == page) {
            app.state.simulate.verification_page = page;
        }
    }
    // The one coverage number a designer acts on, and what is missing from
    // it. Full coverage is the normal state and says nothing.
    section_header(ui, "Specifications", None);
    let coverage = verification_coverage(app);
    if coverage.total > 0 {
        property_row(
            ui,
            "Passing",
            &format!("{} / {}", coverage.passed, coverage.total),
        );
    }
    if !coverage.status.is_empty() {
        rspice_ui_kit::panels::inspector::muted_inspector_copy(ui, &coverage.status);
    }
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

/// One verification flow's row: what it is, what it holds, and its state,
/// stated once. A problem leads the row with its mark; otherwise the row leads
/// with the flow's neutral page glyph.
#[derive(Clone)]
pub(super) struct VerificationFlowPresentation {
    pub(super) label: String,
    pub(super) detail: String,
    pub(super) status: String,
    pub(super) problem: Option<Problem>,
    pub(super) glyph: &'static str,
}

pub(super) const fn verification_flow_label(page: VerificationPage) -> &'static str {
    match page {
        VerificationPage::Yield => "PVT & Monte Carlo",
        VerificationPage::Corners => "Process corners",
        VerificationPage::Tuning => "Tuning",
        VerificationPage::Optimization => "Optimization",
        VerificationPage::DeviceSafety => "Electrical SOA",
        VerificationPage::Regression => "Regression",
    }
}

/// The neutral glyph a flow row leads with when it reports no problem.
const fn verification_flow_glyph(page: VerificationPage) -> &'static str {
    match page {
        VerificationPage::Yield => "M",
        VerificationPage::Corners => "C",
        VerificationPage::Tuning => "T",
        VerificationPage::Optimization => "O",
        VerificationPage::DeviceSafety => "S",
        VerificationPage::Regression => "R",
    }
}

pub(super) fn verification_flow_presentation(
    app: &RSpiceApp,
    page: VerificationPage,
) -> VerificationFlowPresentation {
    let active_run = app.state.model.simulation.active_run();
    match page {
        VerificationPage::Yield => {
            let evidence = active_run.and_then(|run| {
                app.state
                    .model
                    .simulation
                    .retained
                    .yield_provenance()
                    .filter(|provenance| {
                        provenance.source_run_id == run.run_id
                            && provenance.source_dataset_id == run.dataset_id
                    })
                    .map(|provenance| (run, provenance))
            });
            let results = evidence
                .and_then(|(run, _)| {
                    app.state
                        .model
                        .simulation
                        .retained
                        .yield_results_for_dataset(run.dataset_id)
                })
                .unwrap_or(&[]);
            let worst = results.iter().min_by(|left, right| {
                left.yield_percent
                    .partial_cmp(&right.yield_percent)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            VerificationFlowPresentation {
                label: evidence.map_or_else(
                    || verification_flow_label(page).to_owned(),
                    |(run, _)| format!("PVT & Monte Carlo · Run {}", run.id),
                ),
                // The seed and the sampling are the inspector's; the row says
                // how much of the run completed.
                detail: evidence.map_or_else(String::new, |(_, provenance)| {
                    format!(
                        "{} / {} samples",
                        provenance.runs_completed, provenance.runs_requested
                    )
                }),
                status: worst.map_or_else(
                    || "not run".to_owned(),
                    |result| format!("{:.1}% yield", result.yield_percent),
                ),
                problem: worst
                    .is_some_and(|result| result.fail_count > 0)
                    .then_some(Problem::Warning),
                glyph: verification_flow_glyph(page),
            }
        }
        VerificationPage::Corners => {
            let result = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Corner));
            let point_count = result
                .and_then(|analysis| analysis.waveforms.first())
                .map_or(0, |waveform| waveform.x.len());
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: String::new(),
                status: if point_count == 0 {
                    "not run".to_owned()
                } else if result.is_some_and(|analysis| analysis.success) {
                    format!("{point_count} corners")
                } else {
                    "failed / incomplete".to_owned()
                },
                problem: result
                    .is_some_and(|analysis| !analysis.success)
                    .then_some(Problem::Warning),
                glyph: verification_flow_glyph(page),
            }
        }
        VerificationPage::Tuning => {
            let variable_count = app
                .state
                .model
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| app.state.model.workspace.content.plan_data(plan.id()))
                .map_or(0, |payload| payload.design_variables.len());
            let dirty_count = app
                .state
                .simulate
                .verification
                .tuning_variables
                .iter()
                .filter(|draft| draft.is_dirty())
                .count();
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: match variable_count {
                    0 => "No variables".to_owned(),
                    1 => "1 variable".to_owned(),
                    count => format!("{count} variables"),
                },
                // Nothing uncommitted is the normal state, and says nothing.
                status: if dirty_count == 0 {
                    String::new()
                } else {
                    format!("{dirty_count} uncommitted")
                },
                problem: None,
                glyph: verification_flow_glyph(page),
            }
        }
        VerificationPage::Optimization => {
            let result = active_run
                .and_then(|run| verified_analysis(run, crate::state::AnalysisType::Optimization));
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: String::new(),
                status: if result.is_some() {
                    "done".to_owned()
                } else {
                    "not run".to_owned()
                },
                problem: None,
                glyph: verification_flow_glyph(page),
            }
        }
        VerificationPage::DeviceSafety => {
            let soa_evidence =
                active_run.and_then(|run| verified_analysis(run, crate::state::AnalysisType::Soa));
            let has_evidence = soa_evidence.is_some();
            // A retained verdict is stated as the instance inspector states
            // it; only evidence without one is said to lack it.
            if let Some(verdict) =
                soa_evidence.and_then(super::super::inspector::analysis_soa_display)
            {
                return VerificationFlowPresentation {
                    label: verification_flow_label(page).to_owned(),
                    detail: "Worst device stress against its SOA limits".to_owned(),
                    status: verdict.label,
                    problem: verdict.problem,
                    glyph: verification_flow_glyph(page),
                };
            }
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: if has_evidence {
                    "The SOA check ran, but its results are unavailable".to_owned()
                } else {
                    "No SOA check in this run".to_owned()
                },
                status: if has_evidence {
                    "verdict unavailable".to_owned()
                } else {
                    "not run".to_owned()
                },
                problem: has_evidence.then_some(Problem::Warning),
                glyph: verification_flow_glyph(page),
            }
        }
        VerificationPage::Regression => {
            let retained_runs = app
                .state
                .model
                .simulation
                .retained
                .runs
                .iter()
                .filter(|run| {
                    run.analyses
                        .iter()
                        .any(|analysis| analysis.success && analysis.provenance.is_some())
                })
                .count();
            let ready = retained_runs >= 2;
            VerificationFlowPresentation {
                label: verification_flow_label(page).to_owned(),
                detail: String::new(),
                status: if ready {
                    format!("{retained_runs} runs")
                } else {
                    "needs 2 runs".to_owned()
                },
                problem: None,
                glyph: verification_flow_glyph(page),
            }
        }
    }
}

fn verified_analysis(
    run: &crate::state::SimulationRun,
    analysis_type: crate::state::AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    run.analyses.iter().rev().find(|analysis| {
        analysis.analysis_type == analysis_type && analysis.success && analysis.provenance.is_some()
    })
}

pub(super) struct VerificationCoverage {
    pub(super) total: usize,
    pub(super) passed: usize,
    /// What is missing from the coverage, or nothing when every specification
    /// was measured.
    pub(super) status: String,
}

pub(super) fn verification_coverage(app: &RSpiceApp) -> VerificationCoverage {
    let run = app.state.model.simulation.active_run();
    let total = app.state.model.workspace.content.specs.len();
    let values = app
        .state
        .model
        .workspace
        .content
        .specs
        .iter()
        .map(|spec| {
            run.and_then(|run| {
                run.analyses.iter().find_map(|analysis| {
                    if !analysis.success || analysis.provenance.is_none() {
                        return None;
                    }
                    analysis.measurements.iter().find_map(|measurement| {
                        if measurement.name.eq_ignore_ascii_case(&spec.measurement) {
                            // A measurement that missed its own GOAL/TOL or
                            // FAILVALUE contract keeps its value: it was
                            // executed, and it cannot pass.
                            measurement
                                .value_in_unit(&spec.unit)
                                .ok()
                                .flatten()
                                .filter(|value| value.is_finite())
                                .map(|value| {
                                    (value, measurement.passed && measurement.error.is_none())
                                })
                        } else {
                            None
                        }
                    })
                })
            })
        })
        .collect::<Vec<_>>();
    let executed = values.iter().filter(|value| value.is_some()).count();
    let passed = app
        .state
        .model
        .workspace
        .content
        .specs
        .iter()
        .zip(&values)
        .filter(|(spec, value)| {
            value.is_some_and(|(value, contract_held)| contract_held && spec.passes(value))
        })
        .count();
    let gaps = total.saturating_sub(executed);
    VerificationCoverage {
        total,
        passed,
        status: if total == 0 {
            "No specs".to_owned()
        } else if gaps == 0 {
            String::new()
        } else {
            format!(
                "{gaps} spec{} not measured",
                if gaps == 1 { "" } else { "s" }
            )
        },
    }
}
