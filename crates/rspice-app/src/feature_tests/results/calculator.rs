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

/// The tool opens whole inside the workspace, in its lower right, between
/// the toolbar and the status bar. It was placed by a height it did not
/// have, so its footer ran off the bottom of a 1440 x 900 window, and it was
/// 372 pt wide around a 244 pt rail, so its function rows drew over one
/// another.
#[test]
fn the_tool_opens_whole_in_the_lower_right_of_the_workspace() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.results.waveform_calculator_dialog = true;
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 900.0));
    for _ in 0..3 {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |ui| {
                app.prepare_frame(ui.ctx());
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            },
        );
    }
    let tool = ctx
        .memory(|memory| memory.area_rect(egui::Id::new("workbench.calculator-tool")))
        .expect("the tool is on screen");
    let layout = crate::workbench::layout::LayoutSpec::resolve_for_shell(
        screen.width(),
        screen.height(),
        app.state.workbench.coarse_pointer,
        true,
        false,
        app.state.platform.project_lifecycle.is_open(),
        true,
        &app.state.workbench,
    );
    let workspace = egui::Rect::from_min_max(
        egui::pos2(0.0, layout.title_bar_height + layout.toolbar_height),
        egui::pos2(screen.right(), screen.bottom() - layout.status_bar_height),
    );
    assert!(
        workspace.contains_rect(tool),
        "the tool {tool:?} lies inside the workspace {workspace:?}"
    );
    assert!(
        workspace.right() - tool.right() <= 13.0 && workspace.bottom() - tool.bottom() <= 13.0,
        "the tool {tool:?} opens in the lower right of {workspace:?}"
    );
    assert!(
        tool.width() > 372.0,
        "the tool {tool:?} is wider than the rail and its function rows"
    );
}
