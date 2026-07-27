//! Waveform calculator — the egui implementation of
//! the product dialog pattern.
//!
//! Keyboard-first: there is no keypad — the two things a calculator over
//! simulation data actually needs are the active run's signals and the
//! function reference, both click-to-insert at the caret. The result (or
//! error) renders inline under the expression; the dialog footer's
//! "Plot result" hands the expression to the waves strips as an
//! expression trace.

use egui::Ui;

use crate::analysis::calculator::{CalcValue, SimulationContext, evaluator, parser};
use crate::state::SimulationState;
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    PANE_FOOTER_H, PANE_HEADER_H, PANE_RAIL_W, PaneSide, pane_footer, pane_header,
    pane_section_label, two_pane,
};

/// Height of the signal/function panes (the spec's `min-height: 286px`).
const PANE_HEIGHT: f32 = 286.0;

#[derive(Default, Clone)]
pub struct CalculatorPanel {
    /// Current expression text.
    pub expression: String,
    /// Successful expressions, most recent first.
    history: Vec<String>,
    /// Position while cycling history with ↑/↓ (None = live text).
    history_at: Option<usize>,
    /// Live text stashed while cycling history.
    stash: String,
    /// Last evaluation outcome.
    outcome: Option<Result<String, String>>,
    /// Selected function category.
    category: FunctionCategory,
    /// Signal list filter.
    signal_filter: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum FunctionCategory {
    #[default]
    Math,
    Signal,
    Measure,
}

/// One function-reference row: label, one-line hint, what to insert, and
/// how many characters the caret steps back into the inserted text.
struct FunctionEntry {
    label: &'static str,
    hint: &'static str,
    insert: &'static str,
    caret_back: usize,
}

const MATH_FUNCTIONS: &[FunctionEntry] = &[
    FunctionEntry {
        label: "abs(x)",
        hint: "absolute value",
        insert: "abs()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "sqrt(x)",
        hint: "square root",
        insert: "sqrt()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "log(x)",
        hint: "natural log",
        insert: "log()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "log10(x)",
        hint: "log base 10",
        insert: "log10()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "exp(x)",
        hint: "eˣ",
        insert: "exp()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "x ^ n",
        hint: "power",
        insert: " ^ ",
        caret_back: 0,
    },
];

const SIGNAL_FUNCTIONS: &[FunctionEntry] = &[
    FunctionEntry {
        label: "dB(x)",
        hint: "20·log₁₀|x|",
        insert: "dB()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "deriv(x)",
        hint: "d/dt",
        insert: "deriv()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "integ(x)",
        hint: "∫ dt",
        insert: "integ()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "clip(x, lo, hi)",
        hint: "limit range",
        insert: "clip(, , )",
        caret_back: 5,
    },
];

const MEASURE_FUNCTIONS: &[FunctionEntry] = &[
    FunctionEntry {
        label: "avg(x)",
        hint: "mean over the window",
        insert: "avg()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "rms(x)",
        hint: "root mean square",
        insert: "rms()",
        caret_back: 1,
    },
];

impl FunctionCategory {
    fn entries(self) -> &'static [FunctionEntry] {
        match self {
            FunctionCategory::Math => MATH_FUNCTIONS,
            FunctionCategory::Signal => SIGNAL_FUNCTIONS,
            FunctionCategory::Measure => MEASURE_FUNCTIONS,
        }
    }

    fn label(self) -> &'static str {
        match self {
            FunctionCategory::Math => "Math",
            FunctionCategory::Signal => "Signal",
            FunctionCategory::Measure => "Measure",
        }
    }

    const ALL: [FunctionCategory; 3] = [
        FunctionCategory::Math,
        FunctionCategory::Signal,
        FunctionCategory::Measure,
    ];
}

/// One signal row of the rail.
struct SignalRow {
    name: String,
    unit: &'static str,
    color: egui::Color32,
}

