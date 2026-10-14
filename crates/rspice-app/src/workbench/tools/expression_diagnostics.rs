//! Expression and engineering-unit diagnostics.
//!
//! The inline editor tells you about the one expression you are typing. This
//! tool answers the other question — *are any of the expressions I have
//! saved broken?* — over every analysis of the active dataset at once, so a
//! stale expression on a sheet nobody has opened is still visible.
//!
//! Every row reports what the retained evaluation actually produced. Nothing
//! here re-derives a verdict the plot would disagree with: an expression is
//! valid exactly when its cached series evaluated, and the message shown on
//! failure is the evaluator's own.

use egui::{Context, RichText, Ui};

use crate::state::AnalysisResult;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Problem, paint_problem_mark, problem_line};
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, analysis_default_unit, browser_signal_unit,
};

const TOOL_WIDTH: f32 = 620.0;
const TOOL_DEFAULT_HEIGHT: f32 = 400.0;

/// One expression's standing, as the workspace would evaluate it now.
struct ExpressionRow {
    analysis: String,
    text: String,
    /// The unit the result reads in, inferred from the expression's outermost
    /// accessor and the analysis it is bound to.
    unit: &'static str,
    domain: &'static str,
    /// `None` when the expression evaluates; the evaluator's message when it
    /// does not.
    error: Option<String>,
    /// Whether the expression has been evaluated against the current data at
    /// all. An expression on a sheet never opened has no verdict yet, and
    /// claiming one would be an invention.
    evaluated: bool,
}

/// Show the tool when it is open.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.dialogs.expression_diagnostics_dialog {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open = true;
    egui::Window::new("Expression and unit diagnostics")
        .id(egui::Id::new("workbench.expression-diagnostics"))
        .open(&mut open)
        .default_width(TOOL_WIDTH)
        .default_height(TOOL_DEFAULT_HEIGHT)
        .min_width(TOOL_WIDTH * 0.6)
        .resizable(true)
        .collapsible(false)
        .frame(
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius_lg)
                .inner_margin(egui::Margin::symmetric(14, 12))
                .shadow(t.shadow()),
        )
        .show(ctx, |ui| body(ui, app));
    if !open {
        app.state.dialogs.expression_diagnostics_dialog = false;
    }
}

fn body(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.label(
        RichText::new("RESULTS · TYPED EXPRESSIONS · NO SILENT COERCION")
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(c.text_dim),
    );
    ui.add_space(8.0);

    let rows = collect_rows(app);
    if rows.is_empty() {
        ui.label(
            RichText::new(
                "No saved expressions in the active dataset. Add one from a waveform sheet's expression field.",
            )
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(c.text_faint),
        );
        return;
    }

    // A count, and a failure mark only while something blocks. Expressions
    // that evaluate are the ordinary case and say nothing more.
    let blocking = rows.iter().filter(|row| row.error.is_some()).count();
    if let Some(summary) = blocking_summary(rows.len(), blocking) {
        problem_line(ui, Problem::Failure, &summary);
    } else {
        ui.label(
            RichText::new(format!("{} expression(s)", rows.len()))
                .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                .color(c.text_dim),
        );
    }
    ui.add_space(8.0);

    egui::ScrollArea::vertical()
        .id_salt("workbench.expression-diagnostics.rows")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("workbench.expression-diagnostics.grid")
                .num_columns(5)
                .striped(true)
                .spacing(egui::vec2(12.0, 5.0))
                .show(ui, |ui| {
                    for heading in ["Expression", "Analysis", "Domain", "Unit", "Status"] {
                        ui.label(
                            RichText::new(heading)
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(c.text_faint),
                        );
                    }
                    ui.end_row();
                    for row in &rows {
                        ui.label(
                            RichText::new(&row.text)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.text),
                        );
                        ui.label(
                            RichText::new(&row.analysis)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(c.text_dim),
                        );
                        ui.label(
                            RichText::new(row.domain)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(c.text_dim),
                        );
                        ui.label(
                            RichText::new(row.unit)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.text_dim),
                        );
                        match (&row.error, row.evaluated) {
                            (Some(error), _) => {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 6.0;
                                    let (mark, _) = ui.allocate_exact_size(
                                        egui::vec2(11.0, 11.0),
                                        egui::Sense::hover(),
                                    );
                                    paint_problem_mark(ui.painter(), mark, Problem::Failure);
                                    ui.label(
                                        RichText::new(error)
                                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                            .color(c.text),
                                    )
                                    .on_hover_text(Problem::Failure.announce(error));
                                });
                            }
                            // An expression that evaluates leaves its status
                            // blank: success is the absence of a problem.
                            (None, true) => {
                                ui.label("");
                            }
                            (None, false) => {
                                ui.label(
                                    RichText::new("not evaluated yet")
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(c.text_faint),
                                )
                                .on_hover_text(
                                    "Open the sheet that owns this expression to evaluate it against the retained samples.",
                                );
                            }
                        }
                        ui.end_row();
                    }
                });
        });
}

