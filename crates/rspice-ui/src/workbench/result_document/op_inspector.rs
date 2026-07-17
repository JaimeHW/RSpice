//! OP — the per-device operating-point inspector.
//!
//! The Spectre-style OP info as a workspace surface (results-v2 design,
//! section 06): device rows grouped by family, region chips, engineering
//! notation, a docbar filter, and click-to-sort numeric columns. Data
//! comes from the engine's `device_op_report` attached to DC OP analyses.

use egui::Ui;

use crate::common::AppState;
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::measurement_table;

use super::well_hint;

const NAME_W: f32 = 150.0;
const REGION_W: f32 = 96.0;
const VALUE_MIN_W: f32 = 82.0;
const OP_MAX_VALUE_COLUMNS: usize = 8;
const ROW_H: f32 = 25.0;
const CELL_INSET: f32 = 10.0;

fn op_table_min_width() -> f32 {
    NAME_W + REGION_W + VALUE_MIN_W * OP_MAX_VALUE_COLUMNS as f32
}

fn op_column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width, row.height()),
    )
}

fn paint_op_cell(
    ui: &Ui,
    cell: egui::Rect,
    text: impl ToString,
    align: egui::Align2,
    font: egui::FontId,
    color: egui::Color32,
) {
    let x = if align == egui::Align2::RIGHT_CENTER {
        cell.right() - CELL_INSET
    } else {
        cell.left() + CELL_INSET
    };
    ui.painter()
        .with_clip_rect(cell.shrink2(egui::vec2(2.0, 0.0)))
        .text(egui::pos2(x, cell.center().y), align, text, font, color);
}

/// Column metadata per device family: header label and value unit.
fn family_columns(device_kind: &str) -> &'static [(&'static str, &'static str)] {
    match device_kind {
        "MOSFET" => &[
            ("id", "A"),
            ("vgs", "V"),
            ("vds", "V"),
            ("vbs", "V"),
            ("vth", "V"),
            ("gm", "S"),
            ("gds", "S"),
            ("gmb", "S"),
        ],
        "BJT" => &[
            ("ic", "A"),
            ("ib", "A"),
            ("vbe", "V"),
            ("vce", "V"),
            ("beta", ""),
            ("gm", "S"),
        ],
        "DIODE" => &[("vd", "V"), ("id", "A"), ("gd", "S")],
        _ => &[],
    }
}

/// Region chip tint following the design: saturation = ok, linear =
/// trace-2 (blue family), cutoff = err.
fn region_colors(region: &str, t: &Tokens) -> (egui::Color32, egui::Color32) {
    let c = &t.color;
    let fg = match region {
        "saturation" => c.ok,
        "cutoff" => c.err,
        _ => c.traces[1],
    };
    (fg.gamma_multiply(0.16), fg)
}

/// The most recent device-OP report in the active run, with its analysis
/// label for the header.
fn active_report(state: &AppState) -> Option<(&rspice_core::circuit::DeviceOpReport, &str)> {
    let run = state.simulation.active_run()?;
    run.analyses.iter().rev().find_map(|analysis| {
        analysis
            .device_op
            .as_ref()
            .map(|r| (r, analysis.label.as_str()))
    })
}

/// One flattened, filter-matched row.
struct Row<'a> {
    entry: &'a rspice_core::circuit::DeviceOpEntry,
}

impl Row<'_> {
    fn value(&self, key: &str) -> Option<f64> {
        self.entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
    }

    fn matches(&self, filter: &str) -> bool {
        if filter.is_empty() {
            return true;
        }
        let filter = filter.to_ascii_lowercase();
        self.entry.name.to_ascii_lowercase().contains(&filter)
            || self
                .entry
                .device_kind
                .to_ascii_lowercase()
                .contains(&filter)
            || self
                .entry
                .region
                .is_some_and(|region| region.contains(&filter))
    }
}

