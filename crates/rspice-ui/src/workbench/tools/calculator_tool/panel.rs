//! Waveform calculator — the egui half of the floating tool.
//!
//! Keyboard-first: there is no keypad — the two things a calculator over
//! simulation data actually needs are the active run's signals and the
//! function reference, both click-to-insert at the caret. The result (or
//! error) renders inline under the expression; the tool footer's
//! "Plot result" hands the expression to the waves strips as an
//! expression trace.
//!
//! The state this draws lives at
//! `workbench::app_state::session::calculator`, because `AppState` owns it
//! across frames. This module is a second inherent impl on that type:
//! rendering stays with the renderer, session data stays with the session.

use egui::Ui;

use crate::state::SimulationState;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    PANE_FOOTER_H, PANE_HEADER_H, PANE_RAIL_W, PaneSide, pane_footer, pane_header,
    pane_section_label, two_pane,
};
use crate::workbench::app_state::session::calculator::{CalculatorPanel, FunctionCategory};

const PANE_HEIGHT: f32 = 286.0;

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
    FunctionEntry {
        label: "unwrap(x)",
        hint: "continuous phase",
        insert: "unwrap()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "xval(x)",
        hint: "domain as a trace",
        insert: "xval()",
        caret_back: 1,
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
    FunctionEntry {
        label: "min(x)",
        hint: "lowest sample",
        insert: "min()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "max(x)",
        hint: "highest sample",
        insert: "max()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "pp(x)",
        hint: "peak to peak",
        insert: "pp()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "yval(x, at)",
        hint: "value at an x",
        insert: "yval(, )",
        caret_back: 3,
    },
    FunctionEntry {
        label: "cross(x, lvl, n)",
        hint: "x of the nth crossing",
        insert: "cross(, , )",
        caret_back: 5,
    },
    FunctionEntry {
        label: "freq(x)",
        hint: "repetition rate",
        insert: "freq()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "period(x)",
        hint: "1 / freq",
        insert: "period()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "duty(x)",
        hint: "% above mid level",
        insert: "duty()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "rise(x)",
        hint: "10–90 % rising edge",
        insert: "rise()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "fall(x)",
        hint: "90–10 % falling edge",
        insert: "fall()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "overshoot(x)",
        hint: "% past the final value",
        insert: "overshoot()",
        caret_back: 1,
    },
    FunctionEntry {
        label: "settling(x, %)",
        hint: "time into the band",
        insert: "settling(, )",
        caret_back: 3,
    },
    FunctionEntry {
        label: "delay(a, b)",
        hint: "between mid crossings",
        insert: "delay(, )",
        caret_back: 3,
    },
    FunctionEntry {
        label: "thd(x)",
        hint: "% distortion, one counted fundamental",
        insert: "thd()",
        caret_back: 1,
    },
];

/// Stated where the functions are listed, because its absence is otherwise
/// read as an oversight. The calculator's value model carries a real
/// `(x, y)` pair, so there is no complex datum for `mag`/`phase`/`re`/`im`
/// to operate on; AC magnitude and phase are read from the strip that
/// retains them.
const REAL_ONLY_NOTICE: &str = "real series only — no mag/phase/re/im yet";

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
struct SignalRow<'a> {
    name: String,
    unit: &'a str,
    color: egui::Color32,
}

