//! SPECS — the measurements × runs matrix.
//!
//! Rows are `.MEAS` results, columns are the runs in the shelf, the
//! leading rail holds the spec bounds (results-v2 design, section 03).
//! Failing cells tint, the worst violator per row carries a ring, the
//! footer states each run's verdict, and clicking a cell focuses that
//! run across the whole workspace. Bounds are project design intent and
//! persist with the workspace ([`SpecEntry`]); the docbar toggles an
//! inline editor.

use std::collections::BTreeMap;

use egui::Ui;

use crate::common::AppState;
use crate::properties::engineering::parse_engineering_value;
use crate::state::{SimulationRun, SpecEntry};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};

use super::well_hint;

/// One spec row being edited, kept as text so partial input survives
/// frames. Parsed (engineering notation allowed) on apply.
#[derive(Debug, Clone, Default)]
pub struct SpecDraft {
    pub measurement: String,
    pub min: String,
    pub max: String,
    pub unit: String,
}

impl SpecDraft {
    fn from_entry(entry: &SpecEntry) -> Self {
        let fmt = |bound: Option<f64>| bound.map(|v| format!("{v}")).unwrap_or_default();
        Self {
            measurement: entry.measurement.clone(),
            min: fmt(entry.min),
            max: fmt(entry.max),
            unit: entry.unit.clone(),
        }
    }

    /// Parse into a spec entry. `Err` carries the offending field label.
    fn parse(&self) -> Result<Option<SpecEntry>, &'static str> {
        let name = self.measurement.trim();
        if name.is_empty() {
            return Ok(None); // blank rows are simply dropped
        }
        let bound = |text: &str, field| -> Result<Option<f64>, &'static str> {
            let text = text.trim();
            if text.is_empty() {
                return Ok(None);
            }
            parse_engineering_value(text).map(Some).map_err(|_| field)
        };
        Ok(Some(SpecEntry {
            measurement: name.to_owned(),
            min: bound(&self.min, "min")?,
            max: bound(&self.max, "max")?,
            unit: self.unit.trim().to_owned(),
        }))
    }
}

/// The latest evaluated value of one measurement within a run, scanning
/// analyses newest-first. `Some(None)` means the measurement evaluated
/// and failed.
fn run_value(run: &SimulationRun, name: &str) -> Option<Option<f64>> {
    run.analyses.iter().rev().find_map(|analysis| {
        analysis
            .measurements
            .iter()
            .find(|m| m.name.eq_ignore_ascii_case(name))
            .map(|m| m.value)
    })
}

/// Every measurement name seen across the run history, first-seen order
/// preserved per run age (BTreeMap key keeps it stable and case-folded).
fn discovered_measurements(state: &AppState) -> Vec<String> {
    let mut names: BTreeMap<String, String> = BTreeMap::new();
    for run in &state.simulation.runs {
        for analysis in &run.analyses {
            for m in &analysis.measurements {
                names
                    .entry(m.name.to_ascii_lowercase())
                    .or_insert_with(|| m.name.clone());
            }
        }
    }
    names.into_values().collect()
}

/// Names discovered in runs but not yet bounded by a spec.
fn untracked_measurements(state: &AppState) -> Vec<String> {
    discovered_measurements(state)
        .into_iter()
        .filter(|name| {
            !state
                .workspace
                .specs
                .iter()
                .any(|spec| spec.measurement.eq_ignore_ascii_case(name))
        })
        .collect()
}