/// Render the OP inspector center view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some((report, _label)) = active_report(state) else {
        well_hint(
            ui,
            "No operating-point report — run a DC operating point (.op) analysis",
        );
        return;
    };
    // Snapshot what rendering needs so the borrow on `state` can drop.
    let report = report.clone();
    let filter = state.ui.results.op_filter.clone();
    let sort = state.ui.results.op_sort.clone();
    let mut clicked_sort: Option<String> = None;
    let viewport_width = ui.available_width().max(1.0);

    egui::ScrollArea::both()
        .id_salt("rspice.results.op")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let table_width = viewport_width.max(op_table_min_width());
            ui.set_min_width(table_width);
            ui.add_space(4.0);
            for family in ["MOSFET", "BJT", "DIODE"] {
                let rows: Vec<Row> = report
                    .entries
                    .iter()
                    .filter(|entry| entry.device_kind == family)
                    .map(|entry| Row { entry })
                    .filter(|row| row.matches(&filter))
                    .collect();
                if rows.is_empty() {
                    continue;
                }
                let mut rows = rows;
                if let Some((key, ascending)) = &sort {
                    rows.sort_by(|a, b| {
                        let av = a.value(key).unwrap_or(f64::NEG_INFINITY);
                        let bv = b.value(key).unwrap_or(f64::NEG_INFINITY);
                        let ord = av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal);
                        if *ascending { ord } else { ord.reverse() }
                    });
                }

                let columns = family_columns(family);
                let width = table_width;
                let value_w =
                    ((width - NAME_W - REGION_W) / columns.len().max(1) as f32).max(VALUE_MIN_W);

                // Family section header.
                let (head, _) =
                    ui.allocate_exact_size(egui::vec2(width, 26.0), egui::Sense::hover());
                paint_op_cell(
                    ui,
                    head,
                    format!("{family}S — {}", rows.len()),
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_0, FontWeight::Medium),
                    c.text_faint,
                );

                // Column header row (clickable: sorts by that column).
                let (header, _) =
                    ui.allocate_exact_size(egui::vec2(width, 22.0), egui::Sense::hover());
                ui.painter().hline(
                    header.x_range(),
                    header.bottom() - 0.5,
                    egui::Stroke::new(1.0, c.border),
                );
                paint_op_cell(
                    ui,
                    op_column_rect(header, 0.0, NAME_W),
                    "DEVICE",
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                );
                paint_op_cell(
                    ui,
                    op_column_rect(header, NAME_W, REGION_W),
                    "REGION",
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                );
                for (i, (key, _unit)) in columns.iter().enumerate() {
                    let cell =
                        op_column_rect(header, NAME_W + REGION_W + i as f32 * value_w, value_w);
                    let sorted_here = sort.as_ref().is_some_and(|(k, _)| k == key);
                    let label = key.to_uppercase();
                    let galley = ui.painter().layout_no_wrap(
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        if sorted_here { c.accent } else { c.text_faint },
                    );
                    let arrow_width = if sorted_here { 10.0 } else { 0.0 };
                    let pos = egui::pos2(
                        cell.right() - CELL_INSET - arrow_width - galley.size().x,
                        header.center().y - galley.size().y / 2.0,
                    );
                    let response = ui.interact(
                        cell,
                        ui.id().with(("op-sort", family, key)),
                        egui::Sense::click(),
                    );
                    response.widget_info(|| {
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Button,
                            ui.is_enabled(),
                            format!(
                                "Sort {} devices by {} {}",
                                family,
                                key,
                                if sorted_here
                                    && sort.as_ref().is_some_and(|(_, ascending)| *ascending)
                                {
                                    "descending"
                                } else {
                                    "ascending"
                                }
                            ),
                        )
                    });
                    if response.hovered() {
                        ui.painter().rect_filled(cell, 0.0, c.bg_hover);
                    }
                    ui.painter()
                        .with_clip_rect(cell.shrink2(egui::vec2(2.0, 0.0)))
                        .galley(pos, galley, c.text_faint);
                    if sorted_here {
                        let ascending = sort.as_ref().is_some_and(|(_, ascending)| *ascending);
                        let center = egui::pos2(cell.right() - CELL_INSET - 3.5, header.center().y);
                        let direction = if ascending { -1.0 } else { 1.0 };
                        ui.painter()
                            .with_clip_rect(cell)
                            .add(egui::Shape::closed_line(
                                vec![
                                    center + egui::vec2(0.0, direction * 3.0),
                                    center + egui::vec2(3.0, -direction * 2.0),
                                    center + egui::vec2(-3.0, -direction * 2.0),
                                ],
                                egui::Stroke::new(1.0, c.accent),
                            ));
                    }
                    theme::paint_focus_ring(ui, &response, cell.shrink(1.0));
                    if response.clicked() {
                        clicked_sort = Some((*key).to_owned());
                    }
                }

                // Device rows.
                for row in &rows {
                    let (rect, response) =
                        ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
                    response.widget_info(|| {
                        let values = columns
                            .iter()
                            .map(|(key, unit)| {
                                let value = row.value(key).map_or_else(
                                    || "not available".to_owned(),
                                    |value| {
                                        if unit.is_empty() {
                                            format!("{value:.1}")
                                        } else {
                                            fmt_si(value, unit, 3)
                                        }
                                    },
                                );
                                format!("{key} {value}")
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Label,
                            ui.is_enabled(),
                            format!(
                                "{} device {}, region {}, {}",
                                family,
                                row.entry.name,
                                row.entry.region.unwrap_or("not reported"),
                                values,
                            ),
                        )
                    });
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_role(egui::accesskit::Role::Row);
                    });
                    if !ui.is_rect_visible(rect) {
                        continue;
                    }
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
                    }
                    ui.painter().hline(
                        rect.x_range(),
                        rect.bottom() - 0.5,
                        egui::Stroke::new(1.0, c.border.gamma_multiply(0.6)),
                    );

                    paint_op_cell(
                        ui,
                        op_column_rect(rect, 0.0, NAME_W),
                        row.entry.name.as_str(),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        c.text,
                    );

                    if let Some(region) = row.entry.region {
                        let region_cell = op_column_rect(rect, NAME_W, REGION_W);
                        let (bg, fg) = region_colors(region, &t);
                        let galley = ui.painter().layout_no_wrap(
                            region.to_owned(),
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            fg,
                        );
                        let chip = egui::Rect::from_min_size(
                            egui::pos2(
                                region_cell.left() + CELL_INSET,
                                rect.center().y - galley.size().y / 2.0 - 2.0,
                            ),
                            galley.size() + egui::vec2(12.0, 4.0),
                        );
                        let region_painter = ui
                            .painter()
                            .with_clip_rect(region_cell.shrink2(egui::vec2(2.0, 0.0)));
                        region_painter.rect_filled(chip, chip.height() / 2.0, bg);
                        region_painter.galley(chip.min + egui::vec2(6.0, 2.0), galley, fg);
                    }

                    for (i, (key, unit)) in columns.iter().enumerate() {
                        let text = row
                            .value(key)
                            .map(|value| {
                                if unit.is_empty() {
                                    format!("{value:.1}")
                                } else {
                                    fmt_si(value, unit, 3)
                                }
                            })
                            .unwrap_or_else(|| "—".to_owned());
                        paint_op_cell(
                            ui,
                            op_column_rect(rect, NAME_W + REGION_W + i as f32 * value_w, value_w),
                            text,
                            egui::Align2::RIGHT_CENTER,
                            theme::mono(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                    }
                }
                ui.add_space(14.0);
            }
        });

    // Header sort clicks: same column toggles direction, new column starts
    // descending (largest magnitudes first — the analog habit).
    if let Some(key) = clicked_sort {
        let next = match &state.ui.results.op_sort {
            Some((current, ascending)) if *current == key => Some((key, !*ascending)),
            _ => Some((key, false)),
        };
        state.ui.results.op_sort = next;
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;

    #[test]
    fn op_table_keeps_all_mosfet_columns_at_phone_width() {
        assert_eq!(family_columns("MOSFET").len(), OP_MAX_VALUE_COLUMNS);
        assert_eq!(op_table_min_width(), 902.0);
        assert!(op_table_min_width() > 390.0);
    }
}

