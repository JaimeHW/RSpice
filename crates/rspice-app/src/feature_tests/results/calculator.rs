//! The waveform calculator's Plot through a whole application.

use crate::workbench::results::calculator::*;

#[test]
fn plotting_keeps_the_tool_open_and_never_duplicates_a_trace() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    let analysis =
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![
                crate::state::RetainedTrace::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![1.0, 2.0],
                    "#00aaff",
                ),
                crate::state::RetainedTrace::new(
                    "V(in)",
                    vec![0.0, 1.0],
                    vec![1.0, 1.0],
                    "#ffaa00",
                ),
            ]);
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    app.state.results.waveform_calculator_dialog = true;
    app.state.results.calculator_panel.expression = "V(out)/V(in)".to_owned();

    plot_expression(
        &mut app.state.results,
        &mut app.state.model,
        &mut app.state.workbench,
    );
    assert_eq!(app.state.prompt_text(), Some("Plotted V(out)/V(in)"));
    plot_expression(
        &mut app.state.results,
        &mut app.state.model,
        &mut app.state.workbench,
    );
    app.state.drain_notices(&egui::Context::default());
    // The reader is looking at the strips: the answer is the status
    // bar's, and nothing is interrupted for it.
    assert_eq!(
        app.state.prompt_text(),
        Some("V(out)/V(in) is already plotted")
    );
    assert!(app.state.workbench.toasts.activity().is_empty());

    let traces = app
        .state
        .results
        .results
        .session
        .exprs
        .get(&0)
        .expect("trace recorded");
    assert_eq!(traces.len(), 1, "the same expression plotted twice");
    assert_eq!(traces[0].text, "V(out)/V(in)");
    assert!(
        app.state
            .model
            .workspace
            .content
            .visualization_documents_dirty
    );
    assert!(
        app.state.results.waveform_calculator_dialog,
        "plotting must not close a modeless tool"
    );
}

#[test]
fn an_empty_expression_plots_nothing() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.results.calculator_panel.expression = "   ".to_owned();

    plot_expression(
        &mut app.state.results,
        &mut app.state.model,
        &mut app.state.workbench,
    );

    assert!(app.state.results.results.session.exprs.is_empty());
}
