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
use crate::workbench::RSpiceApp;

/// Default geometry of the mockup's modeless lower-right calculator.
const TOOL_WIDTH: f32 = 372.0;
const TOOL_DEFAULT_HEIGHT: f32 = 390.0;
const TOOL_MIN_HEIGHT: f32 = 240.0;
/// Inset from the workspace edges on first open.
const TOOL_MARGIN: f32 = 24.0;

/// Show the calculator when it is open. Modeless: the workspace behind it
/// stays live, so cursors can be placed and traces read while an expression
/// is being composed.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.dialogs.waveform_calculator_dialog {
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
        .show(ctx, |ui| body(ui, app));
    if !open {
        app.state.dialogs.waveform_calculator_dialog = false;
    }
}

fn body(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 0.0;

    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let (panel, simulation) = (&mut app.state.calculator_panel, &app.state.simulation);
            panel.show_body(ui, simulation);
        });

    // The context line names the dataset the expression evaluates against,
    // so a number can never be read against the wrong run.
    let hint = app
        .state
        .calculator_panel
        .context_hint(&app.state.simulation);
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
                        .enabled(!app.state.calculator_panel.expression.trim().is_empty())
                        .show(ui)
                        .on_disabled_hover_text("Enter an expression first")
                        .clicked()
                    {
                        plot_expression(ui.ctx(), app);
                    }
                    if Button::new("Evaluate").show(ui).clicked() {
                        let (panel, simulation) =
                            (&mut app.state.calculator_panel, &app.state.simulation);
                        panel.evaluate(simulation);
                    }
                    if Button::new("Clear").ghost().show(ui).clicked() {
                        app.state.calculator_panel.clear();
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
fn plot_expression(ctx: &Context, app: &mut RSpiceApp) {
    let expression = app.state.calculator_panel.expression.trim().to_owned();
    if expression.is_empty() {
        return;
    }
    let analysis = app.state.simulation.active_analysis_idx.unwrap_or(0);
    let traces = app.state.ui.results.exprs.entry(analysis).or_default();
    if !traces.iter().any(|trace| trace.text == expression) {
        traces.push(crate::workbench::documents::result_document::ExprTrace {
            text: expression.clone(),
            visible: true,
        });
    }
    app.state.ui.results.viewer = crate::workbench::documents::result_document::ResultViewer::Waves;
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    app.state
        .ui
        .toasts
        .info(ctx, format!("Plotted {expression}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plotting_keeps_the_tool_open_and_never_duplicates_a_trace() {
        let mut app = crate::workbench::RSpiceApp::test_instance();
        let ctx = egui::Context::default();
        app.state.dialogs.waveform_calculator_dialog = true;
        app.state.calculator_panel.expression = "V(out)/V(in)".to_owned();

        plot_expression(&ctx, &mut app);
        plot_expression(&ctx, &mut app);

        let traces = app.state.ui.results.exprs.get(&0).expect("trace recorded");
        assert_eq!(traces.len(), 1, "the same expression plotted twice");
        assert_eq!(traces[0].text, "V(out)/V(in)");
        assert!(
            app.state.dialogs.waveform_calculator_dialog,
            "plotting must not close a modeless tool"
        );
    }

    #[test]
    fn an_empty_expression_plots_nothing() {
        let mut app = crate::workbench::RSpiceApp::test_instance();
        let ctx = egui::Context::default();
        app.state.calculator_panel.expression = "   ".to_owned();

        plot_expression(&ctx, &mut app);

        assert!(app.state.ui.results.exprs.is_empty());
    }
}