/// Right panel: report summary (counts by family and region, extremes).
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    use crate::ui::widgets::section_header;

    let Some((report, label)) = active_report(state) else {
        return;
    };

    section_header(ui, "OP report", None);
    let total = report.entries.len();
    let count_kind = |kind: &str| {
        report
            .entries
            .iter()
            .filter(|e| e.device_kind == kind)
            .count()
    };
    let count_region = |region: &str| {
        report
            .entries
            .iter()
            .filter(|e| e.region == Some(region))
            .count()
    };

    let mosfets = count_kind("MOSFET").to_string();
    let bjts = count_kind("BJT").to_string();
    let diodes = count_kind("DIODE").to_string();
    let total_s = total.to_string();
    let mut rows: Vec<(&str, &str)> = vec![("Analysis", label), ("Devices", &total_s)];
    if mosfets != "0" {
        rows.push(("MOSFETs", &mosfets));
    }
    if bjts != "0" {
        rows.push(("BJTs", &bjts));
    }
    if diodes != "0" {
        rows.push(("Diodes", &diodes));
    }
    measurement_table(ui, &rows);

    let sat = count_region("saturation");
    let lin = count_region("linear");
    let cut = count_region("cutoff");
    if sat + lin + cut > 0 {
        ui.add_space(8.0);
        section_header(ui, "Regions", None);
        let sat_s = sat.to_string();
        let lin_s = lin.to_string();
        let cut_s = cut.to_string();
        let mut rows: Vec<(&str, &str)> = Vec::new();
        if sat > 0 {
            rows.push(("Saturation", &sat_s));
        }
        if lin > 0 {
            rows.push(("Linear", &lin_s));
        }
        if cut > 0 {
            rows.push(("Cutoff", &cut_s));
        }
        measurement_table(ui, &rows);
    }

    // The strongest transconductor — the row analog designers look for.
    let best_gm = report
        .entries
        .iter()
        .filter_map(|entry| {
            entry
                .params
                .iter()
                .find(|(name, _)| *name == "gm")
                .map(|(_, gm)| (entry.name.as_str(), *gm))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    if let Some((name, gm)) = best_gm {
        ui.add_space(8.0);
        section_header(ui, "Largest gm", None);
        let value = fmt_si(gm, "S", 3);
        measurement_table(ui, &[(name, value.as_str())]);
    }
}