impl CalculatorPanel {
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
        // The readout is rounded to eight significant digits, so a result
        // is click-to-copy: the clipboard gets the exact f64 behind it.
        ui.add_space(4.0);
        let mut copy: Option<String> = None;
        ui.horizontal(|ui| {
            let (text, color) = match &self.outcome {
                Some(Ok(result)) => (result.readout.as_str(), c.ok),
                Some(Err(error)) => (error.as_str(), c.err),
                None => ("evaluate to see the result here", c.text_faint),
            };
            let mut readout = egui::Label::new(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .color(color),
            );
            if let Some(Ok(_)) = &self.outcome {
                readout = readout.sense(egui::Sense::click());
            }
            let response = ui.add(readout);
            if let Some(Ok(result)) = &self.outcome {
                let exact = result.exact_text();
                let what = if result.exact_is_last_sample {
                    "last sample"
                } else {
                    "result"
                };
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        format!("Copy exact {what}: {exact}"),
                    )
                });
                theme::paint_focus_ring_outset(ui, &response, response.rect);
                let response = response
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text(format!(
                        "Click or press Enter/Space to copy the exact {what}: {exact}"
                    ));
                let keyboard_activate = response.has_focus()
                    && ui.input_mut(|input| {
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                            || input.consume_key(egui::Modifiers::NONE, egui::Key::Space)
                    });
                if response.clicked() || keyboard_activate {
                    copy = Some(exact);
                }
            }
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
        if let Some(exact) = copy {
            ui.ctx().copy_text(exact);
        }
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
                        // The results workspace owns the name→unit mapping.
                        // A local guess here labelled every unrecognised
                        // signal — noise densities, decibel magnitudes,
                        // S-parameters, powers — as volts.
                        let analysis_unit =
                            crate::workbench::documents::result_document::analysis_default_unit(
                                analysis.analysis_type,
                            );
                        let rows: Vec<SignalRow> = analysis
                            .waveforms
                            .iter()
                            .enumerate()
                            .filter(|(_, w)| {
                                filter.is_empty() || w.name.to_lowercase().contains(&filter)
                            })
                            .map(|(index, w)| SignalRow {
                                unit: crate::workbench::documents::result_document::browser_signal_unit(
                                    &w.name,
                                    w.unit.as_deref(),
                                    analysis_unit,
                                ),
                                color: crate::workbench::documents::result_document::waveform_color(
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
            .max_height(PANE_HEIGHT - PANE_HEADER_H - PANE_FOOTER_H)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for entry in self.category.entries() {
                    if function_row(ui, entry).clicked() {
                        *insert = Some((entry.insert.to_owned(), entry.caret_back));
                    }
                }
            });

        pane_footer(ui, REAL_ONLY_NOTICE);
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
            .map(|range| range.primary.index.0.min(total_chars))
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

/// Char index → byte index.
fn char_to_byte(text: &str, at: usize) -> usize {
    text.char_indices()
        .nth(at)
        .map(|(byte, _)| byte)
        .unwrap_or(text.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::calculator::EvaluationError;
    use crate::analysis::calculator::functions::FunctionRegistry;

    /// Every function this pane advertises must be one the evaluator can
    /// actually run.
    ///
    /// This pane listed `dB(x)` for as long as there was no `db` arm in the
    /// dispatch table, so clicking the row inserted a call that came back
    /// "Unknown function: dB". A reference that lies is worse than no
    /// reference, and nothing but this test connects the two lists.
    #[test]
    fn every_advertised_function_is_one_the_evaluator_knows() {
        let mut checked = 0;
        for category in FunctionCategory::ALL {
            for entry in category.entries() {
                // Operator rows (`x ^ n`) name no function.
                let Some(open) = entry.insert.find('(') else {
                    continue;
                };
                let name = &entry.insert[..open];
                // Arity is the row's business; existence is this test's. A
                // deliberately wrong argument list still proves the name
                // resolves, because only an unknown name reports itself as
                // unknown.
                let outcome = FunctionRegistry::dispatch(name, Vec::new());
                assert!(
                    !matches!(outcome, Err(EvaluationError::UnknownFunction(_))),
                    "the {} pane offers {} but the evaluator has no {name}",
                    category.label(),
                    entry.label
                );
                checked += 1;
            }
        }
        assert!(
            checked >= 20,
            "only {checked} function rows were checked; the reference lists more than that"
        );
    }

    /// The insert templates must land the caret where the row promises: in
    /// the parentheses, at the first empty argument slot.
    #[test]
    fn every_insert_template_parks_the_caret_inside_its_parentheses() {
        for category in FunctionCategory::ALL {
            for entry in category.entries() {
                let characters = entry.insert.chars().count();
                assert!(
                    entry.caret_back <= characters,
                    "{}: caret_back {} exceeds the {characters}-character template",
                    entry.label,
                    entry.caret_back
                );
                if entry.insert.ends_with(')') {
                    assert!(
                        entry.caret_back >= 1,
                        "{}: the caret must land inside the parentheses",
                        entry.label
                    );
                    let commas = entry.insert.matches(',').count();
                    assert_eq!(
                        entry.caret_back,
                        1 + 2 * commas,
                        "{}: the caret must land at the first argument slot",
                        entry.label
                    );
                }
            }
        }
    }
}