/// Render the specs-matrix center view (or the inline editor).
pub fn show(ui: &mut Ui, state: &mut AppState) {
    if state.shell.results.spec_drafts.is_some() {
        show_editor(ui, state);
        return;
    }

    let specs = state.workspace.specs.clone();
    let untracked = untracked_measurements(state);
    if specs.is_empty() && untracked.is_empty() {
        well_hint(
            ui,
            "No measurements — add .MEAS statements to the netlist, run, then bound them here",
        );
        return;
    }

    let runs: Vec<(u64, String)> = state
        .simulation
        .runs
        .iter()
        .map(|run| (run.id, format!("#{}", run.id)))
        .collect();
    let active_id = state.simulation.active_run().map(|run| run.id);
    let mut focus_run: Option<u64> = None;

    const NAME_W: f32 = 170.0;
    const RAIL_W: f32 = 130.0;
    const RUN_W: f32 = 92.0;
    const ROW_H: f32 = 25.0;

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let table_w = NAME_W + RAIL_W + runs.len() as f32 * RUN_W + 16.0;

    egui::ScrollArea::both()
        .id_salt("volta.results.specs")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(4.0);
            let width = ui.available_width().max(table_w);
            let run_x = |idx: usize| NAME_W + RAIL_W + (idx as f32 + 1.0) * RUN_W;

            // Header row.
            let (header, _) = ui.allocate_exact_size(egui::vec2(width, 22.0), egui::Sense::hover());
            ui.painter().hline(
                header.x_range(),
                header.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            let head = |x: f32, label: &str, color: egui::Color32, align: egui::Align2| {
                ui.painter().text(
                    egui::pos2(x, header.center().y),
                    align,
                    label,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    color,
                );
            };
            head(
                header.left() + 10.0,
                "MEASUREMENT",
                c.text_faint,
                egui::Align2::LEFT_CENTER,
            );
            head(
                header.left() + NAME_W,
                "SPEC",
                c.text_faint,
                egui::Align2::LEFT_CENTER,
            );
            for (idx, (id, label)) in runs.iter().enumerate() {
                let color = if Some(*id) == active_id {
                    c.accent
                } else {
                    c.text_faint
                };
                head(
                    header.left() + run_x(idx),
                    label,
                    color,
                    egui::Align2::RIGHT_CENTER,
                );
            }

            // One row per spec; failing runs accumulate for the footer.
            let mut failed_runs: Vec<bool> = vec![false; runs.len()];
            for spec in &specs {
                let cells: Vec<Option<Option<f64>>> = state
                    .simulation
                    .runs
                    .iter()
                    .map(|run| run_value(run, &spec.measurement))
                    .collect();
                // The worst violator (largest violation) carries the ring.
                let worst = cells
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, cell)| match cell {
                        Some(Some(v)) if !spec.passes(*v) => Some((idx, spec.violation(*v))),
                        _ => None,
                    })
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(idx, _)| idx);

                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
                if !ui.is_rect_visible(rect) {
                    continue;
                }
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom() - 0.5,
                    egui::Stroke::new(1.0, c.border.gamma_multiply(0.6)),
                );
                ui.painter().text(
                    egui::pos2(rect.left() + 10.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &spec.measurement,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );
                ui.painter().text(
                    egui::pos2(rect.left() + NAME_W, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    rail_text(spec),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                );

                for (idx, cell) in cells.iter().enumerate() {
                    let right = rect.left() + run_x(idx);
                    let cell_rect = egui::Rect::from_min_max(
                        egui::pos2(right - RUN_W + 6.0, rect.top() + 1.0),
                        egui::pos2(right + 6.0, rect.bottom() - 1.0),
                    );
                    let response = ui.interact(
                        cell_rect,
                        ui.id().with(("spec-cell", &spec.measurement, idx)),
                        egui::Sense::click(),
                    );

                    let (text, color) = match cell {
                        Some(Some(v)) => {
                            let pass = spec.passes(*v);
                            if !pass {
                                failed_runs[idx] = true;
                                ui.painter().rect_filled(
                                    cell_rect,
                                    2.0,
                                    c.err.gamma_multiply(0.14),
                                );
                                if worst == Some(idx) {
                                    ui.painter().rect_stroke(
                                        cell_rect.shrink(0.75),
                                        2.0,
                                        egui::Stroke::new(1.5, c.err),
                                    );
                                }
                            }
                            (fmt_si(*v, &spec.unit, 3), if pass { c.text } else { c.err })
                        }
                        Some(None) => {
                            failed_runs[idx] = true;
                            ("fail".to_owned(), c.err)
                        }
                        None => ("—".to_owned(), c.text_faint),
                    };
                    if response.hovered() {
                        ui.painter().rect_filled(cell_rect, 2.0, c.bg_hover);
                    }
                    ui.painter().text(
                        egui::pos2(right, rect.center().y),
                        egui::Align2::RIGHT_CENTER,
                        text,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        color,
                    );
                    if response.clicked() {
                        focus_run = Some(runs[idx].0);
                    }
                }
            }

            // Footer verdict per run — only meaningful with bounded rows.
            if !specs.is_empty() {
                let (foot, _) =
                    ui.allocate_exact_size(egui::vec2(width, 26.0), egui::Sense::hover());
                ui.painter().text(
                    egui::pos2(foot.left() + 10.0, foot.center().y),
                    egui::Align2::LEFT_CENTER,
                    "run verdict",
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_dim,
                );
                for (idx, failed) in failed_runs.iter().enumerate() {
                    let (label, color) = if *failed {
                        ("✕ fail", c.err)
                    } else {
                        ("✓ pass", c.ok)
                    };
                    ui.painter().text(
                        egui::pos2(foot.left() + run_x(idx), foot.center().y),
                        egui::Align2::RIGHT_CENTER,
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Medium),
                        color,
                    );
                }
            }

            // Unbounded measurements: value-only rows inviting a bound.
            if !untracked.is_empty() {
                let (head, _) =
                    ui.allocate_exact_size(egui::vec2(width, 26.0), egui::Sense::hover());
                ui.painter().text(
                    egui::pos2(head.left() + 10.0, head.center().y),
                    egui::Align2::LEFT_CENTER,
                    format!("UNBOUNDED — {}", untracked.len()),
                    theme::mono(tokens::FS_0, FontWeight::Medium),
                    c.text_faint,
                );
                for name in &untracked {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
                    if !ui.is_rect_visible(rect) {
                        continue;
                    }
                    ui.painter().hline(
                        rect.x_range(),
                        rect.bottom() - 0.5,
                        egui::Stroke::new(1.0, c.border.gamma_multiply(0.6)),
                    );
                    ui.painter().text(
                        egui::pos2(rect.left() + 10.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        name,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        c.text_dim,
                    );
                    ui.painter().text(
                        egui::pos2(rect.left() + NAME_W, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        "—",
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    );
                    for (idx, run) in state.simulation.runs.iter().enumerate() {
                        let text = match run_value(run, name) {
                            Some(Some(v)) => fmt_si(v, "", 3),
                            Some(None) => "fail".to_owned(),
                            None => "—".to_owned(),
                        };
                        ui.painter().text(
                            egui::pos2(rect.left() + run_x(idx), rect.center().y),
                            egui::Align2::RIGHT_CENTER,
                            text,
                            theme::mono(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                    }
                }
            }
        });

    if let Some(run_id) = focus_run {
        let index = state
            .simulation
            .runs
            .iter()
            .position(|run| run.id == run_id);
        if let Some(index) = index {
            state.simulation.select_run(index);
            state.shell.results.clear_cursors();
        }
    }
}

/// The rail text: `≥ min · ≤ max unit` with whichever bounds exist.
fn rail_text(spec: &SpecEntry) -> String {
    let mut parts = Vec::new();
    if let Some(min) = spec.min {
        parts.push(format!("≥ {}", fmt_si(min, "", 3).trim()));
    }
    if let Some(max) = spec.max {
        parts.push(format!("≤ {}", fmt_si(max, "", 3).trim()));
    }
    if parts.is_empty() {
        return "—".to_owned();
    }
    let mut text = parts.join(" · ");
    if !spec.unit.is_empty() {
        text.push(' ');
        text.push_str(&spec.unit);
    }
    text
}

/// The inline spec editor: one text row per spec, engineering notation
/// accepted in the bound fields, with discovered-measurement shortcuts.
fn show_editor(ui: &mut Ui, state: &mut AppState) {
    let untracked = untracked_measurements(state);
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some(drafts) = state.shell.results.spec_drafts.as_mut() else {
        return;
    };
    let mut remove: Option<usize> = None;

    egui::ScrollArea::vertical()
        .id_salt("volta.results.specs-edit")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(
                        "Bounds accept engineering notation (10meg, 1u, 3.3). \
                         Empty bound = unbounded side.",
                    )
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
                );
            });
            ui.add_space(6.0);

            // Column captions.
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                for (label, width) in [
                    ("MEASUREMENT", 180.0),
                    ("MIN", 110.0),
                    ("MAX", 110.0),
                    ("UNIT", 70.0),
                ] {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(width, 18.0), egui::Sense::hover());
                    ui.painter().text(
                        egui::pos2(rect.left() + 4.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    );
                }
            });

            for (idx, draft) in drafts.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    let field = |ui: &mut Ui, text: &mut String, width: f32, hint: &str| {
                        ui.add(
                            egui::TextEdit::singleline(text)
                                .desired_width(width)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .hint_text(hint),
                        );
                    };
                    field(ui, &mut draft.measurement, 180.0, "meas name");
                    field(ui, &mut draft.min, 110.0, "—");
                    field(ui, &mut draft.max, 110.0, "—");
                    field(ui, &mut draft.unit, 70.0, "");
                    if ui.small_button("✕").on_hover_text("Remove spec").clicked() {
                        remove = Some(idx);
                    }
                    if let Err(field) = draft.parse() {
                        ui.label(
                            egui::RichText::new(format!("invalid {field}"))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.err),
                        );
                    }
                });
                ui.add_space(2.0);
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                if ui.button("Add spec").clicked() {
                    drafts.push(SpecDraft::default());
                }
            });

            // One-click drafts for measurements seen in runs but unbound.
            let missing: Vec<&String> = untracked
                .iter()
                .filter(|name| {
                    !drafts
                        .iter()
                        .any(|d| d.measurement.eq_ignore_ascii_case(name))
                })
                .collect();
            if !missing.is_empty() {
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("discovered:")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let mut add: Option<String> = None;
                    for name in missing {
                        if ui.small_button(name.as_str()).clicked() {
                            add = Some(name.clone());
                        }
                    }
                    if let Some(name) = add {
                        drafts.push(SpecDraft {
                            measurement: name,
                            ..Default::default()
                        });
                    }
                });
            }
        });

    if let Some(idx) = remove
        && let Some(drafts) = state.shell.results.spec_drafts.as_mut()
    {
        drafts.remove(idx);
    }
}