impl CalculatorPanel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stable id for the expression editor (caret state lives in egui memory).
    fn editor_id(ui: &Ui) -> egui::Id {
        ui.id().with("rspice.calc.editor")
    }

    /// The dialog body. The footer (Evaluate / Plot result / Clear) is the
    /// caller's — it owns the dialog and the cross-state plot action.
    pub fn show_body(&mut self, ui: &mut Ui, simulation: &SimulationState) {
        let t = Tokens::get(ui.ctx());
        let c = t.color;

        caption(ui, "Expression");
        let editor_id = Self::editor_id(ui);
        let response = ui.add(
            egui::TextEdit::singleline(&mut self.expression)
                .id(editor_id)
                .font(theme::mono(tokens::FS_2, FontWeight::Regular))
                .hint_text("dB(V(out) / V(in))")
                .desired_width(f32::INFINITY),
        );
        if response.changed() {
            self.history_at = None;
        }
        self.handle_history_keys(ui, &response);

        // Inline outcome row — result in ok, error in err, hint otherwise.
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let (text, color) = match &self.outcome {
                Some(Ok(value)) => (value.as_str(), c.ok),
                Some(Err(error)) => (error.as_str(), c.err),
                None => ("evaluate to see the result here", c.text_faint),
            };
            ui.label(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .color(color),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !self.history.is_empty() {
                    ui.label(
                        egui::RichText::new("↑/↓ recall history")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                }
            });
        });
        ui.add_space(8.0);

        // Two panes: signals rail + function reference.
        let mut insert: Option<(String, usize)> = None;
        two_pane(ui, PANE_RAIL_W, PANE_HEIGHT, |ui, side| match side {
            PaneSide::Rail => self.signals_pane(ui, simulation, &mut insert),
            PaneSide::Detail => self.functions_pane(ui, &mut insert),
        });

        if let Some((text, caret_back)) = insert {
            self.insert_at_caret(ui.ctx(), editor_id, &text, caret_back);
            ui.ctx().memory_mut(|m| m.request_focus(editor_id));
        }
    }

    /// Footer hint for the dialog: what the expression evaluates against.
    pub fn context_hint(&self, simulation: &SimulationState) -> String {
        match simulation.active_run() {
            Some(run) => format!("evaluates against run #{}", run.id),
            None => "no run yet — signals resolve after a simulation".to_owned(),
        }
    }

    /// Evaluate the current expression against the live waveform set.
    pub fn evaluate(&mut self, simulation: &SimulationState) {
        let text = self.expression.trim();
        if text.is_empty() {
            self.outcome = None;
            return;
        }
        let expr = parser::parse(text);
        let ctx = SimulationContext::new(simulation);
        self.outcome = Some(match evaluator::evaluate(&expr, &ctx) {
            Ok(CalcValue::Scalar(value)) => Ok(format!("= {}", fmt_si(value, "", 4))),
            Ok(CalcValue::Waveform(x, y)) => {
                let last = y.last().copied().unwrap_or(0.0);
                Ok(format!(
                    "= waveform · {} pts · last {}",
                    x.len(),
                    fmt_si(last, "", 3)
                ))
            }
            Err(error) => Err(error.to_string()),
        });
        if matches!(self.outcome, Some(Ok(_))) {
            let owned = text.to_owned();
            self.history.retain(|h| h != &owned);
            self.history.insert(0, owned);
            self.history.truncate(16);
        }
    }

    /// Clear the editor and outcome.
    pub fn clear(&mut self) {
        self.expression.clear();
        self.outcome = None;
        self.history_at = None;
    }

    // -----------------------------------------------------------------
    // panes
    // -----------------------------------------------------------------

    fn signals_pane(
        &mut self,
        ui: &mut Ui,
        simulation: &SimulationState,
        insert: &mut Option<(String, usize)>,
    ) {
        let t = Tokens::get(ui.ctx());
        let c = t.color;

        pane_header(ui, |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.signal_filter)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .hint_text("Filter signals…")
                    .desired_width(ui.available_width()),
            );
        });

        egui::ScrollArea::vertical()
            .id_salt("rspice.calc.signals")
            .max_height(PANE_HEIGHT - PANE_HEADER_H - PANE_FOOTER_H)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let filter = self.signal_filter.to_lowercase();
                let mut any = false;
                if let Some(run) = simulation.active_run() {
                    for analysis in &run.analyses {
                        let rows: Vec<SignalRow> = analysis
                            .waveforms
                            .iter()
                            .enumerate()
                            .filter(|(_, w)| {
                                filter.is_empty() || w.name.to_lowercase().contains(&filter)
                            })
                            .map(|(index, w)| SignalRow {
                                unit: signal_unit(&w.name),
                                color: crate::workbench::result_document::waveform_color(
                                    w, index, &t,
                                ),
                                name: w.name.clone(),
                            })
                            .collect();
                        if rows.is_empty() {
                            continue;
                        }
                        any = true;
                        pane_section_label(ui, &format!("run #{} · {}", run.id, analysis.label));
                        for row in rows {
                            if signal_row(ui, &row).clicked() {
                                *insert = Some((row.name.clone(), 0));
                            }
                        }
                    }
                }
                if !any {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(if filter.is_empty() {
                                "Run a simulation to list its signals"
                            } else {
                                "No signal matches the filter"
                            })
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                        );
                    });
                }
            });

        pane_footer(ui, "click inserts at the caret");
    }

    fn functions_pane(&mut self, ui: &mut Ui, insert: &mut Option<(String, usize)>) {
        let t = Tokens::get(ui.ctx());
        let c = t.color;

        pane_header(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            for category in FunctionCategory::ALL {
                if crate::ui::widgets::chip(ui, category.label(), self.category == category)
                    .clicked()
                {
                    self.category = category;
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("click inserts · caret lands in the parens")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            });
        });

        egui::ScrollArea::vertical()
            .id_salt("rspice.calc.functions")
            .max_height(PANE_HEIGHT - PANE_HEADER_H)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for entry in self.category.entries() {
                    if function_row(ui, entry).clicked() {
                        *insert = Some((entry.insert.to_owned(), entry.caret_back));
                    }
                }
            });
    }

    // -----------------------------------------------------------------
    // editing helpers
    // -----------------------------------------------------------------

    /// Insert at the caret (or append), then place the caret `caret_back`
    /// characters before the end of the inserted text — function templates
    /// land the caret inside their parentheses.
    fn insert_at_caret(
        &mut self,
        ctx: &egui::Context,
        editor_id: egui::Id,
        text: &str,
        caret_back: usize,
    ) {
        let mut state = egui::text_edit::TextEditState::load(ctx, editor_id).unwrap_or_default();
        let total_chars = self.expression.chars().count();
        let at = state
            .cursor
            .char_range()
            .map(|range| range.primary.index.min(total_chars))
            .unwrap_or(total_chars);
        let byte = char_to_byte(&self.expression, at);
        self.expression.insert_str(byte, text);

        let caret = at + text.chars().count() - caret_back.min(text.chars().count());
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::one(
                egui::text::CCursor::new(caret),
            )));
        state.store(ctx, editor_id);
        self.outcome = None;
    }

    /// ↑/↓ recall while the editor has focus.
    fn handle_history_keys(&mut self, ui: &Ui, response: &egui::Response) {
        if !response.has_focus() || self.history.is_empty() {
            return;
        }
        let (up, down) = ui.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowUp),
                i.key_pressed(egui::Key::ArrowDown),
            )
        });
        if up {
            let next = match self.history_at {
                None => {
                    self.stash = self.expression.clone();
                    0
                }
                Some(at) => (at + 1).min(self.history.len() - 1),
            };
            self.history_at = Some(next);
            self.expression = self.history[next].clone();
        } else if down {
            match self.history_at {
                Some(0) | None => {
                    if self.history_at.take().is_some() {
                        self.expression = std::mem::take(&mut self.stash);
                    }
                }
                Some(at) => {
                    self.history_at = Some(at - 1);
                    self.expression = self.history[at - 1].clone();
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// rows (the spec's .it vocabulary; pane chrome lives in ui/widgets/pane.rs)
// ---------------------------------------------------------------------------

fn caption(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.12 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
    ui.add_space(3.0);
}

fn signal_row(ui: &mut Ui, row: &SignalRow) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 26.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Insert signal {}, unit {}", row.name, row.unit),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    let painter = ui.painter();
    if hover > 0.0 {
        painter.rect_filled(
            rect,
            0.0,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    painter.rect_filled(
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 14.0, rect.center().y),
            egui::vec2(7.0, 7.0),
        ),
        2.0,
        row.color,
    );
    painter.text(
        egui::pos2(rect.left() + 26.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &row.name,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        mix(c.text_dim, c.text, hover),
    );
    painter.text(
        egui::pos2(rect.right() - 10.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        row.unit,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn function_row(ui: &mut Ui, entry: &FunctionEntry) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 26.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Insert function {}, {}", entry.label, entry.hint),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    let painter = ui.painter();
    if hover > 0.0 {
        painter.rect_filled(
            rect,
            0.0,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    painter.text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        entry.label,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        mix(c.text_dim, c.text, hover),
    );
    painter.text(
        egui::pos2(rect.right() - 10.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        entry.hint,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Display unit for a waveform name.
fn signal_unit(name: &str) -> &'static str {
    if name.starts_with("I(") || name.starts_with("i(") {
        "A"
    } else if name.starts_with('|') {
        "mag"
    } else if name.starts_with("phase(") {
        "°"
    } else {
        "V"
    }
}

/// Char index → byte index.
fn char_to_byte(text: &str, at: usize) -> usize {
    text.char_indices()
        .nth(at)
        .map(|(byte, _)| byte)
        .unwrap_or(text.len())
}
