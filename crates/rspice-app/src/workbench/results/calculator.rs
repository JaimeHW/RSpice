//! The waveform calculator as a modeless floating tool.
//!
//! A calculator over simulation results is something you keep open *while*
//! reading the plot: you place a cursor, read a value, refine the
//! expression. A modal dialog forbids exactly that, so the calculator is a
//! floating tool that never blocks the workspace beneath it.

mod panel;

use egui::{Context, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::results::ResultsFeature;
use crate::workbench::slots::ResultsState;
use crate::workbench::state::WorkbenchSession;

/// Default geometry of the mockup's modeless lower-right calculator.
const TOOL_WIDTH: f32 = 372.0;
const TOOL_DEFAULT_HEIGHT: f32 = 390.0;
const TOOL_MIN_HEIGHT: f32 = 240.0;
/// Inset from the workspace edges on first open.
const TOOL_MARGIN: f32 = 24.0;

/// Show the calculator when it is open. Modeless: the workspace behind it
/// stays live, so cursors can be placed and traces read while an expression
/// is being composed.
pub fn show(ctx: &Context, cx: &mut Cx<'_, ResultsFeature>) {
    if !cx.state.waveform_calculator_dialog {
        return;
    }
    let t = Tokens::get(ctx);
    let screen = ctx.content_rect();
    let default_pos = egui::pos2(
        (screen.right() - TOOL_WIDTH - TOOL_MARGIN).max(screen.left() + TOOL_MARGIN),
        (screen.bottom() - TOOL_DEFAULT_HEIGHT - TOOL_MARGIN).max(screen.top() + TOOL_MARGIN),
    );

    let mut open = true;
    egui::Window::new("Waveform calculator")
        .id(egui::Id::new("workbench.calculator-tool"))
        .open(&mut open)
        .default_pos(default_pos)
        .default_width(TOOL_WIDTH)
        .default_height(TOOL_DEFAULT_HEIGHT)
        .min_width(TOOL_WIDTH * 0.7)
        .min_height(TOOL_MIN_HEIGHT)
        .resizable(true)
        .collapsible(false)
        .frame(
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius_lg)
                .inner_margin(egui::Margin::ZERO)
                .shadow(t.shadow()),
        )
        .show(ctx, |ui| body(ui, cx));
    if !open {
        cx.state.waveform_calculator_dialog = false;
    }
}

fn body(ui: &mut Ui, cx: &mut Cx<'_, ResultsFeature>) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 0.0;

    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let (panel, simulation) = (&mut cx.state.calculator_panel, &cx.model.simulation);
            panel.show_body(ui, simulation);
        });

    // The context line names the dataset the expression evaluates against,
    // so a number can never be read against the wrong run.
    let hint = cx.state.calculator_panel.context_hint(&cx.model.simulation);
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, t.color.border),
    );
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                ui.label(
                    egui::RichText::new(hint)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if Button::new("Plot")
                        .accent()
                        .enabled(!cx.state.calculator_panel.expression.trim().is_empty())
                        .show(ui)
                        .on_disabled_hover_text("Enter an expression first")
                        .clicked()
                    {
                        plot_expression(cx.state, cx.model, cx.wb);
                    }
                    if Button::new("Evaluate").show(ui).clicked() {
                        let (panel, simulation) =
                            (&mut cx.state.calculator_panel, &cx.model.simulation);
                        panel.evaluate(simulation);
                    }
                    if Button::new("Clear").ghost().show(ui).clicked() {
                        cx.state.calculator_panel.clear();
                    }
                });
            });
        });
}

/// Hand the expression to the waves strips as an expression trace on the
/// active analysis and show it.
///
/// The tool stays open: plotting is a step in composing an expression, not
/// the end of the task.
fn plot_expression(results: &mut ResultsState, model: &mut Model, wb: &mut WorkbenchSession) {
    let expression = results.calculator_panel.expression.trim().to_owned();
    if expression.is_empty() {
        return;
    }
    let Some(analysis_index) = model.simulation.view.active_analysis_idx else {
        wb.prompt("Select a retained result analysis before plotting an expression.");
        return;
    };
    let Some(run) = model.simulation.active_run() else {
        return;
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let analysis_key = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
        run.dataset_id,
        analysis,
    );
    results.calculator_panel.evaluate(&model.simulation);
    if let Some(Err(error)) = results.calculator_panel.outcome.as_ref() {
        wb.prompt(format!("Expression was not plotted: {error}"));
        return;
    }
    let added =
        results
            .results
            .add_expression_trace(&model.simulation, analysis_key, expression.clone());
    let added = match added {
        Ok(added) => added,
        Err(error) => {
            wb.prompt(error);
            return;
        }
    };
    if added {
        model.workspace.content.visualization_documents_dirty = true;
    }
    results.results.session.viewer =
        crate::workbench::documents::result_document::ResultViewer::Waves;
    wb.activate(crate::workbench::state::Workspace::Results);
    // The strips are in front of the reader now; the status bar confirms it.
    wb.prompt(if added {
        format!("Plotted {expression}")
    } else {
        format!("{expression} is already plotted")
    });
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