/// Every saved expression of the active dataset, with the verdict the
/// workspace already holds for it.
fn collect_rows(app: &RSpiceApp) -> Vec<ExpressionRow> {
    let state = &app.state;
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for analysis in &run.analyses {
        let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        let Some(expressions) = state.ui.results.session.analysis_exprs.get(&key) else {
            continue;
        };
        for expression in expressions {
            let cached = state
                .ui
                .results
                .analysis_expr_cache
                .get(&(key, expression.text.clone()));
            rows.push(ExpressionRow {
                analysis: analysis.label.clone(),
                // An expression has no retained waveform to state a unit, so
                // its text and the analysis are all there is to read.
                unit: browser_signal_unit(
                    &expression.text,
                    None,
                    analysis_default_unit(analysis.analysis_type),
                ),
                domain: domain_label(analysis),
                error: cached.and_then(|cached| cached.series.as_ref().err().cloned()),
                evaluated: cached.is_some(),
                text: expression.text.clone(),
            });
        }
    }
    rows
}

/// The line a blocked set of expressions states beside its failure mark, or
/// nothing while every expression evaluates.
fn blocking_summary(expressions: usize, blocking: usize) -> Option<String> {
    (blocking > 0).then(|| format!("{expressions} expression(s) · {blocking} blocking"))
}

fn domain_label(analysis: &AnalysisResult) -> &'static str {
    use crate::state::AnalysisType;
    match analysis.analysis_type {
        AnalysisType::Transient => "TRAN · time",
        AnalysisType::Ac => "AC · frequency",
        AnalysisType::Noise => "NOISE · frequency",
        // A periodic noise spectrum stands on the offset from its carrier. It
        // shared the arm above, which states the absolute frequency a .NOISE
        // run sweeps about a DC operating point -- a different quantity.
        AnalysisType::Pnoise => "PNOISE · offset frequency",
        AnalysisType::DcSweep => "DC · swept source",
        AnalysisType::DcOp => "OP · scalar",
        _ => "retained sweep",
    }
}

#[cfg(test)]
mod tests {
    use super::{blocking_summary, domain_label};
    use crate::state::{AnalysisResult, AnalysisType};

    /// Only a blocked set earns the failure line. It used to read "all
    /// evaluate" in green when nothing was wrong.
    #[test]
    fn only_a_blocking_expression_earns_the_failure_line() {
        assert_eq!(blocking_summary(3, 0), None);
        assert_eq!(
            blocking_summary(3, 1).as_deref(),
            Some("3 expression(s) · 1 blocking")
        );
    }

    /// The diagnostics row states the same quantity the rest of the product
    /// does: a `.PNOISE` spectrum stands on an offset from its carrier, and a
    /// `.NOISE` sweep on an absolute frequency about a DC operating point.
    /// These two shared one arm, and it named the second.
    #[test]
    fn periodic_noise_states_the_offset_it_sweeps_and_plain_noise_its_own_frequency() {
        for (analysis, domain) in [
            (AnalysisType::Pnoise, "PNOISE · offset frequency"),
            (AnalysisType::Noise, "NOISE · frequency"),
        ] {
            assert_eq!(
                domain_label(&AnalysisResult::new(1, analysis, "expression domain")),
                domain,
                "{analysis:?}"
            );
        }
    }
}
