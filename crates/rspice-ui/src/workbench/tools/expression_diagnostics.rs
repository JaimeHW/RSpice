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

    let blocking = rows.iter().filter(|row| row.error.is_some()).count();
    ui.label(
        RichText::new(if blocking == 0 {
            format!("{} expression(s) · all evaluate", rows.len())
        } else {
            format!("{} expression(s) · {blocking} blocking", rows.len())
        })
        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
        .color(if blocking == 0 { c.ok } else { c.err }),
    );
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
                                ui.label(
                                    RichText::new(error)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(c.err),
                                )
                                .on_hover_text(error);
                            }
                            (None, true) => {
                                ui.label(
                                    RichText::new("valid")
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(c.ok),
                                );
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
        let Some(expressions) = state.ui.results.analysis_exprs.get(&key) else {
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
                unit: browser_signal_unit(
                    &expression.text,
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

fn domain_label(analysis: &AnalysisResult) -> &'static str {
    use crate::state::AnalysisType;
    match analysis.analysis_type {
        AnalysisType::Transient => "TRAN · time",
        AnalysisType::Ac => "AC · frequency",
        AnalysisType::Noise | AnalysisType::Pnoise => "NOISE · frequency",
        AnalysisType::DcSweep => "DC · swept source",
        AnalysisType::DcOp => "OP · scalar",
        _ => "retained sweep",
    }
}
