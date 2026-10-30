//! Clearing the design's execution context reaches past the results: the
//! code workspace's runtime and the design check's findings go, a notice
//! loses an offer that names a retired run, and the Console keeps its
//! history.

use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;

#[test]
fn clearing_design_execution_context_clears_source_runtime_and_keeps_the_console() {
    let mut state = AppState::default();
    state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
    state.code.code_workspace.automation.debug.watches.push(
        crate::workbench::documents::code_workspace::AutomationWatch {
            expression: "project.old_value".to_owned(),
            value: "17".to_owned(),
            error: None,
        },
    );
    state.code.netlist.diagnostics = std::sync::Arc::new(
        crate::state::netlist_diagnostics::NetlistDiagnosticCollection::try_new(
            vec![crate::state::netlist_diagnostics::Diagnostic::error(
                "old project diagnostic",
            )],
            "",
        )
        .unwrap(),
    );
    state
        .model
        .log_buffer
        .warning(LogSource::Run, "old project warning");
    state.clear_design_execution_context();

    assert_eq!(
        state.code.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    );
    assert!(
        state
            .code
            .code_workspace
            .automation
            .debug
            .watches
            .is_empty()
    );
    assert_eq!(state.code.netlist.diagnostics.summary().total(), 0);
    assert!(
        state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message == "old project warning"),
        "the Console is the session's record and keeps its history"
    );
}

fn seed_blocking_drc_result(state: &mut AppState) {
    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::MissingGround,
        "missing ground",
        DrcLocation::Global,
    ));
    result.completed = true;
    state
        .publish_active_design_check_result(result)
        .expect("test DRC receipt must publish");
    state
        .model
        .log_buffer
        .log(LogSeverity::Error, LogSource::Studio, "old DRC anchor");
}

/// Run 3 announced in one design is not run 3 of the next: once run
/// history is replaced the notice keeps its words and loses the offer, and
/// following nothing cannot open the wrong run.
#[test]
fn replacing_the_design_retires_run_offers_and_keeps_the_notices() {
    use crate::ui::widgets::{NotificationAction, NotificationCategory, ToastKind};

    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state.workbench.toasts.notify_with_action(
        &ctx,
        NotificationCategory::Job,
        ToastKind::Success,
        "Run 3 complete",
        "2 analyses saved with this run.",
        NotificationAction::OpenRunInResults { run_sequence: 3 },
    );
    state.workbench.toasts.notify_with_action(
        &ctx,
        NotificationCategory::System,
        ToastKind::Error,
        "Export failed",
        "The folder is read-only.",
        NotificationAction::ShowInConsole { log_id: 7 },
    );

    state.clear_design_execution_context();

    let records = state.workbench.toasts.activity();
    assert_eq!(records.len(), 2, "what happened is still true");
    assert_eq!(records[1].title(), "Run 3 complete");
    assert_eq!(records[1].message(), "2 analyses saved with this run.");
    assert_eq!(
        records[1].action(),
        None,
        "the offer named a run the new history does not have"
    );
    assert_eq!(
        records[0].action(),
        Some(NotificationAction::ShowInConsole { log_id: 7 }),
        "the session log survives the switch, and so does the way to it"
    );
}

#[test]
fn clearing_design_execution_context_clears_stale_drc_results() {
    let mut state = AppState::default();
    seed_blocking_drc_result(&mut state);
    assert!(state.current_blocking_drc_result().is_some());
    let logged = state.model.log_buffer.entries().count();

    state.clear_design_execution_context();

    assert!(state.model.design_checks.drc_results.is_none());
    assert!(state.model.diagnostics.current_finding().is_none());
    assert!(state.current_blocking_drc_result().is_none());
    assert_eq!(
        state.model.log_buffer.entries().count(),
        logged,
        "the findings go; the Console's record of the check stays"
    );
}

#[test]
fn clearing_design_execution_context_clears_project_scoped_results_state() {
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, ExprSeries, ExprTrace, ResultViewer,
    };
    use rspice_results_ui::session::ExprEditor;

    let mut state = AppState::default();
    let analysis =
        crate::state::AnalysisResult::new(71, crate::state::AnalysisType::Transient, "Transient");
    let analysis_key = AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
    state.results.results_seen_version = 99;
    let results = &mut state.results.results;
    results.session.viewer = ResultViewer::Fft;
    results.session.phase_continuous = true;
    results.session.cursors.place(1.0);
    results.session.cursor_strip = Some(0);
    results.session.hidden_strips.insert(analysis_key);
    results.session.maximized_strip = Some(analysis_key);
    *results.session.plot_view_mut(ResultViewer::Waves, 0) =
        rspice_results_ui::presentation::PlotView {
            x: Some((0.0, 1.0)),
            y: Some((-1.0, 1.0)),
        };
    results.session.analysis_exprs.insert(
        analysis_key,
        vec![ExprTrace {
            text: "V(out)/V(in)".to_string(),
            complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
            visible: true,
        }],
    );
    results.session.expr_editor = Some(ExprEditor {
        analysis: analysis_key,
        text: "V(out)".to_string(),
        error: Some("stale".to_string()),
        want_focus: false,
    });
    results.analysis_expr_cache.insert(
        (analysis_key, "V(out)".to_string()),
        ExprSeries {
            version: 10,
            series: Err("stale".to_string()),
        },
    );
    results.session.rf_pin.insert(ResultViewer::Smith, (0, 1));
    results.session.op_filter = "M1".to_string();
    results.session.op_sort = Some(("gm".to_string(), true));
    results.session.spec_drafts = Some(Vec::new());
    state.results.specification_editor_route_pending = true;

    state.clear_design_execution_context();
    let results = &state.results.results;

    assert_eq!(state.results.results_seen_version, 0);
    assert_eq!(results.session.viewer, ResultViewer::Fft);
    assert!(results.session.phase_continuous);
    assert!(!results.session.cursors.any());
    assert_eq!(results.session.cursor_strip, None);
    assert!(results.session.hidden_strips.is_empty());
    assert_eq!(results.session.maximized_strip, None);
    assert!(results.session.views.is_empty());
    assert!(results.session.analysis_exprs.is_empty());
    assert!(results.session.expr_editor.is_none());
    assert!(results.analysis_expr_cache.is_empty());
    assert!(results.session.rf_pin.is_empty());
    assert!(results.session.op_filter.is_empty());
    assert_eq!(results.session.op_sort, None);
    assert!(results.session.spec_drafts.is_none());
    assert!(!state.results.specification_editor_route_pending);
}
