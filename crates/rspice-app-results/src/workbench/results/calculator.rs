//! The waveform calculator as a modeless floating tool.
//!
//! A calculator over simulation results is something you keep open *while*
//! reading the plot: you place a cursor, read a value, refine the
//! expression. A modal dialog forbids exactly that, so the calculator is a
//! floating tool that never blocks the workspace beneath it.

mod panel;

use egui::{Context, Ui};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, IconButton};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::results::ResultsFeature;
use crate::workbench::slots::ResultsState;
use crate::workbench::state::WorkbenchSession;

const TITLE: &str = "Waveform calculator";
/// The kit dialog's header, body and footer measures, so the tool reads as
/// one of the dialogs while staying modeless.
const HEADER_HEIGHT: f32 = 48.0;
const EDGE: i8 = 12;
const FOOTER_VERTICAL: i8 = 10;
/// Inset from the workspace's lower-right corner on first open.
const TOOL_MARGIN: f32 = 12.0;

/// Show the calculator when it is open. Modeless: the workspace behind it
/// stays live, so cursors can be placed and traces read while an expression
/// is being composed.
///
/// It wears the kit dialog's chrome (title left, close target right, a
/// footer holding the verbs) and opens in the lower right of the workspace,
/// held between the toolbar and the status bar. It was an egui window with
/// egui's centred title, 372 pt wide for a 244 pt signal rail, so the
/// function list drew over itself, and it was placed by a height it did not
/// have, so its footer ran off the bottom of a 1440 x 900 window.
pub(super) fn show(cx: &mut Cx<'_, ResultsFeature>, ctx: &Context) {
    if !cx.state.waveform_calculator_dialog {
        return;
    }
    let t = Tokens::get(ctx);
    let workspace = workspace_rect(ctx, cx.wb, cx.platform.project_lifecycle.is_open());
    let body_width = panel::body_width(ctx);
    let id = egui::Id::new("workbench.calculator-tool");
    // Esc closes the tool while the reader is in it, as it closes a dialog.
    // Read before the frame's widgets run: the expression field gives up
    // focus to the same key.
    let escaped = ctx.input(|input| input.key_pressed(egui::Key::Escape))
        && ctx
            .memory(|memory| memory.focused())
            .and_then(|focused| ctx.read_response(focused))
            .is_some_and(|focused| focused.layer_id == egui::LayerId::new(egui::Order::Middle, id));

    let mut closed = false;
    egui::Window::new(TITLE)
        .id(id)
        .title_bar(false)
        .pivot(egui::Align2::RIGHT_BOTTOM)
        .default_pos(workspace.right_bottom() - egui::vec2(TOOL_MARGIN, TOOL_MARGIN))
        .constrain_to(workspace)
        .resizable(false)
        .collapsible(false)
        .frame(
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius_lg)
                .inner_margin(egui::Margin::ZERO)
                .shadow(t.shadow()),
        )
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            closed = header(ui, &t);
            body(ui, cx, body_width);
        });
    if closed || escaped {
        cx.state.waveform_calculator_dialog = false;
    }
}

/// The part of the window the workspace owns: under the title bar and the
/// toolbar, over the status bar. The tool never covers the menus.
fn workspace_rect(ctx: &Context, wb: &WorkbenchSession, project_open: bool) -> egui::Rect {
    let screen = ctx.content_rect();
    // The bar heights read here depend on the viewport and the pointer, not
    // on the strips or docks the remaining arguments describe.
    let layout = crate::workbench::layout::LayoutSpec::resolve_for_shell(
        screen.width(),
        screen.height(),
        wb.coarse_pointer,
        true,
        false,
        project_open,
        true,
        wb,
    );
    egui::Rect::from_min_max(
        egui::pos2(
            screen.left(),
            screen.top() + layout.title_bar_height + layout.toolbar_height,
        ),
        egui::pos2(screen.right(), screen.bottom() - layout.status_bar_height),
    )
}

/// The kit dialog's header: the title on the body's left edge, the close
/// target on its right edge, a hairline under both. Returns whether the
/// close target was clicked.
fn header(ui: &mut Ui, t: &Tokens) -> bool {
    let mut closed = false;
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(EDGE, 0))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), HEADER_HEIGHT),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_min_height(HEADER_HEIGHT);
                    ui.label(
                        egui::RichText::new(TITLE)
                            .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        closed = IconButton::new(Icon::Close)
                            .size(28.0, 27.0)
                            .tooltip("Close (Esc)")
                            .show(ui)
                            .clicked();
                    });
                },
            );
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, t.color.border),
    );
    closed
}

fn body(ui: &mut Ui, cx: &mut Cx<'_, ResultsFeature>, body_width: f32) {
    let t = Tokens::get(ui.ctx());

    egui::Frame::new()
        .inner_margin(egui::Margin::same(EDGE))
        .show(ui, |ui| {
            ui.set_width(body_width);
            let (panel, simulation) = (&mut cx.state.calculator_panel, &cx.model.simulation);
            panel.show_body(ui, simulation);
        });

    // The footer names the dataset the expression evaluates against, so a
    // number can never be read against the wrong run, and says why Plot is
    // unavailable while it is.
    let expression_empty = cx.state.calculator_panel.expression.trim().is_empty();
    let hint = if expression_empty {
        "Enter an expression to evaluate or plot".to_owned()
    } else {
        cx.state.calculator_panel.context_hint(&cx.model.simulation)
    };
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, t.color.border),
    );
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(EDGE, FOOTER_VERTICAL))
        .show(ui, |ui| {
            ui.set_width(body_width);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(
                    egui::RichText::new(hint)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if Button::new("Plot")
                        .accent()
                        .enabled(!expression_empty)
                        .show(ui)
                        .clicked()
                    {
                        plot_expression(cx.state, cx.model, cx.wb);
                    }
                    if Button::new("Evaluate")
                        .enabled(!expression_empty)
                        .show(ui)
                        .clicked()
                    {
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
pub fn plot_expression(results: &mut ResultsState, model: &mut Model, wb: &mut WorkbenchSession) {
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