/// Apply the open editor's drafts to the workspace. Returns false (and
/// leaves the editor open) when a bound fails to parse.
pub fn apply_drafts(state: &mut AppState) -> bool {
    let Some(drafts) = state.shell.results.spec_drafts.as_ref() else {
        return true;
    };
    let mut specs = Vec::with_capacity(drafts.len());
    for draft in drafts {
        match draft.parse() {
            Ok(Some(entry)) => specs.push(entry),
            Ok(None) => {}
            Err(_) => return false,
        }
    }
    state.workspace.specs = specs;
    state.shell.results.spec_drafts = None;
    true
}

/// Open the editor seeded from the current workspace specs.
pub fn open_editor(state: &mut AppState) {
    let drafts = state
        .workspace
        .specs
        .iter()
        .map(SpecDraft::from_entry)
        .collect();
    state.shell.results.spec_drafts = Some(drafts);
}

/// Right panel: matrix totals and the worst violation on record.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let specs = &state.workspace.specs;
    if specs.is_empty() {
        section_header(ui, "Specs", None);
        measurement_table(ui, &[("Bounds", "none defined")]);
        return;
    }

    let mut cells = 0usize;
    let mut failures = 0usize;
    let mut worst: Option<(String, u64, f64, f64)> = None; // name, run, value, violation
    for run in &state.simulation.runs {
        for spec in specs {
            let Some(Some(value)) = run_value(run, &spec.measurement) else {
                continue;
            };
            cells += 1;
            if !spec.passes(value) {
                failures += 1;
                let violation = spec.violation(value);
                if worst.as_ref().is_none_or(|(_, _, _, w)| violation > *w) {
                    worst = Some((spec.measurement.clone(), run.id, value, violation));
                }
            }
        }
    }

    section_header(ui, "Specs matrix", None);
    let specs_n = specs.len().to_string();
    let runs_n = state.simulation.runs.len().to_string();
    let cells_s = cells.to_string();
    let fails_s = failures.to_string();
    measurement_table(
        ui,
        &[
            ("Specs", specs_n.as_str()),
            ("Runs", runs_n.as_str()),
            ("Evaluated", cells_s.as_str()),
            ("Failures", fails_s.as_str()),
        ],
    );

    if let Some((name, run_id, value, _)) = worst {
        ui.add_space(8.0);
        section_header(ui, "Worst violation", None);
        let where_s = format!("run #{run_id}");
        let value_s = fmt_si(value, "", 4);
        measurement_table(
            ui,
            &[
                (name.as_str(), value_s.as_str()),
                ("Where", where_s.as_str()),
            ],
        );
    }
}
